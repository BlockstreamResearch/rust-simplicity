/* This file has been automatically generated. */

use crate::bititer::BitIter;
use crate::bitwriter::BitWriter;
use crate::jet::elements::ElementsEnv;
use crate::jet::type_name::TypeName;
use crate::jet::Jet;
use crate::merkle::cmr::Cmr;
use crate::{decode_bits, Error};
use bitcoin_hashes::sha256::Midstate;
use simplicity_sys::c_jets::CTxEnv;
use simplicity_sys::CFrameItem;
use std::io::Write;

/// Elements jet family
#[derive(Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Debug, Hash)]
pub enum Elements {
    Version,
    LockTime,
    InputIsPegin,
    InputPrevOutpoint,
    InputAsset,
    InputAmount,
    InputScriptHash,
    InputSequence,
    InputIssuanceBlinding,
    InputIssuanceContract,
    InputIssuanceEntropy,
    InputIssuanceAssetAmount,
    InputIssuanceTokenAmount,
    OutputAsset,
    OutputAmount,
    OutputNonce,
    OutputScriptHash,
    OutputNullDatum,
    ScriptCmr,
    CurrentIndex,
    CurrentIsPegin,
    CurrentPrevOutpoint,
    CurrentAsset,
    CurrentAmount,
    CurrentScriptHash,
    CurrentSequence,
    CurrentIssuanceBlinding,
    CurrentIssuanceContract,
    CurrentIssuanceEntropy,
    CurrentIssuanceAssetAmount,
    CurrentIssuanceTokenAmount,
    InputsHash,
    OutputsHash,
    NumInputs,
    NumOutputs,
    Fee,
    Add32,
    FullAdd32,
    Sub32,
    FullSub32,
    Mul32,
    FullMul32,
    Eq32Verify,
    Eq256Verify,
    Lt32Verify,
    Sha256,
    Sha256Block,
    Bip0340Verify,
}

impl Jet for Elements {
    type Environment = ElementsEnv;

    fn cmr(&self) -> Cmr {
        let bytes = match self {
            Elements::Version => [
                0x00, 0xb1, 0x45, 0x0b, 0x95, 0x57, 0xb2, 0xf6, 0xf6, 0x81, 0x85, 0x48, 0x10, 0x49,
                0x9f, 0xd1, 0xb9, 0x2f, 0x08, 0x5a, 0x61, 0xfa, 0x44, 0x63, 0x4a, 0xd8, 0xd6, 0x4e,
                0x62, 0xda, 0xf4, 0x93,
            ],
            Elements::LockTime => [
                0xa6, 0x9a, 0x61, 0x3b, 0xef, 0x2c, 0x1d, 0xfa, 0xb4, 0x4e, 0x8f, 0xea, 0x19, 0x6d,
                0x93, 0x5c, 0xfc, 0x41, 0x81, 0xfb, 0x5d, 0xa4, 0x02, 0x57, 0x95, 0xf5, 0x21, 0x0e,
                0x3a, 0x8d, 0x3b, 0xd7,
            ],
            Elements::InputIsPegin => [
                0x21, 0x38, 0x85, 0x64, 0xaf, 0x5c, 0x26, 0x39, 0x7e, 0xd1, 0x72, 0x69, 0xa3, 0x6d,
                0xfa, 0x3e, 0xd6, 0x9c, 0x15, 0x69, 0x6d, 0x12, 0xca, 0x25, 0xbb, 0xdd, 0xb6, 0x4b,
                0xeb, 0x40, 0xad, 0xb3,
            ],
            Elements::InputPrevOutpoint => [
                0x79, 0x71, 0x9d, 0x11, 0x20, 0x5f, 0x1a, 0x1b, 0xb7, 0xbe, 0x3c, 0xe5, 0xbb, 0x2c,
                0x15, 0x5b, 0xa6, 0x3a, 0x10, 0x7e, 0x63, 0x8c, 0x56, 0xf9, 0x73, 0x9d, 0x49, 0xd6,
                0xb8, 0x1d, 0x00, 0x30,
            ],
            Elements::InputAsset => [
                0x5c, 0xa6, 0x34, 0xb3, 0x63, 0x7a, 0x2a, 0x18, 0xd3, 0x45, 0x43, 0xf5, 0x99, 0x7e,
                0x57, 0xd1, 0xa5, 0x9a, 0xb8, 0xe7, 0x26, 0xfc, 0x30, 0x82, 0x08, 0x44, 0x48, 0x61,
                0x48, 0xf5, 0x11, 0x3d,
            ],
            Elements::InputAmount => [
                0x20, 0x2a, 0x92, 0x5f, 0x7c, 0x80, 0x42, 0xe0, 0xa8, 0x80, 0x7a, 0x06, 0x77, 0x73,
                0xf1, 0x46, 0xa2, 0x72, 0xb5, 0xe9, 0x05, 0x7d, 0xf2, 0x27, 0x95, 0xe3, 0x65, 0x4c,
                0xf2, 0x69, 0xb5, 0xef,
            ],
            Elements::InputScriptHash => [
                0xd3, 0x55, 0xb3, 0xbc, 0x46, 0xa7, 0xf2, 0x54, 0x0c, 0xc3, 0xae, 0xb0, 0x4c, 0x70,
                0xd5, 0xbd, 0x97, 0xdc, 0x0a, 0xf3, 0xb8, 0xda, 0x4e, 0xc4, 0x0f, 0xc9, 0x61, 0x43,
                0xea, 0xa1, 0xb7, 0x0b,
            ],
            Elements::InputSequence => [
                0x81, 0xba, 0x00, 0xfc, 0xaf, 0xdf, 0x68, 0x65, 0xf2, 0x7b, 0x66, 0xf3, 0xe8, 0x81,
                0xa0, 0xb6, 0x5d, 0x96, 0x3c, 0x47, 0x8f, 0xc9, 0x9a, 0xb4, 0x3e, 0xb4, 0xeb, 0xcf,
                0xc0, 0x3a, 0xd7, 0x95,
            ],
            Elements::InputIssuanceBlinding => [
                0x1f, 0xf0, 0xa7, 0xad, 0xd4, 0xcb, 0x4c, 0xc9, 0xcc, 0x08, 0x01, 0xdc, 0xd6, 0x7f,
                0xca, 0x38, 0xe8, 0x7a, 0x46, 0x0c, 0x76, 0xb4, 0x45, 0x22, 0x94, 0x10, 0xb7, 0x00,
                0xfa, 0x48, 0x90, 0xef,
            ],
            Elements::InputIssuanceContract => [
                0x6e, 0x95, 0x05, 0x82, 0x40, 0xf5, 0xbb, 0xb6, 0x2f, 0x41, 0xae, 0xf9, 0x4c, 0xf6,
                0xc9, 0x5e, 0x30, 0x83, 0x3e, 0x98, 0x93, 0x7a, 0xa8, 0x57, 0x1e, 0x11, 0x92, 0xed,
                0xf2, 0xca, 0xad, 0xff,
            ],
            Elements::InputIssuanceEntropy => [
                0x2b, 0xcc, 0x0f, 0x48, 0x23, 0x79, 0x32, 0x9a, 0x8e, 0x39, 0x3b, 0x48, 0x5a, 0x7e,
                0xcf, 0xd5, 0xdd, 0x7d, 0x8b, 0x8d, 0x03, 0x1d, 0x94, 0xd9, 0xa3, 0x3e, 0x80, 0x08,
                0x4f, 0x70, 0x6f, 0xa1,
            ],
            Elements::InputIssuanceAssetAmount => [
                0x48, 0x3e, 0x09, 0xfc, 0xe0, 0x7d, 0x20, 0xa6, 0x41, 0x25, 0x97, 0x9d, 0xf3, 0x63,
                0x9f, 0x12, 0x54, 0x60, 0x1f, 0x5a, 0x00, 0x80, 0x2c, 0x1a, 0x49, 0xbd, 0x44, 0xbc,
                0x95, 0xed, 0x70, 0xd5,
            ],
            Elements::InputIssuanceTokenAmount => [
                0xe9, 0x11, 0xef, 0x5a, 0xfa, 0x09, 0xc3, 0x9e, 0x0b, 0x25, 0x61, 0xb0, 0xaf, 0x91,
                0xdc, 0x4e, 0xc6, 0xee, 0x2e, 0xc4, 0x94, 0x1d, 0x76, 0x20, 0x60, 0xfa, 0x04, 0x29,
                0x81, 0x14, 0xd5, 0x52,
            ],
            Elements::OutputAsset => [
                0xb6, 0x95, 0xfb, 0x06, 0x5e, 0xbf, 0x85, 0xb6, 0xc7, 0x6d, 0x05, 0xd1, 0xd4, 0xc7,
                0x82, 0x68, 0x5b, 0x09, 0x4e, 0x22, 0xa9, 0x4f, 0x23, 0x38, 0xdf, 0xad, 0xf6, 0xc4,
                0xbc, 0x98, 0xa9, 0xb1,
            ],
            Elements::OutputAmount => [
                0x73, 0xad, 0xf1, 0xd9, 0x97, 0xbd, 0x82, 0x00, 0x31, 0x05, 0xaf, 0x41, 0x5a, 0x17,
                0x71, 0x90, 0x35, 0xa5, 0x77, 0x42, 0xde, 0x33, 0x93, 0xae, 0x40, 0x82, 0x22, 0x0b,
                0xc5, 0x47, 0xa5, 0x9f,
            ],
            Elements::OutputNonce => [
                0x7a, 0xfb, 0xe9, 0xff, 0x18, 0x85, 0xd9, 0x37, 0x20, 0x71, 0x0a, 0x53, 0x1e, 0x8c,
                0x96, 0xab, 0xbf, 0x26, 0x6a, 0x9c, 0xef, 0x27, 0x0d, 0xe5, 0xea, 0xb8, 0xb9, 0xbc,
                0xe6, 0x82, 0x91, 0xf8,
            ],
            Elements::OutputScriptHash => [
                0xe5, 0x0b, 0x2b, 0xc2, 0x73, 0x9f, 0xea, 0xe2, 0xea, 0xeb, 0x55, 0x2a, 0x9b, 0x34,
                0xfa, 0x7c, 0x71, 0x49, 0xe0, 0xc8, 0x0d, 0xe0, 0xf0, 0x3c, 0xed, 0xf3, 0x62, 0xbb,
                0xa2, 0xa6, 0x76, 0xf0,
            ],
            Elements::OutputNullDatum => [
                0xe2, 0x1a, 0xe0, 0x15, 0x3f, 0x87, 0x87, 0x7f, 0x45, 0xf4, 0x6a, 0x9f, 0xec, 0x22,
                0x84, 0x24, 0x45, 0xd0, 0x82, 0xe2, 0xff, 0x96, 0x12, 0xbd, 0xdb, 0xed, 0xf9, 0x7d,
                0x1c, 0x38, 0x3f, 0x5d,
            ],
            Elements::ScriptCmr => [
                0x9d, 0x16, 0x4f, 0xfd, 0x73, 0xec, 0x16, 0x87, 0xbf, 0x38, 0xa3, 0xf2, 0x2e, 0xfa,
                0xad, 0x10, 0x5e, 0x45, 0xa4, 0x93, 0xd3, 0xb7, 0xfb, 0xf4, 0x3a, 0x09, 0x4d, 0xf0,
                0xf5, 0x0a, 0x29, 0x22,
            ],
            Elements::CurrentIndex => [
                0xfb, 0xaf, 0xe4, 0xb0, 0xa5, 0x8a, 0x12, 0x76, 0xca, 0x02, 0x3c, 0x33, 0x39, 0x15,
                0x1c, 0xf3, 0x50, 0x12, 0x74, 0x72, 0xd8, 0x18, 0xe6, 0x44, 0x77, 0x47, 0x23, 0xcd,
                0xf5, 0x34, 0x3a, 0x81,
            ],
            Elements::CurrentIsPegin => [
                0xd2, 0x5d, 0xbe, 0xef, 0x34, 0xed, 0xf5, 0xbc, 0xd2, 0x37, 0x97, 0x86, 0x06, 0x66,
                0xea, 0x83, 0x96, 0xaa, 0xbc, 0xe4, 0xa4, 0x02, 0x44, 0x76, 0xbb, 0x9d, 0x63, 0x37,
                0xad, 0xc1, 0x23, 0x7d,
            ],
            Elements::CurrentPrevOutpoint => [
                0x2c, 0x7e, 0x49, 0x96, 0x09, 0x14, 0xcb, 0x18, 0x82, 0x46, 0x87, 0xab, 0xc3, 0x16,
                0x5c, 0x16, 0xeb, 0x82, 0xfb, 0x1e, 0x8c, 0xdc, 0x3e, 0x15, 0xf5, 0xc9, 0x6c, 0x0d,
                0x39, 0xb8, 0x26, 0x32,
            ],
            Elements::CurrentAsset => [
                0x91, 0x2b, 0xf8, 0x78, 0x9d, 0xd2, 0x81, 0x8f, 0x97, 0x58, 0xca, 0xc4, 0x2d, 0xd6,
                0xa1, 0x06, 0x5b, 0x9d, 0x4e, 0x28, 0xc3, 0x84, 0x65, 0x8f, 0xc9, 0x67, 0xe4, 0x39,
                0x49, 0xaa, 0xa7, 0x9e,
            ],
            Elements::CurrentAmount => [
                0x35, 0x23, 0xb1, 0xfc, 0x22, 0xdb, 0xbc, 0x9e, 0xaa, 0xc2, 0x6b, 0x9e, 0xea, 0x23,
                0xa8, 0xcd, 0x85, 0xb3, 0xbf, 0x04, 0x56, 0x0a, 0xe0, 0x09, 0xb9, 0x77, 0x72, 0x7c,
                0xc4, 0x7d, 0x2f, 0x2c,
            ],
            Elements::CurrentScriptHash => [
                0x8c, 0x8e, 0xbb, 0xe2, 0x01, 0x8f, 0xf2, 0x06, 0x6a, 0x99, 0x89, 0xe8, 0xfc, 0x5b,
                0x4a, 0xd7, 0x28, 0xff, 0x0f, 0x40, 0x2a, 0x7d, 0x81, 0xa2, 0x70, 0x94, 0x60, 0xf0,
                0x8f, 0xde, 0x3d, 0x8b,
            ],
            Elements::CurrentSequence => [
                0xbc, 0x4b, 0xed, 0xce, 0x1b, 0xc2, 0xd5, 0x94, 0xbc, 0xc5, 0xd8, 0x4f, 0x61, 0xda,
                0xf3, 0x0a, 0x36, 0x1e, 0x50, 0xe3, 0xb4, 0x58, 0x7c, 0x95, 0xbb, 0x88, 0x60, 0xa8,
                0x8b, 0x85, 0xd4, 0xd1,
            ],
            Elements::CurrentIssuanceBlinding => [
                0x72, 0x70, 0xc7, 0x46, 0xce, 0x6e, 0xbd, 0xe4, 0x4c, 0x63, 0xa6, 0x08, 0x6c, 0xc6,
                0x61, 0x23, 0x75, 0xb5, 0xa9, 0xf8, 0x20, 0xf6, 0xc5, 0xb1, 0x59, 0xd2, 0xfa, 0x60,
                0xec, 0xe9, 0x71, 0xd7,
            ],
            Elements::CurrentIssuanceContract => [
                0x66, 0x75, 0x91, 0xb3, 0xdd, 0xad, 0xac, 0xa9, 0xff, 0x85, 0x96, 0xdd, 0xbe, 0x4d,
                0xfd, 0xa3, 0x78, 0xdf, 0x1a, 0xba, 0x7a, 0x9e, 0xe9, 0xf9, 0xee, 0x3d, 0xa2, 0x8d,
                0x7c, 0x01, 0x1c, 0x94,
            ],
            Elements::CurrentIssuanceEntropy => [
                0xfa, 0x0c, 0xf7, 0x68, 0xdf, 0x18, 0x4e, 0xc9, 0x6d, 0x18, 0x13, 0x37, 0x7e, 0xbb,
                0x3d, 0xdc, 0xd1, 0x3d, 0x8a, 0x63, 0xe4, 0x8d, 0x05, 0xde, 0x53, 0xd2, 0xd6, 0x91,
                0x6d, 0x19, 0xd7, 0xe1,
            ],
            Elements::CurrentIssuanceAssetAmount => [
                0x6d, 0x52, 0x0a, 0xb0, 0xa5, 0x77, 0xb4, 0x20, 0x0c, 0xf1, 0x05, 0xff, 0x3f, 0x52,
                0xc4, 0x76, 0x8e, 0x7d, 0x00, 0xc5, 0x3b, 0x6b, 0xe4, 0xb7, 0xea, 0x42, 0xca, 0x85,
                0xf5, 0x99, 0x81, 0x96,
            ],
            Elements::CurrentIssuanceTokenAmount => [
                0x82, 0xc0, 0x18, 0x96, 0x69, 0xa2, 0x11, 0x56, 0x92, 0x41, 0xc0, 0x0c, 0xc4, 0x6f,
                0x34, 0x7b, 0x21, 0x9b, 0x04, 0xda, 0x71, 0x28, 0x0e, 0x54, 0x6b, 0x1d, 0x01, 0xae,
                0xd5, 0xcd, 0xa9, 0xf2,
            ],
            Elements::InputsHash => [
                0x29, 0xbb, 0xe5, 0x6c, 0xec, 0xcf, 0x69, 0x46, 0xe0, 0xe1, 0xc7, 0x45, 0x8e, 0xbf,
                0xdb, 0x4c, 0xca, 0x34, 0x02, 0x64, 0xc4, 0x05, 0x81, 0x82, 0x6c, 0x26, 0x7e, 0x54,
                0xca, 0xec, 0x5d, 0xe2,
            ],
            Elements::OutputsHash => [
                0xc8, 0xaa, 0xe4, 0x68, 0xd7, 0xcb, 0x1d, 0x66, 0xdf, 0x1b, 0x37, 0xa7, 0x78, 0xe6,
                0x67, 0x34, 0xd2, 0x52, 0x56, 0x70, 0xaf, 0x6f, 0xf6, 0xe5, 0x59, 0xca, 0xd4, 0xdf,
                0x83, 0x03, 0x2a, 0xe5,
            ],
            Elements::NumInputs => [
                0x25, 0xe9, 0x21, 0x33, 0xe6, 0xad, 0xf2, 0xa4, 0x4c, 0xf6, 0x62, 0x2b, 0x3a, 0x79,
                0x8d, 0xf5, 0xfe, 0x73, 0xb1, 0x7c, 0xba, 0xf0, 0x75, 0x93, 0xfd, 0x49, 0x06, 0x4f,
                0x4e, 0x91, 0xbc, 0x18,
            ],
            Elements::NumOutputs => [
                0xcd, 0x39, 0x0b, 0x66, 0x79, 0xdc, 0x17, 0x9c, 0x46, 0xb5, 0x40, 0xa7, 0x90, 0x5f,
                0x9e, 0xa7, 0xff, 0x55, 0x23, 0xf9, 0xff, 0xa0, 0x0a, 0xb9, 0x05, 0xcd, 0xe8, 0x24,
                0xde, 0xf9, 0x75, 0x00,
            ],
            Elements::Fee => [
                0xe2, 0x25, 0xdb, 0x38, 0x12, 0xe8, 0xd5, 0xd3, 0xe6, 0x07, 0xa2, 0x17, 0x79, 0xb9,
                0x32, 0xc1, 0x40, 0x52, 0x98, 0xa3, 0x12, 0xa3, 0x47, 0x01, 0xe3, 0xb0, 0xf7, 0x1c,
                0xd4, 0xc9, 0x8c, 0xbc,
            ],
            Elements::Add32 => [
                0x5d, 0x5c, 0x8f, 0xf3, 0x86, 0xd5, 0xa0, 0x14, 0x08, 0xe9, 0xe0, 0x79, 0xed, 0x95,
                0x2c, 0xb9, 0xc1, 0xdc, 0x86, 0x14, 0xfc, 0x1f, 0x3e, 0x54, 0x61, 0xab, 0x1c, 0x30,
                0x24, 0xdc, 0xea, 0x54,
            ],
            Elements::FullAdd32 => [
                0xf0, 0x95, 0x9d, 0x3c, 0xb9, 0x2c, 0x72, 0x8c, 0xd0, 0x86, 0x26, 0x81, 0x71, 0xaa,
                0x1f, 0xdd, 0x5c, 0x97, 0x4c, 0xbe, 0x3f, 0xf6, 0x4a, 0x09, 0x94, 0x13, 0x28, 0x76,
                0x6d, 0x24, 0xbf, 0xf1,
            ],
            Elements::Sub32 => [
                0x01, 0x6d, 0x32, 0x48, 0xee, 0x72, 0x7e, 0xb7, 0x27, 0xc3, 0x3a, 0xa6, 0xf2, 0xcf,
                0xb8, 0xb8, 0x7e, 0x7d, 0x07, 0x46, 0x55, 0x40, 0xdc, 0x3f, 0x9a, 0xb3, 0x22, 0x93,
                0x78, 0x85, 0x2a, 0xc7,
            ],
            Elements::FullSub32 => [
                0x34, 0x73, 0xfa, 0x10, 0xe0, 0xe7, 0xd9, 0x80, 0x2d, 0x53, 0x3b, 0x13, 0x01, 0xb2,
                0x09, 0x83, 0x85, 0x92, 0xb3, 0x1a, 0xf9, 0xd9, 0x14, 0xb0, 0xe7, 0x46, 0x11, 0x32,
                0xf5, 0x3d, 0x79, 0x7a,
            ],
            Elements::Mul32 => [
                0x02, 0x44, 0x52, 0xa5, 0x7a, 0xc5, 0x8c, 0xd0, 0xa1, 0x97, 0x57, 0xbb, 0xf1, 0x68,
                0xa3, 0xa8, 0xcb, 0x6a, 0x02, 0x38, 0xa8, 0x0f, 0x61, 0x81, 0x3e, 0xf7, 0x9c, 0x92,
                0x6c, 0x8f, 0x08, 0x9e,
            ],
            Elements::FullMul32 => [
                0x47, 0xe0, 0xca, 0x35, 0x3a, 0x6f, 0x93, 0x4b, 0xd9, 0x97, 0x5d, 0xfe, 0x04, 0x27,
                0x62, 0x96, 0x42, 0x94, 0xf7, 0x51, 0xd1, 0xd4, 0x6d, 0x39, 0xcf, 0xa5, 0xee, 0x5f,
                0x3a, 0x37, 0x8b, 0xfd,
            ],
            Elements::Eq32Verify => [
                0x32, 0x61, 0x8d, 0x01, 0xfb, 0xfe, 0x81, 0x9f, 0x29, 0x69, 0xb7, 0x1c, 0xda, 0xbf,
                0x40, 0x5d, 0xde, 0x3d, 0xa1, 0x7c, 0x04, 0x45, 0xe8, 0xd0, 0x53, 0x47, 0x65, 0x7c,
                0x5b, 0x53, 0x2f, 0x72,
            ],
            Elements::Eq256Verify => [
                0x7c, 0x1d, 0x68, 0x82, 0xe5, 0x38, 0x22, 0xe8, 0x0c, 0x5d, 0x7d, 0x36, 0xf8, 0x59,
                0xc1, 0xc4, 0x02, 0xfe, 0x29, 0x10, 0xcf, 0xbc, 0xa2, 0x32, 0xc0, 0x67, 0x97, 0x25,
                0x6b, 0xe3, 0xdb, 0x07,
            ],
            Elements::Lt32Verify => [
                0xa1, 0xfa, 0x71, 0x96, 0xbe, 0x58, 0x35, 0x47, 0xcf, 0xb5, 0x15, 0x25, 0xc7, 0x65,
                0x2f, 0xc2, 0x14, 0x0c, 0x70, 0x46, 0xab, 0xab, 0x4a, 0x8c, 0x3a, 0x25, 0x1f, 0x1e,
                0xa3, 0xc3, 0x94, 0xc1,
            ],
            Elements::Sha256 => [
                0xd6, 0x49, 0xd3, 0x03, 0xd1, 0x96, 0xcd, 0x53, 0xfe, 0x29, 0x86, 0xfc, 0x6b, 0x81,
                0x25, 0x08, 0xb5, 0x5d, 0x23, 0xaa, 0xa3, 0x92, 0xf4, 0xf3, 0xa7, 0xd0, 0x8c, 0x6c,
                0xad, 0xb2, 0xf8, 0xac,
            ],
            Elements::Sha256Block => [
                0xd5, 0xb6, 0xf8, 0x48, 0x44, 0x17, 0x32, 0x12, 0xe2, 0x69, 0x9e, 0x99, 0xa8, 0x9b,
                0xcd, 0x3e, 0xb7, 0xf8, 0xe9, 0x6c, 0x0c, 0xc8, 0x46, 0x7f, 0x1c, 0xf2, 0xc2, 0x50,
                0x14, 0x74, 0x59, 0x48,
            ],
            Elements::Bip0340Verify => [
                0xd6, 0xdf, 0x54, 0x94, 0x7e, 0xb9, 0x27, 0x86, 0xb3, 0x79, 0xd3, 0xec, 0x81, 0x93,
                0x99, 0x4a, 0x0f, 0xfe, 0xdd, 0xc2, 0x86, 0x46, 0xa9, 0x21, 0x4e, 0xea, 0xc8, 0xf9,
                0x34, 0x1f, 0x56, 0x4e,
            ],
        };

        Cmr(Midstate(bytes))
    }

    fn source_ty(&self) -> TypeName {
        let name: &'static [u8] = match self {
            Elements::Version => b"1",
            Elements::LockTime => b"1",
            Elements::InputIsPegin => b"i",
            Elements::InputPrevOutpoint => b"i",
            Elements::InputAsset => b"i",
            Elements::InputAmount => b"i",
            Elements::InputScriptHash => b"i",
            Elements::InputSequence => b"i",
            Elements::InputIssuanceBlinding => b"i",
            Elements::InputIssuanceContract => b"i",
            Elements::InputIssuanceEntropy => b"i",
            Elements::InputIssuanceAssetAmount => b"i",
            Elements::InputIssuanceTokenAmount => b"i",
            Elements::OutputAsset => b"i",
            Elements::OutputAmount => b"i",
            Elements::OutputNonce => b"i",
            Elements::OutputScriptHash => b"i",
            Elements::OutputNullDatum => b"*ii",
            Elements::ScriptCmr => b"i",
            Elements::CurrentIndex => b"1",
            Elements::CurrentIsPegin => b"1",
            Elements::CurrentPrevOutpoint => b"1",
            Elements::CurrentAsset => b"1",
            Elements::CurrentAmount => b"1",
            Elements::CurrentScriptHash => b"1",
            Elements::CurrentSequence => b"1",
            Elements::CurrentIssuanceBlinding => b"1",
            Elements::CurrentIssuanceContract => b"1",
            Elements::CurrentIssuanceEntropy => b"1",
            Elements::CurrentIssuanceAssetAmount => b"1",
            Elements::CurrentIssuanceTokenAmount => b"1",
            Elements::InputsHash => b"1",
            Elements::OutputsHash => b"1",
            Elements::NumInputs => b"1",
            Elements::NumOutputs => b"1",
            Elements::Fee => b"h",
            Elements::Add32 => b"l",
            Elements::FullAdd32 => b"*l2",
            Elements::Sub32 => b"l",
            Elements::FullSub32 => b"*l2",
            Elements::Mul32 => b"l",
            Elements::FullMul32 => b"*ll",
            Elements::Eq32Verify => b"l",
            Elements::Eq256Verify => b"*hh",
            Elements::Lt32Verify => b"l",
            Elements::Sha256 => b"h",
            Elements::Sha256Block => b"*h*hh",
            Elements::Bip0340Verify => b"**hh*hh",
        };

        TypeName(name)
    }

    fn target_ty(&self) -> TypeName {
        let name: &'static [u8] = match self {
            Elements::Version => b"i",
            Elements::LockTime => b"i",
            Elements::InputIsPegin => b"+12",
            Elements::InputPrevOutpoint => b"+1*hi",
            Elements::InputAsset => b"+1+*2hh",
            Elements::InputAmount => b"+1+*2hl",
            Elements::InputScriptHash => b"+1h",
            Elements::InputSequence => b"+1i",
            Elements::InputIssuanceBlinding => b"+1+1h",
            Elements::InputIssuanceContract => b"+1+1h",
            Elements::InputIssuanceEntropy => b"+1+1h",
            Elements::InputIssuanceAssetAmount => b"+1+1+*2hl",
            Elements::InputIssuanceTokenAmount => b"+1+1+*2hl",
            Elements::OutputAsset => b"+1+*2hh",
            Elements::OutputAmount => b"+1+*2hl",
            Elements::OutputNonce => b"+1+*2hh",
            Elements::OutputScriptHash => b"+1h",
            Elements::OutputNullDatum => b"+1+1+**22h+2*22",
            Elements::ScriptCmr => b"h",
            Elements::CurrentIndex => b"i",
            Elements::CurrentIsPegin => b"2",
            Elements::CurrentPrevOutpoint => b"*hi",
            Elements::CurrentAsset => b"+*2hh",
            Elements::CurrentAmount => b"+*2hl",
            Elements::CurrentScriptHash => b"h",
            Elements::CurrentSequence => b"i",
            Elements::CurrentIssuanceBlinding => b"+1h",
            Elements::CurrentIssuanceContract => b"+1h",
            Elements::CurrentIssuanceEntropy => b"+1h",
            Elements::CurrentIssuanceAssetAmount => b"+1+*2hl",
            Elements::CurrentIssuanceTokenAmount => b"+1+*2hl",
            Elements::InputsHash => b"h",
            Elements::OutputsHash => b"h",
            Elements::NumInputs => b"i",
            Elements::NumOutputs => b"i",
            Elements::Fee => b"l",
            Elements::Add32 => b"*2i",
            Elements::FullAdd32 => b"*2i",
            Elements::Sub32 => b"*2i",
            Elements::FullSub32 => b"*2i",
            Elements::Mul32 => b"l",
            Elements::FullMul32 => b"l",
            Elements::Eq32Verify => b"1",
            Elements::Eq256Verify => b"1",
            Elements::Lt32Verify => b"1",
            Elements::Sha256 => b"h",
            Elements::Sha256Block => b"h",
            Elements::Bip0340Verify => b"1",
        };

        TypeName(name)
    }

    fn encode<W: Write>(&self, w: &mut BitWriter<W>) -> std::io::Result<usize> {
        let (n, len) = match self {
            Elements::Version => (0, 7),
            Elements::LockTime => (1, 7),
            Elements::InputIsPegin => (1, 6),
            Elements::InputPrevOutpoint => (2, 6),
            Elements::InputAsset => (3, 6),
            Elements::InputAmount => (2, 7),
            Elements::InputScriptHash => (3, 7),
            Elements::InputSequence => (5, 6),
            Elements::InputIssuanceBlinding => (6, 6),
            Elements::InputIssuanceContract => (7, 6),
            Elements::InputIssuanceEntropy => (4, 7),
            Elements::InputIssuanceAssetAmount => (5, 7),
            Elements::InputIssuanceTokenAmount => (9, 6),
            Elements::OutputAsset => (10, 6),
            Elements::OutputAmount => (11, 6),
            Elements::OutputNonce => (6, 7),
            Elements::OutputScriptHash => (7, 7),
            Elements::OutputNullDatum => (13, 6),
            Elements::ScriptCmr => (14, 6),
            Elements::CurrentIndex => (15, 6),
            Elements::CurrentIsPegin => (16, 6),
            Elements::CurrentPrevOutpoint => (17, 6),
            Elements::CurrentAsset => (18, 6),
            Elements::CurrentAmount => (19, 6),
            Elements::CurrentScriptHash => (20, 6),
            Elements::CurrentSequence => (21, 6),
            Elements::CurrentIssuanceBlinding => (22, 6),
            Elements::CurrentIssuanceContract => (23, 6),
            Elements::CurrentIssuanceEntropy => (24, 6),
            Elements::CurrentIssuanceAssetAmount => (25, 6),
            Elements::CurrentIssuanceTokenAmount => (26, 6),
            Elements::InputsHash => (27, 6),
            Elements::OutputsHash => (28, 6),
            Elements::NumInputs => (29, 6),
            Elements::NumOutputs => (30, 6),
            Elements::Fee => (31, 6),
            Elements::Add32 => (16, 5),
            Elements::FullAdd32 => (20, 5),
            Elements::Sub32 => (17, 5),
            Elements::FullSub32 => (21, 5),
            Elements::Mul32 => (9, 4),
            Elements::FullMul32 => (11, 4),
            Elements::Eq32Verify => (116, 7),
            Elements::Eq256Verify => (113, 7),
            Elements::Lt32Verify => (115, 7),
            Elements::Sha256 => (114, 7),
            Elements::Sha256Block => (6, 3),
            Elements::Bip0340Verify => (112, 7),
        };

        w.write_bits_be(n, len)
    }

    fn decode<I: Iterator<Item = u8>>(bits: &mut BitIter<I>) -> Result<Self, Error> {
        decode_bits!(bits, {
            0 => {
                0 => {
                    0 => {
                        0 => {
                            0 => {
                                0 => {
                                    0 => {Elements::Version},
                                    1 => {Elements::LockTime}
                                },
                                1 => {Elements::InputIsPegin}
                            },
                            1 => {
                                0 => {Elements::InputPrevOutpoint},
                                1 => {Elements::InputAsset}
                            }
                        },
                        1 => {
                            0 => {
                                0 => {
                                    0 => {Elements::InputAmount},
                                    1 => {Elements::InputScriptHash}
                                },
                                1 => {Elements::InputSequence}
                            },
                            1 => {
                                0 => {Elements::InputIssuanceBlinding},
                                1 => {Elements::InputIssuanceContract}
                            }
                        }
                    },
                    1 => {
                        0 => {
                            0 => {
                                0 => {
                                    0 => {Elements::InputIssuanceEntropy},
                                    1 => {Elements::InputIssuanceAssetAmount}
                                },
                                1 => {Elements::InputIssuanceTokenAmount}
                            },
                            1 => {
                                0 => {Elements::OutputAsset},
                                1 => {Elements::OutputAmount}
                            }
                        },
                        1 => {
                            0 => {
                                0 => {
                                    0 => {Elements::OutputNonce},
                                    1 => {Elements::OutputScriptHash}
                                },
                                1 => {Elements::OutputNullDatum}
                            },
                            1 => {
                                0 => {Elements::ScriptCmr},
                                1 => {Elements::CurrentIndex}
                            }
                        }
                    }
                },
                1 => {
                    0 => {
                        0 => {
                            0 => {
                                0 => {Elements::CurrentIsPegin},
                                1 => {Elements::CurrentPrevOutpoint}
                            },
                            1 => {
                                0 => {Elements::CurrentAsset},
                                1 => {Elements::CurrentAmount}
                            }
                        },
                        1 => {
                            0 => {
                                0 => {Elements::CurrentScriptHash},
                                1 => {Elements::CurrentSequence}
                            },
                            1 => {
                                0 => {Elements::CurrentIssuanceBlinding},
                                1 => {Elements::CurrentIssuanceContract}
                            }
                        }
                    },
                    1 => {
                        0 => {
                            0 => {
                                0 => {Elements::CurrentIssuanceEntropy},
                                1 => {Elements::CurrentIssuanceAssetAmount}
                            },
                            1 => {
                                0 => {Elements::CurrentIssuanceTokenAmount},
                                1 => {Elements::InputsHash}
                            }
                        },
                        1 => {
                            0 => {
                                0 => {Elements::OutputsHash},
                                1 => {Elements::NumInputs}
                            },
                            1 => {
                                0 => {Elements::NumOutputs},
                                1 => {Elements::Fee}
                            }
                        }
                    }
                }
            },
            1 => {
                0 => {
                    0 => {
                        0 => {
                            0 => {Elements::Add32},
                            1 => {Elements::Sub32}
                        },
                        1 => {Elements::Mul32}
                    },
                    1 => {
                        0 => {
                            0 => {Elements::FullAdd32},
                            1 => {Elements::FullSub32}
                        },
                        1 => {Elements::FullMul32}
                    }
                },
                1 => {
                    0 => {Elements::Sha256Block},
                    1 => {
                        0 => {
                            0 => {
                                0 => {
                                    0 => {Elements::Bip0340Verify},
                                    1 => {Elements::Eq256Verify}
                                },
                                1 => {
                                    0 => {Elements::Sha256},
                                    1 => {Elements::Lt32Verify}
                                }
                            },
                            1 => {
                                0 => {
                                    0 => {Elements::Eq32Verify},
                                    1 => {}
                                },
                                1 => {}
                            }
                        },
                        1 => {}
                    }
                }
            }
        })
    }

    fn c_jet_ptr(&self) -> &'static dyn Fn(&mut CFrameItem, CFrameItem, &CTxEnv) -> bool {
        match self {
            Elements::Version => unimplemented!(),
            Elements::LockTime => &simplicity_sys::c_jets::jets_wrapper::lock_time,
            Elements::InputIsPegin => unimplemented!(),
            Elements::InputPrevOutpoint => unimplemented!(),
            Elements::InputAsset => unimplemented!(),
            Elements::InputAmount => unimplemented!(),
            Elements::InputScriptHash => unimplemented!(),
            Elements::InputSequence => unimplemented!(),
            Elements::InputIssuanceBlinding => unimplemented!(),
            Elements::InputIssuanceContract => unimplemented!(),
            Elements::InputIssuanceEntropy => unimplemented!(),
            Elements::InputIssuanceAssetAmount => unimplemented!(),
            Elements::InputIssuanceTokenAmount => unimplemented!(),
            Elements::OutputAsset => unimplemented!(),
            Elements::OutputAmount => unimplemented!(),
            Elements::OutputNonce => unimplemented!(),
            Elements::OutputScriptHash => unimplemented!(),
            Elements::OutputNullDatum => unimplemented!(),
            Elements::ScriptCmr => unimplemented!(),
            Elements::CurrentIndex => unimplemented!(),
            Elements::CurrentIsPegin => unimplemented!(),
            Elements::CurrentPrevOutpoint => unimplemented!(),
            Elements::CurrentAsset => unimplemented!(),
            Elements::CurrentAmount => unimplemented!(),
            Elements::CurrentScriptHash => unimplemented!(),
            Elements::CurrentSequence => unimplemented!(),
            Elements::CurrentIssuanceBlinding => unimplemented!(),
            Elements::CurrentIssuanceContract => unimplemented!(),
            Elements::CurrentIssuanceEntropy => unimplemented!(),
            Elements::CurrentIssuanceAssetAmount => unimplemented!(),
            Elements::CurrentIssuanceTokenAmount => unimplemented!(),
            Elements::InputsHash => unimplemented!(),
            Elements::OutputsHash => unimplemented!(),
            Elements::NumInputs => unimplemented!(),
            Elements::NumOutputs => unimplemented!(),
            Elements::Fee => unimplemented!(),
            Elements::Add32 => unimplemented!(),
            Elements::FullAdd32 => unimplemented!(),
            Elements::Sub32 => unimplemented!(),
            Elements::FullSub32 => unimplemented!(),
            Elements::Mul32 => unimplemented!(),
            Elements::FullMul32 => unimplemented!(),
            Elements::Eq32Verify => unimplemented!(),
            Elements::Eq256Verify => unimplemented!(),
            Elements::Lt32Verify => unimplemented!(),
            Elements::Sha256 => unimplemented!(),
            Elements::Sha256Block => unimplemented!(),
            Elements::Bip0340Verify => unimplemented!(),
        }
    }
}
