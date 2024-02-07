// SPDX-License-Identifier: CC0-1.0

use crate::ffi::{
    bitstream::CBitstream,
    bitstring::CBitstring,
    c_size_t, c_void,
    dag::{
        computeAnnotatedMerkleRoot, fillWitnessData, verifyNoDuplicateIdentityRoots, CAnalyses,
        CCombinatorCounters,
    },
    deserialize::{decodeMallocDag, decodeWitnessData},
    eval::{analyseBounds, evalTCOProgram},
    sha256::CSha256Midstate,
    type_inference::mallocTypeInference,
    ubounded, SimplicityErr, UBOUNDED_MAX,
};
use std::ptr;

#[cfg(feature = "test-utils")]
pub mod ffi {
    use super::*;

    extern "C" {
        pub static sizeof_ctx8Pruned: c_size_t;
        pub static ctx8Pruned: [u8; 5015];
        pub static ctx8Pruned_amr: [u32; 8];
        pub static ctx8Pruned_cmr: [u32; 8];
        pub static ctx8Pruned_imr: [u32; 8];
        pub static ctx8Pruned_cost: ubounded;

        pub static sizeof_ctx8Unpruned: c_size_t;
        pub static ctx8Unpruned: [u8; 4809];
        pub static ctx8Unpruned_amr: [u32; 8];
        pub static ctx8Unpruned_cmr: [u32; 8];
        pub static ctx8Unpruned_imr: [u32; 8];
        pub static ctx8Unpruned_cost: ubounded;

        pub static sizeof_schnorr0: c_size_t;
        pub static schnorr0: [u8; 137];
        pub static schnorr0_amr: [u32; 8];
        pub static schnorr0_cmr: [u32; 8];
        pub static schnorr0_imr: [u32; 8];
        pub static schnorr0_cost: ubounded;

        pub static sizeof_schnorr6: c_size_t;
        pub static schnorr6: [u8; 137];
        pub static schnorr6_amr: [u32; 8];
        pub static schnorr6_cmr: [u32; 8];
        pub static schnorr6_imr: [u32; 8];
        pub static schnorr6_cost: ubounded;

        /*
        // FIXME enable this test; is not 1->1, requires extra frame setup
        pub static sizeof_hashBlock: c_size_t;
        pub static hashBlock: [u8; 3259];
        pub static hashBlock_amr: [u32; 8];
        pub static hashBlock_cmr: [u32; 8];
        pub static hashBlock_imr: [u32; 8];

        // FIXME enable this test; requires a little but of extra work to set up an Elements env
        pub static elementsCheckSigHashAllTx1: [u8; 1151];
        pub static elementsCheckSigHashAllTx1_amr: [u32; 8];
        pub static elementsCheckSigHashAllTx1_cmr: [u32; 8];
        pub static elementsCheckSigHashAllTx1_imr: [u32; 8];
        */
    }
}

/// The result of parsing, typechecking, and running a Simplicity program
/// through the C FFI
#[derive(Copy, Clone, Debug)]
pub struct TestOutput {
    /// The AMR of the root node
    pub amr: CSha256Midstate,
    /// The CMR of the root node
    pub cmr: CSha256Midstate,
    /// The IMR of the root node
    pub imr: CSha256Midstate,
    /// The cost bound of the program
    pub cost_bound: ubounded,
    /// Whether or not evaluation succeded
    pub eval_result: SimplicityErr,
}

/// How far to run the test
///
/// If you stop the test early, the returned output will be incomplete.
#[derive(Copy, Clone, PartialOrd, Ord, PartialEq, Eq, Debug)]
pub enum TestUpTo {
    /// Just decode the program, not the witnesses. Will compute and retrieve the root CMR.
    DecodeProgram,
    /// Also decode witnesses into a bitstring
    DecodeWitness,
    /// Also do type inference
    TypeInference,
    /// Fill in the witness data, now that their types are known
    FillWitnessData,
    /// Compute and retrieve the root AMR.
    ComputeAmr,
    /// Compute and retrieve the root IMR; check that all IMRs are unique
    ComputeImr,
    /// Compute and retrieve the expected cost without any bounds
    ComputeCostUnbounded,
    /// Compute and retrieve the expected cost with strict bounds
    ComputeCostBounded,
    /// Fail the analysis if insufficient cells are provided
    CheckCellCount,
    /// Fail the analysis if insufficient budget is provided
    CheckBudget,
    /// Check that the program is 1-1. This will exclude arbitrary expressions, so
    /// you may want to stop at `ComputeImr` to get the maximum merkle root coverage.
    CheckOneOne,
    /// Evaluate the program and retrieve whether it succeded.
    Everything,
}

/// Run a program, check its merkle roots, and that it succeeds
pub fn run_test(
    program: &[u8],
    target_amr: &[u32; 8],
    target_cmr: &[u32; 8],
    target_imr: &[u32; 8],
    cost_bound: ubounded,
) {
    let result = run_program(program, TestUpTo::Everything).expect("running program");
    assert_eq!(result.amr, CSha256Midstate { s: *target_amr });
    assert_eq!(result.cmr, CSha256Midstate { s: *target_cmr });
    assert_eq!(result.imr, CSha256Midstate { s: *target_imr });
    assert_eq!(result.cost_bound, cost_bound);
    assert_eq!(result.eval_result, SimplicityErr::NoError);
}

/// Run a program, check its merkle roots, and that it fails
pub fn run_test_fail(
    program: &[u8],
    target_result: SimplicityErr,
    target_amr: &[u32; 8],
    target_cmr: &[u32; 8],
    target_imr: &[u32; 8],
    cost_bound: ubounded,
) {
    let result = run_program(program, TestUpTo::Everything).expect("running program");
    assert_eq!(result.amr, CSha256Midstate { s: *target_amr });
    assert_eq!(result.cmr, CSha256Midstate { s: *target_cmr });
    assert_eq!(result.imr, CSha256Midstate { s: *target_imr });
    assert_eq!(result.cost_bound, cost_bound);
    assert_eq!(result.eval_result, target_result);
}

struct FreeOnDrop(*mut c_void);
impl Drop for FreeOnDrop {
    fn drop(&mut self) {
        unsafe {
            libc::free(self.0);
        }
    }
}

/// Run a program and return data about it
///
/// This is mostly a direct port of `run_program` in C `tests.c`.
pub fn run_program(program: &[u8], test_up_to: TestUpTo) -> Result<TestOutput, SimplicityErr> {
    let mut result = TestOutput {
        amr: CSha256Midstate::default(),
        cmr: CSha256Midstate::default(),
        imr: CSha256Midstate::default(),
        cost_bound: 0,
        eval_result: SimplicityErr::NoError,
    };

    let mut stream = CBitstream::from(program);
    let mut witness = CBitstring::default();
    let mut census = CCombinatorCounters::default();
    unsafe {
        // 1. Parse DAG.
        let mut dag = ptr::null_mut();
        let len =
            SimplicityErr::from_i32(decodeMallocDag(&mut dag, &mut census, &mut stream))? as usize;
        assert!(!dag.is_null());
        let _d1 = FreeOnDrop(dag as *mut c_void);
        if test_up_to <= TestUpTo::DecodeProgram {
            return Ok(result);
        }
        decodeWitnessData(&mut witness, &mut stream).into_result()?;
        if test_up_to <= TestUpTo::DecodeWitness {
            return Ok(result);
        }

        // 2. Check CMR.
        result.cmr = (*dag.offset(len as isize - 1)).cmr;

        // 3. Do type inference.
        let mut type_dag = ptr::null_mut();
        mallocTypeInference(&mut type_dag, dag, len, &census).into_result()?;
        assert!(!type_dag.is_null());
        let _d2 = FreeOnDrop(type_dag as *mut c_void);
        if test_up_to <= TestUpTo::TypeInference {
            return Ok(result);
        }

        // 4. Fill witness data, now that we know the types
        fillWitnessData(dag, type_dag, len as c_size_t, witness).into_result()?;
        if test_up_to <= TestUpTo::FillWitnessData {
            return Ok(result);
        }

        // 5. Check AMR
        let mut analyses = vec![CAnalyses::default(); len];
        computeAnnotatedMerkleRoot(analyses.as_mut_ptr(), dag, type_dag, len);
        result.amr = analyses[len - 1].annotated_merkle_root;
        if test_up_to <= TestUpTo::ComputeAmr {
            return Ok(result);
        }

        // 6. Check IMR
        verifyNoDuplicateIdentityRoots(&mut result.imr, dag, type_dag, len).into_result()?;
        if test_up_to <= TestUpTo::ComputeImr {
            return Ok(result);
        }

        // 7. Test cost computation
        let mut cell_bound: ubounded = 0;
        let mut word_bound: ubounded = 0;
        let mut frame_bound: ubounded = 0;
        let mut cost_bound: ubounded = 0;
        // 7a. Analysis when cost is unbounded
        analyseBounds(
            &mut cell_bound,
            &mut word_bound,
            &mut frame_bound,
            &mut cost_bound,
            UBOUNDED_MAX,
            UBOUNDED_MAX,
            dag,
            type_dag,
            len,
        )
        .into_result()?;
        result.cost_bound = cost_bound;
        if test_up_to <= TestUpTo::ComputeCostUnbounded {
            return Ok(result);
        }
        // 7b. analysis with strict bounds
        analyseBounds(
            &mut cell_bound,
            &mut word_bound,
            &mut frame_bound,
            &mut cost_bound,
            cell_bound,
            cost_bound,
            dag,
            type_dag,
            len,
        )
        .into_result()?;
        if test_up_to <= TestUpTo::ComputeCostBounded {
            return Ok(result);
        }
        // 7c. analysis with strict cell bounds
        if 0 < cell_bound {
            let res = analyseBounds(
                &mut cell_bound,
                &mut word_bound,
                &mut frame_bound,
                &mut cost_bound,
                cell_bound - 1,
                cost_bound,
                dag,
                type_dag,
                len,
            )
            .into_result()
            .expect_err("should fail");
            assert_eq!(res, SimplicityErr::ExecMemory);
        }
        if test_up_to <= TestUpTo::CheckCellCount {
            return Ok(result);
        }
        // 7d. analysis with strict cost bounds
        if 0 < cost_bound {
            let res = analyseBounds(
                &mut cell_bound,
                &mut word_bound,
                &mut frame_bound,
                &mut cost_bound,
                cell_bound,
                cost_bound - 1,
                dag,
                type_dag,
                len,
            )
            .into_result()
            .expect_err("should fail");
            assert_eq!(res, SimplicityErr::ExecBudget);
        }
        if test_up_to <= TestUpTo::CheckBudget {
            return Ok(result);
        }

        // 8. Check that thtis is a 1->1 program. This must be done before evalTCOProgram
        if (*dag.offset(len as isize - 1)).aux_types.types[0] != 0
            || (*dag.offset(len as isize - 1)).aux_types.types[1] != 0
        {
            return Err(SimplicityErr::TypeInferenceNotProgram);
        }
        if test_up_to <= TestUpTo::CheckOneOne {
            return Ok(result);
        }

        // 9. Run the program
        result.eval_result = evalTCOProgram(dag, type_dag, len, ptr::null(), ptr::null());
    }

    Ok(result)
}

pub fn parse_root(ptr: &[u32; 8]) -> [u8; 32] {
    let mut a = [0u8; 32];
    for i in 0..8 {
        let x = ptr[i];
        a[i * 4] = (x >> 24) as u8;
        a[i * 4 + 1] = (x >> 16) as u8;
        a[i * 4 + 2] = (x >> 8) as u8;
        a[i * 4 + 3] = x as u8;
    }
    a
}

/// Data structure to hold test cases from C simplicity
pub struct TestData {
    pub cmr: [u8; 32],
    pub amr: [u8; 32],
    pub imr: [u8; 32],
    pub prog: Vec<u8>,
    pub cost: ubounded,
}

#[cfg(feature = "test-utils")]
mod test_data {
    use super::*;
    use std::slice;
    pub fn schnorr0_test_data() -> TestData {
        unsafe {
            TestData {
                cmr: parse_root(&ffi::schnorr0_cmr),
                amr: parse_root(&ffi::schnorr0_amr),
                imr: parse_root(&ffi::schnorr0_imr),
                prog: slice::from_raw_parts(ffi::schnorr0.as_ptr(), ffi::sizeof_schnorr0).into(),
                cost: ffi::schnorr0_cost,
            }
        }
    }

    pub fn schnorr6_test_data() -> TestData {
        unsafe {
            TestData {
                cmr: parse_root(&ffi::schnorr6_cmr),
                amr: parse_root(&ffi::schnorr6_amr),
                imr: parse_root(&ffi::schnorr6_imr),
                prog: slice::from_raw_parts(ffi::schnorr6.as_ptr(), ffi::sizeof_schnorr6).into(),
                cost: ffi::schnorr6_cost,
            }
        }
    }

    pub fn ctx8_pruned_test_data() -> TestData {
        unsafe {
            TestData {
                cmr: parse_root(&ffi::ctx8Pruned_cmr),
                amr: parse_root(&ffi::ctx8Pruned_amr),
                imr: parse_root(&ffi::ctx8Pruned_imr),
                prog: slice::from_raw_parts(ffi::ctx8Pruned.as_ptr(), ffi::sizeof_ctx8Pruned)
                    .into(),
                cost: ffi::ctx8Pruned_cost,
            }
        }
    }

    pub fn ctx8_unpruned_test_data() -> TestData {
        unsafe {
            TestData {
                cmr: parse_root(&ffi::ctx8Unpruned_cmr),
                amr: parse_root(&ffi::ctx8Unpruned_amr),
                imr: parse_root(&ffi::ctx8Unpruned_imr),
                prog: slice::from_raw_parts(ffi::ctx8Unpruned.as_ptr(), ffi::sizeof_ctx8Unpruned)
                    .into(),
                cost: ffi::ctx8Unpruned_cost,
            }
        }
    }
}

#[cfg(feature = "test-utils")]
pub use test_data::*;

#[cfg(all(test, feature = "test-utils"))]
mod tests {
    use super::*;

    #[test]
    fn ctx8_pruned() {
        unsafe {
            assert_eq!(ffi::sizeof_ctx8Pruned, ffi::ctx8Pruned.len());
            run_test(
                &ffi::ctx8Pruned,
                &ffi::ctx8Pruned_amr,
                &ffi::ctx8Pruned_cmr,
                &ffi::ctx8Pruned_imr,
                ffi::ctx8Pruned_cost,
            );
        }
    }

    #[test]
    fn ctx8_unpruned() {
        unsafe {
            assert_eq!(ffi::sizeof_ctx8Unpruned, ffi::ctx8Unpruned.len());
            run_test_fail(
                &ffi::ctx8Unpruned,
                SimplicityErr::AntiDoS,
                &ffi::ctx8Unpruned_amr,
                &ffi::ctx8Unpruned_cmr,
                &ffi::ctx8Unpruned_imr,
                ffi::ctx8Unpruned_cost,
            );
        }
    }

    #[test]
    fn schnorr0() {
        unsafe {
            assert_eq!(ffi::sizeof_schnorr0, ffi::schnorr0.len());
            run_test(
                &ffi::schnorr0,
                &ffi::schnorr0_amr,
                &ffi::schnorr0_cmr,
                &ffi::schnorr0_imr,
                ffi::schnorr0_cost,
            );
        }
    }

    #[test]
    fn schnorr6() {
        unsafe {
            assert_eq!(ffi::sizeof_schnorr6, ffi::schnorr6.len());
            run_test_fail(
                &ffi::schnorr6,
                SimplicityErr::ExecJet,
                &ffi::schnorr6_amr,
                &ffi::schnorr6_cmr,
                &ffi::schnorr6_imr,
                ffi::schnorr6_cost,
            );
        }
    }
}
