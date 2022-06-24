use crate::encode::BitWriter;
use crate::jet::application::Elements;
use crate::jet::elements::ElementsJetName;
use crate::jet::JetNode;
use std::io::Write;

/// Encode an Elements jet primitive to bits
pub(super) fn encode_primitive<W: Write>(
    jet: &JetNode<Elements>,
    w: &mut BitWriter<W>,
) -> std::io::Result<usize> {
    match jet.name {
        ElementsJetName::Version => w.write_bits_be(128 + 0, 8),
        ElementsJetName::LockTime => w.write_bits_be(128 + 1, 8),
        ElementsJetName::InputIsPegin => w.write_bits_be(64 + 1, 7),
        ElementsJetName::InputPrevOutpoint => w.write_bits_be(64 + 2, 7),
        ElementsJetName::InputAsset => w.write_bits_be(64 + 3, 7),
        ElementsJetName::InputAmount => w.write_bits_be(128 + 2, 8),
        ElementsJetName::InputScriptHash => w.write_bits_be(128 + 3, 8),
        ElementsJetName::InputSequence => w.write_bits_be(64 + 5, 7),
        ElementsJetName::InputIssuanceBlinding => w.write_bits_be(64 + 6, 7),
        ElementsJetName::InputIssuanceContract => w.write_bits_be(64 + 7, 7),
        ElementsJetName::InputIssuanceEntropy => w.write_bits_be(128 + 4, 8),
        ElementsJetName::InputIssuanceAssetAmount => w.write_bits_be(128 + 5, 8),
        ElementsJetName::InputIssuanceTokenAmount => w.write_bits_be(64 + 9, 7),
        ElementsJetName::OutputAsset => w.write_bits_be(64 + 10, 7),
        ElementsJetName::OutputAmount => w.write_bits_be(64 + 11, 7),
        ElementsJetName::OutputNonce => w.write_bits_be(128 + 6, 8),
        ElementsJetName::OutputScriptHash => w.write_bits_be(128 + 7, 8),
        ElementsJetName::OutputNullDatum => w.write_bits_be(64 + 13, 7),
        ElementsJetName::ScriptCmr => w.write_bits_be(64 + 14, 7),
        ElementsJetName::CurrentIndex => w.write_bits_be(64 + 15, 7),
        ElementsJetName::CurrentIsPegin => w.write_bits_be(64 + 16, 7),
        ElementsJetName::CurrentPrevOutpoint => w.write_bits_be(64 + 17, 7),
        ElementsJetName::CurrentAsset => w.write_bits_be(64 + 18, 7),
        ElementsJetName::CurrentAmount => w.write_bits_be(64 + 19, 7),
        ElementsJetName::CurrentScriptHash => w.write_bits_be(64 + 20, 7),
        ElementsJetName::CurrentSequence => w.write_bits_be(64 + 21, 7),
        ElementsJetName::CurrentIssuanceBlinding => w.write_bits_be(64 + 22, 7),
        ElementsJetName::CurrentIssuanceContract => w.write_bits_be(64 + 23, 7),
        ElementsJetName::CurrentIssuanceEntropy => w.write_bits_be(64 + 24, 7),
        ElementsJetName::CurrentIssuanceAssetAmount => w.write_bits_be(64 + 25, 7),
        ElementsJetName::CurrentIssuanceTokenAmount => w.write_bits_be(64 + 26, 7),
        ElementsJetName::InputsHash => w.write_bits_be(64 + 27, 7),
        ElementsJetName::OutputsHash => w.write_bits_be(64 + 28, 7),
        ElementsJetName::NumInputs => w.write_bits_be(64 + 29, 7),
        ElementsJetName::NumOutputs => w.write_bits_be(64 + 30, 7),
        ElementsJetName::Fee => w.write_bits_be(64 + 31, 7),
        _ => unimplemented!("Encode Elements primitive"),
    }
}
