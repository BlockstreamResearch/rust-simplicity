use std::os::raw::{c_uchar, c_uint};

use crate::util;
use bitcoin_hashes::{sha256, Hash};
use libc::size_t;

/// Documentation of CRawInputData/CRawOutputData/CRawTapData/CRaw
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

/// Similar to [`CRawOutputData`], for inputs.
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

/// Similar to [`CRawOutputData`], but for transaction
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
pub struct CSha256 {
    pub data: [u32; 8],
}

#[derive(Debug)]
#[repr(C)]
pub struct CElementsTxEnv {
    tx: *const CTransaction,
    taproot: *const CTapEnv,
    genesis_hash: CSha256,
    sighash_all: CSha256,
    ix: usize, // This is uint_fast32_t in C, which is usize for 32/64 bits
}

#[derive(Debug)]
pub enum CTapEnv {}

extern "C" {

    pub static c_sizeof_rawBuffer: size_t;
    pub static c_sizeof_rawOutput: size_t;
    pub static c_sizeof_rawInput: size_t;
    pub static c_sizeof_rawTransaction: size_t;
    pub static c_sizeof_rawTapEnv: size_t;
    pub static c_sizeof_txEnv: size_t;

    pub static c_alignof_rawBuffer: size_t;
    pub static c_alignof_rawOutput: size_t;
    pub static c_alignof_rawInput: size_t;
    pub static c_alignof_rawTransaction: size_t;
    pub static c_alignof_rawTapEnv: size_t;
    pub static c_alignof_txEnv: size_t;

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
        input: *const CRawInput,
        numInputs: c_uint,
        output: *const CRawOutput,
        numOutputs: c_uint,
        lockTime: c_uint,
    );
    pub fn c_set_rawTapEnv(
        result: *mut CRawTapEnv,
        controlBlock: *const c_uchar,
        branchLen: c_uchar,
        scriptCMR: *const c_uchar,
    );
    pub fn c_set_txEnv(
        result: *mut CElementsTxEnv,
        tx: *const CTransaction,
        taproot: *const CTapEnv,
        genesisHash: *const c_uchar,
        ix: c_uint,
    );
    pub fn elements_simplicity_mallocTapEnv(rawEnv: *const CRawTapEnv) -> *mut CTapEnv;
    pub fn elements_simplicity_mallocTransaction(
        rawTx: *const CRawTransaction,
    ) -> *mut CTransaction;
    pub fn c_free_transaction(tx: *mut CTransaction);
    pub fn c_free_tapEnv(env: *mut CTapEnv);
}
impl CElementsTxEnv {
    pub fn sighash_all(&self) -> sha256::Hash {
        let bytes = util::into_u8_merkle_root(&self.sighash_all.data);
        sha256::Hash::from_inner(bytes)
    }
}

// Pointer must be manually free after dropping
impl Drop for CElementsTxEnv {
    fn drop(&mut self) {
        unsafe {
            c_free_transaction(self.tx as *mut CTransaction);
            c_free_tapEnv(self.taproot as *mut CTapEnv);
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
            assert_eq!(size_of::<usize>(), c_sizeof_UWORD);
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
            assert_eq!(align_of::<usize>(), c_alignof_UWORD);
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
