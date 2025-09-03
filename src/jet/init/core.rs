/* This file has been automatically generated. */

use crate::analysis::Cost;
use crate::decode_bits;
use crate::jet::core::CoreEnv;
use crate::jet::type_name::TypeName;
use crate::jet::Jet;
use crate::merkle::cmr::Cmr;
use crate::{decode, BitIter, BitWriter};
use hashes::sha256::Midstate;
use simplicity_sys::CFrameItem;
use std::io::Write;
use std::{borrow::Borrow, fmt, str};

/// The Core jet family.
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
    TapdataInit,
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

impl Core {
    /// Array of all Core jets.
    pub const ALL: [Self; 368] = [
        Self::Add16,
        Self::Add32,
        Self::Add64,
        Self::Add8,
        Self::All16,
        Self::All32,
        Self::All64,
        Self::All8,
        Self::And1,
        Self::And16,
        Self::And32,
        Self::And64,
        Self::And8,
        Self::Bip0340Verify,
        Self::Ch1,
        Self::Ch16,
        Self::Ch32,
        Self::Ch64,
        Self::Ch8,
        Self::CheckSigVerify,
        Self::Complement1,
        Self::Complement16,
        Self::Complement32,
        Self::Complement64,
        Self::Complement8,
        Self::Decompress,
        Self::Decrement16,
        Self::Decrement32,
        Self::Decrement64,
        Self::Decrement8,
        Self::DivMod128_64,
        Self::DivMod16,
        Self::DivMod32,
        Self::DivMod64,
        Self::DivMod8,
        Self::Divide16,
        Self::Divide32,
        Self::Divide64,
        Self::Divide8,
        Self::Divides16,
        Self::Divides32,
        Self::Divides64,
        Self::Divides8,
        Self::Eq1,
        Self::Eq16,
        Self::Eq256,
        Self::Eq32,
        Self::Eq64,
        Self::Eq8,
        Self::FeAdd,
        Self::FeInvert,
        Self::FeIsOdd,
        Self::FeIsZero,
        Self::FeMultiply,
        Self::FeMultiplyBeta,
        Self::FeNegate,
        Self::FeNormalize,
        Self::FeSquare,
        Self::FeSquareRoot,
        Self::FullAdd16,
        Self::FullAdd32,
        Self::FullAdd64,
        Self::FullAdd8,
        Self::FullDecrement16,
        Self::FullDecrement32,
        Self::FullDecrement64,
        Self::FullDecrement8,
        Self::FullIncrement16,
        Self::FullIncrement32,
        Self::FullIncrement64,
        Self::FullIncrement8,
        Self::FullLeftShift16_1,
        Self::FullLeftShift16_2,
        Self::FullLeftShift16_4,
        Self::FullLeftShift16_8,
        Self::FullLeftShift32_1,
        Self::FullLeftShift32_16,
        Self::FullLeftShift32_2,
        Self::FullLeftShift32_4,
        Self::FullLeftShift32_8,
        Self::FullLeftShift64_1,
        Self::FullLeftShift64_16,
        Self::FullLeftShift64_2,
        Self::FullLeftShift64_32,
        Self::FullLeftShift64_4,
        Self::FullLeftShift64_8,
        Self::FullLeftShift8_1,
        Self::FullLeftShift8_2,
        Self::FullLeftShift8_4,
        Self::FullMultiply16,
        Self::FullMultiply32,
        Self::FullMultiply64,
        Self::FullMultiply8,
        Self::FullRightShift16_1,
        Self::FullRightShift16_2,
        Self::FullRightShift16_4,
        Self::FullRightShift16_8,
        Self::FullRightShift32_1,
        Self::FullRightShift32_16,
        Self::FullRightShift32_2,
        Self::FullRightShift32_4,
        Self::FullRightShift32_8,
        Self::FullRightShift64_1,
        Self::FullRightShift64_16,
        Self::FullRightShift64_2,
        Self::FullRightShift64_32,
        Self::FullRightShift64_4,
        Self::FullRightShift64_8,
        Self::FullRightShift8_1,
        Self::FullRightShift8_2,
        Self::FullRightShift8_4,
        Self::FullSubtract16,
        Self::FullSubtract32,
        Self::FullSubtract64,
        Self::FullSubtract8,
        Self::GeIsOnCurve,
        Self::GeNegate,
        Self::GejAdd,
        Self::GejDouble,
        Self::GejEquiv,
        Self::GejGeAdd,
        Self::GejGeAddEx,
        Self::GejGeEquiv,
        Self::GejInfinity,
        Self::GejIsInfinity,
        Self::GejIsOnCurve,
        Self::GejNegate,
        Self::GejNormalize,
        Self::GejRescale,
        Self::GejXEquiv,
        Self::GejYIsOdd,
        Self::Generate,
        Self::HashToCurve,
        Self::High1,
        Self::High16,
        Self::High32,
        Self::High64,
        Self::High8,
        Self::Increment16,
        Self::Increment32,
        Self::Increment64,
        Self::Increment8,
        Self::IsOne16,
        Self::IsOne32,
        Self::IsOne64,
        Self::IsOne8,
        Self::IsZero16,
        Self::IsZero32,
        Self::IsZero64,
        Self::IsZero8,
        Self::Le16,
        Self::Le32,
        Self::Le64,
        Self::Le8,
        Self::LeftExtend16_32,
        Self::LeftExtend16_64,
        Self::LeftExtend1_16,
        Self::LeftExtend1_32,
        Self::LeftExtend1_64,
        Self::LeftExtend1_8,
        Self::LeftExtend32_64,
        Self::LeftExtend8_16,
        Self::LeftExtend8_32,
        Self::LeftExtend8_64,
        Self::LeftPadHigh16_32,
        Self::LeftPadHigh16_64,
        Self::LeftPadHigh1_16,
        Self::LeftPadHigh1_32,
        Self::LeftPadHigh1_64,
        Self::LeftPadHigh1_8,
        Self::LeftPadHigh32_64,
        Self::LeftPadHigh8_16,
        Self::LeftPadHigh8_32,
        Self::LeftPadHigh8_64,
        Self::LeftPadLow16_32,
        Self::LeftPadLow16_64,
        Self::LeftPadLow1_16,
        Self::LeftPadLow1_32,
        Self::LeftPadLow1_64,
        Self::LeftPadLow1_8,
        Self::LeftPadLow32_64,
        Self::LeftPadLow8_16,
        Self::LeftPadLow8_32,
        Self::LeftPadLow8_64,
        Self::LeftRotate16,
        Self::LeftRotate32,
        Self::LeftRotate64,
        Self::LeftRotate8,
        Self::LeftShift16,
        Self::LeftShift32,
        Self::LeftShift64,
        Self::LeftShift8,
        Self::LeftShiftWith16,
        Self::LeftShiftWith32,
        Self::LeftShiftWith64,
        Self::LeftShiftWith8,
        Self::Leftmost16_1,
        Self::Leftmost16_2,
        Self::Leftmost16_4,
        Self::Leftmost16_8,
        Self::Leftmost32_1,
        Self::Leftmost32_16,
        Self::Leftmost32_2,
        Self::Leftmost32_4,
        Self::Leftmost32_8,
        Self::Leftmost64_1,
        Self::Leftmost64_16,
        Self::Leftmost64_2,
        Self::Leftmost64_32,
        Self::Leftmost64_4,
        Self::Leftmost64_8,
        Self::Leftmost8_1,
        Self::Leftmost8_2,
        Self::Leftmost8_4,
        Self::LinearCombination1,
        Self::LinearVerify1,
        Self::Low1,
        Self::Low16,
        Self::Low32,
        Self::Low64,
        Self::Low8,
        Self::Lt16,
        Self::Lt32,
        Self::Lt64,
        Self::Lt8,
        Self::Maj1,
        Self::Maj16,
        Self::Maj32,
        Self::Maj64,
        Self::Maj8,
        Self::Max16,
        Self::Max32,
        Self::Max64,
        Self::Max8,
        Self::Median16,
        Self::Median32,
        Self::Median64,
        Self::Median8,
        Self::Min16,
        Self::Min32,
        Self::Min64,
        Self::Min8,
        Self::Modulo16,
        Self::Modulo32,
        Self::Modulo64,
        Self::Modulo8,
        Self::Multiply16,
        Self::Multiply32,
        Self::Multiply64,
        Self::Multiply8,
        Self::Negate16,
        Self::Negate32,
        Self::Negate64,
        Self::Negate8,
        Self::One16,
        Self::One32,
        Self::One64,
        Self::One8,
        Self::Or1,
        Self::Or16,
        Self::Or32,
        Self::Or64,
        Self::Or8,
        Self::ParseLock,
        Self::ParseSequence,
        Self::PointVerify1,
        Self::RightExtend16_32,
        Self::RightExtend16_64,
        Self::RightExtend32_64,
        Self::RightExtend8_16,
        Self::RightExtend8_32,
        Self::RightExtend8_64,
        Self::RightPadHigh16_32,
        Self::RightPadHigh16_64,
        Self::RightPadHigh1_16,
        Self::RightPadHigh1_32,
        Self::RightPadHigh1_64,
        Self::RightPadHigh1_8,
        Self::RightPadHigh32_64,
        Self::RightPadHigh8_16,
        Self::RightPadHigh8_32,
        Self::RightPadHigh8_64,
        Self::RightPadLow16_32,
        Self::RightPadLow16_64,
        Self::RightPadLow1_16,
        Self::RightPadLow1_32,
        Self::RightPadLow1_64,
        Self::RightPadLow1_8,
        Self::RightPadLow32_64,
        Self::RightPadLow8_16,
        Self::RightPadLow8_32,
        Self::RightPadLow8_64,
        Self::RightRotate16,
        Self::RightRotate32,
        Self::RightRotate64,
        Self::RightRotate8,
        Self::RightShift16,
        Self::RightShift32,
        Self::RightShift64,
        Self::RightShift8,
        Self::RightShiftWith16,
        Self::RightShiftWith32,
        Self::RightShiftWith64,
        Self::RightShiftWith8,
        Self::Rightmost16_1,
        Self::Rightmost16_2,
        Self::Rightmost16_4,
        Self::Rightmost16_8,
        Self::Rightmost32_1,
        Self::Rightmost32_16,
        Self::Rightmost32_2,
        Self::Rightmost32_4,
        Self::Rightmost32_8,
        Self::Rightmost64_1,
        Self::Rightmost64_16,
        Self::Rightmost64_2,
        Self::Rightmost64_32,
        Self::Rightmost64_4,
        Self::Rightmost64_8,
        Self::Rightmost8_1,
        Self::Rightmost8_2,
        Self::Rightmost8_4,
        Self::ScalarAdd,
        Self::ScalarInvert,
        Self::ScalarIsZero,
        Self::ScalarMultiply,
        Self::ScalarMultiplyLambda,
        Self::ScalarNegate,
        Self::ScalarNormalize,
        Self::ScalarSquare,
        Self::Scale,
        Self::Sha256Block,
        Self::Sha256Ctx8Add1,
        Self::Sha256Ctx8Add128,
        Self::Sha256Ctx8Add16,
        Self::Sha256Ctx8Add2,
        Self::Sha256Ctx8Add256,
        Self::Sha256Ctx8Add32,
        Self::Sha256Ctx8Add4,
        Self::Sha256Ctx8Add512,
        Self::Sha256Ctx8Add64,
        Self::Sha256Ctx8Add8,
        Self::Sha256Ctx8AddBuffer511,
        Self::Sha256Ctx8Finalize,
        Self::Sha256Ctx8Init,
        Self::Sha256Iv,
        Self::Some1,
        Self::Some16,
        Self::Some32,
        Self::Some64,
        Self::Some8,
        Self::Subtract16,
        Self::Subtract32,
        Self::Subtract64,
        Self::Subtract8,
        Self::Swu,
        Self::TapdataInit,
        Self::Verify,
        Self::Xor1,
        Self::Xor16,
        Self::Xor32,
        Self::Xor64,
        Self::Xor8,
        Self::XorXor1,
        Self::XorXor16,
        Self::XorXor32,
        Self::XorXor64,
        Self::XorXor8,
    ];
}

impl Jet for Core {
    type Transaction = core::convert::Infallible;
    type Environment<T>
        = CoreEnv<T>
    where
        T: Borrow<Self::Transaction>;
    type CJetEnvironment = CoreEnv<Self::Transaction>;

    fn c_jet_env<T>(_: &Self::Environment<T>) -> &Self::CJetEnvironment
    where
        T: Borrow<Self::Transaction>,
    {
        &CoreEnv::EMPTY
    }

    fn cmr(&self) -> Cmr {
        let bytes = match self {
            Core::Add16 => [
                0x49, 0x42, 0x5a, 0x86, 0xe2, 0x0a, 0x67, 0x6d, 0x8b, 0x87, 0xe3, 0xc1, 0xa9, 0xb8,
                0xea, 0x6e, 0xc7, 0x5d, 0x85, 0x9c, 0x12, 0xc5, 0x1b, 0xcb, 0x7f, 0xa9, 0xf9, 0x69,
                0x12, 0xc3, 0x49, 0xcf,
            ],
            Core::Add32 => [
                0x46, 0x68, 0xcd, 0x55, 0xe8, 0xd1, 0x59, 0x19, 0x53, 0x32, 0x70, 0x14, 0xec, 0x64,
                0xc8, 0xe7, 0xd5, 0x2b, 0x86, 0xb5, 0x3e, 0x11, 0xc0, 0x14, 0x57, 0xea, 0xf2, 0xc3,
                0xd3, 0xce, 0xbf, 0x9f,
            ],
            Core::Add64 => [
                0xbe, 0x2b, 0x75, 0x19, 0x30, 0x3a, 0x67, 0xee, 0xa6, 0xb4, 0x82, 0x95, 0x0e, 0xda,
                0x83, 0x43, 0x5e, 0x1d, 0xe8, 0x55, 0x9c, 0x39, 0x4a, 0x23, 0x62, 0x22, 0xff, 0x5b,
                0xf0, 0x89, 0xd3, 0x46,
            ],
            Core::Add8 => [
                0xdf, 0xa1, 0x79, 0xad, 0xf4, 0x55, 0x0b, 0x28, 0x48, 0x73, 0xbf, 0x30, 0x12, 0x3e,
                0x0d, 0x4e, 0x54, 0x06, 0x9b, 0x08, 0x58, 0x34, 0xce, 0x56, 0x58, 0x15, 0xef, 0x7e,
                0x45, 0x78, 0x4a, 0xcb,
            ],
            Core::All16 => [
                0x24, 0xf4, 0x82, 0xa5, 0x13, 0xd3, 0x33, 0x62, 0x01, 0x5d, 0x28, 0xdf, 0x4b, 0xb6,
                0xc3, 0xee, 0x08, 0xab, 0x8a, 0xfb, 0xbd, 0x25, 0x57, 0x1f, 0x0e, 0xa8, 0x9d, 0x8c,
                0xab, 0xa3, 0x14, 0x04,
            ],
            Core::All32 => [
                0xa7, 0x16, 0x52, 0x2d, 0x0f, 0x37, 0x87, 0xc8, 0xb4, 0xd5, 0x07, 0x64, 0x7f, 0x1f,
                0x80, 0x7b, 0x67, 0xf3, 0x20, 0xd6, 0xeb, 0x67, 0xb8, 0x4b, 0x60, 0x9c, 0xec, 0x1d,
                0x2f, 0x12, 0x21, 0x8a,
            ],
            Core::All64 => [
                0x7a, 0xee, 0xfe, 0x2e, 0xce, 0x24, 0xba, 0xb3, 0x7c, 0x6e, 0x54, 0x30, 0xee, 0xd4,
                0x19, 0xfc, 0xd5, 0xf0, 0x37, 0x91, 0x2d, 0x17, 0x70, 0xcb, 0x7d, 0x65, 0x20, 0xdc,
                0xe5, 0x25, 0x29, 0x1a,
            ],
            Core::All8 => [
                0x46, 0x37, 0xf4, 0x0e, 0x5f, 0x47, 0x26, 0xb0, 0x05, 0x70, 0x76, 0x5a, 0xc7, 0x94,
                0xe2, 0x9e, 0xd1, 0xbb, 0x26, 0x55, 0xff, 0xc4, 0x12, 0xb2, 0xdc, 0x41, 0x25, 0x8e,
                0x41, 0xaa, 0xc6, 0x24,
            ],
            Core::And1 => [
                0x10, 0x68, 0x4d, 0x0d, 0xd7, 0x2c, 0xb0, 0xa8, 0x26, 0xa8, 0x63, 0x83, 0x4e, 0x01,
                0x1f, 0x50, 0xfa, 0x0d, 0x55, 0x8b, 0xa7, 0x7d, 0x6b, 0x9f, 0x49, 0xa1, 0xac, 0x22,
                0x90, 0x2a, 0x6a, 0xd0,
            ],
            Core::And16 => [
                0x37, 0x3c, 0x73, 0x0f, 0xad, 0x3e, 0x88, 0x47, 0x99, 0x1a, 0xa4, 0x17, 0xd9, 0xf0,
                0x80, 0xee, 0x1c, 0xb8, 0x8a, 0x7f, 0x72, 0x06, 0xf3, 0xfa, 0x84, 0x0b, 0x19, 0x50,
                0x77, 0x61, 0xfb, 0x23,
            ],
            Core::And32 => [
                0x13, 0xb0, 0x2c, 0x4c, 0x60, 0xae, 0x6e, 0xa4, 0x91, 0x16, 0x16, 0x49, 0xac, 0xf9,
                0xa4, 0x7a, 0x70, 0x25, 0xaf, 0x84, 0x7d, 0x5f, 0x58, 0x1e, 0x6f, 0x1c, 0xcc, 0xfb,
                0x21, 0xd3, 0x00, 0x1d,
            ],
            Core::And64 => [
                0x92, 0x18, 0x55, 0x35, 0xd4, 0x50, 0x54, 0x07, 0xde, 0xa3, 0xc8, 0xa6, 0x08, 0x26,
                0xed, 0xe6, 0x4a, 0x8f, 0xbb, 0x3d, 0xb4, 0x86, 0xd5, 0x6f, 0x64, 0x2d, 0x21, 0x7c,
                0x29, 0xcb, 0xd7, 0x95,
            ],
            Core::And8 => [
                0x26, 0x9a, 0x1b, 0x44, 0x62, 0x66, 0xf8, 0xf4, 0xa4, 0xa3, 0x8f, 0xa7, 0xe7, 0xe3,
                0x91, 0x82, 0xf1, 0x52, 0x14, 0x36, 0x14, 0x2b, 0xad, 0xed, 0xf3, 0xaa, 0x63, 0xfb,
                0x2f, 0x17, 0x2d, 0x2f,
            ],
            Core::Bip0340Verify => [
                0x49, 0x15, 0x65, 0xfe, 0x23, 0xa7, 0xbd, 0xc1, 0x84, 0x2b, 0xe7, 0x49, 0x50, 0x93,
                0x37, 0xf9, 0x68, 0x90, 0xd5, 0xb3, 0x58, 0xb3, 0x65, 0x20, 0x90, 0xda, 0x55, 0x66,
                0x54, 0xe2, 0x95, 0x49,
            ],
            Core::Ch1 => [
                0x73, 0xb2, 0xa9, 0x81, 0xd7, 0x21, 0x98, 0x6f, 0x8c, 0xde, 0xd6, 0x97, 0xe0, 0x63,
                0x05, 0xd4, 0x58, 0x54, 0x10, 0x2d, 0xff, 0x20, 0xc0, 0xe5, 0xb9, 0x8a, 0xe1, 0x76,
                0x23, 0x2f, 0xf2, 0x5b,
            ],
            Core::Ch16 => [
                0x78, 0xde, 0x46, 0x5e, 0x61, 0xd9, 0xa5, 0x0f, 0x78, 0x25, 0x2f, 0xf4, 0xab, 0x23,
                0xc9, 0xe6, 0x3a, 0xe6, 0x8c, 0x76, 0x9d, 0x36, 0x66, 0x12, 0x71, 0x20, 0x7d, 0xc6,
                0x93, 0xf4, 0x69, 0xb4,
            ],
            Core::Ch32 => [
                0xed, 0x93, 0xbe, 0xf1, 0xf6, 0x6e, 0x3a, 0x75, 0xe6, 0x12, 0x06, 0x02, 0xec, 0xee,
                0x67, 0x40, 0x65, 0x3e, 0x7b, 0xd4, 0x6e, 0x07, 0xeb, 0x77, 0x14, 0x4e, 0xf1, 0xbb,
                0x2c, 0x9d, 0xe5, 0x3d,
            ],
            Core::Ch64 => [
                0xce, 0xd0, 0x07, 0x9b, 0x0b, 0xd1, 0xcc, 0x00, 0x20, 0x9a, 0x7c, 0xbc, 0x23, 0xf1,
                0x3d, 0xfd, 0x20, 0x28, 0x08, 0xf0, 0xf5, 0x25, 0x7d, 0x8a, 0x50, 0xac, 0x54, 0x3e,
                0x64, 0xee, 0x3a, 0x05,
            ],
            Core::Ch8 => [
                0xc7, 0x07, 0xca, 0x72, 0x3e, 0x24, 0xf6, 0xb2, 0x5b, 0xf3, 0x94, 0xa9, 0x9a, 0x4d,
                0x75, 0xe8, 0x13, 0x79, 0xb4, 0x67, 0x84, 0x38, 0xac, 0x78, 0x9d, 0xee, 0x18, 0x8e,
                0xdc, 0xce, 0x75, 0xfa,
            ],
            Core::CheckSigVerify => [
                0xb5, 0x80, 0x15, 0x54, 0x6d, 0x28, 0x52, 0x66, 0x5d, 0xd2, 0x1b, 0xf1, 0x12, 0x66,
                0x26, 0x70, 0x20, 0xfa, 0x5e, 0x27, 0x50, 0x01, 0xdd, 0x46, 0x18, 0xfa, 0x41, 0x56,
                0x25, 0x95, 0x2e, 0x68,
            ],
            Core::Complement1 => [
                0x1b, 0xcf, 0xae, 0x13, 0xd5, 0xd2, 0x37, 0xa0, 0xbb, 0x9b, 0x1d, 0x75, 0x32, 0x04,
                0x74, 0x62, 0xb2, 0x76, 0x90, 0xde, 0x5c, 0xac, 0x0e, 0x20, 0x19, 0x29, 0x89, 0x64,
                0x57, 0x93, 0x40, 0x60,
            ],
            Core::Complement16 => [
                0x81, 0xad, 0x4d, 0x2c, 0x3d, 0x16, 0xbf, 0x34, 0x0a, 0xf3, 0x88, 0x6d, 0x35, 0x5c,
                0xc5, 0xbd, 0x1d, 0x59, 0x67, 0xe1, 0x6a, 0xce, 0x92, 0x4f, 0x19, 0xec, 0xf7, 0xd4,
                0x86, 0xd6, 0xc7, 0xe9,
            ],
            Core::Complement32 => [
                0x13, 0x74, 0x2c, 0x18, 0x04, 0xa9, 0x6e, 0x6c, 0x03, 0x95, 0x28, 0xbf, 0xd0, 0x3b,
                0x8c, 0xf2, 0xb4, 0x62, 0x52, 0x6b, 0xb1, 0x81, 0xa3, 0xd8, 0xb4, 0x32, 0xf9, 0x9a,
                0xc4, 0xf5, 0xa7, 0xef,
            ],
            Core::Complement64 => [
                0x65, 0xb7, 0xbd, 0x09, 0x36, 0x39, 0xc5, 0x6d, 0xa2, 0x85, 0xce, 0xfa, 0x2d, 0x04,
                0x64, 0x64, 0x5e, 0x14, 0xdd, 0x13, 0x64, 0x2f, 0x34, 0x95, 0x7d, 0x47, 0x37, 0xbd,
                0x52, 0xfa, 0xc5, 0x88,
            ],
            Core::Complement8 => [
                0x95, 0x4b, 0x70, 0xdc, 0xec, 0x53, 0x9e, 0x6b, 0x67, 0xdf, 0xfe, 0xc5, 0x3c, 0xf2,
                0x4a, 0x66, 0x99, 0x39, 0x60, 0x8b, 0x22, 0x3f, 0x5f, 0x8b, 0x6d, 0x12, 0x9d, 0xaa,
                0x48, 0xca, 0x1c, 0xf0,
            ],
            Core::Decompress => [
                0x89, 0x00, 0x56, 0xdf, 0x82, 0x8a, 0x76, 0x6e, 0xe9, 0xf6, 0x56, 0x07, 0x22, 0x1e,
                0x89, 0x46, 0xfa, 0x77, 0xc2, 0x56, 0xbb, 0x96, 0xe2, 0x31, 0xe1, 0x94, 0xd3, 0x00,
                0x8c, 0xf3, 0x56, 0xf6,
            ],
            Core::Decrement16 => [
                0x35, 0xfd, 0xa3, 0x8b, 0x67, 0x2c, 0x38, 0x31, 0xd8, 0xca, 0x11, 0xa4, 0xf3, 0xa9,
                0x59, 0x62, 0x22, 0x52, 0x9e, 0xb1, 0xc1, 0x5f, 0x8c, 0x70, 0x50, 0x13, 0x97, 0x7d,
                0x7d, 0xfb, 0x5d, 0x8b,
            ],
            Core::Decrement32 => [
                0x3b, 0x2b, 0x19, 0x39, 0x55, 0x22, 0x84, 0xf6, 0x14, 0x69, 0x4b, 0xa1, 0x8d, 0xce,
                0x70, 0xce, 0xe4, 0x76, 0xff, 0x42, 0xdc, 0xd0, 0x89, 0xe1, 0xa3, 0xc0, 0xa4, 0x2b,
                0xeb, 0xd1, 0x08, 0xf6,
            ],
            Core::Decrement64 => [
                0x7e, 0xf7, 0xbd, 0xd3, 0x5d, 0xb6, 0x85, 0xae, 0x99, 0x05, 0x53, 0x37, 0x35, 0xa2,
                0xc7, 0xa7, 0xcc, 0xbc, 0x17, 0x08, 0xae, 0x63, 0x6f, 0x93, 0x1b, 0x5c, 0xe0, 0x26,
                0xe5, 0xa1, 0x7f, 0xed,
            ],
            Core::Decrement8 => [
                0xe3, 0x64, 0xf2, 0xe5, 0xc0, 0x8a, 0xe0, 0x11, 0x8e, 0xbe, 0x99, 0x3e, 0x8b, 0x3c,
                0x95, 0x8c, 0x2b, 0xcc, 0x60, 0x62, 0xa3, 0x3b, 0xaa, 0xb9, 0x28, 0xc0, 0x4b, 0x3e,
                0xc9, 0x32, 0xf5, 0x1b,
            ],
            Core::DivMod128_64 => [
                0x9a, 0x94, 0x43, 0xa2, 0xb5, 0x41, 0xe2, 0x9f, 0x27, 0x2f, 0xfd, 0x56, 0x7d, 0x1b,
                0xf7, 0x42, 0xd6, 0x8c, 0xcb, 0xe9, 0x53, 0x8a, 0x87, 0x29, 0x1b, 0x0c, 0xa6, 0x38,
                0x15, 0x63, 0xac, 0x2c,
            ],
            Core::DivMod16 => [
                0x39, 0xbc, 0xb5, 0xc0, 0x1d, 0xc1, 0x80, 0x5c, 0x49, 0x19, 0x89, 0x5c, 0xb5, 0x9e,
                0x8f, 0x3b, 0x41, 0x44, 0x67, 0x17, 0xf7, 0xff, 0x48, 0xfd, 0xc9, 0x37, 0xdd, 0x03,
                0x80, 0x24, 0xa0, 0x8a,
            ],
            Core::DivMod32 => [
                0xfb, 0x12, 0x02, 0xf4, 0xe8, 0x66, 0x3a, 0x87, 0xf5, 0x68, 0x99, 0x2a, 0x18, 0x50,
                0x24, 0xc7, 0x0b, 0x4f, 0x07, 0x9f, 0xbe, 0x95, 0x30, 0x01, 0x0f, 0x6d, 0xb2, 0x84,
                0x21, 0x8a, 0xf6, 0xcd,
            ],
            Core::DivMod64 => [
                0x67, 0x64, 0xdf, 0x5e, 0x2a, 0xa0, 0x30, 0x32, 0x6e, 0xe5, 0x44, 0xc6, 0xe5, 0x3f,
                0xf3, 0x8e, 0xf0, 0xb2, 0x85, 0x17, 0x91, 0x5e, 0xec, 0x65, 0xc7, 0x2e, 0xa5, 0x7a,
                0x12, 0x98, 0x28, 0xeb,
            ],
            Core::DivMod8 => [
                0xd3, 0x00, 0x24, 0x4e, 0x48, 0x0d, 0xd9, 0x74, 0x12, 0x13, 0xe4, 0xcb, 0x0e, 0xba,
                0x83, 0x6d, 0x30, 0x59, 0xe7, 0x78, 0xb8, 0x12, 0x2f, 0x78, 0x90, 0x03, 0x26, 0x73,
                0x73, 0x9c, 0x6a, 0x2c,
            ],
            Core::Divide16 => [
                0x52, 0xab, 0xfe, 0xf1, 0x79, 0x75, 0x4c, 0x90, 0xf9, 0xa4, 0x26, 0x0f, 0x32, 0x3a,
                0x8c, 0xa4, 0x95, 0x15, 0x92, 0x90, 0x2b, 0x8e, 0xcb, 0xd6, 0x4b, 0xa4, 0x26, 0x56,
                0xfa, 0xc0, 0x59, 0x68,
            ],
            Core::Divide32 => [
                0x4a, 0x8a, 0xe5, 0x35, 0x44, 0xe1, 0x47, 0xed, 0x02, 0x25, 0x04, 0x23, 0x79, 0x34,
                0xcc, 0x25, 0x44, 0x79, 0xbc, 0xf9, 0x3d, 0xe1, 0xe1, 0x97, 0x4d, 0xda, 0xb3, 0xbb,
                0x51, 0x6e, 0x60, 0x6c,
            ],
            Core::Divide64 => [
                0xd7, 0x02, 0x5d, 0x05, 0xad, 0xfa, 0xe6, 0x6b, 0x47, 0x10, 0xd0, 0xff, 0x1e, 0x87,
                0xe8, 0x28, 0x15, 0x57, 0x3e, 0x9c, 0xb6, 0x31, 0xb4, 0xc7, 0xd1, 0x3d, 0x2f, 0x1b,
                0xe4, 0xdd, 0x26, 0xd2,
            ],
            Core::Divide8 => [
                0x40, 0xcd, 0x1d, 0xac, 0xea, 0x24, 0x66, 0x9b, 0x6a, 0x58, 0x9b, 0x61, 0x47, 0x54,
                0x74, 0xaf, 0x31, 0xd1, 0x4f, 0x8d, 0x46, 0x87, 0x70, 0x84, 0x52, 0xd3, 0xdf, 0x37,
                0x30, 0x25, 0x31, 0x26,
            ],
            Core::Divides16 => [
                0x10, 0xbb, 0x18, 0x18, 0x0e, 0xab, 0x5b, 0xad, 0xdc, 0x16, 0x5d, 0x03, 0x37, 0xc4,
                0xad, 0xa0, 0x88, 0xe1, 0x57, 0xb1, 0xaa, 0x67, 0x83, 0x34, 0x2a, 0x45, 0x20, 0xa3,
                0x24, 0xdd, 0x9d, 0x2b,
            ],
            Core::Divides32 => [
                0xf5, 0xe8, 0xe7, 0x8c, 0x82, 0x76, 0x9a, 0x48, 0xc9, 0x10, 0x3e, 0x44, 0xdd, 0xb4,
                0x7f, 0x84, 0x1d, 0x76, 0x93, 0xb0, 0x41, 0x9e, 0x5e, 0x7d, 0xa4, 0xe6, 0x8b, 0x78,
                0xb2, 0x37, 0xa5, 0x72,
            ],
            Core::Divides64 => [
                0x9e, 0xbd, 0x55, 0xfa, 0xe4, 0x18, 0x88, 0x5e, 0xea, 0x04, 0xc3, 0xcd, 0xff, 0xf5,
                0x31, 0xb7, 0xd7, 0x14, 0xd0, 0x59, 0x4f, 0xa7, 0xda, 0x87, 0xeb, 0x65, 0x55, 0xd3,
                0x6b, 0x95, 0x3d, 0xb2,
            ],
            Core::Divides8 => [
                0xa2, 0x36, 0xbc, 0x3e, 0x5c, 0xf4, 0xd2, 0x56, 0x40, 0x8b, 0xa3, 0x8c, 0x1e, 0xae,
                0xe7, 0x36, 0x9a, 0x9c, 0x40, 0x2f, 0x74, 0xbc, 0xd1, 0xc8, 0x02, 0xf9, 0x09, 0x4f,
                0xbf, 0x36, 0x80, 0x3d,
            ],
            Core::Eq1 => [
                0x65, 0x49, 0xf9, 0x86, 0x20, 0x3a, 0x64, 0x97, 0x35, 0x6e, 0x43, 0x2b, 0x2a, 0xa1,
                0x60, 0xd6, 0xee, 0x87, 0x0b, 0x11, 0x19, 0x08, 0x65, 0xbd, 0x36, 0xa4, 0x7c, 0xb0,
                0x47, 0x04, 0x33, 0xa5,
            ],
            Core::Eq16 => [
                0x0c, 0x54, 0x02, 0xb0, 0xad, 0xc8, 0xfc, 0x65, 0x70, 0x1b, 0xb7, 0x5b, 0x32, 0x54,
                0xc8, 0x35, 0xf8, 0xfe, 0xc1, 0x30, 0x81, 0xcd, 0x35, 0xe1, 0x32, 0x8f, 0x2b, 0xd7,
                0xdb, 0xd2, 0x3f, 0xa6,
            ],
            Core::Eq256 => [
                0x26, 0x0e, 0x1d, 0x13, 0x6d, 0xd7, 0x44, 0xfc, 0xb0, 0x50, 0x7a, 0x2d, 0x27, 0x70,
                0x27, 0xa7, 0x72, 0x43, 0x54, 0xeb, 0x17, 0x6b, 0x2f, 0xbf, 0x31, 0xc6, 0xc7, 0xd7,
                0xfb, 0x3e, 0xcd, 0x6f,
            ],
            Core::Eq32 => [
                0xf5, 0xd6, 0xed, 0xc8, 0xb6, 0x16, 0x4e, 0x12, 0x5b, 0xbb, 0xef, 0x08, 0xc9, 0xe0,
                0x8a, 0x1e, 0x6f, 0xd4, 0x92, 0xf5, 0xbd, 0xca, 0x6f, 0xdc, 0x8b, 0x5f, 0x5a, 0x6f,
                0x05, 0xc5, 0xab, 0x96,
            ],
            Core::Eq64 => [
                0x1f, 0x93, 0xac, 0xb8, 0x09, 0x2f, 0xa0, 0x6d, 0xea, 0xf3, 0xc3, 0x87, 0xf5, 0x4a,
                0x18, 0xff, 0xea, 0xa6, 0x9a, 0x47, 0xa6, 0xf5, 0xca, 0xf4, 0xae, 0x49, 0x7e, 0x5c,
                0xc2, 0xb3, 0x6c, 0x43,
            ],
            Core::Eq8 => [
                0xd7, 0x52, 0xfa, 0x7f, 0x51, 0x47, 0x30, 0x14, 0xeb, 0xb6, 0x9e, 0x1e, 0x1d, 0x2c,
                0x86, 0xd5, 0x11, 0x48, 0xb6, 0xba, 0xa0, 0x21, 0x37, 0xa4, 0x8f, 0x62, 0xd5, 0x7e,
                0xaf, 0x8d, 0xf1, 0xcd,
            ],
            Core::FeAdd => [
                0xa6, 0xc9, 0x0e, 0x02, 0xfd, 0xe4, 0xee, 0x6e, 0xef, 0x66, 0x67, 0x37, 0x49, 0x2e,
                0x14, 0xaf, 0xc8, 0x76, 0x25, 0x04, 0x97, 0x4a, 0xf5, 0xd5, 0x47, 0x2b, 0xb9, 0x43,
                0x3a, 0xd2, 0xd2, 0x94,
            ],
            Core::FeInvert => [
                0x7c, 0x4a, 0xba, 0xce, 0x33, 0xc7, 0x2b, 0x3b, 0xe1, 0xfd, 0x0e, 0xe3, 0x9f, 0xc6,
                0xcb, 0x3e, 0xe5, 0xc8, 0xf1, 0x1e, 0xf2, 0x19, 0x98, 0xc0, 0x60, 0x2b, 0x52, 0x15,
                0xaa, 0x2a, 0x75, 0xc2,
            ],
            Core::FeIsOdd => [
                0x30, 0xf5, 0x17, 0x1f, 0x58, 0xf1, 0x08, 0x9d, 0x5d, 0xcf, 0xb6, 0xe6, 0x68, 0x3f,
                0x5a, 0xde, 0x98, 0x4c, 0x07, 0x99, 0x76, 0x3c, 0xa7, 0x38, 0x3f, 0x75, 0xdf, 0x1c,
                0xa0, 0x81, 0x3e, 0xfe,
            ],
            Core::FeIsZero => [
                0xb0, 0xb7, 0x4d, 0x86, 0x51, 0xff, 0x55, 0x7c, 0xa9, 0x60, 0x44, 0xdd, 0x97, 0x28,
                0x13, 0x38, 0xa8, 0xf7, 0xd3, 0xac, 0xb3, 0x84, 0x7d, 0x03, 0xac, 0xbf, 0x3d, 0x32,
                0xd9, 0x6f, 0xae, 0x55,
            ],
            Core::FeMultiply => [
                0x50, 0x6b, 0x93, 0x19, 0xc1, 0x7a, 0x14, 0xa9, 0x46, 0x9d, 0x46, 0x27, 0x61, 0xa3,
                0x30, 0x3a, 0xb4, 0x7d, 0xdb, 0x3a, 0x30, 0x79, 0xfb, 0xa3, 0x40, 0x73, 0xaa, 0x55,
                0x42, 0x16, 0xa3, 0x88,
            ],
            Core::FeMultiplyBeta => [
                0x6e, 0x18, 0x0e, 0xea, 0xbe, 0x84, 0x22, 0xb7, 0x99, 0x68, 0xe7, 0x11, 0xdd, 0x00,
                0xa4, 0xb6, 0x57, 0x8b, 0xb2, 0x75, 0xbe, 0xf4, 0x7f, 0xe5, 0xff, 0x96, 0x8f, 0x14,
                0x72, 0xd7, 0x6f, 0x2a,
            ],
            Core::FeNegate => [
                0xd4, 0x37, 0xea, 0x00, 0x33, 0x98, 0x80, 0xb3, 0x83, 0xd8, 0x5f, 0xb2, 0xae, 0xaf,
                0x20, 0x1b, 0xbe, 0x8f, 0xfc, 0x83, 0x70, 0x50, 0x62, 0xf9, 0xc9, 0x68, 0x59, 0x0d,
                0x5d, 0xb3, 0x37, 0xf6,
            ],
            Core::FeNormalize => [
                0xec, 0x0c, 0x3d, 0xd9, 0xc5, 0x28, 0x63, 0x64, 0x78, 0xbe, 0xc0, 0xe1, 0x60, 0xe5,
                0x0a, 0xd9, 0xbf, 0x45, 0x2c, 0x5b, 0x6f, 0x84, 0xe9, 0x40, 0xe1, 0x65, 0x84, 0xeb,
                0x08, 0x5a, 0xce, 0x38,
            ],
            Core::FeSquare => [
                0xb9, 0x04, 0x77, 0x2d, 0x74, 0xa1, 0x85, 0xb8, 0x28, 0xeb, 0x15, 0x47, 0x28, 0xd2,
                0x49, 0xc5, 0x08, 0x47, 0x11, 0xe9, 0xa1, 0x83, 0x2b, 0x89, 0xca, 0xf2, 0xaf, 0x59,
                0xf9, 0x60, 0xe1, 0x18,
            ],
            Core::FeSquareRoot => [
                0x16, 0xfb, 0x9a, 0xce, 0xbe, 0x8b, 0x5b, 0x87, 0xf2, 0xea, 0x7d, 0xb6, 0xaa, 0x3a,
                0x2a, 0xf8, 0x8c, 0xa2, 0xb5, 0x8f, 0x02, 0xcd, 0xc8, 0x7e, 0x7c, 0xe6, 0xbe, 0x0c,
                0x1f, 0xfc, 0xe0, 0x14,
            ],
            Core::FullAdd16 => [
                0xc5, 0x03, 0xb0, 0x78, 0xdd, 0xe3, 0x99, 0xc6, 0x3a, 0xc4, 0xa2, 0x32, 0xbd, 0x2a,
                0x32, 0x9b, 0x04, 0x30, 0x8c, 0x75, 0xea, 0xec, 0x53, 0xa2, 0xf8, 0x89, 0xb8, 0xdf,
                0x0d, 0x03, 0x34, 0x72,
            ],
            Core::FullAdd32 => [
                0xa7, 0xaf, 0xd0, 0x40, 0xfc, 0xb0, 0xb2, 0xf2, 0x71, 0x90, 0x78, 0x1a, 0xe5, 0x3a,
                0x6c, 0xca, 0x00, 0xe9, 0xfe, 0x59, 0x53, 0x11, 0x15, 0xc2, 0x58, 0xcc, 0xb6, 0x9d,
                0x3b, 0xe5, 0xa2, 0x13,
            ],
            Core::FullAdd64 => [
                0x80, 0xa3, 0xef, 0x6c, 0xdb, 0x84, 0xae, 0x7c, 0x8d, 0xbc, 0xf3, 0xa1, 0x84, 0x24,
                0x84, 0xc0, 0x98, 0xdf, 0x6f, 0x19, 0x42, 0x7a, 0x5a, 0x4a, 0xdf, 0xe7, 0x6c, 0xd5,
                0xff, 0x28, 0x36, 0xca,
            ],
            Core::FullAdd8 => [
                0x4b, 0x90, 0x76, 0xb8, 0xc1, 0xad, 0x56, 0xc9, 0xdb, 0x6b, 0xb3, 0xba, 0xf5, 0x93,
                0x89, 0x54, 0x46, 0xce, 0x61, 0xc7, 0x4f, 0x79, 0x7e, 0xb8, 0xb2, 0x30, 0xd2, 0x05,
                0x42, 0x1c, 0x96, 0x17,
            ],
            Core::FullDecrement16 => [
                0xfb, 0xa3, 0xc9, 0x78, 0x6e, 0xa3, 0x07, 0xf6, 0xd8, 0x54, 0x34, 0xfd, 0xa2, 0x56,
                0x24, 0x82, 0x43, 0xa0, 0x0b, 0xac, 0x9a, 0x53, 0x53, 0xb6, 0x1e, 0xd3, 0x9c, 0x60,
                0x55, 0xb6, 0x93, 0xb0,
            ],
            Core::FullDecrement32 => [
                0x62, 0x3d, 0x21, 0xd0, 0x46, 0x79, 0x22, 0xc0, 0x01, 0xc5, 0x65, 0x68, 0x61, 0xd0,
                0xdd, 0xb8, 0x60, 0xc0, 0xc9, 0xa8, 0x6b, 0xd4, 0xcf, 0xdc, 0x37, 0xa1, 0x4c, 0x14,
                0x06, 0xe3, 0x44, 0x6e,
            ],
            Core::FullDecrement64 => [
                0x14, 0x8b, 0x3e, 0xe1, 0xf7, 0x49, 0xea, 0x0b, 0xfb, 0xa7, 0x63, 0xbe, 0xe9, 0x99,
                0xa2, 0x96, 0x77, 0x45, 0x6e, 0xae, 0x9e, 0xf5, 0x3a, 0xd8, 0x78, 0xf8, 0xb6, 0x14,
                0x94, 0xf0, 0x8f, 0x00,
            ],
            Core::FullDecrement8 => [
                0xb4, 0x1a, 0xfe, 0x97, 0x4e, 0xaa, 0x11, 0x82, 0xac, 0x46, 0x10, 0x52, 0x1e, 0x28,
                0x27, 0x81, 0x31, 0x8c, 0xe2, 0x95, 0xa3, 0xf2, 0x3f, 0x0b, 0x87, 0x6a, 0xe2, 0x69,
                0x67, 0x3f, 0xb1, 0xdf,
            ],
            Core::FullIncrement16 => [
                0xa6, 0x8e, 0xcc, 0xdb, 0x9e, 0xad, 0x29, 0x26, 0xc3, 0xe4, 0x5b, 0x4b, 0xae, 0x43,
                0x1c, 0xc4, 0x66, 0xd5, 0x8b, 0x8f, 0xac, 0xc9, 0x5a, 0x1b, 0x48, 0x44, 0xb9, 0x12,
                0xdf, 0x56, 0x76, 0xdf,
            ],
            Core::FullIncrement32 => [
                0xd0, 0xeb, 0x0e, 0x94, 0xa5, 0xc2, 0x57, 0x13, 0xeb, 0x94, 0x4c, 0xad, 0x4d, 0x70,
                0x1c, 0x6a, 0x96, 0x88, 0x09, 0xbc, 0x1a, 0xf9, 0x03, 0xfd, 0xb1, 0x1e, 0x6f, 0xad,
                0x0b, 0xb3, 0x1b, 0x10,
            ],
            Core::FullIncrement64 => [
                0xc0, 0x03, 0xd2, 0xe9, 0xb0, 0xa5, 0x10, 0xc2, 0xdd, 0x78, 0x3e, 0x7d, 0x64, 0xeb,
                0x87, 0xb3, 0x38, 0x55, 0xd3, 0x29, 0x90, 0xdf, 0xc2, 0x86, 0x26, 0x6e, 0x47, 0x8d,
                0xa4, 0xe7, 0x47, 0x91,
            ],
            Core::FullIncrement8 => [
                0x0b, 0xea, 0x24, 0x29, 0x18, 0xf2, 0xdd, 0x17, 0x64, 0x77, 0x78, 0x11, 0xe4, 0x44,
                0x28, 0x63, 0x93, 0x52, 0x25, 0xb0, 0xf8, 0xb2, 0x39, 0xc2, 0x37, 0x52, 0xf9, 0xd8,
                0x53, 0x92, 0xa1, 0x39,
            ],
            Core::FullLeftShift16_1 => [
                0xb3, 0x66, 0xa8, 0x16, 0x92, 0x2f, 0xc4, 0x55, 0x01, 0x0f, 0xe8, 0x8a, 0x5f, 0x6a,
                0x5c, 0xf2, 0xce, 0xa9, 0x17, 0xe1, 0x2b, 0xd1, 0x40, 0xae, 0x6d, 0x43, 0xb6, 0x41,
                0xe5, 0x7f, 0x42, 0xb3,
            ],
            Core::FullLeftShift16_2 => [
                0x27, 0x96, 0x0d, 0x0d, 0xf2, 0xfb, 0xbc, 0x38, 0x99, 0x3d, 0x86, 0xfb, 0x8f, 0x0c,
                0xd2, 0xc3, 0x43, 0x4e, 0xdb, 0x11, 0x04, 0x82, 0x13, 0xc1, 0x41, 0x18, 0x93, 0xca,
                0x99, 0x33, 0xb2, 0xee,
            ],
            Core::FullLeftShift16_4 => [
                0x65, 0x51, 0x37, 0xbe, 0xc5, 0xc0, 0x36, 0x8f, 0x29, 0xbc, 0x99, 0x2c, 0x88, 0x42,
                0x1a, 0x15, 0x98, 0x56, 0x40, 0x39, 0x7b, 0x61, 0x7f, 0xc4, 0x8d, 0x33, 0x21, 0x0f,
                0xc0, 0x05, 0x3a, 0xd1,
            ],
            Core::FullLeftShift16_8 => [
                0x16, 0x8f, 0x57, 0x6a, 0xa5, 0x6e, 0xa4, 0x7e, 0x07, 0x06, 0x46, 0xe7, 0x88, 0x96,
                0xbe, 0xb2, 0x49, 0x8b, 0x1a, 0xe6, 0xb1, 0xff, 0x9c, 0x78, 0x62, 0x70, 0xe9, 0x55,
                0x65, 0x84, 0x19, 0x29,
            ],
            Core::FullLeftShift32_1 => [
                0xd7, 0xcd, 0x52, 0x24, 0x49, 0x11, 0x8e, 0x81, 0x00, 0xa7, 0x66, 0x2f, 0x4d, 0xf0,
                0x39, 0xf8, 0xca, 0xeb, 0xf4, 0x33, 0xeb, 0x03, 0x9e, 0xdc, 0x42, 0xe8, 0x82, 0x37,
                0x92, 0xcc, 0xea, 0x8a,
            ],
            Core::FullLeftShift32_16 => [
                0x8b, 0xd8, 0x0d, 0x4d, 0x2f, 0x8b, 0x22, 0x46, 0xc1, 0x23, 0x15, 0xc4, 0x28, 0x41,
                0xb4, 0xe4, 0x0a, 0x71, 0xae, 0x76, 0x96, 0x6a, 0x08, 0x95, 0x4d, 0x66, 0x6b, 0x86,
                0x32, 0x86, 0x74, 0x37,
            ],
            Core::FullLeftShift32_2 => [
                0x13, 0x06, 0x3d, 0x62, 0x93, 0x83, 0x29, 0x31, 0x1f, 0xb7, 0xda, 0xbb, 0x15, 0xc3,
                0xfe, 0x58, 0xc2, 0x88, 0x76, 0x83, 0x00, 0x97, 0xec, 0xc6, 0xbf, 0xdd, 0x48, 0x0b,
                0xe1, 0x98, 0x81, 0x46,
            ],
            Core::FullLeftShift32_4 => [
                0x25, 0xa1, 0xb5, 0xdd, 0xe5, 0xdb, 0x28, 0x4e, 0x8a, 0x88, 0x21, 0x26, 0x7c, 0x26,
                0x53, 0x01, 0x14, 0xbb, 0xe6, 0x71, 0xcf, 0xaf, 0xb4, 0x4a, 0x60, 0xd7, 0x50, 0x27,
                0x67, 0xdb, 0x78, 0x2e,
            ],
            Core::FullLeftShift32_8 => [
                0xce, 0x54, 0x70, 0xaf, 0xbf, 0xad, 0xbf, 0xba, 0x68, 0xf9, 0xb2, 0xd5, 0xb0, 0x64,
                0x5a, 0x44, 0x08, 0xbe, 0x6f, 0x85, 0xa6, 0x9c, 0x7f, 0x09, 0xd0, 0x96, 0x45, 0x53,
                0x68, 0x04, 0x87, 0x64,
            ],
            Core::FullLeftShift64_1 => [
                0x05, 0x1f, 0x36, 0x05, 0x86, 0xc3, 0x79, 0xac, 0x2c, 0xe3, 0x99, 0xcb, 0xeb, 0x68,
                0x7e, 0x77, 0x53, 0xb1, 0x5d, 0x73, 0x03, 0xdd, 0x31, 0x6c, 0xbd, 0x12, 0x30, 0x12,
                0x08, 0x7c, 0xc6, 0x6f,
            ],
            Core::FullLeftShift64_16 => [
                0xb2, 0x48, 0xbe, 0x4d, 0xfc, 0xb8, 0x8c, 0x5d, 0x89, 0xb1, 0xca, 0x61, 0x86, 0xa0,
                0x41, 0xe9, 0x02, 0xb4, 0xc8, 0xa6, 0x2b, 0xb1, 0x6e, 0x09, 0xfe, 0x15, 0x61, 0x6e,
                0x0e, 0x3a, 0xbd, 0x6d,
            ],
            Core::FullLeftShift64_2 => [
                0x34, 0xbb, 0x51, 0x62, 0x6b, 0x1d, 0x6b, 0x89, 0x7a, 0xbc, 0x15, 0x5d, 0x03, 0x4f,
                0xe0, 0x66, 0x3a, 0x0e, 0xc0, 0xfd, 0x8f, 0x64, 0x0e, 0x5f, 0xe1, 0xbf, 0x3c, 0xb7,
                0x67, 0x0a, 0x29, 0x25,
            ],
            Core::FullLeftShift64_32 => [
                0x9d, 0xac, 0x8c, 0xd7, 0xfd, 0x8b, 0x48, 0x88, 0x9e, 0x55, 0xc5, 0xaa, 0x12, 0xfe,
                0x97, 0xb7, 0x29, 0xfe, 0xbc, 0x04, 0x1a, 0x9f, 0xff, 0x44, 0xc4, 0xd9, 0xb6, 0xf1,
                0xe0, 0x7e, 0xb4, 0x42,
            ],
            Core::FullLeftShift64_4 => [
                0x94, 0xb7, 0x3d, 0xad, 0xc3, 0xee, 0xeb, 0x2e, 0xe4, 0xa4, 0xd4, 0x44, 0xdd, 0x0f,
                0x72, 0xac, 0x30, 0x62, 0x01, 0xf2, 0xff, 0xcf, 0x71, 0x4b, 0x8e, 0xbe, 0x79, 0x82,
                0x74, 0x4c, 0x0c, 0x7e,
            ],
            Core::FullLeftShift64_8 => [
                0x0e, 0xf7, 0x14, 0x75, 0x7e, 0xcf, 0x11, 0xca, 0x3c, 0x73, 0xce, 0x25, 0xef, 0x24,
                0xee, 0x72, 0x95, 0xdd, 0x41, 0x71, 0xcc, 0x16, 0x28, 0x7f, 0xe6, 0x97, 0x1b, 0xd6,
                0x7b, 0x47, 0x8b, 0x46,
            ],
            Core::FullLeftShift8_1 => [
                0x9b, 0xfa, 0x48, 0xb7, 0xad, 0x51, 0x00, 0x91, 0xba, 0x60, 0x85, 0x44, 0x62, 0xd8,
                0x59, 0xef, 0xd1, 0xa2, 0xaa, 0x18, 0x73, 0x1b, 0x5f, 0x0c, 0x9e, 0x2b, 0xa4, 0xd8,
                0x9d, 0x3a, 0xa8, 0x43,
            ],
            Core::FullLeftShift8_2 => [
                0x79, 0x7c, 0x20, 0x87, 0x01, 0xb2, 0xa4, 0xe1, 0x04, 0x9e, 0x83, 0xd6, 0x95, 0xf5,
                0x54, 0xb9, 0x84, 0xf5, 0xdb, 0x28, 0x21, 0x52, 0x55, 0x58, 0x7c, 0x37, 0x34, 0x21,
                0x51, 0xb7, 0x24, 0x1d,
            ],
            Core::FullLeftShift8_4 => [
                0x37, 0xbd, 0xac, 0x91, 0x53, 0x8f, 0x22, 0x19, 0xcb, 0x89, 0xdf, 0x0e, 0xf9, 0xf1,
                0x97, 0xcd, 0x68, 0x03, 0x1f, 0x27, 0x67, 0xe8, 0x94, 0xf0, 0x01, 0xc2, 0x6f, 0xff,
                0x5e, 0xeb, 0x58, 0xcd,
            ],
            Core::FullMultiply16 => [
                0x09, 0xba, 0xff, 0x92, 0x1e, 0x9f, 0x14, 0xd1, 0x20, 0x8d, 0x1d, 0xd8, 0x26, 0x4c,
                0xf1, 0xf3, 0xb8, 0x54, 0xc9, 0xaf, 0x21, 0xf7, 0x78, 0xb2, 0xb5, 0x5a, 0x8a, 0x42,
                0x6d, 0xfe, 0x89, 0x28,
            ],
            Core::FullMultiply32 => [
                0x10, 0xc4, 0xb8, 0xc4, 0xc0, 0xac, 0xd9, 0x73, 0x90, 0xf8, 0x5c, 0xb3, 0xf5, 0xff,
                0xe3, 0x6a, 0x29, 0x20, 0x37, 0xc1, 0x90, 0xee, 0xba, 0xb3, 0xe9, 0x89, 0x34, 0xfe,
                0x93, 0xb2, 0xed, 0x90,
            ],
            Core::FullMultiply64 => [
                0x2d, 0xb1, 0x9d, 0xba, 0x90, 0xef, 0x86, 0x7b, 0x5a, 0x3e, 0x91, 0x4b, 0x89, 0xfd,
                0xa2, 0xda, 0x63, 0x7c, 0xa8, 0x0c, 0x42, 0x67, 0xe1, 0x98, 0x18, 0x37, 0xee, 0x3c,
                0x6f, 0xe3, 0xda, 0xf5,
            ],
            Core::FullMultiply8 => [
                0x7f, 0x46, 0xee, 0x72, 0x84, 0xf3, 0x9e, 0x73, 0x42, 0x75, 0xa2, 0x50, 0x9a, 0x0b,
                0x73, 0x7e, 0xd9, 0x39, 0x11, 0x5f, 0x02, 0x19, 0xa5, 0x74, 0xd4, 0x69, 0xcd, 0x30,
                0xb8, 0x19, 0xef, 0xe3,
            ],
            Core::FullRightShift16_1 => [
                0x7e, 0xbe, 0x0c, 0x66, 0xc3, 0xc7, 0xdc, 0x16, 0xa5, 0x46, 0x9e, 0x91, 0x79, 0x09,
                0x84, 0x17, 0xac, 0x5f, 0x20, 0xa3, 0x9c, 0xc4, 0x1a, 0xc3, 0x82, 0xfb, 0x1d, 0xbd,
                0x98, 0xe8, 0xe3, 0x0f,
            ],
            Core::FullRightShift16_2 => [
                0x8d, 0xb0, 0xc2, 0x16, 0x19, 0xc6, 0x2d, 0x63, 0xd4, 0xc2, 0x7b, 0xfc, 0xf6, 0x47,
                0xd7, 0x09, 0xce, 0x37, 0xbe, 0xd0, 0x57, 0x18, 0xe9, 0x3e, 0x45, 0x15, 0xe2, 0x9e,
                0xf3, 0x73, 0x0c, 0xf4,
            ],
            Core::FullRightShift16_4 => [
                0x5c, 0x74, 0xb1, 0x32, 0x06, 0x31, 0x79, 0x17, 0xe0, 0x70, 0xe5, 0xfc, 0x1c, 0x82,
                0xf4, 0xc5, 0xc2, 0xfb, 0xe9, 0xf3, 0x1b, 0x81, 0x29, 0x46, 0xba, 0x23, 0x0d, 0x8c,
                0x94, 0xd4, 0x06, 0x16,
            ],
            Core::FullRightShift16_8 => [
                0x11, 0x05, 0x81, 0x8a, 0xc9, 0x48, 0xd7, 0xbb, 0x63, 0x47, 0x07, 0xe6, 0x9d, 0xbf,
                0x1f, 0x67, 0x90, 0x58, 0xa1, 0x3d, 0x35, 0xfa, 0xc2, 0xa6, 0x4d, 0xf9, 0x72, 0x62,
                0xf2, 0x42, 0xb6, 0x3b,
            ],
            Core::FullRightShift32_1 => [
                0x9b, 0x42, 0xc8, 0xf3, 0x3b, 0xc5, 0x75, 0x0e, 0x2a, 0x83, 0xaa, 0xdb, 0xf2, 0x9c,
                0xc7, 0xfc, 0xb9, 0x50, 0xfe, 0x5a, 0x40, 0xaa, 0x0e, 0xc5, 0x24, 0x52, 0xe5, 0x33,
                0xf8, 0x25, 0xa1, 0x15,
            ],
            Core::FullRightShift32_16 => [
                0x0a, 0xe5, 0x65, 0x9c, 0x2f, 0xa7, 0x57, 0x94, 0x78, 0xeb, 0xd5, 0x7c, 0x4c, 0x98,
                0xae, 0xe7, 0x77, 0x01, 0x56, 0x45, 0xb2, 0x84, 0x31, 0x81, 0x64, 0xfc, 0xbd, 0x30,
                0x65, 0xfc, 0x87, 0x3c,
            ],
            Core::FullRightShift32_2 => [
                0x57, 0xfb, 0x1c, 0x03, 0xc2, 0xeb, 0x17, 0xf6, 0x23, 0x47, 0x87, 0x34, 0xfd, 0x69,
                0x37, 0xf9, 0xe3, 0xef, 0x02, 0x7c, 0x15, 0x60, 0x03, 0x8f, 0xa6, 0x06, 0x69, 0x05,
                0x17, 0x89, 0xe3, 0x68,
            ],
            Core::FullRightShift32_4 => [
                0x85, 0x82, 0xcd, 0xfa, 0x74, 0xef, 0x46, 0x6b, 0x81, 0x27, 0xb1, 0x97, 0x88, 0x13,
                0x45, 0x93, 0x99, 0x8e, 0x49, 0x69, 0x00, 0xb3, 0x8f, 0x0f, 0x3d, 0x37, 0x58, 0x18,
                0xd6, 0x73, 0x45, 0x1e,
            ],
            Core::FullRightShift32_8 => [
                0xd9, 0x05, 0x93, 0x2e, 0xbf, 0xca, 0x2a, 0x38, 0x61, 0x9d, 0x80, 0x7e, 0x28, 0xff,
                0x2e, 0x0d, 0x3b, 0xe0, 0x8a, 0x26, 0x06, 0x76, 0xd2, 0x57, 0xef, 0xa0, 0x40, 0xc3,
                0x05, 0xaa, 0xdc, 0x33,
            ],
            Core::FullRightShift64_1 => [
                0x3c, 0x15, 0x20, 0x9b, 0x99, 0xd2, 0x84, 0x5e, 0x22, 0x5e, 0x14, 0xe1, 0xe9, 0xe5,
                0xe6, 0xa4, 0x87, 0x8b, 0xc8, 0xce, 0xa3, 0xf9, 0xf3, 0x6b, 0x8b, 0x53, 0x5a, 0xc6,
                0x83, 0xe2, 0x9d, 0x00,
            ],
            Core::FullRightShift64_16 => [
                0x02, 0x85, 0x25, 0x7b, 0x09, 0x0d, 0x8d, 0xa1, 0x28, 0xef, 0x64, 0xa8, 0x0c, 0x8d,
                0x16, 0xfd, 0xc3, 0xbf, 0x5c, 0xe5, 0x0f, 0xcd, 0x56, 0xfe, 0xc5, 0xf9, 0x02, 0x55,
                0xd9, 0xc8, 0xdf, 0x47,
            ],
            Core::FullRightShift64_2 => [
                0x7e, 0xc2, 0xdd, 0x65, 0xc9, 0xe0, 0x13, 0xe3, 0xe4, 0xce, 0x90, 0xfb, 0xeb, 0x3f,
                0xb1, 0xc7, 0x8c, 0xcc, 0x5d, 0x2a, 0x7d, 0x26, 0xd8, 0xaf, 0x77, 0xf9, 0x9d, 0xe8,
                0x4c, 0xf7, 0x29, 0x73,
            ],
            Core::FullRightShift64_32 => [
                0x35, 0x6f, 0x7d, 0xd4, 0x6b, 0xa3, 0x3f, 0x84, 0xb0, 0x66, 0x72, 0xfd, 0xe9, 0xa2,
                0x97, 0x2e, 0x80, 0xf3, 0xea, 0x96, 0x5a, 0xe8, 0xbc, 0x0b, 0xff, 0x67, 0xaa, 0x2f,
                0x69, 0xf1, 0x0b, 0x56,
            ],
            Core::FullRightShift64_4 => [
                0x05, 0x46, 0x4a, 0x33, 0x35, 0xaf, 0xbb, 0x09, 0xd0, 0x46, 0x82, 0x8a, 0x92, 0x2c,
                0x4d, 0xa0, 0xec, 0xee, 0xb1, 0x09, 0x77, 0xe4, 0x68, 0x01, 0xc9, 0x3c, 0xdd, 0x66,
                0x8f, 0x22, 0xee, 0x63,
            ],
            Core::FullRightShift64_8 => [
                0x70, 0x17, 0x2e, 0x1a, 0x69, 0x48, 0xbf, 0x40, 0x12, 0x0e, 0x68, 0xfb, 0x8b, 0x4b,
                0x23, 0xbc, 0x35, 0x5a, 0x12, 0x00, 0x2c, 0xcc, 0x1d, 0xb6, 0x47, 0xc8, 0x9b, 0x12,
                0xd1, 0x0e, 0xc5, 0x06,
            ],
            Core::FullRightShift8_1 => [
                0x56, 0x69, 0xdb, 0xfc, 0xc6, 0x33, 0xec, 0x0b, 0xdf, 0x59, 0xe2, 0x2f, 0x03, 0xed,
                0x4b, 0x64, 0x19, 0x20, 0x95, 0xf5, 0xdf, 0x20, 0xff, 0xc1, 0x2d, 0xd9, 0x0d, 0x7c,
                0xda, 0x11, 0x37, 0x4f,
            ],
            Core::FullRightShift8_2 => [
                0x1f, 0x94, 0x43, 0x61, 0x09, 0xdf, 0x52, 0xb3, 0x45, 0xfa, 0x3a, 0x89, 0xac, 0x2a,
                0x49, 0xed, 0xc9, 0xd2, 0x85, 0xf2, 0x1f, 0x45, 0xed, 0x11, 0xd7, 0x75, 0xf7, 0xf7,
                0xf3, 0x9d, 0x3e, 0x8f,
            ],
            Core::FullRightShift8_4 => [
                0x71, 0x46, 0x98, 0xa2, 0x76, 0x84, 0xb5, 0xba, 0xa6, 0xb6, 0x48, 0x0e, 0xe3, 0xb2,
                0x57, 0xcb, 0xb7, 0xcd, 0xab, 0x74, 0x72, 0xf3, 0x71, 0xa6, 0x27, 0x06, 0x18, 0xc0,
                0xab, 0x12, 0x90, 0x8b,
            ],
            Core::FullSubtract16 => [
                0x40, 0x09, 0x61, 0x52, 0xb5, 0x4e, 0x74, 0x25, 0x45, 0x55, 0xa6, 0x5d, 0xcc, 0xc6,
                0x29, 0xdf, 0x57, 0xb9, 0x79, 0xc8, 0x47, 0x00, 0x54, 0x50, 0x36, 0xfe, 0x19, 0x0a,
                0x6a, 0xf3, 0xd3, 0x8a,
            ],
            Core::FullSubtract32 => [
                0xe7, 0x93, 0x0d, 0x64, 0x35, 0xa9, 0x68, 0x0b, 0xef, 0xb4, 0x9d, 0xb7, 0xd8, 0x7c,
                0x2f, 0x50, 0xaf, 0xd4, 0x6d, 0x98, 0x88, 0x0d, 0xed, 0x50, 0xe5, 0x05, 0x5f, 0xa3,
                0x09, 0xe1, 0xaf, 0xca,
            ],
            Core::FullSubtract64 => [
                0xff, 0x28, 0x1d, 0xf8, 0xc4, 0x2a, 0x31, 0x59, 0xd9, 0xff, 0xa9, 0x25, 0x16, 0xca,
                0x89, 0x3e, 0x23, 0xb0, 0xeb, 0x93, 0x8b, 0x4c, 0xb0, 0xb3, 0xf1, 0x34, 0x46, 0x8e,
                0x9f, 0x4e, 0xbc, 0x46,
            ],
            Core::FullSubtract8 => [
                0x7e, 0x3d, 0xcf, 0xe4, 0x56, 0xae, 0x3c, 0x5c, 0x87, 0xde, 0xbf, 0x04, 0x71, 0x89,
                0xc2, 0x74, 0x82, 0xa4, 0xff, 0x4e, 0x8c, 0xfd, 0x1f, 0x17, 0x30, 0xc8, 0x7d, 0x2b,
                0x7b, 0xff, 0x73, 0xba,
            ],
            Core::GeIsOnCurve => [
                0x7d, 0x44, 0x87, 0x19, 0xf5, 0xf9, 0x57, 0x2b, 0xf5, 0x40, 0x2e, 0x12, 0xd1, 0x93,
                0xaf, 0xf6, 0x77, 0x48, 0x2d, 0x66, 0xff, 0x3d, 0xcf, 0x27, 0x48, 0xf2, 0x5c, 0x6b,
                0x73, 0x77, 0x02, 0x8c,
            ],
            Core::GeNegate => [
                0x3d, 0x2c, 0x8d, 0xe4, 0xc7, 0x01, 0x5f, 0xd3, 0x13, 0x26, 0x95, 0xfd, 0x66, 0xdf,
                0xcf, 0x0f, 0x17, 0x78, 0xc7, 0x91, 0x85, 0x26, 0x8e, 0x9f, 0xae, 0x77, 0x89, 0xda,
                0x53, 0x8e, 0xca, 0x59,
            ],
            Core::GejAdd => [
                0x45, 0xba, 0x7f, 0x3d, 0x1e, 0x1e, 0x6d, 0x34, 0x9f, 0xcf, 0x86, 0x98, 0x7b, 0x0e,
                0x7f, 0x7a, 0xce, 0x66, 0x2e, 0x82, 0x20, 0x1d, 0x35, 0x02, 0x60, 0x45, 0x4e, 0x2f,
                0xfe, 0xec, 0xb5, 0x4d,
            ],
            Core::GejDouble => [
                0x23, 0xe9, 0x78, 0xf3, 0x41, 0x54, 0x11, 0x9b, 0xde, 0xfc, 0x5d, 0x13, 0xfc, 0xfd,
                0x0a, 0x34, 0xa7, 0x5e, 0x37, 0x26, 0xd6, 0xcb, 0x25, 0x81, 0x33, 0x70, 0xad, 0x7d,
                0x9d, 0xe5, 0xe1, 0x33,
            ],
            Core::GejEquiv => [
                0xb9, 0x4b, 0x2a, 0xac, 0x73, 0xa6, 0x7f, 0x44, 0x95, 0x85, 0x99, 0x13, 0x4d, 0xe2,
                0x30, 0x17, 0x9e, 0x9d, 0x6b, 0xb6, 0x47, 0xfd, 0x06, 0x11, 0x15, 0x8a, 0xab, 0xa7,
                0x0b, 0x73, 0xe4, 0x00,
            ],
            Core::GejGeAdd => [
                0xf1, 0x16, 0x0b, 0x6f, 0x5e, 0xe2, 0xc5, 0x82, 0xe4, 0x95, 0x66, 0xe6, 0xc3, 0x86,
                0xb3, 0x80, 0x94, 0xab, 0xc1, 0xa7, 0x18, 0x2d, 0x33, 0xa1, 0x50, 0x1f, 0xa2, 0xaa,
                0xf0, 0x0a, 0xf3, 0xea,
            ],
            Core::GejGeAddEx => [
                0xc3, 0xd7, 0x34, 0x7f, 0xfe, 0x2d, 0x9c, 0x83, 0x9a, 0xac, 0x56, 0x7e, 0x29, 0x98,
                0xe0, 0x16, 0xaf, 0x39, 0x4e, 0x2a, 0x19, 0x29, 0x31, 0x4b, 0x52, 0xe3, 0x1e, 0xed,
                0x67, 0x8e, 0x30, 0xbf,
            ],
            Core::GejGeEquiv => [
                0x27, 0xc2, 0x99, 0x69, 0x13, 0x9f, 0x8d, 0x57, 0xed, 0xc9, 0x89, 0x5c, 0x30, 0x40,
                0x3d, 0xf0, 0x15, 0xc5, 0x0c, 0xe7, 0x21, 0xc3, 0x81, 0xfb, 0x19, 0x7c, 0x0c, 0x04,
                0x03, 0xf1, 0xdb, 0x0c,
            ],
            Core::GejInfinity => [
                0xaa, 0xfb, 0x93, 0x80, 0xd6, 0x1a, 0x7f, 0x14, 0x78, 0x46, 0x80, 0x6b, 0x2c, 0xc3,
                0x74, 0xfb, 0xe8, 0x2d, 0xd1, 0xae, 0xd4, 0x85, 0xb9, 0x8a, 0x0f, 0x16, 0x4b, 0x3a,
                0x54, 0xc2, 0xc0, 0xb0,
            ],
            Core::GejIsInfinity => [
                0xdb, 0x49, 0x5f, 0xd1, 0x31, 0x42, 0xe9, 0xb3, 0x37, 0x63, 0xfc, 0x6d, 0x48, 0xd2,
                0xfb, 0x0e, 0x71, 0xb0, 0xd9, 0xd9, 0x9b, 0xd7, 0x26, 0xf4, 0x7a, 0xd1, 0x3f, 0xc5,
                0x56, 0x06, 0x70, 0xa2,
            ],
            Core::GejIsOnCurve => [
                0xbf, 0x4c, 0xa1, 0x3f, 0xf2, 0x12, 0xe3, 0x4b, 0xf1, 0x7d, 0x90, 0xc1, 0x2e, 0x45,
                0x3d, 0x08, 0xac, 0x7d, 0xaa, 0x4a, 0x47, 0xd5, 0x7e, 0x85, 0xb4, 0x3f, 0x2d, 0x43,
                0x66, 0xd4, 0x3d, 0xda,
            ],
            Core::GejNegate => [
                0x01, 0xbd, 0x1a, 0x35, 0x1f, 0xb8, 0x16, 0x4c, 0x81, 0x3d, 0x91, 0x6d, 0x07, 0x77,
                0x49, 0x99, 0x6b, 0x7d, 0xb1, 0x18, 0xd3, 0x15, 0x86, 0xca, 0x9d, 0x75, 0xe7, 0x56,
                0x35, 0x18, 0xf4, 0x54,
            ],
            Core::GejNormalize => [
                0xec, 0x59, 0x7d, 0x17, 0xe2, 0xef, 0xb6, 0xd2, 0xa0, 0x02, 0xd5, 0x0e, 0x67, 0x75,
                0x27, 0xd3, 0xd4, 0xa2, 0x90, 0x7a, 0x11, 0x9d, 0x68, 0xf1, 0x22, 0x84, 0xb9, 0xa1,
                0xb0, 0xd2, 0x30, 0x3a,
            ],
            Core::GejRescale => [
                0x29, 0x77, 0xd9, 0x53, 0xef, 0x7a, 0x11, 0x56, 0xce, 0xc6, 0xdb, 0x2d, 0xc2, 0x92,
                0x54, 0x12, 0x75, 0xcb, 0xc8, 0x2f, 0xb8, 0x29, 0xfd, 0x67, 0x1b, 0x97, 0x2e, 0x89,
                0xeb, 0xed, 0x0c, 0x24,
            ],
            Core::GejXEquiv => [
                0xf9, 0xf1, 0x89, 0xfc, 0x00, 0xb6, 0x1f, 0x72, 0xf1, 0x0b, 0xaa, 0xa2, 0x1b, 0xcd,
                0x88, 0xe5, 0xd2, 0x2e, 0x0a, 0xa9, 0xb7, 0x50, 0x9a, 0xe1, 0x62, 0xa1, 0x83, 0xa4,
                0xb6, 0x64, 0xa4, 0xaf,
            ],
            Core::GejYIsOdd => [
                0x9e, 0xb6, 0xe4, 0x53, 0x5f, 0xb6, 0x9b, 0xf6, 0x09, 0x91, 0x65, 0x99, 0xf1, 0x34,
                0x5a, 0xd7, 0x73, 0x5d, 0xa3, 0xf3, 0x94, 0x8d, 0x06, 0x86, 0x90, 0x8e, 0x44, 0xf4,
                0x5b, 0x2f, 0xf6, 0x0c,
            ],
            Core::Generate => [
                0x14, 0x88, 0x85, 0xac, 0x73, 0x81, 0x31, 0x13, 0xc5, 0x23, 0xe8, 0x09, 0xbe, 0xa4,
                0x7f, 0xfd, 0x8b, 0x1d, 0xaf, 0x37, 0x8d, 0x9d, 0xd5, 0x4b, 0xf9, 0x66, 0xcc, 0xb8,
                0x83, 0xb1, 0xa9, 0x84,
            ],
            Core::HashToCurve => [
                0xef, 0x4f, 0x54, 0x8b, 0x3c, 0x6c, 0x75, 0x17, 0x5f, 0x2c, 0xe2, 0xd1, 0x99, 0x3b,
                0x2d, 0x19, 0x9b, 0xeb, 0x16, 0xc0, 0xa1, 0x40, 0x17, 0x5c, 0x48, 0xa1, 0x27, 0x7e,
                0xfc, 0x43, 0xa9, 0x9b,
            ],
            Core::High1 => [
                0xb1, 0x09, 0xcf, 0x1c, 0xce, 0x35, 0xf7, 0xe9, 0xb6, 0x49, 0x67, 0x1a, 0x9b, 0x45,
                0xdb, 0xc2, 0x40, 0x99, 0xa7, 0x13, 0xae, 0xb9, 0xa8, 0x9c, 0xc4, 0xcf, 0x6e, 0xf6,
                0xed, 0x8b, 0x30, 0x8b,
            ],
            Core::High16 => [
                0x03, 0x5d, 0xad, 0xd9, 0xd7, 0xbf, 0x74, 0x33, 0x64, 0x45, 0xe7, 0x1d, 0xdc, 0x4d,
                0x82, 0x02, 0x24, 0xff, 0x7e, 0x38, 0xe0, 0xb8, 0xd5, 0x2b, 0xec, 0x97, 0x29, 0xb5,
                0x72, 0xb5, 0x31, 0xf9,
            ],
            Core::High32 => [
                0xc5, 0xf1, 0xdf, 0x0d, 0x64, 0xa2, 0x73, 0x7a, 0x63, 0x1b, 0x3a, 0xae, 0x8f, 0x26,
                0x0e, 0x8b, 0x8d, 0xc1, 0x95, 0x7b, 0xd0, 0x92, 0x91, 0x1b, 0x91, 0xd2, 0x07, 0x8a,
                0xd2, 0x1e, 0x41, 0x8a,
            ],
            Core::High64 => [
                0xa3, 0x12, 0x63, 0x3e, 0x0a, 0x23, 0x05, 0xe6, 0x9b, 0x3f, 0x34, 0x1d, 0x91, 0xd6,
                0x83, 0xdd, 0x94, 0x19, 0x6a, 0x2f, 0x90, 0x05, 0xc9, 0xb1, 0x87, 0x2a, 0x2c, 0x15,
                0xad, 0x46, 0xcf, 0x17,
            ],
            Core::High8 => [
                0xcb, 0xd7, 0x8d, 0x50, 0xaf, 0x77, 0x99, 0x85, 0x5a, 0xdc, 0x49, 0x03, 0xdb, 0xbe,
                0xfc, 0x13, 0x45, 0xd5, 0x14, 0x84, 0xf0, 0x3d, 0x3c, 0x75, 0x5c, 0xaa, 0xa5, 0xca,
                0xa9, 0x7d, 0x4a, 0x14,
            ],
            Core::Increment16 => [
                0x86, 0x77, 0x49, 0x49, 0x39, 0xb2, 0x7b, 0x86, 0xcb, 0x5a, 0x8c, 0x7f, 0x81, 0x72,
                0xad, 0x55, 0x50, 0x95, 0x31, 0xc9, 0xb0, 0xe1, 0x1e, 0x99, 0x75, 0x7e, 0x29, 0x6c,
                0xc3, 0xc7, 0xc1, 0x92,
            ],
            Core::Increment32 => [
                0x6b, 0xdb, 0xab, 0x7c, 0xfc, 0x16, 0xc5, 0x03, 0x36, 0x3c, 0x2f, 0x07, 0x7e, 0x02,
                0xc3, 0x35, 0xda, 0x40, 0x61, 0x75, 0xd1, 0x92, 0xfb, 0xef, 0x50, 0xc0, 0x7f, 0xc2,
                0x79, 0xb3, 0xf4, 0x0c,
            ],
            Core::Increment64 => [
                0x20, 0xe7, 0x5e, 0x71, 0x7c, 0xb7, 0x6d, 0x46, 0x95, 0x56, 0x4f, 0x7c, 0x20, 0x22,
                0x1b, 0x7a, 0x01, 0x43, 0x13, 0x87, 0x38, 0xf1, 0x51, 0xaa, 0x19, 0x5e, 0xb1, 0x70,
                0xec, 0x13, 0xc0, 0x49,
            ],
            Core::Increment8 => [
                0x5f, 0x4e, 0x05, 0x6e, 0xf4, 0xed, 0x8d, 0x68, 0xbf, 0x91, 0x1f, 0xc5, 0xcb, 0x69,
                0x03, 0x7e, 0xbf, 0x6c, 0x92, 0x21, 0x73, 0x43, 0xa8, 0x90, 0x5d, 0x38, 0xc4, 0x32,
                0xc1, 0x83, 0x23, 0x3c,
            ],
            Core::IsOne16 => [
                0x1b, 0xd3, 0xa2, 0x53, 0xdb, 0x24, 0x3f, 0xca, 0x45, 0x53, 0x37, 0x99, 0xfe, 0x91,
                0x48, 0x38, 0xc3, 0x8e, 0x38, 0x06, 0xb1, 0x2b, 0xd7, 0xe8, 0x5c, 0xa7, 0x12, 0x07,
                0xa8, 0x84, 0x62, 0xb0,
            ],
            Core::IsOne32 => [
                0x78, 0xb1, 0xba, 0xe0, 0x99, 0xec, 0x9c, 0x59, 0xcb, 0xf4, 0x12, 0x62, 0x51, 0xc1,
                0xe9, 0x67, 0x41, 0xb3, 0x50, 0xd5, 0x63, 0xbd, 0x74, 0xd5, 0x44, 0x18, 0xba, 0x78,
                0xeb, 0xea, 0x25, 0xbf,
            ],
            Core::IsOne64 => [
                0x81, 0x7b, 0x95, 0xa5, 0x39, 0x5e, 0xfb, 0xec, 0xbb, 0x85, 0x15, 0xa5, 0x5b, 0x3f,
                0xfe, 0x1a, 0x4d, 0x7b, 0xac, 0x6e, 0x23, 0xdb, 0xca, 0x54, 0xad, 0x60, 0x66, 0x66,
                0x2f, 0x20, 0x2b, 0x93,
            ],
            Core::IsOne8 => [
                0xf6, 0x92, 0x54, 0x91, 0xd3, 0x4b, 0x37, 0x74, 0x2c, 0xb0, 0x8d, 0xec, 0x19, 0x3e,
                0xe5, 0x12, 0x5f, 0x93, 0x3c, 0xad, 0xcc, 0x23, 0x2a, 0xed, 0xee, 0xdb, 0x57, 0x2d,
                0x12, 0x60, 0xff, 0xd5,
            ],
            Core::IsZero16 => [
                0x1b, 0xa7, 0x21, 0x3b, 0x58, 0x8b, 0xe0, 0x92, 0xb4, 0x46, 0x59, 0x9c, 0x2a, 0x60,
                0xff, 0x54, 0x67, 0x13, 0x6a, 0x79, 0x75, 0x99, 0x61, 0x0b, 0xd7, 0xa5, 0xf1, 0x78,
                0x04, 0xe3, 0x2a, 0x2c,
            ],
            Core::IsZero32 => [
                0x5e, 0xbf, 0x14, 0x66, 0x93, 0xf0, 0xe2, 0xd2, 0xf9, 0x36, 0x1b, 0x47, 0x6d, 0xba,
                0x34, 0x85, 0x8b, 0x83, 0x2d, 0x66, 0xfa, 0xcf, 0x71, 0x3b, 0xfb, 0x32, 0xc3, 0xbb,
                0x8d, 0xb9, 0xee, 0xbf,
            ],
            Core::IsZero64 => [
                0x19, 0xab, 0x9a, 0xc0, 0xcf, 0x42, 0x66, 0x82, 0x19, 0xba, 0x6c, 0xb8, 0x97, 0xe4,
                0x87, 0xfe, 0x36, 0x80, 0x93, 0x7f, 0xff, 0xa8, 0xd2, 0x03, 0x51, 0x1d, 0xb7, 0x5d,
                0xbb, 0x10, 0xc7, 0xe5,
            ],
            Core::IsZero8 => [
                0x8e, 0xff, 0x62, 0x08, 0x44, 0x07, 0xe9, 0xaf, 0xd5, 0x40, 0xf3, 0x18, 0xf6, 0x6b,
                0xcf, 0x31, 0xdf, 0x1d, 0x42, 0xa5, 0xc1, 0x61, 0xca, 0xe3, 0x5a, 0x29, 0x48, 0x18,
                0x0c, 0xa2, 0xaa, 0x2e,
            ],
            Core::Le16 => [
                0x01, 0x67, 0x05, 0xa7, 0xd7, 0xdc, 0xe1, 0xaf, 0xc6, 0x3e, 0xab, 0x84, 0x20, 0x3f,
                0x5f, 0x42, 0xd6, 0xb6, 0xbb, 0xad, 0x75, 0xce, 0xe3, 0x8c, 0xec, 0x5a, 0x51, 0x5b,
                0x59, 0x97, 0x48, 0x9f,
            ],
            Core::Le32 => [
                0x53, 0x51, 0xfc, 0x5d, 0xeb, 0xe5, 0xb2, 0x98, 0xad, 0x70, 0x57, 0xe4, 0xa5, 0xa7,
                0x6a, 0x3b, 0x9c, 0x65, 0x8a, 0xcd, 0xe7, 0xd1, 0xbb, 0x52, 0xe5, 0x88, 0x9c, 0xa1,
                0xe3, 0x8f, 0x5e, 0xfb,
            ],
            Core::Le64 => [
                0xae, 0x2d, 0xe1, 0xe0, 0xcf, 0x73, 0x0d, 0x1d, 0xcc, 0x96, 0xd7, 0xcc, 0xfe, 0x71,
                0x16, 0x8a, 0x24, 0x0d, 0xea, 0xf8, 0x04, 0x61, 0x5a, 0x7b, 0xa9, 0x20, 0xdc, 0x16,
                0xfd, 0x6e, 0xa4, 0x5f,
            ],
            Core::Le8 => [
                0xaf, 0x29, 0xf6, 0x16, 0x8e, 0xbd, 0xc0, 0x9e, 0xfb, 0xe0, 0xe6, 0x39, 0xcb, 0x75,
                0x0b, 0x12, 0x05, 0x78, 0x8f, 0x90, 0x21, 0xd6, 0x66, 0xef, 0xce, 0xfe, 0x13, 0xf1,
                0x2f, 0x96, 0x71, 0xf0,
            ],
            Core::LeftExtend16_32 => [
                0x28, 0x99, 0x97, 0xfb, 0xa1, 0xfa, 0xe7, 0xec, 0x1c, 0x45, 0x31, 0xc5, 0x0b, 0xbf,
                0x86, 0x71, 0xb8, 0x97, 0x13, 0x9b, 0xdd, 0x3a, 0xad, 0x97, 0xa3, 0x76, 0x39, 0x57,
                0x4a, 0x04, 0x7c, 0x80,
            ],
            Core::LeftExtend16_64 => [
                0x5d, 0xff, 0x21, 0xf6, 0xe6, 0x12, 0x47, 0x75, 0xc5, 0x78, 0xea, 0xf4, 0x85, 0x5c,
                0x0b, 0x01, 0x64, 0xf7, 0x87, 0x9b, 0x17, 0x60, 0xf9, 0x02, 0x7c, 0xb5, 0x0f, 0x7b,
                0x5a, 0xcb, 0x49, 0x18,
            ],
            Core::LeftExtend1_16 => [
                0x8c, 0x87, 0xd7, 0x56, 0xd1, 0x4b, 0xd3, 0xd9, 0xa7, 0x86, 0x90, 0x81, 0x29, 0x12,
                0xb8, 0x94, 0x29, 0xc0, 0x17, 0x1a, 0x41, 0x10, 0x3a, 0x58, 0xc6, 0xe9, 0xf2, 0x25,
                0x14, 0x1a, 0x02, 0x22,
            ],
            Core::LeftExtend1_32 => [
                0xc8, 0xf1, 0x54, 0xd4, 0x6d, 0x2e, 0x78, 0x95, 0xda, 0x1b, 0x33, 0xc2, 0xb3, 0x15,
                0xe6, 0xd4, 0xd4, 0x85, 0x1d, 0xde, 0xe2, 0x8a, 0xef, 0x8b, 0x70, 0x70, 0x90, 0x61,
                0x6b, 0xc7, 0xee, 0xa0,
            ],
            Core::LeftExtend1_64 => [
                0xa3, 0x40, 0x4d, 0xf6, 0x8c, 0xc9, 0x20, 0x75, 0x4c, 0x6e, 0x18, 0x47, 0x20, 0x7d,
                0xb3, 0x84, 0x5d, 0x11, 0xc7, 0x49, 0x09, 0xd0, 0x7c, 0xa8, 0x2a, 0xd1, 0xf1, 0xcc,
                0x67, 0xbf, 0x3a, 0x9b,
            ],
            Core::LeftExtend1_8 => [
                0x3b, 0xca, 0x33, 0x97, 0xb8, 0x3c, 0x27, 0xf3, 0x63, 0x16, 0xf8, 0xb8, 0xb3, 0x03,
                0x35, 0x0a, 0xfe, 0x8b, 0xa0, 0x07, 0x8f, 0x77, 0xf1, 0xd4, 0x2a, 0x9b, 0x78, 0x92,
                0xb2, 0xa4, 0xdb, 0xee,
            ],
            Core::LeftExtend32_64 => [
                0x42, 0xcb, 0xeb, 0x01, 0xfe, 0x7a, 0x3a, 0x6d, 0xd3, 0x31, 0x1d, 0xb3, 0x36, 0x5f,
                0x91, 0xe5, 0xc1, 0x18, 0xc7, 0xe4, 0x1f, 0x03, 0xaa, 0xe7, 0xb2, 0x83, 0xde, 0x6b,
                0xb9, 0x05, 0x3e, 0x6b,
            ],
            Core::LeftExtend8_16 => [
                0x9a, 0x57, 0xc9, 0x6a, 0xf5, 0x71, 0x48, 0x96, 0xb7, 0x24, 0xde, 0x45, 0xeb, 0x9f,
                0xe9, 0x7d, 0x73, 0x69, 0x7d, 0xe6, 0x2e, 0x8d, 0xad, 0x78, 0x71, 0xeb, 0x58, 0xf5,
                0x81, 0xa0, 0x11, 0xbb,
            ],
            Core::LeftExtend8_32 => [
                0xd6, 0x24, 0xbd, 0x40, 0x40, 0x76, 0x3c, 0xb1, 0x3c, 0xca, 0xd4, 0x98, 0xf5, 0x3d,
                0x38, 0xc1, 0x12, 0xf1, 0x92, 0x95, 0x68, 0x26, 0xda, 0xfe, 0xc9, 0xac, 0x91, 0x65,
                0x79, 0x2b, 0x34, 0x7a,
            ],
            Core::LeftExtend8_64 => [
                0x9d, 0xc4, 0xa2, 0x05, 0x4d, 0x5d, 0x26, 0x34, 0x2a, 0xc5, 0x90, 0xb6, 0x67, 0xf1,
                0xb0, 0x1d, 0xf5, 0x4f, 0xd0, 0xcd, 0xaa, 0x40, 0x5e, 0xf8, 0xcb, 0xb7, 0x6f, 0xd8,
                0xf9, 0xb0, 0x0e, 0xe5,
            ],
            Core::LeftPadHigh16_32 => [
                0x05, 0x45, 0xc4, 0xb5, 0x8f, 0x00, 0x4a, 0x21, 0xe7, 0xf1, 0x29, 0xa4, 0xc0, 0x51,
                0x89, 0x97, 0x17, 0x14, 0xca, 0xa2, 0xd9, 0x1d, 0x1d, 0xfd, 0x5f, 0xad, 0x3e, 0x63,
                0x24, 0x49, 0x94, 0x28,
            ],
            Core::LeftPadHigh16_64 => [
                0x1c, 0x61, 0xd0, 0x3d, 0x49, 0x3b, 0xbd, 0x05, 0x82, 0x22, 0x59, 0xd1, 0x73, 0x0a,
                0x8d, 0x7a, 0x5f, 0x55, 0xb0, 0xba, 0x2a, 0x93, 0x91, 0xa6, 0xc8, 0x88, 0x1e, 0xb4,
                0x75, 0x04, 0xaf, 0xfd,
            ],
            Core::LeftPadHigh1_16 => [
                0x56, 0xfd, 0xf5, 0x4f, 0x1f, 0xcd, 0x19, 0x82, 0x5e, 0x7c, 0x3b, 0x79, 0x06, 0x15,
                0xc1, 0xd3, 0xfe, 0x82, 0x88, 0x6c, 0x74, 0x7b, 0xc4, 0x87, 0x59, 0x87, 0xf5, 0x05,
                0x16, 0x94, 0x5f, 0xb3,
            ],
            Core::LeftPadHigh1_32 => [
                0xdb, 0x33, 0x05, 0x9a, 0xbe, 0x2d, 0x43, 0x2d, 0x67, 0xf4, 0x2b, 0x1e, 0x94, 0x27,
                0x56, 0xdc, 0xa6, 0xcd, 0xe6, 0x37, 0x85, 0xe5, 0xbd, 0x43, 0x0d, 0xc8, 0xf4, 0xae,
                0xfc, 0x31, 0xb8, 0xdf,
            ],
            Core::LeftPadHigh1_64 => [
                0x1d, 0x66, 0x9c, 0x1f, 0xa5, 0xfd, 0x3e, 0xf6, 0x6e, 0xb4, 0xae, 0xf6, 0x18, 0x6e,
                0x3e, 0xc1, 0x36, 0xee, 0x75, 0x84, 0x10, 0xdf, 0x3e, 0xde, 0xbb, 0x31, 0xbf, 0x26,
                0xd4, 0x56, 0x20, 0x51,
            ],
            Core::LeftPadHigh1_8 => [
                0x9a, 0x1b, 0xad, 0x3d, 0x8a, 0xb9, 0x00, 0x30, 0x3d, 0xa2, 0x02, 0xf0, 0xf4, 0x49,
                0xf0, 0xb7, 0xe6, 0x79, 0x5c, 0x2a, 0x7c, 0x12, 0x17, 0x18, 0x80, 0x0a, 0xc4, 0x0c,
                0x87, 0xd8, 0x27, 0x29,
            ],
            Core::LeftPadHigh32_64 => [
                0x39, 0x20, 0xcc, 0x4b, 0x33, 0xba, 0xf7, 0xef, 0xa5, 0xca, 0xf9, 0xe7, 0x80, 0x01,
                0x44, 0x67, 0x06, 0xf6, 0xe4, 0xe8, 0x26, 0x56, 0x74, 0x05, 0x7e, 0xed, 0x87, 0x17,
                0x78, 0x08, 0x9e, 0x94,
            ],
            Core::LeftPadHigh8_16 => [
                0x75, 0x2e, 0x29, 0xf2, 0xfe, 0x2b, 0xec, 0xc3, 0xf6, 0x62, 0x90, 0xfe, 0x44, 0xe1,
                0xae, 0xb3, 0x78, 0x41, 0x80, 0xdd, 0x90, 0x5e, 0x19, 0x62, 0x4e, 0x19, 0x5f, 0x21,
                0x6c, 0x07, 0xc5, 0x7c,
            ],
            Core::LeftPadHigh8_32 => [
                0xbe, 0xe8, 0x8f, 0x1c, 0x8c, 0x30, 0x63, 0x4c, 0x6e, 0x95, 0xca, 0xcc, 0x0e, 0x9a,
                0xdd, 0x49, 0x41, 0x32, 0x21, 0xfd, 0xab, 0xbd, 0x8d, 0x4c, 0x0a, 0xcc, 0xf1, 0xca,
                0xe2, 0xd2, 0xa7, 0x78,
            ],
            Core::LeftPadHigh8_64 => [
                0x39, 0x23, 0x87, 0xf6, 0xdc, 0x04, 0xbf, 0xc5, 0x4d, 0xd4, 0xa2, 0x81, 0x19, 0xc8,
                0x1d, 0x15, 0xd7, 0xa5, 0x80, 0x9b, 0xbf, 0x62, 0xfc, 0xc2, 0x7d, 0xc5, 0x5c, 0xf8,
                0x2e, 0x9e, 0x5e, 0xe6,
            ],
            Core::LeftPadLow16_32 => [
                0x4f, 0xfd, 0x6c, 0xb3, 0x40, 0x23, 0x05, 0x82, 0x1d, 0xd8, 0x99, 0x70, 0xd7, 0x22,
                0xd1, 0xc1, 0x3f, 0x1f, 0xf7, 0x73, 0x9f, 0xd5, 0xf3, 0x4b, 0xa1, 0x6c, 0x73, 0x65,
                0x3b, 0x04, 0x47, 0x18,
            ],
            Core::LeftPadLow16_64 => [
                0xbe, 0x3e, 0xb8, 0x5c, 0x5f, 0x19, 0x91, 0x53, 0xfb, 0x1c, 0x46, 0x13, 0x5c, 0x04,
                0xfa, 0xcf, 0xdb, 0xc6, 0xf1, 0xb7, 0x8c, 0x2b, 0xb7, 0xae, 0x75, 0xf1, 0x55, 0xbc,
                0x3e, 0xa0, 0x8a, 0x8b,
            ],
            Core::LeftPadLow1_16 => [
                0xdd, 0xd0, 0x15, 0x3e, 0xf3, 0x12, 0xf2, 0x8d, 0x64, 0x2c, 0xd9, 0x4c, 0xb3, 0x6f,
                0x32, 0x97, 0x75, 0xb0, 0x0d, 0xa8, 0x8f, 0xcc, 0xc4, 0xce, 0xa1, 0xba, 0xe8, 0x9b,
                0xad, 0x13, 0xbe, 0x6b,
            ],
            Core::LeftPadLow1_32 => [
                0xbc, 0x9d, 0x31, 0x14, 0x35, 0x46, 0x7b, 0xc0, 0x8b, 0x10, 0x08, 0xe5, 0x47, 0xaa,
                0x7a, 0x07, 0xe8, 0x3b, 0x15, 0x14, 0x68, 0x61, 0xa9, 0xe9, 0xb5, 0x41, 0x3b, 0xe3,
                0x1b, 0x82, 0xb6, 0xb5,
            ],
            Core::LeftPadLow1_64 => [
                0x8b, 0xc6, 0x2f, 0x93, 0x60, 0x89, 0x4e, 0x48, 0xa4, 0x73, 0x2c, 0x95, 0x76, 0x9c,
                0x8f, 0xaa, 0xe9, 0x56, 0x8f, 0x9d, 0xe8, 0xe8, 0xa2, 0x00, 0x83, 0x6b, 0xd4, 0xe5,
                0x0b, 0x02, 0xcd, 0x84,
            ],
            Core::LeftPadLow1_8 => [
                0xf6, 0x6c, 0xd7, 0xa4, 0x2b, 0x32, 0x0f, 0x97, 0xc1, 0x9f, 0x2d, 0x54, 0x16, 0xcd,
                0xe0, 0x87, 0x25, 0x3a, 0x27, 0x91, 0x29, 0x65, 0xd5, 0x5b, 0x65, 0x71, 0x2a, 0xd8,
                0x09, 0xb8, 0x3c, 0xfd,
            ],
            Core::LeftPadLow32_64 => [
                0xa3, 0x3a, 0x07, 0xb9, 0xbc, 0xf9, 0x45, 0xf6, 0x4f, 0x07, 0x2b, 0x8b, 0x9c, 0x91,
                0x48, 0x39, 0xa5, 0x85, 0xbf, 0xa9, 0xf3, 0x42, 0x5b, 0x14, 0x77, 0x54, 0xab, 0x55,
                0xa8, 0xba, 0x6c, 0x0f,
            ],
            Core::LeftPadLow8_16 => [
                0x2a, 0x51, 0x6a, 0x79, 0x3f, 0x97, 0xc4, 0x5f, 0xea, 0xeb, 0xb1, 0xcc, 0x96, 0x1a,
                0x15, 0x6d, 0x80, 0x35, 0x49, 0x28, 0x79, 0x78, 0x9d, 0x6e, 0xdc, 0x9b, 0x57, 0xe7,
                0x2f, 0x11, 0xe5, 0xb5,
            ],
            Core::LeftPadLow8_32 => [
                0x1a, 0xa2, 0xe4, 0xd0, 0x4b, 0xd6, 0x90, 0x55, 0x12, 0x3d, 0xd6, 0xaa, 0xfe, 0x27,
                0xf5, 0xf7, 0xf4, 0x7c, 0x3b, 0x30, 0x90, 0xc3, 0xa8, 0x27, 0x29, 0x73, 0xfe, 0x2f,
                0x75, 0x16, 0x5a, 0x5d,
            ],
            Core::LeftPadLow8_64 => [
                0xb6, 0x52, 0xe0, 0xae, 0xdd, 0x0f, 0x4f, 0x66, 0xf6, 0xa1, 0xcd, 0x4b, 0xeb, 0xf8,
                0x75, 0xff, 0x7b, 0xbb, 0x2d, 0xd9, 0x9b, 0x06, 0x5b, 0x2d, 0xb5, 0xb5, 0xb5, 0x90,
                0x53, 0x61, 0x61, 0x4d,
            ],
            Core::LeftRotate16 => [
                0x8a, 0x12, 0xff, 0x6a, 0x4b, 0xf2, 0x37, 0x15, 0xdd, 0x3b, 0x76, 0x6b, 0x99, 0x67,
                0xc7, 0x15, 0x8b, 0xf3, 0xed, 0x74, 0xb3, 0xdc, 0xe7, 0x30, 0xaf, 0xfc, 0xf4, 0x66,
                0x16, 0x47, 0x8e, 0xcb,
            ],
            Core::LeftRotate32 => [
                0x2f, 0xcb, 0x52, 0x17, 0x2f, 0xd4, 0x9c, 0x36, 0x21, 0x7d, 0xea, 0xe0, 0xc2, 0x37,
                0x14, 0x32, 0x1f, 0x69, 0xf5, 0xf1, 0x3f, 0x6e, 0x94, 0xb2, 0xbd, 0xfe, 0x4b, 0x74,
                0x88, 0x69, 0x7f, 0xd5,
            ],
            Core::LeftRotate64 => [
                0x72, 0xcc, 0xd6, 0xc4, 0xe5, 0xfd, 0xf6, 0x8a, 0xd3, 0x3b, 0x6d, 0x58, 0xfb, 0x37,
                0x2b, 0xe4, 0xf1, 0xb8, 0x0e, 0xef, 0x70, 0x1f, 0x9d, 0xb7, 0xe5, 0xed, 0x85, 0x9b,
                0x96, 0xb3, 0x62, 0x09,
            ],
            Core::LeftRotate8 => [
                0x1a, 0xae, 0xc9, 0xf3, 0xb7, 0x5d, 0x89, 0xf8, 0x2a, 0x64, 0x98, 0x45, 0x8c, 0x44,
                0x83, 0xcb, 0x9a, 0x78, 0x44, 0x89, 0x05, 0xf3, 0xbb, 0x39, 0xfc, 0x08, 0x3f, 0x14,
                0xdd, 0xcc, 0xdc, 0x9b,
            ],
            Core::LeftShift16 => [
                0x37, 0xac, 0x63, 0x87, 0x21, 0xab, 0x09, 0x7a, 0x96, 0x02, 0xba, 0x4d, 0xc9, 0x2e,
                0x19, 0xb5, 0xa1, 0x85, 0xb2, 0x32, 0x9f, 0x1a, 0xa6, 0x00, 0xcb, 0x9c, 0x15, 0x61,
                0x5a, 0x00, 0x81, 0xf8,
            ],
            Core::LeftShift32 => [
                0x8e, 0x3c, 0x47, 0x3b, 0x28, 0x67, 0xf1, 0x54, 0x73, 0xb3, 0x63, 0x2d, 0xbf, 0xdd,
                0x99, 0x77, 0x55, 0x51, 0xef, 0x5f, 0x9d, 0xba, 0x47, 0x5e, 0x9c, 0xf0, 0x90, 0x75,
                0x80, 0x70, 0xf0, 0xbf,
            ],
            Core::LeftShift64 => [
                0x50, 0x49, 0xf4, 0x04, 0xd1, 0x73, 0x29, 0x9a, 0x3a, 0xee, 0x04, 0xcb, 0xc2, 0x46,
                0x2c, 0xb3, 0x4c, 0x80, 0x69, 0xc1, 0xb6, 0xdb, 0x7f, 0xed, 0x0e, 0x38, 0x8f, 0xf6,
                0xd4, 0x67, 0xa0, 0x86,
            ],
            Core::LeftShift8 => [
                0x83, 0x2f, 0x63, 0x6e, 0x63, 0x44, 0x6c, 0xef, 0xba, 0x8d, 0xf3, 0xa4, 0x6e, 0xfb,
                0xb3, 0x61, 0x59, 0xc1, 0x88, 0x54, 0x56, 0x77, 0x68, 0xad, 0xc9, 0xb8, 0xdb, 0x8a,
                0x07, 0x49, 0x2a, 0x58,
            ],
            Core::LeftShiftWith16 => [
                0xe6, 0x47, 0x62, 0xb1, 0xc5, 0xe6, 0x14, 0x4a, 0x71, 0x81, 0xea, 0xaf, 0x4d, 0xd9,
                0xd9, 0xb3, 0xaa, 0x43, 0xaa, 0xd9, 0x55, 0x15, 0x81, 0x98, 0xee, 0x20, 0x90, 0xeb,
                0xd9, 0xe4, 0xbb, 0x0d,
            ],
            Core::LeftShiftWith32 => [
                0x64, 0x76, 0xba, 0x89, 0x95, 0xf8, 0x3b, 0x5e, 0xe1, 0xeb, 0xc2, 0x2c, 0xb4, 0x16,
                0xf5, 0x58, 0x15, 0x7f, 0x2e, 0x57, 0x69, 0x9a, 0x5c, 0xaf, 0x84, 0x29, 0x1f, 0xf3,
                0xfc, 0x14, 0x83, 0xc1,
            ],
            Core::LeftShiftWith64 => [
                0x06, 0xb8, 0xfe, 0x67, 0xcf, 0xc5, 0x86, 0x32, 0x23, 0x97, 0xaf, 0x02, 0x4f, 0xde,
                0x29, 0x11, 0xf7, 0xae, 0x87, 0xa0, 0x6a, 0xbc, 0x6c, 0x59, 0x30, 0x93, 0x40, 0x97,
                0x15, 0x69, 0x1c, 0x19,
            ],
            Core::LeftShiftWith8 => [
                0xb1, 0xac, 0x9c, 0x68, 0x23, 0x58, 0xc4, 0x5b, 0xab, 0xf4, 0x06, 0x95, 0x56, 0xfe,
                0x6e, 0x37, 0x5b, 0x45, 0x54, 0xde, 0x9e, 0x10, 0xc5, 0x91, 0xc1, 0x48, 0x39, 0x84,
                0x47, 0xac, 0x18, 0x0e,
            ],
            Core::Leftmost16_1 => [
                0x5b, 0xff, 0x4c, 0xb5, 0x58, 0x76, 0x05, 0xd5, 0xfd, 0x05, 0x9d, 0x77, 0x33, 0x49,
                0x0d, 0x7d, 0xd2, 0x2d, 0x27, 0x8b, 0x59, 0x9e, 0x06, 0xd3, 0xb5, 0xdb, 0x6d, 0x79,
                0xf3, 0xc9, 0x23, 0xbd,
            ],
            Core::Leftmost16_2 => [
                0x53, 0x6d, 0xb4, 0x86, 0xb1, 0x22, 0x27, 0xe5, 0xb0, 0x9d, 0x6f, 0xeb, 0xd2, 0x77,
                0x6b, 0x1a, 0xbb, 0xc6, 0x74, 0x99, 0x96, 0xaa, 0x78, 0x3e, 0xd7, 0xe5, 0x37, 0x44,
                0x6b, 0xbf, 0x15, 0x1b,
            ],
            Core::Leftmost16_4 => [
                0xf2, 0x32, 0x13, 0x67, 0x49, 0x6d, 0x1a, 0x77, 0xee, 0xa0, 0x5e, 0x95, 0xe3, 0xb8,
                0x07, 0xd3, 0xba, 0x5f, 0x05, 0x13, 0x6c, 0xe0, 0x91, 0x2a, 0xe7, 0x17, 0xc8, 0x3a,
                0x02, 0x61, 0xb2, 0xe1,
            ],
            Core::Leftmost16_8 => [
                0x24, 0x14, 0x8e, 0xf3, 0x0a, 0xd4, 0x3e, 0xbe, 0xc5, 0x63, 0x72, 0x83, 0x22, 0xc3,
                0xce, 0x11, 0x79, 0xae, 0xd7, 0xa7, 0x82, 0x16, 0xd7, 0x99, 0x88, 0x8b, 0xf1, 0x8b,
                0x39, 0x57, 0x06, 0x71,
            ],
            Core::Leftmost32_1 => [
                0xb9, 0x2e, 0x15, 0xec, 0x5d, 0xa0, 0x7e, 0xe8, 0xed, 0x39, 0x7c, 0xb9, 0xf6, 0x0a,
                0x4c, 0x5d, 0xa8, 0x38, 0x62, 0x93, 0x1a, 0x90, 0x73, 0x59, 0xd2, 0x7c, 0xae, 0xb6,
                0x0e, 0x60, 0xef, 0x8a,
            ],
            Core::Leftmost32_16 => [
                0xad, 0xb0, 0x27, 0xb2, 0x06, 0x56, 0x73, 0x58, 0x53, 0x26, 0xc0, 0x1c, 0x3b, 0xe2,
                0xfa, 0xeb, 0x38, 0x63, 0x49, 0xe2, 0x90, 0x09, 0xb6, 0x57, 0x6e, 0xe5, 0x3a, 0x85,
                0x55, 0x12, 0xcc, 0x67,
            ],
            Core::Leftmost32_2 => [
                0xb7, 0x5b, 0x31, 0xc5, 0x59, 0x12, 0x3d, 0x3d, 0x63, 0x35, 0x98, 0x59, 0x32, 0xb8,
                0xb1, 0xb2, 0x66, 0x4e, 0xe5, 0x97, 0xaf, 0xb1, 0x5f, 0xd1, 0xa4, 0x99, 0xd0, 0x07,
                0xcf, 0xf2, 0x75, 0x5c,
            ],
            Core::Leftmost32_4 => [
                0xcb, 0x75, 0x7e, 0x47, 0x1e, 0x9d, 0x9a, 0x40, 0x77, 0x1d, 0xd1, 0xcf, 0x3c, 0x1b,
                0xf5, 0xd2, 0x3c, 0x17, 0xed, 0x68, 0xcd, 0xbd, 0xb2, 0x2d, 0xad, 0xa1, 0x7a, 0x73,
                0xa7, 0xb4, 0x07, 0xb2,
            ],
            Core::Leftmost32_8 => [
                0xbf, 0xc5, 0x34, 0xb4, 0x9e, 0x06, 0x00, 0x6e, 0x19, 0xf3, 0xb6, 0x8e, 0x0a, 0x02,
                0x39, 0x1c, 0x14, 0x9f, 0x9a, 0x34, 0xf4, 0x3e, 0xe3, 0x6b, 0x9f, 0x1d, 0x79, 0xa7,
                0x9c, 0x9a, 0x9e, 0x4d,
            ],
            Core::Leftmost64_1 => [
                0x1b, 0x1d, 0x4e, 0x92, 0x38, 0x4b, 0x8b, 0x15, 0x9b, 0xa0, 0xd8, 0x06, 0x55, 0x8b,
                0x54, 0x94, 0xe3, 0x61, 0x4e, 0xed, 0xe0, 0x3c, 0x94, 0x6c, 0xea, 0xf1, 0x41, 0xf3,
                0x6f, 0x01, 0xc7, 0x9b,
            ],
            Core::Leftmost64_16 => [
                0x0d, 0xeb, 0xdc, 0x1a, 0xa0, 0x43, 0x30, 0x34, 0x42, 0xe1, 0x8f, 0xe0, 0x3d, 0x8a,
                0x99, 0xd2, 0xbe, 0x6b, 0xb8, 0xa8, 0x69, 0x1a, 0xba, 0x19, 0x56, 0x62, 0x59, 0xe3,
                0x67, 0x60, 0xf7, 0xf9,
            ],
            Core::Leftmost64_2 => [
                0x83, 0x9e, 0xcf, 0xa3, 0x18, 0x70, 0x5c, 0x25, 0x3d, 0x0c, 0x52, 0xff, 0x27, 0xb9,
                0x04, 0x64, 0x92, 0x3d, 0x8c, 0x0e, 0x55, 0xa8, 0x2c, 0x0d, 0x16, 0x24, 0x02, 0x39,
                0x7f, 0x36, 0x53, 0x78,
            ],
            Core::Leftmost64_32 => [
                0x92, 0x91, 0x97, 0xa9, 0x64, 0x28, 0x61, 0xa7, 0x7b, 0xd6, 0x62, 0x58, 0x05, 0x11,
                0x97, 0xbe, 0x86, 0xff, 0x08, 0xe6, 0x28, 0xe3, 0x0f, 0x7e, 0xfc, 0xbd, 0x2c, 0x4d,
                0xfe, 0xcf, 0x9b, 0xdd,
            ],
            Core::Leftmost64_4 => [
                0x02, 0xbd, 0x16, 0x45, 0xd5, 0x75, 0xf0, 0x4b, 0x3c, 0xbb, 0xaa, 0x6d, 0x8c, 0xa9,
                0x86, 0xef, 0x1c, 0x8c, 0xd0, 0xff, 0xe1, 0x65, 0x89, 0x03, 0x93, 0x9d, 0xb7, 0x64,
                0x56, 0x2a, 0x26, 0x47,
            ],
            Core::Leftmost64_8 => [
                0x35, 0x58, 0xb3, 0x1b, 0x3b, 0x6e, 0x8f, 0x9a, 0x28, 0x8f, 0xdc, 0x72, 0xf2, 0x46,
                0x02, 0xbe, 0x05, 0x58, 0x19, 0x10, 0x71, 0xa5, 0x4a, 0x99, 0xfa, 0x03, 0xa0, 0x25,
                0x34, 0xf8, 0x80, 0x05,
            ],
            Core::Leftmost8_1 => [
                0x28, 0x65, 0xef, 0xd4, 0x29, 0x83, 0xcb, 0xe3, 0xf8, 0x16, 0x37, 0x3a, 0xb8, 0xa8,
                0x82, 0xf1, 0x83, 0x17, 0x19, 0x4d, 0xc1, 0xab, 0xa3, 0x8d, 0xa0, 0x30, 0x4b, 0x8c,
                0x14, 0x4b, 0x1d, 0xa4,
            ],
            Core::Leftmost8_2 => [
                0x51, 0x96, 0x4c, 0xb0, 0x74, 0x05, 0xa8, 0xd2, 0x3d, 0x21, 0x87, 0x74, 0x1a, 0x9e,
                0xd3, 0x04, 0xbc, 0xb4, 0x69, 0xd9, 0xac, 0x9f, 0x5d, 0x92, 0x55, 0x82, 0x5c, 0xfd,
                0xa3, 0xda, 0x07, 0xc0,
            ],
            Core::Leftmost8_4 => [
                0x88, 0x3c, 0x94, 0xf8, 0xa2, 0x6c, 0xda, 0xb7, 0xbc, 0x5c, 0xd6, 0x31, 0xe5, 0x22,
                0x55, 0xa8, 0x5e, 0xf6, 0xe0, 0x70, 0x76, 0x64, 0x57, 0xf6, 0x32, 0x1e, 0x2c, 0xcb,
                0x11, 0x9d, 0x9b, 0x2b,
            ],
            Core::LinearCombination1 => [
                0x34, 0x10, 0xa9, 0xee, 0x33, 0x3d, 0xf8, 0xc8, 0xa0, 0x1c, 0x14, 0x11, 0x5b, 0x54,
                0x43, 0x27, 0xe3, 0x24, 0xe2, 0x87, 0xaa, 0x11, 0x07, 0xe0, 0x19, 0x55, 0xbd, 0x20,
                0x50, 0x6e, 0xa9, 0x87,
            ],
            Core::LinearVerify1 => [
                0xdc, 0x66, 0xd3, 0x31, 0xc1, 0x7f, 0x3f, 0xdd, 0xa3, 0x99, 0x46, 0x98, 0x1b, 0x39,
                0xb3, 0x57, 0xd0, 0x55, 0x5c, 0x35, 0x62, 0xec, 0xae, 0x02, 0xaa, 0x2d, 0xad, 0x16,
                0x3e, 0x6c, 0x9a, 0x2e,
            ],
            Core::Low1 => [
                0xfe, 0x62, 0x14, 0xf9, 0x67, 0x15, 0x6d, 0xcd, 0xe6, 0xdd, 0x49, 0xfd, 0xc5, 0x5e,
                0xfb, 0x86, 0x50, 0x69, 0xfe, 0xab, 0xff, 0xf0, 0xfe, 0x93, 0x1d, 0xba, 0x85, 0x31,
                0x34, 0xee, 0xd1, 0x30,
            ],
            Core::Low16 => [
                0x74, 0x93, 0xcf, 0x69, 0x8a, 0x48, 0x82, 0xe5, 0xc3, 0x57, 0x9d, 0x06, 0x51, 0x8e,
                0x7e, 0xca, 0x2b, 0x84, 0x28, 0xf6, 0x2e, 0x2b, 0x51, 0x38, 0x02, 0xab, 0xe6, 0x22,
                0x17, 0x0c, 0x20, 0xfe,
            ],
            Core::Low32 => [
                0x36, 0x2d, 0x66, 0xa4, 0xf0, 0xae, 0xb9, 0x65, 0x84, 0xa5, 0x67, 0x57, 0x82, 0x71,
                0xb1, 0xf7, 0xbb, 0xfc, 0xc2, 0xde, 0x0d, 0xcf, 0x95, 0x79, 0x6b, 0x6f, 0x7a, 0x82,
                0x6b, 0x2a, 0x8a, 0xf7,
            ],
            Core::Low64 => [
                0x97, 0x33, 0x23, 0xbc, 0x2b, 0x92, 0xe4, 0x28, 0x04, 0xd2, 0xe4, 0xf5, 0x8b, 0x86,
                0xf6, 0x5b, 0x56, 0xf9, 0x1d, 0xee, 0xb4, 0x81, 0x0e, 0xab, 0x8a, 0x1d, 0xed, 0xa9,
                0x69, 0x7a, 0x08, 0x72,
            ],
            Core::Low8 => [
                0xcd, 0x1a, 0x85, 0x58, 0xef, 0x99, 0xa3, 0x22, 0x60, 0x21, 0x7a, 0x76, 0x49, 0xff,
                0x51, 0x40, 0xda, 0x69, 0xda, 0x70, 0x06, 0x72, 0x69, 0x0b, 0x27, 0x91, 0x7b, 0x07,
                0xd7, 0xc1, 0x4c, 0x67,
            ],
            Core::Lt16 => [
                0x04, 0xac, 0xa8, 0x7e, 0x3e, 0x17, 0xf8, 0x05, 0xa2, 0x1c, 0xf2, 0x91, 0x7a, 0xee,
                0x99, 0x57, 0xb9, 0x50, 0xb2, 0xdb, 0x5d, 0x7a, 0xe5, 0xc8, 0x26, 0xd4, 0xac, 0x2e,
                0xc9, 0x7b, 0x5a, 0x52,
            ],
            Core::Lt32 => [
                0x23, 0xa0, 0xa5, 0xc1, 0x97, 0x74, 0x7e, 0x3a, 0x95, 0x79, 0xe9, 0x0e, 0x0f, 0x22,
                0xf8, 0x4a, 0x29, 0xbf, 0xb5, 0xf0, 0x7b, 0x84, 0xb5, 0x9b, 0x26, 0x68, 0x8a, 0x0c,
                0xd5, 0x9d, 0xfe, 0xbd,
            ],
            Core::Lt64 => [
                0xd2, 0x99, 0x90, 0x1c, 0x7b, 0x5b, 0x3a, 0x59, 0xff, 0xc8, 0xdd, 0x09, 0x54, 0x5a,
                0x32, 0x38, 0x24, 0xb7, 0x79, 0xa9, 0x9b, 0x2d, 0x1a, 0x2f, 0x87, 0x45, 0x2d, 0x9e,
                0x4b, 0xef, 0xaf, 0x30,
            ],
            Core::Lt8 => [
                0xdd, 0x94, 0x41, 0x3b, 0x52, 0x9c, 0x29, 0x8c, 0x16, 0x96, 0xe9, 0xfb, 0x08, 0xe6,
                0x67, 0x67, 0xb3, 0xf8, 0x33, 0x7a, 0xc0, 0x2e, 0x44, 0xb0, 0x68, 0xe9, 0x40, 0x14,
                0xf7, 0xc4, 0x1f, 0x2a,
            ],
            Core::Maj1 => [
                0x0e, 0x6f, 0xb4, 0x0f, 0xe3, 0x1a, 0x3a, 0x52, 0x6b, 0x44, 0xcf, 0x0b, 0x7c, 0x79,
                0x36, 0xc7, 0x77, 0xcb, 0xba, 0x89, 0x65, 0xa7, 0x25, 0x52, 0x32, 0xa7, 0xcf, 0x53,
                0xa9, 0x22, 0x88, 0x5a,
            ],
            Core::Maj16 => [
                0x38, 0x66, 0x9c, 0xe5, 0xe1, 0xe1, 0x71, 0x47, 0x54, 0x00, 0x73, 0x1b, 0xee, 0xb6,
                0x0b, 0xca, 0xfa, 0xd6, 0x66, 0x04, 0xc9, 0x39, 0x40, 0x16, 0x0c, 0xd7, 0x12, 0x88,
                0x35, 0x55, 0x93, 0x42,
            ],
            Core::Maj32 => [
                0x55, 0x54, 0x34, 0x9b, 0x58, 0x4f, 0x5c, 0x38, 0x72, 0xc7, 0xf4, 0xf2, 0x57, 0x82,
                0x9e, 0x2a, 0xe8, 0x22, 0xd8, 0x23, 0x42, 0x4c, 0xeb, 0x95, 0x98, 0xf0, 0x83, 0x18,
                0x58, 0x6a, 0x88, 0x07,
            ],
            Core::Maj64 => [
                0x73, 0x49, 0x03, 0xba, 0xef, 0xb7, 0x1d, 0x5e, 0xa4, 0x16, 0x48, 0xff, 0x43, 0xee,
                0xe6, 0x98, 0x94, 0xe0, 0x63, 0xb3, 0x88, 0xea, 0x42, 0x2f, 0x96, 0xae, 0xde, 0x19,
                0x3c, 0xea, 0xb8, 0x39,
            ],
            Core::Maj8 => [
                0xba, 0x47, 0xa3, 0x99, 0xdc, 0x94, 0x35, 0xe1, 0x8e, 0x08, 0x0a, 0x4e, 0x18, 0xaf,
                0x7c, 0x65, 0x7f, 0xd3, 0x9f, 0x7c, 0xe7, 0xd6, 0x05, 0x2e, 0x46, 0x90, 0x23, 0x11,
                0xb0, 0x78, 0xd5, 0x85,
            ],
            Core::Max16 => [
                0xaa, 0x55, 0x23, 0x74, 0x6c, 0xab, 0xfa, 0xf5, 0x66, 0x8e, 0x9e, 0x07, 0x37, 0xe5,
                0x6b, 0x06, 0x06, 0x22, 0x51, 0xd7, 0xe8, 0x0a, 0xb9, 0xb9, 0x10, 0x6d, 0x8f, 0x17,
                0x2d, 0xc8, 0x4d, 0xd6,
            ],
            Core::Max32 => [
                0x69, 0x22, 0x96, 0x5d, 0x14, 0x43, 0x45, 0xc9, 0x13, 0xec, 0xb3, 0x0b, 0x5e, 0xd4,
                0x7e, 0x88, 0xda, 0xe3, 0x5c, 0x12, 0x21, 0xf2, 0x6a, 0xa9, 0x2d, 0xd5, 0xa5, 0xf6,
                0x15, 0xdb, 0xdb, 0x53,
            ],
            Core::Max64 => [
                0x8a, 0x9b, 0xe9, 0x07, 0xb6, 0xa4, 0xc3, 0x0a, 0xbc, 0xc0, 0xf2, 0x2d, 0x01, 0x30,
                0x74, 0xc2, 0xd5, 0x6b, 0xb0, 0x81, 0xf2, 0x62, 0x18, 0x57, 0xd5, 0x38, 0xcc, 0x97,
                0x13, 0x1e, 0x44, 0x09,
            ],
            Core::Max8 => [
                0xb4, 0xbf, 0x93, 0x23, 0x40, 0x22, 0xe8, 0x60, 0xfe, 0x76, 0xc0, 0xb5, 0x36, 0x0e,
                0x8b, 0x36, 0xff, 0x81, 0xee, 0x67, 0x05, 0xb5, 0x93, 0xac, 0xdf, 0x65, 0x5a, 0xc6,
                0xe6, 0xd7, 0xae, 0xba,
            ],
            Core::Median16 => [
                0x17, 0xe2, 0xe8, 0x7f, 0x07, 0x60, 0xf4, 0xfb, 0x3c, 0x9f, 0xd0, 0xbe, 0xd0, 0x00,
                0xd7, 0x39, 0x73, 0xab, 0x60, 0xf5, 0xe6, 0xc2, 0xc1, 0xfa, 0xb1, 0x7f, 0x9b, 0x23,
                0xee, 0x6a, 0xca, 0x48,
            ],
            Core::Median32 => [
                0x11, 0x60, 0xae, 0x8e, 0xa8, 0xd3, 0x0f, 0x9a, 0x22, 0x33, 0xc4, 0x8e, 0x73, 0x12,
                0x40, 0xf8, 0x44, 0x93, 0xb8, 0x28, 0xb5, 0x57, 0x93, 0xe2, 0xf4, 0x04, 0x2a, 0x19,
                0x82, 0xac, 0x26, 0xa5,
            ],
            Core::Median64 => [
                0xc8, 0x73, 0x73, 0x64, 0x9e, 0x7e, 0x40, 0x50, 0xbb, 0x73, 0x33, 0x7e, 0x08, 0xeb,
                0x5d, 0xe4, 0x52, 0x28, 0xab, 0x86, 0xad, 0x4e, 0x1f, 0x41, 0x91, 0xe5, 0x20, 0x2a,
                0xa6, 0xaf, 0xa0, 0xc5,
            ],
            Core::Median8 => [
                0xc3, 0xb4, 0xe0, 0x89, 0x8a, 0x21, 0xbd, 0xe9, 0x4d, 0xae, 0xd3, 0x7a, 0x20, 0xad,
                0xf9, 0x0c, 0x8b, 0xe5, 0x69, 0x1a, 0x03, 0xb6, 0xa1, 0xe5, 0x56, 0x38, 0x5d, 0x42,
                0xeb, 0x19, 0x02, 0x2b,
            ],
            Core::Min16 => [
                0x5f, 0xd0, 0x05, 0x1e, 0xdb, 0x37, 0x19, 0xa6, 0x45, 0xb2, 0x72, 0xa0, 0x21, 0x08,
                0xef, 0xbb, 0x3d, 0x9b, 0xc0, 0xf6, 0x06, 0x21, 0xbf, 0x5a, 0x5b, 0xab, 0xe1, 0x16,
                0xd5, 0x55, 0xd5, 0x78,
            ],
            Core::Min32 => [
                0xd8, 0x07, 0x82, 0xa2, 0xb5, 0xd8, 0x6a, 0xb6, 0xb9, 0xc9, 0xc3, 0xfb, 0x77, 0x8a,
                0x34, 0x73, 0xf6, 0x00, 0xb1, 0x85, 0xfe, 0x19, 0x25, 0xee, 0x9f, 0xc2, 0xe8, 0x77,
                0x7e, 0xd2, 0x66, 0x01,
            ],
            Core::Min64 => [
                0xc5, 0xc0, 0x9d, 0x50, 0x13, 0x38, 0xe9, 0xa5, 0x12, 0xcf, 0x89, 0x76, 0xca, 0x4b,
                0x32, 0xb9, 0x24, 0x80, 0xbe, 0xf6, 0xae, 0xb2, 0x9d, 0x36, 0xd5, 0x90, 0xd3, 0x5b,
                0xf9, 0xf9, 0xec, 0xe1,
            ],
            Core::Min8 => [
                0x81, 0xd2, 0x1e, 0x12, 0x81, 0x42, 0x38, 0x81, 0x80, 0x2c, 0x0e, 0x0c, 0x7d, 0x22,
                0xbd, 0x34, 0xd2, 0x6b, 0xd1, 0x2a, 0x4c, 0x4f, 0x1b, 0x70, 0x68, 0xe7, 0xe1, 0x83,
                0x82, 0x08, 0x48, 0xe9,
            ],
            Core::Modulo16 => [
                0xb6, 0xb8, 0x7c, 0xfa, 0xb6, 0x7e, 0x55, 0x19, 0xf1, 0xc9, 0x98, 0xda, 0x47, 0x94,
                0x37, 0xbb, 0x79, 0xe6, 0x74, 0xf7, 0x15, 0xe9, 0xa2, 0xe5, 0x38, 0xee, 0xc5, 0xec,
                0x18, 0xe1, 0x8e, 0xa5,
            ],
            Core::Modulo32 => [
                0x8d, 0x48, 0x6e, 0x83, 0x16, 0x54, 0xf3, 0x8a, 0x32, 0xda, 0x35, 0xeb, 0x7b, 0xb6,
                0x55, 0xa6, 0xed, 0x69, 0x4d, 0xbf, 0xa0, 0x58, 0x95, 0x7d, 0x9f, 0x5c, 0xbf, 0xcc,
                0x57, 0x92, 0xc6, 0x5b,
            ],
            Core::Modulo64 => [
                0x14, 0xdf, 0x20, 0xd9, 0x3d, 0xfd, 0xef, 0xe2, 0x55, 0x9b, 0xac, 0x50, 0xed, 0x38,
                0x19, 0x3b, 0xd7, 0x8b, 0xd6, 0x3f, 0x92, 0x9d, 0x86, 0xfb, 0x4f, 0x29, 0xa7, 0xc5,
                0xaf, 0x32, 0x42, 0xad,
            ],
            Core::Modulo8 => [
                0x2c, 0x75, 0x8a, 0x7c, 0x0f, 0x59, 0xe8, 0x00, 0xe9, 0x4f, 0x3d, 0xc5, 0xa0, 0x01,
                0xbf, 0x8e, 0xd9, 0x43, 0x5f, 0x75, 0xa2, 0xd9, 0x69, 0x30, 0xc5, 0x7e, 0xaa, 0xb0,
                0xcd, 0x80, 0xaf, 0x5c,
            ],
            Core::Multiply16 => [
                0x75, 0xbd, 0x41, 0xf2, 0xd2, 0xb3, 0x39, 0xf0, 0x69, 0xbf, 0xdf, 0xd8, 0x02, 0xd6,
                0x1e, 0x6c, 0xa8, 0xe3, 0xba, 0xd6, 0xfb, 0x6d, 0x95, 0xb6, 0x72, 0x09, 0x5b, 0x93,
                0x34, 0x5f, 0x04, 0x7f,
            ],
            Core::Multiply32 => [
                0x84, 0xcb, 0xe6, 0xce, 0x87, 0x03, 0x79, 0x92, 0x13, 0x87, 0x7c, 0x1b, 0xd5, 0x05,
                0xc7, 0x64, 0x34, 0x33, 0x69, 0x00, 0x2e, 0x50, 0x2c, 0x43, 0xd9, 0x7f, 0x3d, 0x57,
                0x77, 0x2d, 0x6c, 0x87,
            ],
            Core::Multiply64 => [
                0x92, 0x98, 0x7b, 0x80, 0x1b, 0x92, 0xf6, 0x79, 0xeb, 0x96, 0x13, 0x68, 0x84, 0x44,
                0xa1, 0x78, 0x87, 0x50, 0xa8, 0x50, 0x6e, 0x03, 0xa9, 0x21, 0x8c, 0x21, 0xec, 0xc7,
                0x20, 0x82, 0xdc, 0x6a,
            ],
            Core::Multiply8 => [
                0x76, 0x4c, 0xab, 0x71, 0xdb, 0x94, 0x59, 0xa7, 0x69, 0x6d, 0x94, 0x4a, 0x50, 0x09,
                0x5b, 0x1a, 0xeb, 0xdf, 0xd9, 0x28, 0x4b, 0xdb, 0x74, 0x96, 0xa7, 0xb3, 0x02, 0x41,
                0xcc, 0xba, 0x3e, 0xce,
            ],
            Core::Negate16 => [
                0xe7, 0x60, 0xee, 0x40, 0x29, 0xc3, 0x4f, 0x89, 0x74, 0x06, 0xff, 0xde, 0xa5, 0x55,
                0x84, 0x86, 0x62, 0xe8, 0x9c, 0x98, 0x3e, 0x60, 0x70, 0xbd, 0x02, 0x72, 0xad, 0x0f,
                0xa3, 0x42, 0xef, 0xa3,
            ],
            Core::Negate32 => [
                0x84, 0x95, 0xb7, 0x40, 0x09, 0xad, 0x07, 0xc9, 0x30, 0x2a, 0x25, 0xae, 0x56, 0xc3,
                0xe9, 0x73, 0x3f, 0x00, 0xc2, 0xba, 0xa4, 0x10, 0xea, 0xc4, 0xa5, 0x8e, 0x75, 0xdb,
                0x83, 0xaf, 0x1d, 0x22,
            ],
            Core::Negate64 => [
                0x34, 0xe8, 0x9f, 0xaf, 0x34, 0x5a, 0xfd, 0x5e, 0x7b, 0x29, 0x00, 0x14, 0x52, 0xfc,
                0x5f, 0xc2, 0xe3, 0x78, 0x3a, 0xf7, 0xf2, 0x10, 0x16, 0x43, 0xbd, 0x76, 0x70, 0x6a,
                0x6f, 0xc3, 0xf3, 0x6a,
            ],
            Core::Negate8 => [
                0xe8, 0x1b, 0xe0, 0xb1, 0x5c, 0x67, 0x1a, 0xb8, 0xdf, 0x1f, 0x48, 0x69, 0xc5, 0x7f,
                0x11, 0x11, 0x18, 0xcb, 0x66, 0x83, 0x54, 0x97, 0x5c, 0x63, 0x66, 0xec, 0xb2, 0xb8,
                0xbb, 0x7c, 0x15, 0xcf,
            ],
            Core::One16 => [
                0x2e, 0x5e, 0x3d, 0x95, 0xe4, 0x53, 0x16, 0x88, 0x8e, 0x4f, 0x37, 0x09, 0xef, 0x83,
                0x2b, 0x9f, 0xd9, 0xe1, 0x5f, 0x30, 0x71, 0x9b, 0xf5, 0x5f, 0xc2, 0xe0, 0xe0, 0x9a,
                0x36, 0x57, 0xd8, 0x82,
            ],
            Core::One32 => [
                0x06, 0x42, 0x6b, 0x85, 0x3c, 0x1b, 0xcb, 0x33, 0x8a, 0xed, 0xbe, 0x1f, 0x89, 0xa6,
                0xd9, 0xb7, 0xa3, 0xda, 0x03, 0x8c, 0xd0, 0x0a, 0x44, 0x71, 0x18, 0x36, 0x93, 0x49,
                0x66, 0x9e, 0x29, 0x76,
            ],
            Core::One64 => [
                0xab, 0x1d, 0x2c, 0xd9, 0x96, 0x78, 0xda, 0x3c, 0x12, 0x8d, 0x39, 0xad, 0x9f, 0xe6,
                0xff, 0xa9, 0x55, 0xc1, 0x6e, 0x5e, 0xf2, 0xc2, 0x5b, 0xb4, 0x31, 0x83, 0x15, 0x59,
                0x69, 0x51, 0xf4, 0x27,
            ],
            Core::One8 => [
                0x3c, 0xc5, 0xf5, 0x23, 0xd6, 0xa6, 0x35, 0x5d, 0xc9, 0x24, 0xee, 0x0a, 0xc1, 0xf5,
                0xfe, 0x2c, 0x52, 0x12, 0x75, 0xe3, 0xaa, 0x9f, 0x21, 0xd3, 0x1b, 0x08, 0x2d, 0xb2,
                0xac, 0x23, 0x0d, 0x9d,
            ],
            Core::Or1 => [
                0xc4, 0x65, 0x96, 0x43, 0x69, 0xfc, 0xa2, 0x09, 0x7f, 0x83, 0x53, 0x0c, 0x87, 0xbc,
                0xbc, 0x90, 0xc3, 0x06, 0x57, 0x9d, 0x9f, 0x3b, 0xfe, 0xdd, 0xf4, 0xa1, 0x72, 0xa4,
                0xea, 0x0b, 0x58, 0xec,
            ],
            Core::Or16 => [
                0x5a, 0x98, 0x5e, 0x04, 0x3b, 0x85, 0x27, 0x3b, 0x90, 0xf9, 0x0e, 0x20, 0xf8, 0x2b,
                0x75, 0x32, 0x33, 0x51, 0xcf, 0x2a, 0x4e, 0x62, 0xa7, 0xf9, 0xcb, 0x2f, 0x05, 0x96,
                0x40, 0x2e, 0x9e, 0x28,
            ],
            Core::Or32 => [
                0x35, 0x52, 0x38, 0x3a, 0x57, 0xff, 0xb4, 0x8d, 0x63, 0xa0, 0x33, 0x7a, 0xf0, 0xdd,
                0x6e, 0xfa, 0xb6, 0xb4, 0x6c, 0x5d, 0xe1, 0x72, 0x0e, 0x42, 0x0b, 0xdd, 0x1c, 0x82,
                0x27, 0x6b, 0xc9, 0xa9,
            ],
            Core::Or64 => [
                0x51, 0xa1, 0x73, 0xda, 0xdc, 0xa0, 0x1a, 0xc6, 0xf6, 0x2e, 0x75, 0xd5, 0xcd, 0x35,
                0x22, 0xf0, 0x9f, 0xde, 0x62, 0xb1, 0x15, 0x13, 0xe0, 0x68, 0x42, 0x28, 0x52, 0xa4,
                0x91, 0x67, 0xb6, 0x06,
            ],
            Core::Or8 => [
                0x79, 0xef, 0xbd, 0xcb, 0x53, 0x7b, 0xeb, 0xcb, 0x18, 0x8d, 0x11, 0x16, 0xb7, 0x8a,
                0x10, 0x9b, 0xff, 0xbc, 0x2a, 0x6c, 0xe3, 0xd1, 0xf8, 0x70, 0x15, 0x4a, 0x79, 0x56,
                0x09, 0x1b, 0x34, 0x2f,
            ],
            Core::ParseLock => [
                0x3d, 0xb8, 0x45, 0x35, 0xfa, 0x3d, 0x90, 0xef, 0x0b, 0x58, 0x1e, 0x22, 0xb6, 0x1d,
                0x21, 0x27, 0x84, 0x4b, 0x21, 0x16, 0xe8, 0x4f, 0x81, 0x4a, 0x5c, 0xba, 0xc5, 0x2d,
                0xf5, 0x15, 0xf2, 0xd2,
            ],
            Core::ParseSequence => [
                0x38, 0xb2, 0x53, 0x3f, 0x5f, 0xed, 0xe8, 0x69, 0xba, 0xa1, 0x70, 0x69, 0x83, 0xdf,
                0x4c, 0x89, 0xd6, 0x2d, 0x5f, 0x90, 0x80, 0x0b, 0x47, 0xea, 0xb2, 0x11, 0x13, 0x31,
                0x1a, 0x5a, 0xae, 0xc9,
            ],
            Core::PointVerify1 => [
                0xbe, 0x2a, 0x98, 0x90, 0xf1, 0xd5, 0xb6, 0x15, 0x14, 0x7f, 0x82, 0x41, 0xe0, 0x60,
                0x9b, 0x5c, 0xac, 0x01, 0xec, 0xe0, 0xa3, 0xf9, 0x23, 0x68, 0x67, 0xb2, 0xbf, 0xde,
                0xa1, 0xb8, 0x04, 0x4e,
            ],
            Core::RightExtend16_32 => [
                0xdb, 0xf1, 0x8d, 0x87, 0xa7, 0x89, 0x21, 0x39, 0xa3, 0x88, 0xe9, 0xa9, 0x83, 0xc4,
                0x89, 0x92, 0xac, 0x35, 0xa8, 0x45, 0x56, 0xee, 0x0d, 0xef, 0xc1, 0xda, 0xdf, 0x0c,
                0x5f, 0x47, 0x1a, 0x26,
            ],
            Core::RightExtend16_64 => [
                0xd0, 0x11, 0xac, 0xc7, 0x94, 0xe3, 0xc4, 0x78, 0x9a, 0xcc, 0xd0, 0xd5, 0xfe, 0x49,
                0x97, 0xd3, 0x34, 0xd9, 0x1f, 0x08, 0x31, 0xa1, 0xeb, 0x35, 0x04, 0xb4, 0xcb, 0x2d,
                0xdf, 0x47, 0x97, 0xaf,
            ],
            Core::RightExtend32_64 => [
                0xa5, 0xaa, 0x5d, 0xb1, 0xe5, 0x35, 0xe7, 0x23, 0x2a, 0xd3, 0x6d, 0xaf, 0xba, 0x6d,
                0x5a, 0x20, 0x0d, 0x54, 0xeb, 0x85, 0x3b, 0x75, 0xdc, 0x70, 0xa5, 0x94, 0xed, 0x64,
                0xaa, 0x6b, 0xd9, 0xab,
            ],
            Core::RightExtend8_16 => [
                0x81, 0x06, 0xd5, 0x8a, 0x80, 0x66, 0xee, 0x6e, 0x15, 0xe5, 0x5c, 0xa5, 0x2c, 0xb7,
                0xaf, 0xd8, 0xe3, 0x27, 0x75, 0x87, 0xbf, 0xd7, 0xde, 0xc0, 0xbe, 0x37, 0xd4, 0x06,
                0x74, 0x2a, 0x39, 0x31,
            ],
            Core::RightExtend8_32 => [
                0xdf, 0xa4, 0xba, 0xfa, 0x43, 0x2a, 0x53, 0x38, 0xd3, 0x74, 0xde, 0xb6, 0xb7, 0x24,
                0xb7, 0xf6, 0xea, 0xe5, 0x58, 0x61, 0xfe, 0x73, 0x1d, 0x43, 0x04, 0x8a, 0xa3, 0x04,
                0xd1, 0xf7, 0xf9, 0xa2,
            ],
            Core::RightExtend8_64 => [
                0x62, 0x0a, 0x37, 0x03, 0x8b, 0x6f, 0xa1, 0x27, 0x49, 0x5f, 0x0b, 0x46, 0x49, 0x6f,
                0x64, 0x35, 0xdd, 0x2d, 0xad, 0x7e, 0xf0, 0xc0, 0xfd, 0x2c, 0xd6, 0x5f, 0x54, 0xdc,
                0x18, 0x5e, 0x99, 0x7b,
            ],
            Core::RightPadHigh16_32 => [
                0x2b, 0x6a, 0xbc, 0x38, 0x32, 0x1a, 0x7c, 0x54, 0x2f, 0xb1, 0x69, 0x74, 0x62, 0x1c,
                0xed, 0x80, 0x88, 0x0d, 0xb5, 0x19, 0xbb, 0x48, 0x60, 0x93, 0x42, 0x6e, 0x8c, 0xe1,
                0x8e, 0x01, 0x69, 0xb1,
            ],
            Core::RightPadHigh16_64 => [
                0xad, 0x90, 0xd8, 0xff, 0xa5, 0x74, 0x50, 0xb3, 0xb5, 0xe9, 0x09, 0x62, 0x25, 0x34,
                0x9e, 0xd8, 0xf0, 0x72, 0xe1, 0x01, 0x72, 0x93, 0xf3, 0x92, 0xef, 0x85, 0x4e, 0x03,
                0x19, 0xab, 0xc9, 0x34,
            ],
            Core::RightPadHigh1_16 => [
                0x28, 0x81, 0x58, 0xb1, 0xc9, 0x10, 0x87, 0x7b, 0x7e, 0xea, 0x3d, 0xfc, 0xf2, 0xb2,
                0xb7, 0x88, 0x92, 0x28, 0x08, 0xb6, 0xd6, 0xfa, 0x75, 0xf8, 0x96, 0x77, 0x19, 0x04,
                0x8b, 0x14, 0x12, 0x49,
            ],
            Core::RightPadHigh1_32 => [
                0xee, 0x2a, 0xd7, 0x7f, 0x66, 0x8d, 0x3d, 0x6a, 0x2e, 0x68, 0x50, 0x6e, 0x49, 0x04,
                0xcf, 0x50, 0xa0, 0x84, 0x60, 0xe1, 0xd2, 0xb8, 0x6a, 0x81, 0xe1, 0x4e, 0x41, 0xf8,
                0xda, 0x4c, 0xdd, 0xf2,
            ],
            Core::RightPadHigh1_64 => [
                0x3d, 0x6a, 0x7f, 0xe6, 0x9a, 0x11, 0x64, 0x2a, 0xce, 0xd6, 0x84, 0x2b, 0x89, 0xaa,
                0x1b, 0xb8, 0x41, 0x3e, 0x39, 0x90, 0x63, 0xcc, 0x16, 0x78, 0x6a, 0xf7, 0xc0, 0x33,
                0xda, 0xd5, 0x8b, 0x95,
            ],
            Core::RightPadHigh1_8 => [
                0x28, 0x44, 0xbd, 0xfd, 0x6a, 0xba, 0x29, 0xdf, 0x03, 0xf9, 0x3a, 0xa6, 0xae, 0xb2,
                0x1c, 0x06, 0x40, 0x28, 0xdb, 0x05, 0xff, 0x77, 0xd8, 0xd9, 0x1c, 0xfd, 0xcd, 0xef,
                0xb1, 0x90, 0xc5, 0xbd,
            ],
            Core::RightPadHigh32_64 => [
                0xb4, 0x32, 0xe5, 0x32, 0x1a, 0xe1, 0x71, 0x4c, 0xe1, 0x95, 0x29, 0xd8, 0x5f, 0x24,
                0xff, 0x89, 0x87, 0x91, 0x0e, 0xbc, 0xf0, 0x15, 0xf8, 0x7f, 0x15, 0xbb, 0xed, 0x55,
                0xf0, 0xa0, 0xe8, 0x92,
            ],
            Core::RightPadHigh8_16 => [
                0x6f, 0x2d, 0x96, 0xc9, 0x54, 0x13, 0xca, 0x9a, 0xa8, 0xcc, 0x55, 0x0f, 0x25, 0x73,
                0xe1, 0x66, 0x99, 0x56, 0xd6, 0x07, 0x69, 0x2c, 0xf1, 0xca, 0x6d, 0xc7, 0x6d, 0x2f,
                0x2b, 0x4a, 0x3a, 0xc8,
            ],
            Core::RightPadHigh8_32 => [
                0xdf, 0x2c, 0x7f, 0x92, 0x99, 0x00, 0xa4, 0x49, 0x01, 0xe6, 0xff, 0x65, 0x27, 0x6a,
                0x95, 0x1a, 0xeb, 0x95, 0xdf, 0x25, 0x0b, 0x13, 0x97, 0x14, 0xd4, 0x19, 0x54, 0x04,
                0xd7, 0x78, 0x98, 0xed,
            ],
            Core::RightPadHigh8_64 => [
                0x79, 0xc0, 0x1d, 0xa3, 0xe6, 0x0b, 0x9c, 0x69, 0x35, 0xce, 0x3e, 0x15, 0x98, 0xb1,
                0x78, 0x40, 0xaf, 0x82, 0xdc, 0xb0, 0xdd, 0xc6, 0x3a, 0xef, 0x4a, 0x06, 0xe7, 0xf9,
                0xca, 0x5d, 0x27, 0x41,
            ],
            Core::RightPadLow16_32 => [
                0x6f, 0x20, 0x10, 0x27, 0xcc, 0x75, 0x98, 0x02, 0x30, 0xa0, 0x70, 0x85, 0x9c, 0x3e,
                0x38, 0x02, 0x36, 0xa1, 0xcb, 0x10, 0xe6, 0x1a, 0x01, 0xaa, 0x1f, 0x6d, 0x23, 0x1d,
                0x15, 0x14, 0x2f, 0x25,
            ],
            Core::RightPadLow16_64 => [
                0xb8, 0x6e, 0x1f, 0x0b, 0xfe, 0xc6, 0x55, 0x98, 0xd0, 0xa3, 0xd1, 0xec, 0x96, 0x03,
                0x05, 0xb9, 0x67, 0x45, 0x67, 0x3e, 0x1b, 0x16, 0xbf, 0x32, 0x7a, 0x71, 0x68, 0x05,
                0x83, 0xd7, 0x1d, 0x90,
            ],
            Core::RightPadLow1_16 => [
                0x05, 0x2a, 0x64, 0x99, 0xc9, 0x3e, 0xe6, 0xbc, 0x1a, 0xe6, 0x57, 0xf8, 0x5f, 0xd4,
                0xd4, 0xfe, 0x67, 0x7a, 0xbc, 0xee, 0x54, 0x0d, 0x13, 0x40, 0x33, 0x54, 0x2e, 0x9a,
                0xb6, 0x0a, 0x63, 0xdd,
            ],
            Core::RightPadLow1_32 => [
                0x5b, 0x70, 0xd4, 0x28, 0x96, 0x0e, 0x95, 0xcc, 0x40, 0xd5, 0x18, 0x46, 0xf5, 0x3a,
                0x4d, 0x0a, 0x35, 0xc9, 0x01, 0x5d, 0x15, 0x00, 0xb6, 0xbc, 0x84, 0x9b, 0x72, 0x83,
                0x5e, 0x2b, 0xd4, 0x40,
            ],
            Core::RightPadLow1_64 => [
                0x44, 0xef, 0xeb, 0x87, 0xca, 0x2a, 0xd7, 0xfd, 0x4b, 0x73, 0xf1, 0x63, 0x07, 0xc7,
                0xf0, 0x59, 0x02, 0x65, 0x6f, 0x35, 0x09, 0x0f, 0xb0, 0xa4, 0x32, 0x6c, 0x64, 0x89,
                0x88, 0xae, 0x1d, 0x39,
            ],
            Core::RightPadLow1_8 => [
                0x93, 0x40, 0x39, 0x8b, 0xcc, 0x8e, 0xa8, 0x3e, 0xc8, 0x40, 0xbe, 0x72, 0x9d, 0xbb,
                0x8b, 0x81, 0x20, 0x78, 0x24, 0xee, 0x87, 0x5d, 0x15, 0x82, 0x59, 0xd6, 0xda, 0xd2,
                0x0a, 0x83, 0x93, 0x0c,
            ],
            Core::RightPadLow32_64 => [
                0x69, 0x3e, 0x28, 0x10, 0x1e, 0x04, 0xfd, 0xa4, 0x3b, 0x97, 0xe6, 0x11, 0xf0, 0xfe,
                0x98, 0x00, 0x0e, 0x14, 0x30, 0x2e, 0x5d, 0xcd, 0x6e, 0xd6, 0x5e, 0xee, 0x42, 0xe3,
                0x40, 0x14, 0x24, 0x2f,
            ],
            Core::RightPadLow8_16 => [
                0x09, 0x6b, 0x25, 0xc3, 0xc8, 0x41, 0x5f, 0x04, 0xd8, 0x83, 0x27, 0x43, 0xeb, 0x2f,
                0x84, 0x56, 0xd5, 0xf0, 0xa6, 0x44, 0x91, 0x3d, 0x3e, 0xc5, 0x9d, 0x34, 0xf4, 0x55,
                0x25, 0x01, 0xfa, 0x20,
            ],
            Core::RightPadLow8_32 => [
                0xfc, 0x7f, 0x57, 0x22, 0xa6, 0x2a, 0xa2, 0x20, 0x18, 0xcc, 0x81, 0xcd, 0x00, 0xa9,
                0x32, 0x6c, 0x7f, 0xe9, 0xc6, 0x3a, 0xbc, 0xe2, 0xbd, 0xa4, 0xc0, 0xe6, 0x6a, 0x3f,
                0x47, 0xc6, 0x7c, 0x53,
            ],
            Core::RightPadLow8_64 => [
                0xa5, 0xbb, 0x7d, 0x5e, 0xfc, 0xa0, 0xe4, 0x8d, 0x9d, 0x80, 0xc5, 0x02, 0x71, 0x15,
                0xb4, 0x85, 0x78, 0x10, 0x51, 0xe0, 0xef, 0x46, 0xe4, 0xd6, 0x08, 0x31, 0x7a, 0x1c,
                0x42, 0x61, 0xbc, 0x46,
            ],
            Core::RightRotate16 => [
                0x48, 0x2e, 0xa7, 0xe1, 0x21, 0x45, 0x01, 0xd9, 0x3c, 0x9a, 0xd1, 0x6f, 0xa8, 0xb9,
                0x7b, 0xf5, 0xb3, 0x84, 0xfc, 0x2b, 0x54, 0x78, 0x9b, 0x8c, 0xd9, 0xe7, 0x84, 0xcc,
                0xd0, 0xeb, 0x9d, 0x57,
            ],
            Core::RightRotate32 => [
                0x09, 0x41, 0xb6, 0xee, 0xea, 0x9a, 0xf8, 0x19, 0x5b, 0x02, 0x8a, 0xfc, 0x0b, 0xd2,
                0xa5, 0x34, 0x21, 0x8b, 0xf9, 0x0d, 0x1a, 0x0e, 0x37, 0x3d, 0x74, 0x74, 0x18, 0x54,
                0x0b, 0x72, 0x6d, 0x73,
            ],
            Core::RightRotate64 => [
                0x44, 0x4d, 0xbb, 0xc3, 0xdd, 0x2a, 0x11, 0xa5, 0xc7, 0xb0, 0x43, 0x9f, 0xdb, 0xa9,
                0x9a, 0xc7, 0x4a, 0x11, 0xb8, 0xee, 0xb2, 0xdb, 0x30, 0x1e, 0x24, 0x3e, 0xa8, 0x91,
                0x22, 0x90, 0x71, 0x52,
            ],
            Core::RightRotate8 => [
                0x72, 0x65, 0xa3, 0x0c, 0x2e, 0x83, 0x6e, 0x65, 0x54, 0x4a, 0xba, 0x91, 0x1b, 0x64,
                0xd1, 0x8f, 0xa6, 0x9b, 0x17, 0x65, 0x45, 0x85, 0x6c, 0x77, 0xc4, 0xf0, 0xd7, 0x6f,
                0xc3, 0xf5, 0x83, 0x51,
            ],
            Core::RightShift16 => [
                0xcd, 0x57, 0xa3, 0xd3, 0xab, 0x2d, 0x92, 0xd4, 0xf0, 0x86, 0x55, 0x04, 0x3a, 0x8b,
                0x8b, 0xb6, 0x73, 0x89, 0x81, 0xfa, 0xe6, 0xda, 0x01, 0x34, 0xb4, 0xde, 0xda, 0xce,
                0x5f, 0x00, 0x88, 0x60,
            ],
            Core::RightShift32 => [
                0xd6, 0xb3, 0x26, 0xb1, 0xa3, 0x23, 0x57, 0xa3, 0x32, 0x80, 0x7d, 0x3f, 0xa1, 0xb1,
                0x56, 0xc2, 0x8b, 0x16, 0x22, 0xf7, 0x38, 0xde, 0xf1, 0x26, 0x81, 0x46, 0x7f, 0x34,
                0x9b, 0xd3, 0x49, 0x4b,
            ],
            Core::RightShift64 => [
                0xb2, 0x09, 0x5f, 0x2d, 0x47, 0x33, 0x5d, 0x5f, 0x98, 0xc8, 0x54, 0x34, 0xa2, 0xfa,
                0xf5, 0xb0, 0xf7, 0x5c, 0xf8, 0x99, 0x01, 0x2a, 0x34, 0xbb, 0xcd, 0x0a, 0x14, 0xcb,
                0xed, 0xb6, 0x11, 0x07,
            ],
            Core::RightShift8 => [
                0x4b, 0x2b, 0x1a, 0xa2, 0xef, 0x73, 0x21, 0x73, 0x17, 0x0d, 0x62, 0x1a, 0x38, 0xde,
                0xb2, 0x61, 0xe4, 0x73, 0xc0, 0x7c, 0x55, 0x8b, 0x05, 0x5a, 0x25, 0xa8, 0x6e, 0x4e,
                0x32, 0x1a, 0xfc, 0x04,
            ],
            Core::RightShiftWith16 => [
                0x14, 0xb7, 0x76, 0x85, 0x47, 0xb3, 0xd3, 0xf4, 0x7e, 0xe5, 0xc2, 0xb8, 0x0d, 0x9b,
                0xda, 0xe2, 0xae, 0xc1, 0xf9, 0xc6, 0x59, 0x4e, 0xd3, 0x12, 0x7b, 0x12, 0x64, 0x5a,
                0xdc, 0xf5, 0x97, 0x54,
            ],
            Core::RightShiftWith32 => [
                0x32, 0x7b, 0x6e, 0x98, 0xa6, 0xfd, 0x34, 0x0c, 0x60, 0xcf, 0x83, 0xaa, 0x64, 0x99,
                0x33, 0x11, 0x4c, 0xb8, 0xd8, 0x4f, 0x59, 0x0e, 0x01, 0x21, 0x3a, 0x26, 0x10, 0x01,
                0x2b, 0x46, 0x07, 0xea,
            ],
            Core::RightShiftWith64 => [
                0x06, 0x2f, 0xa7, 0x4a, 0xf3, 0x47, 0x6e, 0x59, 0x38, 0x7b, 0xe0, 0x8e, 0x69, 0x49,
                0xa0, 0x05, 0x43, 0xbc, 0x84, 0xa2, 0xb6, 0x89, 0xea, 0x39, 0xad, 0x6e, 0xed, 0x7f,
                0x75, 0x67, 0x85, 0xd4,
            ],
            Core::RightShiftWith8 => [
                0x14, 0x1b, 0xe4, 0x7e, 0x96, 0x7b, 0x2f, 0xd7, 0xc7, 0x12, 0x6c, 0x5a, 0xdf, 0x2d,
                0xfe, 0x47, 0x31, 0x5b, 0xbc, 0x10, 0x53, 0xbb, 0xe6, 0x05, 0xb3, 0x88, 0x98, 0xdb,
                0xed, 0x49, 0xf2, 0x27,
            ],
            Core::Rightmost16_1 => [
                0x3f, 0x3c, 0x43, 0x46, 0x87, 0x17, 0x42, 0x26, 0x5e, 0x87, 0xf0, 0x01, 0xb4, 0x6d,
                0xe7, 0xd1, 0x98, 0x75, 0x1b, 0x34, 0xfa, 0xa1, 0x80, 0x18, 0xde, 0x60, 0xc8, 0x46,
                0x8d, 0x9b, 0x98, 0xa4,
            ],
            Core::Rightmost16_2 => [
                0xc1, 0x8b, 0x9f, 0xdd, 0x34, 0x0a, 0x26, 0x7a, 0xc1, 0x6d, 0x4f, 0x39, 0xee, 0x75,
                0x43, 0x56, 0x52, 0xaa, 0xca, 0x52, 0x56, 0x50, 0xb5, 0x1a, 0x45, 0x87, 0x98, 0x04,
                0x8e, 0x62, 0x7d, 0x51,
            ],
            Core::Rightmost16_4 => [
                0xc6, 0xc5, 0x3f, 0xa7, 0x1e, 0x23, 0x0c, 0xf0, 0x58, 0x51, 0x58, 0xf4, 0x70, 0x58,
                0x8b, 0xac, 0x5c, 0x51, 0x8f, 0x84, 0xf9, 0xfc, 0x23, 0x86, 0x52, 0xf1, 0x75, 0xfb,
                0x6e, 0xa1, 0x8c, 0x11,
            ],
            Core::Rightmost16_8 => [
                0xee, 0x76, 0x9c, 0x1c, 0xc8, 0xa3, 0xfd, 0xd1, 0x83, 0x8f, 0xc9, 0xf0, 0x49, 0x0c,
                0xe7, 0x03, 0x93, 0xfd, 0x91, 0xba, 0x3c, 0xbd, 0x4a, 0xbd, 0x08, 0x64, 0x9f, 0xb9,
                0xc4, 0x43, 0x11, 0xbd,
            ],
            Core::Rightmost32_1 => [
                0x1c, 0x44, 0x23, 0x69, 0xfb, 0x81, 0xf6, 0x11, 0xd3, 0x28, 0x01, 0x0b, 0x86, 0x4b,
                0xcc, 0xb7, 0xf3, 0x5e, 0xd4, 0x77, 0xdf, 0xa3, 0x85, 0x55, 0x74, 0xc1, 0x35, 0x64,
                0xcd, 0xbd, 0xb8, 0x60,
            ],
            Core::Rightmost32_16 => [
                0xad, 0xd2, 0xc3, 0x39, 0x0d, 0x9a, 0xf7, 0xc2, 0x4a, 0x15, 0x9a, 0x37, 0xd6, 0x9d,
                0x44, 0x84, 0xd2, 0xc2, 0x4a, 0x2c, 0xb5, 0xb0, 0xeb, 0x2d, 0x3c, 0x49, 0x3d, 0x98,
                0x12, 0xac, 0xfd, 0x74,
            ],
            Core::Rightmost32_2 => [
                0x00, 0xb8, 0x81, 0x5a, 0xd7, 0x42, 0x3d, 0xd5, 0x8c, 0xb9, 0x8b, 0xe8, 0x2c, 0xad,
                0x26, 0x67, 0x5c, 0x3b, 0xf5, 0x4a, 0x0b, 0xed, 0xba, 0xde, 0x34, 0x64, 0xb4, 0xfe,
                0x5a, 0x4e, 0x8c, 0xe6,
            ],
            Core::Rightmost32_4 => [
                0x84, 0xfa, 0x5a, 0x54, 0xf7, 0x72, 0x9f, 0x9d, 0x68, 0x99, 0x4b, 0xea, 0xb9, 0x3a,
                0xe7, 0x9b, 0x8c, 0x4a, 0x10, 0xd5, 0xb7, 0xae, 0x97, 0x27, 0xaa, 0x17, 0x16, 0xe5,
                0x7d, 0x03, 0x3b, 0x74,
            ],
            Core::Rightmost32_8 => [
                0x7d, 0x38, 0x05, 0xd3, 0xc7, 0x8c, 0x4e, 0xea, 0x91, 0xe3, 0xd3, 0x5e, 0xfd, 0xd4,
                0x7e, 0xed, 0xd4, 0x21, 0xaf, 0x84, 0xd2, 0x19, 0x10, 0x32, 0x93, 0x32, 0xa0, 0xb5,
                0x48, 0x7f, 0xab, 0x63,
            ],
            Core::Rightmost64_1 => [
                0xd3, 0xb1, 0x64, 0xc5, 0xdc, 0x66, 0xcc, 0x7e, 0xf9, 0x23, 0x4f, 0xed, 0xe4, 0xdc,
                0x7f, 0x0d, 0xa5, 0xcd, 0x71, 0xc1, 0xc1, 0xd4, 0xca, 0xd6, 0x0f, 0xb4, 0xec, 0x57,
                0x3e, 0x2b, 0x8a, 0x75,
            ],
            Core::Rightmost64_16 => [
                0xea, 0xe4, 0x34, 0x78, 0xf9, 0xf2, 0xf4, 0x52, 0xef, 0xac, 0x15, 0xee, 0xe6, 0x0f,
                0x8b, 0x52, 0x53, 0xd8, 0x0a, 0x2d, 0x32, 0x12, 0x9b, 0x4e, 0x5b, 0xa3, 0x83, 0x00,
                0xad, 0x98, 0x52, 0xfd,
            ],
            Core::Rightmost64_2 => [
                0x9c, 0xd4, 0xa9, 0x8b, 0xbd, 0xb8, 0xa3, 0x35, 0x85, 0xc0, 0x0f, 0x47, 0xd6, 0xad,
                0xab, 0x7a, 0xf5, 0x42, 0x86, 0xfb, 0x8a, 0xe6, 0x0f, 0x72, 0x30, 0x11, 0xfb, 0x84,
                0xc0, 0xee, 0x78, 0xf9,
            ],
            Core::Rightmost64_32 => [
                0x7f, 0x24, 0x20, 0xae, 0x5b, 0x0f, 0x5a, 0x3f, 0x6f, 0x2e, 0x60, 0xb6, 0x1f, 0x8a,
                0x41, 0x5c, 0x08, 0x8b, 0x94, 0xb2, 0x1c, 0x1a, 0x62, 0xa3, 0xfd, 0xaa, 0xc7, 0x49,
                0xdb, 0xdf, 0x4c, 0x71,
            ],
            Core::Rightmost64_4 => [
                0xe2, 0x65, 0x55, 0x2a, 0x24, 0xfb, 0xcd, 0xec, 0x05, 0x83, 0xd7, 0x18, 0x3e, 0x48,
                0xeb, 0xc2, 0xff, 0x6d, 0x31, 0x65, 0x57, 0xba, 0xc5, 0x91, 0x5c, 0x03, 0xcb, 0x23,
                0x35, 0xd2, 0x32, 0x95,
            ],
            Core::Rightmost64_8 => [
                0x98, 0xcd, 0x95, 0xf9, 0x5d, 0x46, 0x64, 0x1b, 0x04, 0x9e, 0x77, 0xbf, 0x90, 0xee,
                0xa5, 0x98, 0xad, 0xf2, 0x9e, 0xe5, 0x00, 0xe6, 0x50, 0x72, 0x87, 0x54, 0x8b, 0xb1,
                0xcd, 0xaf, 0x78, 0x4d,
            ],
            Core::Rightmost8_1 => [
                0x08, 0x76, 0xfc, 0xd4, 0x69, 0x85, 0x91, 0xf3, 0x31, 0x91, 0x01, 0x57, 0x4c, 0xe1,
                0x53, 0xfc, 0xdf, 0xe9, 0x4f, 0x58, 0x1a, 0xac, 0x5e, 0x75, 0xf3, 0xcd, 0x74, 0x46,
                0xdf, 0x56, 0xf3, 0xc7,
            ],
            Core::Rightmost8_2 => [
                0xb9, 0xf7, 0xb2, 0x90, 0xaf, 0xe7, 0xf1, 0x89, 0xe3, 0x2a, 0xeb, 0xf2, 0xcc, 0x4d,
                0xdc, 0xa9, 0x6b, 0xb0, 0x07, 0x64, 0xc7, 0xbe, 0x28, 0x87, 0xdc, 0xe0, 0x54, 0xd0,
                0x9e, 0x38, 0xc3, 0x53,
            ],
            Core::Rightmost8_4 => [
                0xf2, 0x8e, 0x9a, 0xf5, 0xaf, 0x4c, 0x9c, 0xca, 0x4b, 0x43, 0xcc, 0x6a, 0xdf, 0x9d,
                0x9d, 0x8d, 0x16, 0x9c, 0x87, 0xc5, 0x55, 0x9f, 0x9f, 0x3c, 0xca, 0xc8, 0xf2, 0x35,
                0x2b, 0x62, 0x9f, 0x18,
            ],
            Core::ScalarAdd => [
                0x11, 0xdd, 0xbe, 0xba, 0xeb, 0xf4, 0x21, 0x80, 0xa0, 0xb7, 0xed, 0xdf, 0xfd, 0xc4,
                0x8e, 0xc7, 0x51, 0x13, 0x30, 0xfb, 0x33, 0x15, 0xfa, 0x65, 0xd5, 0x8a, 0xff, 0x66,
                0xb9, 0xca, 0xf2, 0xd4,
            ],
            Core::ScalarInvert => [
                0xa6, 0x39, 0x27, 0x25, 0xbb, 0x2d, 0xad, 0xbb, 0x1e, 0x76, 0xdf, 0x2d, 0xec, 0x57,
                0xdf, 0x55, 0xc3, 0xfc, 0xc5, 0x77, 0x3b, 0x62, 0x21, 0x8a, 0xec, 0x55, 0xa7, 0x5e,
                0x14, 0xf3, 0xd6, 0x0d,
            ],
            Core::ScalarIsZero => [
                0xf7, 0x5e, 0xda, 0x06, 0xce, 0x6a, 0xf0, 0x9f, 0xae, 0x37, 0xdb, 0x4e, 0x62, 0x25,
                0xe6, 0xa8, 0xac, 0x86, 0xa2, 0x36, 0x37, 0x62, 0x7d, 0x62, 0x64, 0x09, 0x19, 0x0f,
                0xf3, 0xb3, 0x9d, 0x90,
            ],
            Core::ScalarMultiply => [
                0x4a, 0x61, 0x67, 0x2a, 0xce, 0xc4, 0x88, 0x77, 0x56, 0xde, 0x1d, 0xb6, 0x04, 0x21,
                0xa1, 0x2b, 0x90, 0x1a, 0x85, 0x8a, 0x6e, 0xe6, 0x35, 0x2e, 0x55, 0x9d, 0x4c, 0xe5,
                0x97, 0x33, 0x52, 0xbe,
            ],
            Core::ScalarMultiplyLambda => [
                0x49, 0xea, 0x9c, 0x3f, 0xb1, 0xd8, 0xff, 0x52, 0xd2, 0xdb, 0x03, 0x46, 0x9f, 0xdf,
                0xe8, 0x50, 0x50, 0x3f, 0xdd, 0xeb, 0x45, 0xe1, 0x6d, 0x26, 0xe8, 0x92, 0x8a, 0xdd,
                0x25, 0x87, 0x0e, 0x91,
            ],
            Core::ScalarNegate => [
                0x1d, 0xbf, 0x8b, 0x49, 0x1e, 0xc6, 0x65, 0x80, 0x3f, 0x63, 0x33, 0x30, 0xd3, 0xff,
                0xb0, 0xe7, 0x81, 0xe6, 0x7c, 0x18, 0x01, 0xac, 0x9d, 0x49, 0xbb, 0xf4, 0x35, 0x89,
                0xab, 0xf7, 0x82, 0xbf,
            ],
            Core::ScalarNormalize => [
                0x46, 0x33, 0x18, 0x0e, 0xa0, 0x2c, 0x4d, 0xf7, 0x81, 0x9d, 0x3d, 0x54, 0xa4, 0x01,
                0x73, 0x4f, 0x96, 0x5b, 0x31, 0xac, 0xc7, 0x84, 0x05, 0x4e, 0xbf, 0xb7, 0x31, 0x68,
                0x16, 0xb0, 0x29, 0xec,
            ],
            Core::ScalarSquare => [
                0x8a, 0x27, 0x9e, 0x6f, 0x61, 0x3a, 0xa9, 0xe9, 0x34, 0xf2, 0xf2, 0xa3, 0x43, 0xc0,
                0xd3, 0x29, 0x1c, 0x36, 0x70, 0xe2, 0x97, 0xdd, 0xae, 0x20, 0x52, 0x9e, 0x82, 0x50,
                0x69, 0xef, 0xea, 0x0e,
            ],
            Core::Scale => [
                0x12, 0x6e, 0x22, 0x12, 0x5b, 0xac, 0x80, 0xb9, 0x9b, 0x7b, 0x73, 0x43, 0xb4, 0xe5,
                0xe5, 0x86, 0x60, 0x82, 0x16, 0x10, 0x5d, 0x4d, 0xe6, 0xf7, 0x94, 0xad, 0xd3, 0x4e,
                0x23, 0xb1, 0x95, 0xca,
            ],
            Core::Sha256Block => [
                0x45, 0x35, 0xf3, 0xe1, 0xab, 0x9f, 0x1b, 0x75, 0x7a, 0x06, 0x91, 0x37, 0xe1, 0xd5,
                0xb1, 0xca, 0xad, 0x8e, 0x31, 0xf7, 0x8d, 0xc5, 0xfb, 0xd0, 0x73, 0x46, 0x49, 0xf9,
                0x40, 0xa7, 0xfc, 0x96,
            ],
            Core::Sha256Ctx8Add1 => [
                0x9a, 0x47, 0x11, 0xb8, 0xc5, 0x69, 0x0e, 0x58, 0x7e, 0x5f, 0x79, 0xe6, 0x8d, 0x6e,
                0xca, 0x04, 0x74, 0x58, 0xaa, 0x63, 0xb8, 0xbc, 0x9e, 0xe5, 0x68, 0x08, 0x6a, 0x4a,
                0x1b, 0x56, 0xd8, 0x34,
            ],
            Core::Sha256Ctx8Add128 => [
                0x1c, 0xb1, 0xdb, 0x8a, 0x05, 0x5b, 0x31, 0x97, 0xac, 0xf0, 0xf0, 0x8c, 0xe9, 0xc6,
                0x35, 0xad, 0xd6, 0x95, 0xb6, 0x0f, 0x23, 0x4b, 0x18, 0xe0, 0xb3, 0x23, 0xc9, 0x37,
                0xb0, 0x38, 0x5a, 0xea,
            ],
            Core::Sha256Ctx8Add16 => [
                0xe0, 0x84, 0x54, 0x75, 0xeb, 0xb9, 0x01, 0x40, 0xfa, 0x4e, 0x01, 0xaf, 0x8a, 0x94,
                0x35, 0x99, 0x1a, 0xd8, 0x7a, 0xf9, 0x8c, 0x08, 0xae, 0xce, 0x11, 0x0e, 0x99, 0xcb,
                0xce, 0xcd, 0xee, 0x79,
            ],
            Core::Sha256Ctx8Add2 => [
                0x7d, 0x69, 0x13, 0x8f, 0x1c, 0x94, 0x2b, 0xee, 0x2f, 0xdf, 0x60, 0x0c, 0xe4, 0x4b,
                0x36, 0xff, 0x97, 0x83, 0x9d, 0xc2, 0xbb, 0xda, 0xfb, 0xd5, 0xfa, 0xb4, 0xdf, 0xbc,
                0x3c, 0x97, 0x6f, 0x29,
            ],
            Core::Sha256Ctx8Add256 => [
                0x4f, 0x5c, 0x29, 0xd5, 0x36, 0x86, 0xc0, 0x60, 0x62, 0xb3, 0x83, 0x24, 0xf8, 0xaf,
                0xf1, 0x7e, 0xc5, 0x56, 0xa2, 0x95, 0xff, 0x09, 0x8b, 0x10, 0xe7, 0x05, 0xdd, 0x22,
                0xe1, 0x3b, 0xc3, 0xc9,
            ],
            Core::Sha256Ctx8Add32 => [
                0xd5, 0x7b, 0x67, 0xb1, 0x74, 0xe7, 0x8e, 0x38, 0xf9, 0xbc, 0xa8, 0xe0, 0x7a, 0xdd,
                0x61, 0xc7, 0x53, 0xe2, 0xc1, 0x56, 0xd8, 0xe9, 0x83, 0x2a, 0xa6, 0x62, 0x04, 0x55,
                0x00, 0xf5, 0x1a, 0x80,
            ],
            Core::Sha256Ctx8Add4 => [
                0x95, 0xda, 0x32, 0x99, 0x3f, 0x5c, 0x7d, 0x00, 0x83, 0x06, 0x4c, 0xdf, 0xf1, 0xbe,
                0xc3, 0xb9, 0x36, 0xc6, 0x38, 0x33, 0x7a, 0xde, 0xc5, 0x47, 0x48, 0x7a, 0xf2, 0x32,
                0xd6, 0x9f, 0xdf, 0x65,
            ],
            Core::Sha256Ctx8Add512 => [
                0x4a, 0xcb, 0x16, 0x3a, 0xa4, 0x8f, 0x09, 0xd5, 0xf2, 0x6d, 0x2b, 0x2a, 0xb1, 0x88,
                0xa6, 0xc6, 0xb6, 0xc4, 0xae, 0xdf, 0x23, 0xc9, 0x19, 0x00, 0x1c, 0x02, 0xee, 0x15,
                0xb3, 0x37, 0xa9, 0x6e,
            ],
            Core::Sha256Ctx8Add64 => [
                0x52, 0xe5, 0x3e, 0xc5, 0x77, 0x0f, 0x9b, 0xe4, 0x06, 0x9a, 0xee, 0xfc, 0xb2, 0x13,
                0x22, 0xb1, 0x3a, 0xb6, 0xe3, 0x94, 0x1f, 0xdc, 0x2c, 0x85, 0xf4, 0xb4, 0x1b, 0xe6,
                0x7d, 0x38, 0xea, 0x7e,
            ],
            Core::Sha256Ctx8Add8 => [
                0xc2, 0x6b, 0x28, 0xaf, 0xe5, 0xe8, 0x66, 0xd8, 0x46, 0x16, 0x81, 0x4d, 0x1a, 0x13,
                0xfb, 0x86, 0x30, 0xb9, 0xe8, 0x4e, 0x5d, 0x78, 0x15, 0x56, 0xc6, 0xd8, 0x23, 0x6e,
                0xfb, 0x45, 0xdf, 0xf9,
            ],
            Core::Sha256Ctx8AddBuffer511 => [
                0xad, 0x69, 0x90, 0x46, 0x48, 0xa8, 0x23, 0x8d, 0x00, 0xd8, 0x51, 0x63, 0xfc, 0xe8,
                0x19, 0x63, 0xa0, 0x04, 0x7a, 0xb5, 0x82, 0xbe, 0x97, 0xa4, 0x14, 0x00, 0x65, 0x59,
                0x79, 0xcf, 0xdd, 0x28,
            ],
            Core::Sha256Ctx8Finalize => [
                0x8e, 0x45, 0xbd, 0xc3, 0x87, 0xd4, 0xed, 0xfa, 0x73, 0x35, 0x25, 0xf3, 0xab, 0x19,
                0xe4, 0x2b, 0x58, 0xec, 0xb1, 0xb5, 0xf6, 0xdc, 0xcf, 0x94, 0xed, 0xbf, 0x59, 0x95,
                0x8a, 0xe3, 0xe1, 0x16,
            ],
            Core::Sha256Ctx8Init => [
                0x63, 0x5f, 0x64, 0x05, 0x84, 0x86, 0x85, 0xc0, 0x11, 0xfe, 0xbd, 0x41, 0xfa, 0xac,
                0x87, 0x4b, 0xbb, 0xf5, 0xb2, 0x4d, 0x5f, 0xb1, 0x2f, 0xed, 0xbc, 0xb6, 0xcb, 0xff,
                0x95, 0xa0, 0xf3, 0x66,
            ],
            Core::Sha256Iv => [
                0x12, 0xe4, 0x59, 0x37, 0x51, 0xc9, 0x46, 0x3b, 0x56, 0x25, 0x03, 0xc1, 0x40, 0xd7,
                0x8b, 0x3b, 0x75, 0x7a, 0x1f, 0x4f, 0x16, 0x32, 0x1d, 0x28, 0x62, 0xd3, 0x25, 0x43,
                0x85, 0x38, 0x97, 0x1b,
            ],
            Core::Some1 => [
                0x15, 0xca, 0x4e, 0x4b, 0x82, 0xc2, 0xf9, 0x1b, 0x9a, 0x79, 0x29, 0x92, 0xcd, 0xc1,
                0xb2, 0x92, 0xab, 0x86, 0xa2, 0xd2, 0x93, 0x9c, 0x9a, 0x64, 0xb5, 0x0b, 0xe6, 0x0b,
                0xda, 0x6a, 0xb4, 0xca,
            ],
            Core::Some16 => [
                0xa9, 0xdf, 0xbb, 0xea, 0xb5, 0x9d, 0xf7, 0x2a, 0x45, 0xfc, 0x3f, 0xc7, 0xac, 0x58,
                0x1e, 0xc8, 0xda, 0x71, 0x3f, 0x2f, 0x81, 0x03, 0xf7, 0x87, 0xaa, 0x1c, 0xee, 0x4e,
                0x0b, 0xa6, 0x48, 0x66,
            ],
            Core::Some32 => [
                0x46, 0x33, 0xa3, 0x97, 0x74, 0x2e, 0xf4, 0x82, 0xbe, 0x2f, 0xa3, 0xfb, 0x64, 0x10,
                0xec, 0x79, 0xc3, 0x73, 0x83, 0x65, 0x69, 0xfb, 0xbc, 0xb1, 0xf9, 0x48, 0xec, 0x32,
                0x48, 0x73, 0x78, 0xb7,
            ],
            Core::Some64 => [
                0x1d, 0xc2, 0x45, 0xac, 0x6f, 0x5b, 0x42, 0x2b, 0xd1, 0x88, 0x6e, 0xf5, 0x14, 0x4c,
                0x4d, 0xc7, 0x2c, 0x96, 0x73, 0x15, 0x59, 0x66, 0x07, 0x6c, 0xd8, 0x39, 0x68, 0x1d,
                0x9e, 0xc7, 0xf8, 0xf5,
            ],
            Core::Some8 => [
                0x33, 0xaf, 0xb9, 0xc6, 0x45, 0x4e, 0x59, 0x0e, 0xc1, 0x3e, 0xd7, 0x5e, 0x1b, 0x7d,
                0x9c, 0x3a, 0x3d, 0xe6, 0x75, 0x2b, 0xcc, 0x7c, 0x1d, 0x4c, 0xb3, 0x63, 0xfa, 0x51,
                0x82, 0x8b, 0xcb, 0x74,
            ],
            Core::Subtract16 => [
                0x4e, 0x06, 0xec, 0x31, 0x37, 0x62, 0x22, 0xe2, 0x5e, 0x27, 0xd0, 0x15, 0x9d, 0xc1,
                0xc0, 0x71, 0x4a, 0x44, 0xca, 0x6a, 0xac, 0xf9, 0x50, 0x5c, 0xaa, 0xd2, 0x80, 0xe9,
                0x73, 0xfb, 0x5c, 0xab,
            ],
            Core::Subtract32 => [
                0xb9, 0xc0, 0xf3, 0x6e, 0x75, 0x22, 0xa8, 0xd9, 0x49, 0x05, 0x0d, 0x51, 0x6a, 0x05,
                0xce, 0x20, 0x3a, 0x1f, 0x9a, 0x9e, 0x37, 0x2f, 0xd2, 0x63, 0xde, 0x38, 0xb0, 0xe9,
                0x03, 0x13, 0x41, 0x98,
            ],
            Core::Subtract64 => [
                0x1c, 0xdb, 0x5c, 0x74, 0xad, 0xd1, 0x02, 0xf5, 0x0f, 0x93, 0x8e, 0xd8, 0x86, 0xf4,
                0x96, 0xe5, 0xba, 0xb2, 0x75, 0x5c, 0x3c, 0x48, 0x4e, 0x88, 0x87, 0x90, 0x3d, 0x2f,
                0x6a, 0x57, 0xf3, 0xaa,
            ],
            Core::Subtract8 => [
                0x4f, 0x21, 0x17, 0xa0, 0xe8, 0x10, 0x59, 0xff, 0x0c, 0xd6, 0x4d, 0x84, 0x88, 0x65,
                0x42, 0xe5, 0x75, 0xea, 0x8d, 0x6e, 0xc0, 0x31, 0x08, 0xfd, 0x0b, 0x50, 0x8b, 0x39,
                0x20, 0x8c, 0xd0, 0xef,
            ],
            Core::Swu => [
                0x00, 0xf5, 0x1f, 0x4f, 0x4b, 0xec, 0xe7, 0x90, 0x03, 0xec, 0xad, 0x48, 0x1a, 0x12,
                0x5a, 0xf7, 0x17, 0x6e, 0x4d, 0xe9, 0x8c, 0x33, 0x92, 0x42, 0x5c, 0xb9, 0x14, 0x66,
                0x26, 0xc1, 0x3b, 0x3b,
            ],
            Core::TapdataInit => [
                0xa4, 0xd0, 0x22, 0xef, 0x5c, 0xf4, 0x67, 0xbc, 0xa0, 0x32, 0x5e, 0x46, 0x3f, 0xca,
                0xce, 0x7c, 0xbd, 0xd6, 0x4f, 0xf8, 0xf7, 0x1c, 0x5c, 0x7f, 0x63, 0xe6, 0x07, 0x84,
                0xaa, 0x0a, 0xc4, 0x86,
            ],
            Core::Verify => [
                0xcd, 0xca, 0x2a, 0x05, 0xe5, 0x2c, 0xef, 0xa5, 0x9d, 0xc7, 0xa5, 0xb0, 0xda, 0xe2,
                0x20, 0x98, 0xfb, 0x89, 0x6e, 0x39, 0x13, 0xbf, 0xdd, 0x44, 0x6b, 0x59, 0x4e, 0x1f,
                0x92, 0x50, 0x78, 0x3e,
            ],
            Core::Xor1 => [
                0x8c, 0x4e, 0x4e, 0x6e, 0xbf, 0x46, 0x30, 0xb2, 0x9b, 0x5a, 0x57, 0xea, 0x79, 0xf0,
                0xc9, 0xaf, 0x6b, 0xff, 0x54, 0xc4, 0xd2, 0xd7, 0x69, 0xbf, 0x51, 0x59, 0x47, 0x74,
                0xa5, 0x2b, 0x99, 0xc9,
            ],
            Core::Xor16 => [
                0xd9, 0xf0, 0xaf, 0x3f, 0xe3, 0xfd, 0x24, 0x7c, 0x1d, 0xf3, 0x4a, 0x25, 0x27, 0x13,
                0xb2, 0xe9, 0x33, 0xa9, 0x45, 0xa5, 0x67, 0x19, 0x48, 0x7f, 0x8e, 0xd7, 0xf5, 0x63,
                0xea, 0x86, 0x1a, 0xb5,
            ],
            Core::Xor32 => [
                0xd5, 0xae, 0x27, 0x12, 0xed, 0xea, 0xf6, 0x76, 0x52, 0x0f, 0xa3, 0xba, 0x0f, 0x40,
                0xbf, 0x4a, 0x16, 0x57, 0x43, 0x7e, 0xff, 0xbd, 0x99, 0x86, 0xd0, 0x6a, 0xe8, 0x1b,
                0x29, 0xa4, 0xf9, 0x8c,
            ],
            Core::Xor64 => [
                0xc4, 0xdf, 0x1c, 0xcf, 0x33, 0x3e, 0xde, 0xbd, 0xd4, 0x0d, 0xea, 0x9a, 0x0e, 0x6c,
                0xbb, 0x83, 0x06, 0x31, 0xe8, 0x3a, 0x94, 0xbb, 0x77, 0x9f, 0xe6, 0x00, 0x7b, 0xc6,
                0xcb, 0x53, 0xa5, 0x44,
            ],
            Core::Xor8 => [
                0x4a, 0xb1, 0x4a, 0x81, 0x4a, 0x39, 0x52, 0x8a, 0x80, 0xfd, 0xb4, 0x30, 0x58, 0x9b,
                0xa4, 0x50, 0x10, 0x4b, 0x9c, 0x72, 0x09, 0xaa, 0x2f, 0xe2, 0x85, 0xcd, 0x60, 0xc0,
                0x90, 0x43, 0x11, 0x4a,
            ],
            Core::XorXor1 => [
                0x18, 0xb9, 0x44, 0x6a, 0x41, 0x66, 0xa3, 0xfe, 0xe2, 0xbc, 0xb2, 0x54, 0x5b, 0xb9,
                0x01, 0x18, 0xdc, 0xf0, 0xe8, 0xf8, 0x86, 0xa1, 0x07, 0x6d, 0x4c, 0x38, 0x60, 0x06,
                0x0c, 0xde, 0x1a, 0x51,
            ],
            Core::XorXor16 => [
                0x94, 0x6c, 0xde, 0x87, 0x2e, 0x30, 0xe6, 0x50, 0x9d, 0xaf, 0xf4, 0x05, 0xf0, 0xe0,
                0xfe, 0xfe, 0x27, 0x55, 0x47, 0xb4, 0x0e, 0xb2, 0x03, 0x84, 0xaf, 0xe9, 0xa8, 0x63,
                0x60, 0xfc, 0x80, 0xef,
            ],
            Core::XorXor32 => [
                0x65, 0x27, 0xdf, 0x67, 0xa5, 0x0d, 0x14, 0x8d, 0xb4, 0xfc, 0x8f, 0xee, 0xc7, 0x84,
                0x55, 0x64, 0x99, 0xa8, 0xc7, 0xf0, 0xfa, 0x7d, 0x28, 0xe6, 0x27, 0x8e, 0x99, 0x7f,
                0x49, 0x59, 0xbe, 0x39,
            ],
            Core::XorXor64 => [
                0xf1, 0x62, 0xf9, 0xe6, 0x56, 0x63, 0xa6, 0x9a, 0xc5, 0xf9, 0x2a, 0x5e, 0xb5, 0x2c,
                0x03, 0x32, 0x39, 0x2e, 0xdd, 0x1e, 0xd1, 0xba, 0x35, 0x5e, 0x6f, 0x19, 0x40, 0x6e,
                0xab, 0xe3, 0xf6, 0xed,
            ],
            Core::XorXor8 => [
                0xe0, 0x6d, 0x69, 0x4c, 0x5b, 0x40, 0x7d, 0xda, 0xd7, 0xaa, 0x1f, 0x88, 0x07, 0x16,
                0xbc, 0xb7, 0x0a, 0xcd, 0xba, 0x75, 0x85, 0xca, 0x40, 0x09, 0x9a, 0x0a, 0x0a, 0x61,
                0xf3, 0xad, 0x2d, 0xb5,
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
            Core::TapdataInit => b"1",
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
            Core::ParseSequence => {
                b"+1+****22*22**22*22***22*22**22*22****22*22**22*22***22*22**22*22"
            }
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
            Core::Sha256Ctx8Add1 => {
                b"**+1h*+1*ll*+1l*+1i*+1****22*22**22*22***22*22**22*22+1***22*22**22*22*lh"
            }
            Core::Sha256Ctx8Add128 => {
                b"**+1h*+1*ll*+1l*+1i*+1****22*22**22*22***22*22**22*22+1***22*22**22*22*lh"
            }
            Core::Sha256Ctx8Add16 => {
                b"**+1h*+1*ll*+1l*+1i*+1****22*22**22*22***22*22**22*22+1***22*22**22*22*lh"
            }
            Core::Sha256Ctx8Add2 => {
                b"**+1h*+1*ll*+1l*+1i*+1****22*22**22*22***22*22**22*22+1***22*22**22*22*lh"
            }
            Core::Sha256Ctx8Add256 => {
                b"**+1h*+1*ll*+1l*+1i*+1****22*22**22*22***22*22**22*22+1***22*22**22*22*lh"
            }
            Core::Sha256Ctx8Add32 => {
                b"**+1h*+1*ll*+1l*+1i*+1****22*22**22*22***22*22**22*22+1***22*22**22*22*lh"
            }
            Core::Sha256Ctx8Add4 => {
                b"**+1h*+1*ll*+1l*+1i*+1****22*22**22*22***22*22**22*22+1***22*22**22*22*lh"
            }
            Core::Sha256Ctx8Add512 => {
                b"**+1h*+1*ll*+1l*+1i*+1****22*22**22*22***22*22**22*22+1***22*22**22*22*lh"
            }
            Core::Sha256Ctx8Add64 => {
                b"**+1h*+1*ll*+1l*+1i*+1****22*22**22*22***22*22**22*22+1***22*22**22*22*lh"
            }
            Core::Sha256Ctx8Add8 => {
                b"**+1h*+1*ll*+1l*+1i*+1****22*22**22*22***22*22**22*22+1***22*22**22*22*lh"
            }
            Core::Sha256Ctx8AddBuffer511 => {
                b"**+1h*+1*ll*+1l*+1i*+1****22*22**22*22***22*22**22*22+1***22*22**22*22*lh"
            }
            Core::Sha256Ctx8Finalize => b"h",
            Core::Sha256Ctx8Init => {
                b"**+1h*+1*ll*+1l*+1i*+1****22*22**22*22***22*22**22*22+1***22*22**22*22*lh"
            }
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
            Core::TapdataInit => {
                b"**+1h*+1*ll*+1l*+1i*+1****22*22**22*22***22*22**22*22+1***22*22**22*22*lh"
            }
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
            Core::TapdataInit => (413, 9),
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
                                            1 => {Core::TapdataInit}
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
            Core::FullLeftShift32_16 => {
                &simplicity_sys::c_jets::jets_wrapper::full_left_shift_32_16
            }
            Core::FullLeftShift32_2 => &simplicity_sys::c_jets::jets_wrapper::full_left_shift_32_2,
            Core::FullLeftShift32_4 => &simplicity_sys::c_jets::jets_wrapper::full_left_shift_32_4,
            Core::FullLeftShift32_8 => &simplicity_sys::c_jets::jets_wrapper::full_left_shift_32_8,
            Core::FullLeftShift64_1 => &simplicity_sys::c_jets::jets_wrapper::full_left_shift_64_1,
            Core::FullLeftShift64_16 => {
                &simplicity_sys::c_jets::jets_wrapper::full_left_shift_64_16
            }
            Core::FullLeftShift64_2 => &simplicity_sys::c_jets::jets_wrapper::full_left_shift_64_2,
            Core::FullLeftShift64_32 => {
                &simplicity_sys::c_jets::jets_wrapper::full_left_shift_64_32
            }
            Core::FullLeftShift64_4 => &simplicity_sys::c_jets::jets_wrapper::full_left_shift_64_4,
            Core::FullLeftShift64_8 => &simplicity_sys::c_jets::jets_wrapper::full_left_shift_64_8,
            Core::FullLeftShift8_1 => &simplicity_sys::c_jets::jets_wrapper::full_left_shift_8_1,
            Core::FullLeftShift8_2 => &simplicity_sys::c_jets::jets_wrapper::full_left_shift_8_2,
            Core::FullLeftShift8_4 => &simplicity_sys::c_jets::jets_wrapper::full_left_shift_8_4,
            Core::FullMultiply16 => &simplicity_sys::c_jets::jets_wrapper::full_multiply_16,
            Core::FullMultiply32 => &simplicity_sys::c_jets::jets_wrapper::full_multiply_32,
            Core::FullMultiply64 => &simplicity_sys::c_jets::jets_wrapper::full_multiply_64,
            Core::FullMultiply8 => &simplicity_sys::c_jets::jets_wrapper::full_multiply_8,
            Core::FullRightShift16_1 => {
                &simplicity_sys::c_jets::jets_wrapper::full_right_shift_16_1
            }
            Core::FullRightShift16_2 => {
                &simplicity_sys::c_jets::jets_wrapper::full_right_shift_16_2
            }
            Core::FullRightShift16_4 => {
                &simplicity_sys::c_jets::jets_wrapper::full_right_shift_16_4
            }
            Core::FullRightShift16_8 => {
                &simplicity_sys::c_jets::jets_wrapper::full_right_shift_16_8
            }
            Core::FullRightShift32_1 => {
                &simplicity_sys::c_jets::jets_wrapper::full_right_shift_32_1
            }
            Core::FullRightShift32_16 => {
                &simplicity_sys::c_jets::jets_wrapper::full_right_shift_32_16
            }
            Core::FullRightShift32_2 => {
                &simplicity_sys::c_jets::jets_wrapper::full_right_shift_32_2
            }
            Core::FullRightShift32_4 => {
                &simplicity_sys::c_jets::jets_wrapper::full_right_shift_32_4
            }
            Core::FullRightShift32_8 => {
                &simplicity_sys::c_jets::jets_wrapper::full_right_shift_32_8
            }
            Core::FullRightShift64_1 => {
                &simplicity_sys::c_jets::jets_wrapper::full_right_shift_64_1
            }
            Core::FullRightShift64_16 => {
                &simplicity_sys::c_jets::jets_wrapper::full_right_shift_64_16
            }
            Core::FullRightShift64_2 => {
                &simplicity_sys::c_jets::jets_wrapper::full_right_shift_64_2
            }
            Core::FullRightShift64_32 => {
                &simplicity_sys::c_jets::jets_wrapper::full_right_shift_64_32
            }
            Core::FullRightShift64_4 => {
                &simplicity_sys::c_jets::jets_wrapper::full_right_shift_64_4
            }
            Core::FullRightShift64_8 => {
                &simplicity_sys::c_jets::jets_wrapper::full_right_shift_64_8
            }
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
            Core::ScalarMultiplyLambda => {
                &simplicity_sys::c_jets::jets_wrapper::scalar_multiply_lambda
            }
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
            Core::Sha256Ctx8AddBuffer511 => {
                &simplicity_sys::c_jets::jets_wrapper::sha_256_ctx_8_add_buffer_511
            }
            Core::Sha256Ctx8Finalize => {
                &simplicity_sys::c_jets::jets_wrapper::sha_256_ctx_8_finalize
            }
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
            Core::TapdataInit => &simplicity_sys::c_jets::jets_wrapper::tapdata_init,
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
            Core::Add16 => Cost::from_milliweight(80),
            Core::Add32 => Cost::from_milliweight(92),
            Core::Add64 => Cost::from_milliweight(105),
            Core::Add8 => Cost::from_milliweight(97),
            Core::All16 => Cost::from_milliweight(60),
            Core::All32 => Cost::from_milliweight(62),
            Core::All64 => Cost::from_milliweight(63),
            Core::All8 => Cost::from_milliweight(50),
            Core::And1 => Cost::from_milliweight(77),
            Core::And16 => Cost::from_milliweight(83),
            Core::And32 => Cost::from_milliweight(77),
            Core::And64 => Cost::from_milliweight(78),
            Core::And8 => Cost::from_milliweight(98),
            Core::Bip0340Verify => Cost::from_milliweight(49421),
            Core::Ch1 => Cost::from_milliweight(50),
            Core::Ch16 => Cost::from_milliweight(83),
            Core::Ch32 => Cost::from_milliweight(69),
            Core::Ch64 => Cost::from_milliweight(78),
            Core::Ch8 => Cost::from_milliweight(86),
            Core::CheckSigVerify => Cost::from_milliweight(50000),
            Core::Complement1 => Cost::from_milliweight(51),
            Core::Complement16 => Cost::from_milliweight(86),
            Core::Complement32 => Cost::from_milliweight(58),
            Core::Complement64 => Cost::from_milliweight(64),
            Core::Complement8 => Cost::from_milliweight(62),
            Core::Decompress => Cost::from_milliweight(10495),
            Core::Decrement16 => Cost::from_milliweight(58),
            Core::Decrement32 => Cost::from_milliweight(57),
            Core::Decrement64 => Cost::from_milliweight(79),
            Core::Decrement8 => Cost::from_milliweight(77),
            Core::DivMod128_64 => Cost::from_milliweight(169),
            Core::DivMod16 => Cost::from_milliweight(92),
            Core::DivMod32 => Cost::from_milliweight(90),
            Core::DivMod64 => Cost::from_milliweight(82),
            Core::DivMod8 => Cost::from_milliweight(91),
            Core::Divide16 => Cost::from_milliweight(85),
            Core::Divide32 => Cost::from_milliweight(82),
            Core::Divide64 => Cost::from_milliweight(81),
            Core::Divide8 => Cost::from_milliweight(85),
            Core::Divides16 => Cost::from_milliweight(84),
            Core::Divides32 => Cost::from_milliweight(80),
            Core::Divides64 => Cost::from_milliweight(67),
            Core::Divides8 => Cost::from_milliweight(73),
            Core::Eq1 => Cost::from_milliweight(63),
            Core::Eq16 => Cost::from_milliweight(68),
            Core::Eq256 => Cost::from_milliweight(188),
            Core::Eq32 => Cost::from_milliweight(74),
            Core::Eq64 => Cost::from_milliweight(82),
            Core::Eq8 => Cost::from_milliweight(76),
            Core::FeAdd => Cost::from_milliweight(777),
            Core::FeInvert => Cost::from_milliweight(3237),
            Core::FeIsOdd => Cost::from_milliweight(313),
            Core::FeIsZero => Cost::from_milliweight(277),
            Core::FeMultiply => Cost::from_milliweight(813),
            Core::FeMultiplyBeta => Cost::from_milliweight(607),
            Core::FeNegate => Cost::from_milliweight(541),
            Core::FeNormalize => Cost::from_milliweight(656),
            Core::FeSquare => Cost::from_milliweight(570),
            Core::FeSquareRoot => Cost::from_milliweight(10162),
            Core::FullAdd16 => Cost::from_milliweight(106),
            Core::FullAdd32 => Cost::from_milliweight(96),
            Core::FullAdd64 => Cost::from_milliweight(93),
            Core::FullAdd8 => Cost::from_milliweight(131),
            Core::FullDecrement16 => Cost::from_milliweight(60),
            Core::FullDecrement32 => Cost::from_milliweight(71),
            Core::FullDecrement64 => Cost::from_milliweight(71),
            Core::FullDecrement8 => Cost::from_milliweight(68),
            Core::FullIncrement16 => Cost::from_milliweight(70),
            Core::FullIncrement32 => Cost::from_milliweight(57),
            Core::FullIncrement64 => Cost::from_milliweight(68),
            Core::FullIncrement8 => Cost::from_milliweight(73),
            Core::FullLeftShift16_1 => Cost::from_milliweight(76),
            Core::FullLeftShift16_2 => Cost::from_milliweight(59),
            Core::FullLeftShift16_4 => Cost::from_milliweight(68),
            Core::FullLeftShift16_8 => Cost::from_milliweight(68),
            Core::FullLeftShift32_1 => Cost::from_milliweight(58),
            Core::FullLeftShift32_16 => Cost::from_milliweight(52),
            Core::FullLeftShift32_2 => Cost::from_milliweight(73),
            Core::FullLeftShift32_4 => Cost::from_milliweight(59),
            Core::FullLeftShift32_8 => Cost::from_milliweight(60),
            Core::FullLeftShift64_1 => Cost::from_milliweight(74),
            Core::FullLeftShift64_16 => Cost::from_milliweight(69),
            Core::FullLeftShift64_2 => Cost::from_milliweight(70),
            Core::FullLeftShift64_32 => Cost::from_milliweight(73),
            Core::FullLeftShift64_4 => Cost::from_milliweight(66),
            Core::FullLeftShift64_8 => Cost::from_milliweight(68),
            Core::FullLeftShift8_1 => Cost::from_milliweight(60),
            Core::FullLeftShift8_2 => Cost::from_milliweight(64),
            Core::FullLeftShift8_4 => Cost::from_milliweight(72),
            Core::FullMultiply16 => Cost::from_milliweight(99),
            Core::FullMultiply32 => Cost::from_milliweight(87),
            Core::FullMultiply64 => Cost::from_milliweight(103),
            Core::FullMultiply8 => Cost::from_milliweight(95),
            Core::FullRightShift16_1 => Cost::from_milliweight(55),
            Core::FullRightShift16_2 => Cost::from_milliweight(60),
            Core::FullRightShift16_4 => Cost::from_milliweight(64),
            Core::FullRightShift16_8 => Cost::from_milliweight(55),
            Core::FullRightShift32_1 => Cost::from_milliweight(49),
            Core::FullRightShift32_16 => Cost::from_milliweight(48),
            Core::FullRightShift32_2 => Cost::from_milliweight(66),
            Core::FullRightShift32_4 => Cost::from_milliweight(49),
            Core::FullRightShift32_8 => Cost::from_milliweight(66),
            Core::FullRightShift64_1 => Cost::from_milliweight(60),
            Core::FullRightShift64_16 => Cost::from_milliweight(73),
            Core::FullRightShift64_2 => Cost::from_milliweight(76),
            Core::FullRightShift64_32 => Cost::from_milliweight(73),
            Core::FullRightShift64_4 => Cost::from_milliweight(56),
            Core::FullRightShift64_8 => Cost::from_milliweight(68),
            Core::FullRightShift8_1 => Cost::from_milliweight(59),
            Core::FullRightShift8_2 => Cost::from_milliweight(49),
            Core::FullRightShift8_4 => Cost::from_milliweight(51),
            Core::FullSubtract16 => Cost::from_milliweight(99),
            Core::FullSubtract32 => Cost::from_milliweight(92),
            Core::FullSubtract64 => Cost::from_milliweight(109),
            Core::FullSubtract8 => Cost::from_milliweight(106),
            Core::GeIsOnCurve => Cost::from_milliweight(688),
            Core::GeNegate => Cost::from_milliweight(1071),
            Core::GejAdd => Cost::from_milliweight(3000),
            Core::GejDouble => Cost::from_milliweight(1862),
            Core::GejEquiv => Cost::from_milliweight(2376),
            Core::GejGeAdd => Cost::from_milliweight(2609),
            Core::GejGeAddEx => Cost::from_milliweight(2860),
            Core::GejGeEquiv => Cost::from_milliweight(1823),
            Core::GejInfinity => Cost::from_milliweight(765),
            Core::GejIsInfinity => Cost::from_milliweight(701),
            Core::GejIsOnCurve => Cost::from_milliweight(1039),
            Core::GejNegate => Cost::from_milliweight(1549),
            Core::GejNormalize => Cost::from_milliweight(4184),
            Core::GejRescale => Cost::from_milliweight(2011),
            Core::GejXEquiv => Cost::from_milliweight(1103),
            Core::GejYIsOdd => Cost::from_milliweight(3702),
            Core::Generate => Cost::from_milliweight(49851),
            Core::HashToCurve => Cost::from_milliweight(69844),
            Core::High1 => Cost::from_milliweight(42),
            Core::High16 => Cost::from_milliweight(50),
            Core::High32 => Cost::from_milliweight(64),
            Core::High64 => Cost::from_milliweight(52),
            Core::High8 => Cost::from_milliweight(59),
            Core::Increment16 => Cost::from_milliweight(56),
            Core::Increment32 => Cost::from_milliweight(73),
            Core::Increment64 => Cost::from_milliweight(64),
            Core::Increment8 => Cost::from_milliweight(69),
            Core::IsOne16 => Cost::from_milliweight(64),
            Core::IsOne32 => Cost::from_milliweight(64),
            Core::IsOne64 => Cost::from_milliweight(66),
            Core::IsOne8 => Cost::from_milliweight(47),
            Core::IsZero16 => Cost::from_milliweight(52),
            Core::IsZero32 => Cost::from_milliweight(58),
            Core::IsZero64 => Cost::from_milliweight(68),
            Core::IsZero8 => Cost::from_milliweight(59),
            Core::Le16 => Cost::from_milliweight(83),
            Core::Le32 => Cost::from_milliweight(99),
            Core::Le64 => Cost::from_milliweight(79),
            Core::Le8 => Cost::from_milliweight(93),
            Core::LeftExtend16_32 => Cost::from_milliweight(72),
            Core::LeftExtend16_64 => Cost::from_milliweight(69),
            Core::LeftExtend1_16 => Cost::from_milliweight(50),
            Core::LeftExtend1_32 => Cost::from_milliweight(48),
            Core::LeftExtend1_64 => Cost::from_milliweight(49),
            Core::LeftExtend1_8 => Cost::from_milliweight(46),
            Core::LeftExtend32_64 => Cost::from_milliweight(69),
            Core::LeftExtend8_16 => Cost::from_milliweight(58),
            Core::LeftExtend8_32 => Cost::from_milliweight(86),
            Core::LeftExtend8_64 => Cost::from_milliweight(98),
            Core::LeftPadHigh16_32 => Cost::from_milliweight(71),
            Core::LeftPadHigh16_64 => Cost::from_milliweight(82),
            Core::LeftPadHigh1_16 => Cost::from_milliweight(106),
            Core::LeftPadHigh1_32 => Cost::from_milliweight(220),
            Core::LeftPadHigh1_64 => Cost::from_milliweight(302),
            Core::LeftPadHigh1_8 => Cost::from_milliweight(73),
            Core::LeftPadHigh32_64 => Cost::from_milliweight(69),
            Core::LeftPadHigh8_16 => Cost::from_milliweight(65),
            Core::LeftPadHigh8_32 => Cost::from_milliweight(105),
            Core::LeftPadHigh8_64 => Cost::from_milliweight(113),
            Core::LeftPadLow16_32 => Cost::from_milliweight(65),
            Core::LeftPadLow16_64 => Cost::from_milliweight(68),
            Core::LeftPadLow1_16 => Cost::from_milliweight(59),
            Core::LeftPadLow1_32 => Cost::from_milliweight(47),
            Core::LeftPadLow1_64 => Cost::from_milliweight(46),
            Core::LeftPadLow1_8 => Cost::from_milliweight(48),
            Core::LeftPadLow32_64 => Cost::from_milliweight(62),
            Core::LeftPadLow8_16 => Cost::from_milliweight(56),
            Core::LeftPadLow8_32 => Cost::from_milliweight(75),
            Core::LeftPadLow8_64 => Cost::from_milliweight(116),
            Core::LeftRotate16 => Cost::from_milliweight(88),
            Core::LeftRotate32 => Cost::from_milliweight(62),
            Core::LeftRotate64 => Cost::from_milliweight(68),
            Core::LeftRotate8 => Cost::from_milliweight(66),
            Core::LeftShift16 => Cost::from_milliweight(109),
            Core::LeftShift32 => Cost::from_milliweight(79),
            Core::LeftShift64 => Cost::from_milliweight(70),
            Core::LeftShift8 => Cost::from_milliweight(72),
            Core::LeftShiftWith16 => Cost::from_milliweight(72),
            Core::LeftShiftWith32 => Cost::from_milliweight(87),
            Core::LeftShiftWith64 => Cost::from_milliweight(97),
            Core::LeftShiftWith8 => Cost::from_milliweight(104),
            Core::Leftmost16_1 => Cost::from_milliweight(68),
            Core::Leftmost16_2 => Cost::from_milliweight(58),
            Core::Leftmost16_4 => Cost::from_milliweight(51),
            Core::Leftmost16_8 => Cost::from_milliweight(62),
            Core::Leftmost32_1 => Cost::from_milliweight(53),
            Core::Leftmost32_16 => Cost::from_milliweight(63),
            Core::Leftmost32_2 => Cost::from_milliweight(62),
            Core::Leftmost32_4 => Cost::from_milliweight(61),
            Core::Leftmost32_8 => Cost::from_milliweight(60),
            Core::Leftmost64_1 => Cost::from_milliweight(65),
            Core::Leftmost64_16 => Cost::from_milliweight(62),
            Core::Leftmost64_2 => Cost::from_milliweight(61),
            Core::Leftmost64_32 => Cost::from_milliweight(77),
            Core::Leftmost64_4 => Cost::from_milliweight(80),
            Core::Leftmost64_8 => Cost::from_milliweight(54),
            Core::Leftmost8_1 => Cost::from_milliweight(54),
            Core::Leftmost8_2 => Cost::from_milliweight(71),
            Core::Leftmost8_4 => Cost::from_milliweight(65),
            Core::LinearCombination1 => Cost::from_milliweight(85743),
            Core::LinearVerify1 => Cost::from_milliweight(43579),
            Core::Low1 => Cost::from_milliweight(40),
            Core::Low16 => Cost::from_milliweight(60),
            Core::Low32 => Cost::from_milliweight(52),
            Core::Low64 => Cost::from_milliweight(50),
            Core::Low8 => Cost::from_milliweight(45),
            Core::Lt16 => Cost::from_milliweight(83),
            Core::Lt32 => Cost::from_milliweight(89),
            Core::Lt64 => Cost::from_milliweight(71),
            Core::Lt8 => Cost::from_milliweight(86),
            Core::Maj1 => Cost::from_milliweight(54),
            Core::Maj16 => Cost::from_milliweight(85),
            Core::Maj32 => Cost::from_milliweight(73),
            Core::Maj64 => Cost::from_milliweight(79),
            Core::Maj8 => Cost::from_milliweight(64),
            Core::Max16 => Cost::from_milliweight(80),
            Core::Max32 => Cost::from_milliweight(70),
            Core::Max64 => Cost::from_milliweight(75),
            Core::Max8 => Cost::from_milliweight(79),
            Core::Median16 => Cost::from_milliweight(80),
            Core::Median32 => Cost::from_milliweight(77),
            Core::Median64 => Cost::from_milliweight(89),
            Core::Median8 => Cost::from_milliweight(77),
            Core::Min16 => Cost::from_milliweight(83),
            Core::Min32 => Cost::from_milliweight(96),
            Core::Min64 => Cost::from_milliweight(82),
            Core::Min8 => Cost::from_milliweight(78),
            Core::Modulo16 => Cost::from_milliweight(85),
            Core::Modulo32 => Cost::from_milliweight(81),
            Core::Modulo64 => Cost::from_milliweight(71),
            Core::Modulo8 => Cost::from_milliweight(85),
            Core::Multiply16 => Cost::from_milliweight(79),
            Core::Multiply32 => Cost::from_milliweight(78),
            Core::Multiply64 => Cost::from_milliweight(72),
            Core::Multiply8 => Cost::from_milliweight(79),
            Core::Negate16 => Cost::from_milliweight(69),
            Core::Negate32 => Cost::from_milliweight(56),
            Core::Negate64 => Cost::from_milliweight(56),
            Core::Negate8 => Cost::from_milliweight(69),
            Core::One16 => Cost::from_milliweight(45),
            Core::One32 => Cost::from_milliweight(45),
            Core::One64 => Cost::from_milliweight(45),
            Core::One8 => Cost::from_milliweight(46),
            Core::Or1 => Cost::from_milliweight(56),
            Core::Or16 => Cost::from_milliweight(78),
            Core::Or32 => Cost::from_milliweight(80),
            Core::Or64 => Cost::from_milliweight(71),
            Core::Or8 => Cost::from_milliweight(81),
            Core::ParseLock => Cost::from_milliweight(82),
            Core::ParseSequence => Cost::from_milliweight(93),
            Core::PointVerify1 => Cost::from_milliweight(41394),
            Core::RightExtend16_32 => Cost::from_milliweight(73),
            Core::RightExtend16_64 => Cost::from_milliweight(70),
            Core::RightExtend32_64 => Cost::from_milliweight(62),
            Core::RightExtend8_16 => Cost::from_milliweight(63),
            Core::RightExtend8_32 => Cost::from_milliweight(69),
            Core::RightExtend8_64 => Cost::from_milliweight(141),
            Core::RightPadHigh16_32 => Cost::from_milliweight(66),
            Core::RightPadHigh16_64 => Cost::from_milliweight(81),
            Core::RightPadHigh1_16 => Cost::from_milliweight(114),
            Core::RightPadHigh1_32 => Cost::from_milliweight(220),
            Core::RightPadHigh1_64 => Cost::from_milliweight(313),
            Core::RightPadHigh1_8 => Cost::from_milliweight(73),
            Core::RightPadHigh32_64 => Cost::from_milliweight(62),
            Core::RightPadHigh8_16 => Cost::from_milliweight(75),
            Core::RightPadHigh8_32 => Cost::from_milliweight(81),
            Core::RightPadHigh8_64 => Cost::from_milliweight(118),
            Core::RightPadLow16_32 => Cost::from_milliweight(62),
            Core::RightPadLow16_64 => Cost::from_milliweight(98),
            Core::RightPadLow1_16 => Cost::from_milliweight(60),
            Core::RightPadLow1_32 => Cost::from_milliweight(47),
            Core::RightPadLow1_64 => Cost::from_milliweight(57),
            Core::RightPadLow1_8 => Cost::from_milliweight(48),
            Core::RightPadLow32_64 => Cost::from_milliweight(74),
            Core::RightPadLow8_16 => Cost::from_milliweight(62),
            Core::RightPadLow8_32 => Cost::from_milliweight(69),
            Core::RightPadLow8_64 => Cost::from_milliweight(98),
            Core::RightRotate16 => Cost::from_milliweight(67),
            Core::RightRotate32 => Cost::from_milliweight(77),
            Core::RightRotate64 => Cost::from_milliweight(64),
            Core::RightRotate8 => Cost::from_milliweight(72),
            Core::RightShift16 => Cost::from_milliweight(60),
            Core::RightShift32 => Cost::from_milliweight(69),
            Core::RightShift64 => Cost::from_milliweight(68),
            Core::RightShift8 => Cost::from_milliweight(63),
            Core::RightShiftWith16 => Cost::from_milliweight(83),
            Core::RightShiftWith32 => Cost::from_milliweight(78),
            Core::RightShiftWith64 => Cost::from_milliweight(72),
            Core::RightShiftWith8 => Cost::from_milliweight(71),
            Core::Rightmost16_1 => Cost::from_milliweight(70),
            Core::Rightmost16_2 => Cost::from_milliweight(65),
            Core::Rightmost16_4 => Cost::from_milliweight(72),
            Core::Rightmost16_8 => Cost::from_milliweight(69),
            Core::Rightmost32_1 => Cost::from_milliweight(70),
            Core::Rightmost32_16 => Cost::from_milliweight(56),
            Core::Rightmost32_2 => Cost::from_milliweight(74),
            Core::Rightmost32_4 => Cost::from_milliweight(57),
            Core::Rightmost32_8 => Cost::from_milliweight(55),
            Core::Rightmost64_1 => Cost::from_milliweight(61),
            Core::Rightmost64_16 => Cost::from_milliweight(63),
            Core::Rightmost64_2 => Cost::from_milliweight(65),
            Core::Rightmost64_32 => Cost::from_milliweight(64),
            Core::Rightmost64_4 => Cost::from_milliweight(57),
            Core::Rightmost64_8 => Cost::from_milliweight(49),
            Core::Rightmost8_1 => Cost::from_milliweight(65),
            Core::Rightmost8_2 => Cost::from_milliweight(63),
            Core::Rightmost8_4 => Cost::from_milliweight(56),
            Core::ScalarAdd => Cost::from_milliweight(778),
            Core::ScalarInvert => Cost::from_milliweight(3178),
            Core::ScalarIsZero => Cost::from_milliweight(271),
            Core::ScalarMultiply => Cost::from_milliweight(793),
            Core::ScalarMultiplyLambda => Cost::from_milliweight(567),
            Core::ScalarNegate => Cost::from_milliweight(516),
            Core::ScalarNormalize => Cost::from_milliweight(500),
            Core::ScalarSquare => Cost::from_milliweight(571),
            Core::Scale => Cost::from_milliweight(73548),
            Core::Sha256Block => Cost::from_milliweight(765),
            Core::Sha256Ctx8Add1 => Cost::from_milliweight(664),
            Core::Sha256Ctx8Add128 => Cost::from_milliweight(1778),
            Core::Sha256Ctx8Add16 => Cost::from_milliweight(781),
            Core::Sha256Ctx8Add2 => Cost::from_milliweight(674),
            Core::Sha256Ctx8Add256 => Cost::from_milliweight(2894),
            Core::Sha256Ctx8Add32 => Cost::from_milliweight(928),
            Core::Sha256Ctx8Add4 => Cost::from_milliweight(656),
            Core::Sha256Ctx8Add512 => Cost::from_milliweight(5161),
            Core::Sha256Ctx8Add64 => Cost::from_milliweight(1220),
            Core::Sha256Ctx8Add8 => Cost::from_milliweight(694),
            Core::Sha256Ctx8AddBuffer511 => Cost::from_milliweight(5137),
            Core::Sha256Ctx8Finalize => Cost::from_milliweight(833),
            Core::Sha256Ctx8Init => Cost::from_milliweight(123),
            Core::Sha256Iv => Cost::from_milliweight(92),
            Core::Some1 => Cost::from_milliweight(60),
            Core::Some16 => Cost::from_milliweight(52),
            Core::Some32 => Cost::from_milliweight(49),
            Core::Some64 => Cost::from_milliweight(62),
            Core::Some8 => Cost::from_milliweight(57),
            Core::Subtract16 => Cost::from_milliweight(93),
            Core::Subtract32 => Cost::from_milliweight(87),
            Core::Subtract64 => Cost::from_milliweight(125),
            Core::Subtract8 => Cost::from_milliweight(96),
            Core::Swu => Cost::from_milliweight(32780),
            Core::TapdataInit => Cost::from_milliweight(1233),
            Core::Verify => Cost::from_milliweight(44),
            Core::Xor1 => Cost::from_milliweight(60),
            Core::Xor16 => Cost::from_milliweight(73),
            Core::Xor32 => Cost::from_milliweight(77),
            Core::Xor64 => Cost::from_milliweight(68),
            Core::Xor8 => Cost::from_milliweight(80),
            Core::XorXor1 => Cost::from_milliweight(50),
            Core::XorXor16 => Cost::from_milliweight(82),
            Core::XorXor32 => Cost::from_milliweight(82),
            Core::XorXor64 => Cost::from_milliweight(80),
            Core::XorXor8 => Cost::from_milliweight(86),
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
            Core::TapdataInit => f.write_str("tapdata_init"),
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
            "tapdata_init" => Ok(Core::TapdataInit),
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
