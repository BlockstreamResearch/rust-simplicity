// SPDX-License-Identifier: CC0-1.0

use hashes::{sha256, Hash};

use crate::ffi::sha256::CSha256Midstate;
use crate::ffi::{c_size_t, c_uchar, c_uint, c_uint_fast32_t};

/// Documentation of RawInputData/RawOutputData/RawTapData/Raw.
///
/// Data structure for holding data that CTransaction points to.
///
/// Why do we this special data structure?
///     1. We need to keep the data in memory until the CTransaction is dropped.
///     2. The memory is Transaction is not saved in the same format as required by FFI.
/// We use more ergonomics in rust to allow better UX which interfere with the FFI. For example,
/// the Value is stored as Tagged Union, but we require it to be a slice of bytes in elements format.
///     3. Allocating inside FFI functions does not work because the memory is freed after the function returns.
///     4. We only create allocations for data fields that are stored differently from the
/// consensus serialization format.
#[derive(Debug)]
pub struct RawOutputData {
    pub asset: Vec<c_uchar>,
    pub value: Vec<c_uchar>,
    pub nonce: Vec<c_uchar>,
    pub surjection_proof: Vec<c_uchar>,
    pub range_proof: Vec<c_uchar>,
}

#[derive(Debug)]
#[repr(C)]
pub struct CElementsRawBuffer {
    pub ptr: *const c_uchar,
    pub len: u32,
}

/// Similar to [`RawOutputData`], for inputs.
#[derive(Debug)]
pub struct RawInputData {
    pub annex: Option<Vec<c_uchar>>,
    // issuance
    pub issuance_amount: Vec<c_uchar>,
    pub issuance_inflation_keys: Vec<c_uchar>,
    pub amount_range_proof: Vec<c_uchar>,
    pub inflation_keys_range_proof: Vec<c_uchar>,
    // spent txo
    pub asset: Vec<c_uchar>,
    pub value: Vec<c_uchar>,
}

/// Similar to [`RawOutputData`], but for transaction
#[derive(Debug)]
pub struct RawTransactionData {
    pub inputs: Vec<RawInputData>,
    pub outputs: Vec<RawOutputData>,
}

#[derive(Debug)]
#[repr(C)]
pub struct CElementsRawOutput {
    asset: *const c_uchar,
    value: *const c_uchar,
    nonce: *const c_uchar,
    script_pubkey: CElementsRawBuffer,
    surjection_proof: CElementsRawBuffer,
    range_proof: CElementsRawBuffer,
}

#[derive(Debug)]
#[repr(C)]
pub struct CElementsRawInput {
    annex: *const CElementsRawBuffer,
    prev_txid: *const c_uchar,
    pegin: *const c_uchar,
    // issuance
    blinding_nonce: *const c_uchar,
    asset_entropy: *const c_uchar,
    amount: *const c_uchar,
    inflation_keys: *const c_uchar,
    amount_range_proof: CElementsRawBuffer,
    inflation_keys_range_proof: CElementsRawBuffer,
    // spent txo
    asset: *const c_uchar,
    value: *const c_uchar,
    script_pubkey: CElementsRawBuffer,
    // inputs
    script_sig: CElementsRawBuffer,
    prev_txout_index: u32,
    sequence: u32,
}

#[derive(Debug)]
#[repr(C)]
pub struct CElementsRawTransaction {
    txid: *const c_uchar,
    inputs: *const CElementsRawInput,
    outputs: *const CElementsRawOutput,
    version: u32,
    locktime: u32,
    n_inputs: u32,
    n_outputs: u32,
}

#[derive(Debug)]
#[repr(C)]
pub struct CElementsRawTapEnv {
    control_block: *const c_uchar,
    script_cmr: *const c_uchar,
    branch_len: u8,
}

#[derive(Debug)]
pub enum CTransaction {}

#[derive(Debug)]
#[repr(C)]
pub struct CElementsTxEnv {
    tx: *const CTransaction,
    taproot: *const CTapEnv,
    genesis_hash: CSha256Midstate,
    sighash_all: CSha256Midstate,
    ix: c_uint_fast32_t,
}

#[derive(Debug)]
pub enum CTapEnv {}

extern "C" {
    #[link_name = "rustsimplicity_0_5_c_sizeof_rawElementsBuffer"]
    pub static c_sizeof_rawElementsBuffer: c_size_t;
    #[link_name = "rustsimplicity_0_5_c_sizeof_rawElementsOutput"]
    pub static c_sizeof_rawElementsOutput: c_size_t;
    #[link_name = "rustsimplicity_0_5_c_sizeof_rawElementsInput"]
    pub static c_sizeof_rawElementsInput: c_size_t;
    #[link_name = "rustsimplicity_0_5_c_sizeof_rawElementsTransaction"]
    pub static c_sizeof_rawElementsTransaction: c_size_t;
    #[link_name = "rustsimplicity_0_5_c_sizeof_rawElementsTapEnv"]
    pub static c_sizeof_rawElementsTapEnv: c_size_t;
    #[link_name = "rustsimplicity_0_5_c_sizeof_txEnv"]
    pub static c_sizeof_txEnv: c_size_t;

    #[link_name = "rustsimplicity_0_5_c_alignof_rawElementsBuffer"]
    pub static c_alignof_rawElementsBuffer: c_size_t;
    #[link_name = "rustsimplicity_0_5_c_alignof_rawElementsOutput"]
    pub static c_alignof_rawElementsOutput: c_size_t;
    #[link_name = "rustsimplicity_0_5_c_alignof_rawElementsInput"]
    pub static c_alignof_rawElementsInput: c_size_t;
    #[link_name = "rustsimplicity_0_5_c_alignof_rawElementsTransaction"]
    pub static c_alignof_rawElementsTransaction: c_size_t;
    #[link_name = "rustsimplicity_0_5_c_alignof_rawElementsTapEnv"]
    pub static c_alignof_rawElementsTapEnv: c_size_t;
    #[link_name = "rustsimplicity_0_5_c_alignof_txEnv"]
    pub static c_alignof_txEnv: c_size_t;

    #[link_name = "rustsimplicity_0_5_c_set_rawElementsBuffer"]
    pub fn c_set_rawElementsBuffer(res: *mut CElementsRawBuffer, buf: *const c_uchar, len: c_uint);
    #[link_name = "rustsimplicity_0_5_c_set_rawElementsOutput"]
    pub fn c_set_rawElementsOutput(
        res: *mut CElementsRawOutput,
        asset: *const c_uchar,
        value: *const c_uchar,
        nonce: *const c_uchar,
        scriptPubKey: *const CElementsRawBuffer,
        surjectionProof: *const CElementsRawBuffer,
        rangeProof: *const CElementsRawBuffer,
    );
    #[link_name = "rustsimplicity_0_5_c_set_rawElementsInput"]
    pub fn c_set_rawElementsInput(
        result: *mut CElementsRawInput,
        annex: *const CElementsRawBuffer,
        pegin: *const c_uchar,
        scriptSig: *const CElementsRawBuffer,
        prevTxid: *const c_uchar,
        prevIx: c_uint,
        asset: *const c_uchar,
        value: *const c_uchar,
        scriptPubKey: *const CElementsRawBuffer,
        sequence: c_uint,
        blindingNonce: *const c_uchar,
        assetEntropy: *const c_uchar,
        amount: *const c_uchar,
        inflationKeys: *const c_uchar,
        amountRangePrf: *const CElementsRawBuffer,
        inflationKeysRangePrf: *const CElementsRawBuffer,
    );

    #[link_name = "rustsimplicity_0_5_c_set_rawElementsTransaction"]
    pub fn c_set_rawElementsTransaction(
        result: *mut CElementsRawTransaction,
        version: c_uint,
        txid: *const c_uchar,
        input: *const CElementsRawInput,
        numInputs: c_uint,
        output: *const CElementsRawOutput,
        numOutputs: c_uint,
        lockTime: c_uint,
    );
    #[link_name = "rustsimplicity_0_5_c_set_rawElementsTapEnv"]
    pub fn c_set_rawElementsTapEnv(
        result: *mut CElementsRawTapEnv,
        controlBlock: *const c_uchar,
        pathLen: c_uchar,
        scriptCMR: *const c_uchar,
    );
    #[link_name = "rustsimplicity_0_5_c_set_txEnv"]
    pub fn c_set_txEnv(
        result: *mut CElementsTxEnv,
        tx: *const CTransaction,
        taproot: *const CTapEnv,
        genesisHash: *const c_uchar,
        ix: c_uint,
    );
    #[link_name = "rustsimplicity_0_5_elements_mallocTapEnv"]
    pub fn simplicity_elements_mallocTapEnv(rawEnv: *const CElementsRawTapEnv) -> *mut CTapEnv;
    #[link_name = "rustsimplicity_0_5_elements_mallocTransaction"]
    pub fn simplicity_elements_mallocTransaction(
        rawTx: *const CElementsRawTransaction,
    ) -> *mut CTransaction;
    #[link_name = "rustsimplicity_0_5_c_free_transaction"]
    pub fn c_free_transaction(tx: *mut CTransaction);
    #[link_name = "rustsimplicity_0_5_c_free_tapEnv"]
    pub fn c_free_tapEnv(env: *mut CTapEnv);
}
impl CElementsTxEnv {
    pub fn sighash_all(&self) -> sha256::Hash {
        let midstate: sha256::Midstate = self.sighash_all.into();
        sha256::Hash::from_byte_array(midstate.to_byte_array())
    }
}

// Pointer must be manually free after dropping
impl Drop for CElementsTxEnv {
    fn drop(&mut self) {
        unsafe {
            crate::alloc::rust_0_5_free(self.tx as *mut u8);
            crate::alloc::rust_0_5_free(self.taproot as *mut u8);
        }
    }
}

impl CElementsRawBuffer {
    pub fn new(buf: &[c_uchar]) -> Self {
        unsafe {
            let mut raw_buffer = std::mem::MaybeUninit::<Self>::uninit();
            c_set_rawElementsBuffer(raw_buffer.as_mut_ptr(), buf.as_ptr(), buf.len() as c_uint);
            raw_buffer.assume_init()
        }
    }
}

#[cfg(test)]
mod tests {
    use std::mem::{align_of, size_of};

    use crate::c_jets::{c_env::*, frame_ffi::*};

    #[test]
    fn test_sizes() {
        unsafe {
            assert_eq!(size_of::<CFrameItem>(), c_sizeof_frameItem);
            assert_eq!(size_of::<CElementsRawBuffer>(), c_sizeof_rawElementsBuffer);
            assert_eq!(size_of::<CElementsRawInput>(), c_sizeof_rawElementsInput);
            assert_eq!(size_of::<CElementsRawOutput>(), c_sizeof_rawElementsOutput);
            assert_eq!(
                size_of::<CElementsRawTransaction>(),
                c_sizeof_rawElementsTransaction
            );
            assert_eq!(size_of::<CElementsRawTapEnv>(), c_sizeof_rawElementsTapEnv);
            assert_eq!(size_of::<CElementsTxEnv>(), c_sizeof_txEnv);
        }
    }

    #[test]
    fn test_aligns() {
        unsafe {
            assert_eq!(align_of::<CFrameItem>(), c_alignof_frameItem);
            assert_eq!(
                align_of::<CElementsRawBuffer>(),
                c_alignof_rawElementsBuffer
            );
            assert_eq!(align_of::<CElementsRawInput>(), c_alignof_rawElementsInput);
            assert_eq!(
                align_of::<CElementsRawOutput>(),
                c_alignof_rawElementsOutput
            );
            assert_eq!(
                align_of::<CElementsRawTransaction>(),
                c_alignof_rawElementsTransaction
            );
            assert_eq!(
                align_of::<CElementsRawTapEnv>(),
                c_alignof_rawElementsTapEnv
            );
            assert_eq!(align_of::<CElementsTxEnv>(), c_alignof_txEnv);
        }
    }
}
