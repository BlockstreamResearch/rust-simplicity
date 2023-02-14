/* This file has been automatically generated. */

use crate::CElementsTxEnv;
use super::{frame_ffi::CFrameItem, elements_ffi};

pub fn add_32<T>(dst: &mut CFrameItem, src: CFrameItem, _env: &T) -> bool {
    unsafe { elements_ffi::add_32(dst, src, std::ptr::null()) }
}

pub fn annex_hash(dst: &mut CFrameItem, src: CFrameItem, env: &CElementsTxEnv) -> bool {
    unsafe { elements_ffi::annex_hash(dst, src, env) }
}

pub fn asset_amount_hash(dst: &mut CFrameItem, src: CFrameItem, env: &CElementsTxEnv) -> bool {
    unsafe { elements_ffi::asset_amount_hash(dst, src, env) }
}

pub fn bip_0340_verify<T>(dst: &mut CFrameItem, src: CFrameItem, _env: &T) -> bool {
    unsafe { elements_ffi::bip_0340_verify(dst, src, std::ptr::null()) }
}

pub fn build_tapbranch(dst: &mut CFrameItem, src: CFrameItem, env: &CElementsTxEnv) -> bool {
    unsafe { elements_ffi::build_tapbranch(dst, src, env) }
}

pub fn build_tapleaf_simplicity(dst: &mut CFrameItem, src: CFrameItem, env: &CElementsTxEnv) -> bool {
    unsafe { elements_ffi::build_tapleaf_simplicity(dst, src, env) }
}

pub fn calculate_asset(dst: &mut CFrameItem, src: CFrameItem, env: &CElementsTxEnv) -> bool {
    unsafe { elements_ffi::calculate_asset(dst, src, env) }
}

pub fn calculate_confidential_token(dst: &mut CFrameItem, src: CFrameItem, env: &CElementsTxEnv) -> bool {
    unsafe { elements_ffi::calculate_confidential_token(dst, src, env) }
}

pub fn calculate_explicit_token(dst: &mut CFrameItem, src: CFrameItem, env: &CElementsTxEnv) -> bool {
    unsafe { elements_ffi::calculate_explicit_token(dst, src, env) }
}

pub fn calculate_issuance_entropy(dst: &mut CFrameItem, src: CFrameItem, env: &CElementsTxEnv) -> bool {
    unsafe { elements_ffi::calculate_issuance_entropy(dst, src, env) }
}

pub fn check_lock_distance(dst: &mut CFrameItem, src: CFrameItem, env: &CElementsTxEnv) -> bool {
    unsafe { elements_ffi::check_lock_distance(dst, src, env) }
}

pub fn check_lock_duration(dst: &mut CFrameItem, src: CFrameItem, env: &CElementsTxEnv) -> bool {
    unsafe { elements_ffi::check_lock_duration(dst, src, env) }
}

pub fn check_lock_height(dst: &mut CFrameItem, src: CFrameItem, env: &CElementsTxEnv) -> bool {
    unsafe { elements_ffi::check_lock_height(dst, src, env) }
}

pub fn check_lock_time(dst: &mut CFrameItem, src: CFrameItem, env: &CElementsTxEnv) -> bool {
    unsafe { elements_ffi::check_lock_time(dst, src, env) }
}

pub fn check_sig_verify<T>(dst: &mut CFrameItem, src: CFrameItem, _env: &T) -> bool {
    unsafe { elements_ffi::check_sig_verify(dst, src, std::ptr::null()) }
}

pub fn current_amount(dst: &mut CFrameItem, src: CFrameItem, env: &CElementsTxEnv) -> bool {
    unsafe { elements_ffi::current_amount(dst, src, env) }
}

pub fn current_annex_hash(dst: &mut CFrameItem, src: CFrameItem, env: &CElementsTxEnv) -> bool {
    unsafe { elements_ffi::current_annex_hash(dst, src, env) }
}

pub fn current_asset(dst: &mut CFrameItem, src: CFrameItem, env: &CElementsTxEnv) -> bool {
    unsafe { elements_ffi::current_asset(dst, src, env) }
}

pub fn current_index(dst: &mut CFrameItem, src: CFrameItem, env: &CElementsTxEnv) -> bool {
    unsafe { elements_ffi::current_index(dst, src, env) }
}

pub fn current_issuance_asset_amount(dst: &mut CFrameItem, src: CFrameItem, env: &CElementsTxEnv) -> bool {
    unsafe { elements_ffi::current_issuance_asset_amount(dst, src, env) }
}

pub fn current_issuance_asset_proof(dst: &mut CFrameItem, src: CFrameItem, env: &CElementsTxEnv) -> bool {
    unsafe { elements_ffi::current_issuance_asset_proof(dst, src, env) }
}

pub fn current_issuance_token_amount(dst: &mut CFrameItem, src: CFrameItem, env: &CElementsTxEnv) -> bool {
    unsafe { elements_ffi::current_issuance_token_amount(dst, src, env) }
}

pub fn current_issuance_token_proof(dst: &mut CFrameItem, src: CFrameItem, env: &CElementsTxEnv) -> bool {
    unsafe { elements_ffi::current_issuance_token_proof(dst, src, env) }
}

pub fn current_new_issuance_contract(dst: &mut CFrameItem, src: CFrameItem, env: &CElementsTxEnv) -> bool {
    unsafe { elements_ffi::current_new_issuance_contract(dst, src, env) }
}

pub fn current_pegin(dst: &mut CFrameItem, src: CFrameItem, env: &CElementsTxEnv) -> bool {
    unsafe { elements_ffi::current_pegin(dst, src, env) }
}

pub fn current_prev_outpoint(dst: &mut CFrameItem, src: CFrameItem, env: &CElementsTxEnv) -> bool {
    unsafe { elements_ffi::current_prev_outpoint(dst, src, env) }
}

pub fn current_reissuance_blinding(dst: &mut CFrameItem, src: CFrameItem, env: &CElementsTxEnv) -> bool {
    unsafe { elements_ffi::current_reissuance_blinding(dst, src, env) }
}

pub fn current_reissuance_entropy(dst: &mut CFrameItem, src: CFrameItem, env: &CElementsTxEnv) -> bool {
    unsafe { elements_ffi::current_reissuance_entropy(dst, src, env) }
}

pub fn current_script_hash(dst: &mut CFrameItem, src: CFrameItem, env: &CElementsTxEnv) -> bool {
    unsafe { elements_ffi::current_script_hash(dst, src, env) }
}

pub fn current_script_sig_hash(dst: &mut CFrameItem, src: CFrameItem, env: &CElementsTxEnv) -> bool {
    unsafe { elements_ffi::current_script_sig_hash(dst, src, env) }
}

pub fn current_sequence(dst: &mut CFrameItem, src: CFrameItem, env: &CElementsTxEnv) -> bool {
    unsafe { elements_ffi::current_sequence(dst, src, env) }
}

pub fn decompress<T>(dst: &mut CFrameItem, src: CFrameItem, _env: &T) -> bool {
    unsafe { elements_ffi::decompress(dst, src, std::ptr::null()) }
}

pub fn eq_256<T>(dst: &mut CFrameItem, src: CFrameItem, _env: &T) -> bool {
    unsafe { elements_ffi::eq_256(dst, src, std::ptr::null()) }
}

pub fn eq_32<T>(dst: &mut CFrameItem, src: CFrameItem, _env: &T) -> bool {
    unsafe { elements_ffi::eq_32(dst, src, std::ptr::null()) }
}

pub fn fe_add<T>(dst: &mut CFrameItem, src: CFrameItem, _env: &T) -> bool {
    unsafe { elements_ffi::fe_add(dst, src, std::ptr::null()) }
}

pub fn fe_invert<T>(dst: &mut CFrameItem, src: CFrameItem, _env: &T) -> bool {
    unsafe { elements_ffi::fe_invert(dst, src, std::ptr::null()) }
}

pub fn fe_is_odd<T>(dst: &mut CFrameItem, src: CFrameItem, _env: &T) -> bool {
    unsafe { elements_ffi::fe_is_odd(dst, src, std::ptr::null()) }
}

pub fn fe_is_zero<T>(dst: &mut CFrameItem, src: CFrameItem, _env: &T) -> bool {
    unsafe { elements_ffi::fe_is_zero(dst, src, std::ptr::null()) }
}

pub fn fe_multiply<T>(dst: &mut CFrameItem, src: CFrameItem, _env: &T) -> bool {
    unsafe { elements_ffi::fe_multiply(dst, src, std::ptr::null()) }
}

pub fn fe_multiply_beta<T>(dst: &mut CFrameItem, src: CFrameItem, _env: &T) -> bool {
    unsafe { elements_ffi::fe_multiply_beta(dst, src, std::ptr::null()) }
}

pub fn fe_negate<T>(dst: &mut CFrameItem, src: CFrameItem, _env: &T) -> bool {
    unsafe { elements_ffi::fe_negate(dst, src, std::ptr::null()) }
}

pub fn fe_normalize<T>(dst: &mut CFrameItem, src: CFrameItem, _env: &T) -> bool {
    unsafe { elements_ffi::fe_normalize(dst, src, std::ptr::null()) }
}

pub fn fe_square<T>(dst: &mut CFrameItem, src: CFrameItem, _env: &T) -> bool {
    unsafe { elements_ffi::fe_square(dst, src, std::ptr::null()) }
}

pub fn fe_square_root<T>(dst: &mut CFrameItem, src: CFrameItem, _env: &T) -> bool {
    unsafe { elements_ffi::fe_square_root(dst, src, std::ptr::null()) }
}

pub fn full_add_32<T>(dst: &mut CFrameItem, src: CFrameItem, _env: &T) -> bool {
    unsafe { elements_ffi::full_add_32(dst, src, std::ptr::null()) }
}

pub fn full_multiply_32<T>(dst: &mut CFrameItem, src: CFrameItem, _env: &T) -> bool {
    unsafe { elements_ffi::full_multiply_32(dst, src, std::ptr::null()) }
}

pub fn full_subtract_32<T>(dst: &mut CFrameItem, src: CFrameItem, _env: &T) -> bool {
    unsafe { elements_ffi::full_subtract_32(dst, src, std::ptr::null()) }
}

pub fn ge_is_on_curve<T>(dst: &mut CFrameItem, src: CFrameItem, _env: &T) -> bool {
    unsafe { elements_ffi::ge_is_on_curve(dst, src, std::ptr::null()) }
}

pub fn ge_negate<T>(dst: &mut CFrameItem, src: CFrameItem, _env: &T) -> bool {
    unsafe { elements_ffi::ge_negate(dst, src, std::ptr::null()) }
}

pub fn gej_add<T>(dst: &mut CFrameItem, src: CFrameItem, _env: &T) -> bool {
    unsafe { elements_ffi::gej_add(dst, src, std::ptr::null()) }
}

pub fn gej_double<T>(dst: &mut CFrameItem, src: CFrameItem, _env: &T) -> bool {
    unsafe { elements_ffi::gej_double(dst, src, std::ptr::null()) }
}

pub fn gej_ge_add<T>(dst: &mut CFrameItem, src: CFrameItem, _env: &T) -> bool {
    unsafe { elements_ffi::gej_ge_add(dst, src, std::ptr::null()) }
}

pub fn gej_ge_add_ex<T>(dst: &mut CFrameItem, src: CFrameItem, _env: &T) -> bool {
    unsafe { elements_ffi::gej_ge_add_ex(dst, src, std::ptr::null()) }
}

pub fn gej_infinity<T>(dst: &mut CFrameItem, src: CFrameItem, _env: &T) -> bool {
    unsafe { elements_ffi::gej_infinity(dst, src, std::ptr::null()) }
}

pub fn gej_is_infinity<T>(dst: &mut CFrameItem, src: CFrameItem, _env: &T) -> bool {
    unsafe { elements_ffi::gej_is_infinity(dst, src, std::ptr::null()) }
}

pub fn gej_is_on_curve<T>(dst: &mut CFrameItem, src: CFrameItem, _env: &T) -> bool {
    unsafe { elements_ffi::gej_is_on_curve(dst, src, std::ptr::null()) }
}

pub fn gej_negate<T>(dst: &mut CFrameItem, src: CFrameItem, _env: &T) -> bool {
    unsafe { elements_ffi::gej_negate(dst, src, std::ptr::null()) }
}

pub fn gej_normalize<T>(dst: &mut CFrameItem, src: CFrameItem, _env: &T) -> bool {
    unsafe { elements_ffi::gej_normalize(dst, src, std::ptr::null()) }
}

pub fn gej_rescale<T>(dst: &mut CFrameItem, src: CFrameItem, _env: &T) -> bool {
    unsafe { elements_ffi::gej_rescale(dst, src, std::ptr::null()) }
}

pub fn gej_x_equiv<T>(dst: &mut CFrameItem, src: CFrameItem, _env: &T) -> bool {
    unsafe { elements_ffi::gej_x_equiv(dst, src, std::ptr::null()) }
}

pub fn gej_y_is_odd<T>(dst: &mut CFrameItem, src: CFrameItem, _env: &T) -> bool {
    unsafe { elements_ffi::gej_y_is_odd(dst, src, std::ptr::null()) }
}

pub fn generate<T>(dst: &mut CFrameItem, src: CFrameItem, _env: &T) -> bool {
    unsafe { elements_ffi::generate(dst, src, std::ptr::null()) }
}

pub fn genesis_block_hash(dst: &mut CFrameItem, src: CFrameItem, env: &CElementsTxEnv) -> bool {
    unsafe { elements_ffi::genesis_block_hash(dst, src, env) }
}

pub fn input_amount(dst: &mut CFrameItem, src: CFrameItem, env: &CElementsTxEnv) -> bool {
    unsafe { elements_ffi::input_amount(dst, src, env) }
}

pub fn input_amounts_hash(dst: &mut CFrameItem, src: CFrameItem, env: &CElementsTxEnv) -> bool {
    unsafe { elements_ffi::input_amounts_hash(dst, src, env) }
}

pub fn input_annex_hash(dst: &mut CFrameItem, src: CFrameItem, env: &CElementsTxEnv) -> bool {
    unsafe { elements_ffi::input_annex_hash(dst, src, env) }
}

pub fn input_annexes_hash(dst: &mut CFrameItem, src: CFrameItem, env: &CElementsTxEnv) -> bool {
    unsafe { elements_ffi::input_annexes_hash(dst, src, env) }
}

pub fn input_asset(dst: &mut CFrameItem, src: CFrameItem, env: &CElementsTxEnv) -> bool {
    unsafe { elements_ffi::input_asset(dst, src, env) }
}

pub fn input_outpoints_hash(dst: &mut CFrameItem, src: CFrameItem, env: &CElementsTxEnv) -> bool {
    unsafe { elements_ffi::input_outpoints_hash(dst, src, env) }
}

pub fn input_pegin(dst: &mut CFrameItem, src: CFrameItem, env: &CElementsTxEnv) -> bool {
    unsafe { elements_ffi::input_pegin(dst, src, env) }
}

pub fn input_prev_outpoint(dst: &mut CFrameItem, src: CFrameItem, env: &CElementsTxEnv) -> bool {
    unsafe { elements_ffi::input_prev_outpoint(dst, src, env) }
}

pub fn input_script_hash(dst: &mut CFrameItem, src: CFrameItem, env: &CElementsTxEnv) -> bool {
    unsafe { elements_ffi::input_script_hash(dst, src, env) }
}

pub fn input_script_sig_hash(dst: &mut CFrameItem, src: CFrameItem, env: &CElementsTxEnv) -> bool {
    unsafe { elements_ffi::input_script_sig_hash(dst, src, env) }
}

pub fn input_script_sigs_hash(dst: &mut CFrameItem, src: CFrameItem, env: &CElementsTxEnv) -> bool {
    unsafe { elements_ffi::input_script_sigs_hash(dst, src, env) }
}

pub fn input_scripts_hash(dst: &mut CFrameItem, src: CFrameItem, env: &CElementsTxEnv) -> bool {
    unsafe { elements_ffi::input_scripts_hash(dst, src, env) }
}

pub fn input_sequence(dst: &mut CFrameItem, src: CFrameItem, env: &CElementsTxEnv) -> bool {
    unsafe { elements_ffi::input_sequence(dst, src, env) }
}

pub fn input_sequences_hash(dst: &mut CFrameItem, src: CFrameItem, env: &CElementsTxEnv) -> bool {
    unsafe { elements_ffi::input_sequences_hash(dst, src, env) }
}

pub fn input_utxos_hash(dst: &mut CFrameItem, src: CFrameItem, env: &CElementsTxEnv) -> bool {
    unsafe { elements_ffi::input_utxos_hash(dst, src, env) }
}

pub fn inputs_hash(dst: &mut CFrameItem, src: CFrameItem, env: &CElementsTxEnv) -> bool {
    unsafe { elements_ffi::inputs_hash(dst, src, env) }
}

pub fn internal_key(dst: &mut CFrameItem, src: CFrameItem, env: &CElementsTxEnv) -> bool {
    unsafe { elements_ffi::internal_key(dst, src, env) }
}

pub fn issuance(dst: &mut CFrameItem, src: CFrameItem, env: &CElementsTxEnv) -> bool {
    unsafe { elements_ffi::issuance(dst, src, env) }
}

pub fn issuance_asset(dst: &mut CFrameItem, src: CFrameItem, env: &CElementsTxEnv) -> bool {
    unsafe { elements_ffi::issuance_asset(dst, src, env) }
}

pub fn issuance_asset_amount(dst: &mut CFrameItem, src: CFrameItem, env: &CElementsTxEnv) -> bool {
    unsafe { elements_ffi::issuance_asset_amount(dst, src, env) }
}

pub fn issuance_asset_amounts_hash(dst: &mut CFrameItem, src: CFrameItem, env: &CElementsTxEnv) -> bool {
    unsafe { elements_ffi::issuance_asset_amounts_hash(dst, src, env) }
}

pub fn issuance_asset_proof(dst: &mut CFrameItem, src: CFrameItem, env: &CElementsTxEnv) -> bool {
    unsafe { elements_ffi::issuance_asset_proof(dst, src, env) }
}

pub fn issuance_blinding_entropy_hash(dst: &mut CFrameItem, src: CFrameItem, env: &CElementsTxEnv) -> bool {
    unsafe { elements_ffi::issuance_blinding_entropy_hash(dst, src, env) }
}

pub fn issuance_entropy(dst: &mut CFrameItem, src: CFrameItem, env: &CElementsTxEnv) -> bool {
    unsafe { elements_ffi::issuance_entropy(dst, src, env) }
}

pub fn issuance_range_proofs_hash(dst: &mut CFrameItem, src: CFrameItem, env: &CElementsTxEnv) -> bool {
    unsafe { elements_ffi::issuance_range_proofs_hash(dst, src, env) }
}

pub fn issuance_token(dst: &mut CFrameItem, src: CFrameItem, env: &CElementsTxEnv) -> bool {
    unsafe { elements_ffi::issuance_token(dst, src, env) }
}

pub fn issuance_token_amount(dst: &mut CFrameItem, src: CFrameItem, env: &CElementsTxEnv) -> bool {
    unsafe { elements_ffi::issuance_token_amount(dst, src, env) }
}

pub fn issuance_token_amounts_hash(dst: &mut CFrameItem, src: CFrameItem, env: &CElementsTxEnv) -> bool {
    unsafe { elements_ffi::issuance_token_amounts_hash(dst, src, env) }
}

pub fn issuance_token_proof(dst: &mut CFrameItem, src: CFrameItem, env: &CElementsTxEnv) -> bool {
    unsafe { elements_ffi::issuance_token_proof(dst, src, env) }
}

pub fn issuances_hash(dst: &mut CFrameItem, src: CFrameItem, env: &CElementsTxEnv) -> bool {
    unsafe { elements_ffi::issuances_hash(dst, src, env) }
}

pub fn le_32<T>(dst: &mut CFrameItem, src: CFrameItem, _env: &T) -> bool {
    unsafe { elements_ffi::le_32(dst, src, std::ptr::null()) }
}

pub fn linear_combination_1<T>(dst: &mut CFrameItem, src: CFrameItem, _env: &T) -> bool {
    unsafe { elements_ffi::linear_combination_1(dst, src, std::ptr::null()) }
}

pub fn linear_verify_1<T>(dst: &mut CFrameItem, src: CFrameItem, _env: &T) -> bool {
    unsafe { elements_ffi::linear_verify_1(dst, src, std::ptr::null()) }
}

pub fn lock_time(dst: &mut CFrameItem, src: CFrameItem, env: &CElementsTxEnv) -> bool {
    unsafe { elements_ffi::lock_time(dst, src, env) }
}

pub fn low_32<T>(dst: &mut CFrameItem, src: CFrameItem, _env: &T) -> bool {
    unsafe { elements_ffi::low_32(dst, src, std::ptr::null()) }
}

pub fn multiply_32<T>(dst: &mut CFrameItem, src: CFrameItem, _env: &T) -> bool {
    unsafe { elements_ffi::multiply_32(dst, src, std::ptr::null()) }
}

pub fn new_issuance_contract(dst: &mut CFrameItem, src: CFrameItem, env: &CElementsTxEnv) -> bool {
    unsafe { elements_ffi::new_issuance_contract(dst, src, env) }
}

pub fn nonce_hash(dst: &mut CFrameItem, src: CFrameItem, env: &CElementsTxEnv) -> bool {
    unsafe { elements_ffi::nonce_hash(dst, src, env) }
}

pub fn num_inputs(dst: &mut CFrameItem, src: CFrameItem, env: &CElementsTxEnv) -> bool {
    unsafe { elements_ffi::num_inputs(dst, src, env) }
}

pub fn num_outputs(dst: &mut CFrameItem, src: CFrameItem, env: &CElementsTxEnv) -> bool {
    unsafe { elements_ffi::num_outputs(dst, src, env) }
}

pub fn one_32<T>(dst: &mut CFrameItem, src: CFrameItem, _env: &T) -> bool {
    unsafe { elements_ffi::one_32(dst, src, std::ptr::null()) }
}

pub fn outpoint_hash(dst: &mut CFrameItem, src: CFrameItem, env: &CElementsTxEnv) -> bool {
    unsafe { elements_ffi::outpoint_hash(dst, src, env) }
}

pub fn output_amount(dst: &mut CFrameItem, src: CFrameItem, env: &CElementsTxEnv) -> bool {
    unsafe { elements_ffi::output_amount(dst, src, env) }
}

pub fn output_amounts_hash(dst: &mut CFrameItem, src: CFrameItem, env: &CElementsTxEnv) -> bool {
    unsafe { elements_ffi::output_amounts_hash(dst, src, env) }
}

pub fn output_asset(dst: &mut CFrameItem, src: CFrameItem, env: &CElementsTxEnv) -> bool {
    unsafe { elements_ffi::output_asset(dst, src, env) }
}

pub fn output_nonce(dst: &mut CFrameItem, src: CFrameItem, env: &CElementsTxEnv) -> bool {
    unsafe { elements_ffi::output_nonce(dst, src, env) }
}

pub fn output_nonces_hash(dst: &mut CFrameItem, src: CFrameItem, env: &CElementsTxEnv) -> bool {
    unsafe { elements_ffi::output_nonces_hash(dst, src, env) }
}

pub fn output_null_datum(dst: &mut CFrameItem, src: CFrameItem, env: &CElementsTxEnv) -> bool {
    unsafe { elements_ffi::output_null_datum(dst, src, env) }
}

pub fn output_range_proof(dst: &mut CFrameItem, src: CFrameItem, env: &CElementsTxEnv) -> bool {
    unsafe { elements_ffi::output_range_proof(dst, src, env) }
}

pub fn output_range_proofs_hash(dst: &mut CFrameItem, src: CFrameItem, env: &CElementsTxEnv) -> bool {
    unsafe { elements_ffi::output_range_proofs_hash(dst, src, env) }
}

pub fn output_script_hash(dst: &mut CFrameItem, src: CFrameItem, env: &CElementsTxEnv) -> bool {
    unsafe { elements_ffi::output_script_hash(dst, src, env) }
}

pub fn output_scripts_hash(dst: &mut CFrameItem, src: CFrameItem, env: &CElementsTxEnv) -> bool {
    unsafe { elements_ffi::output_scripts_hash(dst, src, env) }
}

pub fn output_surjection_proof(dst: &mut CFrameItem, src: CFrameItem, env: &CElementsTxEnv) -> bool {
    unsafe { elements_ffi::output_surjection_proof(dst, src, env) }
}

pub fn output_surjection_proofs_hash(dst: &mut CFrameItem, src: CFrameItem, env: &CElementsTxEnv) -> bool {
    unsafe { elements_ffi::output_surjection_proofs_hash(dst, src, env) }
}

pub fn outputs_hash(dst: &mut CFrameItem, src: CFrameItem, env: &CElementsTxEnv) -> bool {
    unsafe { elements_ffi::outputs_hash(dst, src, env) }
}

pub fn parse_lock<T>(dst: &mut CFrameItem, src: CFrameItem, _env: &T) -> bool {
    unsafe { elements_ffi::parse_lock(dst, src, std::ptr::null()) }
}

pub fn parse_sequence<T>(dst: &mut CFrameItem, src: CFrameItem, _env: &T) -> bool {
    unsafe { elements_ffi::parse_sequence(dst, src, std::ptr::null()) }
}

pub fn point_verify_1<T>(dst: &mut CFrameItem, src: CFrameItem, _env: &T) -> bool {
    unsafe { elements_ffi::point_verify_1(dst, src, std::ptr::null()) }
}

pub fn reissuance_blinding(dst: &mut CFrameItem, src: CFrameItem, env: &CElementsTxEnv) -> bool {
    unsafe { elements_ffi::reissuance_blinding(dst, src, env) }
}

pub fn reissuance_entropy(dst: &mut CFrameItem, src: CFrameItem, env: &CElementsTxEnv) -> bool {
    unsafe { elements_ffi::reissuance_entropy(dst, src, env) }
}

pub fn scalar_add<T>(dst: &mut CFrameItem, src: CFrameItem, _env: &T) -> bool {
    unsafe { elements_ffi::scalar_add(dst, src, std::ptr::null()) }
}

pub fn scalar_invert<T>(dst: &mut CFrameItem, src: CFrameItem, _env: &T) -> bool {
    unsafe { elements_ffi::scalar_invert(dst, src, std::ptr::null()) }
}

pub fn scalar_is_zero<T>(dst: &mut CFrameItem, src: CFrameItem, _env: &T) -> bool {
    unsafe { elements_ffi::scalar_is_zero(dst, src, std::ptr::null()) }
}

pub fn scalar_multiply<T>(dst: &mut CFrameItem, src: CFrameItem, _env: &T) -> bool {
    unsafe { elements_ffi::scalar_multiply(dst, src, std::ptr::null()) }
}

pub fn scalar_multiply_lambda<T>(dst: &mut CFrameItem, src: CFrameItem, _env: &T) -> bool {
    unsafe { elements_ffi::scalar_multiply_lambda(dst, src, std::ptr::null()) }
}

pub fn scalar_negate<T>(dst: &mut CFrameItem, src: CFrameItem, _env: &T) -> bool {
    unsafe { elements_ffi::scalar_negate(dst, src, std::ptr::null()) }
}

pub fn scalar_normalize<T>(dst: &mut CFrameItem, src: CFrameItem, _env: &T) -> bool {
    unsafe { elements_ffi::scalar_normalize(dst, src, std::ptr::null()) }
}

pub fn scalar_square<T>(dst: &mut CFrameItem, src: CFrameItem, _env: &T) -> bool {
    unsafe { elements_ffi::scalar_square(dst, src, std::ptr::null()) }
}

pub fn scale<T>(dst: &mut CFrameItem, src: CFrameItem, _env: &T) -> bool {
    unsafe { elements_ffi::scale(dst, src, std::ptr::null()) }
}

pub fn script_cmr(dst: &mut CFrameItem, src: CFrameItem, env: &CElementsTxEnv) -> bool {
    unsafe { elements_ffi::script_cmr(dst, src, env) }
}

pub fn sha_256_block<T>(dst: &mut CFrameItem, src: CFrameItem, _env: &T) -> bool {
    unsafe { elements_ffi::sha_256_block(dst, src, std::ptr::null()) }
}

pub fn sha_256_ctx_8_add_1<T>(dst: &mut CFrameItem, src: CFrameItem, _env: &T) -> bool {
    unsafe { elements_ffi::sha_256_ctx_8_add_1(dst, src, std::ptr::null()) }
}

pub fn sha_256_ctx_8_add_128<T>(dst: &mut CFrameItem, src: CFrameItem, _env: &T) -> bool {
    unsafe { elements_ffi::sha_256_ctx_8_add_128(dst, src, std::ptr::null()) }
}

pub fn sha_256_ctx_8_add_16<T>(dst: &mut CFrameItem, src: CFrameItem, _env: &T) -> bool {
    unsafe { elements_ffi::sha_256_ctx_8_add_16(dst, src, std::ptr::null()) }
}

pub fn sha_256_ctx_8_add_2<T>(dst: &mut CFrameItem, src: CFrameItem, _env: &T) -> bool {
    unsafe { elements_ffi::sha_256_ctx_8_add_2(dst, src, std::ptr::null()) }
}

pub fn sha_256_ctx_8_add_256<T>(dst: &mut CFrameItem, src: CFrameItem, _env: &T) -> bool {
    unsafe { elements_ffi::sha_256_ctx_8_add_256(dst, src, std::ptr::null()) }
}

pub fn sha_256_ctx_8_add_32<T>(dst: &mut CFrameItem, src: CFrameItem, _env: &T) -> bool {
    unsafe { elements_ffi::sha_256_ctx_8_add_32(dst, src, std::ptr::null()) }
}

pub fn sha_256_ctx_8_add_4<T>(dst: &mut CFrameItem, src: CFrameItem, _env: &T) -> bool {
    unsafe { elements_ffi::sha_256_ctx_8_add_4(dst, src, std::ptr::null()) }
}

pub fn sha_256_ctx_8_add_512<T>(dst: &mut CFrameItem, src: CFrameItem, _env: &T) -> bool {
    unsafe { elements_ffi::sha_256_ctx_8_add_512(dst, src, std::ptr::null()) }
}

pub fn sha_256_ctx_8_add_64<T>(dst: &mut CFrameItem, src: CFrameItem, _env: &T) -> bool {
    unsafe { elements_ffi::sha_256_ctx_8_add_64(dst, src, std::ptr::null()) }
}

pub fn sha_256_ctx_8_add_8<T>(dst: &mut CFrameItem, src: CFrameItem, _env: &T) -> bool {
    unsafe { elements_ffi::sha_256_ctx_8_add_8(dst, src, std::ptr::null()) }
}

pub fn sha_256_ctx_8_add_buffer_511<T>(dst: &mut CFrameItem, src: CFrameItem, _env: &T) -> bool {
    unsafe { elements_ffi::sha_256_ctx_8_add_buffer_511(dst, src, std::ptr::null()) }
}

pub fn sha_256_ctx_8_finalize<T>(dst: &mut CFrameItem, src: CFrameItem, _env: &T) -> bool {
    unsafe { elements_ffi::sha_256_ctx_8_finalize(dst, src, std::ptr::null()) }
}

pub fn sha_256_ctx_8_init<T>(dst: &mut CFrameItem, src: CFrameItem, _env: &T) -> bool {
    unsafe { elements_ffi::sha_256_ctx_8_init(dst, src, std::ptr::null()) }
}

pub fn sha_256_iv<T>(dst: &mut CFrameItem, src: CFrameItem, _env: &T) -> bool {
    unsafe { elements_ffi::sha_256_iv(dst, src, std::ptr::null()) }
}

pub fn sig_all_hash(dst: &mut CFrameItem, src: CFrameItem, env: &CElementsTxEnv) -> bool {
    unsafe { elements_ffi::sig_all_hash(dst, src, env) }
}

pub fn subtract_32<T>(dst: &mut CFrameItem, src: CFrameItem, _env: &T) -> bool {
    unsafe { elements_ffi::subtract_32(dst, src, std::ptr::null()) }
}

pub fn tap_env_hash(dst: &mut CFrameItem, src: CFrameItem, env: &CElementsTxEnv) -> bool {
    unsafe { elements_ffi::tap_env_hash(dst, src, env) }
}

pub fn tapbranch(dst: &mut CFrameItem, src: CFrameItem, env: &CElementsTxEnv) -> bool {
    unsafe { elements_ffi::tapbranch(dst, src, env) }
}

pub fn tapbranch_hash(dst: &mut CFrameItem, src: CFrameItem, env: &CElementsTxEnv) -> bool {
    unsafe { elements_ffi::tapbranch_hash(dst, src, env) }
}

pub fn tapleaf_hash(dst: &mut CFrameItem, src: CFrameItem, env: &CElementsTxEnv) -> bool {
    unsafe { elements_ffi::tapleaf_hash(dst, src, env) }
}

pub fn tapleaf_version(dst: &mut CFrameItem, src: CFrameItem, env: &CElementsTxEnv) -> bool {
    unsafe { elements_ffi::tapleaf_version(dst, src, env) }
}

pub fn tx_hash(dst: &mut CFrameItem, src: CFrameItem, env: &CElementsTxEnv) -> bool {
    unsafe { elements_ffi::tx_hash(dst, src, env) }
}

pub fn tx_is_final(dst: &mut CFrameItem, src: CFrameItem, env: &CElementsTxEnv) -> bool {
    unsafe { elements_ffi::tx_is_final(dst, src, env) }
}

pub fn tx_lock_distance(dst: &mut CFrameItem, src: CFrameItem, env: &CElementsTxEnv) -> bool {
    unsafe { elements_ffi::tx_lock_distance(dst, src, env) }
}

pub fn tx_lock_duration(dst: &mut CFrameItem, src: CFrameItem, env: &CElementsTxEnv) -> bool {
    unsafe { elements_ffi::tx_lock_duration(dst, src, env) }
}

pub fn tx_lock_height(dst: &mut CFrameItem, src: CFrameItem, env: &CElementsTxEnv) -> bool {
    unsafe { elements_ffi::tx_lock_height(dst, src, env) }
}

pub fn tx_lock_time(dst: &mut CFrameItem, src: CFrameItem, env: &CElementsTxEnv) -> bool {
    unsafe { elements_ffi::tx_lock_time(dst, src, env) }
}

pub fn verify<T>(dst: &mut CFrameItem, src: CFrameItem, _env: &T) -> bool {
    unsafe { elements_ffi::verify(dst, src, std::ptr::null()) }
}

pub fn version(dst: &mut CFrameItem, src: CFrameItem, env: &CElementsTxEnv) -> bool {
    unsafe { elements_ffi::version(dst, src, env) }
}

