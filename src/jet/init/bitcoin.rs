/* This file has been automatically generated. */

use crate::bititer::BitIter;
use crate::bitwriter::BitWriter;
use crate::jet::type_name::TypeName;
use crate::jet::Jet;
use crate::merkle::cmr::Cmr;
use crate::decode_bits;
use crate::Error;
use bitcoin_hashes::sha256::Midstate;
use simplicity_sys::CFrameItem;
use std::io::Write;
use crate::jet::bitcoin::BitcoinEnv;

/// Bitcoin jet family
#[derive(Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Debug, Hash)]
pub enum Bitcoin {
    Add32,
    Bip0340Verify,
    CheckLockDistance,
    CheckLockDuration,
    CheckLockHeight,
    CheckLockTime,
    CheckSigVerify,
    CurrentAnnexHash,
    CurrentIndex,
    CurrentPrevOutpoint,
    CurrentScriptSigHash,
    CurrentSequence,
    CurrentValue,
    Decompress,
    Eq256,
    Eq32,
    FeAdd,
    FeInvert,
    FeIsOdd,
    FeIsZero,
    FeMultiply,
    FeMultiplyBeta,
    FeNegate,
    FeNormalize,
    FeSquare,
    FeSquareRoot,
    FullAdd32,
    FullMultiply32,
    FullSubtract32,
    GeIsOnCurve,
    GeNegate,
    GejAdd,
    GejDouble,
    GejGeAdd,
    GejGeAddEx,
    GejInfinity,
    GejIsInfinity,
    GejIsOnCurve,
    GejNegate,
    GejNormalize,
    GejRescale,
    GejXEquiv,
    GejYIsOdd,
    Generate,
    InputAnnexHash,
    InputPrevOutpoint,
    InputScriptSigHash,
    InputSequence,
    InputValue,
    InternalKey,
    Le32,
    LinearCombination1,
    LinearVerify1,
    LockTime,
    Low32,
    Multiply32,
    NumInputs,
    NumOutputs,
    One32,
    OutputScriptHash,
    OutputValue,
    ParseLock,
    ParseSequence,
    PointVerify1,
    ScalarAdd,
    ScalarInvert,
    ScalarIsZero,
    ScalarMultiply,
    ScalarMultiplyLambda,
    ScalarNegate,
    ScalarNormalize,
    ScalarSquare,
    Scale,
    ScriptCMR,
    Sha256Block,
    Sha256Ctx8Add1,
    Sha256Ctx8Add128,
    Sha256Ctx8Add16,
    Sha256Ctx8Add2,
    Sha256Ctx8Add256,
    Sha256Ctx8Add32,
    Sha256Ctx8Add4,
    Sha256Ctx8Add512,
    Sha256Ctx8Add64,
    Sha256Ctx8Add8,
    Sha256Ctx8AddBuffer511,
    Sha256Ctx8Finalize,
    Sha256Ctx8Init,
    Sha256Iv,
    Subtract32,
    Tapbranch,
    TapleafVersion,
    TotalInputValue,
    TotalOutputValue,
    TxIsFinal,
    TxLockDistance,
    TxLockDuration,
    TxLockHeight,
    TxLockTime,
    Verify,
    Version,
}

impl Jet for Bitcoin {

    type Environment = BitcoinEnv;
    type CJetEnvironment = ();

    fn c_jet_env<'env>(&self, _env: &'env Self::Environment) -> &'env Self::CJetEnvironment {
        unimplemented!("Unspecified CJetEnvironment for Bitcoin jets")
    }

    fn cmr(&self) -> Cmr {
        let bytes = match self {
            Bitcoin::Add32 => [
                0xe4, 0x04, 0x66, 0xa7, 0xec, 0xf7, 0x1c, 0xe8, 0x62, 0xfb, 0x3c, 0x15, 0x4c, 0x1e,
                0x8f, 0x84, 0x5d, 0x7e, 0x57, 0x07, 0x46, 0x3a, 0x89, 0x45, 0x37, 0xa3, 0x2f, 0xc7,
                0x21, 0x49, 0x00, 0xad,
            ],
            Bitcoin::Bip0340Verify => [
                0x6d, 0xa5, 0x26, 0xad, 0xe3, 0x4e, 0xb9, 0xd3, 0xaa, 0x8c, 0xd7, 0x05, 0xba, 0xf9,
                0xac, 0xeb, 0x3e, 0xb5, 0x85, 0x23, 0xba, 0x84, 0x28, 0x5c, 0x55, 0xfc, 0x04, 0x6a,
                0x2b, 0x0d, 0x2a, 0xea,
            ],
            Bitcoin::CheckLockDistance => [
                0x00, 0xb3, 0x30, 0xd0, 0x26, 0x2c, 0x6d, 0x7b, 0x09, 0xa4, 0x6d, 0x37, 0xd9, 0x94,
                0x7d, 0xbb, 0xc8, 0x88, 0xbc, 0x8a, 0x0b, 0x92, 0x02, 0xe1, 0xcf, 0x79, 0x0a, 0x28,
                0xa5, 0x34, 0x39, 0x85,
            ],
            Bitcoin::CheckLockDuration => [
                0xcb, 0xc7, 0x56, 0x03, 0x8c, 0x07, 0x08, 0xbb, 0xb1, 0xf7, 0xec, 0x43, 0xbe, 0x7d,
                0x15, 0xef, 0x1c, 0x33, 0x52, 0xf0, 0x6d, 0xd3, 0xf2, 0xd4, 0x7d, 0x87, 0xd5, 0xa8,
                0x78, 0x24, 0x43, 0xf8,
            ],
            Bitcoin::CheckLockHeight => [
                0x82, 0x97, 0x83, 0x2f, 0xb0, 0xdb, 0x5a, 0x67, 0x4c, 0xfb, 0x9a, 0x59, 0x66, 0xb5,
                0x71, 0x86, 0x12, 0x98, 0xba, 0xb2, 0x0a, 0xd7, 0xe3, 0xf3, 0x64, 0x27, 0x6d, 0xf5,
                0x00, 0x1f, 0x96, 0x8e,
            ],
            Bitcoin::CheckLockTime => [
                0x79, 0xcd, 0x0e, 0x35, 0xaf, 0xf2, 0x85, 0x75, 0x3e, 0x33, 0x39, 0xa7, 0x2d, 0xb8,
                0x0b, 0x3a, 0x6e, 0x0d, 0xd7, 0xca, 0x40, 0xf6, 0x50, 0x3b, 0xf9, 0x37, 0x83, 0x9c,
                0x0e, 0x64, 0xcf, 0x25,
            ],
            Bitcoin::CheckSigVerify => [
                0x05, 0xe7, 0xd6, 0xc4, 0x38, 0x6b, 0x46, 0x7a, 0x7d, 0x93, 0xc7, 0x21, 0x76, 0x5c,
                0x90, 0x41, 0x0d, 0x30, 0x9f, 0x08, 0xc6, 0x1c, 0x4e, 0x27, 0xd7, 0xbb, 0x69, 0x20,
                0x7b, 0xa0, 0x86, 0xd3,
            ],
            Bitcoin::CurrentAnnexHash => [
                0x03, 0x7a, 0xab, 0x00, 0x11, 0x41, 0x92, 0x82, 0xff, 0x3e, 0xa0, 0x8c, 0xaa, 0x14,
                0x2c, 0xfb, 0xbc, 0xa4, 0xeb, 0x55, 0xee, 0x69, 0xf6, 0xb2, 0x04, 0xb3, 0x5b, 0xee,
                0xd7, 0x74, 0x47, 0x0f,
            ],
            Bitcoin::CurrentIndex => [
                0x97, 0xea, 0x7e, 0xc0, 0x10, 0x47, 0x4d, 0x27, 0xbf, 0xf2, 0xce, 0xfe, 0x18, 0xa1,
                0xf6, 0x5b, 0x79, 0x98, 0x1c, 0xf8, 0x1a, 0x30, 0xa7, 0xe3, 0xbc, 0xce, 0x87, 0xd3,
                0xb8, 0x2c, 0x98, 0x32,
            ],
            Bitcoin::CurrentPrevOutpoint => [
                0xc3, 0x22, 0x84, 0x06, 0x2c, 0x3c, 0x9b, 0xa3, 0x4e, 0x88, 0x28, 0xe4, 0x8d, 0x0f,
                0x68, 0xaa, 0xa8, 0x0c, 0xd7, 0x8c, 0x90, 0x81, 0x4a, 0xfd, 0x8d, 0x9b, 0xf9, 0xee,
                0x2d, 0x68, 0x00, 0xfc,
            ],
            Bitcoin::CurrentScriptSigHash => [
                0x70, 0x76, 0x74, 0x9f, 0x2b, 0x1d, 0x1e, 0xca, 0x4e, 0xd1, 0x57, 0x75, 0xf7, 0x7d,
                0xd3, 0x90, 0x89, 0x2e, 0xc9, 0x98, 0xbc, 0x50, 0x70, 0xfc, 0xa2, 0x8c, 0xc1, 0xb8,
                0x41, 0x72, 0xfa, 0x57,
            ],
            Bitcoin::CurrentSequence => [
                0xd2, 0xdc, 0x51, 0xef, 0x24, 0x1b, 0xa1, 0xc4, 0x86, 0x84, 0x8f, 0xd2, 0x94, 0xc4,
                0x43, 0xe8, 0x28, 0xec, 0xfc, 0x79, 0x77, 0x27, 0x3e, 0x13, 0xba, 0xa0, 0x2a, 0x58,
                0xac, 0xb3, 0xaa, 0x22,
            ],
            Bitcoin::CurrentValue => [
                0xb6, 0x55, 0xed, 0x67, 0xb1, 0xea, 0x1f, 0x4f, 0x3d, 0x12, 0x5d, 0x57, 0x82, 0xcf,
                0xb6, 0x20, 0x4c, 0x13, 0x1c, 0x02, 0xd7, 0x5a, 0xfc, 0x46, 0x00, 0xc7, 0xb8, 0x3f,
                0x55, 0x6f, 0xd1, 0xeb,
            ],
            Bitcoin::Decompress => [
                0xab, 0x5a, 0x2d, 0xbc, 0x0f, 0x2c, 0x82, 0x08, 0x6d, 0x2d, 0x46, 0xbb, 0xa5, 0x69,
                0x1f, 0x13, 0x12, 0xba, 0xcc, 0x94, 0x6b, 0x08, 0xef, 0xfb, 0xe7, 0xf8, 0x51, 0x51,
                0x14, 0x1f, 0x7d, 0xcf,
            ],
            Bitcoin::Eq256 => [
                0xd8, 0xbb, 0x63, 0x54, 0x45, 0xb5, 0x6b, 0xc0, 0x6a, 0x33, 0x5e, 0x1d, 0xe8, 0xe5,
                0x56, 0xcc, 0x06, 0xf4, 0x93, 0x08, 0x67, 0x4c, 0xb8, 0x71, 0x6a, 0x5b, 0xa8, 0x1c,
                0xce, 0xd7, 0x54, 0x0e,
            ],
            Bitcoin::Eq32 => [
                0x88, 0xbf, 0xc8, 0x93, 0x7c, 0x11, 0x87, 0x3b, 0x06, 0xe3, 0xd8, 0xaf, 0xa4, 0xf9,
                0xf9, 0xe6, 0x22, 0xdb, 0x93, 0xf4, 0x70, 0x6b, 0xde, 0x36, 0xbc, 0x60, 0x14, 0x03,
                0x34, 0x48, 0xf3, 0x01,
            ],
            Bitcoin::FeAdd => [
                0xc0, 0x9d, 0x58, 0xe3, 0x8d, 0xc4, 0xce, 0x1a, 0xcc, 0x09, 0xda, 0xe8, 0xa5, 0x88,
                0x19, 0x08, 0xfe, 0x1e, 0xbc, 0x7f, 0x1f, 0xc7, 0x42, 0xc6, 0x1c, 0xdc, 0x84, 0x93,
                0xf2, 0x33, 0xb9, 0x4a,
            ],
            Bitcoin::FeInvert => [
                0xb6, 0xb1, 0x1b, 0xd6, 0x02, 0x58, 0xd3, 0x26, 0xb1, 0x9b, 0x6f, 0x78, 0x38, 0x7f,
                0xf3, 0xaa, 0xbf, 0x6a, 0x4c, 0x41, 0x9d, 0x0c, 0x5a, 0x8d, 0xda, 0x6b, 0x44, 0x05,
                0x0b, 0xc6, 0xe9, 0xd5,
            ],
            Bitcoin::FeIsOdd => [
                0x4d, 0x21, 0xe4, 0xb5, 0x47, 0x6f, 0xcf, 0x56, 0xa3, 0x69, 0xe5, 0xa1, 0x89, 0xc4,
                0x86, 0xe0, 0x98, 0x7f, 0x93, 0x32, 0xaa, 0xf4, 0xc0, 0xa7, 0x5a, 0xc3, 0xaf, 0x09,
                0xf8, 0x1b, 0x17, 0x09,
            ],
            Bitcoin::FeIsZero => [
                0x52, 0x35, 0x9f, 0x07, 0xed, 0x7a, 0x7c, 0x97, 0x0e, 0x94, 0x7d, 0xc8, 0xc4, 0x99,
                0x40, 0x56, 0x6c, 0x1a, 0x1e, 0x09, 0x5d, 0x03, 0x60, 0x4a, 0xfd, 0xb4, 0x43, 0x3d,
                0xa1, 0x3a, 0x87, 0xec,
            ],
            Bitcoin::FeMultiply => [
                0xac, 0x5a, 0x36, 0x26, 0xb5, 0xfc, 0x2b, 0x5a, 0x20, 0x6a, 0xc1, 0x8f, 0x1b, 0x0f,
                0x9e, 0xca, 0xcb, 0x5c, 0x63, 0x14, 0xc4, 0xef, 0xda, 0x59, 0xe0, 0xfa, 0xd3, 0xa1,
                0xb5, 0x99, 0xa1, 0xbd,
            ],
            Bitcoin::FeMultiplyBeta => [
                0xe7, 0xa6, 0x98, 0xe2, 0x5e, 0xbb, 0xf7, 0x0f, 0x8c, 0xed, 0x31, 0x30, 0xac, 0x04,
                0xd6, 0x74, 0xa9, 0xda, 0x39, 0xe0, 0x61, 0x76, 0x1b, 0xfd, 0x9d, 0x87, 0xd7, 0x94,
                0x89, 0x8f, 0x8a, 0x7a,
            ],
            Bitcoin::FeNegate => [
                0x87, 0x66, 0x58, 0x5d, 0x27, 0xd1, 0x88, 0x63, 0x42, 0x71, 0x44, 0x43, 0x2b, 0xa4,
                0x83, 0xb3, 0x6c, 0xd2, 0xdd, 0x1f, 0x36, 0x18, 0x14, 0x10, 0xac, 0xc7, 0x14, 0x93,
                0x9c, 0x0c, 0xb5, 0x6a,
            ],
            Bitcoin::FeNormalize => [
                0xc0, 0x70, 0xad, 0xba, 0xab, 0x2c, 0x7b, 0xe6, 0xff, 0x57, 0x7a, 0x75, 0x07, 0xaf,
                0xf0, 0xe7, 0x76, 0x57, 0xf3, 0x09, 0xe6, 0x5d, 0x05, 0xfa, 0x23, 0xc1, 0x92, 0x76,
                0xf7, 0x38, 0x52, 0xeb,
            ],
            Bitcoin::FeSquare => [
                0x8e, 0x77, 0xcc, 0x8c, 0x63, 0x69, 0x3a, 0x2a, 0xcd, 0x9a, 0x65, 0x26, 0x6a, 0x02,
                0x89, 0x06, 0xf8, 0x64, 0x21, 0x4a, 0xf6, 0x6b, 0xa5, 0x4c, 0xce, 0x11, 0xac, 0xb0,
                0x37, 0xc9, 0x43, 0x93,
            ],
            Bitcoin::FeSquareRoot => [
                0xf7, 0x71, 0x81, 0x03, 0x30, 0x4c, 0xb4, 0x36, 0x96, 0xbd, 0xf9, 0x2f, 0x61, 0x4c,
                0x83, 0x8d, 0x24, 0xd7, 0xdd, 0x7b, 0xa8, 0xdc, 0x01, 0xab, 0x5c, 0x6a, 0x77, 0x26,
                0x3c, 0x15, 0xf7, 0x29,
            ],
            Bitcoin::FullAdd32 => [
                0x47, 0x27, 0x36, 0x1e, 0xa0, 0x03, 0xc1, 0xa4, 0x83, 0xe5, 0x75, 0x05, 0xcf, 0x5b,
                0x40, 0x5a, 0x82, 0x27, 0xda, 0x1a, 0xdd, 0xc4, 0x7e, 0x2b, 0x01, 0x6c, 0x2d, 0x09,
                0xbe, 0x04, 0x7f, 0xe8,
            ],
            Bitcoin::FullMultiply32 => [
                0xaa, 0xc2, 0x53, 0x61, 0xe5, 0x98, 0xe3, 0x54, 0x38, 0xb9, 0x18, 0xb5, 0x8f, 0xd2,
                0xce, 0xf4, 0xdb, 0x3c, 0x5d, 0x8c, 0x5e, 0x63, 0xaa, 0x4f, 0x25, 0xe9, 0xce, 0xc0,
                0xcf, 0xd9, 0xdf, 0xb1,
            ],
            Bitcoin::FullSubtract32 => [
                0x6d, 0x96, 0xf6, 0x8a, 0x94, 0x5c, 0x22, 0xe7, 0x62, 0x11, 0x5c, 0x09, 0x42, 0x97,
                0xb1, 0x94, 0xbe, 0xdc, 0x0c, 0xe5, 0xa0, 0xc9, 0x2d, 0xb6, 0x4b, 0x83, 0x0a, 0x18,
                0xb4, 0x4d, 0xf0, 0xd0,
            ],
            Bitcoin::GeIsOnCurve => [
                0xd9, 0xaa, 0x66, 0x06, 0x5c, 0xb0, 0xed, 0x2c, 0x71, 0x68, 0x60, 0x9d, 0xfd, 0x62,
                0xab, 0x64, 0x3a, 0xa8, 0x7c, 0x27, 0xe0, 0xdb, 0xbf, 0x96, 0xf2, 0x91, 0x45, 0x28,
                0xfe, 0xef, 0x52, 0xc5,
            ],
            Bitcoin::GeNegate => [
                0xa4, 0x97, 0xe7, 0x1c, 0x40, 0x3c, 0x4c, 0xe2, 0xb7, 0x81, 0x89, 0x3c, 0xd6, 0x9a,
                0x52, 0x85, 0xea, 0x02, 0xd7, 0xb7, 0xfe, 0x8e, 0xdf, 0xac, 0xe7, 0x8a, 0x20, 0x5a,
                0xad, 0x2e, 0xc0, 0x33,
            ],
            Bitcoin::GejAdd => [
                0x80, 0x75, 0xb5, 0x90, 0x28, 0x64, 0x90, 0x87, 0xde, 0xd0, 0xe7, 0x66, 0x6e, 0x1a,
                0x00, 0x51, 0x45, 0x09, 0xd0, 0xc9, 0x87, 0xd8, 0x8d, 0x18, 0x68, 0x1a, 0xc8, 0x9b,
                0xc9, 0x60, 0xeb, 0x25,
            ],
            Bitcoin::GejDouble => [
                0x80, 0xd0, 0x82, 0x5d, 0x57, 0xce, 0x42, 0x4c, 0xc8, 0xb8, 0x9d, 0xc2, 0x51, 0x0d,
                0x7e, 0x65, 0x85, 0x8a, 0x99, 0x4e, 0x8e, 0x98, 0x76, 0x23, 0xd1, 0x0e, 0x34, 0x83,
                0xde, 0x26, 0xdf, 0x9e,
            ],
            Bitcoin::GejGeAdd => [
                0xdd, 0xc1, 0x2b, 0xb2, 0x89, 0x17, 0x5f, 0xf5, 0x87, 0xa5, 0x70, 0xb3, 0x02, 0xaf,
                0x61, 0x08, 0xfa, 0x56, 0x5b, 0xca, 0xce, 0x9c, 0x37, 0x4a, 0x10, 0xb7, 0x83, 0x63,
                0x95, 0x1d, 0xaa, 0x64,
            ],
            Bitcoin::GejGeAddEx => [
                0x4a, 0x89, 0x02, 0xb1, 0x1d, 0x73, 0x9b, 0x4f, 0xb8, 0x48, 0xd1, 0x85, 0x28, 0x73,
                0x6a, 0xb1, 0xd6, 0x68, 0xbe, 0x2e, 0xab, 0x1f, 0xed, 0x8d, 0x06, 0x83, 0x65, 0xb5,
                0x3b, 0xd7, 0x7d, 0x88,
            ],
            Bitcoin::GejInfinity => [
                0x2d, 0x9d, 0x36, 0xb4, 0x6e, 0xad, 0x02, 0xdb, 0x63, 0xb5, 0x85, 0xdc, 0xa6, 0x7e,
                0x5e, 0x4d, 0xcb, 0x94, 0x05, 0x89, 0xbb, 0x13, 0x3c, 0x99, 0x10, 0x0d, 0x61, 0x7c,
                0x27, 0x12, 0x6e, 0x96,
            ],
            Bitcoin::GejIsInfinity => [
                0xe1, 0x86, 0xf9, 0xbd, 0xbe, 0x49, 0x16, 0xc7, 0x2f, 0x6c, 0x3b, 0xc2, 0xad, 0xf3,
                0xe0, 0x47, 0x22, 0xef, 0x4c, 0xec, 0x29, 0x72, 0x53, 0xe3, 0xec, 0xaa, 0x4e, 0x4c,
                0xc5, 0x51, 0xef, 0x2c,
            ],
            Bitcoin::GejIsOnCurve => [
                0xa8, 0xc8, 0x2e, 0x8b, 0x3a, 0x61, 0x99, 0xda, 0x25, 0xb2, 0xb1, 0x9c, 0xd1, 0x49,
                0xf9, 0xff, 0x4c, 0x3f, 0xdc, 0x0b, 0x00, 0xb2, 0x64, 0x80, 0xc0, 0x00, 0x65, 0x53,
                0xd4, 0x3c, 0x1f, 0x6e,
            ],
            Bitcoin::GejNegate => [
                0x71, 0xee, 0xff, 0xb5, 0xb6, 0x37, 0xaf, 0x51, 0xc4, 0x97, 0x80, 0x02, 0xc2, 0x12,
                0xcd, 0xaf, 0x39, 0x6c, 0xf8, 0xef, 0xca, 0x33, 0xaa, 0xb0, 0xf8, 0x33, 0xf8, 0x1a,
                0x9f, 0xb6, 0xa9, 0x89,
            ],
            Bitcoin::GejNormalize => [
                0xb4, 0x19, 0x98, 0xde, 0x56, 0x4e, 0xf6, 0x4f, 0x63, 0xa4, 0xc9, 0xfa, 0xd1, 0x39,
                0x50, 0x64, 0x83, 0x2e, 0x7d, 0x5c, 0x4c, 0x77, 0x1d, 0x18, 0x0f, 0xce, 0xd2, 0x8d,
                0x8a, 0x76, 0x5b, 0xd6,
            ],
            Bitcoin::GejRescale => [
                0xfa, 0x70, 0xaa, 0x15, 0x3d, 0xab, 0x6b, 0xc9, 0x93, 0x55, 0xc1, 0x0c, 0x61, 0xe5,
                0xbf, 0xcf, 0xa3, 0x97, 0xb3, 0x81, 0xf7, 0xef, 0x59, 0x15, 0x9d, 0x83, 0x79, 0x1a,
                0x2a, 0x6a, 0x58, 0x24,
            ],
            Bitcoin::GejXEquiv => [
                0x65, 0xa8, 0x60, 0xfa, 0x7e, 0x74, 0x60, 0x1c, 0xb6, 0xd8, 0x37, 0x55, 0x3b, 0xa1,
                0x9d, 0x60, 0xc4, 0x77, 0x3c, 0x1c, 0x12, 0xb4, 0xb0, 0xf0, 0x27, 0x8b, 0x45, 0xfb,
                0x23, 0xfc, 0xe9, 0x67,
            ],
            Bitcoin::GejYIsOdd => [
                0xcf, 0x91, 0xc7, 0x1d, 0xa7, 0x13, 0x98, 0xec, 0x8c, 0x64, 0xfd, 0xbf, 0x8f, 0xdc,
                0x91, 0x2e, 0xd5, 0xa8, 0xc1, 0xfa, 0xc6, 0x56, 0x6e, 0x0b, 0x59, 0x13, 0xeb, 0xe9,
                0xc4, 0xb1, 0x06, 0x17,
            ],
            Bitcoin::Generate => [
                0x93, 0xc2, 0x40, 0xe4, 0x07, 0xb1, 0xb6, 0xa8, 0xc2, 0xf7, 0x87, 0xf9, 0x3c, 0xa9,
                0x92, 0x92, 0xc0, 0xe6, 0xb2, 0x48, 0x5a, 0x88, 0x2b, 0xfe, 0xac, 0xf9, 0x87, 0x8d,
                0xc3, 0xb1, 0xb9, 0xbe,
            ],
            Bitcoin::InputAnnexHash => [
                0x00, 0x7c, 0x05, 0x77, 0xc6, 0x2a, 0x07, 0xf0, 0xa6, 0x30, 0xd4, 0x08, 0x6c, 0xf0,
                0x6c, 0xad, 0xeb, 0x37, 0x61, 0x80, 0xc8, 0xdf, 0x71, 0xbe, 0x17, 0x62, 0xed, 0xff,
                0x08, 0x5b, 0x62, 0xab,
            ],
            Bitcoin::InputPrevOutpoint => [
                0xf9, 0x76, 0xec, 0x3c, 0x4e, 0x69, 0xcf, 0x0f, 0x83, 0x5f, 0x96, 0x62, 0xd3, 0x6c,
                0xe9, 0x16, 0xe8, 0x55, 0x12, 0x4a, 0x42, 0x0d, 0xb0, 0x77, 0xaa, 0x12, 0x39, 0x34,
                0x28, 0x06, 0x63, 0xc7,
            ],
            Bitcoin::InputScriptSigHash => [
                0x5b, 0x1d, 0xb0, 0xbc, 0x81, 0xf8, 0x6f, 0x53, 0x38, 0xbb, 0x0e, 0xb6, 0xc3, 0x41,
                0x1d, 0xd4, 0x04, 0x65, 0xaf, 0x54, 0x3a, 0xcf, 0x05, 0xc5, 0x1f, 0x36, 0xf2, 0xbe,
                0xdc, 0xd5, 0xc1, 0xcc,
            ],
            Bitcoin::InputSequence => [
                0x76, 0x51, 0x86, 0x70, 0x7b, 0xb6, 0x40, 0x20, 0x7d, 0x6e, 0x5c, 0x08, 0x91, 0x2e,
                0x04, 0xaf, 0xfe, 0xda, 0x3a, 0x56, 0xa0, 0x3b, 0xd9, 0x05, 0x89, 0x77, 0xe5, 0x87,
                0xf5, 0xb6, 0xc9, 0x2d,
            ],
            Bitcoin::InputValue => [
                0xfa, 0xc9, 0xe4, 0xee, 0x34, 0xb9, 0x26, 0xf5, 0xa6, 0x29, 0xc3, 0x83, 0x9e, 0x08,
                0xe6, 0x8d, 0x11, 0x6c, 0x18, 0xa3, 0xb8, 0x14, 0x66, 0x96, 0xd5, 0xb7, 0x09, 0x02,
                0x9a, 0xba, 0x84, 0xe7,
            ],
            Bitcoin::InternalKey => [
                0xac, 0x39, 0x46, 0xd5, 0x85, 0xf8, 0x7c, 0x49, 0x75, 0x6f, 0x62, 0x1e, 0x98, 0x31,
                0x7a, 0x65, 0x95, 0xd9, 0x62, 0x5b, 0x57, 0x49, 0xb0, 0x19, 0xb7, 0x27, 0xc4, 0x36,
                0xa4, 0x92, 0x86, 0x6d,
            ],
            Bitcoin::Le32 => [
                0xea, 0x77, 0x2c, 0x0b, 0x5a, 0xec, 0xde, 0x7d, 0xc1, 0x6e, 0x3f, 0x1f, 0x95, 0x27,
                0x89, 0xf0, 0x13, 0x70, 0x89, 0x04, 0xce, 0xaa, 0x62, 0xcf, 0xdc, 0xf6, 0x4d, 0x30,
                0xa3, 0x91, 0x17, 0xaf,
            ],
            Bitcoin::LinearCombination1 => [
                0x50, 0x49, 0xe2, 0x71, 0xce, 0xf0, 0x93, 0x75, 0xb6, 0x80, 0x52, 0xc0, 0x45, 0xdf,
                0x00, 0xfd, 0xc0, 0x2d, 0x10, 0xbe, 0xe0, 0x1c, 0xbf, 0x62, 0x46, 0xeb, 0xbc, 0xc5,
                0xec, 0xa5, 0xf1, 0x44,
            ],
            Bitcoin::LinearVerify1 => [
                0x46, 0xe3, 0x7c, 0x8c, 0xd9, 0xde, 0xf4, 0xf6, 0xf8, 0x53, 0x78, 0x5b, 0x91, 0xdf,
                0x8b, 0xb5, 0x3b, 0x2e, 0x61, 0x02, 0xcf, 0x88, 0xc8, 0xa6, 0xb8, 0xd6, 0x10, 0x85,
                0xa2, 0x3c, 0x5f, 0xec,
            ],
            Bitcoin::LockTime => [
                0xb0, 0x47, 0x51, 0xf2, 0x56, 0x9e, 0xc4, 0xff, 0x8d, 0x4e, 0xda, 0x91, 0xfc, 0x39,
                0x8b, 0xac, 0x4c, 0xda, 0xc4, 0xc9, 0x6a, 0x40, 0x77, 0xcc, 0xf4, 0x1c, 0xd0, 0xf3,
                0xef, 0x73, 0x58, 0x1b,
            ],
            Bitcoin::Low32 => [
                0x96, 0xbe, 0x53, 0x34, 0x5d, 0x52, 0x14, 0xb0, 0x05, 0xc7, 0xfc, 0x5d, 0xe4, 0x0a,
                0x92, 0x62, 0x56, 0x60, 0x41, 0x40, 0x63, 0x35, 0x27, 0xc1, 0xd0, 0x2b, 0xe0, 0xd5,
                0xf6, 0x0c, 0xc2, 0xad,
            ],
            Bitcoin::Multiply32 => [
                0x16, 0x1f, 0xd0, 0x3a, 0x92, 0xc6, 0x21, 0xb3, 0x28, 0x98, 0x49, 0xff, 0x29, 0xad,
                0x81, 0x34, 0x99, 0xd6, 0x3e, 0xd9, 0x73, 0xdb, 0x0e, 0x97, 0x51, 0x78, 0x54, 0x21,
                0xf5, 0x68, 0xd1, 0x8f,
            ],
            Bitcoin::NumInputs => [
                0x29, 0xae, 0x2b, 0x3d, 0x7c, 0x6d, 0x7e, 0xb4, 0x19, 0x4a, 0x3b, 0xfc, 0x68, 0x70,
                0x35, 0x2c, 0xd4, 0xcb, 0x6c, 0xbc, 0x35, 0xcb, 0x03, 0xa0, 0xcf, 0x60, 0xf9, 0xdc,
                0x6e, 0x3a, 0x14, 0x01,
            ],
            Bitcoin::NumOutputs => [
                0x16, 0x88, 0x70, 0x86, 0x9d, 0x1a, 0xe0, 0x04, 0x9b, 0x59, 0x6d, 0x1d, 0xb0, 0x4d,
                0x6c, 0x59, 0xef, 0x34, 0x83, 0x74, 0xdf, 0x24, 0xa9, 0x4c, 0x37, 0x7a, 0x31, 0xe3,
                0x0f, 0x19, 0xa9, 0xd6,
            ],
            Bitcoin::One32 => [
                0x45, 0x26, 0x09, 0x9d, 0x0f, 0x84, 0x0b, 0xa6, 0xab, 0xe5, 0x5e, 0xd9, 0xf3, 0xa9,
                0xb8, 0xfc, 0xa0, 0x84, 0xf2, 0x51, 0xbf, 0xb2, 0x6d, 0x01, 0x1c, 0xf7, 0xaf, 0x27,
                0xb0, 0xd8, 0x8e, 0x29,
            ],
            Bitcoin::OutputScriptHash => [
                0x74, 0xfe, 0x18, 0xd8, 0xe0, 0x90, 0x46, 0x66, 0x4f, 0x20, 0x20, 0xe2, 0x0f, 0x6f,
                0x68, 0xa9, 0x01, 0xeb, 0x6c, 0xf6, 0x1e, 0x2e, 0x37, 0x71, 0x4c, 0xa6, 0x64, 0xe1,
                0x9f, 0xe5, 0x1e, 0xc2,
            ],
            Bitcoin::OutputValue => [
                0xd5, 0x58, 0x72, 0x6f, 0x1a, 0x8a, 0x93, 0xe6, 0xe7, 0xec, 0x73, 0xdf, 0x60, 0x74,
                0xc0, 0x9d, 0xde, 0x05, 0x2f, 0x52, 0x0f, 0x0b, 0x12, 0x01, 0x99, 0x9a, 0xf6, 0x53,
                0xce, 0x6f, 0x4a, 0xd0,
            ],
            Bitcoin::ParseLock => [
                0xd5, 0x96, 0x9c, 0x25, 0xb5, 0x6e, 0x87, 0xb4, 0xfb, 0x28, 0x80, 0xea, 0xee, 0x90,
                0xcc, 0x94, 0x49, 0xfa, 0xa2, 0xf3, 0xd0, 0x76, 0x0d, 0xe9, 0xfe, 0xe5, 0x65, 0xdf,
                0xcf, 0x6a, 0x16, 0xc6,
            ],
            Bitcoin::ParseSequence => [
                0x04, 0xb9, 0x5b, 0xbe, 0x88, 0x1e, 0x1a, 0x6b, 0x41, 0x9f, 0x82, 0x88, 0x3f, 0xf0,
                0x73, 0xea, 0xdb, 0xc1, 0x4f, 0x2c, 0x8c, 0x56, 0xb3, 0x6f, 0x19, 0xdc, 0xec, 0xac,
                0x27, 0xe7, 0x73, 0x7f,
            ],
            Bitcoin::PointVerify1 => [
                0x91, 0x91, 0x22, 0x37, 0x47, 0x17, 0x87, 0xc5, 0xff, 0x2b, 0xdb, 0xd9, 0x41, 0xbc,
                0xeb, 0xda, 0xee, 0x90, 0xa2, 0x25, 0x24, 0x3c, 0x6a, 0x86, 0x85, 0xdd, 0x5f, 0x81,
                0x22, 0xdc, 0x66, 0xba,
            ],
            Bitcoin::ScalarAdd => [
                0x67, 0xe4, 0x1f, 0xad, 0x70, 0x45, 0x00, 0xce, 0x97, 0x50, 0x91, 0x32, 0xd4, 0xf6,
                0x97, 0x34, 0x2e, 0x85, 0x83, 0xed, 0x7e, 0x9f, 0x7b, 0xed, 0xb9, 0x95, 0xd3, 0x6c,
                0xf6, 0x5f, 0xf3, 0x2e,
            ],
            Bitcoin::ScalarInvert => [
                0xc0, 0xe2, 0xad, 0x1b, 0xa6, 0xbf, 0xd9, 0x10, 0x44, 0x24, 0xf5, 0x94, 0xd0, 0x07,
                0x3e, 0xa1, 0x99, 0x40, 0x5e, 0x5c, 0xa5, 0xb7, 0x12, 0x83, 0x94, 0xb6, 0x13, 0xb9,
                0xe1, 0xbd, 0x36, 0xfc,
            ],
            Bitcoin::ScalarIsZero => [
                0x38, 0xa4, 0x57, 0xca, 0xb1, 0xc3, 0x0c, 0x51, 0x4e, 0x20, 0xe2, 0x41, 0xd5, 0x84,
                0x65, 0x40, 0x75, 0xec, 0x4d, 0x05, 0x49, 0x6c, 0x7e, 0x0b, 0x1c, 0xe2, 0xde, 0x5e,
                0x2f, 0xc1, 0x99, 0x16,
            ],
            Bitcoin::ScalarMultiply => [
                0x14, 0x51, 0x3c, 0xf4, 0x41, 0x17, 0x9e, 0x62, 0xfb, 0x42, 0x93, 0xbb, 0x35, 0x3e,
                0xe5, 0xbf, 0x01, 0xed, 0xdf, 0x8d, 0x81, 0xce, 0x03, 0x10, 0x06, 0x2f, 0x09, 0xa8,
                0x1d, 0x2f, 0xbc, 0xa8,
            ],
            Bitcoin::ScalarMultiplyLambda => [
                0xf6, 0x24, 0x00, 0xf5, 0xbe, 0x74, 0x00, 0xa7, 0x12, 0xe7, 0x4a, 0x1d, 0xc1, 0xa8,
                0x41, 0xe6, 0x02, 0x4a, 0x96, 0x18, 0x55, 0x1c, 0x33, 0x64, 0xc4, 0xe8, 0x15, 0x6d,
                0x1c, 0xdd, 0xed, 0x83,
            ],
            Bitcoin::ScalarNegate => [
                0x6a, 0x97, 0x6a, 0x67, 0x68, 0xbd, 0x72, 0x8f, 0xe2, 0x10, 0x51, 0x85, 0x1c, 0x60,
                0xeb, 0x25, 0x72, 0xe5, 0xd0, 0x6c, 0x95, 0x56, 0x6d, 0xfa, 0xe9, 0x28, 0x20, 0xc8,
                0x42, 0x4a, 0xaa, 0x4e,
            ],
            Bitcoin::ScalarNormalize => [
                0x90, 0xe0, 0x25, 0x78, 0x96, 0x99, 0x0b, 0xa6, 0xf0, 0xb0, 0x76, 0x54, 0x29, 0x19,
                0xcd, 0x06, 0x4a, 0xc0, 0x8b, 0x27, 0x7f, 0xae, 0x34, 0x79, 0xe4, 0x09, 0x18, 0xeb,
                0x6f, 0x5b, 0x07, 0xd8,
            ],
            Bitcoin::ScalarSquare => [
                0xd6, 0x36, 0xb4, 0x9d, 0xc6, 0xb2, 0x26, 0x6c, 0xce, 0xcb, 0x7b, 0xc0, 0x41, 0x68,
                0x82, 0x3b, 0x2a, 0x5d, 0x7a, 0x1d, 0x2a, 0xc3, 0x43, 0xda, 0x60, 0x54, 0x19, 0xd3,
                0x8d, 0xfd, 0xfd, 0xe0,
            ],
            Bitcoin::Scale => [
                0xa1, 0x8f, 0xe4, 0x0d, 0x7a, 0x5c, 0xea, 0x9a, 0x02, 0x88, 0xb7, 0xbd, 0xba, 0xa1,
                0xbf, 0xad, 0x4f, 0x47, 0x5f, 0x4d, 0x1a, 0xd7, 0x7a, 0x3a, 0x88, 0x3f, 0xa3, 0xc8,
                0x22, 0xd6, 0x88, 0x38,
            ],
            Bitcoin::ScriptCMR => [
                0x7c, 0x32, 0x40, 0x89, 0x00, 0x1a, 0x41, 0x2b, 0xc5, 0x18, 0x34, 0xb9, 0xdf, 0x6a,
                0x48, 0xc4, 0x4d, 0x95, 0x34, 0x0b, 0x66, 0x63, 0x62, 0x84, 0x0b, 0x44, 0xf8, 0xf6,
                0x57, 0xf5, 0x94, 0xe7,
            ],
            Bitcoin::Sha256Block => [
                0xdf, 0xc9, 0x27, 0xd3, 0x9b, 0xf7, 0x14, 0x7a, 0x8b, 0x0a, 0x7f, 0x43, 0x79, 0x46,
                0x68, 0x70, 0x82, 0x4d, 0xb1, 0x02, 0x09, 0x0a, 0x00, 0x36, 0x29, 0x23, 0xa3, 0x77,
                0xa9, 0x1a, 0xf6, 0x81,
            ],
            Bitcoin::Sha256Ctx8Add1 => [
                0x2e, 0xcd, 0x3d, 0xbe, 0x41, 0xe1, 0xfb, 0x41, 0x7d, 0x24, 0x21, 0xdd, 0x07, 0xef,
                0x6e, 0x6e, 0xdb, 0xad, 0xb8, 0x82, 0x69, 0x81, 0x61, 0xeb, 0xcc, 0xf6, 0xca, 0xf0,
                0xee, 0xb9, 0x98, 0x30,
            ],
            Bitcoin::Sha256Ctx8Add128 => [
                0x3d, 0x5d, 0x67, 0xe2, 0x7e, 0xfc, 0x25, 0xa7, 0x2d, 0xa6, 0x18, 0x6d, 0x2a, 0x6f,
                0xa1, 0x06, 0xe7, 0x57, 0x43, 0x57, 0xb1, 0xf8, 0x76, 0xfa, 0x68, 0x79, 0xa6, 0x11,
                0x36, 0x82, 0xb7, 0x35,
            ],
            Bitcoin::Sha256Ctx8Add16 => [
                0x7b, 0xa6, 0xef, 0xb6, 0x92, 0xa1, 0x91, 0xe4, 0xdd, 0xd7, 0xa5, 0x84, 0x1d, 0x5a,
                0x06, 0x3d, 0x6a, 0xa3, 0xf0, 0xcf, 0xcd, 0x16, 0xfd, 0x84, 0xde, 0xff, 0x5d, 0x1f,
                0x3b, 0x3b, 0x08, 0xcb,
            ],
            Bitcoin::Sha256Ctx8Add2 => [
                0x12, 0x3c, 0x7d, 0x8c, 0xe5, 0x39, 0xd8, 0xab, 0xc6, 0xda, 0xc5, 0xf9, 0x9c, 0xe8,
                0x23, 0x39, 0xb9, 0x56, 0x41, 0x68, 0xe6, 0x56, 0x43, 0x37, 0x28, 0x5e, 0x89, 0x82,
                0x4a, 0x23, 0x9f, 0x33,
            ],
            Bitcoin::Sha256Ctx8Add256 => [
                0x76, 0xe2, 0xb4, 0xfc, 0xa5, 0x4b, 0x4d, 0x7a, 0x39, 0x0b, 0x92, 0xb0, 0xf0, 0xe1,
                0x20, 0x49, 0x1d, 0xe8, 0x6d, 0xfe, 0xa6, 0xe9, 0x62, 0x58, 0x80, 0x97, 0x23, 0x92,
                0xb1, 0xf5, 0x38, 0x4c,
            ],
            Bitcoin::Sha256Ctx8Add32 => [
                0xa4, 0xd6, 0x39, 0xe4, 0xec, 0x73, 0x39, 0x9d, 0x31, 0x93, 0x9c, 0xe3, 0xc0, 0x25,
                0x06, 0xd1, 0xd2, 0x3a, 0x20, 0x29, 0x3b, 0xb9, 0x5a, 0xb3, 0x7c, 0xb8, 0x7d, 0xa2,
                0xb1, 0x87, 0xc6, 0xf8,
            ],
            Bitcoin::Sha256Ctx8Add4 => [
                0x8a, 0x41, 0xbf, 0xb6, 0x24, 0x29, 0x5a, 0x3d, 0x8f, 0x67, 0xdc, 0x5b, 0xda, 0x5e,
                0x7c, 0x7d, 0x97, 0xc9, 0xcc, 0x30, 0x21, 0xd3, 0x81, 0xce, 0x8b, 0x04, 0x7b, 0x3d,
                0x79, 0xd1, 0x87, 0xc9,
            ],
            Bitcoin::Sha256Ctx8Add512 => [
                0x2c, 0x80, 0x99, 0x15, 0x75, 0x65, 0x4b, 0x3d, 0x81, 0x65, 0x8d, 0xca, 0xde, 0xa5,
                0x19, 0x49, 0xcc, 0x18, 0x25, 0x30, 0xaf, 0x7e, 0x9f, 0x41, 0xee, 0x5b, 0x93, 0xfc,
                0x33, 0xf0, 0xc8, 0x0f,
            ],
            Bitcoin::Sha256Ctx8Add64 => [
                0x10, 0xae, 0x93, 0x7f, 0x66, 0x97, 0x00, 0xd3, 0x21, 0x55, 0x01, 0x53, 0xd4, 0x60,
                0x25, 0x20, 0x8c, 0x03, 0x70, 0xda, 0x86, 0x6d, 0xd3, 0xd4, 0xc9, 0xb3, 0x4d, 0x42,
                0x2c, 0xcc, 0x2c, 0xc8,
            ],
            Bitcoin::Sha256Ctx8Add8 => [
                0x48, 0xd6, 0x97, 0x4c, 0xd3, 0x6c, 0x37, 0x6c, 0x4b, 0xd2, 0x60, 0xd1, 0x1f, 0xf6,
                0x2e, 0x74, 0x20, 0x76, 0x93, 0x4e, 0xc3, 0x10, 0x8d, 0x9f, 0x8f, 0xf2, 0x7f, 0xb4,
                0x54, 0xe4, 0x94, 0x5f,
            ],
            Bitcoin::Sha256Ctx8AddBuffer511 => [
                0x3d, 0x59, 0x9f, 0x24, 0xf3, 0xb1, 0x53, 0x3c, 0x86, 0x64, 0xc4, 0x05, 0x04, 0xdc,
                0x39, 0xbf, 0xc5, 0x75, 0x6f, 0x2d, 0x9a, 0x6a, 0x4d, 0x0d, 0x65, 0xbe, 0x29, 0x9e,
                0x88, 0xb4, 0xa6, 0xd5,
            ],
            Bitcoin::Sha256Ctx8Finalize => [
                0x8c, 0x44, 0xd9, 0x30, 0x4f, 0x8f, 0x85, 0xe8, 0x56, 0x64, 0x90, 0x63, 0xf7, 0x84,
                0x2a, 0xc5, 0xa6, 0xe8, 0x0f, 0xd3, 0x7a, 0x01, 0x41, 0xc9, 0xff, 0xde, 0x74, 0x2f,
                0xbe, 0x8c, 0x9f, 0xf8,
            ],
            Bitcoin::Sha256Ctx8Init => [
                0x88, 0xdb, 0x03, 0x5a, 0x49, 0x0d, 0xa7, 0x89, 0x30, 0xf7, 0x5e, 0x6e, 0x1b, 0x10,
                0xaa, 0x86, 0x8b, 0xd4, 0x75, 0x93, 0xc6, 0x74, 0xf3, 0xbc, 0x4b, 0xb6, 0x24, 0x22,
                0x53, 0x13, 0xe3, 0xba,
            ],
            Bitcoin::Sha256Iv => [
                0xe4, 0xd5, 0x96, 0xe0, 0x0f, 0xf0, 0xf7, 0xdf, 0x3d, 0x99, 0x69, 0xd8, 0x2c, 0xb1,
                0x63, 0x4d, 0x59, 0xeb, 0x9d, 0x0f, 0x4b, 0x24, 0xd8, 0xca, 0x72, 0xe8, 0x48, 0xc1,
                0x0c, 0x75, 0x24, 0x4c,
            ],
            Bitcoin::Subtract32 => [
                0xf7, 0x6e, 0xca, 0xd1, 0xfd, 0xa5, 0x0f, 0x13, 0x5b, 0xdf, 0xe3, 0xe5, 0x33, 0xa1,
                0x50, 0x09, 0x8f, 0x40, 0x62, 0x61, 0xc7, 0x6f, 0x6d, 0xbf, 0x67, 0x25, 0xf1, 0xe3,
                0x88, 0x3c, 0x2a, 0xe2,
            ],
            Bitcoin::Tapbranch => [
                0xd4, 0x17, 0x67, 0x67, 0x56, 0xfa, 0x62, 0x95, 0xac, 0x1f, 0xae, 0x61, 0xe9, 0x74,
                0xb3, 0x3f, 0xf1, 0x72, 0x1f, 0x1e, 0x84, 0xf8, 0xeb, 0x43, 0x7b, 0x0f, 0xbb, 0x9c,
                0xd0, 0xb4, 0x02, 0xb2,
            ],
            Bitcoin::TapleafVersion => [
                0xda, 0x27, 0x66, 0x5c, 0x97, 0x57, 0x02, 0x20, 0xfd, 0xc3, 0x97, 0xf7, 0x9c, 0x2b,
                0xa2, 0x76, 0x3a, 0x51, 0xb0, 0x0d, 0xb9, 0xd5, 0xaa, 0xb7, 0x64, 0xe9, 0x9a, 0x98,
                0x90, 0x3f, 0x25, 0x07,
            ],
            Bitcoin::TotalInputValue => [
                0x22, 0x48, 0xa8, 0xe0, 0xe9, 0x7c, 0x8f, 0x76, 0xd5, 0xde, 0xaf, 0x0e, 0xdf, 0x16,
                0x09, 0x80, 0xc8, 0xfa, 0xe7, 0x80, 0x78, 0xb5, 0xed, 0x71, 0xf7, 0x83, 0x08, 0xe6,
                0x4c, 0xef, 0x1a, 0xd0,
            ],
            Bitcoin::TotalOutputValue => [
                0x70, 0x36, 0x34, 0x2b, 0xbc, 0xee, 0xed, 0xfe, 0xac, 0xd9, 0x0c, 0xd5, 0xe9, 0xcd,
                0xe1, 0xdb, 0x92, 0x06, 0x8a, 0x2c, 0xca, 0x84, 0x66, 0x82, 0xee, 0x53, 0x72, 0xf8,
                0x14, 0x69, 0x36, 0xea,
            ],
            Bitcoin::TxIsFinal => [
                0x17, 0x15, 0x82, 0x2d, 0x05, 0x6c, 0xa9, 0x85, 0x29, 0x16, 0x4e, 0x2a, 0x1c, 0xd6,
                0x87, 0x68, 0xe3, 0xf7, 0x42, 0x0d, 0xdc, 0x10, 0x89, 0xbd, 0x1c, 0x4a, 0x16, 0x96,
                0xb4, 0x2b, 0xb7, 0x03,
            ],
            Bitcoin::TxLockDistance => [
                0x77, 0x92, 0xd8, 0xf5, 0x16, 0xd7, 0x39, 0xae, 0xea, 0x22, 0xac, 0xf5, 0xbe, 0x8a,
                0x5f, 0x06, 0xfe, 0xaf, 0x05, 0x4f, 0xa5, 0x7b, 0x3c, 0x30, 0x45, 0x77, 0x9d, 0x98,
                0x8b, 0x24, 0xa2, 0x69,
            ],
            Bitcoin::TxLockDuration => [
                0xea, 0x87, 0x6a, 0x07, 0x37, 0xce, 0x90, 0xe4, 0xad, 0x13, 0xf2, 0x6f, 0x04, 0x35,
                0x91, 0x42, 0x32, 0xd4, 0xc7, 0x33, 0xdd, 0xdc, 0x21, 0xf5, 0x93, 0x28, 0x23, 0x94,
                0x6b, 0xb4, 0x84, 0x3f,
            ],
            Bitcoin::TxLockHeight => [
                0xaa, 0x3b, 0xdc, 0x79, 0x77, 0x11, 0x61, 0xb2, 0x68, 0x28, 0x14, 0xbe, 0xff, 0xbe,
                0x44, 0xc2, 0xdd, 0xc9, 0xf2, 0x7c, 0x84, 0x34, 0x33, 0xaa, 0xa9, 0x0b, 0x6a, 0x8f,
                0xc7, 0xf7, 0x10, 0xbe,
            ],
            Bitcoin::TxLockTime => [
                0xb9, 0x98, 0x83, 0x3e, 0xaa, 0xd5, 0xb0, 0x5f, 0x36, 0x16, 0xd7, 0x6f, 0x3a, 0xab,
                0x2f, 0x82, 0x61, 0x79, 0x88, 0x60, 0x22, 0x79, 0xd7, 0x35, 0xf6, 0x5c, 0x74, 0x16,
                0x41, 0x14, 0x7c, 0xf8,
            ],
            Bitcoin::Verify => [
                0xa1, 0x75, 0x34, 0xa4, 0x9a, 0xcb, 0xec, 0xb1, 0x55, 0x4d, 0x6e, 0xc5, 0xc6, 0xda,
                0x50, 0xc2, 0x8c, 0x9c, 0xd9, 0x5e, 0xa0, 0xb4, 0x48, 0x86, 0xb8, 0xe0, 0x94, 0xad,
                0xa7, 0x77, 0xc1, 0xd4,
            ],
            Bitcoin::Version => [
                0x5b, 0xf4, 0xf6, 0xe5, 0x46, 0xe2, 0x53, 0xe2, 0xdb, 0x9f, 0xde, 0xc9, 0x52, 0x19,
                0x95, 0x74, 0x45, 0xa6, 0xcc, 0x2d, 0xd6, 0x08, 0x30, 0x9b, 0x89, 0x10, 0xba, 0xf1,
                0xbc, 0x77, 0x3f, 0xa8,
            ],
        };

        Cmr(Midstate(bytes))
    }

    fn source_ty(&self) -> TypeName {
        let name: &'static [u8] = match self {
            Bitcoin::Add32 => b"l",
            Bitcoin::Bip0340Verify => b"**hh*hh",
            Bitcoin::CheckLockDistance => b"****22*22**22*22***22*22**22*22",
            Bitcoin::CheckLockDuration => b"****22*22**22*22***22*22**22*22",
            Bitcoin::CheckLockHeight => b"i",
            Bitcoin::CheckLockTime => b"i",
            Bitcoin::CheckSigVerify => b"**h*hh*hh",
            Bitcoin::CurrentAnnexHash => b"1",
            Bitcoin::CurrentIndex => b"1",
            Bitcoin::CurrentPrevOutpoint => b"1",
            Bitcoin::CurrentScriptSigHash => b"1",
            Bitcoin::CurrentSequence => b"1",
            Bitcoin::CurrentValue => b"1",
            Bitcoin::Decompress => b"*2h",
            Bitcoin::Eq256 => b"*hh",
            Bitcoin::Eq32 => b"l",
            Bitcoin::FeAdd => b"*hh",
            Bitcoin::FeInvert => b"h",
            Bitcoin::FeIsOdd => b"h",
            Bitcoin::FeIsZero => b"h",
            Bitcoin::FeMultiply => b"*hh",
            Bitcoin::FeMultiplyBeta => b"h",
            Bitcoin::FeNegate => b"h",
            Bitcoin::FeNormalize => b"h",
            Bitcoin::FeSquare => b"h",
            Bitcoin::FeSquareRoot => b"h",
            Bitcoin::FullAdd32 => b"*2l",
            Bitcoin::FullMultiply32 => b"*ll",
            Bitcoin::FullSubtract32 => b"*2l",
            Bitcoin::GeIsOnCurve => b"*hh",
            Bitcoin::GeNegate => b"*hh",
            Bitcoin::GejAdd => b"***hhh**hhh",
            Bitcoin::GejDouble => b"**hhh",
            Bitcoin::GejGeAdd => b"***hhh*hh",
            Bitcoin::GejGeAddEx => b"***hhh*hh",
            Bitcoin::GejInfinity => b"1",
            Bitcoin::GejIsInfinity => b"**hhh",
            Bitcoin::GejIsOnCurve => b"**hhh",
            Bitcoin::GejNegate => b"**hhh",
            Bitcoin::GejNormalize => b"**hhh",
            Bitcoin::GejRescale => b"***hhhh",
            Bitcoin::GejXEquiv => b"*h**hhh",
            Bitcoin::GejYIsOdd => b"**hhh",
            Bitcoin::Generate => b"h",
            Bitcoin::InputAnnexHash => b"i",
            Bitcoin::InputPrevOutpoint => b"i",
            Bitcoin::InputScriptSigHash => b"i",
            Bitcoin::InputSequence => b"i",
            Bitcoin::InputValue => b"i",
            Bitcoin::InternalKey => b"1",
            Bitcoin::Le32 => b"l",
            Bitcoin::LinearCombination1 => b"**h**hhhh",
            Bitcoin::LinearVerify1 => b"***h*hhh*hh",
            Bitcoin::LockTime => b"1",
            Bitcoin::Low32 => b"1",
            Bitcoin::Multiply32 => b"l",
            Bitcoin::NumInputs => b"1",
            Bitcoin::NumOutputs => b"1",
            Bitcoin::One32 => b"1",
            Bitcoin::OutputScriptHash => b"i",
            Bitcoin::OutputValue => b"i",
            Bitcoin::ParseLock => b"i",
            Bitcoin::ParseSequence => b"i",
            Bitcoin::PointVerify1 => b"***h*2hh*2h",
            Bitcoin::ScalarAdd => b"*hh",
            Bitcoin::ScalarInvert => b"h",
            Bitcoin::ScalarIsZero => b"h",
            Bitcoin::ScalarMultiply => b"*hh",
            Bitcoin::ScalarMultiplyLambda => b"h",
            Bitcoin::ScalarNegate => b"h",
            Bitcoin::ScalarNormalize => b"h",
            Bitcoin::ScalarSquare => b"h",
            Bitcoin::Scale => b"*h**hhh",
            Bitcoin::ScriptCMR => b"1",
            Bitcoin::Sha256Block => b"*h*hh",
            Bitcoin::Sha256Ctx8Add1 => b"***+1h*+1*ll*+1l*+1i*+1****22*22**22*22***22*22**22*22+1***22*22**22*22*lh***22*22**22*22",
            Bitcoin::Sha256Ctx8Add128 => b"***+1h*+1*ll*+1l*+1i*+1****22*22**22*22***22*22**22*22+1***22*22**22*22*lh**hh*hh",
            Bitcoin::Sha256Ctx8Add16 => b"***+1h*+1*ll*+1l*+1i*+1****22*22**22*22***22*22**22*22+1***22*22**22*22*lh*ll",
            Bitcoin::Sha256Ctx8Add2 => b"***+1h*+1*ll*+1l*+1i*+1****22*22**22*22***22*22**22*22+1***22*22**22*22*lh****22*22**22*22***22*22**22*22",
            Bitcoin::Sha256Ctx8Add256 => b"***+1h*+1*ll*+1l*+1i*+1****22*22**22*22***22*22**22*22+1***22*22**22*22*lh***hh*hh**hh*hh",
            Bitcoin::Sha256Ctx8Add32 => b"***+1h*+1*ll*+1l*+1i*+1****22*22**22*22***22*22**22*22+1***22*22**22*22*lhh",
            Bitcoin::Sha256Ctx8Add4 => b"***+1h*+1*ll*+1l*+1i*+1****22*22**22*22***22*22**22*22+1***22*22**22*22*lhi",
            Bitcoin::Sha256Ctx8Add512 => b"***+1h*+1*ll*+1l*+1i*+1****22*22**22*22***22*22**22*22+1***22*22**22*22*lh****hh*hh**hh*hh***hh*hh**hh*hh",
            Bitcoin::Sha256Ctx8Add64 => b"***+1h*+1*ll*+1l*+1i*+1****22*22**22*22***22*22**22*22+1***22*22**22*22*lh*hh",
            Bitcoin::Sha256Ctx8Add8 => b"***+1h*+1*ll*+1l*+1i*+1****22*22**22*22***22*22**22*22+1***22*22**22*22*lhl",
            Bitcoin::Sha256Ctx8AddBuffer511 => b"***+1h*+1*ll*+1l*+1i*+1****22*22**22*22***22*22**22*22+1***22*22**22*22*lh*+1***hh*hh**hh*hh*+1**hh*hh*+1*hh*+1h*+1*ll*+1l*+1i*+1****22*22**22*22***22*22**22*22+1***22*22**22*22",
            Bitcoin::Sha256Ctx8Finalize => b"**+1h*+1*ll*+1l*+1i*+1****22*22**22*22***22*22**22*22+1***22*22**22*22*lh",
            Bitcoin::Sha256Ctx8Init => b"1",
            Bitcoin::Sha256Iv => b"1",
            Bitcoin::Subtract32 => b"l",
            Bitcoin::Tapbranch => b"***22*22**22*22",
            Bitcoin::TapleafVersion => b"1",
            Bitcoin::TotalInputValue => b"1",
            Bitcoin::TotalOutputValue => b"1",
            Bitcoin::TxIsFinal => b"1",
            Bitcoin::TxLockDistance => b"1",
            Bitcoin::TxLockDuration => b"1",
            Bitcoin::TxLockHeight => b"1",
            Bitcoin::TxLockTime => b"1",
            Bitcoin::Verify => b"2",
            Bitcoin::Version => b"1",
        };

        TypeName(name)
    }

    fn target_ty(&self) -> TypeName {
        let name: &'static [u8] = match self {
            Bitcoin::Add32 => b"*2i",
            Bitcoin::Bip0340Verify => b"1",
            Bitcoin::CheckLockDistance => b"1",
            Bitcoin::CheckLockDuration => b"1",
            Bitcoin::CheckLockHeight => b"1",
            Bitcoin::CheckLockTime => b"1",
            Bitcoin::CheckSigVerify => b"1",
            Bitcoin::CurrentAnnexHash => b"+1h",
            Bitcoin::CurrentIndex => b"i",
            Bitcoin::CurrentPrevOutpoint => b"*hi",
            Bitcoin::CurrentScriptSigHash => b"h",
            Bitcoin::CurrentSequence => b"i",
            Bitcoin::CurrentValue => b"l",
            Bitcoin::Decompress => b"+1*hh",
            Bitcoin::Eq256 => b"2",
            Bitcoin::Eq32 => b"2",
            Bitcoin::FeAdd => b"h",
            Bitcoin::FeInvert => b"h",
            Bitcoin::FeIsOdd => b"2",
            Bitcoin::FeIsZero => b"2",
            Bitcoin::FeMultiply => b"h",
            Bitcoin::FeMultiplyBeta => b"h",
            Bitcoin::FeNegate => b"h",
            Bitcoin::FeNormalize => b"h",
            Bitcoin::FeSquare => b"h",
            Bitcoin::FeSquareRoot => b"+1h",
            Bitcoin::FullAdd32 => b"*2i",
            Bitcoin::FullMultiply32 => b"l",
            Bitcoin::FullSubtract32 => b"*2i",
            Bitcoin::GeIsOnCurve => b"2",
            Bitcoin::GeNegate => b"*hh",
            Bitcoin::GejAdd => b"**hhh",
            Bitcoin::GejDouble => b"**hhh",
            Bitcoin::GejGeAdd => b"**hhh",
            Bitcoin::GejGeAddEx => b"*h**hhh",
            Bitcoin::GejInfinity => b"**hhh",
            Bitcoin::GejIsInfinity => b"2",
            Bitcoin::GejIsOnCurve => b"2",
            Bitcoin::GejNegate => b"**hhh",
            Bitcoin::GejNormalize => b"+1*hh",
            Bitcoin::GejRescale => b"**hhh",
            Bitcoin::GejXEquiv => b"2",
            Bitcoin::GejYIsOdd => b"2",
            Bitcoin::Generate => b"**hhh",
            Bitcoin::InputAnnexHash => b"+1+1h",
            Bitcoin::InputPrevOutpoint => b"+1*hi",
            Bitcoin::InputScriptSigHash => b"+1h",
            Bitcoin::InputSequence => b"+1i",
            Bitcoin::InputValue => b"+1l",
            Bitcoin::InternalKey => b"h",
            Bitcoin::Le32 => b"2",
            Bitcoin::LinearCombination1 => b"**hhh",
            Bitcoin::LinearVerify1 => b"1",
            Bitcoin::LockTime => b"i",
            Bitcoin::Low32 => b"i",
            Bitcoin::Multiply32 => b"l",
            Bitcoin::NumInputs => b"i",
            Bitcoin::NumOutputs => b"i",
            Bitcoin::One32 => b"i",
            Bitcoin::OutputScriptHash => b"+1h",
            Bitcoin::OutputValue => b"+1l",
            Bitcoin::ParseLock => b"+ii",
            Bitcoin::ParseSequence => b"+1+****22*22**22*22***22*22**22*22****22*22**22*22***22*22**22*22",
            Bitcoin::PointVerify1 => b"1",
            Bitcoin::ScalarAdd => b"h",
            Bitcoin::ScalarInvert => b"h",
            Bitcoin::ScalarIsZero => b"2",
            Bitcoin::ScalarMultiply => b"h",
            Bitcoin::ScalarMultiplyLambda => b"h",
            Bitcoin::ScalarNegate => b"h",
            Bitcoin::ScalarNormalize => b"h",
            Bitcoin::ScalarSquare => b"h",
            Bitcoin::Scale => b"**hhh",
            Bitcoin::ScriptCMR => b"h",
            Bitcoin::Sha256Block => b"h",
            Bitcoin::Sha256Ctx8Add1 => b"**+1h*+1*ll*+1l*+1i*+1****22*22**22*22***22*22**22*22+1***22*22**22*22*lh",
            Bitcoin::Sha256Ctx8Add128 => b"**+1h*+1*ll*+1l*+1i*+1****22*22**22*22***22*22**22*22+1***22*22**22*22*lh",
            Bitcoin::Sha256Ctx8Add16 => b"**+1h*+1*ll*+1l*+1i*+1****22*22**22*22***22*22**22*22+1***22*22**22*22*lh",
            Bitcoin::Sha256Ctx8Add2 => b"**+1h*+1*ll*+1l*+1i*+1****22*22**22*22***22*22**22*22+1***22*22**22*22*lh",
            Bitcoin::Sha256Ctx8Add256 => b"**+1h*+1*ll*+1l*+1i*+1****22*22**22*22***22*22**22*22+1***22*22**22*22*lh",
            Bitcoin::Sha256Ctx8Add32 => b"**+1h*+1*ll*+1l*+1i*+1****22*22**22*22***22*22**22*22+1***22*22**22*22*lh",
            Bitcoin::Sha256Ctx8Add4 => b"**+1h*+1*ll*+1l*+1i*+1****22*22**22*22***22*22**22*22+1***22*22**22*22*lh",
            Bitcoin::Sha256Ctx8Add512 => b"**+1h*+1*ll*+1l*+1i*+1****22*22**22*22***22*22**22*22+1***22*22**22*22*lh",
            Bitcoin::Sha256Ctx8Add64 => b"**+1h*+1*ll*+1l*+1i*+1****22*22**22*22***22*22**22*22+1***22*22**22*22*lh",
            Bitcoin::Sha256Ctx8Add8 => b"**+1h*+1*ll*+1l*+1i*+1****22*22**22*22***22*22**22*22+1***22*22**22*22*lh",
            Bitcoin::Sha256Ctx8AddBuffer511 => b"**+1h*+1*ll*+1l*+1i*+1****22*22**22*22***22*22**22*22+1***22*22**22*22*lh",
            Bitcoin::Sha256Ctx8Finalize => b"h",
            Bitcoin::Sha256Ctx8Init => b"**+1h*+1*ll*+1l*+1i*+1****22*22**22*22***22*22**22*22+1***22*22**22*22*lh",
            Bitcoin::Sha256Iv => b"h",
            Bitcoin::Subtract32 => b"*2i",
            Bitcoin::Tapbranch => b"+1h",
            Bitcoin::TapleafVersion => b"***22*22**22*22",
            Bitcoin::TotalInputValue => b"l",
            Bitcoin::TotalOutputValue => b"l",
            Bitcoin::TxIsFinal => b"2",
            Bitcoin::TxLockDistance => b"****22*22**22*22***22*22**22*22",
            Bitcoin::TxLockDuration => b"****22*22**22*22***22*22**22*22",
            Bitcoin::TxLockHeight => b"i",
            Bitcoin::TxLockTime => b"i",
            Bitcoin::Verify => b"1",
            Bitcoin::Version => b"i",
        };

        TypeName(name)
    }

    fn encode<W: Write>(&self, w: &mut BitWriter<W>) -> std::io::Result<usize> {
        let (n, len) = match self {
            Bitcoin::Verify => (0, 3),
            Bitcoin::Low32 => (305, 11),
            Bitcoin::Eq32 => (7025, 15),
            Bitcoin::Eq256 => (14056, 16),
            Bitcoin::One32 => (561, 11),
            Bitcoin::FullAdd32 => (2353, 13),
            Bitcoin::Add32 => (2417, 13),
            Bitcoin::FullSubtract32 => (19697, 16),
            Bitcoin::Subtract32 => (39473, 17),
            Bitcoin::FullMultiply32 => (39729, 17),
            Bitcoin::Multiply32 => (39793, 17),
            Bitcoin::Le32 => (639025, 21),
            Bitcoin::Sha256Block => (20, 6),
            Bitcoin::Sha256Iv => (84, 8),
            Bitcoin::Sha256Ctx8Add1 => (170, 9),
            Bitcoin::Sha256Ctx8Add2 => (684, 11),
            Bitcoin::Sha256Ctx8Add4 => (685, 11),
            Bitcoin::Sha256Ctx8Add8 => (5488, 14),
            Bitcoin::Sha256Ctx8Add16 => (5489, 14),
            Bitcoin::Sha256Ctx8Add32 => (5490, 14),
            Bitcoin::Sha256Ctx8Add64 => (5491, 14),
            Bitcoin::Sha256Ctx8Add128 => (10984, 15),
            Bitcoin::Sha256Ctx8Add256 => (10985, 15),
            Bitcoin::Sha256Ctx8Add512 => (10986, 15),
            Bitcoin::Sha256Ctx8AddBuffer511 => (688, 11),
            Bitcoin::Sha256Ctx8Finalize => (689, 11),
            Bitcoin::Sha256Ctx8Init => (690, 11),
            Bitcoin::PointVerify1 => (192, 9),
            Bitcoin::Decompress => (388, 10),
            Bitcoin::LinearVerify1 => (778, 11),
            Bitcoin::LinearCombination1 => (6240, 14),
            Bitcoin::Scale => (3121, 13),
            Bitcoin::Generate => (3122, 13),
            Bitcoin::GejInfinity => (3123, 13),
            Bitcoin::GejNormalize => (6248, 14),
            Bitcoin::GejNegate => (6249, 14),
            Bitcoin::GeNegate => (6250, 14),
            Bitcoin::GejDouble => (6251, 14),
            Bitcoin::GejAdd => (6252, 14),
            Bitcoin::GejGeAddEx => (6253, 14),
            Bitcoin::GejGeAdd => (6254, 14),
            Bitcoin::GejRescale => (6255, 14),
            Bitcoin::GejIsInfinity => (100096, 18),
            Bitcoin::GejXEquiv => (100099, 18),
            Bitcoin::GejYIsOdd => (100100, 18),
            Bitcoin::GejIsOnCurve => (100101, 18),
            Bitcoin::GeIsOnCurve => (100102, 18),
            Bitcoin::ScalarNormalize => (100103, 18),
            Bitcoin::ScalarNegate => (100104, 18),
            Bitcoin::ScalarAdd => (100105, 18),
            Bitcoin::ScalarSquare => (100106, 18),
            Bitcoin::ScalarMultiply => (100107, 18),
            Bitcoin::ScalarMultiplyLambda => (100108, 18),
            Bitcoin::ScalarInvert => (100109, 18),
            Bitcoin::ScalarIsZero => (100110, 18),
            Bitcoin::FeNormalize => (200227, 19),
            Bitcoin::FeNegate => (200228, 19),
            Bitcoin::FeAdd => (200229, 19),
            Bitcoin::FeSquare => (200230, 19),
            Bitcoin::FeMultiply => (200231, 19),
            Bitcoin::FeMultiplyBeta => (200232, 19),
            Bitcoin::FeInvert => (200233, 19),
            Bitcoin::FeSquareRoot => (200234, 19),
            Bitcoin::FeIsZero => (200235, 19),
            Bitcoin::FeIsOdd => (200236, 19),
            Bitcoin::CheckSigVerify => (98, 8),
            Bitcoin::Bip0340Verify => (396, 10),
            Bitcoin::ParseLock => (102, 8),
            Bitcoin::ParseSequence => (412, 10),
            Bitcoin::CheckLockHeight => (24, 5),
            Bitcoin::CheckLockTime => (100, 7),
            Bitcoin::CheckLockDistance => (101, 7),
            Bitcoin::CheckLockDuration => (816, 10),
            Bitcoin::TxLockHeight => (817, 10),
            Bitcoin::TxLockTime => (818, 10),
            Bitcoin::TxLockDistance => (819, 10),
            Bitcoin::TxLockDuration => (1640, 11),
            Bitcoin::TxIsFinal => (1641, 11),
            Bitcoin::ScriptCMR => (26, 5),
            Bitcoin::InternalKey => (108, 7),
            Bitcoin::CurrentIndex => (109, 7),
            Bitcoin::NumInputs => (880, 10),
            Bitcoin::NumOutputs => (881, 10),
            Bitcoin::LockTime => (882, 10),
            Bitcoin::OutputValue => (1768, 11),
            Bitcoin::OutputScriptHash => (1769, 11),
            Bitcoin::TotalOutputValue => (1770, 11),
            Bitcoin::CurrentPrevOutpoint => (1771, 11),
            Bitcoin::CurrentValue => (1772, 11),
            Bitcoin::CurrentSequence => (1774, 11),
            Bitcoin::CurrentAnnexHash => (1775, 11),
            Bitcoin::CurrentScriptSigHash => (28416, 15),
            Bitcoin::InputPrevOutpoint => (28417, 15),
            Bitcoin::InputValue => (28418, 15),
            Bitcoin::InputSequence => (28420, 15),
            Bitcoin::InputAnnexHash => (28421, 15),
            Bitcoin::InputScriptSigHash => (28422, 15),
            Bitcoin::TotalInputValue => (28423, 15),
            Bitcoin::TapleafVersion => (28424, 15),
            Bitcoin::Tapbranch => (28425, 15),
            Bitcoin::Version => (28426, 15),
        };

        w.write_bits_be(n, len)
    }

    fn decode<I: Iterator<Item = u8>>(bits: &mut BitIter<I>) -> Result<Self, Error> {
        decode_bits!(bits, {
            0 => {
                0 => {
                    0 => {Bitcoin::Verify},
                    1 => {
                        0 => {
                            0 => {
                                0 => {},
                                1 => {
                                    0 => {},
                                    1 => {
                                        0 => {
                                            0 => {
                                                0 => {
                                                    0 => {},
                                                    1 => {Bitcoin::Low32}
                                                },
                                                1 => {}
                                            },
                                            1 => {}
                                        },
                                        1 => {}
                                    }
                                }
                            },
                            1 => {}
                        },
                        1 => {
                            0 => {
                                0 => {},
                                1 => {
                                    0 => {},
                                    1 => {
                                        0 => {
                                            0 => {},
                                            1 => {
                                                0 => {},
                                                1 => {
                                                    0 => {},
                                                    1 => {
                                                        0 => {
                                                            0 => {
                                                                0 => {
                                                                    0 => {},
                                                                    1 => {Bitcoin::Eq32}
                                                                },
                                                                1 => {}
                                                            },
                                                            1 => {
                                                                0 => {
                                                                    0 => {
                                                                        0 => {Bitcoin::Eq256},
                                                                        1 => {}
                                                                    },
                                                                    1 => {}
                                                                },
                                                                1 => {}
                                                            }
                                                        },
                                                        1 => {}
                                                    }
                                                }
                                            }
                                        },
                                        1 => {}
                                    }
                                }
                            },
                            1 => {}
                        }
                    }
                },
                1 => {
                    0 => {
                        0 => {
                            0 => {
                                0 => {},
                                1 => {
                                    0 => {},
                                    1 => {
                                        0 => {
                                            0 => {
                                                0 => {
                                                    0 => {},
                                                    1 => {Bitcoin::One32}
                                                },
                                                1 => {}
                                            },
                                            1 => {}
                                        },
                                        1 => {}
                                    }
                                }
                            },
                            1 => {
                                0 => {
                                    0 => {
                                        0 => {},
                                        1 => {
                                            0 => {},
                                            1 => {
                                                0 => {
                                                    0 => {
                                                        0 => {
                                                            0 => {},
                                                            1 => {Bitcoin::FullAdd32}
                                                        },
                                                        1 => {}
                                                    },
                                                    1 => {}
                                                },
                                                1 => {}
                                            }
                                        }
                                    },
                                    1 => {
                                        0 => {},
                                        1 => {
                                            0 => {},
                                            1 => {
                                                0 => {
                                                    0 => {
                                                        0 => {
                                                            0 => {},
                                                            1 => {Bitcoin::Add32}
                                                        },
                                                        1 => {}
                                                    },
                                                    1 => {}
                                                },
                                                1 => {}
                                            }
                                        }
                                    }
                                },
                                1 => {
                                    0 => {
                                        0 => {
                                            0 => {},
                                            1 => {
                                                0 => {},
                                                1 => {
                                                    0 => {},
                                                    1 => {
                                                        0 => {},
                                                        1 => {
                                                            0 => {
                                                                0 => {
                                                                    0 => {
                                                                        0 => {},
                                                                        1 => {Bitcoin::FullSubtract32}
                                                                    },
                                                                    1 => {}
                                                                },
                                                                1 => {}
                                                            },
                                                            1 => {}
                                                        }
                                                    }
                                                }
                                            }
                                        },
                                        1 => {
                                            0 => {
                                                0 => {
                                                    0 => {
                                                        0 => {},
                                                        1 => {
                                                            0 => {},
                                                            1 => {
                                                                0 => {
                                                                    0 => {
                                                                        0 => {
                                                                            0 => {},
                                                                            1 => {Bitcoin::Subtract32}
                                                                        },
                                                                        1 => {}
                                                                    },
                                                                    1 => {}
                                                                },
                                                                1 => {}
                                                            }
                                                        }
                                                    },
                                                    1 => {}
                                                },
                                                1 => {}
                                            },
                                            1 => {
                                                0 => {
                                                    0 => {
                                                        0 => {},
                                                        1 => {
                                                            0 => {},
                                                            1 => {
                                                                0 => {
                                                                    0 => {
                                                                        0 => {
                                                                            0 => {},
                                                                            1 => {Bitcoin::FullMultiply32}
                                                                        },
                                                                        1 => {}
                                                                    },
                                                                    1 => {}
                                                                },
                                                                1 => {}
                                                            }
                                                        }
                                                    },
                                                    1 => {
                                                        0 => {},
                                                        1 => {
                                                            0 => {},
                                                            1 => {
                                                                0 => {
                                                                    0 => {
                                                                        0 => {
                                                                            0 => {},
                                                                            1 => {Bitcoin::Multiply32}
                                                                        },
                                                                        1 => {}
                                                                    },
                                                                    1 => {}
                                                                },
                                                                1 => {}
                                                            }
                                                        }
                                                    }
                                                },
                                                1 => {}
                                            }
                                        }
                                    },
                                    1 => {
                                        0 => {
                                            0 => {
                                                0 => {
                                                    0 => {
                                                        0 => {
                                                            0 => {
                                                                0 => {
                                                                    0 => {
                                                                        0 => {},
                                                                        1 => {
                                                                            0 => {},
                                                                            1 => {
                                                                                0 => {
                                                                                    0 => {
                                                                                        0 => {
                                                                                            0 => {},
                                                                                            1 => {Bitcoin::Le32}
                                                                                        },
                                                                                        1 => {}
                                                                                    },
                                                                                    1 => {}
                                                                                },
                                                                                1 => {}
                                                                            }
                                                                        }
                                                                    },
                                                                    1 => {}
                                                                },
                                                                1 => {}
                                                            },
                                                            1 => {}
                                                        },
                                                        1 => {}
                                                    },
                                                    1 => {}
                                                },
                                                1 => {}
                                            },
                                            1 => {}
                                        },
                                        1 => {}
                                    }
                                }
                            }
                        },
                        1 => {
                            0 => {
                                0 => {Bitcoin::Sha256Block},
                                1 => {
                                    0 => {
                                        0 => {Bitcoin::Sha256Iv},
                                        1 => {
                                            0 => {Bitcoin::Sha256Ctx8Add1},
                                            1 => {
                                                0 => {
                                                    0 => {Bitcoin::Sha256Ctx8Add2},
                                                    1 => {Bitcoin::Sha256Ctx8Add4}
                                                },
                                                1 => {
                                                    0 => {
                                                        0 => {
                                                            0 => {
                                                                0 => {Bitcoin::Sha256Ctx8Add8},
                                                                1 => {Bitcoin::Sha256Ctx8Add16}
                                                            },
                                                            1 => {
                                                                0 => {Bitcoin::Sha256Ctx8Add32},
                                                                1 => {Bitcoin::Sha256Ctx8Add64}
                                                            }
                                                        },
                                                        1 => {
                                                            0 => {
                                                                0 => {
                                                                    0 => {Bitcoin::Sha256Ctx8Add128},
                                                                    1 => {Bitcoin::Sha256Ctx8Add256}
                                                                },
                                                                1 => {
                                                                    0 => {Bitcoin::Sha256Ctx8Add512},
                                                                    1 => {}
                                                                }
                                                            },
                                                            1 => {}
                                                        }
                                                    },
                                                    1 => {}
                                                }
                                            }
                                        }
                                    },
                                    1 => {
                                        0 => {
                                            0 => {
                                                0 => {
                                                    0 => {Bitcoin::Sha256Ctx8AddBuffer511},
                                                    1 => {Bitcoin::Sha256Ctx8Finalize}
                                                },
                                                1 => {
                                                    0 => {Bitcoin::Sha256Ctx8Init},
                                                    1 => {}
                                                }
                                            },
                                            1 => {}
                                        },
                                        1 => {}
                                    }
                                }
                            },
                            1 => {}
                        }
                    },
                    1 => {
                        0 => {
                            0 => {
                                0 => {
                                    0 => {
                                        0 => {
                                            0 => {Bitcoin::PointVerify1},
                                            1 => {}
                                        },
                                        1 => {
                                            0 => {
                                                0 => {Bitcoin::Decompress},
                                                1 => {
                                                    0 => {Bitcoin::LinearVerify1},
                                                    1 => {}
                                                }
                                            },
                                            1 => {
                                                0 => {
                                                    0 => {
                                                        0 => {
                                                            0 => {
                                                                0 => {Bitcoin::LinearCombination1},
                                                                1 => {}
                                                            },
                                                            1 => {Bitcoin::Scale}
                                                        },
                                                        1 => {
                                                            0 => {Bitcoin::Generate},
                                                            1 => {Bitcoin::GejInfinity}
                                                        }
                                                    },
                                                    1 => {
                                                        0 => {
                                                            0 => {
                                                                0 => {Bitcoin::GejNormalize},
                                                                1 => {Bitcoin::GejNegate}
                                                            },
                                                            1 => {
                                                                0 => {Bitcoin::GeNegate},
                                                                1 => {Bitcoin::GejDouble}
                                                            }
                                                        },
                                                        1 => {
                                                            0 => {
                                                                0 => {Bitcoin::GejAdd},
                                                                1 => {Bitcoin::GejGeAddEx}
                                                            },
                                                            1 => {
                                                                0 => {Bitcoin::GejGeAdd},
                                                                1 => {Bitcoin::GejRescale}
                                                            }
                                                        }
                                                    }
                                                },
                                                1 => {
                                                    0 => {
                                                        0 => {
                                                            0 => {
                                                                0 => {
                                                                    0 => {
                                                                        0 => {
                                                                            0 => {
                                                                                0 => {Bitcoin::GejIsInfinity},
                                                                                1 => {}
                                                                            },
                                                                            1 => {
                                                                                0 => {},
                                                                                1 => {Bitcoin::GejXEquiv}
                                                                            }
                                                                        },
                                                                        1 => {
                                                                            0 => {
                                                                                0 => {Bitcoin::GejYIsOdd},
                                                                                1 => {Bitcoin::GejIsOnCurve}
                                                                            },
                                                                            1 => {
                                                                                0 => {Bitcoin::GeIsOnCurve},
                                                                                1 => {Bitcoin::ScalarNormalize}
                                                                            }
                                                                        }
                                                                    },
                                                                    1 => {
                                                                        0 => {
                                                                            0 => {
                                                                                0 => {Bitcoin::ScalarNegate},
                                                                                1 => {Bitcoin::ScalarAdd}
                                                                            },
                                                                            1 => {
                                                                                0 => {Bitcoin::ScalarSquare},
                                                                                1 => {Bitcoin::ScalarMultiply}
                                                                            }
                                                                        },
                                                                        1 => {
                                                                            0 => {
                                                                                0 => {Bitcoin::ScalarMultiplyLambda},
                                                                                1 => {Bitcoin::ScalarInvert}
                                                                            },
                                                                            1 => {
                                                                                0 => {Bitcoin::ScalarIsZero},
                                                                                1 => {}
                                                                            }
                                                                        }
                                                                    }
                                                                },
                                                                1 => {
                                                                    0 => {
                                                                        0 => {
                                                                            0 => {
                                                                                0 => {},
                                                                                1 => {
                                                                                    0 => {},
                                                                                    1 => {Bitcoin::FeNormalize}
                                                                                }
                                                                            },
                                                                            1 => {
                                                                                0 => {
                                                                                    0 => {Bitcoin::FeNegate},
                                                                                    1 => {Bitcoin::FeAdd}
                                                                                },
                                                                                1 => {
                                                                                    0 => {Bitcoin::FeSquare},
                                                                                    1 => {Bitcoin::FeMultiply}
                                                                                }
                                                                            }
                                                                        },
                                                                        1 => {
                                                                            0 => {
                                                                                0 => {
                                                                                    0 => {Bitcoin::FeMultiplyBeta},
                                                                                    1 => {Bitcoin::FeInvert}
                                                                                },
                                                                                1 => {
                                                                                    0 => {Bitcoin::FeSquareRoot},
                                                                                    1 => {Bitcoin::FeIsZero}
                                                                                }
                                                                            },
                                                                            1 => {
                                                                                0 => {
                                                                                    0 => {Bitcoin::FeIsOdd},
                                                                                    1 => {}
                                                                                },
                                                                                1 => {}
                                                                            }
                                                                        }
                                                                    },
                                                                    1 => {}
                                                                }
                                                            },
                                                            1 => {}
                                                        },
                                                        1 => {}
                                                    },
                                                    1 => {}
                                                }
                                            }
                                        }
                                    },
                                    1 => {
                                        0 => {Bitcoin::CheckSigVerify},
                                        1 => {
                                            0 => {
                                                0 => {Bitcoin::Bip0340Verify},
                                                1 => {}
                                            },
                                            1 => {}
                                        }
                                    }
                                },
                                1 => {
                                    0 => {},
                                    1 => {
                                        0 => {Bitcoin::ParseLock},
                                        1 => {
                                            0 => {
                                                0 => {Bitcoin::ParseSequence},
                                                1 => {}
                                            },
                                            1 => {}
                                        }
                                    }
                                }
                            },
                            1 => {}
                        },
                        1 => {}
                    }
                }
            },
            1 => {
                0 => {},
                1 => {
                    0 => {
                        0 => {
                            0 => {Bitcoin::CheckLockHeight},
                            1 => {
                                0 => {
                                    0 => {Bitcoin::CheckLockTime},
                                    1 => {Bitcoin::CheckLockDistance}
                                },
                                1 => {
                                    0 => {
                                        0 => {
                                            0 => {
                                                0 => {Bitcoin::CheckLockDuration},
                                                1 => {Bitcoin::TxLockHeight}
                                            },
                                            1 => {
                                                0 => {Bitcoin::TxLockTime},
                                                1 => {Bitcoin::TxLockDistance}
                                            }
                                        },
                                        1 => {
                                            0 => {
                                                0 => {
                                                    0 => {Bitcoin::TxLockDuration},
                                                    1 => {Bitcoin::TxIsFinal}
                                                },
                                                1 => {}
                                            },
                                            1 => {}
                                        }
                                    },
                                    1 => {}
                                }
                            }
                        },
                        1 => {
                            0 => {Bitcoin::ScriptCMR},
                            1 => {
                                0 => {
                                    0 => {Bitcoin::InternalKey},
                                    1 => {Bitcoin::CurrentIndex}
                                },
                                1 => {
                                    0 => {
                                        0 => {
                                            0 => {
                                                0 => {Bitcoin::NumInputs},
                                                1 => {Bitcoin::NumOutputs}
                                            },
                                            1 => {
                                                0 => {Bitcoin::LockTime},
                                                1 => {}
                                            }
                                        },
                                        1 => {
                                            0 => {
                                                0 => {
                                                    0 => {Bitcoin::OutputValue},
                                                    1 => {Bitcoin::OutputScriptHash}
                                                },
                                                1 => {
                                                    0 => {Bitcoin::TotalOutputValue},
                                                    1 => {Bitcoin::CurrentPrevOutpoint}
                                                }
                                            },
                                            1 => {
                                                0 => {
                                                    0 => {Bitcoin::CurrentValue},
                                                    1 => {}
                                                },
                                                1 => {
                                                    0 => {Bitcoin::CurrentSequence},
                                                    1 => {Bitcoin::CurrentAnnexHash}
                                                }
                                            }
                                        }
                                    },
                                    1 => {
                                        0 => {
                                            0 => {
                                                0 => {
                                                    0 => {
                                                        0 => {
                                                            0 => {
                                                                0 => {
                                                                    0 => {Bitcoin::CurrentScriptSigHash},
                                                                    1 => {Bitcoin::InputPrevOutpoint}
                                                                },
                                                                1 => {
                                                                    0 => {Bitcoin::InputValue},
                                                                    1 => {}
                                                                }
                                                            },
                                                            1 => {
                                                                0 => {
                                                                    0 => {Bitcoin::InputSequence},
                                                                    1 => {Bitcoin::InputAnnexHash}
                                                                },
                                                                1 => {
                                                                    0 => {Bitcoin::InputScriptSigHash},
                                                                    1 => {Bitcoin::TotalInputValue}
                                                                }
                                                            }
                                                        },
                                                        1 => {
                                                            0 => {
                                                                0 => {
                                                                    0 => {Bitcoin::TapleafVersion},
                                                                    1 => {Bitcoin::Tapbranch}
                                                                },
                                                                1 => {
                                                                    0 => {Bitcoin::Version},
                                                                    1 => {}
                                                                }
                                                            },
                                                            1 => {}
                                                        }
                                                    },
                                                    1 => {}
                                                },
                                                1 => {}
                                            },
                                            1 => {}
                                        },
                                        1 => {}
                                    }
                                }
                            }
                        }
                    },
                    1 => {}
                }
            }
        })
    }

    fn c_jet_ptr(&self) -> &dyn Fn(&mut CFrameItem, CFrameItem, &Self::CJetEnvironment) -> bool {
        match self {
            Bitcoin::Add32 => &simplicity_sys::c_jets::jets_wrapper::add_32,
            Bitcoin::Bip0340Verify => &simplicity_sys::c_jets::jets_wrapper::bip_0340_verify,
            Bitcoin::CheckLockDistance => unimplemented!("Bitcoin jets have not yet been implemented."),
            Bitcoin::CheckLockDuration => unimplemented!("Bitcoin jets have not yet been implemented."),
            Bitcoin::CheckLockHeight => unimplemented!("Bitcoin jets have not yet been implemented."),
            Bitcoin::CheckLockTime => unimplemented!("Bitcoin jets have not yet been implemented."),
            Bitcoin::CheckSigVerify => &simplicity_sys::c_jets::jets_wrapper::check_sig_verify,
            Bitcoin::CurrentAnnexHash => unimplemented!("Bitcoin jets have not yet been implemented."),
            Bitcoin::CurrentIndex => unimplemented!("Bitcoin jets have not yet been implemented."),
            Bitcoin::CurrentPrevOutpoint => unimplemented!("Bitcoin jets have not yet been implemented."),
            Bitcoin::CurrentScriptSigHash => unimplemented!("Bitcoin jets have not yet been implemented."),
            Bitcoin::CurrentSequence => unimplemented!("Bitcoin jets have not yet been implemented."),
            Bitcoin::CurrentValue => unimplemented!("Bitcoin jets have not yet been implemented."),
            Bitcoin::Decompress => &simplicity_sys::c_jets::jets_wrapper::decompress,
            Bitcoin::Eq256 => &simplicity_sys::c_jets::jets_wrapper::eq_256,
            Bitcoin::Eq32 => &simplicity_sys::c_jets::jets_wrapper::eq_32,
            Bitcoin::FeAdd => &simplicity_sys::c_jets::jets_wrapper::fe_add,
            Bitcoin::FeInvert => &simplicity_sys::c_jets::jets_wrapper::fe_invert,
            Bitcoin::FeIsOdd => &simplicity_sys::c_jets::jets_wrapper::fe_is_odd,
            Bitcoin::FeIsZero => &simplicity_sys::c_jets::jets_wrapper::fe_is_zero,
            Bitcoin::FeMultiply => &simplicity_sys::c_jets::jets_wrapper::fe_multiply,
            Bitcoin::FeMultiplyBeta => &simplicity_sys::c_jets::jets_wrapper::fe_multiply_beta,
            Bitcoin::FeNegate => &simplicity_sys::c_jets::jets_wrapper::fe_negate,
            Bitcoin::FeNormalize => &simplicity_sys::c_jets::jets_wrapper::fe_normalize,
            Bitcoin::FeSquare => &simplicity_sys::c_jets::jets_wrapper::fe_square,
            Bitcoin::FeSquareRoot => &simplicity_sys::c_jets::jets_wrapper::fe_square_root,
            Bitcoin::FullAdd32 => &simplicity_sys::c_jets::jets_wrapper::full_add_32,
            Bitcoin::FullMultiply32 => &simplicity_sys::c_jets::jets_wrapper::full_multiply_32,
            Bitcoin::FullSubtract32 => &simplicity_sys::c_jets::jets_wrapper::full_subtract_32,
            Bitcoin::GeIsOnCurve => &simplicity_sys::c_jets::jets_wrapper::ge_is_on_curve,
            Bitcoin::GeNegate => &simplicity_sys::c_jets::jets_wrapper::ge_negate,
            Bitcoin::GejAdd => &simplicity_sys::c_jets::jets_wrapper::gej_add,
            Bitcoin::GejDouble => &simplicity_sys::c_jets::jets_wrapper::gej_double,
            Bitcoin::GejGeAdd => &simplicity_sys::c_jets::jets_wrapper::gej_ge_add,
            Bitcoin::GejGeAddEx => &simplicity_sys::c_jets::jets_wrapper::gej_ge_add_ex,
            Bitcoin::GejInfinity => &simplicity_sys::c_jets::jets_wrapper::gej_infinity,
            Bitcoin::GejIsInfinity => &simplicity_sys::c_jets::jets_wrapper::gej_is_infinity,
            Bitcoin::GejIsOnCurve => &simplicity_sys::c_jets::jets_wrapper::gej_is_on_curve,
            Bitcoin::GejNegate => &simplicity_sys::c_jets::jets_wrapper::gej_negate,
            Bitcoin::GejNormalize => &simplicity_sys::c_jets::jets_wrapper::gej_normalize,
            Bitcoin::GejRescale => &simplicity_sys::c_jets::jets_wrapper::gej_rescale,
            Bitcoin::GejXEquiv => &simplicity_sys::c_jets::jets_wrapper::gej_x_equiv,
            Bitcoin::GejYIsOdd => &simplicity_sys::c_jets::jets_wrapper::gej_y_is_odd,
            Bitcoin::Generate => &simplicity_sys::c_jets::jets_wrapper::generate,
            Bitcoin::InputAnnexHash => unimplemented!("Bitcoin jets have not yet been implemented."),
            Bitcoin::InputPrevOutpoint => unimplemented!("Bitcoin jets have not yet been implemented."),
            Bitcoin::InputScriptSigHash => unimplemented!("Bitcoin jets have not yet been implemented."),
            Bitcoin::InputSequence => unimplemented!("Bitcoin jets have not yet been implemented."),
            Bitcoin::InputValue => unimplemented!("Bitcoin jets have not yet been implemented."),
            Bitcoin::InternalKey => unimplemented!("Bitcoin jets have not yet been implemented."),
            Bitcoin::Le32 => &simplicity_sys::c_jets::jets_wrapper::le_32,
            Bitcoin::LinearCombination1 => &simplicity_sys::c_jets::jets_wrapper::linear_combination_1,
            Bitcoin::LinearVerify1 => &simplicity_sys::c_jets::jets_wrapper::linear_verify_1,
            Bitcoin::LockTime => unimplemented!("Bitcoin jets have not yet been implemented."),
            Bitcoin::Low32 => &simplicity_sys::c_jets::jets_wrapper::low_32,
            Bitcoin::Multiply32 => &simplicity_sys::c_jets::jets_wrapper::multiply_32,
            Bitcoin::NumInputs => unimplemented!("Bitcoin jets have not yet been implemented."),
            Bitcoin::NumOutputs => unimplemented!("Bitcoin jets have not yet been implemented."),
            Bitcoin::One32 => &simplicity_sys::c_jets::jets_wrapper::one_32,
            Bitcoin::OutputScriptHash => unimplemented!("Bitcoin jets have not yet been implemented."),
            Bitcoin::OutputValue => unimplemented!("Bitcoin jets have not yet been implemented."),
            Bitcoin::ParseLock => &simplicity_sys::c_jets::jets_wrapper::parse_lock,
            Bitcoin::ParseSequence => &simplicity_sys::c_jets::jets_wrapper::parse_sequence,
            Bitcoin::PointVerify1 => &simplicity_sys::c_jets::jets_wrapper::point_verify_1,
            Bitcoin::ScalarAdd => &simplicity_sys::c_jets::jets_wrapper::scalar_add,
            Bitcoin::ScalarInvert => &simplicity_sys::c_jets::jets_wrapper::scalar_invert,
            Bitcoin::ScalarIsZero => &simplicity_sys::c_jets::jets_wrapper::scalar_is_zero,
            Bitcoin::ScalarMultiply => &simplicity_sys::c_jets::jets_wrapper::scalar_multiply,
            Bitcoin::ScalarMultiplyLambda => &simplicity_sys::c_jets::jets_wrapper::scalar_multiply_lambda,
            Bitcoin::ScalarNegate => &simplicity_sys::c_jets::jets_wrapper::scalar_negate,
            Bitcoin::ScalarNormalize => &simplicity_sys::c_jets::jets_wrapper::scalar_normalize,
            Bitcoin::ScalarSquare => &simplicity_sys::c_jets::jets_wrapper::scalar_square,
            Bitcoin::Scale => &simplicity_sys::c_jets::jets_wrapper::scale,
            Bitcoin::ScriptCMR => unimplemented!("Bitcoin jets have not yet been implemented."),
            Bitcoin::Sha256Block => &simplicity_sys::c_jets::jets_wrapper::sha_256_block,
            Bitcoin::Sha256Ctx8Add1 => &simplicity_sys::c_jets::jets_wrapper::sha_256_ctx_8_add_1,
            Bitcoin::Sha256Ctx8Add128 => &simplicity_sys::c_jets::jets_wrapper::sha_256_ctx_8_add_128,
            Bitcoin::Sha256Ctx8Add16 => &simplicity_sys::c_jets::jets_wrapper::sha_256_ctx_8_add_16,
            Bitcoin::Sha256Ctx8Add2 => &simplicity_sys::c_jets::jets_wrapper::sha_256_ctx_8_add_2,
            Bitcoin::Sha256Ctx8Add256 => &simplicity_sys::c_jets::jets_wrapper::sha_256_ctx_8_add_256,
            Bitcoin::Sha256Ctx8Add32 => &simplicity_sys::c_jets::jets_wrapper::sha_256_ctx_8_add_32,
            Bitcoin::Sha256Ctx8Add4 => &simplicity_sys::c_jets::jets_wrapper::sha_256_ctx_8_add_4,
            Bitcoin::Sha256Ctx8Add512 => &simplicity_sys::c_jets::jets_wrapper::sha_256_ctx_8_add_512,
            Bitcoin::Sha256Ctx8Add64 => &simplicity_sys::c_jets::jets_wrapper::sha_256_ctx_8_add_64,
            Bitcoin::Sha256Ctx8Add8 => &simplicity_sys::c_jets::jets_wrapper::sha_256_ctx_8_add_8,
            Bitcoin::Sha256Ctx8AddBuffer511 => &simplicity_sys::c_jets::jets_wrapper::sha_256_ctx_8_add_buffer_511,
            Bitcoin::Sha256Ctx8Finalize => &simplicity_sys::c_jets::jets_wrapper::sha_256_ctx_8_finalize,
            Bitcoin::Sha256Ctx8Init => &simplicity_sys::c_jets::jets_wrapper::sha_256_ctx_8_init,
            Bitcoin::Sha256Iv => &simplicity_sys::c_jets::jets_wrapper::sha_256_iv,
            Bitcoin::Subtract32 => &simplicity_sys::c_jets::jets_wrapper::subtract_32,
            Bitcoin::Tapbranch => unimplemented!("Bitcoin jets have not yet been implemented."),
            Bitcoin::TapleafVersion => unimplemented!("Bitcoin jets have not yet been implemented."),
            Bitcoin::TotalInputValue => unimplemented!("Bitcoin jets have not yet been implemented."),
            Bitcoin::TotalOutputValue => unimplemented!("Bitcoin jets have not yet been implemented."),
            Bitcoin::TxIsFinal => unimplemented!("Bitcoin jets have not yet been implemented."),
            Bitcoin::TxLockDistance => unimplemented!("Bitcoin jets have not yet been implemented."),
            Bitcoin::TxLockDuration => unimplemented!("Bitcoin jets have not yet been implemented."),
            Bitcoin::TxLockHeight => unimplemented!("Bitcoin jets have not yet been implemented."),
            Bitcoin::TxLockTime => unimplemented!("Bitcoin jets have not yet been implemented."),
            Bitcoin::Verify => &simplicity_sys::c_jets::jets_wrapper::verify,
            Bitcoin::Version => unimplemented!("Bitcoin jets have not yet been implemented."),
        }
    }
}
