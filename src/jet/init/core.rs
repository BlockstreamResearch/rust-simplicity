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

/// Core jet family
#[derive(Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Debug, Hash)]
pub enum Core {
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
    CheckSigVerify,
    Complement16,
    Complement32,
    Complement64,
    Complement8,
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
    One16,
    One32,
    One64,
    One8,
    Or16,
    Or32,
    Or64,
    Or8,
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
    Verify,
    Xor16,
    Xor32,
    Xor64,
    Xor8,
    XorXor16,
    XorXor32,
    XorXor64,
    XorXor8,
}

impl Jet for Core {

    type Environment = ();
    type CJetEnvironment = ();

    fn c_jet_env<'env>(&self, env: &'env Self::Environment) -> &'env Self::CJetEnvironment {
        env
    }

    fn cmr(&self) -> Cmr {
        let bytes = match self {
            Core::Add16 => [
                0x55, 0xcf, 0x8c, 0xe6, 0x6e, 0x27, 0x5d, 0x01, 0xaf, 0xc1, 0xfb, 0x35, 0x10, 0xd0,
                0xb5, 0x3f, 0xab, 0x54, 0xa4, 0x78, 0x2a, 0x3a, 0x48, 0xdd, 0xac, 0x49, 0x4e, 0x96,
                0x72, 0xfa, 0x04, 0x5d,
            ],
            Core::Add32 => [
                0xfa, 0x87, 0x4e, 0x40, 0x62, 0x72, 0xc9, 0x0c, 0xf0, 0xb8, 0x7b, 0x1e, 0x7b, 0x89,
                0xf6, 0x61, 0xf3, 0x34, 0x1b, 0x0d, 0x61, 0x63, 0x5c, 0xf6, 0xf5, 0xa8, 0xa2, 0xa5,
                0x32, 0x88, 0xcf, 0xe4,
            ],
            Core::Add64 => [
                0x12, 0x56, 0xbc, 0x57, 0x26, 0xcd, 0xb9, 0x54, 0x58, 0x21, 0xd3, 0xd9, 0x6e, 0x7f,
                0xc7, 0x32, 0x86, 0xd7, 0xdb, 0x23, 0xae, 0x65, 0x17, 0x10, 0xff, 0x05, 0xfc, 0xc0,
                0xa9, 0xb7, 0xc3, 0xe9,
            ],
            Core::Add8 => [
                0xd9, 0x4b, 0x8f, 0xb6, 0xb6, 0x02, 0x5d, 0x8d, 0x02, 0x9d, 0xf1, 0xb0, 0xa5, 0x26,
                0xa4, 0x2b, 0x3e, 0xae, 0x94, 0xc8, 0xa9, 0x93, 0x6c, 0xe7, 0xe2, 0xa3, 0xb0, 0x00,
                0x2e, 0x7f, 0x44, 0x5d,
            ],
            Core::All16 => [
                0xc2, 0x62, 0x4c, 0x7f, 0x52, 0x9d, 0x7f, 0x8e, 0x7c, 0xb9, 0x69, 0x24, 0xdf, 0x5c,
                0x48, 0xda, 0x2b, 0xf2, 0xd9, 0x27, 0x96, 0x66, 0x2b, 0x0c, 0x76, 0x78, 0x21, 0xe1,
                0x9e, 0x7a, 0x36, 0x00,
            ],
            Core::All32 => [
                0x4c, 0x06, 0x0c, 0x8b, 0xd9, 0x70, 0x24, 0x41, 0x11, 0x74, 0x65, 0x60, 0xdd, 0x55,
                0xfb, 0xcd, 0x61, 0x04, 0xae, 0x34, 0x35, 0xaa, 0x43, 0x14, 0xe4, 0xac, 0x0e, 0x05,
                0x31, 0x20, 0x5e, 0x20,
            ],
            Core::All64 => [
                0xf5, 0x2f, 0x9f, 0xeb, 0xc7, 0x11, 0x3b, 0xb1, 0xca, 0x14, 0xe3, 0xd2, 0xff, 0x9c,
                0x16, 0x1a, 0x59, 0x8d, 0xe5, 0xaf, 0xe9, 0x28, 0xc6, 0x91, 0x35, 0x5b, 0x9e, 0x86,
                0xbe, 0x13, 0x12, 0x67,
            ],
            Core::All8 => [
                0xe5, 0x1e, 0x0e, 0x23, 0x93, 0x31, 0x5a, 0x99, 0xbf, 0xdb, 0x33, 0x58, 0x4a, 0x4e,
                0xb6, 0xcd, 0x6a, 0x3d, 0x03, 0xf1, 0xa7, 0x60, 0xea, 0xa0, 0x58, 0xb1, 0x4d, 0x33,
                0xd2, 0xd3, 0xf8, 0x2a,
            ],
            Core::And16 => [
                0x25, 0x17, 0x21, 0x39, 0x33, 0x59, 0x3c, 0x62, 0x72, 0xd8, 0xaa, 0x0c, 0x1d, 0x99,
                0x6a, 0x3a, 0x1c, 0x56, 0xf8, 0x0d, 0xd3, 0x5b, 0x05, 0x44, 0x96, 0xf9, 0xc6, 0x37,
                0xf6, 0x9f, 0xf9, 0x21,
            ],
            Core::And32 => [
                0x4f, 0x9d, 0x32, 0x6e, 0x9e, 0x51, 0x51, 0x52, 0x32, 0xf8, 0x47, 0x52, 0x33, 0xf7,
                0x50, 0xc0, 0xa5, 0x72, 0x12, 0x55, 0x80, 0x6d, 0x3d, 0x36, 0xb6, 0x3f, 0xfc, 0x14,
                0x99, 0x72, 0x51, 0x59,
            ],
            Core::And64 => [
                0xdf, 0x3a, 0x07, 0xb2, 0x40, 0xeb, 0x8c, 0x53, 0x33, 0xf7, 0xb0, 0x80, 0x3d, 0x81,
                0xc8, 0x75, 0x69, 0xef, 0x59, 0x97, 0xdb, 0x56, 0xac, 0x37, 0xad, 0x63, 0x05, 0x9f,
                0x0c, 0xc0, 0x77, 0x9c,
            ],
            Core::And8 => [
                0x14, 0x76, 0x9a, 0x23, 0xfc, 0xac, 0x95, 0x46, 0x22, 0x9d, 0x4c, 0x2e, 0xf3, 0xc5,
                0xd7, 0x79, 0xea, 0xf6, 0x24, 0x11, 0xfa, 0x4c, 0xa3, 0xa0, 0x6d, 0x60, 0xc4, 0x17,
                0xf2, 0x5b, 0x3e, 0x30,
            ],
            Core::Bip0340Verify => [
                0xaf, 0x92, 0x4c, 0xbe, 0xc9, 0x41, 0xfd, 0x9b, 0x9e, 0x82, 0x93, 0xef, 0x90, 0x3e,
                0x23, 0x7b, 0x01, 0xdb, 0xcb, 0x34, 0x2b, 0x96, 0x10, 0xbe, 0xf9, 0x5f, 0x59, 0xb8,
                0x13, 0x6e, 0xf8, 0x04,
            ],
            Core::Ch16 => [
                0x72, 0x3b, 0x96, 0x0c, 0x67, 0x34, 0xa9, 0x88, 0x32, 0x29, 0x88, 0x63, 0x28, 0x48,
                0x5d, 0xb7, 0x18, 0x9d, 0x0b, 0x3c, 0xec, 0x24, 0xc8, 0xa0, 0xb2, 0x88, 0xa4, 0x59,
                0x78, 0x67, 0xbb, 0x9c,
            ],
            Core::Ch32 => [
                0xf1, 0x47, 0x82, 0xaf, 0x48, 0xdb, 0x5e, 0xe8, 0x42, 0x4d, 0xf8, 0x4d, 0xb5, 0x5e,
                0xcd, 0x83, 0x42, 0x0e, 0x4a, 0xde, 0x1e, 0x43, 0x11, 0x9a, 0xcb, 0x0a, 0x06, 0x5f,
                0x86, 0x67, 0x50, 0x7d,
            ],
            Core::Ch64 => [
                0xb1, 0x80, 0x0e, 0xac, 0x70, 0x03, 0x8b, 0xdc, 0x3e, 0xe9, 0xdd, 0x14, 0x75, 0xc1,
                0x1f, 0x5a, 0x3b, 0x40, 0xb3, 0xc4, 0x0a, 0xd6, 0xc8, 0xe6, 0x67, 0x48, 0x77, 0x43,
                0x09, 0xc2, 0xc2, 0xac,
            ],
            Core::Ch8 => [
                0x5b, 0x0b, 0x12, 0x35, 0x72, 0x0b, 0xbf, 0x0b, 0x66, 0x6f, 0x35, 0xe6, 0x78, 0x3a,
                0x6a, 0xb6, 0xb7, 0x84, 0x8c, 0x9d, 0xa4, 0x79, 0x44, 0xbc, 0x91, 0x4f, 0x05, 0x60,
                0x38, 0x95, 0x1c, 0xf8,
            ],
            Core::CheckSigVerify => [
                0x29, 0x74, 0x59, 0xd8, 0x37, 0x3f, 0x87, 0x27, 0xda, 0x9f, 0x92, 0xda, 0xea, 0xb7,
                0x19, 0x6b, 0xed, 0x0b, 0xd5, 0x23, 0x64, 0x40, 0x55, 0x21, 0x44, 0x55, 0x72, 0x3a,
                0xba, 0xc0, 0xb0, 0x85,
            ],
            Core::Complement16 => [
                0x9d, 0x56, 0x15, 0xf9, 0x07, 0x22, 0xd4, 0xae, 0xfb, 0xd8, 0xdd, 0x09, 0xc6, 0x82,
                0x88, 0x43, 0x68, 0xa8, 0x8f, 0x47, 0x11, 0xf6, 0x2c, 0xfe, 0x02, 0x9c, 0x9f, 0x07,
                0x29, 0xa2, 0x73, 0x44,
            ],
            Core::Complement32 => [
                0x43, 0x60, 0xc5, 0xd2, 0xa4, 0x57, 0xd4, 0x43, 0x21, 0x9e, 0x02, 0x67, 0x74, 0xe6,
                0x8d, 0xee, 0x23, 0xef, 0x6e, 0xc7, 0xa9, 0x13, 0x72, 0x18, 0x6c, 0x9d, 0x46, 0x03,
                0x94, 0xc1, 0x51, 0x9b,
            ],
            Core::Complement64 => [
                0x24, 0x13, 0x58, 0x82, 0xf7, 0x56, 0x03, 0xd0, 0x69, 0x81, 0x34, 0x90, 0x2e, 0xb5,
                0x0e, 0x14, 0xa9, 0xa4, 0xa7, 0x9f, 0x48, 0xe7, 0x77, 0xa6, 0x97, 0xfc, 0x02, 0xd6,
                0x4c, 0x15, 0x65, 0x89,
            ],
            Core::Complement8 => [
                0x85, 0x8b, 0x61, 0xb0, 0x7e, 0x3d, 0x4e, 0xf1, 0x43, 0x4e, 0xc4, 0xf4, 0xae, 0xc3,
                0x0d, 0x29, 0x59, 0x9b, 0x8a, 0xc3, 0x83, 0x70, 0xe5, 0x8a, 0xfa, 0x47, 0x7b, 0x1e,
                0xd3, 0x9d, 0x6b, 0x47,
            ],
            Core::Decompress => [
                0x59, 0x85, 0xb0, 0x05, 0xf2, 0x01, 0x34, 0x2d, 0x90, 0x97, 0x49, 0x27, 0x30, 0x25,
                0x7d, 0x68, 0x3e, 0xe5, 0x4f, 0x0d, 0x6b, 0xfb, 0x68, 0xfa, 0xe1, 0x82, 0x1d, 0x6a,
                0x01, 0xfe, 0xa2, 0x8c,
            ],
            Core::Decrement16 => [
                0x88, 0x73, 0xfb, 0x6a, 0x44, 0x7d, 0xe2, 0x86, 0xcd, 0x6d, 0x64, 0xb4, 0x92, 0xd8,
                0xa1, 0xd3, 0xd7, 0xcd, 0xdf, 0xbb, 0xe6, 0x75, 0xbf, 0xf5, 0xc5, 0x6e, 0xba, 0x40,
                0xf6, 0x90, 0xdb, 0xd3,
            ],
            Core::Decrement32 => [
                0xca, 0x2b, 0xc9, 0xbd, 0x09, 0xc3, 0x61, 0x30, 0xc5, 0x2e, 0x4c, 0xfa, 0x7d, 0xf6,
                0xd1, 0x08, 0x30, 0x61, 0x66, 0x04, 0x19, 0x16, 0x3d, 0x0a, 0x96, 0xb6, 0x69, 0xa0,
                0x73, 0x96, 0x6a, 0xdd,
            ],
            Core::Decrement64 => [
                0xdb, 0xc8, 0xe7, 0x57, 0xbc, 0x06, 0x96, 0x24, 0xb0, 0x27, 0xa9, 0xf7, 0x6f, 0x0a,
                0xe7, 0x9f, 0xa2, 0x0e, 0x48, 0xb8, 0x91, 0x0f, 0x7a, 0xb4, 0xa2, 0xb1, 0x62, 0x27,
                0xd2, 0xb4, 0xcc, 0xac,
            ],
            Core::Decrement8 => [
                0xd6, 0xec, 0x5f, 0xba, 0x81, 0xdc, 0xbd, 0xd4, 0x4a, 0x1b, 0x3a, 0xb1, 0x94, 0xad,
                0xd7, 0xf5, 0x42, 0xb1, 0x70, 0xca, 0xde, 0xb4, 0x65, 0xdd, 0x2f, 0x31, 0x92, 0x84,
                0x58, 0x1a, 0x76, 0xd6,
            ],
            Core::DivMod16 => [
                0x66, 0xee, 0x6d, 0x63, 0x76, 0x96, 0x76, 0x0e, 0xf0, 0x90, 0x3a, 0x4e, 0xe9, 0xda,
                0x77, 0x15, 0xcb, 0xf9, 0x04, 0xaa, 0x9f, 0x59, 0xd8, 0x4e, 0xcc, 0xb3, 0x5f, 0x90,
                0x47, 0x5d, 0x37, 0xbe,
            ],
            Core::DivMod32 => [
                0x6c, 0x6a, 0xf7, 0x62, 0x0b, 0x1d, 0x71, 0x8b, 0x1e, 0x09, 0x13, 0x30, 0x72, 0xb2,
                0xf2, 0x04, 0x35, 0x8d, 0xbe, 0x58, 0x94, 0xd3, 0xa2, 0xc6, 0x18, 0xc2, 0x59, 0x3e,
                0x62, 0xf0, 0x06, 0xcc,
            ],
            Core::DivMod64 => [
                0x24, 0x39, 0x58, 0x0d, 0xda, 0x93, 0x4d, 0x44, 0x98, 0xfc, 0x41, 0x77, 0x4f, 0x29,
                0x4d, 0xe5, 0x93, 0xf4, 0x4c, 0x63, 0x2f, 0x84, 0xe7, 0x21, 0xa8, 0xd5, 0xbb, 0xde,
                0x34, 0x5c, 0xce, 0x40,
            ],
            Core::DivMod8 => [
                0x06, 0xfb, 0x2a, 0x19, 0x7c, 0x16, 0x1d, 0x0a, 0x77, 0xfc, 0x72, 0x6d, 0x3b, 0xaa,
                0x85, 0x7e, 0x27, 0xf8, 0x69, 0x1b, 0xcc, 0xe2, 0xb4, 0x73, 0xd4, 0xd1, 0x40, 0x8c,
                0x17, 0xe0, 0x32, 0x1d,
            ],
            Core::Divide16 => [
                0x70, 0x1e, 0xf0, 0x5d, 0x05, 0xfe, 0x59, 0x43, 0x54, 0xb2, 0xef, 0x79, 0x6f, 0x4c,
                0xa4, 0x52, 0x76, 0xda, 0xac, 0x15, 0x25, 0x92, 0x76, 0x3e, 0x2a, 0x1f, 0x6e, 0x19,
                0x67, 0x25, 0x76, 0xbc,
            ],
            Core::Divide32 => [
                0xf8, 0x83, 0xb1, 0xc6, 0x4d, 0x04, 0x40, 0x31, 0xd9, 0x9e, 0x04, 0x67, 0x42, 0x54,
                0xfe, 0x2b, 0xb3, 0x44, 0xbc, 0xdd, 0x54, 0x38, 0xd6, 0xea, 0x01, 0x45, 0x8d, 0x9e,
                0x24, 0x97, 0x70, 0x2a,
            ],
            Core::Divide64 => [
                0xaa, 0x84, 0x4c, 0xfe, 0x9a, 0x83, 0xc5, 0x26, 0xd5, 0xfb, 0x05, 0x82, 0x0d, 0x1a,
                0x6f, 0x95, 0x3e, 0xb5, 0x0c, 0x85, 0x6c, 0xda, 0x50, 0x7d, 0xb9, 0xbe, 0x5b, 0x7c,
                0x90, 0xab, 0x2d, 0x14,
            ],
            Core::Divide8 => [
                0xbb, 0x39, 0x21, 0x8f, 0x0b, 0x2d, 0x69, 0x30, 0xa6, 0xe4, 0x17, 0x66, 0xfd, 0x52,
                0x18, 0xfa, 0xd1, 0x81, 0x0d, 0x17, 0x6a, 0xba, 0x80, 0x1a, 0x26, 0x38, 0x0d, 0x35,
                0x1a, 0x2d, 0x79, 0xb1,
            ],
            Core::Divides16 => [
                0x0e, 0x98, 0xa5, 0x3a, 0x62, 0x66, 0xf4, 0xc6, 0x0b, 0xdc, 0xaf, 0x62, 0x0d, 0xbc,
                0xa7, 0x6d, 0xb3, 0xcc, 0x1b, 0x59, 0x8a, 0x9c, 0x66, 0x7a, 0xd1, 0xda, 0x86, 0xb5,
                0xc9, 0xc0, 0xe9, 0x12,
            ],
            Core::Divides32 => [
                0x6e, 0xd2, 0x1a, 0xbf, 0x02, 0x2c, 0x2b, 0xa0, 0x20, 0xe9, 0x1f, 0x0d, 0x09, 0x59,
                0x3b, 0xa5, 0x5f, 0x74, 0x26, 0x26, 0x29, 0x14, 0xe9, 0x55, 0xe1, 0x79, 0x1a, 0x58,
                0x3b, 0x31, 0xc0, 0x7d,
            ],
            Core::Divides64 => [
                0x39, 0x71, 0xea, 0x23, 0x01, 0xba, 0xbb, 0x7b, 0xb5, 0x4f, 0x5f, 0x48, 0x33, 0xc6,
                0x3c, 0xaa, 0x52, 0x79, 0xd4, 0xc2, 0xb6, 0x54, 0x59, 0x2f, 0xbb, 0x50, 0x6e, 0xb2,
                0x03, 0xb0, 0x06, 0x15,
            ],
            Core::Divides8 => [
                0x77, 0x63, 0x5c, 0x01, 0x95, 0x71, 0x9b, 0x99, 0x0f, 0xeb, 0x14, 0x31, 0xb1, 0x24,
                0x4e, 0xb7, 0x55, 0xae, 0xe9, 0xba, 0xce, 0xe4, 0xb2, 0x0c, 0x12, 0xbe, 0x8e, 0x23,
                0x9d, 0x12, 0xf7, 0xdc,
            ],
            Core::Eq16 => [
                0x08, 0x1d, 0x4c, 0x58, 0x0a, 0x51, 0x56, 0x85, 0xa7, 0x00, 0xcb, 0x47, 0xa5, 0x43,
                0x33, 0x04, 0x49, 0x0a, 0xdd, 0x10, 0x1c, 0xbc, 0x07, 0xc3, 0xc4, 0x6e, 0xfb, 0x19,
                0x6b, 0x75, 0xfe, 0x55,
            ],
            Core::Eq256 => [
                0x56, 0x83, 0x53, 0x14, 0x63, 0x05, 0x0b, 0x4f, 0x99, 0x26, 0xd0, 0xe8, 0xff, 0x64,
                0x26, 0x3a, 0x15, 0xce, 0xb1, 0xce, 0xa6, 0x54, 0x12, 0x63, 0x7c, 0x23, 0x5f, 0x46,
                0xd7, 0xbe, 0x66, 0x05,
            ],
            Core::Eq32 => [
                0x73, 0x87, 0xd2, 0x79, 0x8a, 0x02, 0x10, 0x7e, 0x2c, 0x57, 0x36, 0x71, 0x27, 0xf3,
                0xf3, 0x6f, 0xc9, 0xf1, 0x97, 0x0d, 0xaf, 0xba, 0x80, 0xf9, 0x34, 0x0c, 0x16, 0x8b,
                0x50, 0xf7, 0xa4, 0xe2,
            ],
            Core::Eq64 => [
                0x05, 0x72, 0xe1, 0x6b, 0xab, 0x59, 0xd8, 0x47, 0x5f, 0xda, 0x6b, 0xfc, 0xbd, 0x49,
                0x6f, 0xe7, 0x53, 0x41, 0x07, 0xf6, 0x29, 0x1d, 0xd7, 0xee, 0xdb, 0xfb, 0xf2, 0x8b,
                0x76, 0x0a, 0xa4, 0x1a,
            ],
            Core::Eq8 => [
                0xd4, 0x25, 0x79, 0xe4, 0xe7, 0xaf, 0x6d, 0xaa, 0x39, 0xa5, 0x4e, 0x54, 0x27, 0x47,
                0xd4, 0x19, 0x5c, 0x2a, 0xb4, 0x8c, 0x1f, 0xdd, 0x7b, 0x87, 0x32, 0xe7, 0xa5, 0x2f,
                0xed, 0x40, 0x75, 0x99,
            ],
            Core::FeAdd => [
                0xdf, 0x5c, 0x03, 0x4b, 0x64, 0x59, 0xde, 0xd2, 0xf2, 0x07, 0xb5, 0xe4, 0x0f, 0x02,
                0xe5, 0x08, 0x76, 0xee, 0x6b, 0x25, 0x0c, 0xdd, 0x8c, 0x94, 0x9f, 0xbb, 0x78, 0x3b,
                0x5e, 0x90, 0x44, 0xda,
            ],
            Core::FeInvert => [
                0xc1, 0x22, 0x09, 0x8f, 0x95, 0x59, 0x4b, 0x72, 0x1d, 0x3a, 0x51, 0xa3, 0x1d, 0xc1,
                0xb2, 0xc4, 0x0d, 0xec, 0xcb, 0x9b, 0x3c, 0xdc, 0x9a, 0xb9, 0x0e, 0x56, 0x1a, 0xc2,
                0x13, 0x41, 0xbc, 0xe2,
            ],
            Core::FeIsOdd => [
                0xaf, 0xa5, 0x6c, 0x6b, 0x60, 0xc5, 0x2a, 0x39, 0xe5, 0xa1, 0x61, 0x0e, 0xfb, 0x8e,
                0x1a, 0xe5, 0x6f, 0x72, 0x03, 0x72, 0x01, 0xca, 0xc6, 0x04, 0x37, 0xb6, 0x75, 0x8a,
                0x74, 0x46, 0x9b, 0x2a,
            ],
            Core::FeIsZero => [
                0x51, 0x1a, 0xc5, 0xb8, 0x35, 0x9c, 0x20, 0x02, 0x08, 0x0a, 0x16, 0xa1, 0x18, 0x40,
                0xab, 0x1a, 0xe1, 0x5b, 0x60, 0x15, 0x73, 0x72, 0x67, 0x7b, 0x39, 0xa9, 0x2f, 0xd7,
                0xcf, 0x3e, 0x8e, 0xb3,
            ],
            Core::FeMultiply => [
                0x4d, 0xb0, 0xb0, 0xf2, 0xa2, 0xa8, 0x21, 0x38, 0x7a, 0x7d, 0xab, 0x9c, 0x5c, 0x0c,
                0xe8, 0xeb, 0xde, 0x14, 0xd0, 0x39, 0x25, 0xb5, 0x6d, 0x7f, 0x30, 0x9a, 0x5e, 0x2d,
                0x09, 0x9b, 0xf9, 0xab,
            ],
            Core::FeMultiplyBeta => [
                0x45, 0x32, 0xc7, 0x30, 0x5f, 0x04, 0xbb, 0x13, 0x0f, 0xef, 0x0c, 0x4e, 0x16, 0x7b,
                0x9a, 0xc0, 0xcc, 0x07, 0xbc, 0xfc, 0x66, 0xdd, 0x2d, 0x0c, 0x67, 0x8e, 0xd9, 0x13,
                0xae, 0xe2, 0x1d, 0x64,
            ],
            Core::FeNegate => [
                0x3b, 0x1c, 0x7d, 0x40, 0x36, 0x1c, 0x5b, 0x03, 0x73, 0x77, 0x1e, 0xab, 0xb0, 0x66,
                0x28, 0x9f, 0x29, 0x1f, 0x76, 0x65, 0x25, 0xeb, 0x25, 0xab, 0x42, 0xd1, 0x8d, 0xfa,
                0x21, 0x96, 0xda, 0x40,
            ],
            Core::FeNormalize => [
                0xe0, 0x89, 0xae, 0x03, 0x97, 0x27, 0x38, 0xe5, 0x8c, 0xad, 0x4f, 0x74, 0x26, 0xe2,
                0x63, 0xfa, 0x5e, 0x2c, 0x6c, 0x4b, 0x58, 0x69, 0xef, 0x96, 0x9b, 0xec, 0x86, 0xfc,
                0xb7, 0xad, 0x32, 0x05,
            ],
            Core::FeSquare => [
                0x1f, 0x1f, 0x57, 0xf8, 0x8b, 0x28, 0x73, 0xf4, 0xdf, 0x41, 0x63, 0xa6, 0x8f, 0x53,
                0x98, 0xb1, 0x68, 0x2a, 0x91, 0x1b, 0xeb, 0x4d, 0x6d, 0x24, 0xe8, 0xca, 0x27, 0x1e,
                0x35, 0xba, 0xe8, 0xdb,
            ],
            Core::FeSquareRoot => [
                0xb0, 0xea, 0x63, 0x8c, 0xf2, 0x50, 0xe2, 0xa2, 0x8d, 0xcf, 0xc8, 0xfc, 0x04, 0x09,
                0x40, 0x13, 0x10, 0x21, 0x81, 0xf8, 0x16, 0xfe, 0xfb, 0xc6, 0xcb, 0x45, 0x1b, 0xe7,
                0x1e, 0xe8, 0xa8, 0x05,
            ],
            Core::FullAdd16 => [
                0x67, 0xc5, 0x7c, 0x82, 0x46, 0x18, 0x60, 0xc0, 0x22, 0x3a, 0x42, 0x7d, 0xde, 0x99,
                0xe5, 0x12, 0x75, 0x3b, 0x0e, 0xc5, 0x2a, 0x05, 0xdc, 0xbe, 0x71, 0x8e, 0xf3, 0x0a,
                0x26, 0x84, 0xb6, 0x81,
            ],
            Core::FullAdd32 => [
                0x73, 0x9f, 0x5b, 0x1e, 0x9b, 0x40, 0x8b, 0x36, 0x3c, 0xda, 0xd6, 0xbf, 0x00, 0xa8,
                0xbf, 0xf5, 0x22, 0xb3, 0xd5, 0x38, 0x02, 0xea, 0xad, 0xd8, 0x94, 0xa0, 0x10, 0x7d,
                0x20, 0x3a, 0x46, 0x00,
            ],
            Core::FullAdd64 => [
                0x70, 0xe1, 0xee, 0xa5, 0x3b, 0xc9, 0xb0, 0x1b, 0x06, 0x76, 0xb5, 0xea, 0x97, 0x36,
                0x74, 0xd6, 0xf3, 0xc3, 0x56, 0x74, 0x32, 0xf2, 0x64, 0xbc, 0x60, 0xb7, 0xae, 0x1f,
                0xf6, 0x9a, 0x40, 0x2a,
            ],
            Core::FullAdd8 => [
                0x37, 0xff, 0x9d, 0x08, 0x4d, 0xd0, 0x51, 0x7f, 0xb3, 0x89, 0x7d, 0xe0, 0x86, 0x38,
                0x1a, 0x9c, 0x2c, 0x72, 0x44, 0x0e, 0xe1, 0xba, 0xc3, 0x8d, 0xb7, 0xf4, 0xf7, 0x4a,
                0x50, 0x1d, 0x54, 0xdd,
            ],
            Core::FullDecrement16 => [
                0x61, 0xe2, 0xe8, 0x5a, 0xb8, 0xc7, 0xe4, 0xc2, 0xe9, 0xdc, 0x6e, 0xb2, 0xf0, 0x60,
                0x74, 0x9a, 0x03, 0x35, 0x41, 0x2f, 0x9f, 0x3a, 0xc8, 0xfd, 0x68, 0xf2, 0x56, 0x15,
                0xff, 0x06, 0x7e, 0x61,
            ],
            Core::FullDecrement32 => [
                0x8b, 0xe3, 0x11, 0x0b, 0xda, 0xe8, 0xd6, 0x5f, 0x08, 0x33, 0x19, 0x95, 0xf5, 0x84,
                0xbf, 0x36, 0x02, 0x36, 0xa1, 0x4c, 0x61, 0xad, 0x2f, 0xa6, 0xe7, 0xae, 0xac, 0x60,
                0x9b, 0xd4, 0xdb, 0xfa,
            ],
            Core::FullDecrement64 => [
                0xd0, 0x03, 0x87, 0x12, 0x19, 0x99, 0x14, 0x9c, 0x3a, 0x7b, 0xd2, 0x56, 0xce, 0x8b,
                0xd0, 0x2d, 0x71, 0x9f, 0x3d, 0x5c, 0x58, 0xf1, 0x0a, 0xb6, 0x75, 0x7c, 0xc0, 0x23,
                0xd5, 0x5d, 0xbe, 0x1b,
            ],
            Core::FullDecrement8 => [
                0xe8, 0xcc, 0x36, 0x06, 0xb1, 0xdd, 0x7f, 0xcf, 0x64, 0xc9, 0x91, 0xb0, 0xce, 0xb9,
                0xc6, 0x14, 0x6c, 0xcc, 0xd0, 0x89, 0x49, 0x89, 0xd8, 0xd9, 0xab, 0x97, 0xd2, 0x74,
                0x3e, 0xa1, 0xa5, 0xac,
            ],
            Core::FullIncrement16 => [
                0xce, 0xaf, 0x13, 0xe7, 0x8e, 0x3a, 0x30, 0x75, 0xd1, 0x17, 0x48, 0x0c, 0x7b, 0x72,
                0xc2, 0xba, 0xd4, 0x54, 0x8c, 0xcc, 0xfa, 0xf4, 0x8e, 0xa1, 0xeb, 0x67, 0x7f, 0x78,
                0x0e, 0x5c, 0x00, 0x30,
            ],
            Core::FullIncrement32 => [
                0x7e, 0x83, 0x12, 0x88, 0x87, 0x14, 0x66, 0x43, 0x44, 0x5a, 0xf4, 0xca, 0xbb, 0x1d,
                0xa5, 0x0e, 0x82, 0xda, 0xeb, 0x34, 0xb7, 0xd9, 0xa2, 0x42, 0xcd, 0x00, 0xde, 0x5f,
                0xbb, 0xea, 0x51, 0x82,
            ],
            Core::FullIncrement64 => [
                0x93, 0xac, 0xb5, 0x83, 0x1a, 0x59, 0xb2, 0x8a, 0xe4, 0x38, 0x87, 0x6e, 0x31, 0x02,
                0xf9, 0x19, 0x42, 0x7b, 0x82, 0x72, 0x63, 0x0a, 0x8e, 0xd7, 0x85, 0x92, 0xef, 0x54,
                0x3c, 0xfb, 0x65, 0xf6,
            ],
            Core::FullIncrement8 => [
                0x92, 0x1a, 0xaa, 0xa8, 0xca, 0xc8, 0x20, 0x63, 0xfb, 0xfa, 0xc0, 0x51, 0x17, 0xe4,
                0xa9, 0x3e, 0xb2, 0xf0, 0xb9, 0x2d, 0x83, 0x56, 0x21, 0x99, 0xeb, 0xdb, 0x8a, 0xfc,
                0x16, 0xb1, 0xd5, 0x2c,
            ],
            Core::FullMultiply16 => [
                0x15, 0x97, 0x38, 0x6d, 0xa7, 0xf0, 0xd5, 0x56, 0x99, 0x66, 0xd1, 0x71, 0x76, 0xea,
                0xe3, 0x78, 0x1b, 0xec, 0x8f, 0xb4, 0xfd, 0xe5, 0xc8, 0xc7, 0x12, 0xda, 0xea, 0xa8,
                0x6b, 0xd8, 0x3f, 0x37,
            ],
            Core::FullMultiply32 => [
                0xe8, 0xb8, 0x81, 0xa1, 0x26, 0xe2, 0xc0, 0x4e, 0xbd, 0x8d, 0xc7, 0x8d, 0x8e, 0x24,
                0xfd, 0x92, 0xcc, 0x82, 0xba, 0x0c, 0x41, 0xe1, 0xa4, 0x03, 0xe6, 0x43, 0x13, 0x34,
                0xd1, 0x3c, 0x4e, 0xc2,
            ],
            Core::FullMultiply64 => [
                0xdc, 0xcc, 0x7e, 0x3c, 0x39, 0x7c, 0x15, 0x0a, 0xfd, 0xf3, 0x83, 0xbe, 0xd8, 0x9e,
                0x95, 0xe5, 0x05, 0x94, 0xb2, 0x85, 0xb8, 0xc7, 0xd6, 0x24, 0xd1, 0xb9, 0xe0, 0x59,
                0xe9, 0xf7, 0x23, 0x0a,
            ],
            Core::FullMultiply8 => [
                0x20, 0x18, 0x33, 0x05, 0x30, 0x45, 0x46, 0x19, 0xc6, 0x4f, 0x5a, 0x10, 0x9e, 0x7b,
                0x3c, 0xce, 0x81, 0x79, 0x79, 0xdb, 0x53, 0x52, 0x18, 0x9e, 0x31, 0x97, 0xb0, 0x51,
                0x99, 0xc7, 0x4d, 0xe9,
            ],
            Core::FullSubtract16 => [
                0x09, 0x19, 0xce, 0x69, 0x18, 0x98, 0xd2, 0x7c, 0x6f, 0xc4, 0x4c, 0x13, 0x70, 0x64,
                0x52, 0x4e, 0x41, 0x31, 0x46, 0xd1, 0xce, 0x20, 0x17, 0x32, 0xc9, 0x75, 0xb0, 0xd5,
                0x47, 0xef, 0xfa, 0x7c,
            ],
            Core::FullSubtract32 => [
                0x81, 0x0b, 0xd2, 0x82, 0x67, 0x28, 0x23, 0x55, 0xee, 0x69, 0x48, 0x20, 0x71, 0x3e,
                0x68, 0xd0, 0x67, 0xbb, 0xd8, 0xb8, 0x1e, 0xd9, 0x13, 0x8c, 0x33, 0xe5, 0x16, 0x49,
                0x73, 0xa5, 0x62, 0x84,
            ],
            Core::FullSubtract64 => [
                0xc8, 0x67, 0x88, 0x5f, 0xb9, 0x15, 0x4f, 0x6e, 0x4a, 0x68, 0xb8, 0x16, 0x4e, 0xca,
                0xe1, 0xd6, 0x9f, 0xeb, 0x4f, 0x4d, 0xc6, 0xc2, 0xe1, 0xe9, 0x72, 0xaf, 0x8a, 0xf8,
                0xc6, 0x33, 0x47, 0xf1,
            ],
            Core::FullSubtract8 => [
                0xb9, 0xf1, 0xb9, 0xb8, 0xd1, 0x0f, 0xab, 0xbb, 0xb7, 0xdf, 0x46, 0xc8, 0x75, 0x86,
                0x2f, 0x7e, 0x7e, 0x70, 0x14, 0x08, 0x85, 0x60, 0xb0, 0xda, 0x45, 0x48, 0x0a, 0x6c,
                0x7c, 0x12, 0x59, 0xa2,
            ],
            Core::GeIsOnCurve => [
                0x36, 0xd0, 0x5a, 0x72, 0x9a, 0x79, 0xb0, 0x37, 0xb1, 0x0b, 0x3a, 0xb2, 0xb2, 0xc2,
                0xbf, 0xea, 0x03, 0xa0, 0x1a, 0x18, 0xe0, 0xea, 0xf6, 0x5c, 0x6f, 0x9b, 0xc7, 0x45,
                0xc7, 0xc2, 0xe6, 0xfa,
            ],
            Core::GeNegate => [
                0x52, 0x01, 0xd4, 0x5c, 0xc1, 0xde, 0xb3, 0xe2, 0xb9, 0x49, 0xfe, 0x66, 0x0a, 0x07,
                0xc5, 0xce, 0x69, 0x44, 0x3d, 0xbe, 0xc6, 0x7c, 0xc7, 0x6b, 0x24, 0x25, 0xff, 0x32,
                0xe8, 0x52, 0xe5, 0xb4,
            ],
            Core::GejAdd => [
                0x28, 0x54, 0x85, 0xc4, 0x70, 0x84, 0x49, 0x25, 0x10, 0x37, 0x3d, 0xf4, 0x3d, 0xf5,
                0x34, 0x07, 0xac, 0xec, 0x8f, 0xb1, 0xbd, 0x01, 0x03, 0x80, 0x89, 0x7b, 0x51, 0x7c,
                0x39, 0xcd, 0x63, 0x19,
            ],
            Core::GejDouble => [
                0x71, 0x07, 0x74, 0x58, 0x57, 0x75, 0xf9, 0x1f, 0x4c, 0xe5, 0x78, 0xad, 0x8d, 0x1e,
                0x64, 0x45, 0x41, 0xe2, 0x1f, 0xc6, 0xc8, 0x10, 0xab, 0xdb, 0x3b, 0x3e, 0xd2, 0x11,
                0x5e, 0x39, 0xcd, 0xae,
            ],
            Core::GejGeAdd => [
                0x7d, 0x7f, 0x42, 0x6e, 0x42, 0x45, 0x8e, 0x45, 0x77, 0x12, 0x91, 0xcc, 0x9e, 0x60,
                0x7e, 0x67, 0x26, 0x7a, 0x38, 0x85, 0xad, 0xbe, 0xbd, 0xc3, 0x69, 0xdf, 0x59, 0x66,
                0x32, 0x20, 0xbe, 0xfb,
            ],
            Core::GejGeAddEx => [
                0xcd, 0xda, 0xe7, 0x8d, 0x33, 0xa2, 0x21, 0x28, 0xbc, 0x2f, 0x72, 0xa6, 0x02, 0xe0,
                0x06, 0x6f, 0x63, 0xfe, 0x18, 0x62, 0x57, 0xea, 0x34, 0x8c, 0x2b, 0xb1, 0xf7, 0xe9,
                0xbf, 0x9b, 0x0d, 0x73,
            ],
            Core::GejInfinity => [
                0x95, 0xbd, 0x3c, 0xe9, 0x3f, 0x0c, 0x8e, 0x95, 0xaa, 0x2e, 0x60, 0xa9, 0xe8, 0x26,
                0x57, 0xfd, 0x98, 0x0f, 0x3f, 0x27, 0x78, 0x11, 0xfb, 0x6d, 0x39, 0xd1, 0xff, 0x12,
                0x1f, 0x3f, 0x8a, 0x14,
            ],
            Core::GejIsInfinity => [
                0x63, 0x58, 0xd3, 0xdd, 0xc7, 0xab, 0x52, 0xfd, 0x6a, 0xd6, 0x36, 0xad, 0xf9, 0xb9,
                0xf3, 0x7e, 0xaf, 0x79, 0x6f, 0x89, 0xb2, 0xd9, 0xbf, 0xba, 0x97, 0xab, 0xee, 0x3f,
                0x32, 0x7b, 0x30, 0xa4,
            ],
            Core::GejIsOnCurve => [
                0x6b, 0xcc, 0x65, 0xcf, 0xed, 0x04, 0x39, 0xf1, 0x11, 0xee, 0x5f, 0x5b, 0x5b, 0x47,
                0x91, 0xe1, 0xe4, 0xad, 0x7f, 0xf3, 0x69, 0x51, 0xa2, 0x33, 0x1b, 0x18, 0xf9, 0x7f,
                0x7a, 0xf9, 0x13, 0x98,
            ],
            Core::GejNegate => [
                0x0a, 0xe3, 0x66, 0x32, 0x36, 0x47, 0x83, 0x1a, 0x3f, 0x1c, 0x8e, 0x51, 0xf1, 0xf6,
                0xc1, 0x7e, 0xec, 0x93, 0xcf, 0x53, 0x95, 0xa2, 0x76, 0x44, 0x0a, 0x24, 0x6d, 0xeb,
                0xb2, 0xef, 0xc3, 0x91,
            ],
            Core::GejNormalize => [
                0xcb, 0x5a, 0x52, 0xa3, 0x24, 0x29, 0x5b, 0xda, 0xd4, 0x2d, 0x0f, 0xb0, 0x1f, 0x67,
                0xaa, 0xdf, 0x6e, 0x10, 0xe0, 0xb3, 0xd1, 0x8c, 0x93, 0x24, 0xc3, 0xa1, 0x2a, 0x05,
                0xb4, 0xfe, 0x1d, 0x64,
            ],
            Core::GejRescale => [
                0xe1, 0x42, 0x78, 0x60, 0xb5, 0x1b, 0xa7, 0xe7, 0x07, 0x2d, 0x75, 0x69, 0xc5, 0x5b,
                0x83, 0x39, 0xf8, 0x5b, 0x60, 0x22, 0x75, 0x0c, 0x6a, 0xf5, 0xaf, 0x2a, 0x72, 0xf1,
                0xb7, 0xb6, 0xba, 0xee,
            ],
            Core::GejXEquiv => [
                0xa7, 0xee, 0x48, 0xd5, 0xfb, 0x92, 0x6d, 0x41, 0xf7, 0x0b, 0xbb, 0x0f, 0x06, 0x82,
                0x13, 0x75, 0x34, 0x3f, 0x0d, 0x2f, 0x0e, 0x5b, 0x19, 0x91, 0x55, 0x39, 0xd9, 0x10,
                0x1b, 0x8d, 0x84, 0x7a,
            ],
            Core::GejYIsOdd => [
                0xa2, 0xd7, 0x2b, 0x1e, 0x83, 0x88, 0xa1, 0x75, 0x54, 0x27, 0xa7, 0xb2, 0x55, 0x67,
                0x68, 0x31, 0xca, 0x76, 0xea, 0xdd, 0xa2, 0x82, 0xf9, 0x7a, 0x34, 0x3f, 0x0e, 0xb1,
                0x55, 0xf3, 0x50, 0x46,
            ],
            Core::Generate => [
                0x0e, 0x91, 0xf4, 0x55, 0x7c, 0xb7, 0xd4, 0xc3, 0xbb, 0xf3, 0xf2, 0xd0, 0x74, 0xdd,
                0x69, 0x46, 0x42, 0x3a, 0x3b, 0x4f, 0xac, 0xb5, 0x7a, 0x00, 0xca, 0xe4, 0x3f, 0xd6,
                0xa7, 0x35, 0x2a, 0x13,
            ],
            Core::High16 => [
                0xae, 0x53, 0xd2, 0xd0, 0xaa, 0x16, 0x23, 0xb3, 0xa3, 0xc8, 0xb5, 0x53, 0x82, 0x6f,
                0xde, 0xf4, 0x3f, 0xf5, 0x9e, 0xa7, 0xc4, 0xa0, 0x18, 0x5c, 0xf3, 0x4c, 0x16, 0x45,
                0x08, 0xd8, 0xbf, 0xbc,
            ],
            Core::High32 => [
                0x64, 0xe7, 0xc5, 0x45, 0x3c, 0x3f, 0x8c, 0x63, 0x6c, 0x59, 0x61, 0xf4, 0x8a, 0x57,
                0xb0, 0xec, 0x7b, 0xde, 0xad, 0xed, 0xa4, 0xb3, 0x53, 0x11, 0x86, 0x7b, 0xde, 0x1c,
                0xf7, 0xc1, 0xc1, 0xd8,
            ],
            Core::High64 => [
                0xfd, 0xbf, 0xad, 0x11, 0x2a, 0xff, 0x8b, 0xd0, 0x21, 0xab, 0xee, 0xa8, 0x68, 0x88,
                0xdd, 0x28, 0xba, 0x90, 0xb1, 0xd6, 0x36, 0x3b, 0x39, 0x8a, 0x09, 0xdf, 0x9d, 0x5f,
                0x40, 0xe8, 0x06, 0x08,
            ],
            Core::High8 => [
                0x2d, 0x12, 0x02, 0xea, 0x63, 0x3d, 0x2d, 0x65, 0xf4, 0x5e, 0xaf, 0x80, 0xea, 0x03,
                0xe5, 0xcc, 0x3d, 0x7d, 0x9a, 0xe4, 0xa4, 0xb3, 0x71, 0x21, 0x8a, 0x01, 0x60, 0xa6,
                0x93, 0x10, 0x4f, 0x69,
            ],
            Core::Increment16 => [
                0xef, 0x9f, 0x63, 0x27, 0x4d, 0x9f, 0xc0, 0x71, 0x0a, 0xba, 0x34, 0x2f, 0xe7, 0xca,
                0x00, 0xd9, 0x12, 0xbf, 0x27, 0x71, 0xa9, 0x40, 0xbd, 0xd1, 0x27, 0x2d, 0x28, 0x9b,
                0x70, 0x43, 0x00, 0x44,
            ],
            Core::Increment32 => [
                0x42, 0x57, 0xa7, 0x52, 0xd0, 0x89, 0x5a, 0x75, 0x31, 0xa5, 0x21, 0x14, 0x15, 0x53,
                0xd3, 0x30, 0x90, 0xb2, 0x78, 0xd0, 0xbb, 0x79, 0xd1, 0x8b, 0x53, 0xcf, 0x75, 0x3c,
                0x76, 0x40, 0x2a, 0x0e,
            ],
            Core::Increment64 => [
                0xb5, 0x25, 0xe5, 0x54, 0x40, 0x19, 0x3d, 0xc3, 0xd5, 0x1d, 0x9f, 0xc6, 0xb1, 0xf3,
                0xf2, 0x91, 0xf1, 0xa2, 0x77, 0x6e, 0x99, 0xa2, 0x10, 0xc9, 0xfe, 0x33, 0x36, 0x77,
                0xd9, 0x68, 0x50, 0xf2,
            ],
            Core::Increment8 => [
                0xb5, 0x30, 0x23, 0x1a, 0x83, 0x32, 0x0a, 0x13, 0x0f, 0x41, 0x89, 0xf1, 0xef, 0xee,
                0x5a, 0xb2, 0x91, 0x41, 0xdc, 0xe4, 0xc3, 0x14, 0x91, 0x19, 0xcb, 0x21, 0xb9, 0xcb,
                0x06, 0xce, 0x6f, 0xa1,
            ],
            Core::IsOne16 => [
                0xec, 0xc5, 0x6a, 0x7c, 0x8d, 0xae, 0x7f, 0xb0, 0x19, 0xb9, 0xf5, 0x49, 0xf8, 0x3a,
                0x5d, 0xa1, 0x21, 0xf3, 0x74, 0xd8, 0xd4, 0x5b, 0xed, 0x50, 0x77, 0x8e, 0xa6, 0xcf,
                0xa9, 0xbb, 0x9e, 0xa0,
            ],
            Core::IsOne32 => [
                0x3f, 0x3b, 0xc4, 0x6c, 0x8c, 0x53, 0x86, 0x0c, 0x49, 0xc4, 0x70, 0x28, 0x11, 0x10,
                0x90, 0x89, 0xa2, 0xfe, 0xd0, 0xac, 0xdc, 0x3e, 0x54, 0x3b, 0x8f, 0x4c, 0x79, 0x62,
                0x65, 0x5f, 0x7f, 0x93,
            ],
            Core::IsOne64 => [
                0xe7, 0xb9, 0xc6, 0x90, 0x1f, 0x20, 0xd9, 0x8c, 0x65, 0xd3, 0xe7, 0x8a, 0xd1, 0x71,
                0x61, 0x99, 0x93, 0x90, 0x40, 0xd4, 0xe3, 0x97, 0x80, 0xb4, 0x43, 0x44, 0xab, 0x80,
                0xbb, 0x5f, 0x72, 0x3c,
            ],
            Core::IsOne8 => [
                0x6a, 0xa1, 0x40, 0xd2, 0xf0, 0xe5, 0xdd, 0x66, 0xc1, 0x21, 0x06, 0x81, 0x62, 0xef,
                0xc5, 0x79, 0xe5, 0x40, 0xf3, 0xac, 0x1c, 0x83, 0x23, 0xd1, 0xa9, 0x22, 0x6f, 0x0e,
                0xb3, 0x2f, 0x84, 0x66,
            ],
            Core::IsZero16 => [
                0x05, 0x83, 0x68, 0xc8, 0x85, 0xf9, 0x58, 0x18, 0x16, 0x76, 0xde, 0x6d, 0x4d, 0x31,
                0xca, 0xbf, 0x3f, 0x71, 0x42, 0xee, 0xa4, 0x75, 0x21, 0x28, 0x01, 0xeb, 0xe3, 0x73,
                0x79, 0x45, 0x6c, 0x2c,
            ],
            Core::IsZero32 => [
                0x3b, 0x73, 0x91, 0x81, 0x8f, 0x80, 0xcb, 0xd2, 0x2e, 0xd7, 0xe5, 0xe8, 0x52, 0x3c,
                0x82, 0x0f, 0xa0, 0xdd, 0xc1, 0xa6, 0xc1, 0x8b, 0x00, 0x86, 0xd6, 0x53, 0xff, 0xc3,
                0x7e, 0xa7, 0xa1, 0xac,
            ],
            Core::IsZero64 => [
                0xf8, 0x19, 0x13, 0xaf, 0xc6, 0x09, 0xde, 0x33, 0x40, 0xa9, 0xf6, 0x7e, 0x83, 0x92,
                0x7b, 0x57, 0x22, 0x32, 0xeb, 0x51, 0x08, 0xc0, 0x27, 0x8c, 0xbc, 0x21, 0x47, 0x61,
                0x92, 0xe0, 0x3c, 0xc7,
            ],
            Core::IsZero8 => [
                0xfd, 0x9a, 0xc0, 0x8b, 0x59, 0x5b, 0x1e, 0xcb, 0x84, 0x8d, 0xe2, 0x0f, 0xf1, 0x6b,
                0x7c, 0xb2, 0x7a, 0x77, 0x0a, 0x8a, 0x91, 0x8c, 0x01, 0x8b, 0x40, 0x25, 0xf3, 0x6a,
                0xa0, 0xc7, 0x5e, 0x32,
            ],
            Core::Le16 => [
                0x90, 0x0c, 0x66, 0x5e, 0xbe, 0x88, 0x4d, 0x50, 0x71, 0xc3, 0x2b, 0x3a, 0x0b, 0x6b,
                0x65, 0xa1, 0x2c, 0xba, 0xb6, 0xf0, 0x82, 0x48, 0x0e, 0xfa, 0x06, 0x94, 0x5a, 0x06,
                0x5f, 0xe1, 0x58, 0x66,
            ],
            Core::Le32 => [
                0xef, 0x40, 0xfc, 0x7c, 0x21, 0x58, 0x56, 0xbb, 0x7e, 0x87, 0x41, 0x4a, 0x39, 0x5b,
                0x4f, 0x02, 0xe2, 0xc4, 0xbb, 0x16, 0x7c, 0x85, 0x26, 0xed, 0x43, 0x10, 0x5d, 0x56,
                0x42, 0xb0, 0xd6, 0xb6,
            ],
            Core::Le64 => [
                0xf9, 0x1b, 0x28, 0x86, 0xb9, 0xda, 0x94, 0xf7, 0xe7, 0x97, 0xb7, 0x58, 0x14, 0xcf,
                0xee, 0xb2, 0x8c, 0xe0, 0x73, 0xe2, 0x14, 0xe5, 0xa5, 0x5f, 0x70, 0x19, 0xb6, 0x97,
                0x8b, 0x0d, 0x67, 0x42,
            ],
            Core::Le8 => [
                0x8c, 0xfb, 0x5e, 0x97, 0x85, 0x64, 0x45, 0x45, 0xcc, 0x42, 0xeb, 0x6a, 0x14, 0xb7,
                0xe9, 0x58, 0x27, 0x94, 0x99, 0x00, 0xaa, 0xa3, 0xc5, 0xad, 0xc8, 0x02, 0x4a, 0xf3,
                0xc2, 0xa3, 0x1b, 0x44,
            ],
            Core::LinearCombination1 => [
                0x95, 0x07, 0x86, 0xef, 0xa6, 0x5a, 0x71, 0x22, 0xe2, 0x55, 0x4c, 0x6f, 0xb5, 0x51,
                0x24, 0xf9, 0xe5, 0xac, 0xd8, 0x2c, 0x29, 0x81, 0x7a, 0xff, 0xc1, 0x9f, 0xc7, 0xa9,
                0x27, 0xd3, 0xa0, 0x70,
            ],
            Core::LinearVerify1 => [
                0x63, 0x55, 0x71, 0xb1, 0x27, 0xc0, 0x15, 0x65, 0x7c, 0x1b, 0xfb, 0x1d, 0x92, 0x67,
                0xbb, 0x84, 0x6a, 0x7b, 0xf9, 0x49, 0x75, 0x07, 0xae, 0xa6, 0x65, 0x37, 0x35, 0x74,
                0x08, 0xe7, 0x11, 0xa3,
            ],
            Core::Low16 => [
                0xb3, 0x27, 0xae, 0x29, 0x99, 0xed, 0x5f, 0x59, 0x42, 0x15, 0x84, 0xfd, 0x53, 0x7f,
                0x99, 0xc5, 0xfa, 0x10, 0x27, 0x1e, 0x53, 0xe1, 0x33, 0x2b, 0x1f, 0x53, 0x46, 0x35,
                0x54, 0x0d, 0xc8, 0x4d,
            ],
            Core::Low32 => [
                0x3a, 0x13, 0x27, 0x6c, 0x60, 0x97, 0xd2, 0x72, 0xdb, 0x13, 0x98, 0xc1, 0xd0, 0x3e,
                0x13, 0x02, 0x2e, 0x72, 0x9d, 0x73, 0xce, 0x50, 0xf8, 0xb2, 0xe5, 0xb2, 0xaa, 0xf9,
                0x93, 0x6a, 0x2b, 0xe1,
            ],
            Core::Low64 => [
                0x06, 0x19, 0xc9, 0x95, 0x66, 0xf8, 0xd3, 0xdc, 0x82, 0x6e, 0x8e, 0xf9, 0x67, 0x65,
                0x70, 0x19, 0x1d, 0xbc, 0xe1, 0xa0, 0x2e, 0x74, 0xd9, 0x22, 0xb1, 0x12, 0x0c, 0xb7,
                0x2d, 0x38, 0x51, 0x3b,
            ],
            Core::Low8 => [
                0xaa, 0xb5, 0xa1, 0xe3, 0xe6, 0xf0, 0x27, 0x47, 0xd5, 0xb6, 0x48, 0x97, 0xfe, 0x37,
                0xf2, 0xf7, 0x37, 0xca, 0x2b, 0xe4, 0xdc, 0x9c, 0x85, 0xa6, 0xe8, 0x04, 0xad, 0x5e,
                0x95, 0xed, 0x08, 0x33,
            ],
            Core::Lt16 => [
                0x26, 0x7a, 0xb9, 0xe8, 0xde, 0xee, 0xfa, 0x01, 0xbd, 0xd4, 0xd9, 0xfc, 0x52, 0x4b,
                0xda, 0x2c, 0x4a, 0xe5, 0xb9, 0x5e, 0x77, 0x25, 0xab, 0x97, 0xaa, 0x04, 0x22, 0x01,
                0x3e, 0x86, 0xa4, 0xae,
            ],
            Core::Lt32 => [
                0xc3, 0x1a, 0x47, 0x16, 0x1a, 0x1b, 0x5f, 0x77, 0xe2, 0x79, 0xfd, 0x22, 0xba, 0x9a,
                0xb0, 0xc3, 0xe1, 0x9c, 0x77, 0x19, 0xa0, 0x0c, 0xfe, 0xe6, 0x03, 0x6d, 0xb6, 0x8e,
                0x1b, 0x9e, 0xc6, 0xdf,
            ],
            Core::Lt64 => [
                0x58, 0x6e, 0x2b, 0x1f, 0x0b, 0x0c, 0xfa, 0x80, 0x0f, 0x83, 0x1d, 0x35, 0xed, 0x1f,
                0xdb, 0x96, 0x4f, 0xeb, 0x47, 0xbc, 0x62, 0xf7, 0xc0, 0xd4, 0x16, 0xde, 0x3f, 0xd8,
                0x3b, 0xb2, 0x56, 0x42,
            ],
            Core::Lt8 => [
                0xaf, 0xe9, 0x89, 0xbf, 0xd8, 0xba, 0x4d, 0xc1, 0x06, 0xd2, 0xe3, 0xb0, 0xb3, 0x0b,
                0x1d, 0x9b, 0x5e, 0x69, 0x16, 0xcc, 0xce, 0xbc, 0xd9, 0x87, 0x40, 0xb6, 0xa9, 0xc3,
                0xb9, 0x03, 0xb6, 0x26,
            ],
            Core::Maj16 => [
                0xd4, 0x47, 0x91, 0x1b, 0x6f, 0xbd, 0x54, 0x7d, 0x43, 0x05, 0x66, 0xbc, 0xe5, 0x21,
                0xcc, 0xf2, 0x5d, 0xff, 0x0f, 0xf5, 0x2a, 0xe7, 0xab, 0x01, 0x0b, 0x69, 0x5a, 0xe0,
                0x87, 0xec, 0x1b, 0x71,
            ],
            Core::Maj32 => [
                0x66, 0x57, 0x11, 0xe6, 0x27, 0x0b, 0xa2, 0x67, 0x56, 0xc0, 0xc1, 0x85, 0xb7, 0x01,
                0x17, 0xa1, 0x29, 0x6f, 0xf6, 0xc2, 0x9e, 0x42, 0x6a, 0xa9, 0x55, 0xf6, 0x14, 0x95,
                0x85, 0x28, 0x6c, 0x49,
            ],
            Core::Maj64 => [
                0x69, 0xf4, 0xce, 0xd9, 0xc7, 0x12, 0x73, 0x6d, 0x0d, 0x8e, 0xe1, 0xc4, 0xdc, 0xda,
                0x4f, 0xf5, 0x7f, 0x24, 0xeb, 0x31, 0x42, 0xf9, 0xe0, 0x68, 0x48, 0xde, 0x60, 0xbe,
                0x57, 0x5c, 0x91, 0x49,
            ],
            Core::Maj8 => [
                0x6b, 0xcb, 0xfe, 0x9c, 0xf8, 0xc1, 0x32, 0x41, 0x11, 0x77, 0xba, 0x84, 0xf4, 0x0f,
                0xc8, 0x4e, 0x0b, 0x8c, 0x16, 0x44, 0x24, 0x50, 0x5f, 0xdd, 0x3b, 0xdf, 0x27, 0xf7,
                0x2a, 0x0d, 0xf7, 0xf6,
            ],
            Core::Max16 => [
                0x19, 0x34, 0x49, 0xe0, 0xb8, 0x45, 0xbe, 0x48, 0xbb, 0xb1, 0x13, 0x24, 0xba, 0xc6,
                0x34, 0x45, 0x1a, 0x53, 0xbf, 0x03, 0x12, 0xcb, 0x0e, 0x8b, 0x71, 0xb1, 0xd4, 0x63,
                0x67, 0x56, 0x92, 0xfd,
            ],
            Core::Max32 => [
                0xee, 0x99, 0xdd, 0x4c, 0x3d, 0xe4, 0x96, 0xa2, 0x74, 0xce, 0x3c, 0x06, 0x50, 0xf8,
                0x1c, 0x1e, 0xd9, 0xce, 0xa8, 0x5b, 0xaa, 0xf2, 0x9c, 0x21, 0x33, 0x33, 0x70, 0x98,
                0x4c, 0xa2, 0x53, 0x5c,
            ],
            Core::Max64 => [
                0xfc, 0x75, 0xa5, 0xd0, 0xc2, 0xde, 0x6f, 0xc6, 0xc3, 0xf3, 0x52, 0x5c, 0x6e, 0x8a,
                0x89, 0x38, 0x35, 0xaa, 0x9f, 0x7a, 0x8d, 0x3d, 0x35, 0x18, 0x1d, 0x0a, 0x58, 0x1b,
                0x27, 0x23, 0xee, 0x4e,
            ],
            Core::Max8 => [
                0xcf, 0xcf, 0x17, 0x20, 0x71, 0xf0, 0x3c, 0xa6, 0x18, 0xcc, 0xd3, 0x58, 0x21, 0x06,
                0x3d, 0x4e, 0xc1, 0x3c, 0xcc, 0x6d, 0x73, 0xdf, 0x99, 0x46, 0xe9, 0xcd, 0xb7, 0x77,
                0xb1, 0x1f, 0xb4, 0x37,
            ],
            Core::Median16 => [
                0x04, 0xb1, 0x02, 0xc6, 0xbf, 0x51, 0x7d, 0x0d, 0x58, 0x90, 0xd5, 0x5e, 0x0d, 0x1a,
                0x0c, 0x23, 0x95, 0x40, 0x82, 0x0a, 0x0c, 0xde, 0x26, 0x20, 0xd5, 0x2c, 0x60, 0x2c,
                0x65, 0x56, 0xaf, 0x66,
            ],
            Core::Median32 => [
                0x77, 0x9f, 0xeb, 0xff, 0x10, 0xbb, 0x16, 0x40, 0xa7, 0x6e, 0xdb, 0x05, 0x10, 0x58,
                0xd5, 0x8f, 0xe7, 0xc3, 0x79, 0xbb, 0x8b, 0x73, 0xd2, 0xe2, 0xb1, 0xd7, 0xc2, 0x0d,
                0x26, 0x82, 0xc5, 0x0d,
            ],
            Core::Median64 => [
                0xdb, 0x62, 0xcc, 0x65, 0xbe, 0x87, 0xa9, 0x07, 0xb6, 0x87, 0xbc, 0x32, 0x36, 0xf7,
                0x5b, 0x63, 0xa7, 0xb7, 0x3d, 0x75, 0x64, 0x9e, 0xb6, 0x54, 0xeb, 0xac, 0x19, 0x68,
                0x6c, 0x32, 0x62, 0x8e,
            ],
            Core::Median8 => [
                0x89, 0x77, 0x37, 0x02, 0x27, 0x41, 0x9e, 0xc2, 0x26, 0xba, 0x4a, 0xd4, 0x5f, 0x44,
                0x06, 0x4f, 0x82, 0xf9, 0x73, 0x03, 0x94, 0xcb, 0x79, 0x2d, 0xc5, 0x43, 0xa6, 0x5b,
                0xca, 0xae, 0x7b, 0x09,
            ],
            Core::Min16 => [
                0x09, 0x38, 0x26, 0x1d, 0xf2, 0xa5, 0xa0, 0xde, 0x39, 0x71, 0x26, 0xe7, 0x65, 0x41,
                0xe2, 0x16, 0x6b, 0x24, 0x2a, 0xe8, 0x87, 0x57, 0x0d, 0xba, 0xcf, 0x9a, 0x3b, 0x6f,
                0x98, 0x21, 0x33, 0xd9,
            ],
            Core::Min32 => [
                0xa2, 0xb8, 0x5a, 0xe1, 0x70, 0xc9, 0x9f, 0x8f, 0x46, 0xa0, 0xf8, 0xf8, 0xc2, 0x1b,
                0xa6, 0xe1, 0x41, 0x9b, 0x69, 0x1d, 0x46, 0xef, 0x6b, 0x28, 0x73, 0xdf, 0x96, 0xdb,
                0xb9, 0x85, 0x89, 0x00,
            ],
            Core::Min64 => [
                0x3e, 0xa4, 0x86, 0x19, 0x3d, 0x0d, 0x1a, 0xff, 0x0c, 0xc6, 0xea, 0xab, 0x18, 0x31,
                0xf5, 0xf2, 0x5c, 0x30, 0xe7, 0x8d, 0x7e, 0x25, 0xb0, 0xb0, 0xc2, 0x78, 0x1e, 0x1e,
                0xca, 0xbf, 0x46, 0x2e,
            ],
            Core::Min8 => [
                0xe9, 0xb4, 0xe5, 0x41, 0xe3, 0x0d, 0xd5, 0x18, 0x37, 0xa2, 0x38, 0x8c, 0xb0, 0x6c,
                0x3d, 0xe9, 0x05, 0x68, 0xb4, 0xe8, 0xfe, 0x24, 0x02, 0x01, 0xfd, 0xa8, 0x33, 0xae,
                0x57, 0x89, 0x91, 0x64,
            ],
            Core::Modulo16 => [
                0x6d, 0xcf, 0xa1, 0x9b, 0xf6, 0x5f, 0x47, 0x8d, 0x30, 0x6f, 0x11, 0x3f, 0xbb, 0x64,
                0xb3, 0x3a, 0xe2, 0x1e, 0x99, 0xc0, 0xe7, 0x5c, 0x46, 0xa9, 0xd8, 0x7d, 0xbd, 0xaf,
                0x72, 0x9e, 0xc8, 0xa4,
            ],
            Core::Modulo32 => [
                0x33, 0x2a, 0xd0, 0x58, 0x62, 0x6c, 0x5e, 0x58, 0x81, 0xc7, 0x4a, 0xab, 0x45, 0x82,
                0x02, 0x08, 0xef, 0x3b, 0x3e, 0xe5, 0xc0, 0x8a, 0x0d, 0xe2, 0x87, 0x12, 0x45, 0x92,
                0x11, 0x18, 0x21, 0xec,
            ],
            Core::Modulo64 => [
                0x1f, 0x5d, 0x0b, 0x25, 0x5f, 0xe6, 0xd8, 0x3e, 0x33, 0xd2, 0x01, 0x9a, 0x2c, 0x66,
                0xec, 0xe9, 0x75, 0x0c, 0x02, 0x07, 0x3a, 0x10, 0xdf, 0x2a, 0x57, 0x3b, 0xc0, 0xe9,
                0x02, 0xbc, 0x59, 0x1b,
            ],
            Core::Modulo8 => [
                0xfc, 0x66, 0x85, 0xa7, 0x70, 0x98, 0xb1, 0x09, 0x29, 0xc6, 0xa2, 0xba, 0x8c, 0x97,
                0xb9, 0xa0, 0xbc, 0x64, 0xb1, 0xf2, 0x0a, 0xe7, 0x8d, 0x53, 0x9f, 0x59, 0x2b, 0xe9,
                0x72, 0x95, 0x07, 0x9d,
            ],
            Core::Multiply16 => [
                0x53, 0xcb, 0x58, 0x3e, 0xde, 0xe8, 0xbf, 0x8d, 0x65, 0xdb, 0xa9, 0x95, 0x65, 0x70,
                0x8a, 0x94, 0x75, 0xa7, 0xc3, 0x75, 0x3f, 0x09, 0x83, 0xe4, 0x86, 0x1f, 0x22, 0xdf,
                0xb6, 0x6a, 0x3d, 0x58,
            ],
            Core::Multiply32 => [
                0x0f, 0x2d, 0x9f, 0xf6, 0x5b, 0xa4, 0x94, 0x2c, 0x5f, 0xfa, 0x4d, 0x8e, 0x6e, 0x1f,
                0x5d, 0x91, 0x0c, 0x1a, 0xbb, 0xfe, 0x24, 0xd8, 0x65, 0xa0, 0x2e, 0x06, 0x67, 0x20,
                0xe7, 0xa4, 0xcb, 0x66,
            ],
            Core::Multiply64 => [
                0x6a, 0x6d, 0x34, 0xfe, 0xcc, 0xf3, 0xc6, 0xcd, 0xa6, 0x35, 0x5a, 0x41, 0xd0, 0x74,
                0xe3, 0x18, 0xf9, 0x1a, 0xc0, 0x98, 0x21, 0xce, 0x0a, 0x49, 0xe2, 0x52, 0x15, 0xa9,
                0xce, 0x8b, 0x41, 0x57,
            ],
            Core::Multiply8 => [
                0x4e, 0x55, 0x2a, 0xfc, 0x5d, 0xdb, 0xfe, 0x21, 0xf8, 0x0e, 0x69, 0xff, 0xf6, 0xb2,
                0x19, 0x43, 0xd0, 0x99, 0x43, 0x91, 0x83, 0x39, 0x82, 0xef, 0xda, 0xdb, 0x86, 0x7d,
                0x06, 0x16, 0x75, 0x48,
            ],
            Core::Negate16 => [
                0x10, 0xdd, 0xc6, 0x5d, 0x21, 0xf5, 0xad, 0x08, 0x2d, 0x70, 0xe9, 0xaf, 0x9b, 0xe4,
                0x6d, 0xc5, 0xcb, 0x5e, 0xe3, 0xd6, 0x16, 0xa8, 0x3b, 0x61, 0xf6, 0xd1, 0xb4, 0x45,
                0xf7, 0x6f, 0xc2, 0x21,
            ],
            Core::Negate32 => [
                0x30, 0x00, 0x9b, 0x3c, 0x58, 0x8c, 0x0d, 0xee, 0x3e, 0x5b, 0xc9, 0xa4, 0x1b, 0x2d,
                0x0b, 0xb0, 0x8c, 0x0a, 0xf0, 0xd9, 0xb4, 0x2c, 0x88, 0xa5, 0xb7, 0x07, 0xe3, 0x6c,
                0x56, 0x48, 0x2c, 0x9b,
            ],
            Core::Negate64 => [
                0xa0, 0xc2, 0xa1, 0x66, 0x13, 0xa1, 0xfc, 0xbc, 0x13, 0x93, 0xca, 0x07, 0x0b, 0xb5,
                0x58, 0xf6, 0x57, 0x65, 0x9b, 0x07, 0xf0, 0xb4, 0x3e, 0x61, 0xce, 0x9c, 0x62, 0x7e,
                0xd9, 0xd7, 0xdb, 0x63,
            ],
            Core::Negate8 => [
                0xcf, 0xb4, 0x38, 0xb3, 0x07, 0x88, 0xd7, 0xda, 0x99, 0xf7, 0x87, 0xf2, 0xfa, 0x89,
                0x33, 0x3e, 0xde, 0x1c, 0xf6, 0x3b, 0x84, 0x78, 0x02, 0x26, 0x24, 0xd6, 0x15, 0x90,
                0xad, 0x50, 0xa7, 0x37,
            ],
            Core::One16 => [
                0xef, 0x62, 0xcf, 0x64, 0xfe, 0x05, 0x88, 0x46, 0x57, 0x80, 0xe6, 0x00, 0xd2, 0x8c,
                0x83, 0x14, 0x75, 0x1c, 0x74, 0xf5, 0xd2, 0x41, 0x7f, 0xaf, 0x7b, 0x58, 0x72, 0x9a,
                0x79, 0xa4, 0x03, 0xfa,
            ],
            Core::One32 => [
                0x0a, 0x81, 0x86, 0xf9, 0xd6, 0x6a, 0xfb, 0xcc, 0xb1, 0x53, 0xe8, 0x9e, 0x5d, 0x21,
                0x53, 0x0f, 0x0d, 0x66, 0x01, 0xfa, 0x35, 0x9c, 0x4a, 0x30, 0x58, 0xe9, 0x09, 0xbc,
                0xef, 0xfe, 0x20, 0x7d,
            ],
            Core::One64 => [
                0x96, 0x4d, 0xf8, 0xba, 0xb0, 0x50, 0x82, 0x11, 0x74, 0x67, 0x44, 0x76, 0x12, 0x31,
                0x7e, 0x1c, 0x64, 0x53, 0x8d, 0xe6, 0xc9, 0xb2, 0x5d, 0xd6, 0xb1, 0x18, 0x60, 0xdf,
                0x36, 0x73, 0x94, 0x25,
            ],
            Core::One8 => [
                0xd3, 0xdb, 0xa3, 0x01, 0x75, 0x38, 0x9b, 0xf1, 0x33, 0x05, 0x15, 0x6a, 0xf9, 0xd6,
                0x9c, 0x6b, 0xa2, 0x04, 0xd3, 0xb2, 0x69, 0xbf, 0x8e, 0x2b, 0x90, 0x38, 0xe0, 0x63,
                0x50, 0x3a, 0xe5, 0xcf,
            ],
            Core::Or16 => [
                0x01, 0xfb, 0x5f, 0x5a, 0xe2, 0xaa, 0xbb, 0x88, 0x93, 0x52, 0x32, 0x88, 0x89, 0xbc,
                0x69, 0xc0, 0xe5, 0x2e, 0xeb, 0x96, 0x71, 0x5b, 0x18, 0x9e, 0x33, 0x38, 0x04, 0xf0,
                0xa4, 0x92, 0x4c, 0x9c,
            ],
            Core::Or32 => [
                0xb2, 0x75, 0x8f, 0x51, 0x14, 0x49, 0xf2, 0xaa, 0x77, 0x66, 0x43, 0xd8, 0xe0, 0x4b,
                0xfb, 0x6e, 0x10, 0xb4, 0x59, 0x1d, 0xba, 0x91, 0x9e, 0x1c, 0x19, 0x4f, 0xc8, 0x9b,
                0xb0, 0x45, 0xfd, 0x29,
            ],
            Core::Or64 => [
                0xa4, 0x0d, 0x0b, 0x13, 0x84, 0xa3, 0xe0, 0xb0, 0xad, 0xf5, 0x93, 0x6b, 0x78, 0x27,
                0x9e, 0x52, 0x3f, 0xe5, 0x5b, 0xeb, 0x3f, 0xe4, 0x26, 0x68, 0x3b, 0xc8, 0x7e, 0xc9,
                0x9b, 0xbe, 0x27, 0x9a,
            ],
            Core::Or8 => [
                0xc6, 0x03, 0xc1, 0xe8, 0x1b, 0x5a, 0x2e, 0x8e, 0x4c, 0xbf, 0x81, 0xd0, 0x59, 0xa5,
                0xa0, 0xc7, 0xc4, 0xd9, 0x1a, 0x17, 0x92, 0x8c, 0x9d, 0x5c, 0xae, 0xbf, 0x67, 0xb8,
                0x6d, 0x00, 0x34, 0xc3,
            ],
            Core::ParseLock => [
                0x48, 0x71, 0xcb, 0xae, 0x52, 0x78, 0x66, 0xba, 0x12, 0x50, 0x81, 0xdf, 0x03, 0x75,
                0x1e, 0x23, 0x1a, 0x61, 0x28, 0x22, 0xbd, 0x45, 0xde, 0x3b, 0xcf, 0xe6, 0xa8, 0xc5,
                0x14, 0xe2, 0xe7, 0x43,
            ],
            Core::ParseSequence => [
                0xfc, 0xf5, 0xd5, 0xcf, 0x69, 0x08, 0x93, 0x5e, 0x58, 0x5e, 0x9a, 0xfc, 0xc0, 0x07,
                0x17, 0xfd, 0xb1, 0xc5, 0x48, 0x79, 0x5b, 0x23, 0xf0, 0x8b, 0xb8, 0x13, 0x83, 0x17,
                0x7d, 0x93, 0xca, 0xd9,
            ],
            Core::PointVerify1 => [
                0x6a, 0x08, 0x9d, 0x61, 0xca, 0x20, 0x0a, 0x42, 0x58, 0xe8, 0xb5, 0xb4, 0xfe, 0x5c,
                0x08, 0xd5, 0x74, 0x85, 0x62, 0x49, 0x8d, 0x75, 0xf6, 0xc6, 0x26, 0x09, 0xbb, 0x68,
                0xc9, 0x8b, 0x40, 0x7c,
            ],
            Core::ScalarAdd => [
                0xa2, 0x3a, 0xa2, 0xc3, 0x3d, 0xd1, 0xef, 0xf2, 0x49, 0xa1, 0x7c, 0x91, 0xc9, 0x9a,
                0x15, 0x1d, 0x30, 0x87, 0xdc, 0x7c, 0x45, 0x91, 0xcf, 0x94, 0xc5, 0xc6, 0xab, 0xc1,
                0xba, 0x6d, 0x1c, 0xec,
            ],
            Core::ScalarInvert => [
                0xf2, 0x2d, 0xf7, 0xd7, 0x5e, 0xfd, 0xea, 0x1c, 0xfc, 0x53, 0xc5, 0x70, 0xcb, 0x8b,
                0x12, 0xa2, 0xd3, 0x41, 0x06, 0x7c, 0xb5, 0xfe, 0x41, 0xe9, 0x9d, 0xbd, 0x5f, 0xb6,
                0x32, 0x82, 0xe5, 0x1e,
            ],
            Core::ScalarIsZero => [
                0x41, 0x62, 0xf9, 0x33, 0x27, 0x03, 0xd0, 0x1e, 0x8d, 0xa6, 0x36, 0x86, 0xe8, 0x57,
                0xf2, 0x8c, 0x19, 0x82, 0x03, 0x10, 0x72, 0x2e, 0x09, 0x6b, 0xb9, 0x7f, 0xb4, 0x61,
                0xaf, 0xf5, 0x62, 0x0c,
            ],
            Core::ScalarMultiply => [
                0x72, 0xcb, 0x04, 0x36, 0xf6, 0xf1, 0x47, 0x8c, 0xcf, 0x8c, 0x54, 0x56, 0x98, 0xc6,
                0x74, 0xdf, 0x03, 0xc3, 0x9a, 0x9d, 0x2d, 0xab, 0xa1, 0x31, 0x50, 0x33, 0x46, 0xef,
                0x30, 0xd5, 0x8c, 0x26,
            ],
            Core::ScalarMultiplyLambda => [
                0x2b, 0x60, 0x03, 0x37, 0x64, 0xce, 0xec, 0x76, 0x23, 0xaf, 0x54, 0xd3, 0x86, 0xf6,
                0x87, 0xce, 0x30, 0x03, 0x25, 0x70, 0xb5, 0x6e, 0x9d, 0x5c, 0x31, 0x64, 0x8f, 0xbf,
                0x65, 0xe0, 0x14, 0x27,
            ],
            Core::ScalarNegate => [
                0x64, 0x5a, 0xe5, 0x73, 0xd5, 0xd6, 0x8b, 0x2b, 0x1a, 0xfe, 0xac, 0xe9, 0x06, 0x39,
                0xbd, 0x3a, 0x28, 0x9e, 0xeb, 0x82, 0x61, 0xa6, 0xb4, 0x72, 0x4c, 0x23, 0xca, 0xa9,
                0x1b, 0x36, 0x59, 0x3c,
            ],
            Core::ScalarNormalize => [
                0xdd, 0x37, 0x65, 0xa5, 0x16, 0x61, 0xcf, 0x66, 0x06, 0x92, 0x2f, 0x61, 0x9f, 0xca,
                0x64, 0x58, 0x52, 0x9d, 0x1e, 0x92, 0x56, 0x09, 0x83, 0x46, 0x92, 0x22, 0x47, 0xbd,
                0x04, 0x1b, 0xc1, 0x1f,
            ],
            Core::ScalarSquare => [
                0x6b, 0xff, 0xd4, 0x35, 0x3f, 0x64, 0xfe, 0x9d, 0xf8, 0xcf, 0xcb, 0x58, 0x6d, 0xfd,
                0xd0, 0x04, 0x4c, 0xb0, 0x3a, 0x6d, 0xe9, 0x13, 0x7d, 0x4c, 0xe9, 0x94, 0xff, 0x9d,
                0xa1, 0x35, 0xff, 0xe4,
            ],
            Core::Scale => [
                0x22, 0x9c, 0x9f, 0xaf, 0xad, 0xd9, 0x74, 0x5e, 0x00, 0xd1, 0x08, 0xb8, 0x2b, 0x83,
                0x62, 0x05, 0x9e, 0x83, 0x57, 0x0d, 0xfc, 0x36, 0xcb, 0x1a, 0x2c, 0xe9, 0xc5, 0xc2,
                0xd9, 0x13, 0xc6, 0x44,
            ],
            Core::Sha256Block => [
                0x45, 0xb2, 0x78, 0x86, 0xa5, 0xee, 0xd4, 0x09, 0xef, 0x05, 0xb5, 0x30, 0xc7, 0x0e,
                0x36, 0x62, 0xfa, 0xee, 0x43, 0x12, 0xd3, 0x75, 0x9d, 0xf2, 0x41, 0x44, 0x98, 0x36,
                0x1b, 0x51, 0x17, 0xae,
            ],
            Core::Sha256Ctx8Add1 => [
                0x41, 0x66, 0x54, 0x90, 0xd6, 0xed, 0xb4, 0xc5, 0xd5, 0x28, 0x45, 0x68, 0x2d, 0x7c,
                0xbd, 0x1e, 0x6c, 0x07, 0x45, 0xfa, 0xfc, 0xc0, 0x92, 0xf3, 0xef, 0x1b, 0x6f, 0xe9,
                0xcc, 0xb4, 0x2b, 0x59,
            ],
            Core::Sha256Ctx8Add128 => [
                0xed, 0x92, 0xb8, 0x91, 0x9a, 0xbb, 0x89, 0x1c, 0x0f, 0xd2, 0x0f, 0xfe, 0xa6, 0xca,
                0xae, 0xa1, 0xbd, 0x79, 0xb9, 0xe3, 0xa8, 0x2a, 0xfe, 0x2c, 0xc0, 0xce, 0x47, 0x30,
                0x86, 0xf2, 0xae, 0x2a,
            ],
            Core::Sha256Ctx8Add16 => [
                0x1a, 0xd8, 0xc2, 0x19, 0xf9, 0x71, 0xa4, 0xae, 0x29, 0xc2, 0x69, 0x95, 0x97, 0x15,
                0x0a, 0x11, 0x5b, 0xae, 0x2c, 0x09, 0xf3, 0xbc, 0x5f, 0xd8, 0x79, 0x50, 0x21, 0x63,
                0xee, 0xdd, 0x7b, 0x8d,
            ],
            Core::Sha256Ctx8Add2 => [
                0x25, 0xab, 0x7a, 0xa9, 0xf7, 0xf8, 0x57, 0xbd, 0x26, 0x89, 0x5b, 0x89, 0x0d, 0xf3,
                0x33, 0x16, 0xd7, 0x0f, 0x70, 0xb0, 0xc2, 0x68, 0xea, 0x3e, 0xc0, 0x19, 0xe0, 0x3e,
                0x50, 0xc5, 0xbd, 0x7e,
            ],
            Core::Sha256Ctx8Add256 => [
                0x10, 0xde, 0x9d, 0xa4, 0xe8, 0xf7, 0xa2, 0x91, 0x80, 0x84, 0x37, 0x91, 0x2e, 0x24,
                0xd2, 0x48, 0xe8, 0x4d, 0xac, 0xd7, 0x49, 0x68, 0x25, 0xd3, 0xca, 0xc5, 0x06, 0x72,
                0x3c, 0x21, 0x89, 0x1f,
            ],
            Core::Sha256Ctx8Add32 => [
                0x69, 0x56, 0xf8, 0x07, 0xb5, 0x6e, 0xd6, 0x64, 0x07, 0xed, 0xe0, 0xbf, 0xff, 0xb6,
                0x18, 0x69, 0x9f, 0xc5, 0x78, 0x02, 0xbe, 0xd3, 0x44, 0xb6, 0x5e, 0xae, 0xf1, 0x39,
                0xec, 0x69, 0xef, 0x0c,
            ],
            Core::Sha256Ctx8Add4 => [
                0x77, 0xd6, 0xf4, 0xb1, 0x3f, 0x2c, 0x5c, 0x42, 0x5f, 0x45, 0xfc, 0xf1, 0x86, 0x9f,
                0x9a, 0xce, 0x8c, 0xc8, 0x6b, 0xd4, 0x90, 0xe7, 0xb3, 0x14, 0x13, 0x4e, 0xf5, 0x99,
                0xbb, 0xf2, 0x21, 0xba,
            ],
            Core::Sha256Ctx8Add512 => [
                0xa8, 0x76, 0x0d, 0xf3, 0x76, 0xf0, 0x7e, 0x76, 0x2d, 0xf6, 0x55, 0x9f, 0x69, 0xb1,
                0x17, 0x61, 0x38, 0x3b, 0x3f, 0xf6, 0xd8, 0x31, 0xfb, 0x5b, 0x54, 0x2b, 0x91, 0x72,
                0xfa, 0x34, 0xb9, 0x35,
            ],
            Core::Sha256Ctx8Add64 => [
                0x80, 0x48, 0xd4, 0x90, 0x5c, 0x14, 0xb1, 0x82, 0xd8, 0xf8, 0x3c, 0x81, 0x8d, 0x03,
                0x22, 0xc2, 0xd1, 0xe4, 0x79, 0x50, 0x88, 0x98, 0x1a, 0xc7, 0xab, 0x99, 0xc8, 0x7b,
                0x12, 0xc6, 0xeb, 0x88,
            ],
            Core::Sha256Ctx8Add8 => [
                0xe5, 0xc8, 0x39, 0xa5, 0xa3, 0x29, 0x6c, 0xb3, 0xcd, 0x38, 0x29, 0x7f, 0x92, 0xe2,
                0x63, 0x63, 0x0a, 0x6f, 0x29, 0x94, 0xc9, 0xbb, 0xbd, 0x2a, 0xfa, 0x7d, 0xa8, 0x3b,
                0x1e, 0xdc, 0xe8, 0xfb,
            ],
            Core::Sha256Ctx8AddBuffer511 => [
                0x11, 0xb1, 0x3d, 0x73, 0x30, 0xae, 0x18, 0xe9, 0x19, 0x7b, 0x92, 0x1b, 0x81, 0x25,
                0x75, 0x2f, 0x8f, 0xd9, 0xb5, 0x19, 0x72, 0x04, 0xda, 0x5a, 0x7e, 0xa9, 0x24, 0xec,
                0x07, 0x2c, 0xf5, 0xfe,
            ],
            Core::Sha256Ctx8Finalize => [
                0xe8, 0xc2, 0x70, 0xdf, 0x3d, 0x70, 0xa3, 0x95, 0x65, 0xd2, 0xc6, 0x45, 0x3d, 0x68,
                0xe6, 0x1f, 0x45, 0x97, 0x83, 0xf1, 0x9a, 0xa4, 0xf6, 0x2d, 0xb5, 0x6c, 0x9a, 0x1e,
                0xc1, 0x12, 0x5d, 0x6a,
            ],
            Core::Sha256Ctx8Init => [
                0x6e, 0xcf, 0xda, 0x8c, 0x9b, 0xe3, 0xe8, 0x34, 0xc5, 0xad, 0x10, 0xcc, 0xf2, 0x5d,
                0xb1, 0x5e, 0xce, 0x9b, 0xa2, 0xb3, 0x73, 0xe8, 0x5c, 0xa1, 0x81, 0xe7, 0x37, 0x39,
                0x8d, 0xa6, 0xfc, 0x80,
            ],
            Core::Sha256Iv => [
                0x9a, 0xdb, 0x29, 0x4a, 0xa5, 0x63, 0x9a, 0x79, 0x29, 0xac, 0xc5, 0xea, 0x85, 0x8b,
                0x2a, 0x89, 0xb5, 0xc5, 0xbf, 0xc1, 0xd0, 0x93, 0x6f, 0x89, 0x32, 0xae, 0x6b, 0xb2,
                0x9b, 0x3e, 0xe6, 0x69,
            ],
            Core::Some16 => [
                0xd9, 0x80, 0x12, 0xfd, 0x09, 0x90, 0xa1, 0x70, 0x4f, 0x92, 0x80, 0x3e, 0xfc, 0x14,
                0xa9, 0x56, 0xed, 0x3a, 0xd3, 0x7b, 0x3e, 0xfc, 0x40, 0x39, 0x19, 0xaf, 0x1e, 0x20,
                0x8d, 0x0b, 0x33, 0x20,
            ],
            Core::Some32 => [
                0x7a, 0x24, 0x81, 0x51, 0xb7, 0xba, 0x67, 0x00, 0x30, 0xd0, 0x90, 0xf0, 0xa0, 0x0f,
                0x42, 0x3b, 0xbe, 0x15, 0x32, 0x76, 0xb3, 0xa7, 0xa3, 0x03, 0x33, 0x44, 0x9c, 0x67,
                0x60, 0x89, 0x6f, 0xab,
            ],
            Core::Some64 => [
                0xa3, 0xa0, 0x6c, 0x9b, 0xc4, 0x78, 0xc0, 0xd4, 0x25, 0xcb, 0x26, 0x4e, 0xa9, 0xde,
                0x3e, 0x13, 0x8a, 0x7a, 0x13, 0x84, 0x9d, 0xf0, 0x61, 0x69, 0xe8, 0x5b, 0x50, 0x36,
                0xfe, 0x97, 0x69, 0x78,
            ],
            Core::Some8 => [
                0xd6, 0xe3, 0x30, 0x68, 0x8d, 0x02, 0x75, 0x32, 0x09, 0x8a, 0x6e, 0x19, 0xdc, 0x41,
                0x7a, 0x97, 0xd9, 0xde, 0xb0, 0x5a, 0x86, 0xd1, 0xf9, 0xc0, 0xc3, 0xaa, 0xca, 0x74,
                0xeb, 0xc9, 0x58, 0x28,
            ],
            Core::Subtract16 => [
                0x14, 0x23, 0x14, 0xe6, 0x74, 0x26, 0x8d, 0x2f, 0xb6, 0x57, 0xef, 0x84, 0x49, 0x85,
                0x38, 0x92, 0xee, 0xf4, 0x61, 0x40, 0x89, 0x6b, 0x72, 0x0e, 0x46, 0xe8, 0xb0, 0x0d,
                0x6d, 0x50, 0xaf, 0x54,
            ],
            Core::Subtract32 => [
                0x4c, 0x45, 0x9b, 0x19, 0x16, 0xe3, 0x1d, 0x51, 0xcc, 0x03, 0xe6, 0x11, 0x58, 0x41,
                0xfb, 0x1f, 0x92, 0x33, 0xff, 0x2b, 0x38, 0xd8, 0xe7, 0x11, 0xe9, 0xe0, 0x16, 0xcf,
                0x89, 0x65, 0x31, 0xf6,
            ],
            Core::Subtract64 => [
                0x6b, 0x35, 0x3c, 0xa0, 0x35, 0xbf, 0x5a, 0x64, 0xae, 0x2c, 0x2d, 0x4e, 0x77, 0x05,
                0xdf, 0xed, 0x8b, 0x06, 0xaa, 0x58, 0x9c, 0xa1, 0x2c, 0xc9, 0x23, 0xfd, 0x72, 0x4c,
                0xcb, 0xd1, 0x31, 0xed,
            ],
            Core::Subtract8 => [
                0x22, 0xd2, 0x4d, 0x97, 0xa4, 0x01, 0x2b, 0x0a, 0x57, 0x78, 0x7d, 0x72, 0x94, 0xd4,
                0xd9, 0xdb, 0x9d, 0x2b, 0x85, 0xba, 0x89, 0xbb, 0xc8, 0xc1, 0x24, 0x8f, 0xc8, 0x3a,
                0xfd, 0x26, 0x47, 0x17,
            ],
            Core::Verify => [
                0x02, 0x0e, 0x84, 0x01, 0x30, 0x30, 0xec, 0x69, 0xd9, 0xa9, 0x3f, 0xec, 0x71, 0x10,
                0xe7, 0x27, 0xea, 0xd5, 0x12, 0x88, 0x5f, 0xa3, 0xc5, 0x72, 0xd8, 0xcf, 0xc3, 0x47,
                0x2c, 0xa5, 0xc8, 0xe8,
            ],
            Core::Xor16 => [
                0xa6, 0x10, 0x14, 0x40, 0x5b, 0xa1, 0x4a, 0x9b, 0xd4, 0x7d, 0x3f, 0x8f, 0x38, 0x73,
                0x19, 0x76, 0x5d, 0x34, 0xf0, 0x7e, 0xe2, 0x82, 0x58, 0xe1, 0x63, 0xa8, 0xac, 0x1f,
                0x03, 0x74, 0x87, 0xd2,
            ],
            Core::Xor32 => [
                0x14, 0x6d, 0x01, 0x72, 0x51, 0x76, 0xeb, 0x83, 0x20, 0x0a, 0x70, 0xcc, 0x4b, 0x79,
                0x3c, 0xe9, 0x52, 0x5c, 0xfe, 0x28, 0x99, 0x44, 0xd4, 0xbd, 0xf2, 0xf1, 0xc2, 0x9f,
                0x89, 0x0a, 0x86, 0xf9,
            ],
            Core::Xor64 => [
                0x7c, 0x89, 0x81, 0xc7, 0x6f, 0xeb, 0x14, 0x88, 0xe3, 0x86, 0x18, 0x06, 0xe8, 0x23,
                0x36, 0x39, 0x52, 0x56, 0xb5, 0xcb, 0xa4, 0x45, 0xab, 0xf4, 0x8f, 0xec, 0x54, 0x0d,
                0xd3, 0xf9, 0x43, 0x80,
            ],
            Core::Xor8 => [
                0x4e, 0x79, 0x06, 0x51, 0x60, 0xc4, 0x53, 0x68, 0xa0, 0xf6, 0xa7, 0x60, 0xd3, 0x1d,
                0x9e, 0xeb, 0x0b, 0x92, 0xb3, 0x32, 0x74, 0x14, 0x72, 0x37, 0x10, 0x57, 0xad, 0x04,
                0x79, 0x02, 0xd3, 0xda,
            ],
            Core::XorXor16 => [
                0x5b, 0x1c, 0xd2, 0x87, 0x52, 0x72, 0x90, 0x29, 0x6e, 0xba, 0x6d, 0xff, 0xc5, 0x64,
                0x53, 0x6d, 0x4f, 0x7a, 0x04, 0x12, 0x12, 0xaf, 0x8c, 0x27, 0x1c, 0xb5, 0xb1, 0x62,
                0xc2, 0x0b, 0x87, 0x1d,
            ],
            Core::XorXor32 => [
                0x4f, 0x96, 0xb4, 0xf5, 0x65, 0x5d, 0xd3, 0xc7, 0x59, 0xfe, 0x0a, 0x4c, 0x38, 0xd1,
                0x07, 0x94, 0x79, 0xbe, 0xc4, 0xed, 0x99, 0x25, 0x29, 0xff, 0x1a, 0x07, 0x59, 0x24,
                0xbb, 0x1c, 0x63, 0xc1,
            ],
            Core::XorXor64 => [
                0x35, 0x00, 0x87, 0xbf, 0xdc, 0x11, 0x9d, 0x8c, 0x85, 0x77, 0xe9, 0xa2, 0xf5, 0x23,
                0x9e, 0x28, 0xfe, 0xcc, 0x43, 0x0f, 0x4c, 0x2d, 0x41, 0xc0, 0x18, 0x7d, 0x26, 0x3f,
                0x79, 0x66, 0x79, 0x6c,
            ],
            Core::XorXor8 => [
                0xdc, 0x4a, 0x35, 0x81, 0xee, 0xf9, 0xf3, 0xa4, 0x0e, 0xd1, 0x2f, 0xac, 0x10, 0x91,
                0x5c, 0xc6, 0x60, 0x43, 0x20, 0xc0, 0xde, 0xc9, 0x4f, 0x19, 0xe5, 0x85, 0x11, 0x27,
                0xf9, 0x4f, 0x2d, 0x36,
            ],
        };

        Cmr(Midstate(bytes))
    }

    fn source_ty(&self) -> TypeName {
        let name: &'static [u8] = match self {
            Core::Add16 => b"i",
            Core::Add32 => b"l",
            Core::Add64 => b"*ll",
            Core::Add8 => b"****22*22**22*22***22*22**22*22",
            Core::All16 => b"****22*22**22*22***22*22**22*22",
            Core::All32 => b"i",
            Core::All64 => b"l",
            Core::All8 => b"***22*22**22*22",
            Core::And16 => b"i",
            Core::And32 => b"l",
            Core::And64 => b"*ll",
            Core::And8 => b"****22*22**22*22***22*22**22*22",
            Core::Bip0340Verify => b"**hh*hh",
            Core::Ch16 => b"*****22*22**22*22***22*22**22*22i",
            Core::Ch32 => b"*il",
            Core::Ch64 => b"*l*ll",
            Core::Ch8 => b"****22*22**22*22****22*22**22*22***22*22**22*22",
            Core::CheckSigVerify => b"**h*hh*hh",
            Core::Complement16 => b"****22*22**22*22***22*22**22*22",
            Core::Complement32 => b"i",
            Core::Complement64 => b"l",
            Core::Complement8 => b"***22*22**22*22",
            Core::Decompress => b"*2h",
            Core::Decrement16 => b"****22*22**22*22***22*22**22*22",
            Core::Decrement32 => b"i",
            Core::Decrement64 => b"l",
            Core::Decrement8 => b"***22*22**22*22",
            Core::DivMod16 => b"i",
            Core::DivMod32 => b"l",
            Core::DivMod64 => b"*ll",
            Core::DivMod8 => b"****22*22**22*22***22*22**22*22",
            Core::Divide16 => b"i",
            Core::Divide32 => b"l",
            Core::Divide64 => b"*ll",
            Core::Divide8 => b"****22*22**22*22***22*22**22*22",
            Core::Divides16 => b"i",
            Core::Divides32 => b"l",
            Core::Divides64 => b"*ll",
            Core::Divides8 => b"****22*22**22*22***22*22**22*22",
            Core::Eq16 => b"i",
            Core::Eq256 => b"*hh",
            Core::Eq32 => b"l",
            Core::Eq64 => b"*ll",
            Core::Eq8 => b"****22*22**22*22***22*22**22*22",
            Core::FeAdd => b"*hh",
            Core::FeInvert => b"h",
            Core::FeIsOdd => b"h",
            Core::FeIsZero => b"h",
            Core::FeMultiply => b"*hh",
            Core::FeMultiplyBeta => b"h",
            Core::FeNegate => b"h",
            Core::FeNormalize => b"h",
            Core::FeSquare => b"h",
            Core::FeSquareRoot => b"h",
            Core::FullAdd16 => b"*2i",
            Core::FullAdd32 => b"*2l",
            Core::FullAdd64 => b"*2*ll",
            Core::FullAdd8 => b"*2****22*22**22*22***22*22**22*22",
            Core::FullDecrement16 => b"*2****22*22**22*22***22*22**22*22",
            Core::FullDecrement32 => b"*2i",
            Core::FullDecrement64 => b"*2l",
            Core::FullDecrement8 => b"*2***22*22**22*22",
            Core::FullIncrement16 => b"*2****22*22**22*22***22*22**22*22",
            Core::FullIncrement32 => b"*2i",
            Core::FullIncrement64 => b"*2l",
            Core::FullIncrement8 => b"*2***22*22**22*22",
            Core::FullMultiply16 => b"l",
            Core::FullMultiply32 => b"*ll",
            Core::FullMultiply64 => b"h",
            Core::FullMultiply8 => b"i",
            Core::FullSubtract16 => b"*2i",
            Core::FullSubtract32 => b"*2l",
            Core::FullSubtract64 => b"*2*ll",
            Core::FullSubtract8 => b"*2****22*22**22*22***22*22**22*22",
            Core::GeIsOnCurve => b"*hh",
            Core::GeNegate => b"*hh",
            Core::GejAdd => b"***hhh**hhh",
            Core::GejDouble => b"**hhh",
            Core::GejGeAdd => b"***hhh*hh",
            Core::GejGeAddEx => b"***hhh*hh",
            Core::GejInfinity => b"1",
            Core::GejIsInfinity => b"**hhh",
            Core::GejIsOnCurve => b"**hhh",
            Core::GejNegate => b"**hhh",
            Core::GejNormalize => b"**hhh",
            Core::GejRescale => b"***hhhh",
            Core::GejXEquiv => b"*h**hhh",
            Core::GejYIsOdd => b"**hhh",
            Core::Generate => b"h",
            Core::High16 => b"1",
            Core::High32 => b"1",
            Core::High64 => b"1",
            Core::High8 => b"1",
            Core::Increment16 => b"****22*22**22*22***22*22**22*22",
            Core::Increment32 => b"i",
            Core::Increment64 => b"l",
            Core::Increment8 => b"***22*22**22*22",
            Core::IsOne16 => b"****22*22**22*22***22*22**22*22",
            Core::IsOne32 => b"i",
            Core::IsOne64 => b"l",
            Core::IsOne8 => b"***22*22**22*22",
            Core::IsZero16 => b"****22*22**22*22***22*22**22*22",
            Core::IsZero32 => b"i",
            Core::IsZero64 => b"l",
            Core::IsZero8 => b"***22*22**22*22",
            Core::Le16 => b"i",
            Core::Le32 => b"l",
            Core::Le64 => b"*ll",
            Core::Le8 => b"****22*22**22*22***22*22**22*22",
            Core::LinearCombination1 => b"**h**hhhh",
            Core::LinearVerify1 => b"***h*hhh*hh",
            Core::Low16 => b"1",
            Core::Low32 => b"1",
            Core::Low64 => b"1",
            Core::Low8 => b"1",
            Core::Lt16 => b"i",
            Core::Lt32 => b"l",
            Core::Lt64 => b"*ll",
            Core::Lt8 => b"****22*22**22*22***22*22**22*22",
            Core::Maj16 => b"*****22*22**22*22***22*22**22*22i",
            Core::Maj32 => b"*il",
            Core::Maj64 => b"*l*ll",
            Core::Maj8 => b"****22*22**22*22****22*22**22*22***22*22**22*22",
            Core::Max16 => b"i",
            Core::Max32 => b"l",
            Core::Max64 => b"*ll",
            Core::Max8 => b"****22*22**22*22***22*22**22*22",
            Core::Median16 => b"*****22*22**22*22***22*22**22*22i",
            Core::Median32 => b"*il",
            Core::Median64 => b"*l*ll",
            Core::Median8 => b"****22*22**22*22****22*22**22*22***22*22**22*22",
            Core::Min16 => b"i",
            Core::Min32 => b"l",
            Core::Min64 => b"*ll",
            Core::Min8 => b"****22*22**22*22***22*22**22*22",
            Core::Modulo16 => b"i",
            Core::Modulo32 => b"l",
            Core::Modulo64 => b"*ll",
            Core::Modulo8 => b"****22*22**22*22***22*22**22*22",
            Core::Multiply16 => b"i",
            Core::Multiply32 => b"l",
            Core::Multiply64 => b"*ll",
            Core::Multiply8 => b"****22*22**22*22***22*22**22*22",
            Core::Negate16 => b"****22*22**22*22***22*22**22*22",
            Core::Negate32 => b"i",
            Core::Negate64 => b"l",
            Core::Negate8 => b"***22*22**22*22",
            Core::One16 => b"1",
            Core::One32 => b"1",
            Core::One64 => b"1",
            Core::One8 => b"1",
            Core::Or16 => b"i",
            Core::Or32 => b"l",
            Core::Or64 => b"*ll",
            Core::Or8 => b"****22*22**22*22***22*22**22*22",
            Core::ParseLock => b"i",
            Core::ParseSequence => b"i",
            Core::PointVerify1 => b"***h*2hh*2h",
            Core::ScalarAdd => b"*hh",
            Core::ScalarInvert => b"h",
            Core::ScalarIsZero => b"h",
            Core::ScalarMultiply => b"*hh",
            Core::ScalarMultiplyLambda => b"h",
            Core::ScalarNegate => b"h",
            Core::ScalarNormalize => b"h",
            Core::ScalarSquare => b"h",
            Core::Scale => b"*h**hhh",
            Core::Sha256Block => b"*h*hh",
            Core::Sha256Ctx8Add1 => b"***+1h*+1*ll*+1l*+1i*+1****22*22**22*22***22*22**22*22+1***22*22**22*22*lh***22*22**22*22",
            Core::Sha256Ctx8Add128 => b"***+1h*+1*ll*+1l*+1i*+1****22*22**22*22***22*22**22*22+1***22*22**22*22*lh**hh*hh",
            Core::Sha256Ctx8Add16 => b"***+1h*+1*ll*+1l*+1i*+1****22*22**22*22***22*22**22*22+1***22*22**22*22*lh*ll",
            Core::Sha256Ctx8Add2 => b"***+1h*+1*ll*+1l*+1i*+1****22*22**22*22***22*22**22*22+1***22*22**22*22*lh****22*22**22*22***22*22**22*22",
            Core::Sha256Ctx8Add256 => b"***+1h*+1*ll*+1l*+1i*+1****22*22**22*22***22*22**22*22+1***22*22**22*22*lh***hh*hh**hh*hh",
            Core::Sha256Ctx8Add32 => b"***+1h*+1*ll*+1l*+1i*+1****22*22**22*22***22*22**22*22+1***22*22**22*22*lhh",
            Core::Sha256Ctx8Add4 => b"***+1h*+1*ll*+1l*+1i*+1****22*22**22*22***22*22**22*22+1***22*22**22*22*lhi",
            Core::Sha256Ctx8Add512 => b"***+1h*+1*ll*+1l*+1i*+1****22*22**22*22***22*22**22*22+1***22*22**22*22*lh****hh*hh**hh*hh***hh*hh**hh*hh",
            Core::Sha256Ctx8Add64 => b"***+1h*+1*ll*+1l*+1i*+1****22*22**22*22***22*22**22*22+1***22*22**22*22*lh*hh",
            Core::Sha256Ctx8Add8 => b"***+1h*+1*ll*+1l*+1i*+1****22*22**22*22***22*22**22*22+1***22*22**22*22*lhl",
            Core::Sha256Ctx8AddBuffer511 => b"***+1h*+1*ll*+1l*+1i*+1****22*22**22*22***22*22**22*22+1***22*22**22*22*lh*+1***hh*hh**hh*hh*+1**hh*hh*+1*hh*+1h*+1*ll*+1l*+1i*+1****22*22**22*22***22*22**22*22+1***22*22**22*22",
            Core::Sha256Ctx8Finalize => b"**+1h*+1*ll*+1l*+1i*+1****22*22**22*22***22*22**22*22+1***22*22**22*22*lh",
            Core::Sha256Ctx8Init => b"1",
            Core::Sha256Iv => b"1",
            Core::Some16 => b"****22*22**22*22***22*22**22*22",
            Core::Some32 => b"i",
            Core::Some64 => b"l",
            Core::Some8 => b"***22*22**22*22",
            Core::Subtract16 => b"i",
            Core::Subtract32 => b"l",
            Core::Subtract64 => b"*ll",
            Core::Subtract8 => b"****22*22**22*22***22*22**22*22",
            Core::Verify => b"2",
            Core::Xor16 => b"i",
            Core::Xor32 => b"l",
            Core::Xor64 => b"*ll",
            Core::Xor8 => b"****22*22**22*22***22*22**22*22",
            Core::XorXor16 => b"*****22*22**22*22***22*22**22*22i",
            Core::XorXor32 => b"*il",
            Core::XorXor64 => b"*l*ll",
            Core::XorXor8 => b"****22*22**22*22****22*22**22*22***22*22**22*22",
        };

        TypeName(name)
    }

    fn target_ty(&self) -> TypeName {
        let name: &'static [u8] = match self {
            Core::Add16 => b"*2****22*22**22*22***22*22**22*22",
            Core::Add32 => b"*2i",
            Core::Add64 => b"*2l",
            Core::Add8 => b"*2***22*22**22*22",
            Core::All16 => b"2",
            Core::All32 => b"2",
            Core::All64 => b"2",
            Core::All8 => b"2",
            Core::And16 => b"****22*22**22*22***22*22**22*22",
            Core::And32 => b"i",
            Core::And64 => b"l",
            Core::And8 => b"***22*22**22*22",
            Core::Bip0340Verify => b"1",
            Core::Ch16 => b"****22*22**22*22***22*22**22*22",
            Core::Ch32 => b"i",
            Core::Ch64 => b"l",
            Core::Ch8 => b"***22*22**22*22",
            Core::CheckSigVerify => b"1",
            Core::Complement16 => b"****22*22**22*22***22*22**22*22",
            Core::Complement32 => b"i",
            Core::Complement64 => b"l",
            Core::Complement8 => b"***22*22**22*22",
            Core::Decompress => b"+1*hh",
            Core::Decrement16 => b"*2****22*22**22*22***22*22**22*22",
            Core::Decrement32 => b"*2i",
            Core::Decrement64 => b"*2l",
            Core::Decrement8 => b"*2***22*22**22*22",
            Core::DivMod16 => b"i",
            Core::DivMod32 => b"l",
            Core::DivMod64 => b"*ll",
            Core::DivMod8 => b"****22*22**22*22***22*22**22*22",
            Core::Divide16 => b"****22*22**22*22***22*22**22*22",
            Core::Divide32 => b"i",
            Core::Divide64 => b"l",
            Core::Divide8 => b"***22*22**22*22",
            Core::Divides16 => b"2",
            Core::Divides32 => b"2",
            Core::Divides64 => b"2",
            Core::Divides8 => b"2",
            Core::Eq16 => b"2",
            Core::Eq256 => b"2",
            Core::Eq32 => b"2",
            Core::Eq64 => b"2",
            Core::Eq8 => b"2",
            Core::FeAdd => b"h",
            Core::FeInvert => b"h",
            Core::FeIsOdd => b"2",
            Core::FeIsZero => b"2",
            Core::FeMultiply => b"h",
            Core::FeMultiplyBeta => b"h",
            Core::FeNegate => b"h",
            Core::FeNormalize => b"h",
            Core::FeSquare => b"h",
            Core::FeSquareRoot => b"+1h",
            Core::FullAdd16 => b"*2****22*22**22*22***22*22**22*22",
            Core::FullAdd32 => b"*2i",
            Core::FullAdd64 => b"*2l",
            Core::FullAdd8 => b"*2***22*22**22*22",
            Core::FullDecrement16 => b"*2****22*22**22*22***22*22**22*22",
            Core::FullDecrement32 => b"*2i",
            Core::FullDecrement64 => b"*2l",
            Core::FullDecrement8 => b"*2***22*22**22*22",
            Core::FullIncrement16 => b"*2****22*22**22*22***22*22**22*22",
            Core::FullIncrement32 => b"*2i",
            Core::FullIncrement64 => b"*2l",
            Core::FullIncrement8 => b"*2***22*22**22*22",
            Core::FullMultiply16 => b"i",
            Core::FullMultiply32 => b"l",
            Core::FullMultiply64 => b"*ll",
            Core::FullMultiply8 => b"****22*22**22*22***22*22**22*22",
            Core::FullSubtract16 => b"*2****22*22**22*22***22*22**22*22",
            Core::FullSubtract32 => b"*2i",
            Core::FullSubtract64 => b"*2l",
            Core::FullSubtract8 => b"*2***22*22**22*22",
            Core::GeIsOnCurve => b"2",
            Core::GeNegate => b"*hh",
            Core::GejAdd => b"**hhh",
            Core::GejDouble => b"**hhh",
            Core::GejGeAdd => b"**hhh",
            Core::GejGeAddEx => b"*h**hhh",
            Core::GejInfinity => b"**hhh",
            Core::GejIsInfinity => b"2",
            Core::GejIsOnCurve => b"2",
            Core::GejNegate => b"**hhh",
            Core::GejNormalize => b"+1*hh",
            Core::GejRescale => b"**hhh",
            Core::GejXEquiv => b"2",
            Core::GejYIsOdd => b"2",
            Core::Generate => b"**hhh",
            Core::High16 => b"****22*22**22*22***22*22**22*22",
            Core::High32 => b"i",
            Core::High64 => b"l",
            Core::High8 => b"***22*22**22*22",
            Core::Increment16 => b"*2****22*22**22*22***22*22**22*22",
            Core::Increment32 => b"*2i",
            Core::Increment64 => b"*2l",
            Core::Increment8 => b"*2***22*22**22*22",
            Core::IsOne16 => b"2",
            Core::IsOne32 => b"2",
            Core::IsOne64 => b"2",
            Core::IsOne8 => b"2",
            Core::IsZero16 => b"2",
            Core::IsZero32 => b"2",
            Core::IsZero64 => b"2",
            Core::IsZero8 => b"2",
            Core::Le16 => b"2",
            Core::Le32 => b"2",
            Core::Le64 => b"2",
            Core::Le8 => b"2",
            Core::LinearCombination1 => b"**hhh",
            Core::LinearVerify1 => b"1",
            Core::Low16 => b"****22*22**22*22***22*22**22*22",
            Core::Low32 => b"i",
            Core::Low64 => b"l",
            Core::Low8 => b"***22*22**22*22",
            Core::Lt16 => b"2",
            Core::Lt32 => b"2",
            Core::Lt64 => b"2",
            Core::Lt8 => b"2",
            Core::Maj16 => b"****22*22**22*22***22*22**22*22",
            Core::Maj32 => b"i",
            Core::Maj64 => b"l",
            Core::Maj8 => b"***22*22**22*22",
            Core::Max16 => b"****22*22**22*22***22*22**22*22",
            Core::Max32 => b"i",
            Core::Max64 => b"l",
            Core::Max8 => b"***22*22**22*22",
            Core::Median16 => b"****22*22**22*22***22*22**22*22",
            Core::Median32 => b"i",
            Core::Median64 => b"l",
            Core::Median8 => b"***22*22**22*22",
            Core::Min16 => b"****22*22**22*22***22*22**22*22",
            Core::Min32 => b"i",
            Core::Min64 => b"l",
            Core::Min8 => b"***22*22**22*22",
            Core::Modulo16 => b"****22*22**22*22***22*22**22*22",
            Core::Modulo32 => b"i",
            Core::Modulo64 => b"l",
            Core::Modulo8 => b"***22*22**22*22",
            Core::Multiply16 => b"i",
            Core::Multiply32 => b"l",
            Core::Multiply64 => b"*ll",
            Core::Multiply8 => b"****22*22**22*22***22*22**22*22",
            Core::Negate16 => b"*2****22*22**22*22***22*22**22*22",
            Core::Negate32 => b"*2i",
            Core::Negate64 => b"*2l",
            Core::Negate8 => b"*2***22*22**22*22",
            Core::One16 => b"****22*22**22*22***22*22**22*22",
            Core::One32 => b"i",
            Core::One64 => b"l",
            Core::One8 => b"***22*22**22*22",
            Core::Or16 => b"****22*22**22*22***22*22**22*22",
            Core::Or32 => b"i",
            Core::Or64 => b"l",
            Core::Or8 => b"***22*22**22*22",
            Core::ParseLock => b"+ii",
            Core::ParseSequence => b"+1+****22*22**22*22***22*22**22*22****22*22**22*22***22*22**22*22",
            Core::PointVerify1 => b"1",
            Core::ScalarAdd => b"h",
            Core::ScalarInvert => b"h",
            Core::ScalarIsZero => b"2",
            Core::ScalarMultiply => b"h",
            Core::ScalarMultiplyLambda => b"h",
            Core::ScalarNegate => b"h",
            Core::ScalarNormalize => b"h",
            Core::ScalarSquare => b"h",
            Core::Scale => b"**hhh",
            Core::Sha256Block => b"h",
            Core::Sha256Ctx8Add1 => b"**+1h*+1*ll*+1l*+1i*+1****22*22**22*22***22*22**22*22+1***22*22**22*22*lh",
            Core::Sha256Ctx8Add128 => b"**+1h*+1*ll*+1l*+1i*+1****22*22**22*22***22*22**22*22+1***22*22**22*22*lh",
            Core::Sha256Ctx8Add16 => b"**+1h*+1*ll*+1l*+1i*+1****22*22**22*22***22*22**22*22+1***22*22**22*22*lh",
            Core::Sha256Ctx8Add2 => b"**+1h*+1*ll*+1l*+1i*+1****22*22**22*22***22*22**22*22+1***22*22**22*22*lh",
            Core::Sha256Ctx8Add256 => b"**+1h*+1*ll*+1l*+1i*+1****22*22**22*22***22*22**22*22+1***22*22**22*22*lh",
            Core::Sha256Ctx8Add32 => b"**+1h*+1*ll*+1l*+1i*+1****22*22**22*22***22*22**22*22+1***22*22**22*22*lh",
            Core::Sha256Ctx8Add4 => b"**+1h*+1*ll*+1l*+1i*+1****22*22**22*22***22*22**22*22+1***22*22**22*22*lh",
            Core::Sha256Ctx8Add512 => b"**+1h*+1*ll*+1l*+1i*+1****22*22**22*22***22*22**22*22+1***22*22**22*22*lh",
            Core::Sha256Ctx8Add64 => b"**+1h*+1*ll*+1l*+1i*+1****22*22**22*22***22*22**22*22+1***22*22**22*22*lh",
            Core::Sha256Ctx8Add8 => b"**+1h*+1*ll*+1l*+1i*+1****22*22**22*22***22*22**22*22+1***22*22**22*22*lh",
            Core::Sha256Ctx8AddBuffer511 => b"**+1h*+1*ll*+1l*+1i*+1****22*22**22*22***22*22**22*22+1***22*22**22*22*lh",
            Core::Sha256Ctx8Finalize => b"h",
            Core::Sha256Ctx8Init => b"**+1h*+1*ll*+1l*+1i*+1****22*22**22*22***22*22**22*22+1***22*22**22*22*lh",
            Core::Sha256Iv => b"h",
            Core::Some16 => b"2",
            Core::Some32 => b"2",
            Core::Some64 => b"2",
            Core::Some8 => b"2",
            Core::Subtract16 => b"*2****22*22**22*22***22*22**22*22",
            Core::Subtract32 => b"*2i",
            Core::Subtract64 => b"*2l",
            Core::Subtract8 => b"*2***22*22**22*22",
            Core::Verify => b"1",
            Core::Xor16 => b"****22*22**22*22***22*22**22*22",
            Core::Xor32 => b"i",
            Core::Xor64 => b"l",
            Core::Xor8 => b"***22*22**22*22",
            Core::XorXor16 => b"****22*22**22*22***22*22**22*22",
            Core::XorXor32 => b"i",
            Core::XorXor64 => b"l",
            Core::XorXor8 => b"***22*22**22*22",
        };

        TypeName(name)
    }

    fn encode<W: Write>(&self, w: &mut BitWriter<W>) -> std::io::Result<usize> {
        let (n, len) = match self {
            Core::Verify => (0, 2),
            Core::Low8 => (37, 7),
            Core::Low16 => (304, 10),
            Core::Low32 => (305, 10),
            Core::Low64 => (306, 10),
            Core::High8 => (45, 7),
            Core::High16 => (368, 10),
            Core::High32 => (369, 10),
            Core::High64 => (370, 10),
            Core::Complement8 => (389, 10),
            Core::Complement16 => (3120, 13),
            Core::Complement32 => (3121, 13),
            Core::Complement64 => (3122, 13),
            Core::And8 => (397, 10),
            Core::And16 => (3184, 13),
            Core::And32 => (3185, 13),
            Core::And64 => (3186, 13),
            Core::Or8 => (405, 10),
            Core::Or16 => (3248, 13),
            Core::Or32 => (3249, 13),
            Core::Or64 => (3250, 13),
            Core::Xor8 => (413, 10),
            Core::Xor16 => (3312, 13),
            Core::Xor32 => (3313, 13),
            Core::Xor64 => (3314, 13),
            Core::Maj8 => (837, 11),
            Core::Maj16 => (6704, 14),
            Core::Maj32 => (6705, 14),
            Core::Maj64 => (6706, 14),
            Core::XorXor8 => (845, 11),
            Core::XorXor16 => (6768, 14),
            Core::XorXor32 => (6769, 14),
            Core::XorXor64 => (6770, 14),
            Core::Ch8 => (853, 11),
            Core::Ch16 => (6832, 14),
            Core::Ch32 => (6833, 14),
            Core::Ch64 => (6834, 14),
            Core::Some8 => (861, 11),
            Core::Some16 => (6896, 14),
            Core::Some32 => (6897, 14),
            Core::Some64 => (6898, 14),
            Core::All8 => (869, 11),
            Core::All16 => (6960, 14),
            Core::All32 => (6961, 14),
            Core::All64 => (6962, 14),
            Core::Eq8 => (877, 11),
            Core::Eq16 => (7024, 14),
            Core::Eq32 => (7025, 14),
            Core::Eq64 => (7026, 14),
            Core::Eq256 => (14056, 15),
            Core::One8 => (69, 7),
            Core::One16 => (560, 10),
            Core::One32 => (561, 10),
            Core::One64 => (562, 10),
            Core::FullAdd8 => (293, 9),
            Core::FullAdd16 => (2352, 12),
            Core::FullAdd32 => (2353, 12),
            Core::FullAdd64 => (2354, 12),
            Core::Add8 => (301, 9),
            Core::Add16 => (2416, 12),
            Core::Add32 => (2417, 12),
            Core::Add64 => (2418, 12),
            Core::FullIncrement8 => (2437, 12),
            Core::FullIncrement16 => (19504, 15),
            Core::FullIncrement32 => (19505, 15),
            Core::FullIncrement64 => (19506, 15),
            Core::Increment8 => (2445, 12),
            Core::Increment16 => (19568, 15),
            Core::Increment32 => (19569, 15),
            Core::Increment64 => (19570, 15),
            Core::FullSubtract8 => (2461, 12),
            Core::FullSubtract16 => (19696, 15),
            Core::FullSubtract32 => (19697, 15),
            Core::FullSubtract64 => (19698, 15),
            Core::Subtract8 => (4933, 13),
            Core::Subtract16 => (39472, 16),
            Core::Subtract32 => (39473, 16),
            Core::Subtract64 => (39474, 16),
            Core::Negate8 => (4941, 13),
            Core::Negate16 => (39536, 16),
            Core::Negate32 => (39537, 16),
            Core::Negate64 => (39538, 16),
            Core::FullDecrement8 => (4949, 13),
            Core::FullDecrement16 => (39600, 16),
            Core::FullDecrement32 => (39601, 16),
            Core::FullDecrement64 => (39602, 16),
            Core::Decrement8 => (4957, 13),
            Core::Decrement16 => (39664, 16),
            Core::Decrement32 => (39665, 16),
            Core::Decrement64 => (39666, 16),
            Core::FullMultiply8 => (4965, 13),
            Core::FullMultiply16 => (39728, 16),
            Core::FullMultiply32 => (39729, 16),
            Core::FullMultiply64 => (39730, 16),
            Core::Multiply8 => (4973, 13),
            Core::Multiply16 => (39792, 16),
            Core::Multiply32 => (39793, 16),
            Core::Multiply64 => (39794, 16),
            Core::IsZero8 => (4981, 13),
            Core::IsZero16 => (39856, 16),
            Core::IsZero32 => (39857, 16),
            Core::IsZero64 => (39858, 16),
            Core::IsOne8 => (4989, 13),
            Core::IsOne16 => (39920, 16),
            Core::IsOne32 => (39921, 16),
            Core::IsOne64 => (39922, 16),
            Core::Le8 => (79877, 17),
            Core::Le16 => (639024, 20),
            Core::Le32 => (639025, 20),
            Core::Le64 => (639026, 20),
            Core::Lt8 => (79885, 17),
            Core::Lt16 => (639088, 20),
            Core::Lt32 => (639089, 20),
            Core::Lt64 => (639090, 20),
            Core::Min8 => (79893, 17),
            Core::Min16 => (639152, 20),
            Core::Min32 => (639153, 20),
            Core::Min64 => (639154, 20),
            Core::Max8 => (79901, 17),
            Core::Max16 => (639216, 20),
            Core::Max32 => (639217, 20),
            Core::Max64 => (639218, 20),
            Core::Median8 => (79909, 17),
            Core::Median16 => (639280, 20),
            Core::Median32 => (639281, 20),
            Core::Median64 => (639282, 20),
            Core::DivMod8 => (79925, 17),
            Core::DivMod16 => (639408, 20),
            Core::DivMod32 => (639409, 20),
            Core::DivMod64 => (639410, 20),
            Core::Divide8 => (79933, 17),
            Core::Divide16 => (639472, 20),
            Core::Divide32 => (639473, 20),
            Core::Divide64 => (639474, 20),
            Core::Modulo8 => (79941, 17),
            Core::Modulo16 => (639536, 20),
            Core::Modulo32 => (639537, 20),
            Core::Modulo64 => (639538, 20),
            Core::Divides8 => (79949, 17),
            Core::Divides16 => (639600, 20),
            Core::Divides32 => (639601, 20),
            Core::Divides64 => (639602, 20),
            Core::Sha256Block => (20, 5),
            Core::Sha256Iv => (84, 7),
            Core::Sha256Ctx8Add1 => (170, 8),
            Core::Sha256Ctx8Add2 => (684, 10),
            Core::Sha256Ctx8Add4 => (685, 10),
            Core::Sha256Ctx8Add8 => (5488, 13),
            Core::Sha256Ctx8Add16 => (5489, 13),
            Core::Sha256Ctx8Add32 => (5490, 13),
            Core::Sha256Ctx8Add64 => (5491, 13),
            Core::Sha256Ctx8Add128 => (10984, 14),
            Core::Sha256Ctx8Add256 => (10985, 14),
            Core::Sha256Ctx8Add512 => (10986, 14),
            Core::Sha256Ctx8AddBuffer511 => (688, 10),
            Core::Sha256Ctx8Finalize => (689, 10),
            Core::Sha256Ctx8Init => (690, 10),
            Core::PointVerify1 => (192, 8),
            Core::Decompress => (388, 9),
            Core::LinearVerify1 => (778, 10),
            Core::LinearCombination1 => (6240, 13),
            Core::Scale => (3121, 12),
            Core::Generate => (3122, 12),
            Core::GejInfinity => (3123, 12),
            Core::GejNormalize => (6248, 13),
            Core::GejNegate => (6249, 13),
            Core::GeNegate => (6250, 13),
            Core::GejDouble => (6251, 13),
            Core::GejAdd => (6252, 13),
            Core::GejGeAddEx => (6253, 13),
            Core::GejGeAdd => (6254, 13),
            Core::GejRescale => (6255, 13),
            Core::GejIsInfinity => (100096, 17),
            Core::GejXEquiv => (100099, 17),
            Core::GejYIsOdd => (100100, 17),
            Core::GejIsOnCurve => (100101, 17),
            Core::GeIsOnCurve => (100102, 17),
            Core::ScalarNormalize => (100103, 17),
            Core::ScalarNegate => (100104, 17),
            Core::ScalarAdd => (100105, 17),
            Core::ScalarSquare => (100106, 17),
            Core::ScalarMultiply => (100107, 17),
            Core::ScalarMultiplyLambda => (100108, 17),
            Core::ScalarInvert => (100109, 17),
            Core::ScalarIsZero => (100110, 17),
            Core::FeNormalize => (200227, 18),
            Core::FeNegate => (200228, 18),
            Core::FeAdd => (200229, 18),
            Core::FeSquare => (200230, 18),
            Core::FeMultiply => (200231, 18),
            Core::FeMultiplyBeta => (200232, 18),
            Core::FeInvert => (200233, 18),
            Core::FeSquareRoot => (200234, 18),
            Core::FeIsZero => (200235, 18),
            Core::FeIsOdd => (200236, 18),
            Core::CheckSigVerify => (98, 7),
            Core::Bip0340Verify => (396, 9),
            Core::ParseLock => (102, 7),
            Core::ParseSequence => (412, 9),
        };

        w.write_bits_be(n, len)
    }

    fn decode<I: Iterator<Item = u8>>(bits: &mut BitIter<I>) -> Result<Self, decode::Error> {
        decode_bits!(bits, {
            0 => {
                0 => {Core::Verify},
                1 => {
                    0 => {
                        0 => {
                            0 => {},
                            1 => {
                                0 => {
                                    0 => {},
                                    1 => {Core::Low8}
                                },
                                1 => {
                                    0 => {
                                        0 => {
                                            0 => {
                                                0 => {Core::Low16},
                                                1 => {Core::Low32}
                                            },
                                            1 => {
                                                0 => {Core::Low64},
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
                                    1 => {Core::High8}
                                },
                                1 => {
                                    0 => {
                                        0 => {
                                            0 => {
                                                0 => {Core::High16},
                                                1 => {Core::High32}
                                            },
                                            1 => {
                                                0 => {Core::High64},
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
                                                1 => {Core::Complement8}
                                            },
                                            1 => {
                                                0 => {
                                                    0 => {
                                                        0 => {
                                                            0 => {Core::Complement16},
                                                            1 => {Core::Complement32}
                                                        },
                                                        1 => {
                                                            0 => {Core::Complement64},
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
                                                1 => {Core::And8}
                                            },
                                            1 => {
                                                0 => {
                                                    0 => {
                                                        0 => {
                                                            0 => {Core::And16},
                                                            1 => {Core::And32}
                                                        },
                                                        1 => {
                                                            0 => {Core::And64},
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
                                                1 => {Core::Or8}
                                            },
                                            1 => {
                                                0 => {
                                                    0 => {
                                                        0 => {
                                                            0 => {Core::Or16},
                                                            1 => {Core::Or32}
                                                        },
                                                        1 => {
                                                            0 => {Core::Or64},
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
                                                1 => {Core::Xor8}
                                            },
                                            1 => {
                                                0 => {
                                                    0 => {
                                                        0 => {
                                                            0 => {Core::Xor16},
                                                            1 => {Core::Xor32}
                                                        },
                                                        1 => {
                                                            0 => {Core::Xor64},
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
                                                    1 => {Core::Maj8}
                                                },
                                                1 => {
                                                    0 => {
                                                        0 => {
                                                            0 => {
                                                                0 => {Core::Maj16},
                                                                1 => {Core::Maj32}
                                                            },
                                                            1 => {
                                                                0 => {Core::Maj64},
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
                                                    1 => {Core::XorXor8}
                                                },
                                                1 => {
                                                    0 => {
                                                        0 => {
                                                            0 => {
                                                                0 => {Core::XorXor16},
                                                                1 => {Core::XorXor32}
                                                            },
                                                            1 => {
                                                                0 => {Core::XorXor64},
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
                                                    1 => {Core::Ch8}
                                                },
                                                1 => {
                                                    0 => {
                                                        0 => {
                                                            0 => {
                                                                0 => {Core::Ch16},
                                                                1 => {Core::Ch32}
                                                            },
                                                            1 => {
                                                                0 => {Core::Ch64},
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
                                                    1 => {Core::Some8}
                                                },
                                                1 => {
                                                    0 => {
                                                        0 => {
                                                            0 => {
                                                                0 => {Core::Some16},
                                                                1 => {Core::Some32}
                                                            },
                                                            1 => {
                                                                0 => {Core::Some64},
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
                                                    1 => {Core::All8}
                                                },
                                                1 => {
                                                    0 => {
                                                        0 => {
                                                            0 => {
                                                                0 => {Core::All16},
                                                                1 => {Core::All32}
                                                            },
                                                            1 => {
                                                                0 => {Core::All64},
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
                                                    1 => {Core::Eq8}
                                                },
                                                1 => {
                                                    0 => {
                                                        0 => {
                                                            0 => {
                                                                0 => {Core::Eq16},
                                                                1 => {Core::Eq32}
                                                            },
                                                            1 => {
                                                                0 => {Core::Eq64},
                                                                1 => {}
                                                            }
                                                        },
                                                        1 => {
                                                            0 => {
                                                                0 => {
                                                                    0 => {Core::Eq256},
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
                                    1 => {Core::One8}
                                },
                                1 => {
                                    0 => {
                                        0 => {
                                            0 => {
                                                0 => {Core::One16},
                                                1 => {Core::One32}
                                            },
                                            1 => {
                                                0 => {Core::One64},
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
                                            1 => {Core::FullAdd8}
                                        },
                                        1 => {
                                            0 => {
                                                0 => {
                                                    0 => {
                                                        0 => {Core::FullAdd16},
                                                        1 => {Core::FullAdd32}
                                                    },
                                                    1 => {
                                                        0 => {Core::FullAdd64},
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
                                            1 => {Core::Add8}
                                        },
                                        1 => {
                                            0 => {
                                                0 => {
                                                    0 => {
                                                        0 => {Core::Add16},
                                                        1 => {Core::Add32}
                                                    },
                                                    1 => {
                                                        0 => {Core::Add64},
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
                                                        1 => {Core::FullIncrement8}
                                                    },
                                                    1 => {
                                                        0 => {
                                                            0 => {
                                                                0 => {
                                                                    0 => {Core::FullIncrement16},
                                                                    1 => {Core::FullIncrement32}
                                                                },
                                                                1 => {
                                                                    0 => {Core::FullIncrement64},
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
                                                        1 => {Core::Increment8}
                                                    },
                                                    1 => {
                                                        0 => {
                                                            0 => {
                                                                0 => {
                                                                    0 => {Core::Increment16},
                                                                    1 => {Core::Increment32}
                                                                },
                                                                1 => {
                                                                    0 => {Core::Increment64},
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
                                                        1 => {Core::FullSubtract8}
                                                    },
                                                    1 => {
                                                        0 => {
                                                            0 => {
                                                                0 => {
                                                                    0 => {Core::FullSubtract16},
                                                                    1 => {Core::FullSubtract32}
                                                                },
                                                                1 => {
                                                                    0 => {Core::FullSubtract64},
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
                                                            1 => {Core::Subtract8}
                                                        },
                                                        1 => {
                                                            0 => {
                                                                0 => {
                                                                    0 => {
                                                                        0 => {Core::Subtract16},
                                                                        1 => {Core::Subtract32}
                                                                    },
                                                                    1 => {
                                                                        0 => {Core::Subtract64},
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
                                                            1 => {Core::Negate8}
                                                        },
                                                        1 => {
                                                            0 => {
                                                                0 => {
                                                                    0 => {
                                                                        0 => {Core::Negate16},
                                                                        1 => {Core::Negate32}
                                                                    },
                                                                    1 => {
                                                                        0 => {Core::Negate64},
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
                                                            1 => {Core::FullDecrement8}
                                                        },
                                                        1 => {
                                                            0 => {
                                                                0 => {
                                                                    0 => {
                                                                        0 => {Core::FullDecrement16},
                                                                        1 => {Core::FullDecrement32}
                                                                    },
                                                                    1 => {
                                                                        0 => {Core::FullDecrement64},
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
                                                            1 => {Core::Decrement8}
                                                        },
                                                        1 => {
                                                            0 => {
                                                                0 => {
                                                                    0 => {
                                                                        0 => {Core::Decrement16},
                                                                        1 => {Core::Decrement32}
                                                                    },
                                                                    1 => {
                                                                        0 => {Core::Decrement64},
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
                                                            1 => {Core::FullMultiply8}
                                                        },
                                                        1 => {
                                                            0 => {
                                                                0 => {
                                                                    0 => {
                                                                        0 => {Core::FullMultiply16},
                                                                        1 => {Core::FullMultiply32}
                                                                    },
                                                                    1 => {
                                                                        0 => {Core::FullMultiply64},
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
                                                            1 => {Core::Multiply8}
                                                        },
                                                        1 => {
                                                            0 => {
                                                                0 => {
                                                                    0 => {
                                                                        0 => {Core::Multiply16},
                                                                        1 => {Core::Multiply32}
                                                                    },
                                                                    1 => {
                                                                        0 => {Core::Multiply64},
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
                                                            1 => {Core::IsZero8}
                                                        },
                                                        1 => {
                                                            0 => {
                                                                0 => {
                                                                    0 => {
                                                                        0 => {Core::IsZero16},
                                                                        1 => {Core::IsZero32}
                                                                    },
                                                                    1 => {
                                                                        0 => {Core::IsZero64},
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
                                                            1 => {Core::IsOne8}
                                                        },
                                                        1 => {
                                                            0 => {
                                                                0 => {
                                                                    0 => {
                                                                        0 => {Core::IsOne16},
                                                                        1 => {Core::IsOne32}
                                                                    },
                                                                    1 => {
                                                                        0 => {Core::IsOne64},
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
                                                                            1 => {Core::Le8}
                                                                        },
                                                                        1 => {
                                                                            0 => {
                                                                                0 => {
                                                                                    0 => {
                                                                                        0 => {Core::Le16},
                                                                                        1 => {Core::Le32}
                                                                                    },
                                                                                    1 => {
                                                                                        0 => {Core::Le64},
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
                                                                            1 => {Core::Lt8}
                                                                        },
                                                                        1 => {
                                                                            0 => {
                                                                                0 => {
                                                                                    0 => {
                                                                                        0 => {Core::Lt16},
                                                                                        1 => {Core::Lt32}
                                                                                    },
                                                                                    1 => {
                                                                                        0 => {Core::Lt64},
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
                                                                            1 => {Core::Min8}
                                                                        },
                                                                        1 => {
                                                                            0 => {
                                                                                0 => {
                                                                                    0 => {
                                                                                        0 => {Core::Min16},
                                                                                        1 => {Core::Min32}
                                                                                    },
                                                                                    1 => {
                                                                                        0 => {Core::Min64},
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
                                                                            1 => {Core::Max8}
                                                                        },
                                                                        1 => {
                                                                            0 => {
                                                                                0 => {
                                                                                    0 => {
                                                                                        0 => {Core::Max16},
                                                                                        1 => {Core::Max32}
                                                                                    },
                                                                                    1 => {
                                                                                        0 => {Core::Max64},
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
                                                                            1 => {Core::Median8}
                                                                        },
                                                                        1 => {
                                                                            0 => {
                                                                                0 => {
                                                                                    0 => {
                                                                                        0 => {Core::Median16},
                                                                                        1 => {Core::Median32}
                                                                                    },
                                                                                    1 => {
                                                                                        0 => {Core::Median64},
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
                                                                            1 => {Core::DivMod8}
                                                                        },
                                                                        1 => {
                                                                            0 => {
                                                                                0 => {
                                                                                    0 => {
                                                                                        0 => {Core::DivMod16},
                                                                                        1 => {Core::DivMod32}
                                                                                    },
                                                                                    1 => {
                                                                                        0 => {Core::DivMod64},
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
                                                                            1 => {Core::Divide8}
                                                                        },
                                                                        1 => {
                                                                            0 => {
                                                                                0 => {
                                                                                    0 => {
                                                                                        0 => {Core::Divide16},
                                                                                        1 => {Core::Divide32}
                                                                                    },
                                                                                    1 => {
                                                                                        0 => {Core::Divide64},
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
                                                                            1 => {Core::Modulo8}
                                                                        },
                                                                        1 => {
                                                                            0 => {
                                                                                0 => {
                                                                                    0 => {
                                                                                        0 => {Core::Modulo16},
                                                                                        1 => {Core::Modulo32}
                                                                                    },
                                                                                    1 => {
                                                                                        0 => {Core::Modulo64},
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
                                                                            1 => {Core::Divides8}
                                                                        },
                                                                        1 => {
                                                                            0 => {
                                                                                0 => {
                                                                                    0 => {
                                                                                        0 => {Core::Divides16},
                                                                                        1 => {Core::Divides32}
                                                                                    },
                                                                                    1 => {
                                                                                        0 => {Core::Divides64},
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
                            0 => {Core::Sha256Block},
                            1 => {
                                0 => {
                                    0 => {Core::Sha256Iv},
                                    1 => {
                                        0 => {Core::Sha256Ctx8Add1},
                                        1 => {
                                            0 => {
                                                0 => {Core::Sha256Ctx8Add2},
                                                1 => {Core::Sha256Ctx8Add4}
                                            },
                                            1 => {
                                                0 => {
                                                    0 => {
                                                        0 => {
                                                            0 => {Core::Sha256Ctx8Add8},
                                                            1 => {Core::Sha256Ctx8Add16}
                                                        },
                                                        1 => {
                                                            0 => {Core::Sha256Ctx8Add32},
                                                            1 => {Core::Sha256Ctx8Add64}
                                                        }
                                                    },
                                                    1 => {
                                                        0 => {
                                                            0 => {
                                                                0 => {Core::Sha256Ctx8Add128},
                                                                1 => {Core::Sha256Ctx8Add256}
                                                            },
                                                            1 => {
                                                                0 => {Core::Sha256Ctx8Add512},
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
                                                0 => {Core::Sha256Ctx8AddBuffer511},
                                                1 => {Core::Sha256Ctx8Finalize}
                                            },
                                            1 => {
                                                0 => {Core::Sha256Ctx8Init},
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
                                        0 => {Core::PointVerify1},
                                        1 => {}
                                    },
                                    1 => {
                                        0 => {
                                            0 => {Core::Decompress},
                                            1 => {
                                                0 => {Core::LinearVerify1},
                                                1 => {}
                                            }
                                        },
                                        1 => {
                                            0 => {
                                                0 => {
                                                    0 => {
                                                        0 => {
                                                            0 => {Core::LinearCombination1},
                                                            1 => {}
                                                        },
                                                        1 => {Core::Scale}
                                                    },
                                                    1 => {
                                                        0 => {Core::Generate},
                                                        1 => {Core::GejInfinity}
                                                    }
                                                },
                                                1 => {
                                                    0 => {
                                                        0 => {
                                                            0 => {Core::GejNormalize},
                                                            1 => {Core::GejNegate}
                                                        },
                                                        1 => {
                                                            0 => {Core::GeNegate},
                                                            1 => {Core::GejDouble}
                                                        }
                                                    },
                                                    1 => {
                                                        0 => {
                                                            0 => {Core::GejAdd},
                                                            1 => {Core::GejGeAddEx}
                                                        },
                                                        1 => {
                                                            0 => {Core::GejGeAdd},
                                                            1 => {Core::GejRescale}
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
                                                                            0 => {Core::GejIsInfinity},
                                                                            1 => {}
                                                                        },
                                                                        1 => {
                                                                            0 => {},
                                                                            1 => {Core::GejXEquiv}
                                                                        }
                                                                    },
                                                                    1 => {
                                                                        0 => {
                                                                            0 => {Core::GejYIsOdd},
                                                                            1 => {Core::GejIsOnCurve}
                                                                        },
                                                                        1 => {
                                                                            0 => {Core::GeIsOnCurve},
                                                                            1 => {Core::ScalarNormalize}
                                                                        }
                                                                    }
                                                                },
                                                                1 => {
                                                                    0 => {
                                                                        0 => {
                                                                            0 => {Core::ScalarNegate},
                                                                            1 => {Core::ScalarAdd}
                                                                        },
                                                                        1 => {
                                                                            0 => {Core::ScalarSquare},
                                                                            1 => {Core::ScalarMultiply}
                                                                        }
                                                                    },
                                                                    1 => {
                                                                        0 => {
                                                                            0 => {Core::ScalarMultiplyLambda},
                                                                            1 => {Core::ScalarInvert}
                                                                        },
                                                                        1 => {
                                                                            0 => {Core::ScalarIsZero},
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
                                                                                1 => {Core::FeNormalize}
                                                                            }
                                                                        },
                                                                        1 => {
                                                                            0 => {
                                                                                0 => {Core::FeNegate},
                                                                                1 => {Core::FeAdd}
                                                                            },
                                                                            1 => {
                                                                                0 => {Core::FeSquare},
                                                                                1 => {Core::FeMultiply}
                                                                            }
                                                                        }
                                                                    },
                                                                    1 => {
                                                                        0 => {
                                                                            0 => {
                                                                                0 => {Core::FeMultiplyBeta},
                                                                                1 => {Core::FeInvert}
                                                                            },
                                                                            1 => {
                                                                                0 => {Core::FeSquareRoot},
                                                                                1 => {Core::FeIsZero}
                                                                            }
                                                                        },
                                                                        1 => {
                                                                            0 => {
                                                                                0 => {Core::FeIsOdd},
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
                                    0 => {Core::CheckSigVerify},
                                    1 => {
                                        0 => {
                                            0 => {Core::Bip0340Verify},
                                            1 => {}
                                        },
                                        1 => {}
                                    }
                                }
                            },
                            1 => {
                                0 => {},
                                1 => {
                                    0 => {Core::ParseLock},
                                    1 => {
                                        0 => {
                                            0 => {Core::ParseSequence},
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
        })
    }

    fn c_jet_ptr(&self) -> &dyn Fn(&mut CFrameItem, CFrameItem, &Self::CJetEnvironment) -> bool {
        match self {
            Core::Add16 => &simplicity_sys::c_jets::jets_wrapper::add_16,
            Core::Add32 => &simplicity_sys::c_jets::jets_wrapper::add_32,
            Core::Add64 => &simplicity_sys::c_jets::jets_wrapper::add_64,
            Core::Add8 => &simplicity_sys::c_jets::jets_wrapper::add_8,
            Core::All16 => &simplicity_sys::c_jets::jets_wrapper::all_16,
            Core::All32 => &simplicity_sys::c_jets::jets_wrapper::all_32,
            Core::All64 => &simplicity_sys::c_jets::jets_wrapper::all_64,
            Core::All8 => &simplicity_sys::c_jets::jets_wrapper::all_8,
            Core::And16 => &simplicity_sys::c_jets::jets_wrapper::and_16,
            Core::And32 => &simplicity_sys::c_jets::jets_wrapper::and_32,
            Core::And64 => &simplicity_sys::c_jets::jets_wrapper::and_64,
            Core::And8 => &simplicity_sys::c_jets::jets_wrapper::and_8,
            Core::Bip0340Verify => &simplicity_sys::c_jets::jets_wrapper::bip_0340_verify,
            Core::Ch16 => &simplicity_sys::c_jets::jets_wrapper::ch_16,
            Core::Ch32 => &simplicity_sys::c_jets::jets_wrapper::ch_32,
            Core::Ch64 => &simplicity_sys::c_jets::jets_wrapper::ch_64,
            Core::Ch8 => &simplicity_sys::c_jets::jets_wrapper::ch_8,
            Core::CheckSigVerify => &simplicity_sys::c_jets::jets_wrapper::check_sig_verify,
            Core::Complement16 => &simplicity_sys::c_jets::jets_wrapper::complement_16,
            Core::Complement32 => &simplicity_sys::c_jets::jets_wrapper::complement_32,
            Core::Complement64 => &simplicity_sys::c_jets::jets_wrapper::complement_64,
            Core::Complement8 => &simplicity_sys::c_jets::jets_wrapper::complement_8,
            Core::Decompress => &simplicity_sys::c_jets::jets_wrapper::decompress,
            Core::Decrement16 => &simplicity_sys::c_jets::jets_wrapper::decrement_16,
            Core::Decrement32 => &simplicity_sys::c_jets::jets_wrapper::decrement_32,
            Core::Decrement64 => &simplicity_sys::c_jets::jets_wrapper::decrement_64,
            Core::Decrement8 => &simplicity_sys::c_jets::jets_wrapper::decrement_8,
            Core::DivMod16 => &simplicity_sys::c_jets::jets_wrapper::div_mod_16,
            Core::DivMod32 => &simplicity_sys::c_jets::jets_wrapper::div_mod_32,
            Core::DivMod64 => &simplicity_sys::c_jets::jets_wrapper::div_mod_64,
            Core::DivMod8 => &simplicity_sys::c_jets::jets_wrapper::div_mod_8,
            Core::Divide16 => &simplicity_sys::c_jets::jets_wrapper::divide_16,
            Core::Divide32 => &simplicity_sys::c_jets::jets_wrapper::divide_32,
            Core::Divide64 => &simplicity_sys::c_jets::jets_wrapper::divide_64,
            Core::Divide8 => &simplicity_sys::c_jets::jets_wrapper::divide_8,
            Core::Divides16 => &simplicity_sys::c_jets::jets_wrapper::divides_16,
            Core::Divides32 => &simplicity_sys::c_jets::jets_wrapper::divides_32,
            Core::Divides64 => &simplicity_sys::c_jets::jets_wrapper::divides_64,
            Core::Divides8 => &simplicity_sys::c_jets::jets_wrapper::divides_8,
            Core::Eq16 => &simplicity_sys::c_jets::jets_wrapper::eq_16,
            Core::Eq256 => &simplicity_sys::c_jets::jets_wrapper::eq_256,
            Core::Eq32 => &simplicity_sys::c_jets::jets_wrapper::eq_32,
            Core::Eq64 => &simplicity_sys::c_jets::jets_wrapper::eq_64,
            Core::Eq8 => &simplicity_sys::c_jets::jets_wrapper::eq_8,
            Core::FeAdd => &simplicity_sys::c_jets::jets_wrapper::fe_add,
            Core::FeInvert => &simplicity_sys::c_jets::jets_wrapper::fe_invert,
            Core::FeIsOdd => &simplicity_sys::c_jets::jets_wrapper::fe_is_odd,
            Core::FeIsZero => &simplicity_sys::c_jets::jets_wrapper::fe_is_zero,
            Core::FeMultiply => &simplicity_sys::c_jets::jets_wrapper::fe_multiply,
            Core::FeMultiplyBeta => &simplicity_sys::c_jets::jets_wrapper::fe_multiply_beta,
            Core::FeNegate => &simplicity_sys::c_jets::jets_wrapper::fe_negate,
            Core::FeNormalize => &simplicity_sys::c_jets::jets_wrapper::fe_normalize,
            Core::FeSquare => &simplicity_sys::c_jets::jets_wrapper::fe_square,
            Core::FeSquareRoot => &simplicity_sys::c_jets::jets_wrapper::fe_square_root,
            Core::FullAdd16 => &simplicity_sys::c_jets::jets_wrapper::full_add_16,
            Core::FullAdd32 => &simplicity_sys::c_jets::jets_wrapper::full_add_32,
            Core::FullAdd64 => &simplicity_sys::c_jets::jets_wrapper::full_add_64,
            Core::FullAdd8 => &simplicity_sys::c_jets::jets_wrapper::full_add_8,
            Core::FullDecrement16 => &simplicity_sys::c_jets::jets_wrapper::full_decrement_16,
            Core::FullDecrement32 => &simplicity_sys::c_jets::jets_wrapper::full_decrement_32,
            Core::FullDecrement64 => &simplicity_sys::c_jets::jets_wrapper::full_decrement_64,
            Core::FullDecrement8 => &simplicity_sys::c_jets::jets_wrapper::full_decrement_8,
            Core::FullIncrement16 => &simplicity_sys::c_jets::jets_wrapper::full_increment_16,
            Core::FullIncrement32 => &simplicity_sys::c_jets::jets_wrapper::full_increment_32,
            Core::FullIncrement64 => &simplicity_sys::c_jets::jets_wrapper::full_increment_64,
            Core::FullIncrement8 => &simplicity_sys::c_jets::jets_wrapper::full_increment_8,
            Core::FullMultiply16 => &simplicity_sys::c_jets::jets_wrapper::full_multiply_16,
            Core::FullMultiply32 => &simplicity_sys::c_jets::jets_wrapper::full_multiply_32,
            Core::FullMultiply64 => &simplicity_sys::c_jets::jets_wrapper::full_multiply_64,
            Core::FullMultiply8 => &simplicity_sys::c_jets::jets_wrapper::full_multiply_8,
            Core::FullSubtract16 => &simplicity_sys::c_jets::jets_wrapper::full_subtract_16,
            Core::FullSubtract32 => &simplicity_sys::c_jets::jets_wrapper::full_subtract_32,
            Core::FullSubtract64 => &simplicity_sys::c_jets::jets_wrapper::full_subtract_64,
            Core::FullSubtract8 => &simplicity_sys::c_jets::jets_wrapper::full_subtract_8,
            Core::GeIsOnCurve => &simplicity_sys::c_jets::jets_wrapper::ge_is_on_curve,
            Core::GeNegate => &simplicity_sys::c_jets::jets_wrapper::ge_negate,
            Core::GejAdd => &simplicity_sys::c_jets::jets_wrapper::gej_add,
            Core::GejDouble => &simplicity_sys::c_jets::jets_wrapper::gej_double,
            Core::GejGeAdd => &simplicity_sys::c_jets::jets_wrapper::gej_ge_add,
            Core::GejGeAddEx => &simplicity_sys::c_jets::jets_wrapper::gej_ge_add_ex,
            Core::GejInfinity => &simplicity_sys::c_jets::jets_wrapper::gej_infinity,
            Core::GejIsInfinity => &simplicity_sys::c_jets::jets_wrapper::gej_is_infinity,
            Core::GejIsOnCurve => &simplicity_sys::c_jets::jets_wrapper::gej_is_on_curve,
            Core::GejNegate => &simplicity_sys::c_jets::jets_wrapper::gej_negate,
            Core::GejNormalize => &simplicity_sys::c_jets::jets_wrapper::gej_normalize,
            Core::GejRescale => &simplicity_sys::c_jets::jets_wrapper::gej_rescale,
            Core::GejXEquiv => &simplicity_sys::c_jets::jets_wrapper::gej_x_equiv,
            Core::GejYIsOdd => &simplicity_sys::c_jets::jets_wrapper::gej_y_is_odd,
            Core::Generate => &simplicity_sys::c_jets::jets_wrapper::generate,
            Core::High16 => &simplicity_sys::c_jets::jets_wrapper::high_16,
            Core::High32 => &simplicity_sys::c_jets::jets_wrapper::high_32,
            Core::High64 => &simplicity_sys::c_jets::jets_wrapper::high_64,
            Core::High8 => &simplicity_sys::c_jets::jets_wrapper::high_8,
            Core::Increment16 => &simplicity_sys::c_jets::jets_wrapper::increment_16,
            Core::Increment32 => &simplicity_sys::c_jets::jets_wrapper::increment_32,
            Core::Increment64 => &simplicity_sys::c_jets::jets_wrapper::increment_64,
            Core::Increment8 => &simplicity_sys::c_jets::jets_wrapper::increment_8,
            Core::IsOne16 => &simplicity_sys::c_jets::jets_wrapper::is_one_16,
            Core::IsOne32 => &simplicity_sys::c_jets::jets_wrapper::is_one_32,
            Core::IsOne64 => &simplicity_sys::c_jets::jets_wrapper::is_one_64,
            Core::IsOne8 => &simplicity_sys::c_jets::jets_wrapper::is_one_8,
            Core::IsZero16 => &simplicity_sys::c_jets::jets_wrapper::is_zero_16,
            Core::IsZero32 => &simplicity_sys::c_jets::jets_wrapper::is_zero_32,
            Core::IsZero64 => &simplicity_sys::c_jets::jets_wrapper::is_zero_64,
            Core::IsZero8 => &simplicity_sys::c_jets::jets_wrapper::is_zero_8,
            Core::Le16 => &simplicity_sys::c_jets::jets_wrapper::le_16,
            Core::Le32 => &simplicity_sys::c_jets::jets_wrapper::le_32,
            Core::Le64 => &simplicity_sys::c_jets::jets_wrapper::le_64,
            Core::Le8 => &simplicity_sys::c_jets::jets_wrapper::le_8,
            Core::LinearCombination1 => &simplicity_sys::c_jets::jets_wrapper::linear_combination_1,
            Core::LinearVerify1 => &simplicity_sys::c_jets::jets_wrapper::linear_verify_1,
            Core::Low16 => &simplicity_sys::c_jets::jets_wrapper::low_16,
            Core::Low32 => &simplicity_sys::c_jets::jets_wrapper::low_32,
            Core::Low64 => &simplicity_sys::c_jets::jets_wrapper::low_64,
            Core::Low8 => &simplicity_sys::c_jets::jets_wrapper::low_8,
            Core::Lt16 => &simplicity_sys::c_jets::jets_wrapper::lt_16,
            Core::Lt32 => &simplicity_sys::c_jets::jets_wrapper::lt_32,
            Core::Lt64 => &simplicity_sys::c_jets::jets_wrapper::lt_64,
            Core::Lt8 => &simplicity_sys::c_jets::jets_wrapper::lt_8,
            Core::Maj16 => &simplicity_sys::c_jets::jets_wrapper::maj_16,
            Core::Maj32 => &simplicity_sys::c_jets::jets_wrapper::maj_32,
            Core::Maj64 => &simplicity_sys::c_jets::jets_wrapper::maj_64,
            Core::Maj8 => &simplicity_sys::c_jets::jets_wrapper::maj_8,
            Core::Max16 => &simplicity_sys::c_jets::jets_wrapper::max_16,
            Core::Max32 => &simplicity_sys::c_jets::jets_wrapper::max_32,
            Core::Max64 => &simplicity_sys::c_jets::jets_wrapper::max_64,
            Core::Max8 => &simplicity_sys::c_jets::jets_wrapper::max_8,
            Core::Median16 => &simplicity_sys::c_jets::jets_wrapper::median_16,
            Core::Median32 => &simplicity_sys::c_jets::jets_wrapper::median_32,
            Core::Median64 => &simplicity_sys::c_jets::jets_wrapper::median_64,
            Core::Median8 => &simplicity_sys::c_jets::jets_wrapper::median_8,
            Core::Min16 => &simplicity_sys::c_jets::jets_wrapper::min_16,
            Core::Min32 => &simplicity_sys::c_jets::jets_wrapper::min_32,
            Core::Min64 => &simplicity_sys::c_jets::jets_wrapper::min_64,
            Core::Min8 => &simplicity_sys::c_jets::jets_wrapper::min_8,
            Core::Modulo16 => &simplicity_sys::c_jets::jets_wrapper::modulo_16,
            Core::Modulo32 => &simplicity_sys::c_jets::jets_wrapper::modulo_32,
            Core::Modulo64 => &simplicity_sys::c_jets::jets_wrapper::modulo_64,
            Core::Modulo8 => &simplicity_sys::c_jets::jets_wrapper::modulo_8,
            Core::Multiply16 => &simplicity_sys::c_jets::jets_wrapper::multiply_16,
            Core::Multiply32 => &simplicity_sys::c_jets::jets_wrapper::multiply_32,
            Core::Multiply64 => &simplicity_sys::c_jets::jets_wrapper::multiply_64,
            Core::Multiply8 => &simplicity_sys::c_jets::jets_wrapper::multiply_8,
            Core::Negate16 => &simplicity_sys::c_jets::jets_wrapper::negate_16,
            Core::Negate32 => &simplicity_sys::c_jets::jets_wrapper::negate_32,
            Core::Negate64 => &simplicity_sys::c_jets::jets_wrapper::negate_64,
            Core::Negate8 => &simplicity_sys::c_jets::jets_wrapper::negate_8,
            Core::One16 => &simplicity_sys::c_jets::jets_wrapper::one_16,
            Core::One32 => &simplicity_sys::c_jets::jets_wrapper::one_32,
            Core::One64 => &simplicity_sys::c_jets::jets_wrapper::one_64,
            Core::One8 => &simplicity_sys::c_jets::jets_wrapper::one_8,
            Core::Or16 => &simplicity_sys::c_jets::jets_wrapper::or_16,
            Core::Or32 => &simplicity_sys::c_jets::jets_wrapper::or_32,
            Core::Or64 => &simplicity_sys::c_jets::jets_wrapper::or_64,
            Core::Or8 => &simplicity_sys::c_jets::jets_wrapper::or_8,
            Core::ParseLock => &simplicity_sys::c_jets::jets_wrapper::parse_lock,
            Core::ParseSequence => &simplicity_sys::c_jets::jets_wrapper::parse_sequence,
            Core::PointVerify1 => &simplicity_sys::c_jets::jets_wrapper::point_verify_1,
            Core::ScalarAdd => &simplicity_sys::c_jets::jets_wrapper::scalar_add,
            Core::ScalarInvert => &simplicity_sys::c_jets::jets_wrapper::scalar_invert,
            Core::ScalarIsZero => &simplicity_sys::c_jets::jets_wrapper::scalar_is_zero,
            Core::ScalarMultiply => &simplicity_sys::c_jets::jets_wrapper::scalar_multiply,
            Core::ScalarMultiplyLambda => &simplicity_sys::c_jets::jets_wrapper::scalar_multiply_lambda,
            Core::ScalarNegate => &simplicity_sys::c_jets::jets_wrapper::scalar_negate,
            Core::ScalarNormalize => &simplicity_sys::c_jets::jets_wrapper::scalar_normalize,
            Core::ScalarSquare => &simplicity_sys::c_jets::jets_wrapper::scalar_square,
            Core::Scale => &simplicity_sys::c_jets::jets_wrapper::scale,
            Core::Sha256Block => &simplicity_sys::c_jets::jets_wrapper::sha_256_block,
            Core::Sha256Ctx8Add1 => &simplicity_sys::c_jets::jets_wrapper::sha_256_ctx_8_add_1,
            Core::Sha256Ctx8Add128 => &simplicity_sys::c_jets::jets_wrapper::sha_256_ctx_8_add_128,
            Core::Sha256Ctx8Add16 => &simplicity_sys::c_jets::jets_wrapper::sha_256_ctx_8_add_16,
            Core::Sha256Ctx8Add2 => &simplicity_sys::c_jets::jets_wrapper::sha_256_ctx_8_add_2,
            Core::Sha256Ctx8Add256 => &simplicity_sys::c_jets::jets_wrapper::sha_256_ctx_8_add_256,
            Core::Sha256Ctx8Add32 => &simplicity_sys::c_jets::jets_wrapper::sha_256_ctx_8_add_32,
            Core::Sha256Ctx8Add4 => &simplicity_sys::c_jets::jets_wrapper::sha_256_ctx_8_add_4,
            Core::Sha256Ctx8Add512 => &simplicity_sys::c_jets::jets_wrapper::sha_256_ctx_8_add_512,
            Core::Sha256Ctx8Add64 => &simplicity_sys::c_jets::jets_wrapper::sha_256_ctx_8_add_64,
            Core::Sha256Ctx8Add8 => &simplicity_sys::c_jets::jets_wrapper::sha_256_ctx_8_add_8,
            Core::Sha256Ctx8AddBuffer511 => &simplicity_sys::c_jets::jets_wrapper::sha_256_ctx_8_add_buffer_511,
            Core::Sha256Ctx8Finalize => &simplicity_sys::c_jets::jets_wrapper::sha_256_ctx_8_finalize,
            Core::Sha256Ctx8Init => &simplicity_sys::c_jets::jets_wrapper::sha_256_ctx_8_init,
            Core::Sha256Iv => &simplicity_sys::c_jets::jets_wrapper::sha_256_iv,
            Core::Some16 => &simplicity_sys::c_jets::jets_wrapper::some_16,
            Core::Some32 => &simplicity_sys::c_jets::jets_wrapper::some_32,
            Core::Some64 => &simplicity_sys::c_jets::jets_wrapper::some_64,
            Core::Some8 => &simplicity_sys::c_jets::jets_wrapper::some_8,
            Core::Subtract16 => &simplicity_sys::c_jets::jets_wrapper::subtract_16,
            Core::Subtract32 => &simplicity_sys::c_jets::jets_wrapper::subtract_32,
            Core::Subtract64 => &simplicity_sys::c_jets::jets_wrapper::subtract_64,
            Core::Subtract8 => &simplicity_sys::c_jets::jets_wrapper::subtract_8,
            Core::Verify => &simplicity_sys::c_jets::jets_wrapper::verify,
            Core::Xor16 => &simplicity_sys::c_jets::jets_wrapper::xor_16,
            Core::Xor32 => &simplicity_sys::c_jets::jets_wrapper::xor_32,
            Core::Xor64 => &simplicity_sys::c_jets::jets_wrapper::xor_64,
            Core::Xor8 => &simplicity_sys::c_jets::jets_wrapper::xor_8,
            Core::XorXor16 => &simplicity_sys::c_jets::jets_wrapper::xor_xor_16,
            Core::XorXor32 => &simplicity_sys::c_jets::jets_wrapper::xor_xor_32,
            Core::XorXor64 => &simplicity_sys::c_jets::jets_wrapper::xor_xor_64,
            Core::XorXor8 => &simplicity_sys::c_jets::jets_wrapper::xor_xor_8,
        }
    }

    fn cost(&self) -> Cost {
        match self {
            Core::Add16 => Cost::from_milliweight(133),
            Core::Add32 => Cost::from_milliweight(133),
            Core::Add64 => Cost::from_milliweight(133),
            Core::Add8 => Cost::from_milliweight(133),
            Core::All16 => Cost::from_milliweight(98),
            Core::All32 => Cost::from_milliweight(98),
            Core::All64 => Cost::from_milliweight(98),
            Core::All8 => Cost::from_milliweight(98),
            Core::And16 => Cost::from_milliweight(133),
            Core::And32 => Cost::from_milliweight(133),
            Core::And64 => Cost::from_milliweight(133),
            Core::And8 => Cost::from_milliweight(133),
            Core::Bip0340Verify => Cost::from_milliweight(50735),
            Core::Ch16 => Cost::from_milliweight(133),
            Core::Ch32 => Cost::from_milliweight(133),
            Core::Ch64 => Cost::from_milliweight(133),
            Core::Ch8 => Cost::from_milliweight(133),
            Core::CheckSigVerify => Cost::from_milliweight(50000),
            Core::Complement16 => Cost::from_milliweight(133),
            Core::Complement32 => Cost::from_milliweight(133),
            Core::Complement64 => Cost::from_milliweight(133),
            Core::Complement8 => Cost::from_milliweight(133),
            Core::Decompress => Cost::from_milliweight(11173),
            Core::Decrement16 => Cost::from_milliweight(109),
            Core::Decrement32 => Cost::from_milliweight(109),
            Core::Decrement64 => Cost::from_milliweight(109),
            Core::Decrement8 => Cost::from_milliweight(109),
            Core::DivMod16 => Cost::from_milliweight(138),
            Core::DivMod32 => Cost::from_milliweight(138),
            Core::DivMod64 => Cost::from_milliweight(138),
            Core::DivMod8 => Cost::from_milliweight(138),
            Core::Divide16 => Cost::from_milliweight(138),
            Core::Divide32 => Cost::from_milliweight(138),
            Core::Divide64 => Cost::from_milliweight(138),
            Core::Divide8 => Cost::from_milliweight(138),
            Core::Divides16 => Cost::from_milliweight(138),
            Core::Divides32 => Cost::from_milliweight(138),
            Core::Divides64 => Cost::from_milliweight(138),
            Core::Divides8 => Cost::from_milliweight(138),
            Core::Eq16 => Cost::from_milliweight(69),
            Core::Eq256 => Cost::from_milliweight(153),
            Core::Eq32 => Cost::from_milliweight(69),
            Core::Eq64 => Cost::from_milliweight(69),
            Core::Eq8 => Cost::from_milliweight(69),
            Core::FeAdd => Cost::from_milliweight(1408),
            Core::FeInvert => Cost::from_milliweight(4008),
            Core::FeIsOdd => Cost::from_milliweight(311),
            Core::FeIsZero => Cost::from_milliweight(299),
            Core::FeMultiply => Cost::from_milliweight(1109),
            Core::FeMultiplyBeta => Cost::from_milliweight(755),
            Core::FeNegate => Cost::from_milliweight(2187),
            Core::FeNormalize => Cost::from_milliweight(2672),
            Core::FeSquare => Cost::from_milliweight(800),
            Core::FeSquareRoot => Cost::from_milliweight(12716),
            Core::FullAdd16 => Cost::from_milliweight(93),
            Core::FullAdd32 => Cost::from_milliweight(93),
            Core::FullAdd64 => Cost::from_milliweight(93),
            Core::FullAdd8 => Cost::from_milliweight(93),
            Core::FullDecrement16 => Cost::from_milliweight(109),
            Core::FullDecrement32 => Cost::from_milliweight(109),
            Core::FullDecrement64 => Cost::from_milliweight(109),
            Core::FullDecrement8 => Cost::from_milliweight(109),
            Core::FullIncrement16 => Cost::from_milliweight(133),
            Core::FullIncrement32 => Cost::from_milliweight(133),
            Core::FullIncrement64 => Cost::from_milliweight(133),
            Core::FullIncrement8 => Cost::from_milliweight(133),
            Core::FullMultiply16 => Cost::from_milliweight(138),
            Core::FullMultiply32 => Cost::from_milliweight(138),
            Core::FullMultiply64 => Cost::from_milliweight(138),
            Core::FullMultiply8 => Cost::from_milliweight(138),
            Core::FullSubtract16 => Cost::from_milliweight(104),
            Core::FullSubtract32 => Cost::from_milliweight(104),
            Core::FullSubtract64 => Cost::from_milliweight(104),
            Core::FullSubtract8 => Cost::from_milliweight(104),
            Core::GeIsOnCurve => Cost::from_milliweight(768),
            Core::GeNegate => Cost::from_milliweight(1596),
            Core::GejAdd => Cost::from_milliweight(3274),
            Core::GejDouble => Cost::from_milliweight(1899),
            Core::GejGeAdd => Cost::from_milliweight(3004),
            Core::GejGeAddEx => Cost::from_milliweight(2981),
            Core::GejInfinity => Cost::from_milliweight(1042),
            Core::GejIsInfinity => Cost::from_milliweight(1509),
            Core::GejIsOnCurve => Cost::from_milliweight(1286),
            Core::GejNegate => Cost::from_milliweight(1596),
            Core::GejNormalize => Cost::from_milliweight(5931),
            Core::GejRescale => Cost::from_milliweight(2462),
            Core::GejXEquiv => Cost::from_milliweight(1383),
            Core::GejYIsOdd => Cost::from_milliweight(5506),
            Core::Generate => Cost::from_milliweight(49026),
            Core::High16 => Cost::from_milliweight(57),
            Core::High32 => Cost::from_milliweight(57),
            Core::High64 => Cost::from_milliweight(57),
            Core::High8 => Cost::from_milliweight(57),
            Core::Increment16 => Cost::from_milliweight(133),
            Core::Increment32 => Cost::from_milliweight(133),
            Core::Increment64 => Cost::from_milliweight(133),
            Core::Increment8 => Cost::from_milliweight(133),
            Core::IsOne16 => Cost::from_milliweight(98),
            Core::IsOne32 => Cost::from_milliweight(98),
            Core::IsOne64 => Cost::from_milliweight(98),
            Core::IsOne8 => Cost::from_milliweight(98),
            Core::IsZero16 => Cost::from_milliweight(98),
            Core::IsZero32 => Cost::from_milliweight(98),
            Core::IsZero64 => Cost::from_milliweight(98),
            Core::IsZero8 => Cost::from_milliweight(98),
            Core::Le16 => Cost::from_milliweight(98),
            Core::Le32 => Cost::from_milliweight(98),
            Core::Le64 => Cost::from_milliweight(98),
            Core::Le8 => Cost::from_milliweight(98),
            Core::LinearCombination1 => Cost::from_milliweight(83117),
            Core::LinearVerify1 => Cost::from_milliweight(49791),
            Core::Low16 => Cost::from_milliweight(57),
            Core::Low32 => Cost::from_milliweight(57),
            Core::Low64 => Cost::from_milliweight(57),
            Core::Low8 => Cost::from_milliweight(57),
            Core::Lt16 => Cost::from_milliweight(98),
            Core::Lt32 => Cost::from_milliweight(98),
            Core::Lt64 => Cost::from_milliweight(98),
            Core::Lt8 => Cost::from_milliweight(98),
            Core::Maj16 => Cost::from_milliweight(133),
            Core::Maj32 => Cost::from_milliweight(133),
            Core::Maj64 => Cost::from_milliweight(133),
            Core::Maj8 => Cost::from_milliweight(133),
            Core::Max16 => Cost::from_milliweight(98),
            Core::Max32 => Cost::from_milliweight(98),
            Core::Max64 => Cost::from_milliweight(98),
            Core::Max8 => Cost::from_milliweight(98),
            Core::Median16 => Cost::from_milliweight(98),
            Core::Median32 => Cost::from_milliweight(98),
            Core::Median64 => Cost::from_milliweight(98),
            Core::Median8 => Cost::from_milliweight(98),
            Core::Min16 => Cost::from_milliweight(98),
            Core::Min32 => Cost::from_milliweight(98),
            Core::Min64 => Cost::from_milliweight(98),
            Core::Min8 => Cost::from_milliweight(98),
            Core::Modulo16 => Cost::from_milliweight(138),
            Core::Modulo32 => Cost::from_milliweight(138),
            Core::Modulo64 => Cost::from_milliweight(138),
            Core::Modulo8 => Cost::from_milliweight(138),
            Core::Multiply16 => Cost::from_milliweight(87),
            Core::Multiply32 => Cost::from_milliweight(87),
            Core::Multiply64 => Cost::from_milliweight(87),
            Core::Multiply8 => Cost::from_milliweight(87),
            Core::Negate16 => Cost::from_milliweight(109),
            Core::Negate32 => Cost::from_milliweight(109),
            Core::Negate64 => Cost::from_milliweight(109),
            Core::Negate8 => Cost::from_milliweight(109),
            Core::One16 => Cost::from_milliweight(53),
            Core::One32 => Cost::from_milliweight(53),
            Core::One64 => Cost::from_milliweight(53),
            Core::One8 => Cost::from_milliweight(53),
            Core::Or16 => Cost::from_milliweight(133),
            Core::Or32 => Cost::from_milliweight(133),
            Core::Or64 => Cost::from_milliweight(133),
            Core::Or8 => Cost::from_milliweight(133),
            Core::ParseLock => Cost::from_milliweight(84),
            Core::ParseSequence => Cost::from_milliweight(82),
            Core::PointVerify1 => Cost::from_milliweight(56153),
            Core::ScalarAdd => Cost::from_milliweight(862),
            Core::ScalarInvert => Cost::from_milliweight(3853),
            Core::ScalarIsZero => Cost::from_milliweight(323),
            Core::ScalarMultiply => Cost::from_milliweight(1070),
            Core::ScalarMultiplyLambda => Cost::from_milliweight(736),
            Core::ScalarNegate => Cost::from_milliweight(726),
            Core::ScalarNormalize => Cost::from_milliweight(740),
            Core::ScalarSquare => Cost::from_milliweight(822),
            Core::Scale => Cost::from_milliweight(75458),
            Core::Sha256Block => Cost::from_milliweight(1046),
            Core::Sha256Ctx8Add1 => Cost::from_milliweight(454),
            Core::Sha256Ctx8Add128 => Cost::from_milliweight(401),
            Core::Sha256Ctx8Add16 => Cost::from_milliweight(340),
            Core::Sha256Ctx8Add2 => Cost::from_milliweight(576),
            Core::Sha256Ctx8Add256 => Cost::from_milliweight(572),
            Core::Sha256Ctx8Add32 => Cost::from_milliweight(326),
            Core::Sha256Ctx8Add4 => Cost::from_milliweight(570),
            Core::Sha256Ctx8Add512 => Cost::from_milliweight(559),
            Core::Sha256Ctx8Add64 => Cost::from_milliweight(475),
            Core::Sha256Ctx8Add8 => Cost::from_milliweight(330),
            Core::Sha256Ctx8AddBuffer511 => Cost::from_milliweight(412),
            Core::Sha256Ctx8Finalize => Cost::from_milliweight(1205),
            Core::Sha256Ctx8Init => Cost::from_milliweight(237),
            Core::Sha256Iv => Cost::from_milliweight(107),
            Core::Some16 => Cost::from_milliweight(98),
            Core::Some32 => Cost::from_milliweight(98),
            Core::Some64 => Cost::from_milliweight(98),
            Core::Some8 => Cost::from_milliweight(98),
            Core::Subtract16 => Cost::from_milliweight(109),
            Core::Subtract32 => Cost::from_milliweight(109),
            Core::Subtract64 => Cost::from_milliweight(109),
            Core::Subtract8 => Cost::from_milliweight(109),
            Core::Verify => Cost::from_milliweight(45),
            Core::Xor16 => Cost::from_milliweight(133),
            Core::Xor32 => Cost::from_milliweight(133),
            Core::Xor64 => Cost::from_milliweight(133),
            Core::Xor8 => Cost::from_milliweight(133),
            Core::XorXor16 => Cost::from_milliweight(133),
            Core::XorXor32 => Cost::from_milliweight(133),
            Core::XorXor64 => Cost::from_milliweight(133),
            Core::XorXor8 => Cost::from_milliweight(133),
        }
    }
}

impl fmt::Display for Core {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Core::Add16 => f.write_str("add_16"),
            Core::Add32 => f.write_str("add_32"),
            Core::Add64 => f.write_str("add_64"),
            Core::Add8 => f.write_str("add_8"),
            Core::All16 => f.write_str("all_16"),
            Core::All32 => f.write_str("all_32"),
            Core::All64 => f.write_str("all_64"),
            Core::All8 => f.write_str("all_8"),
            Core::And16 => f.write_str("and_16"),
            Core::And32 => f.write_str("and_32"),
            Core::And64 => f.write_str("and_64"),
            Core::And8 => f.write_str("and_8"),
            Core::Bip0340Verify => f.write_str("bip_0340_verify"),
            Core::Ch16 => f.write_str("ch_16"),
            Core::Ch32 => f.write_str("ch_32"),
            Core::Ch64 => f.write_str("ch_64"),
            Core::Ch8 => f.write_str("ch_8"),
            Core::CheckSigVerify => f.write_str("check_sig_verify"),
            Core::Complement16 => f.write_str("complement_16"),
            Core::Complement32 => f.write_str("complement_32"),
            Core::Complement64 => f.write_str("complement_64"),
            Core::Complement8 => f.write_str("complement_8"),
            Core::Decompress => f.write_str("decompress"),
            Core::Decrement16 => f.write_str("decrement_16"),
            Core::Decrement32 => f.write_str("decrement_32"),
            Core::Decrement64 => f.write_str("decrement_64"),
            Core::Decrement8 => f.write_str("decrement_8"),
            Core::DivMod16 => f.write_str("div_mod_16"),
            Core::DivMod32 => f.write_str("div_mod_32"),
            Core::DivMod64 => f.write_str("div_mod_64"),
            Core::DivMod8 => f.write_str("div_mod_8"),
            Core::Divide16 => f.write_str("divide_16"),
            Core::Divide32 => f.write_str("divide_32"),
            Core::Divide64 => f.write_str("divide_64"),
            Core::Divide8 => f.write_str("divide_8"),
            Core::Divides16 => f.write_str("divides_16"),
            Core::Divides32 => f.write_str("divides_32"),
            Core::Divides64 => f.write_str("divides_64"),
            Core::Divides8 => f.write_str("divides_8"),
            Core::Eq16 => f.write_str("eq_16"),
            Core::Eq256 => f.write_str("eq_256"),
            Core::Eq32 => f.write_str("eq_32"),
            Core::Eq64 => f.write_str("eq_64"),
            Core::Eq8 => f.write_str("eq_8"),
            Core::FeAdd => f.write_str("fe_add"),
            Core::FeInvert => f.write_str("fe_invert"),
            Core::FeIsOdd => f.write_str("fe_is_odd"),
            Core::FeIsZero => f.write_str("fe_is_zero"),
            Core::FeMultiply => f.write_str("fe_multiply"),
            Core::FeMultiplyBeta => f.write_str("fe_multiply_beta"),
            Core::FeNegate => f.write_str("fe_negate"),
            Core::FeNormalize => f.write_str("fe_normalize"),
            Core::FeSquare => f.write_str("fe_square"),
            Core::FeSquareRoot => f.write_str("fe_square_root"),
            Core::FullAdd16 => f.write_str("full_add_16"),
            Core::FullAdd32 => f.write_str("full_add_32"),
            Core::FullAdd64 => f.write_str("full_add_64"),
            Core::FullAdd8 => f.write_str("full_add_8"),
            Core::FullDecrement16 => f.write_str("full_decrement_16"),
            Core::FullDecrement32 => f.write_str("full_decrement_32"),
            Core::FullDecrement64 => f.write_str("full_decrement_64"),
            Core::FullDecrement8 => f.write_str("full_decrement_8"),
            Core::FullIncrement16 => f.write_str("full_increment_16"),
            Core::FullIncrement32 => f.write_str("full_increment_32"),
            Core::FullIncrement64 => f.write_str("full_increment_64"),
            Core::FullIncrement8 => f.write_str("full_increment_8"),
            Core::FullMultiply16 => f.write_str("full_multiply_16"),
            Core::FullMultiply32 => f.write_str("full_multiply_32"),
            Core::FullMultiply64 => f.write_str("full_multiply_64"),
            Core::FullMultiply8 => f.write_str("full_multiply_8"),
            Core::FullSubtract16 => f.write_str("full_subtract_16"),
            Core::FullSubtract32 => f.write_str("full_subtract_32"),
            Core::FullSubtract64 => f.write_str("full_subtract_64"),
            Core::FullSubtract8 => f.write_str("full_subtract_8"),
            Core::GeIsOnCurve => f.write_str("ge_is_on_curve"),
            Core::GeNegate => f.write_str("ge_negate"),
            Core::GejAdd => f.write_str("gej_add"),
            Core::GejDouble => f.write_str("gej_double"),
            Core::GejGeAdd => f.write_str("gej_ge_add"),
            Core::GejGeAddEx => f.write_str("gej_ge_add_ex"),
            Core::GejInfinity => f.write_str("gej_infinity"),
            Core::GejIsInfinity => f.write_str("gej_is_infinity"),
            Core::GejIsOnCurve => f.write_str("gej_is_on_curve"),
            Core::GejNegate => f.write_str("gej_negate"),
            Core::GejNormalize => f.write_str("gej_normalize"),
            Core::GejRescale => f.write_str("gej_rescale"),
            Core::GejXEquiv => f.write_str("gej_x_equiv"),
            Core::GejYIsOdd => f.write_str("gej_y_is_odd"),
            Core::Generate => f.write_str("generate"),
            Core::High16 => f.write_str("high_16"),
            Core::High32 => f.write_str("high_32"),
            Core::High64 => f.write_str("high_64"),
            Core::High8 => f.write_str("high_8"),
            Core::Increment16 => f.write_str("increment_16"),
            Core::Increment32 => f.write_str("increment_32"),
            Core::Increment64 => f.write_str("increment_64"),
            Core::Increment8 => f.write_str("increment_8"),
            Core::IsOne16 => f.write_str("is_one_16"),
            Core::IsOne32 => f.write_str("is_one_32"),
            Core::IsOne64 => f.write_str("is_one_64"),
            Core::IsOne8 => f.write_str("is_one_8"),
            Core::IsZero16 => f.write_str("is_zero_16"),
            Core::IsZero32 => f.write_str("is_zero_32"),
            Core::IsZero64 => f.write_str("is_zero_64"),
            Core::IsZero8 => f.write_str("is_zero_8"),
            Core::Le16 => f.write_str("le_16"),
            Core::Le32 => f.write_str("le_32"),
            Core::Le64 => f.write_str("le_64"),
            Core::Le8 => f.write_str("le_8"),
            Core::LinearCombination1 => f.write_str("linear_combination_1"),
            Core::LinearVerify1 => f.write_str("linear_verify_1"),
            Core::Low16 => f.write_str("low_16"),
            Core::Low32 => f.write_str("low_32"),
            Core::Low64 => f.write_str("low_64"),
            Core::Low8 => f.write_str("low_8"),
            Core::Lt16 => f.write_str("lt_16"),
            Core::Lt32 => f.write_str("lt_32"),
            Core::Lt64 => f.write_str("lt_64"),
            Core::Lt8 => f.write_str("lt_8"),
            Core::Maj16 => f.write_str("maj_16"),
            Core::Maj32 => f.write_str("maj_32"),
            Core::Maj64 => f.write_str("maj_64"),
            Core::Maj8 => f.write_str("maj_8"),
            Core::Max16 => f.write_str("max_16"),
            Core::Max32 => f.write_str("max_32"),
            Core::Max64 => f.write_str("max_64"),
            Core::Max8 => f.write_str("max_8"),
            Core::Median16 => f.write_str("median_16"),
            Core::Median32 => f.write_str("median_32"),
            Core::Median64 => f.write_str("median_64"),
            Core::Median8 => f.write_str("median_8"),
            Core::Min16 => f.write_str("min_16"),
            Core::Min32 => f.write_str("min_32"),
            Core::Min64 => f.write_str("min_64"),
            Core::Min8 => f.write_str("min_8"),
            Core::Modulo16 => f.write_str("modulo_16"),
            Core::Modulo32 => f.write_str("modulo_32"),
            Core::Modulo64 => f.write_str("modulo_64"),
            Core::Modulo8 => f.write_str("modulo_8"),
            Core::Multiply16 => f.write_str("multiply_16"),
            Core::Multiply32 => f.write_str("multiply_32"),
            Core::Multiply64 => f.write_str("multiply_64"),
            Core::Multiply8 => f.write_str("multiply_8"),
            Core::Negate16 => f.write_str("negate_16"),
            Core::Negate32 => f.write_str("negate_32"),
            Core::Negate64 => f.write_str("negate_64"),
            Core::Negate8 => f.write_str("negate_8"),
            Core::One16 => f.write_str("one_16"),
            Core::One32 => f.write_str("one_32"),
            Core::One64 => f.write_str("one_64"),
            Core::One8 => f.write_str("one_8"),
            Core::Or16 => f.write_str("or_16"),
            Core::Or32 => f.write_str("or_32"),
            Core::Or64 => f.write_str("or_64"),
            Core::Or8 => f.write_str("or_8"),
            Core::ParseLock => f.write_str("parse_lock"),
            Core::ParseSequence => f.write_str("parse_sequence"),
            Core::PointVerify1 => f.write_str("point_verify_1"),
            Core::ScalarAdd => f.write_str("scalar_add"),
            Core::ScalarInvert => f.write_str("scalar_invert"),
            Core::ScalarIsZero => f.write_str("scalar_is_zero"),
            Core::ScalarMultiply => f.write_str("scalar_multiply"),
            Core::ScalarMultiplyLambda => f.write_str("scalar_multiply_lambda"),
            Core::ScalarNegate => f.write_str("scalar_negate"),
            Core::ScalarNormalize => f.write_str("scalar_normalize"),
            Core::ScalarSquare => f.write_str("scalar_square"),
            Core::Scale => f.write_str("scale"),
            Core::Sha256Block => f.write_str("sha_256_block"),
            Core::Sha256Ctx8Add1 => f.write_str("sha_256_ctx_8_add_1"),
            Core::Sha256Ctx8Add128 => f.write_str("sha_256_ctx_8_add_128"),
            Core::Sha256Ctx8Add16 => f.write_str("sha_256_ctx_8_add_16"),
            Core::Sha256Ctx8Add2 => f.write_str("sha_256_ctx_8_add_2"),
            Core::Sha256Ctx8Add256 => f.write_str("sha_256_ctx_8_add_256"),
            Core::Sha256Ctx8Add32 => f.write_str("sha_256_ctx_8_add_32"),
            Core::Sha256Ctx8Add4 => f.write_str("sha_256_ctx_8_add_4"),
            Core::Sha256Ctx8Add512 => f.write_str("sha_256_ctx_8_add_512"),
            Core::Sha256Ctx8Add64 => f.write_str("sha_256_ctx_8_add_64"),
            Core::Sha256Ctx8Add8 => f.write_str("sha_256_ctx_8_add_8"),
            Core::Sha256Ctx8AddBuffer511 => f.write_str("sha_256_ctx_8_add_buffer_511"),
            Core::Sha256Ctx8Finalize => f.write_str("sha_256_ctx_8_finalize"),
            Core::Sha256Ctx8Init => f.write_str("sha_256_ctx_8_init"),
            Core::Sha256Iv => f.write_str("sha_256_iv"),
            Core::Some16 => f.write_str("some_16"),
            Core::Some32 => f.write_str("some_32"),
            Core::Some64 => f.write_str("some_64"),
            Core::Some8 => f.write_str("some_8"),
            Core::Subtract16 => f.write_str("subtract_16"),
            Core::Subtract32 => f.write_str("subtract_32"),
            Core::Subtract64 => f.write_str("subtract_64"),
            Core::Subtract8 => f.write_str("subtract_8"),
            Core::Verify => f.write_str("verify"),
            Core::Xor16 => f.write_str("xor_16"),
            Core::Xor32 => f.write_str("xor_32"),
            Core::Xor64 => f.write_str("xor_64"),
            Core::Xor8 => f.write_str("xor_8"),
            Core::XorXor16 => f.write_str("xor_xor_16"),
            Core::XorXor32 => f.write_str("xor_xor_32"),
            Core::XorXor64 => f.write_str("xor_xor_64"),
            Core::XorXor8 => f.write_str("xor_xor_8"),
        }
    }
}

impl str::FromStr for Core {
    type Err = crate::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "add_16" => Ok(Core::Add16),
            "add_32" => Ok(Core::Add32),
            "add_64" => Ok(Core::Add64),
            "add_8" => Ok(Core::Add8),
            "all_16" => Ok(Core::All16),
            "all_32" => Ok(Core::All32),
            "all_64" => Ok(Core::All64),
            "all_8" => Ok(Core::All8),
            "and_16" => Ok(Core::And16),
            "and_32" => Ok(Core::And32),
            "and_64" => Ok(Core::And64),
            "and_8" => Ok(Core::And8),
            "bip_0340_verify" => Ok(Core::Bip0340Verify),
            "ch_16" => Ok(Core::Ch16),
            "ch_32" => Ok(Core::Ch32),
            "ch_64" => Ok(Core::Ch64),
            "ch_8" => Ok(Core::Ch8),
            "check_sig_verify" => Ok(Core::CheckSigVerify),
            "complement_16" => Ok(Core::Complement16),
            "complement_32" => Ok(Core::Complement32),
            "complement_64" => Ok(Core::Complement64),
            "complement_8" => Ok(Core::Complement8),
            "decompress" => Ok(Core::Decompress),
            "decrement_16" => Ok(Core::Decrement16),
            "decrement_32" => Ok(Core::Decrement32),
            "decrement_64" => Ok(Core::Decrement64),
            "decrement_8" => Ok(Core::Decrement8),
            "div_mod_16" => Ok(Core::DivMod16),
            "div_mod_32" => Ok(Core::DivMod32),
            "div_mod_64" => Ok(Core::DivMod64),
            "div_mod_8" => Ok(Core::DivMod8),
            "divide_16" => Ok(Core::Divide16),
            "divide_32" => Ok(Core::Divide32),
            "divide_64" => Ok(Core::Divide64),
            "divide_8" => Ok(Core::Divide8),
            "divides_16" => Ok(Core::Divides16),
            "divides_32" => Ok(Core::Divides32),
            "divides_64" => Ok(Core::Divides64),
            "divides_8" => Ok(Core::Divides8),
            "eq_16" => Ok(Core::Eq16),
            "eq_256" => Ok(Core::Eq256),
            "eq_32" => Ok(Core::Eq32),
            "eq_64" => Ok(Core::Eq64),
            "eq_8" => Ok(Core::Eq8),
            "fe_add" => Ok(Core::FeAdd),
            "fe_invert" => Ok(Core::FeInvert),
            "fe_is_odd" => Ok(Core::FeIsOdd),
            "fe_is_zero" => Ok(Core::FeIsZero),
            "fe_multiply" => Ok(Core::FeMultiply),
            "fe_multiply_beta" => Ok(Core::FeMultiplyBeta),
            "fe_negate" => Ok(Core::FeNegate),
            "fe_normalize" => Ok(Core::FeNormalize),
            "fe_square" => Ok(Core::FeSquare),
            "fe_square_root" => Ok(Core::FeSquareRoot),
            "full_add_16" => Ok(Core::FullAdd16),
            "full_add_32" => Ok(Core::FullAdd32),
            "full_add_64" => Ok(Core::FullAdd64),
            "full_add_8" => Ok(Core::FullAdd8),
            "full_decrement_16" => Ok(Core::FullDecrement16),
            "full_decrement_32" => Ok(Core::FullDecrement32),
            "full_decrement_64" => Ok(Core::FullDecrement64),
            "full_decrement_8" => Ok(Core::FullDecrement8),
            "full_increment_16" => Ok(Core::FullIncrement16),
            "full_increment_32" => Ok(Core::FullIncrement32),
            "full_increment_64" => Ok(Core::FullIncrement64),
            "full_increment_8" => Ok(Core::FullIncrement8),
            "full_multiply_16" => Ok(Core::FullMultiply16),
            "full_multiply_32" => Ok(Core::FullMultiply32),
            "full_multiply_64" => Ok(Core::FullMultiply64),
            "full_multiply_8" => Ok(Core::FullMultiply8),
            "full_subtract_16" => Ok(Core::FullSubtract16),
            "full_subtract_32" => Ok(Core::FullSubtract32),
            "full_subtract_64" => Ok(Core::FullSubtract64),
            "full_subtract_8" => Ok(Core::FullSubtract8),
            "ge_is_on_curve" => Ok(Core::GeIsOnCurve),
            "ge_negate" => Ok(Core::GeNegate),
            "gej_add" => Ok(Core::GejAdd),
            "gej_double" => Ok(Core::GejDouble),
            "gej_ge_add" => Ok(Core::GejGeAdd),
            "gej_ge_add_ex" => Ok(Core::GejGeAddEx),
            "gej_infinity" => Ok(Core::GejInfinity),
            "gej_is_infinity" => Ok(Core::GejIsInfinity),
            "gej_is_on_curve" => Ok(Core::GejIsOnCurve),
            "gej_negate" => Ok(Core::GejNegate),
            "gej_normalize" => Ok(Core::GejNormalize),
            "gej_rescale" => Ok(Core::GejRescale),
            "gej_x_equiv" => Ok(Core::GejXEquiv),
            "gej_y_is_odd" => Ok(Core::GejYIsOdd),
            "generate" => Ok(Core::Generate),
            "high_16" => Ok(Core::High16),
            "high_32" => Ok(Core::High32),
            "high_64" => Ok(Core::High64),
            "high_8" => Ok(Core::High8),
            "increment_16" => Ok(Core::Increment16),
            "increment_32" => Ok(Core::Increment32),
            "increment_64" => Ok(Core::Increment64),
            "increment_8" => Ok(Core::Increment8),
            "is_one_16" => Ok(Core::IsOne16),
            "is_one_32" => Ok(Core::IsOne32),
            "is_one_64" => Ok(Core::IsOne64),
            "is_one_8" => Ok(Core::IsOne8),
            "is_zero_16" => Ok(Core::IsZero16),
            "is_zero_32" => Ok(Core::IsZero32),
            "is_zero_64" => Ok(Core::IsZero64),
            "is_zero_8" => Ok(Core::IsZero8),
            "le_16" => Ok(Core::Le16),
            "le_32" => Ok(Core::Le32),
            "le_64" => Ok(Core::Le64),
            "le_8" => Ok(Core::Le8),
            "linear_combination_1" => Ok(Core::LinearCombination1),
            "linear_verify_1" => Ok(Core::LinearVerify1),
            "low_16" => Ok(Core::Low16),
            "low_32" => Ok(Core::Low32),
            "low_64" => Ok(Core::Low64),
            "low_8" => Ok(Core::Low8),
            "lt_16" => Ok(Core::Lt16),
            "lt_32" => Ok(Core::Lt32),
            "lt_64" => Ok(Core::Lt64),
            "lt_8" => Ok(Core::Lt8),
            "maj_16" => Ok(Core::Maj16),
            "maj_32" => Ok(Core::Maj32),
            "maj_64" => Ok(Core::Maj64),
            "maj_8" => Ok(Core::Maj8),
            "max_16" => Ok(Core::Max16),
            "max_32" => Ok(Core::Max32),
            "max_64" => Ok(Core::Max64),
            "max_8" => Ok(Core::Max8),
            "median_16" => Ok(Core::Median16),
            "median_32" => Ok(Core::Median32),
            "median_64" => Ok(Core::Median64),
            "median_8" => Ok(Core::Median8),
            "min_16" => Ok(Core::Min16),
            "min_32" => Ok(Core::Min32),
            "min_64" => Ok(Core::Min64),
            "min_8" => Ok(Core::Min8),
            "modulo_16" => Ok(Core::Modulo16),
            "modulo_32" => Ok(Core::Modulo32),
            "modulo_64" => Ok(Core::Modulo64),
            "modulo_8" => Ok(Core::Modulo8),
            "multiply_16" => Ok(Core::Multiply16),
            "multiply_32" => Ok(Core::Multiply32),
            "multiply_64" => Ok(Core::Multiply64),
            "multiply_8" => Ok(Core::Multiply8),
            "negate_16" => Ok(Core::Negate16),
            "negate_32" => Ok(Core::Negate32),
            "negate_64" => Ok(Core::Negate64),
            "negate_8" => Ok(Core::Negate8),
            "one_16" => Ok(Core::One16),
            "one_32" => Ok(Core::One32),
            "one_64" => Ok(Core::One64),
            "one_8" => Ok(Core::One8),
            "or_16" => Ok(Core::Or16),
            "or_32" => Ok(Core::Or32),
            "or_64" => Ok(Core::Or64),
            "or_8" => Ok(Core::Or8),
            "parse_lock" => Ok(Core::ParseLock),
            "parse_sequence" => Ok(Core::ParseSequence),
            "point_verify_1" => Ok(Core::PointVerify1),
            "scalar_add" => Ok(Core::ScalarAdd),
            "scalar_invert" => Ok(Core::ScalarInvert),
            "scalar_is_zero" => Ok(Core::ScalarIsZero),
            "scalar_multiply" => Ok(Core::ScalarMultiply),
            "scalar_multiply_lambda" => Ok(Core::ScalarMultiplyLambda),
            "scalar_negate" => Ok(Core::ScalarNegate),
            "scalar_normalize" => Ok(Core::ScalarNormalize),
            "scalar_square" => Ok(Core::ScalarSquare),
            "scale" => Ok(Core::Scale),
            "sha_256_block" => Ok(Core::Sha256Block),
            "sha_256_ctx_8_add_1" => Ok(Core::Sha256Ctx8Add1),
            "sha_256_ctx_8_add_128" => Ok(Core::Sha256Ctx8Add128),
            "sha_256_ctx_8_add_16" => Ok(Core::Sha256Ctx8Add16),
            "sha_256_ctx_8_add_2" => Ok(Core::Sha256Ctx8Add2),
            "sha_256_ctx_8_add_256" => Ok(Core::Sha256Ctx8Add256),
            "sha_256_ctx_8_add_32" => Ok(Core::Sha256Ctx8Add32),
            "sha_256_ctx_8_add_4" => Ok(Core::Sha256Ctx8Add4),
            "sha_256_ctx_8_add_512" => Ok(Core::Sha256Ctx8Add512),
            "sha_256_ctx_8_add_64" => Ok(Core::Sha256Ctx8Add64),
            "sha_256_ctx_8_add_8" => Ok(Core::Sha256Ctx8Add8),
            "sha_256_ctx_8_add_buffer_511" => Ok(Core::Sha256Ctx8AddBuffer511),
            "sha_256_ctx_8_finalize" => Ok(Core::Sha256Ctx8Finalize),
            "sha_256_ctx_8_init" => Ok(Core::Sha256Ctx8Init),
            "sha_256_iv" => Ok(Core::Sha256Iv),
            "some_16" => Ok(Core::Some16),
            "some_32" => Ok(Core::Some32),
            "some_64" => Ok(Core::Some64),
            "some_8" => Ok(Core::Some8),
            "subtract_16" => Ok(Core::Subtract16),
            "subtract_32" => Ok(Core::Subtract32),
            "subtract_64" => Ok(Core::Subtract64),
            "subtract_8" => Ok(Core::Subtract8),
            "verify" => Ok(Core::Verify),
            "xor_16" => Ok(Core::Xor16),
            "xor_32" => Ok(Core::Xor32),
            "xor_64" => Ok(Core::Xor64),
            "xor_8" => Ok(Core::Xor8),
            "xor_xor_16" => Ok(Core::XorXor16),
            "xor_xor_32" => Ok(Core::XorXor32),
            "xor_xor_64" => Ok(Core::XorXor64),
            "xor_xor_8" => Ok(Core::XorXor8),
            x => Err(crate::Error::InvalidJetName(x.to_owned())),
        }
    }
}
