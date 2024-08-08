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
    And1,
    And16,
    And32,
    And64,
    And8,
    Bip0340Verify,
    Ch1,
    Ch16,
    Ch32,
    Ch64,
    Ch8,
    CheckSigVerify,
    Complement1,
    Complement16,
    Complement32,
    Complement64,
    Complement8,
    Decompress,
    Decrement16,
    Decrement32,
    Decrement64,
    Decrement8,
    DivMod128_64,
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
    Eq1,
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
    FullLeftShift16_1,
    FullLeftShift16_2,
    FullLeftShift16_4,
    FullLeftShift16_8,
    FullLeftShift32_1,
    FullLeftShift32_16,
    FullLeftShift32_2,
    FullLeftShift32_4,
    FullLeftShift32_8,
    FullLeftShift64_1,
    FullLeftShift64_16,
    FullLeftShift64_2,
    FullLeftShift64_32,
    FullLeftShift64_4,
    FullLeftShift64_8,
    FullLeftShift8_1,
    FullLeftShift8_2,
    FullLeftShift8_4,
    FullMultiply16,
    FullMultiply32,
    FullMultiply64,
    FullMultiply8,
    FullRightShift16_1,
    FullRightShift16_2,
    FullRightShift16_4,
    FullRightShift16_8,
    FullRightShift32_1,
    FullRightShift32_16,
    FullRightShift32_2,
    FullRightShift32_4,
    FullRightShift32_8,
    FullRightShift64_1,
    FullRightShift64_16,
    FullRightShift64_2,
    FullRightShift64_32,
    FullRightShift64_4,
    FullRightShift64_8,
    FullRightShift8_1,
    FullRightShift8_2,
    FullRightShift8_4,
    FullSubtract16,
    FullSubtract32,
    FullSubtract64,
    FullSubtract8,
    GeIsOnCurve,
    GeNegate,
    GejAdd,
    GejDouble,
    GejEquiv,
    GejGeAdd,
    GejGeAddEx,
    GejGeEquiv,
    GejInfinity,
    GejIsInfinity,
    GejIsOnCurve,
    GejNegate,
    GejNormalize,
    GejRescale,
    GejXEquiv,
    GejYIsOdd,
    Generate,
    HashToCurve,
    High1,
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
    LeftExtend16_32,
    LeftExtend16_64,
    LeftExtend1_16,
    LeftExtend1_32,
    LeftExtend1_64,
    LeftExtend1_8,
    LeftExtend32_64,
    LeftExtend8_16,
    LeftExtend8_32,
    LeftExtend8_64,
    LeftPadHigh16_32,
    LeftPadHigh16_64,
    LeftPadHigh1_16,
    LeftPadHigh1_32,
    LeftPadHigh1_64,
    LeftPadHigh1_8,
    LeftPadHigh32_64,
    LeftPadHigh8_16,
    LeftPadHigh8_32,
    LeftPadHigh8_64,
    LeftPadLow16_32,
    LeftPadLow16_64,
    LeftPadLow1_16,
    LeftPadLow1_32,
    LeftPadLow1_64,
    LeftPadLow1_8,
    LeftPadLow32_64,
    LeftPadLow8_16,
    LeftPadLow8_32,
    LeftPadLow8_64,
    LeftRotate16,
    LeftRotate32,
    LeftRotate64,
    LeftRotate8,
    LeftShift16,
    LeftShift32,
    LeftShift64,
    LeftShift8,
    LeftShiftWith16,
    LeftShiftWith32,
    LeftShiftWith64,
    LeftShiftWith8,
    Leftmost16_1,
    Leftmost16_2,
    Leftmost16_4,
    Leftmost16_8,
    Leftmost32_1,
    Leftmost32_16,
    Leftmost32_2,
    Leftmost32_4,
    Leftmost32_8,
    Leftmost64_1,
    Leftmost64_16,
    Leftmost64_2,
    Leftmost64_32,
    Leftmost64_4,
    Leftmost64_8,
    Leftmost8_1,
    Leftmost8_2,
    Leftmost8_4,
    LinearCombination1,
    LinearVerify1,
    Low1,
    Low16,
    Low32,
    Low64,
    Low8,
    Lt16,
    Lt32,
    Lt64,
    Lt8,
    Maj1,
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
    Or1,
    Or16,
    Or32,
    Or64,
    Or8,
    ParseLock,
    ParseSequence,
    PointVerify1,
    RightExtend16_32,
    RightExtend16_64,
    RightExtend32_64,
    RightExtend8_16,
    RightExtend8_32,
    RightExtend8_64,
    RightPadHigh16_32,
    RightPadHigh16_64,
    RightPadHigh1_16,
    RightPadHigh1_32,
    RightPadHigh1_64,
    RightPadHigh1_8,
    RightPadHigh32_64,
    RightPadHigh8_16,
    RightPadHigh8_32,
    RightPadHigh8_64,
    RightPadLow16_32,
    RightPadLow16_64,
    RightPadLow1_16,
    RightPadLow1_32,
    RightPadLow1_64,
    RightPadLow1_8,
    RightPadLow32_64,
    RightPadLow8_16,
    RightPadLow8_32,
    RightPadLow8_64,
    RightRotate16,
    RightRotate32,
    RightRotate64,
    RightRotate8,
    RightShift16,
    RightShift32,
    RightShift64,
    RightShift8,
    RightShiftWith16,
    RightShiftWith32,
    RightShiftWith64,
    RightShiftWith8,
    Rightmost16_1,
    Rightmost16_2,
    Rightmost16_4,
    Rightmost16_8,
    Rightmost32_1,
    Rightmost32_16,
    Rightmost32_2,
    Rightmost32_4,
    Rightmost32_8,
    Rightmost64_1,
    Rightmost64_16,
    Rightmost64_2,
    Rightmost64_32,
    Rightmost64_4,
    Rightmost64_8,
    Rightmost8_1,
    Rightmost8_2,
    Rightmost8_4,
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
    Some1,
    Some16,
    Some32,
    Some64,
    Some8,
    Subtract16,
    Subtract32,
    Subtract64,
    Subtract8,
    Swu,
    Verify,
    Xor1,
    Xor16,
    Xor32,
    Xor64,
    Xor8,
    XorXor1,
    XorXor16,
    XorXor32,
    XorXor64,
    XorXor8,
}

impl Jet for Core {

    type Environment = ();
    type CJetEnvironment = ();

    fn c_jet_env(env: &Self::Environment) -> &Self::CJetEnvironment {
        env
    }

    fn cmr(&self) -> Cmr {
        let bytes = match self {
            Core::Add16 => [
                0xc4, 0xdd, 0x8c, 0x30, 0x97, 0xf9, 0x5b, 0xf9, 0x22, 0x9a, 0xf0, 0xc3, 0x48, 0x31,
                0x70, 0x32, 0x3a, 0xa2, 0x70, 0xaf, 0x42, 0x8a, 0x72, 0x57, 0x18, 0x14, 0x3b, 0xb5,
                0xfc, 0x80, 0x81, 0x1c,
            ],
            Core::Add32 => [
                0xd9, 0x8c, 0x0f, 0x69, 0xf6, 0xb1, 0xd4, 0x35, 0x29, 0x4d, 0x97, 0x83, 0x5e, 0xac,
                0x94, 0x01, 0x77, 0xa2, 0xfe, 0xaf, 0x0a, 0x6e, 0x9e, 0x47, 0x34, 0xc9, 0x12, 0x04,
                0x8e, 0x9e, 0xee, 0x42,
            ],
            Core::Add64 => [
                0xe6, 0x13, 0xeb, 0x03, 0x10, 0xa4, 0x4c, 0x22, 0xa6, 0x94, 0xee, 0x55, 0x49, 0x0d,
                0xa4, 0xea, 0xe3, 0x88, 0x11, 0xbf, 0xf8, 0x4a, 0xa0, 0x21, 0x31, 0xd9, 0x8f, 0x49,
                0x9d, 0x26, 0x1e, 0x71,
            ],
            Core::Add8 => [
                0x4a, 0x1f, 0x50, 0x9e, 0x5d, 0xe5, 0xc3, 0x98, 0x5f, 0x76, 0xa0, 0xdb, 0x65, 0xb0,
                0x4e, 0x06, 0x03, 0xbd, 0x0c, 0x1d, 0x9c, 0x5b, 0xbc, 0xb9, 0xa3, 0xec, 0x8e, 0x53,
                0xa0, 0xba, 0x32, 0x97,
            ],
            Core::All16 => [
                0x13, 0xf6, 0xeb, 0xb8, 0xda, 0x74, 0x0d, 0xb8, 0x25, 0x92, 0x04, 0x68, 0x80, 0x2c,
                0x4c, 0xf4, 0x12, 0x45, 0x01, 0x6b, 0xdf, 0x2f, 0xa0, 0x54, 0x8c, 0x86, 0xf4, 0x15,
                0x83, 0x86, 0x6c, 0x73,
            ],
            Core::All32 => [
                0xbb, 0x51, 0x5d, 0x4a, 0x22, 0xf3, 0x24, 0x5e, 0xcf, 0x57, 0x50, 0x35, 0xed, 0x18,
                0x7a, 0x03, 0x86, 0x14, 0x52, 0x4b, 0x35, 0xae, 0xd6, 0x44, 0x82, 0x46, 0x89, 0xde,
                0x38, 0xa4, 0xa1, 0xe0,
            ],
            Core::All64 => [
                0xcd, 0x15, 0x1d, 0xe9, 0x17, 0x57, 0x56, 0x55, 0xa7, 0xa0, 0xf3, 0x5d, 0xa9, 0xcd,
                0xaf, 0x6e, 0x2b, 0xbf, 0x8a, 0x8d, 0x4f, 0xce, 0xac, 0x68, 0xce, 0xe4, 0xb0, 0x08,
                0xe6, 0x65, 0xbf, 0xc3,
            ],
            Core::All8 => [
                0x0d, 0x3c, 0xbf, 0xc2, 0xcc, 0x2a, 0x9a, 0x06, 0x28, 0xa2, 0xff, 0x8a, 0xfc, 0x0a,
                0x5f, 0x93, 0x17, 0x3e, 0xde, 0x4c, 0x23, 0x4c, 0x57, 0x21, 0x4c, 0x5d, 0x74, 0xaf,
                0x6b, 0x87, 0x3c, 0xe9,
            ],
            Core::And1 => [
                0x56, 0xd1, 0xd4, 0xd6, 0xc1, 0xe1, 0x1f, 0x60, 0xd2, 0x96, 0x60, 0xb1, 0x81, 0xa1,
                0x33, 0x5a, 0x4f, 0x51, 0x1d, 0x3b, 0x9c, 0x04, 0xa3, 0x33, 0xe9, 0xe0, 0x14, 0x18,
                0xb0, 0xd1, 0x81, 0x8f,
            ],
            Core::And16 => [
                0xd7, 0x7a, 0x11, 0x44, 0xc5, 0x2e, 0xd2, 0xb7, 0x69, 0x88, 0xdf, 0x9b, 0x29, 0xcd,
                0x53, 0x97, 0x8c, 0x44, 0xc6, 0xd5, 0x40, 0xf4, 0xbc, 0x54, 0x56, 0xd0, 0xb3, 0xb5,
                0x3c, 0xd4, 0x00, 0xe5,
            ],
            Core::And32 => [
                0x43, 0x82, 0xf9, 0x34, 0x33, 0x79, 0xd4, 0x42, 0x2e, 0x2a, 0x01, 0x51, 0x04, 0xcf,
                0x55, 0xdc, 0x54, 0xa8, 0xf7, 0x1e, 0xef, 0xb0, 0x62, 0x29, 0x26, 0xd6, 0x4b, 0x6c,
                0xe2, 0x2b, 0xea, 0x11,
            ],
            Core::And64 => [
                0x3e, 0xb8, 0xb4, 0x2d, 0x81, 0x5f, 0x0f, 0xc3, 0x9a, 0x34, 0x99, 0xb0, 0x35, 0x4e,
                0xfd, 0x05, 0xa6, 0x03, 0xf5, 0xef, 0x5e, 0x51, 0x17, 0x66, 0xc0, 0x11, 0x2c, 0xc1,
                0x48, 0xe2, 0x11, 0x55,
            ],
            Core::And8 => [
                0x0e, 0x0e, 0xe0, 0x7c, 0x74, 0x9a, 0x3f, 0xa7, 0x12, 0x96, 0xb9, 0xd4, 0x59, 0xb0,
                0x6a, 0x0a, 0x4b, 0xff, 0xb7, 0xf5, 0xbb, 0x99, 0xa9, 0x25, 0x4a, 0x7d, 0x13, 0xa8,
                0xce, 0x44, 0x4c, 0xd7,
            ],
            Core::Bip0340Verify => [
                0x08, 0x73, 0xc4, 0xd7, 0xb8, 0x99, 0x8b, 0x2b, 0x6b, 0x8e, 0x43, 0xee, 0x4a, 0xc0,
                0xc2, 0xc7, 0xad, 0x25, 0xb3, 0x60, 0x0e, 0x34, 0x2b, 0x20, 0x1d, 0x46, 0x00, 0x1b,
                0x76, 0xc0, 0x56, 0x7f,
            ],
            Core::Ch1 => [
                0x29, 0x7c, 0x25, 0x1a, 0xfe, 0x74, 0x1b, 0x19, 0x22, 0xa2, 0x12, 0xe3, 0x93, 0xd3,
                0xd3, 0xb1, 0xc3, 0x2e, 0x6b, 0x33, 0xee, 0x2c, 0xfd, 0x5e, 0xfa, 0xe3, 0xfb, 0x1a,
                0xbe, 0x59, 0x51, 0x13,
            ],
            Core::Ch16 => [
                0x81, 0xd3, 0x38, 0x86, 0xb5, 0x25, 0xe7, 0xf2, 0x80, 0x19, 0xd4, 0x44, 0xd4, 0xd4,
                0xb4, 0x22, 0x76, 0xd3, 0x3f, 0xa9, 0x7b, 0x91, 0x7a, 0xdb, 0xef, 0x4a, 0x88, 0x11,
                0x3e, 0xa6, 0x71, 0x1e,
            ],
            Core::Ch32 => [
                0x77, 0x00, 0x22, 0x00, 0xec, 0xac, 0x62, 0xe7, 0x65, 0xc0, 0x55, 0xb5, 0x0b, 0xd2,
                0x25, 0xa8, 0x54, 0xcd, 0xa3, 0x18, 0x07, 0x1b, 0x2e, 0x70, 0xea, 0x30, 0xde, 0x9d,
                0x1e, 0x32, 0x19, 0xaa,
            ],
            Core::Ch64 => [
                0xe9, 0x7c, 0xdd, 0x2a, 0x47, 0xac, 0xf4, 0x0b, 0xe6, 0x87, 0x2a, 0x57, 0x4b, 0xba,
                0xdb, 0x5d, 0xb5, 0x1b, 0xac, 0x1a, 0xd9, 0xcf, 0x63, 0xe7, 0xd8, 0x4c, 0x3b, 0x2f,
                0xb9, 0xab, 0x6d, 0xdb,
            ],
            Core::Ch8 => [
                0x5b, 0xc6, 0x77, 0x08, 0x75, 0x1e, 0x96, 0x79, 0x13, 0xfb, 0x75, 0xfc, 0x25, 0x76,
                0xa0, 0x80, 0x70, 0x0c, 0x67, 0xf1, 0x2c, 0x36, 0x3b, 0xa0, 0x19, 0x17, 0x51, 0x3e,
                0x6c, 0x78, 0x14, 0x57,
            ],
            Core::CheckSigVerify => [
                0xa5, 0x16, 0xdd, 0x5c, 0x9c, 0xab, 0xd0, 0x41, 0x12, 0x0e, 0xa3, 0x68, 0xab, 0x4c,
                0xcd, 0x92, 0x31, 0x25, 0x5e, 0xf5, 0x79, 0x96, 0x85, 0x58, 0xc1, 0xaa, 0x6a, 0x21,
                0x2f, 0x16, 0x83, 0x29,
            ],
            Core::Complement1 => [
                0x9e, 0xa1, 0xeb, 0xfb, 0xbf, 0x07, 0x3b, 0xf0, 0xc3, 0x12, 0x66, 0x03, 0x74, 0x9c,
                0x6c, 0x1f, 0x15, 0x05, 0x03, 0x9e, 0xab, 0x4f, 0x26, 0x64, 0x3f, 0x03, 0x30, 0xd4,
                0x8b, 0xd6, 0x84, 0x94,
            ],
            Core::Complement16 => [
                0x7d, 0xa0, 0xcc, 0x5e, 0x1f, 0x55, 0x03, 0x6c, 0x08, 0x64, 0x6d, 0xbb, 0x37, 0x55,
                0x7a, 0xd9, 0x03, 0x38, 0x63, 0x05, 0x9d, 0xfb, 0xf3, 0x4a, 0x19, 0xf2, 0x28, 0xe8,
                0x96, 0xd6, 0x4c, 0x0f,
            ],
            Core::Complement32 => [
                0x54, 0x18, 0xc4, 0x64, 0xe4, 0x94, 0x97, 0x21, 0x63, 0xa3, 0x35, 0x3b, 0x81, 0x17,
                0xe6, 0xe7, 0x67, 0x49, 0x13, 0x48, 0xc0, 0x76, 0x3f, 0x96, 0x22, 0x74, 0x4a, 0x83,
                0x08, 0x2e, 0xf8, 0x41,
            ],
            Core::Complement64 => [
                0x00, 0x4e, 0xb9, 0xed, 0x9c, 0x10, 0xa4, 0xa5, 0xa8, 0x97, 0x94, 0x8e, 0x54, 0x1d,
                0x36, 0xdc, 0x1f, 0xb6, 0x50, 0xc8, 0xda, 0x72, 0x6f, 0x8d, 0xd1, 0x97, 0xce, 0x8c,
                0x74, 0x41, 0xf7, 0x08,
            ],
            Core::Complement8 => [
                0x94, 0x76, 0xd5, 0x1d, 0x6b, 0x1b, 0xe4, 0x4d, 0xcf, 0xf2, 0x19, 0x47, 0xd5, 0xbf,
                0x54, 0x0a, 0x66, 0x61, 0x6f, 0xf7, 0xb9, 0x32, 0x8a, 0x67, 0xf5, 0xb8, 0x3a, 0x2f,
                0x6f, 0x06, 0xf0, 0x2c,
            ],
            Core::Decompress => [
                0xca, 0xbe, 0x59, 0x5c, 0xb7, 0xbb, 0xb3, 0xa1, 0x39, 0xa7, 0x79, 0x6e, 0x43, 0x3a,
                0xd0, 0xb6, 0xc7, 0x4d, 0x07, 0x3f, 0x00, 0x1e, 0x8c, 0x08, 0x0c, 0x80, 0x0a, 0x78,
                0x2c, 0x7e, 0x79, 0x04,
            ],
            Core::Decrement16 => [
                0x25, 0x04, 0x10, 0x1b, 0x7d, 0xa5, 0xce, 0x31, 0x17, 0x43, 0xcf, 0x5b, 0x3a, 0x7a,
                0x6b, 0xb9, 0xd5, 0x7c, 0x24, 0xae, 0xbb, 0xb6, 0x76, 0x62, 0xd5, 0x3c, 0xda, 0x2c,
                0x9a, 0xc2, 0x70, 0x0c,
            ],
            Core::Decrement32 => [
                0x4a, 0xec, 0x41, 0x05, 0x53, 0x1f, 0x24, 0x57, 0x10, 0xac, 0xb3, 0xc3, 0x30, 0xad,
                0x25, 0xfb, 0x29, 0x01, 0xb3, 0x54, 0x83, 0xef, 0xbd, 0xca, 0x52, 0xe1, 0x91, 0x79,
                0xa4, 0x26, 0x2c, 0x5b,
            ],
            Core::Decrement64 => [
                0x98, 0x3a, 0xcb, 0x90, 0x6d, 0xe7, 0xbb, 0xf8, 0x31, 0xde, 0xa5, 0x62, 0x75, 0xf0,
                0x85, 0xfd, 0x5a, 0xcd, 0xdc, 0x51, 0x9b, 0x64, 0x8f, 0x3e, 0x10, 0xe3, 0x21, 0x2d,
                0x0d, 0x6d, 0xcd, 0x74,
            ],
            Core::Decrement8 => [
                0xa7, 0x5b, 0x1c, 0x5d, 0x93, 0xfe, 0x91, 0xe2, 0xc9, 0xef, 0xd3, 0x24, 0x24, 0x0c,
                0x84, 0xfe, 0xe4, 0x73, 0x32, 0xd1, 0x21, 0x42, 0x39, 0xc8, 0x7f, 0x5a, 0xfb, 0xcb,
                0x69, 0x12, 0x8c, 0x31,
            ],
            Core::DivMod128_64 => [
                0xe1, 0xfb, 0x88, 0x1f, 0x23, 0xbd, 0xc5, 0x7f, 0x84, 0x23, 0xc6, 0x1c, 0xfe, 0x62,
                0x23, 0x91, 0x5c, 0x76, 0xeb, 0x2f, 0x52, 0x5c, 0x41, 0xe2, 0xa2, 0x22, 0x1c, 0x78,
                0xb7, 0xff, 0xaf, 0xb7,
            ],
            Core::DivMod16 => [
                0x6e, 0x04, 0xad, 0x32, 0x8f, 0xba, 0x2d, 0x24, 0xef, 0xad, 0x2c, 0xb4, 0x0b, 0xca,
                0xd8, 0x24, 0x42, 0xce, 0x35, 0xb1, 0x2f, 0xcf, 0x6f, 0x80, 0x02, 0x2f, 0x52, 0xfa,
                0x77, 0x18, 0xa0, 0xfa,
            ],
            Core::DivMod32 => [
                0x5a, 0x5c, 0xc9, 0x4f, 0x74, 0xb3, 0x13, 0xc2, 0x60, 0xa1, 0x0e, 0x2a, 0x28, 0xdb,
                0x2e, 0xa0, 0xf8, 0xc3, 0x41, 0x56, 0x23, 0xda, 0xc2, 0x12, 0xdf, 0xe0, 0xef, 0x9d,
                0x19, 0x5f, 0x9d, 0x05,
            ],
            Core::DivMod64 => [
                0xef, 0xb5, 0xaf, 0x40, 0x15, 0x4a, 0x2d, 0x63, 0xda, 0x28, 0x5f, 0x2a, 0x2b, 0x5a,
                0xd0, 0x85, 0xd9, 0xa2, 0xfa, 0x63, 0x3f, 0x2a, 0x73, 0x62, 0x83, 0x40, 0x48, 0x54,
                0x30, 0x6c, 0xef, 0x50,
            ],
            Core::DivMod8 => [
                0xaf, 0xfb, 0x7a, 0xb0, 0x1d, 0x9a, 0xdf, 0x1a, 0xf4, 0x5c, 0x9a, 0x0e, 0x59, 0x8d,
                0x54, 0xa5, 0xeb, 0x29, 0xa6, 0xd6, 0x3f, 0x05, 0x94, 0xdc, 0x62, 0x4f, 0x8e, 0x44,
                0x6f, 0x42, 0xa4, 0x5d,
            ],
            Core::Divide16 => [
                0x98, 0xf1, 0xf9, 0x26, 0x0d, 0xf1, 0xd9, 0xe9, 0x72, 0x9d, 0x03, 0x7d, 0x8f, 0x30,
                0x53, 0x7e, 0xde, 0x60, 0x9c, 0x5a, 0xb9, 0x46, 0x32, 0x70, 0x70, 0x89, 0x2c, 0xc4,
                0x0d, 0x42, 0x0a, 0xaa,
            ],
            Core::Divide32 => [
                0x20, 0x18, 0x03, 0x03, 0x0c, 0xc9, 0x87, 0x7d, 0x34, 0x96, 0xe9, 0xe1, 0xbf, 0xd5,
                0xae, 0x32, 0x8a, 0x50, 0x62, 0xe3, 0xc0, 0x23, 0x70, 0x03, 0x1a, 0xe1, 0xad, 0xdf,
                0xef, 0x93, 0x81, 0x55,
            ],
            Core::Divide64 => [
                0x02, 0xbf, 0x0b, 0x2a, 0x31, 0x79, 0x6c, 0x47, 0x5a, 0xf1, 0xfd, 0x0f, 0xb8, 0xed,
                0xaf, 0xed, 0x4f, 0x4e, 0x7a, 0xe1, 0xec, 0xf8, 0xce, 0xc9, 0xf4, 0x22, 0x96, 0x4e,
                0x09, 0x03, 0x03, 0x6f,
            ],
            Core::Divide8 => [
                0xc0, 0x2e, 0xc4, 0x80, 0xbf, 0x0b, 0x59, 0x58, 0x6e, 0x39, 0x4a, 0xdb, 0x2c, 0x91,
                0xca, 0xda, 0x14, 0xed, 0x79, 0xa7, 0x9e, 0x0e, 0x39, 0x19, 0xe7, 0x66, 0x74, 0x92,
                0xcb, 0xf2, 0x47, 0x38,
            ],
            Core::Divides16 => [
                0xde, 0x9a, 0x8e, 0x43, 0x53, 0xa4, 0x0d, 0x70, 0x78, 0x11, 0x96, 0x47, 0x05, 0x90,
                0x62, 0x73, 0x0c, 0xf8, 0x36, 0xf5, 0x47, 0xa0, 0x95, 0x2d, 0x07, 0x0a, 0x27, 0x48,
                0x0c, 0x74, 0x39, 0xb1,
            ],
            Core::Divides32 => [
                0x72, 0xc7, 0x14, 0x8b, 0xc0, 0x01, 0x4a, 0x1b, 0xbf, 0x0e, 0x2b, 0x7d, 0xc9, 0xab,
                0xa4, 0xd0, 0x62, 0x04, 0x17, 0x38, 0x36, 0x71, 0x88, 0x13, 0x20, 0x3e, 0x55, 0xa0,
                0x7f, 0x4c, 0xa3, 0x7d,
            ],
            Core::Divides64 => [
                0xdb, 0x1c, 0x8c, 0xb5, 0x3a, 0xc1, 0xea, 0x8c, 0xcf, 0x64, 0x6d, 0x03, 0xe3, 0xd5,
                0x9b, 0xde, 0x27, 0x75, 0xf5, 0x8d, 0x95, 0xbf, 0xe7, 0x1a, 0x87, 0x0d, 0x67, 0x8d,
                0xd8, 0xdb, 0xc9, 0x31,
            ],
            Core::Divides8 => [
                0xc5, 0x4a, 0x5b, 0xf8, 0x33, 0xaf, 0xab, 0xa3, 0x1a, 0x34, 0x89, 0xbd, 0x8d, 0xe8,
                0xff, 0x81, 0xe7, 0xdc, 0x11, 0xfc, 0xbd, 0xe8, 0x2a, 0x0d, 0x28, 0xe1, 0x09, 0x22,
                0x42, 0x94, 0xb8, 0x37,
            ],
            Core::Eq1 => [
                0x8c, 0xe1, 0x46, 0xaf, 0xbc, 0x2b, 0x64, 0xcb, 0x54, 0x9b, 0x16, 0x0c, 0x2c, 0xf7,
                0xc9, 0x49, 0x64, 0xab, 0x93, 0x1a, 0xd9, 0xc3, 0x92, 0x76, 0xc7, 0xf2, 0x04, 0xee,
                0x34, 0xa5, 0x1e, 0x65,
            ],
            Core::Eq16 => [
                0x24, 0xf1, 0xac, 0xb6, 0xd1, 0x6b, 0x3b, 0xe5, 0xde, 0x47, 0xfe, 0x83, 0x43, 0x28,
                0xfb, 0x51, 0x04, 0x95, 0xfb, 0xba, 0x72, 0xcd, 0x1d, 0xc8, 0x9a, 0x33, 0x98, 0x3f,
                0x8c, 0x87, 0x33, 0xed,
            ],
            Core::Eq256 => [
                0xf3, 0x2e, 0x9b, 0x6c, 0x91, 0x2c, 0x50, 0xdd, 0x9a, 0x24, 0x10, 0xc8, 0x02, 0x4b,
                0x45, 0x48, 0x07, 0x07, 0xd9, 0xb3, 0x71, 0x4b, 0x52, 0x49, 0xc5, 0xf3, 0x01, 0xf2,
                0x1d, 0xb3, 0xe6, 0x2b,
            ],
            Core::Eq32 => [
                0x03, 0x43, 0x57, 0x56, 0xec, 0x24, 0x4c, 0x99, 0xff, 0x73, 0x32, 0x21, 0x57, 0xc2,
                0x6c, 0x36, 0x6b, 0xf5, 0x2e, 0xfb, 0x31, 0x4d, 0xce, 0x0c, 0x23, 0x61, 0xdd, 0x53,
                0x49, 0x1a, 0xa4, 0xbe,
            ],
            Core::Eq64 => [
                0x7d, 0xa4, 0x67, 0x58, 0x80, 0x60, 0x93, 0x66, 0xd7, 0x4d, 0x53, 0x0c, 0xa1, 0x64,
                0xa0, 0xe0, 0x9a, 0xc9, 0xce, 0x76, 0xf4, 0xf9, 0xe9, 0x43, 0x22, 0x06, 0x3c, 0xb3,
                0x26, 0xfe, 0xf6, 0xbb,
            ],
            Core::Eq8 => [
                0x9b, 0x50, 0xfe, 0x9d, 0x35, 0xb1, 0x0a, 0x3d, 0xb8, 0x66, 0x31, 0x1c, 0x53, 0x71,
                0x35, 0xd0, 0x80, 0xe4, 0x61, 0x43, 0x68, 0xdb, 0x28, 0x84, 0x33, 0xd8, 0xbc, 0x64,
                0x9b, 0x9f, 0x86, 0x62,
            ],
            Core::FeAdd => [
                0xac, 0xad, 0xfc, 0xf4, 0x80, 0x56, 0x26, 0xe3, 0x74, 0x59, 0x0f, 0x33, 0xca, 0xd7,
                0xa9, 0x83, 0xa7, 0x32, 0x0a, 0xc8, 0x1c, 0xed, 0x3d, 0x02, 0x54, 0xdc, 0x97, 0xe3,
                0xa2, 0x53, 0x5a, 0x82,
            ],
            Core::FeInvert => [
                0x2e, 0xa7, 0x6f, 0x67, 0x97, 0x5f, 0xce, 0xa3, 0xbd, 0x46, 0xfb, 0xb1, 0x4d, 0xe8,
                0x1f, 0x49, 0xfd, 0x08, 0xef, 0xcb, 0xc0, 0x87, 0x5a, 0xb2, 0x1d, 0xc9, 0x89, 0x76,
                0xa5, 0x30, 0x93, 0x22,
            ],
            Core::FeIsOdd => [
                0x7e, 0xde, 0xf0, 0x20, 0x5d, 0x4b, 0x80, 0x41, 0x82, 0x36, 0x9c, 0x31, 0xc2, 0x19,
                0x02, 0x46, 0x9b, 0x0f, 0x7a, 0x5b, 0x29, 0xb5, 0xdf, 0x90, 0xc3, 0x3f, 0x4b, 0xda,
                0xa0, 0x5d, 0x7c, 0x03,
            ],
            Core::FeIsZero => [
                0xa9, 0x43, 0x03, 0x64, 0x2e, 0x1a, 0x0f, 0x47, 0x79, 0x15, 0xa2, 0xd2, 0x51, 0xf8,
                0x45, 0x5c, 0x56, 0x87, 0x7e, 0xca, 0x79, 0xba, 0x40, 0xad, 0xc3, 0x82, 0x26, 0x8c,
                0xab, 0x2d, 0x63, 0xb7,
            ],
            Core::FeMultiply => [
                0xac, 0x64, 0xd5, 0x34, 0x72, 0xab, 0xa4, 0x89, 0xa8, 0xac, 0xf3, 0xac, 0x8a, 0xc7,
                0x9e, 0x8b, 0x11, 0x82, 0x43, 0x3e, 0x53, 0x50, 0xc4, 0xc3, 0x78, 0x2d, 0x45, 0xad,
                0xf1, 0xc5, 0x6d, 0xcb,
            ],
            Core::FeMultiplyBeta => [
                0xca, 0x2e, 0xa7, 0x97, 0xbe, 0x89, 0xe4, 0x1d, 0xb5, 0x15, 0x86, 0xea, 0xdd, 0xe3,
                0x36, 0x38, 0x11, 0xd0, 0x1d, 0x5d, 0x4c, 0x61, 0x42, 0xf5, 0x7e, 0x1b, 0x99, 0xf2,
                0xae, 0x03, 0x06, 0x1c,
            ],
            Core::FeNegate => [
                0x01, 0x42, 0xba, 0x1a, 0x10, 0x13, 0x13, 0x57, 0x81, 0x3b, 0x09, 0xf9, 0xce, 0x18,
                0xc0, 0x6b, 0x4b, 0xab, 0x74, 0x2c, 0xb4, 0x85, 0xd1, 0x79, 0x8b, 0x4a, 0x31, 0x3a,
                0x92, 0x93, 0xfa, 0x7e,
            ],
            Core::FeNormalize => [
                0x70, 0xe5, 0x57, 0x07, 0x96, 0x39, 0x41, 0x1d, 0xab, 0x31, 0xb3, 0xe4, 0xf4, 0xb5,
                0x60, 0x5a, 0x4b, 0x15, 0x59, 0x3a, 0x1a, 0xf7, 0x9b, 0xbd, 0xd9, 0xf9, 0xa1, 0x10,
                0xa8, 0x5b, 0x25, 0x81,
            ],
            Core::FeSquare => [
                0xcf, 0xa0, 0x2a, 0x6a, 0x95, 0x09, 0x1b, 0x14, 0x97, 0x00, 0x07, 0xb9, 0x1a, 0x01,
                0xba, 0x12, 0x57, 0xfe, 0x3b, 0x84, 0x95, 0x63, 0xcf, 0x42, 0xd0, 0x64, 0x94, 0xc3,
                0xb8, 0xd0, 0x4f, 0x9d,
            ],
            Core::FeSquareRoot => [
                0xf5, 0x1d, 0x5e, 0xd2, 0x19, 0xbc, 0x89, 0x41, 0xb2, 0x41, 0x29, 0x57, 0x2e, 0x84,
                0xba, 0xf8, 0x1a, 0x8c, 0xd2, 0xfe, 0x0e, 0x4e, 0xf2, 0x38, 0x86, 0x9c, 0xb2, 0xda,
                0x29, 0x6b, 0x0f, 0xa8,
            ],
            Core::FullAdd16 => [
                0xd3, 0x99, 0x2e, 0x38, 0xab, 0x1e, 0xf2, 0xc3, 0x83, 0x81, 0x30, 0x41, 0xdb, 0x13,
                0x69, 0xdd, 0xd5, 0xc3, 0xaa, 0x81, 0xae, 0x90, 0x24, 0xc5, 0xea, 0x26, 0x61, 0x19,
                0x6b, 0xd1, 0x1f, 0x01,
            ],
            Core::FullAdd32 => [
                0x46, 0x2b, 0x9c, 0x3b, 0xa6, 0x57, 0x6c, 0x11, 0x14, 0x55, 0x33, 0x77, 0x5b, 0xdf,
                0x95, 0x5b, 0x60, 0x48, 0x2e, 0x6e, 0x30, 0x80, 0xa8, 0x19, 0xf5, 0xcc, 0x73, 0xa5,
                0xca, 0x68, 0x30, 0xc1,
            ],
            Core::FullAdd64 => [
                0xc0, 0xfc, 0x79, 0x99, 0xd9, 0x1c, 0xa6, 0xf7, 0xb5, 0xf8, 0x72, 0xee, 0x6a, 0x8e,
                0x08, 0xea, 0x2a, 0x11, 0xaf, 0x7b, 0x92, 0xe7, 0xde, 0x60, 0x25, 0x17, 0x19, 0xd7,
                0x85, 0xb5, 0xf3, 0xe2,
            ],
            Core::FullAdd8 => [
                0x27, 0x14, 0xec, 0xbb, 0x32, 0x4f, 0xb9, 0xfb, 0x71, 0x9e, 0x09, 0x71, 0x6b, 0xdb,
                0xdf, 0xb5, 0x64, 0x66, 0xcb, 0x4f, 0x41, 0xf9, 0x01, 0xb1, 0x66, 0x78, 0xb0, 0x02,
                0x16, 0x17, 0x41, 0xaf,
            ],
            Core::FullDecrement16 => [
                0xbd, 0x0d, 0xd6, 0x91, 0xf8, 0x55, 0x44, 0x47, 0x04, 0xd9, 0xbb, 0xc2, 0xea, 0xf0,
                0x29, 0xc6, 0xca, 0x5b, 0x45, 0xcf, 0x92, 0x44, 0x1f, 0x7f, 0x68, 0xb4, 0x78, 0xb6,
                0x11, 0xb5, 0x97, 0x28,
            ],
            Core::FullDecrement32 => [
                0x27, 0x88, 0xc3, 0xa0, 0x17, 0x35, 0xa6, 0x4c, 0x0c, 0x36, 0x19, 0x03, 0xd7, 0x1e,
                0x21, 0xce, 0x10, 0x05, 0xce, 0x35, 0x74, 0xe6, 0x9a, 0x2a, 0x68, 0x0b, 0x87, 0x3e,
                0x5f, 0x73, 0x76, 0x6a,
            ],
            Core::FullDecrement64 => [
                0x14, 0x4b, 0xf9, 0xfe, 0x04, 0xba, 0x8c, 0x93, 0xa7, 0x83, 0x5b, 0x16, 0x36, 0xae,
                0x8f, 0x7d, 0x50, 0x5c, 0x05, 0x08, 0x92, 0xdd, 0xbc, 0xc8, 0xf9, 0x72, 0x5f, 0x6b,
                0x09, 0xc3, 0xe4, 0x7c,
            ],
            Core::FullDecrement8 => [
                0x3b, 0x0d, 0x4c, 0x9a, 0x02, 0xbd, 0xd9, 0x03, 0xb1, 0x2f, 0x49, 0xb8, 0xca, 0xd0,
                0xe3, 0xf0, 0x6c, 0xb9, 0xbc, 0xb6, 0xc8, 0xb1, 0x98, 0x95, 0x6d, 0xff, 0xce, 0x48,
                0x90, 0x8a, 0x63, 0xf1,
            ],
            Core::FullIncrement16 => [
                0x91, 0x81, 0x8a, 0x49, 0xe9, 0x5e, 0x22, 0x8c, 0x3b, 0x51, 0xc7, 0x30, 0x9b, 0x3d,
                0xc6, 0x85, 0xcc, 0xc2, 0xbd, 0xa1, 0xfb, 0x3c, 0xdd, 0xc0, 0xcf, 0x24, 0xa5, 0xc5,
                0x9d, 0x77, 0xe7, 0xea,
            ],
            Core::FullIncrement32 => [
                0x70, 0x47, 0x04, 0xc6, 0xd7, 0xf7, 0x57, 0xee, 0xe9, 0xf8, 0x8b, 0x2b, 0x32, 0x7b,
                0xc1, 0x17, 0x43, 0xcb, 0xd9, 0xa7, 0x5d, 0xec, 0xa2, 0xe9, 0x6d, 0x62, 0x8d, 0x53,
                0x13, 0xe6, 0x16, 0x30,
            ],
            Core::FullIncrement64 => [
                0x99, 0x9c, 0xad, 0xea, 0x9c, 0xf0, 0xbd, 0xda, 0xab, 0x12, 0xa3, 0x98, 0xcf, 0x52,
                0xd1, 0x8e, 0x08, 0xfb, 0x5c, 0x18, 0x5d, 0xca, 0x00, 0xf3, 0x5e, 0xde, 0x63, 0x27,
                0x7e, 0x9b, 0xac, 0x4e,
            ],
            Core::FullIncrement8 => [
                0x2d, 0x64, 0x5c, 0xb4, 0x77, 0x8d, 0xa1, 0x7a, 0x0d, 0xad, 0x32, 0x6f, 0xfe, 0xdc,
                0x2b, 0x04, 0xf6, 0x2d, 0x02, 0x7d, 0x7b, 0x40, 0x56, 0x0d, 0x16, 0xcd, 0x64, 0x9f,
                0xd5, 0xa4, 0xd7, 0x2a,
            ],
            Core::FullLeftShift16_1 => [
                0x32, 0xd0, 0x6f, 0x29, 0x46, 0x52, 0xb3, 0x64, 0x90, 0x58, 0xa4, 0x66, 0x67, 0x3f,
                0x04, 0xf0, 0x88, 0x80, 0xaa, 0xa1, 0x92, 0x6f, 0x90, 0x8a, 0x92, 0x56, 0xa9, 0x9e,
                0x1e, 0x2e, 0x0b, 0xdc,
            ],
            Core::FullLeftShift16_2 => [
                0xd1, 0xa5, 0xfb, 0x91, 0xaa, 0x70, 0x44, 0x5d, 0x29, 0xfd, 0xb1, 0x69, 0x40, 0x37,
                0xa9, 0x5d, 0x84, 0xe6, 0x57, 0x1b, 0x3e, 0x77, 0x73, 0xfc, 0x16, 0xf5, 0x6e, 0xb8,
                0x8b, 0x67, 0x64, 0x39,
            ],
            Core::FullLeftShift16_4 => [
                0x94, 0x52, 0xa9, 0xb6, 0x75, 0x49, 0x17, 0x8f, 0x93, 0xc7, 0xbb, 0x34, 0x36, 0xbe,
                0x06, 0x31, 0x4b, 0x4a, 0xda, 0xe5, 0xde, 0x0e, 0x31, 0x53, 0xcb, 0xd3, 0x2d, 0x55,
                0x9d, 0xf6, 0xf4, 0x28,
            ],
            Core::FullLeftShift16_8 => [
                0xcd, 0xb8, 0x8c, 0x96, 0x01, 0x73, 0x2a, 0x76, 0x66, 0x5a, 0xc9, 0x28, 0x21, 0x68,
                0x3b, 0xaf, 0xf1, 0x84, 0x4b, 0x67, 0x0b, 0xff, 0x66, 0x39, 0x17, 0xc8, 0x91, 0x3b,
                0xa5, 0x1c, 0x0f, 0x08,
            ],
            Core::FullLeftShift32_1 => [
                0xef, 0x47, 0x06, 0xca, 0x44, 0xa8, 0xc5, 0x5f, 0x46, 0x5f, 0xa9, 0xb3, 0x9c, 0x3c,
                0x69, 0xc8, 0x12, 0xef, 0xe3, 0xd9, 0xdf, 0xb6, 0xc9, 0xa5, 0xc0, 0x50, 0x92, 0xb5,
                0x7f, 0xc3, 0xc1, 0x53,
            ],
            Core::FullLeftShift32_16 => [
                0x13, 0x04, 0x05, 0xb9, 0x90, 0xdf, 0xa1, 0x81, 0xa7, 0x6b, 0xb2, 0x96, 0x5f, 0x6b,
                0x70, 0x6d, 0xe5, 0xb0, 0x46, 0x47, 0x43, 0x8a, 0x5c, 0xab, 0x6e, 0xc3, 0xdf, 0xb6,
                0xae, 0x21, 0xfa, 0x8a,
            ],
            Core::FullLeftShift32_2 => [
                0xcc, 0x4b, 0x94, 0x98, 0xee, 0x41, 0x23, 0x85, 0xc2, 0x3b, 0xff, 0x77, 0xdf, 0x13,
                0x7a, 0xd2, 0x96, 0xa5, 0xb8, 0x30, 0x31, 0xc1, 0x51, 0x8d, 0xbf, 0x4f, 0xdf, 0x6a,
                0xbb, 0x56, 0x7c, 0x2a,
            ],
            Core::FullLeftShift32_4 => [
                0xbf, 0xad, 0xc9, 0x53, 0x41, 0x00, 0x47, 0xd9, 0xef, 0x39, 0x8b, 0xd3, 0x3a, 0x02,
                0xaa, 0xd0, 0xd4, 0x8c, 0xbd, 0x7b, 0x6a, 0xe2, 0xde, 0x8d, 0xc1, 0xf7, 0x6b, 0x0c,
                0xe4, 0x4f, 0x80, 0xec,
            ],
            Core::FullLeftShift32_8 => [
                0xac, 0x93, 0x4c, 0x70, 0xaa, 0xb8, 0x10, 0xf0, 0xae, 0x52, 0x7b, 0xc0, 0x74, 0x9e,
                0x96, 0xce, 0x00, 0x68, 0xe2, 0x37, 0xce, 0xe2, 0xa8, 0x66, 0x48, 0x3d, 0xdf, 0xaa,
                0x21, 0x16, 0xd9, 0x31,
            ],
            Core::FullLeftShift64_1 => [
                0xbf, 0x32, 0xd9, 0xdb, 0xb2, 0x39, 0xe9, 0x88, 0x01, 0x49, 0x20, 0xcc, 0x4d, 0xf2,
                0xe4, 0x35, 0xc7, 0x12, 0x01, 0x7b, 0x6a, 0x94, 0x0e, 0x77, 0xc2, 0x03, 0x71, 0x3d,
                0xbf, 0xb0, 0xfe, 0x53,
            ],
            Core::FullLeftShift64_16 => [
                0x13, 0xde, 0x96, 0x78, 0x5b, 0xd9, 0x7f, 0x6f, 0x78, 0xec, 0x2a, 0x37, 0xf5, 0x52,
                0xd4, 0xd7, 0xda, 0xe5, 0x76, 0x8e, 0x39, 0xb4, 0xbe, 0xbd, 0xf3, 0xbc, 0x7c, 0xeb,
                0x72, 0x3c, 0x59, 0x47,
            ],
            Core::FullLeftShift64_2 => [
                0x19, 0xd3, 0xb1, 0x41, 0xcb, 0x15, 0x6c, 0xce, 0x2a, 0xc5, 0x93, 0x7a, 0x27, 0xa2,
                0x84, 0x36, 0xdc, 0x32, 0x5d, 0xd7, 0xce, 0x42, 0x47, 0x6e, 0xe2, 0xca, 0xb3, 0x5f,
                0xb4, 0x01, 0x10, 0xe0,
            ],
            Core::FullLeftShift64_32 => [
                0x6a, 0x20, 0x76, 0x15, 0xbc, 0x66, 0x9d, 0x3f, 0xc1, 0x74, 0x13, 0xa4, 0x2b, 0x13,
                0xa0, 0x4a, 0x40, 0x4b, 0xb5, 0xd4, 0x03, 0x6d, 0xe3, 0x7f, 0xe0, 0xa5, 0x6b, 0x33,
                0x6a, 0x5f, 0xf5, 0xa6,
            ],
            Core::FullLeftShift64_4 => [
                0x57, 0xf5, 0x96, 0xcf, 0xe6, 0x7a, 0xf5, 0x8a, 0xe6, 0xbc, 0x27, 0xa1, 0x2c, 0xaf,
                0xe5, 0x3a, 0x1d, 0x5e, 0x6a, 0xce, 0x4e, 0x54, 0x05, 0x1a, 0x3a, 0xe4, 0x1c, 0x1e,
                0xb3, 0x25, 0xf5, 0x3b,
            ],
            Core::FullLeftShift64_8 => [
                0x05, 0x85, 0x7a, 0x00, 0x9d, 0x20, 0x31, 0xbe, 0x6d, 0xf1, 0xd8, 0x7e, 0x26, 0xe1,
                0x11, 0x18, 0xab, 0xd0, 0x6b, 0x0f, 0xf0, 0xc8, 0x78, 0xc3, 0x36, 0xf9, 0xa0, 0x76,
                0x11, 0xda, 0xa8, 0xd4,
            ],
            Core::FullLeftShift8_1 => [
                0x4f, 0xdc, 0x52, 0x2a, 0x23, 0x9f, 0x25, 0x84, 0x8c, 0x28, 0xd4, 0x5b, 0x39, 0x24,
                0x63, 0x91, 0xbe, 0x6a, 0x28, 0x14, 0xb8, 0x93, 0xcf, 0x19, 0x79, 0xce, 0x31, 0xb6,
                0x07, 0xcf, 0x17, 0x4e,
            ],
            Core::FullLeftShift8_2 => [
                0xdc, 0xf6, 0x33, 0x3d, 0x5e, 0x28, 0xbf, 0xaa, 0xfe, 0xe6, 0xf2, 0x0a, 0x85, 0x5c,
                0xef, 0xb0, 0x05, 0xf0, 0xef, 0x32, 0x8b, 0xdc, 0xce, 0xa6, 0x4a, 0x8f, 0x24, 0x46,
                0xf9, 0x66, 0xa6, 0xf2,
            ],
            Core::FullLeftShift8_4 => [
                0xf7, 0xed, 0x36, 0xeb, 0xd7, 0xb4, 0xee, 0x3e, 0x77, 0x82, 0xd0, 0x74, 0x4c, 0x47,
                0xba, 0x1e, 0xf8, 0xfa, 0x56, 0xdb, 0x70, 0x60, 0xe2, 0x26, 0xe4, 0xe9, 0x26, 0x29,
                0x92, 0x6f, 0x43, 0x6c,
            ],
            Core::FullMultiply16 => [
                0xc0, 0xd7, 0x36, 0xca, 0x68, 0x4f, 0x82, 0x72, 0xdc, 0xfb, 0xe3, 0x8d, 0xcc, 0x54,
                0x29, 0xb3, 0x03, 0xa6, 0x82, 0x32, 0xe5, 0xfc, 0xd2, 0x85, 0x14, 0xe2, 0x2c, 0x0b,
                0xe1, 0xf4, 0x70, 0xf6,
            ],
            Core::FullMultiply32 => [
                0xe7, 0x20, 0x58, 0x97, 0x97, 0xf6, 0xca, 0x4a, 0xd7, 0x47, 0x71, 0xf8, 0xf6, 0xc7,
                0xb6, 0xaa, 0xd7, 0x8b, 0xce, 0x91, 0x9f, 0xd7, 0x6d, 0x69, 0xff, 0xf5, 0x99, 0xa4,
                0x02, 0xe3, 0x52, 0x40,
            ],
            Core::FullMultiply64 => [
                0xd9, 0x66, 0x91, 0x86, 0x21, 0x44, 0x87, 0xfb, 0xaa, 0xbc, 0x08, 0x52, 0x15, 0xeb,
                0x01, 0xc6, 0x2b, 0x3a, 0x28, 0xde, 0x41, 0x19, 0x19, 0xbc, 0x4f, 0x37, 0xfe, 0x43,
                0xc2, 0xd0, 0x05, 0x21,
            ],
            Core::FullMultiply8 => [
                0x72, 0x8a, 0xc8, 0xaa, 0xdb, 0x37, 0xa0, 0xf2, 0x23, 0xee, 0xf9, 0x8c, 0xf2, 0x78,
                0x48, 0x3d, 0xf1, 0x94, 0x86, 0x9b, 0x2e, 0x37, 0x73, 0xf1, 0x27, 0x32, 0x9e, 0xea,
                0xb5, 0x4c, 0x62, 0xca,
            ],
            Core::FullRightShift16_1 => [
                0x28, 0x43, 0x7c, 0x34, 0x7a, 0x0f, 0x31, 0x6e, 0x99, 0xf3, 0x44, 0x4f, 0x32, 0xd9,
                0x49, 0xc6, 0xd1, 0xa5, 0x70, 0xd6, 0xe1, 0xf9, 0xe4, 0x7f, 0x7e, 0xa2, 0xc4, 0x98,
                0x7f, 0xec, 0xb0, 0xb4,
            ],
            Core::FullRightShift16_2 => [
                0x7b, 0x2d, 0x0d, 0xd3, 0xec, 0xf7, 0x9e, 0xcf, 0xc0, 0xbf, 0x92, 0x3a, 0x92, 0xa4,
                0x3a, 0xc6, 0x51, 0x4b, 0xbf, 0xc2, 0x9f, 0x67, 0x73, 0xd7, 0xb3, 0x63, 0xc3, 0xac,
                0xf6, 0xf6, 0x72, 0xc4,
            ],
            Core::FullRightShift16_4 => [
                0xec, 0x59, 0xde, 0x5c, 0x9c, 0x88, 0x77, 0x75, 0xe4, 0xbc, 0x55, 0x6b, 0x9c, 0xf1,
                0xa7, 0x72, 0x3b, 0xbe, 0xb6, 0x14, 0x75, 0xd5, 0xa5, 0xa4, 0xed, 0xf1, 0x0b, 0x61,
                0x2b, 0xf9, 0xa6, 0x80,
            ],
            Core::FullRightShift16_8 => [
                0x26, 0x4f, 0x2a, 0x31, 0xf0, 0x7b, 0xcc, 0x54, 0x74, 0x27, 0xfa, 0xff, 0x0d, 0x09,
                0xc2, 0x66, 0x5b, 0x30, 0xed, 0x30, 0xf9, 0xf8, 0xa6, 0xc5, 0x65, 0x01, 0x84, 0x00,
                0x14, 0x4a, 0x4d, 0x6a,
            ],
            Core::FullRightShift32_1 => [
                0x19, 0xd7, 0x8c, 0xce, 0x30, 0xf7, 0x1e, 0xac, 0xf5, 0x9a, 0x8c, 0xf0, 0xf6, 0x02,
                0x8b, 0x28, 0x4d, 0x4a, 0xa9, 0x64, 0xc0, 0xc1, 0x6a, 0xd4, 0xaf, 0xc7, 0xea, 0x47,
                0x51, 0x65, 0xcb, 0xa0,
            ],
            Core::FullRightShift32_16 => [
                0x88, 0x76, 0x74, 0x15, 0x79, 0x92, 0x4a, 0xf6, 0xd4, 0x6f, 0xbd, 0x79, 0x53, 0x4a,
                0x09, 0x07, 0x7b, 0xf0, 0xec, 0xbc, 0x97, 0x10, 0x08, 0xc7, 0xde, 0xc6, 0x5d, 0x4c,
                0x96, 0xe5, 0xd8, 0x31,
            ],
            Core::FullRightShift32_2 => [
                0x5d, 0x3e, 0x11, 0x9d, 0x40, 0xe6, 0x24, 0xc5, 0xc5, 0x6a, 0x7b, 0xbd, 0xf8, 0xbb,
                0x86, 0xfb, 0x68, 0x89, 0xdc, 0x01, 0x72, 0x48, 0x7c, 0x35, 0x8a, 0x53, 0x1a, 0xf2,
                0xfc, 0xc5, 0x1d, 0xc7,
            ],
            Core::FullRightShift32_4 => [
                0x58, 0x3a, 0xc6, 0x98, 0x39, 0xd2, 0x35, 0xd9, 0xc0, 0x61, 0x39, 0xf5, 0xb7, 0x98,
                0xd2, 0xd5, 0xa3, 0x49, 0xed, 0xc2, 0x0b, 0xbd, 0x62, 0xba, 0xd1, 0xd8, 0x12, 0x85,
                0xd6, 0xad, 0x22, 0xe3,
            ],
            Core::FullRightShift32_8 => [
                0x0d, 0x9c, 0xc0, 0x9b, 0xbc, 0x80, 0xe4, 0xcd, 0x76, 0x4f, 0x13, 0xbb, 0x2f, 0x7f,
                0xb9, 0xc5, 0x92, 0x12, 0x0f, 0x92, 0x18, 0xd6, 0x0c, 0xac, 0x8e, 0xea, 0xc5, 0x99,
                0x25, 0xdb, 0xf7, 0x5d,
            ],
            Core::FullRightShift64_1 => [
                0x2e, 0xbc, 0x1c, 0x95, 0x5d, 0x26, 0x56, 0x48, 0xa8, 0xb9, 0x1b, 0x6f, 0x5e, 0xce,
                0x5f, 0x23, 0xfa, 0x9c, 0x2c, 0x6c, 0x88, 0xde, 0x7c, 0xbe, 0x3f, 0x7c, 0xc8, 0x38,
                0x5b, 0x21, 0x91, 0x17,
            ],
            Core::FullRightShift64_16 => [
                0x46, 0x58, 0xdd, 0x0c, 0x37, 0xf3, 0xec, 0x6b, 0xe5, 0x22, 0x41, 0x73, 0xd2, 0xff,
                0x9b, 0x7c, 0x91, 0x01, 0x74, 0x2f, 0x69, 0x0f, 0x15, 0xa7, 0x2e, 0x66, 0x6c, 0x87,
                0x0e, 0x1d, 0x90, 0x55,
            ],
            Core::FullRightShift64_2 => [
                0x88, 0x9f, 0x8b, 0x7b, 0xc1, 0x1e, 0x9b, 0x74, 0xea, 0x93, 0x37, 0x4f, 0xfe, 0xd1,
                0x14, 0x64, 0xe7, 0x83, 0x7d, 0xee, 0x18, 0x19, 0x81, 0xcb, 0xe4, 0x78, 0x6f, 0x57,
                0xa2, 0x65, 0xc3, 0x4f,
            ],
            Core::FullRightShift64_32 => [
                0x06, 0x24, 0x67, 0xcd, 0x19, 0x4b, 0xb4, 0xfe, 0x1d, 0xd5, 0xed, 0xe5, 0x7b, 0xc6,
                0x51, 0x80, 0x94, 0x8e, 0xba, 0xc9, 0x87, 0x8b, 0x2c, 0x04, 0x2b, 0xe3, 0x56, 0xf2,
                0xf4, 0x22, 0xc2, 0xec,
            ],
            Core::FullRightShift64_4 => [
                0x99, 0x45, 0x72, 0x13, 0x88, 0x48, 0x37, 0x43, 0x7e, 0x8a, 0xb6, 0x36, 0xd7, 0xf3,
                0x51, 0x3a, 0x24, 0xd9, 0x4e, 0x74, 0xe5, 0x99, 0x0c, 0x2c, 0x24, 0xe8, 0xdf, 0xe1,
                0x4c, 0xf9, 0x98, 0x6f,
            ],
            Core::FullRightShift64_8 => [
                0x00, 0x8d, 0x5b, 0x6a, 0x26, 0xa0, 0xe1, 0x3f, 0x17, 0xa9, 0x24, 0xa9, 0xe0, 0x0f,
                0x5b, 0x07, 0xc8, 0x12, 0x3b, 0x59, 0x7e, 0x63, 0xcd, 0xf9, 0x48, 0x1d, 0xd1, 0x6f,
                0x43, 0x32, 0xe6, 0xdf,
            ],
            Core::FullRightShift8_1 => [
                0x59, 0x1a, 0xde, 0xe6, 0xd4, 0xa3, 0xdd, 0xf3, 0x94, 0x7b, 0x48, 0x61, 0x5b, 0x7f,
                0x6e, 0xcd, 0xf1, 0x16, 0x57, 0x36, 0x4c, 0x3a, 0xee, 0xcf, 0xad, 0x43, 0xf2, 0x8e,
                0x11, 0xa7, 0x5e, 0x56,
            ],
            Core::FullRightShift8_2 => [
                0xbf, 0x18, 0xaf, 0x96, 0x4c, 0x49, 0xb6, 0x2e, 0xe7, 0x99, 0x21, 0xfd, 0xe0, 0xeb,
                0x32, 0x52, 0xf4, 0x16, 0x38, 0x67, 0x77, 0x17, 0x3c, 0xa6, 0xf6, 0xea, 0xd6, 0x89,
                0xb1, 0x90, 0xfb, 0x2f,
            ],
            Core::FullRightShift8_4 => [
                0x8d, 0x05, 0xa2, 0x81, 0xbe, 0x22, 0x0a, 0x8c, 0xa3, 0xeb, 0xd9, 0xa8, 0x62, 0x67,
                0x08, 0xf9, 0x1f, 0x28, 0x6d, 0xa4, 0x22, 0x22, 0xc7, 0xa5, 0x51, 0x8e, 0x17, 0x9c,
                0x09, 0xe5, 0xa0, 0x2f,
            ],
            Core::FullSubtract16 => [
                0x79, 0x55, 0x12, 0x6e, 0x2c, 0x3d, 0x86, 0x2c, 0x1e, 0x44, 0x9e, 0xf6, 0xe4, 0x35,
                0xdf, 0x3e, 0xd5, 0xa6, 0x4c, 0x20, 0x11, 0x43, 0x57, 0x8e, 0xcf, 0xd3, 0xf5, 0x1b,
                0xaf, 0x2b, 0xe2, 0x35,
            ],
            Core::FullSubtract32 => [
                0x26, 0x18, 0xa4, 0x11, 0x47, 0xc7, 0x9f, 0x69, 0x09, 0xa6, 0x51, 0xb4, 0x92, 0xb5,
                0x4e, 0xd3, 0xc4, 0x05, 0x7a, 0x2e, 0xc0, 0x39, 0xf9, 0x1c, 0x20, 0x10, 0x54, 0x54,
                0xac, 0x61, 0x03, 0x79,
            ],
            Core::FullSubtract64 => [
                0x5e, 0x05, 0xb1, 0x0a, 0xcf, 0x51, 0xc2, 0xf2, 0xb7, 0xf8, 0x72, 0x6d, 0x3f, 0x41,
                0x4a, 0xea, 0x2f, 0xb6, 0x31, 0x1f, 0x6d, 0xd6, 0x72, 0x45, 0xbd, 0x4a, 0x35, 0xb1,
                0x69, 0x53, 0xaf, 0x13,
            ],
            Core::FullSubtract8 => [
                0x68, 0xd2, 0x79, 0xe1, 0x6f, 0xe8, 0x4b, 0xa5, 0xe7, 0x44, 0xcd, 0xba, 0x86, 0xef,
                0xf3, 0xa3, 0xe0, 0xda, 0x05, 0x54, 0x7e, 0xfa, 0x98, 0x17, 0xdf, 0xb9, 0x9f, 0xde,
                0x70, 0x21, 0x46, 0xac,
            ],
            Core::GeIsOnCurve => [
                0xf5, 0x13, 0xc5, 0x94, 0x61, 0x5a, 0x9f, 0x2f, 0xcf, 0x35, 0x60, 0x55, 0xe2, 0xf7,
                0x74, 0x88, 0x16, 0xa3, 0xd4, 0xba, 0x54, 0x15, 0xbf, 0x12, 0xca, 0x17, 0xa5, 0xca,
                0xe9, 0xac, 0x72, 0x5a,
            ],
            Core::GeNegate => [
                0xc4, 0x3f, 0x47, 0xb1, 0x1c, 0xdb, 0xc9, 0xca, 0x85, 0xb6, 0x88, 0x05, 0xa4, 0xa7,
                0x0b, 0x82, 0xbf, 0x93, 0x8c, 0x1c, 0x0b, 0x2c, 0x1b, 0x27, 0x71, 0xa8, 0x6b, 0xfd,
                0x99, 0x37, 0x34, 0xa7,
            ],
            Core::GejAdd => [
                0xf5, 0x64, 0x35, 0xe4, 0xad, 0x5a, 0xa9, 0x56, 0x27, 0xaa, 0xa2, 0xf5, 0xf2, 0xba,
                0xd4, 0xe4, 0x5e, 0xf6, 0x2e, 0x4a, 0x8a, 0x83, 0xc0, 0x8e, 0x31, 0xe8, 0x10, 0x09,
                0x5c, 0xe3, 0x43, 0xb2,
            ],
            Core::GejDouble => [
                0xa0, 0x8e, 0xa3, 0x9e, 0xeb, 0x49, 0x0b, 0x60, 0x49, 0xc7, 0x2a, 0xb9, 0x21, 0x5d,
                0x61, 0x05, 0x8e, 0xd1, 0xee, 0x33, 0x77, 0x94, 0x21, 0x84, 0x0d, 0x3e, 0x3d, 0x43,
                0xe6, 0x37, 0x19, 0xfc,
            ],
            Core::GejEquiv => [
                0x9c, 0xa3, 0x67, 0x6b, 0xcc, 0xc5, 0x44, 0xed, 0x7a, 0xa3, 0xf7, 0x51, 0x75, 0xe9,
                0xf2, 0x3e, 0x03, 0x0c, 0xd6, 0x4f, 0xb6, 0xa5, 0x89, 0x8d, 0x90, 0xd2, 0x3e, 0x0d,
                0xc2, 0xae, 0xed, 0xd1,
            ],
            Core::GejGeAdd => [
                0xc8, 0xf3, 0xd8, 0x6d, 0x13, 0x1e, 0x35, 0xea, 0x67, 0xb2, 0x50, 0xa0, 0x8e, 0xdb,
                0xf5, 0x8c, 0x6f, 0x05, 0x43, 0x9c, 0x41, 0xfc, 0xbd, 0x32, 0x90, 0x8d, 0xc8, 0x6a,
                0xe8, 0xce, 0x5d, 0xf7,
            ],
            Core::GejGeAddEx => [
                0x8b, 0xcf, 0x3c, 0xcb, 0x4b, 0xf6, 0x91, 0x97, 0x49, 0x89, 0xfc, 0x4e, 0x86, 0x99,
                0x6c, 0x66, 0xba, 0xf1, 0xb0, 0xab, 0xa8, 0xc9, 0x64, 0x65, 0x86, 0x2a, 0x81, 0x84,
                0xa1, 0x29, 0xa0, 0xae,
            ],
            Core::GejGeEquiv => [
                0x67, 0xb2, 0x8f, 0x43, 0xbf, 0x1e, 0xb5, 0xc0, 0x4f, 0x24, 0x56, 0x59, 0x7e, 0xab,
                0x77, 0x6e, 0x48, 0xd4, 0x98, 0xfd, 0x8a, 0xe5, 0x28, 0x4d, 0xb3, 0x52, 0xa7, 0x3d,
                0x1b, 0xdd, 0x28, 0x5b,
            ],
            Core::GejInfinity => [
                0xc9, 0xcc, 0xee, 0x79, 0xe6, 0xfd, 0x32, 0x71, 0xfb, 0xb3, 0x84, 0x56, 0x88, 0x74,
                0x24, 0x95, 0x3b, 0x25, 0x4b, 0x9e, 0x90, 0xb7, 0xb7, 0x79, 0xfd, 0x57, 0x89, 0x68,
                0x45, 0x41, 0x0c, 0x66,
            ],
            Core::GejIsInfinity => [
                0x99, 0xd3, 0x98, 0xd5, 0xf2, 0x38, 0xc0, 0x55, 0xee, 0x32, 0xa9, 0x75, 0xdd, 0x44,
                0x5f, 0xbc, 0x5e, 0xce, 0x59, 0x9f, 0x74, 0xa3, 0x19, 0x0a, 0x63, 0x6d, 0xdd, 0x5d,
                0xd1, 0x11, 0xf5, 0x87,
            ],
            Core::GejIsOnCurve => [
                0x6c, 0x73, 0xc2, 0x19, 0xa7, 0xe9, 0x22, 0x8f, 0x6f, 0x48, 0x2c, 0x11, 0x17, 0x43,
                0x31, 0x1c, 0x49, 0x8b, 0xac, 0xd0, 0xc1, 0xfc, 0x00, 0xac, 0x3c, 0xe4, 0x1f, 0x49,
                0x73, 0x5b, 0xa2, 0xff,
            ],
            Core::GejNegate => [
                0xb1, 0xec, 0x2b, 0x7b, 0x3a, 0x34, 0x29, 0x4d, 0xf8, 0x99, 0x12, 0x91, 0x1d, 0x35,
                0x1f, 0xe0, 0xef, 0x23, 0x3f, 0xdc, 0x60, 0x11, 0x58, 0x76, 0xbd, 0xcf, 0x61, 0x1d,
                0x59, 0x0b, 0x08, 0xd6,
            ],
            Core::GejNormalize => [
                0x3c, 0xc6, 0xcf, 0x4c, 0xed, 0x32, 0x3e, 0xa6, 0xdc, 0xf4, 0x18, 0x44, 0xdb, 0x49,
                0x53, 0xaa, 0x3d, 0xb2, 0x9d, 0x08, 0xff, 0xfc, 0x18, 0x6f, 0x53, 0x8a, 0x38, 0x7a,
                0x6c, 0xbf, 0x72, 0x27,
            ],
            Core::GejRescale => [
                0xd1, 0xa8, 0x87, 0x77, 0xd2, 0x20, 0x1b, 0xf8, 0x4c, 0x94, 0xe5, 0x85, 0xc6, 0x12,
                0xcd, 0x18, 0xe1, 0xaa, 0x4a, 0x18, 0x62, 0x4c, 0xca, 0x5d, 0xe5, 0xfb, 0xe4, 0x41,
                0xc3, 0x68, 0xb1, 0x90,
            ],
            Core::GejXEquiv => [
                0xb9, 0xfe, 0x21, 0xa5, 0xed, 0x60, 0x69, 0x4d, 0x04, 0x2b, 0xa2, 0x1c, 0xad, 0xfc,
                0xd9, 0x44, 0x3c, 0x6f, 0xf7, 0x28, 0xf6, 0x8e, 0x51, 0xd7, 0xd5, 0x5e, 0x05, 0xa9,
                0xbb, 0x7a, 0x1d, 0x1b,
            ],
            Core::GejYIsOdd => [
                0xaf, 0x25, 0x25, 0x24, 0xfb, 0x38, 0xc4, 0x4f, 0xa7, 0xca, 0x6f, 0x0d, 0x69, 0x3e,
                0xe3, 0x43, 0x06, 0xa3, 0x1e, 0xd9, 0x21, 0xf9, 0x31, 0x1c, 0x91, 0x72, 0x63, 0x89,
                0xb1, 0xf6, 0x83, 0x76,
            ],
            Core::Generate => [
                0xe5, 0x87, 0x2b, 0xc9, 0xf0, 0xc1, 0xf8, 0x55, 0x4d, 0xe0, 0x17, 0x84, 0x38, 0xa9,
                0xf6, 0x49, 0x94, 0x76, 0xca, 0xc8, 0xc5, 0x80, 0xf3, 0xd0, 0xb6, 0x0d, 0x85, 0x48,
                0x85, 0x18, 0x4c, 0xb6,
            ],
            Core::HashToCurve => [
                0xa1, 0x7b, 0x59, 0xe3, 0xf0, 0x14, 0xb1, 0x58, 0xad, 0x47, 0xb5, 0xbb, 0x42, 0x90,
                0xf1, 0x7b, 0x5b, 0xb7, 0x0b, 0xc6, 0x47, 0x1d, 0x3b, 0xe6, 0xd3, 0x86, 0x98, 0xe5,
                0x23, 0x4a, 0xb6, 0xf2,
            ],
            Core::High1 => [
                0x72, 0x18, 0x87, 0x56, 0xe0, 0x64, 0xfe, 0xf6, 0x07, 0x3d, 0x82, 0x53, 0x0e, 0x95,
                0xee, 0x3a, 0x04, 0x88, 0x9c, 0x6b, 0xf9, 0xe4, 0x4e, 0x05, 0x96, 0x22, 0x40, 0x4e,
                0xa1, 0x7c, 0x69, 0x4a,
            ],
            Core::High16 => [
                0x0e, 0x41, 0x4a, 0x0c, 0xe6, 0x81, 0x47, 0x85, 0xe0, 0x19, 0xf1, 0x4d, 0x47, 0x71,
                0x40, 0x76, 0xe8, 0x7b, 0x12, 0x3e, 0x9a, 0x21, 0x22, 0xb1, 0x64, 0x3c, 0xc0, 0x66,
                0x8c, 0x6e, 0xce, 0x09,
            ],
            Core::High32 => [
                0x08, 0x03, 0x29, 0x7a, 0xc5, 0xfb, 0xea, 0x98, 0x5e, 0xf6, 0x19, 0xa0, 0x38, 0xbe,
                0xd0, 0x68, 0xf8, 0x60, 0xb2, 0x30, 0x95, 0x26, 0x0e, 0x92, 0x6a, 0xf7, 0xd8, 0x9d,
                0xd0, 0xcc, 0x93, 0x21,
            ],
            Core::High64 => [
                0x47, 0x1c, 0xe7, 0xd2, 0xd1, 0xbc, 0xcb, 0xcc, 0xb1, 0x8a, 0xbd, 0xe3, 0xe5, 0x34,
                0x79, 0x53, 0xbc, 0x7e, 0x63, 0xc8, 0xb4, 0x3f, 0x1a, 0x7c, 0xee, 0x95, 0x53, 0x38,
                0x2c, 0x6b, 0x8d, 0x21,
            ],
            Core::High8 => [
                0x60, 0xc6, 0x69, 0xbb, 0x56, 0xac, 0xa8, 0x8d, 0xa1, 0x3e, 0x09, 0x30, 0x11, 0xa7,
                0xbb, 0xed, 0x55, 0x07, 0x9e, 0xa9, 0xbc, 0xdb, 0x84, 0x38, 0xf3, 0xdc, 0x53, 0xe1,
                0x2c, 0x20, 0x40, 0x9d,
            ],
            Core::Increment16 => [
                0x49, 0xc6, 0x29, 0xdb, 0x72, 0x65, 0xd6, 0xa8, 0xd2, 0x6e, 0x83, 0xf6, 0x5d, 0x62,
                0xe7, 0x38, 0x54, 0xb9, 0xdd, 0xb2, 0x1d, 0x2a, 0xa9, 0xc1, 0xbd, 0x94, 0x6e, 0x4d,
                0xa0, 0xb2, 0x79, 0x32,
            ],
            Core::Increment32 => [
                0x95, 0x7e, 0xde, 0xc0, 0xdc, 0x98, 0xcf, 0x13, 0xfd, 0xa5, 0xb2, 0x19, 0x31, 0xf0,
                0x87, 0xe3, 0x64, 0x8e, 0xf2, 0xd7, 0xc6, 0xbe, 0xc8, 0xc5, 0x16, 0xe3, 0x24, 0x4f,
                0x1b, 0xbc, 0x35, 0x9d,
            ],
            Core::Increment64 => [
                0x48, 0xb1, 0x92, 0x7c, 0x68, 0x40, 0x4a, 0xb0, 0x1b, 0xe8, 0x9f, 0xda, 0xcc, 0xb1,
                0x64, 0xeb, 0xee, 0x4d, 0x69, 0x44, 0xca, 0x8e, 0xf0, 0x39, 0x99, 0xb2, 0x76, 0x29,
                0x75, 0xd2, 0x2f, 0xf8,
            ],
            Core::Increment8 => [
                0xe0, 0x79, 0x6c, 0x02, 0x7b, 0xf4, 0xb0, 0x79, 0x47, 0xff, 0x99, 0x1b, 0x27, 0x82,
                0x07, 0xd3, 0x26, 0xba, 0xe0, 0x74, 0xa4, 0x58, 0x3b, 0x56, 0x42, 0xdf, 0x04, 0x43,
                0x29, 0x7c, 0x90, 0x1a,
            ],
            Core::IsOne16 => [
                0xa6, 0x86, 0xb2, 0x5c, 0x80, 0xa2, 0x1f, 0x78, 0xfb, 0x30, 0x74, 0xc0, 0xf9, 0x76,
                0xdc, 0x0b, 0x8f, 0xe1, 0x06, 0xc1, 0x3d, 0xca, 0x01, 0xcb, 0xd6, 0xb4, 0xae, 0xf6,
                0xd0, 0x4f, 0x26, 0x43,
            ],
            Core::IsOne32 => [
                0xfc, 0xfa, 0xdf, 0x11, 0xd7, 0xb1, 0xbb, 0x23, 0x9f, 0x29, 0xd1, 0xca, 0x9f, 0xdc,
                0xf3, 0x5b, 0x72, 0x8a, 0x1a, 0x62, 0x23, 0x76, 0x07, 0x1d, 0x3c, 0x4d, 0x88, 0xa1,
                0x3b, 0x12, 0x26, 0x84,
            ],
            Core::IsOne64 => [
                0xcf, 0x69, 0x10, 0x72, 0x37, 0x54, 0xba, 0x79, 0x43, 0x95, 0xda, 0x5d, 0x58, 0xd6,
                0x1c, 0x19, 0x00, 0x3e, 0x45, 0xee, 0xcf, 0xbf, 0xc9, 0x0d, 0x49, 0xfe, 0x65, 0x7f,
                0x60, 0x7c, 0x45, 0x1c,
            ],
            Core::IsOne8 => [
                0x56, 0xa2, 0x2d, 0x0e, 0xcb, 0x20, 0x86, 0xd4, 0x3d, 0x52, 0xfd, 0x1d, 0x46, 0x69,
                0x8a, 0xd4, 0x42, 0x6f, 0x92, 0x35, 0x6a, 0x14, 0xce, 0x06, 0x50, 0x94, 0x1e, 0x52,
                0x07, 0x2c, 0x96, 0xa8,
            ],
            Core::IsZero16 => [
                0xf4, 0x9a, 0xbd, 0xbd, 0x6a, 0x3f, 0x17, 0xc8, 0x7d, 0xba, 0xbd, 0x40, 0xe6, 0x61,
                0x1c, 0x47, 0x7d, 0x43, 0x17, 0x71, 0x90, 0x56, 0x45, 0xef, 0xd3, 0xb2, 0x2d, 0x37,
                0x91, 0xab, 0xe6, 0xbd,
            ],
            Core::IsZero32 => [
                0x93, 0x5d, 0x13, 0xda, 0x64, 0x19, 0x0a, 0x36, 0x04, 0x73, 0x5a, 0xf8, 0x40, 0x21,
                0xbc, 0x12, 0x50, 0x34, 0x97, 0xee, 0x4b, 0x0f, 0xf4, 0xb7, 0x38, 0xfc, 0x84, 0x7e,
                0x9d, 0x34, 0x19, 0x21,
            ],
            Core::IsZero64 => [
                0xe9, 0xb5, 0x74, 0x75, 0x13, 0x30, 0xf9, 0xfe, 0xae, 0xa5, 0x2a, 0xe4, 0xcd, 0xd3,
                0xa2, 0xe4, 0x6a, 0x99, 0x9c, 0x4f, 0x41, 0xc8, 0xb5, 0x24, 0x00, 0x2c, 0x68, 0x05,
                0x66, 0x58, 0x46, 0x21,
            ],
            Core::IsZero8 => [
                0xfd, 0x7f, 0xc5, 0x05, 0xc2, 0x24, 0xe8, 0xbc, 0x02, 0x08, 0x7f, 0x9e, 0xeb, 0xc4,
                0x85, 0xe9, 0x97, 0xca, 0x15, 0x37, 0x48, 0xc7, 0x81, 0x12, 0x2e, 0x1b, 0x9a, 0xc0,
                0xfd, 0x94, 0xb9, 0x32,
            ],
            Core::Le16 => [
                0x98, 0xaf, 0x70, 0xd8, 0x94, 0xf3, 0xd4, 0x39, 0xbe, 0x06, 0x01, 0xe2, 0x4a, 0x33,
                0xf0, 0x65, 0x0d, 0x23, 0x65, 0xbc, 0xf0, 0x23, 0x02, 0x88, 0x62, 0xeb, 0xfd, 0xa4,
                0xd5, 0x0a, 0x2d, 0xca,
            ],
            Core::Le32 => [
                0x04, 0xa8, 0x35, 0x1a, 0x4c, 0x97, 0xd3, 0xb5, 0xde, 0x4f, 0x91, 0xb2, 0xe9, 0x73,
                0x9d, 0x9b, 0x3c, 0xc6, 0xad, 0x49, 0xca, 0x07, 0x10, 0x9c, 0x2c, 0xc4, 0x91, 0x8f,
                0xaf, 0xac, 0x66, 0xb5,
            ],
            Core::Le64 => [
                0x19, 0x4c, 0x01, 0xd8, 0xaa, 0x68, 0x95, 0x17, 0x11, 0xe6, 0x15, 0x14, 0x0e, 0x2b,
                0x5a, 0xce, 0x42, 0x6d, 0xac, 0xd1, 0xfd, 0x6d, 0x86, 0xc7, 0xa7, 0x2f, 0x3a, 0xaa,
                0x08, 0x9c, 0x24, 0x67,
            ],
            Core::Le8 => [
                0xb1, 0x33, 0xad, 0x2d, 0x1a, 0x61, 0x78, 0xb1, 0x98, 0x94, 0xc9, 0xdc, 0xe5, 0x92,
                0x4c, 0x19, 0x9a, 0x04, 0x5f, 0xab, 0x4e, 0x38, 0x8f, 0x16, 0x9a, 0xe1, 0xd8, 0x69,
                0x81, 0x14, 0x9a, 0x8b,
            ],
            Core::LeftExtend16_32 => [
                0x9d, 0xbb, 0x9d, 0x79, 0x97, 0xc1, 0xb1, 0xfa, 0x59, 0x13, 0x1e, 0x79, 0xaa, 0xd1,
                0x6b, 0x9f, 0xca, 0x81, 0xc4, 0x68, 0x32, 0x2d, 0x56, 0xcc, 0x0e, 0x71, 0xfa, 0x05,
                0xdd, 0x80, 0xc9, 0x6c,
            ],
            Core::LeftExtend16_64 => [
                0x5d, 0x33, 0x29, 0xd2, 0xc2, 0x9b, 0xe1, 0x04, 0xde, 0x74, 0xdd, 0xaf, 0xc9, 0xb2,
                0x2f, 0xf6, 0x5d, 0x26, 0x8b, 0xa2, 0xcc, 0xdd, 0xa6, 0x77, 0xe8, 0xfb, 0xfc, 0x65,
                0x01, 0x30, 0xa3, 0x71,
            ],
            Core::LeftExtend1_16 => [
                0x74, 0xee, 0x5a, 0x71, 0x88, 0x9e, 0x97, 0xc0, 0x3d, 0x9b, 0x90, 0x4c, 0x67, 0x9f,
                0xf7, 0xf2, 0xe8, 0xc7, 0x41, 0x0e, 0x6d, 0xc7, 0x48, 0xe0, 0xb5, 0x48, 0xeb, 0xfd,
                0x96, 0x03, 0xd1, 0xad,
            ],
            Core::LeftExtend1_32 => [
                0x97, 0xac, 0x64, 0x03, 0xdf, 0x61, 0xc7, 0x84, 0x13, 0x96, 0xc9, 0x3e, 0x14, 0x1f,
                0xaf, 0xc2, 0xa0, 0x96, 0x6e, 0x3e, 0x49, 0x74, 0x87, 0x7b, 0x9e, 0x94, 0x2b, 0xe5,
                0x9c, 0x78, 0xc8, 0x00,
            ],
            Core::LeftExtend1_64 => [
                0x54, 0x8a, 0xeb, 0x2b, 0x27, 0xb9, 0xef, 0x2a, 0x0b, 0xec, 0x50, 0xb6, 0x38, 0x52,
                0xfe, 0x24, 0x54, 0x9e, 0x60, 0xa7, 0x43, 0xb9, 0x78, 0x9e, 0xc9, 0x8a, 0xd4, 0xf8,
                0x73, 0x77, 0x13, 0xed,
            ],
            Core::LeftExtend1_8 => [
                0x74, 0x88, 0xda, 0x61, 0x1c, 0xfc, 0xdf, 0xa4, 0xff, 0x3d, 0x63, 0xf8, 0xc3, 0x6d,
                0x5a, 0x62, 0x9c, 0x58, 0x7e, 0x92, 0x20, 0xbd, 0x98, 0x9b, 0xda, 0x72, 0x06, 0x78,
                0xa9, 0xc9, 0x7b, 0xf7,
            ],
            Core::LeftExtend32_64 => [
                0x34, 0x33, 0x50, 0x9a, 0x5d, 0xe5, 0x69, 0x5d, 0x92, 0xca, 0xb9, 0x25, 0x51, 0x43,
                0xd6, 0x06, 0x67, 0x05, 0x68, 0xc1, 0x47, 0xf4, 0x74, 0xd1, 0x35, 0x93, 0xf5, 0x39,
                0x5b, 0x9a, 0xd2, 0xc1,
            ],
            Core::LeftExtend8_16 => [
                0x42, 0x0d, 0x0a, 0x56, 0x99, 0xb8, 0x87, 0x5a, 0xcc, 0x2d, 0x18, 0xfd, 0xa9, 0x0c,
                0x8a, 0x72, 0x74, 0x6d, 0x3e, 0x5e, 0x0c, 0xd8, 0xc1, 0x81, 0x89, 0x7a, 0x81, 0x4c,
                0x4c, 0x43, 0x39, 0x69,
            ],
            Core::LeftExtend8_32 => [
                0xc3, 0x0e, 0x47, 0x5d, 0x46, 0x50, 0xab, 0xe4, 0xae, 0x15, 0xc3, 0x27, 0x4a, 0xe8,
                0x83, 0x8d, 0xd5, 0x15, 0x88, 0xd1, 0x5b, 0xd0, 0xc6, 0xdc, 0xac, 0xf9, 0x82, 0xaa,
                0x46, 0x7b, 0xea, 0x62,
            ],
            Core::LeftExtend8_64 => [
                0xa6, 0x3a, 0x6e, 0x66, 0x72, 0xe1, 0xb6, 0xc7, 0x56, 0x54, 0x9a, 0x3e, 0xee, 0x01,
                0x94, 0x8e, 0x71, 0x19, 0x94, 0xfe, 0x18, 0xd7, 0xca, 0xed, 0xf1, 0x09, 0x15, 0xc1,
                0x60, 0xd7, 0xa8, 0x0d,
            ],
            Core::LeftPadHigh16_32 => [
                0x72, 0xec, 0xa5, 0xc1, 0x13, 0x74, 0x3e, 0xf0, 0xb6, 0x02, 0x55, 0xce, 0x56, 0x78,
                0x77, 0x72, 0xc7, 0xe7, 0x5e, 0xec, 0xb0, 0xe3, 0xb9, 0x1e, 0x1a, 0x84, 0x7d, 0x75,
                0xf4, 0x85, 0x14, 0x87,
            ],
            Core::LeftPadHigh16_64 => [
                0xf6, 0xaf, 0x32, 0xbf, 0xe6, 0x69, 0x76, 0x11, 0x57, 0xce, 0xb1, 0xe8, 0x30, 0x27,
                0x7d, 0x61, 0x0e, 0x12, 0x4f, 0x15, 0xd5, 0x08, 0x68, 0x5e, 0x40, 0x37, 0xf8, 0xfb,
                0x38, 0x8f, 0xbd, 0xa9,
            ],
            Core::LeftPadHigh1_16 => [
                0xa7, 0xef, 0x7f, 0xc5, 0x2d, 0x9a, 0x03, 0x21, 0xf9, 0x22, 0x14, 0xc5, 0x87, 0x68,
                0x9c, 0x12, 0x69, 0x6e, 0x4c, 0x46, 0x02, 0xbf, 0x53, 0x95, 0x81, 0xaf, 0xaa, 0xeb,
                0xd7, 0x79, 0x45, 0xd6,
            ],
            Core::LeftPadHigh1_32 => [
                0xb5, 0x48, 0x26, 0x36, 0xc7, 0xaf, 0x61, 0xf6, 0x88, 0xf1, 0x2b, 0x4d, 0xce, 0x20,
                0x9f, 0xcb, 0x1a, 0x73, 0x2c, 0x49, 0x0a, 0xbb, 0x23, 0x6e, 0x73, 0x96, 0x8a, 0x01,
                0xd4, 0xd9, 0x35, 0x1e,
            ],
            Core::LeftPadHigh1_64 => [
                0x6a, 0x9f, 0x2a, 0x1f, 0xf5, 0xd4, 0x6c, 0x8e, 0x6b, 0x7c, 0x3b, 0x81, 0x01, 0xaf,
                0xa0, 0x73, 0x55, 0xc4, 0xf6, 0x53, 0xc1, 0x46, 0x7e, 0x45, 0xcd, 0x25, 0xcc, 0x09,
                0x2e, 0x3a, 0x11, 0xfa,
            ],
            Core::LeftPadHigh1_8 => [
                0xd7, 0x75, 0xa8, 0x8a, 0x24, 0x80, 0x04, 0xbd, 0x82, 0x1f, 0xa0, 0x80, 0x18, 0x52,
                0x1f, 0xe8, 0x81, 0x71, 0xe8, 0x42, 0x84, 0xe0, 0xda, 0xff, 0x4b, 0xc9, 0xe6, 0xca,
                0x1d, 0x8f, 0x8d, 0xe4,
            ],
            Core::LeftPadHigh32_64 => [
                0x0b, 0xf4, 0x89, 0x4b, 0xa0, 0xe5, 0x43, 0x60, 0x18, 0xf7, 0xdd, 0xd1, 0xe1, 0xa1,
                0x2c, 0xa7, 0xcd, 0x73, 0x09, 0x6f, 0xef, 0x06, 0x6a, 0xd4, 0xb7, 0x78, 0xcc, 0x24,
                0x19, 0x57, 0xfa, 0xfc,
            ],
            Core::LeftPadHigh8_16 => [
                0x06, 0xda, 0xf4, 0xf8, 0xd5, 0x94, 0x4f, 0x1f, 0x86, 0xdf, 0xbc, 0x37, 0x05, 0x87,
                0xb4, 0x4d, 0x88, 0x71, 0x61, 0x2b, 0x7a, 0x96, 0xb0, 0x54, 0x75, 0x61, 0x92, 0xdb,
                0x8f, 0xac, 0x19, 0xb7,
            ],
            Core::LeftPadHigh8_32 => [
                0xbc, 0x20, 0x7c, 0x1e, 0x1e, 0xae, 0x88, 0xcd, 0x1b, 0xb4, 0xf9, 0x4c, 0x61, 0xdb,
                0xd8, 0x50, 0xbb, 0xfa, 0x64, 0xa6, 0x20, 0xeb, 0xc0, 0x4f, 0x0c, 0x86, 0x92, 0x8e,
                0xad, 0x2c, 0x32, 0x12,
            ],
            Core::LeftPadHigh8_64 => [
                0xfa, 0x0c, 0x41, 0xc8, 0xce, 0x56, 0x09, 0x64, 0x4c, 0x74, 0xfa, 0x32, 0x8d, 0x64,
                0xc1, 0xdc, 0xa2, 0x80, 0x50, 0xd5, 0x3d, 0xfe, 0xa6, 0xa1, 0xe6, 0xdf, 0xb4, 0x20,
                0xd9, 0xb4, 0x74, 0x67,
            ],
            Core::LeftPadLow16_32 => [
                0x23, 0xf7, 0x47, 0x96, 0xb8, 0x30, 0x20, 0xeb, 0xbf, 0x5a, 0xa5, 0xb6, 0x9a, 0xa1,
                0xf3, 0xd7, 0x3d, 0xf9, 0xb1, 0xc2, 0x67, 0xe0, 0x12, 0x4c, 0x5f, 0xfe, 0x18, 0x2a,
                0x63, 0xcd, 0x20, 0x22,
            ],
            Core::LeftPadLow16_64 => [
                0xe5, 0xd6, 0x46, 0x1a, 0xf1, 0x33, 0x1c, 0xeb, 0xa4, 0x55, 0x85, 0xe5, 0x6a, 0xe6,
                0xe1, 0x28, 0xdc, 0x7d, 0xaa, 0x0e, 0x07, 0x08, 0x61, 0xca, 0x6d, 0x19, 0xd8, 0x54,
                0x33, 0x88, 0x3d, 0xc4,
            ],
            Core::LeftPadLow1_16 => [
                0x09, 0xdb, 0xcc, 0x20, 0x32, 0x82, 0x88, 0x55, 0x2a, 0xa9, 0xe9, 0x42, 0x6c, 0x5c,
                0xee, 0x24, 0xdf, 0xce, 0xba, 0x87, 0x22, 0x74, 0xae, 0xd5, 0x5a, 0x83, 0x20, 0x46,
                0x0a, 0x17, 0x91, 0xb2,
            ],
            Core::LeftPadLow1_32 => [
                0x1d, 0x27, 0x63, 0xf2, 0x60, 0xd4, 0x72, 0x89, 0x6c, 0x88, 0x1c, 0x72, 0xfb, 0x22,
                0xb9, 0xa1, 0x19, 0x25, 0x8e, 0x12, 0x99, 0x61, 0x09, 0xd9, 0xc9, 0x49, 0xca, 0x61,
                0xf9, 0x37, 0xda, 0x0c,
            ],
            Core::LeftPadLow1_64 => [
                0xa3, 0x76, 0x87, 0x8d, 0xfe, 0x35, 0x20, 0x47, 0x29, 0xf9, 0x1c, 0xdf, 0x5b, 0xe6,
                0x58, 0xab, 0xc5, 0xe7, 0x02, 0x5a, 0xd9, 0x62, 0x84, 0xc7, 0xf6, 0x31, 0x0e, 0x9f,
                0x9b, 0x1a, 0x5a, 0x3e,
            ],
            Core::LeftPadLow1_8 => [
                0x74, 0xc2, 0x06, 0x1b, 0x87, 0xf5, 0x19, 0x0f, 0xe7, 0x43, 0x22, 0xea, 0xc2, 0x42,
                0x2b, 0x38, 0xe3, 0x7e, 0x9f, 0xcc, 0x4d, 0x60, 0x97, 0xeb, 0x5c, 0xa4, 0xe3, 0x0b,
                0x12, 0x9b, 0x7a, 0x12,
            ],
            Core::LeftPadLow32_64 => [
                0xf3, 0x1a, 0xe2, 0x84, 0x31, 0x3f, 0xdf, 0x37, 0x73, 0xe3, 0xee, 0x43, 0x96, 0x7d,
                0xc8, 0xc6, 0x7b, 0x73, 0xb7, 0x80, 0x06, 0x69, 0x66, 0x71, 0xd9, 0x0d, 0xd2, 0x59,
                0xd9, 0xa8, 0x06, 0x13,
            ],
            Core::LeftPadLow8_16 => [
                0x19, 0x04, 0x99, 0xa4, 0x9f, 0x87, 0x37, 0x32, 0xbf, 0xeb, 0x6d, 0x24, 0x3a, 0xfb,
                0x3b, 0x66, 0xdb, 0x07, 0xdc, 0xe3, 0x3f, 0xfe, 0x71, 0x73, 0x28, 0x41, 0x67, 0x08,
                0x70, 0x17, 0x00, 0xd4,
            ],
            Core::LeftPadLow8_32 => [
                0xb2, 0xcd, 0x3f, 0xf0, 0xb4, 0xde, 0x89, 0x0e, 0x28, 0xb4, 0x27, 0x96, 0x41, 0x65,
                0x2f, 0xa0, 0x8f, 0x2a, 0x89, 0x87, 0x94, 0xf6, 0xee, 0x6b, 0x12, 0x82, 0xa7, 0x98,
                0x26, 0x44, 0x8d, 0x4a,
            ],
            Core::LeftPadLow8_64 => [
                0x6e, 0x72, 0x89, 0x08, 0x25, 0x4e, 0x09, 0xf4, 0xd1, 0xa6, 0x2c, 0xae, 0x8d, 0x8a,
                0x59, 0x96, 0xe0, 0x2b, 0xd7, 0x28, 0x0d, 0x49, 0x67, 0x44, 0x40, 0x2f, 0xf3, 0x22,
                0xcd, 0x12, 0x02, 0x8f,
            ],
            Core::LeftRotate16 => [
                0x2c, 0xe2, 0x67, 0xc9, 0xf8, 0xe4, 0x4e, 0x32, 0x65, 0xb0, 0x29, 0x9e, 0x55, 0x74,
                0x29, 0x76, 0xcc, 0xec, 0xeb, 0x3a, 0x9c, 0x41, 0x06, 0x17, 0x6f, 0xbf, 0x17, 0xe6,
                0x3d, 0x48, 0xd0, 0x6c,
            ],
            Core::LeftRotate32 => [
                0x0b, 0xef, 0x6b, 0x5f, 0x77, 0x1e, 0xcc, 0x6e, 0x6f, 0x5c, 0x26, 0xf6, 0x62, 0x2d,
                0xe8, 0x62, 0x4e, 0x44, 0x34, 0x97, 0x59, 0x95, 0x14, 0xf5, 0xbc, 0x68, 0xf6, 0x87,
                0x48, 0x80, 0xe0, 0x7a,
            ],
            Core::LeftRotate64 => [
                0xcc, 0x91, 0x42, 0x19, 0xd1, 0x4f, 0xe5, 0xd2, 0x20, 0xe2, 0xaa, 0x9b, 0x64, 0xf2,
                0x4b, 0x47, 0x5d, 0x80, 0xf2, 0xca, 0xfb, 0x75, 0x31, 0xd4, 0x26, 0x40, 0x96, 0x7c,
                0x08, 0x5f, 0x9c, 0xa4,
            ],
            Core::LeftRotate8 => [
                0xbd, 0x71, 0x19, 0xce, 0xb2, 0x3e, 0x56, 0xa0, 0x75, 0x9c, 0xda, 0xad, 0x8f, 0xd6,
                0x65, 0x17, 0x46, 0xf2, 0x96, 0x9c, 0x5e, 0xa7, 0xc5, 0x70, 0x6c, 0x49, 0x2e, 0xe0,
                0xcd, 0xa6, 0x52, 0xc2,
            ],
            Core::LeftShift16 => [
                0xfd, 0x9b, 0x6d, 0x05, 0xad, 0xbd, 0xb9, 0x9b, 0xc7, 0x4a, 0xab, 0x73, 0x33, 0x84,
                0x72, 0x4e, 0x30, 0x72, 0xd2, 0xa2, 0xc2, 0xdf, 0xeb, 0x34, 0x5e, 0x24, 0xb5, 0x07,
                0x4b, 0x52, 0x78, 0x59,
            ],
            Core::LeftShift32 => [
                0xa4, 0xb3, 0xb3, 0xfc, 0x73, 0x45, 0xd3, 0x8f, 0xce, 0x54, 0x3f, 0x40, 0x1a, 0xe6,
                0xbd, 0x07, 0xba, 0x27, 0x42, 0xc3, 0x3d, 0x9a, 0xfa, 0xb3, 0xb3, 0xcc, 0xa3, 0xa0,
                0x25, 0x97, 0x3c, 0x98,
            ],
            Core::LeftShift64 => [
                0x62, 0xb7, 0x11, 0xe8, 0x4b, 0x67, 0x64, 0x8d, 0x34, 0x72, 0xac, 0xe1, 0x86, 0x29,
                0x1e, 0xb6, 0xd7, 0x73, 0xdd, 0xc0, 0xb8, 0xa3, 0xfb, 0xc2, 0x08, 0xa2, 0x75, 0xe9,
                0xdb, 0x91, 0xdd, 0xdd,
            ],
            Core::LeftShift8 => [
                0x18, 0x7b, 0xba, 0x8e, 0x41, 0x63, 0xbd, 0xd0, 0xdd, 0xb5, 0x48, 0xff, 0x6e, 0xfa,
                0x6b, 0x7e, 0x2f, 0x3d, 0x09, 0x82, 0x97, 0xd2, 0x53, 0x3e, 0x77, 0xe8, 0xa7, 0x6b,
                0x6f, 0x2d, 0xd3, 0x24,
            ],
            Core::LeftShiftWith16 => [
                0x2d, 0xd4, 0x1a, 0xc4, 0xf1, 0xd6, 0xd9, 0x68, 0x70, 0xff, 0x49, 0x66, 0x31, 0xd4,
                0x0a, 0x4f, 0xf7, 0xed, 0xeb, 0x13, 0x7d, 0x63, 0xda, 0xfe, 0xe0, 0x02, 0xfa, 0x2d,
                0xda, 0x29, 0xa7, 0x57,
            ],
            Core::LeftShiftWith32 => [
                0x9b, 0xfb, 0x24, 0x90, 0xbb, 0xa1, 0xeb, 0x3b, 0x7e, 0x25, 0x6a, 0x26, 0x09, 0x66,
                0xdc, 0x90, 0x9e, 0x73, 0xfd, 0x12, 0x59, 0x1e, 0xe1, 0x40, 0x67, 0x37, 0x18, 0xf9,
                0xf1, 0x0f, 0xc4, 0x9f,
            ],
            Core::LeftShiftWith64 => [
                0x85, 0x77, 0x46, 0x93, 0x7d, 0xe2, 0xc4, 0xc5, 0xa5, 0x5a, 0x90, 0xf3, 0x16, 0x0e,
                0xb9, 0x91, 0xdd, 0xbb, 0x60, 0x74, 0xf8, 0xa7, 0xc7, 0xc5, 0x8e, 0x8b, 0x2e, 0x38,
                0x22, 0x18, 0xa9, 0xfa,
            ],
            Core::LeftShiftWith8 => [
                0xf6, 0xf5, 0xbc, 0x6f, 0x43, 0xf6, 0x61, 0xe2, 0x72, 0x7c, 0x9e, 0xc8, 0x19, 0x63,
                0x88, 0xbc, 0xc4, 0x2b, 0x20, 0x36, 0x06, 0xa7, 0x78, 0xd5, 0x53, 0xa4, 0xea, 0x35,
                0x3f, 0xfc, 0x4b, 0xe6,
            ],
            Core::Leftmost16_1 => [
                0x7f, 0xaf, 0x54, 0x9e, 0xf6, 0x20, 0xd0, 0xd8, 0xf9, 0x0b, 0x6d, 0x08, 0x42, 0x0c,
                0xf5, 0xe9, 0x8e, 0x93, 0x84, 0xea, 0x71, 0x6b, 0x82, 0xd7, 0xd8, 0xe1, 0x97, 0xa8,
                0xc4, 0x6e, 0x33, 0xce,
            ],
            Core::Leftmost16_2 => [
                0xf2, 0xac, 0x7c, 0x05, 0x96, 0x4a, 0x69, 0x9c, 0xe5, 0x95, 0xfb, 0x21, 0xee, 0xc1,
                0x61, 0xe8, 0x7a, 0x11, 0xb6, 0x51, 0x63, 0x0d, 0x9a, 0x28, 0x93, 0x09, 0x4e, 0xa3,
                0xf3, 0x31, 0x72, 0x43,
            ],
            Core::Leftmost16_4 => [
                0x5a, 0x5e, 0xc5, 0x03, 0x9a, 0xa3, 0xaa, 0x0e, 0x58, 0xf7, 0xce, 0x64, 0xef, 0xd0,
                0x3a, 0xc2, 0x97, 0x41, 0x0f, 0x2c, 0xf4, 0x58, 0x17, 0xaf, 0x54, 0xef, 0xa6, 0xa0,
                0x07, 0x6b, 0x6f, 0x70,
            ],
            Core::Leftmost16_8 => [
                0xe2, 0x25, 0x43, 0xf8, 0xfe, 0x86, 0x1d, 0xba, 0xee, 0x74, 0x4b, 0x0b, 0x8f, 0x58,
                0xb4, 0x7b, 0x9d, 0x13, 0xfb, 0x35, 0xa9, 0x50, 0x86, 0x3a, 0x59, 0x09, 0x7c, 0xf4,
                0xd4, 0x27, 0x24, 0xb9,
            ],
            Core::Leftmost32_1 => [
                0xb3, 0x6d, 0xb0, 0x59, 0xdd, 0x42, 0xd7, 0x07, 0xbf, 0xa8, 0x39, 0xf8, 0x3b, 0xc2,
                0xed, 0x1d, 0xb5, 0x01, 0xb2, 0x3a, 0xf9, 0xaf, 0x71, 0x3f, 0x41, 0xff, 0x75, 0x5f,
                0xe2, 0xb3, 0xe9, 0x89,
            ],
            Core::Leftmost32_16 => [
                0xf1, 0x3e, 0xe3, 0x7b, 0xe6, 0x12, 0xe9, 0xdf, 0xf4, 0x90, 0x7e, 0xc3, 0xf1, 0xdd,
                0xe8, 0x6c, 0x58, 0xfe, 0xd4, 0xb1, 0x1a, 0x93, 0xb3, 0xbb, 0x2c, 0x79, 0x58, 0x00,
                0x49, 0x97, 0x92, 0x2e,
            ],
            Core::Leftmost32_2 => [
                0x57, 0xd0, 0x25, 0x1a, 0xb5, 0x66, 0xa3, 0xd2, 0x48, 0xec, 0x6e, 0x7c, 0xd1, 0xe3,
                0xe9, 0xc7, 0x1b, 0x1d, 0xde, 0x0c, 0xf2, 0x84, 0x4a, 0x8c, 0x8a, 0xe7, 0x48, 0xbd,
                0x18, 0xd3, 0x73, 0x5c,
            ],
            Core::Leftmost32_4 => [
                0x66, 0x68, 0x9b, 0xd5, 0xeb, 0x6a, 0x6d, 0xd5, 0xfc, 0x6b, 0x52, 0xfe, 0x90, 0xc1,
                0x74, 0xef, 0x0f, 0xa8, 0xb5, 0x17, 0x99, 0x30, 0x85, 0xc9, 0x01, 0xc5, 0x70, 0x2b,
                0xbd, 0x1f, 0xac, 0xe7,
            ],
            Core::Leftmost32_8 => [
                0xc8, 0x85, 0x1f, 0xf6, 0x22, 0x06, 0xe8, 0x65, 0xc3, 0xb2, 0x66, 0x41, 0x98, 0x1d,
                0x0a, 0xe1, 0x60, 0xd3, 0xd4, 0x43, 0x7a, 0x98, 0x00, 0x62, 0x79, 0xdb, 0xb3, 0xa2,
                0x57, 0x76, 0x69, 0x8e,
            ],
            Core::Leftmost64_1 => [
                0xe0, 0x5e, 0xa1, 0x9d, 0x39, 0xee, 0x23, 0x5d, 0xde, 0x58, 0xe3, 0x1e, 0x4b, 0x95,
                0x34, 0xb6, 0xcb, 0x39, 0x9b, 0x1f, 0x2f, 0xa1, 0x81, 0x48, 0x8c, 0xed, 0xc2, 0x03,
                0xcd, 0x95, 0x96, 0x2a,
            ],
            Core::Leftmost64_16 => [
                0xc3, 0xdd, 0xb0, 0x79, 0xc1, 0x12, 0x92, 0x5d, 0x42, 0x3b, 0xbd, 0x30, 0xd9, 0x02,
                0x26, 0x1e, 0x68, 0xa4, 0x1c, 0x3d, 0xe4, 0x38, 0x11, 0x66, 0xe1, 0x40, 0x05, 0x02,
                0x8b, 0xa8, 0x4b, 0x98,
            ],
            Core::Leftmost64_2 => [
                0x26, 0x4d, 0xb1, 0x23, 0x78, 0xdf, 0xad, 0xc1, 0xad, 0x6f, 0xf8, 0xd5, 0x78, 0xec,
                0x4d, 0xa1, 0x85, 0x32, 0xa1, 0xe6, 0xbd, 0xab, 0x7c, 0x17, 0x56, 0x9f, 0xdd, 0x7b,
                0xf4, 0x06, 0xfb, 0x20,
            ],
            Core::Leftmost64_32 => [
                0x19, 0xa5, 0xd5, 0x3f, 0xe6, 0xb3, 0x0b, 0xe0, 0x2b, 0x91, 0xba, 0xfc, 0x63, 0xcb,
                0xe5, 0x50, 0x55, 0x98, 0xc1, 0x42, 0x55, 0xd8, 0x9e, 0x33, 0x48, 0x56, 0xdb, 0x5b,
                0x5b, 0xa2, 0x0d, 0xed,
            ],
            Core::Leftmost64_4 => [
                0x4c, 0x6f, 0xc8, 0xa0, 0x65, 0x46, 0x8e, 0x47, 0x51, 0x84, 0x4a, 0xef, 0xb6, 0xcc,
                0x49, 0x57, 0x58, 0x64, 0xf5, 0x67, 0xc5, 0x62, 0x86, 0x65, 0x4c, 0x43, 0xb3, 0xf8,
                0x33, 0x30, 0x32, 0x31,
            ],
            Core::Leftmost64_8 => [
                0x98, 0xb5, 0xe0, 0x16, 0xbd, 0x41, 0x45, 0x77, 0x8d, 0xfc, 0x63, 0x17, 0xdc, 0x65,
                0xd6, 0xc7, 0x5f, 0xef, 0x28, 0x57, 0x96, 0xb5, 0x9d, 0xea, 0xf1, 0x4a, 0xa0, 0x31,
                0x99, 0xf9, 0x48, 0x96,
            ],
            Core::Leftmost8_1 => [
                0x38, 0xe3, 0x98, 0x5f, 0x1f, 0x99, 0xea, 0xb1, 0xa7, 0x87, 0xe7, 0x20, 0x0c, 0x30,
                0xcd, 0x9b, 0x95, 0x63, 0x1f, 0x95, 0xb4, 0x19, 0xfd, 0x76, 0xec, 0xe1, 0x99, 0x67,
                0xd1, 0xb0, 0x30, 0x69,
            ],
            Core::Leftmost8_2 => [
                0x51, 0xdb, 0xca, 0xeb, 0xc3, 0xac, 0x16, 0x43, 0xe5, 0xb0, 0x8c, 0xa2, 0x55, 0x29,
                0xf0, 0xe2, 0xb0, 0xbd, 0x9b, 0xdf, 0xa0, 0x41, 0xfd, 0x4d, 0xb7, 0x70, 0x29, 0xd0,
                0xc5, 0xff, 0x15, 0xb9,
            ],
            Core::Leftmost8_4 => [
                0xb3, 0x94, 0x20, 0x9c, 0x9b, 0x34, 0x28, 0x4d, 0xde, 0x65, 0xea, 0x97, 0x35, 0xe4,
                0xeb, 0x0c, 0xdf, 0x01, 0xce, 0x87, 0x10, 0xe5, 0x8c, 0x5e, 0xd2, 0x0e, 0x8b, 0x94,
                0x6b, 0xf3, 0xf6, 0xea,
            ],
            Core::LinearCombination1 => [
                0xfd, 0xd4, 0x7c, 0xa5, 0xc3, 0xb8, 0xbe, 0xd9, 0xf5, 0xf8, 0x27, 0x32, 0x9a, 0xf7,
                0xc0, 0x8b, 0x88, 0xd3, 0x54, 0x21, 0xd3, 0xd0, 0x2f, 0x3d, 0x6c, 0xae, 0x3e, 0x5b,
                0x49, 0xdc, 0xbe, 0xa3,
            ],
            Core::LinearVerify1 => [
                0x7b, 0xe5, 0xd0, 0x46, 0x72, 0x84, 0x64, 0xb8, 0x30, 0x1d, 0x89, 0xd9, 0xd0, 0xde,
                0x8e, 0x19, 0x67, 0x5d, 0xf4, 0xd9, 0x10, 0x04, 0x7b, 0x25, 0xb3, 0x72, 0xd2, 0xc4,
                0x16, 0x6b, 0x3a, 0x0d,
            ],
            Core::Low1 => [
                0xf3, 0xd1, 0x0b, 0x07, 0x49, 0x33, 0x2c, 0xdd, 0xae, 0x0a, 0x49, 0xcc, 0x02, 0x65,
                0x7b, 0x02, 0x58, 0xe0, 0xa5, 0xa7, 0x05, 0x95, 0x81, 0x00, 0x22, 0xc5, 0xa6, 0x4e,
                0x90, 0x40, 0x88, 0xb0,
            ],
            Core::Low16 => [
                0x67, 0x0d, 0x1c, 0xf0, 0x8d, 0x65, 0x5f, 0x40, 0xfd, 0xa2, 0x27, 0x7a, 0xa9, 0x15,
                0x54, 0x14, 0xa3, 0xb4, 0xd8, 0x0e, 0x97, 0xf6, 0xb8, 0x67, 0xd4, 0xe5, 0x62, 0xe3,
                0xa0, 0x5b, 0x46, 0x45,
            ],
            Core::Low32 => [
                0xd9, 0xbf, 0x78, 0xc4, 0x9f, 0x8d, 0x6b, 0x7a, 0xe4, 0xb4, 0xb0, 0x84, 0xf0, 0xd3,
                0x64, 0xf7, 0x2b, 0x04, 0x62, 0xff, 0x39, 0xc3, 0x21, 0xc0, 0x2d, 0x92, 0xb5, 0x42,
                0x60, 0x86, 0xd0, 0xb5,
            ],
            Core::Low64 => [
                0x36, 0x4d, 0xa0, 0x86, 0xeb, 0xe3, 0x01, 0x1b, 0x1b, 0x58, 0x5f, 0x5c, 0xb9, 0xab,
                0xc8, 0x7e, 0xe5, 0x7b, 0x56, 0x37, 0xcb, 0x95, 0xec, 0xf3, 0x12, 0x93, 0x8d, 0xf2,
                0x86, 0x30, 0xfd, 0xa4,
            ],
            Core::Low8 => [
                0xd4, 0x24, 0x4f, 0x7c, 0x17, 0xb0, 0x49, 0x04, 0x8a, 0xc5, 0x8b, 0x4d, 0xa5, 0x78,
                0x16, 0x9d, 0x0e, 0xd6, 0x51, 0x49, 0x40, 0x8e, 0xa9, 0xb0, 0x33, 0xb3, 0xd8, 0xb7,
                0x7d, 0x76, 0xc6, 0x87,
            ],
            Core::Lt16 => [
                0x32, 0x60, 0x28, 0x44, 0xfb, 0x68, 0xeb, 0x7a, 0xf6, 0xf7, 0xf1, 0xbd, 0xa3, 0x50,
                0xe2, 0x29, 0x2c, 0x3f, 0xc2, 0x87, 0xf8, 0x1e, 0xca, 0x17, 0xf0, 0x68, 0xf4, 0xfa,
                0x94, 0x2d, 0x40, 0x2a,
            ],
            Core::Lt32 => [
                0xb7, 0x03, 0xd3, 0x5d, 0xee, 0xd9, 0x6c, 0xe3, 0x9d, 0xf8, 0xad, 0x21, 0x4f, 0x74,
                0x89, 0x8a, 0x17, 0x65, 0xaa, 0x14, 0x06, 0xbc, 0xe4, 0xbf, 0x9a, 0xfc, 0x5c, 0x63,
                0xa8, 0x38, 0x2d, 0x9e,
            ],
            Core::Lt64 => [
                0x94, 0xf1, 0xe4, 0x8d, 0x7c, 0x87, 0xc1, 0x80, 0x83, 0x84, 0x1e, 0xc1, 0x24, 0x67,
                0x94, 0x2b, 0x76, 0xa8, 0x93, 0xfa, 0x08, 0xe9, 0x41, 0x23, 0xb4, 0xfe, 0x80, 0x20,
                0xd5, 0xf1, 0xd9, 0xaf,
            ],
            Core::Lt8 => [
                0xf4, 0x0a, 0x8c, 0xfa, 0x78, 0x39, 0x93, 0x9c, 0xa4, 0xdd, 0x3d, 0x01, 0x22, 0xd4,
                0xb5, 0x6c, 0xc3, 0xc6, 0x54, 0xe3, 0x5e, 0x2a, 0xf2, 0xd3, 0x51, 0x1f, 0xc9, 0x44,
                0x6c, 0xf0, 0xf8, 0xaf,
            ],
            Core::Maj1 => [
                0x79, 0xd1, 0x85, 0xf5, 0x3d, 0x2e, 0x2f, 0xab, 0x87, 0x54, 0xc1, 0xb9, 0x58, 0xa5,
                0xa1, 0xb5, 0xb9, 0xdf, 0xa5, 0x0d, 0x90, 0x0c, 0xe4, 0x3a, 0xd7, 0x35, 0x61, 0xb4,
                0x4a, 0x88, 0x78, 0x22,
            ],
            Core::Maj16 => [
                0xa1, 0x7f, 0x14, 0xf2, 0x9c, 0xe6, 0x4c, 0xc0, 0xa3, 0x41, 0x07, 0x06, 0x74, 0x7a,
                0x48, 0x32, 0x19, 0x0a, 0xf2, 0x3c, 0x03, 0xca, 0xc1, 0x64, 0xa4, 0x7f, 0xc1, 0x27,
                0xb4, 0x8f, 0x34, 0x28,
            ],
            Core::Maj32 => [
                0xe1, 0x9f, 0xad, 0xfa, 0xc6, 0xc7, 0x85, 0xb8, 0x98, 0x29, 0xa9, 0x1e, 0xc0, 0x21,
                0x59, 0x35, 0xc4, 0x9f, 0xbc, 0x9d, 0x20, 0xc6, 0xbe, 0x78, 0xde, 0x9a, 0x63, 0x55,
                0xe8, 0x18, 0x13, 0x56,
            ],
            Core::Maj64 => [
                0xf1, 0x19, 0xb2, 0xf2, 0x38, 0x3b, 0x9f, 0xe6, 0xdd, 0x76, 0x8e, 0x5e, 0xe2, 0x7d,
                0xb4, 0xf5, 0x78, 0x38, 0x68, 0xcf, 0x9b, 0x9e, 0x13, 0x85, 0x41, 0x00, 0x39, 0x90,
                0x1d, 0x17, 0xc2, 0x30,
            ],
            Core::Maj8 => [
                0x41, 0x56, 0x21, 0xcb, 0x9e, 0x42, 0x8f, 0x72, 0xeb, 0x29, 0x56, 0xe4, 0x69, 0x6f,
                0x70, 0x65, 0xde, 0xe6, 0xa8, 0xda, 0x5e, 0xed, 0xde, 0x32, 0x36, 0x2b, 0x81, 0x24,
                0xa1, 0x95, 0xdd, 0x50,
            ],
            Core::Max16 => [
                0xce, 0xb7, 0x1d, 0x05, 0xf9, 0x55, 0x18, 0xd3, 0xd4, 0x09, 0x25, 0xe7, 0x3c, 0xa2,
                0x9b, 0x7c, 0x3e, 0xc6, 0x2b, 0x92, 0xaf, 0x47, 0x88, 0x04, 0x8f, 0x7b, 0xe8, 0x06,
                0x52, 0xf6, 0xba, 0x2b,
            ],
            Core::Max32 => [
                0x67, 0x9f, 0x51, 0xce, 0xca, 0x3e, 0x7b, 0xfc, 0x4b, 0x8b, 0x8c, 0x14, 0x5f, 0xbe,
                0x06, 0x47, 0x3f, 0x65, 0x3b, 0x21, 0xfb, 0x8f, 0x58, 0x38, 0x39, 0xfc, 0x9f, 0x9e,
                0x22, 0x1a, 0x99, 0xe7,
            ],
            Core::Max64 => [
                0x6a, 0xeb, 0x60, 0x2c, 0x1f, 0x2d, 0xe2, 0x0e, 0x6a, 0xee, 0x7e, 0x23, 0x3b, 0x29,
                0xdb, 0x2e, 0xe5, 0x33, 0xa0, 0xc1, 0x1e, 0x33, 0x63, 0xfc, 0x67, 0xb3, 0x0d, 0x6c,
                0x78, 0x33, 0x83, 0x52,
            ],
            Core::Max8 => [
                0x5c, 0x1f, 0x36, 0x9a, 0xfa, 0xac, 0x6b, 0x74, 0xb5, 0x7e, 0xdf, 0x3c, 0xec, 0xfb,
                0x61, 0xf2, 0x07, 0xe0, 0x59, 0x39, 0xb1, 0xc6, 0xfc, 0x73, 0x5b, 0x07, 0x83, 0x6f,
                0x39, 0x73, 0xd9, 0x04,
            ],
            Core::Median16 => [
                0xc0, 0x9a, 0x30, 0x0b, 0x6d, 0x02, 0x3c, 0xa4, 0x09, 0x27, 0x9e, 0x42, 0x27, 0x47,
                0x0d, 0xd8, 0xbd, 0x52, 0xf1, 0x68, 0xf1, 0xf8, 0xbf, 0xa9, 0x38, 0x77, 0x9f, 0x03,
                0xc8, 0x6e, 0xf1, 0x52,
            ],
            Core::Median32 => [
                0x84, 0x81, 0x00, 0x59, 0x86, 0xf4, 0x29, 0x48, 0x05, 0x1a, 0x36, 0x4a, 0xd4, 0x18,
                0x70, 0x2e, 0xbc, 0x76, 0xfa, 0x36, 0x53, 0x44, 0xaa, 0x2e, 0xa1, 0xa6, 0x30, 0x05,
                0xe6, 0x1f, 0xe8, 0xbb,
            ],
            Core::Median64 => [
                0xcc, 0xba, 0xa9, 0x00, 0x0f, 0x3c, 0x0c, 0x9e, 0xc7, 0x1b, 0xc0, 0xdb, 0x5d, 0xa8,
                0x52, 0x83, 0x6f, 0xa9, 0x12, 0x28, 0xb3, 0x73, 0xf5, 0x6b, 0xce, 0xf0, 0xb2, 0x25,
                0x85, 0xd4, 0x70, 0xff,
            ],
            Core::Median8 => [
                0xc5, 0xb2, 0xc0, 0x01, 0xdc, 0x50, 0x4f, 0xf4, 0xc2, 0xe0, 0x52, 0x86, 0x56, 0xd4,
                0xda, 0xbf, 0x33, 0xfb, 0x41, 0x42, 0xd0, 0x26, 0xab, 0x61, 0x37, 0x4c, 0x23, 0x89,
                0x0b, 0xda, 0x1f, 0x18,
            ],
            Core::Min16 => [
                0xb2, 0x26, 0x58, 0x69, 0xea, 0x19, 0x0b, 0xba, 0x73, 0x0c, 0xd7, 0xd7, 0xcd, 0x05,
                0x24, 0xd1, 0x3e, 0xbf, 0x76, 0x7e, 0x1b, 0x12, 0x5c, 0x44, 0x1b, 0x1b, 0xab, 0x0e,
                0x3d, 0xbe, 0xef, 0x26,
            ],
            Core::Min32 => [
                0x6d, 0x92, 0x1b, 0xfb, 0xaf, 0x93, 0x50, 0x63, 0xbd, 0x4b, 0x4e, 0x48, 0xd6, 0xd1,
                0x36, 0x64, 0xbd, 0x5d, 0x0a, 0xc2, 0x5f, 0x5e, 0xe3, 0x0e, 0xbe, 0x51, 0x8e, 0x79,
                0x11, 0x51, 0x75, 0x0a,
            ],
            Core::Min64 => [
                0x05, 0xd9, 0xc0, 0xa2, 0xa7, 0x1f, 0x88, 0x7d, 0x76, 0x46, 0xe4, 0x15, 0x82, 0x72,
                0x30, 0xcf, 0xbe, 0xcc, 0x0d, 0x82, 0x36, 0xc5, 0x62, 0xf2, 0xdf, 0xda, 0x3d, 0xcd,
                0xe1, 0x3b, 0x2f, 0x03,
            ],
            Core::Min8 => [
                0x57, 0xba, 0x25, 0x7d, 0xc7, 0x0a, 0x97, 0x5e, 0xfb, 0x22, 0xd7, 0xe0, 0xa9, 0x98,
                0xae, 0xa9, 0xd5, 0x7f, 0x20, 0x8a, 0x74, 0x56, 0x5a, 0x68, 0xf2, 0xac, 0x8b, 0x50,
                0x1e, 0x22, 0x0d, 0xc0,
            ],
            Core::Modulo16 => [
                0x7a, 0xbd, 0xe1, 0xdc, 0xab, 0x9d, 0x23, 0xea, 0xe1, 0xd0, 0xa0, 0xa6, 0xb3, 0x84,
                0xfb, 0x39, 0xdf, 0x21, 0xc3, 0x83, 0x16, 0xc0, 0xbb, 0xcc, 0x6c, 0x70, 0x6d, 0x7b,
                0x5f, 0x3d, 0x68, 0xbb,
            ],
            Core::Modulo32 => [
                0x7d, 0x12, 0x78, 0x66, 0xce, 0xb0, 0x03, 0xec, 0x84, 0xa2, 0x70, 0x2a, 0xff, 0x40,
                0x59, 0xa7, 0xc6, 0xfb, 0x00, 0xa5, 0xa5, 0xba, 0xce, 0x90, 0xb9, 0xd9, 0xc0, 0x20,
                0xb5, 0xdc, 0x02, 0x97,
            ],
            Core::Modulo64 => [
                0xaa, 0xfe, 0xd9, 0xe3, 0x7b, 0x3d, 0x3e, 0xd4, 0x1d, 0xb1, 0x71, 0x16, 0x7b, 0x0f,
                0xa2, 0x3c, 0x8b, 0x82, 0x3a, 0xdd, 0x9a, 0x48, 0xad, 0xde, 0x24, 0xba, 0x20, 0xf9,
                0x19, 0x4b, 0x07, 0x9c,
            ],
            Core::Modulo8 => [
                0x90, 0xf3, 0xac, 0xeb, 0x58, 0x47, 0x71, 0x39, 0x06, 0xd2, 0xf8, 0xef, 0x7c, 0xb4,
                0xb6, 0x6b, 0x08, 0x92, 0x81, 0x0e, 0xa8, 0x01, 0xbf, 0xa9, 0xd7, 0x46, 0xda, 0x1a,
                0xd6, 0xd9, 0x80, 0x09,
            ],
            Core::Multiply16 => [
                0x68, 0x68, 0xa8, 0x5d, 0x9f, 0x76, 0xc9, 0x58, 0xff, 0xda, 0x84, 0x78, 0x05, 0xf5,
                0xeb, 0xc0, 0x9f, 0x43, 0xbd, 0xdb, 0xc1, 0x40, 0x95, 0x2a, 0xbb, 0x2a, 0x4c, 0xc0,
                0x19, 0x27, 0xeb, 0xdd,
            ],
            Core::Multiply32 => [
                0x69, 0xf9, 0x1b, 0x99, 0x4d, 0x83, 0x40, 0xe4, 0x8c, 0x0f, 0x11, 0x24, 0x99, 0x0d,
                0xe0, 0x3b, 0x51, 0x2a, 0xdc, 0xec, 0xd7, 0xbd, 0xeb, 0x29, 0xe4, 0x09, 0xa3, 0x51,
                0x19, 0xf1, 0x1f, 0xac,
            ],
            Core::Multiply64 => [
                0xd1, 0xf1, 0x48, 0xf2, 0xd4, 0x50, 0xe8, 0x03, 0x47, 0x43, 0x43, 0xe5, 0x9d, 0x2e,
                0x0c, 0x4d, 0xa7, 0xd6, 0xb0, 0x97, 0x2c, 0xfa, 0x25, 0x1c, 0x9a, 0x82, 0x2a, 0xb4,
                0x42, 0x63, 0xe1, 0x62,
            ],
            Core::Multiply8 => [
                0x2a, 0x8c, 0x03, 0x63, 0x2e, 0xcc, 0x5e, 0x61, 0xe3, 0xf1, 0x20, 0x3a, 0x91, 0x2b,
                0x1a, 0x77, 0x3e, 0xb6, 0xbf, 0x63, 0x4b, 0x6b, 0xdf, 0x56, 0x63, 0xb8, 0x43, 0x1d,
                0xb6, 0xf5, 0x5b, 0xe3,
            ],
            Core::Negate16 => [
                0xf1, 0xa2, 0x24, 0x28, 0x06, 0xaa, 0xdf, 0xc4, 0xdc, 0x11, 0xde, 0x65, 0xe8, 0x77,
                0xb0, 0xd8, 0x73, 0xb5, 0xd5, 0x65, 0xee, 0xe3, 0xf8, 0x48, 0xa7, 0x44, 0x28, 0x31,
                0xe5, 0x2c, 0x63, 0x0a,
            ],
            Core::Negate32 => [
                0xdb, 0x75, 0xba, 0x2c, 0x60, 0x95, 0x2e, 0x76, 0x5d, 0x7a, 0x5c, 0xe4, 0xaa, 0xa3,
                0x12, 0x64, 0x99, 0x0f, 0xcc, 0xac, 0x75, 0x0a, 0x80, 0xad, 0xc6, 0x78, 0x78, 0x7f,
                0x5f, 0x72, 0xbf, 0xdd,
            ],
            Core::Negate64 => [
                0xc9, 0x1b, 0xcd, 0xb7, 0x1d, 0x19, 0x6a, 0x3b, 0xf6, 0x5c, 0xa8, 0x3f, 0x22, 0x5a,
                0x5e, 0x1b, 0xbb, 0x73, 0xbd, 0x2e, 0x35, 0x75, 0xcf, 0x16, 0x5e, 0x5a, 0x47, 0x17,
                0xe9, 0x47, 0x37, 0xeb,
            ],
            Core::Negate8 => [
                0x8a, 0xff, 0xfa, 0xb3, 0x6b, 0xbc, 0xa0, 0xe9, 0x3f, 0xff, 0x2c, 0x67, 0x27, 0x24,
                0xe4, 0xb7, 0xbc, 0x24, 0xf6, 0x38, 0xe4, 0x04, 0x89, 0x12, 0x66, 0x47, 0x88, 0xf0,
                0x38, 0xc9, 0x95, 0xcf,
            ],
            Core::One16 => [
                0xeb, 0xe5, 0x31, 0x62, 0x82, 0x42, 0xd1, 0x17, 0x7f, 0xa9, 0x4e, 0xcf, 0x46, 0x92,
                0x21, 0xbe, 0x15, 0x90, 0x5e, 0x07, 0xf3, 0xc8, 0xfa, 0x15, 0xd3, 0xd9, 0xd4, 0xaa,
                0x6a, 0x81, 0x5c, 0x8d,
            ],
            Core::One32 => [
                0x99, 0x1f, 0x9e, 0x3a, 0xcb, 0x6f, 0xf7, 0x8f, 0x03, 0xf5, 0x78, 0x03, 0x54, 0x10,
                0x96, 0xbe, 0x79, 0x44, 0xc9, 0x18, 0xc2, 0xaf, 0x3d, 0x68, 0xa3, 0x21, 0x0d, 0x0b,
                0x9f, 0x99, 0x0e, 0xc2,
            ],
            Core::One64 => [
                0xfe, 0x30, 0x8d, 0x3f, 0x68, 0x97, 0xcd, 0xce, 0xed, 0xc0, 0x2c, 0xed, 0x16, 0x30,
                0xe5, 0x5e, 0x87, 0xbd, 0xcf, 0xec, 0xd4, 0xae, 0xc3, 0xce, 0x54, 0x94, 0x8f, 0x57,
                0xdf, 0xe1, 0x8e, 0x27,
            ],
            Core::One8 => [
                0x68, 0x24, 0x82, 0x61, 0x8b, 0xee, 0x00, 0xbb, 0x85, 0xdb, 0x83, 0xe3, 0x9d, 0xc4,
                0x0c, 0x28, 0x51, 0xf6, 0x4c, 0x00, 0x69, 0x53, 0x44, 0x0f, 0x43, 0xb9, 0x73, 0xaf,
                0x7a, 0x72, 0x32, 0xc9,
            ],
            Core::Or1 => [
                0xa9, 0xa6, 0xa7, 0x36, 0x9f, 0x3f, 0xd7, 0x7e, 0xfd, 0x36, 0xb8, 0xa4, 0x1d, 0xad,
                0xd7, 0x0c, 0x1a, 0x4e, 0xde, 0x70, 0x62, 0x3b, 0x14, 0x0a, 0x8d, 0x50, 0xde, 0xc7,
                0x4d, 0xd4, 0x91, 0x2d,
            ],
            Core::Or16 => [
                0xda, 0x96, 0xd8, 0x33, 0x21, 0x55, 0xc3, 0x63, 0x8c, 0xf2, 0xa6, 0x20, 0x40, 0xb2,
                0x24, 0xc1, 0x3c, 0x1b, 0xae, 0xee, 0x91, 0x32, 0x18, 0x53, 0x3d, 0xef, 0xed, 0x9e,
                0x16, 0x6a, 0x64, 0x83,
            ],
            Core::Or32 => [
                0xb6, 0x72, 0xb0, 0xf7, 0xc9, 0x29, 0x74, 0x97, 0xa0, 0xc9, 0xcc, 0x84, 0xd7, 0xf7,
                0xdd, 0xe2, 0x6d, 0x6d, 0xe5, 0xe4, 0xbe, 0xc1, 0x4b, 0x20, 0xe0, 0x10, 0xce, 0xe1,
                0xcc, 0xc0, 0x8f, 0x09,
            ],
            Core::Or64 => [
                0x13, 0x3d, 0x50, 0x5c, 0xe5, 0x03, 0xfb, 0xeb, 0x13, 0x29, 0x58, 0x0f, 0x5b, 0x96,
                0xa0, 0xd9, 0x7f, 0x3a, 0x5a, 0xba, 0x79, 0xd7, 0x1b, 0xbd, 0x85, 0x88, 0x4a, 0x12,
                0x81, 0x39, 0xc0, 0x08,
            ],
            Core::Or8 => [
                0x8e, 0xf1, 0xc6, 0xa1, 0xca, 0xe0, 0x74, 0x03, 0xda, 0xcd, 0x8c, 0x13, 0x2e, 0x88,
                0x99, 0xc1, 0x85, 0x87, 0x45, 0xc8, 0x6d, 0x1f, 0xdd, 0x4c, 0x9f, 0xda, 0x63, 0x62,
                0x51, 0x00, 0x48, 0xb3,
            ],
            Core::ParseLock => [
                0xce, 0x03, 0xe1, 0x95, 0xff, 0x84, 0x1f, 0x93, 0x09, 0xee, 0x27, 0x7d, 0xc8, 0x54,
                0xf9, 0xc3, 0xd4, 0xcb, 0x65, 0x54, 0x20, 0xe9, 0x21, 0x13, 0x59, 0xe1, 0xf2, 0x04,
                0x64, 0x1a, 0x23, 0xa6,
            ],
            Core::ParseSequence => [
                0xc8, 0xa1, 0xcb, 0x4e, 0xad, 0xf2, 0xa8, 0xda, 0x31, 0x2c, 0x0d, 0x27, 0x35, 0xcf,
                0x97, 0xde, 0xe0, 0xdb, 0xae, 0x4d, 0x73, 0x11, 0x09, 0xca, 0x01, 0x94, 0x52, 0x1e,
                0x70, 0x7d, 0x44, 0x5b,
            ],
            Core::PointVerify1 => [
                0xd6, 0x3f, 0xf0, 0xe5, 0xa5, 0x00, 0x85, 0x94, 0xd5, 0x55, 0xa6, 0x15, 0xe8, 0x1f,
                0xd2, 0xa6, 0x9a, 0x48, 0x1b, 0x4f, 0x45, 0x7f, 0x42, 0xf4, 0x82, 0x83, 0xde, 0x2b,
                0x52, 0x23, 0x84, 0xe0,
            ],
            Core::RightExtend16_32 => [
                0x1f, 0xe0, 0x5d, 0x19, 0x7e, 0xe6, 0xf1, 0x5e, 0xfb, 0x17, 0x54, 0x52, 0x36, 0xef,
                0xeb, 0x38, 0xc1, 0x6e, 0x7a, 0x5a, 0xdc, 0x82, 0x8b, 0x9a, 0xa0, 0x4a, 0x3f, 0x6e,
                0x46, 0x78, 0xb5, 0xba,
            ],
            Core::RightExtend16_64 => [
                0x7c, 0xb9, 0xcc, 0x78, 0x65, 0x52, 0x85, 0xa1, 0x62, 0xff, 0xd8, 0x0c, 0x6d, 0x59,
                0xf8, 0x28, 0xec, 0xdb, 0x75, 0xcb, 0x64, 0xb0, 0x82, 0x74, 0xf8, 0x9d, 0x20, 0x10,
                0x7c, 0x2b, 0xe4, 0x43,
            ],
            Core::RightExtend32_64 => [
                0x32, 0x5d, 0x1d, 0xff, 0x73, 0x73, 0x83, 0xf4, 0x31, 0x3d, 0xcb, 0x23, 0xad, 0xf9,
                0x1e, 0x2a, 0xa4, 0x42, 0x30, 0x38, 0x84, 0x62, 0x83, 0x2f, 0xa1, 0x5f, 0xe4, 0x33,
                0x36, 0x33, 0xd9, 0x30,
            ],
            Core::RightExtend8_16 => [
                0x53, 0x46, 0xf1, 0x5d, 0x34, 0x9b, 0xfe, 0xbf, 0x2b, 0x5b, 0x4b, 0x45, 0xf9, 0xc0,
                0xfc, 0x39, 0x39, 0x20, 0x67, 0xdc, 0x6f, 0x97, 0x97, 0x51, 0xa1, 0x59, 0x66, 0xb5,
                0xd1, 0x7e, 0xfb, 0x9f,
            ],
            Core::RightExtend8_32 => [
                0xd5, 0xae, 0xe5, 0x10, 0xdf, 0xd5, 0x0a, 0xca, 0xbd, 0x8a, 0xc3, 0x29, 0xd2, 0x97,
                0x48, 0xa6, 0xf5, 0x94, 0x08, 0xc2, 0x68, 0x63, 0xab, 0x62, 0x4b, 0xef, 0xc2, 0x44,
                0x7e, 0xc2, 0x6a, 0x44,
            ],
            Core::RightExtend8_64 => [
                0xa8, 0x43, 0xa3, 0x15, 0xf6, 0xdc, 0xd9, 0xbc, 0xff, 0x58, 0x68, 0x59, 0xed, 0x6c,
                0x31, 0xd8, 0xcb, 0x98, 0x69, 0x07, 0x1b, 0x45, 0xb6, 0x79, 0x57, 0xed, 0x15, 0xae,
                0xd2, 0xe0, 0xae, 0x5d,
            ],
            Core::RightPadHigh16_32 => [
                0x75, 0xbe, 0xc3, 0x00, 0x6c, 0x04, 0xce, 0x01, 0x8f, 0x4a, 0x61, 0x10, 0xaa, 0xb3,
                0xc5, 0x5c, 0xad, 0x45, 0x32, 0x48, 0xc1, 0xe7, 0xeb, 0x04, 0x2c, 0x82, 0x57, 0xe6,
                0xb9, 0x9c, 0xcd, 0x05,
            ],
            Core::RightPadHigh16_64 => [
                0x93, 0x60, 0x59, 0x28, 0xb8, 0x49, 0xb0, 0xf8, 0xdf, 0xbc, 0x62, 0xa1, 0x25, 0x90,
                0xbb, 0x1a, 0x92, 0x0b, 0xa5, 0x9d, 0x5b, 0xdc, 0x49, 0xdf, 0x0f, 0xaa, 0xb6, 0xa3,
                0x35, 0x1a, 0x96, 0x61,
            ],
            Core::RightPadHigh1_16 => [
                0x34, 0xc1, 0x08, 0xae, 0x8a, 0x54, 0xe2, 0xc6, 0x49, 0x9b, 0xfe, 0xc5, 0xd6, 0x93,
                0x28, 0x15, 0xf8, 0x87, 0x14, 0x9c, 0x11, 0x49, 0xb7, 0x89, 0x94, 0x60, 0x3f, 0x40,
                0xf1, 0xff, 0xd2, 0xe7,
            ],
            Core::RightPadHigh1_32 => [
                0x5b, 0x4a, 0x76, 0xed, 0x0a, 0x50, 0xac, 0xe5, 0x2b, 0x79, 0x1a, 0xb9, 0xa5, 0xc5,
                0xeb, 0xcb, 0x3c, 0xbb, 0xa0, 0xda, 0x3a, 0x95, 0xac, 0x40, 0x51, 0xeb, 0xa9, 0xab,
                0x15, 0x0f, 0xd0, 0xdc,
            ],
            Core::RightPadHigh1_64 => [
                0xd4, 0x99, 0xb4, 0xbc, 0xf5, 0xdf, 0x6a, 0x71, 0xc8, 0x66, 0x4f, 0x96, 0xb7, 0xb9,
                0x64, 0xae, 0x2b, 0xbb, 0xda, 0x55, 0x21, 0x97, 0x97, 0xec, 0x6b, 0x9d, 0x2d, 0xf4,
                0xa5, 0xa5, 0xe8, 0xba,
            ],
            Core::RightPadHigh1_8 => [
                0x5e, 0xe6, 0xa8, 0x38, 0x2f, 0x9b, 0xbd, 0xc0, 0xc5, 0x81, 0x03, 0x6e, 0x51, 0xf1,
                0xd9, 0x27, 0x69, 0x96, 0x72, 0x2a, 0x18, 0x24, 0x13, 0xa1, 0x03, 0xcb, 0x98, 0xe9,
                0xa8, 0x98, 0xf9, 0x1e,
            ],
            Core::RightPadHigh32_64 => [
                0x06, 0x2e, 0x02, 0xf4, 0x87, 0x97, 0x78, 0x7b, 0xef, 0x0e, 0xab, 0x59, 0x67, 0x58,
                0x05, 0x11, 0xdc, 0xbc, 0xd7, 0x44, 0xbe, 0x65, 0xc4, 0xf6, 0xc3, 0x1b, 0x89, 0xe5,
                0x47, 0xe2, 0xe1, 0xd5,
            ],
            Core::RightPadHigh8_16 => [
                0xa5, 0xc9, 0x85, 0xe8, 0xae, 0xa8, 0x42, 0xfa, 0xdf, 0x9a, 0x74, 0x8b, 0x96, 0x4c,
                0xa5, 0x15, 0x04, 0xe9, 0xeb, 0xb4, 0xac, 0x49, 0x45, 0x52, 0x64, 0x8f, 0x3a, 0x67,
                0xa1, 0x60, 0xc4, 0xc5,
            ],
            Core::RightPadHigh8_32 => [
                0xbf, 0x92, 0xf3, 0x9f, 0xdc, 0xde, 0xe4, 0xc4, 0x96, 0x12, 0x18, 0x70, 0x75, 0x30,
                0xcb, 0x10, 0x08, 0xad, 0xf7, 0x55, 0x89, 0xd2, 0x50, 0xca, 0xff, 0xb3, 0x70, 0xb3,
                0x3e, 0xba, 0xe2, 0x38,
            ],
            Core::RightPadHigh8_64 => [
                0xce, 0x7e, 0x8a, 0x0e, 0x4a, 0x9b, 0x04, 0x01, 0x5c, 0xc2, 0x4f, 0x26, 0x99, 0x0f,
                0x68, 0xdc, 0x93, 0xb6, 0x7b, 0x3f, 0x67, 0x6f, 0xbf, 0xea, 0xa1, 0x7f, 0xd7, 0x4c,
                0xd6, 0x03, 0x57, 0xc1,
            ],
            Core::RightPadLow16_32 => [
                0xb0, 0xf7, 0x15, 0x1d, 0x0a, 0x79, 0xde, 0x08, 0xf2, 0x33, 0x80, 0x6d, 0x4f, 0x7e,
                0x1f, 0x88, 0xc3, 0x29, 0xb9, 0x64, 0x62, 0x5e, 0x8e, 0x02, 0xf0, 0xa0, 0xa6, 0x2c,
                0x31, 0xd0, 0xe6, 0x00,
            ],
            Core::RightPadLow16_64 => [
                0x6d, 0x14, 0x1c, 0xd9, 0xc8, 0xdc, 0xfc, 0xef, 0xb1, 0xcf, 0x85, 0x3b, 0xf3, 0x72,
                0x78, 0xed, 0xe8, 0xef, 0x3c, 0x55, 0x8f, 0x39, 0xed, 0xeb, 0xbc, 0xaf, 0xe4, 0x4b,
                0x54, 0x00, 0x71, 0x50,
            ],
            Core::RightPadLow1_16 => [
                0x33, 0x10, 0x1e, 0x54, 0x87, 0xdd, 0x66, 0xc2, 0x45, 0xe8, 0x75, 0x4d, 0xfa, 0x16,
                0x6a, 0x06, 0xce, 0x53, 0xc5, 0xb5, 0x4f, 0xc6, 0xd8, 0x10, 0x0e, 0x2a, 0xec, 0x6c,
                0xfe, 0x93, 0x9b, 0xc9,
            ],
            Core::RightPadLow1_32 => [
                0x5f, 0xa4, 0x98, 0xc3, 0xb7, 0xc6, 0xd3, 0x48, 0xa4, 0x22, 0xb8, 0xca, 0x4f, 0x47,
                0xd0, 0x69, 0x55, 0x2a, 0xdb, 0xe2, 0xbc, 0x65, 0x59, 0xdf, 0x18, 0x68, 0x43, 0x43,
                0x87, 0x48, 0x43, 0xd3,
            ],
            Core::RightPadLow1_64 => [
                0x07, 0xbb, 0x6d, 0x70, 0xc5, 0x35, 0x4f, 0x44, 0x51, 0xfb, 0x06, 0x1b, 0x37, 0xe9,
                0xef, 0xda, 0x7e, 0x83, 0x6e, 0xfd, 0x45, 0x06, 0x14, 0x05, 0xa5, 0xea, 0x1a, 0x2f,
                0xf3, 0x15, 0x83, 0x06,
            ],
            Core::RightPadLow1_8 => [
                0xcd, 0x72, 0x15, 0xd3, 0x9b, 0x80, 0xb1, 0x8b, 0x6e, 0x78, 0xec, 0x56, 0x8b, 0x48,
                0x82, 0x58, 0x9f, 0x82, 0x71, 0x04, 0xd6, 0x85, 0xb0, 0xd4, 0xec, 0x68, 0x9d, 0x10,
                0x59, 0x75, 0x71, 0x22,
            ],
            Core::RightPadLow32_64 => [
                0x00, 0x78, 0xa0, 0x7c, 0x1a, 0x2b, 0xa8, 0x48, 0x6f, 0x16, 0x05, 0xd7, 0xfa, 0x54,
                0xb3, 0x77, 0xe8, 0x4c, 0xb6, 0xdd, 0xc5, 0xe4, 0xc0, 0x3d, 0xaf, 0x98, 0xfb, 0xa9,
                0x0f, 0xd2, 0xf6, 0x62,
            ],
            Core::RightPadLow8_16 => [
                0xc9, 0x04, 0x8e, 0x29, 0xac, 0xd7, 0x26, 0x74, 0x03, 0x00, 0xc9, 0xa6, 0xcf, 0xf1,
                0x17, 0xc0, 0xec, 0x77, 0x09, 0x0d, 0xa5, 0xae, 0xed, 0xf9, 0x70, 0x3f, 0x9a, 0xff,
                0xc9, 0x1d, 0x9a, 0xb7,
            ],
            Core::RightPadLow8_32 => [
                0x51, 0xe7, 0xf9, 0x34, 0xf0, 0x6e, 0xc1, 0xc9, 0x49, 0x91, 0x86, 0xba, 0xa5, 0x18,
                0x0a, 0x9d, 0xed, 0x56, 0x6d, 0x58, 0xb4, 0x2a, 0xb2, 0xf7, 0x0f, 0xf3, 0x6a, 0x4e,
                0x88, 0xba, 0x09, 0x78,
            ],
            Core::RightPadLow8_64 => [
                0x48, 0xcd, 0x20, 0xc3, 0x8c, 0x1e, 0xa9, 0xa0, 0xab, 0x74, 0xd5, 0x1d, 0xf4, 0x85,
                0xef, 0x63, 0x90, 0x4b, 0x8f, 0x9a, 0x09, 0xef, 0xfd, 0x98, 0xfe, 0xba, 0xcd, 0x17,
                0x17, 0xf1, 0x8a, 0xd8,
            ],
            Core::RightRotate16 => [
                0x65, 0xad, 0x75, 0x5e, 0xe8, 0x09, 0xc7, 0x27, 0x6b, 0xc0, 0x9d, 0xe1, 0x9b, 0x67,
                0xd1, 0x96, 0x9d, 0xe3, 0x2c, 0x53, 0xb3, 0xa1, 0x01, 0x21, 0xaf, 0x90, 0x56, 0x56,
                0x6a, 0x3c, 0xe8, 0xef,
            ],
            Core::RightRotate32 => [
                0xd8, 0x13, 0x63, 0x7c, 0xba, 0x61, 0x09, 0x4e, 0xb5, 0xdc, 0x58, 0xf9, 0x32, 0x59,
                0xac, 0x3b, 0xda, 0x0a, 0xf2, 0x9c, 0x3d, 0xcb, 0x05, 0x51, 0xe1, 0x45, 0x20, 0x3e,
                0x0b, 0x8d, 0xfe, 0x20,
            ],
            Core::RightRotate64 => [
                0x7b, 0x8d, 0xf8, 0xb8, 0x85, 0x7e, 0xcf, 0xb4, 0xac, 0xc1, 0x83, 0xa3, 0x55, 0xd3,
                0x63, 0x0a, 0x9c, 0x64, 0x5f, 0x9d, 0xc4, 0x05, 0xe8, 0x87, 0x06, 0x61, 0x93, 0xcb,
                0xcd, 0xa4, 0x4a, 0x2e,
            ],
            Core::RightRotate8 => [
                0x89, 0x28, 0x2b, 0x3e, 0x43, 0x0b, 0xba, 0xd5, 0xae, 0x7b, 0x1a, 0xd7, 0x4d, 0x10,
                0x61, 0x4c, 0xed, 0xfd, 0xc8, 0x01, 0xbf, 0xa3, 0xce, 0x17, 0x5f, 0x9e, 0x08, 0xbf,
                0x40, 0x31, 0xdd, 0x8e,
            ],
            Core::RightShift16 => [
                0x86, 0x04, 0x0b, 0x4b, 0xdf, 0x11, 0x80, 0x7f, 0xb1, 0x0d, 0xc7, 0xca, 0x98, 0xf8,
                0x3e, 0x53, 0x02, 0xc5, 0x84, 0xc9, 0x26, 0x71, 0xd9, 0xa8, 0xca, 0xdd, 0xf7, 0x78,
                0x0b, 0xd6, 0x08, 0x5f,
            ],
            Core::RightShift32 => [
                0xd1, 0xd0, 0xcc, 0xdb, 0xcc, 0x11, 0xa0, 0xe0, 0x0a, 0xf9, 0x3e, 0x7a, 0xe5, 0xbb,
                0xe2, 0x7f, 0x9d, 0x7f, 0xa7, 0xf2, 0xc2, 0x51, 0xf6, 0x03, 0x35, 0x24, 0x5f, 0xa0,
                0x0c, 0x5a, 0xb9, 0xfc,
            ],
            Core::RightShift64 => [
                0x51, 0x7b, 0xa7, 0xcc, 0xb3, 0xab, 0x80, 0x2c, 0x83, 0xd7, 0x06, 0x8c, 0x3c, 0x87,
                0x1c, 0x17, 0xe2, 0x04, 0xff, 0x0e, 0xea, 0x02, 0xab, 0x42, 0x01, 0x81, 0x73, 0x3c,
                0x79, 0xa4, 0x2b, 0xb6,
            ],
            Core::RightShift8 => [
                0x0f, 0x29, 0x8a, 0x1a, 0x90, 0xd1, 0xf3, 0x84, 0xc2, 0x45, 0xaf, 0x0b, 0xf1, 0x72,
                0x64, 0x2c, 0x51, 0xf5, 0x1d, 0x47, 0x5a, 0x41, 0x47, 0xdc, 0xe8, 0xc7, 0x21, 0x68,
                0xe7, 0x7f, 0x0c, 0xb2,
            ],
            Core::RightShiftWith16 => [
                0xcf, 0x05, 0xf8, 0x42, 0x11, 0x0f, 0x98, 0x19, 0xa5, 0x34, 0x5c, 0x23, 0x2b, 0xeb,
                0x47, 0x2b, 0x9c, 0x97, 0x69, 0x84, 0xa4, 0x14, 0x70, 0xbe, 0xfc, 0x0b, 0x02, 0x5d,
                0x46, 0xd0, 0x42, 0x71,
            ],
            Core::RightShiftWith32 => [
                0x04, 0x04, 0x41, 0x95, 0x69, 0xd5, 0x7f, 0xe3, 0x89, 0x32, 0x30, 0xdb, 0x5f, 0xa0,
                0xd8, 0xf9, 0x47, 0x30, 0x23, 0xa4, 0x2e, 0x01, 0xc2, 0x04, 0x93, 0x1c, 0xd7, 0x34,
                0xfb, 0x44, 0xa9, 0xf8,
            ],
            Core::RightShiftWith64 => [
                0x77, 0x26, 0x7b, 0x05, 0x1e, 0x5a, 0x10, 0x39, 0x11, 0x85, 0x90, 0x3b, 0x1a, 0x98,
                0xde, 0x32, 0xfc, 0xf2, 0x4e, 0x6a, 0xe6, 0x87, 0x1c, 0x6f, 0x15, 0xb6, 0x86, 0x52,
                0xb2, 0x4a, 0x66, 0xc4,
            ],
            Core::RightShiftWith8 => [
                0xa9, 0xcb, 0x66, 0x14, 0xef, 0x32, 0x6a, 0x66, 0x47, 0x74, 0xd6, 0xdc, 0xe6, 0x53,
                0xe1, 0xd6, 0xc0, 0x89, 0x58, 0x34, 0x43, 0xae, 0x91, 0x7f, 0x0f, 0xb4, 0xc5, 0xc2,
                0x2b, 0x9a, 0xb4, 0x44,
            ],
            Core::Rightmost16_1 => [
                0x64, 0x8b, 0x9a, 0x0f, 0x90, 0x32, 0x28, 0xba, 0x27, 0xd9, 0x59, 0x41, 0xfd, 0x54,
                0xe9, 0x2e, 0xa7, 0xb3, 0x5f, 0xee, 0xb2, 0x8a, 0x79, 0x86, 0xb6, 0x2d, 0xa9, 0x2d,
                0x7d, 0x51, 0x24, 0x70,
            ],
            Core::Rightmost16_2 => [
                0x27, 0x62, 0x2f, 0x1c, 0x01, 0x73, 0x62, 0x31, 0x09, 0xf9, 0x20, 0xf8, 0xcd, 0x87,
                0x66, 0xbb, 0xf6, 0x0d, 0x30, 0x6a, 0xd1, 0x98, 0x3f, 0xdb, 0xc1, 0x46, 0x1d, 0x23,
                0xa4, 0x1f, 0xa9, 0x44,
            ],
            Core::Rightmost16_4 => [
                0x79, 0x87, 0x86, 0x8f, 0xf3, 0xfe, 0x7f, 0x44, 0x68, 0xa3, 0x15, 0x0e, 0x3a, 0x42,
                0xcd, 0x0f, 0x32, 0x5d, 0x48, 0x92, 0x81, 0x89, 0xa4, 0x85, 0xaa, 0x62, 0xc0, 0x13,
                0x94, 0xc4, 0xe8, 0x34,
            ],
            Core::Rightmost16_8 => [
                0xef, 0x85, 0xcf, 0x36, 0xb7, 0x9d, 0x9c, 0xfa, 0xed, 0x2c, 0x63, 0x3d, 0x75, 0x1e,
                0x2e, 0x38, 0xea, 0xae, 0x5a, 0xc0, 0x21, 0x05, 0x0b, 0x13, 0x47, 0x29, 0xb9, 0xdb,
                0x51, 0xbd, 0x55, 0xa3,
            ],
            Core::Rightmost32_1 => [
                0x07, 0x38, 0xe6, 0x9e, 0xbf, 0xcf, 0x85, 0xd8, 0xe0, 0xdb, 0x90, 0x66, 0xc2, 0x0a,
                0xfc, 0x0a, 0x0a, 0x62, 0xd5, 0x4d, 0xb8, 0xec, 0x35, 0x7a, 0xce, 0x45, 0xba, 0x4d,
                0x41, 0x55, 0xd0, 0x49,
            ],
            Core::Rightmost32_16 => [
                0x08, 0xc1, 0x2f, 0xf6, 0x30, 0x90, 0x6c, 0xdc, 0x05, 0xd3, 0x17, 0x16, 0x04, 0x5d,
                0x96, 0x2a, 0x6c, 0xb3, 0x95, 0x2c, 0x89, 0xae, 0xb6, 0xc7, 0x1e, 0x47, 0xfb, 0xa6,
                0x84, 0x5a, 0x4d, 0x8a,
            ],
            Core::Rightmost32_2 => [
                0xf7, 0x55, 0xdf, 0x38, 0x97, 0x22, 0xae, 0x31, 0x23, 0x88, 0x4d, 0x9f, 0x03, 0xf2,
                0x60, 0xf1, 0x2a, 0xb2, 0x2a, 0x35, 0x32, 0x03, 0xd3, 0xcd, 0xfe, 0x19, 0x83, 0x08,
                0x21, 0x81, 0x03, 0x4b,
            ],
            Core::Rightmost32_4 => [
                0x39, 0x60, 0x04, 0xe0, 0xf7, 0x2c, 0xc2, 0x47, 0x58, 0x5a, 0x6b, 0xf9, 0x17, 0xf7,
                0x78, 0x1a, 0xae, 0xb1, 0xb6, 0x2a, 0x87, 0x76, 0x20, 0x58, 0x72, 0x26, 0x7e, 0x4f,
                0x18, 0xf0, 0xa7, 0x75,
            ],
            Core::Rightmost32_8 => [
                0x41, 0x93, 0x6e, 0xa9, 0x76, 0x9d, 0xe1, 0xf7, 0x86, 0x6c, 0xd5, 0xd0, 0x68, 0xac,
                0x6c, 0x2e, 0xa7, 0x47, 0x0a, 0x39, 0x39, 0xc2, 0xf1, 0x7d, 0x34, 0xfe, 0x91, 0xaf,
                0xc6, 0x8b, 0x1f, 0x3a,
            ],
            Core::Rightmost64_1 => [
                0x45, 0x06, 0xf2, 0x04, 0xb3, 0xd8, 0xc9, 0xa6, 0x25, 0xfa, 0x9e, 0x8f, 0xe6, 0xcc,
                0x14, 0xba, 0xe7, 0x1d, 0x61, 0xa2, 0x3c, 0xc4, 0x6a, 0xd5, 0xca, 0x25, 0xc6, 0x7a,
                0x01, 0x9d, 0x9d, 0x4b,
            ],
            Core::Rightmost64_16 => [
                0x8a, 0xf7, 0x0d, 0x2b, 0x92, 0xa7, 0xd0, 0x04, 0xdb, 0x8d, 0x79, 0x6e, 0xc1, 0x2b,
                0x75, 0xe4, 0xe4, 0x4d, 0x92, 0x62, 0x82, 0x32, 0x57, 0xca, 0xb3, 0xcb, 0x45, 0xfb,
                0x72, 0x2a, 0x9b, 0x9b,
            ],
            Core::Rightmost64_2 => [
                0xb6, 0xcb, 0xca, 0xd7, 0x76, 0xfc, 0xa4, 0xf7, 0x7d, 0x5a, 0x7c, 0xf0, 0x85, 0x00,
                0xce, 0xb2, 0xca, 0xfc, 0xdc, 0xe1, 0xe3, 0x3f, 0xae, 0x08, 0x75, 0x29, 0xf5, 0xa1,
                0x55, 0x41, 0xbd, 0x98,
            ],
            Core::Rightmost64_32 => [
                0x7f, 0xb0, 0x85, 0x18, 0xd5, 0xcc, 0x0c, 0x38, 0x0e, 0xa4, 0x55, 0xbf, 0xb2, 0xf4,
                0x5f, 0x2b, 0x55, 0xf5, 0x34, 0x68, 0x10, 0x9c, 0x36, 0x5e, 0xac, 0xff, 0x1f, 0x09,
                0x52, 0x2c, 0x6d, 0x51,
            ],
            Core::Rightmost64_4 => [
                0xdb, 0x61, 0xfd, 0xe7, 0xa4, 0x1c, 0xbc, 0x68, 0x77, 0xd2, 0xe6, 0x4e, 0x5c, 0x5f,
                0x86, 0xe2, 0x67, 0xce, 0x84, 0x3c, 0xc2, 0xab, 0x52, 0xf9, 0x1a, 0x4a, 0x6d, 0x4c,
                0x64, 0xc6, 0xf4, 0xbf,
            ],
            Core::Rightmost64_8 => [
                0x05, 0x66, 0x65, 0xd9, 0x3f, 0xbe, 0x67, 0xe7, 0x13, 0x7d, 0x40, 0xe7, 0x50, 0x00,
                0x82, 0x8d, 0xe2, 0x54, 0x75, 0x7d, 0x47, 0xe7, 0x54, 0xc5, 0x7e, 0x2e, 0x29, 0x27,
                0x46, 0xb3, 0xd6, 0xe3,
            ],
            Core::Rightmost8_1 => [
                0x3d, 0x31, 0xef, 0x3d, 0x18, 0xb6, 0xaf, 0x64, 0x10, 0x46, 0xd0, 0xdf, 0xe4, 0x58,
                0xb6, 0xda, 0x92, 0x13, 0xab, 0x54, 0xaa, 0xd8, 0xab, 0x1f, 0x48, 0x02, 0x33, 0xe9,
                0x30, 0xcc, 0x77, 0x4b,
            ],
            Core::Rightmost8_2 => [
                0x25, 0xc2, 0xa1, 0xbe, 0xbc, 0x03, 0xaa, 0x9d, 0x1e, 0x26, 0x61, 0x0e, 0x58, 0x77,
                0xad, 0x7a, 0xb2, 0xf7, 0xa4, 0xa2, 0x4b, 0xe7, 0x68, 0x6c, 0x49, 0x78, 0xe5, 0xbd,
                0x21, 0x97, 0xe8, 0xbd,
            ],
            Core::Rightmost8_4 => [
                0x43, 0xcc, 0x17, 0xf6, 0xad, 0x5d, 0x42, 0x20, 0x53, 0xf3, 0x51, 0x9e, 0x0a, 0xa2,
                0x26, 0xee, 0x06, 0xc5, 0x7c, 0xed, 0x8d, 0x05, 0x31, 0xec, 0x83, 0x94, 0xaf, 0x29,
                0x05, 0xc1, 0x44, 0xa0,
            ],
            Core::ScalarAdd => [
                0x21, 0xd0, 0x3b, 0x80, 0x63, 0x0a, 0x82, 0x5a, 0x2a, 0xcc, 0x13, 0x3d, 0x24, 0x08,
                0xfd, 0xc7, 0x71, 0x8e, 0x42, 0x52, 0x13, 0xa9, 0x2f, 0x95, 0xff, 0x97, 0x62, 0xb8,
                0xc5, 0xe9, 0x3f, 0x64,
            ],
            Core::ScalarInvert => [
                0x32, 0x77, 0x4c, 0x56, 0x7a, 0x8a, 0xef, 0xee, 0xd0, 0xef, 0xa7, 0x4c, 0xbb, 0xb6,
                0xaf, 0x27, 0x34, 0xa2, 0xb1, 0x1b, 0x14, 0x6a, 0xbe, 0x92, 0x06, 0x7c, 0x9f, 0xab,
                0xa6, 0x90, 0xb6, 0x85,
            ],
            Core::ScalarIsZero => [
                0x7b, 0x9b, 0xe7, 0x66, 0x49, 0x08, 0x90, 0x91, 0x68, 0xe3, 0xea, 0x60, 0xbd, 0x10,
                0x61, 0xec, 0x9a, 0x31, 0xe9, 0xa1, 0xd4, 0x76, 0x88, 0xba, 0x12, 0x7a, 0x39, 0xc1,
                0xc5, 0x8d, 0xe4, 0xfb,
            ],
            Core::ScalarMultiply => [
                0x25, 0xef, 0xe6, 0x40, 0x18, 0x6b, 0xc5, 0xea, 0x9f, 0x0e, 0x5d, 0xc8, 0x02, 0xd6,
                0xd2, 0xfe, 0x1c, 0x77, 0x75, 0x0f, 0xaa, 0x02, 0x4e, 0x07, 0x1b, 0xd8, 0x32, 0xdb,
                0xe6, 0x79, 0x02, 0x4b,
            ],
            Core::ScalarMultiplyLambda => [
                0x23, 0x2b, 0x9f, 0xcf, 0x9d, 0x85, 0xc1, 0xa5, 0x11, 0x62, 0xa3, 0xea, 0x55, 0x08,
                0x16, 0x31, 0x5c, 0x8e, 0x77, 0xe4, 0x64, 0x03, 0xac, 0xff, 0xa7, 0xfe, 0x6d, 0xbb,
                0x1f, 0x25, 0x01, 0x1a,
            ],
            Core::ScalarNegate => [
                0x3a, 0x63, 0x32, 0x9d, 0x66, 0x45, 0xa7, 0xc0, 0x4e, 0x7f, 0x99, 0xd9, 0xca, 0x76,
                0x29, 0x2c, 0xb5, 0x7b, 0xea, 0x2f, 0x9f, 0x90, 0x84, 0xf3, 0x05, 0xe0, 0x58, 0x3e,
                0x45, 0x5b, 0x82, 0xee,
            ],
            Core::ScalarNormalize => [
                0x27, 0xef, 0x9f, 0x9c, 0x0a, 0x5e, 0xd2, 0xfc, 0xd7, 0xa2, 0x84, 0xd1, 0x4c, 0x51,
                0x16, 0xaf, 0xab, 0x70, 0xe4, 0x89, 0x0f, 0x38, 0x1c, 0xc2, 0xda, 0x88, 0xa5, 0xf3,
                0xb5, 0x0e, 0x43, 0xc9,
            ],
            Core::ScalarSquare => [
                0xd9, 0xde, 0x19, 0x19, 0xcd, 0x12, 0xd2, 0x11, 0x70, 0x3f, 0xbb, 0xd0, 0x4e, 0x20,
                0x01, 0xd5, 0x0d, 0x29, 0x36, 0x06, 0x78, 0x64, 0x12, 0xa8, 0xd6, 0xdf, 0x36, 0xdc,
                0x1f, 0x97, 0xe3, 0xbe,
            ],
            Core::Scale => [
                0xbd, 0x69, 0x5b, 0xe1, 0x8e, 0xf7, 0x75, 0xf1, 0x2d, 0x6e, 0xe8, 0xe8, 0xe0, 0x8b,
                0xbb, 0x6b, 0xa9, 0xfb, 0xff, 0xc8, 0x75, 0x24, 0x70, 0x72, 0x5e, 0x8b, 0x06, 0x11,
                0x0d, 0x8c, 0x5a, 0x56,
            ],
            Core::Sha256Block => [
                0x97, 0xbb, 0x59, 0xb7, 0x93, 0xae, 0x9c, 0xe4, 0x9d, 0xc0, 0x78, 0x13, 0x4b, 0x6a,
                0x9e, 0x85, 0x5a, 0xcc, 0x6d, 0x50, 0xc0, 0xe5, 0xbd, 0x3f, 0xa6, 0x71, 0xce, 0xbe,
                0xea, 0xb0, 0x7a, 0xe2,
            ],
            Core::Sha256Ctx8Add1 => [
                0x02, 0x7b, 0x6e, 0xec, 0xd2, 0xb3, 0x82, 0x29, 0x2c, 0x5e, 0xa4, 0x65, 0x29, 0x40,
                0x8b, 0x01, 0xb2, 0xd4, 0xc7, 0x39, 0xd9, 0x11, 0x60, 0x1d, 0x02, 0xc6, 0xbf, 0x3a,
                0x18, 0x6b, 0xb7, 0xdf,
            ],
            Core::Sha256Ctx8Add128 => [
                0xe7, 0xe5, 0xe3, 0xb3, 0x94, 0x4a, 0x31, 0x1b, 0xd4, 0x1a, 0x06, 0x0d, 0x55, 0xe1,
                0x7b, 0x08, 0xb5, 0xff, 0x75, 0x02, 0x08, 0x52, 0xe6, 0x6b, 0xda, 0x62, 0xb6, 0xdf,
                0x94, 0x3b, 0xb8, 0xd4,
            ],
            Core::Sha256Ctx8Add16 => [
                0x77, 0x87, 0x22, 0x34, 0x3b, 0x78, 0xcf, 0xb7, 0x7f, 0xd5, 0xe7, 0xd5, 0xb1, 0x9e,
                0x93, 0x12, 0xe9, 0x97, 0xbb, 0xa3, 0x52, 0xee, 0xcb, 0x39, 0xa3, 0xc3, 0x32, 0xff,
                0x48, 0x09, 0xaa, 0x14,
            ],
            Core::Sha256Ctx8Add2 => [
                0x72, 0xaa, 0x6d, 0xe3, 0x4c, 0x20, 0x72, 0xe6, 0x92, 0x3b, 0x22, 0x80, 0xb7, 0xcf,
                0x52, 0x1e, 0x1e, 0x05, 0x7d, 0x6b, 0xec, 0xdd, 0xa7, 0x75, 0x9a, 0x79, 0xac, 0x18,
                0x8a, 0x26, 0x2e, 0x32,
            ],
            Core::Sha256Ctx8Add256 => [
                0xa4, 0x88, 0xeb, 0x1e, 0x75, 0x85, 0xcd, 0x02, 0x27, 0xad, 0xbb, 0xc8, 0x54, 0x9c,
                0x9f, 0xeb, 0x65, 0x87, 0xf7, 0x8b, 0x43, 0x92, 0x2b, 0x57, 0x44, 0x01, 0x5f, 0x1b,
                0xb6, 0x21, 0xda, 0xa0,
            ],
            Core::Sha256Ctx8Add32 => [
                0x50, 0x38, 0x33, 0x5e, 0xd7, 0x69, 0x56, 0xf3, 0x45, 0x1e, 0x68, 0x78, 0x10, 0x02,
                0xe9, 0x63, 0xa8, 0x4d, 0xd6, 0x55, 0xcb, 0xf2, 0xb8, 0x41, 0x63, 0x26, 0x91, 0x14,
                0x77, 0xae, 0x3c, 0xa2,
            ],
            Core::Sha256Ctx8Add4 => [
                0x23, 0xb6, 0x9b, 0x90, 0xe6, 0x9e, 0xe5, 0x0f, 0x91, 0x67, 0x04, 0x6e, 0x89, 0x99,
                0xe3, 0xb9, 0x10, 0xc6, 0x54, 0xa7, 0x97, 0x91, 0xd3, 0xbd, 0x0e, 0x0e, 0x7f, 0x81,
                0x54, 0x2b, 0x9d, 0x36,
            ],
            Core::Sha256Ctx8Add512 => [
                0xf7, 0x57, 0xd6, 0x9a, 0x8d, 0xb2, 0xc4, 0xba, 0x74, 0x46, 0xf9, 0x6a, 0x81, 0x4d,
                0x74, 0x61, 0xf4, 0xb3, 0xd0, 0x87, 0xc0, 0xe4, 0x25, 0x46, 0x8d, 0x72, 0x3d, 0x14,
                0x85, 0xbc, 0x85, 0x1b,
            ],
            Core::Sha256Ctx8Add64 => [
                0x9a, 0x17, 0x9b, 0xbd, 0x8a, 0x03, 0x29, 0x37, 0xe5, 0x90, 0xff, 0xbd, 0xb3, 0x1a,
                0x28, 0x47, 0xa6, 0xf6, 0x9f, 0x9b, 0xb4, 0xdd, 0xcd, 0x2e, 0x60, 0x6e, 0xdd, 0x45,
                0x4a, 0xd8, 0x40, 0x31,
            ],
            Core::Sha256Ctx8Add8 => [
                0x77, 0xb2, 0xeb, 0x5e, 0x6f, 0xa0, 0x3a, 0xac, 0x09, 0x86, 0x07, 0xc8, 0x3d, 0x10,
                0x61, 0x76, 0xbd, 0xf3, 0x3c, 0x8c, 0xbb, 0xa7, 0xb9, 0x2d, 0x27, 0xe0, 0x35, 0x2a,
                0x2f, 0x1e, 0xd3, 0xbe,
            ],
            Core::Sha256Ctx8AddBuffer511 => [
                0xd4, 0x7b, 0xb1, 0xcb, 0x4c, 0xaa, 0xff, 0x17, 0x41, 0x2a, 0x73, 0x0d, 0xd9, 0x12,
                0xdb, 0xb3, 0x7d, 0xcc, 0xc9, 0x0b, 0x27, 0xd3, 0x95, 0xc4, 0xf8, 0x45, 0x90, 0x08,
                0xa5, 0xf2, 0x44, 0xc7,
            ],
            Core::Sha256Ctx8Finalize => [
                0x71, 0x65, 0x80, 0xc1, 0xe0, 0x0d, 0x6a, 0x66, 0x1f, 0xd9, 0xbe, 0x6b, 0x61, 0xde,
                0x5b, 0xef, 0x63, 0xa8, 0x4a, 0x4e, 0x67, 0x5b, 0xeb, 0x0b, 0x0a, 0x47, 0x19, 0x73,
                0x5e, 0xa3, 0xa2, 0x85,
            ],
            Core::Sha256Ctx8Init => [
                0x82, 0xbb, 0x12, 0xfe, 0x68, 0x8d, 0x2a, 0xe3, 0x7e, 0x30, 0x18, 0x93, 0xa3, 0xf0,
                0xd3, 0x07, 0x5c, 0xbf, 0x99, 0x34, 0xd0, 0x75, 0xf7, 0x3c, 0x2b, 0x36, 0x4a, 0xf3,
                0xd1, 0x12, 0x10, 0x2a,
            ],
            Core::Sha256Iv => [
                0xd2, 0xbb, 0x57, 0x20, 0x0d, 0xad, 0xd3, 0xa6, 0x73, 0x78, 0xdb, 0x79, 0x66, 0x10,
                0xf5, 0x9c, 0x33, 0xd9, 0xa4, 0x0d, 0x2d, 0xfc, 0xc9, 0x9f, 0x03, 0xb0, 0x8b, 0x71,
                0x55, 0xfb, 0xbb, 0xea,
            ],
            Core::Some1 => [
                0x15, 0x1b, 0xcb, 0x76, 0xc2, 0x31, 0xab, 0x25, 0xe3, 0x71, 0x09, 0x28, 0xd9, 0xb3,
                0xb7, 0xa2, 0xe1, 0x7f, 0x84, 0xd7, 0xcf, 0x8d, 0xa8, 0x84, 0x5c, 0xf8, 0x7a, 0x40,
                0x81, 0x6f, 0x31, 0x68,
            ],
            Core::Some16 => [
                0x87, 0x29, 0x5e, 0x8a, 0x0f, 0x02, 0x99, 0xf3, 0xa7, 0xcb, 0x73, 0x8d, 0xd5, 0xf8,
                0xdd, 0xec, 0xc3, 0xe4, 0x8d, 0x18, 0x74, 0xa5, 0x5a, 0x0c, 0xcf, 0x7c, 0x2d, 0x30,
                0xaa, 0xd0, 0x79, 0x87,
            ],
            Core::Some32 => [
                0x85, 0x2f, 0x5e, 0x22, 0x24, 0x66, 0x77, 0xc4, 0x9a, 0x6c, 0x68, 0xec, 0x39, 0x73,
                0xcb, 0x53, 0x00, 0x82, 0x65, 0xcf, 0x17, 0xd4, 0x6f, 0x60, 0x00, 0x59, 0x41, 0xba,
                0x57, 0xd4, 0x94, 0x7e,
            ],
            Core::Some64 => [
                0xfe, 0x9b, 0x3a, 0x25, 0xc9, 0x57, 0x17, 0x49, 0xe2, 0xa5, 0x63, 0xf4, 0x10, 0xa5,
                0x65, 0x0f, 0x41, 0x17, 0x4e, 0x23, 0x26, 0x91, 0x07, 0x2b, 0x2d, 0x54, 0x74, 0xf9,
                0x3c, 0x7a, 0x22, 0xc0,
            ],
            Core::Some8 => [
                0x49, 0xdc, 0x24, 0x20, 0x4c, 0x56, 0x44, 0xea, 0x98, 0xb0, 0xd8, 0xa8, 0xa1, 0xf1,
                0x5f, 0x20, 0x8b, 0x4a, 0xfa, 0xd5, 0x9d, 0x89, 0x95, 0xab, 0x96, 0xc4, 0x49, 0x8d,
                0x1c, 0xd4, 0x10, 0xa0,
            ],
            Core::Subtract16 => [
                0x51, 0x2b, 0x7c, 0x68, 0x81, 0xbf, 0x8d, 0x8f, 0xa5, 0xd9, 0x35, 0xe5, 0x25, 0x64,
                0xe2, 0x86, 0x75, 0x1f, 0x77, 0xe6, 0x33, 0x75, 0x63, 0x7d, 0xf2, 0xd7, 0x22, 0x75,
                0x68, 0xc6, 0x94, 0x7f,
            ],
            Core::Subtract32 => [
                0xf5, 0x3f, 0x3e, 0x87, 0x1a, 0x7d, 0xa3, 0xf0, 0x9e, 0x56, 0x26, 0x97, 0x83, 0x5e,
                0x1c, 0xe1, 0x0d, 0xf8, 0xee, 0x14, 0x11, 0x63, 0xad, 0x93, 0xb5, 0x7c, 0x8e, 0x6d,
                0x74, 0x38, 0x2b, 0xbc,
            ],
            Core::Subtract64 => [
                0x5f, 0x08, 0xf7, 0x40, 0x53, 0xb2, 0xf0, 0x19, 0xb6, 0x2a, 0x1b, 0xb8, 0x28, 0xb7,
                0x99, 0xe0, 0x91, 0xe8, 0x21, 0x7e, 0xc8, 0x89, 0xdd, 0x4d, 0x37, 0x6e, 0x01, 0x2c,
                0x95, 0x79, 0xb9, 0x81,
            ],
            Core::Subtract8 => [
                0x77, 0x86, 0x49, 0xf8, 0x39, 0x35, 0xcd, 0x3f, 0xfc, 0x04, 0xf6, 0xc6, 0x62, 0x0a,
                0x33, 0x96, 0x5d, 0x0e, 0xf1, 0xdd, 0x17, 0xe7, 0xb5, 0x0f, 0xa4, 0x10, 0x76, 0xd8,
                0xad, 0x87, 0x6b, 0x1d,
            ],
            Core::Swu => [
                0x20, 0x49, 0xd4, 0x6c, 0x4c, 0x4d, 0x6f, 0x85, 0x6e, 0x04, 0xf7, 0xae, 0x20, 0x28,
                0x73, 0x36, 0xc2, 0xf2, 0xa6, 0xb8, 0xc1, 0xe2, 0x9d, 0x7e, 0xbb, 0xa3, 0xaf, 0x2f,
                0xd3, 0xa8, 0x80, 0x9b,
            ],
            Core::Verify => [
                0x22, 0xc0, 0xe3, 0x62, 0x34, 0x19, 0x0a, 0xf6, 0xc8, 0x16, 0x1e, 0x41, 0xf9, 0xe0,
                0x00, 0x13, 0xb2, 0x43, 0xc8, 0x96, 0x77, 0x69, 0x1a, 0x62, 0xe7, 0x98, 0x72, 0xfa,
                0x03, 0xbf, 0xa6, 0x77,
            ],
            Core::Xor1 => [
                0x7b, 0x76, 0x0e, 0xe0, 0xc7, 0x89, 0x6a, 0xb7, 0x38, 0xcc, 0x50, 0xef, 0x7d, 0x63,
                0xf6, 0xd2, 0xbf, 0x59, 0x61, 0xa9, 0x28, 0x64, 0x47, 0xce, 0x06, 0x4e, 0xc1, 0x8d,
                0x5e, 0xde, 0x6f, 0x6c,
            ],
            Core::Xor16 => [
                0xef, 0x2d, 0x8a, 0x0f, 0x66, 0x9d, 0x20, 0xaa, 0xfe, 0x11, 0xa6, 0xc3, 0x39, 0x06,
                0x36, 0x92, 0x3a, 0xd3, 0x76, 0x14, 0x0d, 0x3e, 0xa6, 0x94, 0xd0, 0x87, 0x2f, 0xa4,
                0xd5, 0x4b, 0x90, 0xef,
            ],
            Core::Xor32 => [
                0x21, 0x9b, 0x1d, 0xd6, 0xe7, 0x2b, 0x77, 0x75, 0x19, 0x51, 0x91, 0x6e, 0xe4, 0xdd,
                0xfd, 0x56, 0xac, 0xf0, 0xce, 0x7a, 0x30, 0xa4, 0xcd, 0x73, 0x9c, 0x51, 0xbd, 0x99,
                0x42, 0xa2, 0xeb, 0x07,
            ],
            Core::Xor64 => [
                0x0b, 0xbb, 0x85, 0x0a, 0xd0, 0x3f, 0x9c, 0x96, 0xa5, 0xd4, 0x67, 0xc4, 0x61, 0x12,
                0xf1, 0x04, 0x6a, 0x7a, 0x92, 0x27, 0x3f, 0x2b, 0xb0, 0x1d, 0x20, 0xca, 0x4d, 0xb6,
                0xba, 0x75, 0x3e, 0x79,
            ],
            Core::Xor8 => [
                0x95, 0x0b, 0x65, 0x31, 0x62, 0x36, 0xe8, 0x3e, 0xd1, 0x4b, 0xd1, 0x71, 0xe3, 0x4d,
                0x55, 0xaf, 0x0a, 0x3f, 0x63, 0x7c, 0xc4, 0x7d, 0xc7, 0x17, 0xf7, 0x7f, 0xdb, 0x4b,
                0x30, 0xdf, 0x01, 0x57,
            ],
            Core::XorXor1 => [
                0x3f, 0x30, 0xf9, 0x59, 0x87, 0x0d, 0xe1, 0xd1, 0x33, 0x37, 0x4e, 0x14, 0x09, 0xfd,
                0xc3, 0x4d, 0xe4, 0x92, 0x91, 0x70, 0x68, 0x30, 0xb8, 0x3d, 0xac, 0x9d, 0x4e, 0x67,
                0xc4, 0xeb, 0x60, 0x48,
            ],
            Core::XorXor16 => [
                0x2e, 0x1a, 0x09, 0xe3, 0xcb, 0xa2, 0x75, 0x66, 0x02, 0xe8, 0x7b, 0x86, 0xf2, 0xb0,
                0x7f, 0x2d, 0x23, 0x7b, 0xfc, 0xac, 0x5b, 0x1b, 0x8a, 0xb7, 0xb5, 0x29, 0xaf, 0x42,
                0xd3, 0x11, 0x02, 0x94,
            ],
            Core::XorXor32 => [
                0x85, 0xa3, 0x79, 0xa1, 0xe5, 0xdb, 0x45, 0x72, 0x37, 0xdf, 0x27, 0xf4, 0xcf, 0xcd,
                0xc8, 0x75, 0x60, 0xff, 0xd3, 0x73, 0x8e, 0xbe, 0xc8, 0x25, 0x52, 0xe7, 0x3b, 0xce,
                0x89, 0xbc, 0x0b, 0x37,
            ],
            Core::XorXor64 => [
                0x4c, 0x12, 0x68, 0xc5, 0x75, 0x42, 0x50, 0x89, 0xbe, 0x42, 0x47, 0x2e, 0x97, 0x2b,
                0x2a, 0xe0, 0x0e, 0xde, 0x58, 0x34, 0xc9, 0x84, 0xd9, 0x6d, 0xcd, 0x59, 0xa0, 0xc2,
                0x21, 0x15, 0x2f, 0x93,
            ],
            Core::XorXor8 => [
                0x61, 0xf2, 0x74, 0x7f, 0x8d, 0x83, 0xb6, 0xa5, 0xad, 0xaa, 0x42, 0xf5, 0xc0, 0x40,
                0x8c, 0x9b, 0xd4, 0xe9, 0x9e, 0x92, 0x70, 0xa7, 0x58, 0x71, 0x40, 0x6c, 0xec, 0x44,
                0x89, 0xa5, 0xfb, 0xe7,
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
            Core::And1 => b"*22",
            Core::And16 => b"i",
            Core::And32 => b"l",
            Core::And64 => b"*ll",
            Core::And8 => b"****22*22**22*22***22*22**22*22",
            Core::Bip0340Verify => b"**hh*hh",
            Core::Ch1 => b"*2*22",
            Core::Ch16 => b"*****22*22**22*22***22*22**22*22i",
            Core::Ch32 => b"*il",
            Core::Ch64 => b"*l*ll",
            Core::Ch8 => b"****22*22**22*22****22*22**22*22***22*22**22*22",
            Core::CheckSigVerify => b"**h*hh*hh",
            Core::Complement1 => b"2",
            Core::Complement16 => b"****22*22**22*22***22*22**22*22",
            Core::Complement32 => b"i",
            Core::Complement64 => b"l",
            Core::Complement8 => b"***22*22**22*22",
            Core::Decompress => b"*2h",
            Core::Decrement16 => b"****22*22**22*22***22*22**22*22",
            Core::Decrement32 => b"i",
            Core::Decrement64 => b"l",
            Core::Decrement8 => b"***22*22**22*22",
            Core::DivMod128_64 => b"**lll",
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
            Core::Eq1 => b"*22",
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
            Core::FullLeftShift16_1 => b"*****22*22**22*22***22*22**22*222",
            Core::FullLeftShift16_2 => b"*****22*22**22*22***22*22**22*22*22",
            Core::FullLeftShift16_4 => b"*****22*22**22*22***22*22**22*22**22*22",
            Core::FullLeftShift16_8 => b"*****22*22**22*22***22*22**22*22***22*22**22*22",
            Core::FullLeftShift32_1 => b"*i2",
            Core::FullLeftShift32_16 => b"*i****22*22**22*22***22*22**22*22",
            Core::FullLeftShift32_2 => b"*i*22",
            Core::FullLeftShift32_4 => b"*i**22*22",
            Core::FullLeftShift32_8 => b"*i***22*22**22*22",
            Core::FullLeftShift64_1 => b"*l2",
            Core::FullLeftShift64_16 => b"*l****22*22**22*22***22*22**22*22",
            Core::FullLeftShift64_2 => b"*l*22",
            Core::FullLeftShift64_32 => b"*li",
            Core::FullLeftShift64_4 => b"*l**22*22",
            Core::FullLeftShift64_8 => b"*l***22*22**22*22",
            Core::FullLeftShift8_1 => b"****22*22**22*222",
            Core::FullLeftShift8_2 => b"****22*22**22*22*22",
            Core::FullLeftShift8_4 => b"****22*22**22*22**22*22",
            Core::FullMultiply16 => b"l",
            Core::FullMultiply32 => b"*ll",
            Core::FullMultiply64 => b"h",
            Core::FullMultiply8 => b"i",
            Core::FullRightShift16_1 => b"*2****22*22**22*22***22*22**22*22",
            Core::FullRightShift16_2 => b"**22****22*22**22*22***22*22**22*22",
            Core::FullRightShift16_4 => b"***22*22****22*22**22*22***22*22**22*22",
            Core::FullRightShift16_8 => b"****22*22**22*22****22*22**22*22***22*22**22*22",
            Core::FullRightShift32_1 => b"*2i",
            Core::FullRightShift32_16 => b"*****22*22**22*22***22*22**22*22i",
            Core::FullRightShift32_2 => b"**22i",
            Core::FullRightShift32_4 => b"***22*22i",
            Core::FullRightShift32_8 => b"****22*22**22*22i",
            Core::FullRightShift64_1 => b"*2l",
            Core::FullRightShift64_16 => b"*****22*22**22*22***22*22**22*22l",
            Core::FullRightShift64_2 => b"**22l",
            Core::FullRightShift64_32 => b"*il",
            Core::FullRightShift64_4 => b"***22*22l",
            Core::FullRightShift64_8 => b"****22*22**22*22l",
            Core::FullRightShift8_1 => b"*2***22*22**22*22",
            Core::FullRightShift8_2 => b"**22***22*22**22*22",
            Core::FullRightShift8_4 => b"***22*22***22*22**22*22",
            Core::FullSubtract16 => b"*2i",
            Core::FullSubtract32 => b"*2l",
            Core::FullSubtract64 => b"*2*ll",
            Core::FullSubtract8 => b"*2****22*22**22*22***22*22**22*22",
            Core::GeIsOnCurve => b"*hh",
            Core::GeNegate => b"*hh",
            Core::GejAdd => b"***hhh**hhh",
            Core::GejDouble => b"**hhh",
            Core::GejEquiv => b"***hhh**hhh",
            Core::GejGeAdd => b"***hhh*hh",
            Core::GejGeAddEx => b"***hhh*hh",
            Core::GejGeEquiv => b"***hhh*hh",
            Core::GejInfinity => b"1",
            Core::GejIsInfinity => b"**hhh",
            Core::GejIsOnCurve => b"**hhh",
            Core::GejNegate => b"**hhh",
            Core::GejNormalize => b"**hhh",
            Core::GejRescale => b"***hhhh",
            Core::GejXEquiv => b"*h**hhh",
            Core::GejYIsOdd => b"**hhh",
            Core::Generate => b"h",
            Core::HashToCurve => b"h",
            Core::High1 => b"1",
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
            Core::LeftExtend16_32 => b"****22*22**22*22***22*22**22*22",
            Core::LeftExtend16_64 => b"****22*22**22*22***22*22**22*22",
            Core::LeftExtend1_16 => b"2",
            Core::LeftExtend1_32 => b"2",
            Core::LeftExtend1_64 => b"2",
            Core::LeftExtend1_8 => b"2",
            Core::LeftExtend32_64 => b"i",
            Core::LeftExtend8_16 => b"***22*22**22*22",
            Core::LeftExtend8_32 => b"***22*22**22*22",
            Core::LeftExtend8_64 => b"***22*22**22*22",
            Core::LeftPadHigh16_32 => b"****22*22**22*22***22*22**22*22",
            Core::LeftPadHigh16_64 => b"****22*22**22*22***22*22**22*22",
            Core::LeftPadHigh1_16 => b"2",
            Core::LeftPadHigh1_32 => b"2",
            Core::LeftPadHigh1_64 => b"2",
            Core::LeftPadHigh1_8 => b"2",
            Core::LeftPadHigh32_64 => b"i",
            Core::LeftPadHigh8_16 => b"***22*22**22*22",
            Core::LeftPadHigh8_32 => b"***22*22**22*22",
            Core::LeftPadHigh8_64 => b"***22*22**22*22",
            Core::LeftPadLow16_32 => b"****22*22**22*22***22*22**22*22",
            Core::LeftPadLow16_64 => b"****22*22**22*22***22*22**22*22",
            Core::LeftPadLow1_16 => b"2",
            Core::LeftPadLow1_32 => b"2",
            Core::LeftPadLow1_64 => b"2",
            Core::LeftPadLow1_8 => b"2",
            Core::LeftPadLow32_64 => b"i",
            Core::LeftPadLow8_16 => b"***22*22**22*22",
            Core::LeftPadLow8_32 => b"***22*22**22*22",
            Core::LeftPadLow8_64 => b"***22*22**22*22",
            Core::LeftRotate16 => b"***22*22****22*22**22*22***22*22**22*22",
            Core::LeftRotate32 => b"****22*22**22*22i",
            Core::LeftRotate64 => b"****22*22**22*22l",
            Core::LeftRotate8 => b"***22*22***22*22**22*22",
            Core::LeftShift16 => b"***22*22****22*22**22*22***22*22**22*22",
            Core::LeftShift32 => b"****22*22**22*22i",
            Core::LeftShift64 => b"****22*22**22*22l",
            Core::LeftShift8 => b"***22*22***22*22**22*22",
            Core::LeftShiftWith16 => b"*2***22*22****22*22**22*22***22*22**22*22",
            Core::LeftShiftWith32 => b"*2****22*22**22*22i",
            Core::LeftShiftWith64 => b"*2****22*22**22*22l",
            Core::LeftShiftWith8 => b"*2***22*22***22*22**22*22",
            Core::Leftmost16_1 => b"****22*22**22*22***22*22**22*22",
            Core::Leftmost16_2 => b"****22*22**22*22***22*22**22*22",
            Core::Leftmost16_4 => b"****22*22**22*22***22*22**22*22",
            Core::Leftmost16_8 => b"****22*22**22*22***22*22**22*22",
            Core::Leftmost32_1 => b"i",
            Core::Leftmost32_16 => b"i",
            Core::Leftmost32_2 => b"i",
            Core::Leftmost32_4 => b"i",
            Core::Leftmost32_8 => b"i",
            Core::Leftmost64_1 => b"l",
            Core::Leftmost64_16 => b"l",
            Core::Leftmost64_2 => b"l",
            Core::Leftmost64_32 => b"l",
            Core::Leftmost64_4 => b"l",
            Core::Leftmost64_8 => b"l",
            Core::Leftmost8_1 => b"***22*22**22*22",
            Core::Leftmost8_2 => b"***22*22**22*22",
            Core::Leftmost8_4 => b"***22*22**22*22",
            Core::LinearCombination1 => b"**h**hhhh",
            Core::LinearVerify1 => b"***h*hhh*hh",
            Core::Low1 => b"1",
            Core::Low16 => b"1",
            Core::Low32 => b"1",
            Core::Low64 => b"1",
            Core::Low8 => b"1",
            Core::Lt16 => b"i",
            Core::Lt32 => b"l",
            Core::Lt64 => b"*ll",
            Core::Lt8 => b"****22*22**22*22***22*22**22*22",
            Core::Maj1 => b"*2*22",
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
            Core::Or1 => b"*22",
            Core::Or16 => b"i",
            Core::Or32 => b"l",
            Core::Or64 => b"*ll",
            Core::Or8 => b"****22*22**22*22***22*22**22*22",
            Core::ParseLock => b"i",
            Core::ParseSequence => b"i",
            Core::PointVerify1 => b"***h*2hh*2h",
            Core::RightExtend16_32 => b"****22*22**22*22***22*22**22*22",
            Core::RightExtend16_64 => b"****22*22**22*22***22*22**22*22",
            Core::RightExtend32_64 => b"i",
            Core::RightExtend8_16 => b"***22*22**22*22",
            Core::RightExtend8_32 => b"***22*22**22*22",
            Core::RightExtend8_64 => b"***22*22**22*22",
            Core::RightPadHigh16_32 => b"****22*22**22*22***22*22**22*22",
            Core::RightPadHigh16_64 => b"****22*22**22*22***22*22**22*22",
            Core::RightPadHigh1_16 => b"2",
            Core::RightPadHigh1_32 => b"2",
            Core::RightPadHigh1_64 => b"2",
            Core::RightPadHigh1_8 => b"2",
            Core::RightPadHigh32_64 => b"i",
            Core::RightPadHigh8_16 => b"***22*22**22*22",
            Core::RightPadHigh8_32 => b"***22*22**22*22",
            Core::RightPadHigh8_64 => b"***22*22**22*22",
            Core::RightPadLow16_32 => b"****22*22**22*22***22*22**22*22",
            Core::RightPadLow16_64 => b"****22*22**22*22***22*22**22*22",
            Core::RightPadLow1_16 => b"2",
            Core::RightPadLow1_32 => b"2",
            Core::RightPadLow1_64 => b"2",
            Core::RightPadLow1_8 => b"2",
            Core::RightPadLow32_64 => b"i",
            Core::RightPadLow8_16 => b"***22*22**22*22",
            Core::RightPadLow8_32 => b"***22*22**22*22",
            Core::RightPadLow8_64 => b"***22*22**22*22",
            Core::RightRotate16 => b"***22*22****22*22**22*22***22*22**22*22",
            Core::RightRotate32 => b"****22*22**22*22i",
            Core::RightRotate64 => b"****22*22**22*22l",
            Core::RightRotate8 => b"***22*22***22*22**22*22",
            Core::RightShift16 => b"***22*22****22*22**22*22***22*22**22*22",
            Core::RightShift32 => b"****22*22**22*22i",
            Core::RightShift64 => b"****22*22**22*22l",
            Core::RightShift8 => b"***22*22***22*22**22*22",
            Core::RightShiftWith16 => b"*2***22*22****22*22**22*22***22*22**22*22",
            Core::RightShiftWith32 => b"*2****22*22**22*22i",
            Core::RightShiftWith64 => b"*2****22*22**22*22l",
            Core::RightShiftWith8 => b"*2***22*22***22*22**22*22",
            Core::Rightmost16_1 => b"****22*22**22*22***22*22**22*22",
            Core::Rightmost16_2 => b"****22*22**22*22***22*22**22*22",
            Core::Rightmost16_4 => b"****22*22**22*22***22*22**22*22",
            Core::Rightmost16_8 => b"****22*22**22*22***22*22**22*22",
            Core::Rightmost32_1 => b"i",
            Core::Rightmost32_16 => b"i",
            Core::Rightmost32_2 => b"i",
            Core::Rightmost32_4 => b"i",
            Core::Rightmost32_8 => b"i",
            Core::Rightmost64_1 => b"l",
            Core::Rightmost64_16 => b"l",
            Core::Rightmost64_2 => b"l",
            Core::Rightmost64_32 => b"l",
            Core::Rightmost64_4 => b"l",
            Core::Rightmost64_8 => b"l",
            Core::Rightmost8_1 => b"***22*22**22*22",
            Core::Rightmost8_2 => b"***22*22**22*22",
            Core::Rightmost8_4 => b"***22*22**22*22",
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
            Core::Some1 => b"2",
            Core::Some16 => b"****22*22**22*22***22*22**22*22",
            Core::Some32 => b"i",
            Core::Some64 => b"l",
            Core::Some8 => b"***22*22**22*22",
            Core::Subtract16 => b"i",
            Core::Subtract32 => b"l",
            Core::Subtract64 => b"*ll",
            Core::Subtract8 => b"****22*22**22*22***22*22**22*22",
            Core::Swu => b"h",
            Core::Verify => b"2",
            Core::Xor1 => b"*22",
            Core::Xor16 => b"i",
            Core::Xor32 => b"l",
            Core::Xor64 => b"*ll",
            Core::Xor8 => b"****22*22**22*22***22*22**22*22",
            Core::XorXor1 => b"*2*22",
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
            Core::And1 => b"2",
            Core::And16 => b"****22*22**22*22***22*22**22*22",
            Core::And32 => b"i",
            Core::And64 => b"l",
            Core::And8 => b"***22*22**22*22",
            Core::Bip0340Verify => b"1",
            Core::Ch1 => b"2",
            Core::Ch16 => b"****22*22**22*22***22*22**22*22",
            Core::Ch32 => b"i",
            Core::Ch64 => b"l",
            Core::Ch8 => b"***22*22**22*22",
            Core::CheckSigVerify => b"1",
            Core::Complement1 => b"2",
            Core::Complement16 => b"****22*22**22*22***22*22**22*22",
            Core::Complement32 => b"i",
            Core::Complement64 => b"l",
            Core::Complement8 => b"***22*22**22*22",
            Core::Decompress => b"+1*hh",
            Core::Decrement16 => b"*2****22*22**22*22***22*22**22*22",
            Core::Decrement32 => b"*2i",
            Core::Decrement64 => b"*2l",
            Core::Decrement8 => b"*2***22*22**22*22",
            Core::DivMod128_64 => b"*ll",
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
            Core::Eq1 => b"2",
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
            Core::FullLeftShift16_1 => b"*2****22*22**22*22***22*22**22*22",
            Core::FullLeftShift16_2 => b"**22****22*22**22*22***22*22**22*22",
            Core::FullLeftShift16_4 => b"***22*22****22*22**22*22***22*22**22*22",
            Core::FullLeftShift16_8 => b"****22*22**22*22****22*22**22*22***22*22**22*22",
            Core::FullLeftShift32_1 => b"*2i",
            Core::FullLeftShift32_16 => b"*****22*22**22*22***22*22**22*22i",
            Core::FullLeftShift32_2 => b"**22i",
            Core::FullLeftShift32_4 => b"***22*22i",
            Core::FullLeftShift32_8 => b"****22*22**22*22i",
            Core::FullLeftShift64_1 => b"*2l",
            Core::FullLeftShift64_16 => b"*****22*22**22*22***22*22**22*22l",
            Core::FullLeftShift64_2 => b"**22l",
            Core::FullLeftShift64_32 => b"*il",
            Core::FullLeftShift64_4 => b"***22*22l",
            Core::FullLeftShift64_8 => b"****22*22**22*22l",
            Core::FullLeftShift8_1 => b"*2***22*22**22*22",
            Core::FullLeftShift8_2 => b"**22***22*22**22*22",
            Core::FullLeftShift8_4 => b"***22*22***22*22**22*22",
            Core::FullMultiply16 => b"i",
            Core::FullMultiply32 => b"l",
            Core::FullMultiply64 => b"*ll",
            Core::FullMultiply8 => b"****22*22**22*22***22*22**22*22",
            Core::FullRightShift16_1 => b"*****22*22**22*22***22*22**22*222",
            Core::FullRightShift16_2 => b"*****22*22**22*22***22*22**22*22*22",
            Core::FullRightShift16_4 => b"*****22*22**22*22***22*22**22*22**22*22",
            Core::FullRightShift16_8 => b"*****22*22**22*22***22*22**22*22***22*22**22*22",
            Core::FullRightShift32_1 => b"*i2",
            Core::FullRightShift32_16 => b"*i****22*22**22*22***22*22**22*22",
            Core::FullRightShift32_2 => b"*i*22",
            Core::FullRightShift32_4 => b"*i**22*22",
            Core::FullRightShift32_8 => b"*i***22*22**22*22",
            Core::FullRightShift64_1 => b"*l2",
            Core::FullRightShift64_16 => b"*l****22*22**22*22***22*22**22*22",
            Core::FullRightShift64_2 => b"*l*22",
            Core::FullRightShift64_32 => b"*li",
            Core::FullRightShift64_4 => b"*l**22*22",
            Core::FullRightShift64_8 => b"*l***22*22**22*22",
            Core::FullRightShift8_1 => b"****22*22**22*222",
            Core::FullRightShift8_2 => b"****22*22**22*22*22",
            Core::FullRightShift8_4 => b"****22*22**22*22**22*22",
            Core::FullSubtract16 => b"*2****22*22**22*22***22*22**22*22",
            Core::FullSubtract32 => b"*2i",
            Core::FullSubtract64 => b"*2l",
            Core::FullSubtract8 => b"*2***22*22**22*22",
            Core::GeIsOnCurve => b"2",
            Core::GeNegate => b"*hh",
            Core::GejAdd => b"**hhh",
            Core::GejDouble => b"**hhh",
            Core::GejEquiv => b"2",
            Core::GejGeAdd => b"**hhh",
            Core::GejGeAddEx => b"*h**hhh",
            Core::GejGeEquiv => b"2",
            Core::GejInfinity => b"**hhh",
            Core::GejIsInfinity => b"2",
            Core::GejIsOnCurve => b"2",
            Core::GejNegate => b"**hhh",
            Core::GejNormalize => b"+1*hh",
            Core::GejRescale => b"**hhh",
            Core::GejXEquiv => b"2",
            Core::GejYIsOdd => b"2",
            Core::Generate => b"**hhh",
            Core::HashToCurve => b"*hh",
            Core::High1 => b"2",
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
            Core::LeftExtend16_32 => b"i",
            Core::LeftExtend16_64 => b"l",
            Core::LeftExtend1_16 => b"****22*22**22*22***22*22**22*22",
            Core::LeftExtend1_32 => b"i",
            Core::LeftExtend1_64 => b"l",
            Core::LeftExtend1_8 => b"***22*22**22*22",
            Core::LeftExtend32_64 => b"l",
            Core::LeftExtend8_16 => b"****22*22**22*22***22*22**22*22",
            Core::LeftExtend8_32 => b"i",
            Core::LeftExtend8_64 => b"l",
            Core::LeftPadHigh16_32 => b"i",
            Core::LeftPadHigh16_64 => b"l",
            Core::LeftPadHigh1_16 => b"****22*22**22*22***22*22**22*22",
            Core::LeftPadHigh1_32 => b"i",
            Core::LeftPadHigh1_64 => b"l",
            Core::LeftPadHigh1_8 => b"***22*22**22*22",
            Core::LeftPadHigh32_64 => b"l",
            Core::LeftPadHigh8_16 => b"****22*22**22*22***22*22**22*22",
            Core::LeftPadHigh8_32 => b"i",
            Core::LeftPadHigh8_64 => b"l",
            Core::LeftPadLow16_32 => b"i",
            Core::LeftPadLow16_64 => b"l",
            Core::LeftPadLow1_16 => b"****22*22**22*22***22*22**22*22",
            Core::LeftPadLow1_32 => b"i",
            Core::LeftPadLow1_64 => b"l",
            Core::LeftPadLow1_8 => b"***22*22**22*22",
            Core::LeftPadLow32_64 => b"l",
            Core::LeftPadLow8_16 => b"****22*22**22*22***22*22**22*22",
            Core::LeftPadLow8_32 => b"i",
            Core::LeftPadLow8_64 => b"l",
            Core::LeftRotate16 => b"****22*22**22*22***22*22**22*22",
            Core::LeftRotate32 => b"i",
            Core::LeftRotate64 => b"l",
            Core::LeftRotate8 => b"***22*22**22*22",
            Core::LeftShift16 => b"****22*22**22*22***22*22**22*22",
            Core::LeftShift32 => b"i",
            Core::LeftShift64 => b"l",
            Core::LeftShift8 => b"***22*22**22*22",
            Core::LeftShiftWith16 => b"****22*22**22*22***22*22**22*22",
            Core::LeftShiftWith32 => b"i",
            Core::LeftShiftWith64 => b"l",
            Core::LeftShiftWith8 => b"***22*22**22*22",
            Core::Leftmost16_1 => b"2",
            Core::Leftmost16_2 => b"*22",
            Core::Leftmost16_4 => b"**22*22",
            Core::Leftmost16_8 => b"***22*22**22*22",
            Core::Leftmost32_1 => b"2",
            Core::Leftmost32_16 => b"****22*22**22*22***22*22**22*22",
            Core::Leftmost32_2 => b"*22",
            Core::Leftmost32_4 => b"**22*22",
            Core::Leftmost32_8 => b"***22*22**22*22",
            Core::Leftmost64_1 => b"2",
            Core::Leftmost64_16 => b"****22*22**22*22***22*22**22*22",
            Core::Leftmost64_2 => b"*22",
            Core::Leftmost64_32 => b"i",
            Core::Leftmost64_4 => b"**22*22",
            Core::Leftmost64_8 => b"***22*22**22*22",
            Core::Leftmost8_1 => b"2",
            Core::Leftmost8_2 => b"*22",
            Core::Leftmost8_4 => b"**22*22",
            Core::LinearCombination1 => b"**hhh",
            Core::LinearVerify1 => b"1",
            Core::Low1 => b"2",
            Core::Low16 => b"****22*22**22*22***22*22**22*22",
            Core::Low32 => b"i",
            Core::Low64 => b"l",
            Core::Low8 => b"***22*22**22*22",
            Core::Lt16 => b"2",
            Core::Lt32 => b"2",
            Core::Lt64 => b"2",
            Core::Lt8 => b"2",
            Core::Maj1 => b"2",
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
            Core::Or1 => b"2",
            Core::Or16 => b"****22*22**22*22***22*22**22*22",
            Core::Or32 => b"i",
            Core::Or64 => b"l",
            Core::Or8 => b"***22*22**22*22",
            Core::ParseLock => b"+ii",
            Core::ParseSequence => b"+1+****22*22**22*22***22*22**22*22****22*22**22*22***22*22**22*22",
            Core::PointVerify1 => b"1",
            Core::RightExtend16_32 => b"i",
            Core::RightExtend16_64 => b"l",
            Core::RightExtend32_64 => b"l",
            Core::RightExtend8_16 => b"****22*22**22*22***22*22**22*22",
            Core::RightExtend8_32 => b"i",
            Core::RightExtend8_64 => b"l",
            Core::RightPadHigh16_32 => b"i",
            Core::RightPadHigh16_64 => b"l",
            Core::RightPadHigh1_16 => b"****22*22**22*22***22*22**22*22",
            Core::RightPadHigh1_32 => b"i",
            Core::RightPadHigh1_64 => b"l",
            Core::RightPadHigh1_8 => b"***22*22**22*22",
            Core::RightPadHigh32_64 => b"l",
            Core::RightPadHigh8_16 => b"****22*22**22*22***22*22**22*22",
            Core::RightPadHigh8_32 => b"i",
            Core::RightPadHigh8_64 => b"l",
            Core::RightPadLow16_32 => b"i",
            Core::RightPadLow16_64 => b"l",
            Core::RightPadLow1_16 => b"****22*22**22*22***22*22**22*22",
            Core::RightPadLow1_32 => b"i",
            Core::RightPadLow1_64 => b"l",
            Core::RightPadLow1_8 => b"***22*22**22*22",
            Core::RightPadLow32_64 => b"l",
            Core::RightPadLow8_16 => b"****22*22**22*22***22*22**22*22",
            Core::RightPadLow8_32 => b"i",
            Core::RightPadLow8_64 => b"l",
            Core::RightRotate16 => b"****22*22**22*22***22*22**22*22",
            Core::RightRotate32 => b"i",
            Core::RightRotate64 => b"l",
            Core::RightRotate8 => b"***22*22**22*22",
            Core::RightShift16 => b"****22*22**22*22***22*22**22*22",
            Core::RightShift32 => b"i",
            Core::RightShift64 => b"l",
            Core::RightShift8 => b"***22*22**22*22",
            Core::RightShiftWith16 => b"****22*22**22*22***22*22**22*22",
            Core::RightShiftWith32 => b"i",
            Core::RightShiftWith64 => b"l",
            Core::RightShiftWith8 => b"***22*22**22*22",
            Core::Rightmost16_1 => b"2",
            Core::Rightmost16_2 => b"*22",
            Core::Rightmost16_4 => b"**22*22",
            Core::Rightmost16_8 => b"***22*22**22*22",
            Core::Rightmost32_1 => b"2",
            Core::Rightmost32_16 => b"****22*22**22*22***22*22**22*22",
            Core::Rightmost32_2 => b"*22",
            Core::Rightmost32_4 => b"**22*22",
            Core::Rightmost32_8 => b"***22*22**22*22",
            Core::Rightmost64_1 => b"2",
            Core::Rightmost64_16 => b"****22*22**22*22***22*22**22*22",
            Core::Rightmost64_2 => b"*22",
            Core::Rightmost64_32 => b"i",
            Core::Rightmost64_4 => b"**22*22",
            Core::Rightmost64_8 => b"***22*22**22*22",
            Core::Rightmost8_1 => b"2",
            Core::Rightmost8_2 => b"*22",
            Core::Rightmost8_4 => b"**22*22",
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
            Core::Some1 => b"2",
            Core::Some16 => b"2",
            Core::Some32 => b"2",
            Core::Some64 => b"2",
            Core::Some8 => b"2",
            Core::Subtract16 => b"*2****22*22**22*22***22*22**22*22",
            Core::Subtract32 => b"*2i",
            Core::Subtract64 => b"*2l",
            Core::Subtract8 => b"*2***22*22**22*22",
            Core::Swu => b"*hh",
            Core::Verify => b"1",
            Core::Xor1 => b"2",
            Core::Xor16 => b"****22*22**22*22***22*22**22*22",
            Core::Xor32 => b"i",
            Core::Xor64 => b"l",
            Core::Xor8 => b"***22*22**22*22",
            Core::XorXor1 => b"2",
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
            Core::Low1 => (8, 5),
            Core::Low8 => (37, 7),
            Core::Low16 => (304, 10),
            Core::Low32 => (305, 10),
            Core::Low64 => (306, 10),
            Core::High1 => (10, 5),
            Core::High8 => (45, 7),
            Core::High16 => (368, 10),
            Core::High32 => (369, 10),
            Core::High64 => (370, 10),
            Core::Complement1 => (96, 8),
            Core::Complement8 => (389, 10),
            Core::Complement16 => (3120, 13),
            Core::Complement32 => (3121, 13),
            Core::Complement64 => (3122, 13),
            Core::And1 => (98, 8),
            Core::And8 => (397, 10),
            Core::And16 => (3184, 13),
            Core::And32 => (3185, 13),
            Core::And64 => (3186, 13),
            Core::Or1 => (100, 8),
            Core::Or8 => (405, 10),
            Core::Or16 => (3248, 13),
            Core::Or32 => (3249, 13),
            Core::Or64 => (3250, 13),
            Core::Xor1 => (102, 8),
            Core::Xor8 => (413, 10),
            Core::Xor16 => (3312, 13),
            Core::Xor32 => (3313, 13),
            Core::Xor64 => (3314, 13),
            Core::Maj1 => (208, 9),
            Core::Maj8 => (837, 11),
            Core::Maj16 => (6704, 14),
            Core::Maj32 => (6705, 14),
            Core::Maj64 => (6706, 14),
            Core::XorXor1 => (210, 9),
            Core::XorXor8 => (845, 11),
            Core::XorXor16 => (6768, 14),
            Core::XorXor32 => (6769, 14),
            Core::XorXor64 => (6770, 14),
            Core::Ch1 => (212, 9),
            Core::Ch8 => (853, 11),
            Core::Ch16 => (6832, 14),
            Core::Ch32 => (6833, 14),
            Core::Ch64 => (6834, 14),
            Core::Some1 => (214, 9),
            Core::Some8 => (861, 11),
            Core::Some16 => (6896, 14),
            Core::Some32 => (6897, 14),
            Core::Some64 => (6898, 14),
            Core::All8 => (869, 11),
            Core::All16 => (6960, 14),
            Core::All32 => (6961, 14),
            Core::All64 => (6962, 14),
            Core::Eq1 => (218, 9),
            Core::Eq8 => (877, 11),
            Core::Eq16 => (7024, 14),
            Core::Eq32 => (7025, 14),
            Core::Eq64 => (7026, 14),
            Core::Eq256 => (14056, 15),
            Core::FullLeftShift8_1 => (1765, 12),
            Core::FullLeftShift16_1 => (14128, 15),
            Core::FullLeftShift32_1 => (14129, 15),
            Core::FullLeftShift64_1 => (14130, 15),
            Core::FullLeftShift8_2 => (7076, 14),
            Core::FullLeftShift16_2 => (7077, 14),
            Core::FullLeftShift32_2 => (56624, 17),
            Core::FullLeftShift64_2 => (56625, 17),
            Core::FullLeftShift8_4 => (1770, 12),
            Core::FullLeftShift16_4 => (7084, 14),
            Core::FullLeftShift32_4 => (7085, 14),
            Core::FullLeftShift64_4 => (56688, 17),
            Core::FullLeftShift16_8 => (14176, 15),
            Core::FullLeftShift32_8 => (56708, 17),
            Core::FullLeftShift64_8 => (56709, 17),
            Core::FullLeftShift32_16 => (14178, 15),
            Core::FullLeftShift64_16 => (56716, 17),
            Core::FullLeftShift64_32 => (14180, 15),
            Core::FullRightShift8_1 => (1781, 12),
            Core::FullRightShift16_1 => (14256, 15),
            Core::FullRightShift32_1 => (14257, 15),
            Core::FullRightShift64_1 => (14258, 15),
            Core::FullRightShift8_2 => (7140, 14),
            Core::FullRightShift16_2 => (7141, 14),
            Core::FullRightShift32_2 => (57136, 17),
            Core::FullRightShift64_2 => (57137, 17),
            Core::FullRightShift8_4 => (1786, 12),
            Core::FullRightShift16_4 => (7148, 14),
            Core::FullRightShift32_4 => (7149, 14),
            Core::FullRightShift64_4 => (57200, 17),
            Core::FullRightShift16_8 => (14304, 15),
            Core::FullRightShift32_8 => (57220, 17),
            Core::FullRightShift64_8 => (57221, 17),
            Core::FullRightShift32_16 => (14306, 15),
            Core::FullRightShift64_16 => (57228, 17),
            Core::FullRightShift64_32 => (14308, 15),
            Core::Leftmost8_1 => (28677, 16),
            Core::Leftmost16_1 => (229424, 19),
            Core::Leftmost32_1 => (229425, 19),
            Core::Leftmost64_1 => (229426, 19),
            Core::Leftmost8_2 => (114724, 18),
            Core::Leftmost16_2 => (114725, 18),
            Core::Leftmost32_2 => (917808, 21),
            Core::Leftmost64_2 => (917809, 21),
            Core::Leftmost8_4 => (28682, 16),
            Core::Leftmost16_4 => (114732, 18),
            Core::Leftmost32_4 => (114733, 18),
            Core::Leftmost64_4 => (917872, 21),
            Core::Leftmost16_8 => (229472, 19),
            Core::Leftmost32_8 => (917892, 21),
            Core::Leftmost64_8 => (917893, 21),
            Core::Leftmost32_16 => (229474, 19),
            Core::Leftmost64_16 => (917900, 21),
            Core::Leftmost64_32 => (229476, 19),
            Core::Rightmost8_1 => (28693, 16),
            Core::Rightmost16_1 => (229552, 19),
            Core::Rightmost32_1 => (229553, 19),
            Core::Rightmost64_1 => (229554, 19),
            Core::Rightmost8_2 => (114788, 18),
            Core::Rightmost16_2 => (114789, 18),
            Core::Rightmost32_2 => (918320, 21),
            Core::Rightmost64_2 => (918321, 21),
            Core::Rightmost8_4 => (28698, 16),
            Core::Rightmost16_4 => (114796, 18),
            Core::Rightmost32_4 => (114797, 18),
            Core::Rightmost64_4 => (918384, 21),
            Core::Rightmost16_8 => (229600, 19),
            Core::Rightmost32_8 => (918404, 21),
            Core::Rightmost64_8 => (918405, 21),
            Core::Rightmost32_16 => (229602, 19),
            Core::Rightmost64_16 => (918412, 21),
            Core::Rightmost64_32 => (229604, 19),
            Core::LeftPadLow1_8 => (28709, 16),
            Core::LeftPadLow1_16 => (229680, 19),
            Core::LeftPadLow1_32 => (229681, 19),
            Core::LeftPadLow1_64 => (229682, 19),
            Core::LeftPadLow8_16 => (229728, 19),
            Core::LeftPadLow8_32 => (918916, 21),
            Core::LeftPadLow8_64 => (918917, 21),
            Core::LeftPadLow16_32 => (229730, 19),
            Core::LeftPadLow16_64 => (918924, 21),
            Core::LeftPadLow32_64 => (229732, 19),
            Core::LeftPadHigh1_8 => (28725, 16),
            Core::LeftPadHigh1_16 => (229808, 19),
            Core::LeftPadHigh1_32 => (229809, 19),
            Core::LeftPadHigh1_64 => (229810, 19),
            Core::LeftPadHigh8_16 => (229856, 19),
            Core::LeftPadHigh8_32 => (919428, 21),
            Core::LeftPadHigh8_64 => (919429, 21),
            Core::LeftPadHigh16_32 => (229858, 19),
            Core::LeftPadHigh16_64 => (919436, 21),
            Core::LeftPadHigh32_64 => (229860, 19),
            Core::LeftExtend1_8 => (28741, 16),
            Core::LeftExtend1_16 => (229936, 19),
            Core::LeftExtend1_32 => (229937, 19),
            Core::LeftExtend1_64 => (229938, 19),
            Core::LeftExtend8_16 => (229984, 19),
            Core::LeftExtend8_32 => (919940, 21),
            Core::LeftExtend8_64 => (919941, 21),
            Core::LeftExtend16_32 => (229986, 19),
            Core::LeftExtend16_64 => (919948, 21),
            Core::LeftExtend32_64 => (229988, 19),
            Core::RightPadLow1_8 => (28757, 16),
            Core::RightPadLow1_16 => (230064, 19),
            Core::RightPadLow1_32 => (230065, 19),
            Core::RightPadLow1_64 => (230066, 19),
            Core::RightPadLow8_16 => (230112, 19),
            Core::RightPadLow8_32 => (920452, 21),
            Core::RightPadLow8_64 => (920453, 21),
            Core::RightPadLow16_32 => (230114, 19),
            Core::RightPadLow16_64 => (920460, 21),
            Core::RightPadLow32_64 => (230116, 19),
            Core::RightPadHigh1_8 => (28773, 16),
            Core::RightPadHigh1_16 => (230192, 19),
            Core::RightPadHigh1_32 => (230193, 19),
            Core::RightPadHigh1_64 => (230194, 19),
            Core::RightPadHigh8_16 => (230240, 19),
            Core::RightPadHigh8_32 => (920964, 21),
            Core::RightPadHigh8_64 => (920965, 21),
            Core::RightPadHigh16_32 => (230242, 19),
            Core::RightPadHigh16_64 => (920972, 21),
            Core::RightPadHigh32_64 => (230244, 19),
            Core::RightExtend8_16 => (230368, 19),
            Core::RightExtend8_32 => (921476, 21),
            Core::RightExtend8_64 => (921477, 21),
            Core::RightExtend16_32 => (230370, 19),
            Core::RightExtend16_64 => (921484, 21),
            Core::RightExtend32_64 => (230372, 19),
            Core::LeftShiftWith8 => (14405, 15),
            Core::LeftShiftWith16 => (115248, 18),
            Core::LeftShiftWith32 => (115249, 18),
            Core::LeftShiftWith64 => (115250, 18),
            Core::RightShiftWith8 => (14413, 15),
            Core::RightShiftWith16 => (115312, 18),
            Core::RightShiftWith32 => (115313, 18),
            Core::RightShiftWith64 => (115314, 18),
            Core::LeftShift8 => (14421, 15),
            Core::LeftShift16 => (115376, 18),
            Core::LeftShift32 => (115377, 18),
            Core::LeftShift64 => (115378, 18),
            Core::RightShift8 => (14429, 15),
            Core::RightShift16 => (115440, 18),
            Core::RightShift32 => (115441, 18),
            Core::RightShift64 => (115442, 18),
            Core::LeftRotate8 => (14437, 15),
            Core::LeftRotate16 => (115504, 18),
            Core::LeftRotate32 => (115505, 18),
            Core::LeftRotate64 => (115506, 18),
            Core::RightRotate8 => (14445, 15),
            Core::RightRotate16 => (115568, 18),
            Core::RightRotate32 => (115569, 18),
            Core::RightRotate64 => (115570, 18),
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
            Core::DivMod128_64 => (639346, 20),
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
            Core::GejEquiv => (100097, 17),
            Core::GejGeEquiv => (100098, 17),
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
            Core::HashToCurve => (200238, 18),
            Core::Swu => (200239, 18),
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
                            0 => {Core::Low1},
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
                            0 => {Core::High1},
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
                                        0 => {Core::Complement1},
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
                                        0 => {Core::And1},
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
                                        0 => {Core::Or1},
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
                                        0 => {Core::Xor1},
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
                                            0 => {Core::Maj1},
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
                                            0 => {Core::XorXor1},
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
                                            0 => {Core::Ch1},
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
                                            0 => {Core::Some1},
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
                                            0 => {Core::Eq1},
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
                                    1 => {
                                        0 => {
                                            0 => {
                                                0 => {},
                                                1 => {
                                                    0 => {
                                                        0 => {},
                                                        1 => {Core::FullLeftShift8_1}
                                                    },
                                                    1 => {
                                                        0 => {
                                                            0 => {
                                                                0 => {
                                                                    0 => {Core::FullLeftShift16_1},
                                                                    1 => {Core::FullLeftShift32_1}
                                                                },
                                                                1 => {
                                                                    0 => {Core::FullLeftShift64_1},
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
                                                                0 => {Core::FullLeftShift8_2},
                                                                1 => {Core::FullLeftShift16_2}
                                                            },
                                                            1 => {
                                                                0 => {
                                                                    0 => {
                                                                        0 => {
                                                                            0 => {Core::FullLeftShift32_2},
                                                                            1 => {Core::FullLeftShift64_2}
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
                                                        0 => {Core::FullLeftShift8_4},
                                                        1 => {
                                                            0 => {
                                                                0 => {Core::FullLeftShift16_4},
                                                                1 => {Core::FullLeftShift32_4}
                                                            },
                                                            1 => {
                                                                0 => {
                                                                    0 => {
                                                                        0 => {
                                                                            0 => {Core::FullLeftShift64_4},
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
                                                        0 => {
                                                            0 => {
                                                                0 => {
                                                                    0 => {Core::FullLeftShift16_8},
                                                                    1 => {
                                                                        0 => {
                                                                            0 => {Core::FullLeftShift32_8},
                                                                            1 => {Core::FullLeftShift64_8}
                                                                        },
                                                                        1 => {}
                                                                    }
                                                                },
                                                                1 => {
                                                                    0 => {Core::FullLeftShift32_16},
                                                                    1 => {
                                                                        0 => {
                                                                            0 => {Core::FullLeftShift64_16},
                                                                            1 => {}
                                                                        },
                                                                        1 => {}
                                                                    }
                                                                }
                                                            },
                                                            1 => {
                                                                0 => {
                                                                    0 => {Core::FullLeftShift64_32},
                                                                    1 => {}
                                                                },
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
                                                0 => {},
                                                1 => {
                                                    0 => {
                                                        0 => {},
                                                        1 => {Core::FullRightShift8_1}
                                                    },
                                                    1 => {
                                                        0 => {
                                                            0 => {
                                                                0 => {
                                                                    0 => {Core::FullRightShift16_1},
                                                                    1 => {Core::FullRightShift32_1}
                                                                },
                                                                1 => {
                                                                    0 => {Core::FullRightShift64_1},
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
                                                                0 => {Core::FullRightShift8_2},
                                                                1 => {Core::FullRightShift16_2}
                                                            },
                                                            1 => {
                                                                0 => {
                                                                    0 => {
                                                                        0 => {
                                                                            0 => {Core::FullRightShift32_2},
                                                                            1 => {Core::FullRightShift64_2}
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
                                                        0 => {Core::FullRightShift8_4},
                                                        1 => {
                                                            0 => {
                                                                0 => {Core::FullRightShift16_4},
                                                                1 => {Core::FullRightShift32_4}
                                                            },
                                                            1 => {
                                                                0 => {
                                                                    0 => {
                                                                        0 => {
                                                                            0 => {Core::FullRightShift64_4},
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
                                                        0 => {
                                                            0 => {
                                                                0 => {
                                                                    0 => {Core::FullRightShift16_8},
                                                                    1 => {
                                                                        0 => {
                                                                            0 => {Core::FullRightShift32_8},
                                                                            1 => {Core::FullRightShift64_8}
                                                                        },
                                                                        1 => {}
                                                                    }
                                                                },
                                                                1 => {
                                                                    0 => {Core::FullRightShift32_16},
                                                                    1 => {
                                                                        0 => {
                                                                            0 => {Core::FullRightShift64_16},
                                                                            1 => {}
                                                                        },
                                                                        1 => {}
                                                                    }
                                                                }
                                                            },
                                                            1 => {
                                                                0 => {
                                                                    0 => {Core::FullRightShift64_32},
                                                                    1 => {}
                                                                },
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
                                                            0 => {
                                                                0 => {},
                                                                1 => {
                                                                    0 => {
                                                                        0 => {},
                                                                        1 => {Core::Leftmost8_1}
                                                                    },
                                                                    1 => {
                                                                        0 => {
                                                                            0 => {
                                                                                0 => {
                                                                                    0 => {Core::Leftmost16_1},
                                                                                    1 => {Core::Leftmost32_1}
                                                                                },
                                                                                1 => {
                                                                                    0 => {Core::Leftmost64_1},
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
                                                                                0 => {Core::Leftmost8_2},
                                                                                1 => {Core::Leftmost16_2}
                                                                            },
                                                                            1 => {
                                                                                0 => {
                                                                                    0 => {
                                                                                        0 => {
                                                                                            0 => {Core::Leftmost32_2},
                                                                                            1 => {Core::Leftmost64_2}
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
                                                                        0 => {Core::Leftmost8_4},
                                                                        1 => {
                                                                            0 => {
                                                                                0 => {Core::Leftmost16_4},
                                                                                1 => {Core::Leftmost32_4}
                                                                            },
                                                                            1 => {
                                                                                0 => {
                                                                                    0 => {
                                                                                        0 => {
                                                                                            0 => {Core::Leftmost64_4},
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
                                                                        0 => {
                                                                            0 => {
                                                                                0 => {
                                                                                    0 => {Core::Leftmost16_8},
                                                                                    1 => {
                                                                                        0 => {
                                                                                            0 => {Core::Leftmost32_8},
                                                                                            1 => {Core::Leftmost64_8}
                                                                                        },
                                                                                        1 => {}
                                                                                    }
                                                                                },
                                                                                1 => {
                                                                                    0 => {Core::Leftmost32_16},
                                                                                    1 => {
                                                                                        0 => {
                                                                                            0 => {Core::Leftmost64_16},
                                                                                            1 => {}
                                                                                        },
                                                                                        1 => {}
                                                                                    }
                                                                                }
                                                                            },
                                                                            1 => {
                                                                                0 => {
                                                                                    0 => {Core::Leftmost64_32},
                                                                                    1 => {}
                                                                                },
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
                                                                0 => {},
                                                                1 => {
                                                                    0 => {
                                                                        0 => {},
                                                                        1 => {Core::Rightmost8_1}
                                                                    },
                                                                    1 => {
                                                                        0 => {
                                                                            0 => {
                                                                                0 => {
                                                                                    0 => {Core::Rightmost16_1},
                                                                                    1 => {Core::Rightmost32_1}
                                                                                },
                                                                                1 => {
                                                                                    0 => {Core::Rightmost64_1},
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
                                                                                0 => {Core::Rightmost8_2},
                                                                                1 => {Core::Rightmost16_2}
                                                                            },
                                                                            1 => {
                                                                                0 => {
                                                                                    0 => {
                                                                                        0 => {
                                                                                            0 => {Core::Rightmost32_2},
                                                                                            1 => {Core::Rightmost64_2}
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
                                                                        0 => {Core::Rightmost8_4},
                                                                        1 => {
                                                                            0 => {
                                                                                0 => {Core::Rightmost16_4},
                                                                                1 => {Core::Rightmost32_4}
                                                                            },
                                                                            1 => {
                                                                                0 => {
                                                                                    0 => {
                                                                                        0 => {
                                                                                            0 => {Core::Rightmost64_4},
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
                                                                        0 => {
                                                                            0 => {
                                                                                0 => {
                                                                                    0 => {Core::Rightmost16_8},
                                                                                    1 => {
                                                                                        0 => {
                                                                                            0 => {Core::Rightmost32_8},
                                                                                            1 => {Core::Rightmost64_8}
                                                                                        },
                                                                                        1 => {}
                                                                                    }
                                                                                },
                                                                                1 => {
                                                                                    0 => {Core::Rightmost32_16},
                                                                                    1 => {
                                                                                        0 => {
                                                                                            0 => {Core::Rightmost64_16},
                                                                                            1 => {}
                                                                                        },
                                                                                        1 => {}
                                                                                    }
                                                                                }
                                                                            },
                                                                            1 => {
                                                                                0 => {
                                                                                    0 => {Core::Rightmost64_32},
                                                                                    1 => {}
                                                                                },
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
                                                                0 => {},
                                                                1 => {
                                                                    0 => {
                                                                        0 => {},
                                                                        1 => {Core::LeftPadLow1_8}
                                                                    },
                                                                    1 => {
                                                                        0 => {
                                                                            0 => {
                                                                                0 => {
                                                                                    0 => {Core::LeftPadLow1_16},
                                                                                    1 => {Core::LeftPadLow1_32}
                                                                                },
                                                                                1 => {
                                                                                    0 => {Core::LeftPadLow1_64},
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
                                                                        0 => {
                                                                            0 => {
                                                                                0 => {
                                                                                    0 => {Core::LeftPadLow8_16},
                                                                                    1 => {
                                                                                        0 => {
                                                                                            0 => {Core::LeftPadLow8_32},
                                                                                            1 => {Core::LeftPadLow8_64}
                                                                                        },
                                                                                        1 => {}
                                                                                    }
                                                                                },
                                                                                1 => {
                                                                                    0 => {Core::LeftPadLow16_32},
                                                                                    1 => {
                                                                                        0 => {
                                                                                            0 => {Core::LeftPadLow16_64},
                                                                                            1 => {}
                                                                                        },
                                                                                        1 => {}
                                                                                    }
                                                                                }
                                                                            },
                                                                            1 => {
                                                                                0 => {
                                                                                    0 => {Core::LeftPadLow32_64},
                                                                                    1 => {}
                                                                                },
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
                                                                0 => {},
                                                                1 => {
                                                                    0 => {
                                                                        0 => {},
                                                                        1 => {Core::LeftPadHigh1_8}
                                                                    },
                                                                    1 => {
                                                                        0 => {
                                                                            0 => {
                                                                                0 => {
                                                                                    0 => {Core::LeftPadHigh1_16},
                                                                                    1 => {Core::LeftPadHigh1_32}
                                                                                },
                                                                                1 => {
                                                                                    0 => {Core::LeftPadHigh1_64},
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
                                                                        0 => {
                                                                            0 => {
                                                                                0 => {
                                                                                    0 => {Core::LeftPadHigh8_16},
                                                                                    1 => {
                                                                                        0 => {
                                                                                            0 => {Core::LeftPadHigh8_32},
                                                                                            1 => {Core::LeftPadHigh8_64}
                                                                                        },
                                                                                        1 => {}
                                                                                    }
                                                                                },
                                                                                1 => {
                                                                                    0 => {Core::LeftPadHigh16_32},
                                                                                    1 => {
                                                                                        0 => {
                                                                                            0 => {Core::LeftPadHigh16_64},
                                                                                            1 => {}
                                                                                        },
                                                                                        1 => {}
                                                                                    }
                                                                                }
                                                                            },
                                                                            1 => {
                                                                                0 => {
                                                                                    0 => {Core::LeftPadHigh32_64},
                                                                                    1 => {}
                                                                                },
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
                                                                        1 => {Core::LeftExtend1_8}
                                                                    },
                                                                    1 => {
                                                                        0 => {
                                                                            0 => {
                                                                                0 => {
                                                                                    0 => {Core::LeftExtend1_16},
                                                                                    1 => {Core::LeftExtend1_32}
                                                                                },
                                                                                1 => {
                                                                                    0 => {Core::LeftExtend1_64},
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
                                                                        0 => {
                                                                            0 => {
                                                                                0 => {
                                                                                    0 => {Core::LeftExtend8_16},
                                                                                    1 => {
                                                                                        0 => {
                                                                                            0 => {Core::LeftExtend8_32},
                                                                                            1 => {Core::LeftExtend8_64}
                                                                                        },
                                                                                        1 => {}
                                                                                    }
                                                                                },
                                                                                1 => {
                                                                                    0 => {Core::LeftExtend16_32},
                                                                                    1 => {
                                                                                        0 => {
                                                                                            0 => {Core::LeftExtend16_64},
                                                                                            1 => {}
                                                                                        },
                                                                                        1 => {}
                                                                                    }
                                                                                }
                                                                            },
                                                                            1 => {
                                                                                0 => {
                                                                                    0 => {Core::LeftExtend32_64},
                                                                                    1 => {}
                                                                                },
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
                                                                0 => {},
                                                                1 => {
                                                                    0 => {
                                                                        0 => {},
                                                                        1 => {Core::RightPadLow1_8}
                                                                    },
                                                                    1 => {
                                                                        0 => {
                                                                            0 => {
                                                                                0 => {
                                                                                    0 => {Core::RightPadLow1_16},
                                                                                    1 => {Core::RightPadLow1_32}
                                                                                },
                                                                                1 => {
                                                                                    0 => {Core::RightPadLow1_64},
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
                                                                        0 => {
                                                                            0 => {
                                                                                0 => {
                                                                                    0 => {Core::RightPadLow8_16},
                                                                                    1 => {
                                                                                        0 => {
                                                                                            0 => {Core::RightPadLow8_32},
                                                                                            1 => {Core::RightPadLow8_64}
                                                                                        },
                                                                                        1 => {}
                                                                                    }
                                                                                },
                                                                                1 => {
                                                                                    0 => {Core::RightPadLow16_32},
                                                                                    1 => {
                                                                                        0 => {
                                                                                            0 => {Core::RightPadLow16_64},
                                                                                            1 => {}
                                                                                        },
                                                                                        1 => {}
                                                                                    }
                                                                                }
                                                                            },
                                                                            1 => {
                                                                                0 => {
                                                                                    0 => {Core::RightPadLow32_64},
                                                                                    1 => {}
                                                                                },
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
                                                                0 => {},
                                                                1 => {
                                                                    0 => {
                                                                        0 => {},
                                                                        1 => {Core::RightPadHigh1_8}
                                                                    },
                                                                    1 => {
                                                                        0 => {
                                                                            0 => {
                                                                                0 => {
                                                                                    0 => {Core::RightPadHigh1_16},
                                                                                    1 => {Core::RightPadHigh1_32}
                                                                                },
                                                                                1 => {
                                                                                    0 => {Core::RightPadHigh1_64},
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
                                                                        0 => {
                                                                            0 => {
                                                                                0 => {
                                                                                    0 => {Core::RightPadHigh8_16},
                                                                                    1 => {
                                                                                        0 => {
                                                                                            0 => {Core::RightPadHigh8_32},
                                                                                            1 => {Core::RightPadHigh8_64}
                                                                                        },
                                                                                        1 => {}
                                                                                    }
                                                                                },
                                                                                1 => {
                                                                                    0 => {Core::RightPadHigh16_32},
                                                                                    1 => {
                                                                                        0 => {
                                                                                            0 => {Core::RightPadHigh16_64},
                                                                                            1 => {}
                                                                                        },
                                                                                        1 => {}
                                                                                    }
                                                                                }
                                                                            },
                                                                            1 => {
                                                                                0 => {
                                                                                    0 => {Core::RightPadHigh32_64},
                                                                                    1 => {}
                                                                                },
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
                                                                0 => {},
                                                                1 => {
                                                                    0 => {
                                                                        0 => {
                                                                            0 => {
                                                                                0 => {
                                                                                    0 => {Core::RightExtend8_16},
                                                                                    1 => {
                                                                                        0 => {
                                                                                            0 => {Core::RightExtend8_32},
                                                                                            1 => {Core::RightExtend8_64}
                                                                                        },
                                                                                        1 => {}
                                                                                    }
                                                                                },
                                                                                1 => {
                                                                                    0 => {Core::RightExtend16_32},
                                                                                    1 => {
                                                                                        0 => {
                                                                                            0 => {Core::RightExtend16_64},
                                                                                            1 => {}
                                                                                        },
                                                                                        1 => {}
                                                                                    }
                                                                                }
                                                                            },
                                                                            1 => {
                                                                                0 => {
                                                                                    0 => {Core::RightExtend32_64},
                                                                                    1 => {}
                                                                                },
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
                                                                    1 => {Core::LeftShiftWith8}
                                                                },
                                                                1 => {
                                                                    0 => {
                                                                        0 => {
                                                                            0 => {
                                                                                0 => {Core::LeftShiftWith16},
                                                                                1 => {Core::LeftShiftWith32}
                                                                            },
                                                                            1 => {
                                                                                0 => {Core::LeftShiftWith64},
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
                                                                    1 => {Core::RightShiftWith8}
                                                                },
                                                                1 => {
                                                                    0 => {
                                                                        0 => {
                                                                            0 => {
                                                                                0 => {Core::RightShiftWith16},
                                                                                1 => {Core::RightShiftWith32}
                                                                            },
                                                                            1 => {
                                                                                0 => {Core::RightShiftWith64},
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
                                                                    1 => {Core::LeftShift8}
                                                                },
                                                                1 => {
                                                                    0 => {
                                                                        0 => {
                                                                            0 => {
                                                                                0 => {Core::LeftShift16},
                                                                                1 => {Core::LeftShift32}
                                                                            },
                                                                            1 => {
                                                                                0 => {Core::LeftShift64},
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
                                                                    1 => {Core::RightShift8}
                                                                },
                                                                1 => {
                                                                    0 => {
                                                                        0 => {
                                                                            0 => {
                                                                                0 => {Core::RightShift16},
                                                                                1 => {Core::RightShift32}
                                                                            },
                                                                            1 => {
                                                                                0 => {Core::RightShift64},
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
                                                                    1 => {Core::LeftRotate8}
                                                                },
                                                                1 => {
                                                                    0 => {
                                                                        0 => {
                                                                            0 => {
                                                                                0 => {Core::LeftRotate16},
                                                                                1 => {Core::LeftRotate32}
                                                                            },
                                                                            1 => {
                                                                                0 => {Core::LeftRotate64},
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
                                                                    1 => {Core::RightRotate8}
                                                                },
                                                                1 => {
                                                                    0 => {
                                                                        0 => {
                                                                            0 => {
                                                                                0 => {Core::RightRotate16},
                                                                                1 => {Core::RightRotate32}
                                                                            },
                                                                            1 => {
                                                                                0 => {Core::RightRotate64},
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
                                                                1 => {
                                                                    0 => {},
                                                                    1 => {
                                                                        0 => {},
                                                                        1 => {
                                                                            0 => {
                                                                                0 => {
                                                                                    0 => {},
                                                                                    1 => {
                                                                                        0 => {Core::DivMod128_64},
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
                                                                            1 => {Core::GejEquiv}
                                                                        },
                                                                        1 => {
                                                                            0 => {Core::GejGeEquiv},
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
                                                                            1 => {
                                                                                0 => {Core::HashToCurve},
                                                                                1 => {Core::Swu}
                                                                            }
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
            Core::And1 => &simplicity_sys::c_jets::jets_wrapper::and_1,
            Core::And16 => &simplicity_sys::c_jets::jets_wrapper::and_16,
            Core::And32 => &simplicity_sys::c_jets::jets_wrapper::and_32,
            Core::And64 => &simplicity_sys::c_jets::jets_wrapper::and_64,
            Core::And8 => &simplicity_sys::c_jets::jets_wrapper::and_8,
            Core::Bip0340Verify => &simplicity_sys::c_jets::jets_wrapper::bip_0340_verify,
            Core::Ch1 => &simplicity_sys::c_jets::jets_wrapper::ch_1,
            Core::Ch16 => &simplicity_sys::c_jets::jets_wrapper::ch_16,
            Core::Ch32 => &simplicity_sys::c_jets::jets_wrapper::ch_32,
            Core::Ch64 => &simplicity_sys::c_jets::jets_wrapper::ch_64,
            Core::Ch8 => &simplicity_sys::c_jets::jets_wrapper::ch_8,
            Core::CheckSigVerify => &simplicity_sys::c_jets::jets_wrapper::check_sig_verify,
            Core::Complement1 => &simplicity_sys::c_jets::jets_wrapper::complement_1,
            Core::Complement16 => &simplicity_sys::c_jets::jets_wrapper::complement_16,
            Core::Complement32 => &simplicity_sys::c_jets::jets_wrapper::complement_32,
            Core::Complement64 => &simplicity_sys::c_jets::jets_wrapper::complement_64,
            Core::Complement8 => &simplicity_sys::c_jets::jets_wrapper::complement_8,
            Core::Decompress => &simplicity_sys::c_jets::jets_wrapper::decompress,
            Core::Decrement16 => &simplicity_sys::c_jets::jets_wrapper::decrement_16,
            Core::Decrement32 => &simplicity_sys::c_jets::jets_wrapper::decrement_32,
            Core::Decrement64 => &simplicity_sys::c_jets::jets_wrapper::decrement_64,
            Core::Decrement8 => &simplicity_sys::c_jets::jets_wrapper::decrement_8,
            Core::DivMod128_64 => &simplicity_sys::c_jets::jets_wrapper::div_mod_128_64,
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
            Core::Eq1 => &simplicity_sys::c_jets::jets_wrapper::eq_1,
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
            Core::FullLeftShift16_1 => &simplicity_sys::c_jets::jets_wrapper::full_left_shift_16_1,
            Core::FullLeftShift16_2 => &simplicity_sys::c_jets::jets_wrapper::full_left_shift_16_2,
            Core::FullLeftShift16_4 => &simplicity_sys::c_jets::jets_wrapper::full_left_shift_16_4,
            Core::FullLeftShift16_8 => &simplicity_sys::c_jets::jets_wrapper::full_left_shift_16_8,
            Core::FullLeftShift32_1 => &simplicity_sys::c_jets::jets_wrapper::full_left_shift_32_1,
            Core::FullLeftShift32_16 => &simplicity_sys::c_jets::jets_wrapper::full_left_shift_32_16,
            Core::FullLeftShift32_2 => &simplicity_sys::c_jets::jets_wrapper::full_left_shift_32_2,
            Core::FullLeftShift32_4 => &simplicity_sys::c_jets::jets_wrapper::full_left_shift_32_4,
            Core::FullLeftShift32_8 => &simplicity_sys::c_jets::jets_wrapper::full_left_shift_32_8,
            Core::FullLeftShift64_1 => &simplicity_sys::c_jets::jets_wrapper::full_left_shift_64_1,
            Core::FullLeftShift64_16 => &simplicity_sys::c_jets::jets_wrapper::full_left_shift_64_16,
            Core::FullLeftShift64_2 => &simplicity_sys::c_jets::jets_wrapper::full_left_shift_64_2,
            Core::FullLeftShift64_32 => &simplicity_sys::c_jets::jets_wrapper::full_left_shift_64_32,
            Core::FullLeftShift64_4 => &simplicity_sys::c_jets::jets_wrapper::full_left_shift_64_4,
            Core::FullLeftShift64_8 => &simplicity_sys::c_jets::jets_wrapper::full_left_shift_64_8,
            Core::FullLeftShift8_1 => &simplicity_sys::c_jets::jets_wrapper::full_left_shift_8_1,
            Core::FullLeftShift8_2 => &simplicity_sys::c_jets::jets_wrapper::full_left_shift_8_2,
            Core::FullLeftShift8_4 => &simplicity_sys::c_jets::jets_wrapper::full_left_shift_8_4,
            Core::FullMultiply16 => &simplicity_sys::c_jets::jets_wrapper::full_multiply_16,
            Core::FullMultiply32 => &simplicity_sys::c_jets::jets_wrapper::full_multiply_32,
            Core::FullMultiply64 => &simplicity_sys::c_jets::jets_wrapper::full_multiply_64,
            Core::FullMultiply8 => &simplicity_sys::c_jets::jets_wrapper::full_multiply_8,
            Core::FullRightShift16_1 => &simplicity_sys::c_jets::jets_wrapper::full_right_shift_16_1,
            Core::FullRightShift16_2 => &simplicity_sys::c_jets::jets_wrapper::full_right_shift_16_2,
            Core::FullRightShift16_4 => &simplicity_sys::c_jets::jets_wrapper::full_right_shift_16_4,
            Core::FullRightShift16_8 => &simplicity_sys::c_jets::jets_wrapper::full_right_shift_16_8,
            Core::FullRightShift32_1 => &simplicity_sys::c_jets::jets_wrapper::full_right_shift_32_1,
            Core::FullRightShift32_16 => &simplicity_sys::c_jets::jets_wrapper::full_right_shift_32_16,
            Core::FullRightShift32_2 => &simplicity_sys::c_jets::jets_wrapper::full_right_shift_32_2,
            Core::FullRightShift32_4 => &simplicity_sys::c_jets::jets_wrapper::full_right_shift_32_4,
            Core::FullRightShift32_8 => &simplicity_sys::c_jets::jets_wrapper::full_right_shift_32_8,
            Core::FullRightShift64_1 => &simplicity_sys::c_jets::jets_wrapper::full_right_shift_64_1,
            Core::FullRightShift64_16 => &simplicity_sys::c_jets::jets_wrapper::full_right_shift_64_16,
            Core::FullRightShift64_2 => &simplicity_sys::c_jets::jets_wrapper::full_right_shift_64_2,
            Core::FullRightShift64_32 => &simplicity_sys::c_jets::jets_wrapper::full_right_shift_64_32,
            Core::FullRightShift64_4 => &simplicity_sys::c_jets::jets_wrapper::full_right_shift_64_4,
            Core::FullRightShift64_8 => &simplicity_sys::c_jets::jets_wrapper::full_right_shift_64_8,
            Core::FullRightShift8_1 => &simplicity_sys::c_jets::jets_wrapper::full_right_shift_8_1,
            Core::FullRightShift8_2 => &simplicity_sys::c_jets::jets_wrapper::full_right_shift_8_2,
            Core::FullRightShift8_4 => &simplicity_sys::c_jets::jets_wrapper::full_right_shift_8_4,
            Core::FullSubtract16 => &simplicity_sys::c_jets::jets_wrapper::full_subtract_16,
            Core::FullSubtract32 => &simplicity_sys::c_jets::jets_wrapper::full_subtract_32,
            Core::FullSubtract64 => &simplicity_sys::c_jets::jets_wrapper::full_subtract_64,
            Core::FullSubtract8 => &simplicity_sys::c_jets::jets_wrapper::full_subtract_8,
            Core::GeIsOnCurve => &simplicity_sys::c_jets::jets_wrapper::ge_is_on_curve,
            Core::GeNegate => &simplicity_sys::c_jets::jets_wrapper::ge_negate,
            Core::GejAdd => &simplicity_sys::c_jets::jets_wrapper::gej_add,
            Core::GejDouble => &simplicity_sys::c_jets::jets_wrapper::gej_double,
            Core::GejEquiv => &simplicity_sys::c_jets::jets_wrapper::gej_equiv,
            Core::GejGeAdd => &simplicity_sys::c_jets::jets_wrapper::gej_ge_add,
            Core::GejGeAddEx => &simplicity_sys::c_jets::jets_wrapper::gej_ge_add_ex,
            Core::GejGeEquiv => &simplicity_sys::c_jets::jets_wrapper::gej_ge_equiv,
            Core::GejInfinity => &simplicity_sys::c_jets::jets_wrapper::gej_infinity,
            Core::GejIsInfinity => &simplicity_sys::c_jets::jets_wrapper::gej_is_infinity,
            Core::GejIsOnCurve => &simplicity_sys::c_jets::jets_wrapper::gej_is_on_curve,
            Core::GejNegate => &simplicity_sys::c_jets::jets_wrapper::gej_negate,
            Core::GejNormalize => &simplicity_sys::c_jets::jets_wrapper::gej_normalize,
            Core::GejRescale => &simplicity_sys::c_jets::jets_wrapper::gej_rescale,
            Core::GejXEquiv => &simplicity_sys::c_jets::jets_wrapper::gej_x_equiv,
            Core::GejYIsOdd => &simplicity_sys::c_jets::jets_wrapper::gej_y_is_odd,
            Core::Generate => &simplicity_sys::c_jets::jets_wrapper::generate,
            Core::HashToCurve => &simplicity_sys::c_jets::jets_wrapper::hash_to_curve,
            Core::High1 => &simplicity_sys::c_jets::jets_wrapper::high_1,
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
            Core::LeftExtend16_32 => &simplicity_sys::c_jets::jets_wrapper::left_extend_16_32,
            Core::LeftExtend16_64 => &simplicity_sys::c_jets::jets_wrapper::left_extend_16_64,
            Core::LeftExtend1_16 => &simplicity_sys::c_jets::jets_wrapper::left_extend_1_16,
            Core::LeftExtend1_32 => &simplicity_sys::c_jets::jets_wrapper::left_extend_1_32,
            Core::LeftExtend1_64 => &simplicity_sys::c_jets::jets_wrapper::left_extend_1_64,
            Core::LeftExtend1_8 => &simplicity_sys::c_jets::jets_wrapper::left_extend_1_8,
            Core::LeftExtend32_64 => &simplicity_sys::c_jets::jets_wrapper::left_extend_32_64,
            Core::LeftExtend8_16 => &simplicity_sys::c_jets::jets_wrapper::left_extend_8_16,
            Core::LeftExtend8_32 => &simplicity_sys::c_jets::jets_wrapper::left_extend_8_32,
            Core::LeftExtend8_64 => &simplicity_sys::c_jets::jets_wrapper::left_extend_8_64,
            Core::LeftPadHigh16_32 => &simplicity_sys::c_jets::jets_wrapper::left_pad_high_16_32,
            Core::LeftPadHigh16_64 => &simplicity_sys::c_jets::jets_wrapper::left_pad_high_16_64,
            Core::LeftPadHigh1_16 => &simplicity_sys::c_jets::jets_wrapper::left_pad_high_1_16,
            Core::LeftPadHigh1_32 => &simplicity_sys::c_jets::jets_wrapper::left_pad_high_1_32,
            Core::LeftPadHigh1_64 => &simplicity_sys::c_jets::jets_wrapper::left_pad_high_1_64,
            Core::LeftPadHigh1_8 => &simplicity_sys::c_jets::jets_wrapper::left_pad_high_1_8,
            Core::LeftPadHigh32_64 => &simplicity_sys::c_jets::jets_wrapper::left_pad_high_32_64,
            Core::LeftPadHigh8_16 => &simplicity_sys::c_jets::jets_wrapper::left_pad_high_8_16,
            Core::LeftPadHigh8_32 => &simplicity_sys::c_jets::jets_wrapper::left_pad_high_8_32,
            Core::LeftPadHigh8_64 => &simplicity_sys::c_jets::jets_wrapper::left_pad_high_8_64,
            Core::LeftPadLow16_32 => &simplicity_sys::c_jets::jets_wrapper::left_pad_low_16_32,
            Core::LeftPadLow16_64 => &simplicity_sys::c_jets::jets_wrapper::left_pad_low_16_64,
            Core::LeftPadLow1_16 => &simplicity_sys::c_jets::jets_wrapper::left_pad_low_1_16,
            Core::LeftPadLow1_32 => &simplicity_sys::c_jets::jets_wrapper::left_pad_low_1_32,
            Core::LeftPadLow1_64 => &simplicity_sys::c_jets::jets_wrapper::left_pad_low_1_64,
            Core::LeftPadLow1_8 => &simplicity_sys::c_jets::jets_wrapper::left_pad_low_1_8,
            Core::LeftPadLow32_64 => &simplicity_sys::c_jets::jets_wrapper::left_pad_low_32_64,
            Core::LeftPadLow8_16 => &simplicity_sys::c_jets::jets_wrapper::left_pad_low_8_16,
            Core::LeftPadLow8_32 => &simplicity_sys::c_jets::jets_wrapper::left_pad_low_8_32,
            Core::LeftPadLow8_64 => &simplicity_sys::c_jets::jets_wrapper::left_pad_low_8_64,
            Core::LeftRotate16 => &simplicity_sys::c_jets::jets_wrapper::left_rotate_16,
            Core::LeftRotate32 => &simplicity_sys::c_jets::jets_wrapper::left_rotate_32,
            Core::LeftRotate64 => &simplicity_sys::c_jets::jets_wrapper::left_rotate_64,
            Core::LeftRotate8 => &simplicity_sys::c_jets::jets_wrapper::left_rotate_8,
            Core::LeftShift16 => &simplicity_sys::c_jets::jets_wrapper::left_shift_16,
            Core::LeftShift32 => &simplicity_sys::c_jets::jets_wrapper::left_shift_32,
            Core::LeftShift64 => &simplicity_sys::c_jets::jets_wrapper::left_shift_64,
            Core::LeftShift8 => &simplicity_sys::c_jets::jets_wrapper::left_shift_8,
            Core::LeftShiftWith16 => &simplicity_sys::c_jets::jets_wrapper::left_shift_with_16,
            Core::LeftShiftWith32 => &simplicity_sys::c_jets::jets_wrapper::left_shift_with_32,
            Core::LeftShiftWith64 => &simplicity_sys::c_jets::jets_wrapper::left_shift_with_64,
            Core::LeftShiftWith8 => &simplicity_sys::c_jets::jets_wrapper::left_shift_with_8,
            Core::Leftmost16_1 => &simplicity_sys::c_jets::jets_wrapper::leftmost_16_1,
            Core::Leftmost16_2 => &simplicity_sys::c_jets::jets_wrapper::leftmost_16_2,
            Core::Leftmost16_4 => &simplicity_sys::c_jets::jets_wrapper::leftmost_16_4,
            Core::Leftmost16_8 => &simplicity_sys::c_jets::jets_wrapper::leftmost_16_8,
            Core::Leftmost32_1 => &simplicity_sys::c_jets::jets_wrapper::leftmost_32_1,
            Core::Leftmost32_16 => &simplicity_sys::c_jets::jets_wrapper::leftmost_32_16,
            Core::Leftmost32_2 => &simplicity_sys::c_jets::jets_wrapper::leftmost_32_2,
            Core::Leftmost32_4 => &simplicity_sys::c_jets::jets_wrapper::leftmost_32_4,
            Core::Leftmost32_8 => &simplicity_sys::c_jets::jets_wrapper::leftmost_32_8,
            Core::Leftmost64_1 => &simplicity_sys::c_jets::jets_wrapper::leftmost_64_1,
            Core::Leftmost64_16 => &simplicity_sys::c_jets::jets_wrapper::leftmost_64_16,
            Core::Leftmost64_2 => &simplicity_sys::c_jets::jets_wrapper::leftmost_64_2,
            Core::Leftmost64_32 => &simplicity_sys::c_jets::jets_wrapper::leftmost_64_32,
            Core::Leftmost64_4 => &simplicity_sys::c_jets::jets_wrapper::leftmost_64_4,
            Core::Leftmost64_8 => &simplicity_sys::c_jets::jets_wrapper::leftmost_64_8,
            Core::Leftmost8_1 => &simplicity_sys::c_jets::jets_wrapper::leftmost_8_1,
            Core::Leftmost8_2 => &simplicity_sys::c_jets::jets_wrapper::leftmost_8_2,
            Core::Leftmost8_4 => &simplicity_sys::c_jets::jets_wrapper::leftmost_8_4,
            Core::LinearCombination1 => &simplicity_sys::c_jets::jets_wrapper::linear_combination_1,
            Core::LinearVerify1 => &simplicity_sys::c_jets::jets_wrapper::linear_verify_1,
            Core::Low1 => &simplicity_sys::c_jets::jets_wrapper::low_1,
            Core::Low16 => &simplicity_sys::c_jets::jets_wrapper::low_16,
            Core::Low32 => &simplicity_sys::c_jets::jets_wrapper::low_32,
            Core::Low64 => &simplicity_sys::c_jets::jets_wrapper::low_64,
            Core::Low8 => &simplicity_sys::c_jets::jets_wrapper::low_8,
            Core::Lt16 => &simplicity_sys::c_jets::jets_wrapper::lt_16,
            Core::Lt32 => &simplicity_sys::c_jets::jets_wrapper::lt_32,
            Core::Lt64 => &simplicity_sys::c_jets::jets_wrapper::lt_64,
            Core::Lt8 => &simplicity_sys::c_jets::jets_wrapper::lt_8,
            Core::Maj1 => &simplicity_sys::c_jets::jets_wrapper::maj_1,
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
            Core::Or1 => &simplicity_sys::c_jets::jets_wrapper::or_1,
            Core::Or16 => &simplicity_sys::c_jets::jets_wrapper::or_16,
            Core::Or32 => &simplicity_sys::c_jets::jets_wrapper::or_32,
            Core::Or64 => &simplicity_sys::c_jets::jets_wrapper::or_64,
            Core::Or8 => &simplicity_sys::c_jets::jets_wrapper::or_8,
            Core::ParseLock => &simplicity_sys::c_jets::jets_wrapper::parse_lock,
            Core::ParseSequence => &simplicity_sys::c_jets::jets_wrapper::parse_sequence,
            Core::PointVerify1 => &simplicity_sys::c_jets::jets_wrapper::point_verify_1,
            Core::RightExtend16_32 => &simplicity_sys::c_jets::jets_wrapper::right_extend_16_32,
            Core::RightExtend16_64 => &simplicity_sys::c_jets::jets_wrapper::right_extend_16_64,
            Core::RightExtend32_64 => &simplicity_sys::c_jets::jets_wrapper::right_extend_32_64,
            Core::RightExtend8_16 => &simplicity_sys::c_jets::jets_wrapper::right_extend_8_16,
            Core::RightExtend8_32 => &simplicity_sys::c_jets::jets_wrapper::right_extend_8_32,
            Core::RightExtend8_64 => &simplicity_sys::c_jets::jets_wrapper::right_extend_8_64,
            Core::RightPadHigh16_32 => &simplicity_sys::c_jets::jets_wrapper::right_pad_high_16_32,
            Core::RightPadHigh16_64 => &simplicity_sys::c_jets::jets_wrapper::right_pad_high_16_64,
            Core::RightPadHigh1_16 => &simplicity_sys::c_jets::jets_wrapper::right_pad_high_1_16,
            Core::RightPadHigh1_32 => &simplicity_sys::c_jets::jets_wrapper::right_pad_high_1_32,
            Core::RightPadHigh1_64 => &simplicity_sys::c_jets::jets_wrapper::right_pad_high_1_64,
            Core::RightPadHigh1_8 => &simplicity_sys::c_jets::jets_wrapper::right_pad_high_1_8,
            Core::RightPadHigh32_64 => &simplicity_sys::c_jets::jets_wrapper::right_pad_high_32_64,
            Core::RightPadHigh8_16 => &simplicity_sys::c_jets::jets_wrapper::right_pad_high_8_16,
            Core::RightPadHigh8_32 => &simplicity_sys::c_jets::jets_wrapper::right_pad_high_8_32,
            Core::RightPadHigh8_64 => &simplicity_sys::c_jets::jets_wrapper::right_pad_high_8_64,
            Core::RightPadLow16_32 => &simplicity_sys::c_jets::jets_wrapper::right_pad_low_16_32,
            Core::RightPadLow16_64 => &simplicity_sys::c_jets::jets_wrapper::right_pad_low_16_64,
            Core::RightPadLow1_16 => &simplicity_sys::c_jets::jets_wrapper::right_pad_low_1_16,
            Core::RightPadLow1_32 => &simplicity_sys::c_jets::jets_wrapper::right_pad_low_1_32,
            Core::RightPadLow1_64 => &simplicity_sys::c_jets::jets_wrapper::right_pad_low_1_64,
            Core::RightPadLow1_8 => &simplicity_sys::c_jets::jets_wrapper::right_pad_low_1_8,
            Core::RightPadLow32_64 => &simplicity_sys::c_jets::jets_wrapper::right_pad_low_32_64,
            Core::RightPadLow8_16 => &simplicity_sys::c_jets::jets_wrapper::right_pad_low_8_16,
            Core::RightPadLow8_32 => &simplicity_sys::c_jets::jets_wrapper::right_pad_low_8_32,
            Core::RightPadLow8_64 => &simplicity_sys::c_jets::jets_wrapper::right_pad_low_8_64,
            Core::RightRotate16 => &simplicity_sys::c_jets::jets_wrapper::right_rotate_16,
            Core::RightRotate32 => &simplicity_sys::c_jets::jets_wrapper::right_rotate_32,
            Core::RightRotate64 => &simplicity_sys::c_jets::jets_wrapper::right_rotate_64,
            Core::RightRotate8 => &simplicity_sys::c_jets::jets_wrapper::right_rotate_8,
            Core::RightShift16 => &simplicity_sys::c_jets::jets_wrapper::right_shift_16,
            Core::RightShift32 => &simplicity_sys::c_jets::jets_wrapper::right_shift_32,
            Core::RightShift64 => &simplicity_sys::c_jets::jets_wrapper::right_shift_64,
            Core::RightShift8 => &simplicity_sys::c_jets::jets_wrapper::right_shift_8,
            Core::RightShiftWith16 => &simplicity_sys::c_jets::jets_wrapper::right_shift_with_16,
            Core::RightShiftWith32 => &simplicity_sys::c_jets::jets_wrapper::right_shift_with_32,
            Core::RightShiftWith64 => &simplicity_sys::c_jets::jets_wrapper::right_shift_with_64,
            Core::RightShiftWith8 => &simplicity_sys::c_jets::jets_wrapper::right_shift_with_8,
            Core::Rightmost16_1 => &simplicity_sys::c_jets::jets_wrapper::rightmost_16_1,
            Core::Rightmost16_2 => &simplicity_sys::c_jets::jets_wrapper::rightmost_16_2,
            Core::Rightmost16_4 => &simplicity_sys::c_jets::jets_wrapper::rightmost_16_4,
            Core::Rightmost16_8 => &simplicity_sys::c_jets::jets_wrapper::rightmost_16_8,
            Core::Rightmost32_1 => &simplicity_sys::c_jets::jets_wrapper::rightmost_32_1,
            Core::Rightmost32_16 => &simplicity_sys::c_jets::jets_wrapper::rightmost_32_16,
            Core::Rightmost32_2 => &simplicity_sys::c_jets::jets_wrapper::rightmost_32_2,
            Core::Rightmost32_4 => &simplicity_sys::c_jets::jets_wrapper::rightmost_32_4,
            Core::Rightmost32_8 => &simplicity_sys::c_jets::jets_wrapper::rightmost_32_8,
            Core::Rightmost64_1 => &simplicity_sys::c_jets::jets_wrapper::rightmost_64_1,
            Core::Rightmost64_16 => &simplicity_sys::c_jets::jets_wrapper::rightmost_64_16,
            Core::Rightmost64_2 => &simplicity_sys::c_jets::jets_wrapper::rightmost_64_2,
            Core::Rightmost64_32 => &simplicity_sys::c_jets::jets_wrapper::rightmost_64_32,
            Core::Rightmost64_4 => &simplicity_sys::c_jets::jets_wrapper::rightmost_64_4,
            Core::Rightmost64_8 => &simplicity_sys::c_jets::jets_wrapper::rightmost_64_8,
            Core::Rightmost8_1 => &simplicity_sys::c_jets::jets_wrapper::rightmost_8_1,
            Core::Rightmost8_2 => &simplicity_sys::c_jets::jets_wrapper::rightmost_8_2,
            Core::Rightmost8_4 => &simplicity_sys::c_jets::jets_wrapper::rightmost_8_4,
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
            Core::Some1 => &simplicity_sys::c_jets::jets_wrapper::some_1,
            Core::Some16 => &simplicity_sys::c_jets::jets_wrapper::some_16,
            Core::Some32 => &simplicity_sys::c_jets::jets_wrapper::some_32,
            Core::Some64 => &simplicity_sys::c_jets::jets_wrapper::some_64,
            Core::Some8 => &simplicity_sys::c_jets::jets_wrapper::some_8,
            Core::Subtract16 => &simplicity_sys::c_jets::jets_wrapper::subtract_16,
            Core::Subtract32 => &simplicity_sys::c_jets::jets_wrapper::subtract_32,
            Core::Subtract64 => &simplicity_sys::c_jets::jets_wrapper::subtract_64,
            Core::Subtract8 => &simplicity_sys::c_jets::jets_wrapper::subtract_8,
            Core::Swu => &simplicity_sys::c_jets::jets_wrapper::swu,
            Core::Verify => &simplicity_sys::c_jets::jets_wrapper::verify,
            Core::Xor1 => &simplicity_sys::c_jets::jets_wrapper::xor_1,
            Core::Xor16 => &simplicity_sys::c_jets::jets_wrapper::xor_16,
            Core::Xor32 => &simplicity_sys::c_jets::jets_wrapper::xor_32,
            Core::Xor64 => &simplicity_sys::c_jets::jets_wrapper::xor_64,
            Core::Xor8 => &simplicity_sys::c_jets::jets_wrapper::xor_8,
            Core::XorXor1 => &simplicity_sys::c_jets::jets_wrapper::xor_xor_1,
            Core::XorXor16 => &simplicity_sys::c_jets::jets_wrapper::xor_xor_16,
            Core::XorXor32 => &simplicity_sys::c_jets::jets_wrapper::xor_xor_32,
            Core::XorXor64 => &simplicity_sys::c_jets::jets_wrapper::xor_xor_64,
            Core::XorXor8 => &simplicity_sys::c_jets::jets_wrapper::xor_xor_8,
        }
    }

    fn cost(&self) -> Cost {
        match self {
            Core::Add16 => Cost::from_milliweight(108),
            Core::Add32 => Cost::from_milliweight(117),
            Core::Add64 => Cost::from_milliweight(109),
            Core::Add8 => Cost::from_milliweight(112),
            Core::All16 => Cost::from_milliweight(62),
            Core::All32 => Cost::from_milliweight(65),
            Core::All64 => Cost::from_milliweight(79),
            Core::All8 => Cost::from_milliweight(76),
            Core::And1 => Cost::from_milliweight(79),
            Core::And16 => Cost::from_milliweight(88),
            Core::And32 => Cost::from_milliweight(94),
            Core::And64 => Cost::from_milliweight(93),
            Core::And8 => Cost::from_milliweight(91),
            Core::Bip0340Verify => Cost::from_milliweight(49087),
            Core::Ch1 => Cost::from_milliweight(78),
            Core::Ch16 => Cost::from_milliweight(94),
            Core::Ch32 => Cost::from_milliweight(91),
            Core::Ch64 => Cost::from_milliweight(91),
            Core::Ch8 => Cost::from_milliweight(77),
            Core::CheckSigVerify => Cost::from_milliweight(50000),
            Core::Complement1 => Cost::from_milliweight(79),
            Core::Complement16 => Cost::from_milliweight(75),
            Core::Complement32 => Cost::from_milliweight(93),
            Core::Complement64 => Cost::from_milliweight(88),
            Core::Complement8 => Cost::from_milliweight(80),
            Core::Decompress => Cost::from_milliweight(10861),
            Core::Decrement16 => Cost::from_milliweight(85),
            Core::Decrement32 => Cost::from_milliweight(91),
            Core::Decrement64 => Cost::from_milliweight(89),
            Core::Decrement8 => Cost::from_milliweight(79),
            Core::DivMod128_64 => Cost::from_milliweight(208),
            Core::DivMod16 => Cost::from_milliweight(118),
            Core::DivMod32 => Cost::from_milliweight(115),
            Core::DivMod64 => Cost::from_milliweight(86),
            Core::DivMod8 => Cost::from_milliweight(128),
            Core::Divide16 => Cost::from_milliweight(98),
            Core::Divide32 => Cost::from_milliweight(100),
            Core::Divide64 => Cost::from_milliweight(101),
            Core::Divide8 => Cost::from_milliweight(108),
            Core::Divides16 => Cost::from_milliweight(93),
            Core::Divides32 => Cost::from_milliweight(87),
            Core::Divides64 => Cost::from_milliweight(91),
            Core::Divides8 => Cost::from_milliweight(98),
            Core::Eq1 => Cost::from_milliweight(74),
            Core::Eq16 => Cost::from_milliweight(84),
            Core::Eq256 => Cost::from_milliweight(225),
            Core::Eq32 => Cost::from_milliweight(88),
            Core::Eq64 => Cost::from_milliweight(100),
            Core::Eq8 => Cost::from_milliweight(95),
            Core::FeAdd => Cost::from_milliweight(755),
            Core::FeInvert => Cost::from_milliweight(3175),
            Core::FeIsOdd => Cost::from_milliweight(290),
            Core::FeIsZero => Cost::from_milliweight(268),
            Core::FeMultiply => Cost::from_milliweight(808),
            Core::FeMultiplyBeta => Cost::from_milliweight(579),
            Core::FeNegate => Cost::from_milliweight(531),
            Core::FeNormalize => Cost::from_milliweight(521),
            Core::FeSquare => Cost::from_milliweight(556),
            Core::FeSquareRoot => Cost::from_milliweight(10275),
            Core::FullAdd16 => Cost::from_milliweight(121),
            Core::FullAdd32 => Cost::from_milliweight(119),
            Core::FullAdd64 => Cost::from_milliweight(121),
            Core::FullAdd8 => Cost::from_milliweight(127),
            Core::FullDecrement16 => Cost::from_milliweight(92),
            Core::FullDecrement32 => Cost::from_milliweight(107),
            Core::FullDecrement64 => Cost::from_milliweight(81),
            Core::FullDecrement8 => Cost::from_milliweight(91),
            Core::FullIncrement16 => Cost::from_milliweight(89),
            Core::FullIncrement32 => Cost::from_milliweight(104),
            Core::FullIncrement64 => Cost::from_milliweight(99),
            Core::FullIncrement8 => Cost::from_milliweight(72),
            Core::FullLeftShift16_1 => Cost::from_milliweight(83),
            Core::FullLeftShift16_2 => Cost::from_milliweight(83),
            Core::FullLeftShift16_4 => Cost::from_milliweight(89),
            Core::FullLeftShift16_8 => Cost::from_milliweight(65),
            Core::FullLeftShift32_1 => Cost::from_milliweight(84),
            Core::FullLeftShift32_16 => Cost::from_milliweight(81),
            Core::FullLeftShift32_2 => Cost::from_milliweight(67),
            Core::FullLeftShift32_4 => Cost::from_milliweight(84),
            Core::FullLeftShift32_8 => Cost::from_milliweight(91),
            Core::FullLeftShift64_1 => Cost::from_milliweight(99),
            Core::FullLeftShift64_16 => Cost::from_milliweight(90),
            Core::FullLeftShift64_2 => Cost::from_milliweight(94),
            Core::FullLeftShift64_32 => Cost::from_milliweight(86),
            Core::FullLeftShift64_4 => Cost::from_milliweight(94),
            Core::FullLeftShift64_8 => Cost::from_milliweight(86),
            Core::FullLeftShift8_1 => Cost::from_milliweight(96),
            Core::FullLeftShift8_2 => Cost::from_milliweight(96),
            Core::FullLeftShift8_4 => Cost::from_milliweight(85),
            Core::FullMultiply16 => Cost::from_milliweight(112),
            Core::FullMultiply32 => Cost::from_milliweight(96),
            Core::FullMultiply64 => Cost::from_milliweight(127),
            Core::FullMultiply8 => Cost::from_milliweight(109),
            Core::FullRightShift16_1 => Cost::from_milliweight(80),
            Core::FullRightShift16_2 => Cost::from_milliweight(79),
            Core::FullRightShift16_4 => Cost::from_milliweight(88),
            Core::FullRightShift16_8 => Cost::from_milliweight(57),
            Core::FullRightShift32_1 => Cost::from_milliweight(74),
            Core::FullRightShift32_16 => Cost::from_milliweight(64),
            Core::FullRightShift32_2 => Cost::from_milliweight(63),
            Core::FullRightShift32_4 => Cost::from_milliweight(71),
            Core::FullRightShift32_8 => Cost::from_milliweight(84),
            Core::FullRightShift64_1 => Cost::from_milliweight(99),
            Core::FullRightShift64_16 => Cost::from_milliweight(86),
            Core::FullRightShift64_2 => Cost::from_milliweight(86),
            Core::FullRightShift64_32 => Cost::from_milliweight(73),
            Core::FullRightShift64_4 => Cost::from_milliweight(93),
            Core::FullRightShift64_8 => Cost::from_milliweight(99),
            Core::FullRightShift8_1 => Cost::from_milliweight(88),
            Core::FullRightShift8_2 => Cost::from_milliweight(86),
            Core::FullRightShift8_4 => Cost::from_milliweight(89),
            Core::FullSubtract16 => Cost::from_milliweight(121),
            Core::FullSubtract32 => Cost::from_milliweight(116),
            Core::FullSubtract64 => Cost::from_milliweight(98),
            Core::FullSubtract8 => Cost::from_milliweight(126),
            Core::GeIsOnCurve => Cost::from_milliweight(642),
            Core::GeNegate => Cost::from_milliweight(945),
            Core::GejAdd => Cost::from_milliweight(2897),
            Core::GejDouble => Cost::from_milliweight(1764),
            Core::GejEquiv => Cost::from_milliweight(2220),
            Core::GejGeAdd => Cost::from_milliweight(2477),
            Core::GejGeAddEx => Cost::from_milliweight(2719),
            Core::GejGeEquiv => Cost::from_milliweight(1765),
            Core::GejInfinity => Cost::from_milliweight(716),
            Core::GejIsInfinity => Cost::from_milliweight(666),
            Core::GejIsOnCurve => Cost::from_milliweight(1016),
            Core::GejNegate => Cost::from_milliweight(1381),
            Core::GejNormalize => Cost::from_milliweight(4099),
            Core::GejRescale => Cost::from_milliweight(1908),
            Core::GejXEquiv => Cost::from_milliweight(1047),
            Core::GejYIsOdd => Cost::from_milliweight(3651),
            Core::Generate => Cost::from_milliweight(50071),
            Core::HashToCurve => Cost::from_milliweight(68094),
            Core::High1 => Cost::from_milliweight(57),
            Core::High16 => Cost::from_milliweight(66),
            Core::High32 => Cost::from_milliweight(58),
            Core::High64 => Cost::from_milliweight(68),
            Core::High8 => Cost::from_milliweight(59),
            Core::Increment16 => Cost::from_milliweight(69),
            Core::Increment32 => Cost::from_milliweight(92),
            Core::Increment64 => Cost::from_milliweight(87),
            Core::Increment8 => Cost::from_milliweight(85),
            Core::IsOne16 => Cost::from_milliweight(82),
            Core::IsOne32 => Cost::from_milliweight(65),
            Core::IsOne64 => Cost::from_milliweight(83),
            Core::IsOne8 => Cost::from_milliweight(91),
            Core::IsZero16 => Cost::from_milliweight(75),
            Core::IsZero32 => Cost::from_milliweight(85),
            Core::IsZero64 => Cost::from_milliweight(80),
            Core::IsZero8 => Cost::from_milliweight(77),
            Core::Le16 => Cost::from_milliweight(112),
            Core::Le32 => Cost::from_milliweight(93),
            Core::Le64 => Cost::from_milliweight(93),
            Core::Le8 => Cost::from_milliweight(109),
            Core::LeftExtend16_32 => Cost::from_milliweight(86),
            Core::LeftExtend16_64 => Cost::from_milliweight(89),
            Core::LeftExtend1_16 => Cost::from_milliweight(67),
            Core::LeftExtend1_32 => Cost::from_milliweight(60),
            Core::LeftExtend1_64 => Cost::from_milliweight(76),
            Core::LeftExtend1_8 => Cost::from_milliweight(65),
            Core::LeftExtend32_64 => Cost::from_milliweight(63),
            Core::LeftExtend8_16 => Cost::from_milliweight(88),
            Core::LeftExtend8_32 => Cost::from_milliweight(90),
            Core::LeftExtend8_64 => Cost::from_milliweight(107),
            Core::LeftPadHigh16_32 => Cost::from_milliweight(91),
            Core::LeftPadHigh16_64 => Cost::from_milliweight(110),
            Core::LeftPadHigh1_16 => Cost::from_milliweight(141),
            Core::LeftPadHigh1_32 => Cost::from_milliweight(263),
            Core::LeftPadHigh1_64 => Cost::from_milliweight(422),
            Core::LeftPadHigh1_8 => Cost::from_milliweight(99),
            Core::LeftPadHigh32_64 => Cost::from_milliweight(93),
            Core::LeftPadHigh8_16 => Cost::from_milliweight(88),
            Core::LeftPadHigh8_32 => Cost::from_milliweight(103),
            Core::LeftPadHigh8_64 => Cost::from_milliweight(136),
            Core::LeftPadLow16_32 => Cost::from_milliweight(69),
            Core::LeftPadLow16_64 => Cost::from_milliweight(106),
            Core::LeftPadLow1_16 => Cost::from_milliweight(65),
            Core::LeftPadLow1_32 => Cost::from_milliweight(63),
            Core::LeftPadLow1_64 => Cost::from_milliweight(61),
            Core::LeftPadLow1_8 => Cost::from_milliweight(56),
            Core::LeftPadLow32_64 => Cost::from_milliweight(91),
            Core::LeftPadLow8_16 => Cost::from_milliweight(66),
            Core::LeftPadLow8_32 => Cost::from_milliweight(61),
            Core::LeftPadLow8_64 => Cost::from_milliweight(112),
            Core::LeftRotate16 => Cost::from_milliweight(77),
            Core::LeftRotate32 => Cost::from_milliweight(106),
            Core::LeftRotate64 => Cost::from_milliweight(98),
            Core::LeftRotate8 => Cost::from_milliweight(88),
            Core::LeftShift16 => Cost::from_milliweight(72),
            Core::LeftShift32 => Cost::from_milliweight(78),
            Core::LeftShift64 => Cost::from_milliweight(82),
            Core::LeftShift8 => Cost::from_milliweight(91),
            Core::LeftShiftWith16 => Cost::from_milliweight(83),
            Core::LeftShiftWith32 => Cost::from_milliweight(95),
            Core::LeftShiftWith64 => Cost::from_milliweight(103),
            Core::LeftShiftWith8 => Cost::from_milliweight(107),
            Core::Leftmost16_1 => Cost::from_milliweight(93),
            Core::Leftmost16_2 => Cost::from_milliweight(90),
            Core::Leftmost16_4 => Cost::from_milliweight(75),
            Core::Leftmost16_8 => Cost::from_milliweight(71),
            Core::Leftmost32_1 => Cost::from_milliweight(77),
            Core::Leftmost32_16 => Cost::from_milliweight(102),
            Core::Leftmost32_2 => Cost::from_milliweight(66),
            Core::Leftmost32_4 => Cost::from_milliweight(52),
            Core::Leftmost32_8 => Cost::from_milliweight(103),
            Core::Leftmost64_1 => Cost::from_milliweight(78),
            Core::Leftmost64_16 => Cost::from_milliweight(88),
            Core::Leftmost64_2 => Cost::from_milliweight(71),
            Core::Leftmost64_32 => Cost::from_milliweight(90),
            Core::Leftmost64_4 => Cost::from_milliweight(79),
            Core::Leftmost64_8 => Cost::from_milliweight(86),
            Core::Leftmost8_1 => Cost::from_milliweight(90),
            Core::Leftmost8_2 => Cost::from_milliweight(90),
            Core::Leftmost8_4 => Cost::from_milliweight(87),
            Core::LinearCombination1 => Cost::from_milliweight(84674),
            Core::LinearVerify1 => Cost::from_milliweight(43364),
            Core::Low1 => Cost::from_milliweight(38),
            Core::Low16 => Cost::from_milliweight(69),
            Core::Low32 => Cost::from_milliweight(62),
            Core::Low64 => Cost::from_milliweight(47),
            Core::Low8 => Cost::from_milliweight(47),
            Core::Lt16 => Cost::from_milliweight(123),
            Core::Lt32 => Cost::from_milliweight(107),
            Core::Lt64 => Cost::from_milliweight(76),
            Core::Lt8 => Cost::from_milliweight(107),
            Core::Maj1 => Cost::from_milliweight(62),
            Core::Maj16 => Cost::from_milliweight(80),
            Core::Maj32 => Cost::from_milliweight(96),
            Core::Maj64 => Cost::from_milliweight(93),
            Core::Maj8 => Cost::from_milliweight(94),
            Core::Max16 => Cost::from_milliweight(114),
            Core::Max32 => Cost::from_milliweight(92),
            Core::Max64 => Cost::from_milliweight(104),
            Core::Max8 => Cost::from_milliweight(96),
            Core::Median16 => Cost::from_milliweight(123),
            Core::Median32 => Cost::from_milliweight(101),
            Core::Median64 => Cost::from_milliweight(109),
            Core::Median8 => Cost::from_milliweight(122),
            Core::Min16 => Cost::from_milliweight(97),
            Core::Min32 => Cost::from_milliweight(113),
            Core::Min64 => Cost::from_milliweight(102),
            Core::Min8 => Cost::from_milliweight(99),
            Core::Modulo16 => Cost::from_milliweight(103),
            Core::Modulo32 => Cost::from_milliweight(102),
            Core::Modulo64 => Cost::from_milliweight(85),
            Core::Modulo8 => Cost::from_milliweight(102),
            Core::Multiply16 => Cost::from_milliweight(90),
            Core::Multiply32 => Cost::from_milliweight(90),
            Core::Multiply64 => Cost::from_milliweight(85),
            Core::Multiply8 => Cost::from_milliweight(93),
            Core::Negate16 => Cost::from_milliweight(70),
            Core::Negate32 => Cost::from_milliweight(85),
            Core::Negate64 => Cost::from_milliweight(94),
            Core::Negate8 => Cost::from_milliweight(91),
            Core::One16 => Cost::from_milliweight(60),
            Core::One32 => Cost::from_milliweight(59),
            Core::One64 => Cost::from_milliweight(59),
            Core::One8 => Cost::from_milliweight(62),
            Core::Or1 => Cost::from_milliweight(77),
            Core::Or16 => Cost::from_milliweight(94),
            Core::Or32 => Cost::from_milliweight(105),
            Core::Or64 => Cost::from_milliweight(99),
            Core::Or8 => Cost::from_milliweight(93),
            Core::ParseLock => Cost::from_milliweight(97),
            Core::ParseSequence => Cost::from_milliweight(116),
            Core::PointVerify1 => Cost::from_milliweight(41494),
            Core::RightExtend16_32 => Cost::from_milliweight(74),
            Core::RightExtend16_64 => Cost::from_milliweight(82),
            Core::RightExtend32_64 => Cost::from_milliweight(94),
            Core::RightExtend8_16 => Cost::from_milliweight(76),
            Core::RightExtend8_32 => Cost::from_milliweight(106),
            Core::RightExtend8_64 => Cost::from_milliweight(124),
            Core::RightPadHigh16_32 => Cost::from_milliweight(70),
            Core::RightPadHigh16_64 => Cost::from_milliweight(88),
            Core::RightPadHigh1_16 => Cost::from_milliweight(143),
            Core::RightPadHigh1_32 => Cost::from_milliweight(223),
            Core::RightPadHigh1_64 => Cost::from_milliweight(476),
            Core::RightPadHigh1_8 => Cost::from_milliweight(107),
            Core::RightPadHigh32_64 => Cost::from_milliweight(94),
            Core::RightPadHigh8_16 => Cost::from_milliweight(89),
            Core::RightPadHigh8_32 => Cost::from_milliweight(110),
            Core::RightPadHigh8_64 => Cost::from_milliweight(107),
            Core::RightPadLow16_32 => Cost::from_milliweight(71),
            Core::RightPadLow16_64 => Cost::from_milliweight(96),
            Core::RightPadLow1_16 => Cost::from_milliweight(81),
            Core::RightPadLow1_32 => Cost::from_milliweight(75),
            Core::RightPadLow1_64 => Cost::from_milliweight(73),
            Core::RightPadLow1_8 => Cost::from_milliweight(68),
            Core::RightPadLow32_64 => Cost::from_milliweight(80),
            Core::RightPadLow8_16 => Cost::from_milliweight(75),
            Core::RightPadLow8_32 => Cost::from_milliweight(77),
            Core::RightPadLow8_64 => Cost::from_milliweight(82),
            Core::RightRotate16 => Cost::from_milliweight(99),
            Core::RightRotate32 => Cost::from_milliweight(92),
            Core::RightRotate64 => Cost::from_milliweight(93),
            Core::RightRotate8 => Cost::from_milliweight(75),
            Core::RightShift16 => Cost::from_milliweight(84),
            Core::RightShift32 => Cost::from_milliweight(88),
            Core::RightShift64 => Cost::from_milliweight(91),
            Core::RightShift8 => Cost::from_milliweight(88),
            Core::RightShiftWith16 => Cost::from_milliweight(105),
            Core::RightShiftWith32 => Cost::from_milliweight(92),
            Core::RightShiftWith64 => Cost::from_milliweight(97),
            Core::RightShiftWith8 => Cost::from_milliweight(103),
            Core::Rightmost16_1 => Cost::from_milliweight(70),
            Core::Rightmost16_2 => Cost::from_milliweight(82),
            Core::Rightmost16_4 => Cost::from_milliweight(76),
            Core::Rightmost16_8 => Cost::from_milliweight(69),
            Core::Rightmost32_1 => Cost::from_milliweight(90),
            Core::Rightmost32_16 => Cost::from_milliweight(64),
            Core::Rightmost32_2 => Cost::from_milliweight(74),
            Core::Rightmost32_4 => Cost::from_milliweight(92),
            Core::Rightmost32_8 => Cost::from_milliweight(78),
            Core::Rightmost64_1 => Cost::from_milliweight(77),
            Core::Rightmost64_16 => Cost::from_milliweight(86),
            Core::Rightmost64_2 => Cost::from_milliweight(74),
            Core::Rightmost64_32 => Cost::from_milliweight(76),
            Core::Rightmost64_4 => Cost::from_milliweight(70),
            Core::Rightmost64_8 => Cost::from_milliweight(69),
            Core::Rightmost8_1 => Cost::from_milliweight(79),
            Core::Rightmost8_2 => Cost::from_milliweight(98),
            Core::Rightmost8_4 => Cost::from_milliweight(98),
            Core::ScalarAdd => Cost::from_milliweight(739),
            Core::ScalarInvert => Cost::from_milliweight(3193),
            Core::ScalarIsZero => Cost::from_milliweight(271),
            Core::ScalarMultiply => Cost::from_milliweight(774),
            Core::ScalarMultiplyLambda => Cost::from_milliweight(557),
            Core::ScalarNegate => Cost::from_milliweight(490),
            Core::ScalarNormalize => Cost::from_milliweight(472),
            Core::ScalarSquare => Cost::from_milliweight(575),
            Core::Scale => Cost::from_milliweight(72675),
            Core::Sha256Block => Cost::from_milliweight(771),
            Core::Sha256Ctx8Add1 => Cost::from_milliweight(642),
            Core::Sha256Ctx8Add128 => Cost::from_milliweight(1779),
            Core::Sha256Ctx8Add16 => Cost::from_milliweight(747),
            Core::Sha256Ctx8Add2 => Cost::from_milliweight(661),
            Core::Sha256Ctx8Add256 => Cost::from_milliweight(2912),
            Core::Sha256Ctx8Add32 => Cost::from_milliweight(896),
            Core::Sha256Ctx8Add4 => Cost::from_milliweight(645),
            Core::Sha256Ctx8Add512 => Cost::from_milliweight(5299),
            Core::Sha256Ctx8Add64 => Cost::from_milliweight(1187),
            Core::Sha256Ctx8Add8 => Cost::from_milliweight(674),
            Core::Sha256Ctx8AddBuffer511 => Cost::from_milliweight(5060),
            Core::Sha256Ctx8Finalize => Cost::from_milliweight(835),
            Core::Sha256Ctx8Init => Cost::from_milliweight(118),
            Core::Sha256Iv => Cost::from_milliweight(93),
            Core::Some1 => Cost::from_milliweight(70),
            Core::Some16 => Cost::from_milliweight(63),
            Core::Some32 => Cost::from_milliweight(64),
            Core::Some64 => Cost::from_milliweight(93),
            Core::Some8 => Cost::from_milliweight(75),
            Core::Subtract16 => Cost::from_milliweight(113),
            Core::Subtract32 => Cost::from_milliweight(118),
            Core::Subtract64 => Cost::from_milliweight(115),
            Core::Subtract8 => Cost::from_milliweight(109),
            Core::Swu => Cost::from_milliweight(32120),
            Core::Verify => Cost::from_milliweight(57),
            Core::Xor1 => Cost::from_milliweight(67),
            Core::Xor16 => Cost::from_milliweight(83),
            Core::Xor32 => Cost::from_milliweight(92),
            Core::Xor64 => Cost::from_milliweight(95),
            Core::Xor8 => Cost::from_milliweight(85),
            Core::XorXor1 => Cost::from_milliweight(72),
            Core::XorXor16 => Cost::from_milliweight(79),
            Core::XorXor32 => Cost::from_milliweight(96),
            Core::XorXor64 => Cost::from_milliweight(93),
            Core::XorXor8 => Cost::from_milliweight(98),
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
            Core::And1 => f.write_str("and_1"),
            Core::And16 => f.write_str("and_16"),
            Core::And32 => f.write_str("and_32"),
            Core::And64 => f.write_str("and_64"),
            Core::And8 => f.write_str("and_8"),
            Core::Bip0340Verify => f.write_str("bip_0340_verify"),
            Core::Ch1 => f.write_str("ch_1"),
            Core::Ch16 => f.write_str("ch_16"),
            Core::Ch32 => f.write_str("ch_32"),
            Core::Ch64 => f.write_str("ch_64"),
            Core::Ch8 => f.write_str("ch_8"),
            Core::CheckSigVerify => f.write_str("check_sig_verify"),
            Core::Complement1 => f.write_str("complement_1"),
            Core::Complement16 => f.write_str("complement_16"),
            Core::Complement32 => f.write_str("complement_32"),
            Core::Complement64 => f.write_str("complement_64"),
            Core::Complement8 => f.write_str("complement_8"),
            Core::Decompress => f.write_str("decompress"),
            Core::Decrement16 => f.write_str("decrement_16"),
            Core::Decrement32 => f.write_str("decrement_32"),
            Core::Decrement64 => f.write_str("decrement_64"),
            Core::Decrement8 => f.write_str("decrement_8"),
            Core::DivMod128_64 => f.write_str("div_mod_128_64"),
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
            Core::Eq1 => f.write_str("eq_1"),
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
            Core::FullLeftShift16_1 => f.write_str("full_left_shift_16_1"),
            Core::FullLeftShift16_2 => f.write_str("full_left_shift_16_2"),
            Core::FullLeftShift16_4 => f.write_str("full_left_shift_16_4"),
            Core::FullLeftShift16_8 => f.write_str("full_left_shift_16_8"),
            Core::FullLeftShift32_1 => f.write_str("full_left_shift_32_1"),
            Core::FullLeftShift32_16 => f.write_str("full_left_shift_32_16"),
            Core::FullLeftShift32_2 => f.write_str("full_left_shift_32_2"),
            Core::FullLeftShift32_4 => f.write_str("full_left_shift_32_4"),
            Core::FullLeftShift32_8 => f.write_str("full_left_shift_32_8"),
            Core::FullLeftShift64_1 => f.write_str("full_left_shift_64_1"),
            Core::FullLeftShift64_16 => f.write_str("full_left_shift_64_16"),
            Core::FullLeftShift64_2 => f.write_str("full_left_shift_64_2"),
            Core::FullLeftShift64_32 => f.write_str("full_left_shift_64_32"),
            Core::FullLeftShift64_4 => f.write_str("full_left_shift_64_4"),
            Core::FullLeftShift64_8 => f.write_str("full_left_shift_64_8"),
            Core::FullLeftShift8_1 => f.write_str("full_left_shift_8_1"),
            Core::FullLeftShift8_2 => f.write_str("full_left_shift_8_2"),
            Core::FullLeftShift8_4 => f.write_str("full_left_shift_8_4"),
            Core::FullMultiply16 => f.write_str("full_multiply_16"),
            Core::FullMultiply32 => f.write_str("full_multiply_32"),
            Core::FullMultiply64 => f.write_str("full_multiply_64"),
            Core::FullMultiply8 => f.write_str("full_multiply_8"),
            Core::FullRightShift16_1 => f.write_str("full_right_shift_16_1"),
            Core::FullRightShift16_2 => f.write_str("full_right_shift_16_2"),
            Core::FullRightShift16_4 => f.write_str("full_right_shift_16_4"),
            Core::FullRightShift16_8 => f.write_str("full_right_shift_16_8"),
            Core::FullRightShift32_1 => f.write_str("full_right_shift_32_1"),
            Core::FullRightShift32_16 => f.write_str("full_right_shift_32_16"),
            Core::FullRightShift32_2 => f.write_str("full_right_shift_32_2"),
            Core::FullRightShift32_4 => f.write_str("full_right_shift_32_4"),
            Core::FullRightShift32_8 => f.write_str("full_right_shift_32_8"),
            Core::FullRightShift64_1 => f.write_str("full_right_shift_64_1"),
            Core::FullRightShift64_16 => f.write_str("full_right_shift_64_16"),
            Core::FullRightShift64_2 => f.write_str("full_right_shift_64_2"),
            Core::FullRightShift64_32 => f.write_str("full_right_shift_64_32"),
            Core::FullRightShift64_4 => f.write_str("full_right_shift_64_4"),
            Core::FullRightShift64_8 => f.write_str("full_right_shift_64_8"),
            Core::FullRightShift8_1 => f.write_str("full_right_shift_8_1"),
            Core::FullRightShift8_2 => f.write_str("full_right_shift_8_2"),
            Core::FullRightShift8_4 => f.write_str("full_right_shift_8_4"),
            Core::FullSubtract16 => f.write_str("full_subtract_16"),
            Core::FullSubtract32 => f.write_str("full_subtract_32"),
            Core::FullSubtract64 => f.write_str("full_subtract_64"),
            Core::FullSubtract8 => f.write_str("full_subtract_8"),
            Core::GeIsOnCurve => f.write_str("ge_is_on_curve"),
            Core::GeNegate => f.write_str("ge_negate"),
            Core::GejAdd => f.write_str("gej_add"),
            Core::GejDouble => f.write_str("gej_double"),
            Core::GejEquiv => f.write_str("gej_equiv"),
            Core::GejGeAdd => f.write_str("gej_ge_add"),
            Core::GejGeAddEx => f.write_str("gej_ge_add_ex"),
            Core::GejGeEquiv => f.write_str("gej_ge_equiv"),
            Core::GejInfinity => f.write_str("gej_infinity"),
            Core::GejIsInfinity => f.write_str("gej_is_infinity"),
            Core::GejIsOnCurve => f.write_str("gej_is_on_curve"),
            Core::GejNegate => f.write_str("gej_negate"),
            Core::GejNormalize => f.write_str("gej_normalize"),
            Core::GejRescale => f.write_str("gej_rescale"),
            Core::GejXEquiv => f.write_str("gej_x_equiv"),
            Core::GejYIsOdd => f.write_str("gej_y_is_odd"),
            Core::Generate => f.write_str("generate"),
            Core::HashToCurve => f.write_str("hash_to_curve"),
            Core::High1 => f.write_str("high_1"),
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
            Core::LeftExtend16_32 => f.write_str("left_extend_16_32"),
            Core::LeftExtend16_64 => f.write_str("left_extend_16_64"),
            Core::LeftExtend1_16 => f.write_str("left_extend_1_16"),
            Core::LeftExtend1_32 => f.write_str("left_extend_1_32"),
            Core::LeftExtend1_64 => f.write_str("left_extend_1_64"),
            Core::LeftExtend1_8 => f.write_str("left_extend_1_8"),
            Core::LeftExtend32_64 => f.write_str("left_extend_32_64"),
            Core::LeftExtend8_16 => f.write_str("left_extend_8_16"),
            Core::LeftExtend8_32 => f.write_str("left_extend_8_32"),
            Core::LeftExtend8_64 => f.write_str("left_extend_8_64"),
            Core::LeftPadHigh16_32 => f.write_str("left_pad_high_16_32"),
            Core::LeftPadHigh16_64 => f.write_str("left_pad_high_16_64"),
            Core::LeftPadHigh1_16 => f.write_str("left_pad_high_1_16"),
            Core::LeftPadHigh1_32 => f.write_str("left_pad_high_1_32"),
            Core::LeftPadHigh1_64 => f.write_str("left_pad_high_1_64"),
            Core::LeftPadHigh1_8 => f.write_str("left_pad_high_1_8"),
            Core::LeftPadHigh32_64 => f.write_str("left_pad_high_32_64"),
            Core::LeftPadHigh8_16 => f.write_str("left_pad_high_8_16"),
            Core::LeftPadHigh8_32 => f.write_str("left_pad_high_8_32"),
            Core::LeftPadHigh8_64 => f.write_str("left_pad_high_8_64"),
            Core::LeftPadLow16_32 => f.write_str("left_pad_low_16_32"),
            Core::LeftPadLow16_64 => f.write_str("left_pad_low_16_64"),
            Core::LeftPadLow1_16 => f.write_str("left_pad_low_1_16"),
            Core::LeftPadLow1_32 => f.write_str("left_pad_low_1_32"),
            Core::LeftPadLow1_64 => f.write_str("left_pad_low_1_64"),
            Core::LeftPadLow1_8 => f.write_str("left_pad_low_1_8"),
            Core::LeftPadLow32_64 => f.write_str("left_pad_low_32_64"),
            Core::LeftPadLow8_16 => f.write_str("left_pad_low_8_16"),
            Core::LeftPadLow8_32 => f.write_str("left_pad_low_8_32"),
            Core::LeftPadLow8_64 => f.write_str("left_pad_low_8_64"),
            Core::LeftRotate16 => f.write_str("left_rotate_16"),
            Core::LeftRotate32 => f.write_str("left_rotate_32"),
            Core::LeftRotate64 => f.write_str("left_rotate_64"),
            Core::LeftRotate8 => f.write_str("left_rotate_8"),
            Core::LeftShift16 => f.write_str("left_shift_16"),
            Core::LeftShift32 => f.write_str("left_shift_32"),
            Core::LeftShift64 => f.write_str("left_shift_64"),
            Core::LeftShift8 => f.write_str("left_shift_8"),
            Core::LeftShiftWith16 => f.write_str("left_shift_with_16"),
            Core::LeftShiftWith32 => f.write_str("left_shift_with_32"),
            Core::LeftShiftWith64 => f.write_str("left_shift_with_64"),
            Core::LeftShiftWith8 => f.write_str("left_shift_with_8"),
            Core::Leftmost16_1 => f.write_str("leftmost_16_1"),
            Core::Leftmost16_2 => f.write_str("leftmost_16_2"),
            Core::Leftmost16_4 => f.write_str("leftmost_16_4"),
            Core::Leftmost16_8 => f.write_str("leftmost_16_8"),
            Core::Leftmost32_1 => f.write_str("leftmost_32_1"),
            Core::Leftmost32_16 => f.write_str("leftmost_32_16"),
            Core::Leftmost32_2 => f.write_str("leftmost_32_2"),
            Core::Leftmost32_4 => f.write_str("leftmost_32_4"),
            Core::Leftmost32_8 => f.write_str("leftmost_32_8"),
            Core::Leftmost64_1 => f.write_str("leftmost_64_1"),
            Core::Leftmost64_16 => f.write_str("leftmost_64_16"),
            Core::Leftmost64_2 => f.write_str("leftmost_64_2"),
            Core::Leftmost64_32 => f.write_str("leftmost_64_32"),
            Core::Leftmost64_4 => f.write_str("leftmost_64_4"),
            Core::Leftmost64_8 => f.write_str("leftmost_64_8"),
            Core::Leftmost8_1 => f.write_str("leftmost_8_1"),
            Core::Leftmost8_2 => f.write_str("leftmost_8_2"),
            Core::Leftmost8_4 => f.write_str("leftmost_8_4"),
            Core::LinearCombination1 => f.write_str("linear_combination_1"),
            Core::LinearVerify1 => f.write_str("linear_verify_1"),
            Core::Low1 => f.write_str("low_1"),
            Core::Low16 => f.write_str("low_16"),
            Core::Low32 => f.write_str("low_32"),
            Core::Low64 => f.write_str("low_64"),
            Core::Low8 => f.write_str("low_8"),
            Core::Lt16 => f.write_str("lt_16"),
            Core::Lt32 => f.write_str("lt_32"),
            Core::Lt64 => f.write_str("lt_64"),
            Core::Lt8 => f.write_str("lt_8"),
            Core::Maj1 => f.write_str("maj_1"),
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
            Core::Or1 => f.write_str("or_1"),
            Core::Or16 => f.write_str("or_16"),
            Core::Or32 => f.write_str("or_32"),
            Core::Or64 => f.write_str("or_64"),
            Core::Or8 => f.write_str("or_8"),
            Core::ParseLock => f.write_str("parse_lock"),
            Core::ParseSequence => f.write_str("parse_sequence"),
            Core::PointVerify1 => f.write_str("point_verify_1"),
            Core::RightExtend16_32 => f.write_str("right_extend_16_32"),
            Core::RightExtend16_64 => f.write_str("right_extend_16_64"),
            Core::RightExtend32_64 => f.write_str("right_extend_32_64"),
            Core::RightExtend8_16 => f.write_str("right_extend_8_16"),
            Core::RightExtend8_32 => f.write_str("right_extend_8_32"),
            Core::RightExtend8_64 => f.write_str("right_extend_8_64"),
            Core::RightPadHigh16_32 => f.write_str("right_pad_high_16_32"),
            Core::RightPadHigh16_64 => f.write_str("right_pad_high_16_64"),
            Core::RightPadHigh1_16 => f.write_str("right_pad_high_1_16"),
            Core::RightPadHigh1_32 => f.write_str("right_pad_high_1_32"),
            Core::RightPadHigh1_64 => f.write_str("right_pad_high_1_64"),
            Core::RightPadHigh1_8 => f.write_str("right_pad_high_1_8"),
            Core::RightPadHigh32_64 => f.write_str("right_pad_high_32_64"),
            Core::RightPadHigh8_16 => f.write_str("right_pad_high_8_16"),
            Core::RightPadHigh8_32 => f.write_str("right_pad_high_8_32"),
            Core::RightPadHigh8_64 => f.write_str("right_pad_high_8_64"),
            Core::RightPadLow16_32 => f.write_str("right_pad_low_16_32"),
            Core::RightPadLow16_64 => f.write_str("right_pad_low_16_64"),
            Core::RightPadLow1_16 => f.write_str("right_pad_low_1_16"),
            Core::RightPadLow1_32 => f.write_str("right_pad_low_1_32"),
            Core::RightPadLow1_64 => f.write_str("right_pad_low_1_64"),
            Core::RightPadLow1_8 => f.write_str("right_pad_low_1_8"),
            Core::RightPadLow32_64 => f.write_str("right_pad_low_32_64"),
            Core::RightPadLow8_16 => f.write_str("right_pad_low_8_16"),
            Core::RightPadLow8_32 => f.write_str("right_pad_low_8_32"),
            Core::RightPadLow8_64 => f.write_str("right_pad_low_8_64"),
            Core::RightRotate16 => f.write_str("right_rotate_16"),
            Core::RightRotate32 => f.write_str("right_rotate_32"),
            Core::RightRotate64 => f.write_str("right_rotate_64"),
            Core::RightRotate8 => f.write_str("right_rotate_8"),
            Core::RightShift16 => f.write_str("right_shift_16"),
            Core::RightShift32 => f.write_str("right_shift_32"),
            Core::RightShift64 => f.write_str("right_shift_64"),
            Core::RightShift8 => f.write_str("right_shift_8"),
            Core::RightShiftWith16 => f.write_str("right_shift_with_16"),
            Core::RightShiftWith32 => f.write_str("right_shift_with_32"),
            Core::RightShiftWith64 => f.write_str("right_shift_with_64"),
            Core::RightShiftWith8 => f.write_str("right_shift_with_8"),
            Core::Rightmost16_1 => f.write_str("rightmost_16_1"),
            Core::Rightmost16_2 => f.write_str("rightmost_16_2"),
            Core::Rightmost16_4 => f.write_str("rightmost_16_4"),
            Core::Rightmost16_8 => f.write_str("rightmost_16_8"),
            Core::Rightmost32_1 => f.write_str("rightmost_32_1"),
            Core::Rightmost32_16 => f.write_str("rightmost_32_16"),
            Core::Rightmost32_2 => f.write_str("rightmost_32_2"),
            Core::Rightmost32_4 => f.write_str("rightmost_32_4"),
            Core::Rightmost32_8 => f.write_str("rightmost_32_8"),
            Core::Rightmost64_1 => f.write_str("rightmost_64_1"),
            Core::Rightmost64_16 => f.write_str("rightmost_64_16"),
            Core::Rightmost64_2 => f.write_str("rightmost_64_2"),
            Core::Rightmost64_32 => f.write_str("rightmost_64_32"),
            Core::Rightmost64_4 => f.write_str("rightmost_64_4"),
            Core::Rightmost64_8 => f.write_str("rightmost_64_8"),
            Core::Rightmost8_1 => f.write_str("rightmost_8_1"),
            Core::Rightmost8_2 => f.write_str("rightmost_8_2"),
            Core::Rightmost8_4 => f.write_str("rightmost_8_4"),
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
            Core::Some1 => f.write_str("some_1"),
            Core::Some16 => f.write_str("some_16"),
            Core::Some32 => f.write_str("some_32"),
            Core::Some64 => f.write_str("some_64"),
            Core::Some8 => f.write_str("some_8"),
            Core::Subtract16 => f.write_str("subtract_16"),
            Core::Subtract32 => f.write_str("subtract_32"),
            Core::Subtract64 => f.write_str("subtract_64"),
            Core::Subtract8 => f.write_str("subtract_8"),
            Core::Swu => f.write_str("swu"),
            Core::Verify => f.write_str("verify"),
            Core::Xor1 => f.write_str("xor_1"),
            Core::Xor16 => f.write_str("xor_16"),
            Core::Xor32 => f.write_str("xor_32"),
            Core::Xor64 => f.write_str("xor_64"),
            Core::Xor8 => f.write_str("xor_8"),
            Core::XorXor1 => f.write_str("xor_xor_1"),
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
            "and_1" => Ok(Core::And1),
            "and_16" => Ok(Core::And16),
            "and_32" => Ok(Core::And32),
            "and_64" => Ok(Core::And64),
            "and_8" => Ok(Core::And8),
            "bip_0340_verify" => Ok(Core::Bip0340Verify),
            "ch_1" => Ok(Core::Ch1),
            "ch_16" => Ok(Core::Ch16),
            "ch_32" => Ok(Core::Ch32),
            "ch_64" => Ok(Core::Ch64),
            "ch_8" => Ok(Core::Ch8),
            "check_sig_verify" => Ok(Core::CheckSigVerify),
            "complement_1" => Ok(Core::Complement1),
            "complement_16" => Ok(Core::Complement16),
            "complement_32" => Ok(Core::Complement32),
            "complement_64" => Ok(Core::Complement64),
            "complement_8" => Ok(Core::Complement8),
            "decompress" => Ok(Core::Decompress),
            "decrement_16" => Ok(Core::Decrement16),
            "decrement_32" => Ok(Core::Decrement32),
            "decrement_64" => Ok(Core::Decrement64),
            "decrement_8" => Ok(Core::Decrement8),
            "div_mod_128_64" => Ok(Core::DivMod128_64),
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
            "eq_1" => Ok(Core::Eq1),
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
            "full_left_shift_16_1" => Ok(Core::FullLeftShift16_1),
            "full_left_shift_16_2" => Ok(Core::FullLeftShift16_2),
            "full_left_shift_16_4" => Ok(Core::FullLeftShift16_4),
            "full_left_shift_16_8" => Ok(Core::FullLeftShift16_8),
            "full_left_shift_32_1" => Ok(Core::FullLeftShift32_1),
            "full_left_shift_32_16" => Ok(Core::FullLeftShift32_16),
            "full_left_shift_32_2" => Ok(Core::FullLeftShift32_2),
            "full_left_shift_32_4" => Ok(Core::FullLeftShift32_4),
            "full_left_shift_32_8" => Ok(Core::FullLeftShift32_8),
            "full_left_shift_64_1" => Ok(Core::FullLeftShift64_1),
            "full_left_shift_64_16" => Ok(Core::FullLeftShift64_16),
            "full_left_shift_64_2" => Ok(Core::FullLeftShift64_2),
            "full_left_shift_64_32" => Ok(Core::FullLeftShift64_32),
            "full_left_shift_64_4" => Ok(Core::FullLeftShift64_4),
            "full_left_shift_64_8" => Ok(Core::FullLeftShift64_8),
            "full_left_shift_8_1" => Ok(Core::FullLeftShift8_1),
            "full_left_shift_8_2" => Ok(Core::FullLeftShift8_2),
            "full_left_shift_8_4" => Ok(Core::FullLeftShift8_4),
            "full_multiply_16" => Ok(Core::FullMultiply16),
            "full_multiply_32" => Ok(Core::FullMultiply32),
            "full_multiply_64" => Ok(Core::FullMultiply64),
            "full_multiply_8" => Ok(Core::FullMultiply8),
            "full_right_shift_16_1" => Ok(Core::FullRightShift16_1),
            "full_right_shift_16_2" => Ok(Core::FullRightShift16_2),
            "full_right_shift_16_4" => Ok(Core::FullRightShift16_4),
            "full_right_shift_16_8" => Ok(Core::FullRightShift16_8),
            "full_right_shift_32_1" => Ok(Core::FullRightShift32_1),
            "full_right_shift_32_16" => Ok(Core::FullRightShift32_16),
            "full_right_shift_32_2" => Ok(Core::FullRightShift32_2),
            "full_right_shift_32_4" => Ok(Core::FullRightShift32_4),
            "full_right_shift_32_8" => Ok(Core::FullRightShift32_8),
            "full_right_shift_64_1" => Ok(Core::FullRightShift64_1),
            "full_right_shift_64_16" => Ok(Core::FullRightShift64_16),
            "full_right_shift_64_2" => Ok(Core::FullRightShift64_2),
            "full_right_shift_64_32" => Ok(Core::FullRightShift64_32),
            "full_right_shift_64_4" => Ok(Core::FullRightShift64_4),
            "full_right_shift_64_8" => Ok(Core::FullRightShift64_8),
            "full_right_shift_8_1" => Ok(Core::FullRightShift8_1),
            "full_right_shift_8_2" => Ok(Core::FullRightShift8_2),
            "full_right_shift_8_4" => Ok(Core::FullRightShift8_4),
            "full_subtract_16" => Ok(Core::FullSubtract16),
            "full_subtract_32" => Ok(Core::FullSubtract32),
            "full_subtract_64" => Ok(Core::FullSubtract64),
            "full_subtract_8" => Ok(Core::FullSubtract8),
            "ge_is_on_curve" => Ok(Core::GeIsOnCurve),
            "ge_negate" => Ok(Core::GeNegate),
            "gej_add" => Ok(Core::GejAdd),
            "gej_double" => Ok(Core::GejDouble),
            "gej_equiv" => Ok(Core::GejEquiv),
            "gej_ge_add" => Ok(Core::GejGeAdd),
            "gej_ge_add_ex" => Ok(Core::GejGeAddEx),
            "gej_ge_equiv" => Ok(Core::GejGeEquiv),
            "gej_infinity" => Ok(Core::GejInfinity),
            "gej_is_infinity" => Ok(Core::GejIsInfinity),
            "gej_is_on_curve" => Ok(Core::GejIsOnCurve),
            "gej_negate" => Ok(Core::GejNegate),
            "gej_normalize" => Ok(Core::GejNormalize),
            "gej_rescale" => Ok(Core::GejRescale),
            "gej_x_equiv" => Ok(Core::GejXEquiv),
            "gej_y_is_odd" => Ok(Core::GejYIsOdd),
            "generate" => Ok(Core::Generate),
            "hash_to_curve" => Ok(Core::HashToCurve),
            "high_1" => Ok(Core::High1),
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
            "left_extend_16_32" => Ok(Core::LeftExtend16_32),
            "left_extend_16_64" => Ok(Core::LeftExtend16_64),
            "left_extend_1_16" => Ok(Core::LeftExtend1_16),
            "left_extend_1_32" => Ok(Core::LeftExtend1_32),
            "left_extend_1_64" => Ok(Core::LeftExtend1_64),
            "left_extend_1_8" => Ok(Core::LeftExtend1_8),
            "left_extend_32_64" => Ok(Core::LeftExtend32_64),
            "left_extend_8_16" => Ok(Core::LeftExtend8_16),
            "left_extend_8_32" => Ok(Core::LeftExtend8_32),
            "left_extend_8_64" => Ok(Core::LeftExtend8_64),
            "left_pad_high_16_32" => Ok(Core::LeftPadHigh16_32),
            "left_pad_high_16_64" => Ok(Core::LeftPadHigh16_64),
            "left_pad_high_1_16" => Ok(Core::LeftPadHigh1_16),
            "left_pad_high_1_32" => Ok(Core::LeftPadHigh1_32),
            "left_pad_high_1_64" => Ok(Core::LeftPadHigh1_64),
            "left_pad_high_1_8" => Ok(Core::LeftPadHigh1_8),
            "left_pad_high_32_64" => Ok(Core::LeftPadHigh32_64),
            "left_pad_high_8_16" => Ok(Core::LeftPadHigh8_16),
            "left_pad_high_8_32" => Ok(Core::LeftPadHigh8_32),
            "left_pad_high_8_64" => Ok(Core::LeftPadHigh8_64),
            "left_pad_low_16_32" => Ok(Core::LeftPadLow16_32),
            "left_pad_low_16_64" => Ok(Core::LeftPadLow16_64),
            "left_pad_low_1_16" => Ok(Core::LeftPadLow1_16),
            "left_pad_low_1_32" => Ok(Core::LeftPadLow1_32),
            "left_pad_low_1_64" => Ok(Core::LeftPadLow1_64),
            "left_pad_low_1_8" => Ok(Core::LeftPadLow1_8),
            "left_pad_low_32_64" => Ok(Core::LeftPadLow32_64),
            "left_pad_low_8_16" => Ok(Core::LeftPadLow8_16),
            "left_pad_low_8_32" => Ok(Core::LeftPadLow8_32),
            "left_pad_low_8_64" => Ok(Core::LeftPadLow8_64),
            "left_rotate_16" => Ok(Core::LeftRotate16),
            "left_rotate_32" => Ok(Core::LeftRotate32),
            "left_rotate_64" => Ok(Core::LeftRotate64),
            "left_rotate_8" => Ok(Core::LeftRotate8),
            "left_shift_16" => Ok(Core::LeftShift16),
            "left_shift_32" => Ok(Core::LeftShift32),
            "left_shift_64" => Ok(Core::LeftShift64),
            "left_shift_8" => Ok(Core::LeftShift8),
            "left_shift_with_16" => Ok(Core::LeftShiftWith16),
            "left_shift_with_32" => Ok(Core::LeftShiftWith32),
            "left_shift_with_64" => Ok(Core::LeftShiftWith64),
            "left_shift_with_8" => Ok(Core::LeftShiftWith8),
            "leftmost_16_1" => Ok(Core::Leftmost16_1),
            "leftmost_16_2" => Ok(Core::Leftmost16_2),
            "leftmost_16_4" => Ok(Core::Leftmost16_4),
            "leftmost_16_8" => Ok(Core::Leftmost16_8),
            "leftmost_32_1" => Ok(Core::Leftmost32_1),
            "leftmost_32_16" => Ok(Core::Leftmost32_16),
            "leftmost_32_2" => Ok(Core::Leftmost32_2),
            "leftmost_32_4" => Ok(Core::Leftmost32_4),
            "leftmost_32_8" => Ok(Core::Leftmost32_8),
            "leftmost_64_1" => Ok(Core::Leftmost64_1),
            "leftmost_64_16" => Ok(Core::Leftmost64_16),
            "leftmost_64_2" => Ok(Core::Leftmost64_2),
            "leftmost_64_32" => Ok(Core::Leftmost64_32),
            "leftmost_64_4" => Ok(Core::Leftmost64_4),
            "leftmost_64_8" => Ok(Core::Leftmost64_8),
            "leftmost_8_1" => Ok(Core::Leftmost8_1),
            "leftmost_8_2" => Ok(Core::Leftmost8_2),
            "leftmost_8_4" => Ok(Core::Leftmost8_4),
            "linear_combination_1" => Ok(Core::LinearCombination1),
            "linear_verify_1" => Ok(Core::LinearVerify1),
            "low_1" => Ok(Core::Low1),
            "low_16" => Ok(Core::Low16),
            "low_32" => Ok(Core::Low32),
            "low_64" => Ok(Core::Low64),
            "low_8" => Ok(Core::Low8),
            "lt_16" => Ok(Core::Lt16),
            "lt_32" => Ok(Core::Lt32),
            "lt_64" => Ok(Core::Lt64),
            "lt_8" => Ok(Core::Lt8),
            "maj_1" => Ok(Core::Maj1),
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
            "or_1" => Ok(Core::Or1),
            "or_16" => Ok(Core::Or16),
            "or_32" => Ok(Core::Or32),
            "or_64" => Ok(Core::Or64),
            "or_8" => Ok(Core::Or8),
            "parse_lock" => Ok(Core::ParseLock),
            "parse_sequence" => Ok(Core::ParseSequence),
            "point_verify_1" => Ok(Core::PointVerify1),
            "right_extend_16_32" => Ok(Core::RightExtend16_32),
            "right_extend_16_64" => Ok(Core::RightExtend16_64),
            "right_extend_32_64" => Ok(Core::RightExtend32_64),
            "right_extend_8_16" => Ok(Core::RightExtend8_16),
            "right_extend_8_32" => Ok(Core::RightExtend8_32),
            "right_extend_8_64" => Ok(Core::RightExtend8_64),
            "right_pad_high_16_32" => Ok(Core::RightPadHigh16_32),
            "right_pad_high_16_64" => Ok(Core::RightPadHigh16_64),
            "right_pad_high_1_16" => Ok(Core::RightPadHigh1_16),
            "right_pad_high_1_32" => Ok(Core::RightPadHigh1_32),
            "right_pad_high_1_64" => Ok(Core::RightPadHigh1_64),
            "right_pad_high_1_8" => Ok(Core::RightPadHigh1_8),
            "right_pad_high_32_64" => Ok(Core::RightPadHigh32_64),
            "right_pad_high_8_16" => Ok(Core::RightPadHigh8_16),
            "right_pad_high_8_32" => Ok(Core::RightPadHigh8_32),
            "right_pad_high_8_64" => Ok(Core::RightPadHigh8_64),
            "right_pad_low_16_32" => Ok(Core::RightPadLow16_32),
            "right_pad_low_16_64" => Ok(Core::RightPadLow16_64),
            "right_pad_low_1_16" => Ok(Core::RightPadLow1_16),
            "right_pad_low_1_32" => Ok(Core::RightPadLow1_32),
            "right_pad_low_1_64" => Ok(Core::RightPadLow1_64),
            "right_pad_low_1_8" => Ok(Core::RightPadLow1_8),
            "right_pad_low_32_64" => Ok(Core::RightPadLow32_64),
            "right_pad_low_8_16" => Ok(Core::RightPadLow8_16),
            "right_pad_low_8_32" => Ok(Core::RightPadLow8_32),
            "right_pad_low_8_64" => Ok(Core::RightPadLow8_64),
            "right_rotate_16" => Ok(Core::RightRotate16),
            "right_rotate_32" => Ok(Core::RightRotate32),
            "right_rotate_64" => Ok(Core::RightRotate64),
            "right_rotate_8" => Ok(Core::RightRotate8),
            "right_shift_16" => Ok(Core::RightShift16),
            "right_shift_32" => Ok(Core::RightShift32),
            "right_shift_64" => Ok(Core::RightShift64),
            "right_shift_8" => Ok(Core::RightShift8),
            "right_shift_with_16" => Ok(Core::RightShiftWith16),
            "right_shift_with_32" => Ok(Core::RightShiftWith32),
            "right_shift_with_64" => Ok(Core::RightShiftWith64),
            "right_shift_with_8" => Ok(Core::RightShiftWith8),
            "rightmost_16_1" => Ok(Core::Rightmost16_1),
            "rightmost_16_2" => Ok(Core::Rightmost16_2),
            "rightmost_16_4" => Ok(Core::Rightmost16_4),
            "rightmost_16_8" => Ok(Core::Rightmost16_8),
            "rightmost_32_1" => Ok(Core::Rightmost32_1),
            "rightmost_32_16" => Ok(Core::Rightmost32_16),
            "rightmost_32_2" => Ok(Core::Rightmost32_2),
            "rightmost_32_4" => Ok(Core::Rightmost32_4),
            "rightmost_32_8" => Ok(Core::Rightmost32_8),
            "rightmost_64_1" => Ok(Core::Rightmost64_1),
            "rightmost_64_16" => Ok(Core::Rightmost64_16),
            "rightmost_64_2" => Ok(Core::Rightmost64_2),
            "rightmost_64_32" => Ok(Core::Rightmost64_32),
            "rightmost_64_4" => Ok(Core::Rightmost64_4),
            "rightmost_64_8" => Ok(Core::Rightmost64_8),
            "rightmost_8_1" => Ok(Core::Rightmost8_1),
            "rightmost_8_2" => Ok(Core::Rightmost8_2),
            "rightmost_8_4" => Ok(Core::Rightmost8_4),
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
            "some_1" => Ok(Core::Some1),
            "some_16" => Ok(Core::Some16),
            "some_32" => Ok(Core::Some32),
            "some_64" => Ok(Core::Some64),
            "some_8" => Ok(Core::Some8),
            "subtract_16" => Ok(Core::Subtract16),
            "subtract_32" => Ok(Core::Subtract32),
            "subtract_64" => Ok(Core::Subtract64),
            "subtract_8" => Ok(Core::Subtract8),
            "swu" => Ok(Core::Swu),
            "verify" => Ok(Core::Verify),
            "xor_1" => Ok(Core::Xor1),
            "xor_16" => Ok(Core::Xor16),
            "xor_32" => Ok(Core::Xor32),
            "xor_64" => Ok(Core::Xor64),
            "xor_8" => Ok(Core::Xor8),
            "xor_xor_1" => Ok(Core::XorXor1),
            "xor_xor_16" => Ok(Core::XorXor16),
            "xor_xor_32" => Ok(Core::XorXor32),
            "xor_xor_64" => Ok(Core::XorXor64),
            "xor_xor_8" => Ok(Core::XorXor8),
            x => Err(crate::Error::InvalidJetName(x.to_owned())),
        }
    }
}
