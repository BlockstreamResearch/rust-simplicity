// SPDX-License-Identifier: CC0-1.0

use hashes::sha256;

use crate::c_jets::frame_ffi::CFrameItem;
use crate::ffi::sha256::CSha256Midstate;
use crate::ffi::{c_size_t, c_uchar, c_uint, c_uint_fast32_t};

#[derive(Debug)]
#[repr(C)]
pub struct CRawBuffer {
    pub ptr: *const c_uchar,
    pub len: u32,
}

#[derive(Debug)]
#[repr(C)]
pub struct CRawOutput {
    pub value: u64,
    pub script_pubkey: CRawBuffer,
}

#[repr(C)]
pub struct CRawInput<'raw> {
    pub annex: *const CRawBuffer,
    pub prev_txid: &'raw [c_uchar; 32],
    pub txo: CRawOutput,
    pub script_sig: CRawBuffer,
    pub prev_txout_index: u32,
    pub sequence: u32,
}

#[derive(Debug)]
#[repr(C)]
pub struct CRawTransaction<'raw> {
    pub txid: &'raw [c_uchar; 32],
    pub inputs: *const CRawInput<'raw>,
    pub outputs: *const CRawOutput,
    pub n_inputs: u32,
    pub n_outputs: u32,
    pub version: u32,
    pub locktime: u32,
}

#[derive(Debug)]
#[repr(C)]
pub struct CRawTapEnv {
    pub control_block: *const c_uchar,
    pub script_cmr: *const c_uchar,
    pub branch_len: u8,
}

#[repr(C)]
pub struct CTransaction {
    _data: (),
}

#[derive(Debug)]
#[repr(C)]
pub struct CTxEnv {
    tx: *const CTransaction,
    taproot: *const CTapEnv,
    sighash_all: CSha256Midstate,
    ix: c_uint_fast32_t,
}

#[repr(C)]
pub struct CTapEnv {
    _data: (),
}

// Will uncomment in a later commit; need to update libsimplicity first so these
// symbols have something to link against.
extern "C" {
    #[link_name = "rustsimplicity_0_8_c_sizeof_rawBitcoinBuffer"]
    pub static c_sizeof_rawBuffer: c_size_t;
    #[link_name = "rustsimplicity_0_8_c_sizeof_rawBitcoinOutput"]
    pub static c_sizeof_rawOutput: c_size_t;
    #[link_name = "rustsimplicity_0_8_c_sizeof_rawBitcoinInput"]
    pub static c_sizeof_rawInput: c_size_t;
    #[link_name = "rustsimplicity_0_8_c_sizeof_rawBitcoinTransaction"]
    pub static c_sizeof_rawTransaction: c_size_t;
    #[link_name = "rustsimplicity_0_8_c_sizeof_rawBitcoinTapEnv"]
    pub static c_sizeof_rawTapEnv: c_size_t;
    #[link_name = "rustsimplicity_0_8_c_sizeof_bitcoinTxEnv"]
    pub static c_sizeof_txEnv: c_size_t;

    #[link_name = "rustsimplicity_0_8_c_alignof_rawBitcoinBuffer"]
    pub static c_alignof_rawBuffer: c_size_t;
    #[link_name = "rustsimplicity_0_8_c_alignof_rawBitcoinOutput"]
    pub static c_alignof_rawOutput: c_size_t;
    #[link_name = "rustsimplicity_0_8_c_alignof_rawBitcoinInput"]
    pub static c_alignof_rawInput: c_size_t;
    #[link_name = "rustsimplicity_0_8_c_alignof_rawBitcoinTransaction"]
    pub static c_alignof_rawTransaction: c_size_t;
    #[link_name = "rustsimplicity_0_8_c_alignof_rawBitcoinTapEnv"]
    pub static c_alignof_rawTapEnv: c_size_t;
    #[link_name = "rustsimplicity_0_8_c_alignof_bitcoinTxEnv"]
    pub static c_alignof_txEnv: c_size_t;

    #[link_name = "rustsimplicity_0_8_c_bitcoin_set_txEnv"]
    pub fn c_set_txEnv(
        result: *mut CTxEnv,
        tx: *const CTransaction,
        taproot: *const CTapEnv,
        ix: c_uint,
    );
    #[link_name = "rustsimplicity_0_8_bitcoin_mallocTapEnv"]
    pub fn simplicity_mallocTapEnv(rawEnv: *const CRawTapEnv) -> *mut CTapEnv;
    #[link_name = "rustsimplicity_0_8_bitcoin_mallocTransaction"]
    pub fn simplicity_mallocTransaction(rawTx: *const CRawTransaction) -> *mut CTransaction;

    // Bitcoin jets whose C implementations read the transaction environment.
    // Each has signature `bool f(frameItem* dst, frameItem src, const txEnv* env)`
    // (note `src` is passed *by value*, matching the C definitions in bitcoinJets.c).
    #[link_name = "rustsimplicity_0_8_bitcoin_sig_all_hash"]
    pub fn bitcoin_sig_all_hash(dst: *mut CFrameItem, src: CFrameItem, env: *const CTxEnv) -> bool;
    #[link_name = "rustsimplicity_0_8_bitcoin_check_lock_height"]
    pub fn bitcoin_check_lock_height(
        dst: *mut CFrameItem,
        src: CFrameItem,
        env: *const CTxEnv,
    ) -> bool;
    #[link_name = "rustsimplicity_0_8_bitcoin_check_lock_distance"]
    pub fn bitcoin_check_lock_distance(
        dst: *mut CFrameItem,
        src: CFrameItem,
        env: *const CTxEnv,
    ) -> bool;
    #[link_name = "rustsimplicity_0_8_bitcoin_check_lock_duration"]
    pub fn bitcoin_check_lock_duration(
        dst: *mut CFrameItem,
        src: CFrameItem,
        env: *const CTxEnv,
    ) -> bool;
    #[link_name = "rustsimplicity_0_8_bitcoin_current_index"]
    pub fn bitcoin_current_index(dst: *mut CFrameItem, src: CFrameItem, env: *const CTxEnv)
        -> bool;
    #[link_name = "rustsimplicity_0_8_bitcoin_num_inputs"]
    pub fn bitcoin_num_inputs(dst: *mut CFrameItem, src: CFrameItem, env: *const CTxEnv) -> bool;
    #[link_name = "rustsimplicity_0_8_bitcoin_tx_hash"]
    pub fn bitcoin_tx_hash(dst: *mut CFrameItem, src: CFrameItem, env: *const CTxEnv) -> bool;
    #[link_name = "rustsimplicity_0_8_bitcoin_tap_env_hash"]
    pub fn bitcoin_tap_env_hash(dst: *mut CFrameItem, src: CFrameItem, env: *const CTxEnv) -> bool;
    #[link_name = "rustsimplicity_0_8_bitcoin_annex_hash"]
    pub fn bitcoin_annex_hash(dst: *mut CFrameItem, src: CFrameItem, env: *const CTxEnv) -> bool;
    #[link_name = "rustsimplicity_0_8_bitcoin_build_tapbranch"]
    pub fn bitcoin_build_tapbranch(
        dst: *mut CFrameItem,
        src: CFrameItem,
        env: *const CTxEnv,
    ) -> bool;
    #[link_name = "rustsimplicity_0_8_bitcoin_build_tapleaf_simplicity"]
    pub fn bitcoin_build_tapleaf_simplicity(
        dst: *mut CFrameItem,
        src: CFrameItem,
        env: *const CTxEnv,
    ) -> bool;
    #[link_name = "rustsimplicity_0_8_bitcoin_build_taptweak"]
    pub fn bitcoin_build_taptweak(
        dst: *mut CFrameItem,
        src: CFrameItem,
        env: *const CTxEnv,
    ) -> bool;
    #[link_name = "rustsimplicity_0_8_bitcoin_check_lock_time"]
    pub fn bitcoin_check_lock_time(
        dst: *mut CFrameItem,
        src: CFrameItem,
        env: *const CTxEnv,
    ) -> bool;
    #[link_name = "rustsimplicity_0_8_bitcoin_current_annex_hash"]
    pub fn bitcoin_current_annex_hash(
        dst: *mut CFrameItem,
        src: CFrameItem,
        env: *const CTxEnv,
    ) -> bool;
    #[link_name = "rustsimplicity_0_8_bitcoin_current_prev_outpoint"]
    pub fn bitcoin_current_prev_outpoint(
        dst: *mut CFrameItem,
        src: CFrameItem,
        env: *const CTxEnv,
    ) -> bool;
    #[link_name = "rustsimplicity_0_8_bitcoin_current_script_hash"]
    pub fn bitcoin_current_script_hash(
        dst: *mut CFrameItem,
        src: CFrameItem,
        env: *const CTxEnv,
    ) -> bool;
    #[link_name = "rustsimplicity_0_8_bitcoin_current_script_sig_hash"]
    pub fn bitcoin_current_script_sig_hash(
        dst: *mut CFrameItem,
        src: CFrameItem,
        env: *const CTxEnv,
    ) -> bool;
    #[link_name = "rustsimplicity_0_8_bitcoin_current_sequence"]
    pub fn bitcoin_current_sequence(
        dst: *mut CFrameItem,
        src: CFrameItem,
        env: *const CTxEnv,
    ) -> bool;
    #[link_name = "rustsimplicity_0_8_bitcoin_current_value"]
    pub fn bitcoin_current_value(dst: *mut CFrameItem, src: CFrameItem, env: *const CTxEnv)
        -> bool;
    #[link_name = "rustsimplicity_0_8_bitcoin_fee"]
    pub fn bitcoin_fee(dst: *mut CFrameItem, src: CFrameItem, env: *const CTxEnv) -> bool;
    #[link_name = "rustsimplicity_0_8_bitcoin_input_annex_hash"]
    pub fn bitcoin_input_annex_hash(
        dst: *mut CFrameItem,
        src: CFrameItem,
        env: *const CTxEnv,
    ) -> bool;
    #[link_name = "rustsimplicity_0_8_bitcoin_input_annexes_hash"]
    pub fn bitcoin_input_annexes_hash(
        dst: *mut CFrameItem,
        src: CFrameItem,
        env: *const CTxEnv,
    ) -> bool;
    #[link_name = "rustsimplicity_0_8_bitcoin_input_hash"]
    pub fn bitcoin_input_hash(dst: *mut CFrameItem, src: CFrameItem, env: *const CTxEnv) -> bool;
    #[link_name = "rustsimplicity_0_8_bitcoin_input_outpoints_hash"]
    pub fn bitcoin_input_outpoints_hash(
        dst: *mut CFrameItem,
        src: CFrameItem,
        env: *const CTxEnv,
    ) -> bool;
    #[link_name = "rustsimplicity_0_8_bitcoin_input_prev_outpoint"]
    pub fn bitcoin_input_prev_outpoint(
        dst: *mut CFrameItem,
        src: CFrameItem,
        env: *const CTxEnv,
    ) -> bool;
    #[link_name = "rustsimplicity_0_8_bitcoin_input_script_hash"]
    pub fn bitcoin_input_script_hash(
        dst: *mut CFrameItem,
        src: CFrameItem,
        env: *const CTxEnv,
    ) -> bool;
    #[link_name = "rustsimplicity_0_8_bitcoin_input_script_sig_hash"]
    pub fn bitcoin_input_script_sig_hash(
        dst: *mut CFrameItem,
        src: CFrameItem,
        env: *const CTxEnv,
    ) -> bool;
    #[link_name = "rustsimplicity_0_8_bitcoin_input_script_sigs_hash"]
    pub fn bitcoin_input_script_sigs_hash(
        dst: *mut CFrameItem,
        src: CFrameItem,
        env: *const CTxEnv,
    ) -> bool;
    #[link_name = "rustsimplicity_0_8_bitcoin_input_scripts_hash"]
    pub fn bitcoin_input_scripts_hash(
        dst: *mut CFrameItem,
        src: CFrameItem,
        env: *const CTxEnv,
    ) -> bool;
    #[link_name = "rustsimplicity_0_8_bitcoin_input_sequence"]
    pub fn bitcoin_input_sequence(
        dst: *mut CFrameItem,
        src: CFrameItem,
        env: *const CTxEnv,
    ) -> bool;
    #[link_name = "rustsimplicity_0_8_bitcoin_input_sequences_hash"]
    pub fn bitcoin_input_sequences_hash(
        dst: *mut CFrameItem,
        src: CFrameItem,
        env: *const CTxEnv,
    ) -> bool;
    #[link_name = "rustsimplicity_0_8_bitcoin_input_utxo_hash"]
    pub fn bitcoin_input_utxo_hash(
        dst: *mut CFrameItem,
        src: CFrameItem,
        env: *const CTxEnv,
    ) -> bool;
    #[link_name = "rustsimplicity_0_8_bitcoin_input_utxos_hash"]
    pub fn bitcoin_input_utxos_hash(
        dst: *mut CFrameItem,
        src: CFrameItem,
        env: *const CTxEnv,
    ) -> bool;
    #[link_name = "rustsimplicity_0_8_bitcoin_input_value"]
    pub fn bitcoin_input_value(dst: *mut CFrameItem, src: CFrameItem, env: *const CTxEnv) -> bool;
    #[link_name = "rustsimplicity_0_8_bitcoin_input_values_hash"]
    pub fn bitcoin_input_values_hash(
        dst: *mut CFrameItem,
        src: CFrameItem,
        env: *const CTxEnv,
    ) -> bool;
    #[link_name = "rustsimplicity_0_8_bitcoin_inputs_hash"]
    pub fn bitcoin_inputs_hash(dst: *mut CFrameItem, src: CFrameItem, env: *const CTxEnv) -> bool;
    #[link_name = "rustsimplicity_0_8_bitcoin_internal_key"]
    pub fn bitcoin_internal_key(dst: *mut CFrameItem, src: CFrameItem, env: *const CTxEnv) -> bool;
    #[link_name = "rustsimplicity_0_8_bitcoin_lock_time"]
    pub fn bitcoin_lock_time(dst: *mut CFrameItem, src: CFrameItem, env: *const CTxEnv) -> bool;
    #[link_name = "rustsimplicity_0_8_bitcoin_num_outputs"]
    pub fn bitcoin_num_outputs(dst: *mut CFrameItem, src: CFrameItem, env: *const CTxEnv) -> bool;
    #[link_name = "rustsimplicity_0_8_bitcoin_outpoint_hash"]
    pub fn bitcoin_outpoint_hash(dst: *mut CFrameItem, src: CFrameItem, env: *const CTxEnv)
        -> bool;
    #[link_name = "rustsimplicity_0_8_bitcoin_output_hash"]
    pub fn bitcoin_output_hash(dst: *mut CFrameItem, src: CFrameItem, env: *const CTxEnv) -> bool;
    #[link_name = "rustsimplicity_0_8_bitcoin_output_script_hash"]
    pub fn bitcoin_output_script_hash(
        dst: *mut CFrameItem,
        src: CFrameItem,
        env: *const CTxEnv,
    ) -> bool;
    #[link_name = "rustsimplicity_0_8_bitcoin_output_scripts_hash"]
    pub fn bitcoin_output_scripts_hash(
        dst: *mut CFrameItem,
        src: CFrameItem,
        env: *const CTxEnv,
    ) -> bool;
    #[link_name = "rustsimplicity_0_8_bitcoin_output_value"]
    pub fn bitcoin_output_value(dst: *mut CFrameItem, src: CFrameItem, env: *const CTxEnv) -> bool;
    #[link_name = "rustsimplicity_0_8_bitcoin_output_values_hash"]
    pub fn bitcoin_output_values_hash(
        dst: *mut CFrameItem,
        src: CFrameItem,
        env: *const CTxEnv,
    ) -> bool;
    #[link_name = "rustsimplicity_0_8_bitcoin_outputs_hash"]
    pub fn bitcoin_outputs_hash(dst: *mut CFrameItem, src: CFrameItem, env: *const CTxEnv) -> bool;
    #[link_name = "rustsimplicity_0_8_bitcoin_script_cmr"]
    pub fn bitcoin_script_cmr(dst: *mut CFrameItem, src: CFrameItem, env: *const CTxEnv) -> bool;
    #[link_name = "rustsimplicity_0_8_bitcoin_tapleaf_hash"]
    pub fn bitcoin_tapleaf_hash(dst: *mut CFrameItem, src: CFrameItem, env: *const CTxEnv) -> bool;
    #[link_name = "rustsimplicity_0_8_bitcoin_tapleaf_version"]
    pub fn bitcoin_tapleaf_version(
        dst: *mut CFrameItem,
        src: CFrameItem,
        env: *const CTxEnv,
    ) -> bool;
    #[link_name = "rustsimplicity_0_8_bitcoin_tappath"]
    pub fn bitcoin_tappath(dst: *mut CFrameItem, src: CFrameItem, env: *const CTxEnv) -> bool;
    #[link_name = "rustsimplicity_0_8_bitcoin_tappath_hash"]
    pub fn bitcoin_tappath_hash(dst: *mut CFrameItem, src: CFrameItem, env: *const CTxEnv) -> bool;
    #[link_name = "rustsimplicity_0_8_bitcoin_total_input_value"]
    pub fn bitcoin_total_input_value(
        dst: *mut CFrameItem,
        src: CFrameItem,
        env: *const CTxEnv,
    ) -> bool;
    #[link_name = "rustsimplicity_0_8_bitcoin_total_output_value"]
    pub fn bitcoin_total_output_value(
        dst: *mut CFrameItem,
        src: CFrameItem,
        env: *const CTxEnv,
    ) -> bool;
    #[link_name = "rustsimplicity_0_8_bitcoin_transaction_id"]
    pub fn bitcoin_transaction_id(
        dst: *mut CFrameItem,
        src: CFrameItem,
        env: *const CTxEnv,
    ) -> bool;
    #[link_name = "rustsimplicity_0_8_bitcoin_tx_is_final"]
    pub fn bitcoin_tx_is_final(dst: *mut CFrameItem, src: CFrameItem, env: *const CTxEnv) -> bool;
    #[link_name = "rustsimplicity_0_8_bitcoin_tx_lock_distance"]
    pub fn bitcoin_tx_lock_distance(
        dst: *mut CFrameItem,
        src: CFrameItem,
        env: *const CTxEnv,
    ) -> bool;
    #[link_name = "rustsimplicity_0_8_bitcoin_tx_lock_duration"]
    pub fn bitcoin_tx_lock_duration(
        dst: *mut CFrameItem,
        src: CFrameItem,
        env: *const CTxEnv,
    ) -> bool;
    #[link_name = "rustsimplicity_0_8_bitcoin_tx_lock_height"]
    pub fn bitcoin_tx_lock_height(
        dst: *mut CFrameItem,
        src: CFrameItem,
        env: *const CTxEnv,
    ) -> bool;
    #[link_name = "rustsimplicity_0_8_bitcoin_tx_lock_time"]
    pub fn bitcoin_tx_lock_time(dst: *mut CFrameItem, src: CFrameItem, env: *const CTxEnv) -> bool;
    #[link_name = "rustsimplicity_0_8_bitcoin_version"]
    pub fn bitcoin_version(dst: *mut CFrameItem, src: CFrameItem, env: *const CTxEnv) -> bool;
}

impl CTxEnv {
    pub fn sighash_all(&self) -> sha256::Hash {
        let midstate: sha256::Midstate = self.sighash_all.into();
        sha256::Hash::from_byte_array(midstate.to_parts().0)
    }
}

// Pointer must be manually free after dropping
impl Drop for CTxEnv {
    fn drop(&mut self) {
        unsafe {
            crate::alloc::rust_0_8_free(self.tx as *mut u8);
            crate::alloc::rust_0_8_free(self.taproot as *mut u8);
        }
    }
}

impl CRawBuffer {
    pub fn new(buf: &[c_uchar]) -> Self {
        Self {
            ptr: buf.as_ptr(),
            len: buf.len().try_into().expect("sane buffer lengths"),
        }
    }
}

// Safe wrappers around the bitcoin environment jets above. These expose a
// Rust-safe signature (`&mut CFrameItem`, `&CTxEnv`) that can be used directly
// as a `JetEnvironment::c_jet_ptr` function pointer, mirroring how the
// elements backend exposes its jets via the `jets_wrapper` module. We cannot
// hand out the raw `extern "C"` functions as fn pointers because `c_jet_ptr`
// returns a safe (non-extern) function pointer.
macro_rules! bitcoin_env_jet_wrapper {
    ($name:ident, $ffi:ident) => {
        pub fn $name(dst: &mut CFrameItem, src: CFrameItem, env: &CTxEnv) -> bool {
            // SAFETY: we pass valid references through as pointers to a C
            // function that treats them as borrowed (`frameItem* dst`,
            // `const txEnv* env`) and `src` is forwarded by value exactly as
            // the C `frameItem src` parameter expects.
            unsafe { $ffi(dst, src, env) }
        }
    };
}

bitcoin_env_jet_wrapper!(sig_all_hash, bitcoin_sig_all_hash);
bitcoin_env_jet_wrapper!(check_lock_height, bitcoin_check_lock_height);
bitcoin_env_jet_wrapper!(check_lock_distance, bitcoin_check_lock_distance);
bitcoin_env_jet_wrapper!(check_lock_duration, bitcoin_check_lock_duration);
bitcoin_env_jet_wrapper!(current_index, bitcoin_current_index);
bitcoin_env_jet_wrapper!(num_inputs, bitcoin_num_inputs);
bitcoin_env_jet_wrapper!(tx_hash, bitcoin_tx_hash);
bitcoin_env_jet_wrapper!(tap_env_hash, bitcoin_tap_env_hash);
bitcoin_env_jet_wrapper!(annex_hash, bitcoin_annex_hash);
bitcoin_env_jet_wrapper!(build_tapbranch, bitcoin_build_tapbranch);
bitcoin_env_jet_wrapper!(build_tapleaf_simplicity, bitcoin_build_tapleaf_simplicity);
bitcoin_env_jet_wrapper!(build_taptweak, bitcoin_build_taptweak);
bitcoin_env_jet_wrapper!(check_lock_time, bitcoin_check_lock_time);
bitcoin_env_jet_wrapper!(current_annex_hash, bitcoin_current_annex_hash);
bitcoin_env_jet_wrapper!(current_prev_outpoint, bitcoin_current_prev_outpoint);
bitcoin_env_jet_wrapper!(current_script_hash, bitcoin_current_script_hash);
bitcoin_env_jet_wrapper!(current_script_sig_hash, bitcoin_current_script_sig_hash);
bitcoin_env_jet_wrapper!(current_sequence, bitcoin_current_sequence);
bitcoin_env_jet_wrapper!(current_value, bitcoin_current_value);
bitcoin_env_jet_wrapper!(fee, bitcoin_fee);
bitcoin_env_jet_wrapper!(input_annex_hash, bitcoin_input_annex_hash);
bitcoin_env_jet_wrapper!(input_annexes_hash, bitcoin_input_annexes_hash);
bitcoin_env_jet_wrapper!(input_hash, bitcoin_input_hash);
bitcoin_env_jet_wrapper!(input_outpoints_hash, bitcoin_input_outpoints_hash);
bitcoin_env_jet_wrapper!(input_prev_outpoint, bitcoin_input_prev_outpoint);
bitcoin_env_jet_wrapper!(input_script_hash, bitcoin_input_script_hash);
bitcoin_env_jet_wrapper!(input_script_sig_hash, bitcoin_input_script_sig_hash);
bitcoin_env_jet_wrapper!(input_script_sigs_hash, bitcoin_input_script_sigs_hash);
bitcoin_env_jet_wrapper!(input_scripts_hash, bitcoin_input_scripts_hash);
bitcoin_env_jet_wrapper!(input_sequence, bitcoin_input_sequence);
bitcoin_env_jet_wrapper!(input_sequences_hash, bitcoin_input_sequences_hash);
bitcoin_env_jet_wrapper!(input_utxo_hash, bitcoin_input_utxo_hash);
bitcoin_env_jet_wrapper!(input_utxos_hash, bitcoin_input_utxos_hash);
bitcoin_env_jet_wrapper!(input_value, bitcoin_input_value);
bitcoin_env_jet_wrapper!(input_values_hash, bitcoin_input_values_hash);
bitcoin_env_jet_wrapper!(inputs_hash, bitcoin_inputs_hash);
bitcoin_env_jet_wrapper!(internal_key, bitcoin_internal_key);
bitcoin_env_jet_wrapper!(lock_time, bitcoin_lock_time);
bitcoin_env_jet_wrapper!(num_outputs, bitcoin_num_outputs);
bitcoin_env_jet_wrapper!(outpoint_hash, bitcoin_outpoint_hash);
bitcoin_env_jet_wrapper!(output_hash, bitcoin_output_hash);
bitcoin_env_jet_wrapper!(output_script_hash, bitcoin_output_script_hash);
bitcoin_env_jet_wrapper!(output_scripts_hash, bitcoin_output_scripts_hash);
bitcoin_env_jet_wrapper!(output_value, bitcoin_output_value);
bitcoin_env_jet_wrapper!(output_values_hash, bitcoin_output_values_hash);
bitcoin_env_jet_wrapper!(outputs_hash, bitcoin_outputs_hash);
bitcoin_env_jet_wrapper!(script_cmr, bitcoin_script_cmr);
bitcoin_env_jet_wrapper!(tapleaf_hash, bitcoin_tapleaf_hash);
bitcoin_env_jet_wrapper!(tapleaf_version, bitcoin_tapleaf_version);
bitcoin_env_jet_wrapper!(tappath, bitcoin_tappath);
bitcoin_env_jet_wrapper!(tappath_hash, bitcoin_tappath_hash);
bitcoin_env_jet_wrapper!(total_input_value, bitcoin_total_input_value);
bitcoin_env_jet_wrapper!(total_output_value, bitcoin_total_output_value);
bitcoin_env_jet_wrapper!(transaction_id, bitcoin_transaction_id);
bitcoin_env_jet_wrapper!(tx_is_final, bitcoin_tx_is_final);
bitcoin_env_jet_wrapper!(tx_lock_distance, bitcoin_tx_lock_distance);
bitcoin_env_jet_wrapper!(tx_lock_duration, bitcoin_tx_lock_duration);
bitcoin_env_jet_wrapper!(tx_lock_height, bitcoin_tx_lock_height);
bitcoin_env_jet_wrapper!(tx_lock_time, bitcoin_tx_lock_time);
bitcoin_env_jet_wrapper!(version, bitcoin_version);

// Will uncomment in a later commit; need to update libsimplicity first.
#[cfg(test)]
mod tests {
    use core::mem::{align_of, size_of};

    use crate::c_jets::frame_ffi::{c_alignof_frameItem, c_sizeof_frameItem, CFrameItem};

    use super::*;

    #[test]
    fn test_sizes() {
        unsafe {
            assert_eq!(size_of::<CFrameItem>(), c_sizeof_frameItem);
            assert_eq!(size_of::<CRawBuffer>(), c_sizeof_rawBuffer);
            assert_eq!(size_of::<CRawInput>(), c_sizeof_rawInput);
            assert_eq!(size_of::<CRawOutput>(), c_sizeof_rawOutput);
            assert_eq!(size_of::<CRawTransaction>(), c_sizeof_rawTransaction);
            assert_eq!(size_of::<CRawTapEnv>(), c_sizeof_rawTapEnv);
            assert_eq!(size_of::<CTxEnv>(), c_sizeof_txEnv);
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
            assert_eq!(align_of::<CTxEnv>(), c_alignof_txEnv);
        }
    }
}
