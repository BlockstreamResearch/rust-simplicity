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
pub struct CRawBuffer {
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
pub struct CRawOutput {
    asset: *const c_uchar,
    value: *const c_uchar,
    nonce: *const c_uchar,
    script_pubkey: CRawBuffer,
    surjection_proof: CRawBuffer,
    range_proof: CRawBuffer,
}

#[derive(Debug)]
#[repr(C)]
pub struct CRawInput {
    annex: *const CRawBuffer,
    prev_txid: *const c_uchar,
    pegin: *const c_uchar,
    // issuance
    blinding_nonce: *const c_uchar,
    asset_entropy: *const c_uchar,
    amount: *const c_uchar,
    inflation_keys: *const c_uchar,
    amount_range_proof: CRawBuffer,
    inflation_keys_range_proof: CRawBuffer,
    // spent txo
    asset: *const c_uchar,
    value: *const c_uchar,
    script_pubkey: CRawBuffer,
    // inputs
    script_sig: CRawBuffer,
    prev_txout_index: u32,
    sequence: u32,
}

#[derive(Debug)]
#[repr(C)]
pub struct CRawTransaction {
    txid: *const c_uchar,
    inputs: *const CRawInput,
    outputs: *const CRawOutput,
    version: u32,
    locktime: u32,
    n_inputs: u32,
    n_outputs: u32,
}

#[derive(Debug)]
#[repr(C)]
pub struct CRawTapEnv {
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

    pub static c_sizeof_rawBuffer: c_size_t;
    pub static c_sizeof_rawOutput: c_size_t;
    pub static c_sizeof_rawInput: c_size_t;
    pub static c_sizeof_rawTransaction: c_size_t;
    pub static c_sizeof_rawTapEnv: c_size_t;
    pub static c_sizeof_txEnv: c_size_t;

    pub static c_alignof_rawBuffer: c_size_t;
    pub static c_alignof_rawOutput: c_size_t;
    pub static c_alignof_rawInput: c_size_t;
    pub static c_alignof_rawTransaction: c_size_t;
    pub static c_alignof_rawTapEnv: c_size_t;
    pub static c_alignof_txEnv: c_size_t;

    pub fn c_set_rawBuffer(res: *mut CRawBuffer, buf: *const c_uchar, len: c_uint);
    pub fn c_set_rawOutput(
        res: *mut CRawOutput,
        asset: *const c_uchar,
        value: *const c_uchar,
        nonce: *const c_uchar,
        scriptPubKey: *const CRawBuffer,
        surjectionProof: *const CRawBuffer,
        rangeProof: *const CRawBuffer,
    );
    pub fn c_set_rawInput(
        result: *mut CRawInput,
        annex: *const CRawBuffer,
        pegin: *const c_uchar,
        scriptSig: *const CRawBuffer,
        prevTxid: *const c_uchar,
        prevIx: c_uint,
        asset: *const c_uchar,
        value: *const c_uchar,
        scriptPubKey: *const CRawBuffer,
        sequence: c_uint,
        blindingNonce: *const c_uchar,
        assetEntropy: *const c_uchar,
        amount: *const c_uchar,
        inflationKeys: *const c_uchar,
        amountRangePrf: *const CRawBuffer,
        inflationKeysRangePrf: *const CRawBuffer,
    );

    pub fn c_set_rawTransaction(
        result: *mut CRawTransaction,
        version: c_uint,
        txid: *const c_uchar,
        input: *const CRawInput,
        numInputs: c_uint,
        output: *const CRawOutput,
        numOutputs: c_uint,
        lockTime: c_uint,
    );
    pub fn c_set_rawTapEnv(
        result: *mut CRawTapEnv,
        controlBlock: *const c_uchar,
        pathLen: c_uchar,
        scriptCMR: *const c_uchar,
    );
    pub fn c_set_txEnv(
        result: *mut CElementsTxEnv,
        tx: *const CTransaction,
        taproot: *const CTapEnv,
        genesisHash: *const c_uchar,
        ix: c_uint,
    );
    pub fn simplicity_elements_mallocTapEnv(rawEnv: *const CRawTapEnv) -> *mut CTapEnv;
    pub fn simplicity_elements_mallocTransaction(
        rawTx: *const CRawTransaction,
    ) -> *mut CTransaction;
    pub fn c_free_transaction(tx: *mut CTransaction);
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
            crate::alloc::rust_0_4_free(self.tx as *mut u8);
            crate::alloc::rust_0_4_free(self.taproot as *mut u8);
        }
    }
}

impl CRawBuffer {
    pub fn new(buf: &[c_uchar]) -> Self {
        unsafe {
            let mut raw_buffer = std::mem::MaybeUninit::<CRawBuffer>::uninit();
            c_set_rawBuffer(raw_buffer.as_mut_ptr(), buf.as_ptr(), buf.len() as c_uint);
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
            assert_eq!(size_of::<CRawBuffer>(), c_sizeof_rawBuffer);
            assert_eq!(size_of::<CRawInput>(), c_sizeof_rawInput);
            assert_eq!(size_of::<CRawOutput>(), c_sizeof_rawOutput);
            assert_eq!(size_of::<CRawTransaction>(), c_sizeof_rawTransaction);
            assert_eq!(size_of::<CRawTapEnv>(), c_sizeof_rawTapEnv);
            assert_eq!(size_of::<CElementsTxEnv>(), c_sizeof_txEnv);
        }
    }

    #[test]
    fn test_aligns() {
        unsafe {
            assert_eq!(align_of::<CFrameItem>(), c_alignof_frameItem);
            assert_eq!(align_of::<CRawBuffer>(), c_alignof_rawBuffer);
            assert_eq!(align_of::<CRawInput>(), c_alignof_rawInput);
            assert_eq!(align_of::<CRawOutput>(), c_alignof_rawOutput);
            assert_eq!(align_of::<CRawTransaction>(), c_alignof_rawTransaction);
            assert_eq!(align_of::<CRawTapEnv>(), c_alignof_rawTapEnv);
            assert_eq!(align_of::<CElementsTxEnv>(), c_alignof_txEnv);
        }
    }
}
