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
use std::{fmt, str};
use crate::jet::bitcoin::BitcoinEnv;

/// Bitcoin jet family
#[derive(Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Debug, Hash)]
pub enum Bitcoin {
    Add16,
    Add32,
    Add64,
    Add8,
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
    Decrement16,
    Decrement32,
    Decrement64,
    Decrement8,
    DivMod16,
    DivMod32,
    DivMod64,
    DivMod8,
    Divide16,
    Divide32,
    Divide64,
    Divide8,
    Divides16,
    Divides32,
    Divides64,
    Divides8,
    Eq16,
    Eq256,
    Eq32,
    Eq64,
    Eq8,
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
    FullAdd16,
    FullAdd32,
    FullAdd64,
    FullAdd8,
    FullDecrement16,
    FullDecrement32,
    FullDecrement64,
    FullDecrement8,
    FullIncrement16,
    FullIncrement32,
    FullIncrement64,
    FullIncrement8,
    FullMultiply16,
    FullMultiply32,
    FullMultiply64,
    FullMultiply8,
    FullSubtract16,
    FullSubtract32,
    FullSubtract64,
    FullSubtract8,
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
    Increment16,
    Increment32,
    Increment64,
    Increment8,
    InputAnnexHash,
    InputPrevOutpoint,
    InputScriptSigHash,
    InputSequence,
    InputValue,
    InternalKey,
    IsOne16,
    IsOne32,
    IsOne64,
    IsOne8,
    IsZero16,
    IsZero32,
    IsZero64,
    IsZero8,
    Le16,
    Le32,
    Le64,
    Le8,
    LinearCombination1,
    LinearVerify1,
    LockTime,
    Low16,
    Low32,
    Low64,
    Low8,
    Lt16,
    Lt32,
    Lt64,
    Lt8,
    Max16,
    Max32,
    Max64,
    Max8,
    Median16,
    Median32,
    Median64,
    Median8,
    Min16,
    Min32,
    Min64,
    Min8,
    Modulo16,
    Modulo32,
    Modulo64,
    Modulo8,
    Multiply16,
    Multiply32,
    Multiply64,
    Multiply8,
    Negate16,
    Negate32,
    Negate64,
    Negate8,
    NumInputs,
    NumOutputs,
    One16,
    One32,
    One64,
    One8,
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
    Subtract16,
    Subtract32,
    Subtract64,
    Subtract8,
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
            Bitcoin::Add16 => [
                0x51, 0x7a, 0x52, 0x5b, 0x98, 0xc8, 0x78, 0x60, 0x88, 0x41, 0x82, 0xcf, 0xc0, 0x14,
                0x81, 0x64, 0xaf, 0x35, 0x14, 0x63, 0xe0, 0xfb, 0xac, 0x34, 0x60, 0x65, 0x2b, 0x2f,
                0xb5, 0xed, 0xcf, 0x19,
            ],
            Bitcoin::Add32 => [
                0xe4, 0x04, 0x66, 0xa7, 0xec, 0xf7, 0x1c, 0xe8, 0x62, 0xfb, 0x3c, 0x15, 0x4c, 0x1e,
                0x8f, 0x84, 0x5d, 0x7e, 0x57, 0x07, 0x46, 0x3a, 0x89, 0x45, 0x37, 0xa3, 0x2f, 0xc7,
                0x21, 0x49, 0x00, 0xad,
            ],
            Bitcoin::Add64 => [
                0x81, 0x2e, 0x6e, 0x75, 0x70, 0x12, 0x1f, 0x8c, 0xc3, 0x52, 0xc0, 0x9a, 0x0a, 0xc7,
                0x1a, 0xe6, 0xcc, 0xfe, 0x85, 0x9e, 0x84, 0x97, 0x60, 0xbd, 0x47, 0x8d, 0x29, 0x8a,
                0xea, 0x82, 0xae, 0xb0,
            ],
            Bitcoin::Add8 => [
                0x62, 0x61, 0x3c, 0x74, 0x98, 0xfe, 0x9c, 0x5e, 0x98, 0xa1, 0xf2, 0x4e, 0xe1, 0x07,
                0x0a, 0x02, 0xe9, 0xdc, 0xf2, 0x7f, 0x7f, 0x73, 0x63, 0x8c, 0x0a, 0x8f, 0xce, 0x85,
                0xb7, 0x64, 0x15, 0x1a,
            ],
            Bitcoin::Bip0340Verify => [
                0xc6, 0x19, 0xa5, 0x79, 0x12, 0xb7, 0x6c, 0x30, 0x5d, 0xc9, 0xe2, 0x62, 0xf6, 0x98,
                0xe2, 0x05, 0x06, 0x1f, 0x46, 0x26, 0xc0, 0x70, 0x04, 0x7b, 0x0e, 0x1b, 0xa9, 0x99,
                0xdb, 0xf1, 0x00, 0x6c,
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
                0xfe, 0x5e, 0x35, 0xd8, 0xa4, 0xa8, 0x46, 0xa1, 0x05, 0xac, 0x4b, 0x03, 0xfc, 0xe4,
                0xa9, 0x7a, 0x55, 0xbc, 0x40, 0xb5, 0x50, 0x4d, 0x9a, 0x60, 0x3c, 0x23, 0x9b, 0x05,
                0x59, 0xe3, 0x11, 0x02,
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
            Bitcoin::Decrement16 => [
                0x4a, 0x10, 0x94, 0x9e, 0xb5, 0x98, 0xc0, 0x04, 0xbc, 0xae, 0xaf, 0xb8, 0x2f, 0x02,
                0xac, 0x62, 0x06, 0xc2, 0xa0, 0xf2, 0x32, 0xff, 0x95, 0x40, 0xa4, 0xff, 0x02, 0xa5,
                0x54, 0x71, 0xf8, 0xfa,
            ],
            Bitcoin::Decrement32 => [
                0x29, 0x1e, 0xab, 0x34, 0x34, 0xe2, 0x01, 0x77, 0x5e, 0xcb, 0x68, 0x21, 0x5b, 0x54,
                0x03, 0xfe, 0x8c, 0x9e, 0xef, 0xbe, 0x88, 0x0c, 0xb2, 0xe0, 0x96, 0xe7, 0x60, 0x22,
                0xb8, 0x30, 0x1b, 0x59,
            ],
            Bitcoin::Decrement64 => [
                0xe1, 0x8e, 0xce, 0x15, 0xc9, 0x14, 0xaa, 0x0c, 0x22, 0x2d, 0xec, 0x3b, 0x0c, 0xa0,
                0xab, 0x39, 0xcd, 0x81, 0xed, 0x38, 0x25, 0x6c, 0xf8, 0x77, 0x0b, 0x78, 0x17, 0x3b,
                0xb9, 0xc6, 0xb4, 0x87,
            ],
            Bitcoin::Decrement8 => [
                0xd2, 0xaf, 0xfa, 0xd9, 0xae, 0xa4, 0xfd, 0xff, 0x91, 0x6d, 0xbe, 0x5f, 0x90, 0xa6,
                0xf9, 0x3e, 0x2f, 0x4d, 0x30, 0xba, 0x87, 0x3a, 0x52, 0x81, 0xa4, 0x41, 0xd4, 0xa7,
                0x93, 0x70, 0xbe, 0x96,
            ],
            Bitcoin::DivMod16 => [
                0xf5, 0x09, 0x47, 0x22, 0x26, 0xc5, 0x32, 0x5a, 0x4a, 0x41, 0x7d, 0x68, 0xc9, 0x97,
                0xd6, 0x39, 0x5d, 0xe0, 0x99, 0x29, 0xdd, 0xc2, 0x27, 0x1e, 0xc7, 0x28, 0x20, 0x46,
                0xed, 0xbd, 0xdc, 0x29,
            ],
            Bitcoin::DivMod32 => [
                0xa2, 0x90, 0x83, 0x06, 0x91, 0xf7, 0x3a, 0x5f, 0x40, 0x89, 0x18, 0x6f, 0xce, 0x3b,
                0x22, 0xff, 0x4c, 0xc1, 0x0a, 0xee, 0x1a, 0xb2, 0x58, 0xa4, 0xac, 0x51, 0x86, 0x4d,
                0x12, 0xf4, 0xe3, 0x3b,
            ],
            Bitcoin::DivMod64 => [
                0x64, 0x97, 0x16, 0xf1, 0x59, 0x72, 0xf7, 0xb9, 0xc7, 0x0f, 0x9a, 0xd2, 0x2f, 0x65,
                0xb3, 0x91, 0xff, 0x68, 0x52, 0xac, 0xc8, 0x78, 0x37, 0xbb, 0xce, 0xdd, 0x17, 0x1d,
                0xcb, 0x8b, 0xd3, 0x14,
            ],
            Bitcoin::DivMod8 => [
                0xd9, 0xbe, 0x81, 0xb7, 0x53, 0x49, 0xe3, 0xbb, 0x22, 0x21, 0xc4, 0xd4, 0x5a, 0xe0,
                0x4f, 0x8d, 0xf1, 0xbe, 0x4e, 0x7e, 0x76, 0x8b, 0x36, 0x83, 0xb1, 0xa1, 0x55, 0x68,
                0xb8, 0x31, 0xfe, 0xfb,
            ],
            Bitcoin::Divide16 => [
                0xd3, 0x1f, 0xf5, 0x9b, 0x05, 0x84, 0x17, 0x2f, 0xc1, 0xe4, 0xce, 0x0e, 0x5e, 0x89,
                0xbe, 0x12, 0x80, 0x90, 0x8e, 0x49, 0x47, 0x28, 0xee, 0xd7, 0x53, 0xbf, 0xa7, 0x1f,
                0xb6, 0xd8, 0x51, 0x93,
            ],
            Bitcoin::Divide32 => [
                0xab, 0x97, 0x45, 0xca, 0xf9, 0x3e, 0x6b, 0xaa, 0x48, 0xaf, 0x0e, 0xe3, 0xd2, 0xbe,
                0xf3, 0x9d, 0xb8, 0x0f, 0xe5, 0x4d, 0x06, 0xd2, 0xa8, 0xa6, 0x78, 0x75, 0x6d, 0x07,
                0x75, 0x32, 0x5f, 0xc5,
            ],
            Bitcoin::Divide64 => [
                0xae, 0x18, 0xfb, 0xff, 0xb0, 0x4e, 0x5c, 0x0d, 0x9f, 0xe9, 0x73, 0xa4, 0x5d, 0xcc,
                0x9a, 0xcd, 0x0e, 0xdd, 0xe7, 0x74, 0xb8, 0x0a, 0x6d, 0xb6, 0x63, 0x91, 0x23, 0x75,
                0x63, 0x46, 0xde, 0xec,
            ],
            Bitcoin::Divide8 => [
                0x61, 0x37, 0x2c, 0x0c, 0xd4, 0xa7, 0xe4, 0x78, 0x2d, 0x01, 0xd5, 0xb9, 0xce, 0x03,
                0x4d, 0xa9, 0x2c, 0x8a, 0x35, 0x13, 0x84, 0xcd, 0x28, 0xd6, 0x48, 0x48, 0x47, 0x93,
                0xc7, 0x08, 0x66, 0x39,
            ],
            Bitcoin::Divides16 => [
                0xf6, 0x6f, 0x19, 0x64, 0x09, 0x34, 0xa8, 0x25, 0xc4, 0x66, 0x08, 0xcd, 0x06, 0x5d,
                0xca, 0x61, 0x5d, 0x9c, 0xb8, 0x8b, 0x6d, 0x6e, 0xeb, 0xd6, 0x3c, 0xde, 0x22, 0x12,
                0x9b, 0xb4, 0xf6, 0xb4,
            ],
            Bitcoin::Divides32 => [
                0x38, 0x59, 0x74, 0x5d, 0x44, 0x09, 0x64, 0xa8, 0x80, 0x2e, 0xbe, 0x32, 0xb1, 0x0c,
                0x56, 0xf1, 0x86, 0x52, 0xd8, 0xe0, 0xde, 0xd3, 0xc8, 0x8d, 0xe0, 0x32, 0x76, 0xbf,
                0xb3, 0xcf, 0x15, 0x63,
            ],
            Bitcoin::Divides64 => [
                0x38, 0xa5, 0xd0, 0x32, 0xe9, 0x2f, 0x57, 0x8f, 0x4d, 0x39, 0xa3, 0x19, 0x2a, 0xce,
                0x98, 0xc7, 0x50, 0xfa, 0x1c, 0x4b, 0x13, 0x58, 0xa6, 0x0e, 0xec, 0x9a, 0x23, 0xc6,
                0x93, 0x47, 0x7e, 0x54,
            ],
            Bitcoin::Divides8 => [
                0x54, 0x6b, 0x4e, 0x70, 0x78, 0xa7, 0xd4, 0xf2, 0x7b, 0x44, 0xe6, 0x50, 0xdc, 0x29,
                0xb3, 0x61, 0x42, 0x97, 0xb7, 0xea, 0x32, 0xab, 0x9a, 0x94, 0x89, 0xac, 0xcc, 0x81,
                0xfc, 0x8d, 0xb6, 0xcd,
            ],
            Bitcoin::Eq16 => [
                0x9a, 0x5e, 0x8d, 0xb0, 0xdd, 0x21, 0xfb, 0xbc, 0x8c, 0x6a, 0x57, 0xa5, 0xf5, 0x49,
                0x61, 0x83, 0xca, 0xd7, 0x88, 0xed, 0xb8, 0x9c, 0x81, 0x59, 0x9d, 0x77, 0x09, 0xf3,
                0xb1, 0x97, 0x1e, 0x81,
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
            Bitcoin::Eq64 => [
                0xdb, 0xd0, 0xa3, 0x5e, 0x1f, 0x78, 0xe6, 0x1a, 0xd6, 0x75, 0xbd, 0xc1, 0x46, 0x29,
                0xc0, 0xd8, 0x32, 0xae, 0x1d, 0xe6, 0x6b, 0xd7, 0x73, 0xaa, 0x5c, 0xb3, 0x15, 0x79,
                0x81, 0x4c, 0x16, 0xba,
            ],
            Bitcoin::Eq8 => [
                0xcb, 0xe1, 0xb7, 0x99, 0xb0, 0x70, 0xeb, 0xad, 0xb0, 0x3d, 0x6a, 0x2e, 0xb3, 0x16,
                0x2c, 0xf5, 0x2c, 0xa0, 0x15, 0xc9, 0xb6, 0xc6, 0x95, 0x90, 0x31, 0x52, 0xc1, 0xe3,
                0x48, 0x7d, 0xca, 0x3d,
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
            Bitcoin::FullAdd16 => [
                0xf8, 0xe1, 0x52, 0xa9, 0x0a, 0xd9, 0x2b, 0x82, 0x05, 0xc5, 0x56, 0x98, 0x21, 0xef,
                0x09, 0x79, 0x90, 0xd9, 0xf7, 0xf9, 0x8f, 0x21, 0xca, 0xa8, 0xd9, 0xad, 0x55, 0xb5,
                0xed, 0xbc, 0xb6, 0x4d,
            ],
            Bitcoin::FullAdd32 => [
                0x47, 0x27, 0x36, 0x1e, 0xa0, 0x03, 0xc1, 0xa4, 0x83, 0xe5, 0x75, 0x05, 0xcf, 0x5b,
                0x40, 0x5a, 0x82, 0x27, 0xda, 0x1a, 0xdd, 0xc4, 0x7e, 0x2b, 0x01, 0x6c, 0x2d, 0x09,
                0xbe, 0x04, 0x7f, 0xe8,
            ],
            Bitcoin::FullAdd64 => [
                0x8e, 0x29, 0x53, 0x1e, 0xa3, 0x0a, 0x34, 0xc0, 0x72, 0x97, 0x86, 0x07, 0x7f, 0x5d,
                0xc3, 0x79, 0xcb, 0x22, 0x45, 0xfd, 0xe4, 0x41, 0xdb, 0x16, 0xa6, 0x85, 0x6e, 0x26,
                0x80, 0x3d, 0x26, 0xb9,
            ],
            Bitcoin::FullAdd8 => [
                0x3f, 0xd9, 0x59, 0x05, 0x5a, 0x7b, 0x6f, 0x51, 0xb1, 0x47, 0x44, 0x75, 0x37, 0x66,
                0x02, 0x8f, 0x51, 0x83, 0x1b, 0xbd, 0x7c, 0xf7, 0x0e, 0x1e, 0xcc, 0x70, 0x06, 0x0d,
                0xfc, 0xcf, 0xb6, 0x4c,
            ],
            Bitcoin::FullDecrement16 => [
                0xb8, 0x35, 0xbc, 0xde, 0x81, 0x20, 0x7b, 0x8a, 0x1f, 0x68, 0x6b, 0xb5, 0xad, 0x00,
                0x8d, 0xd7, 0xf2, 0x7f, 0xfb, 0xa2, 0xd9, 0x1f, 0xab, 0x34, 0x63, 0x52, 0xc4, 0x11,
                0x8a, 0x13, 0xba, 0x57,
            ],
            Bitcoin::FullDecrement32 => [
                0x91, 0xde, 0x80, 0x21, 0xe3, 0xae, 0x86, 0x0b, 0xd8, 0x27, 0x04, 0xb2, 0xe5, 0x14,
                0x9d, 0xfe, 0x62, 0xb7, 0x4e, 0x1a, 0x6c, 0x71, 0xdb, 0xf0, 0xcb, 0xea, 0xbb, 0x7b,
                0x9e, 0xb2, 0x6a, 0x2b,
            ],
            Bitcoin::FullDecrement64 => [
                0x9e, 0x85, 0xc0, 0x20, 0xcd, 0x4d, 0xbb, 0x7a, 0x5d, 0xfb, 0x69, 0x7a, 0x44, 0x4c,
                0x06, 0x74, 0x8e, 0x22, 0x87, 0xae, 0x22, 0x37, 0x81, 0x3b, 0x1c, 0x5f, 0xa4, 0xa5,
                0x95, 0x8e, 0x6f, 0x1b,
            ],
            Bitcoin::FullDecrement8 => [
                0x16, 0xdb, 0x3f, 0x6d, 0x60, 0x02, 0xd0, 0x70, 0x51, 0xff, 0xf6, 0xf4, 0x94, 0x04,
                0x66, 0x61, 0xf0, 0x38, 0x79, 0xd6, 0x36, 0x54, 0x67, 0xfc, 0xaa, 0xf9, 0xab, 0xab,
                0xa0, 0xc7, 0xf2, 0x81,
            ],
            Bitcoin::FullIncrement16 => [
                0x08, 0xf3, 0xe2, 0x48, 0x41, 0xfe, 0x94, 0x8d, 0x15, 0x29, 0x76, 0x52, 0x0f, 0x32,
                0xdd, 0x12, 0xf7, 0x5c, 0x38, 0x2a, 0xb9, 0xf5, 0x96, 0x6a, 0x16, 0x2c, 0x05, 0x06,
                0xe2, 0xb3, 0x7b, 0x55,
            ],
            Bitcoin::FullIncrement32 => [
                0xff, 0x69, 0xcd, 0xe4, 0x86, 0xc1, 0x91, 0xf4, 0x26, 0x60, 0x7c, 0xaa, 0xdb, 0x45,
                0xaa, 0x4b, 0x82, 0xd2, 0x1f, 0xc3, 0x5f, 0x99, 0xba, 0x0d, 0x7f, 0x56, 0xa8, 0x89,
                0x4c, 0x24, 0xa2, 0x6c,
            ],
            Bitcoin::FullIncrement64 => [
                0x4f, 0x08, 0x0c, 0x7a, 0xb1, 0x90, 0xbb, 0x41, 0xc2, 0x18, 0x8a, 0x37, 0xa4, 0xb3,
                0xb5, 0x2d, 0x6f, 0x75, 0x66, 0x86, 0x1f, 0x5b, 0x46, 0xa9, 0x80, 0x2d, 0xbe, 0xb1,
                0x42, 0x35, 0x0a, 0xe0,
            ],
            Bitcoin::FullIncrement8 => [
                0xba, 0x53, 0x0e, 0x83, 0x49, 0xb1, 0x3d, 0x90, 0x27, 0xff, 0x14, 0x16, 0xa4, 0x12,
                0x12, 0x1f, 0x9b, 0x6b, 0x5e, 0x3c, 0x62, 0xea, 0xa1, 0x18, 0x2a, 0xd2, 0x44, 0xb0,
                0xf7, 0x0f, 0x12, 0xc5,
            ],
            Bitcoin::FullMultiply16 => [
                0x78, 0x3d, 0x05, 0xcf, 0xb8, 0x4d, 0x0c, 0x91, 0x76, 0x60, 0x38, 0x26, 0xc1, 0x6c,
                0x5c, 0xf7, 0x30, 0xd0, 0x1c, 0x50, 0xa1, 0x69, 0x29, 0x79, 0xf7, 0x92, 0xe0, 0x98,
                0xf6, 0x99, 0x80, 0x12,
            ],
            Bitcoin::FullMultiply32 => [
                0xaa, 0xc2, 0x53, 0x61, 0xe5, 0x98, 0xe3, 0x54, 0x38, 0xb9, 0x18, 0xb5, 0x8f, 0xd2,
                0xce, 0xf4, 0xdb, 0x3c, 0x5d, 0x8c, 0x5e, 0x63, 0xaa, 0x4f, 0x25, 0xe9, 0xce, 0xc0,
                0xcf, 0xd9, 0xdf, 0xb1,
            ],
            Bitcoin::FullMultiply64 => [
                0x4e, 0x7b, 0xe1, 0x50, 0x1c, 0xda, 0x15, 0xae, 0x5c, 0xfb, 0x21, 0xee, 0xec, 0xe0,
                0x66, 0xe1, 0x02, 0x0e, 0x0f, 0xd9, 0xb3, 0xe8, 0x36, 0x5b, 0x68, 0x14, 0x02, 0x70,
                0xf9, 0xbc, 0x69, 0x47,
            ],
            Bitcoin::FullMultiply8 => [
                0xd5, 0xe4, 0xc5, 0x6d, 0xe7, 0x83, 0x94, 0x0d, 0xd6, 0xb6, 0xec, 0xe1, 0x3e, 0xa5,
                0x8e, 0xf7, 0x9e, 0x1b, 0x70, 0x84, 0xba, 0x2c, 0x0c, 0xc5, 0x71, 0x20, 0xb7, 0xc5,
                0xc6, 0x2b, 0x3f, 0xdc,
            ],
            Bitcoin::FullSubtract16 => [
                0xfc, 0x6d, 0x4c, 0xd4, 0xa9, 0x9d, 0xbc, 0x0e, 0x01, 0xa5, 0x1e, 0x3d, 0x98, 0x6d,
                0x6b, 0x04, 0x1c, 0x65, 0x57, 0xb8, 0xfc, 0x2e, 0x8b, 0x8c, 0x2f, 0xb6, 0xd2, 0x29,
                0x2a, 0x31, 0x12, 0x61,
            ],
            Bitcoin::FullSubtract32 => [
                0x6d, 0x96, 0xf6, 0x8a, 0x94, 0x5c, 0x22, 0xe7, 0x62, 0x11, 0x5c, 0x09, 0x42, 0x97,
                0xb1, 0x94, 0xbe, 0xdc, 0x0c, 0xe5, 0xa0, 0xc9, 0x2d, 0xb6, 0x4b, 0x83, 0x0a, 0x18,
                0xb4, 0x4d, 0xf0, 0xd0,
            ],
            Bitcoin::FullSubtract64 => [
                0xcd, 0x12, 0x1b, 0x7f, 0x43, 0x84, 0x0f, 0x96, 0xc4, 0x0a, 0xa4, 0x05, 0x6d, 0xef,
                0x80, 0xd1, 0x77, 0xb6, 0x19, 0xb2, 0x50, 0x5a, 0x94, 0x7a, 0x21, 0x47, 0x55, 0xfa,
                0x83, 0x2c, 0x7f, 0x0c,
            ],
            Bitcoin::FullSubtract8 => [
                0x11, 0x62, 0x66, 0x81, 0x54, 0x66, 0xfe, 0x6e, 0x6d, 0x47, 0x57, 0xf6, 0xb8, 0xad,
                0xe3, 0x34, 0xbe, 0xcd, 0xa9, 0xdb, 0x8c, 0x4e, 0xd7, 0xe3, 0x28, 0x13, 0x1f, 0x36,
                0x98, 0x08, 0x67, 0xf2,
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
                0x86, 0x0a, 0x61, 0x5b, 0xb2, 0x5d, 0x22, 0xc1, 0x0a, 0x04, 0x48, 0x72, 0xd1, 0xb8,
                0xfb, 0x04, 0x98, 0x25, 0x86, 0x86, 0x28, 0x81, 0x81, 0x9c, 0xed, 0x25, 0xd6, 0x75,
                0xdc, 0x4f, 0x7d, 0xfe,
            ],
            Bitcoin::GejDouble => [
                0x73, 0x2c, 0x6a, 0xb2, 0x5e, 0xab, 0x89, 0x66, 0x8d, 0x0c, 0xe2, 0x1c, 0x5b, 0x36,
                0x50, 0x18, 0x83, 0xb8, 0xdb, 0x67, 0x86, 0x4f, 0xf3, 0x4f, 0x98, 0x1a, 0x56, 0x8e,
                0xb9, 0xc5, 0xe8, 0xf0,
            ],
            Bitcoin::GejGeAdd => [
                0xd2, 0x89, 0x43, 0x14, 0xa4, 0x8e, 0xa3, 0xf7, 0x15, 0x88, 0x91, 0x07, 0x48, 0xb6,
                0x75, 0x3a, 0x9e, 0x53, 0x9f, 0xf2, 0xb3, 0x6d, 0x1b, 0xf0, 0x89, 0x74, 0x93, 0xdc,
                0x14, 0x0a, 0xa3, 0xce,
            ],
            Bitcoin::GejGeAddEx => [
                0x86, 0x64, 0x35, 0x4f, 0x50, 0x56, 0x65, 0xc8, 0xbe, 0x4b, 0x3b, 0xc1, 0xfa, 0x08,
                0x6f, 0x4f, 0x02, 0xf3, 0xaf, 0x69, 0x1d, 0x20, 0x9e, 0x85, 0x7c, 0x6f, 0x61, 0x5b,
                0x0d, 0xc4, 0x80, 0x32,
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
                0x31, 0x78, 0x42, 0xd3, 0xdd, 0x20, 0x4c, 0x31, 0xc5, 0xd8, 0x34, 0x86, 0x94, 0x0d,
                0x15, 0xbb, 0x6a, 0x05, 0x3e, 0x3d, 0x25, 0x61, 0xee, 0x13, 0x6e, 0xa9, 0x1e, 0x74,
                0x67, 0x74, 0x41, 0xae,
            ],
            Bitcoin::Increment16 => [
                0x89, 0xca, 0xc7, 0x0e, 0x2b, 0xf1, 0x4c, 0xd4, 0xd4, 0x32, 0x75, 0xa0, 0x13, 0x5a,
                0x9f, 0xab, 0xc0, 0xeb, 0x5a, 0x33, 0xf6, 0x2d, 0x0f, 0x4f, 0xa4, 0x4a, 0x3c, 0xa7,
                0x3e, 0xda, 0x85, 0x27,
            ],
            Bitcoin::Increment32 => [
                0x44, 0xf6, 0x64, 0x2e, 0x7b, 0x8d, 0xe6, 0x98, 0x7b, 0x5f, 0x1e, 0x9e, 0x2f, 0x2e,
                0x28, 0x4a, 0x4c, 0xcb, 0xbc, 0x3c, 0x75, 0x5f, 0x23, 0x11, 0xc3, 0x4b, 0x50, 0xf9,
                0x4f, 0xa4, 0x48, 0xbe,
            ],
            Bitcoin::Increment64 => [
                0xe0, 0xb2, 0x61, 0x72, 0x28, 0x67, 0x29, 0xf5, 0xcd, 0xaf, 0x25, 0x16, 0x18, 0xb9,
                0x9e, 0x8e, 0xab, 0x93, 0xd8, 0x4a, 0xb9, 0xba, 0x87, 0x03, 0x06, 0xe6, 0x4d, 0xbc,
                0x5e, 0x3e, 0x09, 0x3d,
            ],
            Bitcoin::Increment8 => [
                0x71, 0xd9, 0x4f, 0xdb, 0x37, 0x95, 0x9c, 0x9a, 0x89, 0xcc, 0x1b, 0x79, 0x71, 0x2c,
                0xa1, 0x67, 0xda, 0xea, 0x47, 0xbe, 0xf8, 0x5f, 0x92, 0xd4, 0x06, 0x6b, 0x6e, 0x94,
                0xcc, 0x16, 0x16, 0xb7,
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
            Bitcoin::IsOne16 => [
                0x67, 0x31, 0xde, 0x96, 0x8d, 0x5c, 0xa6, 0xd1, 0x96, 0xd9, 0x0f, 0xb9, 0x3e, 0x23,
                0x2e, 0x20, 0x90, 0x1b, 0x49, 0x34, 0xfd, 0x17, 0xb7, 0x21, 0x14, 0x96, 0x92, 0xf8,
                0xfc, 0xa4, 0x3a, 0x4f,
            ],
            Bitcoin::IsOne32 => [
                0xea, 0x1a, 0x8c, 0xe7, 0x2f, 0x57, 0xbe, 0x4e, 0x29, 0x11, 0xb9, 0x14, 0xe2, 0x06,
                0x47, 0xda, 0xdc, 0xec, 0x87, 0xad, 0x13, 0x5c, 0xdf, 0x64, 0x3b, 0x67, 0x7f, 0xc2,
                0x14, 0x9d, 0xd6, 0x65,
            ],
            Bitcoin::IsOne64 => [
                0xf2, 0x8a, 0xe2, 0x7a, 0x4e, 0x4b, 0xeb, 0x42, 0xeb, 0x71, 0x9c, 0x5e, 0xae, 0xa4,
                0xc1, 0xaf, 0xac, 0x66, 0x8b, 0xdc, 0x08, 0x6a, 0x5f, 0x15, 0x4e, 0xb5, 0x79, 0x13,
                0x4f, 0xc2, 0x06, 0x42,
            ],
            Bitcoin::IsOne8 => [
                0xf1, 0xa2, 0x81, 0xd5, 0x56, 0xbf, 0x41, 0xcd, 0xa8, 0x78, 0x6c, 0x04, 0x53, 0x0e,
                0x32, 0x19, 0xfd, 0x58, 0x2f, 0xba, 0x2d, 0xff, 0x4e, 0x99, 0xbd, 0x27, 0x5d, 0x7f,
                0xf7, 0x9a, 0x45, 0x0f,
            ],
            Bitcoin::IsZero16 => [
                0x58, 0x22, 0x9c, 0x26, 0x64, 0x15, 0xd1, 0x05, 0xcf, 0x22, 0xb4, 0x2a, 0x9e, 0xde,
                0x72, 0x51, 0x48, 0xb3, 0xbc, 0x01, 0x76, 0x47, 0x5d, 0xe6, 0x2a, 0x74, 0x2d, 0x8e,
                0x11, 0xb5, 0x58, 0x39,
            ],
            Bitcoin::IsZero32 => [
                0x3d, 0x03, 0xf7, 0x9a, 0xda, 0x94, 0x9a, 0x47, 0xc4, 0x6e, 0x4e, 0x97, 0x36, 0xb5,
                0x2d, 0x2a, 0xc1, 0x93, 0x57, 0xd6, 0xca, 0xe9, 0x71, 0x0d, 0xd6, 0xde, 0xd8, 0x2d,
                0x12, 0x0a, 0xa5, 0xb5,
            ],
            Bitcoin::IsZero64 => [
                0xb4, 0x39, 0x4c, 0x5d, 0x5f, 0xd1, 0xdf, 0x3b, 0x6d, 0x02, 0x48, 0x95, 0x3f, 0xb2,
                0x61, 0x4c, 0x86, 0x7f, 0x96, 0xee, 0x4e, 0x0b, 0x05, 0x23, 0x0a, 0xe3, 0x52, 0x88,
                0x7f, 0x38, 0x32, 0x46,
            ],
            Bitcoin::IsZero8 => [
                0xdc, 0x3a, 0xae, 0xa2, 0x21, 0x96, 0xcc, 0x94, 0x94, 0xce, 0xe4, 0x24, 0xdc, 0x71,
                0x60, 0xad, 0x52, 0x8c, 0x62, 0xd7, 0x4d, 0xdf, 0x99, 0xd2, 0xdb, 0x9e, 0x7b, 0x5e,
                0x7e, 0xd2, 0xa0, 0x1b,
            ],
            Bitcoin::Le16 => [
                0x17, 0x43, 0xd1, 0xbc, 0x39, 0x38, 0xed, 0xc8, 0xdc, 0x65, 0x78, 0xa3, 0x52, 0xc4,
                0xf3, 0xec, 0x1f, 0x82, 0xea, 0xae, 0xfe, 0xa9, 0x33, 0x61, 0x89, 0xb9, 0xe2, 0x5c,
                0x3f, 0x13, 0x29, 0xf4,
            ],
            Bitcoin::Le32 => [
                0xea, 0x77, 0x2c, 0x0b, 0x5a, 0xec, 0xde, 0x7d, 0xc1, 0x6e, 0x3f, 0x1f, 0x95, 0x27,
                0x89, 0xf0, 0x13, 0x70, 0x89, 0x04, 0xce, 0xaa, 0x62, 0xcf, 0xdc, 0xf6, 0x4d, 0x30,
                0xa3, 0x91, 0x17, 0xaf,
            ],
            Bitcoin::Le64 => [
                0x37, 0xb6, 0x54, 0xea, 0xd2, 0x33, 0xc3, 0xef, 0xb8, 0x0b, 0x4f, 0x88, 0xa6, 0x13,
                0xba, 0x70, 0x24, 0xd9, 0x8e, 0xb3, 0x1a, 0x2c, 0x3e, 0xdc, 0x93, 0xb5, 0x1a, 0xa4,
                0x60, 0xbc, 0xf7, 0x83,
            ],
            Bitcoin::Le8 => [
                0xff, 0xdf, 0x7f, 0x6a, 0x8d, 0x3b, 0x2c, 0x1b, 0x06, 0xbd, 0x86, 0x40, 0xbd, 0x98,
                0xda, 0xa8, 0x9d, 0x2f, 0xcc, 0xa3, 0xa8, 0xb0, 0xc5, 0x70, 0x43, 0xea, 0xc2, 0x9a,
                0x05, 0xe7, 0xf2, 0xd3,
            ],
            Bitcoin::LinearCombination1 => [
                0x35, 0xcb, 0xfa, 0x56, 0x1b, 0x44, 0xf4, 0x48, 0xdc, 0x84, 0x3a, 0xa6, 0x05, 0x2a,
                0xdd, 0x1a, 0x4f, 0xf6, 0xfb, 0x56, 0xd6, 0x86, 0xc3, 0x21, 0xc5, 0xf8, 0x3b, 0x55,
                0x25, 0xef, 0xe1, 0x8b,
            ],
            Bitcoin::LinearVerify1 => [
                0x34, 0xae, 0x24, 0x21, 0x03, 0xad, 0x8c, 0xf1, 0x09, 0x82, 0x44, 0x44, 0x13, 0xdd,
                0x0d, 0xcb, 0x57, 0x77, 0x48, 0x67, 0x90, 0xdd, 0x21, 0x98, 0xeb, 0xbb, 0xe6, 0x4a,
                0x5a, 0xef, 0x0a, 0x35,
            ],
            Bitcoin::LockTime => [
                0xb0, 0x47, 0x51, 0xf2, 0x56, 0x9e, 0xc4, 0xff, 0x8d, 0x4e, 0xda, 0x91, 0xfc, 0x39,
                0x8b, 0xac, 0x4c, 0xda, 0xc4, 0xc9, 0x6a, 0x40, 0x77, 0xcc, 0xf4, 0x1c, 0xd0, 0xf3,
                0xef, 0x73, 0x58, 0x1b,
            ],
            Bitcoin::Low16 => [
                0x11, 0xd9, 0xe7, 0xcd, 0x36, 0x81, 0x3e, 0x73, 0xa9, 0x47, 0x92, 0xcf, 0xae, 0x99,
                0x63, 0xef, 0x4f, 0x64, 0x64, 0x8d, 0xf4, 0x8a, 0xac, 0xff, 0xe5, 0x7f, 0xb7, 0x0f,
                0x4e, 0x19, 0xac, 0xac,
            ],
            Bitcoin::Low32 => [
                0x96, 0xbe, 0x53, 0x34, 0x5d, 0x52, 0x14, 0xb0, 0x05, 0xc7, 0xfc, 0x5d, 0xe4, 0x0a,
                0x92, 0x62, 0x56, 0x60, 0x41, 0x40, 0x63, 0x35, 0x27, 0xc1, 0xd0, 0x2b, 0xe0, 0xd5,
                0xf6, 0x0c, 0xc2, 0xad,
            ],
            Bitcoin::Low64 => [
                0x61, 0x2a, 0x86, 0xb8, 0x22, 0x44, 0x94, 0xd8, 0x23, 0x0e, 0xe1, 0x16, 0xba, 0x47,
                0x89, 0x46, 0x4c, 0x57, 0xa7, 0xe0, 0x58, 0x52, 0x95, 0xb6, 0x1e, 0xc5, 0xc9, 0x6f,
                0x59, 0x83, 0x77, 0x60,
            ],
            Bitcoin::Low8 => [
                0x43, 0x95, 0xde, 0x24, 0xfd, 0x70, 0xf5, 0xc8, 0x62, 0xfe, 0xe6, 0x8c, 0x6f, 0x67,
                0xd6, 0x17, 0x12, 0xd8, 0xd6, 0x00, 0x3a, 0x38, 0xe1, 0xbd, 0x85, 0xbe, 0x4b, 0x94,
                0x52, 0x69, 0x92, 0xba,
            ],
            Bitcoin::Lt16 => [
                0xf9, 0xd9, 0xee, 0xbe, 0xc9, 0xc2, 0x3b, 0x0b, 0x9e, 0xb8, 0xfd, 0x72, 0x2a, 0x31,
                0x92, 0xcf, 0x7d, 0x28, 0x7d, 0xaa, 0xcf, 0xa5, 0x3e, 0x5d, 0xdc, 0xe7, 0x1b, 0x26,
                0x92, 0x94, 0x0f, 0x1c,
            ],
            Bitcoin::Lt32 => [
                0xc5, 0x23, 0x3b, 0x33, 0x0d, 0xec, 0x92, 0x06, 0x5e, 0x9e, 0xd3, 0x2d, 0x20, 0xd4,
                0xe0, 0xcf, 0xfd, 0x97, 0x56, 0xef, 0x11, 0xc8, 0x43, 0x86, 0x9f, 0xa7, 0x54, 0x37,
                0xef, 0x00, 0xda, 0xaf,
            ],
            Bitcoin::Lt64 => [
                0x3b, 0x8c, 0xbf, 0xda, 0x60, 0x38, 0xef, 0x26, 0x95, 0x06, 0x03, 0xdd, 0xd8, 0x61,
                0x9c, 0x19, 0x7f, 0x8d, 0xbe, 0x4e, 0x15, 0x4c, 0x15, 0x0b, 0x93, 0xd3, 0x8d, 0x0a,
                0xb9, 0x68, 0xf7, 0x67,
            ],
            Bitcoin::Lt8 => [
                0x4e, 0xe0, 0x55, 0x24, 0x23, 0xc4, 0x59, 0x57, 0x5f, 0xca, 0x1b, 0x39, 0xfc, 0xdc,
                0xf0, 0x56, 0x6f, 0xa8, 0x82, 0x1f, 0x14, 0x0c, 0x0a, 0x3c, 0xa4, 0x71, 0x35, 0xbb,
                0x3c, 0xd6, 0xa1, 0xf9,
            ],
            Bitcoin::Max16 => [
                0x13, 0xc1, 0xad, 0xf5, 0x49, 0xc7, 0x73, 0x0b, 0x87, 0x43, 0xf8, 0x95, 0x06, 0x4f,
                0xa4, 0xab, 0xd7, 0x79, 0xf4, 0xb4, 0x1d, 0x6d, 0xf7, 0xc6, 0x33, 0x08, 0x71, 0xa3,
                0x7a, 0xa7, 0x8c, 0xd7,
            ],
            Bitcoin::Max32 => [
                0xc7, 0xf9, 0xaa, 0x5b, 0x7e, 0x5e, 0x03, 0xeb, 0xad, 0x3b, 0xec, 0x46, 0xe7, 0x73,
                0x45, 0x9f, 0xf8, 0x6a, 0x73, 0x9f, 0x9a, 0x09, 0x01, 0x9e, 0xcc, 0x0b, 0x6c, 0x3c,
                0xdf, 0xf4, 0x1a, 0x7b,
            ],
            Bitcoin::Max64 => [
                0x12, 0x7d, 0xd5, 0x9e, 0x06, 0xc5, 0xd6, 0x93, 0x04, 0xeb, 0x92, 0xcf, 0xd6, 0xb4,
                0xf3, 0x65, 0xb7, 0xc2, 0xcd, 0x62, 0x5e, 0x90, 0x45, 0xbe, 0xde, 0xab, 0xf2, 0x31,
                0xc0, 0x14, 0xe8, 0x60,
            ],
            Bitcoin::Max8 => [
                0xaf, 0xe3, 0xc6, 0x8d, 0x17, 0x39, 0x5d, 0xea, 0x61, 0x8e, 0x15, 0xbd, 0x34, 0xae,
                0xad, 0xb8, 0xcb, 0xfb, 0xc8, 0x56, 0x00, 0x71, 0xfb, 0xd1, 0x61, 0x4d, 0xa8, 0xb1,
                0x3f, 0xdf, 0x5d, 0xdd,
            ],
            Bitcoin::Median16 => [
                0xa1, 0xa4, 0x1f, 0x4a, 0x60, 0x17, 0x94, 0x89, 0x9a, 0xa0, 0x50, 0x75, 0x66, 0x3e,
                0x6a, 0xfd, 0xe4, 0xb2, 0x13, 0x82, 0xb2, 0x77, 0x8d, 0xaf, 0x8f, 0x5c, 0x26, 0xeb,
                0xb3, 0xd3, 0xab, 0x53,
            ],
            Bitcoin::Median32 => [
                0xc5, 0xfe, 0x19, 0x50, 0x41, 0x84, 0xdc, 0xa2, 0xd3, 0xe8, 0xd0, 0x86, 0xf2, 0x11,
                0xe6, 0xab, 0x5f, 0x44, 0x6d, 0x6a, 0xcf, 0xc1, 0xe8, 0xfb, 0xa8, 0x12, 0x75, 0x90,
                0xd8, 0xcf, 0xc0, 0x4e,
            ],
            Bitcoin::Median64 => [
                0xf5, 0x32, 0x56, 0x7c, 0x74, 0x17, 0xcb, 0x0d, 0x19, 0xe4, 0x18, 0xd7, 0x2b, 0x31,
                0x9a, 0xf8, 0xa1, 0xce, 0x3c, 0xca, 0x90, 0x3b, 0xa2, 0x6d, 0x3b, 0x8f, 0x98, 0xf2,
                0xd8, 0x31, 0xa6, 0x1d,
            ],
            Bitcoin::Median8 => [
                0x1a, 0x64, 0xb1, 0x35, 0xf4, 0xb3, 0xa8, 0x04, 0xdb, 0xc1, 0x2a, 0x30, 0x59, 0xd9,
                0x54, 0xa1, 0x8c, 0xfb, 0x7f, 0x0b, 0x9f, 0xc5, 0x2e, 0xb9, 0x5f, 0x4a, 0x1c, 0x30,
                0xef, 0x48, 0x6c, 0xad,
            ],
            Bitcoin::Min16 => [
                0x26, 0xc0, 0x2d, 0x7c, 0xff, 0xb8, 0x84, 0xad, 0x54, 0x92, 0x88, 0x2b, 0x4b, 0x96,
                0x62, 0x8e, 0x07, 0x53, 0xeb, 0xa9, 0xf5, 0x13, 0x7f, 0x13, 0x49, 0xe9, 0x43, 0xaf,
                0x70, 0x3a, 0xd4, 0x39,
            ],
            Bitcoin::Min32 => [
                0xc6, 0x18, 0xd5, 0x77, 0xa5, 0xbd, 0x0c, 0xe3, 0xeb, 0x3d, 0xbb, 0xe5, 0x1b, 0xad,
                0x5c, 0x9f, 0x9f, 0x10, 0xce, 0xc4, 0x70, 0x59, 0xcb, 0xb5, 0x82, 0x0f, 0x8a, 0xba,
                0x05, 0x47, 0xa5, 0xca,
            ],
            Bitcoin::Min64 => [
                0xdb, 0x6d, 0xe6, 0x99, 0x42, 0x80, 0x6c, 0xfe, 0xbf, 0x21, 0xae, 0x01, 0x80, 0x67,
                0x3e, 0xd3, 0x72, 0x93, 0x42, 0xc0, 0x53, 0x48, 0x5b, 0x9b, 0x7e, 0xa7, 0xae, 0xaa,
                0x51, 0xb6, 0xbf, 0xd7,
            ],
            Bitcoin::Min8 => [
                0x25, 0xc8, 0xdc, 0x61, 0xf7, 0x3e, 0xfa, 0xe8, 0xd7, 0xdf, 0x91, 0x70, 0xc5, 0xf0,
                0xcb, 0xb9, 0xf7, 0x62, 0x65, 0xa3, 0xad, 0x95, 0xcc, 0x5c, 0x27, 0xe7, 0x28, 0x74,
                0x05, 0xe0, 0x6d, 0x8c,
            ],
            Bitcoin::Modulo16 => [
                0x25, 0x22, 0x08, 0xa3, 0xe2, 0x82, 0x5d, 0x78, 0xa9, 0x49, 0x5d, 0x81, 0x02, 0x5b,
                0x83, 0x99, 0x08, 0xaa, 0x70, 0x54, 0xe9, 0x0b, 0x0d, 0x2d, 0xdb, 0xbb, 0x0c, 0x8d,
                0x84, 0xe5, 0xc8, 0x37,
            ],
            Bitcoin::Modulo32 => [
                0x1a, 0xa0, 0xf2, 0xed, 0x81, 0xe6, 0xaf, 0x95, 0x8d, 0x1a, 0x3b, 0x72, 0xa6, 0xda,
                0x7f, 0x17, 0x38, 0x53, 0x63, 0xf2, 0xd2, 0xbd, 0x20, 0xd2, 0x93, 0x50, 0x6c, 0x0a,
                0x18, 0x58, 0x28, 0xfb,
            ],
            Bitcoin::Modulo64 => [
                0x00, 0xd4, 0x46, 0x3d, 0xec, 0x3c, 0x73, 0x0d, 0x7f, 0xed, 0x1a, 0x23, 0xc6, 0x6b,
                0xc3, 0xf4, 0xa2, 0xc7, 0x16, 0xa0, 0xce, 0x8e, 0x5d, 0x80, 0x50, 0x5b, 0x28, 0xef,
                0xb9, 0x3d, 0xbc, 0x8b,
            ],
            Bitcoin::Modulo8 => [
                0x57, 0x8a, 0xe6, 0x7d, 0xa0, 0x09, 0xf3, 0xfe, 0xc1, 0x0f, 0x45, 0x64, 0x40, 0xaa,
                0xcd, 0x54, 0x16, 0x0a, 0x16, 0xf4, 0x47, 0x5f, 0x05, 0x8f, 0xb3, 0x4b, 0xd2, 0xd3,
                0x75, 0x10, 0xc9, 0x56,
            ],
            Bitcoin::Multiply16 => [
                0x3c, 0xd8, 0x35, 0x22, 0x72, 0xaa, 0xa2, 0xc5, 0x49, 0x65, 0xf0, 0xe8, 0x66, 0xa6,
                0xe0, 0x81, 0xb2, 0x09, 0x84, 0x8c, 0x3b, 0x0a, 0x90, 0x6e, 0xcf, 0x02, 0x64, 0x15,
                0xa6, 0x53, 0x4a, 0xb3,
            ],
            Bitcoin::Multiply32 => [
                0x16, 0x1f, 0xd0, 0x3a, 0x92, 0xc6, 0x21, 0xb3, 0x28, 0x98, 0x49, 0xff, 0x29, 0xad,
                0x81, 0x34, 0x99, 0xd6, 0x3e, 0xd9, 0x73, 0xdb, 0x0e, 0x97, 0x51, 0x78, 0x54, 0x21,
                0xf5, 0x68, 0xd1, 0x8f,
            ],
            Bitcoin::Multiply64 => [
                0x22, 0x25, 0xf1, 0xaf, 0x99, 0xf5, 0x2f, 0xf9, 0x49, 0xea, 0x46, 0xb8, 0xf1, 0xce,
                0xf6, 0x2f, 0x68, 0xaa, 0x3a, 0x42, 0x60, 0x11, 0x2e, 0xc9, 0xcd, 0x74, 0xd7, 0x8e,
                0xbe, 0x1d, 0x15, 0xe7,
            ],
            Bitcoin::Multiply8 => [
                0x80, 0x24, 0x3b, 0x83, 0x65, 0x8c, 0x33, 0x0c, 0x3b, 0xc1, 0x93, 0x9e, 0x45, 0x4f,
                0x53, 0xaa, 0x74, 0xdf, 0x6c, 0xf0, 0xa1, 0x9d, 0x1d, 0x67, 0xf2, 0x14, 0x6c, 0x9c,
                0xbe, 0xe9, 0x51, 0x30,
            ],
            Bitcoin::Negate16 => [
                0x02, 0x69, 0xd7, 0x3f, 0x09, 0x4d, 0x59, 0x41, 0x19, 0x73, 0xea, 0xcd, 0xd5, 0xd3,
                0xd9, 0xe4, 0x7c, 0xab, 0xb8, 0x27, 0x7e, 0x6e, 0xf4, 0x11, 0x15, 0x7a, 0x44, 0x8a,
                0xe1, 0x25, 0x2d, 0x33,
            ],
            Bitcoin::Negate32 => [
                0xf6, 0xb2, 0x1e, 0x3f, 0x59, 0x3e, 0xbd, 0x97, 0xec, 0x16, 0x1f, 0xd4, 0xf8, 0x54,
                0x43, 0xd3, 0x65, 0xc0, 0x23, 0x07, 0x5a, 0x22, 0xb6, 0x8c, 0xa2, 0x6c, 0xf6, 0xb7,
                0x8b, 0xab, 0x94, 0xb0,
            ],
            Bitcoin::Negate64 => [
                0x2b, 0xf4, 0x8c, 0x08, 0xa5, 0x7f, 0x18, 0x36, 0xea, 0x2f, 0x57, 0x22, 0xc0, 0x79,
                0x59, 0xa7, 0xd6, 0x4c, 0xbe, 0xf7, 0x05, 0xdc, 0xc3, 0xca, 0xba, 0x3a, 0x90, 0x05,
                0x03, 0xd4, 0x89, 0x59,
            ],
            Bitcoin::Negate8 => [
                0x25, 0x6d, 0x50, 0x79, 0x8b, 0x7f, 0xe9, 0xe7, 0xcb, 0x16, 0x96, 0xfb, 0x18, 0x83,
                0x19, 0x01, 0xef, 0x88, 0x95, 0x2c, 0xc4, 0x60, 0x6b, 0x68, 0x97, 0x4a, 0x79, 0xca,
                0x8a, 0x8a, 0x00, 0x44,
            ],
            Bitcoin::NumInputs => [
                0xcd, 0xd8, 0x4d, 0x52, 0x00, 0x47, 0x12, 0x6e, 0x09, 0xce, 0x13, 0x6b, 0xf8, 0xb2,
                0x00, 0xe8, 0x23, 0x2a, 0xe6, 0x7a, 0xc2, 0xcb, 0x82, 0x72, 0xa5, 0xa0, 0x52, 0xc5,
                0x8d, 0x65, 0x5c, 0xbf,
            ],
            Bitcoin::NumOutputs => [
                0x94, 0x20, 0xc4, 0x03, 0xda, 0x78, 0xcf, 0x4b, 0x55, 0x48, 0x15, 0x71, 0x6b, 0x28,
                0x3f, 0x87, 0x58, 0xc2, 0xd3, 0x62, 0x37, 0x7c, 0x8e, 0x68, 0x9a, 0x4a, 0x74, 0x08,
                0xd4, 0x3a, 0x0d, 0xc9,
            ],
            Bitcoin::One16 => [
                0x8e, 0x67, 0xce, 0x64, 0xee, 0x18, 0xb6, 0x44, 0xc5, 0x5f, 0xd0, 0x7c, 0x71, 0x93,
                0x2c, 0xa6, 0xe0, 0x29, 0x0b, 0xab, 0xef, 0xf9, 0x49, 0x25, 0xdd, 0x6d, 0xf2, 0x3a,
                0x2e, 0xe4, 0xd1, 0xa2,
            ],
            Bitcoin::One32 => [
                0x45, 0x26, 0x09, 0x9d, 0x0f, 0x84, 0x0b, 0xa6, 0xab, 0xe5, 0x5e, 0xd9, 0xf3, 0xa9,
                0xb8, 0xfc, 0xa0, 0x84, 0xf2, 0x51, 0xbf, 0xb2, 0x6d, 0x01, 0x1c, 0xf7, 0xaf, 0x27,
                0xb0, 0xd8, 0x8e, 0x29,
            ],
            Bitcoin::One64 => [
                0xc1, 0xa3, 0x4a, 0x00, 0xe3, 0x1e, 0xe1, 0x55, 0x2e, 0x6d, 0x09, 0xc6, 0x70, 0x77,
                0x2b, 0xcb, 0x18, 0x63, 0xc1, 0x0f, 0x14, 0x82, 0xd0, 0x8c, 0xf0, 0xd9, 0xa1, 0x83,
                0x2c, 0x59, 0xec, 0xf4,
            ],
            Bitcoin::One8 => [
                0xf9, 0x58, 0x76, 0x11, 0x81, 0xef, 0xf2, 0x30, 0xcb, 0xc5, 0x1c, 0xc0, 0xe4, 0x5f,
                0x66, 0x91, 0x1b, 0x32, 0x19, 0x16, 0x0e, 0x62, 0x7f, 0xa8, 0x10, 0xc0, 0x7d, 0xfe,
                0xa0, 0xb9, 0x9d, 0x6a,
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
                0xef, 0xe2, 0x87, 0xa5, 0xcb, 0xe2, 0x81, 0x8b, 0x11, 0x3d, 0x87, 0x00, 0x22, 0x32,
                0x6e, 0xb3, 0x0a, 0x4b, 0xe0, 0xad, 0xa4, 0x40, 0x23, 0xd5, 0x45, 0xb6, 0xc1, 0x51,
                0xf0, 0xcd, 0xa7, 0x08,
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
                0xac, 0x65, 0xf0, 0xb8, 0x0d, 0x5e, 0xfc, 0xeb, 0xb5, 0x01, 0xe5, 0xe9, 0x62, 0xf4,
                0x4f, 0xb8, 0x14, 0x6b, 0x6d, 0x4e, 0x46, 0x5b, 0xea, 0x49, 0xbc, 0x74, 0x09, 0x5e,
                0x21, 0xfb, 0xaa, 0xc0,
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
                0x2f, 0xec, 0x1d, 0x6b, 0x08, 0xa8, 0xf8, 0x19, 0xb8, 0xf4, 0x6a, 0x60, 0xfb, 0x90,
                0xac, 0x17, 0xb8, 0x84, 0x64, 0xcf, 0xb6, 0x08, 0x38, 0x49, 0xaf, 0xea, 0xde, 0x1f,
                0x34, 0xf1, 0xe7, 0xbc,
            ],
            Bitcoin::Sha256Ctx8Add128 => [
                0x20, 0x0b, 0x99, 0x32, 0xe7, 0x4d, 0x3d, 0x13, 0x51, 0x2d, 0x64, 0x44, 0xee, 0x64,
                0x45, 0xed, 0x17, 0xee, 0x33, 0xd2, 0x9f, 0x39, 0x79, 0xd8, 0x39, 0x24, 0xe6, 0xa5,
                0x9c, 0x99, 0xd8, 0x10,
            ],
            Bitcoin::Sha256Ctx8Add16 => [
                0x29, 0x26, 0x3f, 0x97, 0xa3, 0xf4, 0xbd, 0x82, 0x7f, 0x20, 0x2d, 0x4b, 0xf9, 0xea,
                0x57, 0xaa, 0x84, 0x1a, 0xae, 0x6e, 0xe9, 0xa1, 0x3f, 0xa7, 0xb4, 0x59, 0xe7, 0x99,
                0x9b, 0x15, 0x44, 0x97,
            ],
            Bitcoin::Sha256Ctx8Add2 => [
                0xb4, 0x59, 0x0d, 0x05, 0x6c, 0x52, 0xbe, 0x9c, 0x16, 0x61, 0x00, 0x03, 0x53, 0x50,
                0xd8, 0xf4, 0x3d, 0x31, 0x0f, 0x3f, 0x57, 0x56, 0x17, 0xc6, 0x39, 0x81, 0x95, 0x48,
                0xb5, 0xb4, 0x40, 0x87,
            ],
            Bitcoin::Sha256Ctx8Add256 => [
                0xcf, 0x64, 0x38, 0x27, 0x7c, 0x66, 0xb4, 0xe6, 0x30, 0x7e, 0xcf, 0xac, 0x31, 0x52,
                0x26, 0xc7, 0xb9, 0xaa, 0xdb, 0xda, 0x33, 0xeb, 0xe1, 0x9d, 0x62, 0x04, 0x65, 0xef,
                0x48, 0x44, 0xd8, 0x08,
            ],
            Bitcoin::Sha256Ctx8Add32 => [
                0x69, 0xa6, 0xec, 0xb2, 0x7a, 0x04, 0x49, 0xf3, 0xd6, 0x7a, 0xb9, 0xd9, 0x61, 0x96,
                0x1d, 0x6a, 0xa8, 0x15, 0x07, 0xde, 0x69, 0x51, 0xb0, 0xd4, 0xce, 0xaf, 0xf6, 0xd8,
                0xbd, 0x38, 0x80, 0x27,
            ],
            Bitcoin::Sha256Ctx8Add4 => [
                0xa5, 0x82, 0xd2, 0xbc, 0xdf, 0xaf, 0x71, 0xfa, 0xf7, 0xb9, 0xae, 0x04, 0x5f, 0x97,
                0xaa, 0xc0, 0x55, 0xc4, 0x77, 0x9c, 0x6e, 0x80, 0x22, 0xe9, 0xfe, 0x11, 0xfb, 0xa2,
                0xa4, 0x44, 0xad, 0xb1,
            ],
            Bitcoin::Sha256Ctx8Add512 => [
                0x89, 0x5f, 0x1b, 0x6c, 0x9a, 0x41, 0xf8, 0x3f, 0xd3, 0x27, 0x23, 0xc7, 0xa2, 0x2d,
                0xe2, 0x65, 0xa1, 0x27, 0x90, 0xaa, 0xa8, 0x83, 0x17, 0xac, 0x01, 0xdd, 0xbb, 0xd1,
                0x4b, 0x60, 0xd4, 0x1d,
            ],
            Bitcoin::Sha256Ctx8Add64 => [
                0x78, 0x15, 0x65, 0x01, 0x89, 0xa1, 0x17, 0xba, 0x09, 0x12, 0x08, 0x8b, 0x79, 0x29,
                0x7a, 0x07, 0x66, 0xb6, 0x09, 0x45, 0x87, 0xbf, 0x5d, 0xa6, 0xe8, 0xf4, 0x6e, 0x9c,
                0x37, 0xb4, 0x34, 0x91,
            ],
            Bitcoin::Sha256Ctx8Add8 => [
                0x1e, 0x8d, 0xb5, 0xba, 0x26, 0x05, 0xf0, 0xf3, 0x05, 0xd5, 0xb7, 0xd5, 0x0c, 0x09,
                0x00, 0x6e, 0xa4, 0x85, 0x58, 0x46, 0x3f, 0x6e, 0x0c, 0x85, 0xfe, 0x50, 0x51, 0xf0,
                0xc0, 0x53, 0xaf, 0xeb,
            ],
            Bitcoin::Sha256Ctx8AddBuffer511 => [
                0xf8, 0x9d, 0x7e, 0x21, 0x9b, 0x3e, 0xd8, 0x81, 0x4d, 0xd9, 0xf5, 0xc1, 0xa5, 0xda,
                0xb6, 0xba, 0xd6, 0xd3, 0x2d, 0xc5, 0x72, 0xa5, 0x21, 0x35, 0x39, 0x2c, 0x81, 0x43,
                0x0e, 0x12, 0x03, 0xd1,
            ],
            Bitcoin::Sha256Ctx8Finalize => [
                0xc9, 0xcd, 0x48, 0x36, 0x64, 0xf9, 0xaf, 0x94, 0xd5, 0xe9, 0x41, 0xdb, 0xb9, 0xe1,
                0xe9, 0xb7, 0xc6, 0x4f, 0x48, 0xef, 0x15, 0xc9, 0x06, 0x5f, 0xd1, 0xbb, 0x7f, 0xd9,
                0xc5, 0x6c, 0xbd, 0xc6,
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
            Bitcoin::Subtract16 => [
                0x62, 0x4a, 0xf4, 0x3d, 0x27, 0x5e, 0x2a, 0x20, 0x70, 0x3e, 0x66, 0x52, 0x3d, 0x35,
                0xdd, 0x34, 0xef, 0x8e, 0x8b, 0x29, 0x3a, 0x57, 0x38, 0x0a, 0x21, 0xdb, 0xf6, 0x10,
                0xcd, 0x62, 0x7a, 0xa3,
            ],
            Bitcoin::Subtract32 => [
                0xf7, 0x6e, 0xca, 0xd1, 0xfd, 0xa5, 0x0f, 0x13, 0x5b, 0xdf, 0xe3, 0xe5, 0x33, 0xa1,
                0x50, 0x09, 0x8f, 0x40, 0x62, 0x61, 0xc7, 0x6f, 0x6d, 0xbf, 0x67, 0x25, 0xf1, 0xe3,
                0x88, 0x3c, 0x2a, 0xe2,
            ],
            Bitcoin::Subtract64 => [
                0x99, 0x4b, 0x25, 0x00, 0x38, 0x7d, 0x8d, 0x62, 0x02, 0x09, 0x67, 0x74, 0x16, 0xb9,
                0xe6, 0x04, 0x52, 0x6c, 0x70, 0x8f, 0xf1, 0xf9, 0x65, 0xc9, 0xc9, 0x12, 0x52, 0x04,
                0x7a, 0x3f, 0x57, 0xb3,
            ],
            Bitcoin::Subtract8 => [
                0x57, 0xa0, 0xd0, 0x8e, 0x5f, 0x01, 0xe8, 0x38, 0x8d, 0x78, 0x5c, 0xcb, 0x26, 0x57,
                0xc4, 0xea, 0x98, 0xb8, 0x54, 0xa6, 0x58, 0x5c, 0x65, 0x8a, 0x21, 0x0d, 0xb0, 0x90,
                0x1d, 0x10, 0xf9, 0x48,
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
            Bitcoin::Add16 => b"i",
            Bitcoin::Add32 => b"l",
            Bitcoin::Add64 => b"*ll",
            Bitcoin::Add8 => b"****22*22**22*22***22*22**22*22",
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
            Bitcoin::Decrement16 => b"****22*22**22*22***22*22**22*22",
            Bitcoin::Decrement32 => b"i",
            Bitcoin::Decrement64 => b"l",
            Bitcoin::Decrement8 => b"***22*22**22*22",
            Bitcoin::DivMod16 => b"i",
            Bitcoin::DivMod32 => b"l",
            Bitcoin::DivMod64 => b"*ll",
            Bitcoin::DivMod8 => b"****22*22**22*22***22*22**22*22",
            Bitcoin::Divide16 => b"i",
            Bitcoin::Divide32 => b"l",
            Bitcoin::Divide64 => b"*ll",
            Bitcoin::Divide8 => b"****22*22**22*22***22*22**22*22",
            Bitcoin::Divides16 => b"i",
            Bitcoin::Divides32 => b"l",
            Bitcoin::Divides64 => b"*ll",
            Bitcoin::Divides8 => b"****22*22**22*22***22*22**22*22",
            Bitcoin::Eq16 => b"i",
            Bitcoin::Eq256 => b"*hh",
            Bitcoin::Eq32 => b"l",
            Bitcoin::Eq64 => b"*ll",
            Bitcoin::Eq8 => b"****22*22**22*22***22*22**22*22",
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
            Bitcoin::FullAdd16 => b"*2i",
            Bitcoin::FullAdd32 => b"*2l",
            Bitcoin::FullAdd64 => b"*2*ll",
            Bitcoin::FullAdd8 => b"*2****22*22**22*22***22*22**22*22",
            Bitcoin::FullDecrement16 => b"*2****22*22**22*22***22*22**22*22",
            Bitcoin::FullDecrement32 => b"*2i",
            Bitcoin::FullDecrement64 => b"*2l",
            Bitcoin::FullDecrement8 => b"*2***22*22**22*22",
            Bitcoin::FullIncrement16 => b"*2****22*22**22*22***22*22**22*22",
            Bitcoin::FullIncrement32 => b"*2i",
            Bitcoin::FullIncrement64 => b"*2l",
            Bitcoin::FullIncrement8 => b"*2***22*22**22*22",
            Bitcoin::FullMultiply16 => b"l",
            Bitcoin::FullMultiply32 => b"*ll",
            Bitcoin::FullMultiply64 => b"h",
            Bitcoin::FullMultiply8 => b"i",
            Bitcoin::FullSubtract16 => b"*2i",
            Bitcoin::FullSubtract32 => b"*2l",
            Bitcoin::FullSubtract64 => b"*2*ll",
            Bitcoin::FullSubtract8 => b"*2****22*22**22*22***22*22**22*22",
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
            Bitcoin::Increment16 => b"****22*22**22*22***22*22**22*22",
            Bitcoin::Increment32 => b"i",
            Bitcoin::Increment64 => b"l",
            Bitcoin::Increment8 => b"***22*22**22*22",
            Bitcoin::InputAnnexHash => b"i",
            Bitcoin::InputPrevOutpoint => b"i",
            Bitcoin::InputScriptSigHash => b"i",
            Bitcoin::InputSequence => b"i",
            Bitcoin::InputValue => b"i",
            Bitcoin::InternalKey => b"1",
            Bitcoin::IsOne16 => b"****22*22**22*22***22*22**22*22",
            Bitcoin::IsOne32 => b"i",
            Bitcoin::IsOne64 => b"l",
            Bitcoin::IsOne8 => b"***22*22**22*22",
            Bitcoin::IsZero16 => b"****22*22**22*22***22*22**22*22",
            Bitcoin::IsZero32 => b"i",
            Bitcoin::IsZero64 => b"l",
            Bitcoin::IsZero8 => b"***22*22**22*22",
            Bitcoin::Le16 => b"i",
            Bitcoin::Le32 => b"l",
            Bitcoin::Le64 => b"*ll",
            Bitcoin::Le8 => b"****22*22**22*22***22*22**22*22",
            Bitcoin::LinearCombination1 => b"**h**hhhh",
            Bitcoin::LinearVerify1 => b"***h*hhh*hh",
            Bitcoin::LockTime => b"1",
            Bitcoin::Low16 => b"1",
            Bitcoin::Low32 => b"1",
            Bitcoin::Low64 => b"1",
            Bitcoin::Low8 => b"1",
            Bitcoin::Lt16 => b"i",
            Bitcoin::Lt32 => b"l",
            Bitcoin::Lt64 => b"*ll",
            Bitcoin::Lt8 => b"****22*22**22*22***22*22**22*22",
            Bitcoin::Max16 => b"i",
            Bitcoin::Max32 => b"l",
            Bitcoin::Max64 => b"*ll",
            Bitcoin::Max8 => b"****22*22**22*22***22*22**22*22",
            Bitcoin::Median16 => b"*****22*22**22*22***22*22**22*22i",
            Bitcoin::Median32 => b"*il",
            Bitcoin::Median64 => b"*l*ll",
            Bitcoin::Median8 => b"****22*22**22*22****22*22**22*22***22*22**22*22",
            Bitcoin::Min16 => b"i",
            Bitcoin::Min32 => b"l",
            Bitcoin::Min64 => b"*ll",
            Bitcoin::Min8 => b"****22*22**22*22***22*22**22*22",
            Bitcoin::Modulo16 => b"i",
            Bitcoin::Modulo32 => b"l",
            Bitcoin::Modulo64 => b"*ll",
            Bitcoin::Modulo8 => b"****22*22**22*22***22*22**22*22",
            Bitcoin::Multiply16 => b"i",
            Bitcoin::Multiply32 => b"l",
            Bitcoin::Multiply64 => b"*ll",
            Bitcoin::Multiply8 => b"****22*22**22*22***22*22**22*22",
            Bitcoin::Negate16 => b"****22*22**22*22***22*22**22*22",
            Bitcoin::Negate32 => b"i",
            Bitcoin::Negate64 => b"l",
            Bitcoin::Negate8 => b"***22*22**22*22",
            Bitcoin::NumInputs => b"1",
            Bitcoin::NumOutputs => b"1",
            Bitcoin::One16 => b"1",
            Bitcoin::One32 => b"1",
            Bitcoin::One64 => b"1",
            Bitcoin::One8 => b"1",
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
            Bitcoin::Subtract16 => b"i",
            Bitcoin::Subtract32 => b"l",
            Bitcoin::Subtract64 => b"*ll",
            Bitcoin::Subtract8 => b"****22*22**22*22***22*22**22*22",
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
            Bitcoin::Add16 => b"*2****22*22**22*22***22*22**22*22",
            Bitcoin::Add32 => b"*2i",
            Bitcoin::Add64 => b"*2l",
            Bitcoin::Add8 => b"*2***22*22**22*22",
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
            Bitcoin::Decrement16 => b"*2****22*22**22*22***22*22**22*22",
            Bitcoin::Decrement32 => b"*2i",
            Bitcoin::Decrement64 => b"*2l",
            Bitcoin::Decrement8 => b"*2***22*22**22*22",
            Bitcoin::DivMod16 => b"i",
            Bitcoin::DivMod32 => b"l",
            Bitcoin::DivMod64 => b"*ll",
            Bitcoin::DivMod8 => b"****22*22**22*22***22*22**22*22",
            Bitcoin::Divide16 => b"****22*22**22*22***22*22**22*22",
            Bitcoin::Divide32 => b"i",
            Bitcoin::Divide64 => b"l",
            Bitcoin::Divide8 => b"***22*22**22*22",
            Bitcoin::Divides16 => b"2",
            Bitcoin::Divides32 => b"2",
            Bitcoin::Divides64 => b"2",
            Bitcoin::Divides8 => b"2",
            Bitcoin::Eq16 => b"2",
            Bitcoin::Eq256 => b"2",
            Bitcoin::Eq32 => b"2",
            Bitcoin::Eq64 => b"2",
            Bitcoin::Eq8 => b"2",
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
            Bitcoin::FullAdd16 => b"*2****22*22**22*22***22*22**22*22",
            Bitcoin::FullAdd32 => b"*2i",
            Bitcoin::FullAdd64 => b"*2l",
            Bitcoin::FullAdd8 => b"*2***22*22**22*22",
            Bitcoin::FullDecrement16 => b"*2****22*22**22*22***22*22**22*22",
            Bitcoin::FullDecrement32 => b"*2i",
            Bitcoin::FullDecrement64 => b"*2l",
            Bitcoin::FullDecrement8 => b"*2***22*22**22*22",
            Bitcoin::FullIncrement16 => b"*2****22*22**22*22***22*22**22*22",
            Bitcoin::FullIncrement32 => b"*2i",
            Bitcoin::FullIncrement64 => b"*2l",
            Bitcoin::FullIncrement8 => b"*2***22*22**22*22",
            Bitcoin::FullMultiply16 => b"i",
            Bitcoin::FullMultiply32 => b"l",
            Bitcoin::FullMultiply64 => b"*ll",
            Bitcoin::FullMultiply8 => b"****22*22**22*22***22*22**22*22",
            Bitcoin::FullSubtract16 => b"*2****22*22**22*22***22*22**22*22",
            Bitcoin::FullSubtract32 => b"*2i",
            Bitcoin::FullSubtract64 => b"*2l",
            Bitcoin::FullSubtract8 => b"*2***22*22**22*22",
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
            Bitcoin::Increment16 => b"*2****22*22**22*22***22*22**22*22",
            Bitcoin::Increment32 => b"*2i",
            Bitcoin::Increment64 => b"*2l",
            Bitcoin::Increment8 => b"*2***22*22**22*22",
            Bitcoin::InputAnnexHash => b"+1+1h",
            Bitcoin::InputPrevOutpoint => b"+1*hi",
            Bitcoin::InputScriptSigHash => b"+1h",
            Bitcoin::InputSequence => b"+1i",
            Bitcoin::InputValue => b"+1l",
            Bitcoin::InternalKey => b"h",
            Bitcoin::IsOne16 => b"2",
            Bitcoin::IsOne32 => b"2",
            Bitcoin::IsOne64 => b"2",
            Bitcoin::IsOne8 => b"2",
            Bitcoin::IsZero16 => b"2",
            Bitcoin::IsZero32 => b"2",
            Bitcoin::IsZero64 => b"2",
            Bitcoin::IsZero8 => b"2",
            Bitcoin::Le16 => b"2",
            Bitcoin::Le32 => b"2",
            Bitcoin::Le64 => b"2",
            Bitcoin::Le8 => b"2",
            Bitcoin::LinearCombination1 => b"**hhh",
            Bitcoin::LinearVerify1 => b"1",
            Bitcoin::LockTime => b"i",
            Bitcoin::Low16 => b"****22*22**22*22***22*22**22*22",
            Bitcoin::Low32 => b"i",
            Bitcoin::Low64 => b"l",
            Bitcoin::Low8 => b"***22*22**22*22",
            Bitcoin::Lt16 => b"2",
            Bitcoin::Lt32 => b"2",
            Bitcoin::Lt64 => b"2",
            Bitcoin::Lt8 => b"2",
            Bitcoin::Max16 => b"****22*22**22*22***22*22**22*22",
            Bitcoin::Max32 => b"i",
            Bitcoin::Max64 => b"l",
            Bitcoin::Max8 => b"***22*22**22*22",
            Bitcoin::Median16 => b"****22*22**22*22***22*22**22*22",
            Bitcoin::Median32 => b"i",
            Bitcoin::Median64 => b"l",
            Bitcoin::Median8 => b"***22*22**22*22",
            Bitcoin::Min16 => b"****22*22**22*22***22*22**22*22",
            Bitcoin::Min32 => b"i",
            Bitcoin::Min64 => b"l",
            Bitcoin::Min8 => b"***22*22**22*22",
            Bitcoin::Modulo16 => b"****22*22**22*22***22*22**22*22",
            Bitcoin::Modulo32 => b"i",
            Bitcoin::Modulo64 => b"l",
            Bitcoin::Modulo8 => b"***22*22**22*22",
            Bitcoin::Multiply16 => b"i",
            Bitcoin::Multiply32 => b"l",
            Bitcoin::Multiply64 => b"*ll",
            Bitcoin::Multiply8 => b"****22*22**22*22***22*22**22*22",
            Bitcoin::Negate16 => b"*2****22*22**22*22***22*22**22*22",
            Bitcoin::Negate32 => b"*2i",
            Bitcoin::Negate64 => b"*2l",
            Bitcoin::Negate8 => b"*2***22*22**22*22",
            Bitcoin::NumInputs => b"i",
            Bitcoin::NumOutputs => b"i",
            Bitcoin::One16 => b"****22*22**22*22***22*22**22*22",
            Bitcoin::One32 => b"i",
            Bitcoin::One64 => b"l",
            Bitcoin::One8 => b"***22*22**22*22",
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
            Bitcoin::Subtract16 => b"*2****22*22**22*22***22*22**22*22",
            Bitcoin::Subtract32 => b"*2i",
            Bitcoin::Subtract64 => b"*2l",
            Bitcoin::Subtract8 => b"*2***22*22**22*22",
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
            Bitcoin::Low8 => (37, 8),
            Bitcoin::Low16 => (304, 11),
            Bitcoin::Low32 => (305, 11),
            Bitcoin::Low64 => (306, 11),
            Bitcoin::Eq8 => (877, 12),
            Bitcoin::Eq16 => (7024, 15),
            Bitcoin::Eq32 => (7025, 15),
            Bitcoin::Eq64 => (7026, 15),
            Bitcoin::Eq256 => (14056, 16),
            Bitcoin::One8 => (69, 8),
            Bitcoin::One16 => (560, 11),
            Bitcoin::One32 => (561, 11),
            Bitcoin::One64 => (562, 11),
            Bitcoin::FullAdd8 => (293, 10),
            Bitcoin::FullAdd16 => (2352, 13),
            Bitcoin::FullAdd32 => (2353, 13),
            Bitcoin::FullAdd64 => (2354, 13),
            Bitcoin::Add8 => (301, 10),
            Bitcoin::Add16 => (2416, 13),
            Bitcoin::Add32 => (2417, 13),
            Bitcoin::Add64 => (2418, 13),
            Bitcoin::FullIncrement8 => (2437, 13),
            Bitcoin::FullIncrement16 => (19504, 16),
            Bitcoin::FullIncrement32 => (19505, 16),
            Bitcoin::FullIncrement64 => (19506, 16),
            Bitcoin::Increment8 => (2445, 13),
            Bitcoin::Increment16 => (19568, 16),
            Bitcoin::Increment32 => (19569, 16),
            Bitcoin::Increment64 => (19570, 16),
            Bitcoin::FullSubtract8 => (2461, 13),
            Bitcoin::FullSubtract16 => (19696, 16),
            Bitcoin::FullSubtract32 => (19697, 16),
            Bitcoin::FullSubtract64 => (19698, 16),
            Bitcoin::Subtract8 => (4933, 14),
            Bitcoin::Subtract16 => (39472, 17),
            Bitcoin::Subtract32 => (39473, 17),
            Bitcoin::Subtract64 => (39474, 17),
            Bitcoin::Negate8 => (4941, 14),
            Bitcoin::Negate16 => (39536, 17),
            Bitcoin::Negate32 => (39537, 17),
            Bitcoin::Negate64 => (39538, 17),
            Bitcoin::FullDecrement8 => (4949, 14),
            Bitcoin::FullDecrement16 => (39600, 17),
            Bitcoin::FullDecrement32 => (39601, 17),
            Bitcoin::FullDecrement64 => (39602, 17),
            Bitcoin::Decrement8 => (4957, 14),
            Bitcoin::Decrement16 => (39664, 17),
            Bitcoin::Decrement32 => (39665, 17),
            Bitcoin::Decrement64 => (39666, 17),
            Bitcoin::FullMultiply8 => (4965, 14),
            Bitcoin::FullMultiply16 => (39728, 17),
            Bitcoin::FullMultiply32 => (39729, 17),
            Bitcoin::FullMultiply64 => (39730, 17),
            Bitcoin::Multiply8 => (4973, 14),
            Bitcoin::Multiply16 => (39792, 17),
            Bitcoin::Multiply32 => (39793, 17),
            Bitcoin::Multiply64 => (39794, 17),
            Bitcoin::IsZero8 => (4981, 14),
            Bitcoin::IsZero16 => (39856, 17),
            Bitcoin::IsZero32 => (39857, 17),
            Bitcoin::IsZero64 => (39858, 17),
            Bitcoin::IsOne8 => (4989, 14),
            Bitcoin::IsOne16 => (39920, 17),
            Bitcoin::IsOne32 => (39921, 17),
            Bitcoin::IsOne64 => (39922, 17),
            Bitcoin::Le8 => (79877, 18),
            Bitcoin::Le16 => (639024, 21),
            Bitcoin::Le32 => (639025, 21),
            Bitcoin::Le64 => (639026, 21),
            Bitcoin::Lt8 => (79885, 18),
            Bitcoin::Lt16 => (639088, 21),
            Bitcoin::Lt32 => (639089, 21),
            Bitcoin::Lt64 => (639090, 21),
            Bitcoin::Min8 => (79893, 18),
            Bitcoin::Min16 => (639152, 21),
            Bitcoin::Min32 => (639153, 21),
            Bitcoin::Min64 => (639154, 21),
            Bitcoin::Max8 => (79901, 18),
            Bitcoin::Max16 => (639216, 21),
            Bitcoin::Max32 => (639217, 21),
            Bitcoin::Max64 => (639218, 21),
            Bitcoin::Median8 => (79909, 18),
            Bitcoin::Median16 => (639280, 21),
            Bitcoin::Median32 => (639281, 21),
            Bitcoin::Median64 => (639282, 21),
            Bitcoin::DivMod8 => (79925, 18),
            Bitcoin::DivMod16 => (639408, 21),
            Bitcoin::DivMod32 => (639409, 21),
            Bitcoin::DivMod64 => (639410, 21),
            Bitcoin::Divide8 => (79933, 18),
            Bitcoin::Divide16 => (639472, 21),
            Bitcoin::Divide32 => (639473, 21),
            Bitcoin::Divide64 => (639474, 21),
            Bitcoin::Modulo8 => (79941, 18),
            Bitcoin::Modulo16 => (639536, 21),
            Bitcoin::Modulo32 => (639537, 21),
            Bitcoin::Modulo64 => (639538, 21),
            Bitcoin::Divides8 => (79949, 18),
            Bitcoin::Divides16 => (639600, 21),
            Bitcoin::Divides32 => (639601, 21),
            Bitcoin::Divides64 => (639602, 21),
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
                                    0 => {
                                        0 => {},
                                        1 => {Bitcoin::Low8}
                                    },
                                    1 => {
                                        0 => {
                                            0 => {
                                                0 => {
                                                    0 => {Bitcoin::Low16},
                                                    1 => {Bitcoin::Low32}
                                                },
                                                1 => {
                                                    0 => {Bitcoin::Low64},
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
                                                    0 => {
                                                        0 => {},
                                                        1 => {Bitcoin::Eq8}
                                                    },
                                                    1 => {
                                                        0 => {
                                                            0 => {
                                                                0 => {
                                                                    0 => {Bitcoin::Eq16},
                                                                    1 => {Bitcoin::Eq32}
                                                                },
                                                                1 => {
                                                                    0 => {Bitcoin::Eq64},
                                                                    1 => {}
                                                                }
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
                                    0 => {
                                        0 => {},
                                        1 => {Bitcoin::One8}
                                    },
                                    1 => {
                                        0 => {
                                            0 => {
                                                0 => {
                                                    0 => {Bitcoin::One16},
                                                    1 => {Bitcoin::One32}
                                                },
                                                1 => {
                                                    0 => {Bitcoin::One64},
                                                    1 => {}
                                                }
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
                                            0 => {
                                                0 => {},
                                                1 => {Bitcoin::FullAdd8}
                                            },
                                            1 => {
                                                0 => {
                                                    0 => {
                                                        0 => {
                                                            0 => {Bitcoin::FullAdd16},
                                                            1 => {Bitcoin::FullAdd32}
                                                        },
                                                        1 => {
                                                            0 => {Bitcoin::FullAdd64},
                                                            1 => {}
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
                                                0 => {},
                                                1 => {Bitcoin::Add8}
                                            },
                                            1 => {
                                                0 => {
                                                    0 => {
                                                        0 => {
                                                            0 => {Bitcoin::Add16},
                                                            1 => {Bitcoin::Add32}
                                                        },
                                                        1 => {
                                                            0 => {Bitcoin::Add64},
                                                            1 => {}
                                                        }
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
                                            0 => {
                                                0 => {
                                                    0 => {},
                                                    1 => {
                                                        0 => {
                                                            0 => {},
                                                            1 => {Bitcoin::FullIncrement8}
                                                        },
                                                        1 => {
                                                            0 => {
                                                                0 => {
                                                                    0 => {
                                                                        0 => {Bitcoin::FullIncrement16},
                                                                        1 => {Bitcoin::FullIncrement32}
                                                                    },
                                                                    1 => {
                                                                        0 => {Bitcoin::FullIncrement64},
                                                                        1 => {}
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
                                                            0 => {},
                                                            1 => {Bitcoin::Increment8}
                                                        },
                                                        1 => {
                                                            0 => {
                                                                0 => {
                                                                    0 => {
                                                                        0 => {Bitcoin::Increment16},
                                                                        1 => {Bitcoin::Increment32}
                                                                    },
                                                                    1 => {
                                                                        0 => {Bitcoin::Increment64},
                                                                        1 => {}
                                                                    }
                                                                },
                                                                1 => {}
                                                            },
                                                            1 => {}
                                                        }
                                                    }
                                                }
                                            },
                                            1 => {
                                                0 => {},
                                                1 => {
                                                    0 => {},
                                                    1 => {
                                                        0 => {
                                                            0 => {},
                                                            1 => {Bitcoin::FullSubtract8}
                                                        },
                                                        1 => {
                                                            0 => {
                                                                0 => {
                                                                    0 => {
                                                                        0 => {Bitcoin::FullSubtract16},
                                                                        1 => {Bitcoin::FullSubtract32}
                                                                    },
                                                                    1 => {
                                                                        0 => {Bitcoin::FullSubtract64},
                                                                        1 => {}
                                                                    }
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
                                                            0 => {
                                                                0 => {},
                                                                1 => {Bitcoin::Subtract8}
                                                            },
                                                            1 => {
                                                                0 => {
                                                                    0 => {
                                                                        0 => {
                                                                            0 => {Bitcoin::Subtract16},
                                                                            1 => {Bitcoin::Subtract32}
                                                                        },
                                                                        1 => {
                                                                            0 => {Bitcoin::Subtract64},
                                                                            1 => {}
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
                                                                0 => {},
                                                                1 => {Bitcoin::Negate8}
                                                            },
                                                            1 => {
                                                                0 => {
                                                                    0 => {
                                                                        0 => {
                                                                            0 => {Bitcoin::Negate16},
                                                                            1 => {Bitcoin::Negate32}
                                                                        },
                                                                        1 => {
                                                                            0 => {Bitcoin::Negate64},
                                                                            1 => {}
                                                                        }
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
                                                        0 => {},
                                                        1 => {
                                                            0 => {
                                                                0 => {},
                                                                1 => {Bitcoin::FullDecrement8}
                                                            },
                                                            1 => {
                                                                0 => {
                                                                    0 => {
                                                                        0 => {
                                                                            0 => {Bitcoin::FullDecrement16},
                                                                            1 => {Bitcoin::FullDecrement32}
                                                                        },
                                                                        1 => {
                                                                            0 => {Bitcoin::FullDecrement64},
                                                                            1 => {}
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
                                                                0 => {},
                                                                1 => {Bitcoin::Decrement8}
                                                            },
                                                            1 => {
                                                                0 => {
                                                                    0 => {
                                                                        0 => {
                                                                            0 => {Bitcoin::Decrement16},
                                                                            1 => {Bitcoin::Decrement32}
                                                                        },
                                                                        1 => {
                                                                            0 => {Bitcoin::Decrement64},
                                                                            1 => {}
                                                                        }
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
                                                        0 => {},
                                                        1 => {
                                                            0 => {
                                                                0 => {},
                                                                1 => {Bitcoin::FullMultiply8}
                                                            },
                                                            1 => {
                                                                0 => {
                                                                    0 => {
                                                                        0 => {
                                                                            0 => {Bitcoin::FullMultiply16},
                                                                            1 => {Bitcoin::FullMultiply32}
                                                                        },
                                                                        1 => {
                                                                            0 => {Bitcoin::FullMultiply64},
                                                                            1 => {}
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
                                                                0 => {},
                                                                1 => {Bitcoin::Multiply8}
                                                            },
                                                            1 => {
                                                                0 => {
                                                                    0 => {
                                                                        0 => {
                                                                            0 => {Bitcoin::Multiply16},
                                                                            1 => {Bitcoin::Multiply32}
                                                                        },
                                                                        1 => {
                                                                            0 => {Bitcoin::Multiply64},
                                                                            1 => {}
                                                                        }
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
                                                        0 => {},
                                                        1 => {
                                                            0 => {
                                                                0 => {},
                                                                1 => {Bitcoin::IsZero8}
                                                            },
                                                            1 => {
                                                                0 => {
                                                                    0 => {
                                                                        0 => {
                                                                            0 => {Bitcoin::IsZero16},
                                                                            1 => {Bitcoin::IsZero32}
                                                                        },
                                                                        1 => {
                                                                            0 => {Bitcoin::IsZero64},
                                                                            1 => {}
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
                                                                0 => {},
                                                                1 => {Bitcoin::IsOne8}
                                                            },
                                                            1 => {
                                                                0 => {
                                                                    0 => {
                                                                        0 => {
                                                                            0 => {Bitcoin::IsOne16},
                                                                            1 => {Bitcoin::IsOne32}
                                                                        },
                                                                        1 => {
                                                                            0 => {Bitcoin::IsOne64},
                                                                            1 => {}
                                                                        }
                                                                    },
                                                                    1 => {}
                                                                },
                                                                1 => {}
                                                            }
                                                        }
                                                    }
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
                                                                    0 => {
                                                                        0 => {},
                                                                        1 => {
                                                                            0 => {
                                                                                0 => {},
                                                                                1 => {Bitcoin::Le8}
                                                                            },
                                                                            1 => {
                                                                                0 => {
                                                                                    0 => {
                                                                                        0 => {
                                                                                            0 => {Bitcoin::Le16},
                                                                                            1 => {Bitcoin::Le32}
                                                                                        },
                                                                                        1 => {
                                                                                            0 => {Bitcoin::Le64},
                                                                                            1 => {}
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
                                                                                0 => {},
                                                                                1 => {Bitcoin::Lt8}
                                                                            },
                                                                            1 => {
                                                                                0 => {
                                                                                    0 => {
                                                                                        0 => {
                                                                                            0 => {Bitcoin::Lt16},
                                                                                            1 => {Bitcoin::Lt32}
                                                                                        },
                                                                                        1 => {
                                                                                            0 => {Bitcoin::Lt64},
                                                                                            1 => {}
                                                                                        }
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
                                                                        0 => {},
                                                                        1 => {
                                                                            0 => {
                                                                                0 => {},
                                                                                1 => {Bitcoin::Min8}
                                                                            },
                                                                            1 => {
                                                                                0 => {
                                                                                    0 => {
                                                                                        0 => {
                                                                                            0 => {Bitcoin::Min16},
                                                                                            1 => {Bitcoin::Min32}
                                                                                        },
                                                                                        1 => {
                                                                                            0 => {Bitcoin::Min64},
                                                                                            1 => {}
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
                                                                                0 => {},
                                                                                1 => {Bitcoin::Max8}
                                                                            },
                                                                            1 => {
                                                                                0 => {
                                                                                    0 => {
                                                                                        0 => {
                                                                                            0 => {Bitcoin::Max16},
                                                                                            1 => {Bitcoin::Max32}
                                                                                        },
                                                                                        1 => {
                                                                                            0 => {Bitcoin::Max64},
                                                                                            1 => {}
                                                                                        }
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
                                                                        0 => {},
                                                                        1 => {
                                                                            0 => {
                                                                                0 => {},
                                                                                1 => {Bitcoin::Median8}
                                                                            },
                                                                            1 => {
                                                                                0 => {
                                                                                    0 => {
                                                                                        0 => {
                                                                                            0 => {Bitcoin::Median16},
                                                                                            1 => {Bitcoin::Median32}
                                                                                        },
                                                                                        1 => {
                                                                                            0 => {Bitcoin::Median64},
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
                                                                },
                                                                1 => {
                                                                    0 => {
                                                                        0 => {},
                                                                        1 => {
                                                                            0 => {
                                                                                0 => {},
                                                                                1 => {Bitcoin::DivMod8}
                                                                            },
                                                                            1 => {
                                                                                0 => {
                                                                                    0 => {
                                                                                        0 => {
                                                                                            0 => {Bitcoin::DivMod16},
                                                                                            1 => {Bitcoin::DivMod32}
                                                                                        },
                                                                                        1 => {
                                                                                            0 => {Bitcoin::DivMod64},
                                                                                            1 => {}
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
                                                                                0 => {},
                                                                                1 => {Bitcoin::Divide8}
                                                                            },
                                                                            1 => {
                                                                                0 => {
                                                                                    0 => {
                                                                                        0 => {
                                                                                            0 => {Bitcoin::Divide16},
                                                                                            1 => {Bitcoin::Divide32}
                                                                                        },
                                                                                        1 => {
                                                                                            0 => {Bitcoin::Divide64},
                                                                                            1 => {}
                                                                                        }
                                                                                    },
                                                                                    1 => {}
                                                                                },
                                                                                1 => {}
                                                                            }
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
                                                                            0 => {
                                                                                0 => {},
                                                                                1 => {Bitcoin::Modulo8}
                                                                            },
                                                                            1 => {
                                                                                0 => {
                                                                                    0 => {
                                                                                        0 => {
                                                                                            0 => {Bitcoin::Modulo16},
                                                                                            1 => {Bitcoin::Modulo32}
                                                                                        },
                                                                                        1 => {
                                                                                            0 => {Bitcoin::Modulo64},
                                                                                            1 => {}
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
                                                                                0 => {},
                                                                                1 => {Bitcoin::Divides8}
                                                                            },
                                                                            1 => {
                                                                                0 => {
                                                                                    0 => {
                                                                                        0 => {
                                                                                            0 => {Bitcoin::Divides16},
                                                                                            1 => {Bitcoin::Divides32}
                                                                                        },
                                                                                        1 => {
                                                                                            0 => {Bitcoin::Divides64},
                                                                                            1 => {}
                                                                                        }
                                                                                    },
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
            Bitcoin::Add16 => &simplicity_sys::c_jets::jets_wrapper::add_16,
            Bitcoin::Add32 => &simplicity_sys::c_jets::jets_wrapper::add_32,
            Bitcoin::Add64 => &simplicity_sys::c_jets::jets_wrapper::add_64,
            Bitcoin::Add8 => &simplicity_sys::c_jets::jets_wrapper::add_8,
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
            Bitcoin::Decrement16 => &simplicity_sys::c_jets::jets_wrapper::decrement_16,
            Bitcoin::Decrement32 => &simplicity_sys::c_jets::jets_wrapper::decrement_32,
            Bitcoin::Decrement64 => &simplicity_sys::c_jets::jets_wrapper::decrement_64,
            Bitcoin::Decrement8 => &simplicity_sys::c_jets::jets_wrapper::decrement_8,
            Bitcoin::DivMod16 => &simplicity_sys::c_jets::jets_wrapper::div_mod_16,
            Bitcoin::DivMod32 => &simplicity_sys::c_jets::jets_wrapper::div_mod_32,
            Bitcoin::DivMod64 => &simplicity_sys::c_jets::jets_wrapper::div_mod_64,
            Bitcoin::DivMod8 => &simplicity_sys::c_jets::jets_wrapper::div_mod_8,
            Bitcoin::Divide16 => &simplicity_sys::c_jets::jets_wrapper::divide_16,
            Bitcoin::Divide32 => &simplicity_sys::c_jets::jets_wrapper::divide_32,
            Bitcoin::Divide64 => &simplicity_sys::c_jets::jets_wrapper::divide_64,
            Bitcoin::Divide8 => &simplicity_sys::c_jets::jets_wrapper::divide_8,
            Bitcoin::Divides16 => &simplicity_sys::c_jets::jets_wrapper::divides_16,
            Bitcoin::Divides32 => &simplicity_sys::c_jets::jets_wrapper::divides_32,
            Bitcoin::Divides64 => &simplicity_sys::c_jets::jets_wrapper::divides_64,
            Bitcoin::Divides8 => &simplicity_sys::c_jets::jets_wrapper::divides_8,
            Bitcoin::Eq16 => &simplicity_sys::c_jets::jets_wrapper::eq_16,
            Bitcoin::Eq256 => &simplicity_sys::c_jets::jets_wrapper::eq_256,
            Bitcoin::Eq32 => &simplicity_sys::c_jets::jets_wrapper::eq_32,
            Bitcoin::Eq64 => &simplicity_sys::c_jets::jets_wrapper::eq_64,
            Bitcoin::Eq8 => &simplicity_sys::c_jets::jets_wrapper::eq_8,
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
            Bitcoin::FullAdd16 => &simplicity_sys::c_jets::jets_wrapper::full_add_16,
            Bitcoin::FullAdd32 => &simplicity_sys::c_jets::jets_wrapper::full_add_32,
            Bitcoin::FullAdd64 => &simplicity_sys::c_jets::jets_wrapper::full_add_64,
            Bitcoin::FullAdd8 => &simplicity_sys::c_jets::jets_wrapper::full_add_8,
            Bitcoin::FullDecrement16 => &simplicity_sys::c_jets::jets_wrapper::full_decrement_16,
            Bitcoin::FullDecrement32 => &simplicity_sys::c_jets::jets_wrapper::full_decrement_32,
            Bitcoin::FullDecrement64 => &simplicity_sys::c_jets::jets_wrapper::full_decrement_64,
            Bitcoin::FullDecrement8 => &simplicity_sys::c_jets::jets_wrapper::full_decrement_8,
            Bitcoin::FullIncrement16 => &simplicity_sys::c_jets::jets_wrapper::full_increment_16,
            Bitcoin::FullIncrement32 => &simplicity_sys::c_jets::jets_wrapper::full_increment_32,
            Bitcoin::FullIncrement64 => &simplicity_sys::c_jets::jets_wrapper::full_increment_64,
            Bitcoin::FullIncrement8 => &simplicity_sys::c_jets::jets_wrapper::full_increment_8,
            Bitcoin::FullMultiply16 => &simplicity_sys::c_jets::jets_wrapper::full_multiply_16,
            Bitcoin::FullMultiply32 => &simplicity_sys::c_jets::jets_wrapper::full_multiply_32,
            Bitcoin::FullMultiply64 => &simplicity_sys::c_jets::jets_wrapper::full_multiply_64,
            Bitcoin::FullMultiply8 => &simplicity_sys::c_jets::jets_wrapper::full_multiply_8,
            Bitcoin::FullSubtract16 => &simplicity_sys::c_jets::jets_wrapper::full_subtract_16,
            Bitcoin::FullSubtract32 => &simplicity_sys::c_jets::jets_wrapper::full_subtract_32,
            Bitcoin::FullSubtract64 => &simplicity_sys::c_jets::jets_wrapper::full_subtract_64,
            Bitcoin::FullSubtract8 => &simplicity_sys::c_jets::jets_wrapper::full_subtract_8,
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
            Bitcoin::Increment16 => &simplicity_sys::c_jets::jets_wrapper::increment_16,
            Bitcoin::Increment32 => &simplicity_sys::c_jets::jets_wrapper::increment_32,
            Bitcoin::Increment64 => &simplicity_sys::c_jets::jets_wrapper::increment_64,
            Bitcoin::Increment8 => &simplicity_sys::c_jets::jets_wrapper::increment_8,
            Bitcoin::InputAnnexHash => unimplemented!("Bitcoin jets have not yet been implemented."),
            Bitcoin::InputPrevOutpoint => unimplemented!("Bitcoin jets have not yet been implemented."),
            Bitcoin::InputScriptSigHash => unimplemented!("Bitcoin jets have not yet been implemented."),
            Bitcoin::InputSequence => unimplemented!("Bitcoin jets have not yet been implemented."),
            Bitcoin::InputValue => unimplemented!("Bitcoin jets have not yet been implemented."),
            Bitcoin::InternalKey => unimplemented!("Bitcoin jets have not yet been implemented."),
            Bitcoin::IsOne16 => &simplicity_sys::c_jets::jets_wrapper::is_one_16,
            Bitcoin::IsOne32 => &simplicity_sys::c_jets::jets_wrapper::is_one_32,
            Bitcoin::IsOne64 => &simplicity_sys::c_jets::jets_wrapper::is_one_64,
            Bitcoin::IsOne8 => &simplicity_sys::c_jets::jets_wrapper::is_one_8,
            Bitcoin::IsZero16 => &simplicity_sys::c_jets::jets_wrapper::is_zero_16,
            Bitcoin::IsZero32 => &simplicity_sys::c_jets::jets_wrapper::is_zero_32,
            Bitcoin::IsZero64 => &simplicity_sys::c_jets::jets_wrapper::is_zero_64,
            Bitcoin::IsZero8 => &simplicity_sys::c_jets::jets_wrapper::is_zero_8,
            Bitcoin::Le16 => &simplicity_sys::c_jets::jets_wrapper::le_16,
            Bitcoin::Le32 => &simplicity_sys::c_jets::jets_wrapper::le_32,
            Bitcoin::Le64 => &simplicity_sys::c_jets::jets_wrapper::le_64,
            Bitcoin::Le8 => &simplicity_sys::c_jets::jets_wrapper::le_8,
            Bitcoin::LinearCombination1 => &simplicity_sys::c_jets::jets_wrapper::linear_combination_1,
            Bitcoin::LinearVerify1 => &simplicity_sys::c_jets::jets_wrapper::linear_verify_1,
            Bitcoin::LockTime => unimplemented!("Bitcoin jets have not yet been implemented."),
            Bitcoin::Low16 => &simplicity_sys::c_jets::jets_wrapper::low_16,
            Bitcoin::Low32 => &simplicity_sys::c_jets::jets_wrapper::low_32,
            Bitcoin::Low64 => &simplicity_sys::c_jets::jets_wrapper::low_64,
            Bitcoin::Low8 => &simplicity_sys::c_jets::jets_wrapper::low_8,
            Bitcoin::Lt16 => &simplicity_sys::c_jets::jets_wrapper::lt_16,
            Bitcoin::Lt32 => &simplicity_sys::c_jets::jets_wrapper::lt_32,
            Bitcoin::Lt64 => &simplicity_sys::c_jets::jets_wrapper::lt_64,
            Bitcoin::Lt8 => &simplicity_sys::c_jets::jets_wrapper::lt_8,
            Bitcoin::Max16 => &simplicity_sys::c_jets::jets_wrapper::max_16,
            Bitcoin::Max32 => &simplicity_sys::c_jets::jets_wrapper::max_32,
            Bitcoin::Max64 => &simplicity_sys::c_jets::jets_wrapper::max_64,
            Bitcoin::Max8 => &simplicity_sys::c_jets::jets_wrapper::max_8,
            Bitcoin::Median16 => &simplicity_sys::c_jets::jets_wrapper::median_16,
            Bitcoin::Median32 => &simplicity_sys::c_jets::jets_wrapper::median_32,
            Bitcoin::Median64 => &simplicity_sys::c_jets::jets_wrapper::median_64,
            Bitcoin::Median8 => &simplicity_sys::c_jets::jets_wrapper::median_8,
            Bitcoin::Min16 => &simplicity_sys::c_jets::jets_wrapper::min_16,
            Bitcoin::Min32 => &simplicity_sys::c_jets::jets_wrapper::min_32,
            Bitcoin::Min64 => &simplicity_sys::c_jets::jets_wrapper::min_64,
            Bitcoin::Min8 => &simplicity_sys::c_jets::jets_wrapper::min_8,
            Bitcoin::Modulo16 => &simplicity_sys::c_jets::jets_wrapper::modulo_16,
            Bitcoin::Modulo32 => &simplicity_sys::c_jets::jets_wrapper::modulo_32,
            Bitcoin::Modulo64 => &simplicity_sys::c_jets::jets_wrapper::modulo_64,
            Bitcoin::Modulo8 => &simplicity_sys::c_jets::jets_wrapper::modulo_8,
            Bitcoin::Multiply16 => &simplicity_sys::c_jets::jets_wrapper::multiply_16,
            Bitcoin::Multiply32 => &simplicity_sys::c_jets::jets_wrapper::multiply_32,
            Bitcoin::Multiply64 => &simplicity_sys::c_jets::jets_wrapper::multiply_64,
            Bitcoin::Multiply8 => &simplicity_sys::c_jets::jets_wrapper::multiply_8,
            Bitcoin::Negate16 => &simplicity_sys::c_jets::jets_wrapper::negate_16,
            Bitcoin::Negate32 => &simplicity_sys::c_jets::jets_wrapper::negate_32,
            Bitcoin::Negate64 => &simplicity_sys::c_jets::jets_wrapper::negate_64,
            Bitcoin::Negate8 => &simplicity_sys::c_jets::jets_wrapper::negate_8,
            Bitcoin::NumInputs => unimplemented!("Bitcoin jets have not yet been implemented."),
            Bitcoin::NumOutputs => unimplemented!("Bitcoin jets have not yet been implemented."),
            Bitcoin::One16 => &simplicity_sys::c_jets::jets_wrapper::one_16,
            Bitcoin::One32 => &simplicity_sys::c_jets::jets_wrapper::one_32,
            Bitcoin::One64 => &simplicity_sys::c_jets::jets_wrapper::one_64,
            Bitcoin::One8 => &simplicity_sys::c_jets::jets_wrapper::one_8,
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
            Bitcoin::Subtract16 => &simplicity_sys::c_jets::jets_wrapper::subtract_16,
            Bitcoin::Subtract32 => &simplicity_sys::c_jets::jets_wrapper::subtract_32,
            Bitcoin::Subtract64 => &simplicity_sys::c_jets::jets_wrapper::subtract_64,
            Bitcoin::Subtract8 => &simplicity_sys::c_jets::jets_wrapper::subtract_8,
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

impl fmt::Display for Bitcoin {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Bitcoin::Add16 => f.write_str("add_16"),
            Bitcoin::Add32 => f.write_str("add_32"),
            Bitcoin::Add64 => f.write_str("add_64"),
            Bitcoin::Add8 => f.write_str("add_8"),
            Bitcoin::Bip0340Verify => f.write_str("bip_0340_verify"),
            Bitcoin::CheckLockDistance => f.write_str("check_lock_distance"),
            Bitcoin::CheckLockDuration => f.write_str("check_lock_duration"),
            Bitcoin::CheckLockHeight => f.write_str("check_lock_height"),
            Bitcoin::CheckLockTime => f.write_str("check_lock_time"),
            Bitcoin::CheckSigVerify => f.write_str("check_sig_verify"),
            Bitcoin::CurrentAnnexHash => f.write_str("current_annex_hash"),
            Bitcoin::CurrentIndex => f.write_str("current_index"),
            Bitcoin::CurrentPrevOutpoint => f.write_str("current_prev_outpoint"),
            Bitcoin::CurrentScriptSigHash => f.write_str("current_script_sig_hash"),
            Bitcoin::CurrentSequence => f.write_str("current_sequence"),
            Bitcoin::CurrentValue => f.write_str("current_value"),
            Bitcoin::Decompress => f.write_str("decompress"),
            Bitcoin::Decrement16 => f.write_str("decrement_16"),
            Bitcoin::Decrement32 => f.write_str("decrement_32"),
            Bitcoin::Decrement64 => f.write_str("decrement_64"),
            Bitcoin::Decrement8 => f.write_str("decrement_8"),
            Bitcoin::DivMod16 => f.write_str("div_mod_16"),
            Bitcoin::DivMod32 => f.write_str("div_mod_32"),
            Bitcoin::DivMod64 => f.write_str("div_mod_64"),
            Bitcoin::DivMod8 => f.write_str("div_mod_8"),
            Bitcoin::Divide16 => f.write_str("divide_16"),
            Bitcoin::Divide32 => f.write_str("divide_32"),
            Bitcoin::Divide64 => f.write_str("divide_64"),
            Bitcoin::Divide8 => f.write_str("divide_8"),
            Bitcoin::Divides16 => f.write_str("divides_16"),
            Bitcoin::Divides32 => f.write_str("divides_32"),
            Bitcoin::Divides64 => f.write_str("divides_64"),
            Bitcoin::Divides8 => f.write_str("divides_8"),
            Bitcoin::Eq16 => f.write_str("eq_16"),
            Bitcoin::Eq256 => f.write_str("eq_256"),
            Bitcoin::Eq32 => f.write_str("eq_32"),
            Bitcoin::Eq64 => f.write_str("eq_64"),
            Bitcoin::Eq8 => f.write_str("eq_8"),
            Bitcoin::FeAdd => f.write_str("fe_add"),
            Bitcoin::FeInvert => f.write_str("fe_invert"),
            Bitcoin::FeIsOdd => f.write_str("fe_is_odd"),
            Bitcoin::FeIsZero => f.write_str("fe_is_zero"),
            Bitcoin::FeMultiply => f.write_str("fe_multiply"),
            Bitcoin::FeMultiplyBeta => f.write_str("fe_multiply_beta"),
            Bitcoin::FeNegate => f.write_str("fe_negate"),
            Bitcoin::FeNormalize => f.write_str("fe_normalize"),
            Bitcoin::FeSquare => f.write_str("fe_square"),
            Bitcoin::FeSquareRoot => f.write_str("fe_square_root"),
            Bitcoin::FullAdd16 => f.write_str("full_add_16"),
            Bitcoin::FullAdd32 => f.write_str("full_add_32"),
            Bitcoin::FullAdd64 => f.write_str("full_add_64"),
            Bitcoin::FullAdd8 => f.write_str("full_add_8"),
            Bitcoin::FullDecrement16 => f.write_str("full_decrement_16"),
            Bitcoin::FullDecrement32 => f.write_str("full_decrement_32"),
            Bitcoin::FullDecrement64 => f.write_str("full_decrement_64"),
            Bitcoin::FullDecrement8 => f.write_str("full_decrement_8"),
            Bitcoin::FullIncrement16 => f.write_str("full_increment_16"),
            Bitcoin::FullIncrement32 => f.write_str("full_increment_32"),
            Bitcoin::FullIncrement64 => f.write_str("full_increment_64"),
            Bitcoin::FullIncrement8 => f.write_str("full_increment_8"),
            Bitcoin::FullMultiply16 => f.write_str("full_multiply_16"),
            Bitcoin::FullMultiply32 => f.write_str("full_multiply_32"),
            Bitcoin::FullMultiply64 => f.write_str("full_multiply_64"),
            Bitcoin::FullMultiply8 => f.write_str("full_multiply_8"),
            Bitcoin::FullSubtract16 => f.write_str("full_subtract_16"),
            Bitcoin::FullSubtract32 => f.write_str("full_subtract_32"),
            Bitcoin::FullSubtract64 => f.write_str("full_subtract_64"),
            Bitcoin::FullSubtract8 => f.write_str("full_subtract_8"),
            Bitcoin::GeIsOnCurve => f.write_str("ge_is_on_curve"),
            Bitcoin::GeNegate => f.write_str("ge_negate"),
            Bitcoin::GejAdd => f.write_str("gej_add"),
            Bitcoin::GejDouble => f.write_str("gej_double"),
            Bitcoin::GejGeAdd => f.write_str("gej_ge_add"),
            Bitcoin::GejGeAddEx => f.write_str("gej_ge_add_ex"),
            Bitcoin::GejInfinity => f.write_str("gej_infinity"),
            Bitcoin::GejIsInfinity => f.write_str("gej_is_infinity"),
            Bitcoin::GejIsOnCurve => f.write_str("gej_is_on_curve"),
            Bitcoin::GejNegate => f.write_str("gej_negate"),
            Bitcoin::GejNormalize => f.write_str("gej_normalize"),
            Bitcoin::GejRescale => f.write_str("gej_rescale"),
            Bitcoin::GejXEquiv => f.write_str("gej_x_equiv"),
            Bitcoin::GejYIsOdd => f.write_str("gej_y_is_odd"),
            Bitcoin::Generate => f.write_str("generate"),
            Bitcoin::Increment16 => f.write_str("increment_16"),
            Bitcoin::Increment32 => f.write_str("increment_32"),
            Bitcoin::Increment64 => f.write_str("increment_64"),
            Bitcoin::Increment8 => f.write_str("increment_8"),
            Bitcoin::InputAnnexHash => f.write_str("input_annex_hash"),
            Bitcoin::InputPrevOutpoint => f.write_str("input_prev_outpoint"),
            Bitcoin::InputScriptSigHash => f.write_str("input_script_sig_hash"),
            Bitcoin::InputSequence => f.write_str("input_sequence"),
            Bitcoin::InputValue => f.write_str("input_value"),
            Bitcoin::InternalKey => f.write_str("internal_key"),
            Bitcoin::IsOne16 => f.write_str("is_one_16"),
            Bitcoin::IsOne32 => f.write_str("is_one_32"),
            Bitcoin::IsOne64 => f.write_str("is_one_64"),
            Bitcoin::IsOne8 => f.write_str("is_one_8"),
            Bitcoin::IsZero16 => f.write_str("is_zero_16"),
            Bitcoin::IsZero32 => f.write_str("is_zero_32"),
            Bitcoin::IsZero64 => f.write_str("is_zero_64"),
            Bitcoin::IsZero8 => f.write_str("is_zero_8"),
            Bitcoin::Le16 => f.write_str("le_16"),
            Bitcoin::Le32 => f.write_str("le_32"),
            Bitcoin::Le64 => f.write_str("le_64"),
            Bitcoin::Le8 => f.write_str("le_8"),
            Bitcoin::LinearCombination1 => f.write_str("linear_combination_1"),
            Bitcoin::LinearVerify1 => f.write_str("linear_verify_1"),
            Bitcoin::LockTime => f.write_str("lock_time"),
            Bitcoin::Low16 => f.write_str("low_16"),
            Bitcoin::Low32 => f.write_str("low_32"),
            Bitcoin::Low64 => f.write_str("low_64"),
            Bitcoin::Low8 => f.write_str("low_8"),
            Bitcoin::Lt16 => f.write_str("lt_16"),
            Bitcoin::Lt32 => f.write_str("lt_32"),
            Bitcoin::Lt64 => f.write_str("lt_64"),
            Bitcoin::Lt8 => f.write_str("lt_8"),
            Bitcoin::Max16 => f.write_str("max_16"),
            Bitcoin::Max32 => f.write_str("max_32"),
            Bitcoin::Max64 => f.write_str("max_64"),
            Bitcoin::Max8 => f.write_str("max_8"),
            Bitcoin::Median16 => f.write_str("median_16"),
            Bitcoin::Median32 => f.write_str("median_32"),
            Bitcoin::Median64 => f.write_str("median_64"),
            Bitcoin::Median8 => f.write_str("median_8"),
            Bitcoin::Min16 => f.write_str("min_16"),
            Bitcoin::Min32 => f.write_str("min_32"),
            Bitcoin::Min64 => f.write_str("min_64"),
            Bitcoin::Min8 => f.write_str("min_8"),
            Bitcoin::Modulo16 => f.write_str("modulo_16"),
            Bitcoin::Modulo32 => f.write_str("modulo_32"),
            Bitcoin::Modulo64 => f.write_str("modulo_64"),
            Bitcoin::Modulo8 => f.write_str("modulo_8"),
            Bitcoin::Multiply16 => f.write_str("multiply_16"),
            Bitcoin::Multiply32 => f.write_str("multiply_32"),
            Bitcoin::Multiply64 => f.write_str("multiply_64"),
            Bitcoin::Multiply8 => f.write_str("multiply_8"),
            Bitcoin::Negate16 => f.write_str("negate_16"),
            Bitcoin::Negate32 => f.write_str("negate_32"),
            Bitcoin::Negate64 => f.write_str("negate_64"),
            Bitcoin::Negate8 => f.write_str("negate_8"),
            Bitcoin::NumInputs => f.write_str("num_inputs"),
            Bitcoin::NumOutputs => f.write_str("num_outputs"),
            Bitcoin::One16 => f.write_str("one_16"),
            Bitcoin::One32 => f.write_str("one_32"),
            Bitcoin::One64 => f.write_str("one_64"),
            Bitcoin::One8 => f.write_str("one_8"),
            Bitcoin::OutputScriptHash => f.write_str("output_script_hash"),
            Bitcoin::OutputValue => f.write_str("output_value"),
            Bitcoin::ParseLock => f.write_str("parse_lock"),
            Bitcoin::ParseSequence => f.write_str("parse_sequence"),
            Bitcoin::PointVerify1 => f.write_str("point_verify_1"),
            Bitcoin::ScalarAdd => f.write_str("scalar_add"),
            Bitcoin::ScalarInvert => f.write_str("scalar_invert"),
            Bitcoin::ScalarIsZero => f.write_str("scalar_is_zero"),
            Bitcoin::ScalarMultiply => f.write_str("scalar_multiply"),
            Bitcoin::ScalarMultiplyLambda => f.write_str("scalar_multiply_lambda"),
            Bitcoin::ScalarNegate => f.write_str("scalar_negate"),
            Bitcoin::ScalarNormalize => f.write_str("scalar_normalize"),
            Bitcoin::ScalarSquare => f.write_str("scalar_square"),
            Bitcoin::Scale => f.write_str("scale"),
            Bitcoin::ScriptCMR => f.write_str("script_cmr"),
            Bitcoin::Sha256Block => f.write_str("sha_256_block"),
            Bitcoin::Sha256Ctx8Add1 => f.write_str("sha_256_ctx_8_add_1"),
            Bitcoin::Sha256Ctx8Add128 => f.write_str("sha_256_ctx_8_add_128"),
            Bitcoin::Sha256Ctx8Add16 => f.write_str("sha_256_ctx_8_add_16"),
            Bitcoin::Sha256Ctx8Add2 => f.write_str("sha_256_ctx_8_add_2"),
            Bitcoin::Sha256Ctx8Add256 => f.write_str("sha_256_ctx_8_add_256"),
            Bitcoin::Sha256Ctx8Add32 => f.write_str("sha_256_ctx_8_add_32"),
            Bitcoin::Sha256Ctx8Add4 => f.write_str("sha_256_ctx_8_add_4"),
            Bitcoin::Sha256Ctx8Add512 => f.write_str("sha_256_ctx_8_add_512"),
            Bitcoin::Sha256Ctx8Add64 => f.write_str("sha_256_ctx_8_add_64"),
            Bitcoin::Sha256Ctx8Add8 => f.write_str("sha_256_ctx_8_add_8"),
            Bitcoin::Sha256Ctx8AddBuffer511 => f.write_str("sha_256_ctx_8_add_buffer_511"),
            Bitcoin::Sha256Ctx8Finalize => f.write_str("sha_256_ctx_8_finalize"),
            Bitcoin::Sha256Ctx8Init => f.write_str("sha_256_ctx_8_init"),
            Bitcoin::Sha256Iv => f.write_str("sha_256_iv"),
            Bitcoin::Subtract16 => f.write_str("subtract_16"),
            Bitcoin::Subtract32 => f.write_str("subtract_32"),
            Bitcoin::Subtract64 => f.write_str("subtract_64"),
            Bitcoin::Subtract8 => f.write_str("subtract_8"),
            Bitcoin::Tapbranch => f.write_str("tapbranch"),
            Bitcoin::TapleafVersion => f.write_str("tapleaf_version"),
            Bitcoin::TotalInputValue => f.write_str("total_input_value"),
            Bitcoin::TotalOutputValue => f.write_str("total_output_value"),
            Bitcoin::TxIsFinal => f.write_str("tx_is_final"),
            Bitcoin::TxLockDistance => f.write_str("tx_lock_distance"),
            Bitcoin::TxLockDuration => f.write_str("tx_lock_duration"),
            Bitcoin::TxLockHeight => f.write_str("tx_lock_height"),
            Bitcoin::TxLockTime => f.write_str("tx_lock_time"),
            Bitcoin::Verify => f.write_str("verify"),
            Bitcoin::Version => f.write_str("version"),
        }
    }
}

impl str::FromStr for Bitcoin {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self, Error> {
        match s {
            "add_16" => Ok(Bitcoin::Add16),
            "add_32" => Ok(Bitcoin::Add32),
            "add_64" => Ok(Bitcoin::Add64),
            "add_8" => Ok(Bitcoin::Add8),
            "bip_0340_verify" => Ok(Bitcoin::Bip0340Verify),
            "check_lock_distance" => Ok(Bitcoin::CheckLockDistance),
            "check_lock_duration" => Ok(Bitcoin::CheckLockDuration),
            "check_lock_height" => Ok(Bitcoin::CheckLockHeight),
            "check_lock_time" => Ok(Bitcoin::CheckLockTime),
            "check_sig_verify" => Ok(Bitcoin::CheckSigVerify),
            "current_annex_hash" => Ok(Bitcoin::CurrentAnnexHash),
            "current_index" => Ok(Bitcoin::CurrentIndex),
            "current_prev_outpoint" => Ok(Bitcoin::CurrentPrevOutpoint),
            "current_script_sig_hash" => Ok(Bitcoin::CurrentScriptSigHash),
            "current_sequence" => Ok(Bitcoin::CurrentSequence),
            "current_value" => Ok(Bitcoin::CurrentValue),
            "decompress" => Ok(Bitcoin::Decompress),
            "decrement_16" => Ok(Bitcoin::Decrement16),
            "decrement_32" => Ok(Bitcoin::Decrement32),
            "decrement_64" => Ok(Bitcoin::Decrement64),
            "decrement_8" => Ok(Bitcoin::Decrement8),
            "div_mod_16" => Ok(Bitcoin::DivMod16),
            "div_mod_32" => Ok(Bitcoin::DivMod32),
            "div_mod_64" => Ok(Bitcoin::DivMod64),
            "div_mod_8" => Ok(Bitcoin::DivMod8),
            "divide_16" => Ok(Bitcoin::Divide16),
            "divide_32" => Ok(Bitcoin::Divide32),
            "divide_64" => Ok(Bitcoin::Divide64),
            "divide_8" => Ok(Bitcoin::Divide8),
            "divides_16" => Ok(Bitcoin::Divides16),
            "divides_32" => Ok(Bitcoin::Divides32),
            "divides_64" => Ok(Bitcoin::Divides64),
            "divides_8" => Ok(Bitcoin::Divides8),
            "eq_16" => Ok(Bitcoin::Eq16),
            "eq_256" => Ok(Bitcoin::Eq256),
            "eq_32" => Ok(Bitcoin::Eq32),
            "eq_64" => Ok(Bitcoin::Eq64),
            "eq_8" => Ok(Bitcoin::Eq8),
            "fe_add" => Ok(Bitcoin::FeAdd),
            "fe_invert" => Ok(Bitcoin::FeInvert),
            "fe_is_odd" => Ok(Bitcoin::FeIsOdd),
            "fe_is_zero" => Ok(Bitcoin::FeIsZero),
            "fe_multiply" => Ok(Bitcoin::FeMultiply),
            "fe_multiply_beta" => Ok(Bitcoin::FeMultiplyBeta),
            "fe_negate" => Ok(Bitcoin::FeNegate),
            "fe_normalize" => Ok(Bitcoin::FeNormalize),
            "fe_square" => Ok(Bitcoin::FeSquare),
            "fe_square_root" => Ok(Bitcoin::FeSquareRoot),
            "full_add_16" => Ok(Bitcoin::FullAdd16),
            "full_add_32" => Ok(Bitcoin::FullAdd32),
            "full_add_64" => Ok(Bitcoin::FullAdd64),
            "full_add_8" => Ok(Bitcoin::FullAdd8),
            "full_decrement_16" => Ok(Bitcoin::FullDecrement16),
            "full_decrement_32" => Ok(Bitcoin::FullDecrement32),
            "full_decrement_64" => Ok(Bitcoin::FullDecrement64),
            "full_decrement_8" => Ok(Bitcoin::FullDecrement8),
            "full_increment_16" => Ok(Bitcoin::FullIncrement16),
            "full_increment_32" => Ok(Bitcoin::FullIncrement32),
            "full_increment_64" => Ok(Bitcoin::FullIncrement64),
            "full_increment_8" => Ok(Bitcoin::FullIncrement8),
            "full_multiply_16" => Ok(Bitcoin::FullMultiply16),
            "full_multiply_32" => Ok(Bitcoin::FullMultiply32),
            "full_multiply_64" => Ok(Bitcoin::FullMultiply64),
            "full_multiply_8" => Ok(Bitcoin::FullMultiply8),
            "full_subtract_16" => Ok(Bitcoin::FullSubtract16),
            "full_subtract_32" => Ok(Bitcoin::FullSubtract32),
            "full_subtract_64" => Ok(Bitcoin::FullSubtract64),
            "full_subtract_8" => Ok(Bitcoin::FullSubtract8),
            "ge_is_on_curve" => Ok(Bitcoin::GeIsOnCurve),
            "ge_negate" => Ok(Bitcoin::GeNegate),
            "gej_add" => Ok(Bitcoin::GejAdd),
            "gej_double" => Ok(Bitcoin::GejDouble),
            "gej_ge_add" => Ok(Bitcoin::GejGeAdd),
            "gej_ge_add_ex" => Ok(Bitcoin::GejGeAddEx),
            "gej_infinity" => Ok(Bitcoin::GejInfinity),
            "gej_is_infinity" => Ok(Bitcoin::GejIsInfinity),
            "gej_is_on_curve" => Ok(Bitcoin::GejIsOnCurve),
            "gej_negate" => Ok(Bitcoin::GejNegate),
            "gej_normalize" => Ok(Bitcoin::GejNormalize),
            "gej_rescale" => Ok(Bitcoin::GejRescale),
            "gej_x_equiv" => Ok(Bitcoin::GejXEquiv),
            "gej_y_is_odd" => Ok(Bitcoin::GejYIsOdd),
            "generate" => Ok(Bitcoin::Generate),
            "increment_16" => Ok(Bitcoin::Increment16),
            "increment_32" => Ok(Bitcoin::Increment32),
            "increment_64" => Ok(Bitcoin::Increment64),
            "increment_8" => Ok(Bitcoin::Increment8),
            "input_annex_hash" => Ok(Bitcoin::InputAnnexHash),
            "input_prev_outpoint" => Ok(Bitcoin::InputPrevOutpoint),
            "input_script_sig_hash" => Ok(Bitcoin::InputScriptSigHash),
            "input_sequence" => Ok(Bitcoin::InputSequence),
            "input_value" => Ok(Bitcoin::InputValue),
            "internal_key" => Ok(Bitcoin::InternalKey),
            "is_one_16" => Ok(Bitcoin::IsOne16),
            "is_one_32" => Ok(Bitcoin::IsOne32),
            "is_one_64" => Ok(Bitcoin::IsOne64),
            "is_one_8" => Ok(Bitcoin::IsOne8),
            "is_zero_16" => Ok(Bitcoin::IsZero16),
            "is_zero_32" => Ok(Bitcoin::IsZero32),
            "is_zero_64" => Ok(Bitcoin::IsZero64),
            "is_zero_8" => Ok(Bitcoin::IsZero8),
            "le_16" => Ok(Bitcoin::Le16),
            "le_32" => Ok(Bitcoin::Le32),
            "le_64" => Ok(Bitcoin::Le64),
            "le_8" => Ok(Bitcoin::Le8),
            "linear_combination_1" => Ok(Bitcoin::LinearCombination1),
            "linear_verify_1" => Ok(Bitcoin::LinearVerify1),
            "lock_time" => Ok(Bitcoin::LockTime),
            "low_16" => Ok(Bitcoin::Low16),
            "low_32" => Ok(Bitcoin::Low32),
            "low_64" => Ok(Bitcoin::Low64),
            "low_8" => Ok(Bitcoin::Low8),
            "lt_16" => Ok(Bitcoin::Lt16),
            "lt_32" => Ok(Bitcoin::Lt32),
            "lt_64" => Ok(Bitcoin::Lt64),
            "lt_8" => Ok(Bitcoin::Lt8),
            "max_16" => Ok(Bitcoin::Max16),
            "max_32" => Ok(Bitcoin::Max32),
            "max_64" => Ok(Bitcoin::Max64),
            "max_8" => Ok(Bitcoin::Max8),
            "median_16" => Ok(Bitcoin::Median16),
            "median_32" => Ok(Bitcoin::Median32),
            "median_64" => Ok(Bitcoin::Median64),
            "median_8" => Ok(Bitcoin::Median8),
            "min_16" => Ok(Bitcoin::Min16),
            "min_32" => Ok(Bitcoin::Min32),
            "min_64" => Ok(Bitcoin::Min64),
            "min_8" => Ok(Bitcoin::Min8),
            "modulo_16" => Ok(Bitcoin::Modulo16),
            "modulo_32" => Ok(Bitcoin::Modulo32),
            "modulo_64" => Ok(Bitcoin::Modulo64),
            "modulo_8" => Ok(Bitcoin::Modulo8),
            "multiply_16" => Ok(Bitcoin::Multiply16),
            "multiply_32" => Ok(Bitcoin::Multiply32),
            "multiply_64" => Ok(Bitcoin::Multiply64),
            "multiply_8" => Ok(Bitcoin::Multiply8),
            "negate_16" => Ok(Bitcoin::Negate16),
            "negate_32" => Ok(Bitcoin::Negate32),
            "negate_64" => Ok(Bitcoin::Negate64),
            "negate_8" => Ok(Bitcoin::Negate8),
            "num_inputs" => Ok(Bitcoin::NumInputs),
            "num_outputs" => Ok(Bitcoin::NumOutputs),
            "one_16" => Ok(Bitcoin::One16),
            "one_32" => Ok(Bitcoin::One32),
            "one_64" => Ok(Bitcoin::One64),
            "one_8" => Ok(Bitcoin::One8),
            "output_script_hash" => Ok(Bitcoin::OutputScriptHash),
            "output_value" => Ok(Bitcoin::OutputValue),
            "parse_lock" => Ok(Bitcoin::ParseLock),
            "parse_sequence" => Ok(Bitcoin::ParseSequence),
            "point_verify_1" => Ok(Bitcoin::PointVerify1),
            "scalar_add" => Ok(Bitcoin::ScalarAdd),
            "scalar_invert" => Ok(Bitcoin::ScalarInvert),
            "scalar_is_zero" => Ok(Bitcoin::ScalarIsZero),
            "scalar_multiply" => Ok(Bitcoin::ScalarMultiply),
            "scalar_multiply_lambda" => Ok(Bitcoin::ScalarMultiplyLambda),
            "scalar_negate" => Ok(Bitcoin::ScalarNegate),
            "scalar_normalize" => Ok(Bitcoin::ScalarNormalize),
            "scalar_square" => Ok(Bitcoin::ScalarSquare),
            "scale" => Ok(Bitcoin::Scale),
            "script_cmr" => Ok(Bitcoin::ScriptCMR),
            "sha_256_block" => Ok(Bitcoin::Sha256Block),
            "sha_256_ctx_8_add_1" => Ok(Bitcoin::Sha256Ctx8Add1),
            "sha_256_ctx_8_add_128" => Ok(Bitcoin::Sha256Ctx8Add128),
            "sha_256_ctx_8_add_16" => Ok(Bitcoin::Sha256Ctx8Add16),
            "sha_256_ctx_8_add_2" => Ok(Bitcoin::Sha256Ctx8Add2),
            "sha_256_ctx_8_add_256" => Ok(Bitcoin::Sha256Ctx8Add256),
            "sha_256_ctx_8_add_32" => Ok(Bitcoin::Sha256Ctx8Add32),
            "sha_256_ctx_8_add_4" => Ok(Bitcoin::Sha256Ctx8Add4),
            "sha_256_ctx_8_add_512" => Ok(Bitcoin::Sha256Ctx8Add512),
            "sha_256_ctx_8_add_64" => Ok(Bitcoin::Sha256Ctx8Add64),
            "sha_256_ctx_8_add_8" => Ok(Bitcoin::Sha256Ctx8Add8),
            "sha_256_ctx_8_add_buffer_511" => Ok(Bitcoin::Sha256Ctx8AddBuffer511),
            "sha_256_ctx_8_finalize" => Ok(Bitcoin::Sha256Ctx8Finalize),
            "sha_256_ctx_8_init" => Ok(Bitcoin::Sha256Ctx8Init),
            "sha_256_iv" => Ok(Bitcoin::Sha256Iv),
            "subtract_16" => Ok(Bitcoin::Subtract16),
            "subtract_32" => Ok(Bitcoin::Subtract32),
            "subtract_64" => Ok(Bitcoin::Subtract64),
            "subtract_8" => Ok(Bitcoin::Subtract8),
            "tapbranch" => Ok(Bitcoin::Tapbranch),
            "tapleaf_version" => Ok(Bitcoin::TapleafVersion),
            "total_input_value" => Ok(Bitcoin::TotalInputValue),
            "total_output_value" => Ok(Bitcoin::TotalOutputValue),
            "tx_is_final" => Ok(Bitcoin::TxIsFinal),
            "tx_lock_distance" => Ok(Bitcoin::TxLockDistance),
            "tx_lock_duration" => Ok(Bitcoin::TxLockDuration),
            "tx_lock_height" => Ok(Bitcoin::TxLockHeight),
            "tx_lock_time" => Ok(Bitcoin::TxLockTime),
            "verify" => Ok(Bitcoin::Verify),
            "version" => Ok(Bitcoin::Version),
            x => Err(Error::InvalidJetName(x.to_owned())),
        }
    }
}
