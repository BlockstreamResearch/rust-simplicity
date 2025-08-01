/* This module defines primitives and jets that are specific to the Elements application for Simplicity.
 */
#ifndef SIMPLICITY_ELEMENTS_ELEMENTSJETS_H
#define SIMPLICITY_ELEMENTS_ELEMENTSJETS_H

#include "../jets.h"

/* Jets for the Elements application of Simplicity. */
bool rustsimplicity_0_5_version(frameItem* dst, frameItem src, const txEnv* env);
bool rustsimplicity_0_5_lock_time(frameItem* dst, frameItem src, const txEnv* env);
bool rustsimplicity_0_5_input_pegin(frameItem* dst, frameItem src, const txEnv* env);
bool rustsimplicity_0_5_input_prev_outpoint(frameItem* dst, frameItem src, const txEnv* env);
bool rustsimplicity_0_5_input_asset(frameItem* dst, frameItem src, const txEnv* env);
bool rustsimplicity_0_5_input_amount(frameItem* dst, frameItem src, const txEnv* env);
bool rustsimplicity_0_5_input_script_hash(frameItem* dst, frameItem src, const txEnv* env);
bool rustsimplicity_0_5_input_sequence(frameItem* dst, frameItem src, const txEnv* env);
bool rustsimplicity_0_5_input_annex_hash(frameItem* dst, frameItem src, const txEnv* env);
bool rustsimplicity_0_5_input_script_sig_hash(frameItem* dst, frameItem src, const txEnv* env);
bool rustsimplicity_0_5_reissuance_blinding(frameItem* dst, frameItem src, const txEnv* env);
bool rustsimplicity_0_5_new_issuance_contract(frameItem* dst, frameItem src, const txEnv* env);
bool rustsimplicity_0_5_reissuance_entropy(frameItem* dst, frameItem src, const txEnv* env);
bool rustsimplicity_0_5_issuance_asset_amount(frameItem* dst, frameItem src, const txEnv* env);
bool rustsimplicity_0_5_issuance_token_amount(frameItem* dst, frameItem src, const txEnv* env);
bool rustsimplicity_0_5_issuance_asset_proof(frameItem* dst, frameItem src, const txEnv* env);
bool rustsimplicity_0_5_issuance_token_proof(frameItem* dst, frameItem src, const txEnv* env);
bool rustsimplicity_0_5_output_asset(frameItem* dst, frameItem src, const txEnv* env);
bool rustsimplicity_0_5_output_amount(frameItem* dst, frameItem src, const txEnv* env);
bool rustsimplicity_0_5_output_nonce(frameItem* dst, frameItem src, const txEnv* env);
bool rustsimplicity_0_5_output_script_hash(frameItem* dst, frameItem src, const txEnv* env);
bool rustsimplicity_0_5_output_null_datum(frameItem* dst, frameItem src, const txEnv* env);
bool rustsimplicity_0_5_output_is_fee(frameItem* dst, frameItem src, const txEnv* env);
bool rustsimplicity_0_5_output_surjection_proof(frameItem* dst, frameItem src, const txEnv* env);
bool rustsimplicity_0_5_output_range_proof(frameItem* dst, frameItem src, const txEnv* env);
bool rustsimplicity_0_5_total_fee(frameItem* dst, frameItem src, const txEnv* env);
bool rustsimplicity_0_5_genesis_block_hash(frameItem* dst, frameItem src, const txEnv* env);
bool rustsimplicity_0_5_script_cmr(frameItem* dst, frameItem src, const txEnv* env);
bool rustsimplicity_0_5_transaction_id(frameItem* dst, frameItem src, const txEnv* env);
bool rustsimplicity_0_5_current_index(frameItem* dst, frameItem src, const txEnv* env);
bool rustsimplicity_0_5_current_pegin(frameItem* dst, frameItem src, const txEnv* env);
bool rustsimplicity_0_5_current_prev_outpoint(frameItem* dst, frameItem src, const txEnv* env);
bool rustsimplicity_0_5_current_asset(frameItem* dst, frameItem src, const txEnv* env);
bool rustsimplicity_0_5_current_amount(frameItem* dst, frameItem src, const txEnv* env);
bool rustsimplicity_0_5_current_script_hash(frameItem* dst, frameItem src, const txEnv* env);
bool rustsimplicity_0_5_current_sequence(frameItem* dst, frameItem src, const txEnv* env);
bool rustsimplicity_0_5_current_reissuance_blinding(frameItem* dst, frameItem src, const txEnv* env);
bool rustsimplicity_0_5_current_new_issuance_contract(frameItem* dst, frameItem src, const txEnv* env);
bool rustsimplicity_0_5_current_reissuance_entropy(frameItem* dst, frameItem src, const txEnv* env);
bool rustsimplicity_0_5_current_issuance_asset_amount(frameItem* dst, frameItem src, const txEnv* env);
bool rustsimplicity_0_5_current_issuance_token_amount(frameItem* dst, frameItem src, const txEnv* env);
bool rustsimplicity_0_5_current_issuance_asset_proof(frameItem* dst, frameItem src, const txEnv* env);
bool rustsimplicity_0_5_current_issuance_token_proof(frameItem* dst, frameItem src, const txEnv* env);
bool rustsimplicity_0_5_current_annex_hash(frameItem* dst, frameItem src, const txEnv* env);
bool rustsimplicity_0_5_current_script_sig_hash(frameItem* dst, frameItem src, const txEnv* env);
bool rustsimplicity_0_5_tapleaf_version(frameItem* dst, frameItem src, const txEnv* env);
bool rustsimplicity_0_5_tappath(frameItem* dst, frameItem src, const txEnv* env);
bool rustsimplicity_0_5_internal_key(frameItem* dst, frameItem src, const txEnv* env);
bool rustsimplicity_0_5_num_inputs(frameItem* dst, frameItem src, const txEnv* env);
bool rustsimplicity_0_5_num_outputs(frameItem* dst, frameItem src, const txEnv* env);
bool rustsimplicity_0_5_tx_is_final(frameItem* dst, frameItem src, const txEnv* env);
bool rustsimplicity_0_5_tx_lock_height(frameItem* dst, frameItem src, const txEnv* env);
bool rustsimplicity_0_5_tx_lock_time(frameItem* dst, frameItem src, const txEnv* env);
bool rustsimplicity_0_5_tx_lock_distance(frameItem* dst, frameItem src, const txEnv* env);
bool rustsimplicity_0_5_tx_lock_duration(frameItem* dst, frameItem src, const txEnv* env);
bool rustsimplicity_0_5_check_lock_height(frameItem* dst, frameItem src, const txEnv* env);
bool rustsimplicity_0_5_check_lock_time(frameItem* dst, frameItem src, const txEnv* env);
bool rustsimplicity_0_5_check_lock_distance(frameItem* dst, frameItem src, const txEnv* env);
bool rustsimplicity_0_5_check_lock_duration(frameItem* dst, frameItem src, const txEnv* env);
bool rustsimplicity_0_5_calculate_issuance_entropy(frameItem* dst, frameItem src, const txEnv* env);
bool rustsimplicity_0_5_calculate_asset(frameItem* dst, frameItem src, const txEnv* env);
bool rustsimplicity_0_5_calculate_explicit_token(frameItem* dst, frameItem src, const txEnv* env);
bool rustsimplicity_0_5_calculate_confidential_token(frameItem* dst, frameItem src, const txEnv* env);
bool rustsimplicity_0_5_lbtc_asset(frameItem* dst, frameItem src, const txEnv* env);
bool rustsimplicity_0_5_build_tapleaf_simplicity(frameItem* dst, frameItem src, const txEnv* env);
bool rustsimplicity_0_5_build_tapbranch(frameItem* dst, frameItem src, const txEnv* env);
bool rustsimplicity_0_5_build_taptweak(frameItem* dst, frameItem src, const txEnv* env);
bool rustsimplicity_0_5_outpoint_hash(frameItem* dst, frameItem src, const txEnv* env);
bool rustsimplicity_0_5_asset_amount_hash(frameItem* dst, frameItem src, const txEnv* env);
bool rustsimplicity_0_5_nonce_hash(frameItem* dst, frameItem src, const txEnv* env);
bool rustsimplicity_0_5_annex_hash(frameItem* dst, frameItem src, const txEnv* env);
bool rustsimplicity_0_5_issuance(frameItem* dst, frameItem src, const txEnv* env);
bool rustsimplicity_0_5_issuance_entropy(frameItem* dst, frameItem src, const txEnv* env);
bool rustsimplicity_0_5_issuance_asset(frameItem* dst, frameItem src, const txEnv* env);
bool rustsimplicity_0_5_issuance_token(frameItem* dst, frameItem src, const txEnv* env);
bool rustsimplicity_0_5_output_amounts_hash(frameItem* dst, frameItem src, const txEnv* env);
bool rustsimplicity_0_5_output_nonces_hash(frameItem* dst, frameItem src, const txEnv* env);
bool rustsimplicity_0_5_output_scripts_hash(frameItem* dst, frameItem src, const txEnv* env);
bool rustsimplicity_0_5_output_range_proofs_hash(frameItem* dst, frameItem src, const txEnv* env);
bool rustsimplicity_0_5_output_surjection_proofs_hash(frameItem* dst, frameItem src, const txEnv* env);
bool rustsimplicity_0_5_outputs_hash(frameItem* dst, frameItem src, const txEnv* env);
bool rustsimplicity_0_5_output_hash(frameItem* dst, frameItem src, const txEnv* env);
bool rustsimplicity_0_5_input_outpoints_hash(frameItem* dst, frameItem src, const txEnv* env);
bool rustsimplicity_0_5_input_amounts_hash(frameItem* dst, frameItem src, const txEnv* env);
bool rustsimplicity_0_5_input_scripts_hash(frameItem* dst, frameItem src, const txEnv* env);
bool rustsimplicity_0_5_input_utxos_hash(frameItem* dst, frameItem src, const txEnv* env);
bool rustsimplicity_0_5_input_utxo_hash(frameItem* dst, frameItem src, const txEnv* env);
bool rustsimplicity_0_5_input_sequences_hash(frameItem* dst, frameItem src, const txEnv* env);
bool rustsimplicity_0_5_input_annexes_hash(frameItem* dst, frameItem src, const txEnv* env);
bool rustsimplicity_0_5_input_script_sigs_hash(frameItem* dst, frameItem src, const txEnv* env);
bool rustsimplicity_0_5_inputs_hash(frameItem* dst, frameItem src, const txEnv* env);
bool rustsimplicity_0_5_input_hash(frameItem* dst, frameItem src, const txEnv* env);
bool rustsimplicity_0_5_issuance_asset_amounts_hash(frameItem* dst, frameItem src, const txEnv* env);
bool rustsimplicity_0_5_issuance_token_amounts_hash(frameItem* dst, frameItem src, const txEnv* env);
bool rustsimplicity_0_5_issuance_range_proofs_hash(frameItem* dst, frameItem src, const txEnv* env);
bool rustsimplicity_0_5_issuance_blinding_entropy_hash(frameItem* dst, frameItem src, const txEnv* env);
bool rustsimplicity_0_5_issuances_hash(frameItem* dst, frameItem src, const txEnv* env);
bool rustsimplicity_0_5_issuance_hash(frameItem* dst, frameItem src, const txEnv* env);
bool rustsimplicity_0_5_tx_hash(frameItem* dst, frameItem src, const txEnv* env);
bool rustsimplicity_0_5_tapleaf_hash(frameItem* dst, frameItem src, const txEnv* env);
bool rustsimplicity_0_5_tappath_hash(frameItem* dst, frameItem src, const txEnv* env);
bool rustsimplicity_0_5_tap_env_hash(frameItem* dst, frameItem src, const txEnv* env);
bool rustsimplicity_0_5_sig_all_hash(frameItem* dst, frameItem src, const txEnv* env);

#endif
