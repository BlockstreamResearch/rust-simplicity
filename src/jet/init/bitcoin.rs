/* This file has been automatically generated. */

use crate::jet::type_name::TypeName;
use crate::jet::Jet;
use crate::merkle::cmr::Cmr;
use crate::decode_bits;
use crate::{decode, BitIter, BitWriter};
use crate::analysis::Cost;
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
    All16,
    All32,
    All64,
    All8,
    And16,
    And32,
    And64,
    And8,
    Bip0340Verify,
    Ch16,
    Ch32,
    Ch64,
    Ch8,
    CheckLockDistance,
    CheckLockDuration,
    CheckLockHeight,
    CheckLockTime,
    CheckSigVerify,
    Complement16,
    Complement32,
    Complement64,
    Complement8,
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
    High16,
    High32,
    High64,
    High8,
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
    Maj16,
    Maj32,
    Maj64,
    Maj8,
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
    Or16,
    Or32,
    Or64,
    Or8,
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
    Some16,
    Some32,
    Some64,
    Some8,
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
    Xor16,
    Xor32,
    Xor64,
    Xor8,
    XorXor16,
    XorXor32,
    XorXor64,
    XorXor8,
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
                0x55, 0xcf, 0x8c, 0xe6, 0x6e, 0x27, 0x5d, 0x01, 0xaf, 0xc1, 0xfb, 0x35, 0x10, 0xd0,
                0xb5, 0x3f, 0xab, 0x54, 0xa4, 0x78, 0x2a, 0x3a, 0x48, 0xdd, 0xac, 0x49, 0x4e, 0x96,
                0x72, 0xfa, 0x04, 0x5d,
            ],
            Bitcoin::Add32 => [
                0xfa, 0x87, 0x4e, 0x40, 0x62, 0x72, 0xc9, 0x0c, 0xf0, 0xb8, 0x7b, 0x1e, 0x7b, 0x89,
                0xf6, 0x61, 0xf3, 0x34, 0x1b, 0x0d, 0x61, 0x63, 0x5c, 0xf6, 0xf5, 0xa8, 0xa2, 0xa5,
                0x32, 0x88, 0xcf, 0xe4,
            ],
            Bitcoin::Add64 => [
                0x12, 0x56, 0xbc, 0x57, 0x26, 0xcd, 0xb9, 0x54, 0x58, 0x21, 0xd3, 0xd9, 0x6e, 0x7f,
                0xc7, 0x32, 0x86, 0xd7, 0xdb, 0x23, 0xae, 0x65, 0x17, 0x10, 0xff, 0x05, 0xfc, 0xc0,
                0xa9, 0xb7, 0xc3, 0xe9,
            ],
            Bitcoin::Add8 => [
                0xd9, 0x4b, 0x8f, 0xb6, 0xb6, 0x02, 0x5d, 0x8d, 0x02, 0x9d, 0xf1, 0xb0, 0xa5, 0x26,
                0xa4, 0x2b, 0x3e, 0xae, 0x94, 0xc8, 0xa9, 0x93, 0x6c, 0xe7, 0xe2, 0xa3, 0xb0, 0x00,
                0x2e, 0x7f, 0x44, 0x5d,
            ],
            Bitcoin::All16 => [
                0xc2, 0x62, 0x4c, 0x7f, 0x52, 0x9d, 0x7f, 0x8e, 0x7c, 0xb9, 0x69, 0x24, 0xdf, 0x5c,
                0x48, 0xda, 0x2b, 0xf2, 0xd9, 0x27, 0x96, 0x66, 0x2b, 0x0c, 0x76, 0x78, 0x21, 0xe1,
                0x9e, 0x7a, 0x36, 0x00,
            ],
            Bitcoin::All32 => [
                0x4c, 0x06, 0x0c, 0x8b, 0xd9, 0x70, 0x24, 0x41, 0x11, 0x74, 0x65, 0x60, 0xdd, 0x55,
                0xfb, 0xcd, 0x61, 0x04, 0xae, 0x34, 0x35, 0xaa, 0x43, 0x14, 0xe4, 0xac, 0x0e, 0x05,
                0x31, 0x20, 0x5e, 0x20,
            ],
            Bitcoin::All64 => [
                0xf5, 0x2f, 0x9f, 0xeb, 0xc7, 0x11, 0x3b, 0xb1, 0xca, 0x14, 0xe3, 0xd2, 0xff, 0x9c,
                0x16, 0x1a, 0x59, 0x8d, 0xe5, 0xaf, 0xe9, 0x28, 0xc6, 0x91, 0x35, 0x5b, 0x9e, 0x86,
                0xbe, 0x13, 0x12, 0x67,
            ],
            Bitcoin::All8 => [
                0xe5, 0x1e, 0x0e, 0x23, 0x93, 0x31, 0x5a, 0x99, 0xbf, 0xdb, 0x33, 0x58, 0x4a, 0x4e,
                0xb6, 0xcd, 0x6a, 0x3d, 0x03, 0xf1, 0xa7, 0x60, 0xea, 0xa0, 0x58, 0xb1, 0x4d, 0x33,
                0xd2, 0xd3, 0xf8, 0x2a,
            ],
            Bitcoin::And16 => [
                0x25, 0x17, 0x21, 0x39, 0x33, 0x59, 0x3c, 0x62, 0x72, 0xd8, 0xaa, 0x0c, 0x1d, 0x99,
                0x6a, 0x3a, 0x1c, 0x56, 0xf8, 0x0d, 0xd3, 0x5b, 0x05, 0x44, 0x96, 0xf9, 0xc6, 0x37,
                0xf6, 0x9f, 0xf9, 0x21,
            ],
            Bitcoin::And32 => [
                0x4f, 0x9d, 0x32, 0x6e, 0x9e, 0x51, 0x51, 0x52, 0x32, 0xf8, 0x47, 0x52, 0x33, 0xf7,
                0x50, 0xc0, 0xa5, 0x72, 0x12, 0x55, 0x80, 0x6d, 0x3d, 0x36, 0xb6, 0x3f, 0xfc, 0x14,
                0x99, 0x72, 0x51, 0x59,
            ],
            Bitcoin::And64 => [
                0xdf, 0x3a, 0x07, 0xb2, 0x40, 0xeb, 0x8c, 0x53, 0x33, 0xf7, 0xb0, 0x80, 0x3d, 0x81,
                0xc8, 0x75, 0x69, 0xef, 0x59, 0x97, 0xdb, 0x56, 0xac, 0x37, 0xad, 0x63, 0x05, 0x9f,
                0x0c, 0xc0, 0x77, 0x9c,
            ],
            Bitcoin::And8 => [
                0x14, 0x76, 0x9a, 0x23, 0xfc, 0xac, 0x95, 0x46, 0x22, 0x9d, 0x4c, 0x2e, 0xf3, 0xc5,
                0xd7, 0x79, 0xea, 0xf6, 0x24, 0x11, 0xfa, 0x4c, 0xa3, 0xa0, 0x6d, 0x60, 0xc4, 0x17,
                0xf2, 0x5b, 0x3e, 0x30,
            ],
            Bitcoin::Bip0340Verify => [
                0xaf, 0x92, 0x4c, 0xbe, 0xc9, 0x41, 0xfd, 0x9b, 0x9e, 0x82, 0x93, 0xef, 0x90, 0x3e,
                0x23, 0x7b, 0x01, 0xdb, 0xcb, 0x34, 0x2b, 0x96, 0x10, 0xbe, 0xf9, 0x5f, 0x59, 0xb8,
                0x13, 0x6e, 0xf8, 0x04,
            ],
            Bitcoin::Ch16 => [
                0x72, 0x3b, 0x96, 0x0c, 0x67, 0x34, 0xa9, 0x88, 0x32, 0x29, 0x88, 0x63, 0x28, 0x48,
                0x5d, 0xb7, 0x18, 0x9d, 0x0b, 0x3c, 0xec, 0x24, 0xc8, 0xa0, 0xb2, 0x88, 0xa4, 0x59,
                0x78, 0x67, 0xbb, 0x9c,
            ],
            Bitcoin::Ch32 => [
                0xf1, 0x47, 0x82, 0xaf, 0x48, 0xdb, 0x5e, 0xe8, 0x42, 0x4d, 0xf8, 0x4d, 0xb5, 0x5e,
                0xcd, 0x83, 0x42, 0x0e, 0x4a, 0xde, 0x1e, 0x43, 0x11, 0x9a, 0xcb, 0x0a, 0x06, 0x5f,
                0x86, 0x67, 0x50, 0x7d,
            ],
            Bitcoin::Ch64 => [
                0xb1, 0x80, 0x0e, 0xac, 0x70, 0x03, 0x8b, 0xdc, 0x3e, 0xe9, 0xdd, 0x14, 0x75, 0xc1,
                0x1f, 0x5a, 0x3b, 0x40, 0xb3, 0xc4, 0x0a, 0xd6, 0xc8, 0xe6, 0x67, 0x48, 0x77, 0x43,
                0x09, 0xc2, 0xc2, 0xac,
            ],
            Bitcoin::Ch8 => [
                0x5b, 0x0b, 0x12, 0x35, 0x72, 0x0b, 0xbf, 0x0b, 0x66, 0x6f, 0x35, 0xe6, 0x78, 0x3a,
                0x6a, 0xb6, 0xb7, 0x84, 0x8c, 0x9d, 0xa4, 0x79, 0x44, 0xbc, 0x91, 0x4f, 0x05, 0x60,
                0x38, 0x95, 0x1c, 0xf8,
            ],
            Bitcoin::CheckLockDistance => [
                0x1f, 0x8a, 0x4c, 0x17, 0x38, 0x71, 0x22, 0x16, 0x36, 0xb3, 0x3e, 0xf9, 0xf4, 0x9a,
                0x72, 0x0a, 0xf6, 0x41, 0x04, 0xff, 0xb7, 0x6c, 0x5c, 0x5d, 0xbb, 0x8c, 0xd7, 0xcb,
                0x4d, 0xbc, 0x53, 0x97,
            ],
            Bitcoin::CheckLockDuration => [
                0x20, 0xe2, 0x66, 0x5b, 0xac, 0x79, 0x6c, 0xa4, 0xc4, 0xa5, 0x5c, 0x02, 0x3e, 0x73,
                0x2d, 0xbf, 0x2c, 0xb3, 0x1d, 0x4d, 0xde, 0x49, 0x97, 0xd5, 0x97, 0xf8, 0x6b, 0xa7,
                0x96, 0xbf, 0x3c, 0x70,
            ],
            Bitcoin::CheckLockHeight => [
                0x45, 0x25, 0x39, 0xe0, 0xf0, 0x9c, 0x7b, 0x5a, 0x3f, 0xa2, 0x95, 0xed, 0xff, 0xa7,
                0xd5, 0x75, 0x72, 0x7e, 0xb7, 0xc1, 0xb2, 0x7d, 0x6f, 0x87, 0xc8, 0xc7, 0xa0, 0x5c,
                0x2f, 0x3b, 0x5d, 0x6c,
            ],
            Bitcoin::CheckLockTime => [
                0x0b, 0x11, 0x12, 0x30, 0x8d, 0x66, 0x39, 0x93, 0xf5, 0xad, 0x6f, 0x30, 0xd6, 0x74,
                0x3f, 0x86, 0x82, 0x2c, 0x3f, 0x69, 0xeb, 0x10, 0xd5, 0xb7, 0x03, 0x52, 0x35, 0xfc,
                0xd2, 0x53, 0x15, 0xe4,
            ],
            Bitcoin::CheckSigVerify => [
                0x29, 0x74, 0x59, 0xd8, 0x37, 0x3f, 0x87, 0x27, 0xda, 0x9f, 0x92, 0xda, 0xea, 0xb7,
                0x19, 0x6b, 0xed, 0x0b, 0xd5, 0x23, 0x64, 0x40, 0x55, 0x21, 0x44, 0x55, 0x72, 0x3a,
                0xba, 0xc0, 0xb0, 0x85,
            ],
            Bitcoin::Complement16 => [
                0x9d, 0x56, 0x15, 0xf9, 0x07, 0x22, 0xd4, 0xae, 0xfb, 0xd8, 0xdd, 0x09, 0xc6, 0x82,
                0x88, 0x43, 0x68, 0xa8, 0x8f, 0x47, 0x11, 0xf6, 0x2c, 0xfe, 0x02, 0x9c, 0x9f, 0x07,
                0x29, 0xa2, 0x73, 0x44,
            ],
            Bitcoin::Complement32 => [
                0x43, 0x60, 0xc5, 0xd2, 0xa4, 0x57, 0xd4, 0x43, 0x21, 0x9e, 0x02, 0x67, 0x74, 0xe6,
                0x8d, 0xee, 0x23, 0xef, 0x6e, 0xc7, 0xa9, 0x13, 0x72, 0x18, 0x6c, 0x9d, 0x46, 0x03,
                0x94, 0xc1, 0x51, 0x9b,
            ],
            Bitcoin::Complement64 => [
                0x24, 0x13, 0x58, 0x82, 0xf7, 0x56, 0x03, 0xd0, 0x69, 0x81, 0x34, 0x90, 0x2e, 0xb5,
                0x0e, 0x14, 0xa9, 0xa4, 0xa7, 0x9f, 0x48, 0xe7, 0x77, 0xa6, 0x97, 0xfc, 0x02, 0xd6,
                0x4c, 0x15, 0x65, 0x89,
            ],
            Bitcoin::Complement8 => [
                0x85, 0x8b, 0x61, 0xb0, 0x7e, 0x3d, 0x4e, 0xf1, 0x43, 0x4e, 0xc4, 0xf4, 0xae, 0xc3,
                0x0d, 0x29, 0x59, 0x9b, 0x8a, 0xc3, 0x83, 0x70, 0xe5, 0x8a, 0xfa, 0x47, 0x7b, 0x1e,
                0xd3, 0x9d, 0x6b, 0x47,
            ],
            Bitcoin::CurrentAnnexHash => [
                0x07, 0x25, 0xd0, 0xe0, 0xc8, 0xd8, 0xb0, 0xeb, 0xad, 0xce, 0x99, 0x69, 0xf2, 0xc5,
                0x9d, 0xcb, 0xf9, 0x32, 0x01, 0x7f, 0xa7, 0x2d, 0xd4, 0xae, 0x35, 0x2f, 0x61, 0xa0,
                0x2f, 0xe7, 0x3c, 0x52,
            ],
            Bitcoin::CurrentIndex => [
                0x19, 0xdb, 0x0f, 0xcc, 0x0c, 0x0f, 0x07, 0x58, 0x72, 0x5a, 0x61, 0x25, 0x79, 0x04,
                0x9d, 0x84, 0xad, 0x8a, 0x29, 0x9e, 0xab, 0x96, 0x7f, 0xf8, 0x12, 0x06, 0xc9, 0x1e,
                0x1d, 0x8b, 0xb4, 0x58,
            ],
            Bitcoin::CurrentPrevOutpoint => [
                0x17, 0x3a, 0x6c, 0x07, 0xcf, 0x52, 0x30, 0x1f, 0x7a, 0x92, 0x5d, 0x61, 0x23, 0x2a,
                0xc6, 0x4a, 0x47, 0x35, 0xa4, 0xce, 0x27, 0x85, 0xe5, 0xfe, 0xcd, 0x9b, 0x67, 0xa4,
                0x52, 0x7e, 0xd1, 0x2f,
            ],
            Bitcoin::CurrentScriptSigHash => [
                0x6e, 0x71, 0xc9, 0x38, 0x47, 0x91, 0x07, 0x15, 0x7c, 0x54, 0x10, 0xa9, 0x92, 0x16,
                0xbb, 0x0d, 0xd2, 0xb7, 0xb4, 0x4f, 0x5a, 0x0d, 0xbe, 0x59, 0x75, 0xb2, 0x05, 0xc4,
                0x0e, 0x56, 0x94, 0x3f,
            ],
            Bitcoin::CurrentSequence => [
                0xe4, 0x98, 0x4e, 0x87, 0xc0, 0x93, 0x1a, 0x61, 0x8f, 0x46, 0x25, 0x9b, 0x3a, 0x07,
                0x9c, 0xd5, 0x89, 0x5a, 0x3e, 0xfe, 0x4f, 0x92, 0xeb, 0x58, 0xb0, 0x83, 0xb4, 0xb2,
                0x1a, 0x57, 0xd6, 0x88,
            ],
            Bitcoin::CurrentValue => [
                0x73, 0xd9, 0x3f, 0xa2, 0x86, 0x03, 0xb9, 0xb9, 0xc0, 0x6d, 0x79, 0x3a, 0x9d, 0xc3,
                0x84, 0x16, 0x52, 0x65, 0xb6, 0xb3, 0xbc, 0x71, 0xa5, 0x9b, 0x07, 0xce, 0xba, 0x8c,
                0x97, 0x88, 0x27, 0x74,
            ],
            Bitcoin::Decompress => [
                0x59, 0x85, 0xb0, 0x05, 0xf2, 0x01, 0x34, 0x2d, 0x90, 0x97, 0x49, 0x27, 0x30, 0x25,
                0x7d, 0x68, 0x3e, 0xe5, 0x4f, 0x0d, 0x6b, 0xfb, 0x68, 0xfa, 0xe1, 0x82, 0x1d, 0x6a,
                0x01, 0xfe, 0xa2, 0x8c,
            ],
            Bitcoin::Decrement16 => [
                0x88, 0x73, 0xfb, 0x6a, 0x44, 0x7d, 0xe2, 0x86, 0xcd, 0x6d, 0x64, 0xb4, 0x92, 0xd8,
                0xa1, 0xd3, 0xd7, 0xcd, 0xdf, 0xbb, 0xe6, 0x75, 0xbf, 0xf5, 0xc5, 0x6e, 0xba, 0x40,
                0xf6, 0x90, 0xdb, 0xd3,
            ],
            Bitcoin::Decrement32 => [
                0xca, 0x2b, 0xc9, 0xbd, 0x09, 0xc3, 0x61, 0x30, 0xc5, 0x2e, 0x4c, 0xfa, 0x7d, 0xf6,
                0xd1, 0x08, 0x30, 0x61, 0x66, 0x04, 0x19, 0x16, 0x3d, 0x0a, 0x96, 0xb6, 0x69, 0xa0,
                0x73, 0x96, 0x6a, 0xdd,
            ],
            Bitcoin::Decrement64 => [
                0xdb, 0xc8, 0xe7, 0x57, 0xbc, 0x06, 0x96, 0x24, 0xb0, 0x27, 0xa9, 0xf7, 0x6f, 0x0a,
                0xe7, 0x9f, 0xa2, 0x0e, 0x48, 0xb8, 0x91, 0x0f, 0x7a, 0xb4, 0xa2, 0xb1, 0x62, 0x27,
                0xd2, 0xb4, 0xcc, 0xac,
            ],
            Bitcoin::Decrement8 => [
                0xd6, 0xec, 0x5f, 0xba, 0x81, 0xdc, 0xbd, 0xd4, 0x4a, 0x1b, 0x3a, 0xb1, 0x94, 0xad,
                0xd7, 0xf5, 0x42, 0xb1, 0x70, 0xca, 0xde, 0xb4, 0x65, 0xdd, 0x2f, 0x31, 0x92, 0x84,
                0x58, 0x1a, 0x76, 0xd6,
            ],
            Bitcoin::DivMod16 => [
                0x66, 0xee, 0x6d, 0x63, 0x76, 0x96, 0x76, 0x0e, 0xf0, 0x90, 0x3a, 0x4e, 0xe9, 0xda,
                0x77, 0x15, 0xcb, 0xf9, 0x04, 0xaa, 0x9f, 0x59, 0xd8, 0x4e, 0xcc, 0xb3, 0x5f, 0x90,
                0x47, 0x5d, 0x37, 0xbe,
            ],
            Bitcoin::DivMod32 => [
                0x6c, 0x6a, 0xf7, 0x62, 0x0b, 0x1d, 0x71, 0x8b, 0x1e, 0x09, 0x13, 0x30, 0x72, 0xb2,
                0xf2, 0x04, 0x35, 0x8d, 0xbe, 0x58, 0x94, 0xd3, 0xa2, 0xc6, 0x18, 0xc2, 0x59, 0x3e,
                0x62, 0xf0, 0x06, 0xcc,
            ],
            Bitcoin::DivMod64 => [
                0x24, 0x39, 0x58, 0x0d, 0xda, 0x93, 0x4d, 0x44, 0x98, 0xfc, 0x41, 0x77, 0x4f, 0x29,
                0x4d, 0xe5, 0x93, 0xf4, 0x4c, 0x63, 0x2f, 0x84, 0xe7, 0x21, 0xa8, 0xd5, 0xbb, 0xde,
                0x34, 0x5c, 0xce, 0x40,
            ],
            Bitcoin::DivMod8 => [
                0x06, 0xfb, 0x2a, 0x19, 0x7c, 0x16, 0x1d, 0x0a, 0x77, 0xfc, 0x72, 0x6d, 0x3b, 0xaa,
                0x85, 0x7e, 0x27, 0xf8, 0x69, 0x1b, 0xcc, 0xe2, 0xb4, 0x73, 0xd4, 0xd1, 0x40, 0x8c,
                0x17, 0xe0, 0x32, 0x1d,
            ],
            Bitcoin::Divide16 => [
                0x70, 0x1e, 0xf0, 0x5d, 0x05, 0xfe, 0x59, 0x43, 0x54, 0xb2, 0xef, 0x79, 0x6f, 0x4c,
                0xa4, 0x52, 0x76, 0xda, 0xac, 0x15, 0x25, 0x92, 0x76, 0x3e, 0x2a, 0x1f, 0x6e, 0x19,
                0x67, 0x25, 0x76, 0xbc,
            ],
            Bitcoin::Divide32 => [
                0xf8, 0x83, 0xb1, 0xc6, 0x4d, 0x04, 0x40, 0x31, 0xd9, 0x9e, 0x04, 0x67, 0x42, 0x54,
                0xfe, 0x2b, 0xb3, 0x44, 0xbc, 0xdd, 0x54, 0x38, 0xd6, 0xea, 0x01, 0x45, 0x8d, 0x9e,
                0x24, 0x97, 0x70, 0x2a,
            ],
            Bitcoin::Divide64 => [
                0xaa, 0x84, 0x4c, 0xfe, 0x9a, 0x83, 0xc5, 0x26, 0xd5, 0xfb, 0x05, 0x82, 0x0d, 0x1a,
                0x6f, 0x95, 0x3e, 0xb5, 0x0c, 0x85, 0x6c, 0xda, 0x50, 0x7d, 0xb9, 0xbe, 0x5b, 0x7c,
                0x90, 0xab, 0x2d, 0x14,
            ],
            Bitcoin::Divide8 => [
                0xbb, 0x39, 0x21, 0x8f, 0x0b, 0x2d, 0x69, 0x30, 0xa6, 0xe4, 0x17, 0x66, 0xfd, 0x52,
                0x18, 0xfa, 0xd1, 0x81, 0x0d, 0x17, 0x6a, 0xba, 0x80, 0x1a, 0x26, 0x38, 0x0d, 0x35,
                0x1a, 0x2d, 0x79, 0xb1,
            ],
            Bitcoin::Divides16 => [
                0x0e, 0x98, 0xa5, 0x3a, 0x62, 0x66, 0xf4, 0xc6, 0x0b, 0xdc, 0xaf, 0x62, 0x0d, 0xbc,
                0xa7, 0x6d, 0xb3, 0xcc, 0x1b, 0x59, 0x8a, 0x9c, 0x66, 0x7a, 0xd1, 0xda, 0x86, 0xb5,
                0xc9, 0xc0, 0xe9, 0x12,
            ],
            Bitcoin::Divides32 => [
                0x6e, 0xd2, 0x1a, 0xbf, 0x02, 0x2c, 0x2b, 0xa0, 0x20, 0xe9, 0x1f, 0x0d, 0x09, 0x59,
                0x3b, 0xa5, 0x5f, 0x74, 0x26, 0x26, 0x29, 0x14, 0xe9, 0x55, 0xe1, 0x79, 0x1a, 0x58,
                0x3b, 0x31, 0xc0, 0x7d,
            ],
            Bitcoin::Divides64 => [
                0x39, 0x71, 0xea, 0x23, 0x01, 0xba, 0xbb, 0x7b, 0xb5, 0x4f, 0x5f, 0x48, 0x33, 0xc6,
                0x3c, 0xaa, 0x52, 0x79, 0xd4, 0xc2, 0xb6, 0x54, 0x59, 0x2f, 0xbb, 0x50, 0x6e, 0xb2,
                0x03, 0xb0, 0x06, 0x15,
            ],
            Bitcoin::Divides8 => [
                0x77, 0x63, 0x5c, 0x01, 0x95, 0x71, 0x9b, 0x99, 0x0f, 0xeb, 0x14, 0x31, 0xb1, 0x24,
                0x4e, 0xb7, 0x55, 0xae, 0xe9, 0xba, 0xce, 0xe4, 0xb2, 0x0c, 0x12, 0xbe, 0x8e, 0x23,
                0x9d, 0x12, 0xf7, 0xdc,
            ],
            Bitcoin::Eq16 => [
                0x08, 0x1d, 0x4c, 0x58, 0x0a, 0x51, 0x56, 0x85, 0xa7, 0x00, 0xcb, 0x47, 0xa5, 0x43,
                0x33, 0x04, 0x49, 0x0a, 0xdd, 0x10, 0x1c, 0xbc, 0x07, 0xc3, 0xc4, 0x6e, 0xfb, 0x19,
                0x6b, 0x75, 0xfe, 0x55,
            ],
            Bitcoin::Eq256 => [
                0x56, 0x83, 0x53, 0x14, 0x63, 0x05, 0x0b, 0x4f, 0x99, 0x26, 0xd0, 0xe8, 0xff, 0x64,
                0x26, 0x3a, 0x15, 0xce, 0xb1, 0xce, 0xa6, 0x54, 0x12, 0x63, 0x7c, 0x23, 0x5f, 0x46,
                0xd7, 0xbe, 0x66, 0x05,
            ],
            Bitcoin::Eq32 => [
                0x73, 0x87, 0xd2, 0x79, 0x8a, 0x02, 0x10, 0x7e, 0x2c, 0x57, 0x36, 0x71, 0x27, 0xf3,
                0xf3, 0x6f, 0xc9, 0xf1, 0x97, 0x0d, 0xaf, 0xba, 0x80, 0xf9, 0x34, 0x0c, 0x16, 0x8b,
                0x50, 0xf7, 0xa4, 0xe2,
            ],
            Bitcoin::Eq64 => [
                0x05, 0x72, 0xe1, 0x6b, 0xab, 0x59, 0xd8, 0x47, 0x5f, 0xda, 0x6b, 0xfc, 0xbd, 0x49,
                0x6f, 0xe7, 0x53, 0x41, 0x07, 0xf6, 0x29, 0x1d, 0xd7, 0xee, 0xdb, 0xfb, 0xf2, 0x8b,
                0x76, 0x0a, 0xa4, 0x1a,
            ],
            Bitcoin::Eq8 => [
                0xd4, 0x25, 0x79, 0xe4, 0xe7, 0xaf, 0x6d, 0xaa, 0x39, 0xa5, 0x4e, 0x54, 0x27, 0x47,
                0xd4, 0x19, 0x5c, 0x2a, 0xb4, 0x8c, 0x1f, 0xdd, 0x7b, 0x87, 0x32, 0xe7, 0xa5, 0x2f,
                0xed, 0x40, 0x75, 0x99,
            ],
            Bitcoin::FeAdd => [
                0xdf, 0x5c, 0x03, 0x4b, 0x64, 0x59, 0xde, 0xd2, 0xf2, 0x07, 0xb5, 0xe4, 0x0f, 0x02,
                0xe5, 0x08, 0x76, 0xee, 0x6b, 0x25, 0x0c, 0xdd, 0x8c, 0x94, 0x9f, 0xbb, 0x78, 0x3b,
                0x5e, 0x90, 0x44, 0xda,
            ],
            Bitcoin::FeInvert => [
                0xc1, 0x22, 0x09, 0x8f, 0x95, 0x59, 0x4b, 0x72, 0x1d, 0x3a, 0x51, 0xa3, 0x1d, 0xc1,
                0xb2, 0xc4, 0x0d, 0xec, 0xcb, 0x9b, 0x3c, 0xdc, 0x9a, 0xb9, 0x0e, 0x56, 0x1a, 0xc2,
                0x13, 0x41, 0xbc, 0xe2,
            ],
            Bitcoin::FeIsOdd => [
                0xaf, 0xa5, 0x6c, 0x6b, 0x60, 0xc5, 0x2a, 0x39, 0xe5, 0xa1, 0x61, 0x0e, 0xfb, 0x8e,
                0x1a, 0xe5, 0x6f, 0x72, 0x03, 0x72, 0x01, 0xca, 0xc6, 0x04, 0x37, 0xb6, 0x75, 0x8a,
                0x74, 0x46, 0x9b, 0x2a,
            ],
            Bitcoin::FeIsZero => [
                0x51, 0x1a, 0xc5, 0xb8, 0x35, 0x9c, 0x20, 0x02, 0x08, 0x0a, 0x16, 0xa1, 0x18, 0x40,
                0xab, 0x1a, 0xe1, 0x5b, 0x60, 0x15, 0x73, 0x72, 0x67, 0x7b, 0x39, 0xa9, 0x2f, 0xd7,
                0xcf, 0x3e, 0x8e, 0xb3,
            ],
            Bitcoin::FeMultiply => [
                0x4d, 0xb0, 0xb0, 0xf2, 0xa2, 0xa8, 0x21, 0x38, 0x7a, 0x7d, 0xab, 0x9c, 0x5c, 0x0c,
                0xe8, 0xeb, 0xde, 0x14, 0xd0, 0x39, 0x25, 0xb5, 0x6d, 0x7f, 0x30, 0x9a, 0x5e, 0x2d,
                0x09, 0x9b, 0xf9, 0xab,
            ],
            Bitcoin::FeMultiplyBeta => [
                0x45, 0x32, 0xc7, 0x30, 0x5f, 0x04, 0xbb, 0x13, 0x0f, 0xef, 0x0c, 0x4e, 0x16, 0x7b,
                0x9a, 0xc0, 0xcc, 0x07, 0xbc, 0xfc, 0x66, 0xdd, 0x2d, 0x0c, 0x67, 0x8e, 0xd9, 0x13,
                0xae, 0xe2, 0x1d, 0x64,
            ],
            Bitcoin::FeNegate => [
                0x3b, 0x1c, 0x7d, 0x40, 0x36, 0x1c, 0x5b, 0x03, 0x73, 0x77, 0x1e, 0xab, 0xb0, 0x66,
                0x28, 0x9f, 0x29, 0x1f, 0x76, 0x65, 0x25, 0xeb, 0x25, 0xab, 0x42, 0xd1, 0x8d, 0xfa,
                0x21, 0x96, 0xda, 0x40,
            ],
            Bitcoin::FeNormalize => [
                0xe0, 0x89, 0xae, 0x03, 0x97, 0x27, 0x38, 0xe5, 0x8c, 0xad, 0x4f, 0x74, 0x26, 0xe2,
                0x63, 0xfa, 0x5e, 0x2c, 0x6c, 0x4b, 0x58, 0x69, 0xef, 0x96, 0x9b, 0xec, 0x86, 0xfc,
                0xb7, 0xad, 0x32, 0x05,
            ],
            Bitcoin::FeSquare => [
                0x1f, 0x1f, 0x57, 0xf8, 0x8b, 0x28, 0x73, 0xf4, 0xdf, 0x41, 0x63, 0xa6, 0x8f, 0x53,
                0x98, 0xb1, 0x68, 0x2a, 0x91, 0x1b, 0xeb, 0x4d, 0x6d, 0x24, 0xe8, 0xca, 0x27, 0x1e,
                0x35, 0xba, 0xe8, 0xdb,
            ],
            Bitcoin::FeSquareRoot => [
                0xb0, 0xea, 0x63, 0x8c, 0xf2, 0x50, 0xe2, 0xa2, 0x8d, 0xcf, 0xc8, 0xfc, 0x04, 0x09,
                0x40, 0x13, 0x10, 0x21, 0x81, 0xf8, 0x16, 0xfe, 0xfb, 0xc6, 0xcb, 0x45, 0x1b, 0xe7,
                0x1e, 0xe8, 0xa8, 0x05,
            ],
            Bitcoin::FullAdd16 => [
                0x67, 0xc5, 0x7c, 0x82, 0x46, 0x18, 0x60, 0xc0, 0x22, 0x3a, 0x42, 0x7d, 0xde, 0x99,
                0xe5, 0x12, 0x75, 0x3b, 0x0e, 0xc5, 0x2a, 0x05, 0xdc, 0xbe, 0x71, 0x8e, 0xf3, 0x0a,
                0x26, 0x84, 0xb6, 0x81,
            ],
            Bitcoin::FullAdd32 => [
                0x73, 0x9f, 0x5b, 0x1e, 0x9b, 0x40, 0x8b, 0x36, 0x3c, 0xda, 0xd6, 0xbf, 0x00, 0xa8,
                0xbf, 0xf5, 0x22, 0xb3, 0xd5, 0x38, 0x02, 0xea, 0xad, 0xd8, 0x94, 0xa0, 0x10, 0x7d,
                0x20, 0x3a, 0x46, 0x00,
            ],
            Bitcoin::FullAdd64 => [
                0x70, 0xe1, 0xee, 0xa5, 0x3b, 0xc9, 0xb0, 0x1b, 0x06, 0x76, 0xb5, 0xea, 0x97, 0x36,
                0x74, 0xd6, 0xf3, 0xc3, 0x56, 0x74, 0x32, 0xf2, 0x64, 0xbc, 0x60, 0xb7, 0xae, 0x1f,
                0xf6, 0x9a, 0x40, 0x2a,
            ],
            Bitcoin::FullAdd8 => [
                0x37, 0xff, 0x9d, 0x08, 0x4d, 0xd0, 0x51, 0x7f, 0xb3, 0x89, 0x7d, 0xe0, 0x86, 0x38,
                0x1a, 0x9c, 0x2c, 0x72, 0x44, 0x0e, 0xe1, 0xba, 0xc3, 0x8d, 0xb7, 0xf4, 0xf7, 0x4a,
                0x50, 0x1d, 0x54, 0xdd,
            ],
            Bitcoin::FullDecrement16 => [
                0x61, 0xe2, 0xe8, 0x5a, 0xb8, 0xc7, 0xe4, 0xc2, 0xe9, 0xdc, 0x6e, 0xb2, 0xf0, 0x60,
                0x74, 0x9a, 0x03, 0x35, 0x41, 0x2f, 0x9f, 0x3a, 0xc8, 0xfd, 0x68, 0xf2, 0x56, 0x15,
                0xff, 0x06, 0x7e, 0x61,
            ],
            Bitcoin::FullDecrement32 => [
                0x8b, 0xe3, 0x11, 0x0b, 0xda, 0xe8, 0xd6, 0x5f, 0x08, 0x33, 0x19, 0x95, 0xf5, 0x84,
                0xbf, 0x36, 0x02, 0x36, 0xa1, 0x4c, 0x61, 0xad, 0x2f, 0xa6, 0xe7, 0xae, 0xac, 0x60,
                0x9b, 0xd4, 0xdb, 0xfa,
            ],
            Bitcoin::FullDecrement64 => [
                0xd0, 0x03, 0x87, 0x12, 0x19, 0x99, 0x14, 0x9c, 0x3a, 0x7b, 0xd2, 0x56, 0xce, 0x8b,
                0xd0, 0x2d, 0x71, 0x9f, 0x3d, 0x5c, 0x58, 0xf1, 0x0a, 0xb6, 0x75, 0x7c, 0xc0, 0x23,
                0xd5, 0x5d, 0xbe, 0x1b,
            ],
            Bitcoin::FullDecrement8 => [
                0xe8, 0xcc, 0x36, 0x06, 0xb1, 0xdd, 0x7f, 0xcf, 0x64, 0xc9, 0x91, 0xb0, 0xce, 0xb9,
                0xc6, 0x14, 0x6c, 0xcc, 0xd0, 0x89, 0x49, 0x89, 0xd8, 0xd9, 0xab, 0x97, 0xd2, 0x74,
                0x3e, 0xa1, 0xa5, 0xac,
            ],
            Bitcoin::FullIncrement16 => [
                0xce, 0xaf, 0x13, 0xe7, 0x8e, 0x3a, 0x30, 0x75, 0xd1, 0x17, 0x48, 0x0c, 0x7b, 0x72,
                0xc2, 0xba, 0xd4, 0x54, 0x8c, 0xcc, 0xfa, 0xf4, 0x8e, 0xa1, 0xeb, 0x67, 0x7f, 0x78,
                0x0e, 0x5c, 0x00, 0x30,
            ],
            Bitcoin::FullIncrement32 => [
                0x7e, 0x83, 0x12, 0x88, 0x87, 0x14, 0x66, 0x43, 0x44, 0x5a, 0xf4, 0xca, 0xbb, 0x1d,
                0xa5, 0x0e, 0x82, 0xda, 0xeb, 0x34, 0xb7, 0xd9, 0xa2, 0x42, 0xcd, 0x00, 0xde, 0x5f,
                0xbb, 0xea, 0x51, 0x82,
            ],
            Bitcoin::FullIncrement64 => [
                0x93, 0xac, 0xb5, 0x83, 0x1a, 0x59, 0xb2, 0x8a, 0xe4, 0x38, 0x87, 0x6e, 0x31, 0x02,
                0xf9, 0x19, 0x42, 0x7b, 0x82, 0x72, 0x63, 0x0a, 0x8e, 0xd7, 0x85, 0x92, 0xef, 0x54,
                0x3c, 0xfb, 0x65, 0xf6,
            ],
            Bitcoin::FullIncrement8 => [
                0x92, 0x1a, 0xaa, 0xa8, 0xca, 0xc8, 0x20, 0x63, 0xfb, 0xfa, 0xc0, 0x51, 0x17, 0xe4,
                0xa9, 0x3e, 0xb2, 0xf0, 0xb9, 0x2d, 0x83, 0x56, 0x21, 0x99, 0xeb, 0xdb, 0x8a, 0xfc,
                0x16, 0xb1, 0xd5, 0x2c,
            ],
            Bitcoin::FullMultiply16 => [
                0x15, 0x97, 0x38, 0x6d, 0xa7, 0xf0, 0xd5, 0x56, 0x99, 0x66, 0xd1, 0x71, 0x76, 0xea,
                0xe3, 0x78, 0x1b, 0xec, 0x8f, 0xb4, 0xfd, 0xe5, 0xc8, 0xc7, 0x12, 0xda, 0xea, 0xa8,
                0x6b, 0xd8, 0x3f, 0x37,
            ],
            Bitcoin::FullMultiply32 => [
                0xe8, 0xb8, 0x81, 0xa1, 0x26, 0xe2, 0xc0, 0x4e, 0xbd, 0x8d, 0xc7, 0x8d, 0x8e, 0x24,
                0xfd, 0x92, 0xcc, 0x82, 0xba, 0x0c, 0x41, 0xe1, 0xa4, 0x03, 0xe6, 0x43, 0x13, 0x34,
                0xd1, 0x3c, 0x4e, 0xc2,
            ],
            Bitcoin::FullMultiply64 => [
                0xdc, 0xcc, 0x7e, 0x3c, 0x39, 0x7c, 0x15, 0x0a, 0xfd, 0xf3, 0x83, 0xbe, 0xd8, 0x9e,
                0x95, 0xe5, 0x05, 0x94, 0xb2, 0x85, 0xb8, 0xc7, 0xd6, 0x24, 0xd1, 0xb9, 0xe0, 0x59,
                0xe9, 0xf7, 0x23, 0x0a,
            ],
            Bitcoin::FullMultiply8 => [
                0x20, 0x18, 0x33, 0x05, 0x30, 0x45, 0x46, 0x19, 0xc6, 0x4f, 0x5a, 0x10, 0x9e, 0x7b,
                0x3c, 0xce, 0x81, 0x79, 0x79, 0xdb, 0x53, 0x52, 0x18, 0x9e, 0x31, 0x97, 0xb0, 0x51,
                0x99, 0xc7, 0x4d, 0xe9,
            ],
            Bitcoin::FullSubtract16 => [
                0x09, 0x19, 0xce, 0x69, 0x18, 0x98, 0xd2, 0x7c, 0x6f, 0xc4, 0x4c, 0x13, 0x70, 0x64,
                0x52, 0x4e, 0x41, 0x31, 0x46, 0xd1, 0xce, 0x20, 0x17, 0x32, 0xc9, 0x75, 0xb0, 0xd5,
                0x47, 0xef, 0xfa, 0x7c,
            ],
            Bitcoin::FullSubtract32 => [
                0x81, 0x0b, 0xd2, 0x82, 0x67, 0x28, 0x23, 0x55, 0xee, 0x69, 0x48, 0x20, 0x71, 0x3e,
                0x68, 0xd0, 0x67, 0xbb, 0xd8, 0xb8, 0x1e, 0xd9, 0x13, 0x8c, 0x33, 0xe5, 0x16, 0x49,
                0x73, 0xa5, 0x62, 0x84,
            ],
            Bitcoin::FullSubtract64 => [
                0xc8, 0x67, 0x88, 0x5f, 0xb9, 0x15, 0x4f, 0x6e, 0x4a, 0x68, 0xb8, 0x16, 0x4e, 0xca,
                0xe1, 0xd6, 0x9f, 0xeb, 0x4f, 0x4d, 0xc6, 0xc2, 0xe1, 0xe9, 0x72, 0xaf, 0x8a, 0xf8,
                0xc6, 0x33, 0x47, 0xf1,
            ],
            Bitcoin::FullSubtract8 => [
                0xb9, 0xf1, 0xb9, 0xb8, 0xd1, 0x0f, 0xab, 0xbb, 0xb7, 0xdf, 0x46, 0xc8, 0x75, 0x86,
                0x2f, 0x7e, 0x7e, 0x70, 0x14, 0x08, 0x85, 0x60, 0xb0, 0xda, 0x45, 0x48, 0x0a, 0x6c,
                0x7c, 0x12, 0x59, 0xa2,
            ],
            Bitcoin::GeIsOnCurve => [
                0x36, 0xd0, 0x5a, 0x72, 0x9a, 0x79, 0xb0, 0x37, 0xb1, 0x0b, 0x3a, 0xb2, 0xb2, 0xc2,
                0xbf, 0xea, 0x03, 0xa0, 0x1a, 0x18, 0xe0, 0xea, 0xf6, 0x5c, 0x6f, 0x9b, 0xc7, 0x45,
                0xc7, 0xc2, 0xe6, 0xfa,
            ],
            Bitcoin::GeNegate => [
                0x52, 0x01, 0xd4, 0x5c, 0xc1, 0xde, 0xb3, 0xe2, 0xb9, 0x49, 0xfe, 0x66, 0x0a, 0x07,
                0xc5, 0xce, 0x69, 0x44, 0x3d, 0xbe, 0xc6, 0x7c, 0xc7, 0x6b, 0x24, 0x25, 0xff, 0x32,
                0xe8, 0x52, 0xe5, 0xb4,
            ],
            Bitcoin::GejAdd => [
                0x28, 0x54, 0x85, 0xc4, 0x70, 0x84, 0x49, 0x25, 0x10, 0x37, 0x3d, 0xf4, 0x3d, 0xf5,
                0x34, 0x07, 0xac, 0xec, 0x8f, 0xb1, 0xbd, 0x01, 0x03, 0x80, 0x89, 0x7b, 0x51, 0x7c,
                0x39, 0xcd, 0x63, 0x19,
            ],
            Bitcoin::GejDouble => [
                0x71, 0x07, 0x74, 0x58, 0x57, 0x75, 0xf9, 0x1f, 0x4c, 0xe5, 0x78, 0xad, 0x8d, 0x1e,
                0x64, 0x45, 0x41, 0xe2, 0x1f, 0xc6, 0xc8, 0x10, 0xab, 0xdb, 0x3b, 0x3e, 0xd2, 0x11,
                0x5e, 0x39, 0xcd, 0xae,
            ],
            Bitcoin::GejGeAdd => [
                0x7d, 0x7f, 0x42, 0x6e, 0x42, 0x45, 0x8e, 0x45, 0x77, 0x12, 0x91, 0xcc, 0x9e, 0x60,
                0x7e, 0x67, 0x26, 0x7a, 0x38, 0x85, 0xad, 0xbe, 0xbd, 0xc3, 0x69, 0xdf, 0x59, 0x66,
                0x32, 0x20, 0xbe, 0xfb,
            ],
            Bitcoin::GejGeAddEx => [
                0xcd, 0xda, 0xe7, 0x8d, 0x33, 0xa2, 0x21, 0x28, 0xbc, 0x2f, 0x72, 0xa6, 0x02, 0xe0,
                0x06, 0x6f, 0x63, 0xfe, 0x18, 0x62, 0x57, 0xea, 0x34, 0x8c, 0x2b, 0xb1, 0xf7, 0xe9,
                0xbf, 0x9b, 0x0d, 0x73,
            ],
            Bitcoin::GejInfinity => [
                0x95, 0xbd, 0x3c, 0xe9, 0x3f, 0x0c, 0x8e, 0x95, 0xaa, 0x2e, 0x60, 0xa9, 0xe8, 0x26,
                0x57, 0xfd, 0x98, 0x0f, 0x3f, 0x27, 0x78, 0x11, 0xfb, 0x6d, 0x39, 0xd1, 0xff, 0x12,
                0x1f, 0x3f, 0x8a, 0x14,
            ],
            Bitcoin::GejIsInfinity => [
                0x63, 0x58, 0xd3, 0xdd, 0xc7, 0xab, 0x52, 0xfd, 0x6a, 0xd6, 0x36, 0xad, 0xf9, 0xb9,
                0xf3, 0x7e, 0xaf, 0x79, 0x6f, 0x89, 0xb2, 0xd9, 0xbf, 0xba, 0x97, 0xab, 0xee, 0x3f,
                0x32, 0x7b, 0x30, 0xa4,
            ],
            Bitcoin::GejIsOnCurve => [
                0x6b, 0xcc, 0x65, 0xcf, 0xed, 0x04, 0x39, 0xf1, 0x11, 0xee, 0x5f, 0x5b, 0x5b, 0x47,
                0x91, 0xe1, 0xe4, 0xad, 0x7f, 0xf3, 0x69, 0x51, 0xa2, 0x33, 0x1b, 0x18, 0xf9, 0x7f,
                0x7a, 0xf9, 0x13, 0x98,
            ],
            Bitcoin::GejNegate => [
                0x0a, 0xe3, 0x66, 0x32, 0x36, 0x47, 0x83, 0x1a, 0x3f, 0x1c, 0x8e, 0x51, 0xf1, 0xf6,
                0xc1, 0x7e, 0xec, 0x93, 0xcf, 0x53, 0x95, 0xa2, 0x76, 0x44, 0x0a, 0x24, 0x6d, 0xeb,
                0xb2, 0xef, 0xc3, 0x91,
            ],
            Bitcoin::GejNormalize => [
                0xcb, 0x5a, 0x52, 0xa3, 0x24, 0x29, 0x5b, 0xda, 0xd4, 0x2d, 0x0f, 0xb0, 0x1f, 0x67,
                0xaa, 0xdf, 0x6e, 0x10, 0xe0, 0xb3, 0xd1, 0x8c, 0x93, 0x24, 0xc3, 0xa1, 0x2a, 0x05,
                0xb4, 0xfe, 0x1d, 0x64,
            ],
            Bitcoin::GejRescale => [
                0xe1, 0x42, 0x78, 0x60, 0xb5, 0x1b, 0xa7, 0xe7, 0x07, 0x2d, 0x75, 0x69, 0xc5, 0x5b,
                0x83, 0x39, 0xf8, 0x5b, 0x60, 0x22, 0x75, 0x0c, 0x6a, 0xf5, 0xaf, 0x2a, 0x72, 0xf1,
                0xb7, 0xb6, 0xba, 0xee,
            ],
            Bitcoin::GejXEquiv => [
                0xa7, 0xee, 0x48, 0xd5, 0xfb, 0x92, 0x6d, 0x41, 0xf7, 0x0b, 0xbb, 0x0f, 0x06, 0x82,
                0x13, 0x75, 0x34, 0x3f, 0x0d, 0x2f, 0x0e, 0x5b, 0x19, 0x91, 0x55, 0x39, 0xd9, 0x10,
                0x1b, 0x8d, 0x84, 0x7a,
            ],
            Bitcoin::GejYIsOdd => [
                0xa2, 0xd7, 0x2b, 0x1e, 0x83, 0x88, 0xa1, 0x75, 0x54, 0x27, 0xa7, 0xb2, 0x55, 0x67,
                0x68, 0x31, 0xca, 0x76, 0xea, 0xdd, 0xa2, 0x82, 0xf9, 0x7a, 0x34, 0x3f, 0x0e, 0xb1,
                0x55, 0xf3, 0x50, 0x46,
            ],
            Bitcoin::Generate => [
                0x0e, 0x91, 0xf4, 0x55, 0x7c, 0xb7, 0xd4, 0xc3, 0xbb, 0xf3, 0xf2, 0xd0, 0x74, 0xdd,
                0x69, 0x46, 0x42, 0x3a, 0x3b, 0x4f, 0xac, 0xb5, 0x7a, 0x00, 0xca, 0xe4, 0x3f, 0xd6,
                0xa7, 0x35, 0x2a, 0x13,
            ],
            Bitcoin::High16 => [
                0xae, 0x53, 0xd2, 0xd0, 0xaa, 0x16, 0x23, 0xb3, 0xa3, 0xc8, 0xb5, 0x53, 0x82, 0x6f,
                0xde, 0xf4, 0x3f, 0xf5, 0x9e, 0xa7, 0xc4, 0xa0, 0x18, 0x5c, 0xf3, 0x4c, 0x16, 0x45,
                0x08, 0xd8, 0xbf, 0xbc,
            ],
            Bitcoin::High32 => [
                0x64, 0xe7, 0xc5, 0x45, 0x3c, 0x3f, 0x8c, 0x63, 0x6c, 0x59, 0x61, 0xf4, 0x8a, 0x57,
                0xb0, 0xec, 0x7b, 0xde, 0xad, 0xed, 0xa4, 0xb3, 0x53, 0x11, 0x86, 0x7b, 0xde, 0x1c,
                0xf7, 0xc1, 0xc1, 0xd8,
            ],
            Bitcoin::High64 => [
                0xfd, 0xbf, 0xad, 0x11, 0x2a, 0xff, 0x8b, 0xd0, 0x21, 0xab, 0xee, 0xa8, 0x68, 0x88,
                0xdd, 0x28, 0xba, 0x90, 0xb1, 0xd6, 0x36, 0x3b, 0x39, 0x8a, 0x09, 0xdf, 0x9d, 0x5f,
                0x40, 0xe8, 0x06, 0x08,
            ],
            Bitcoin::High8 => [
                0x2d, 0x12, 0x02, 0xea, 0x63, 0x3d, 0x2d, 0x65, 0xf4, 0x5e, 0xaf, 0x80, 0xea, 0x03,
                0xe5, 0xcc, 0x3d, 0x7d, 0x9a, 0xe4, 0xa4, 0xb3, 0x71, 0x21, 0x8a, 0x01, 0x60, 0xa6,
                0x93, 0x10, 0x4f, 0x69,
            ],
            Bitcoin::Increment16 => [
                0xef, 0x9f, 0x63, 0x27, 0x4d, 0x9f, 0xc0, 0x71, 0x0a, 0xba, 0x34, 0x2f, 0xe7, 0xca,
                0x00, 0xd9, 0x12, 0xbf, 0x27, 0x71, 0xa9, 0x40, 0xbd, 0xd1, 0x27, 0x2d, 0x28, 0x9b,
                0x70, 0x43, 0x00, 0x44,
            ],
            Bitcoin::Increment32 => [
                0x42, 0x57, 0xa7, 0x52, 0xd0, 0x89, 0x5a, 0x75, 0x31, 0xa5, 0x21, 0x14, 0x15, 0x53,
                0xd3, 0x30, 0x90, 0xb2, 0x78, 0xd0, 0xbb, 0x79, 0xd1, 0x8b, 0x53, 0xcf, 0x75, 0x3c,
                0x76, 0x40, 0x2a, 0x0e,
            ],
            Bitcoin::Increment64 => [
                0xb5, 0x25, 0xe5, 0x54, 0x40, 0x19, 0x3d, 0xc3, 0xd5, 0x1d, 0x9f, 0xc6, 0xb1, 0xf3,
                0xf2, 0x91, 0xf1, 0xa2, 0x77, 0x6e, 0x99, 0xa2, 0x10, 0xc9, 0xfe, 0x33, 0x36, 0x77,
                0xd9, 0x68, 0x50, 0xf2,
            ],
            Bitcoin::Increment8 => [
                0xb5, 0x30, 0x23, 0x1a, 0x83, 0x32, 0x0a, 0x13, 0x0f, 0x41, 0x89, 0xf1, 0xef, 0xee,
                0x5a, 0xb2, 0x91, 0x41, 0xdc, 0xe4, 0xc3, 0x14, 0x91, 0x19, 0xcb, 0x21, 0xb9, 0xcb,
                0x06, 0xce, 0x6f, 0xa1,
            ],
            Bitcoin::InputAnnexHash => [
                0xf0, 0x1c, 0x26, 0x02, 0xed, 0x64, 0x02, 0xdb, 0x1b, 0xd0, 0xa5, 0xb5, 0xcc, 0x09,
                0xc9, 0x93, 0xba, 0xec, 0x66, 0x4f, 0x68, 0xa2, 0xeb, 0xa0, 0x2c, 0xc6, 0x49, 0x86,
                0x10, 0x2a, 0x15, 0x3f,
            ],
            Bitcoin::InputPrevOutpoint => [
                0x14, 0xdc, 0x8f, 0x63, 0x25, 0xce, 0x3c, 0x0c, 0x62, 0x42, 0xf6, 0x42, 0x16, 0x2f,
                0x30, 0xcf, 0xb5, 0xc8, 0x03, 0x31, 0x8b, 0xb5, 0x57, 0x5a, 0x45, 0x26, 0x54, 0x85,
                0x5b, 0x54, 0xbd, 0x16,
            ],
            Bitcoin::InputScriptSigHash => [
                0xdc, 0x78, 0x13, 0x7b, 0x8a, 0xf6, 0x8e, 0x96, 0xa3, 0x49, 0x51, 0xbe, 0x21, 0x1f,
                0x2a, 0xc2, 0x10, 0x62, 0x9a, 0xbe, 0x77, 0x00, 0x7c, 0x59, 0x94, 0xb6, 0x33, 0xe7,
                0x3d, 0xf9, 0xa2, 0x97,
            ],
            Bitcoin::InputSequence => [
                0x26, 0x65, 0x21, 0x23, 0xdb, 0x79, 0x71, 0x2f, 0x8e, 0x82, 0x18, 0xf7, 0x50, 0x5a,
                0xa2, 0x9b, 0x93, 0x1c, 0x4f, 0xe3, 0xef, 0xc2, 0x45, 0xa7, 0x57, 0xcc, 0x4d, 0xeb,
                0x0e, 0x1f, 0x81, 0x60,
            ],
            Bitcoin::InputValue => [
                0x6d, 0xd0, 0x70, 0x5e, 0x75, 0xe6, 0x72, 0x5f, 0x4b, 0x1b, 0x61, 0x3e, 0x2c, 0x49,
                0x56, 0xd1, 0xc2, 0x39, 0xd2, 0x08, 0xd2, 0x9b, 0x0b, 0x39, 0xa5, 0xe2, 0x01, 0x0e,
                0x3f, 0x3d, 0xea, 0xdd,
            ],
            Bitcoin::InternalKey => [
                0x09, 0xfc, 0x6e, 0xc6, 0x89, 0xb2, 0x60, 0x7a, 0xc2, 0xc6, 0xad, 0x37, 0xef, 0x3d,
                0x50, 0xed, 0x05, 0x35, 0x20, 0xe4, 0x1c, 0xb4, 0x2d, 0xd1, 0x62, 0x00, 0xbd, 0x4f,
                0xdf, 0x95, 0x5a, 0xa5,
            ],
            Bitcoin::IsOne16 => [
                0xec, 0xc5, 0x6a, 0x7c, 0x8d, 0xae, 0x7f, 0xb0, 0x19, 0xb9, 0xf5, 0x49, 0xf8, 0x3a,
                0x5d, 0xa1, 0x21, 0xf3, 0x74, 0xd8, 0xd4, 0x5b, 0xed, 0x50, 0x77, 0x8e, 0xa6, 0xcf,
                0xa9, 0xbb, 0x9e, 0xa0,
            ],
            Bitcoin::IsOne32 => [
                0x3f, 0x3b, 0xc4, 0x6c, 0x8c, 0x53, 0x86, 0x0c, 0x49, 0xc4, 0x70, 0x28, 0x11, 0x10,
                0x90, 0x89, 0xa2, 0xfe, 0xd0, 0xac, 0xdc, 0x3e, 0x54, 0x3b, 0x8f, 0x4c, 0x79, 0x62,
                0x65, 0x5f, 0x7f, 0x93,
            ],
            Bitcoin::IsOne64 => [
                0xe7, 0xb9, 0xc6, 0x90, 0x1f, 0x20, 0xd9, 0x8c, 0x65, 0xd3, 0xe7, 0x8a, 0xd1, 0x71,
                0x61, 0x99, 0x93, 0x90, 0x40, 0xd4, 0xe3, 0x97, 0x80, 0xb4, 0x43, 0x44, 0xab, 0x80,
                0xbb, 0x5f, 0x72, 0x3c,
            ],
            Bitcoin::IsOne8 => [
                0x6a, 0xa1, 0x40, 0xd2, 0xf0, 0xe5, 0xdd, 0x66, 0xc1, 0x21, 0x06, 0x81, 0x62, 0xef,
                0xc5, 0x79, 0xe5, 0x40, 0xf3, 0xac, 0x1c, 0x83, 0x23, 0xd1, 0xa9, 0x22, 0x6f, 0x0e,
                0xb3, 0x2f, 0x84, 0x66,
            ],
            Bitcoin::IsZero16 => [
                0x05, 0x83, 0x68, 0xc8, 0x85, 0xf9, 0x58, 0x18, 0x16, 0x76, 0xde, 0x6d, 0x4d, 0x31,
                0xca, 0xbf, 0x3f, 0x71, 0x42, 0xee, 0xa4, 0x75, 0x21, 0x28, 0x01, 0xeb, 0xe3, 0x73,
                0x79, 0x45, 0x6c, 0x2c,
            ],
            Bitcoin::IsZero32 => [
                0x3b, 0x73, 0x91, 0x81, 0x8f, 0x80, 0xcb, 0xd2, 0x2e, 0xd7, 0xe5, 0xe8, 0x52, 0x3c,
                0x82, 0x0f, 0xa0, 0xdd, 0xc1, 0xa6, 0xc1, 0x8b, 0x00, 0x86, 0xd6, 0x53, 0xff, 0xc3,
                0x7e, 0xa7, 0xa1, 0xac,
            ],
            Bitcoin::IsZero64 => [
                0xf8, 0x19, 0x13, 0xaf, 0xc6, 0x09, 0xde, 0x33, 0x40, 0xa9, 0xf6, 0x7e, 0x83, 0x92,
                0x7b, 0x57, 0x22, 0x32, 0xeb, 0x51, 0x08, 0xc0, 0x27, 0x8c, 0xbc, 0x21, 0x47, 0x61,
                0x92, 0xe0, 0x3c, 0xc7,
            ],
            Bitcoin::IsZero8 => [
                0xfd, 0x9a, 0xc0, 0x8b, 0x59, 0x5b, 0x1e, 0xcb, 0x84, 0x8d, 0xe2, 0x0f, 0xf1, 0x6b,
                0x7c, 0xb2, 0x7a, 0x77, 0x0a, 0x8a, 0x91, 0x8c, 0x01, 0x8b, 0x40, 0x25, 0xf3, 0x6a,
                0xa0, 0xc7, 0x5e, 0x32,
            ],
            Bitcoin::Le16 => [
                0x90, 0x0c, 0x66, 0x5e, 0xbe, 0x88, 0x4d, 0x50, 0x71, 0xc3, 0x2b, 0x3a, 0x0b, 0x6b,
                0x65, 0xa1, 0x2c, 0xba, 0xb6, 0xf0, 0x82, 0x48, 0x0e, 0xfa, 0x06, 0x94, 0x5a, 0x06,
                0x5f, 0xe1, 0x58, 0x66,
            ],
            Bitcoin::Le32 => [
                0xef, 0x40, 0xfc, 0x7c, 0x21, 0x58, 0x56, 0xbb, 0x7e, 0x87, 0x41, 0x4a, 0x39, 0x5b,
                0x4f, 0x02, 0xe2, 0xc4, 0xbb, 0x16, 0x7c, 0x85, 0x26, 0xed, 0x43, 0x10, 0x5d, 0x56,
                0x42, 0xb0, 0xd6, 0xb6,
            ],
            Bitcoin::Le64 => [
                0xf9, 0x1b, 0x28, 0x86, 0xb9, 0xda, 0x94, 0xf7, 0xe7, 0x97, 0xb7, 0x58, 0x14, 0xcf,
                0xee, 0xb2, 0x8c, 0xe0, 0x73, 0xe2, 0x14, 0xe5, 0xa5, 0x5f, 0x70, 0x19, 0xb6, 0x97,
                0x8b, 0x0d, 0x67, 0x42,
            ],
            Bitcoin::Le8 => [
                0x8c, 0xfb, 0x5e, 0x97, 0x85, 0x64, 0x45, 0x45, 0xcc, 0x42, 0xeb, 0x6a, 0x14, 0xb7,
                0xe9, 0x58, 0x27, 0x94, 0x99, 0x00, 0xaa, 0xa3, 0xc5, 0xad, 0xc8, 0x02, 0x4a, 0xf3,
                0xc2, 0xa3, 0x1b, 0x44,
            ],
            Bitcoin::LinearCombination1 => [
                0x95, 0x07, 0x86, 0xef, 0xa6, 0x5a, 0x71, 0x22, 0xe2, 0x55, 0x4c, 0x6f, 0xb5, 0x51,
                0x24, 0xf9, 0xe5, 0xac, 0xd8, 0x2c, 0x29, 0x81, 0x7a, 0xff, 0xc1, 0x9f, 0xc7, 0xa9,
                0x27, 0xd3, 0xa0, 0x70,
            ],
            Bitcoin::LinearVerify1 => [
                0x63, 0x55, 0x71, 0xb1, 0x27, 0xc0, 0x15, 0x65, 0x7c, 0x1b, 0xfb, 0x1d, 0x92, 0x67,
                0xbb, 0x84, 0x6a, 0x7b, 0xf9, 0x49, 0x75, 0x07, 0xae, 0xa6, 0x65, 0x37, 0x35, 0x74,
                0x08, 0xe7, 0x11, 0xa3,
            ],
            Bitcoin::LockTime => [
                0x00, 0x85, 0xc6, 0x80, 0x7e, 0xa7, 0xba, 0x60, 0x56, 0x03, 0xe9, 0x23, 0xf7, 0x7a,
                0x24, 0x03, 0xf3, 0x71, 0xb8, 0x85, 0xe7, 0xdd, 0xb0, 0x74, 0x29, 0xa9, 0x89, 0xe1,
                0xa1, 0x06, 0xe9, 0xce,
            ],
            Bitcoin::Low16 => [
                0xb3, 0x27, 0xae, 0x29, 0x99, 0xed, 0x5f, 0x59, 0x42, 0x15, 0x84, 0xfd, 0x53, 0x7f,
                0x99, 0xc5, 0xfa, 0x10, 0x27, 0x1e, 0x53, 0xe1, 0x33, 0x2b, 0x1f, 0x53, 0x46, 0x35,
                0x54, 0x0d, 0xc8, 0x4d,
            ],
            Bitcoin::Low32 => [
                0x3a, 0x13, 0x27, 0x6c, 0x60, 0x97, 0xd2, 0x72, 0xdb, 0x13, 0x98, 0xc1, 0xd0, 0x3e,
                0x13, 0x02, 0x2e, 0x72, 0x9d, 0x73, 0xce, 0x50, 0xf8, 0xb2, 0xe5, 0xb2, 0xaa, 0xf9,
                0x93, 0x6a, 0x2b, 0xe1,
            ],
            Bitcoin::Low64 => [
                0x06, 0x19, 0xc9, 0x95, 0x66, 0xf8, 0xd3, 0xdc, 0x82, 0x6e, 0x8e, 0xf9, 0x67, 0x65,
                0x70, 0x19, 0x1d, 0xbc, 0xe1, 0xa0, 0x2e, 0x74, 0xd9, 0x22, 0xb1, 0x12, 0x0c, 0xb7,
                0x2d, 0x38, 0x51, 0x3b,
            ],
            Bitcoin::Low8 => [
                0xaa, 0xb5, 0xa1, 0xe3, 0xe6, 0xf0, 0x27, 0x47, 0xd5, 0xb6, 0x48, 0x97, 0xfe, 0x37,
                0xf2, 0xf7, 0x37, 0xca, 0x2b, 0xe4, 0xdc, 0x9c, 0x85, 0xa6, 0xe8, 0x04, 0xad, 0x5e,
                0x95, 0xed, 0x08, 0x33,
            ],
            Bitcoin::Lt16 => [
                0x26, 0x7a, 0xb9, 0xe8, 0xde, 0xee, 0xfa, 0x01, 0xbd, 0xd4, 0xd9, 0xfc, 0x52, 0x4b,
                0xda, 0x2c, 0x4a, 0xe5, 0xb9, 0x5e, 0x77, 0x25, 0xab, 0x97, 0xaa, 0x04, 0x22, 0x01,
                0x3e, 0x86, 0xa4, 0xae,
            ],
            Bitcoin::Lt32 => [
                0xc3, 0x1a, 0x47, 0x16, 0x1a, 0x1b, 0x5f, 0x77, 0xe2, 0x79, 0xfd, 0x22, 0xba, 0x9a,
                0xb0, 0xc3, 0xe1, 0x9c, 0x77, 0x19, 0xa0, 0x0c, 0xfe, 0xe6, 0x03, 0x6d, 0xb6, 0x8e,
                0x1b, 0x9e, 0xc6, 0xdf,
            ],
            Bitcoin::Lt64 => [
                0x58, 0x6e, 0x2b, 0x1f, 0x0b, 0x0c, 0xfa, 0x80, 0x0f, 0x83, 0x1d, 0x35, 0xed, 0x1f,
                0xdb, 0x96, 0x4f, 0xeb, 0x47, 0xbc, 0x62, 0xf7, 0xc0, 0xd4, 0x16, 0xde, 0x3f, 0xd8,
                0x3b, 0xb2, 0x56, 0x42,
            ],
            Bitcoin::Lt8 => [
                0xaf, 0xe9, 0x89, 0xbf, 0xd8, 0xba, 0x4d, 0xc1, 0x06, 0xd2, 0xe3, 0xb0, 0xb3, 0x0b,
                0x1d, 0x9b, 0x5e, 0x69, 0x16, 0xcc, 0xce, 0xbc, 0xd9, 0x87, 0x40, 0xb6, 0xa9, 0xc3,
                0xb9, 0x03, 0xb6, 0x26,
            ],
            Bitcoin::Maj16 => [
                0xd4, 0x47, 0x91, 0x1b, 0x6f, 0xbd, 0x54, 0x7d, 0x43, 0x05, 0x66, 0xbc, 0xe5, 0x21,
                0xcc, 0xf2, 0x5d, 0xff, 0x0f, 0xf5, 0x2a, 0xe7, 0xab, 0x01, 0x0b, 0x69, 0x5a, 0xe0,
                0x87, 0xec, 0x1b, 0x71,
            ],
            Bitcoin::Maj32 => [
                0x66, 0x57, 0x11, 0xe6, 0x27, 0x0b, 0xa2, 0x67, 0x56, 0xc0, 0xc1, 0x85, 0xb7, 0x01,
                0x17, 0xa1, 0x29, 0x6f, 0xf6, 0xc2, 0x9e, 0x42, 0x6a, 0xa9, 0x55, 0xf6, 0x14, 0x95,
                0x85, 0x28, 0x6c, 0x49,
            ],
            Bitcoin::Maj64 => [
                0x69, 0xf4, 0xce, 0xd9, 0xc7, 0x12, 0x73, 0x6d, 0x0d, 0x8e, 0xe1, 0xc4, 0xdc, 0xda,
                0x4f, 0xf5, 0x7f, 0x24, 0xeb, 0x31, 0x42, 0xf9, 0xe0, 0x68, 0x48, 0xde, 0x60, 0xbe,
                0x57, 0x5c, 0x91, 0x49,
            ],
            Bitcoin::Maj8 => [
                0x6b, 0xcb, 0xfe, 0x9c, 0xf8, 0xc1, 0x32, 0x41, 0x11, 0x77, 0xba, 0x84, 0xf4, 0x0f,
                0xc8, 0x4e, 0x0b, 0x8c, 0x16, 0x44, 0x24, 0x50, 0x5f, 0xdd, 0x3b, 0xdf, 0x27, 0xf7,
                0x2a, 0x0d, 0xf7, 0xf6,
            ],
            Bitcoin::Max16 => [
                0x19, 0x34, 0x49, 0xe0, 0xb8, 0x45, 0xbe, 0x48, 0xbb, 0xb1, 0x13, 0x24, 0xba, 0xc6,
                0x34, 0x45, 0x1a, 0x53, 0xbf, 0x03, 0x12, 0xcb, 0x0e, 0x8b, 0x71, 0xb1, 0xd4, 0x63,
                0x67, 0x56, 0x92, 0xfd,
            ],
            Bitcoin::Max32 => [
                0xee, 0x99, 0xdd, 0x4c, 0x3d, 0xe4, 0x96, 0xa2, 0x74, 0xce, 0x3c, 0x06, 0x50, 0xf8,
                0x1c, 0x1e, 0xd9, 0xce, 0xa8, 0x5b, 0xaa, 0xf2, 0x9c, 0x21, 0x33, 0x33, 0x70, 0x98,
                0x4c, 0xa2, 0x53, 0x5c,
            ],
            Bitcoin::Max64 => [
                0xfc, 0x75, 0xa5, 0xd0, 0xc2, 0xde, 0x6f, 0xc6, 0xc3, 0xf3, 0x52, 0x5c, 0x6e, 0x8a,
                0x89, 0x38, 0x35, 0xaa, 0x9f, 0x7a, 0x8d, 0x3d, 0x35, 0x18, 0x1d, 0x0a, 0x58, 0x1b,
                0x27, 0x23, 0xee, 0x4e,
            ],
            Bitcoin::Max8 => [
                0xcf, 0xcf, 0x17, 0x20, 0x71, 0xf0, 0x3c, 0xa6, 0x18, 0xcc, 0xd3, 0x58, 0x21, 0x06,
                0x3d, 0x4e, 0xc1, 0x3c, 0xcc, 0x6d, 0x73, 0xdf, 0x99, 0x46, 0xe9, 0xcd, 0xb7, 0x77,
                0xb1, 0x1f, 0xb4, 0x37,
            ],
            Bitcoin::Median16 => [
                0x04, 0xb1, 0x02, 0xc6, 0xbf, 0x51, 0x7d, 0x0d, 0x58, 0x90, 0xd5, 0x5e, 0x0d, 0x1a,
                0x0c, 0x23, 0x95, 0x40, 0x82, 0x0a, 0x0c, 0xde, 0x26, 0x20, 0xd5, 0x2c, 0x60, 0x2c,
                0x65, 0x56, 0xaf, 0x66,
            ],
            Bitcoin::Median32 => [
                0x77, 0x9f, 0xeb, 0xff, 0x10, 0xbb, 0x16, 0x40, 0xa7, 0x6e, 0xdb, 0x05, 0x10, 0x58,
                0xd5, 0x8f, 0xe7, 0xc3, 0x79, 0xbb, 0x8b, 0x73, 0xd2, 0xe2, 0xb1, 0xd7, 0xc2, 0x0d,
                0x26, 0x82, 0xc5, 0x0d,
            ],
            Bitcoin::Median64 => [
                0xdb, 0x62, 0xcc, 0x65, 0xbe, 0x87, 0xa9, 0x07, 0xb6, 0x87, 0xbc, 0x32, 0x36, 0xf7,
                0x5b, 0x63, 0xa7, 0xb7, 0x3d, 0x75, 0x64, 0x9e, 0xb6, 0x54, 0xeb, 0xac, 0x19, 0x68,
                0x6c, 0x32, 0x62, 0x8e,
            ],
            Bitcoin::Median8 => [
                0x89, 0x77, 0x37, 0x02, 0x27, 0x41, 0x9e, 0xc2, 0x26, 0xba, 0x4a, 0xd4, 0x5f, 0x44,
                0x06, 0x4f, 0x82, 0xf9, 0x73, 0x03, 0x94, 0xcb, 0x79, 0x2d, 0xc5, 0x43, 0xa6, 0x5b,
                0xca, 0xae, 0x7b, 0x09,
            ],
            Bitcoin::Min16 => [
                0x09, 0x38, 0x26, 0x1d, 0xf2, 0xa5, 0xa0, 0xde, 0x39, 0x71, 0x26, 0xe7, 0x65, 0x41,
                0xe2, 0x16, 0x6b, 0x24, 0x2a, 0xe8, 0x87, 0x57, 0x0d, 0xba, 0xcf, 0x9a, 0x3b, 0x6f,
                0x98, 0x21, 0x33, 0xd9,
            ],
            Bitcoin::Min32 => [
                0xa2, 0xb8, 0x5a, 0xe1, 0x70, 0xc9, 0x9f, 0x8f, 0x46, 0xa0, 0xf8, 0xf8, 0xc2, 0x1b,
                0xa6, 0xe1, 0x41, 0x9b, 0x69, 0x1d, 0x46, 0xef, 0x6b, 0x28, 0x73, 0xdf, 0x96, 0xdb,
                0xb9, 0x85, 0x89, 0x00,
            ],
            Bitcoin::Min64 => [
                0x3e, 0xa4, 0x86, 0x19, 0x3d, 0x0d, 0x1a, 0xff, 0x0c, 0xc6, 0xea, 0xab, 0x18, 0x31,
                0xf5, 0xf2, 0x5c, 0x30, 0xe7, 0x8d, 0x7e, 0x25, 0xb0, 0xb0, 0xc2, 0x78, 0x1e, 0x1e,
                0xca, 0xbf, 0x46, 0x2e,
            ],
            Bitcoin::Min8 => [
                0xe9, 0xb4, 0xe5, 0x41, 0xe3, 0x0d, 0xd5, 0x18, 0x37, 0xa2, 0x38, 0x8c, 0xb0, 0x6c,
                0x3d, 0xe9, 0x05, 0x68, 0xb4, 0xe8, 0xfe, 0x24, 0x02, 0x01, 0xfd, 0xa8, 0x33, 0xae,
                0x57, 0x89, 0x91, 0x64,
            ],
            Bitcoin::Modulo16 => [
                0x6d, 0xcf, 0xa1, 0x9b, 0xf6, 0x5f, 0x47, 0x8d, 0x30, 0x6f, 0x11, 0x3f, 0xbb, 0x64,
                0xb3, 0x3a, 0xe2, 0x1e, 0x99, 0xc0, 0xe7, 0x5c, 0x46, 0xa9, 0xd8, 0x7d, 0xbd, 0xaf,
                0x72, 0x9e, 0xc8, 0xa4,
            ],
            Bitcoin::Modulo32 => [
                0x33, 0x2a, 0xd0, 0x58, 0x62, 0x6c, 0x5e, 0x58, 0x81, 0xc7, 0x4a, 0xab, 0x45, 0x82,
                0x02, 0x08, 0xef, 0x3b, 0x3e, 0xe5, 0xc0, 0x8a, 0x0d, 0xe2, 0x87, 0x12, 0x45, 0x92,
                0x11, 0x18, 0x21, 0xec,
            ],
            Bitcoin::Modulo64 => [
                0x1f, 0x5d, 0x0b, 0x25, 0x5f, 0xe6, 0xd8, 0x3e, 0x33, 0xd2, 0x01, 0x9a, 0x2c, 0x66,
                0xec, 0xe9, 0x75, 0x0c, 0x02, 0x07, 0x3a, 0x10, 0xdf, 0x2a, 0x57, 0x3b, 0xc0, 0xe9,
                0x02, 0xbc, 0x59, 0x1b,
            ],
            Bitcoin::Modulo8 => [
                0xfc, 0x66, 0x85, 0xa7, 0x70, 0x98, 0xb1, 0x09, 0x29, 0xc6, 0xa2, 0xba, 0x8c, 0x97,
                0xb9, 0xa0, 0xbc, 0x64, 0xb1, 0xf2, 0x0a, 0xe7, 0x8d, 0x53, 0x9f, 0x59, 0x2b, 0xe9,
                0x72, 0x95, 0x07, 0x9d,
            ],
            Bitcoin::Multiply16 => [
                0x53, 0xcb, 0x58, 0x3e, 0xde, 0xe8, 0xbf, 0x8d, 0x65, 0xdb, 0xa9, 0x95, 0x65, 0x70,
                0x8a, 0x94, 0x75, 0xa7, 0xc3, 0x75, 0x3f, 0x09, 0x83, 0xe4, 0x86, 0x1f, 0x22, 0xdf,
                0xb6, 0x6a, 0x3d, 0x58,
            ],
            Bitcoin::Multiply32 => [
                0x0f, 0x2d, 0x9f, 0xf6, 0x5b, 0xa4, 0x94, 0x2c, 0x5f, 0xfa, 0x4d, 0x8e, 0x6e, 0x1f,
                0x5d, 0x91, 0x0c, 0x1a, 0xbb, 0xfe, 0x24, 0xd8, 0x65, 0xa0, 0x2e, 0x06, 0x67, 0x20,
                0xe7, 0xa4, 0xcb, 0x66,
            ],
            Bitcoin::Multiply64 => [
                0x6a, 0x6d, 0x34, 0xfe, 0xcc, 0xf3, 0xc6, 0xcd, 0xa6, 0x35, 0x5a, 0x41, 0xd0, 0x74,
                0xe3, 0x18, 0xf9, 0x1a, 0xc0, 0x98, 0x21, 0xce, 0x0a, 0x49, 0xe2, 0x52, 0x15, 0xa9,
                0xce, 0x8b, 0x41, 0x57,
            ],
            Bitcoin::Multiply8 => [
                0x4e, 0x55, 0x2a, 0xfc, 0x5d, 0xdb, 0xfe, 0x21, 0xf8, 0x0e, 0x69, 0xff, 0xf6, 0xb2,
                0x19, 0x43, 0xd0, 0x99, 0x43, 0x91, 0x83, 0x39, 0x82, 0xef, 0xda, 0xdb, 0x86, 0x7d,
                0x06, 0x16, 0x75, 0x48,
            ],
            Bitcoin::Negate16 => [
                0x10, 0xdd, 0xc6, 0x5d, 0x21, 0xf5, 0xad, 0x08, 0x2d, 0x70, 0xe9, 0xaf, 0x9b, 0xe4,
                0x6d, 0xc5, 0xcb, 0x5e, 0xe3, 0xd6, 0x16, 0xa8, 0x3b, 0x61, 0xf6, 0xd1, 0xb4, 0x45,
                0xf7, 0x6f, 0xc2, 0x21,
            ],
            Bitcoin::Negate32 => [
                0x30, 0x00, 0x9b, 0x3c, 0x58, 0x8c, 0x0d, 0xee, 0x3e, 0x5b, 0xc9, 0xa4, 0x1b, 0x2d,
                0x0b, 0xb0, 0x8c, 0x0a, 0xf0, 0xd9, 0xb4, 0x2c, 0x88, 0xa5, 0xb7, 0x07, 0xe3, 0x6c,
                0x56, 0x48, 0x2c, 0x9b,
            ],
            Bitcoin::Negate64 => [
                0xa0, 0xc2, 0xa1, 0x66, 0x13, 0xa1, 0xfc, 0xbc, 0x13, 0x93, 0xca, 0x07, 0x0b, 0xb5,
                0x58, 0xf6, 0x57, 0x65, 0x9b, 0x07, 0xf0, 0xb4, 0x3e, 0x61, 0xce, 0x9c, 0x62, 0x7e,
                0xd9, 0xd7, 0xdb, 0x63,
            ],
            Bitcoin::Negate8 => [
                0xcf, 0xb4, 0x38, 0xb3, 0x07, 0x88, 0xd7, 0xda, 0x99, 0xf7, 0x87, 0xf2, 0xfa, 0x89,
                0x33, 0x3e, 0xde, 0x1c, 0xf6, 0x3b, 0x84, 0x78, 0x02, 0x26, 0x24, 0xd6, 0x15, 0x90,
                0xad, 0x50, 0xa7, 0x37,
            ],
            Bitcoin::NumInputs => [
                0x1b, 0x71, 0x43, 0x1e, 0xa1, 0x5b, 0xa2, 0x2f, 0xf9, 0x21, 0xe6, 0xac, 0x21, 0xdd,
                0x29, 0x2d, 0x21, 0x14, 0x0a, 0x32, 0xdb, 0x06, 0x88, 0x42, 0x0c, 0x37, 0x97, 0x07,
                0x14, 0x29, 0x07, 0x4a,
            ],
            Bitcoin::NumOutputs => [
                0xd6, 0xf4, 0x7a, 0x61, 0x73, 0x62, 0x0b, 0x01, 0x11, 0xe1, 0x66, 0xd8, 0x09, 0x5b,
                0x5d, 0xdb, 0x00, 0x27, 0x5c, 0x83, 0x8a, 0xc0, 0x91, 0xea, 0x82, 0x9b, 0xf8, 0xaa,
                0x70, 0x4e, 0x4a, 0xcd,
            ],
            Bitcoin::One16 => [
                0xef, 0x62, 0xcf, 0x64, 0xfe, 0x05, 0x88, 0x46, 0x57, 0x80, 0xe6, 0x00, 0xd2, 0x8c,
                0x83, 0x14, 0x75, 0x1c, 0x74, 0xf5, 0xd2, 0x41, 0x7f, 0xaf, 0x7b, 0x58, 0x72, 0x9a,
                0x79, 0xa4, 0x03, 0xfa,
            ],
            Bitcoin::One32 => [
                0x0a, 0x81, 0x86, 0xf9, 0xd6, 0x6a, 0xfb, 0xcc, 0xb1, 0x53, 0xe8, 0x9e, 0x5d, 0x21,
                0x53, 0x0f, 0x0d, 0x66, 0x01, 0xfa, 0x35, 0x9c, 0x4a, 0x30, 0x58, 0xe9, 0x09, 0xbc,
                0xef, 0xfe, 0x20, 0x7d,
            ],
            Bitcoin::One64 => [
                0x96, 0x4d, 0xf8, 0xba, 0xb0, 0x50, 0x82, 0x11, 0x74, 0x67, 0x44, 0x76, 0x12, 0x31,
                0x7e, 0x1c, 0x64, 0x53, 0x8d, 0xe6, 0xc9, 0xb2, 0x5d, 0xd6, 0xb1, 0x18, 0x60, 0xdf,
                0x36, 0x73, 0x94, 0x25,
            ],
            Bitcoin::One8 => [
                0xd3, 0xdb, 0xa3, 0x01, 0x75, 0x38, 0x9b, 0xf1, 0x33, 0x05, 0x15, 0x6a, 0xf9, 0xd6,
                0x9c, 0x6b, 0xa2, 0x04, 0xd3, 0xb2, 0x69, 0xbf, 0x8e, 0x2b, 0x90, 0x38, 0xe0, 0x63,
                0x50, 0x3a, 0xe5, 0xcf,
            ],
            Bitcoin::Or16 => [
                0x01, 0xfb, 0x5f, 0x5a, 0xe2, 0xaa, 0xbb, 0x88, 0x93, 0x52, 0x32, 0x88, 0x89, 0xbc,
                0x69, 0xc0, 0xe5, 0x2e, 0xeb, 0x96, 0x71, 0x5b, 0x18, 0x9e, 0x33, 0x38, 0x04, 0xf0,
                0xa4, 0x92, 0x4c, 0x9c,
            ],
            Bitcoin::Or32 => [
                0xb2, 0x75, 0x8f, 0x51, 0x14, 0x49, 0xf2, 0xaa, 0x77, 0x66, 0x43, 0xd8, 0xe0, 0x4b,
                0xfb, 0x6e, 0x10, 0xb4, 0x59, 0x1d, 0xba, 0x91, 0x9e, 0x1c, 0x19, 0x4f, 0xc8, 0x9b,
                0xb0, 0x45, 0xfd, 0x29,
            ],
            Bitcoin::Or64 => [
                0xa4, 0x0d, 0x0b, 0x13, 0x84, 0xa3, 0xe0, 0xb0, 0xad, 0xf5, 0x93, 0x6b, 0x78, 0x27,
                0x9e, 0x52, 0x3f, 0xe5, 0x5b, 0xeb, 0x3f, 0xe4, 0x26, 0x68, 0x3b, 0xc8, 0x7e, 0xc9,
                0x9b, 0xbe, 0x27, 0x9a,
            ],
            Bitcoin::Or8 => [
                0xc6, 0x03, 0xc1, 0xe8, 0x1b, 0x5a, 0x2e, 0x8e, 0x4c, 0xbf, 0x81, 0xd0, 0x59, 0xa5,
                0xa0, 0xc7, 0xc4, 0xd9, 0x1a, 0x17, 0x92, 0x8c, 0x9d, 0x5c, 0xae, 0xbf, 0x67, 0xb8,
                0x6d, 0x00, 0x34, 0xc3,
            ],
            Bitcoin::OutputScriptHash => [
                0x1d, 0xd1, 0x4c, 0x91, 0xe0, 0x7e, 0x5b, 0xc1, 0xf4, 0x8c, 0xe1, 0xf4, 0x3d, 0x53,
                0x07, 0xf1, 0x60, 0x96, 0xf5, 0x09, 0x3b, 0x65, 0x9d, 0xcf, 0x07, 0x3c, 0x80, 0x2d,
                0x7c, 0x49, 0x0e, 0xd2,
            ],
            Bitcoin::OutputValue => [
                0x76, 0xce, 0xfc, 0x59, 0x99, 0xd8, 0xdb, 0x66, 0xfb, 0x7e, 0xf0, 0x5f, 0xf6, 0x5e,
                0xa8, 0xfc, 0x0a, 0xd2, 0x61, 0x59, 0x2d, 0x96, 0xdc, 0xac, 0xe6, 0xb9, 0x21, 0xf4,
                0x61, 0xc1, 0x07, 0x91,
            ],
            Bitcoin::ParseLock => [
                0x48, 0x71, 0xcb, 0xae, 0x52, 0x78, 0x66, 0xba, 0x12, 0x50, 0x81, 0xdf, 0x03, 0x75,
                0x1e, 0x23, 0x1a, 0x61, 0x28, 0x22, 0xbd, 0x45, 0xde, 0x3b, 0xcf, 0xe6, 0xa8, 0xc5,
                0x14, 0xe2, 0xe7, 0x43,
            ],
            Bitcoin::ParseSequence => [
                0xfc, 0xf5, 0xd5, 0xcf, 0x69, 0x08, 0x93, 0x5e, 0x58, 0x5e, 0x9a, 0xfc, 0xc0, 0x07,
                0x17, 0xfd, 0xb1, 0xc5, 0x48, 0x79, 0x5b, 0x23, 0xf0, 0x8b, 0xb8, 0x13, 0x83, 0x17,
                0x7d, 0x93, 0xca, 0xd9,
            ],
            Bitcoin::PointVerify1 => [
                0x6a, 0x08, 0x9d, 0x61, 0xca, 0x20, 0x0a, 0x42, 0x58, 0xe8, 0xb5, 0xb4, 0xfe, 0x5c,
                0x08, 0xd5, 0x74, 0x85, 0x62, 0x49, 0x8d, 0x75, 0xf6, 0xc6, 0x26, 0x09, 0xbb, 0x68,
                0xc9, 0x8b, 0x40, 0x7c,
            ],
            Bitcoin::ScalarAdd => [
                0xa2, 0x3a, 0xa2, 0xc3, 0x3d, 0xd1, 0xef, 0xf2, 0x49, 0xa1, 0x7c, 0x91, 0xc9, 0x9a,
                0x15, 0x1d, 0x30, 0x87, 0xdc, 0x7c, 0x45, 0x91, 0xcf, 0x94, 0xc5, 0xc6, 0xab, 0xc1,
                0xba, 0x6d, 0x1c, 0xec,
            ],
            Bitcoin::ScalarInvert => [
                0xf2, 0x2d, 0xf7, 0xd7, 0x5e, 0xfd, 0xea, 0x1c, 0xfc, 0x53, 0xc5, 0x70, 0xcb, 0x8b,
                0x12, 0xa2, 0xd3, 0x41, 0x06, 0x7c, 0xb5, 0xfe, 0x41, 0xe9, 0x9d, 0xbd, 0x5f, 0xb6,
                0x32, 0x82, 0xe5, 0x1e,
            ],
            Bitcoin::ScalarIsZero => [
                0x41, 0x62, 0xf9, 0x33, 0x27, 0x03, 0xd0, 0x1e, 0x8d, 0xa6, 0x36, 0x86, 0xe8, 0x57,
                0xf2, 0x8c, 0x19, 0x82, 0x03, 0x10, 0x72, 0x2e, 0x09, 0x6b, 0xb9, 0x7f, 0xb4, 0x61,
                0xaf, 0xf5, 0x62, 0x0c,
            ],
            Bitcoin::ScalarMultiply => [
                0x72, 0xcb, 0x04, 0x36, 0xf6, 0xf1, 0x47, 0x8c, 0xcf, 0x8c, 0x54, 0x56, 0x98, 0xc6,
                0x74, 0xdf, 0x03, 0xc3, 0x9a, 0x9d, 0x2d, 0xab, 0xa1, 0x31, 0x50, 0x33, 0x46, 0xef,
                0x30, 0xd5, 0x8c, 0x26,
            ],
            Bitcoin::ScalarMultiplyLambda => [
                0x2b, 0x60, 0x03, 0x37, 0x64, 0xce, 0xec, 0x76, 0x23, 0xaf, 0x54, 0xd3, 0x86, 0xf6,
                0x87, 0xce, 0x30, 0x03, 0x25, 0x70, 0xb5, 0x6e, 0x9d, 0x5c, 0x31, 0x64, 0x8f, 0xbf,
                0x65, 0xe0, 0x14, 0x27,
            ],
            Bitcoin::ScalarNegate => [
                0x64, 0x5a, 0xe5, 0x73, 0xd5, 0xd6, 0x8b, 0x2b, 0x1a, 0xfe, 0xac, 0xe9, 0x06, 0x39,
                0xbd, 0x3a, 0x28, 0x9e, 0xeb, 0x82, 0x61, 0xa6, 0xb4, 0x72, 0x4c, 0x23, 0xca, 0xa9,
                0x1b, 0x36, 0x59, 0x3c,
            ],
            Bitcoin::ScalarNormalize => [
                0xdd, 0x37, 0x65, 0xa5, 0x16, 0x61, 0xcf, 0x66, 0x06, 0x92, 0x2f, 0x61, 0x9f, 0xca,
                0x64, 0x58, 0x52, 0x9d, 0x1e, 0x92, 0x56, 0x09, 0x83, 0x46, 0x92, 0x22, 0x47, 0xbd,
                0x04, 0x1b, 0xc1, 0x1f,
            ],
            Bitcoin::ScalarSquare => [
                0x6b, 0xff, 0xd4, 0x35, 0x3f, 0x64, 0xfe, 0x9d, 0xf8, 0xcf, 0xcb, 0x58, 0x6d, 0xfd,
                0xd0, 0x04, 0x4c, 0xb0, 0x3a, 0x6d, 0xe9, 0x13, 0x7d, 0x4c, 0xe9, 0x94, 0xff, 0x9d,
                0xa1, 0x35, 0xff, 0xe4,
            ],
            Bitcoin::Scale => [
                0x22, 0x9c, 0x9f, 0xaf, 0xad, 0xd9, 0x74, 0x5e, 0x00, 0xd1, 0x08, 0xb8, 0x2b, 0x83,
                0x62, 0x05, 0x9e, 0x83, 0x57, 0x0d, 0xfc, 0x36, 0xcb, 0x1a, 0x2c, 0xe9, 0xc5, 0xc2,
                0xd9, 0x13, 0xc6, 0x44,
            ],
            Bitcoin::ScriptCMR => [
                0xee, 0x7b, 0x41, 0x67, 0x15, 0x64, 0xd4, 0x20, 0x3e, 0xea, 0x8e, 0xaf, 0x03, 0x87,
                0xbd, 0x9e, 0x4b, 0xed, 0x50, 0x16, 0x96, 0x3e, 0xcb, 0x25, 0x67, 0xb3, 0x0e, 0x9e,
                0xa0, 0xff, 0xaf, 0x5b,
            ],
            Bitcoin::Sha256Block => [
                0x45, 0xb2, 0x78, 0x86, 0xa5, 0xee, 0xd4, 0x09, 0xef, 0x05, 0xb5, 0x30, 0xc7, 0x0e,
                0x36, 0x62, 0xfa, 0xee, 0x43, 0x12, 0xd3, 0x75, 0x9d, 0xf2, 0x41, 0x44, 0x98, 0x36,
                0x1b, 0x51, 0x17, 0xae,
            ],
            Bitcoin::Sha256Ctx8Add1 => [
                0x41, 0x66, 0x54, 0x90, 0xd6, 0xed, 0xb4, 0xc5, 0xd5, 0x28, 0x45, 0x68, 0x2d, 0x7c,
                0xbd, 0x1e, 0x6c, 0x07, 0x45, 0xfa, 0xfc, 0xc0, 0x92, 0xf3, 0xef, 0x1b, 0x6f, 0xe9,
                0xcc, 0xb4, 0x2b, 0x59,
            ],
            Bitcoin::Sha256Ctx8Add128 => [
                0xed, 0x92, 0xb8, 0x91, 0x9a, 0xbb, 0x89, 0x1c, 0x0f, 0xd2, 0x0f, 0xfe, 0xa6, 0xca,
                0xae, 0xa1, 0xbd, 0x79, 0xb9, 0xe3, 0xa8, 0x2a, 0xfe, 0x2c, 0xc0, 0xce, 0x47, 0x30,
                0x86, 0xf2, 0xae, 0x2a,
            ],
            Bitcoin::Sha256Ctx8Add16 => [
                0x1a, 0xd8, 0xc2, 0x19, 0xf9, 0x71, 0xa4, 0xae, 0x29, 0xc2, 0x69, 0x95, 0x97, 0x15,
                0x0a, 0x11, 0x5b, 0xae, 0x2c, 0x09, 0xf3, 0xbc, 0x5f, 0xd8, 0x79, 0x50, 0x21, 0x63,
                0xee, 0xdd, 0x7b, 0x8d,
            ],
            Bitcoin::Sha256Ctx8Add2 => [
                0x25, 0xab, 0x7a, 0xa9, 0xf7, 0xf8, 0x57, 0xbd, 0x26, 0x89, 0x5b, 0x89, 0x0d, 0xf3,
                0x33, 0x16, 0xd7, 0x0f, 0x70, 0xb0, 0xc2, 0x68, 0xea, 0x3e, 0xc0, 0x19, 0xe0, 0x3e,
                0x50, 0xc5, 0xbd, 0x7e,
            ],
            Bitcoin::Sha256Ctx8Add256 => [
                0x10, 0xde, 0x9d, 0xa4, 0xe8, 0xf7, 0xa2, 0x91, 0x80, 0x84, 0x37, 0x91, 0x2e, 0x24,
                0xd2, 0x48, 0xe8, 0x4d, 0xac, 0xd7, 0x49, 0x68, 0x25, 0xd3, 0xca, 0xc5, 0x06, 0x72,
                0x3c, 0x21, 0x89, 0x1f,
            ],
            Bitcoin::Sha256Ctx8Add32 => [
                0x69, 0x56, 0xf8, 0x07, 0xb5, 0x6e, 0xd6, 0x64, 0x07, 0xed, 0xe0, 0xbf, 0xff, 0xb6,
                0x18, 0x69, 0x9f, 0xc5, 0x78, 0x02, 0xbe, 0xd3, 0x44, 0xb6, 0x5e, 0xae, 0xf1, 0x39,
                0xec, 0x69, 0xef, 0x0c,
            ],
            Bitcoin::Sha256Ctx8Add4 => [
                0x77, 0xd6, 0xf4, 0xb1, 0x3f, 0x2c, 0x5c, 0x42, 0x5f, 0x45, 0xfc, 0xf1, 0x86, 0x9f,
                0x9a, 0xce, 0x8c, 0xc8, 0x6b, 0xd4, 0x90, 0xe7, 0xb3, 0x14, 0x13, 0x4e, 0xf5, 0x99,
                0xbb, 0xf2, 0x21, 0xba,
            ],
            Bitcoin::Sha256Ctx8Add512 => [
                0xa8, 0x76, 0x0d, 0xf3, 0x76, 0xf0, 0x7e, 0x76, 0x2d, 0xf6, 0x55, 0x9f, 0x69, 0xb1,
                0x17, 0x61, 0x38, 0x3b, 0x3f, 0xf6, 0xd8, 0x31, 0xfb, 0x5b, 0x54, 0x2b, 0x91, 0x72,
                0xfa, 0x34, 0xb9, 0x35,
            ],
            Bitcoin::Sha256Ctx8Add64 => [
                0x80, 0x48, 0xd4, 0x90, 0x5c, 0x14, 0xb1, 0x82, 0xd8, 0xf8, 0x3c, 0x81, 0x8d, 0x03,
                0x22, 0xc2, 0xd1, 0xe4, 0x79, 0x50, 0x88, 0x98, 0x1a, 0xc7, 0xab, 0x99, 0xc8, 0x7b,
                0x12, 0xc6, 0xeb, 0x88,
            ],
            Bitcoin::Sha256Ctx8Add8 => [
                0xe5, 0xc8, 0x39, 0xa5, 0xa3, 0x29, 0x6c, 0xb3, 0xcd, 0x38, 0x29, 0x7f, 0x92, 0xe2,
                0x63, 0x63, 0x0a, 0x6f, 0x29, 0x94, 0xc9, 0xbb, 0xbd, 0x2a, 0xfa, 0x7d, 0xa8, 0x3b,
                0x1e, 0xdc, 0xe8, 0xfb,
            ],
            Bitcoin::Sha256Ctx8AddBuffer511 => [
                0x11, 0xb1, 0x3d, 0x73, 0x30, 0xae, 0x18, 0xe9, 0x19, 0x7b, 0x92, 0x1b, 0x81, 0x25,
                0x75, 0x2f, 0x8f, 0xd9, 0xb5, 0x19, 0x72, 0x04, 0xda, 0x5a, 0x7e, 0xa9, 0x24, 0xec,
                0x07, 0x2c, 0xf5, 0xfe,
            ],
            Bitcoin::Sha256Ctx8Finalize => [
                0xe8, 0xc2, 0x70, 0xdf, 0x3d, 0x70, 0xa3, 0x95, 0x65, 0xd2, 0xc6, 0x45, 0x3d, 0x68,
                0xe6, 0x1f, 0x45, 0x97, 0x83, 0xf1, 0x9a, 0xa4, 0xf6, 0x2d, 0xb5, 0x6c, 0x9a, 0x1e,
                0xc1, 0x12, 0x5d, 0x6a,
            ],
            Bitcoin::Sha256Ctx8Init => [
                0x6e, 0xcf, 0xda, 0x8c, 0x9b, 0xe3, 0xe8, 0x34, 0xc5, 0xad, 0x10, 0xcc, 0xf2, 0x5d,
                0xb1, 0x5e, 0xce, 0x9b, 0xa2, 0xb3, 0x73, 0xe8, 0x5c, 0xa1, 0x81, 0xe7, 0x37, 0x39,
                0x8d, 0xa6, 0xfc, 0x80,
            ],
            Bitcoin::Sha256Iv => [
                0x9a, 0xdb, 0x29, 0x4a, 0xa5, 0x63, 0x9a, 0x79, 0x29, 0xac, 0xc5, 0xea, 0x85, 0x8b,
                0x2a, 0x89, 0xb5, 0xc5, 0xbf, 0xc1, 0xd0, 0x93, 0x6f, 0x89, 0x32, 0xae, 0x6b, 0xb2,
                0x9b, 0x3e, 0xe6, 0x69,
            ],
            Bitcoin::Some16 => [
                0xd9, 0x80, 0x12, 0xfd, 0x09, 0x90, 0xa1, 0x70, 0x4f, 0x92, 0x80, 0x3e, 0xfc, 0x14,
                0xa9, 0x56, 0xed, 0x3a, 0xd3, 0x7b, 0x3e, 0xfc, 0x40, 0x39, 0x19, 0xaf, 0x1e, 0x20,
                0x8d, 0x0b, 0x33, 0x20,
            ],
            Bitcoin::Some32 => [
                0x7a, 0x24, 0x81, 0x51, 0xb7, 0xba, 0x67, 0x00, 0x30, 0xd0, 0x90, 0xf0, 0xa0, 0x0f,
                0x42, 0x3b, 0xbe, 0x15, 0x32, 0x76, 0xb3, 0xa7, 0xa3, 0x03, 0x33, 0x44, 0x9c, 0x67,
                0x60, 0x89, 0x6f, 0xab,
            ],
            Bitcoin::Some64 => [
                0xa3, 0xa0, 0x6c, 0x9b, 0xc4, 0x78, 0xc0, 0xd4, 0x25, 0xcb, 0x26, 0x4e, 0xa9, 0xde,
                0x3e, 0x13, 0x8a, 0x7a, 0x13, 0x84, 0x9d, 0xf0, 0x61, 0x69, 0xe8, 0x5b, 0x50, 0x36,
                0xfe, 0x97, 0x69, 0x78,
            ],
            Bitcoin::Some8 => [
                0xd6, 0xe3, 0x30, 0x68, 0x8d, 0x02, 0x75, 0x32, 0x09, 0x8a, 0x6e, 0x19, 0xdc, 0x41,
                0x7a, 0x97, 0xd9, 0xde, 0xb0, 0x5a, 0x86, 0xd1, 0xf9, 0xc0, 0xc3, 0xaa, 0xca, 0x74,
                0xeb, 0xc9, 0x58, 0x28,
            ],
            Bitcoin::Subtract16 => [
                0x14, 0x23, 0x14, 0xe6, 0x74, 0x26, 0x8d, 0x2f, 0xb6, 0x57, 0xef, 0x84, 0x49, 0x85,
                0x38, 0x92, 0xee, 0xf4, 0x61, 0x40, 0x89, 0x6b, 0x72, 0x0e, 0x46, 0xe8, 0xb0, 0x0d,
                0x6d, 0x50, 0xaf, 0x54,
            ],
            Bitcoin::Subtract32 => [
                0x4c, 0x45, 0x9b, 0x19, 0x16, 0xe3, 0x1d, 0x51, 0xcc, 0x03, 0xe6, 0x11, 0x58, 0x41,
                0xfb, 0x1f, 0x92, 0x33, 0xff, 0x2b, 0x38, 0xd8, 0xe7, 0x11, 0xe9, 0xe0, 0x16, 0xcf,
                0x89, 0x65, 0x31, 0xf6,
            ],
            Bitcoin::Subtract64 => [
                0x6b, 0x35, 0x3c, 0xa0, 0x35, 0xbf, 0x5a, 0x64, 0xae, 0x2c, 0x2d, 0x4e, 0x77, 0x05,
                0xdf, 0xed, 0x8b, 0x06, 0xaa, 0x58, 0x9c, 0xa1, 0x2c, 0xc9, 0x23, 0xfd, 0x72, 0x4c,
                0xcb, 0xd1, 0x31, 0xed,
            ],
            Bitcoin::Subtract8 => [
                0x22, 0xd2, 0x4d, 0x97, 0xa4, 0x01, 0x2b, 0x0a, 0x57, 0x78, 0x7d, 0x72, 0x94, 0xd4,
                0xd9, 0xdb, 0x9d, 0x2b, 0x85, 0xba, 0x89, 0xbb, 0xc8, 0xc1, 0x24, 0x8f, 0xc8, 0x3a,
                0xfd, 0x26, 0x47, 0x17,
            ],
            Bitcoin::Tapbranch => [
                0x1d, 0xee, 0xf4, 0xcb, 0x0e, 0xe8, 0x2d, 0x5a, 0xa5, 0xac, 0x10, 0x9c, 0x81, 0x06,
                0x93, 0x42, 0x5e, 0xdb, 0xd3, 0x1e, 0x68, 0x4f, 0xb4, 0xf8, 0x8a, 0x67, 0x4f, 0x16,
                0x1f, 0xdc, 0x44, 0x71,
            ],
            Bitcoin::TapleafVersion => [
                0xef, 0x80, 0x02, 0x85, 0x0d, 0x52, 0x38, 0x09, 0x1d, 0x26, 0xf1, 0x8e, 0xe8, 0x61,
                0xf9, 0x7a, 0x4f, 0x4b, 0xdb, 0xa6, 0x6f, 0x7b, 0xda, 0x4d, 0xf6, 0x09, 0xc2, 0x74,
                0x06, 0x2d, 0xf8, 0x76,
            ],
            Bitcoin::TotalInputValue => [
                0xe3, 0x71, 0x79, 0xb5, 0xab, 0x68, 0xc0, 0xb1, 0x8b, 0xc7, 0xae, 0x14, 0xb7, 0x85,
                0x40, 0x70, 0xf7, 0xc0, 0xb5, 0x37, 0xe0, 0x6d, 0xb5, 0xc2, 0xb8, 0x3b, 0x63, 0xc2,
                0x1f, 0x7c, 0xe5, 0xd4,
            ],
            Bitcoin::TotalOutputValue => [
                0x5e, 0xd9, 0x33, 0xec, 0x34, 0x0b, 0x53, 0x2e, 0x0b, 0x20, 0x51, 0x04, 0x58, 0x8b,
                0xae, 0x81, 0xe1, 0xd2, 0x22, 0x87, 0x26, 0xed, 0x62, 0x93, 0xb3, 0x54, 0x6d, 0xfe,
                0x46, 0xa2, 0xfd, 0x13,
            ],
            Bitcoin::TxIsFinal => [
                0x41, 0x74, 0xd6, 0xdc, 0x06, 0xde, 0x71, 0xbd, 0xbd, 0x98, 0x11, 0x73, 0xa8, 0xb3,
                0x57, 0xcb, 0x81, 0x56, 0x1a, 0xdc, 0xa7, 0x9a, 0x1e, 0xf5, 0x05, 0x46, 0x99, 0xb4,
                0xaf, 0x51, 0x21, 0xfc,
            ],
            Bitcoin::TxLockDistance => [
                0x6d, 0xf8, 0xcd, 0x84, 0xf9, 0x8b, 0x1b, 0xcf, 0xfe, 0xf7, 0x13, 0xfb, 0x0d, 0x5f,
                0x17, 0x2e, 0x68, 0xa4, 0x3a, 0x77, 0x48, 0x27, 0x01, 0x1f, 0xe8, 0xd4, 0x1a, 0x33,
                0xdb, 0x57, 0x58, 0x1f,
            ],
            Bitcoin::TxLockDuration => [
                0x03, 0xa0, 0x5c, 0xba, 0x34, 0x01, 0x17, 0xa6, 0x44, 0x61, 0x48, 0x73, 0x9f, 0x5f,
                0xeb, 0xa0, 0xdc, 0xdd, 0xfd, 0x3e, 0x85, 0xcb, 0x40, 0x67, 0x5b, 0x85, 0x55, 0xaa,
                0xe1, 0x92, 0x92, 0xc8,
            ],
            Bitcoin::TxLockHeight => [
                0x91, 0x5d, 0x4d, 0x6f, 0xa3, 0x08, 0x98, 0x28, 0x27, 0x36, 0x7f, 0x01, 0x5d, 0x5d,
                0xe8, 0xb1, 0x15, 0x7f, 0x46, 0xfc, 0x77, 0xdb, 0x10, 0xd0, 0xc7, 0xad, 0x78, 0x46,
                0x14, 0xe6, 0xda, 0x1a,
            ],
            Bitcoin::TxLockTime => [
                0x41, 0xf9, 0xf2, 0x76, 0x69, 0x33, 0x2e, 0x07, 0x30, 0x4b, 0x4e, 0x5b, 0x5f, 0xbb,
                0xe3, 0x8f, 0xbf, 0x77, 0xa0, 0x7f, 0x12, 0xa9, 0x2d, 0x3b, 0x31, 0xbe, 0x38, 0xea,
                0x2f, 0x57, 0x4e, 0x1f,
            ],
            Bitcoin::Verify => [
                0x02, 0x0e, 0x84, 0x01, 0x30, 0x30, 0xec, 0x69, 0xd9, 0xa9, 0x3f, 0xec, 0x71, 0x10,
                0xe7, 0x27, 0xea, 0xd5, 0x12, 0x88, 0x5f, 0xa3, 0xc5, 0x72, 0xd8, 0xcf, 0xc3, 0x47,
                0x2c, 0xa5, 0xc8, 0xe8,
            ],
            Bitcoin::Version => [
                0x31, 0xe1, 0xba, 0x28, 0xbe, 0xc5, 0xd4, 0x16, 0xd8, 0x3d, 0xa1, 0x9a, 0x28, 0xbb,
                0xde, 0x8d, 0x4d, 0x33, 0xa7, 0x3f, 0x25, 0x19, 0x05, 0x2c, 0xd0, 0x94, 0xa2, 0x5e,
                0xd5, 0x19, 0xc6, 0x60,
            ],
            Bitcoin::Xor16 => [
                0xa6, 0x10, 0x14, 0x40, 0x5b, 0xa1, 0x4a, 0x9b, 0xd4, 0x7d, 0x3f, 0x8f, 0x38, 0x73,
                0x19, 0x76, 0x5d, 0x34, 0xf0, 0x7e, 0xe2, 0x82, 0x58, 0xe1, 0x63, 0xa8, 0xac, 0x1f,
                0x03, 0x74, 0x87, 0xd2,
            ],
            Bitcoin::Xor32 => [
                0x14, 0x6d, 0x01, 0x72, 0x51, 0x76, 0xeb, 0x83, 0x20, 0x0a, 0x70, 0xcc, 0x4b, 0x79,
                0x3c, 0xe9, 0x52, 0x5c, 0xfe, 0x28, 0x99, 0x44, 0xd4, 0xbd, 0xf2, 0xf1, 0xc2, 0x9f,
                0x89, 0x0a, 0x86, 0xf9,
            ],
            Bitcoin::Xor64 => [
                0x7c, 0x89, 0x81, 0xc7, 0x6f, 0xeb, 0x14, 0x88, 0xe3, 0x86, 0x18, 0x06, 0xe8, 0x23,
                0x36, 0x39, 0x52, 0x56, 0xb5, 0xcb, 0xa4, 0x45, 0xab, 0xf4, 0x8f, 0xec, 0x54, 0x0d,
                0xd3, 0xf9, 0x43, 0x80,
            ],
            Bitcoin::Xor8 => [
                0x4e, 0x79, 0x06, 0x51, 0x60, 0xc4, 0x53, 0x68, 0xa0, 0xf6, 0xa7, 0x60, 0xd3, 0x1d,
                0x9e, 0xeb, 0x0b, 0x92, 0xb3, 0x32, 0x74, 0x14, 0x72, 0x37, 0x10, 0x57, 0xad, 0x04,
                0x79, 0x02, 0xd3, 0xda,
            ],
            Bitcoin::XorXor16 => [
                0x5b, 0x1c, 0xd2, 0x87, 0x52, 0x72, 0x90, 0x29, 0x6e, 0xba, 0x6d, 0xff, 0xc5, 0x64,
                0x53, 0x6d, 0x4f, 0x7a, 0x04, 0x12, 0x12, 0xaf, 0x8c, 0x27, 0x1c, 0xb5, 0xb1, 0x62,
                0xc2, 0x0b, 0x87, 0x1d,
            ],
            Bitcoin::XorXor32 => [
                0x4f, 0x96, 0xb4, 0xf5, 0x65, 0x5d, 0xd3, 0xc7, 0x59, 0xfe, 0x0a, 0x4c, 0x38, 0xd1,
                0x07, 0x94, 0x79, 0xbe, 0xc4, 0xed, 0x99, 0x25, 0x29, 0xff, 0x1a, 0x07, 0x59, 0x24,
                0xbb, 0x1c, 0x63, 0xc1,
            ],
            Bitcoin::XorXor64 => [
                0x35, 0x00, 0x87, 0xbf, 0xdc, 0x11, 0x9d, 0x8c, 0x85, 0x77, 0xe9, 0xa2, 0xf5, 0x23,
                0x9e, 0x28, 0xfe, 0xcc, 0x43, 0x0f, 0x4c, 0x2d, 0x41, 0xc0, 0x18, 0x7d, 0x26, 0x3f,
                0x79, 0x66, 0x79, 0x6c,
            ],
            Bitcoin::XorXor8 => [
                0xdc, 0x4a, 0x35, 0x81, 0xee, 0xf9, 0xf3, 0xa4, 0x0e, 0xd1, 0x2f, 0xac, 0x10, 0x91,
                0x5c, 0xc6, 0x60, 0x43, 0x20, 0xc0, 0xde, 0xc9, 0x4f, 0x19, 0xe5, 0x85, 0x11, 0x27,
                0xf9, 0x4f, 0x2d, 0x36,
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
            Bitcoin::All16 => b"****22*22**22*22***22*22**22*22",
            Bitcoin::All32 => b"i",
            Bitcoin::All64 => b"l",
            Bitcoin::All8 => b"***22*22**22*22",
            Bitcoin::And16 => b"i",
            Bitcoin::And32 => b"l",
            Bitcoin::And64 => b"*ll",
            Bitcoin::And8 => b"****22*22**22*22***22*22**22*22",
            Bitcoin::Bip0340Verify => b"**hh*hh",
            Bitcoin::Ch16 => b"*****22*22**22*22***22*22**22*22i",
            Bitcoin::Ch32 => b"*il",
            Bitcoin::Ch64 => b"*l*ll",
            Bitcoin::Ch8 => b"****22*22**22*22****22*22**22*22***22*22**22*22",
            Bitcoin::CheckLockDistance => b"****22*22**22*22***22*22**22*22",
            Bitcoin::CheckLockDuration => b"****22*22**22*22***22*22**22*22",
            Bitcoin::CheckLockHeight => b"i",
            Bitcoin::CheckLockTime => b"i",
            Bitcoin::CheckSigVerify => b"**h*hh*hh",
            Bitcoin::Complement16 => b"****22*22**22*22***22*22**22*22",
            Bitcoin::Complement32 => b"i",
            Bitcoin::Complement64 => b"l",
            Bitcoin::Complement8 => b"***22*22**22*22",
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
            Bitcoin::High16 => b"1",
            Bitcoin::High32 => b"1",
            Bitcoin::High64 => b"1",
            Bitcoin::High8 => b"1",
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
            Bitcoin::Maj16 => b"*****22*22**22*22***22*22**22*22i",
            Bitcoin::Maj32 => b"*il",
            Bitcoin::Maj64 => b"*l*ll",
            Bitcoin::Maj8 => b"****22*22**22*22****22*22**22*22***22*22**22*22",
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
            Bitcoin::Or16 => b"i",
            Bitcoin::Or32 => b"l",
            Bitcoin::Or64 => b"*ll",
            Bitcoin::Or8 => b"****22*22**22*22***22*22**22*22",
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
            Bitcoin::Some16 => b"****22*22**22*22***22*22**22*22",
            Bitcoin::Some32 => b"i",
            Bitcoin::Some64 => b"l",
            Bitcoin::Some8 => b"***22*22**22*22",
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
            Bitcoin::Xor16 => b"i",
            Bitcoin::Xor32 => b"l",
            Bitcoin::Xor64 => b"*ll",
            Bitcoin::Xor8 => b"****22*22**22*22***22*22**22*22",
            Bitcoin::XorXor16 => b"*****22*22**22*22***22*22**22*22i",
            Bitcoin::XorXor32 => b"*il",
            Bitcoin::XorXor64 => b"*l*ll",
            Bitcoin::XorXor8 => b"****22*22**22*22****22*22**22*22***22*22**22*22",
        };

        TypeName(name)
    }

    fn target_ty(&self) -> TypeName {
        let name: &'static [u8] = match self {
            Bitcoin::Add16 => b"*2****22*22**22*22***22*22**22*22",
            Bitcoin::Add32 => b"*2i",
            Bitcoin::Add64 => b"*2l",
            Bitcoin::Add8 => b"*2***22*22**22*22",
            Bitcoin::All16 => b"2",
            Bitcoin::All32 => b"2",
            Bitcoin::All64 => b"2",
            Bitcoin::All8 => b"2",
            Bitcoin::And16 => b"****22*22**22*22***22*22**22*22",
            Bitcoin::And32 => b"i",
            Bitcoin::And64 => b"l",
            Bitcoin::And8 => b"***22*22**22*22",
            Bitcoin::Bip0340Verify => b"1",
            Bitcoin::Ch16 => b"****22*22**22*22***22*22**22*22",
            Bitcoin::Ch32 => b"i",
            Bitcoin::Ch64 => b"l",
            Bitcoin::Ch8 => b"***22*22**22*22",
            Bitcoin::CheckLockDistance => b"1",
            Bitcoin::CheckLockDuration => b"1",
            Bitcoin::CheckLockHeight => b"1",
            Bitcoin::CheckLockTime => b"1",
            Bitcoin::CheckSigVerify => b"1",
            Bitcoin::Complement16 => b"****22*22**22*22***22*22**22*22",
            Bitcoin::Complement32 => b"i",
            Bitcoin::Complement64 => b"l",
            Bitcoin::Complement8 => b"***22*22**22*22",
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
            Bitcoin::High16 => b"****22*22**22*22***22*22**22*22",
            Bitcoin::High32 => b"i",
            Bitcoin::High64 => b"l",
            Bitcoin::High8 => b"***22*22**22*22",
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
            Bitcoin::Maj16 => b"****22*22**22*22***22*22**22*22",
            Bitcoin::Maj32 => b"i",
            Bitcoin::Maj64 => b"l",
            Bitcoin::Maj8 => b"***22*22**22*22",
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
            Bitcoin::Or16 => b"****22*22**22*22***22*22**22*22",
            Bitcoin::Or32 => b"i",
            Bitcoin::Or64 => b"l",
            Bitcoin::Or8 => b"***22*22**22*22",
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
            Bitcoin::Some16 => b"2",
            Bitcoin::Some32 => b"2",
            Bitcoin::Some64 => b"2",
            Bitcoin::Some8 => b"2",
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
            Bitcoin::Xor16 => b"****22*22**22*22***22*22**22*22",
            Bitcoin::Xor32 => b"i",
            Bitcoin::Xor64 => b"l",
            Bitcoin::Xor8 => b"***22*22**22*22",
            Bitcoin::XorXor16 => b"****22*22**22*22***22*22**22*22",
            Bitcoin::XorXor32 => b"i",
            Bitcoin::XorXor64 => b"l",
            Bitcoin::XorXor8 => b"***22*22**22*22",
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
            Bitcoin::High8 => (45, 8),
            Bitcoin::High16 => (368, 11),
            Bitcoin::High32 => (369, 11),
            Bitcoin::High64 => (370, 11),
            Bitcoin::Complement8 => (389, 11),
            Bitcoin::Complement16 => (3120, 14),
            Bitcoin::Complement32 => (3121, 14),
            Bitcoin::Complement64 => (3122, 14),
            Bitcoin::And8 => (397, 11),
            Bitcoin::And16 => (3184, 14),
            Bitcoin::And32 => (3185, 14),
            Bitcoin::And64 => (3186, 14),
            Bitcoin::Or8 => (405, 11),
            Bitcoin::Or16 => (3248, 14),
            Bitcoin::Or32 => (3249, 14),
            Bitcoin::Or64 => (3250, 14),
            Bitcoin::Xor8 => (413, 11),
            Bitcoin::Xor16 => (3312, 14),
            Bitcoin::Xor32 => (3313, 14),
            Bitcoin::Xor64 => (3314, 14),
            Bitcoin::Maj8 => (837, 12),
            Bitcoin::Maj16 => (6704, 15),
            Bitcoin::Maj32 => (6705, 15),
            Bitcoin::Maj64 => (6706, 15),
            Bitcoin::XorXor8 => (845, 12),
            Bitcoin::XorXor16 => (6768, 15),
            Bitcoin::XorXor32 => (6769, 15),
            Bitcoin::XorXor64 => (6770, 15),
            Bitcoin::Ch8 => (853, 12),
            Bitcoin::Ch16 => (6832, 15),
            Bitcoin::Ch32 => (6833, 15),
            Bitcoin::Ch64 => (6834, 15),
            Bitcoin::Some8 => (861, 12),
            Bitcoin::Some16 => (6896, 15),
            Bitcoin::Some32 => (6897, 15),
            Bitcoin::Some64 => (6898, 15),
            Bitcoin::All8 => (869, 12),
            Bitcoin::All16 => (6960, 15),
            Bitcoin::All32 => (6961, 15),
            Bitcoin::All64 => (6962, 15),
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

    fn decode<I: Iterator<Item = u8>>(bits: &mut BitIter<I>) -> Result<Self, decode::Error> {
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
                            1 => {
                                0 => {},
                                1 => {
                                    0 => {
                                        0 => {},
                                        1 => {Bitcoin::High8}
                                    },
                                    1 => {
                                        0 => {
                                            0 => {
                                                0 => {
                                                    0 => {Bitcoin::High16},
                                                    1 => {Bitcoin::High32}
                                                },
                                                1 => {
                                                    0 => {Bitcoin::High64},
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
                                                    1 => {Bitcoin::Complement8}
                                                },
                                                1 => {
                                                    0 => {
                                                        0 => {
                                                            0 => {
                                                                0 => {Bitcoin::Complement16},
                                                                1 => {Bitcoin::Complement32}
                                                            },
                                                            1 => {
                                                                0 => {Bitcoin::Complement64},
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
                                                    1 => {Bitcoin::And8}
                                                },
                                                1 => {
                                                    0 => {
                                                        0 => {
                                                            0 => {
                                                                0 => {Bitcoin::And16},
                                                                1 => {Bitcoin::And32}
                                                            },
                                                            1 => {
                                                                0 => {Bitcoin::And64},
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
                                                    1 => {Bitcoin::Or8}
                                                },
                                                1 => {
                                                    0 => {
                                                        0 => {
                                                            0 => {
                                                                0 => {Bitcoin::Or16},
                                                                1 => {Bitcoin::Or32}
                                                            },
                                                            1 => {
                                                                0 => {Bitcoin::Or64},
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
                                                    1 => {Bitcoin::Xor8}
                                                },
                                                1 => {
                                                    0 => {
                                                        0 => {
                                                            0 => {
                                                                0 => {Bitcoin::Xor16},
                                                                1 => {Bitcoin::Xor32}
                                                            },
                                                            1 => {
                                                                0 => {Bitcoin::Xor64},
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
                                                        1 => {Bitcoin::Maj8}
                                                    },
                                                    1 => {
                                                        0 => {
                                                            0 => {
                                                                0 => {
                                                                    0 => {Bitcoin::Maj16},
                                                                    1 => {Bitcoin::Maj32}
                                                                },
                                                                1 => {
                                                                    0 => {Bitcoin::Maj64},
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
                                                        1 => {Bitcoin::XorXor8}
                                                    },
                                                    1 => {
                                                        0 => {
                                                            0 => {
                                                                0 => {
                                                                    0 => {Bitcoin::XorXor16},
                                                                    1 => {Bitcoin::XorXor32}
                                                                },
                                                                1 => {
                                                                    0 => {Bitcoin::XorXor64},
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
                                                        1 => {Bitcoin::Ch8}
                                                    },
                                                    1 => {
                                                        0 => {
                                                            0 => {
                                                                0 => {
                                                                    0 => {Bitcoin::Ch16},
                                                                    1 => {Bitcoin::Ch32}
                                                                },
                                                                1 => {
                                                                    0 => {Bitcoin::Ch64},
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
                                                        1 => {Bitcoin::Some8}
                                                    },
                                                    1 => {
                                                        0 => {
                                                            0 => {
                                                                0 => {
                                                                    0 => {Bitcoin::Some16},
                                                                    1 => {Bitcoin::Some32}
                                                                },
                                                                1 => {
                                                                    0 => {Bitcoin::Some64},
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
                                                        1 => {Bitcoin::All8}
                                                    },
                                                    1 => {
                                                        0 => {
                                                            0 => {
                                                                0 => {
                                                                    0 => {Bitcoin::All16},
                                                                    1 => {Bitcoin::All32}
                                                                },
                                                                1 => {
                                                                    0 => {Bitcoin::All64},
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
        unimplemented!("Bitcoin jets have not yet been implemented.")
    }

    fn cost(&self) -> Cost {
        unimplemented!("Unspecified cost of Bitcoin jets")
    }
}

impl fmt::Display for Bitcoin {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Bitcoin::Add16 => f.write_str("add_16"),
            Bitcoin::Add32 => f.write_str("add_32"),
            Bitcoin::Add64 => f.write_str("add_64"),
            Bitcoin::Add8 => f.write_str("add_8"),
            Bitcoin::All16 => f.write_str("all_16"),
            Bitcoin::All32 => f.write_str("all_32"),
            Bitcoin::All64 => f.write_str("all_64"),
            Bitcoin::All8 => f.write_str("all_8"),
            Bitcoin::And16 => f.write_str("and_16"),
            Bitcoin::And32 => f.write_str("and_32"),
            Bitcoin::And64 => f.write_str("and_64"),
            Bitcoin::And8 => f.write_str("and_8"),
            Bitcoin::Bip0340Verify => f.write_str("bip_0340_verify"),
            Bitcoin::Ch16 => f.write_str("ch_16"),
            Bitcoin::Ch32 => f.write_str("ch_32"),
            Bitcoin::Ch64 => f.write_str("ch_64"),
            Bitcoin::Ch8 => f.write_str("ch_8"),
            Bitcoin::CheckLockDistance => f.write_str("check_lock_distance"),
            Bitcoin::CheckLockDuration => f.write_str("check_lock_duration"),
            Bitcoin::CheckLockHeight => f.write_str("check_lock_height"),
            Bitcoin::CheckLockTime => f.write_str("check_lock_time"),
            Bitcoin::CheckSigVerify => f.write_str("check_sig_verify"),
            Bitcoin::Complement16 => f.write_str("complement_16"),
            Bitcoin::Complement32 => f.write_str("complement_32"),
            Bitcoin::Complement64 => f.write_str("complement_64"),
            Bitcoin::Complement8 => f.write_str("complement_8"),
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
            Bitcoin::High16 => f.write_str("high_16"),
            Bitcoin::High32 => f.write_str("high_32"),
            Bitcoin::High64 => f.write_str("high_64"),
            Bitcoin::High8 => f.write_str("high_8"),
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
            Bitcoin::Maj16 => f.write_str("maj_16"),
            Bitcoin::Maj32 => f.write_str("maj_32"),
            Bitcoin::Maj64 => f.write_str("maj_64"),
            Bitcoin::Maj8 => f.write_str("maj_8"),
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
            Bitcoin::Or16 => f.write_str("or_16"),
            Bitcoin::Or32 => f.write_str("or_32"),
            Bitcoin::Or64 => f.write_str("or_64"),
            Bitcoin::Or8 => f.write_str("or_8"),
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
            Bitcoin::Some16 => f.write_str("some_16"),
            Bitcoin::Some32 => f.write_str("some_32"),
            Bitcoin::Some64 => f.write_str("some_64"),
            Bitcoin::Some8 => f.write_str("some_8"),
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
            Bitcoin::Xor16 => f.write_str("xor_16"),
            Bitcoin::Xor32 => f.write_str("xor_32"),
            Bitcoin::Xor64 => f.write_str("xor_64"),
            Bitcoin::Xor8 => f.write_str("xor_8"),
            Bitcoin::XorXor16 => f.write_str("xor_xor_16"),
            Bitcoin::XorXor32 => f.write_str("xor_xor_32"),
            Bitcoin::XorXor64 => f.write_str("xor_xor_64"),
            Bitcoin::XorXor8 => f.write_str("xor_xor_8"),
        }
    }
}

impl str::FromStr for Bitcoin {
    type Err = crate::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "add_16" => Ok(Bitcoin::Add16),
            "add_32" => Ok(Bitcoin::Add32),
            "add_64" => Ok(Bitcoin::Add64),
            "add_8" => Ok(Bitcoin::Add8),
            "all_16" => Ok(Bitcoin::All16),
            "all_32" => Ok(Bitcoin::All32),
            "all_64" => Ok(Bitcoin::All64),
            "all_8" => Ok(Bitcoin::All8),
            "and_16" => Ok(Bitcoin::And16),
            "and_32" => Ok(Bitcoin::And32),
            "and_64" => Ok(Bitcoin::And64),
            "and_8" => Ok(Bitcoin::And8),
            "bip_0340_verify" => Ok(Bitcoin::Bip0340Verify),
            "ch_16" => Ok(Bitcoin::Ch16),
            "ch_32" => Ok(Bitcoin::Ch32),
            "ch_64" => Ok(Bitcoin::Ch64),
            "ch_8" => Ok(Bitcoin::Ch8),
            "check_lock_distance" => Ok(Bitcoin::CheckLockDistance),
            "check_lock_duration" => Ok(Bitcoin::CheckLockDuration),
            "check_lock_height" => Ok(Bitcoin::CheckLockHeight),
            "check_lock_time" => Ok(Bitcoin::CheckLockTime),
            "check_sig_verify" => Ok(Bitcoin::CheckSigVerify),
            "complement_16" => Ok(Bitcoin::Complement16),
            "complement_32" => Ok(Bitcoin::Complement32),
            "complement_64" => Ok(Bitcoin::Complement64),
            "complement_8" => Ok(Bitcoin::Complement8),
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
            "high_16" => Ok(Bitcoin::High16),
            "high_32" => Ok(Bitcoin::High32),
            "high_64" => Ok(Bitcoin::High64),
            "high_8" => Ok(Bitcoin::High8),
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
            "maj_16" => Ok(Bitcoin::Maj16),
            "maj_32" => Ok(Bitcoin::Maj32),
            "maj_64" => Ok(Bitcoin::Maj64),
            "maj_8" => Ok(Bitcoin::Maj8),
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
            "or_16" => Ok(Bitcoin::Or16),
            "or_32" => Ok(Bitcoin::Or32),
            "or_64" => Ok(Bitcoin::Or64),
            "or_8" => Ok(Bitcoin::Or8),
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
            "some_16" => Ok(Bitcoin::Some16),
            "some_32" => Ok(Bitcoin::Some32),
            "some_64" => Ok(Bitcoin::Some64),
            "some_8" => Ok(Bitcoin::Some8),
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
            "xor_16" => Ok(Bitcoin::Xor16),
            "xor_32" => Ok(Bitcoin::Xor32),
            "xor_64" => Ok(Bitcoin::Xor64),
            "xor_8" => Ok(Bitcoin::Xor8),
            "xor_xor_16" => Ok(Bitcoin::XorXor16),
            "xor_xor_32" => Ok(Bitcoin::XorXor32),
            "xor_xor_64" => Ok(Bitcoin::XorXor64),
            "xor_xor_8" => Ok(Bitcoin::XorXor8),
            x => Err(crate::Error::InvalidJetName(x.to_owned())),
        }
    }
}
