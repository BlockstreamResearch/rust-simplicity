use crate::bitwriter::BitWriter;
use crate::jet::application::Bitcoin;
use crate::jet::bitcoin::BitcoinJetName;
use crate::jet::JetNode;
use std::io::Write;

/// Encode a Bitcoin jet primitive to bits
pub(super) fn encode_primitive<W: Write>(
    jet: &JetNode<Bitcoin>,
    w: &mut BitWriter<W>,
) -> std::io::Result<usize> {
    match jet.name {
        BitcoinJetName::Version => w.write_bits_be(64 + 0, 7),
        BitcoinJetName::LockTime => w.write_bits_be(64 + 1, 7),
        BitcoinJetName::InputsHash => w.write_bits_be(32 + 1, 6),
        BitcoinJetName::OutputsHash => w.write_bits_be(32 + 2, 6),
        BitcoinJetName::NumInputs => w.write_bits_be(32 + 3, 6),
        BitcoinJetName::TotalInputValue => w.write_bits_be(32 + 4, 6),
        BitcoinJetName::CurrentPrevOutpoint => w.write_bits_be(32 + 5, 6),
        BitcoinJetName::CurrentValue => w.write_bits_be(32 + 6, 6),
        BitcoinJetName::CurrentSequence => w.write_bits_be(32 + 7, 6),
        BitcoinJetName::CurrentIndex => w.write_bits_be(64 + 16, 7),
        BitcoinJetName::InputPrevOutpoint => w.write_bits_be(64 + 17, 7),
        BitcoinJetName::InputValue => w.write_bits_be(32 + 9, 6),
        BitcoinJetName::InputSequence => w.write_bits_be(32 + 10, 6),
        BitcoinJetName::NumOutputs => w.write_bits_be(32 + 11, 6),
        BitcoinJetName::TotalOutputValue => w.write_bits_be(32 + 12, 6),
        BitcoinJetName::OutputValue => w.write_bits_be(32 + 13, 6),
        BitcoinJetName::OutputScriptHash => w.write_bits_be(32 + 14, 6),
        BitcoinJetName::ScriptCMR => w.write_bits_be(32 + 15, 6),
        _ => unimplemented!("Encode Bitcoin primitive"),
    }
}
