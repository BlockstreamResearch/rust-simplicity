// SPDX-License-Identifier: CC0-1.0

#[cfg(feature = "bitcoin")]
mod tests {
    use simplicity::jet::Bitcoin;
    use simplicity::jet::BitcoinEnv;
    use simplicity::jet::Jet;
    use simplicity::node::{ConstructNode, JetConstructible, CoreConstructible, SimpleFinalizer};
    use simplicity::types;
    use simplicity::{BitMachine, Value, Word};
    use std::sync::Arc;
    use bitcoin::{Transaction, TxIn, TxOut, Amount, ScriptBuf};
    use bitcoin::absolute::LockTime;

    #[test]
    fn test_cmrs() {
        // These CMRs are retrieved from simplicity-sys/depend/simplicity/bitcoin/primitiveJetNode.inc
        // Format: [u32; 8] -> [u8; 32] (Big Endian)
        let cases = vec![
            (Bitcoin::NumInputs, [0x5c5ac4ffu32, 0x6da56cb3u32, 0x72b23266u32, 0x6e8334b9u32, 0xe2cfb0dcu32, 0xb418f161u32, 0xbff149e8u32, 0x4ec92c3eu32]),
            (Bitcoin::NumOutputs, [0x98a1cca7u32, 0x05dfcfafu32, 0xd3a69e9au32, 0xdc05ba47u32, 0xe1fefa6au32, 0x29f34286u32, 0x2048e496u32, 0x8648c3d7u32]),
            (Bitcoin::InputValue, [0x7d3c3f95u32, 0x5b2cf0d0u32, 0xd1280a1bu32, 0xb1204692u32, 0x92d1329cu32, 0x83a9c2ffu32, 0x7e7e1eb3u32, 0xf69783a3u32]),
            (Bitcoin::InputUtxosHash, [0xd6f90cd1u32, 0x04e1a5c6u32, 0x1a4b5000u32, 0xad9aba8du32, 0x43004bf9u32, 0x43df325fu32, 0xa636d1a2u32, 0x2beca0cbu32]),
        ];

        for (jet, expected) in cases {
            let cmr = jet.cmr();
            let mut expected_bytes = [0u8; 32];
            for i in 0..8 {
                expected_bytes[i*4..i*4+4].copy_from_slice(&(expected[i] as u32).to_be_bytes());
            }
            assert_eq!(cmr.to_byte_array(), expected_bytes, "CMR mismatch for {:?}", jet);
        }
    }

    fn execute_jet(program: Arc<ConstructNode<Bitcoin>>, env: &BitcoinEnv) -> Value {
        let prog = program
            .finalize_types_non_program()
            .expect("finalizing types")
            .finalize(&mut SimpleFinalizer::new(std::iter::empty()))
            .expect("finalizing");
        let mut mac = BitMachine::for_program(&prog).expect("program has reasonable bounds");
        mac.exec(&prog, env).expect("executing")
    }

    #[test]
    fn test_execution() {
        let tx = Transaction {
            version: bitcoin::transaction::Version::TWO,
            lock_time: LockTime::ZERO,
            input: vec![
                TxIn::default(),
                TxIn::default(),
            ],
            output: vec![
                TxOut { value: Amount::from_sat(100), script_pubkey: ScriptBuf::new() },
                TxOut { value: Amount::from_sat(200), script_pubkey: ScriptBuf::new() },
                TxOut { value: Amount::from_sat(300), script_pubkey: ScriptBuf::new() },
            ],
        };

        let spent_outputs = vec![
            TxOut { value: Amount::from_sat(1000), script_pubkey: ScriptBuf::new() },
            TxOut { value: Amount::from_sat(2000), script_pubkey: ScriptBuf::new() },
        ];
        let env = BitcoinEnv::new(tx, spent_outputs, 0);

        types::Context::with_context(|ctx| {
            // 1. Test NumInputs
            let prog = Arc::<ConstructNode<Bitcoin>>::jet(&ctx, Bitcoin::NumInputs);
            assert_eq!(
                execute_jet(prog, &env),
                Value::u32(2),
            );

            // 2. Test NumOutputs
            let prog = Arc::<ConstructNode<Bitcoin>>::jet(&ctx, Bitcoin::NumOutputs);
            assert_eq!(
                execute_jet(prog, &env),
                Value::u32(3),
            );

            // 3. Test InputValue (Input 0)
            let input_val_node = Arc::<ConstructNode<_>>::comp(
                &Arc::<ConstructNode<_>>::const_word(&ctx, Word::u32(0)),
                &Arc::<ConstructNode<_>>::jet(&ctx, Bitcoin::InputValue),
            ).unwrap();
            
            // This will fail or panic until implemented.
            let _ = execute_jet(input_val_node, &env);
        });
    }
}
