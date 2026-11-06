// SPDX-License-Identifier: CC0-1.0

//! Checks of pruning on generated programs

use arbitrary::Unstructured;
use simplicity::bit_machine::ExecutionError;
use simplicity::ffi::tests::{ffi::SimplicityErr, run_program, TestUpTo};
use simplicity::hashes::sha256::Midstate;
use simplicity::jet::{elements::ElementsEnv, Elements};
use simplicity::{BitIter, BitMachine, Cost, RedeemNode};

use crate::generate_program;

/// Programs which cost more than this are skipped. Their cost comes from running
/// the same nodes many times, which takes time without exercising pruning more.
const MAX_COST: Cost = Cost::from_milliweight(1_000_000_000);

/// What [`check_prune`] did with a generated program.
#[derive(Copy, Clone, PartialEq, Eq, Debug)]
pub enum PruneOutcome {
    /// The program was too expensive or too large to run, and was skipped.
    Skipped,
    /// Pruning failed with [`ExecutionError::PruningFailed`]: it could not produce a
    /// program which consensus accepts.
    PruningFailed,
    /// The program was pruned and passed every check.
    Checked,
}

/// Generates a program from `data`, prunes it, and checks the pruned program
/// against the Rust decoder and Bit Machine, and against libsimplicity.
pub fn check_prune(data: &[u8]) -> PruneOutcome {
    let program = match generate_program(&mut Unstructured::new(data)) {
        Ok(program) => program,
        Err(_) => return PruneOutcome::Skipped,
    };
    if program.bounds().cost > MAX_COST {
        return PruneOutcome::Skipped;
    }
    let env = ElementsEnv::dummy();
    let mut mac = match BitMachine::for_program(&program) {
        Ok(mac) => mac,
        Err(_) => return PruneOutcome::Skipped,
    };
    mac.exec(&program, &env)
        .expect("generated programs contain nothing that fails");

    let pruned = match program.prune(&env) {
        Ok(pruned) => pruned,
        Err(ExecutionError::PruningFailed) => return PruneOutcome::PruningFailed,
        Err(e) => panic!("pruning should succeed when execution does: {e}"),
    };
    assert_eq!(pruned.cmr(), program.cmr(), "pruning should keep the CMR");
    BitMachine::for_program(&pruned)
        .expect("the pruned program is no larger")
        .exec(&pruned, &env)
        .expect("the pruned program should run");

    // A decoder infers the types of the pruned program from its encoding, so it
    // must infer the types that pruning did to read the witness data back. The
    // IHR only covers the outer types of the program, the AMR all of them.
    let (prog, wit) = pruned.to_vec_with_witness();
    let decoded =
        RedeemNode::decode::<_, _, Elements>(BitIter::from(&prog[..]), BitIter::from(&wit[..]))
            .expect("the pruned program should decode");
    assert_eq!(
        decoded.ihr(),
        pruned.ihr(),
        "the decoder should infer the types that pruning did"
    );
    assert_eq!(
        decoded.amr(),
        pruned.amr(),
        "the decoder should infer the types that pruning did"
    );
    assert_eq!(decoded.to_vec_with_witness(), (prog.clone(), wit.clone()));

    // libsimplicity, the consensus implementation, accepts the pruned program
    // and computes the same roots.
    let c_pruned = run_program(&prog, &wit, TestUpTo::Everything, None, None)
        .expect("libsimplicity should decode the pruned program");
    assert_eq!(
        c_pruned.eval_result,
        SimplicityErr::NoError,
        "libsimplicity should accept the pruned program"
    );
    assert_eq!(
        &Midstate::from(c_pruned.cmr).to_parts().0,
        pruned.cmr().as_ref()
    );
    assert_eq!(
        &Midstate::from(c_pruned.amr).to_parts().0,
        pruned.amr().as_ref()
    );
    assert_eq!(
        &Midstate::from(c_pruned.ihr).to_parts().0,
        pruned.ihr().as_ref()
    );

    // libsimplicity rejects the unpruned program for its unexecuted branches
    // exactly when pruning hid a branch, so both agree on what was executed.
    // This relies on generated programs having the types that inference gives
    // them, so that the encoding of the unpruned program decodes to the same
    // program.
    let unpruned = program.to_vec_with_witness();
    let c_unpruned = run_program(&unpruned.0, &unpruned.1, TestUpTo::Everything, None, None)
        .expect("libsimplicity should decode the unpruned program");
    let expected = if unpruned == (prog, wit) {
        SimplicityErr::NoError
    } else {
        SimplicityErr::AntiDoS
    };
    assert_eq!(
        c_unpruned.eval_result, expected,
        "libsimplicity should reject the unpruned program exactly when pruning changed it"
    );
    PruneOutcome::Checked
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::pseudorandom_inputs;

    #[test]
    fn check_prune_on_pseudorandom_inputs() {
        const N: usize = 1000;
        let (mut skipped, mut failed, mut checked) = (0, 0, 0);
        for data in pseudorandom_inputs(N) {
            match check_prune(&data) {
                PruneOutcome::Skipped => skipped += 1,
                PruneOutcome::PruningFailed => failed += 1,
                PruneOutcome::Checked => checked += 1,
            }
        }
        let stats =
            format!("of {N}: skipped {skipped}, pruning failed {failed}, checked {checked}");
        // Pruning cannot yet resolve every set of case nodes which become one node
        // when pruned, but generated programs should rarely contain one.
        assert!(failed <= N / 100, "{stats}");
        assert!(checked >= N * 95 / 100, "{stats}");
    }
}
