// SPDX-License-Identifier: CC0-1.0

//! Tests that the Bitcoin backend executes tx-context jets through the C implementations,
//! using a real transaction environment.

use std::sync::Arc;

use bitcoin::absolute::LockTime;
use bitcoin::hashes::{sha256, Hash as _};
use bitcoin::key::Parity;
use bitcoin::taproot::{ControlBlock, LeafVersion, TapNodeHash};
use bitcoin::transaction::Version;
use bitcoin::{Amount, OutPoint, ScriptBuf, Sequence, Transaction, TxIn, TxOut, Txid, Witness};

use crate::bit_machine::ExecutionError;
use crate::jet::{Bitcoin, BitcoinEnv};
use crate::node::{ConstructNode, CoreConstructible as _};
use crate::types::{self, Final};
use crate::{BitMachine, Cmr, Value};

type Env = BitcoinEnv<Transaction>;

/// The bytes `start, start + 1, ..`: a hash whose byte order matters.
fn bytes32(start: u8) -> [u8; 32] {
    core::array::from_fn(|i| start + i as u8)
}

/// The parts of a spending context.
/// A test changes the field it is about and then calls [`Fixture::env`].
struct Fixture {
    tx: Transaction,
    /// The UTXO spent by each input of `tx`.
    utxos: Vec<TxOut>,
    ix: u32,
    cmr: Cmr,
    control_block: ControlBlock,
}

impl Fixture {
    /// A spend of the middle one of three inputs, with two outputs.
    ///
    /// The inputs, the UTXOs they spend and the outputs all differ,
    /// and no two jets of the same type read the same value (hence version 4),
    /// so that nothing can be mistaken for something else.
    ///
    /// Only the spent input has a final sequence,
    /// so the transaction as a whole is not final and its lock time applies.
    fn new() -> Self {
        let txin = |txid, vout, sequence| TxIn {
            previous_output: OutPoint::new(Txid::from_byte_array(bytes32(txid)), vout),
            sequence,
            ..TxIn::default()
        };
        let txout = |sat, script_pubkey: &[u8]| TxOut {
            value: Amount::from_sat(sat),
            script_pubkey: ScriptBuf::from(script_pubkey.to_vec()),
        };

        Fixture {
            tx: Transaction {
                version: Version(4),
                lock_time: LockTime::from_consensus(700_000),
                input: vec![
                    txin(0x10, 2, Sequence::ENABLE_LOCKTIME_NO_RBF),
                    txin(0x20, 0, Sequence::MAX),
                    txin(0x30, 1, Sequence::ENABLE_RBF_NO_LOCKTIME),
                ],
                output: vec![txout(25_000, &[0x6a]), txout(34_000, &[0x6a; 2])],
            },
            utxos: vec![
                txout(30_000, &[0x51]),
                txout(10_000, &[0x52; 2]),
                txout(20_000, &[0x53; 3]),
            ],
            ix: 1,
            cmr: Cmr::from_byte_array(bytes32(0xc0)),
            control_block: ControlBlock {
                leaf_version: LeafVersion::from_consensus(0xbe).unwrap(),
                output_key_parity: Parity::Odd,
                internal_key: INTERNAL_KEY.parse().unwrap(),
                merkle_branch: [bytes32(0x70), bytes32(0x90)]
                    .map(TapNodeHash::from_byte_array)
                    .into(),
            },
        }
    }

    fn env(&self) -> Env {
        BitcoinEnv::new(
            self.tx.clone(),
            &self.utxos,
            self.ix,
            self.cmr,
            self.control_block.clone(),
        )
    }
}

/// The BIP-341 NUMS point.
const INTERNAL_KEY: &str = "50929b74c1a04954b78b4b6035e97a5e078a5a0f28ec96d547bfee9ace803ac0";

// The environment hashes of `Fixture::new()`.
const TX_HASH: &str = "1032104862fdcc0a976cad6da0fbb47ddd67930a1f83cc7d69a4d5cf6addea69";
const TAP_ENV_HASH: &str = "c744c712ef26db826c3904d7471bf4d5d1a53fdf7c663a9a5e4d681026f6c06f";
const SIG_ALL_HASH: &str = "c65d5cff65c29c1417783930350af2045e0ec92d6dfa7f0ff02a4826453c7c8b";

/// Runs `jet` on `input` against `env`.
fn run_jet(jet: Bitcoin, input: &Value, env: &Env) -> Result<Value, ExecutionError> {
    types::Context::with_context(|ctx| {
        let prog = Arc::<ConstructNode>::comp(
            &Arc::<ConstructNode>::scribe(&ctx, input),
            &Arc::<ConstructNode>::jet(&ctx, &jet),
        )
        .unwrap();
        BitMachine::test_exec(prog, env)
    })
}

#[test]
fn jets_read_the_environment() {
    let fixture = Fixture::new();
    let env = fixture.env();
    let txid = fixture.tx.compute_txid().to_byte_array();

    for (jet, expected) in [
        (Bitcoin::Version, Value::u32(4)),
        (Bitcoin::LockTime, Value::u32(700_000)),
        (Bitcoin::NumInputs, Value::u32(3)),
        (Bitcoin::NumOutputs, Value::u32(2)),
        (Bitcoin::CurrentIndex, Value::u32(1)),
        (Bitcoin::CurrentValue, Value::u64(10_000)),
        (Bitcoin::ScriptCMR, Value::u256(bytes32(0xc0))),
        (Bitcoin::TransactionId, Value::u256(txid)),
    ] {
        let value = run_jet(jet, &Value::unit(), &env).unwrap();
        assert_eq!(value, expected, "{jet:?}");
    }

    // The indexed jets return the value at each index, and nothing past the end.
    for (jet, sats) in [
        (Bitcoin::InputValue, [30_000, 10_000, 20_000].as_slice()),
        (Bitcoin::OutputValue, [25_000, 34_000].as_slice()),
    ] {
        let mut expected = Vec::from_iter(sats.iter().map(|&sat| Value::some(Value::u64(sat))));
        expected.push(Value::none(Final::u64()));

        for (i, expected) in expected.into_iter().enumerate() {
            let value = run_jet(jet, &Value::u32(i as u32), &env).unwrap();
            assert_eq!(value, expected, "{jet:?} {i}");
        }
    }
}

#[test]
fn env_hashes_match_independent_digests() {
    let env = Fixture::new().env();

    for (jet, expected) in [
        (Bitcoin::TxHash, TX_HASH),
        (Bitcoin::TapEnvHash, TAP_ENV_HASH),
        (Bitcoin::SigAllHash, SIG_ALL_HASH),
    ] {
        let expected: [u8; 32] = hex::decode_to_array(expected).unwrap();
        let value = run_jet(jet, &Value::unit(), &env).unwrap();
        assert_eq!(value, Value::u256(expected), "{jet:?}");
    }
}

/// `check_lock_height` and `check_lock_time` accept a 32-bit value iff it is at
/// most the transaction's lock height or lock time.
///
/// A lock time below 500,000,000 is a height and any other one is a timestamp.
/// The one that does not apply reads as 0.
/// Both read as 0 when the transaction is final, i.e. when every input has sequence `0xffffffff`.
/// Finality is a property of the whole transaction, not of the input being spent.
#[test]
fn check_lock_height_and_time() {
    // (lock time, whether every input is final, lock height, lock time as the jets read it)
    for (lock_time, all_final, height, time) in [
        (5, false, 5, 0),
        (5, true, 0, 0),
        // The last lock time that is a height, and the first one that is a timestamp.
        (499_999_999, false, 499_999_999, 0),
        (500_000_000, false, 0, 500_000_000),
        (500_000_000, true, 0, 0),
    ] {
        let mut fixture = Fixture::new();
        fixture.tx.lock_time = LockTime::from_consensus(lock_time);
        if all_final {
            for input in &mut fixture.tx.input {
                input.sequence = Sequence::MAX;
            }
        }
        let env = fixture.env();

        // `LockTime` returns the field itself, whether or not it applies.
        let raw = run_jet(Bitcoin::LockTime, &Value::unit(), &env).unwrap();
        assert_eq!(raw, Value::u32(lock_time), "{lock_time} {all_final}");

        for (jet, limit) in [
            (Bitcoin::CheckLockHeight, height),
            (Bitcoin::CheckLockTime, time),
        ] {
            let accepts = |x| run_jet(jet, &Value::u32(x), &env).is_ok();
            let case = (jet, lock_time, all_final);
            assert!(accepts(0) && accepts(limit), "{case:?}");
            assert!(!accepts(limit + 1), "{case:?}");
        }
    }
}

#[test]
fn annex_jets_hash_the_annex_after_its_0x50() {
    let annex_hash = |annex: Option<&[u8]>| match annex {
        Some(annex) => Value::some(Value::u256(sha256::Hash::hash(annex).to_byte_array())),
        None => Value::none(Final::u256()),
    };

    // (witness of the current input, its annex without the 0x50)
    let cases: &[(&[&[u8]], Option<&[u8]>)] = &[
        (&[], None),
        // A single item is a key-path signature, even if it starts with 0x50
        (&[&[0x50, 0xaa]], None),
        (&[&[0x01], &[0x51, 0xaa]], None),
        (&[&[0x01], &[]], None),
        // Only the last item can be the annex
        (&[&[0x50, 0xaa], &[0x01]], None),
        // A key-path and a script-path spend with the same annex
        (&[&[0x01], &[0x50, 0xaa]], Some(&[0xaa])),
        (&[&[0x01], &[0x02], &[0x50, 0xaa]], Some(&[0xaa])),
        // 0x50 alone is an empty annex, not a missing one
        (&[&[0x01], &[0x50]], Some(&[])),
    ];

    for &(witness, annex) in cases {
        let mut fixture = Fixture::new();
        // Another input's annex, which must stay with that input
        fixture.tx.input[0].witness = Witness::from_slice(&[vec![0x01], vec![0x50, 0xcc]]);
        fixture.tx.input[1].witness = Witness::from_slice(witness);
        let env = fixture.env();

        let value = run_jet(Bitcoin::CurrentAnnexHash, &Value::unit(), &env).unwrap();
        assert_eq!(value, annex_hash(annex), "{witness:?}");

        for (i, annex) in [Some(&[0xcc][..]), annex, None].into_iter().enumerate() {
            let value = run_jet(Bitcoin::InputAnnexHash, &Value::u32(i as u32), &env).unwrap();
            assert_eq!(value, Value::some(annex_hash(annex)), "{witness:?} {i}");
        }
    }
}
