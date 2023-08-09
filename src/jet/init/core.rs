/* This file has been automatically generated. */

use crate::jet::type_name::TypeName;
use crate::jet::Jet;
use crate::merkle::cmr::Cmr;
use crate::decode_bits;
use crate::{decode, BitIter, BitWriter};
use crate::analysis::Cost;
use hashes::sha256::Midstate;
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
                0xab, 0x71, 0x0d, 0x45, 0xbd, 0xda, 0x1e, 0xe2, 0x4e, 0xb1, 0x8f, 0x64, 0xb9, 0x55,
                0xac, 0xc2, 0x7f, 0x07, 0x63, 0x9e, 0x22, 0xa0, 0x29, 0x05, 0x38, 0x12, 0xb6, 0xf5,
                0xac, 0xd0, 0x0c, 0x05,
            ],
            Core::Add32 => [
                0x91, 0xed, 0x4c, 0x3d, 0x0d, 0x94, 0xd1, 0x11, 0xc7, 0x2b, 0xd4, 0x31, 0xc0, 0xac,
                0xbb, 0x0d, 0xe1, 0xc5, 0x89, 0x95, 0x64, 0x0b, 0x58, 0xcf, 0x70, 0xd3, 0x7e, 0xad,
                0xb7, 0x40, 0x10, 0x1a,
            ],
            Core::Add64 => [
                0xa5, 0xe6, 0x55, 0x6a, 0x21, 0xcc, 0xe1, 0x05, 0xe9, 0x94, 0xe1, 0x65, 0xa4, 0x03,
                0x65, 0x7e, 0xea, 0x7e, 0xef, 0x36, 0x1b, 0x10, 0x41, 0x04, 0x5f, 0xf3, 0x26, 0xa3,
                0x27, 0x02, 0xaa, 0xfe,
            ],
            Core::Add8 => [
                0x51, 0xe5, 0xaf, 0x2c, 0x99, 0xe0, 0xd4, 0xd3, 0x93, 0x43, 0x0b, 0x3e, 0xd3, 0x2d,
                0xb0, 0x83, 0xa9, 0xa0, 0x20, 0x64, 0x6d, 0x36, 0x64, 0x4e, 0x54, 0x70, 0xbb, 0x9d,
                0xb9, 0xfd, 0x0e, 0x22,
            ],
            Core::All16 => [
                0xcc, 0xc0, 0x06, 0x86, 0x73, 0xe3, 0x0d, 0x34, 0x42, 0xe6, 0x16, 0x48, 0x7a, 0x4b,
                0x53, 0xa1, 0x95, 0xf5, 0xc2, 0xcf, 0x13, 0x89, 0x1c, 0xc6, 0x3f, 0x7e, 0x86, 0x2b,
                0xa0, 0xd9, 0x29, 0xa8,
            ],
            Core::All32 => [
                0xb2, 0xc0, 0xd7, 0x7f, 0xeb, 0xc6, 0xf1, 0x73, 0xb1, 0x42, 0xc2, 0xb6, 0x2b, 0xb8,
                0xf5, 0xfd, 0xbd, 0x37, 0x9a, 0x1d, 0x7d, 0xdf, 0x89, 0x51, 0x33, 0x60, 0xa0, 0x2e,
                0xeb, 0x41, 0x94, 0x00,
            ],
            Core::All64 => [
                0x1c, 0x9a, 0x3a, 0x89, 0x5a, 0x18, 0xe1, 0xe7, 0x81, 0x82, 0x13, 0x0d, 0xdf, 0xc8,
                0x97, 0x2a, 0x28, 0xae, 0x45, 0xe4, 0xb4, 0xea, 0xfd, 0x1b, 0xcf, 0xed, 0x5d, 0xd6,
                0x09, 0x06, 0x57, 0xff,
            ],
            Core::All8 => [
                0xd7, 0xf5, 0x0e, 0xc5, 0x65, 0x03, 0xb3, 0x29, 0x96, 0x95, 0x9d, 0xe1, 0xad, 0x48,
                0x67, 0xb8, 0xb0, 0x59, 0xd9, 0x6a, 0xc8, 0xcc, 0x4e, 0x1d, 0x4f, 0x89, 0x60, 0x7f,
                0x06, 0x0e, 0xd1, 0x46,
            ],
            Core::And16 => [
                0x92, 0x2f, 0x86, 0x4b, 0xa8, 0xdc, 0x2b, 0x6b, 0x9d, 0x31, 0x63, 0x4d, 0x31, 0x16,
                0xcb, 0x38, 0xd1, 0xae, 0x0d, 0x7c, 0x21, 0x50, 0xea, 0x1d, 0x4a, 0xec, 0x1f, 0xa3,
                0x93, 0xac, 0xf6, 0xf2,
            ],
            Core::And32 => [
                0xe8, 0xad, 0x05, 0xb4, 0x6c, 0xf6, 0x2c, 0x0d, 0x48, 0x45, 0x42, 0xa6, 0xbb, 0x43,
                0xf9, 0xaa, 0xdf, 0x0e, 0x14, 0x8b, 0xd3, 0x45, 0xe6, 0x62, 0x51, 0x17, 0xab, 0x61,
                0xed, 0xea, 0x0b, 0xd5,
            ],
            Core::And64 => [
                0x79, 0x71, 0xf1, 0x8c, 0x00, 0xcf, 0xf1, 0x66, 0xa0, 0xd2, 0x1b, 0x7a, 0x18, 0xf0,
                0xed, 0xc8, 0x0e, 0xc7, 0xd3, 0x92, 0x71, 0xe4, 0x13, 0xef, 0x08, 0x25, 0xe4, 0x97,
                0xb0, 0xca, 0x2d, 0x1c,
            ],
            Core::And8 => [
                0x58, 0x33, 0x65, 0x36, 0x0b, 0xec, 0xef, 0x53, 0xc3, 0x19, 0x0d, 0xcd, 0xa5, 0x45,
                0x16, 0x75, 0xd7, 0x20, 0xdf, 0x45, 0xb0, 0x67, 0x43, 0x40, 0x8b, 0xcd, 0x66, 0x43,
                0xae, 0x35, 0x80, 0xdb,
            ],
            Core::Bip0340Verify => [
                0xea, 0x0f, 0x81, 0xfa, 0x0a, 0xc2, 0xe2, 0x03, 0x62, 0xcb, 0x09, 0x61, 0x2d, 0x19,
                0x9f, 0xcc, 0x45, 0xbe, 0x91, 0xb0, 0xd5, 0x8d, 0x87, 0x82, 0x25, 0x72, 0xe9, 0xec,
                0xe2, 0x4e, 0x10, 0xc1,
            ],
            Core::Ch16 => [
                0xc6, 0xdd, 0x71, 0x70, 0xc5, 0xf6, 0x72, 0x68, 0x31, 0x7e, 0x19, 0x5e, 0x8b, 0x30,
                0xea, 0xf3, 0x4c, 0x23, 0xb0, 0x0c, 0x5a, 0xb9, 0xc3, 0x1e, 0xed, 0x33, 0xe5, 0x2d,
                0xbf, 0x00, 0xf5, 0x12,
            ],
            Core::Ch32 => [
                0x51, 0xbb, 0x48, 0x45, 0x66, 0x4f, 0x60, 0x1d, 0xa4, 0x0d, 0xaf, 0x5e, 0x2a, 0x6f,
                0x0e, 0x1b, 0x4f, 0xed, 0x70, 0x5c, 0xb0, 0x00, 0x60, 0x59, 0x35, 0x8a, 0x06, 0xd6,
                0x74, 0x3e, 0xba, 0xb9,
            ],
            Core::Ch64 => [
                0x16, 0x87, 0x09, 0x97, 0xe0, 0xfb, 0x1f, 0xbe, 0xfe, 0x81, 0xcc, 0x21, 0x6d, 0x72,
                0x4d, 0x6f, 0x44, 0xb3, 0x16, 0x3b, 0x33, 0x31, 0x78, 0xff, 0x36, 0x06, 0xc5, 0x67,
                0x24, 0x82, 0x96, 0x57,
            ],
            Core::Ch8 => [
                0x1f, 0x6e, 0x22, 0x5b, 0x36, 0x9a, 0x75, 0x89, 0x0b, 0xd5, 0xd7, 0xa6, 0xca, 0x40,
                0xf4, 0x83, 0x47, 0xf1, 0xb1, 0x94, 0xf5, 0x14, 0x7a, 0x1b, 0x1b, 0x10, 0xf8, 0x60,
                0xf9, 0x35, 0x22, 0xa8,
            ],
            Core::CheckSigVerify => [
                0xe8, 0xfe, 0xb5, 0x34, 0x34, 0xe2, 0xbd, 0xb5, 0xcf, 0xe6, 0xa9, 0x1f, 0xa8, 0xf9,
                0xe1, 0x77, 0xf9, 0x41, 0xa6, 0x7c, 0xc6, 0xce, 0xd8, 0x69, 0x74, 0x6f, 0x1a, 0x8e,
                0xb6, 0x50, 0x6f, 0x76,
            ],
            Core::Complement16 => [
                0x35, 0x3d, 0x1a, 0x7f, 0x4d, 0x54, 0x79, 0x55, 0x42, 0x29, 0xc7, 0x43, 0xed, 0xa6,
                0x5b, 0x6a, 0xe2, 0x01, 0x9a, 0x36, 0x51, 0x96, 0x9b, 0xd6, 0x52, 0xa1, 0x42, 0xc3,
                0x19, 0x04, 0xdc, 0xba,
            ],
            Core::Complement32 => [
                0x29, 0xe5, 0x39, 0xb4, 0xf8, 0x60, 0x4e, 0x79, 0xe7, 0x5c, 0x17, 0x90, 0x88, 0xeb,
                0x73, 0x08, 0xeb, 0x10, 0x8c, 0xc6, 0xe3, 0x34, 0xc5, 0x09, 0x81, 0xc8, 0x8b, 0x70,
                0x4f, 0x1c, 0x7c, 0x5b,
            ],
            Core::Complement64 => [
                0x16, 0x28, 0x87, 0x3a, 0x31, 0xd5, 0xaa, 0xcf, 0x55, 0x78, 0xba, 0x26, 0xc5, 0x8b,
                0xb7, 0x95, 0x3b, 0xf6, 0x2b, 0x87, 0x1a, 0x15, 0x98, 0x77, 0xb1, 0x48, 0x84, 0x0c,
                0x22, 0xe8, 0x59, 0x8d,
            ],
            Core::Complement8 => [
                0x13, 0xd1, 0xc5, 0x24, 0xc9, 0xf6, 0xe8, 0x8f, 0xda, 0x60, 0xa8, 0x9f, 0x41, 0xb8,
                0x2d, 0xa7, 0xbd, 0x98, 0x8d, 0x98, 0xf9, 0x0f, 0x6b, 0xfa, 0x69, 0x74, 0x16, 0x7d,
                0xe7, 0x31, 0x4c, 0x30,
            ],
            Core::Decompress => [
                0x41, 0x9c, 0x87, 0xe5, 0xe9, 0x03, 0x4b, 0x5d, 0x45, 0x3f, 0xa6, 0x68, 0x19, 0xdb,
                0x5a, 0x36, 0xa4, 0xa5, 0x84, 0x35, 0x68, 0x83, 0x3e, 0x85, 0x70, 0xec, 0x77, 0x29,
                0xb1, 0x0a, 0x40, 0xa5,
            ],
            Core::Decrement16 => [
                0xb4, 0x83, 0xb1, 0xe9, 0xd9, 0xef, 0x74, 0x10, 0x68, 0x33, 0xf6, 0x6f, 0x68, 0x0c,
                0x22, 0x6e, 0xf6, 0x98, 0x22, 0x3c, 0x26, 0xcf, 0x8d, 0xe5, 0x72, 0xed, 0x54, 0x0f,
                0x2c, 0x12, 0x6d, 0xaf,
            ],
            Core::Decrement32 => [
                0x82, 0x66, 0xc0, 0x7d, 0x1c, 0xbf, 0x8f, 0x65, 0xa3, 0xeb, 0x8f, 0xd3, 0x2e, 0xf6,
                0x06, 0x01, 0xce, 0x28, 0x9f, 0xfb, 0x03, 0xae, 0x3e, 0x37, 0x5b, 0x9c, 0x76, 0x10,
                0x35, 0x7f, 0xe9, 0xdf,
            ],
            Core::Decrement64 => [
                0x6e, 0xe6, 0xd1, 0xa4, 0x96, 0x5d, 0x37, 0x85, 0xbf, 0xf0, 0xa3, 0xe1, 0xe1, 0x69,
                0x0b, 0x16, 0xfd, 0xce, 0x2b, 0x32, 0x61, 0xc7, 0x77, 0x19, 0xaf, 0xb6, 0x9c, 0x8f,
                0xab, 0x4b, 0x9a, 0x5a,
            ],
            Core::Decrement8 => [
                0xa7, 0x5f, 0xe6, 0x22, 0x50, 0x99, 0x3f, 0xb4, 0xd5, 0x37, 0xae, 0x86, 0xf3, 0x01,
                0x45, 0x19, 0x24, 0x66, 0xdf, 0x31, 0x27, 0xd5, 0xac, 0x2d, 0x42, 0x0a, 0x7b, 0x71,
                0x74, 0x98, 0xb5, 0x48,
            ],
            Core::DivMod16 => [
                0x07, 0x1a, 0x9f, 0xc0, 0xa5, 0x34, 0xe7, 0x52, 0x49, 0x28, 0x74, 0xa9, 0xdc, 0x1e,
                0xc5, 0x31, 0xd0, 0x7d, 0x0c, 0x22, 0x09, 0x95, 0xce, 0xd1, 0x22, 0x7a, 0x1a, 0xc1,
                0xe3, 0xe7, 0xf1, 0xa7,
            ],
            Core::DivMod32 => [
                0x93, 0x7f, 0x39, 0xaa, 0x8e, 0x26, 0xa7, 0x66, 0x52, 0xed, 0x51, 0xcd, 0xb7, 0xd4,
                0x89, 0xb9, 0x33, 0x27, 0x3f, 0x27, 0x5b, 0xde, 0x68, 0xf5, 0x4e, 0x59, 0xb4, 0x9b,
                0x26, 0x6b, 0x6f, 0x8c,
            ],
            Core::DivMod64 => [
                0xa2, 0x68, 0x17, 0x27, 0x26, 0x40, 0x28, 0x56, 0x53, 0x71, 0x39, 0xcd, 0x0a, 0x23,
                0xfe, 0x34, 0x03, 0xb7, 0x65, 0xef, 0x74, 0xb5, 0x77, 0xdc, 0x61, 0xfd, 0xb2, 0x0d,
                0x4d, 0x7a, 0xc4, 0xdf,
            ],
            Core::DivMod8 => [
                0xbf, 0x7c, 0xaf, 0x50, 0xce, 0xae, 0x7b, 0x12, 0xb3, 0x8d, 0x82, 0xc9, 0x3e, 0x46,
                0x01, 0x93, 0x69, 0x41, 0xf6, 0x05, 0x73, 0xdf, 0x98, 0xe7, 0x2a, 0x9c, 0x5a, 0x13,
                0x49, 0x07, 0x39, 0xd2,
            ],
            Core::Divide16 => [
                0x50, 0x79, 0x73, 0x13, 0xf7, 0x6d, 0xd6, 0x55, 0x85, 0xef, 0x82, 0x8c, 0xe0, 0x1c,
                0xdc, 0x13, 0x2a, 0x48, 0x9c, 0xda, 0xff, 0x43, 0x49, 0x86, 0xdd, 0x89, 0x99, 0xdb,
                0xde, 0xaa, 0x61, 0x7b,
            ],
            Core::Divide32 => [
                0x4d, 0xf2, 0xa1, 0x25, 0x5f, 0x07, 0x1f, 0x8a, 0x2b, 0xfc, 0x9a, 0x80, 0x5e, 0xdf,
                0x99, 0xd3, 0xc8, 0x90, 0xf5, 0xb1, 0xc4, 0xb8, 0x1a, 0x8f, 0xa6, 0xa9, 0x6c, 0x95,
                0xce, 0xa1, 0xa3, 0xfb,
            ],
            Core::Divide64 => [
                0xba, 0x62, 0xd1, 0x87, 0xb9, 0x60, 0x24, 0x30, 0xd0, 0x00, 0x1d, 0x91, 0xd1, 0x22,
                0x3f, 0xed, 0x79, 0x28, 0xd3, 0x20, 0x95, 0xae, 0xfc, 0x57, 0x22, 0xe4, 0xb3, 0x0c,
                0xe6, 0xac, 0x1c, 0xf4,
            ],
            Core::Divide8 => [
                0xcd, 0x0e, 0xb6, 0xf3, 0x2f, 0x77, 0xe2, 0xd9, 0x78, 0x08, 0x06, 0x4e, 0x96, 0x1d,
                0xab, 0x46, 0xee, 0x3e, 0x26, 0x47, 0xb2, 0x35, 0x7a, 0x7b, 0x58, 0x92, 0xd3, 0x6f,
                0xbc, 0x23, 0x2d, 0xc0,
            ],
            Core::Divides16 => [
                0x0a, 0xd6, 0x0f, 0x11, 0xf0, 0x12, 0x3e, 0x30, 0x4a, 0x9b, 0x77, 0x72, 0x5c, 0xa6,
                0x5e, 0x70, 0xb7, 0x10, 0x6b, 0x4d, 0x05, 0x64, 0x4a, 0xc6, 0xd8, 0x45, 0x2a, 0x56,
                0xe7, 0xfb, 0x6f, 0x90,
            ],
            Core::Divides32 => [
                0xd6, 0x5a, 0x47, 0x93, 0x65, 0xd8, 0xef, 0x47, 0xe4, 0x57, 0x01, 0x6c, 0xf3, 0xe4,
                0xc1, 0x27, 0x89, 0x10, 0x36, 0x68, 0x54, 0x92, 0xcb, 0x85, 0xf8, 0xcc, 0x2c, 0xc1,
                0x77, 0x17, 0x11, 0x13,
            ],
            Core::Divides64 => [
                0xb7, 0x3d, 0xdb, 0xb9, 0x32, 0xc8, 0xb8, 0x60, 0xe8, 0x2e, 0x99, 0x2a, 0xcc, 0x72,
                0x59, 0xa7, 0x8c, 0x0f, 0xdd, 0x32, 0x00, 0x90, 0x36, 0xef, 0x65, 0xc4, 0x76, 0x08,
                0x36, 0x8b, 0x03, 0xc8,
            ],
            Core::Divides8 => [
                0xe4, 0x24, 0x0f, 0x08, 0x24, 0xdf, 0x3a, 0xce, 0x65, 0x09, 0x36, 0x26, 0x00, 0x3a,
                0x6b, 0x97, 0x5d, 0x27, 0xfe, 0x89, 0xaf, 0x19, 0x99, 0x21, 0x4d, 0x43, 0x8b, 0x05,
                0xf1, 0xa1, 0x59, 0xed,
            ],
            Core::Eq16 => [
                0x05, 0xdf, 0xef, 0xd7, 0x4a, 0x26, 0xab, 0x79, 0x89, 0xa8, 0x50, 0x93, 0x31, 0xa0,
                0x3f, 0x33, 0x40, 0x18, 0x21, 0xa0, 0x1a, 0x75, 0x04, 0xe8, 0x09, 0xb1, 0x14, 0x12,
                0x82, 0x26, 0xd6, 0xfd,
            ],
            Core::Eq256 => [
                0xf5, 0x27, 0x49, 0xf1, 0xf2, 0xfc, 0xda, 0xaf, 0xa3, 0x62, 0x54, 0xf1, 0x63, 0xe6,
                0x70, 0x46, 0xe4, 0x88, 0x1e, 0xed, 0xca, 0xfd, 0x17, 0x01, 0x5e, 0x98, 0xe0, 0xcc,
                0x14, 0x41, 0xa5, 0x44,
            ],
            Core::Eq32 => [
                0xd4, 0x9e, 0xc0, 0x1c, 0x58, 0xa1, 0xd9, 0x55, 0xca, 0xeb, 0x56, 0xf7, 0xd7, 0xb0,
                0xf3, 0x9f, 0x18, 0x19, 0xa9, 0xd2, 0xbd, 0xcd, 0x34, 0xcb, 0x10, 0x8b, 0x63, 0xe3,
                0x12, 0x2e, 0x04, 0xc6,
            ],
            Core::Eq64 => [
                0xf1, 0x96, 0x89, 0xad, 0x94, 0x7e, 0x24, 0x55, 0x89, 0xa4, 0xe1, 0x87, 0x1a, 0x17,
                0x52, 0xc6, 0x66, 0x66, 0x6b, 0x43, 0x42, 0x79, 0x09, 0xfb, 0x3a, 0xcd, 0x95, 0xf0,
                0x8b, 0xc6, 0x93, 0x0a,
            ],
            Core::Eq8 => [
                0x7b, 0x7e, 0xb8, 0x29, 0x73, 0x8d, 0x38, 0x3c, 0xc2, 0x5f, 0xbd, 0x59, 0xc7, 0xc6,
                0x13, 0xbb, 0x96, 0x55, 0x7d, 0x82, 0xf5, 0x91, 0x4f, 0x81, 0x42, 0x1d, 0xac, 0x18,
                0xe8, 0x01, 0x53, 0x38,
            ],
            Core::FeAdd => [
                0x9f, 0x0f, 0xa0, 0xf8, 0x4d, 0x29, 0xd2, 0xc3, 0x6d, 0x98, 0xba, 0xaa, 0xdd, 0x1a,
                0xad, 0x0c, 0x9b, 0xd4, 0xe1, 0xae, 0x7d, 0xe9, 0x86, 0x92, 0xdd, 0x2a, 0x9c, 0x00,
                0xf4, 0x05, 0x03, 0xd4,
            ],
            Core::FeInvert => [
                0x01, 0x48, 0xa6, 0x06, 0x5a, 0x45, 0xb7, 0xe9, 0x56, 0x4b, 0x29, 0x09, 0x14, 0x07,
                0x3a, 0xad, 0xbb, 0x89, 0x5d, 0xdb, 0xf6, 0xff, 0x93, 0xf4, 0x5e, 0x55, 0x2c, 0x54,
                0x9d, 0x2e, 0x99, 0x00,
            ],
            Core::FeIsOdd => [
                0xd6, 0xb7, 0xd5, 0x44, 0xe0, 0x7d, 0xf3, 0x2a, 0x14, 0x0d, 0x43, 0x15, 0x4f, 0x49,
                0x03, 0xd7, 0x5c, 0x20, 0x1d, 0x80, 0x52, 0x64, 0xf2, 0xa4, 0xfb, 0x8c, 0x10, 0xe4,
                0x16, 0x49, 0x0c, 0x04,
            ],
            Core::FeIsZero => [
                0xaa, 0x0d, 0x4d, 0xc7, 0xe1, 0xd9, 0xf0, 0x2a, 0x38, 0xd1, 0xd3, 0x65, 0x8e, 0x1a,
                0xf7, 0xfe, 0x9c, 0x29, 0xdf, 0xba, 0xd0, 0xa0, 0x94, 0x38, 0x1a, 0xe7, 0xe5, 0xcd,
                0x63, 0x4b, 0x81, 0xe3,
            ],
            Core::FeMultiply => [
                0xd9, 0xc7, 0x4e, 0xb3, 0x9b, 0x8b, 0xad, 0x28, 0x21, 0xfd, 0xed, 0xfb, 0x77, 0x1f,
                0xbf, 0xf4, 0xc9, 0x05, 0x63, 0x1a, 0xbc, 0x50, 0xf2, 0x7c, 0xbf, 0x54, 0x78, 0x43,
                0xdd, 0x02, 0x56, 0x0d,
            ],
            Core::FeMultiplyBeta => [
                0x2f, 0x46, 0xce, 0xda, 0xf4, 0xae, 0x91, 0xf5, 0x29, 0x8c, 0xeb, 0x33, 0x04, 0xca,
                0x9d, 0xe8, 0xcc, 0xc4, 0x16, 0xe6, 0x7b, 0xa6, 0xf6, 0xa6, 0x06, 0xa6, 0x55, 0xb9,
                0x27, 0x92, 0x9a, 0x0f,
            ],
            Core::FeNegate => [
                0x6d, 0x39, 0xca, 0xe2, 0x6d, 0x28, 0x08, 0x7b, 0x4a, 0x97, 0xca, 0xe2, 0xd6, 0x47,
                0x90, 0xf4, 0xad, 0xe2, 0x58, 0x95, 0xc0, 0xa3, 0x7d, 0x31, 0x25, 0x83, 0x4a, 0x13,
                0xc7, 0x67, 0x0a, 0xce,
            ],
            Core::FeNormalize => [
                0x38, 0xc6, 0xbe, 0x1d, 0xeb, 0xb3, 0xfa, 0xfd, 0xaa, 0x31, 0x8f, 0xba, 0xe4, 0xf3,
                0xab, 0xca, 0x3e, 0x5e, 0x11, 0x7b, 0x01, 0x75, 0x6a, 0x56, 0x7d, 0xf2, 0x94, 0x0d,
                0xe5, 0x63, 0x86, 0x6c,
            ],
            Core::FeSquare => [
                0xfb, 0xc7, 0x62, 0x56, 0x79, 0xfe, 0xac, 0x0a, 0xaf, 0x53, 0xc2, 0x9b, 0xdd, 0x9e,
                0x8a, 0xc6, 0x1d, 0x0e, 0xf2, 0xb8, 0x0e, 0x49, 0x3f, 0x3a, 0x60, 0x23, 0x84, 0xf8,
                0x31, 0x8b, 0x68, 0xbd,
            ],
            Core::FeSquareRoot => [
                0x86, 0xb3, 0xa1, 0x33, 0x34, 0x54, 0x1b, 0x43, 0x9e, 0xf2, 0x41, 0x15, 0x6f, 0x34,
                0xde, 0xf6, 0x6d, 0x32, 0xba, 0xaa, 0xf6, 0xc6, 0xca, 0x4f, 0x69, 0x21, 0x58, 0x0c,
                0x32, 0xe8, 0x49, 0x18,
            ],
            Core::FullAdd16 => [
                0xb6, 0x4c, 0x43, 0x31, 0x5b, 0x94, 0x56, 0xed, 0x80, 0x70, 0xc1, 0xc8, 0x29, 0xb7,
                0x38, 0x2d, 0xc8, 0x5e, 0xc0, 0xde, 0x75, 0x4d, 0x4d, 0x9c, 0xde, 0xd3, 0xce, 0x69,
                0x8c, 0x9a, 0x10, 0x7b,
            ],
            Core::FullAdd32 => [
                0x91, 0x85, 0x39, 0x4d, 0xb2, 0xbe, 0xb6, 0x4d, 0xd0, 0x8f, 0x83, 0x20, 0x52, 0xf7,
                0x6f, 0xb0, 0xe0, 0x86, 0xcc, 0x5e, 0xa4, 0xed, 0x7a, 0xc8, 0x5d, 0x22, 0x20, 0x0d,
                0xdb, 0x74, 0xab, 0x26,
            ],
            Core::FullAdd64 => [
                0xd3, 0x8a, 0x1e, 0x79, 0x3d, 0x39, 0xc4, 0xda, 0xbb, 0x30, 0x54, 0x44, 0xb7, 0x07,
                0x8c, 0x09, 0x24, 0x73, 0x94, 0xa0, 0xcc, 0x56, 0x19, 0x27, 0x58, 0x53, 0x02, 0xb6,
                0x84, 0xa5, 0x43, 0xb5,
            ],
            Core::FullAdd8 => [
                0x99, 0x3f, 0x63, 0x34, 0x4f, 0x9c, 0x4e, 0xd3, 0xaf, 0x29, 0x1d, 0x71, 0xf6, 0x1f,
                0xc4, 0x09, 0x47, 0x50, 0x68, 0x21, 0xef, 0xf7, 0x00, 0xe5, 0x21, 0xc1, 0xec, 0xbb,
                0x32, 0xdf, 0xcc, 0x45,
            ],
            Core::FullDecrement16 => [
                0x46, 0xf2, 0x73, 0x40, 0x31, 0x03, 0x8e, 0xa6, 0xf0, 0xe1, 0xda, 0x7f, 0x9f, 0x04,
                0xcc, 0xa7, 0x77, 0xe6, 0xc6, 0x9f, 0xff, 0xc1, 0x10, 0x85, 0x15, 0x86, 0x8d, 0x51,
                0xed, 0xc8, 0xac, 0xcb,
            ],
            Core::FullDecrement32 => [
                0x22, 0xab, 0xbf, 0x3d, 0x6a, 0xcf, 0x0b, 0x8c, 0xb7, 0xf5, 0x8f, 0x9c, 0x7e, 0x92,
                0x7a, 0xa1, 0xa1, 0xbb, 0x8f, 0xd0, 0xb9, 0x96, 0x23, 0xa1, 0x10, 0x04, 0xac, 0x53,
                0x65, 0x56, 0xab, 0x02,
            ],
            Core::FullDecrement64 => [
                0xd5, 0x39, 0xaa, 0xe2, 0x25, 0x27, 0xd4, 0x61, 0x35, 0x46, 0x1e, 0x40, 0x59, 0x07,
                0x1e, 0x9d, 0x5b, 0x2f, 0xad, 0xfc, 0xa1, 0x3f, 0xb0, 0x2a, 0xb5, 0x72, 0x3e, 0x8a,
                0x91, 0x6a, 0x61, 0x5c,
            ],
            Core::FullDecrement8 => [
                0x3f, 0x63, 0x6f, 0x40, 0x16, 0x9d, 0x07, 0x2f, 0x36, 0x82, 0x0b, 0x1f, 0x3f, 0x39,
                0xc1, 0x74, 0x86, 0x24, 0xd9, 0xd5, 0xfb, 0x95, 0x7a, 0x8d, 0x49, 0x44, 0xf4, 0xfd,
                0x0d, 0x48, 0x59, 0x7a,
            ],
            Core::FullIncrement16 => [
                0x99, 0x64, 0x22, 0xaf, 0xc5, 0x2e, 0x28, 0xb1, 0x25, 0xaa, 0x70, 0xb2, 0xd0, 0x7c,
                0xa4, 0x61, 0x32, 0x2f, 0xab, 0x6a, 0x45, 0xbb, 0x76, 0x9d, 0xc1, 0x2d, 0xb2, 0xfa,
                0x9a, 0xf9, 0xbd, 0xbb,
            ],
            Core::FullIncrement32 => [
                0x1a, 0x56, 0x55, 0xfa, 0xe2, 0xaf, 0x0a, 0xe6, 0xa6, 0xac, 0xb5, 0x87, 0xa8, 0x6c,
                0x72, 0xbb, 0x30, 0x88, 0xd7, 0x65, 0x6d, 0x25, 0x30, 0x95, 0x13, 0x7c, 0x9e, 0xd0,
                0x44, 0x89, 0x20, 0xbd,
            ],
            Core::FullIncrement64 => [
                0x80, 0x7d, 0xc6, 0x66, 0xd3, 0xb1, 0xd3, 0xfd, 0x65, 0xa4, 0x03, 0x73, 0xf6, 0xdf,
                0xc6, 0xef, 0x64, 0x19, 0xfa, 0x93, 0x06, 0x5c, 0x1a, 0x90, 0x00, 0xde, 0xf7, 0x54,
                0x2c, 0x8e, 0xe8, 0xed,
            ],
            Core::FullIncrement8 => [
                0x1f, 0xfb, 0x9a, 0x76, 0x2c, 0x99, 0xcb, 0x4d, 0x52, 0x8c, 0xba, 0x4d, 0xc6, 0x1f,
                0x13, 0x4f, 0xdb, 0x41, 0x07, 0xe1, 0xf4, 0x9b, 0x07, 0x6a, 0x57, 0x70, 0xf3, 0x91,
                0x95, 0x95, 0x46, 0x6c,
            ],
            Core::FullMultiply16 => [
                0x8f, 0x46, 0xfa, 0xa9, 0xec, 0x05, 0xe3, 0xd3, 0xae, 0xee, 0x3e, 0xc4, 0x92, 0xec,
                0x53, 0x4c, 0xde, 0x6f, 0xc0, 0x33, 0x61, 0xe6, 0x19, 0x99, 0x4e, 0x8d, 0x04, 0x73,
                0xd6, 0xb0, 0x9e, 0x5d,
            ],
            Core::FullMultiply32 => [
                0x6c, 0x60, 0x02, 0x1f, 0x3b, 0x6f, 0x33, 0xe0, 0x8c, 0xa4, 0x2e, 0x4c, 0x7e, 0x5a,
                0xcf, 0xf4, 0xcc, 0x7e, 0x0d, 0x69, 0x86, 0xdf, 0xb3, 0x55, 0xab, 0xe0, 0x56, 0x17,
                0x57, 0xf7, 0x25, 0xcb,
            ],
            Core::FullMultiply64 => [
                0x05, 0xb2, 0x7c, 0xe5, 0x8a, 0xb0, 0x86, 0xdd, 0xb2, 0x5a, 0x1c, 0xf9, 0xe3, 0x09,
                0x1f, 0xdf, 0x23, 0x21, 0x4c, 0x54, 0x31, 0x50, 0x40, 0x32, 0x4d, 0x4f, 0x6f, 0x10,
                0x95, 0x3e, 0xd2, 0x45,
            ],
            Core::FullMultiply8 => [
                0xcc, 0x22, 0xf3, 0x08, 0x18, 0xeb, 0x2d, 0x7b, 0xe5, 0x11, 0x42, 0xe6, 0xb2, 0xc4,
                0xeb, 0x64, 0xad, 0x7c, 0x83, 0xd5, 0xc1, 0x95, 0xcd, 0xdf, 0x58, 0x1b, 0xd9, 0x75,
                0x62, 0x38, 0x14, 0x6b,
            ],
            Core::FullSubtract16 => [
                0x41, 0x6d, 0x74, 0x7b, 0xf0, 0x18, 0x85, 0xb0, 0x9e, 0xcb, 0x8f, 0x05, 0x7e, 0x6b,
                0x15, 0x40, 0xe5, 0x42, 0xfc, 0xde, 0x49, 0xa7, 0x74, 0x61, 0xc0, 0x5a, 0xc7, 0x30,
                0x84, 0x5b, 0xdf, 0x8e,
            ],
            Core::FullSubtract32 => [
                0xb1, 0xbc, 0x9d, 0xf5, 0x70, 0x27, 0x52, 0xb4, 0xb3, 0x05, 0x12, 0xaa, 0xef, 0x54,
                0x9c, 0x6b, 0x0e, 0x82, 0x47, 0x71, 0x09, 0x0b, 0x94, 0xef, 0x96, 0x89, 0xbd, 0x4a,
                0xe3, 0xab, 0xa8, 0x81,
            ],
            Core::FullSubtract64 => [
                0xc2, 0x44, 0xb5, 0x36, 0xd5, 0x3c, 0x79, 0xa2, 0xd4, 0x5c, 0xdf, 0x8b, 0x56, 0x1b,
                0xf5, 0x10, 0x22, 0xe5, 0xd5, 0x81, 0x98, 0x49, 0x50, 0x78, 0x47, 0x78, 0xde, 0x9e,
                0x6a, 0x90, 0x05, 0x2f,
            ],
            Core::FullSubtract8 => [
                0x98, 0xa6, 0x0f, 0x3a, 0x31, 0x98, 0xca, 0x68, 0x60, 0x2a, 0xa0, 0xa5, 0x23, 0xad,
                0xc1, 0x77, 0xc3, 0x66, 0xe8, 0x03, 0x31, 0x9e, 0xee, 0x88, 0x16, 0x0e, 0xa3, 0xb0,
                0xef, 0xd7, 0x10, 0x4c,
            ],
            Core::GeIsOnCurve => [
                0xf3, 0xcd, 0xa5, 0x4a, 0x99, 0xb7, 0x01, 0x1b, 0xd4, 0x88, 0xd3, 0x1e, 0xca, 0xfc,
                0xac, 0x34, 0x7a, 0x35, 0x7b, 0x4d, 0xeb, 0x79, 0x2b, 0x8c, 0xfa, 0x3d, 0x00, 0xc7,
                0xb1, 0x09, 0x7e, 0x4c,
            ],
            Core::GeNegate => [
                0xa9, 0xfe, 0x3b, 0xe6, 0x70, 0xd1, 0x5c, 0x78, 0x99, 0x70, 0xcc, 0xf7, 0x6c, 0xa8,
                0x69, 0xf8, 0x0b, 0xbb, 0x77, 0x2b, 0x6e, 0xa0, 0xae, 0x85, 0x18, 0xd3, 0xa8, 0xd6,
                0xf8, 0x83, 0xa6, 0x42,
            ],
            Core::GejAdd => [
                0x93, 0x35, 0x13, 0x59, 0x9e, 0x03, 0x6b, 0x4f, 0xb8, 0xfc, 0x94, 0xcc, 0x8c, 0xee,
                0x43, 0x7d, 0xa1, 0x0f, 0x22, 0xcb, 0xf8, 0x91, 0x0e, 0x48, 0xa5, 0x34, 0x7f, 0xaa,
                0x36, 0x3b, 0x40, 0x4c,
            ],
            Core::GejDouble => [
                0x08, 0xa1, 0xce, 0x0a, 0x24, 0x79, 0xc6, 0xd1, 0x42, 0x45, 0x99, 0xfd, 0x6b, 0xa9,
                0x1e, 0xd3, 0xb5, 0xb9, 0x56, 0x23, 0x8b, 0xa1, 0x27, 0x6d, 0xf5, 0x1f, 0x06, 0x9b,
                0xb2, 0x47, 0xb1, 0x8d,
            ],
            Core::GejGeAdd => [
                0xfd, 0x6a, 0x62, 0xe5, 0xee, 0x48, 0x6d, 0x81, 0x6d, 0x7d, 0x50, 0x78, 0x3a, 0xe6,
                0xc3, 0x99, 0x76, 0xbc, 0x7a, 0x3e, 0x19, 0x67, 0xe8, 0x4a, 0x3e, 0x3a, 0x1a, 0xdc,
                0x64, 0x5e, 0x45, 0xf8,
            ],
            Core::GejGeAddEx => [
                0xfd, 0x09, 0xfb, 0xc0, 0xa6, 0xac, 0xa5, 0x78, 0x76, 0x61, 0xda, 0x11, 0xee, 0xe9,
                0x3d, 0x4f, 0xbb, 0xd7, 0x4d, 0xe1, 0x1c, 0xf5, 0x31, 0x6d, 0xb1, 0x46, 0x42, 0x3e,
                0xf8, 0x90, 0xf1, 0x18,
            ],
            Core::GejInfinity => [
                0x73, 0x8f, 0xf0, 0x80, 0xff, 0x20, 0x81, 0x15, 0x4e, 0x96, 0xcb, 0x1b, 0xad, 0x2a,
                0x74, 0xf6, 0xe8, 0xea, 0xe3, 0x38, 0xcd, 0x56, 0x11, 0x9c, 0x5b, 0x15, 0x60, 0x43,
                0x90, 0x89, 0xf8, 0xf1,
            ],
            Core::GejIsInfinity => [
                0x68, 0xe7, 0xf0, 0x52, 0xf0, 0xc5, 0xc9, 0xe2, 0x6d, 0xcc, 0x37, 0x27, 0xb8, 0x0b,
                0x00, 0xa4, 0x7a, 0xaa, 0xa3, 0x69, 0xb3, 0xa9, 0x6d, 0x18, 0x7e, 0xb8, 0x0a, 0x5c,
                0xaa, 0xc2, 0x9d, 0x98,
            ],
            Core::GejIsOnCurve => [
                0x5c, 0x64, 0xd0, 0x6a, 0x8e, 0x63, 0x11, 0x33, 0xfe, 0x42, 0x5d, 0xd1, 0xac, 0x35,
                0x4e, 0x4e, 0x83, 0x44, 0xdc, 0x2a, 0x21, 0x94, 0x82, 0x75, 0x9e, 0xeb, 0x55, 0x7d,
                0x8f, 0x3e, 0x87, 0xb4,
            ],
            Core::GejNegate => [
                0x67, 0x5e, 0xeb, 0xfe, 0x53, 0x7f, 0x27, 0x74, 0xea, 0xaf, 0xcb, 0x9f, 0x0a, 0xa4,
                0xe4, 0x87, 0x95, 0x76, 0x5c, 0x18, 0x6e, 0x25, 0x02, 0xb9, 0x7d, 0xc3, 0x0d, 0xbd,
                0x22, 0x5d, 0x14, 0xf5,
            ],
            Core::GejNormalize => [
                0x7c, 0x89, 0x63, 0x42, 0x1e, 0xf4, 0x6d, 0x02, 0xe8, 0xd5, 0x76, 0xd6, 0xbd, 0x06,
                0x0b, 0xe8, 0x8a, 0xf5, 0xd3, 0xa3, 0xd8, 0xa7, 0xa9, 0x4c, 0x99, 0x75, 0x82, 0x97,
                0xd6, 0x06, 0x4b, 0xd8,
            ],
            Core::GejRescale => [
                0x02, 0x58, 0xf6, 0xb5, 0x8e, 0x2d, 0x53, 0x60, 0x36, 0xe7, 0xae, 0x59, 0xc5, 0x45,
                0x09, 0x61, 0xb4, 0xe8, 0x2e, 0x09, 0x5f, 0x3b, 0xb7, 0x82, 0x7c, 0xfb, 0xba, 0x59,
                0x19, 0x72, 0x41, 0x44,
            ],
            Core::GejXEquiv => [
                0xb7, 0x1a, 0x68, 0xc6, 0x1b, 0xea, 0x01, 0x41, 0x55, 0x4b, 0x9b, 0xec, 0x06, 0x97,
                0xe9, 0xce, 0x9e, 0x44, 0x7c, 0x2c, 0x94, 0xc2, 0xf9, 0x3f, 0xbf, 0x10, 0xf6, 0x6c,
                0x50, 0xf2, 0x25, 0x6e,
            ],
            Core::GejYIsOdd => [
                0x82, 0xdf, 0x24, 0x26, 0xe2, 0xcc, 0xc6, 0xf5, 0xae, 0x2b, 0x4a, 0xc2, 0x25, 0xe2,
                0xa9, 0x4c, 0xf4, 0x9c, 0xbe, 0x62, 0xe6, 0x4d, 0xf4, 0xa9, 0x33, 0xc1, 0x16, 0xae,
                0xb9, 0x93, 0xe8, 0x3d,
            ],
            Core::Generate => [
                0x90, 0xa3, 0x4b, 0xdd, 0x0b, 0xee, 0x48, 0xf8, 0x56, 0xd8, 0xcc, 0xf6, 0x98, 0x1a,
                0x72, 0x64, 0x8e, 0x3b, 0x51, 0x10, 0x9b, 0x3b, 0xb2, 0x55, 0x20, 0x78, 0x14, 0x10,
                0x57, 0x7b, 0xfc, 0x81,
            ],
            Core::High16 => [
                0x1f, 0x52, 0x46, 0x4d, 0xfa, 0x38, 0x74, 0xf6, 0xda, 0x23, 0x3b, 0x46, 0x88, 0x79,
                0x93, 0x77, 0xd8, 0x47, 0x32, 0x65, 0x8c, 0xda, 0x1b, 0x43, 0x71, 0x49, 0x84, 0x82,
                0x59, 0xcb, 0xd5, 0x30,
            ],
            Core::High32 => [
                0x60, 0xe0, 0xaa, 0x26, 0xc8, 0xd7, 0x30, 0x5c, 0x44, 0x14, 0xb4, 0x5d, 0xce, 0x3b,
                0x29, 0xf1, 0x07, 0x3e, 0xc1, 0x87, 0x62, 0x60, 0x82, 0xbe, 0x37, 0x72, 0x34, 0xcf,
                0xd5, 0x58, 0xc9, 0x72,
            ],
            Core::High64 => [
                0xaf, 0x1a, 0xd2, 0xbd, 0x95, 0xca, 0x2f, 0x4e, 0x48, 0x62, 0xd4, 0xa3, 0x9b, 0x8a,
                0x4b, 0x79, 0x80, 0x55, 0x01, 0xbf, 0x8a, 0x6d, 0x40, 0x7e, 0x97, 0x2c, 0xf0, 0x06,
                0x61, 0xc3, 0xa3, 0x76,
            ],
            Core::High8 => [
                0x3b, 0x30, 0xa5, 0x2e, 0x98, 0x3e, 0xb1, 0xb8, 0x61, 0x3f, 0x34, 0x80, 0x1b, 0x04,
                0x64, 0x49, 0xa5, 0xe1, 0xff, 0x32, 0x27, 0xb0, 0x96, 0xb9, 0x3c, 0xa4, 0xad, 0x9b,
                0x3f, 0x4b, 0xab, 0x82,
            ],
            Core::Increment16 => [
                0x7f, 0xcd, 0x35, 0x7e, 0x97, 0x74, 0xb6, 0x02, 0x8c, 0x27, 0xfb, 0xd4, 0x48, 0x1c,
                0x28, 0x22, 0xb7, 0x45, 0xcf, 0x2c, 0xb2, 0xab, 0x8d, 0xcd, 0xc7, 0x07, 0x9f, 0x22,
                0x5c, 0xb1, 0x5d, 0x6d,
            ],
            Core::Increment32 => [
                0x10, 0x26, 0x81, 0x25, 0xcb, 0x54, 0xb1, 0x73, 0x9e, 0x02, 0x9f, 0x20, 0x4d, 0x6e,
                0xf0, 0xf7, 0x21, 0x68, 0x72, 0xfd, 0x04, 0x5e, 0xef, 0x08, 0x96, 0x1a, 0x79, 0xde,
                0xf8, 0x89, 0xc6, 0x9c,
            ],
            Core::Increment64 => [
                0x72, 0x8b, 0xec, 0x59, 0xca, 0x50, 0xc2, 0x33, 0xd5, 0xb3, 0xda, 0x1d, 0x81, 0x7c,
                0xf8, 0x3f, 0x11, 0x12, 0xe2, 0xba, 0xda, 0xed, 0x0f, 0xa8, 0xd4, 0xfc, 0xf1, 0x22,
                0xc0, 0x06, 0x29, 0x9c,
            ],
            Core::Increment8 => [
                0xef, 0xc2, 0x4a, 0x38, 0xa6, 0xab, 0x37, 0x3c, 0xa7, 0xdf, 0x08, 0xf9, 0xe4, 0x3c,
                0xca, 0x03, 0xcf, 0xd0, 0x42, 0x82, 0x72, 0x37, 0xf5, 0x1e, 0x4c, 0x70, 0x7f, 0xb1,
                0x66, 0xf0, 0xad, 0x25,
            ],
            Core::IsOne16 => [
                0x7c, 0xb3, 0xd4, 0xed, 0xbf, 0x46, 0x40, 0xd2, 0xe9, 0xc7, 0x03, 0xc6, 0x8a, 0xcb,
                0xbf, 0xc4, 0x52, 0xef, 0xd6, 0x0e, 0xb1, 0xc2, 0xe0, 0x4f, 0x18, 0x7e, 0x7d, 0x70,
                0xed, 0x45, 0x7a, 0x39,
            ],
            Core::IsOne32 => [
                0x91, 0xd7, 0x4e, 0x95, 0xef, 0x74, 0x4e, 0x01, 0xe5, 0x6d, 0xf6, 0xac, 0x36, 0xdf,
                0x18, 0x39, 0xab, 0x6c, 0x60, 0x8c, 0x7a, 0xb0, 0xaf, 0xc1, 0xfc, 0xc8, 0x7e, 0xb5,
                0x0a, 0x27, 0xd7, 0x99,
            ],
            Core::IsOne64 => [
                0x7d, 0xdf, 0xc1, 0x3a, 0x05, 0x43, 0x39, 0x8e, 0x8f, 0x0e, 0x07, 0xe6, 0xca, 0x05,
                0x44, 0xb5, 0xff, 0x0a, 0x3e, 0xe8, 0x38, 0x25, 0xb2, 0x18, 0xb3, 0xb8, 0xb0, 0xe4,
                0x2e, 0x96, 0x52, 0xa9,
            ],
            Core::IsOne8 => [
                0xbc, 0xd4, 0x16, 0x8e, 0x02, 0x83, 0x2d, 0xcc, 0xc9, 0xbf, 0xb5, 0x7f, 0x9a, 0x94,
                0xd2, 0x2c, 0xc0, 0x2d, 0xc7, 0x4b, 0xf4, 0x58, 0xc8, 0x17, 0x3e, 0xa3, 0x9e, 0x9e,
                0x16, 0xe6, 0x08, 0x34,
            ],
            Core::IsZero16 => [
                0xb5, 0x16, 0x84, 0x8c, 0x3f, 0x28, 0x2c, 0xa7, 0xa8, 0x0b, 0x19, 0x50, 0x5e, 0x8f,
                0xd1, 0x38, 0x80, 0xc9, 0x93, 0xa9, 0x4d, 0xa5, 0x9b, 0xe7, 0x72, 0x32, 0x52, 0xbc,
                0x62, 0xa3, 0xc9, 0x24,
            ],
            Core::IsZero32 => [
                0x69, 0x91, 0xf0, 0x26, 0xaf, 0x70, 0x7d, 0xd9, 0x54, 0x1d, 0xe6, 0x25, 0x67, 0x0f,
                0x83, 0x20, 0x80, 0x0d, 0x5b, 0xbd, 0x48, 0x04, 0x7d, 0x44, 0xc8, 0xee, 0x0d, 0xb1,
                0x33, 0x91, 0x19, 0x36,
            ],
            Core::IsZero64 => [
                0xe2, 0xf8, 0x90, 0xc6, 0x7d, 0x35, 0xdc, 0xe2, 0x4f, 0x7b, 0x2c, 0xa6, 0x09, 0xaf,
                0x8b, 0xd2, 0x35, 0xbe, 0x6c, 0xd6, 0x99, 0x97, 0x0a, 0x43, 0xc6, 0x9b, 0xbb, 0xe2,
                0xf2, 0x1d, 0x15, 0x9c,
            ],
            Core::IsZero8 => [
                0xa9, 0xc8, 0xc4, 0x77, 0xad, 0xd8, 0xa9, 0xf5, 0x58, 0x52, 0xc5, 0x1c, 0xe1, 0xfb,
                0xeb, 0x9a, 0x4c, 0x50, 0xc8, 0xf0, 0xb6, 0x3a, 0x55, 0xe8, 0x92, 0x40, 0x09, 0xa8,
                0xf5, 0x9d, 0x3c, 0x69,
            ],
            Core::Le16 => [
                0xd0, 0x26, 0x9d, 0x47, 0xd2, 0x62, 0xcc, 0xb6, 0x7e, 0x39, 0xee, 0xb8, 0x8f, 0x7e,
                0xe0, 0xec, 0xd6, 0xf1, 0xf9, 0x64, 0x10, 0x25, 0x2d, 0x59, 0xce, 0x35, 0xcb, 0x87,
                0x1d, 0x09, 0xca, 0x64,
            ],
            Core::Le32 => [
                0x2c, 0xea, 0x91, 0x87, 0x17, 0x3a, 0x50, 0x70, 0x4e, 0x93, 0xa6, 0x31, 0xd9, 0xcf,
                0xb2, 0x6a, 0xde, 0x50, 0xf6, 0xf1, 0x9d, 0xf4, 0xc3, 0x59, 0x15, 0xf4, 0x9d, 0x22,
                0xa3, 0xea, 0xd4, 0xd5,
            ],
            Core::Le64 => [
                0x09, 0x2b, 0xb9, 0x2e, 0x9d, 0x8b, 0xe5, 0xf3, 0xcd, 0x88, 0x45, 0x9e, 0xca, 0x90,
                0x9e, 0x2a, 0x49, 0xaf, 0x7a, 0xc9, 0xf0, 0xdf, 0x11, 0x59, 0xc1, 0xac, 0x1a, 0xc9,
                0x1b, 0xc5, 0x1b, 0x0c,
            ],
            Core::Le8 => [
                0x02, 0x7d, 0x93, 0xd3, 0xc2, 0x47, 0xc3, 0x90, 0xe4, 0x5f, 0xf3, 0x24, 0x08, 0xcc,
                0x49, 0xa5, 0xc1, 0x7d, 0x55, 0xb0, 0x49, 0x31, 0xda, 0xcc, 0x03, 0x0a, 0x3a, 0x50,
                0x4a, 0x0e, 0x3c, 0x75,
            ],
            Core::LinearCombination1 => [
                0x69, 0x6c, 0x5f, 0x28, 0xf9, 0xe7, 0x59, 0xd9, 0x6c, 0x2a, 0x0f, 0x99, 0x03, 0x79,
                0xfc, 0xcf, 0x08, 0x42, 0x65, 0xf6, 0x33, 0x93, 0x26, 0x1d, 0x72, 0x49, 0xbb, 0xa3,
                0x1d, 0xfa, 0x24, 0x41,
            ],
            Core::LinearVerify1 => [
                0x9d, 0x09, 0x52, 0xbf, 0x99, 0x97, 0xd8, 0x51, 0xa8, 0xcb, 0xbf, 0x8b, 0xdf, 0x2d,
                0x5e, 0xf4, 0x10, 0x86, 0x45, 0xcd, 0xdf, 0xf3, 0xf4, 0xf0, 0x0d, 0x38, 0x24, 0xdd,
                0x10, 0x72, 0x75, 0x2d,
            ],
            Core::Low16 => [
                0xe3, 0x85, 0x5b, 0xb4, 0x64, 0x5a, 0x08, 0x25, 0xea, 0xae, 0x04, 0x12, 0x51, 0x5a,
                0x8b, 0x15, 0xa3, 0x7b, 0x30, 0x5d, 0xed, 0x6f, 0x11, 0x9a, 0x3f, 0x44, 0x25, 0x31,
                0x29, 0x7a, 0xb6, 0x9c,
            ],
            Core::Low32 => [
                0x75, 0xd2, 0xfb, 0x6b, 0xe6, 0x3b, 0x8b, 0x2c, 0x35, 0x21, 0x12, 0x3b, 0x5b, 0xa7,
                0x7f, 0x78, 0x1b, 0xe9, 0xff, 0x1c, 0x1a, 0x75, 0xa8, 0xe2, 0xce, 0xef, 0x62, 0x06,
                0xfb, 0x73, 0x45, 0x89,
            ],
            Core::Low64 => [
                0xea, 0x12, 0x49, 0xb3, 0xe2, 0x2e, 0x95, 0xf3, 0xc6, 0xff, 0x59, 0x1e, 0x87, 0x2c,
                0x03, 0xb6, 0xf1, 0xb4, 0xa1, 0xd8, 0xc0, 0x45, 0xc5, 0xa3, 0xba, 0xc7, 0xd9, 0x21,
                0x27, 0x28, 0x15, 0xb6,
            ],
            Core::Low8 => [
                0x53, 0xe0, 0x0b, 0x91, 0x24, 0xd9, 0x60, 0x14, 0x7a, 0xb5, 0x72, 0x44, 0x5d, 0x37,
                0x3b, 0x42, 0x0a, 0x22, 0x21, 0x44, 0x81, 0xbd, 0xec, 0x5e, 0x6a, 0x3f, 0x05, 0xd5,
                0xab, 0x8e, 0x19, 0x84,
            ],
            Core::Lt16 => [
                0x11, 0xb4, 0xc0, 0x33, 0x26, 0x30, 0xbc, 0xc6, 0xc6, 0x04, 0x4c, 0x3f, 0x88, 0x7b,
                0x2c, 0x50, 0x39, 0x74, 0x3f, 0x8a, 0xd9, 0x08, 0x97, 0x8e, 0x7b, 0xa3, 0x0a, 0x18,
                0xf0, 0x5f, 0xec, 0xaf,
            ],
            Core::Lt32 => [
                0x04, 0x20, 0x22, 0x45, 0xd7, 0x59, 0x0d, 0x5d, 0x9c, 0x68, 0xb8, 0x4f, 0x26, 0x0f,
                0xd3, 0x6f, 0x0f, 0xce, 0xb6, 0x2a, 0xea, 0x20, 0x4d, 0xd7, 0xc5, 0x73, 0x6b, 0xf6,
                0xb9, 0x41, 0xb7, 0x65,
            ],
            Core::Lt64 => [
                0xe8, 0x6f, 0x1e, 0xda, 0x2e, 0x39, 0xef, 0x08, 0x7e, 0x3f, 0x03, 0x16, 0x62, 0xb8,
                0xb1, 0xda, 0x8d, 0xf1, 0xa3, 0x84, 0x31, 0xe3, 0x1b, 0xdd, 0x56, 0x7f, 0x26, 0x7f,
                0xa0, 0x88, 0xe3, 0x38,
            ],
            Core::Lt8 => [
                0xa0, 0x3e, 0x3c, 0xb8, 0x4b, 0xa0, 0xc6, 0xd5, 0xc9, 0x77, 0x93, 0x92, 0xe5, 0x8f,
                0x89, 0xa7, 0x60, 0x1c, 0xd5, 0x25, 0x49, 0xde, 0x6d, 0x75, 0xff, 0x71, 0x55, 0xb1,
                0xf1, 0x0a, 0x2e, 0x48,
            ],
            Core::Maj16 => [
                0x29, 0x87, 0x64, 0xb7, 0xe8, 0xfb, 0x49, 0xb5, 0xd9, 0x55, 0x9b, 0x70, 0x90, 0x1f,
                0x1a, 0x17, 0x96, 0x56, 0x86, 0x4e, 0xef, 0xf3, 0xe2, 0x69, 0x21, 0x12, 0x6f, 0x70,
                0x30, 0xf1, 0x4a, 0x65,
            ],
            Core::Maj32 => [
                0xb8, 0xc1, 0x7d, 0xea, 0x76, 0xe2, 0x3b, 0x5d, 0x71, 0x5c, 0xb0, 0x0f, 0x90, 0x11,
                0x39, 0x30, 0xfe, 0xb9, 0x1e, 0x86, 0xc8, 0x39, 0x41, 0x1c, 0x18, 0xe9, 0xac, 0x86,
                0xfc, 0x82, 0x78, 0xc7,
            ],
            Core::Maj64 => [
                0x01, 0xd8, 0x5a, 0xb4, 0xbb, 0xc8, 0x9a, 0xc3, 0xa6, 0xf4, 0xce, 0x55, 0x64, 0xe1,
                0xa8, 0x42, 0xd7, 0x30, 0x31, 0x26, 0x8c, 0x9e, 0xb9, 0x8d, 0xf9, 0x3e, 0x12, 0x09,
                0x42, 0x0b, 0x4c, 0x0f,
            ],
            Core::Maj8 => [
                0x83, 0xae, 0x95, 0xcb, 0x13, 0x96, 0xc2, 0xc8, 0x14, 0xb0, 0x19, 0xf6, 0x10, 0x37,
                0x1c, 0xb1, 0x8f, 0x14, 0xb0, 0xf0, 0x6e, 0xad, 0xc2, 0x85, 0xfa, 0xec, 0x3a, 0x4d,
                0x5b, 0x22, 0x9a, 0xe1,
            ],
            Core::Max16 => [
                0xa1, 0x7d, 0x86, 0xd1, 0xdb, 0xb4, 0x2f, 0x02, 0xd6, 0xf7, 0x7d, 0x73, 0x51, 0x31,
                0xb7, 0xa5, 0x95, 0x86, 0x1d, 0x24, 0x95, 0x5e, 0x98, 0x58, 0x89, 0x4b, 0x04, 0x76,
                0xee, 0x74, 0x68, 0xe3,
            ],
            Core::Max32 => [
                0x2d, 0x8d, 0xb7, 0xe8, 0xda, 0x9e, 0x71, 0x13, 0xf3, 0x38, 0xa3, 0x2f, 0x50, 0xe4,
                0xb0, 0xfe, 0x55, 0xab, 0x7e, 0xb3, 0xcf, 0x4a, 0x07, 0x74, 0xa8, 0x99, 0x4e, 0x81,
                0x84, 0xf9, 0x98, 0x39,
            ],
            Core::Max64 => [
                0x87, 0xe7, 0xbb, 0x87, 0xb8, 0xd3, 0x99, 0x51, 0x6e, 0x23, 0x9f, 0x5f, 0x80, 0xeb,
                0xfe, 0x2e, 0x4e, 0x83, 0x90, 0x46, 0x97, 0x5f, 0xac, 0x45, 0x31, 0xcf, 0xaf, 0x84,
                0xb2, 0x02, 0xff, 0x89,
            ],
            Core::Max8 => [
                0xfa, 0xe5, 0x3c, 0x8c, 0x42, 0x1b, 0xd1, 0x5c, 0x6e, 0x31, 0x1f, 0xd4, 0xb1, 0x07,
                0xf6, 0x4e, 0x7a, 0x53, 0xe2, 0xa3, 0xdc, 0x72, 0xab, 0x0c, 0xc1, 0x46, 0x59, 0xa4,
                0x64, 0x6a, 0x05, 0x7b,
            ],
            Core::Median16 => [
                0x1c, 0x9e, 0xb3, 0x76, 0x95, 0xd4, 0x2f, 0x70, 0x71, 0x93, 0xd4, 0x87, 0xe2, 0x58,
                0xa1, 0x46, 0x84, 0xa3, 0x9b, 0x4d, 0x83, 0x5c, 0x6b, 0xd3, 0x4e, 0x33, 0xf7, 0x0c,
                0x00, 0x55, 0xc1, 0xc6,
            ],
            Core::Median32 => [
                0x43, 0x66, 0x91, 0x97, 0x15, 0x50, 0x21, 0xbe, 0x47, 0xd6, 0x7b, 0x01, 0xae, 0x70,
                0xc1, 0x25, 0xa7, 0x4b, 0x27, 0x5c, 0x46, 0xf4, 0xf6, 0x0c, 0xd2, 0xe7, 0x54, 0xa5,
                0x69, 0x75, 0x58, 0xdc,
            ],
            Core::Median64 => [
                0x18, 0x29, 0x1e, 0xa2, 0xe0, 0x88, 0x4f, 0x11, 0x29, 0xeb, 0x68, 0x1f, 0x68, 0x9b,
                0xd7, 0x07, 0x00, 0xdb, 0x9e, 0x3f, 0xa4, 0xec, 0x0d, 0x3f, 0xd1, 0x84, 0x60, 0x5a,
                0x96, 0x43, 0xed, 0xb2,
            ],
            Core::Median8 => [
                0x01, 0xd8, 0x19, 0xb8, 0x33, 0xbf, 0x99, 0x73, 0x6b, 0x1a, 0xd1, 0xfa, 0xdb, 0x9e,
                0x8d, 0xf6, 0xfb, 0x94, 0xbd, 0xf6, 0x26, 0x7e, 0x16, 0x5b, 0xfb, 0x80, 0x85, 0xc4,
                0xd2, 0x71, 0x70, 0xe3,
            ],
            Core::Min16 => [
                0x6f, 0x65, 0xef, 0x43, 0x92, 0xf4, 0x55, 0x4a, 0x1b, 0x90, 0xa6, 0x2e, 0xb4, 0xbb,
                0x35, 0x7e, 0x46, 0x8d, 0x5e, 0x85, 0xe7, 0x3b, 0x01, 0x83, 0xfb, 0x36, 0x6a, 0x75,
                0xaf, 0x0d, 0x53, 0x02,
            ],
            Core::Min32 => [
                0x24, 0xd0, 0xd0, 0x83, 0x63, 0xe5, 0xcb, 0xda, 0xa9, 0xe0, 0xe0, 0xd2, 0x3f, 0x6d,
                0x43, 0x20, 0xa0, 0xc4, 0x91, 0x59, 0x36, 0xc5, 0x12, 0x6a, 0xc1, 0x70, 0x74, 0x5a,
                0xe0, 0x73, 0x6b, 0x3e,
            ],
            Core::Min64 => [
                0xde, 0x6c, 0x8c, 0x3f, 0xaf, 0x26, 0x59, 0xfd, 0xf5, 0x2d, 0x44, 0xea, 0xe4, 0xd3,
                0x83, 0x54, 0xad, 0x09, 0x75, 0x58, 0xff, 0x8f, 0xac, 0xe2, 0xe4, 0xa5, 0x37, 0x17,
                0x38, 0xb4, 0x73, 0xfb,
            ],
            Core::Min8 => [
                0xcb, 0x1e, 0x4f, 0x64, 0xd8, 0x34, 0x48, 0x59, 0x0d, 0xbc, 0x5a, 0xd2, 0x56, 0x88,
                0x46, 0x11, 0xb1, 0x15, 0x6b, 0x9a, 0xd2, 0x35, 0x44, 0x0f, 0x44, 0xe6, 0x60, 0x29,
                0x4e, 0xff, 0xfc, 0x2c,
            ],
            Core::Modulo16 => [
                0xb6, 0x4f, 0xe4, 0xab, 0xf6, 0x84, 0xbc, 0x0c, 0xb4, 0x1c, 0x29, 0xce, 0x9d, 0xc2,
                0xc5, 0xd8, 0xef, 0xd5, 0x2c, 0x46, 0x3d, 0x08, 0xcb, 0x88, 0xcf, 0x6c, 0x84, 0x88,
                0x90, 0xe2, 0x80, 0xcd,
            ],
            Core::Modulo32 => [
                0x34, 0xda, 0x46, 0x60, 0xae, 0xea, 0xfe, 0x01, 0x52, 0x9d, 0x8c, 0xe9, 0xe1, 0x52,
                0x84, 0x23, 0xcb, 0x87, 0x03, 0x24, 0x2e, 0x67, 0x2c, 0x61, 0x09, 0xfe, 0xc5, 0xc1,
                0x16, 0x79, 0x50, 0x6d,
            ],
            Core::Modulo64 => [
                0xcb, 0x1c, 0x3c, 0x88, 0x45, 0x2b, 0x06, 0xaf, 0xa9, 0x7f, 0x37, 0xf9, 0x55, 0x3d,
                0xbd, 0x28, 0x9b, 0xda, 0x35, 0x3a, 0x8b, 0xd1, 0xdb, 0x52, 0x73, 0xe4, 0xa3, 0xcb,
                0xc1, 0x8f, 0x63, 0xcd,
            ],
            Core::Modulo8 => [
                0xc9, 0x77, 0xb5, 0x79, 0xa7, 0x93, 0x13, 0xd2, 0x1a, 0x0a, 0x7c, 0xf5, 0x5d, 0xbf,
                0x77, 0xee, 0xa3, 0x24, 0x23, 0xb1, 0xeb, 0xf8, 0xdd, 0x48, 0x97, 0x0b, 0x4e, 0xb5,
                0x0a, 0xc2, 0x96, 0xde,
            ],
            Core::Multiply16 => [
                0xc9, 0xe1, 0xcb, 0x83, 0xb9, 0x0e, 0xd4, 0xef, 0xea, 0x73, 0xcd, 0x85, 0xe7, 0x5f,
                0x9d, 0x04, 0x23, 0xfe, 0xd3, 0x12, 0x6f, 0x64, 0x71, 0xe4, 0x45, 0x75, 0xc7, 0xf9,
                0x6c, 0x49, 0x4c, 0xb1,
            ],
            Core::Multiply32 => [
                0x2a, 0x82, 0xb6, 0x2f, 0x5f, 0xb9, 0x45, 0xc0, 0x4e, 0xb7, 0x8a, 0x48, 0x9c, 0x28,
                0xae, 0x2f, 0xa1, 0x37, 0x6c, 0x49, 0x64, 0xd9, 0xe2, 0x06, 0x26, 0x2d, 0x13, 0xb0,
                0xd5, 0x16, 0xc5, 0x4a,
            ],
            Core::Multiply64 => [
                0x41, 0x36, 0xa3, 0x38, 0xa8, 0x2e, 0xdd, 0x39, 0xbf, 0x9f, 0x69, 0xac, 0xad, 0x06,
                0xa6, 0x29, 0xef, 0x5e, 0xea, 0xdc, 0xdc, 0xd0, 0xa5, 0x7f, 0xca, 0x2e, 0xf2, 0x45,
                0x96, 0x6d, 0x89, 0x2e,
            ],
            Core::Multiply8 => [
                0x25, 0x82, 0x99, 0x3d, 0x80, 0xb9, 0x8e, 0xd1, 0xe0, 0x6d, 0x30, 0x13, 0xb9, 0x3c,
                0x41, 0x2c, 0xc2, 0x34, 0x3a, 0x79, 0xc9, 0xa7, 0x25, 0x38, 0x4c, 0xa1, 0x18, 0x11,
                0xd0, 0xb9, 0x08, 0x0b,
            ],
            Core::Negate16 => [
                0xdf, 0xb1, 0x32, 0x60, 0x3d, 0x54, 0x1a, 0x98, 0x94, 0x8e, 0x16, 0x2f, 0xc9, 0x58,
                0xb0, 0x55, 0xb4, 0x9e, 0x60, 0x06, 0x85, 0x0b, 0xe6, 0x67, 0xad, 0x0c, 0x94, 0xbf,
                0xa4, 0x51, 0x21, 0x55,
            ],
            Core::Negate32 => [
                0xd8, 0x1e, 0xb9, 0xd8, 0x8a, 0x19, 0xd9, 0x14, 0x03, 0x0e, 0x31, 0x86, 0x38, 0xef,
                0xd8, 0x53, 0x5b, 0xd9, 0x65, 0xe9, 0x51, 0x36, 0xfc, 0xa4, 0x98, 0xd2, 0x1d, 0x30,
                0x55, 0x30, 0x27, 0x08,
            ],
            Core::Negate64 => [
                0x38, 0xd7, 0x4e, 0x41, 0xe4, 0x67, 0x04, 0x44, 0xb9, 0xe7, 0x86, 0x80, 0x2e, 0x17,
                0x43, 0x43, 0x88, 0x46, 0x22, 0x0a, 0x60, 0xac, 0xcd, 0xd9, 0xc6, 0x62, 0x25, 0xc4,
                0x7e, 0x47, 0x05, 0x53,
            ],
            Core::Negate8 => [
                0xbc, 0xc2, 0x52, 0xb8, 0x13, 0xf2, 0x19, 0x7e, 0xbd, 0x25, 0xa9, 0xcb, 0x91, 0x3f,
                0x26, 0x97, 0xcd, 0xd0, 0x8a, 0x61, 0x68, 0x93, 0x6a, 0x0c, 0x65, 0xfc, 0x89, 0x7e,
                0x33, 0xbb, 0xfd, 0xfc,
            ],
            Core::One16 => [
                0xf6, 0xa1, 0x4c, 0x50, 0x61, 0x90, 0xcc, 0x5d, 0xbd, 0xfb, 0x24, 0x92, 0x02, 0x7a,
                0x4d, 0xe3, 0x79, 0xdd, 0x2d, 0x57, 0x85, 0xe0, 0x51, 0x2d, 0x43, 0xfa, 0xeb, 0x1c,
                0xd2, 0x51, 0x28, 0x14,
            ],
            Core::One32 => [
                0xc4, 0x2b, 0x70, 0xb5, 0x2a, 0xa3, 0x97, 0x8c, 0x28, 0x82, 0xb8, 0xc2, 0x32, 0x36,
                0x4b, 0xe8, 0xf4, 0x8e, 0xd7, 0x3d, 0x33, 0x08, 0x6b, 0xe7, 0x2c, 0x38, 0xf1, 0xc3,
                0x66, 0xf4, 0x35, 0x02,
            ],
            Core::One64 => [
                0x40, 0xc3, 0x17, 0xbc, 0x23, 0xce, 0x5a, 0x48, 0x6a, 0xe3, 0x25, 0x05, 0xa3, 0x8a,
                0xd1, 0x7a, 0x84, 0x47, 0x9a, 0x10, 0x86, 0x71, 0x5b, 0x92, 0xac, 0x29, 0x60, 0xbd,
                0xfe, 0x0e, 0xdb, 0xc9,
            ],
            Core::One8 => [
                0xc0, 0xd3, 0x51, 0xba, 0x41, 0x26, 0xb6, 0x0e, 0x88, 0x8d, 0xcc, 0x1e, 0x59, 0xf1,
                0x23, 0x98, 0xb4, 0xd8, 0x5e, 0xfd, 0x18, 0xfe, 0x3d, 0xa4, 0x74, 0xa2, 0x96, 0x6b,
                0x04, 0xa6, 0x13, 0x17,
            ],
            Core::Or16 => [
                0x58, 0xd7, 0x09, 0xe8, 0xaa, 0xf4, 0x12, 0xff, 0x27, 0x99, 0x54, 0x7a, 0xc9, 0x14,
                0x4a, 0x79, 0x77, 0xce, 0x49, 0xcc, 0x8e, 0x95, 0x63, 0xd4, 0x5d, 0x68, 0x00, 0xad,
                0x5e, 0x4e, 0xff, 0x2e,
            ],
            Core::Or32 => [
                0x8e, 0x03, 0x78, 0xd2, 0x41, 0xc1, 0x42, 0x37, 0xd2, 0x18, 0xa4, 0x8f, 0xfd, 0x04,
                0x51, 0x63, 0x4d, 0xe5, 0x63, 0xa4, 0xd4, 0xba, 0x7c, 0x7a, 0xc8, 0xc0, 0x42, 0x8d,
                0x33, 0xd1, 0x67, 0x8e,
            ],
            Core::Or64 => [
                0xc0, 0x3a, 0xbc, 0x73, 0x13, 0xff, 0xbd, 0x09, 0xc8, 0x69, 0xb4, 0xa8, 0xfd, 0x61,
                0x0c, 0x94, 0x12, 0xd0, 0x5a, 0x94, 0x26, 0x09, 0xaf, 0xaf, 0xbc, 0x8b, 0xa6, 0x07,
                0xb4, 0x5d, 0xb8, 0xd8,
            ],
            Core::Or8 => [
                0x0a, 0x28, 0xc7, 0xf0, 0xd0, 0x94, 0x42, 0x58, 0xb1, 0xa0, 0x49, 0x00, 0xf0, 0x33,
                0x82, 0x07, 0x31, 0xef, 0xea, 0xb6, 0x44, 0x10, 0x2c, 0x7a, 0xa5, 0x45, 0xaa, 0x32,
                0x76, 0xbd, 0x22, 0x73,
            ],
            Core::ParseLock => [
                0x13, 0x83, 0xd3, 0x28, 0xa4, 0x0b, 0xc6, 0x34, 0x21, 0x74, 0xba, 0xaf, 0x9a, 0x39,
                0x09, 0x32, 0xc1, 0xfe, 0x4f, 0x2f, 0x4a, 0x8e, 0xc7, 0xfd, 0x21, 0x8b, 0xab, 0xf1,
                0x80, 0x5f, 0xb7, 0xe5,
            ],
            Core::ParseSequence => [
                0xc9, 0x61, 0x77, 0x69, 0x10, 0x6d, 0x45, 0xa3, 0x70, 0xf2, 0x93, 0xaa, 0x22, 0x2e,
                0xe6, 0xb4, 0x15, 0x02, 0xe2, 0x74, 0xce, 0x41, 0xb9, 0x05, 0x9c, 0x9d, 0xde, 0x89,
                0x73, 0xcd, 0x80, 0xc7,
            ],
            Core::PointVerify1 => [
                0x86, 0x94, 0x47, 0xdc, 0x82, 0xa9, 0xed, 0xf8, 0xb1, 0x44, 0xa3, 0xee, 0xba, 0xfd,
                0x7a, 0x5d, 0xcb, 0x70, 0xf4, 0x58, 0x3d, 0x95, 0xd7, 0x85, 0xdc, 0x01, 0xfd, 0xb3,
                0x61, 0x48, 0x1e, 0xd5,
            ],
            Core::ScalarAdd => [
                0x47, 0xfa, 0xd9, 0x4f, 0xfa, 0x74, 0x1b, 0x30, 0x22, 0x34, 0x80, 0x5e, 0x5d, 0xa5,
                0x43, 0xce, 0xcc, 0x8e, 0x0b, 0xf8, 0x61, 0xc6, 0x07, 0xe3, 0xd2, 0x5a, 0x84, 0x2c,
                0xe6, 0x1e, 0x06, 0xda,
            ],
            Core::ScalarInvert => [
                0x6d, 0x59, 0xed, 0xd6, 0xa8, 0x2f, 0x20, 0x85, 0xa7, 0x9e, 0x82, 0x2b, 0x1c, 0xd0,
                0x21, 0x43, 0x1d, 0x80, 0xe3, 0xf3, 0x63, 0x5b, 0x25, 0x5b, 0xc7, 0x9a, 0xbd, 0x47,
                0x21, 0xf8, 0x2f, 0xdb,
            ],
            Core::ScalarIsZero => [
                0xb2, 0x18, 0x81, 0xe2, 0xbd, 0xfa, 0x6a, 0xf0, 0x05, 0xf1, 0xbd, 0x1e, 0x36, 0xe0,
                0x42, 0x11, 0x27, 0x48, 0x47, 0x14, 0x77, 0xe2, 0x08, 0x9f, 0xc1, 0xe3, 0x51, 0xc7,
                0x15, 0x02, 0xc5, 0xca,
            ],
            Core::ScalarMultiply => [
                0xe6, 0xf6, 0xb6, 0x55, 0x01, 0xda, 0x00, 0xb2, 0x7b, 0x1f, 0x4e, 0xfe, 0x31, 0xc2,
                0x71, 0x2f, 0x31, 0xd1, 0xb1, 0x76, 0xa9, 0xcc, 0xdd, 0x0c, 0xf0, 0x0c, 0x58, 0x7a,
                0xf1, 0x2a, 0x05, 0xfd,
            ],
            Core::ScalarMultiplyLambda => [
                0x4d, 0xdf, 0x8f, 0xcb, 0x58, 0xff, 0xae, 0x7b, 0x3a, 0xa6, 0xbc, 0x52, 0xff, 0xae,
                0x76, 0xad, 0xa3, 0x37, 0x04, 0x5b, 0xa0, 0x09, 0x80, 0xf5, 0x21, 0x58, 0x09, 0x9b,
                0x53, 0x9a, 0x94, 0x2e,
            ],
            Core::ScalarNegate => [
                0x00, 0x7f, 0xba, 0x3d, 0x50, 0xde, 0xb6, 0x03, 0xdd, 0x23, 0x3e, 0x94, 0x63, 0x11,
                0x7d, 0x49, 0x36, 0x03, 0x3a, 0x50, 0xf6, 0x82, 0x21, 0x90, 0x0c, 0x93, 0x19, 0x53,
                0xf8, 0x2c, 0x16, 0x03,
            ],
            Core::ScalarNormalize => [
                0xbb, 0xfa, 0x23, 0x5a, 0x95, 0x76, 0xd3, 0xd6, 0xeb, 0x82, 0xf3, 0xc9, 0x44, 0x27,
                0x69, 0x4f, 0xc5, 0xd9, 0x7a, 0x90, 0xd4, 0xc4, 0xee, 0x32, 0xcd, 0x80, 0xd0, 0xdb,
                0xad, 0x99, 0xd9, 0x3a,
            ],
            Core::ScalarSquare => [
                0xa5, 0xcb, 0x22, 0x26, 0xa0, 0x9f, 0x26, 0xaa, 0x53, 0xf1, 0x54, 0x80, 0x34, 0xcd,
                0x58, 0xcc, 0x63, 0x1b, 0xea, 0x69, 0x72, 0x65, 0x8a, 0x9c, 0x55, 0x69, 0x8c, 0x39,
                0x7f, 0x0a, 0x84, 0x5b,
            ],
            Core::Scale => [
                0xa7, 0x23, 0x53, 0x8c, 0x2d, 0x33, 0xf7, 0x67, 0x9a, 0xa9, 0x48, 0xdd, 0x96, 0x40,
                0x7c, 0x3d, 0xdd, 0xaf, 0x02, 0xac, 0x27, 0xff, 0x2e, 0xc2, 0x8d, 0x4c, 0xc2, 0x7e,
                0xea, 0xfe, 0x6b, 0xc3,
            ],
            Core::Sha256Block => [
                0xd3, 0xe3, 0xc6, 0x9a, 0xed, 0x82, 0x2b, 0x24, 0xf9, 0x46, 0x14, 0xf2, 0xc2, 0xd7,
                0xcb, 0x39, 0x1d, 0x3a, 0x17, 0x17, 0xf1, 0x3d, 0xe9, 0xd5, 0x89, 0xcf, 0x0a, 0x14,
                0x62, 0x46, 0x68, 0xde,
            ],
            Core::Sha256Ctx8Add1 => [
                0x4a, 0xaf, 0xee, 0x24, 0x56, 0xb5, 0x1e, 0x53, 0x62, 0x2c, 0x9f, 0x82, 0xc3, 0xe4,
                0x6c, 0x91, 0xb2, 0xd7, 0x9b, 0x65, 0x9c, 0x54, 0xf5, 0xfa, 0x67, 0x1b, 0x04, 0xd6,
                0x0a, 0x42, 0xc5, 0x6a,
            ],
            Core::Sha256Ctx8Add128 => [
                0xae, 0x96, 0x07, 0x80, 0x6c, 0xc3, 0xdd, 0x3a, 0xe4, 0x54, 0x23, 0xbe, 0x24, 0x22,
                0x69, 0x6a, 0xf4, 0xe9, 0xad, 0xfc, 0xc5, 0xac, 0xa8, 0x8e, 0xa7, 0x96, 0xc6, 0xfb,
                0x64, 0xef, 0x24, 0x86,
            ],
            Core::Sha256Ctx8Add16 => [
                0x35, 0x91, 0x2a, 0x51, 0xd9, 0xba, 0xdb, 0x3f, 0x88, 0x47, 0xed, 0xdc, 0x53, 0xc8,
                0xb2, 0xeb, 0xdb, 0x62, 0xdd, 0x3b, 0x49, 0x7f, 0xe0, 0xb4, 0x77, 0x26, 0xc0, 0xc7,
                0x02, 0x9e, 0xcd, 0x89,
            ],
            Core::Sha256Ctx8Add2 => [
                0x86, 0xfa, 0xfa, 0x03, 0x8b, 0x9d, 0x86, 0xf7, 0xb5, 0x5e, 0x5a, 0xd7, 0x31, 0x8b,
                0x14, 0x0f, 0x56, 0x28, 0x91, 0xc9, 0x11, 0x4a, 0xd0, 0xca, 0xcb, 0x99, 0x53, 0xc6,
                0xa2, 0xed, 0xb9, 0x94,
            ],
            Core::Sha256Ctx8Add256 => [
                0xe7, 0xc6, 0x52, 0x1b, 0x4e, 0x3a, 0xdb, 0xbb, 0x34, 0xdf, 0x28, 0x6a, 0x41, 0xdc,
                0xa1, 0x37, 0xf6, 0xa1, 0xc3, 0xee, 0x40, 0x6f, 0xd5, 0x96, 0x28, 0x71, 0x4d, 0xb1,
                0x07, 0x46, 0x62, 0xa5,
            ],
            Core::Sha256Ctx8Add32 => [
                0x16, 0xfa, 0xdf, 0xc2, 0xfb, 0x9e, 0x73, 0x83, 0x85, 0xbf, 0x08, 0x19, 0x14, 0xaf,
                0xc9, 0x74, 0x30, 0xed, 0xce, 0x32, 0x0e, 0x12, 0x03, 0xac, 0xc2, 0x05, 0x49, 0xdb,
                0x89, 0xa1, 0xc2, 0x9e,
            ],
            Core::Sha256Ctx8Add4 => [
                0xb7, 0xa3, 0x0a, 0xec, 0x7d, 0x6d, 0x78, 0xad, 0x83, 0x57, 0xa4, 0xc6, 0x2b, 0xe7,
                0xbf, 0x6d, 0xe0, 0x7e, 0x44, 0x01, 0x06, 0xb5, 0x3c, 0xcb, 0xf9, 0xef, 0x41, 0x02,
                0x2f, 0x03, 0xd7, 0xe2,
            ],
            Core::Sha256Ctx8Add512 => [
                0xb3, 0x19, 0xe2, 0x84, 0x6a, 0xd9, 0xce, 0x46, 0xe8, 0xb4, 0x78, 0xc6, 0x47, 0x00,
                0x7b, 0xbd, 0x4b, 0x3a, 0x1a, 0x45, 0xbd, 0x2e, 0xe4, 0x79, 0x96, 0xfa, 0x0a, 0x2a,
                0xf4, 0x0e, 0xbd, 0x52,
            ],
            Core::Sha256Ctx8Add64 => [
                0x75, 0x14, 0xd9, 0x08, 0x17, 0x86, 0x02, 0x4d, 0x71, 0x18, 0x99, 0x15, 0xb0, 0x05,
                0xd7, 0xc2, 0x21, 0x74, 0x8e, 0x48, 0x2f, 0xd8, 0x16, 0xb8, 0x81, 0x14, 0x5b, 0xe2,
                0x97, 0x97, 0xb4, 0xdd,
            ],
            Core::Sha256Ctx8Add8 => [
                0xb0, 0xb7, 0x13, 0x1a, 0x41, 0x12, 0xc6, 0x53, 0x06, 0xe2, 0x2a, 0xd3, 0x54, 0x70,
                0x4e, 0x3e, 0xc2, 0xd7, 0x44, 0x43, 0x4f, 0x4a, 0x4d, 0x6a, 0xe8, 0xbc, 0xfd, 0x35,
                0x39, 0xc1, 0x0a, 0xdf,
            ],
            Core::Sha256Ctx8AddBuffer511 => [
                0xc2, 0x88, 0xe0, 0xd1, 0xfd, 0xdd, 0xb9, 0xe0, 0xae, 0xf4, 0xfa, 0x05, 0x9c, 0xc8,
                0x83, 0x0e, 0x71, 0x25, 0xa6, 0x10, 0xb6, 0xa3, 0xa7, 0x1e, 0x8a, 0xa4, 0x96, 0x69,
                0x6e, 0xa7, 0x73, 0xfb,
            ],
            Core::Sha256Ctx8Finalize => [
                0x59, 0x68, 0x67, 0x2b, 0x01, 0xc7, 0x2f, 0x8e, 0x84, 0xfa, 0xc9, 0xd1, 0x7e, 0x1e,
                0x9d, 0xf8, 0xf8, 0x78, 0x6b, 0x4b, 0x47, 0x04, 0x04, 0x4d, 0xd5, 0x7e, 0xaf, 0x77,
                0x36, 0x03, 0x88, 0x8f,
            ],
            Core::Sha256Ctx8Init => [
                0x79, 0xa0, 0x29, 0x55, 0x61, 0xf6, 0x55, 0x4f, 0x04, 0xcb, 0x94, 0xc9, 0x89, 0x68,
                0xd1, 0xbd, 0x69, 0x66, 0xfc, 0x97, 0x85, 0x53, 0x83, 0x2b, 0x20, 0xf7, 0x5c, 0x2b,
                0x15, 0x56, 0x59, 0xba,
            ],
            Core::Sha256Iv => [
                0x90, 0x25, 0x1f, 0x75, 0xab, 0xf9, 0x37, 0x8f, 0xfa, 0x81, 0x5e, 0x74, 0x2c, 0x89,
                0x4a, 0x38, 0xb9, 0x42, 0xf2, 0x8c, 0xff, 0xe9, 0xa4, 0x47, 0xca, 0xbb, 0xe6, 0xdd,
                0x2c, 0x2c, 0xd0, 0x26,
            ],
            Core::Some16 => [
                0xce, 0xcb, 0x74, 0xb8, 0x31, 0x35, 0xf5, 0x34, 0x0b, 0x6d, 0x88, 0xf3, 0xb7, 0x12,
                0xf1, 0x50, 0x4d, 0x2e, 0x50, 0x32, 0x86, 0x7f, 0xa7, 0xb9, 0xa2, 0xc4, 0xb4, 0x24,
                0xd9, 0x65, 0xc7, 0x8e,
            ],
            Core::Some32 => [
                0x0c, 0xe7, 0xca, 0xb8, 0x0f, 0x07, 0xb1, 0x2d, 0x9c, 0x9b, 0x94, 0xc4, 0x71, 0xd9,
                0x58, 0xa1, 0xe7, 0x24, 0x14, 0xb9, 0x34, 0x8c, 0x1c, 0x53, 0xe0, 0x32, 0x4c, 0x2b,
                0x89, 0xde, 0x26, 0xf8,
            ],
            Core::Some64 => [
                0x2f, 0xe1, 0xb4, 0x08, 0x34, 0xae, 0x9f, 0x5c, 0xcb, 0xcf, 0xa8, 0xe7, 0x91, 0x9e,
                0xb1, 0x1c, 0x71, 0x46, 0x88, 0x91, 0x71, 0x10, 0xae, 0xea, 0x8d, 0xb7, 0x78, 0xd8,
                0x5d, 0xa4, 0xe6, 0x43,
            ],
            Core::Some8 => [
                0x33, 0xd9, 0x58, 0xa6, 0x8c, 0x24, 0xf6, 0x3b, 0xe7, 0xb0, 0x06, 0x99, 0x39, 0xcc,
                0xfc, 0x22, 0xb6, 0xa4, 0x34, 0x80, 0x52, 0x5f, 0x78, 0x56, 0x70, 0xed, 0xd1, 0x34,
                0xff, 0x24, 0xf6, 0xd0,
            ],
            Core::Subtract16 => [
                0x7d, 0x22, 0xf4, 0x99, 0x7a, 0x66, 0xd1, 0x74, 0x31, 0xc2, 0x34, 0x81, 0xf4, 0x0a,
                0x44, 0xd3, 0xb4, 0x5f, 0x9d, 0x8a, 0xbf, 0x54, 0x96, 0x32, 0x54, 0x52, 0x77, 0x1a,
                0x87, 0xc7, 0x7c, 0x6b,
            ],
            Core::Subtract32 => [
                0x5b, 0x3b, 0xe7, 0xfd, 0x8a, 0xb0, 0x14, 0x31, 0x49, 0xc1, 0xd9, 0xf9, 0xd8, 0x8a,
                0x9d, 0x99, 0xea, 0x70, 0xfd, 0xc0, 0x86, 0xbc, 0xf3, 0x88, 0xbf, 0x9f, 0x4d, 0xd9,
                0xe1, 0x2d, 0x17, 0x3c,
            ],
            Core::Subtract64 => [
                0x17, 0x65, 0x3e, 0xdb, 0xdd, 0xea, 0x93, 0xa4, 0xc2, 0x0f, 0xf3, 0x59, 0xad, 0xc9,
                0x97, 0x5b, 0x7c, 0x2a, 0xbf, 0x8c, 0xf3, 0x2a, 0x1b, 0xc1, 0x47, 0xe2, 0xc9, 0x00,
                0x25, 0xbc, 0xfd, 0x19,
            ],
            Core::Subtract8 => [
                0x77, 0x86, 0x49, 0xf8, 0x39, 0x35, 0xcd, 0x3f, 0xfc, 0x04, 0xf6, 0xc6, 0x62, 0x0a,
                0x33, 0x96, 0x5d, 0x0e, 0xf1, 0xdd, 0x17, 0xe7, 0xb5, 0x0f, 0xa4, 0x10, 0x76, 0xd8,
                0xad, 0x87, 0x6b, 0x1d,
            ],
            Core::Verify => [
                0xa2, 0x06, 0x23, 0xb6, 0x86, 0xa6, 0xbd, 0xb6, 0x18, 0xb5, 0x0b, 0x4c, 0x5e, 0x8e,
                0xee, 0x06, 0x01, 0xcc, 0x84, 0x01, 0x34, 0xcb, 0x9e, 0x54, 0x80, 0xf7, 0xb6, 0xde,
                0x89, 0x49, 0x48, 0x9e,
            ],
            Core::Xor16 => [
                0xed, 0x59, 0x8a, 0xe3, 0x6d, 0x75, 0x61, 0xe7, 0xaa, 0x6e, 0xdc, 0x8c, 0x88, 0xa2,
                0xc8, 0x86, 0x66, 0xd6, 0x3f, 0xaf, 0x1d, 0xb3, 0x22, 0xde, 0x2a, 0xc6, 0x32, 0x47,
                0x0d, 0x41, 0xf1, 0xfa,
            ],
            Core::Xor32 => [
                0x0f, 0x1e, 0xa1, 0xed, 0x1f, 0xf2, 0x28, 0x93, 0x8b, 0x66, 0x38, 0x55, 0xd1, 0x50,
                0xc2, 0x8e, 0x7d, 0xfa, 0x0c, 0x17, 0x84, 0x16, 0x18, 0x34, 0x30, 0x68, 0x3f, 0xca,
                0x1e, 0x99, 0xb5, 0xb4,
            ],
            Core::Xor64 => [
                0x7d, 0x97, 0x32, 0x8d, 0xb2, 0x12, 0xf1, 0xaa, 0x00, 0xd6, 0x15, 0x62, 0x07, 0x5e,
                0x02, 0xc0, 0xfe, 0x49, 0x36, 0x3a, 0x32, 0x0b, 0xf2, 0x07, 0x4b, 0x7b, 0xc3, 0x00,
                0xe2, 0x3a, 0xb2, 0xed,
            ],
            Core::Xor8 => [
                0x87, 0x6c, 0xd5, 0x80, 0x98, 0x6b, 0xc2, 0x08, 0x5a, 0xc4, 0xf4, 0xeb, 0x5e, 0x02,
                0x1e, 0x05, 0x63, 0x50, 0x2d, 0x1c, 0x97, 0x08, 0x70, 0xf0, 0x3e, 0x80, 0x03, 0xb0,
                0x4f, 0xe6, 0xcf, 0x7d,
            ],
            Core::XorXor16 => [
                0x25, 0xde, 0xc0, 0x2c, 0x5c, 0xaa, 0xed, 0xa6, 0x6e, 0xa2, 0xab, 0x32, 0xe5, 0xea,
                0x41, 0xb7, 0x68, 0x24, 0x12, 0x21, 0x20, 0xa3, 0xca, 0x0d, 0x9a, 0x4e, 0xd5, 0x0e,
                0x8c, 0x31, 0xc2, 0x45,
            ],
            Core::XorXor32 => [
                0x1e, 0x00, 0x76, 0xa0, 0x99, 0x93, 0x28, 0xaa, 0xce, 0x84, 0xc4, 0x82, 0x78, 0x0a,
                0xe5, 0x86, 0x93, 0x7d, 0x3e, 0x36, 0x5a, 0x3b, 0x32, 0xee, 0x73, 0xe2, 0xbf, 0x04,
                0xc8, 0x25, 0x37, 0xd2,
            ],
            Core::XorXor64 => [
                0x2b, 0x14, 0xc3, 0x27, 0xc5, 0xd5, 0x3c, 0xd1, 0x25, 0xe7, 0x42, 0x9f, 0x2b, 0x97,
                0x66, 0x4a, 0x85, 0x77, 0x8c, 0xdf, 0xad, 0x24, 0x53, 0xec, 0xb1, 0x93, 0xb3, 0x5d,
                0x8d, 0x31, 0xac, 0x47,
            ],
            Core::XorXor8 => [
                0xe2, 0x57, 0xcb, 0x12, 0xa9, 0x63, 0x24, 0x88, 0xa1, 0x5d, 0x49, 0x84, 0xfa, 0x38,
                0x88, 0x90, 0x3a, 0x95, 0x93, 0x86, 0xe8, 0x4f, 0x1c, 0x07, 0xe9, 0xd2, 0x9d, 0xf7,
                0x12, 0x04, 0x70, 0x71,
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
