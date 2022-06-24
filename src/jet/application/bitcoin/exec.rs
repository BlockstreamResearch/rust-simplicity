use crate::exec::BitMachine;
use crate::jet::application::bitcoin::environment::BitcoinEnv;
use crate::jet::application::bitcoin::BitcoinError;
use crate::jet::application::{Bitcoin, Core};
use crate::jet::bitcoin::BitcoinJetName;
use crate::jet::{Application, JetNode};
use bitcoin_hashes::{sha256, Hash, HashEngine};
use byteorder::{LittleEndian, WriteBytesExt};
use std::convert::TryInto;

pub fn exec_jet(
    jet: &JetNode<Bitcoin>,
    mac: &mut BitMachine,
    env: &BitcoinEnv,
) -> Result<(), BitcoinError> {
    // FIXME finish this
    match jet.name {
        BitcoinJetName::Version => mac.write_u32(env.tx.version as u32),
        BitcoinJetName::LockTime => mac.write_u32(env.tx.lock_time),
        BitcoinJetName::InputsHash => {
            let mut eng = sha256::Hash::engine();
            for input in &env.tx.input {
                eng.input(&input.previous_output.txid[..]);
                eng.write_u32::<LittleEndian>(input.previous_output.vout)
                    .unwrap();
                eng.write_u64::<LittleEndian>(99_998_000).unwrap(); // value FIXME
                eng.write_u32::<LittleEndian>(input.sequence).unwrap();
            }
            mac.write_bytes(&sha256::Hash::from_engine(eng)[..]);
        }
        BitcoinJetName::OutputsHash => {
            let mut eng = sha256::Hash::engine();
            for output in &env.tx.output {
                eng.write_u64::<LittleEndian>(output.value).unwrap();
                eng.input(&sha256::Hash::hash(&output.script_pubkey[..]));
            }
            mac.write_bytes(&sha256::Hash::from_engine(eng)[..]);
        }
        // FIXME don't hardcode this
        BitcoinJetName::CurrentValue => {
            mac.write_u64(99_998_000);
        }
        BitcoinJetName::CurrentIndex => {
            mac.write_u32(0);
        }
        BitcoinJetName::CurrentSequence
        | BitcoinJetName::InputPrevOutpoint
        | BitcoinJetName::InputValue
        | BitcoinJetName::InputSequence
        | BitcoinJetName::NumOutputs
        | BitcoinJetName::TotalOutputValue
        | BitcoinJetName::OutputValue
        | BitcoinJetName::OutputScriptHash
        | BitcoinJetName::NumInputs
        | BitcoinJetName::TotalInputValue
        | BitcoinJetName::CurrentPrevOutpoint
        | BitcoinJetName::ScriptCMR => {
            unimplemented!("Execute Bitcoin primitive")
        }
        x => {
            let core_jet: &'static JetNode<Core> =
                x.try_into().expect("Convert Bitcoin jet into Core jet");
            Core::exec_jet(core_jet, mac, &())?;
        }
    }

    Ok(())
}
