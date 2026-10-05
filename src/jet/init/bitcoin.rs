/* This file has been automatically generated. */

use crate::jet::type_name::TypeName;
use crate::jet::Jet;
use crate::merkle::cmr::Cmr;
use crate::decode_bits;
use crate::{decode, BitIter, BitWriter};
use crate::analysis::Cost;
use simplicity_sys::CFrameItem;
use simplicity_sys::c_jets::c_env::bitcoin as c_bitcoin;
use std::io::Write;
use std::{fmt, str};

/// The Bitcoin jet family.
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
    And1,
    And16,
    And32,
    And64,
    And8,
    AnnexHash,
    Bip0340Verify,
    BuildTapbranch,
    BuildTapleafSimplicity,
    BuildTaptweak,
    Ch1,
    Ch16,
    Ch32,
    Ch64,
    Ch8,
    CheckLockDistance,
    CheckLockDuration,
    CheckLockHeight,
    CheckLockTime,
    CheckSigVerify,
    Complement1,
    Complement16,
    Complement32,
    Complement64,
    Complement8,
    CurrentAnnexHash,
    CurrentIndex,
    CurrentPrevOutpoint,
    CurrentScriptHash,
    CurrentScriptSigHash,
    CurrentSequence,
    CurrentValue,
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
    Fee,
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
    InputAnnexHash,
    InputAnnexesHash,
    InputHash,
    InputOutpointsHash,
    InputPrevOutpoint,
    InputScriptHash,
    InputScriptSigHash,
    InputScriptSigsHash,
    InputScriptsHash,
    InputSequence,
    InputSequencesHash,
    InputUtxoHash,
    InputUtxosHash,
    InputValue,
    InputValuesHash,
    InputsHash,
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
    LockTime,
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
    NumInputs,
    NumOutputs,
    One16,
    One32,
    One64,
    One8,
    Or1,
    Or16,
    Or32,
    Or64,
    Or8,
    OutpointHash,
    OutputHash,
    OutputScriptHash,
    OutputScriptsHash,
    OutputValue,
    OutputValuesHash,
    OutputsHash,
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
    SigAllHash,
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
    TapEnvHash,
    TapdataInit,
    TapleafHash,
    TapleafVersion,
    Tappath,
    TappathHash,
    TotalInputValue,
    TotalOutputValue,
    TransactionId,
    TxHash,
    TxIsFinal,
    TxLockDistance,
    TxLockDuration,
    TxLockHeight,
    TxLockTime,
    Verify,
    Version,
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

impl Bitcoin {
    /// Array of all Bitcoin jets.
    pub const ALL: [Self; 428] = [
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
        Self::AnnexHash,
        Self::Bip0340Verify,
        Self::BuildTapbranch,
        Self::BuildTapleafSimplicity,
        Self::BuildTaptweak,
        Self::Ch1,
        Self::Ch16,
        Self::Ch32,
        Self::Ch64,
        Self::Ch8,
        Self::CheckLockDistance,
        Self::CheckLockDuration,
        Self::CheckLockHeight,
        Self::CheckLockTime,
        Self::CheckSigVerify,
        Self::Complement1,
        Self::Complement16,
        Self::Complement32,
        Self::Complement64,
        Self::Complement8,
        Self::CurrentAnnexHash,
        Self::CurrentIndex,
        Self::CurrentPrevOutpoint,
        Self::CurrentScriptHash,
        Self::CurrentScriptSigHash,
        Self::CurrentSequence,
        Self::CurrentValue,
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
        Self::Fee,
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
        Self::InputAnnexHash,
        Self::InputAnnexesHash,
        Self::InputHash,
        Self::InputOutpointsHash,
        Self::InputPrevOutpoint,
        Self::InputScriptHash,
        Self::InputScriptSigHash,
        Self::InputScriptSigsHash,
        Self::InputScriptsHash,
        Self::InputSequence,
        Self::InputSequencesHash,
        Self::InputUtxoHash,
        Self::InputUtxosHash,
        Self::InputValue,
        Self::InputValuesHash,
        Self::InputsHash,
        Self::InternalKey,
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
        Self::LockTime,
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
        Self::NumInputs,
        Self::NumOutputs,
        Self::One16,
        Self::One32,
        Self::One64,
        Self::One8,
        Self::Or1,
        Self::Or16,
        Self::Or32,
        Self::Or64,
        Self::Or8,
        Self::OutpointHash,
        Self::OutputHash,
        Self::OutputScriptHash,
        Self::OutputScriptsHash,
        Self::OutputValue,
        Self::OutputValuesHash,
        Self::OutputsHash,
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
        Self::ScriptCMR,
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
        Self::SigAllHash,
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
        Self::TapEnvHash,
        Self::TapdataInit,
        Self::TapleafHash,
        Self::TapleafVersion,
        Self::Tappath,
        Self::TappathHash,
        Self::TotalInputValue,
        Self::TotalOutputValue,
        Self::TransactionId,
        Self::TxHash,
        Self::TxIsFinal,
        Self::TxLockDistance,
        Self::TxLockDuration,
        Self::TxLockHeight,
        Self::TxLockTime,
        Self::Verify,
        Self::Version,
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

impl Jet for Bitcoin {

    fn cmr(&self) -> Cmr {
        let bytes = match self {
            Bitcoin::Add16 => [
                0x49, 0x42, 0x5a, 0x86, 0xe2, 0x0a, 0x67, 0x6d, 0x8b, 0x87, 0xe3, 0xc1, 0xa9, 0xb8,
                0xea, 0x6e, 0xc7, 0x5d, 0x85, 0x9c, 0x12, 0xc5, 0x1b, 0xcb, 0x7f, 0xa9, 0xf9, 0x69,
                0x12, 0xc3, 0x49, 0xcf,
            ],
            Bitcoin::Add32 => [
                0x46, 0x68, 0xcd, 0x55, 0xe8, 0xd1, 0x59, 0x19, 0x53, 0x32, 0x70, 0x14, 0xec, 0x64,
                0xc8, 0xe7, 0xd5, 0x2b, 0x86, 0xb5, 0x3e, 0x11, 0xc0, 0x14, 0x57, 0xea, 0xf2, 0xc3,
                0xd3, 0xce, 0xbf, 0x9f,
            ],
            Bitcoin::Add64 => [
                0xbe, 0x2b, 0x75, 0x19, 0x30, 0x3a, 0x67, 0xee, 0xa6, 0xb4, 0x82, 0x95, 0x0e, 0xda,
                0x83, 0x43, 0x5e, 0x1d, 0xe8, 0x55, 0x9c, 0x39, 0x4a, 0x23, 0x62, 0x22, 0xff, 0x5b,
                0xf0, 0x89, 0xd3, 0x46,
            ],
            Bitcoin::Add8 => [
                0xdf, 0xa1, 0x79, 0xad, 0xf4, 0x55, 0x0b, 0x28, 0x48, 0x73, 0xbf, 0x30, 0x12, 0x3e,
                0x0d, 0x4e, 0x54, 0x06, 0x9b, 0x08, 0x58, 0x34, 0xce, 0x56, 0x58, 0x15, 0xef, 0x7e,
                0x45, 0x78, 0x4a, 0xcb,
            ],
            Bitcoin::All16 => [
                0x24, 0xf4, 0x82, 0xa5, 0x13, 0xd3, 0x33, 0x62, 0x01, 0x5d, 0x28, 0xdf, 0x4b, 0xb6,
                0xc3, 0xee, 0x08, 0xab, 0x8a, 0xfb, 0xbd, 0x25, 0x57, 0x1f, 0x0e, 0xa8, 0x9d, 0x8c,
                0xab, 0xa3, 0x14, 0x04,
            ],
            Bitcoin::All32 => [
                0xa7, 0x16, 0x52, 0x2d, 0x0f, 0x37, 0x87, 0xc8, 0xb4, 0xd5, 0x07, 0x64, 0x7f, 0x1f,
                0x80, 0x7b, 0x67, 0xf3, 0x20, 0xd6, 0xeb, 0x67, 0xb8, 0x4b, 0x60, 0x9c, 0xec, 0x1d,
                0x2f, 0x12, 0x21, 0x8a,
            ],
            Bitcoin::All64 => [
                0x7a, 0xee, 0xfe, 0x2e, 0xce, 0x24, 0xba, 0xb3, 0x7c, 0x6e, 0x54, 0x30, 0xee, 0xd4,
                0x19, 0xfc, 0xd5, 0xf0, 0x37, 0x91, 0x2d, 0x17, 0x70, 0xcb, 0x7d, 0x65, 0x20, 0xdc,
                0xe5, 0x25, 0x29, 0x1a,
            ],
            Bitcoin::All8 => [
                0x46, 0x37, 0xf4, 0x0e, 0x5f, 0x47, 0x26, 0xb0, 0x05, 0x70, 0x76, 0x5a, 0xc7, 0x94,
                0xe2, 0x9e, 0xd1, 0xbb, 0x26, 0x55, 0xff, 0xc4, 0x12, 0xb2, 0xdc, 0x41, 0x25, 0x8e,
                0x41, 0xaa, 0xc6, 0x24,
            ],
            Bitcoin::And1 => [
                0x10, 0x68, 0x4d, 0x0d, 0xd7, 0x2c, 0xb0, 0xa8, 0x26, 0xa8, 0x63, 0x83, 0x4e, 0x01,
                0x1f, 0x50, 0xfa, 0x0d, 0x55, 0x8b, 0xa7, 0x7d, 0x6b, 0x9f, 0x49, 0xa1, 0xac, 0x22,
                0x90, 0x2a, 0x6a, 0xd0,
            ],
            Bitcoin::And16 => [
                0x37, 0x3c, 0x73, 0x0f, 0xad, 0x3e, 0x88, 0x47, 0x99, 0x1a, 0xa4, 0x17, 0xd9, 0xf0,
                0x80, 0xee, 0x1c, 0xb8, 0x8a, 0x7f, 0x72, 0x06, 0xf3, 0xfa, 0x84, 0x0b, 0x19, 0x50,
                0x77, 0x61, 0xfb, 0x23,
            ],
            Bitcoin::And32 => [
                0x13, 0xb0, 0x2c, 0x4c, 0x60, 0xae, 0x6e, 0xa4, 0x91, 0x16, 0x16, 0x49, 0xac, 0xf9,
                0xa4, 0x7a, 0x70, 0x25, 0xaf, 0x84, 0x7d, 0x5f, 0x58, 0x1e, 0x6f, 0x1c, 0xcc, 0xfb,
                0x21, 0xd3, 0x00, 0x1d,
            ],
            Bitcoin::And64 => [
                0x92, 0x18, 0x55, 0x35, 0xd4, 0x50, 0x54, 0x07, 0xde, 0xa3, 0xc8, 0xa6, 0x08, 0x26,
                0xed, 0xe6, 0x4a, 0x8f, 0xbb, 0x3d, 0xb4, 0x86, 0xd5, 0x6f, 0x64, 0x2d, 0x21, 0x7c,
                0x29, 0xcb, 0xd7, 0x95,
            ],
            Bitcoin::And8 => [
                0x26, 0x9a, 0x1b, 0x44, 0x62, 0x66, 0xf8, 0xf4, 0xa4, 0xa3, 0x8f, 0xa7, 0xe7, 0xe3,
                0x91, 0x82, 0xf1, 0x52, 0x14, 0x36, 0x14, 0x2b, 0xad, 0xed, 0xf3, 0xaa, 0x63, 0xfb,
                0x2f, 0x17, 0x2d, 0x2f,
            ],
            Bitcoin::AnnexHash => [
                0x51, 0xfa, 0x19, 0x13, 0xa6, 0x29, 0x73, 0x48, 0x4a, 0x70, 0x0a, 0xf3, 0xff, 0x93,
                0x26, 0x94, 0xd3, 0x89, 0x0a, 0xe0, 0xad, 0x87, 0xa4, 0x55, 0xda, 0xf4, 0x90, 0x6f,
                0x22, 0x4a, 0x48, 0xd5,
            ],
            Bitcoin::Bip0340Verify => [
                0x49, 0x15, 0x65, 0xfe, 0x23, 0xa7, 0xbd, 0xc1, 0x84, 0x2b, 0xe7, 0x49, 0x50, 0x93,
                0x37, 0xf9, 0x68, 0x90, 0xd5, 0xb3, 0x58, 0xb3, 0x65, 0x20, 0x90, 0xda, 0x55, 0x66,
                0x54, 0xe2, 0x95, 0x49,
            ],
            Bitcoin::BuildTapbranch => [
                0xef, 0x8e, 0x92, 0x91, 0x9b, 0x26, 0x08, 0xea, 0x6b, 0x5b, 0xcd, 0xd3, 0x9c, 0x51,
                0x78, 0xe5, 0x46, 0x24, 0x95, 0x53, 0x91, 0x4b, 0x0f, 0x8d, 0x33, 0x73, 0x5d, 0x28,
                0x86, 0xe1, 0xa5, 0xe8,
            ],
            Bitcoin::BuildTapleafSimplicity => [
                0x22, 0x41, 0x19, 0x09, 0x98, 0x41, 0x47, 0xde, 0x8f, 0x5a, 0x04, 0x28, 0x35, 0x8b,
                0x47, 0x16, 0xdb, 0x08, 0x76, 0x64, 0xa7, 0x28, 0x56, 0x08, 0x52, 0xb0, 0xe6, 0x16,
                0xeb, 0xc6, 0x2d, 0x30,
            ],
            Bitcoin::BuildTaptweak => [
                0xf1, 0x92, 0xdb, 0x17, 0x06, 0x60, 0x8d, 0xef, 0x16, 0x5d, 0xbd, 0xda, 0x72, 0xa3,
                0x8c, 0x88, 0x82, 0xcb, 0x36, 0xc6, 0xda, 0x47, 0x07, 0x0d, 0x8a, 0x5f, 0x58, 0x96,
                0xfb, 0xda, 0x84, 0x34,
            ],
            Bitcoin::Ch1 => [
                0x73, 0xb2, 0xa9, 0x81, 0xd7, 0x21, 0x98, 0x6f, 0x8c, 0xde, 0xd6, 0x97, 0xe0, 0x63,
                0x05, 0xd4, 0x58, 0x54, 0x10, 0x2d, 0xff, 0x20, 0xc0, 0xe5, 0xb9, 0x8a, 0xe1, 0x76,
                0x23, 0x2f, 0xf2, 0x5b,
            ],
            Bitcoin::Ch16 => [
                0x78, 0xde, 0x46, 0x5e, 0x61, 0xd9, 0xa5, 0x0f, 0x78, 0x25, 0x2f, 0xf4, 0xab, 0x23,
                0xc9, 0xe6, 0x3a, 0xe6, 0x8c, 0x76, 0x9d, 0x36, 0x66, 0x12, 0x71, 0x20, 0x7d, 0xc6,
                0x93, 0xf4, 0x69, 0xb4,
            ],
            Bitcoin::Ch32 => [
                0xed, 0x93, 0xbe, 0xf1, 0xf6, 0x6e, 0x3a, 0x75, 0xe6, 0x12, 0x06, 0x02, 0xec, 0xee,
                0x67, 0x40, 0x65, 0x3e, 0x7b, 0xd4, 0x6e, 0x07, 0xeb, 0x77, 0x14, 0x4e, 0xf1, 0xbb,
                0x2c, 0x9d, 0xe5, 0x3d,
            ],
            Bitcoin::Ch64 => [
                0xce, 0xd0, 0x07, 0x9b, 0x0b, 0xd1, 0xcc, 0x00, 0x20, 0x9a, 0x7c, 0xbc, 0x23, 0xf1,
                0x3d, 0xfd, 0x20, 0x28, 0x08, 0xf0, 0xf5, 0x25, 0x7d, 0x8a, 0x50, 0xac, 0x54, 0x3e,
                0x64, 0xee, 0x3a, 0x05,
            ],
            Bitcoin::Ch8 => [
                0xc7, 0x07, 0xca, 0x72, 0x3e, 0x24, 0xf6, 0xb2, 0x5b, 0xf3, 0x94, 0xa9, 0x9a, 0x4d,
                0x75, 0xe8, 0x13, 0x79, 0xb4, 0x67, 0x84, 0x38, 0xac, 0x78, 0x9d, 0xee, 0x18, 0x8e,
                0xdc, 0xce, 0x75, 0xfa,
            ],
            Bitcoin::CheckLockDistance => [
                0x38, 0xfd, 0xf7, 0xdd, 0x28, 0x53, 0x86, 0x70, 0xfb, 0x34, 0xc1, 0xbf, 0xe7, 0x2f,
                0x17, 0xe2, 0xca, 0x57, 0x84, 0xf8, 0x7f, 0xed, 0x88, 0xea, 0xb5, 0x84, 0x79, 0x2b,
                0x39, 0x74, 0xbd, 0x18,
            ],
            Bitcoin::CheckLockDuration => [
                0x77, 0x50, 0x38, 0x32, 0x6e, 0xae, 0x25, 0xc7, 0x20, 0x9b, 0x24, 0x43, 0x06, 0xea,
                0xa9, 0xf9, 0x20, 0x4c, 0x7e, 0xce, 0x2d, 0xd6, 0x3b, 0x45, 0x2e, 0x10, 0x01, 0x7f,
                0xa4, 0xea, 0x53, 0xcf,
            ],
            Bitcoin::CheckLockHeight => [
                0xb9, 0x0f, 0x15, 0x1f, 0x45, 0xb4, 0xeb, 0x37, 0x21, 0x06, 0x21, 0xf9, 0x70, 0x0a,
                0x36, 0xc8, 0xc5, 0x04, 0xbe, 0xe0, 0x67, 0x77, 0x11, 0x63, 0xa8, 0xb8, 0x3a, 0x77,
                0x18, 0xe6, 0x68, 0x6a,
            ],
            Bitcoin::CheckLockTime => [
                0xa4, 0x51, 0xeb, 0xb2, 0x25, 0xd6, 0xb1, 0x33, 0xa5, 0xe6, 0x35, 0x39, 0x78, 0x00,
                0xd4, 0x87, 0xb3, 0x96, 0x8b, 0x0d, 0xe5, 0x5c, 0x96, 0xeb, 0x82, 0xf2, 0x90, 0xec,
                0xff, 0x9c, 0x25, 0x90,
            ],
            Bitcoin::CheckSigVerify => [
                0xb5, 0x80, 0x15, 0x54, 0x6d, 0x28, 0x52, 0x66, 0x5d, 0xd2, 0x1b, 0xf1, 0x12, 0x66,
                0x26, 0x70, 0x20, 0xfa, 0x5e, 0x27, 0x50, 0x01, 0xdd, 0x46, 0x18, 0xfa, 0x41, 0x56,
                0x25, 0x95, 0x2e, 0x68,
            ],
            Bitcoin::Complement1 => [
                0x1b, 0xcf, 0xae, 0x13, 0xd5, 0xd2, 0x37, 0xa0, 0xbb, 0x9b, 0x1d, 0x75, 0x32, 0x04,
                0x74, 0x62, 0xb2, 0x76, 0x90, 0xde, 0x5c, 0xac, 0x0e, 0x20, 0x19, 0x29, 0x89, 0x64,
                0x57, 0x93, 0x40, 0x60,
            ],
            Bitcoin::Complement16 => [
                0x81, 0xad, 0x4d, 0x2c, 0x3d, 0x16, 0xbf, 0x34, 0x0a, 0xf3, 0x88, 0x6d, 0x35, 0x5c,
                0xc5, 0xbd, 0x1d, 0x59, 0x67, 0xe1, 0x6a, 0xce, 0x92, 0x4f, 0x19, 0xec, 0xf7, 0xd4,
                0x86, 0xd6, 0xc7, 0xe9,
            ],
            Bitcoin::Complement32 => [
                0x13, 0x74, 0x2c, 0x18, 0x04, 0xa9, 0x6e, 0x6c, 0x03, 0x95, 0x28, 0xbf, 0xd0, 0x3b,
                0x8c, 0xf2, 0xb4, 0x62, 0x52, 0x6b, 0xb1, 0x81, 0xa3, 0xd8, 0xb4, 0x32, 0xf9, 0x9a,
                0xc4, 0xf5, 0xa7, 0xef,
            ],
            Bitcoin::Complement64 => [
                0x65, 0xb7, 0xbd, 0x09, 0x36, 0x39, 0xc5, 0x6d, 0xa2, 0x85, 0xce, 0xfa, 0x2d, 0x04,
                0x64, 0x64, 0x5e, 0x14, 0xdd, 0x13, 0x64, 0x2f, 0x34, 0x95, 0x7d, 0x47, 0x37, 0xbd,
                0x52, 0xfa, 0xc5, 0x88,
            ],
            Bitcoin::Complement8 => [
                0x95, 0x4b, 0x70, 0xdc, 0xec, 0x53, 0x9e, 0x6b, 0x67, 0xdf, 0xfe, 0xc5, 0x3c, 0xf2,
                0x4a, 0x66, 0x99, 0x39, 0x60, 0x8b, 0x22, 0x3f, 0x5f, 0x8b, 0x6d, 0x12, 0x9d, 0xaa,
                0x48, 0xca, 0x1c, 0xf0,
            ],
            Bitcoin::CurrentAnnexHash => [
                0xce, 0xd9, 0x02, 0x2e, 0xdc, 0x69, 0x24, 0x1f, 0xe7, 0x07, 0x49, 0xa7, 0xf5, 0xd4,
                0x89, 0xc3, 0x13, 0x5e, 0xe8, 0xc9, 0x6f, 0xe6, 0x4c, 0x44, 0x64, 0x40, 0x1f, 0x98,
                0x51, 0xd7, 0xa1, 0x7d,
            ],
            Bitcoin::CurrentIndex => [
                0x0e, 0x8c, 0x96, 0x4c, 0x2f, 0x2b, 0x34, 0x90, 0x36, 0x2f, 0x3b, 0xbc, 0x74, 0x83,
                0xde, 0xa3, 0x7f, 0xda, 0x81, 0x0b, 0x69, 0x31, 0x4f, 0xf6, 0x64, 0xfe, 0xa0, 0xe3,
                0x27, 0x08, 0xec, 0x8f,
            ],
            Bitcoin::CurrentPrevOutpoint => [
                0x64, 0x43, 0x39, 0x1b, 0x34, 0x40, 0x86, 0x84, 0x6d, 0x5a, 0x17, 0xa6, 0x2e, 0x3e,
                0x06, 0x28, 0x2a, 0xd6, 0x96, 0x2c, 0x4d, 0xfc, 0xed, 0x2f, 0x6a, 0x83, 0xd0, 0xdf,
                0xbf, 0x6a, 0x5c, 0x54,
            ],
            Bitcoin::CurrentScriptHash => [
                0x23, 0x49, 0x8d, 0xd6, 0x64, 0x5e, 0xd1, 0x38, 0xb3, 0x44, 0x93, 0x7c, 0xf6, 0x54,
                0xaa, 0xff, 0xa6, 0x27, 0xf8, 0x5a, 0x47, 0xca, 0xa6, 0x89, 0x54, 0xf1, 0x3f, 0x4c,
                0x6a, 0x4d, 0xc7, 0x72,
            ],
            Bitcoin::CurrentScriptSigHash => [
                0x34, 0xa3, 0xc5, 0x5d, 0x14, 0x73, 0x74, 0xd8, 0xf3, 0xa1, 0x76, 0x1b, 0xba, 0xb2,
                0x86, 0x96, 0x84, 0x55, 0x40, 0x4c, 0x2c, 0xa0, 0x3f, 0x69, 0x39, 0x3f, 0x63, 0x39,
                0xd0, 0xf5, 0x8b, 0x59,
            ],
            Bitcoin::CurrentSequence => [
                0xc4, 0x9f, 0x76, 0xac, 0x79, 0xf8, 0xf1, 0x5c, 0x40, 0x9b, 0xa8, 0x15, 0xc1, 0x6e,
                0xdc, 0xb8, 0xd1, 0x1e, 0x9a, 0x07, 0x56, 0x5c, 0x8e, 0x09, 0xb6, 0x3e, 0x7f, 0xdf,
                0x31, 0x03, 0x00, 0x9f,
            ],
            Bitcoin::CurrentValue => [
                0x91, 0xb9, 0x6e, 0x82, 0x9e, 0x3b, 0x49, 0x72, 0xb0, 0xcb, 0x09, 0x1a, 0x0a, 0x90,
                0x4b, 0xa4, 0x11, 0x33, 0x8a, 0xbf, 0xc0, 0x8d, 0xa7, 0x86, 0xd5, 0xa8, 0x4f, 0x04,
                0x9b, 0x5b, 0xa3, 0xb8,
            ],
            Bitcoin::Decompress => [
                0x89, 0x00, 0x56, 0xdf, 0x82, 0x8a, 0x76, 0x6e, 0xe9, 0xf6, 0x56, 0x07, 0x22, 0x1e,
                0x89, 0x46, 0xfa, 0x77, 0xc2, 0x56, 0xbb, 0x96, 0xe2, 0x31, 0xe1, 0x94, 0xd3, 0x00,
                0x8c, 0xf3, 0x56, 0xf6,
            ],
            Bitcoin::Decrement16 => [
                0x35, 0xfd, 0xa3, 0x8b, 0x67, 0x2c, 0x38, 0x31, 0xd8, 0xca, 0x11, 0xa4, 0xf3, 0xa9,
                0x59, 0x62, 0x22, 0x52, 0x9e, 0xb1, 0xc1, 0x5f, 0x8c, 0x70, 0x50, 0x13, 0x97, 0x7d,
                0x7d, 0xfb, 0x5d, 0x8b,
            ],
            Bitcoin::Decrement32 => [
                0x3b, 0x2b, 0x19, 0x39, 0x55, 0x22, 0x84, 0xf6, 0x14, 0x69, 0x4b, 0xa1, 0x8d, 0xce,
                0x70, 0xce, 0xe4, 0x76, 0xff, 0x42, 0xdc, 0xd0, 0x89, 0xe1, 0xa3, 0xc0, 0xa4, 0x2b,
                0xeb, 0xd1, 0x08, 0xf6,
            ],
            Bitcoin::Decrement64 => [
                0x7e, 0xf7, 0xbd, 0xd3, 0x5d, 0xb6, 0x85, 0xae, 0x99, 0x05, 0x53, 0x37, 0x35, 0xa2,
                0xc7, 0xa7, 0xcc, 0xbc, 0x17, 0x08, 0xae, 0x63, 0x6f, 0x93, 0x1b, 0x5c, 0xe0, 0x26,
                0xe5, 0xa1, 0x7f, 0xed,
            ],
            Bitcoin::Decrement8 => [
                0xe3, 0x64, 0xf2, 0xe5, 0xc0, 0x8a, 0xe0, 0x11, 0x8e, 0xbe, 0x99, 0x3e, 0x8b, 0x3c,
                0x95, 0x8c, 0x2b, 0xcc, 0x60, 0x62, 0xa3, 0x3b, 0xaa, 0xb9, 0x28, 0xc0, 0x4b, 0x3e,
                0xc9, 0x32, 0xf5, 0x1b,
            ],
            Bitcoin::DivMod128_64 => [
                0x9a, 0x94, 0x43, 0xa2, 0xb5, 0x41, 0xe2, 0x9f, 0x27, 0x2f, 0xfd, 0x56, 0x7d, 0x1b,
                0xf7, 0x42, 0xd6, 0x8c, 0xcb, 0xe9, 0x53, 0x8a, 0x87, 0x29, 0x1b, 0x0c, 0xa6, 0x38,
                0x15, 0x63, 0xac, 0x2c,
            ],
            Bitcoin::DivMod16 => [
                0x39, 0xbc, 0xb5, 0xc0, 0x1d, 0xc1, 0x80, 0x5c, 0x49, 0x19, 0x89, 0x5c, 0xb5, 0x9e,
                0x8f, 0x3b, 0x41, 0x44, 0x67, 0x17, 0xf7, 0xff, 0x48, 0xfd, 0xc9, 0x37, 0xdd, 0x03,
                0x80, 0x24, 0xa0, 0x8a,
            ],
            Bitcoin::DivMod32 => [
                0xfb, 0x12, 0x02, 0xf4, 0xe8, 0x66, 0x3a, 0x87, 0xf5, 0x68, 0x99, 0x2a, 0x18, 0x50,
                0x24, 0xc7, 0x0b, 0x4f, 0x07, 0x9f, 0xbe, 0x95, 0x30, 0x01, 0x0f, 0x6d, 0xb2, 0x84,
                0x21, 0x8a, 0xf6, 0xcd,
            ],
            Bitcoin::DivMod64 => [
                0x67, 0x64, 0xdf, 0x5e, 0x2a, 0xa0, 0x30, 0x32, 0x6e, 0xe5, 0x44, 0xc6, 0xe5, 0x3f,
                0xf3, 0x8e, 0xf0, 0xb2, 0x85, 0x17, 0x91, 0x5e, 0xec, 0x65, 0xc7, 0x2e, 0xa5, 0x7a,
                0x12, 0x98, 0x28, 0xeb,
            ],
            Bitcoin::DivMod8 => [
                0xd3, 0x00, 0x24, 0x4e, 0x48, 0x0d, 0xd9, 0x74, 0x12, 0x13, 0xe4, 0xcb, 0x0e, 0xba,
                0x83, 0x6d, 0x30, 0x59, 0xe7, 0x78, 0xb8, 0x12, 0x2f, 0x78, 0x90, 0x03, 0x26, 0x73,
                0x73, 0x9c, 0x6a, 0x2c,
            ],
            Bitcoin::Divide16 => [
                0x52, 0xab, 0xfe, 0xf1, 0x79, 0x75, 0x4c, 0x90, 0xf9, 0xa4, 0x26, 0x0f, 0x32, 0x3a,
                0x8c, 0xa4, 0x95, 0x15, 0x92, 0x90, 0x2b, 0x8e, 0xcb, 0xd6, 0x4b, 0xa4, 0x26, 0x56,
                0xfa, 0xc0, 0x59, 0x68,
            ],
            Bitcoin::Divide32 => [
                0x4a, 0x8a, 0xe5, 0x35, 0x44, 0xe1, 0x47, 0xed, 0x02, 0x25, 0x04, 0x23, 0x79, 0x34,
                0xcc, 0x25, 0x44, 0x79, 0xbc, 0xf9, 0x3d, 0xe1, 0xe1, 0x97, 0x4d, 0xda, 0xb3, 0xbb,
                0x51, 0x6e, 0x60, 0x6c,
            ],
            Bitcoin::Divide64 => [
                0xd7, 0x02, 0x5d, 0x05, 0xad, 0xfa, 0xe6, 0x6b, 0x47, 0x10, 0xd0, 0xff, 0x1e, 0x87,
                0xe8, 0x28, 0x15, 0x57, 0x3e, 0x9c, 0xb6, 0x31, 0xb4, 0xc7, 0xd1, 0x3d, 0x2f, 0x1b,
                0xe4, 0xdd, 0x26, 0xd2,
            ],
            Bitcoin::Divide8 => [
                0x40, 0xcd, 0x1d, 0xac, 0xea, 0x24, 0x66, 0x9b, 0x6a, 0x58, 0x9b, 0x61, 0x47, 0x54,
                0x74, 0xaf, 0x31, 0xd1, 0x4f, 0x8d, 0x46, 0x87, 0x70, 0x84, 0x52, 0xd3, 0xdf, 0x37,
                0x30, 0x25, 0x31, 0x26,
            ],
            Bitcoin::Divides16 => [
                0x10, 0xbb, 0x18, 0x18, 0x0e, 0xab, 0x5b, 0xad, 0xdc, 0x16, 0x5d, 0x03, 0x37, 0xc4,
                0xad, 0xa0, 0x88, 0xe1, 0x57, 0xb1, 0xaa, 0x67, 0x83, 0x34, 0x2a, 0x45, 0x20, 0xa3,
                0x24, 0xdd, 0x9d, 0x2b,
            ],
            Bitcoin::Divides32 => [
                0xf5, 0xe8, 0xe7, 0x8c, 0x82, 0x76, 0x9a, 0x48, 0xc9, 0x10, 0x3e, 0x44, 0xdd, 0xb4,
                0x7f, 0x84, 0x1d, 0x76, 0x93, 0xb0, 0x41, 0x9e, 0x5e, 0x7d, 0xa4, 0xe6, 0x8b, 0x78,
                0xb2, 0x37, 0xa5, 0x72,
            ],
            Bitcoin::Divides64 => [
                0x9e, 0xbd, 0x55, 0xfa, 0xe4, 0x18, 0x88, 0x5e, 0xea, 0x04, 0xc3, 0xcd, 0xff, 0xf5,
                0x31, 0xb7, 0xd7, 0x14, 0xd0, 0x59, 0x4f, 0xa7, 0xda, 0x87, 0xeb, 0x65, 0x55, 0xd3,
                0x6b, 0x95, 0x3d, 0xb2,
            ],
            Bitcoin::Divides8 => [
                0xa2, 0x36, 0xbc, 0x3e, 0x5c, 0xf4, 0xd2, 0x56, 0x40, 0x8b, 0xa3, 0x8c, 0x1e, 0xae,
                0xe7, 0x36, 0x9a, 0x9c, 0x40, 0x2f, 0x74, 0xbc, 0xd1, 0xc8, 0x02, 0xf9, 0x09, 0x4f,
                0xbf, 0x36, 0x80, 0x3d,
            ],
            Bitcoin::Eq1 => [
                0x65, 0x49, 0xf9, 0x86, 0x20, 0x3a, 0x64, 0x97, 0x35, 0x6e, 0x43, 0x2b, 0x2a, 0xa1,
                0x60, 0xd6, 0xee, 0x87, 0x0b, 0x11, 0x19, 0x08, 0x65, 0xbd, 0x36, 0xa4, 0x7c, 0xb0,
                0x47, 0x04, 0x33, 0xa5,
            ],
            Bitcoin::Eq16 => [
                0x0c, 0x54, 0x02, 0xb0, 0xad, 0xc8, 0xfc, 0x65, 0x70, 0x1b, 0xb7, 0x5b, 0x32, 0x54,
                0xc8, 0x35, 0xf8, 0xfe, 0xc1, 0x30, 0x81, 0xcd, 0x35, 0xe1, 0x32, 0x8f, 0x2b, 0xd7,
                0xdb, 0xd2, 0x3f, 0xa6,
            ],
            Bitcoin::Eq256 => [
                0x26, 0x0e, 0x1d, 0x13, 0x6d, 0xd7, 0x44, 0xfc, 0xb0, 0x50, 0x7a, 0x2d, 0x27, 0x70,
                0x27, 0xa7, 0x72, 0x43, 0x54, 0xeb, 0x17, 0x6b, 0x2f, 0xbf, 0x31, 0xc6, 0xc7, 0xd7,
                0xfb, 0x3e, 0xcd, 0x6f,
            ],
            Bitcoin::Eq32 => [
                0xf5, 0xd6, 0xed, 0xc8, 0xb6, 0x16, 0x4e, 0x12, 0x5b, 0xbb, 0xef, 0x08, 0xc9, 0xe0,
                0x8a, 0x1e, 0x6f, 0xd4, 0x92, 0xf5, 0xbd, 0xca, 0x6f, 0xdc, 0x8b, 0x5f, 0x5a, 0x6f,
                0x05, 0xc5, 0xab, 0x96,
            ],
            Bitcoin::Eq64 => [
                0x1f, 0x93, 0xac, 0xb8, 0x09, 0x2f, 0xa0, 0x6d, 0xea, 0xf3, 0xc3, 0x87, 0xf5, 0x4a,
                0x18, 0xff, 0xea, 0xa6, 0x9a, 0x47, 0xa6, 0xf5, 0xca, 0xf4, 0xae, 0x49, 0x7e, 0x5c,
                0xc2, 0xb3, 0x6c, 0x43,
            ],
            Bitcoin::Eq8 => [
                0xd7, 0x52, 0xfa, 0x7f, 0x51, 0x47, 0x30, 0x14, 0xeb, 0xb6, 0x9e, 0x1e, 0x1d, 0x2c,
                0x86, 0xd5, 0x11, 0x48, 0xb6, 0xba, 0xa0, 0x21, 0x37, 0xa4, 0x8f, 0x62, 0xd5, 0x7e,
                0xaf, 0x8d, 0xf1, 0xcd,
            ],
            Bitcoin::FeAdd => [
                0xa6, 0xc9, 0x0e, 0x02, 0xfd, 0xe4, 0xee, 0x6e, 0xef, 0x66, 0x67, 0x37, 0x49, 0x2e,
                0x14, 0xaf, 0xc8, 0x76, 0x25, 0x04, 0x97, 0x4a, 0xf5, 0xd5, 0x47, 0x2b, 0xb9, 0x43,
                0x3a, 0xd2, 0xd2, 0x94,
            ],
            Bitcoin::FeInvert => [
                0x7c, 0x4a, 0xba, 0xce, 0x33, 0xc7, 0x2b, 0x3b, 0xe1, 0xfd, 0x0e, 0xe3, 0x9f, 0xc6,
                0xcb, 0x3e, 0xe5, 0xc8, 0xf1, 0x1e, 0xf2, 0x19, 0x98, 0xc0, 0x60, 0x2b, 0x52, 0x15,
                0xaa, 0x2a, 0x75, 0xc2,
            ],
            Bitcoin::FeIsOdd => [
                0x30, 0xf5, 0x17, 0x1f, 0x58, 0xf1, 0x08, 0x9d, 0x5d, 0xcf, 0xb6, 0xe6, 0x68, 0x3f,
                0x5a, 0xde, 0x98, 0x4c, 0x07, 0x99, 0x76, 0x3c, 0xa7, 0x38, 0x3f, 0x75, 0xdf, 0x1c,
                0xa0, 0x81, 0x3e, 0xfe,
            ],
            Bitcoin::FeIsZero => [
                0xb0, 0xb7, 0x4d, 0x86, 0x51, 0xff, 0x55, 0x7c, 0xa9, 0x60, 0x44, 0xdd, 0x97, 0x28,
                0x13, 0x38, 0xa8, 0xf7, 0xd3, 0xac, 0xb3, 0x84, 0x7d, 0x03, 0xac, 0xbf, 0x3d, 0x32,
                0xd9, 0x6f, 0xae, 0x55,
            ],
            Bitcoin::FeMultiply => [
                0x50, 0x6b, 0x93, 0x19, 0xc1, 0x7a, 0x14, 0xa9, 0x46, 0x9d, 0x46, 0x27, 0x61, 0xa3,
                0x30, 0x3a, 0xb4, 0x7d, 0xdb, 0x3a, 0x30, 0x79, 0xfb, 0xa3, 0x40, 0x73, 0xaa, 0x55,
                0x42, 0x16, 0xa3, 0x88,
            ],
            Bitcoin::FeMultiplyBeta => [
                0x6e, 0x18, 0x0e, 0xea, 0xbe, 0x84, 0x22, 0xb7, 0x99, 0x68, 0xe7, 0x11, 0xdd, 0x00,
                0xa4, 0xb6, 0x57, 0x8b, 0xb2, 0x75, 0xbe, 0xf4, 0x7f, 0xe5, 0xff, 0x96, 0x8f, 0x14,
                0x72, 0xd7, 0x6f, 0x2a,
            ],
            Bitcoin::FeNegate => [
                0xd4, 0x37, 0xea, 0x00, 0x33, 0x98, 0x80, 0xb3, 0x83, 0xd8, 0x5f, 0xb2, 0xae, 0xaf,
                0x20, 0x1b, 0xbe, 0x8f, 0xfc, 0x83, 0x70, 0x50, 0x62, 0xf9, 0xc9, 0x68, 0x59, 0x0d,
                0x5d, 0xb3, 0x37, 0xf6,
            ],
            Bitcoin::FeNormalize => [
                0xec, 0x0c, 0x3d, 0xd9, 0xc5, 0x28, 0x63, 0x64, 0x78, 0xbe, 0xc0, 0xe1, 0x60, 0xe5,
                0x0a, 0xd9, 0xbf, 0x45, 0x2c, 0x5b, 0x6f, 0x84, 0xe9, 0x40, 0xe1, 0x65, 0x84, 0xeb,
                0x08, 0x5a, 0xce, 0x38,
            ],
            Bitcoin::FeSquare => [
                0xb9, 0x04, 0x77, 0x2d, 0x74, 0xa1, 0x85, 0xb8, 0x28, 0xeb, 0x15, 0x47, 0x28, 0xd2,
                0x49, 0xc5, 0x08, 0x47, 0x11, 0xe9, 0xa1, 0x83, 0x2b, 0x89, 0xca, 0xf2, 0xaf, 0x59,
                0xf9, 0x60, 0xe1, 0x18,
            ],
            Bitcoin::FeSquareRoot => [
                0x16, 0xfb, 0x9a, 0xce, 0xbe, 0x8b, 0x5b, 0x87, 0xf2, 0xea, 0x7d, 0xb6, 0xaa, 0x3a,
                0x2a, 0xf8, 0x8c, 0xa2, 0xb5, 0x8f, 0x02, 0xcd, 0xc8, 0x7e, 0x7c, 0xe6, 0xbe, 0x0c,
                0x1f, 0xfc, 0xe0, 0x14,
            ],
            Bitcoin::Fee => [
                0xfb, 0x9c, 0xa9, 0x39, 0x81, 0x67, 0x3d, 0x1d, 0x23, 0xae, 0xd2, 0x4d, 0x61, 0x2c,
                0x1f, 0x5d, 0xc7, 0xcd, 0x49, 0xf8, 0x6d, 0x34, 0x8e, 0x67, 0xa1, 0x5b, 0xc4, 0xa7,
                0x13, 0x0a, 0xe1, 0x85,
            ],
            Bitcoin::FullAdd16 => [
                0xc5, 0x03, 0xb0, 0x78, 0xdd, 0xe3, 0x99, 0xc6, 0x3a, 0xc4, 0xa2, 0x32, 0xbd, 0x2a,
                0x32, 0x9b, 0x04, 0x30, 0x8c, 0x75, 0xea, 0xec, 0x53, 0xa2, 0xf8, 0x89, 0xb8, 0xdf,
                0x0d, 0x03, 0x34, 0x72,
            ],
            Bitcoin::FullAdd32 => [
                0xa7, 0xaf, 0xd0, 0x40, 0xfc, 0xb0, 0xb2, 0xf2, 0x71, 0x90, 0x78, 0x1a, 0xe5, 0x3a,
                0x6c, 0xca, 0x00, 0xe9, 0xfe, 0x59, 0x53, 0x11, 0x15, 0xc2, 0x58, 0xcc, 0xb6, 0x9d,
                0x3b, 0xe5, 0xa2, 0x13,
            ],
            Bitcoin::FullAdd64 => [
                0x80, 0xa3, 0xef, 0x6c, 0xdb, 0x84, 0xae, 0x7c, 0x8d, 0xbc, 0xf3, 0xa1, 0x84, 0x24,
                0x84, 0xc0, 0x98, 0xdf, 0x6f, 0x19, 0x42, 0x7a, 0x5a, 0x4a, 0xdf, 0xe7, 0x6c, 0xd5,
                0xff, 0x28, 0x36, 0xca,
            ],
            Bitcoin::FullAdd8 => [
                0x4b, 0x90, 0x76, 0xb8, 0xc1, 0xad, 0x56, 0xc9, 0xdb, 0x6b, 0xb3, 0xba, 0xf5, 0x93,
                0x89, 0x54, 0x46, 0xce, 0x61, 0xc7, 0x4f, 0x79, 0x7e, 0xb8, 0xb2, 0x30, 0xd2, 0x05,
                0x42, 0x1c, 0x96, 0x17,
            ],
            Bitcoin::FullDecrement16 => [
                0xfb, 0xa3, 0xc9, 0x78, 0x6e, 0xa3, 0x07, 0xf6, 0xd8, 0x54, 0x34, 0xfd, 0xa2, 0x56,
                0x24, 0x82, 0x43, 0xa0, 0x0b, 0xac, 0x9a, 0x53, 0x53, 0xb6, 0x1e, 0xd3, 0x9c, 0x60,
                0x55, 0xb6, 0x93, 0xb0,
            ],
            Bitcoin::FullDecrement32 => [
                0x62, 0x3d, 0x21, 0xd0, 0x46, 0x79, 0x22, 0xc0, 0x01, 0xc5, 0x65, 0x68, 0x61, 0xd0,
                0xdd, 0xb8, 0x60, 0xc0, 0xc9, 0xa8, 0x6b, 0xd4, 0xcf, 0xdc, 0x37, 0xa1, 0x4c, 0x14,
                0x06, 0xe3, 0x44, 0x6e,
            ],
            Bitcoin::FullDecrement64 => [
                0x14, 0x8b, 0x3e, 0xe1, 0xf7, 0x49, 0xea, 0x0b, 0xfb, 0xa7, 0x63, 0xbe, 0xe9, 0x99,
                0xa2, 0x96, 0x77, 0x45, 0x6e, 0xae, 0x9e, 0xf5, 0x3a, 0xd8, 0x78, 0xf8, 0xb6, 0x14,
                0x94, 0xf0, 0x8f, 0x00,
            ],
            Bitcoin::FullDecrement8 => [
                0xb4, 0x1a, 0xfe, 0x97, 0x4e, 0xaa, 0x11, 0x82, 0xac, 0x46, 0x10, 0x52, 0x1e, 0x28,
                0x27, 0x81, 0x31, 0x8c, 0xe2, 0x95, 0xa3, 0xf2, 0x3f, 0x0b, 0x87, 0x6a, 0xe2, 0x69,
                0x67, 0x3f, 0xb1, 0xdf,
            ],
            Bitcoin::FullIncrement16 => [
                0xa6, 0x8e, 0xcc, 0xdb, 0x9e, 0xad, 0x29, 0x26, 0xc3, 0xe4, 0x5b, 0x4b, 0xae, 0x43,
                0x1c, 0xc4, 0x66, 0xd5, 0x8b, 0x8f, 0xac, 0xc9, 0x5a, 0x1b, 0x48, 0x44, 0xb9, 0x12,
                0xdf, 0x56, 0x76, 0xdf,
            ],
            Bitcoin::FullIncrement32 => [
                0xd0, 0xeb, 0x0e, 0x94, 0xa5, 0xc2, 0x57, 0x13, 0xeb, 0x94, 0x4c, 0xad, 0x4d, 0x70,
                0x1c, 0x6a, 0x96, 0x88, 0x09, 0xbc, 0x1a, 0xf9, 0x03, 0xfd, 0xb1, 0x1e, 0x6f, 0xad,
                0x0b, 0xb3, 0x1b, 0x10,
            ],
            Bitcoin::FullIncrement64 => [
                0xc0, 0x03, 0xd2, 0xe9, 0xb0, 0xa5, 0x10, 0xc2, 0xdd, 0x78, 0x3e, 0x7d, 0x64, 0xeb,
                0x87, 0xb3, 0x38, 0x55, 0xd3, 0x29, 0x90, 0xdf, 0xc2, 0x86, 0x26, 0x6e, 0x47, 0x8d,
                0xa4, 0xe7, 0x47, 0x91,
            ],
            Bitcoin::FullIncrement8 => [
                0x0b, 0xea, 0x24, 0x29, 0x18, 0xf2, 0xdd, 0x17, 0x64, 0x77, 0x78, 0x11, 0xe4, 0x44,
                0x28, 0x63, 0x93, 0x52, 0x25, 0xb0, 0xf8, 0xb2, 0x39, 0xc2, 0x37, 0x52, 0xf9, 0xd8,
                0x53, 0x92, 0xa1, 0x39,
            ],
            Bitcoin::FullLeftShift16_1 => [
                0xb3, 0x66, 0xa8, 0x16, 0x92, 0x2f, 0xc4, 0x55, 0x01, 0x0f, 0xe8, 0x8a, 0x5f, 0x6a,
                0x5c, 0xf2, 0xce, 0xa9, 0x17, 0xe1, 0x2b, 0xd1, 0x40, 0xae, 0x6d, 0x43, 0xb6, 0x41,
                0xe5, 0x7f, 0x42, 0xb3,
            ],
            Bitcoin::FullLeftShift16_2 => [
                0x27, 0x96, 0x0d, 0x0d, 0xf2, 0xfb, 0xbc, 0x38, 0x99, 0x3d, 0x86, 0xfb, 0x8f, 0x0c,
                0xd2, 0xc3, 0x43, 0x4e, 0xdb, 0x11, 0x04, 0x82, 0x13, 0xc1, 0x41, 0x18, 0x93, 0xca,
                0x99, 0x33, 0xb2, 0xee,
            ],
            Bitcoin::FullLeftShift16_4 => [
                0x65, 0x51, 0x37, 0xbe, 0xc5, 0xc0, 0x36, 0x8f, 0x29, 0xbc, 0x99, 0x2c, 0x88, 0x42,
                0x1a, 0x15, 0x98, 0x56, 0x40, 0x39, 0x7b, 0x61, 0x7f, 0xc4, 0x8d, 0x33, 0x21, 0x0f,
                0xc0, 0x05, 0x3a, 0xd1,
            ],
            Bitcoin::FullLeftShift16_8 => [
                0x16, 0x8f, 0x57, 0x6a, 0xa5, 0x6e, 0xa4, 0x7e, 0x07, 0x06, 0x46, 0xe7, 0x88, 0x96,
                0xbe, 0xb2, 0x49, 0x8b, 0x1a, 0xe6, 0xb1, 0xff, 0x9c, 0x78, 0x62, 0x70, 0xe9, 0x55,
                0x65, 0x84, 0x19, 0x29,
            ],
            Bitcoin::FullLeftShift32_1 => [
                0xd7, 0xcd, 0x52, 0x24, 0x49, 0x11, 0x8e, 0x81, 0x00, 0xa7, 0x66, 0x2f, 0x4d, 0xf0,
                0x39, 0xf8, 0xca, 0xeb, 0xf4, 0x33, 0xeb, 0x03, 0x9e, 0xdc, 0x42, 0xe8, 0x82, 0x37,
                0x92, 0xcc, 0xea, 0x8a,
            ],
            Bitcoin::FullLeftShift32_16 => [
                0x8b, 0xd8, 0x0d, 0x4d, 0x2f, 0x8b, 0x22, 0x46, 0xc1, 0x23, 0x15, 0xc4, 0x28, 0x41,
                0xb4, 0xe4, 0x0a, 0x71, 0xae, 0x76, 0x96, 0x6a, 0x08, 0x95, 0x4d, 0x66, 0x6b, 0x86,
                0x32, 0x86, 0x74, 0x37,
            ],
            Bitcoin::FullLeftShift32_2 => [
                0x13, 0x06, 0x3d, 0x62, 0x93, 0x83, 0x29, 0x31, 0x1f, 0xb7, 0xda, 0xbb, 0x15, 0xc3,
                0xfe, 0x58, 0xc2, 0x88, 0x76, 0x83, 0x00, 0x97, 0xec, 0xc6, 0xbf, 0xdd, 0x48, 0x0b,
                0xe1, 0x98, 0x81, 0x46,
            ],
            Bitcoin::FullLeftShift32_4 => [
                0x25, 0xa1, 0xb5, 0xdd, 0xe5, 0xdb, 0x28, 0x4e, 0x8a, 0x88, 0x21, 0x26, 0x7c, 0x26,
                0x53, 0x01, 0x14, 0xbb, 0xe6, 0x71, 0xcf, 0xaf, 0xb4, 0x4a, 0x60, 0xd7, 0x50, 0x27,
                0x67, 0xdb, 0x78, 0x2e,
            ],
            Bitcoin::FullLeftShift32_8 => [
                0xce, 0x54, 0x70, 0xaf, 0xbf, 0xad, 0xbf, 0xba, 0x68, 0xf9, 0xb2, 0xd5, 0xb0, 0x64,
                0x5a, 0x44, 0x08, 0xbe, 0x6f, 0x85, 0xa6, 0x9c, 0x7f, 0x09, 0xd0, 0x96, 0x45, 0x53,
                0x68, 0x04, 0x87, 0x64,
            ],
            Bitcoin::FullLeftShift64_1 => [
                0x05, 0x1f, 0x36, 0x05, 0x86, 0xc3, 0x79, 0xac, 0x2c, 0xe3, 0x99, 0xcb, 0xeb, 0x68,
                0x7e, 0x77, 0x53, 0xb1, 0x5d, 0x73, 0x03, 0xdd, 0x31, 0x6c, 0xbd, 0x12, 0x30, 0x12,
                0x08, 0x7c, 0xc6, 0x6f,
            ],
            Bitcoin::FullLeftShift64_16 => [
                0xb2, 0x48, 0xbe, 0x4d, 0xfc, 0xb8, 0x8c, 0x5d, 0x89, 0xb1, 0xca, 0x61, 0x86, 0xa0,
                0x41, 0xe9, 0x02, 0xb4, 0xc8, 0xa6, 0x2b, 0xb1, 0x6e, 0x09, 0xfe, 0x15, 0x61, 0x6e,
                0x0e, 0x3a, 0xbd, 0x6d,
            ],
            Bitcoin::FullLeftShift64_2 => [
                0x34, 0xbb, 0x51, 0x62, 0x6b, 0x1d, 0x6b, 0x89, 0x7a, 0xbc, 0x15, 0x5d, 0x03, 0x4f,
                0xe0, 0x66, 0x3a, 0x0e, 0xc0, 0xfd, 0x8f, 0x64, 0x0e, 0x5f, 0xe1, 0xbf, 0x3c, 0xb7,
                0x67, 0x0a, 0x29, 0x25,
            ],
            Bitcoin::FullLeftShift64_32 => [
                0x9d, 0xac, 0x8c, 0xd7, 0xfd, 0x8b, 0x48, 0x88, 0x9e, 0x55, 0xc5, 0xaa, 0x12, 0xfe,
                0x97, 0xb7, 0x29, 0xfe, 0xbc, 0x04, 0x1a, 0x9f, 0xff, 0x44, 0xc4, 0xd9, 0xb6, 0xf1,
                0xe0, 0x7e, 0xb4, 0x42,
            ],
            Bitcoin::FullLeftShift64_4 => [
                0x94, 0xb7, 0x3d, 0xad, 0xc3, 0xee, 0xeb, 0x2e, 0xe4, 0xa4, 0xd4, 0x44, 0xdd, 0x0f,
                0x72, 0xac, 0x30, 0x62, 0x01, 0xf2, 0xff, 0xcf, 0x71, 0x4b, 0x8e, 0xbe, 0x79, 0x82,
                0x74, 0x4c, 0x0c, 0x7e,
            ],
            Bitcoin::FullLeftShift64_8 => [
                0x0e, 0xf7, 0x14, 0x75, 0x7e, 0xcf, 0x11, 0xca, 0x3c, 0x73, 0xce, 0x25, 0xef, 0x24,
                0xee, 0x72, 0x95, 0xdd, 0x41, 0x71, 0xcc, 0x16, 0x28, 0x7f, 0xe6, 0x97, 0x1b, 0xd6,
                0x7b, 0x47, 0x8b, 0x46,
            ],
            Bitcoin::FullLeftShift8_1 => [
                0x9b, 0xfa, 0x48, 0xb7, 0xad, 0x51, 0x00, 0x91, 0xba, 0x60, 0x85, 0x44, 0x62, 0xd8,
                0x59, 0xef, 0xd1, 0xa2, 0xaa, 0x18, 0x73, 0x1b, 0x5f, 0x0c, 0x9e, 0x2b, 0xa4, 0xd8,
                0x9d, 0x3a, 0xa8, 0x43,
            ],
            Bitcoin::FullLeftShift8_2 => [
                0x79, 0x7c, 0x20, 0x87, 0x01, 0xb2, 0xa4, 0xe1, 0x04, 0x9e, 0x83, 0xd6, 0x95, 0xf5,
                0x54, 0xb9, 0x84, 0xf5, 0xdb, 0x28, 0x21, 0x52, 0x55, 0x58, 0x7c, 0x37, 0x34, 0x21,
                0x51, 0xb7, 0x24, 0x1d,
            ],
            Bitcoin::FullLeftShift8_4 => [
                0x37, 0xbd, 0xac, 0x91, 0x53, 0x8f, 0x22, 0x19, 0xcb, 0x89, 0xdf, 0x0e, 0xf9, 0xf1,
                0x97, 0xcd, 0x68, 0x03, 0x1f, 0x27, 0x67, 0xe8, 0x94, 0xf0, 0x01, 0xc2, 0x6f, 0xff,
                0x5e, 0xeb, 0x58, 0xcd,
            ],
            Bitcoin::FullMultiply16 => [
                0x09, 0xba, 0xff, 0x92, 0x1e, 0x9f, 0x14, 0xd1, 0x20, 0x8d, 0x1d, 0xd8, 0x26, 0x4c,
                0xf1, 0xf3, 0xb8, 0x54, 0xc9, 0xaf, 0x21, 0xf7, 0x78, 0xb2, 0xb5, 0x5a, 0x8a, 0x42,
                0x6d, 0xfe, 0x89, 0x28,
            ],
            Bitcoin::FullMultiply32 => [
                0x10, 0xc4, 0xb8, 0xc4, 0xc0, 0xac, 0xd9, 0x73, 0x90, 0xf8, 0x5c, 0xb3, 0xf5, 0xff,
                0xe3, 0x6a, 0x29, 0x20, 0x37, 0xc1, 0x90, 0xee, 0xba, 0xb3, 0xe9, 0x89, 0x34, 0xfe,
                0x93, 0xb2, 0xed, 0x90,
            ],
            Bitcoin::FullMultiply64 => [
                0x2d, 0xb1, 0x9d, 0xba, 0x90, 0xef, 0x86, 0x7b, 0x5a, 0x3e, 0x91, 0x4b, 0x89, 0xfd,
                0xa2, 0xda, 0x63, 0x7c, 0xa8, 0x0c, 0x42, 0x67, 0xe1, 0x98, 0x18, 0x37, 0xee, 0x3c,
                0x6f, 0xe3, 0xda, 0xf5,
            ],
            Bitcoin::FullMultiply8 => [
                0x7f, 0x46, 0xee, 0x72, 0x84, 0xf3, 0x9e, 0x73, 0x42, 0x75, 0xa2, 0x50, 0x9a, 0x0b,
                0x73, 0x7e, 0xd9, 0x39, 0x11, 0x5f, 0x02, 0x19, 0xa5, 0x74, 0xd4, 0x69, 0xcd, 0x30,
                0xb8, 0x19, 0xef, 0xe3,
            ],
            Bitcoin::FullRightShift16_1 => [
                0x7e, 0xbe, 0x0c, 0x66, 0xc3, 0xc7, 0xdc, 0x16, 0xa5, 0x46, 0x9e, 0x91, 0x79, 0x09,
                0x84, 0x17, 0xac, 0x5f, 0x20, 0xa3, 0x9c, 0xc4, 0x1a, 0xc3, 0x82, 0xfb, 0x1d, 0xbd,
                0x98, 0xe8, 0xe3, 0x0f,
            ],
            Bitcoin::FullRightShift16_2 => [
                0x8d, 0xb0, 0xc2, 0x16, 0x19, 0xc6, 0x2d, 0x63, 0xd4, 0xc2, 0x7b, 0xfc, 0xf6, 0x47,
                0xd7, 0x09, 0xce, 0x37, 0xbe, 0xd0, 0x57, 0x18, 0xe9, 0x3e, 0x45, 0x15, 0xe2, 0x9e,
                0xf3, 0x73, 0x0c, 0xf4,
            ],
            Bitcoin::FullRightShift16_4 => [
                0x5c, 0x74, 0xb1, 0x32, 0x06, 0x31, 0x79, 0x17, 0xe0, 0x70, 0xe5, 0xfc, 0x1c, 0x82,
                0xf4, 0xc5, 0xc2, 0xfb, 0xe9, 0xf3, 0x1b, 0x81, 0x29, 0x46, 0xba, 0x23, 0x0d, 0x8c,
                0x94, 0xd4, 0x06, 0x16,
            ],
            Bitcoin::FullRightShift16_8 => [
                0x11, 0x05, 0x81, 0x8a, 0xc9, 0x48, 0xd7, 0xbb, 0x63, 0x47, 0x07, 0xe6, 0x9d, 0xbf,
                0x1f, 0x67, 0x90, 0x58, 0xa1, 0x3d, 0x35, 0xfa, 0xc2, 0xa6, 0x4d, 0xf9, 0x72, 0x62,
                0xf2, 0x42, 0xb6, 0x3b,
            ],
            Bitcoin::FullRightShift32_1 => [
                0x9b, 0x42, 0xc8, 0xf3, 0x3b, 0xc5, 0x75, 0x0e, 0x2a, 0x83, 0xaa, 0xdb, 0xf2, 0x9c,
                0xc7, 0xfc, 0xb9, 0x50, 0xfe, 0x5a, 0x40, 0xaa, 0x0e, 0xc5, 0x24, 0x52, 0xe5, 0x33,
                0xf8, 0x25, 0xa1, 0x15,
            ],
            Bitcoin::FullRightShift32_16 => [
                0x0a, 0xe5, 0x65, 0x9c, 0x2f, 0xa7, 0x57, 0x94, 0x78, 0xeb, 0xd5, 0x7c, 0x4c, 0x98,
                0xae, 0xe7, 0x77, 0x01, 0x56, 0x45, 0xb2, 0x84, 0x31, 0x81, 0x64, 0xfc, 0xbd, 0x30,
                0x65, 0xfc, 0x87, 0x3c,
            ],
            Bitcoin::FullRightShift32_2 => [
                0x57, 0xfb, 0x1c, 0x03, 0xc2, 0xeb, 0x17, 0xf6, 0x23, 0x47, 0x87, 0x34, 0xfd, 0x69,
                0x37, 0xf9, 0xe3, 0xef, 0x02, 0x7c, 0x15, 0x60, 0x03, 0x8f, 0xa6, 0x06, 0x69, 0x05,
                0x17, 0x89, 0xe3, 0x68,
            ],
            Bitcoin::FullRightShift32_4 => [
                0x85, 0x82, 0xcd, 0xfa, 0x74, 0xef, 0x46, 0x6b, 0x81, 0x27, 0xb1, 0x97, 0x88, 0x13,
                0x45, 0x93, 0x99, 0x8e, 0x49, 0x69, 0x00, 0xb3, 0x8f, 0x0f, 0x3d, 0x37, 0x58, 0x18,
                0xd6, 0x73, 0x45, 0x1e,
            ],
            Bitcoin::FullRightShift32_8 => [
                0xd9, 0x05, 0x93, 0x2e, 0xbf, 0xca, 0x2a, 0x38, 0x61, 0x9d, 0x80, 0x7e, 0x28, 0xff,
                0x2e, 0x0d, 0x3b, 0xe0, 0x8a, 0x26, 0x06, 0x76, 0xd2, 0x57, 0xef, 0xa0, 0x40, 0xc3,
                0x05, 0xaa, 0xdc, 0x33,
            ],
            Bitcoin::FullRightShift64_1 => [
                0x3c, 0x15, 0x20, 0x9b, 0x99, 0xd2, 0x84, 0x5e, 0x22, 0x5e, 0x14, 0xe1, 0xe9, 0xe5,
                0xe6, 0xa4, 0x87, 0x8b, 0xc8, 0xce, 0xa3, 0xf9, 0xf3, 0x6b, 0x8b, 0x53, 0x5a, 0xc6,
                0x83, 0xe2, 0x9d, 0x00,
            ],
            Bitcoin::FullRightShift64_16 => [
                0x02, 0x85, 0x25, 0x7b, 0x09, 0x0d, 0x8d, 0xa1, 0x28, 0xef, 0x64, 0xa8, 0x0c, 0x8d,
                0x16, 0xfd, 0xc3, 0xbf, 0x5c, 0xe5, 0x0f, 0xcd, 0x56, 0xfe, 0xc5, 0xf9, 0x02, 0x55,
                0xd9, 0xc8, 0xdf, 0x47,
            ],
            Bitcoin::FullRightShift64_2 => [
                0x7e, 0xc2, 0xdd, 0x65, 0xc9, 0xe0, 0x13, 0xe3, 0xe4, 0xce, 0x90, 0xfb, 0xeb, 0x3f,
                0xb1, 0xc7, 0x8c, 0xcc, 0x5d, 0x2a, 0x7d, 0x26, 0xd8, 0xaf, 0x77, 0xf9, 0x9d, 0xe8,
                0x4c, 0xf7, 0x29, 0x73,
            ],
            Bitcoin::FullRightShift64_32 => [
                0x35, 0x6f, 0x7d, 0xd4, 0x6b, 0xa3, 0x3f, 0x84, 0xb0, 0x66, 0x72, 0xfd, 0xe9, 0xa2,
                0x97, 0x2e, 0x80, 0xf3, 0xea, 0x96, 0x5a, 0xe8, 0xbc, 0x0b, 0xff, 0x67, 0xaa, 0x2f,
                0x69, 0xf1, 0x0b, 0x56,
            ],
            Bitcoin::FullRightShift64_4 => [
                0x05, 0x46, 0x4a, 0x33, 0x35, 0xaf, 0xbb, 0x09, 0xd0, 0x46, 0x82, 0x8a, 0x92, 0x2c,
                0x4d, 0xa0, 0xec, 0xee, 0xb1, 0x09, 0x77, 0xe4, 0x68, 0x01, 0xc9, 0x3c, 0xdd, 0x66,
                0x8f, 0x22, 0xee, 0x63,
            ],
            Bitcoin::FullRightShift64_8 => [
                0x70, 0x17, 0x2e, 0x1a, 0x69, 0x48, 0xbf, 0x40, 0x12, 0x0e, 0x68, 0xfb, 0x8b, 0x4b,
                0x23, 0xbc, 0x35, 0x5a, 0x12, 0x00, 0x2c, 0xcc, 0x1d, 0xb6, 0x47, 0xc8, 0x9b, 0x12,
                0xd1, 0x0e, 0xc5, 0x06,
            ],
            Bitcoin::FullRightShift8_1 => [
                0x56, 0x69, 0xdb, 0xfc, 0xc6, 0x33, 0xec, 0x0b, 0xdf, 0x59, 0xe2, 0x2f, 0x03, 0xed,
                0x4b, 0x64, 0x19, 0x20, 0x95, 0xf5, 0xdf, 0x20, 0xff, 0xc1, 0x2d, 0xd9, 0x0d, 0x7c,
                0xda, 0x11, 0x37, 0x4f,
            ],
            Bitcoin::FullRightShift8_2 => [
                0x1f, 0x94, 0x43, 0x61, 0x09, 0xdf, 0x52, 0xb3, 0x45, 0xfa, 0x3a, 0x89, 0xac, 0x2a,
                0x49, 0xed, 0xc9, 0xd2, 0x85, 0xf2, 0x1f, 0x45, 0xed, 0x11, 0xd7, 0x75, 0xf7, 0xf7,
                0xf3, 0x9d, 0x3e, 0x8f,
            ],
            Bitcoin::FullRightShift8_4 => [
                0x71, 0x46, 0x98, 0xa2, 0x76, 0x84, 0xb5, 0xba, 0xa6, 0xb6, 0x48, 0x0e, 0xe3, 0xb2,
                0x57, 0xcb, 0xb7, 0xcd, 0xab, 0x74, 0x72, 0xf3, 0x71, 0xa6, 0x27, 0x06, 0x18, 0xc0,
                0xab, 0x12, 0x90, 0x8b,
            ],
            Bitcoin::FullSubtract16 => [
                0x40, 0x09, 0x61, 0x52, 0xb5, 0x4e, 0x74, 0x25, 0x45, 0x55, 0xa6, 0x5d, 0xcc, 0xc6,
                0x29, 0xdf, 0x57, 0xb9, 0x79, 0xc8, 0x47, 0x00, 0x54, 0x50, 0x36, 0xfe, 0x19, 0x0a,
                0x6a, 0xf3, 0xd3, 0x8a,
            ],
            Bitcoin::FullSubtract32 => [
                0xe7, 0x93, 0x0d, 0x64, 0x35, 0xa9, 0x68, 0x0b, 0xef, 0xb4, 0x9d, 0xb7, 0xd8, 0x7c,
                0x2f, 0x50, 0xaf, 0xd4, 0x6d, 0x98, 0x88, 0x0d, 0xed, 0x50, 0xe5, 0x05, 0x5f, 0xa3,
                0x09, 0xe1, 0xaf, 0xca,
            ],
            Bitcoin::FullSubtract64 => [
                0xff, 0x28, 0x1d, 0xf8, 0xc4, 0x2a, 0x31, 0x59, 0xd9, 0xff, 0xa9, 0x25, 0x16, 0xca,
                0x89, 0x3e, 0x23, 0xb0, 0xeb, 0x93, 0x8b, 0x4c, 0xb0, 0xb3, 0xf1, 0x34, 0x46, 0x8e,
                0x9f, 0x4e, 0xbc, 0x46,
            ],
            Bitcoin::FullSubtract8 => [
                0x7e, 0x3d, 0xcf, 0xe4, 0x56, 0xae, 0x3c, 0x5c, 0x87, 0xde, 0xbf, 0x04, 0x71, 0x89,
                0xc2, 0x74, 0x82, 0xa4, 0xff, 0x4e, 0x8c, 0xfd, 0x1f, 0x17, 0x30, 0xc8, 0x7d, 0x2b,
                0x7b, 0xff, 0x73, 0xba,
            ],
            Bitcoin::GeIsOnCurve => [
                0x7d, 0x44, 0x87, 0x19, 0xf5, 0xf9, 0x57, 0x2b, 0xf5, 0x40, 0x2e, 0x12, 0xd1, 0x93,
                0xaf, 0xf6, 0x77, 0x48, 0x2d, 0x66, 0xff, 0x3d, 0xcf, 0x27, 0x48, 0xf2, 0x5c, 0x6b,
                0x73, 0x77, 0x02, 0x8c,
            ],
            Bitcoin::GeNegate => [
                0x3d, 0x2c, 0x8d, 0xe4, 0xc7, 0x01, 0x5f, 0xd3, 0x13, 0x26, 0x95, 0xfd, 0x66, 0xdf,
                0xcf, 0x0f, 0x17, 0x78, 0xc7, 0x91, 0x85, 0x26, 0x8e, 0x9f, 0xae, 0x77, 0x89, 0xda,
                0x53, 0x8e, 0xca, 0x59,
            ],
            Bitcoin::GejAdd => [
                0x45, 0xba, 0x7f, 0x3d, 0x1e, 0x1e, 0x6d, 0x34, 0x9f, 0xcf, 0x86, 0x98, 0x7b, 0x0e,
                0x7f, 0x7a, 0xce, 0x66, 0x2e, 0x82, 0x20, 0x1d, 0x35, 0x02, 0x60, 0x45, 0x4e, 0x2f,
                0xfe, 0xec, 0xb5, 0x4d,
            ],
            Bitcoin::GejDouble => [
                0x23, 0xe9, 0x78, 0xf3, 0x41, 0x54, 0x11, 0x9b, 0xde, 0xfc, 0x5d, 0x13, 0xfc, 0xfd,
                0x0a, 0x34, 0xa7, 0x5e, 0x37, 0x26, 0xd6, 0xcb, 0x25, 0x81, 0x33, 0x70, 0xad, 0x7d,
                0x9d, 0xe5, 0xe1, 0x33,
            ],
            Bitcoin::GejEquiv => [
                0xb9, 0x4b, 0x2a, 0xac, 0x73, 0xa6, 0x7f, 0x44, 0x95, 0x85, 0x99, 0x13, 0x4d, 0xe2,
                0x30, 0x17, 0x9e, 0x9d, 0x6b, 0xb6, 0x47, 0xfd, 0x06, 0x11, 0x15, 0x8a, 0xab, 0xa7,
                0x0b, 0x73, 0xe4, 0x00,
            ],
            Bitcoin::GejGeAdd => [
                0xf1, 0x16, 0x0b, 0x6f, 0x5e, 0xe2, 0xc5, 0x82, 0xe4, 0x95, 0x66, 0xe6, 0xc3, 0x86,
                0xb3, 0x80, 0x94, 0xab, 0xc1, 0xa7, 0x18, 0x2d, 0x33, 0xa1, 0x50, 0x1f, 0xa2, 0xaa,
                0xf0, 0x0a, 0xf3, 0xea,
            ],
            Bitcoin::GejGeAddEx => [
                0xc3, 0xd7, 0x34, 0x7f, 0xfe, 0x2d, 0x9c, 0x83, 0x9a, 0xac, 0x56, 0x7e, 0x29, 0x98,
                0xe0, 0x16, 0xaf, 0x39, 0x4e, 0x2a, 0x19, 0x29, 0x31, 0x4b, 0x52, 0xe3, 0x1e, 0xed,
                0x67, 0x8e, 0x30, 0xbf,
            ],
            Bitcoin::GejGeEquiv => [
                0x27, 0xc2, 0x99, 0x69, 0x13, 0x9f, 0x8d, 0x57, 0xed, 0xc9, 0x89, 0x5c, 0x30, 0x40,
                0x3d, 0xf0, 0x15, 0xc5, 0x0c, 0xe7, 0x21, 0xc3, 0x81, 0xfb, 0x19, 0x7c, 0x0c, 0x04,
                0x03, 0xf1, 0xdb, 0x0c,
            ],
            Bitcoin::GejInfinity => [
                0xaa, 0xfb, 0x93, 0x80, 0xd6, 0x1a, 0x7f, 0x14, 0x78, 0x46, 0x80, 0x6b, 0x2c, 0xc3,
                0x74, 0xfb, 0xe8, 0x2d, 0xd1, 0xae, 0xd4, 0x85, 0xb9, 0x8a, 0x0f, 0x16, 0x4b, 0x3a,
                0x54, 0xc2, 0xc0, 0xb0,
            ],
            Bitcoin::GejIsInfinity => [
                0xdb, 0x49, 0x5f, 0xd1, 0x31, 0x42, 0xe9, 0xb3, 0x37, 0x63, 0xfc, 0x6d, 0x48, 0xd2,
                0xfb, 0x0e, 0x71, 0xb0, 0xd9, 0xd9, 0x9b, 0xd7, 0x26, 0xf4, 0x7a, 0xd1, 0x3f, 0xc5,
                0x56, 0x06, 0x70, 0xa2,
            ],
            Bitcoin::GejIsOnCurve => [
                0xbf, 0x4c, 0xa1, 0x3f, 0xf2, 0x12, 0xe3, 0x4b, 0xf1, 0x7d, 0x90, 0xc1, 0x2e, 0x45,
                0x3d, 0x08, 0xac, 0x7d, 0xaa, 0x4a, 0x47, 0xd5, 0x7e, 0x85, 0xb4, 0x3f, 0x2d, 0x43,
                0x66, 0xd4, 0x3d, 0xda,
            ],
            Bitcoin::GejNegate => [
                0x01, 0xbd, 0x1a, 0x35, 0x1f, 0xb8, 0x16, 0x4c, 0x81, 0x3d, 0x91, 0x6d, 0x07, 0x77,
                0x49, 0x99, 0x6b, 0x7d, 0xb1, 0x18, 0xd3, 0x15, 0x86, 0xca, 0x9d, 0x75, 0xe7, 0x56,
                0x35, 0x18, 0xf4, 0x54,
            ],
            Bitcoin::GejNormalize => [
                0xec, 0x59, 0x7d, 0x17, 0xe2, 0xef, 0xb6, 0xd2, 0xa0, 0x02, 0xd5, 0x0e, 0x67, 0x75,
                0x27, 0xd3, 0xd4, 0xa2, 0x90, 0x7a, 0x11, 0x9d, 0x68, 0xf1, 0x22, 0x84, 0xb9, 0xa1,
                0xb0, 0xd2, 0x30, 0x3a,
            ],
            Bitcoin::GejRescale => [
                0x29, 0x77, 0xd9, 0x53, 0xef, 0x7a, 0x11, 0x56, 0xce, 0xc6, 0xdb, 0x2d, 0xc2, 0x92,
                0x54, 0x12, 0x75, 0xcb, 0xc8, 0x2f, 0xb8, 0x29, 0xfd, 0x67, 0x1b, 0x97, 0x2e, 0x89,
                0xeb, 0xed, 0x0c, 0x24,
            ],
            Bitcoin::GejXEquiv => [
                0xf9, 0xf1, 0x89, 0xfc, 0x00, 0xb6, 0x1f, 0x72, 0xf1, 0x0b, 0xaa, 0xa2, 0x1b, 0xcd,
                0x88, 0xe5, 0xd2, 0x2e, 0x0a, 0xa9, 0xb7, 0x50, 0x9a, 0xe1, 0x62, 0xa1, 0x83, 0xa4,
                0xb6, 0x64, 0xa4, 0xaf,
            ],
            Bitcoin::GejYIsOdd => [
                0x9e, 0xb6, 0xe4, 0x53, 0x5f, 0xb6, 0x9b, 0xf6, 0x09, 0x91, 0x65, 0x99, 0xf1, 0x34,
                0x5a, 0xd7, 0x73, 0x5d, 0xa3, 0xf3, 0x94, 0x8d, 0x06, 0x86, 0x90, 0x8e, 0x44, 0xf4,
                0x5b, 0x2f, 0xf6, 0x0c,
            ],
            Bitcoin::Generate => [
                0x14, 0x88, 0x85, 0xac, 0x73, 0x81, 0x31, 0x13, 0xc5, 0x23, 0xe8, 0x09, 0xbe, 0xa4,
                0x7f, 0xfd, 0x8b, 0x1d, 0xaf, 0x37, 0x8d, 0x9d, 0xd5, 0x4b, 0xf9, 0x66, 0xcc, 0xb8,
                0x83, 0xb1, 0xa9, 0x84,
            ],
            Bitcoin::HashToCurve => [
                0xef, 0x4f, 0x54, 0x8b, 0x3c, 0x6c, 0x75, 0x17, 0x5f, 0x2c, 0xe2, 0xd1, 0x99, 0x3b,
                0x2d, 0x19, 0x9b, 0xeb, 0x16, 0xc0, 0xa1, 0x40, 0x17, 0x5c, 0x48, 0xa1, 0x27, 0x7e,
                0xfc, 0x43, 0xa9, 0x9b,
            ],
            Bitcoin::High1 => [
                0xb1, 0x09, 0xcf, 0x1c, 0xce, 0x35, 0xf7, 0xe9, 0xb6, 0x49, 0x67, 0x1a, 0x9b, 0x45,
                0xdb, 0xc2, 0x40, 0x99, 0xa7, 0x13, 0xae, 0xb9, 0xa8, 0x9c, 0xc4, 0xcf, 0x6e, 0xf6,
                0xed, 0x8b, 0x30, 0x8b,
            ],
            Bitcoin::High16 => [
                0x03, 0x5d, 0xad, 0xd9, 0xd7, 0xbf, 0x74, 0x33, 0x64, 0x45, 0xe7, 0x1d, 0xdc, 0x4d,
                0x82, 0x02, 0x24, 0xff, 0x7e, 0x38, 0xe0, 0xb8, 0xd5, 0x2b, 0xec, 0x97, 0x29, 0xb5,
                0x72, 0xb5, 0x31, 0xf9,
            ],
            Bitcoin::High32 => [
                0xc5, 0xf1, 0xdf, 0x0d, 0x64, 0xa2, 0x73, 0x7a, 0x63, 0x1b, 0x3a, 0xae, 0x8f, 0x26,
                0x0e, 0x8b, 0x8d, 0xc1, 0x95, 0x7b, 0xd0, 0x92, 0x91, 0x1b, 0x91, 0xd2, 0x07, 0x8a,
                0xd2, 0x1e, 0x41, 0x8a,
            ],
            Bitcoin::High64 => [
                0xa3, 0x12, 0x63, 0x3e, 0x0a, 0x23, 0x05, 0xe6, 0x9b, 0x3f, 0x34, 0x1d, 0x91, 0xd6,
                0x83, 0xdd, 0x94, 0x19, 0x6a, 0x2f, 0x90, 0x05, 0xc9, 0xb1, 0x87, 0x2a, 0x2c, 0x15,
                0xad, 0x46, 0xcf, 0x17,
            ],
            Bitcoin::High8 => [
                0xcb, 0xd7, 0x8d, 0x50, 0xaf, 0x77, 0x99, 0x85, 0x5a, 0xdc, 0x49, 0x03, 0xdb, 0xbe,
                0xfc, 0x13, 0x45, 0xd5, 0x14, 0x84, 0xf0, 0x3d, 0x3c, 0x75, 0x5c, 0xaa, 0xa5, 0xca,
                0xa9, 0x7d, 0x4a, 0x14,
            ],
            Bitcoin::Increment16 => [
                0x86, 0x77, 0x49, 0x49, 0x39, 0xb2, 0x7b, 0x86, 0xcb, 0x5a, 0x8c, 0x7f, 0x81, 0x72,
                0xad, 0x55, 0x50, 0x95, 0x31, 0xc9, 0xb0, 0xe1, 0x1e, 0x99, 0x75, 0x7e, 0x29, 0x6c,
                0xc3, 0xc7, 0xc1, 0x92,
            ],
            Bitcoin::Increment32 => [
                0x6b, 0xdb, 0xab, 0x7c, 0xfc, 0x16, 0xc5, 0x03, 0x36, 0x3c, 0x2f, 0x07, 0x7e, 0x02,
                0xc3, 0x35, 0xda, 0x40, 0x61, 0x75, 0xd1, 0x92, 0xfb, 0xef, 0x50, 0xc0, 0x7f, 0xc2,
                0x79, 0xb3, 0xf4, 0x0c,
            ],
            Bitcoin::Increment64 => [
                0x20, 0xe7, 0x5e, 0x71, 0x7c, 0xb7, 0x6d, 0x46, 0x95, 0x56, 0x4f, 0x7c, 0x20, 0x22,
                0x1b, 0x7a, 0x01, 0x43, 0x13, 0x87, 0x38, 0xf1, 0x51, 0xaa, 0x19, 0x5e, 0xb1, 0x70,
                0xec, 0x13, 0xc0, 0x49,
            ],
            Bitcoin::Increment8 => [
                0x5f, 0x4e, 0x05, 0x6e, 0xf4, 0xed, 0x8d, 0x68, 0xbf, 0x91, 0x1f, 0xc5, 0xcb, 0x69,
                0x03, 0x7e, 0xbf, 0x6c, 0x92, 0x21, 0x73, 0x43, 0xa8, 0x90, 0x5d, 0x38, 0xc4, 0x32,
                0xc1, 0x83, 0x23, 0x3c,
            ],
            Bitcoin::InputAnnexHash => [
                0x94, 0x5b, 0x14, 0x7e, 0x5f, 0x0e, 0xdb, 0x83, 0x2d, 0x62, 0x34, 0x8c, 0xae, 0xea,
                0xc2, 0x24, 0x56, 0xee, 0xe9, 0x44, 0x65, 0x37, 0x6d, 0xbf, 0x59, 0x6b, 0x9d, 0x62,
                0x98, 0x5b, 0x01, 0xb6,
            ],
            Bitcoin::InputAnnexesHash => [
                0x89, 0xb6, 0x02, 0x70, 0x44, 0x14, 0x1f, 0x20, 0x65, 0xb6, 0xf2, 0x36, 0xcf, 0xcc,
                0x13, 0xb9, 0x68, 0x48, 0x5e, 0x00, 0x74, 0x6b, 0x78, 0x59, 0x28, 0x69, 0x03, 0xc6,
                0x8c, 0x7f, 0x88, 0x0d,
            ],
            Bitcoin::InputHash => [
                0x3a, 0xd2, 0x22, 0x31, 0xd6, 0xb9, 0xdf, 0xf6, 0xa0, 0xb4, 0xdb, 0xcf, 0xf0, 0x44,
                0xe1, 0x1c, 0x08, 0x2e, 0x04, 0x68, 0x4e, 0x73, 0xce, 0x95, 0xc2, 0x05, 0xe0, 0xc4,
                0x49, 0x6e, 0x5e, 0xca,
            ],
            Bitcoin::InputOutpointsHash => [
                0x17, 0x59, 0xc6, 0xf4, 0x70, 0xb9, 0xab, 0xa6, 0x2a, 0x31, 0x79, 0x1e, 0xa0, 0x10,
                0x56, 0xe3, 0x60, 0x8b, 0xdf, 0x22, 0xf5, 0xdd, 0x43, 0xbf, 0x7d, 0xb0, 0x0e, 0xa5,
                0x82, 0x3f, 0x7c, 0xce,
            ],
            Bitcoin::InputPrevOutpoint => [
                0x5d, 0x17, 0x12, 0x42, 0x09, 0xee, 0x05, 0x21, 0x24, 0xe5, 0x52, 0x38, 0xe0, 0xb6,
                0xa6, 0xfe, 0x85, 0xa4, 0x86, 0x88, 0xc1, 0xe1, 0x5e, 0x61, 0x81, 0xde, 0x94, 0xf7,
                0x8d, 0xb0, 0x40, 0x18,
            ],
            Bitcoin::InputScriptHash => [
                0x05, 0x62, 0x41, 0x2f, 0x02, 0x1d, 0xa4, 0x81, 0x9b, 0x01, 0x9c, 0x8c, 0xd8, 0x87,
                0xd5, 0x30, 0x49, 0x2b, 0x9f, 0x94, 0x2c, 0x4e, 0xbf, 0x21, 0xae, 0x39, 0xed, 0x32,
                0x33, 0x0d, 0x7c, 0xfe,
            ],
            Bitcoin::InputScriptSigHash => [
                0x1c, 0x7e, 0xfb, 0x37, 0xcb, 0xc4, 0xb9, 0x96, 0xd9, 0xb8, 0xc6, 0x17, 0x08, 0xe6,
                0x65, 0x73, 0xce, 0x87, 0xc1, 0xf0, 0xaa, 0x05, 0x22, 0xd0, 0x65, 0xba, 0x90, 0x25,
                0x05, 0x6e, 0x3f, 0x80,
            ],
            Bitcoin::InputScriptSigsHash => [
                0xe7, 0x29, 0xc5, 0xf5, 0x8a, 0x93, 0x89, 0x08, 0x1a, 0x5c, 0xa8, 0x45, 0xf7, 0x6c,
                0xb9, 0x80, 0xf0, 0x85, 0x99, 0xce, 0xb3, 0xc7, 0xd7, 0xea, 0xe1, 0x14, 0x5b, 0x53,
                0xa6, 0x0a, 0xe3, 0xd5,
            ],
            Bitcoin::InputScriptsHash => [
                0xf1, 0x77, 0x67, 0x1d, 0x60, 0x96, 0x0e, 0x46, 0x3a, 0x9a, 0x8f, 0x7e, 0x1f, 0x52,
                0x13, 0xaa, 0xba, 0x91, 0x22, 0xd2, 0xdb, 0x75, 0xfe, 0x91, 0xf8, 0xf1, 0xcf, 0x91,
                0xbe, 0x00, 0x19, 0x07,
            ],
            Bitcoin::InputSequence => [
                0x5d, 0xd1, 0x7f, 0xe1, 0x55, 0x0f, 0x48, 0xf7, 0xba, 0xed, 0x4d, 0x06, 0x80, 0x08,
                0xd4, 0xa1, 0xff, 0x98, 0xcb, 0xeb, 0xe2, 0x54, 0x1c, 0x5d, 0xc7, 0x7a, 0xc5, 0x3a,
                0xd8, 0x3f, 0xa7, 0x79,
            ],
            Bitcoin::InputSequencesHash => [
                0x55, 0x97, 0x3d, 0x64, 0x43, 0x45, 0x74, 0x72, 0x81, 0x37, 0x73, 0x64, 0xba, 0xd2,
                0xa8, 0x0b, 0x75, 0x8f, 0x45, 0x66, 0x60, 0xc1, 0x8d, 0xe5, 0xc0, 0x1a, 0x38, 0x82,
                0x07, 0x1d, 0x50, 0xe8,
            ],
            Bitcoin::InputUtxoHash => [
                0x93, 0x1e, 0x4e, 0x95, 0xe0, 0xa6, 0x47, 0x60, 0x27, 0x6e, 0x91, 0xb5, 0xdc, 0x74,
                0x67, 0x80, 0xd0, 0x69, 0x7d, 0x0a, 0xf5, 0xaa, 0xf5, 0xbb, 0xc8, 0x1d, 0xbe, 0xb3,
                0x98, 0x59, 0x6a, 0xbb,
            ],
            Bitcoin::InputUtxosHash => [
                0xd6, 0xf9, 0x0c, 0xd1, 0x04, 0xe1, 0xa5, 0xc6, 0x1a, 0x4b, 0x50, 0x00, 0xad, 0x9a,
                0xba, 0x8d, 0x43, 0x00, 0x4b, 0xf9, 0x43, 0xdf, 0x32, 0x5f, 0xa6, 0x36, 0xd1, 0xa2,
                0x2b, 0xec, 0xa0, 0xcb,
            ],
            Bitcoin::InputValue => [
                0x7d, 0x3c, 0x3f, 0x95, 0x5b, 0x2c, 0xf0, 0xd0, 0xd1, 0x28, 0x0a, 0x1b, 0xb1, 0x20,
                0x46, 0x92, 0x92, 0xd1, 0x32, 0x9c, 0x83, 0xa9, 0xc2, 0xff, 0x7e, 0x7e, 0x1e, 0xb3,
                0xf6, 0x97, 0x83, 0xa3,
            ],
            Bitcoin::InputValuesHash => [
                0x29, 0x83, 0x9e, 0xad, 0x0e, 0xb0, 0x3f, 0xe4, 0x65, 0x42, 0xe3, 0x6d, 0x71, 0xe9,
                0xe6, 0xaf, 0xdf, 0x96, 0x93, 0x01, 0x53, 0x3d, 0x74, 0xee, 0x09, 0x9b, 0x12, 0x66,
                0xa2, 0x50, 0x55, 0x2c,
            ],
            Bitcoin::InputsHash => [
                0xab, 0xbf, 0xe1, 0xc7, 0xd1, 0x15, 0xc4, 0x19, 0x1f, 0x50, 0x48, 0x39, 0xf9, 0x8c,
                0x3f, 0x20, 0x42, 0x2b, 0x84, 0xe7, 0xfa, 0x14, 0xda, 0x14, 0x02, 0x89, 0x6c, 0x4d,
                0x98, 0xbf, 0xa8, 0xd8,
            ],
            Bitcoin::InternalKey => [
                0x37, 0x48, 0x36, 0x99, 0x28, 0x10, 0x02, 0x2f, 0x88, 0xe0, 0x14, 0x5b, 0xca, 0xd7,
                0x7f, 0x4a, 0x84, 0x91, 0xfa, 0x80, 0x83, 0xcb, 0x51, 0xc3, 0x01, 0xfc, 0xf7, 0xa1,
                0x34, 0x78, 0xc2, 0xcc,
            ],
            Bitcoin::IsOne16 => [
                0x1b, 0xd3, 0xa2, 0x53, 0xdb, 0x24, 0x3f, 0xca, 0x45, 0x53, 0x37, 0x99, 0xfe, 0x91,
                0x48, 0x38, 0xc3, 0x8e, 0x38, 0x06, 0xb1, 0x2b, 0xd7, 0xe8, 0x5c, 0xa7, 0x12, 0x07,
                0xa8, 0x84, 0x62, 0xb0,
            ],
            Bitcoin::IsOne32 => [
                0x78, 0xb1, 0xba, 0xe0, 0x99, 0xec, 0x9c, 0x59, 0xcb, 0xf4, 0x12, 0x62, 0x51, 0xc1,
                0xe9, 0x67, 0x41, 0xb3, 0x50, 0xd5, 0x63, 0xbd, 0x74, 0xd5, 0x44, 0x18, 0xba, 0x78,
                0xeb, 0xea, 0x25, 0xbf,
            ],
            Bitcoin::IsOne64 => [
                0x81, 0x7b, 0x95, 0xa5, 0x39, 0x5e, 0xfb, 0xec, 0xbb, 0x85, 0x15, 0xa5, 0x5b, 0x3f,
                0xfe, 0x1a, 0x4d, 0x7b, 0xac, 0x6e, 0x23, 0xdb, 0xca, 0x54, 0xad, 0x60, 0x66, 0x66,
                0x2f, 0x20, 0x2b, 0x93,
            ],
            Bitcoin::IsOne8 => [
                0xf6, 0x92, 0x54, 0x91, 0xd3, 0x4b, 0x37, 0x74, 0x2c, 0xb0, 0x8d, 0xec, 0x19, 0x3e,
                0xe5, 0x12, 0x5f, 0x93, 0x3c, 0xad, 0xcc, 0x23, 0x2a, 0xed, 0xee, 0xdb, 0x57, 0x2d,
                0x12, 0x60, 0xff, 0xd5,
            ],
            Bitcoin::IsZero16 => [
                0x1b, 0xa7, 0x21, 0x3b, 0x58, 0x8b, 0xe0, 0x92, 0xb4, 0x46, 0x59, 0x9c, 0x2a, 0x60,
                0xff, 0x54, 0x67, 0x13, 0x6a, 0x79, 0x75, 0x99, 0x61, 0x0b, 0xd7, 0xa5, 0xf1, 0x78,
                0x04, 0xe3, 0x2a, 0x2c,
            ],
            Bitcoin::IsZero32 => [
                0x5e, 0xbf, 0x14, 0x66, 0x93, 0xf0, 0xe2, 0xd2, 0xf9, 0x36, 0x1b, 0x47, 0x6d, 0xba,
                0x34, 0x85, 0x8b, 0x83, 0x2d, 0x66, 0xfa, 0xcf, 0x71, 0x3b, 0xfb, 0x32, 0xc3, 0xbb,
                0x8d, 0xb9, 0xee, 0xbf,
            ],
            Bitcoin::IsZero64 => [
                0x19, 0xab, 0x9a, 0xc0, 0xcf, 0x42, 0x66, 0x82, 0x19, 0xba, 0x6c, 0xb8, 0x97, 0xe4,
                0x87, 0xfe, 0x36, 0x80, 0x93, 0x7f, 0xff, 0xa8, 0xd2, 0x03, 0x51, 0x1d, 0xb7, 0x5d,
                0xbb, 0x10, 0xc7, 0xe5,
            ],
            Bitcoin::IsZero8 => [
                0x8e, 0xff, 0x62, 0x08, 0x44, 0x07, 0xe9, 0xaf, 0xd5, 0x40, 0xf3, 0x18, 0xf6, 0x6b,
                0xcf, 0x31, 0xdf, 0x1d, 0x42, 0xa5, 0xc1, 0x61, 0xca, 0xe3, 0x5a, 0x29, 0x48, 0x18,
                0x0c, 0xa2, 0xaa, 0x2e,
            ],
            Bitcoin::Le16 => [
                0x01, 0x67, 0x05, 0xa7, 0xd7, 0xdc, 0xe1, 0xaf, 0xc6, 0x3e, 0xab, 0x84, 0x20, 0x3f,
                0x5f, 0x42, 0xd6, 0xb6, 0xbb, 0xad, 0x75, 0xce, 0xe3, 0x8c, 0xec, 0x5a, 0x51, 0x5b,
                0x59, 0x97, 0x48, 0x9f,
            ],
            Bitcoin::Le32 => [
                0x53, 0x51, 0xfc, 0x5d, 0xeb, 0xe5, 0xb2, 0x98, 0xad, 0x70, 0x57, 0xe4, 0xa5, 0xa7,
                0x6a, 0x3b, 0x9c, 0x65, 0x8a, 0xcd, 0xe7, 0xd1, 0xbb, 0x52, 0xe5, 0x88, 0x9c, 0xa1,
                0xe3, 0x8f, 0x5e, 0xfb,
            ],
            Bitcoin::Le64 => [
                0xae, 0x2d, 0xe1, 0xe0, 0xcf, 0x73, 0x0d, 0x1d, 0xcc, 0x96, 0xd7, 0xcc, 0xfe, 0x71,
                0x16, 0x8a, 0x24, 0x0d, 0xea, 0xf8, 0x04, 0x61, 0x5a, 0x7b, 0xa9, 0x20, 0xdc, 0x16,
                0xfd, 0x6e, 0xa4, 0x5f,
            ],
            Bitcoin::Le8 => [
                0xaf, 0x29, 0xf6, 0x16, 0x8e, 0xbd, 0xc0, 0x9e, 0xfb, 0xe0, 0xe6, 0x39, 0xcb, 0x75,
                0x0b, 0x12, 0x05, 0x78, 0x8f, 0x90, 0x21, 0xd6, 0x66, 0xef, 0xce, 0xfe, 0x13, 0xf1,
                0x2f, 0x96, 0x71, 0xf0,
            ],
            Bitcoin::LeftExtend16_32 => [
                0x28, 0x99, 0x97, 0xfb, 0xa1, 0xfa, 0xe7, 0xec, 0x1c, 0x45, 0x31, 0xc5, 0x0b, 0xbf,
                0x86, 0x71, 0xb8, 0x97, 0x13, 0x9b, 0xdd, 0x3a, 0xad, 0x97, 0xa3, 0x76, 0x39, 0x57,
                0x4a, 0x04, 0x7c, 0x80,
            ],
            Bitcoin::LeftExtend16_64 => [
                0x5d, 0xff, 0x21, 0xf6, 0xe6, 0x12, 0x47, 0x75, 0xc5, 0x78, 0xea, 0xf4, 0x85, 0x5c,
                0x0b, 0x01, 0x64, 0xf7, 0x87, 0x9b, 0x17, 0x60, 0xf9, 0x02, 0x7c, 0xb5, 0x0f, 0x7b,
                0x5a, 0xcb, 0x49, 0x18,
            ],
            Bitcoin::LeftExtend1_16 => [
                0x8c, 0x87, 0xd7, 0x56, 0xd1, 0x4b, 0xd3, 0xd9, 0xa7, 0x86, 0x90, 0x81, 0x29, 0x12,
                0xb8, 0x94, 0x29, 0xc0, 0x17, 0x1a, 0x41, 0x10, 0x3a, 0x58, 0xc6, 0xe9, 0xf2, 0x25,
                0x14, 0x1a, 0x02, 0x22,
            ],
            Bitcoin::LeftExtend1_32 => [
                0xc8, 0xf1, 0x54, 0xd4, 0x6d, 0x2e, 0x78, 0x95, 0xda, 0x1b, 0x33, 0xc2, 0xb3, 0x15,
                0xe6, 0xd4, 0xd4, 0x85, 0x1d, 0xde, 0xe2, 0x8a, 0xef, 0x8b, 0x70, 0x70, 0x90, 0x61,
                0x6b, 0xc7, 0xee, 0xa0,
            ],
            Bitcoin::LeftExtend1_64 => [
                0xa3, 0x40, 0x4d, 0xf6, 0x8c, 0xc9, 0x20, 0x75, 0x4c, 0x6e, 0x18, 0x47, 0x20, 0x7d,
                0xb3, 0x84, 0x5d, 0x11, 0xc7, 0x49, 0x09, 0xd0, 0x7c, 0xa8, 0x2a, 0xd1, 0xf1, 0xcc,
                0x67, 0xbf, 0x3a, 0x9b,
            ],
            Bitcoin::LeftExtend1_8 => [
                0x3b, 0xca, 0x33, 0x97, 0xb8, 0x3c, 0x27, 0xf3, 0x63, 0x16, 0xf8, 0xb8, 0xb3, 0x03,
                0x35, 0x0a, 0xfe, 0x8b, 0xa0, 0x07, 0x8f, 0x77, 0xf1, 0xd4, 0x2a, 0x9b, 0x78, 0x92,
                0xb2, 0xa4, 0xdb, 0xee,
            ],
            Bitcoin::LeftExtend32_64 => [
                0x42, 0xcb, 0xeb, 0x01, 0xfe, 0x7a, 0x3a, 0x6d, 0xd3, 0x31, 0x1d, 0xb3, 0x36, 0x5f,
                0x91, 0xe5, 0xc1, 0x18, 0xc7, 0xe4, 0x1f, 0x03, 0xaa, 0xe7, 0xb2, 0x83, 0xde, 0x6b,
                0xb9, 0x05, 0x3e, 0x6b,
            ],
            Bitcoin::LeftExtend8_16 => [
                0x9a, 0x57, 0xc9, 0x6a, 0xf5, 0x71, 0x48, 0x96, 0xb7, 0x24, 0xde, 0x45, 0xeb, 0x9f,
                0xe9, 0x7d, 0x73, 0x69, 0x7d, 0xe6, 0x2e, 0x8d, 0xad, 0x78, 0x71, 0xeb, 0x58, 0xf5,
                0x81, 0xa0, 0x11, 0xbb,
            ],
            Bitcoin::LeftExtend8_32 => [
                0xd6, 0x24, 0xbd, 0x40, 0x40, 0x76, 0x3c, 0xb1, 0x3c, 0xca, 0xd4, 0x98, 0xf5, 0x3d,
                0x38, 0xc1, 0x12, 0xf1, 0x92, 0x95, 0x68, 0x26, 0xda, 0xfe, 0xc9, 0xac, 0x91, 0x65,
                0x79, 0x2b, 0x34, 0x7a,
            ],
            Bitcoin::LeftExtend8_64 => [
                0x9d, 0xc4, 0xa2, 0x05, 0x4d, 0x5d, 0x26, 0x34, 0x2a, 0xc5, 0x90, 0xb6, 0x67, 0xf1,
                0xb0, 0x1d, 0xf5, 0x4f, 0xd0, 0xcd, 0xaa, 0x40, 0x5e, 0xf8, 0xcb, 0xb7, 0x6f, 0xd8,
                0xf9, 0xb0, 0x0e, 0xe5,
            ],
            Bitcoin::LeftPadHigh16_32 => [
                0x05, 0x45, 0xc4, 0xb5, 0x8f, 0x00, 0x4a, 0x21, 0xe7, 0xf1, 0x29, 0xa4, 0xc0, 0x51,
                0x89, 0x97, 0x17, 0x14, 0xca, 0xa2, 0xd9, 0x1d, 0x1d, 0xfd, 0x5f, 0xad, 0x3e, 0x63,
                0x24, 0x49, 0x94, 0x28,
            ],
            Bitcoin::LeftPadHigh16_64 => [
                0x1c, 0x61, 0xd0, 0x3d, 0x49, 0x3b, 0xbd, 0x05, 0x82, 0x22, 0x59, 0xd1, 0x73, 0x0a,
                0x8d, 0x7a, 0x5f, 0x55, 0xb0, 0xba, 0x2a, 0x93, 0x91, 0xa6, 0xc8, 0x88, 0x1e, 0xb4,
                0x75, 0x04, 0xaf, 0xfd,
            ],
            Bitcoin::LeftPadHigh1_16 => [
                0x56, 0xfd, 0xf5, 0x4f, 0x1f, 0xcd, 0x19, 0x82, 0x5e, 0x7c, 0x3b, 0x79, 0x06, 0x15,
                0xc1, 0xd3, 0xfe, 0x82, 0x88, 0x6c, 0x74, 0x7b, 0xc4, 0x87, 0x59, 0x87, 0xf5, 0x05,
                0x16, 0x94, 0x5f, 0xb3,
            ],
            Bitcoin::LeftPadHigh1_32 => [
                0xdb, 0x33, 0x05, 0x9a, 0xbe, 0x2d, 0x43, 0x2d, 0x67, 0xf4, 0x2b, 0x1e, 0x94, 0x27,
                0x56, 0xdc, 0xa6, 0xcd, 0xe6, 0x37, 0x85, 0xe5, 0xbd, 0x43, 0x0d, 0xc8, 0xf4, 0xae,
                0xfc, 0x31, 0xb8, 0xdf,
            ],
            Bitcoin::LeftPadHigh1_64 => [
                0x1d, 0x66, 0x9c, 0x1f, 0xa5, 0xfd, 0x3e, 0xf6, 0x6e, 0xb4, 0xae, 0xf6, 0x18, 0x6e,
                0x3e, 0xc1, 0x36, 0xee, 0x75, 0x84, 0x10, 0xdf, 0x3e, 0xde, 0xbb, 0x31, 0xbf, 0x26,
                0xd4, 0x56, 0x20, 0x51,
            ],
            Bitcoin::LeftPadHigh1_8 => [
                0x9a, 0x1b, 0xad, 0x3d, 0x8a, 0xb9, 0x00, 0x30, 0x3d, 0xa2, 0x02, 0xf0, 0xf4, 0x49,
                0xf0, 0xb7, 0xe6, 0x79, 0x5c, 0x2a, 0x7c, 0x12, 0x17, 0x18, 0x80, 0x0a, 0xc4, 0x0c,
                0x87, 0xd8, 0x27, 0x29,
            ],
            Bitcoin::LeftPadHigh32_64 => [
                0x39, 0x20, 0xcc, 0x4b, 0x33, 0xba, 0xf7, 0xef, 0xa5, 0xca, 0xf9, 0xe7, 0x80, 0x01,
                0x44, 0x67, 0x06, 0xf6, 0xe4, 0xe8, 0x26, 0x56, 0x74, 0x05, 0x7e, 0xed, 0x87, 0x17,
                0x78, 0x08, 0x9e, 0x94,
            ],
            Bitcoin::LeftPadHigh8_16 => [
                0x75, 0x2e, 0x29, 0xf2, 0xfe, 0x2b, 0xec, 0xc3, 0xf6, 0x62, 0x90, 0xfe, 0x44, 0xe1,
                0xae, 0xb3, 0x78, 0x41, 0x80, 0xdd, 0x90, 0x5e, 0x19, 0x62, 0x4e, 0x19, 0x5f, 0x21,
                0x6c, 0x07, 0xc5, 0x7c,
            ],
            Bitcoin::LeftPadHigh8_32 => [
                0xbe, 0xe8, 0x8f, 0x1c, 0x8c, 0x30, 0x63, 0x4c, 0x6e, 0x95, 0xca, 0xcc, 0x0e, 0x9a,
                0xdd, 0x49, 0x41, 0x32, 0x21, 0xfd, 0xab, 0xbd, 0x8d, 0x4c, 0x0a, 0xcc, 0xf1, 0xca,
                0xe2, 0xd2, 0xa7, 0x78,
            ],
            Bitcoin::LeftPadHigh8_64 => [
                0x39, 0x23, 0x87, 0xf6, 0xdc, 0x04, 0xbf, 0xc5, 0x4d, 0xd4, 0xa2, 0x81, 0x19, 0xc8,
                0x1d, 0x15, 0xd7, 0xa5, 0x80, 0x9b, 0xbf, 0x62, 0xfc, 0xc2, 0x7d, 0xc5, 0x5c, 0xf8,
                0x2e, 0x9e, 0x5e, 0xe6,
            ],
            Bitcoin::LeftPadLow16_32 => [
                0x4f, 0xfd, 0x6c, 0xb3, 0x40, 0x23, 0x05, 0x82, 0x1d, 0xd8, 0x99, 0x70, 0xd7, 0x22,
                0xd1, 0xc1, 0x3f, 0x1f, 0xf7, 0x73, 0x9f, 0xd5, 0xf3, 0x4b, 0xa1, 0x6c, 0x73, 0x65,
                0x3b, 0x04, 0x47, 0x18,
            ],
            Bitcoin::LeftPadLow16_64 => [
                0xbe, 0x3e, 0xb8, 0x5c, 0x5f, 0x19, 0x91, 0x53, 0xfb, 0x1c, 0x46, 0x13, 0x5c, 0x04,
                0xfa, 0xcf, 0xdb, 0xc6, 0xf1, 0xb7, 0x8c, 0x2b, 0xb7, 0xae, 0x75, 0xf1, 0x55, 0xbc,
                0x3e, 0xa0, 0x8a, 0x8b,
            ],
            Bitcoin::LeftPadLow1_16 => [
                0xdd, 0xd0, 0x15, 0x3e, 0xf3, 0x12, 0xf2, 0x8d, 0x64, 0x2c, 0xd9, 0x4c, 0xb3, 0x6f,
                0x32, 0x97, 0x75, 0xb0, 0x0d, 0xa8, 0x8f, 0xcc, 0xc4, 0xce, 0xa1, 0xba, 0xe8, 0x9b,
                0xad, 0x13, 0xbe, 0x6b,
            ],
            Bitcoin::LeftPadLow1_32 => [
                0xbc, 0x9d, 0x31, 0x14, 0x35, 0x46, 0x7b, 0xc0, 0x8b, 0x10, 0x08, 0xe5, 0x47, 0xaa,
                0x7a, 0x07, 0xe8, 0x3b, 0x15, 0x14, 0x68, 0x61, 0xa9, 0xe9, 0xb5, 0x41, 0x3b, 0xe3,
                0x1b, 0x82, 0xb6, 0xb5,
            ],
            Bitcoin::LeftPadLow1_64 => [
                0x8b, 0xc6, 0x2f, 0x93, 0x60, 0x89, 0x4e, 0x48, 0xa4, 0x73, 0x2c, 0x95, 0x76, 0x9c,
                0x8f, 0xaa, 0xe9, 0x56, 0x8f, 0x9d, 0xe8, 0xe8, 0xa2, 0x00, 0x83, 0x6b, 0xd4, 0xe5,
                0x0b, 0x02, 0xcd, 0x84,
            ],
            Bitcoin::LeftPadLow1_8 => [
                0xf6, 0x6c, 0xd7, 0xa4, 0x2b, 0x32, 0x0f, 0x97, 0xc1, 0x9f, 0x2d, 0x54, 0x16, 0xcd,
                0xe0, 0x87, 0x25, 0x3a, 0x27, 0x91, 0x29, 0x65, 0xd5, 0x5b, 0x65, 0x71, 0x2a, 0xd8,
                0x09, 0xb8, 0x3c, 0xfd,
            ],
            Bitcoin::LeftPadLow32_64 => [
                0xa3, 0x3a, 0x07, 0xb9, 0xbc, 0xf9, 0x45, 0xf6, 0x4f, 0x07, 0x2b, 0x8b, 0x9c, 0x91,
                0x48, 0x39, 0xa5, 0x85, 0xbf, 0xa9, 0xf3, 0x42, 0x5b, 0x14, 0x77, 0x54, 0xab, 0x55,
                0xa8, 0xba, 0x6c, 0x0f,
            ],
            Bitcoin::LeftPadLow8_16 => [
                0x2a, 0x51, 0x6a, 0x79, 0x3f, 0x97, 0xc4, 0x5f, 0xea, 0xeb, 0xb1, 0xcc, 0x96, 0x1a,
                0x15, 0x6d, 0x80, 0x35, 0x49, 0x28, 0x79, 0x78, 0x9d, 0x6e, 0xdc, 0x9b, 0x57, 0xe7,
                0x2f, 0x11, 0xe5, 0xb5,
            ],
            Bitcoin::LeftPadLow8_32 => [
                0x1a, 0xa2, 0xe4, 0xd0, 0x4b, 0xd6, 0x90, 0x55, 0x12, 0x3d, 0xd6, 0xaa, 0xfe, 0x27,
                0xf5, 0xf7, 0xf4, 0x7c, 0x3b, 0x30, 0x90, 0xc3, 0xa8, 0x27, 0x29, 0x73, 0xfe, 0x2f,
                0x75, 0x16, 0x5a, 0x5d,
            ],
            Bitcoin::LeftPadLow8_64 => [
                0xb6, 0x52, 0xe0, 0xae, 0xdd, 0x0f, 0x4f, 0x66, 0xf6, 0xa1, 0xcd, 0x4b, 0xeb, 0xf8,
                0x75, 0xff, 0x7b, 0xbb, 0x2d, 0xd9, 0x9b, 0x06, 0x5b, 0x2d, 0xb5, 0xb5, 0xb5, 0x90,
                0x53, 0x61, 0x61, 0x4d,
            ],
            Bitcoin::LeftRotate16 => [
                0x8a, 0x12, 0xff, 0x6a, 0x4b, 0xf2, 0x37, 0x15, 0xdd, 0x3b, 0x76, 0x6b, 0x99, 0x67,
                0xc7, 0x15, 0x8b, 0xf3, 0xed, 0x74, 0xb3, 0xdc, 0xe7, 0x30, 0xaf, 0xfc, 0xf4, 0x66,
                0x16, 0x47, 0x8e, 0xcb,
            ],
            Bitcoin::LeftRotate32 => [
                0x2f, 0xcb, 0x52, 0x17, 0x2f, 0xd4, 0x9c, 0x36, 0x21, 0x7d, 0xea, 0xe0, 0xc2, 0x37,
                0x14, 0x32, 0x1f, 0x69, 0xf5, 0xf1, 0x3f, 0x6e, 0x94, 0xb2, 0xbd, 0xfe, 0x4b, 0x74,
                0x88, 0x69, 0x7f, 0xd5,
            ],
            Bitcoin::LeftRotate64 => [
                0x72, 0xcc, 0xd6, 0xc4, 0xe5, 0xfd, 0xf6, 0x8a, 0xd3, 0x3b, 0x6d, 0x58, 0xfb, 0x37,
                0x2b, 0xe4, 0xf1, 0xb8, 0x0e, 0xef, 0x70, 0x1f, 0x9d, 0xb7, 0xe5, 0xed, 0x85, 0x9b,
                0x96, 0xb3, 0x62, 0x09,
            ],
            Bitcoin::LeftRotate8 => [
                0x1a, 0xae, 0xc9, 0xf3, 0xb7, 0x5d, 0x89, 0xf8, 0x2a, 0x64, 0x98, 0x45, 0x8c, 0x44,
                0x83, 0xcb, 0x9a, 0x78, 0x44, 0x89, 0x05, 0xf3, 0xbb, 0x39, 0xfc, 0x08, 0x3f, 0x14,
                0xdd, 0xcc, 0xdc, 0x9b,
            ],
            Bitcoin::LeftShift16 => [
                0x37, 0xac, 0x63, 0x87, 0x21, 0xab, 0x09, 0x7a, 0x96, 0x02, 0xba, 0x4d, 0xc9, 0x2e,
                0x19, 0xb5, 0xa1, 0x85, 0xb2, 0x32, 0x9f, 0x1a, 0xa6, 0x00, 0xcb, 0x9c, 0x15, 0x61,
                0x5a, 0x00, 0x81, 0xf8,
            ],
            Bitcoin::LeftShift32 => [
                0x8e, 0x3c, 0x47, 0x3b, 0x28, 0x67, 0xf1, 0x54, 0x73, 0xb3, 0x63, 0x2d, 0xbf, 0xdd,
                0x99, 0x77, 0x55, 0x51, 0xef, 0x5f, 0x9d, 0xba, 0x47, 0x5e, 0x9c, 0xf0, 0x90, 0x75,
                0x80, 0x70, 0xf0, 0xbf,
            ],
            Bitcoin::LeftShift64 => [
                0x50, 0x49, 0xf4, 0x04, 0xd1, 0x73, 0x29, 0x9a, 0x3a, 0xee, 0x04, 0xcb, 0xc2, 0x46,
                0x2c, 0xb3, 0x4c, 0x80, 0x69, 0xc1, 0xb6, 0xdb, 0x7f, 0xed, 0x0e, 0x38, 0x8f, 0xf6,
                0xd4, 0x67, 0xa0, 0x86,
            ],
            Bitcoin::LeftShift8 => [
                0x83, 0x2f, 0x63, 0x6e, 0x63, 0x44, 0x6c, 0xef, 0xba, 0x8d, 0xf3, 0xa4, 0x6e, 0xfb,
                0xb3, 0x61, 0x59, 0xc1, 0x88, 0x54, 0x56, 0x77, 0x68, 0xad, 0xc9, 0xb8, 0xdb, 0x8a,
                0x07, 0x49, 0x2a, 0x58,
            ],
            Bitcoin::LeftShiftWith16 => [
                0xe6, 0x47, 0x62, 0xb1, 0xc5, 0xe6, 0x14, 0x4a, 0x71, 0x81, 0xea, 0xaf, 0x4d, 0xd9,
                0xd9, 0xb3, 0xaa, 0x43, 0xaa, 0xd9, 0x55, 0x15, 0x81, 0x98, 0xee, 0x20, 0x90, 0xeb,
                0xd9, 0xe4, 0xbb, 0x0d,
            ],
            Bitcoin::LeftShiftWith32 => [
                0x64, 0x76, 0xba, 0x89, 0x95, 0xf8, 0x3b, 0x5e, 0xe1, 0xeb, 0xc2, 0x2c, 0xb4, 0x16,
                0xf5, 0x58, 0x15, 0x7f, 0x2e, 0x57, 0x69, 0x9a, 0x5c, 0xaf, 0x84, 0x29, 0x1f, 0xf3,
                0xfc, 0x14, 0x83, 0xc1,
            ],
            Bitcoin::LeftShiftWith64 => [
                0x06, 0xb8, 0xfe, 0x67, 0xcf, 0xc5, 0x86, 0x32, 0x23, 0x97, 0xaf, 0x02, 0x4f, 0xde,
                0x29, 0x11, 0xf7, 0xae, 0x87, 0xa0, 0x6a, 0xbc, 0x6c, 0x59, 0x30, 0x93, 0x40, 0x97,
                0x15, 0x69, 0x1c, 0x19,
            ],
            Bitcoin::LeftShiftWith8 => [
                0xb1, 0xac, 0x9c, 0x68, 0x23, 0x58, 0xc4, 0x5b, 0xab, 0xf4, 0x06, 0x95, 0x56, 0xfe,
                0x6e, 0x37, 0x5b, 0x45, 0x54, 0xde, 0x9e, 0x10, 0xc5, 0x91, 0xc1, 0x48, 0x39, 0x84,
                0x47, 0xac, 0x18, 0x0e,
            ],
            Bitcoin::Leftmost16_1 => [
                0x5b, 0xff, 0x4c, 0xb5, 0x58, 0x76, 0x05, 0xd5, 0xfd, 0x05, 0x9d, 0x77, 0x33, 0x49,
                0x0d, 0x7d, 0xd2, 0x2d, 0x27, 0x8b, 0x59, 0x9e, 0x06, 0xd3, 0xb5, 0xdb, 0x6d, 0x79,
                0xf3, 0xc9, 0x23, 0xbd,
            ],
            Bitcoin::Leftmost16_2 => [
                0x53, 0x6d, 0xb4, 0x86, 0xb1, 0x22, 0x27, 0xe5, 0xb0, 0x9d, 0x6f, 0xeb, 0xd2, 0x77,
                0x6b, 0x1a, 0xbb, 0xc6, 0x74, 0x99, 0x96, 0xaa, 0x78, 0x3e, 0xd7, 0xe5, 0x37, 0x44,
                0x6b, 0xbf, 0x15, 0x1b,
            ],
            Bitcoin::Leftmost16_4 => [
                0xf2, 0x32, 0x13, 0x67, 0x49, 0x6d, 0x1a, 0x77, 0xee, 0xa0, 0x5e, 0x95, 0xe3, 0xb8,
                0x07, 0xd3, 0xba, 0x5f, 0x05, 0x13, 0x6c, 0xe0, 0x91, 0x2a, 0xe7, 0x17, 0xc8, 0x3a,
                0x02, 0x61, 0xb2, 0xe1,
            ],
            Bitcoin::Leftmost16_8 => [
                0x24, 0x14, 0x8e, 0xf3, 0x0a, 0xd4, 0x3e, 0xbe, 0xc5, 0x63, 0x72, 0x83, 0x22, 0xc3,
                0xce, 0x11, 0x79, 0xae, 0xd7, 0xa7, 0x82, 0x16, 0xd7, 0x99, 0x88, 0x8b, 0xf1, 0x8b,
                0x39, 0x57, 0x06, 0x71,
            ],
            Bitcoin::Leftmost32_1 => [
                0xb9, 0x2e, 0x15, 0xec, 0x5d, 0xa0, 0x7e, 0xe8, 0xed, 0x39, 0x7c, 0xb9, 0xf6, 0x0a,
                0x4c, 0x5d, 0xa8, 0x38, 0x62, 0x93, 0x1a, 0x90, 0x73, 0x59, 0xd2, 0x7c, 0xae, 0xb6,
                0x0e, 0x60, 0xef, 0x8a,
            ],
            Bitcoin::Leftmost32_16 => [
                0xad, 0xb0, 0x27, 0xb2, 0x06, 0x56, 0x73, 0x58, 0x53, 0x26, 0xc0, 0x1c, 0x3b, 0xe2,
                0xfa, 0xeb, 0x38, 0x63, 0x49, 0xe2, 0x90, 0x09, 0xb6, 0x57, 0x6e, 0xe5, 0x3a, 0x85,
                0x55, 0x12, 0xcc, 0x67,
            ],
            Bitcoin::Leftmost32_2 => [
                0xb7, 0x5b, 0x31, 0xc5, 0x59, 0x12, 0x3d, 0x3d, 0x63, 0x35, 0x98, 0x59, 0x32, 0xb8,
                0xb1, 0xb2, 0x66, 0x4e, 0xe5, 0x97, 0xaf, 0xb1, 0x5f, 0xd1, 0xa4, 0x99, 0xd0, 0x07,
                0xcf, 0xf2, 0x75, 0x5c,
            ],
            Bitcoin::Leftmost32_4 => [
                0xcb, 0x75, 0x7e, 0x47, 0x1e, 0x9d, 0x9a, 0x40, 0x77, 0x1d, 0xd1, 0xcf, 0x3c, 0x1b,
                0xf5, 0xd2, 0x3c, 0x17, 0xed, 0x68, 0xcd, 0xbd, 0xb2, 0x2d, 0xad, 0xa1, 0x7a, 0x73,
                0xa7, 0xb4, 0x07, 0xb2,
            ],
            Bitcoin::Leftmost32_8 => [
                0xbf, 0xc5, 0x34, 0xb4, 0x9e, 0x06, 0x00, 0x6e, 0x19, 0xf3, 0xb6, 0x8e, 0x0a, 0x02,
                0x39, 0x1c, 0x14, 0x9f, 0x9a, 0x34, 0xf4, 0x3e, 0xe3, 0x6b, 0x9f, 0x1d, 0x79, 0xa7,
                0x9c, 0x9a, 0x9e, 0x4d,
            ],
            Bitcoin::Leftmost64_1 => [
                0x1b, 0x1d, 0x4e, 0x92, 0x38, 0x4b, 0x8b, 0x15, 0x9b, 0xa0, 0xd8, 0x06, 0x55, 0x8b,
                0x54, 0x94, 0xe3, 0x61, 0x4e, 0xed, 0xe0, 0x3c, 0x94, 0x6c, 0xea, 0xf1, 0x41, 0xf3,
                0x6f, 0x01, 0xc7, 0x9b,
            ],
            Bitcoin::Leftmost64_16 => [
                0x0d, 0xeb, 0xdc, 0x1a, 0xa0, 0x43, 0x30, 0x34, 0x42, 0xe1, 0x8f, 0xe0, 0x3d, 0x8a,
                0x99, 0xd2, 0xbe, 0x6b, 0xb8, 0xa8, 0x69, 0x1a, 0xba, 0x19, 0x56, 0x62, 0x59, 0xe3,
                0x67, 0x60, 0xf7, 0xf9,
            ],
            Bitcoin::Leftmost64_2 => [
                0x83, 0x9e, 0xcf, 0xa3, 0x18, 0x70, 0x5c, 0x25, 0x3d, 0x0c, 0x52, 0xff, 0x27, 0xb9,
                0x04, 0x64, 0x92, 0x3d, 0x8c, 0x0e, 0x55, 0xa8, 0x2c, 0x0d, 0x16, 0x24, 0x02, 0x39,
                0x7f, 0x36, 0x53, 0x78,
            ],
            Bitcoin::Leftmost64_32 => [
                0x92, 0x91, 0x97, 0xa9, 0x64, 0x28, 0x61, 0xa7, 0x7b, 0xd6, 0x62, 0x58, 0x05, 0x11,
                0x97, 0xbe, 0x86, 0xff, 0x08, 0xe6, 0x28, 0xe3, 0x0f, 0x7e, 0xfc, 0xbd, 0x2c, 0x4d,
                0xfe, 0xcf, 0x9b, 0xdd,
            ],
            Bitcoin::Leftmost64_4 => [
                0x02, 0xbd, 0x16, 0x45, 0xd5, 0x75, 0xf0, 0x4b, 0x3c, 0xbb, 0xaa, 0x6d, 0x8c, 0xa9,
                0x86, 0xef, 0x1c, 0x8c, 0xd0, 0xff, 0xe1, 0x65, 0x89, 0x03, 0x93, 0x9d, 0xb7, 0x64,
                0x56, 0x2a, 0x26, 0x47,
            ],
            Bitcoin::Leftmost64_8 => [
                0x35, 0x58, 0xb3, 0x1b, 0x3b, 0x6e, 0x8f, 0x9a, 0x28, 0x8f, 0xdc, 0x72, 0xf2, 0x46,
                0x02, 0xbe, 0x05, 0x58, 0x19, 0x10, 0x71, 0xa5, 0x4a, 0x99, 0xfa, 0x03, 0xa0, 0x25,
                0x34, 0xf8, 0x80, 0x05,
            ],
            Bitcoin::Leftmost8_1 => [
                0x28, 0x65, 0xef, 0xd4, 0x29, 0x83, 0xcb, 0xe3, 0xf8, 0x16, 0x37, 0x3a, 0xb8, 0xa8,
                0x82, 0xf1, 0x83, 0x17, 0x19, 0x4d, 0xc1, 0xab, 0xa3, 0x8d, 0xa0, 0x30, 0x4b, 0x8c,
                0x14, 0x4b, 0x1d, 0xa4,
            ],
            Bitcoin::Leftmost8_2 => [
                0x51, 0x96, 0x4c, 0xb0, 0x74, 0x05, 0xa8, 0xd2, 0x3d, 0x21, 0x87, 0x74, 0x1a, 0x9e,
                0xd3, 0x04, 0xbc, 0xb4, 0x69, 0xd9, 0xac, 0x9f, 0x5d, 0x92, 0x55, 0x82, 0x5c, 0xfd,
                0xa3, 0xda, 0x07, 0xc0,
            ],
            Bitcoin::Leftmost8_4 => [
                0x88, 0x3c, 0x94, 0xf8, 0xa2, 0x6c, 0xda, 0xb7, 0xbc, 0x5c, 0xd6, 0x31, 0xe5, 0x22,
                0x55, 0xa8, 0x5e, 0xf6, 0xe0, 0x70, 0x76, 0x64, 0x57, 0xf6, 0x32, 0x1e, 0x2c, 0xcb,
                0x11, 0x9d, 0x9b, 0x2b,
            ],
            Bitcoin::LinearCombination1 => [
                0x34, 0x10, 0xa9, 0xee, 0x33, 0x3d, 0xf8, 0xc8, 0xa0, 0x1c, 0x14, 0x11, 0x5b, 0x54,
                0x43, 0x27, 0xe3, 0x24, 0xe2, 0x87, 0xaa, 0x11, 0x07, 0xe0, 0x19, 0x55, 0xbd, 0x20,
                0x50, 0x6e, 0xa9, 0x87,
            ],
            Bitcoin::LinearVerify1 => [
                0xdc, 0x66, 0xd3, 0x31, 0xc1, 0x7f, 0x3f, 0xdd, 0xa3, 0x99, 0x46, 0x98, 0x1b, 0x39,
                0xb3, 0x57, 0xd0, 0x55, 0x5c, 0x35, 0x62, 0xec, 0xae, 0x02, 0xaa, 0x2d, 0xad, 0x16,
                0x3e, 0x6c, 0x9a, 0x2e,
            ],
            Bitcoin::LockTime => [
                0x9a, 0xe0, 0xac, 0xc3, 0x7b, 0xc2, 0x04, 0x47, 0x79, 0xb0, 0x7c, 0x3d, 0x46, 0x02,
                0xa5, 0xfd, 0xe8, 0xbc, 0x33, 0xf8, 0x79, 0xf6, 0x6b, 0x73, 0x9b, 0x10, 0xf0, 0x1a,
                0xeb, 0x11, 0x54, 0xec,
            ],
            Bitcoin::Low1 => [
                0xfe, 0x62, 0x14, 0xf9, 0x67, 0x15, 0x6d, 0xcd, 0xe6, 0xdd, 0x49, 0xfd, 0xc5, 0x5e,
                0xfb, 0x86, 0x50, 0x69, 0xfe, 0xab, 0xff, 0xf0, 0xfe, 0x93, 0x1d, 0xba, 0x85, 0x31,
                0x34, 0xee, 0xd1, 0x30,
            ],
            Bitcoin::Low16 => [
                0x74, 0x93, 0xcf, 0x69, 0x8a, 0x48, 0x82, 0xe5, 0xc3, 0x57, 0x9d, 0x06, 0x51, 0x8e,
                0x7e, 0xca, 0x2b, 0x84, 0x28, 0xf6, 0x2e, 0x2b, 0x51, 0x38, 0x02, 0xab, 0xe6, 0x22,
                0x17, 0x0c, 0x20, 0xfe,
            ],
            Bitcoin::Low32 => [
                0x36, 0x2d, 0x66, 0xa4, 0xf0, 0xae, 0xb9, 0x65, 0x84, 0xa5, 0x67, 0x57, 0x82, 0x71,
                0xb1, 0xf7, 0xbb, 0xfc, 0xc2, 0xde, 0x0d, 0xcf, 0x95, 0x79, 0x6b, 0x6f, 0x7a, 0x82,
                0x6b, 0x2a, 0x8a, 0xf7,
            ],
            Bitcoin::Low64 => [
                0x97, 0x33, 0x23, 0xbc, 0x2b, 0x92, 0xe4, 0x28, 0x04, 0xd2, 0xe4, 0xf5, 0x8b, 0x86,
                0xf6, 0x5b, 0x56, 0xf9, 0x1d, 0xee, 0xb4, 0x81, 0x0e, 0xab, 0x8a, 0x1d, 0xed, 0xa9,
                0x69, 0x7a, 0x08, 0x72,
            ],
            Bitcoin::Low8 => [
                0xcd, 0x1a, 0x85, 0x58, 0xef, 0x99, 0xa3, 0x22, 0x60, 0x21, 0x7a, 0x76, 0x49, 0xff,
                0x51, 0x40, 0xda, 0x69, 0xda, 0x70, 0x06, 0x72, 0x69, 0x0b, 0x27, 0x91, 0x7b, 0x07,
                0xd7, 0xc1, 0x4c, 0x67,
            ],
            Bitcoin::Lt16 => [
                0x04, 0xac, 0xa8, 0x7e, 0x3e, 0x17, 0xf8, 0x05, 0xa2, 0x1c, 0xf2, 0x91, 0x7a, 0xee,
                0x99, 0x57, 0xb9, 0x50, 0xb2, 0xdb, 0x5d, 0x7a, 0xe5, 0xc8, 0x26, 0xd4, 0xac, 0x2e,
                0xc9, 0x7b, 0x5a, 0x52,
            ],
            Bitcoin::Lt32 => [
                0x23, 0xa0, 0xa5, 0xc1, 0x97, 0x74, 0x7e, 0x3a, 0x95, 0x79, 0xe9, 0x0e, 0x0f, 0x22,
                0xf8, 0x4a, 0x29, 0xbf, 0xb5, 0xf0, 0x7b, 0x84, 0xb5, 0x9b, 0x26, 0x68, 0x8a, 0x0c,
                0xd5, 0x9d, 0xfe, 0xbd,
            ],
            Bitcoin::Lt64 => [
                0xd2, 0x99, 0x90, 0x1c, 0x7b, 0x5b, 0x3a, 0x59, 0xff, 0xc8, 0xdd, 0x09, 0x54, 0x5a,
                0x32, 0x38, 0x24, 0xb7, 0x79, 0xa9, 0x9b, 0x2d, 0x1a, 0x2f, 0x87, 0x45, 0x2d, 0x9e,
                0x4b, 0xef, 0xaf, 0x30,
            ],
            Bitcoin::Lt8 => [
                0xdd, 0x94, 0x41, 0x3b, 0x52, 0x9c, 0x29, 0x8c, 0x16, 0x96, 0xe9, 0xfb, 0x08, 0xe6,
                0x67, 0x67, 0xb3, 0xf8, 0x33, 0x7a, 0xc0, 0x2e, 0x44, 0xb0, 0x68, 0xe9, 0x40, 0x14,
                0xf7, 0xc4, 0x1f, 0x2a,
            ],
            Bitcoin::Maj1 => [
                0x0e, 0x6f, 0xb4, 0x0f, 0xe3, 0x1a, 0x3a, 0x52, 0x6b, 0x44, 0xcf, 0x0b, 0x7c, 0x79,
                0x36, 0xc7, 0x77, 0xcb, 0xba, 0x89, 0x65, 0xa7, 0x25, 0x52, 0x32, 0xa7, 0xcf, 0x53,
                0xa9, 0x22, 0x88, 0x5a,
            ],
            Bitcoin::Maj16 => [
                0x38, 0x66, 0x9c, 0xe5, 0xe1, 0xe1, 0x71, 0x47, 0x54, 0x00, 0x73, 0x1b, 0xee, 0xb6,
                0x0b, 0xca, 0xfa, 0xd6, 0x66, 0x04, 0xc9, 0x39, 0x40, 0x16, 0x0c, 0xd7, 0x12, 0x88,
                0x35, 0x55, 0x93, 0x42,
            ],
            Bitcoin::Maj32 => [
                0x55, 0x54, 0x34, 0x9b, 0x58, 0x4f, 0x5c, 0x38, 0x72, 0xc7, 0xf4, 0xf2, 0x57, 0x82,
                0x9e, 0x2a, 0xe8, 0x22, 0xd8, 0x23, 0x42, 0x4c, 0xeb, 0x95, 0x98, 0xf0, 0x83, 0x18,
                0x58, 0x6a, 0x88, 0x07,
            ],
            Bitcoin::Maj64 => [
                0x73, 0x49, 0x03, 0xba, 0xef, 0xb7, 0x1d, 0x5e, 0xa4, 0x16, 0x48, 0xff, 0x43, 0xee,
                0xe6, 0x98, 0x94, 0xe0, 0x63, 0xb3, 0x88, 0xea, 0x42, 0x2f, 0x96, 0xae, 0xde, 0x19,
                0x3c, 0xea, 0xb8, 0x39,
            ],
            Bitcoin::Maj8 => [
                0xba, 0x47, 0xa3, 0x99, 0xdc, 0x94, 0x35, 0xe1, 0x8e, 0x08, 0x0a, 0x4e, 0x18, 0xaf,
                0x7c, 0x65, 0x7f, 0xd3, 0x9f, 0x7c, 0xe7, 0xd6, 0x05, 0x2e, 0x46, 0x90, 0x23, 0x11,
                0xb0, 0x78, 0xd5, 0x85,
            ],
            Bitcoin::Max16 => [
                0xaa, 0x55, 0x23, 0x74, 0x6c, 0xab, 0xfa, 0xf5, 0x66, 0x8e, 0x9e, 0x07, 0x37, 0xe5,
                0x6b, 0x06, 0x06, 0x22, 0x51, 0xd7, 0xe8, 0x0a, 0xb9, 0xb9, 0x10, 0x6d, 0x8f, 0x17,
                0x2d, 0xc8, 0x4d, 0xd6,
            ],
            Bitcoin::Max32 => [
                0x69, 0x22, 0x96, 0x5d, 0x14, 0x43, 0x45, 0xc9, 0x13, 0xec, 0xb3, 0x0b, 0x5e, 0xd4,
                0x7e, 0x88, 0xda, 0xe3, 0x5c, 0x12, 0x21, 0xf2, 0x6a, 0xa9, 0x2d, 0xd5, 0xa5, 0xf6,
                0x15, 0xdb, 0xdb, 0x53,
            ],
            Bitcoin::Max64 => [
                0x8a, 0x9b, 0xe9, 0x07, 0xb6, 0xa4, 0xc3, 0x0a, 0xbc, 0xc0, 0xf2, 0x2d, 0x01, 0x30,
                0x74, 0xc2, 0xd5, 0x6b, 0xb0, 0x81, 0xf2, 0x62, 0x18, 0x57, 0xd5, 0x38, 0xcc, 0x97,
                0x13, 0x1e, 0x44, 0x09,
            ],
            Bitcoin::Max8 => [
                0xb4, 0xbf, 0x93, 0x23, 0x40, 0x22, 0xe8, 0x60, 0xfe, 0x76, 0xc0, 0xb5, 0x36, 0x0e,
                0x8b, 0x36, 0xff, 0x81, 0xee, 0x67, 0x05, 0xb5, 0x93, 0xac, 0xdf, 0x65, 0x5a, 0xc6,
                0xe6, 0xd7, 0xae, 0xba,
            ],
            Bitcoin::Median16 => [
                0x17, 0xe2, 0xe8, 0x7f, 0x07, 0x60, 0xf4, 0xfb, 0x3c, 0x9f, 0xd0, 0xbe, 0xd0, 0x00,
                0xd7, 0x39, 0x73, 0xab, 0x60, 0xf5, 0xe6, 0xc2, 0xc1, 0xfa, 0xb1, 0x7f, 0x9b, 0x23,
                0xee, 0x6a, 0xca, 0x48,
            ],
            Bitcoin::Median32 => [
                0x11, 0x60, 0xae, 0x8e, 0xa8, 0xd3, 0x0f, 0x9a, 0x22, 0x33, 0xc4, 0x8e, 0x73, 0x12,
                0x40, 0xf8, 0x44, 0x93, 0xb8, 0x28, 0xb5, 0x57, 0x93, 0xe2, 0xf4, 0x04, 0x2a, 0x19,
                0x82, 0xac, 0x26, 0xa5,
            ],
            Bitcoin::Median64 => [
                0xc8, 0x73, 0x73, 0x64, 0x9e, 0x7e, 0x40, 0x50, 0xbb, 0x73, 0x33, 0x7e, 0x08, 0xeb,
                0x5d, 0xe4, 0x52, 0x28, 0xab, 0x86, 0xad, 0x4e, 0x1f, 0x41, 0x91, 0xe5, 0x20, 0x2a,
                0xa6, 0xaf, 0xa0, 0xc5,
            ],
            Bitcoin::Median8 => [
                0xc3, 0xb4, 0xe0, 0x89, 0x8a, 0x21, 0xbd, 0xe9, 0x4d, 0xae, 0xd3, 0x7a, 0x20, 0xad,
                0xf9, 0x0c, 0x8b, 0xe5, 0x69, 0x1a, 0x03, 0xb6, 0xa1, 0xe5, 0x56, 0x38, 0x5d, 0x42,
                0xeb, 0x19, 0x02, 0x2b,
            ],
            Bitcoin::Min16 => [
                0x5f, 0xd0, 0x05, 0x1e, 0xdb, 0x37, 0x19, 0xa6, 0x45, 0xb2, 0x72, 0xa0, 0x21, 0x08,
                0xef, 0xbb, 0x3d, 0x9b, 0xc0, 0xf6, 0x06, 0x21, 0xbf, 0x5a, 0x5b, 0xab, 0xe1, 0x16,
                0xd5, 0x55, 0xd5, 0x78,
            ],
            Bitcoin::Min32 => [
                0xd8, 0x07, 0x82, 0xa2, 0xb5, 0xd8, 0x6a, 0xb6, 0xb9, 0xc9, 0xc3, 0xfb, 0x77, 0x8a,
                0x34, 0x73, 0xf6, 0x00, 0xb1, 0x85, 0xfe, 0x19, 0x25, 0xee, 0x9f, 0xc2, 0xe8, 0x77,
                0x7e, 0xd2, 0x66, 0x01,
            ],
            Bitcoin::Min64 => [
                0xc5, 0xc0, 0x9d, 0x50, 0x13, 0x38, 0xe9, 0xa5, 0x12, 0xcf, 0x89, 0x76, 0xca, 0x4b,
                0x32, 0xb9, 0x24, 0x80, 0xbe, 0xf6, 0xae, 0xb2, 0x9d, 0x36, 0xd5, 0x90, 0xd3, 0x5b,
                0xf9, 0xf9, 0xec, 0xe1,
            ],
            Bitcoin::Min8 => [
                0x81, 0xd2, 0x1e, 0x12, 0x81, 0x42, 0x38, 0x81, 0x80, 0x2c, 0x0e, 0x0c, 0x7d, 0x22,
                0xbd, 0x34, 0xd2, 0x6b, 0xd1, 0x2a, 0x4c, 0x4f, 0x1b, 0x70, 0x68, 0xe7, 0xe1, 0x83,
                0x82, 0x08, 0x48, 0xe9,
            ],
            Bitcoin::Modulo16 => [
                0xb6, 0xb8, 0x7c, 0xfa, 0xb6, 0x7e, 0x55, 0x19, 0xf1, 0xc9, 0x98, 0xda, 0x47, 0x94,
                0x37, 0xbb, 0x79, 0xe6, 0x74, 0xf7, 0x15, 0xe9, 0xa2, 0xe5, 0x38, 0xee, 0xc5, 0xec,
                0x18, 0xe1, 0x8e, 0xa5,
            ],
            Bitcoin::Modulo32 => [
                0x8d, 0x48, 0x6e, 0x83, 0x16, 0x54, 0xf3, 0x8a, 0x32, 0xda, 0x35, 0xeb, 0x7b, 0xb6,
                0x55, 0xa6, 0xed, 0x69, 0x4d, 0xbf, 0xa0, 0x58, 0x95, 0x7d, 0x9f, 0x5c, 0xbf, 0xcc,
                0x57, 0x92, 0xc6, 0x5b,
            ],
            Bitcoin::Modulo64 => [
                0x14, 0xdf, 0x20, 0xd9, 0x3d, 0xfd, 0xef, 0xe2, 0x55, 0x9b, 0xac, 0x50, 0xed, 0x38,
                0x19, 0x3b, 0xd7, 0x8b, 0xd6, 0x3f, 0x92, 0x9d, 0x86, 0xfb, 0x4f, 0x29, 0xa7, 0xc5,
                0xaf, 0x32, 0x42, 0xad,
            ],
            Bitcoin::Modulo8 => [
                0x2c, 0x75, 0x8a, 0x7c, 0x0f, 0x59, 0xe8, 0x00, 0xe9, 0x4f, 0x3d, 0xc5, 0xa0, 0x01,
                0xbf, 0x8e, 0xd9, 0x43, 0x5f, 0x75, 0xa2, 0xd9, 0x69, 0x30, 0xc5, 0x7e, 0xaa, 0xb0,
                0xcd, 0x80, 0xaf, 0x5c,
            ],
            Bitcoin::Multiply16 => [
                0x75, 0xbd, 0x41, 0xf2, 0xd2, 0xb3, 0x39, 0xf0, 0x69, 0xbf, 0xdf, 0xd8, 0x02, 0xd6,
                0x1e, 0x6c, 0xa8, 0xe3, 0xba, 0xd6, 0xfb, 0x6d, 0x95, 0xb6, 0x72, 0x09, 0x5b, 0x93,
                0x34, 0x5f, 0x04, 0x7f,
            ],
            Bitcoin::Multiply32 => [
                0x84, 0xcb, 0xe6, 0xce, 0x87, 0x03, 0x79, 0x92, 0x13, 0x87, 0x7c, 0x1b, 0xd5, 0x05,
                0xc7, 0x64, 0x34, 0x33, 0x69, 0x00, 0x2e, 0x50, 0x2c, 0x43, 0xd9, 0x7f, 0x3d, 0x57,
                0x77, 0x2d, 0x6c, 0x87,
            ],
            Bitcoin::Multiply64 => [
                0x92, 0x98, 0x7b, 0x80, 0x1b, 0x92, 0xf6, 0x79, 0xeb, 0x96, 0x13, 0x68, 0x84, 0x44,
                0xa1, 0x78, 0x87, 0x50, 0xa8, 0x50, 0x6e, 0x03, 0xa9, 0x21, 0x8c, 0x21, 0xec, 0xc7,
                0x20, 0x82, 0xdc, 0x6a,
            ],
            Bitcoin::Multiply8 => [
                0x76, 0x4c, 0xab, 0x71, 0xdb, 0x94, 0x59, 0xa7, 0x69, 0x6d, 0x94, 0x4a, 0x50, 0x09,
                0x5b, 0x1a, 0xeb, 0xdf, 0xd9, 0x28, 0x4b, 0xdb, 0x74, 0x96, 0xa7, 0xb3, 0x02, 0x41,
                0xcc, 0xba, 0x3e, 0xce,
            ],
            Bitcoin::Negate16 => [
                0xe7, 0x60, 0xee, 0x40, 0x29, 0xc3, 0x4f, 0x89, 0x74, 0x06, 0xff, 0xde, 0xa5, 0x55,
                0x84, 0x86, 0x62, 0xe8, 0x9c, 0x98, 0x3e, 0x60, 0x70, 0xbd, 0x02, 0x72, 0xad, 0x0f,
                0xa3, 0x42, 0xef, 0xa3,
            ],
            Bitcoin::Negate32 => [
                0x84, 0x95, 0xb7, 0x40, 0x09, 0xad, 0x07, 0xc9, 0x30, 0x2a, 0x25, 0xae, 0x56, 0xc3,
                0xe9, 0x73, 0x3f, 0x00, 0xc2, 0xba, 0xa4, 0x10, 0xea, 0xc4, 0xa5, 0x8e, 0x75, 0xdb,
                0x83, 0xaf, 0x1d, 0x22,
            ],
            Bitcoin::Negate64 => [
                0x34, 0xe8, 0x9f, 0xaf, 0x34, 0x5a, 0xfd, 0x5e, 0x7b, 0x29, 0x00, 0x14, 0x52, 0xfc,
                0x5f, 0xc2, 0xe3, 0x78, 0x3a, 0xf7, 0xf2, 0x10, 0x16, 0x43, 0xbd, 0x76, 0x70, 0x6a,
                0x6f, 0xc3, 0xf3, 0x6a,
            ],
            Bitcoin::Negate8 => [
                0xe8, 0x1b, 0xe0, 0xb1, 0x5c, 0x67, 0x1a, 0xb8, 0xdf, 0x1f, 0x48, 0x69, 0xc5, 0x7f,
                0x11, 0x11, 0x18, 0xcb, 0x66, 0x83, 0x54, 0x97, 0x5c, 0x63, 0x66, 0xec, 0xb2, 0xb8,
                0xbb, 0x7c, 0x15, 0xcf,
            ],
            Bitcoin::NumInputs => [
                0x5c, 0x5a, 0xc4, 0xff, 0x6d, 0xa5, 0x6c, 0xb3, 0x72, 0xb2, 0x32, 0x66, 0x6e, 0x83,
                0x34, 0xb9, 0xe2, 0xcf, 0xb0, 0xdc, 0xb4, 0x18, 0xf1, 0x61, 0xbf, 0xf1, 0x49, 0xe8,
                0x4e, 0xc9, 0x2c, 0x3e,
            ],
            Bitcoin::NumOutputs => [
                0x98, 0xa1, 0xcc, 0xa7, 0x05, 0xdf, 0xcf, 0xaf, 0xd3, 0xa6, 0x9e, 0x9a, 0xdc, 0x05,
                0xba, 0x47, 0xe1, 0xfe, 0xfa, 0x6a, 0x29, 0xf3, 0x42, 0x86, 0x20, 0x48, 0xe4, 0x96,
                0x86, 0x48, 0xc3, 0xd7,
            ],
            Bitcoin::One16 => [
                0x2e, 0x5e, 0x3d, 0x95, 0xe4, 0x53, 0x16, 0x88, 0x8e, 0x4f, 0x37, 0x09, 0xef, 0x83,
                0x2b, 0x9f, 0xd9, 0xe1, 0x5f, 0x30, 0x71, 0x9b, 0xf5, 0x5f, 0xc2, 0xe0, 0xe0, 0x9a,
                0x36, 0x57, 0xd8, 0x82,
            ],
            Bitcoin::One32 => [
                0x06, 0x42, 0x6b, 0x85, 0x3c, 0x1b, 0xcb, 0x33, 0x8a, 0xed, 0xbe, 0x1f, 0x89, 0xa6,
                0xd9, 0xb7, 0xa3, 0xda, 0x03, 0x8c, 0xd0, 0x0a, 0x44, 0x71, 0x18, 0x36, 0x93, 0x49,
                0x66, 0x9e, 0x29, 0x76,
            ],
            Bitcoin::One64 => [
                0xab, 0x1d, 0x2c, 0xd9, 0x96, 0x78, 0xda, 0x3c, 0x12, 0x8d, 0x39, 0xad, 0x9f, 0xe6,
                0xff, 0xa9, 0x55, 0xc1, 0x6e, 0x5e, 0xf2, 0xc2, 0x5b, 0xb4, 0x31, 0x83, 0x15, 0x59,
                0x69, 0x51, 0xf4, 0x27,
            ],
            Bitcoin::One8 => [
                0x3c, 0xc5, 0xf5, 0x23, 0xd6, 0xa6, 0x35, 0x5d, 0xc9, 0x24, 0xee, 0x0a, 0xc1, 0xf5,
                0xfe, 0x2c, 0x52, 0x12, 0x75, 0xe3, 0xaa, 0x9f, 0x21, 0xd3, 0x1b, 0x08, 0x2d, 0xb2,
                0xac, 0x23, 0x0d, 0x9d,
            ],
            Bitcoin::Or1 => [
                0xc4, 0x65, 0x96, 0x43, 0x69, 0xfc, 0xa2, 0x09, 0x7f, 0x83, 0x53, 0x0c, 0x87, 0xbc,
                0xbc, 0x90, 0xc3, 0x06, 0x57, 0x9d, 0x9f, 0x3b, 0xfe, 0xdd, 0xf4, 0xa1, 0x72, 0xa4,
                0xea, 0x0b, 0x58, 0xec,
            ],
            Bitcoin::Or16 => [
                0x5a, 0x98, 0x5e, 0x04, 0x3b, 0x85, 0x27, 0x3b, 0x90, 0xf9, 0x0e, 0x20, 0xf8, 0x2b,
                0x75, 0x32, 0x33, 0x51, 0xcf, 0x2a, 0x4e, 0x62, 0xa7, 0xf9, 0xcb, 0x2f, 0x05, 0x96,
                0x40, 0x2e, 0x9e, 0x28,
            ],
            Bitcoin::Or32 => [
                0x35, 0x52, 0x38, 0x3a, 0x57, 0xff, 0xb4, 0x8d, 0x63, 0xa0, 0x33, 0x7a, 0xf0, 0xdd,
                0x6e, 0xfa, 0xb6, 0xb4, 0x6c, 0x5d, 0xe1, 0x72, 0x0e, 0x42, 0x0b, 0xdd, 0x1c, 0x82,
                0x27, 0x6b, 0xc9, 0xa9,
            ],
            Bitcoin::Or64 => [
                0x51, 0xa1, 0x73, 0xda, 0xdc, 0xa0, 0x1a, 0xc6, 0xf6, 0x2e, 0x75, 0xd5, 0xcd, 0x35,
                0x22, 0xf0, 0x9f, 0xde, 0x62, 0xb1, 0x15, 0x13, 0xe0, 0x68, 0x42, 0x28, 0x52, 0xa4,
                0x91, 0x67, 0xb6, 0x06,
            ],
            Bitcoin::Or8 => [
                0x79, 0xef, 0xbd, 0xcb, 0x53, 0x7b, 0xeb, 0xcb, 0x18, 0x8d, 0x11, 0x16, 0xb7, 0x8a,
                0x10, 0x9b, 0xff, 0xbc, 0x2a, 0x6c, 0xe3, 0xd1, 0xf8, 0x70, 0x15, 0x4a, 0x79, 0x56,
                0x09, 0x1b, 0x34, 0x2f,
            ],
            Bitcoin::OutpointHash => [
                0x3a, 0x1a, 0xe9, 0x0e, 0x16, 0x7f, 0xb4, 0x0d, 0x6e, 0x13, 0xb4, 0x51, 0xad, 0x67,
                0x41, 0x0d, 0x8d, 0xd9, 0x91, 0xc8, 0x7d, 0x6a, 0x4a, 0x59, 0xcc, 0x76, 0xc6, 0x3f,
                0x3b, 0x9e, 0x5e, 0x56,
            ],
            Bitcoin::OutputHash => [
                0x91, 0x21, 0x1f, 0xc6, 0x01, 0x1a, 0x64, 0x93, 0x00, 0xc6, 0xbe, 0xe9, 0x4f, 0xdd,
                0x48, 0xa9, 0x7f, 0xa2, 0xa9, 0xb6, 0xf2, 0x84, 0xbe, 0x01, 0x5d, 0x46, 0x2d, 0x17,
                0xde, 0x66, 0x4a, 0xc3,
            ],
            Bitcoin::OutputScriptHash => [
                0xbd, 0xfd, 0xb2, 0x31, 0xf4, 0xf1, 0xa6, 0x2c, 0x9d, 0x7b, 0x03, 0x93, 0x1e, 0x7f,
                0x19, 0xa4, 0x54, 0x6a, 0xf2, 0x34, 0x75, 0x4c, 0xbf, 0x70, 0x05, 0x9f, 0xdd, 0x42,
                0xbb, 0xbc, 0x41, 0x26,
            ],
            Bitcoin::OutputScriptsHash => [
                0xff, 0x20, 0xbc, 0x43, 0x65, 0xe7, 0x17, 0x07, 0x57, 0x1c, 0x6e, 0x17, 0x38, 0xe1,
                0xed, 0x32, 0x6f, 0x7c, 0x35, 0x1d, 0xe1, 0x30, 0x22, 0xae, 0xa3, 0xd6, 0x40, 0x6b,
                0x8a, 0xee, 0x8e, 0x3b,
            ],
            Bitcoin::OutputValue => [
                0x93, 0x36, 0x43, 0xb6, 0xc5, 0xa6, 0x22, 0x0a, 0xbb, 0xca, 0x6f, 0x35, 0x09, 0xfe,
                0xff, 0x6d, 0x13, 0xef, 0xa6, 0xc9, 0xfa, 0xe9, 0x59, 0x24, 0x57, 0x53, 0x64, 0xf2,
                0xb1, 0x64, 0xd2, 0xbc,
            ],
            Bitcoin::OutputValuesHash => [
                0x22, 0x89, 0x93, 0x79, 0x00, 0x57, 0x06, 0x6f, 0x20, 0x16, 0x97, 0x1d, 0xf5, 0x5e,
                0x6f, 0x67, 0xd2, 0x52, 0xef, 0xb6, 0xda, 0xab, 0xd0, 0xfc, 0x56, 0x6a, 0x8d, 0x21,
                0x56, 0xef, 0xbb, 0xfc,
            ],
            Bitcoin::OutputsHash => [
                0xf2, 0xeb, 0x6d, 0x0f, 0x01, 0x8e, 0x6f, 0x15, 0xe3, 0x5b, 0xaa, 0x82, 0xe5, 0x7e,
                0x14, 0xfe, 0x34, 0x37, 0x96, 0xf2, 0x19, 0x68, 0x26, 0xbe, 0xd7, 0xc7, 0x87, 0x55,
                0x98, 0xd6, 0x64, 0x1d,
            ],
            Bitcoin::ParseLock => [
                0x3d, 0xb8, 0x45, 0x35, 0xfa, 0x3d, 0x90, 0xef, 0x0b, 0x58, 0x1e, 0x22, 0xb6, 0x1d,
                0x21, 0x27, 0x84, 0x4b, 0x21, 0x16, 0xe8, 0x4f, 0x81, 0x4a, 0x5c, 0xba, 0xc5, 0x2d,
                0xf5, 0x15, 0xf2, 0xd2,
            ],
            Bitcoin::ParseSequence => [
                0x38, 0xb2, 0x53, 0x3f, 0x5f, 0xed, 0xe8, 0x69, 0xba, 0xa1, 0x70, 0x69, 0x83, 0xdf,
                0x4c, 0x89, 0xd6, 0x2d, 0x5f, 0x90, 0x80, 0x0b, 0x47, 0xea, 0xb2, 0x11, 0x13, 0x31,
                0x1a, 0x5a, 0xae, 0xc9,
            ],
            Bitcoin::PointVerify1 => [
                0xbe, 0x2a, 0x98, 0x90, 0xf1, 0xd5, 0xb6, 0x15, 0x14, 0x7f, 0x82, 0x41, 0xe0, 0x60,
                0x9b, 0x5c, 0xac, 0x01, 0xec, 0xe0, 0xa3, 0xf9, 0x23, 0x68, 0x67, 0xb2, 0xbf, 0xde,
                0xa1, 0xb8, 0x04, 0x4e,
            ],
            Bitcoin::RightExtend16_32 => [
                0xdb, 0xf1, 0x8d, 0x87, 0xa7, 0x89, 0x21, 0x39, 0xa3, 0x88, 0xe9, 0xa9, 0x83, 0xc4,
                0x89, 0x92, 0xac, 0x35, 0xa8, 0x45, 0x56, 0xee, 0x0d, 0xef, 0xc1, 0xda, 0xdf, 0x0c,
                0x5f, 0x47, 0x1a, 0x26,
            ],
            Bitcoin::RightExtend16_64 => [
                0xd0, 0x11, 0xac, 0xc7, 0x94, 0xe3, 0xc4, 0x78, 0x9a, 0xcc, 0xd0, 0xd5, 0xfe, 0x49,
                0x97, 0xd3, 0x34, 0xd9, 0x1f, 0x08, 0x31, 0xa1, 0xeb, 0x35, 0x04, 0xb4, 0xcb, 0x2d,
                0xdf, 0x47, 0x97, 0xaf,
            ],
            Bitcoin::RightExtend32_64 => [
                0xa5, 0xaa, 0x5d, 0xb1, 0xe5, 0x35, 0xe7, 0x23, 0x2a, 0xd3, 0x6d, 0xaf, 0xba, 0x6d,
                0x5a, 0x20, 0x0d, 0x54, 0xeb, 0x85, 0x3b, 0x75, 0xdc, 0x70, 0xa5, 0x94, 0xed, 0x64,
                0xaa, 0x6b, 0xd9, 0xab,
            ],
            Bitcoin::RightExtend8_16 => [
                0x81, 0x06, 0xd5, 0x8a, 0x80, 0x66, 0xee, 0x6e, 0x15, 0xe5, 0x5c, 0xa5, 0x2c, 0xb7,
                0xaf, 0xd8, 0xe3, 0x27, 0x75, 0x87, 0xbf, 0xd7, 0xde, 0xc0, 0xbe, 0x37, 0xd4, 0x06,
                0x74, 0x2a, 0x39, 0x31,
            ],
            Bitcoin::RightExtend8_32 => [
                0xdf, 0xa4, 0xba, 0xfa, 0x43, 0x2a, 0x53, 0x38, 0xd3, 0x74, 0xde, 0xb6, 0xb7, 0x24,
                0xb7, 0xf6, 0xea, 0xe5, 0x58, 0x61, 0xfe, 0x73, 0x1d, 0x43, 0x04, 0x8a, 0xa3, 0x04,
                0xd1, 0xf7, 0xf9, 0xa2,
            ],
            Bitcoin::RightExtend8_64 => [
                0x62, 0x0a, 0x37, 0x03, 0x8b, 0x6f, 0xa1, 0x27, 0x49, 0x5f, 0x0b, 0x46, 0x49, 0x6f,
                0x64, 0x35, 0xdd, 0x2d, 0xad, 0x7e, 0xf0, 0xc0, 0xfd, 0x2c, 0xd6, 0x5f, 0x54, 0xdc,
                0x18, 0x5e, 0x99, 0x7b,
            ],
            Bitcoin::RightPadHigh16_32 => [
                0x2b, 0x6a, 0xbc, 0x38, 0x32, 0x1a, 0x7c, 0x54, 0x2f, 0xb1, 0x69, 0x74, 0x62, 0x1c,
                0xed, 0x80, 0x88, 0x0d, 0xb5, 0x19, 0xbb, 0x48, 0x60, 0x93, 0x42, 0x6e, 0x8c, 0xe1,
                0x8e, 0x01, 0x69, 0xb1,
            ],
            Bitcoin::RightPadHigh16_64 => [
                0xad, 0x90, 0xd8, 0xff, 0xa5, 0x74, 0x50, 0xb3, 0xb5, 0xe9, 0x09, 0x62, 0x25, 0x34,
                0x9e, 0xd8, 0xf0, 0x72, 0xe1, 0x01, 0x72, 0x93, 0xf3, 0x92, 0xef, 0x85, 0x4e, 0x03,
                0x19, 0xab, 0xc9, 0x34,
            ],
            Bitcoin::RightPadHigh1_16 => [
                0x28, 0x81, 0x58, 0xb1, 0xc9, 0x10, 0x87, 0x7b, 0x7e, 0xea, 0x3d, 0xfc, 0xf2, 0xb2,
                0xb7, 0x88, 0x92, 0x28, 0x08, 0xb6, 0xd6, 0xfa, 0x75, 0xf8, 0x96, 0x77, 0x19, 0x04,
                0x8b, 0x14, 0x12, 0x49,
            ],
            Bitcoin::RightPadHigh1_32 => [
                0xee, 0x2a, 0xd7, 0x7f, 0x66, 0x8d, 0x3d, 0x6a, 0x2e, 0x68, 0x50, 0x6e, 0x49, 0x04,
                0xcf, 0x50, 0xa0, 0x84, 0x60, 0xe1, 0xd2, 0xb8, 0x6a, 0x81, 0xe1, 0x4e, 0x41, 0xf8,
                0xda, 0x4c, 0xdd, 0xf2,
            ],
            Bitcoin::RightPadHigh1_64 => [
                0x3d, 0x6a, 0x7f, 0xe6, 0x9a, 0x11, 0x64, 0x2a, 0xce, 0xd6, 0x84, 0x2b, 0x89, 0xaa,
                0x1b, 0xb8, 0x41, 0x3e, 0x39, 0x90, 0x63, 0xcc, 0x16, 0x78, 0x6a, 0xf7, 0xc0, 0x33,
                0xda, 0xd5, 0x8b, 0x95,
            ],
            Bitcoin::RightPadHigh1_8 => [
                0x28, 0x44, 0xbd, 0xfd, 0x6a, 0xba, 0x29, 0xdf, 0x03, 0xf9, 0x3a, 0xa6, 0xae, 0xb2,
                0x1c, 0x06, 0x40, 0x28, 0xdb, 0x05, 0xff, 0x77, 0xd8, 0xd9, 0x1c, 0xfd, 0xcd, 0xef,
                0xb1, 0x90, 0xc5, 0xbd,
            ],
            Bitcoin::RightPadHigh32_64 => [
                0xb4, 0x32, 0xe5, 0x32, 0x1a, 0xe1, 0x71, 0x4c, 0xe1, 0x95, 0x29, 0xd8, 0x5f, 0x24,
                0xff, 0x89, 0x87, 0x91, 0x0e, 0xbc, 0xf0, 0x15, 0xf8, 0x7f, 0x15, 0xbb, 0xed, 0x55,
                0xf0, 0xa0, 0xe8, 0x92,
            ],
            Bitcoin::RightPadHigh8_16 => [
                0x6f, 0x2d, 0x96, 0xc9, 0x54, 0x13, 0xca, 0x9a, 0xa8, 0xcc, 0x55, 0x0f, 0x25, 0x73,
                0xe1, 0x66, 0x99, 0x56, 0xd6, 0x07, 0x69, 0x2c, 0xf1, 0xca, 0x6d, 0xc7, 0x6d, 0x2f,
                0x2b, 0x4a, 0x3a, 0xc8,
            ],
            Bitcoin::RightPadHigh8_32 => [
                0xdf, 0x2c, 0x7f, 0x92, 0x99, 0x00, 0xa4, 0x49, 0x01, 0xe6, 0xff, 0x65, 0x27, 0x6a,
                0x95, 0x1a, 0xeb, 0x95, 0xdf, 0x25, 0x0b, 0x13, 0x97, 0x14, 0xd4, 0x19, 0x54, 0x04,
                0xd7, 0x78, 0x98, 0xed,
            ],
            Bitcoin::RightPadHigh8_64 => [
                0x79, 0xc0, 0x1d, 0xa3, 0xe6, 0x0b, 0x9c, 0x69, 0x35, 0xce, 0x3e, 0x15, 0x98, 0xb1,
                0x78, 0x40, 0xaf, 0x82, 0xdc, 0xb0, 0xdd, 0xc6, 0x3a, 0xef, 0x4a, 0x06, 0xe7, 0xf9,
                0xca, 0x5d, 0x27, 0x41,
            ],
            Bitcoin::RightPadLow16_32 => [
                0x6f, 0x20, 0x10, 0x27, 0xcc, 0x75, 0x98, 0x02, 0x30, 0xa0, 0x70, 0x85, 0x9c, 0x3e,
                0x38, 0x02, 0x36, 0xa1, 0xcb, 0x10, 0xe6, 0x1a, 0x01, 0xaa, 0x1f, 0x6d, 0x23, 0x1d,
                0x15, 0x14, 0x2f, 0x25,
            ],
            Bitcoin::RightPadLow16_64 => [
                0xb8, 0x6e, 0x1f, 0x0b, 0xfe, 0xc6, 0x55, 0x98, 0xd0, 0xa3, 0xd1, 0xec, 0x96, 0x03,
                0x05, 0xb9, 0x67, 0x45, 0x67, 0x3e, 0x1b, 0x16, 0xbf, 0x32, 0x7a, 0x71, 0x68, 0x05,
                0x83, 0xd7, 0x1d, 0x90,
            ],
            Bitcoin::RightPadLow1_16 => [
                0x05, 0x2a, 0x64, 0x99, 0xc9, 0x3e, 0xe6, 0xbc, 0x1a, 0xe6, 0x57, 0xf8, 0x5f, 0xd4,
                0xd4, 0xfe, 0x67, 0x7a, 0xbc, 0xee, 0x54, 0x0d, 0x13, 0x40, 0x33, 0x54, 0x2e, 0x9a,
                0xb6, 0x0a, 0x63, 0xdd,
            ],
            Bitcoin::RightPadLow1_32 => [
                0x5b, 0x70, 0xd4, 0x28, 0x96, 0x0e, 0x95, 0xcc, 0x40, 0xd5, 0x18, 0x46, 0xf5, 0x3a,
                0x4d, 0x0a, 0x35, 0xc9, 0x01, 0x5d, 0x15, 0x00, 0xb6, 0xbc, 0x84, 0x9b, 0x72, 0x83,
                0x5e, 0x2b, 0xd4, 0x40,
            ],
            Bitcoin::RightPadLow1_64 => [
                0x44, 0xef, 0xeb, 0x87, 0xca, 0x2a, 0xd7, 0xfd, 0x4b, 0x73, 0xf1, 0x63, 0x07, 0xc7,
                0xf0, 0x59, 0x02, 0x65, 0x6f, 0x35, 0x09, 0x0f, 0xb0, 0xa4, 0x32, 0x6c, 0x64, 0x89,
                0x88, 0xae, 0x1d, 0x39,
            ],
            Bitcoin::RightPadLow1_8 => [
                0x93, 0x40, 0x39, 0x8b, 0xcc, 0x8e, 0xa8, 0x3e, 0xc8, 0x40, 0xbe, 0x72, 0x9d, 0xbb,
                0x8b, 0x81, 0x20, 0x78, 0x24, 0xee, 0x87, 0x5d, 0x15, 0x82, 0x59, 0xd6, 0xda, 0xd2,
                0x0a, 0x83, 0x93, 0x0c,
            ],
            Bitcoin::RightPadLow32_64 => [
                0x69, 0x3e, 0x28, 0x10, 0x1e, 0x04, 0xfd, 0xa4, 0x3b, 0x97, 0xe6, 0x11, 0xf0, 0xfe,
                0x98, 0x00, 0x0e, 0x14, 0x30, 0x2e, 0x5d, 0xcd, 0x6e, 0xd6, 0x5e, 0xee, 0x42, 0xe3,
                0x40, 0x14, 0x24, 0x2f,
            ],
            Bitcoin::RightPadLow8_16 => [
                0x09, 0x6b, 0x25, 0xc3, 0xc8, 0x41, 0x5f, 0x04, 0xd8, 0x83, 0x27, 0x43, 0xeb, 0x2f,
                0x84, 0x56, 0xd5, 0xf0, 0xa6, 0x44, 0x91, 0x3d, 0x3e, 0xc5, 0x9d, 0x34, 0xf4, 0x55,
                0x25, 0x01, 0xfa, 0x20,
            ],
            Bitcoin::RightPadLow8_32 => [
                0xfc, 0x7f, 0x57, 0x22, 0xa6, 0x2a, 0xa2, 0x20, 0x18, 0xcc, 0x81, 0xcd, 0x00, 0xa9,
                0x32, 0x6c, 0x7f, 0xe9, 0xc6, 0x3a, 0xbc, 0xe2, 0xbd, 0xa4, 0xc0, 0xe6, 0x6a, 0x3f,
                0x47, 0xc6, 0x7c, 0x53,
            ],
            Bitcoin::RightPadLow8_64 => [
                0xa5, 0xbb, 0x7d, 0x5e, 0xfc, 0xa0, 0xe4, 0x8d, 0x9d, 0x80, 0xc5, 0x02, 0x71, 0x15,
                0xb4, 0x85, 0x78, 0x10, 0x51, 0xe0, 0xef, 0x46, 0xe4, 0xd6, 0x08, 0x31, 0x7a, 0x1c,
                0x42, 0x61, 0xbc, 0x46,
            ],
            Bitcoin::RightRotate16 => [
                0x48, 0x2e, 0xa7, 0xe1, 0x21, 0x45, 0x01, 0xd9, 0x3c, 0x9a, 0xd1, 0x6f, 0xa8, 0xb9,
                0x7b, 0xf5, 0xb3, 0x84, 0xfc, 0x2b, 0x54, 0x78, 0x9b, 0x8c, 0xd9, 0xe7, 0x84, 0xcc,
                0xd0, 0xeb, 0x9d, 0x57,
            ],
            Bitcoin::RightRotate32 => [
                0x09, 0x41, 0xb6, 0xee, 0xea, 0x9a, 0xf8, 0x19, 0x5b, 0x02, 0x8a, 0xfc, 0x0b, 0xd2,
                0xa5, 0x34, 0x21, 0x8b, 0xf9, 0x0d, 0x1a, 0x0e, 0x37, 0x3d, 0x74, 0x74, 0x18, 0x54,
                0x0b, 0x72, 0x6d, 0x73,
            ],
            Bitcoin::RightRotate64 => [
                0x44, 0x4d, 0xbb, 0xc3, 0xdd, 0x2a, 0x11, 0xa5, 0xc7, 0xb0, 0x43, 0x9f, 0xdb, 0xa9,
                0x9a, 0xc7, 0x4a, 0x11, 0xb8, 0xee, 0xb2, 0xdb, 0x30, 0x1e, 0x24, 0x3e, 0xa8, 0x91,
                0x22, 0x90, 0x71, 0x52,
            ],
            Bitcoin::RightRotate8 => [
                0x72, 0x65, 0xa3, 0x0c, 0x2e, 0x83, 0x6e, 0x65, 0x54, 0x4a, 0xba, 0x91, 0x1b, 0x64,
                0xd1, 0x8f, 0xa6, 0x9b, 0x17, 0x65, 0x45, 0x85, 0x6c, 0x77, 0xc4, 0xf0, 0xd7, 0x6f,
                0xc3, 0xf5, 0x83, 0x51,
            ],
            Bitcoin::RightShift16 => [
                0xcd, 0x57, 0xa3, 0xd3, 0xab, 0x2d, 0x92, 0xd4, 0xf0, 0x86, 0x55, 0x04, 0x3a, 0x8b,
                0x8b, 0xb6, 0x73, 0x89, 0x81, 0xfa, 0xe6, 0xda, 0x01, 0x34, 0xb4, 0xde, 0xda, 0xce,
                0x5f, 0x00, 0x88, 0x60,
            ],
            Bitcoin::RightShift32 => [
                0xd6, 0xb3, 0x26, 0xb1, 0xa3, 0x23, 0x57, 0xa3, 0x32, 0x80, 0x7d, 0x3f, 0xa1, 0xb1,
                0x56, 0xc2, 0x8b, 0x16, 0x22, 0xf7, 0x38, 0xde, 0xf1, 0x26, 0x81, 0x46, 0x7f, 0x34,
                0x9b, 0xd3, 0x49, 0x4b,
            ],
            Bitcoin::RightShift64 => [
                0xb2, 0x09, 0x5f, 0x2d, 0x47, 0x33, 0x5d, 0x5f, 0x98, 0xc8, 0x54, 0x34, 0xa2, 0xfa,
                0xf5, 0xb0, 0xf7, 0x5c, 0xf8, 0x99, 0x01, 0x2a, 0x34, 0xbb, 0xcd, 0x0a, 0x14, 0xcb,
                0xed, 0xb6, 0x11, 0x07,
            ],
            Bitcoin::RightShift8 => [
                0x4b, 0x2b, 0x1a, 0xa2, 0xef, 0x73, 0x21, 0x73, 0x17, 0x0d, 0x62, 0x1a, 0x38, 0xde,
                0xb2, 0x61, 0xe4, 0x73, 0xc0, 0x7c, 0x55, 0x8b, 0x05, 0x5a, 0x25, 0xa8, 0x6e, 0x4e,
                0x32, 0x1a, 0xfc, 0x04,
            ],
            Bitcoin::RightShiftWith16 => [
                0x14, 0xb7, 0x76, 0x85, 0x47, 0xb3, 0xd3, 0xf4, 0x7e, 0xe5, 0xc2, 0xb8, 0x0d, 0x9b,
                0xda, 0xe2, 0xae, 0xc1, 0xf9, 0xc6, 0x59, 0x4e, 0xd3, 0x12, 0x7b, 0x12, 0x64, 0x5a,
                0xdc, 0xf5, 0x97, 0x54,
            ],
            Bitcoin::RightShiftWith32 => [
                0x32, 0x7b, 0x6e, 0x98, 0xa6, 0xfd, 0x34, 0x0c, 0x60, 0xcf, 0x83, 0xaa, 0x64, 0x99,
                0x33, 0x11, 0x4c, 0xb8, 0xd8, 0x4f, 0x59, 0x0e, 0x01, 0x21, 0x3a, 0x26, 0x10, 0x01,
                0x2b, 0x46, 0x07, 0xea,
            ],
            Bitcoin::RightShiftWith64 => [
                0x06, 0x2f, 0xa7, 0x4a, 0xf3, 0x47, 0x6e, 0x59, 0x38, 0x7b, 0xe0, 0x8e, 0x69, 0x49,
                0xa0, 0x05, 0x43, 0xbc, 0x84, 0xa2, 0xb6, 0x89, 0xea, 0x39, 0xad, 0x6e, 0xed, 0x7f,
                0x75, 0x67, 0x85, 0xd4,
            ],
            Bitcoin::RightShiftWith8 => [
                0x14, 0x1b, 0xe4, 0x7e, 0x96, 0x7b, 0x2f, 0xd7, 0xc7, 0x12, 0x6c, 0x5a, 0xdf, 0x2d,
                0xfe, 0x47, 0x31, 0x5b, 0xbc, 0x10, 0x53, 0xbb, 0xe6, 0x05, 0xb3, 0x88, 0x98, 0xdb,
                0xed, 0x49, 0xf2, 0x27,
            ],
            Bitcoin::Rightmost16_1 => [
                0x3f, 0x3c, 0x43, 0x46, 0x87, 0x17, 0x42, 0x26, 0x5e, 0x87, 0xf0, 0x01, 0xb4, 0x6d,
                0xe7, 0xd1, 0x98, 0x75, 0x1b, 0x34, 0xfa, 0xa1, 0x80, 0x18, 0xde, 0x60, 0xc8, 0x46,
                0x8d, 0x9b, 0x98, 0xa4,
            ],
            Bitcoin::Rightmost16_2 => [
                0xc1, 0x8b, 0x9f, 0xdd, 0x34, 0x0a, 0x26, 0x7a, 0xc1, 0x6d, 0x4f, 0x39, 0xee, 0x75,
                0x43, 0x56, 0x52, 0xaa, 0xca, 0x52, 0x56, 0x50, 0xb5, 0x1a, 0x45, 0x87, 0x98, 0x04,
                0x8e, 0x62, 0x7d, 0x51,
            ],
            Bitcoin::Rightmost16_4 => [
                0xc6, 0xc5, 0x3f, 0xa7, 0x1e, 0x23, 0x0c, 0xf0, 0x58, 0x51, 0x58, 0xf4, 0x70, 0x58,
                0x8b, 0xac, 0x5c, 0x51, 0x8f, 0x84, 0xf9, 0xfc, 0x23, 0x86, 0x52, 0xf1, 0x75, 0xfb,
                0x6e, 0xa1, 0x8c, 0x11,
            ],
            Bitcoin::Rightmost16_8 => [
                0xee, 0x76, 0x9c, 0x1c, 0xc8, 0xa3, 0xfd, 0xd1, 0x83, 0x8f, 0xc9, 0xf0, 0x49, 0x0c,
                0xe7, 0x03, 0x93, 0xfd, 0x91, 0xba, 0x3c, 0xbd, 0x4a, 0xbd, 0x08, 0x64, 0x9f, 0xb9,
                0xc4, 0x43, 0x11, 0xbd,
            ],
            Bitcoin::Rightmost32_1 => [
                0x1c, 0x44, 0x23, 0x69, 0xfb, 0x81, 0xf6, 0x11, 0xd3, 0x28, 0x01, 0x0b, 0x86, 0x4b,
                0xcc, 0xb7, 0xf3, 0x5e, 0xd4, 0x77, 0xdf, 0xa3, 0x85, 0x55, 0x74, 0xc1, 0x35, 0x64,
                0xcd, 0xbd, 0xb8, 0x60,
            ],
            Bitcoin::Rightmost32_16 => [
                0xad, 0xd2, 0xc3, 0x39, 0x0d, 0x9a, 0xf7, 0xc2, 0x4a, 0x15, 0x9a, 0x37, 0xd6, 0x9d,
                0x44, 0x84, 0xd2, 0xc2, 0x4a, 0x2c, 0xb5, 0xb0, 0xeb, 0x2d, 0x3c, 0x49, 0x3d, 0x98,
                0x12, 0xac, 0xfd, 0x74,
            ],
            Bitcoin::Rightmost32_2 => [
                0x00, 0xb8, 0x81, 0x5a, 0xd7, 0x42, 0x3d, 0xd5, 0x8c, 0xb9, 0x8b, 0xe8, 0x2c, 0xad,
                0x26, 0x67, 0x5c, 0x3b, 0xf5, 0x4a, 0x0b, 0xed, 0xba, 0xde, 0x34, 0x64, 0xb4, 0xfe,
                0x5a, 0x4e, 0x8c, 0xe6,
            ],
            Bitcoin::Rightmost32_4 => [
                0x84, 0xfa, 0x5a, 0x54, 0xf7, 0x72, 0x9f, 0x9d, 0x68, 0x99, 0x4b, 0xea, 0xb9, 0x3a,
                0xe7, 0x9b, 0x8c, 0x4a, 0x10, 0xd5, 0xb7, 0xae, 0x97, 0x27, 0xaa, 0x17, 0x16, 0xe5,
                0x7d, 0x03, 0x3b, 0x74,
            ],
            Bitcoin::Rightmost32_8 => [
                0x7d, 0x38, 0x05, 0xd3, 0xc7, 0x8c, 0x4e, 0xea, 0x91, 0xe3, 0xd3, 0x5e, 0xfd, 0xd4,
                0x7e, 0xed, 0xd4, 0x21, 0xaf, 0x84, 0xd2, 0x19, 0x10, 0x32, 0x93, 0x32, 0xa0, 0xb5,
                0x48, 0x7f, 0xab, 0x63,
            ],
            Bitcoin::Rightmost64_1 => [
                0xd3, 0xb1, 0x64, 0xc5, 0xdc, 0x66, 0xcc, 0x7e, 0xf9, 0x23, 0x4f, 0xed, 0xe4, 0xdc,
                0x7f, 0x0d, 0xa5, 0xcd, 0x71, 0xc1, 0xc1, 0xd4, 0xca, 0xd6, 0x0f, 0xb4, 0xec, 0x57,
                0x3e, 0x2b, 0x8a, 0x75,
            ],
            Bitcoin::Rightmost64_16 => [
                0xea, 0xe4, 0x34, 0x78, 0xf9, 0xf2, 0xf4, 0x52, 0xef, 0xac, 0x15, 0xee, 0xe6, 0x0f,
                0x8b, 0x52, 0x53, 0xd8, 0x0a, 0x2d, 0x32, 0x12, 0x9b, 0x4e, 0x5b, 0xa3, 0x83, 0x00,
                0xad, 0x98, 0x52, 0xfd,
            ],
            Bitcoin::Rightmost64_2 => [
                0x9c, 0xd4, 0xa9, 0x8b, 0xbd, 0xb8, 0xa3, 0x35, 0x85, 0xc0, 0x0f, 0x47, 0xd6, 0xad,
                0xab, 0x7a, 0xf5, 0x42, 0x86, 0xfb, 0x8a, 0xe6, 0x0f, 0x72, 0x30, 0x11, 0xfb, 0x84,
                0xc0, 0xee, 0x78, 0xf9,
            ],
            Bitcoin::Rightmost64_32 => [
                0x7f, 0x24, 0x20, 0xae, 0x5b, 0x0f, 0x5a, 0x3f, 0x6f, 0x2e, 0x60, 0xb6, 0x1f, 0x8a,
                0x41, 0x5c, 0x08, 0x8b, 0x94, 0xb2, 0x1c, 0x1a, 0x62, 0xa3, 0xfd, 0xaa, 0xc7, 0x49,
                0xdb, 0xdf, 0x4c, 0x71,
            ],
            Bitcoin::Rightmost64_4 => [
                0xe2, 0x65, 0x55, 0x2a, 0x24, 0xfb, 0xcd, 0xec, 0x05, 0x83, 0xd7, 0x18, 0x3e, 0x48,
                0xeb, 0xc2, 0xff, 0x6d, 0x31, 0x65, 0x57, 0xba, 0xc5, 0x91, 0x5c, 0x03, 0xcb, 0x23,
                0x35, 0xd2, 0x32, 0x95,
            ],
            Bitcoin::Rightmost64_8 => [
                0x98, 0xcd, 0x95, 0xf9, 0x5d, 0x46, 0x64, 0x1b, 0x04, 0x9e, 0x77, 0xbf, 0x90, 0xee,
                0xa5, 0x98, 0xad, 0xf2, 0x9e, 0xe5, 0x00, 0xe6, 0x50, 0x72, 0x87, 0x54, 0x8b, 0xb1,
                0xcd, 0xaf, 0x78, 0x4d,
            ],
            Bitcoin::Rightmost8_1 => [
                0x08, 0x76, 0xfc, 0xd4, 0x69, 0x85, 0x91, 0xf3, 0x31, 0x91, 0x01, 0x57, 0x4c, 0xe1,
                0x53, 0xfc, 0xdf, 0xe9, 0x4f, 0x58, 0x1a, 0xac, 0x5e, 0x75, 0xf3, 0xcd, 0x74, 0x46,
                0xdf, 0x56, 0xf3, 0xc7,
            ],
            Bitcoin::Rightmost8_2 => [
                0xb9, 0xf7, 0xb2, 0x90, 0xaf, 0xe7, 0xf1, 0x89, 0xe3, 0x2a, 0xeb, 0xf2, 0xcc, 0x4d,
                0xdc, 0xa9, 0x6b, 0xb0, 0x07, 0x64, 0xc7, 0xbe, 0x28, 0x87, 0xdc, 0xe0, 0x54, 0xd0,
                0x9e, 0x38, 0xc3, 0x53,
            ],
            Bitcoin::Rightmost8_4 => [
                0xf2, 0x8e, 0x9a, 0xf5, 0xaf, 0x4c, 0x9c, 0xca, 0x4b, 0x43, 0xcc, 0x6a, 0xdf, 0x9d,
                0x9d, 0x8d, 0x16, 0x9c, 0x87, 0xc5, 0x55, 0x9f, 0x9f, 0x3c, 0xca, 0xc8, 0xf2, 0x35,
                0x2b, 0x62, 0x9f, 0x18,
            ],
            Bitcoin::ScalarAdd => [
                0x11, 0xdd, 0xbe, 0xba, 0xeb, 0xf4, 0x21, 0x80, 0xa0, 0xb7, 0xed, 0xdf, 0xfd, 0xc4,
                0x8e, 0xc7, 0x51, 0x13, 0x30, 0xfb, 0x33, 0x15, 0xfa, 0x65, 0xd5, 0x8a, 0xff, 0x66,
                0xb9, 0xca, 0xf2, 0xd4,
            ],
            Bitcoin::ScalarInvert => [
                0xa6, 0x39, 0x27, 0x25, 0xbb, 0x2d, 0xad, 0xbb, 0x1e, 0x76, 0xdf, 0x2d, 0xec, 0x57,
                0xdf, 0x55, 0xc3, 0xfc, 0xc5, 0x77, 0x3b, 0x62, 0x21, 0x8a, 0xec, 0x55, 0xa7, 0x5e,
                0x14, 0xf3, 0xd6, 0x0d,
            ],
            Bitcoin::ScalarIsZero => [
                0xf7, 0x5e, 0xda, 0x06, 0xce, 0x6a, 0xf0, 0x9f, 0xae, 0x37, 0xdb, 0x4e, 0x62, 0x25,
                0xe6, 0xa8, 0xac, 0x86, 0xa2, 0x36, 0x37, 0x62, 0x7d, 0x62, 0x64, 0x09, 0x19, 0x0f,
                0xf3, 0xb3, 0x9d, 0x90,
            ],
            Bitcoin::ScalarMultiply => [
                0x4a, 0x61, 0x67, 0x2a, 0xce, 0xc4, 0x88, 0x77, 0x56, 0xde, 0x1d, 0xb6, 0x04, 0x21,
                0xa1, 0x2b, 0x90, 0x1a, 0x85, 0x8a, 0x6e, 0xe6, 0x35, 0x2e, 0x55, 0x9d, 0x4c, 0xe5,
                0x97, 0x33, 0x52, 0xbe,
            ],
            Bitcoin::ScalarMultiplyLambda => [
                0x49, 0xea, 0x9c, 0x3f, 0xb1, 0xd8, 0xff, 0x52, 0xd2, 0xdb, 0x03, 0x46, 0x9f, 0xdf,
                0xe8, 0x50, 0x50, 0x3f, 0xdd, 0xeb, 0x45, 0xe1, 0x6d, 0x26, 0xe8, 0x92, 0x8a, 0xdd,
                0x25, 0x87, 0x0e, 0x91,
            ],
            Bitcoin::ScalarNegate => [
                0x1d, 0xbf, 0x8b, 0x49, 0x1e, 0xc6, 0x65, 0x80, 0x3f, 0x63, 0x33, 0x30, 0xd3, 0xff,
                0xb0, 0xe7, 0x81, 0xe6, 0x7c, 0x18, 0x01, 0xac, 0x9d, 0x49, 0xbb, 0xf4, 0x35, 0x89,
                0xab, 0xf7, 0x82, 0xbf,
            ],
            Bitcoin::ScalarNormalize => [
                0x46, 0x33, 0x18, 0x0e, 0xa0, 0x2c, 0x4d, 0xf7, 0x81, 0x9d, 0x3d, 0x54, 0xa4, 0x01,
                0x73, 0x4f, 0x96, 0x5b, 0x31, 0xac, 0xc7, 0x84, 0x05, 0x4e, 0xbf, 0xb7, 0x31, 0x68,
                0x16, 0xb0, 0x29, 0xec,
            ],
            Bitcoin::ScalarSquare => [
                0x8a, 0x27, 0x9e, 0x6f, 0x61, 0x3a, 0xa9, 0xe9, 0x34, 0xf2, 0xf2, 0xa3, 0x43, 0xc0,
                0xd3, 0x29, 0x1c, 0x36, 0x70, 0xe2, 0x97, 0xdd, 0xae, 0x20, 0x52, 0x9e, 0x82, 0x50,
                0x69, 0xef, 0xea, 0x0e,
            ],
            Bitcoin::Scale => [
                0x12, 0x6e, 0x22, 0x12, 0x5b, 0xac, 0x80, 0xb9, 0x9b, 0x7b, 0x73, 0x43, 0xb4, 0xe5,
                0xe5, 0x86, 0x60, 0x82, 0x16, 0x10, 0x5d, 0x4d, 0xe6, 0xf7, 0x94, 0xad, 0xd3, 0x4e,
                0x23, 0xb1, 0x95, 0xca,
            ],
            Bitcoin::ScriptCMR => [
                0xa8, 0xa4, 0xa6, 0x22, 0x10, 0xb5, 0xe4, 0x95, 0x0e, 0x25, 0x34, 0x24, 0x7c, 0x74,
                0x11, 0xd1, 0xc8, 0xff, 0x22, 0x86, 0x5b, 0x54, 0x56, 0xbb, 0xb2, 0x16, 0x38, 0xe9,
                0x14, 0xf5, 0xe5, 0x28,
            ],
            Bitcoin::Sha256Block => [
                0x45, 0x35, 0xf3, 0xe1, 0xab, 0x9f, 0x1b, 0x75, 0x7a, 0x06, 0x91, 0x37, 0xe1, 0xd5,
                0xb1, 0xca, 0xad, 0x8e, 0x31, 0xf7, 0x8d, 0xc5, 0xfb, 0xd0, 0x73, 0x46, 0x49, 0xf9,
                0x40, 0xa7, 0xfc, 0x96,
            ],
            Bitcoin::Sha256Ctx8Add1 => [
                0x9a, 0x47, 0x11, 0xb8, 0xc5, 0x69, 0x0e, 0x58, 0x7e, 0x5f, 0x79, 0xe6, 0x8d, 0x6e,
                0xca, 0x04, 0x74, 0x58, 0xaa, 0x63, 0xb8, 0xbc, 0x9e, 0xe5, 0x68, 0x08, 0x6a, 0x4a,
                0x1b, 0x56, 0xd8, 0x34,
            ],
            Bitcoin::Sha256Ctx8Add128 => [
                0x1c, 0xb1, 0xdb, 0x8a, 0x05, 0x5b, 0x31, 0x97, 0xac, 0xf0, 0xf0, 0x8c, 0xe9, 0xc6,
                0x35, 0xad, 0xd6, 0x95, 0xb6, 0x0f, 0x23, 0x4b, 0x18, 0xe0, 0xb3, 0x23, 0xc9, 0x37,
                0xb0, 0x38, 0x5a, 0xea,
            ],
            Bitcoin::Sha256Ctx8Add16 => [
                0xe0, 0x84, 0x54, 0x75, 0xeb, 0xb9, 0x01, 0x40, 0xfa, 0x4e, 0x01, 0xaf, 0x8a, 0x94,
                0x35, 0x99, 0x1a, 0xd8, 0x7a, 0xf9, 0x8c, 0x08, 0xae, 0xce, 0x11, 0x0e, 0x99, 0xcb,
                0xce, 0xcd, 0xee, 0x79,
            ],
            Bitcoin::Sha256Ctx8Add2 => [
                0x7d, 0x69, 0x13, 0x8f, 0x1c, 0x94, 0x2b, 0xee, 0x2f, 0xdf, 0x60, 0x0c, 0xe4, 0x4b,
                0x36, 0xff, 0x97, 0x83, 0x9d, 0xc2, 0xbb, 0xda, 0xfb, 0xd5, 0xfa, 0xb4, 0xdf, 0xbc,
                0x3c, 0x97, 0x6f, 0x29,
            ],
            Bitcoin::Sha256Ctx8Add256 => [
                0x4f, 0x5c, 0x29, 0xd5, 0x36, 0x86, 0xc0, 0x60, 0x62, 0xb3, 0x83, 0x24, 0xf8, 0xaf,
                0xf1, 0x7e, 0xc5, 0x56, 0xa2, 0x95, 0xff, 0x09, 0x8b, 0x10, 0xe7, 0x05, 0xdd, 0x22,
                0xe1, 0x3b, 0xc3, 0xc9,
            ],
            Bitcoin::Sha256Ctx8Add32 => [
                0xd5, 0x7b, 0x67, 0xb1, 0x74, 0xe7, 0x8e, 0x38, 0xf9, 0xbc, 0xa8, 0xe0, 0x7a, 0xdd,
                0x61, 0xc7, 0x53, 0xe2, 0xc1, 0x56, 0xd8, 0xe9, 0x83, 0x2a, 0xa6, 0x62, 0x04, 0x55,
                0x00, 0xf5, 0x1a, 0x80,
            ],
            Bitcoin::Sha256Ctx8Add4 => [
                0x95, 0xda, 0x32, 0x99, 0x3f, 0x5c, 0x7d, 0x00, 0x83, 0x06, 0x4c, 0xdf, 0xf1, 0xbe,
                0xc3, 0xb9, 0x36, 0xc6, 0x38, 0x33, 0x7a, 0xde, 0xc5, 0x47, 0x48, 0x7a, 0xf2, 0x32,
                0xd6, 0x9f, 0xdf, 0x65,
            ],
            Bitcoin::Sha256Ctx8Add512 => [
                0x4a, 0xcb, 0x16, 0x3a, 0xa4, 0x8f, 0x09, 0xd5, 0xf2, 0x6d, 0x2b, 0x2a, 0xb1, 0x88,
                0xa6, 0xc6, 0xb6, 0xc4, 0xae, 0xdf, 0x23, 0xc9, 0x19, 0x00, 0x1c, 0x02, 0xee, 0x15,
                0xb3, 0x37, 0xa9, 0x6e,
            ],
            Bitcoin::Sha256Ctx8Add64 => [
                0x52, 0xe5, 0x3e, 0xc5, 0x77, 0x0f, 0x9b, 0xe4, 0x06, 0x9a, 0xee, 0xfc, 0xb2, 0x13,
                0x22, 0xb1, 0x3a, 0xb6, 0xe3, 0x94, 0x1f, 0xdc, 0x2c, 0x85, 0xf4, 0xb4, 0x1b, 0xe6,
                0x7d, 0x38, 0xea, 0x7e,
            ],
            Bitcoin::Sha256Ctx8Add8 => [
                0xc2, 0x6b, 0x28, 0xaf, 0xe5, 0xe8, 0x66, 0xd8, 0x46, 0x16, 0x81, 0x4d, 0x1a, 0x13,
                0xfb, 0x86, 0x30, 0xb9, 0xe8, 0x4e, 0x5d, 0x78, 0x15, 0x56, 0xc6, 0xd8, 0x23, 0x6e,
                0xfb, 0x45, 0xdf, 0xf9,
            ],
            Bitcoin::Sha256Ctx8AddBuffer511 => [
                0xad, 0x69, 0x90, 0x46, 0x48, 0xa8, 0x23, 0x8d, 0x00, 0xd8, 0x51, 0x63, 0xfc, 0xe8,
                0x19, 0x63, 0xa0, 0x04, 0x7a, 0xb5, 0x82, 0xbe, 0x97, 0xa4, 0x14, 0x00, 0x65, 0x59,
                0x79, 0xcf, 0xdd, 0x28,
            ],
            Bitcoin::Sha256Ctx8Finalize => [
                0x8e, 0x45, 0xbd, 0xc3, 0x87, 0xd4, 0xed, 0xfa, 0x73, 0x35, 0x25, 0xf3, 0xab, 0x19,
                0xe4, 0x2b, 0x58, 0xec, 0xb1, 0xb5, 0xf6, 0xdc, 0xcf, 0x94, 0xed, 0xbf, 0x59, 0x95,
                0x8a, 0xe3, 0xe1, 0x16,
            ],
            Bitcoin::Sha256Ctx8Init => [
                0x63, 0x5f, 0x64, 0x05, 0x84, 0x86, 0x85, 0xc0, 0x11, 0xfe, 0xbd, 0x41, 0xfa, 0xac,
                0x87, 0x4b, 0xbb, 0xf5, 0xb2, 0x4d, 0x5f, 0xb1, 0x2f, 0xed, 0xbc, 0xb6, 0xcb, 0xff,
                0x95, 0xa0, 0xf3, 0x66,
            ],
            Bitcoin::Sha256Iv => [
                0x12, 0xe4, 0x59, 0x37, 0x51, 0xc9, 0x46, 0x3b, 0x56, 0x25, 0x03, 0xc1, 0x40, 0xd7,
                0x8b, 0x3b, 0x75, 0x7a, 0x1f, 0x4f, 0x16, 0x32, 0x1d, 0x28, 0x62, 0xd3, 0x25, 0x43,
                0x85, 0x38, 0x97, 0x1b,
            ],
            Bitcoin::SigAllHash => [
                0x09, 0x78, 0xb9, 0xe5, 0x0b, 0x9e, 0x8e, 0x09, 0x8b, 0x27, 0xf2, 0xb8, 0xb5, 0x9d,
                0xe5, 0x4f, 0x62, 0xba, 0x7c, 0x13, 0x33, 0xdf, 0x3b, 0xed, 0x22, 0x1e, 0x26, 0x62,
                0x68, 0x05, 0xbc, 0x55,
            ],
            Bitcoin::Some1 => [
                0x15, 0xca, 0x4e, 0x4b, 0x82, 0xc2, 0xf9, 0x1b, 0x9a, 0x79, 0x29, 0x92, 0xcd, 0xc1,
                0xb2, 0x92, 0xab, 0x86, 0xa2, 0xd2, 0x93, 0x9c, 0x9a, 0x64, 0xb5, 0x0b, 0xe6, 0x0b,
                0xda, 0x6a, 0xb4, 0xca,
            ],
            Bitcoin::Some16 => [
                0xa9, 0xdf, 0xbb, 0xea, 0xb5, 0x9d, 0xf7, 0x2a, 0x45, 0xfc, 0x3f, 0xc7, 0xac, 0x58,
                0x1e, 0xc8, 0xda, 0x71, 0x3f, 0x2f, 0x81, 0x03, 0xf7, 0x87, 0xaa, 0x1c, 0xee, 0x4e,
                0x0b, 0xa6, 0x48, 0x66,
            ],
            Bitcoin::Some32 => [
                0x46, 0x33, 0xa3, 0x97, 0x74, 0x2e, 0xf4, 0x82, 0xbe, 0x2f, 0xa3, 0xfb, 0x64, 0x10,
                0xec, 0x79, 0xc3, 0x73, 0x83, 0x65, 0x69, 0xfb, 0xbc, 0xb1, 0xf9, 0x48, 0xec, 0x32,
                0x48, 0x73, 0x78, 0xb7,
            ],
            Bitcoin::Some64 => [
                0x1d, 0xc2, 0x45, 0xac, 0x6f, 0x5b, 0x42, 0x2b, 0xd1, 0x88, 0x6e, 0xf5, 0x14, 0x4c,
                0x4d, 0xc7, 0x2c, 0x96, 0x73, 0x15, 0x59, 0x66, 0x07, 0x6c, 0xd8, 0x39, 0x68, 0x1d,
                0x9e, 0xc7, 0xf8, 0xf5,
            ],
            Bitcoin::Some8 => [
                0x33, 0xaf, 0xb9, 0xc6, 0x45, 0x4e, 0x59, 0x0e, 0xc1, 0x3e, 0xd7, 0x5e, 0x1b, 0x7d,
                0x9c, 0x3a, 0x3d, 0xe6, 0x75, 0x2b, 0xcc, 0x7c, 0x1d, 0x4c, 0xb3, 0x63, 0xfa, 0x51,
                0x82, 0x8b, 0xcb, 0x74,
            ],
            Bitcoin::Subtract16 => [
                0x4e, 0x06, 0xec, 0x31, 0x37, 0x62, 0x22, 0xe2, 0x5e, 0x27, 0xd0, 0x15, 0x9d, 0xc1,
                0xc0, 0x71, 0x4a, 0x44, 0xca, 0x6a, 0xac, 0xf9, 0x50, 0x5c, 0xaa, 0xd2, 0x80, 0xe9,
                0x73, 0xfb, 0x5c, 0xab,
            ],
            Bitcoin::Subtract32 => [
                0xb9, 0xc0, 0xf3, 0x6e, 0x75, 0x22, 0xa8, 0xd9, 0x49, 0x05, 0x0d, 0x51, 0x6a, 0x05,
                0xce, 0x20, 0x3a, 0x1f, 0x9a, 0x9e, 0x37, 0x2f, 0xd2, 0x63, 0xde, 0x38, 0xb0, 0xe9,
                0x03, 0x13, 0x41, 0x98,
            ],
            Bitcoin::Subtract64 => [
                0x1c, 0xdb, 0x5c, 0x74, 0xad, 0xd1, 0x02, 0xf5, 0x0f, 0x93, 0x8e, 0xd8, 0x86, 0xf4,
                0x96, 0xe5, 0xba, 0xb2, 0x75, 0x5c, 0x3c, 0x48, 0x4e, 0x88, 0x87, 0x90, 0x3d, 0x2f,
                0x6a, 0x57, 0xf3, 0xaa,
            ],
            Bitcoin::Subtract8 => [
                0x4f, 0x21, 0x17, 0xa0, 0xe8, 0x10, 0x59, 0xff, 0x0c, 0xd6, 0x4d, 0x84, 0x88, 0x65,
                0x42, 0xe5, 0x75, 0xea, 0x8d, 0x6e, 0xc0, 0x31, 0x08, 0xfd, 0x0b, 0x50, 0x8b, 0x39,
                0x20, 0x8c, 0xd0, 0xef,
            ],
            Bitcoin::Swu => [
                0x00, 0xf5, 0x1f, 0x4f, 0x4b, 0xec, 0xe7, 0x90, 0x03, 0xec, 0xad, 0x48, 0x1a, 0x12,
                0x5a, 0xf7, 0x17, 0x6e, 0x4d, 0xe9, 0x8c, 0x33, 0x92, 0x42, 0x5c, 0xb9, 0x14, 0x66,
                0x26, 0xc1, 0x3b, 0x3b,
            ],
            Bitcoin::TapEnvHash => [
                0x19, 0xd9, 0x94, 0x4c, 0x4d, 0x45, 0x7c, 0x70, 0xba, 0xbc, 0x45, 0xcc, 0xcb, 0xd5,
                0x73, 0xb3, 0x9a, 0x51, 0xd0, 0xc9, 0x91, 0x15, 0xb4, 0x12, 0x78, 0x3b, 0x49, 0x00,
                0x82, 0xfd, 0x0e, 0x58,
            ],
            Bitcoin::TapdataInit => [
                0xa4, 0xd0, 0x22, 0xef, 0x5c, 0xf4, 0x67, 0xbc, 0xa0, 0x32, 0x5e, 0x46, 0x3f, 0xca,
                0xce, 0x7c, 0xbd, 0xd6, 0x4f, 0xf8, 0xf7, 0x1c, 0x5c, 0x7f, 0x63, 0xe6, 0x07, 0x84,
                0xaa, 0x0a, 0xc4, 0x86,
            ],
            Bitcoin::TapleafHash => [
                0x0c, 0x07, 0x16, 0xfe, 0x5d, 0x97, 0x8e, 0xa8, 0xe0, 0xc7, 0x5a, 0xdc, 0x82, 0x10,
                0xd6, 0x60, 0x06, 0x2e, 0x3d, 0xa0, 0x6f, 0x1a, 0x66, 0x61, 0x31, 0x79, 0x27, 0xd3,
                0xb8, 0x4b, 0x50, 0x73,
            ],
            Bitcoin::TapleafVersion => [
                0xe2, 0xec, 0xb1, 0xcb, 0x0e, 0xd4, 0xed, 0x93, 0x48, 0x35, 0x45, 0xfd, 0x8a, 0x62,
                0xf8, 0xaa, 0x10, 0x17, 0x53, 0x49, 0xff, 0xcc, 0x5a, 0xd3, 0xde, 0x7f, 0x34, 0x84,
                0xea, 0x1f, 0x10, 0x3f,
            ],
            Bitcoin::Tappath => [
                0x99, 0xe8, 0x21, 0x1e, 0x8c, 0x1b, 0xe6, 0xd9, 0xca, 0x98, 0xd3, 0xd4, 0x3d, 0x91,
                0x42, 0x56, 0x7e, 0x06, 0xa8, 0x40, 0x22, 0x39, 0x33, 0x03, 0xfa, 0xb0, 0xb5, 0x7d,
                0x39, 0x40, 0x74, 0xe8,
            ],
            Bitcoin::TappathHash => [
                0x02, 0x11, 0x54, 0x6d, 0x07, 0x78, 0xe7, 0x87, 0x14, 0x1e, 0xce, 0x65, 0xdf, 0x6c,
                0xd1, 0xdb, 0x31, 0x38, 0x8f, 0xc1, 0x42, 0x19, 0x68, 0xc8, 0xcf, 0xd8, 0xd7, 0x59,
                0x26, 0xdb, 0x47, 0xb3,
            ],
            Bitcoin::TotalInputValue => [
                0x81, 0x28, 0x7a, 0x15, 0x89, 0xdd, 0xc2, 0x17, 0x99, 0x26, 0xb7, 0x06, 0x5e, 0x9b,
                0x26, 0x77, 0xb7, 0xfb, 0x09, 0x9b, 0x95, 0xf9, 0x47, 0xa7, 0xdd, 0x59, 0x0c, 0xdd,
                0x4c, 0xcf, 0x4a, 0x56,
            ],
            Bitcoin::TotalOutputValue => [
                0xba, 0x03, 0x2f, 0x3e, 0x62, 0xf8, 0xfc, 0xb0, 0x4b, 0x04, 0x29, 0xa2, 0x53, 0xb5,
                0xec, 0x57, 0xe4, 0xc8, 0x7a, 0xe2, 0xe9, 0x51, 0xe1, 0x45, 0xd7, 0x65, 0x0e, 0x7f,
                0x0c, 0x63, 0xe5, 0x55,
            ],
            Bitcoin::TransactionId => [
                0x5c, 0x49, 0xea, 0x98, 0x6b, 0x32, 0x8b, 0xe2, 0xa4, 0xe0, 0xb3, 0x15, 0xb6, 0xb3,
                0xfe, 0xf2, 0x3c, 0x1f, 0x68, 0x56, 0xbb, 0xc3, 0xb0, 0x56, 0xc9, 0x9c, 0xf5, 0x9f,
                0xbd, 0x54, 0x6e, 0x65,
            ],
            Bitcoin::TxHash => [
                0x54, 0xe5, 0x3c, 0x99, 0x93, 0xab, 0xd5, 0x5d, 0x1f, 0x85, 0x23, 0xd2, 0xbb, 0x21,
                0x7b, 0x32, 0xe6, 0xfe, 0x86, 0x1f, 0x84, 0xc9, 0x86, 0xb7, 0xee, 0x8b, 0xdc, 0x68,
                0x81, 0x06, 0x87, 0x4a,
            ],
            Bitcoin::TxIsFinal => [
                0x7b, 0x0e, 0x4f, 0x4c, 0xa8, 0xe5, 0xaf, 0x61, 0xa1, 0xd3, 0x45, 0x4e, 0x11, 0xef,
                0x9a, 0xb6, 0x88, 0x70, 0x61, 0x21, 0x1c, 0x00, 0x90, 0xeb, 0xa9, 0x55, 0x3d, 0xa2,
                0xe4, 0x5d, 0x84, 0x73,
            ],
            Bitcoin::TxLockDistance => [
                0xb6, 0xfb, 0xaa, 0xc2, 0x10, 0xa3, 0x06, 0xbe, 0x8b, 0x58, 0xd0, 0xd7, 0xb1, 0x56,
                0x3f, 0x62, 0x23, 0x36, 0xd2, 0xae, 0xb5, 0x6c, 0x39, 0x3d, 0x27, 0x64, 0x45, 0xa9,
                0xa2, 0x2c, 0xc4, 0xa7,
            ],
            Bitcoin::TxLockDuration => [
                0x53, 0x57, 0x28, 0x18, 0xe7, 0xe5, 0xb9, 0x8f, 0x96, 0x8c, 0x9d, 0xa7, 0xdf, 0x50,
                0x90, 0xd9, 0x82, 0x6f, 0x9b, 0xcf, 0x84, 0xb6, 0x36, 0x39, 0x5e, 0xea, 0x32, 0x1b,
                0x69, 0x09, 0xec, 0xe9,
            ],
            Bitcoin::TxLockHeight => [
                0x44, 0x9f, 0x61, 0xdf, 0x1a, 0x7b, 0xad, 0x8d, 0x9e, 0x0a, 0x96, 0x67, 0x14, 0x22,
                0x7e, 0x57, 0x10, 0x07, 0xf9, 0x3c, 0x51, 0x7b, 0x80, 0x5d, 0x61, 0x65, 0x10, 0xe8,
                0xff, 0x62, 0x17, 0x2b,
            ],
            Bitcoin::TxLockTime => [
                0x31, 0xdf, 0x36, 0x3f, 0x17, 0xb2, 0xcf, 0xc9, 0x7a, 0x1f, 0x93, 0x72, 0xc1, 0x4a,
                0x32, 0x58, 0x64, 0xd7, 0xcb, 0x13, 0xaa, 0x8d, 0x52, 0x15, 0xfb, 0x25, 0x3d, 0x33,
                0x10, 0x91, 0x77, 0x62,
            ],
            Bitcoin::Verify => [
                0xcd, 0xca, 0x2a, 0x05, 0xe5, 0x2c, 0xef, 0xa5, 0x9d, 0xc7, 0xa5, 0xb0, 0xda, 0xe2,
                0x20, 0x98, 0xfb, 0x89, 0x6e, 0x39, 0x13, 0xbf, 0xdd, 0x44, 0x6b, 0x59, 0x4e, 0x1f,
                0x92, 0x50, 0x78, 0x3e,
            ],
            Bitcoin::Version => [
                0x83, 0x73, 0x58, 0x64, 0x00, 0xb6, 0x79, 0x0b, 0x46, 0xab, 0x04, 0x10, 0x52, 0x3c,
                0xf0, 0x1e, 0xb7, 0x4d, 0x10, 0xfa, 0xf4, 0x8a, 0x3a, 0xcc, 0x86, 0xc4, 0xc5, 0x1d,
                0x06, 0xa5, 0x2c, 0x49,
            ],
            Bitcoin::Xor1 => [
                0x8c, 0x4e, 0x4e, 0x6e, 0xbf, 0x46, 0x30, 0xb2, 0x9b, 0x5a, 0x57, 0xea, 0x79, 0xf0,
                0xc9, 0xaf, 0x6b, 0xff, 0x54, 0xc4, 0xd2, 0xd7, 0x69, 0xbf, 0x51, 0x59, 0x47, 0x74,
                0xa5, 0x2b, 0x99, 0xc9,
            ],
            Bitcoin::Xor16 => [
                0xd9, 0xf0, 0xaf, 0x3f, 0xe3, 0xfd, 0x24, 0x7c, 0x1d, 0xf3, 0x4a, 0x25, 0x27, 0x13,
                0xb2, 0xe9, 0x33, 0xa9, 0x45, 0xa5, 0x67, 0x19, 0x48, 0x7f, 0x8e, 0xd7, 0xf5, 0x63,
                0xea, 0x86, 0x1a, 0xb5,
            ],
            Bitcoin::Xor32 => [
                0xd5, 0xae, 0x27, 0x12, 0xed, 0xea, 0xf6, 0x76, 0x52, 0x0f, 0xa3, 0xba, 0x0f, 0x40,
                0xbf, 0x4a, 0x16, 0x57, 0x43, 0x7e, 0xff, 0xbd, 0x99, 0x86, 0xd0, 0x6a, 0xe8, 0x1b,
                0x29, 0xa4, 0xf9, 0x8c,
            ],
            Bitcoin::Xor64 => [
                0xc4, 0xdf, 0x1c, 0xcf, 0x33, 0x3e, 0xde, 0xbd, 0xd4, 0x0d, 0xea, 0x9a, 0x0e, 0x6c,
                0xbb, 0x83, 0x06, 0x31, 0xe8, 0x3a, 0x94, 0xbb, 0x77, 0x9f, 0xe6, 0x00, 0x7b, 0xc6,
                0xcb, 0x53, 0xa5, 0x44,
            ],
            Bitcoin::Xor8 => [
                0x4a, 0xb1, 0x4a, 0x81, 0x4a, 0x39, 0x52, 0x8a, 0x80, 0xfd, 0xb4, 0x30, 0x58, 0x9b,
                0xa4, 0x50, 0x10, 0x4b, 0x9c, 0x72, 0x09, 0xaa, 0x2f, 0xe2, 0x85, 0xcd, 0x60, 0xc0,
                0x90, 0x43, 0x11, 0x4a,
            ],
            Bitcoin::XorXor1 => [
                0x18, 0xb9, 0x44, 0x6a, 0x41, 0x66, 0xa3, 0xfe, 0xe2, 0xbc, 0xb2, 0x54, 0x5b, 0xb9,
                0x01, 0x18, 0xdc, 0xf0, 0xe8, 0xf8, 0x86, 0xa1, 0x07, 0x6d, 0x4c, 0x38, 0x60, 0x06,
                0x0c, 0xde, 0x1a, 0x51,
            ],
            Bitcoin::XorXor16 => [
                0x94, 0x6c, 0xde, 0x87, 0x2e, 0x30, 0xe6, 0x50, 0x9d, 0xaf, 0xf4, 0x05, 0xf0, 0xe0,
                0xfe, 0xfe, 0x27, 0x55, 0x47, 0xb4, 0x0e, 0xb2, 0x03, 0x84, 0xaf, 0xe9, 0xa8, 0x63,
                0x60, 0xfc, 0x80, 0xef,
            ],
            Bitcoin::XorXor32 => [
                0x65, 0x27, 0xdf, 0x67, 0xa5, 0x0d, 0x14, 0x8d, 0xb4, 0xfc, 0x8f, 0xee, 0xc7, 0x84,
                0x55, 0x64, 0x99, 0xa8, 0xc7, 0xf0, 0xfa, 0x7d, 0x28, 0xe6, 0x27, 0x8e, 0x99, 0x7f,
                0x49, 0x59, 0xbe, 0x39,
            ],
            Bitcoin::XorXor64 => [
                0xf1, 0x62, 0xf9, 0xe6, 0x56, 0x63, 0xa6, 0x9a, 0xc5, 0xf9, 0x2a, 0x5e, 0xb5, 0x2c,
                0x03, 0x32, 0x39, 0x2e, 0xdd, 0x1e, 0xd1, 0xba, 0x35, 0x5e, 0x6f, 0x19, 0x40, 0x6e,
                0xab, 0xe3, 0xf6, 0xed,
            ],
            Bitcoin::XorXor8 => [
                0xe0, 0x6d, 0x69, 0x4c, 0x5b, 0x40, 0x7d, 0xda, 0xd7, 0xaa, 0x1f, 0x88, 0x07, 0x16,
                0xbc, 0xb7, 0x0a, 0xcd, 0xba, 0x75, 0x85, 0xca, 0x40, 0x09, 0x9a, 0x0a, 0x0a, 0x61,
                0xf3, 0xad, 0x2d, 0xb5,
            ],
        };

        Cmr::from_byte_array(bytes)
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
            Bitcoin::And1 => b"*22",
            Bitcoin::And16 => b"i",
            Bitcoin::And32 => b"l",
            Bitcoin::And64 => b"*ll",
            Bitcoin::And8 => b"****22*22**22*22***22*22**22*22",
            Bitcoin::AnnexHash => b"***+1h*+1*ll*+1l*+1i*+1****22*22**22*22***22*22**22*22+1***22*22**22*22*lh+1h",
            Bitcoin::Bip0340Verify => b"**hh*hh",
            Bitcoin::BuildTapbranch => b"*hh",
            Bitcoin::BuildTapleafSimplicity => b"h",
            Bitcoin::BuildTaptweak => b"*hh",
            Bitcoin::Ch1 => b"*2*22",
            Bitcoin::Ch16 => b"*****22*22**22*22***22*22**22*22i",
            Bitcoin::Ch32 => b"*il",
            Bitcoin::Ch64 => b"*l*ll",
            Bitcoin::Ch8 => b"****22*22**22*22****22*22**22*22***22*22**22*22",
            Bitcoin::CheckLockDistance => b"****22*22**22*22***22*22**22*22",
            Bitcoin::CheckLockDuration => b"****22*22**22*22***22*22**22*22",
            Bitcoin::CheckLockHeight => b"i",
            Bitcoin::CheckLockTime => b"i",
            Bitcoin::CheckSigVerify => b"**h*hh*hh",
            Bitcoin::Complement1 => b"2",
            Bitcoin::Complement16 => b"****22*22**22*22***22*22**22*22",
            Bitcoin::Complement32 => b"i",
            Bitcoin::Complement64 => b"l",
            Bitcoin::Complement8 => b"***22*22**22*22",
            Bitcoin::CurrentAnnexHash => b"1",
            Bitcoin::CurrentIndex => b"1",
            Bitcoin::CurrentPrevOutpoint => b"1",
            Bitcoin::CurrentScriptHash => b"1",
            Bitcoin::CurrentScriptSigHash => b"1",
            Bitcoin::CurrentSequence => b"1",
            Bitcoin::CurrentValue => b"1",
            Bitcoin::Decompress => b"*2h",
            Bitcoin::Decrement16 => b"****22*22**22*22***22*22**22*22",
            Bitcoin::Decrement32 => b"i",
            Bitcoin::Decrement64 => b"l",
            Bitcoin::Decrement8 => b"***22*22**22*22",
            Bitcoin::DivMod128_64 => b"**lll",
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
            Bitcoin::Eq1 => b"*22",
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
            Bitcoin::Fee => b"1",
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
            Bitcoin::FullLeftShift16_1 => b"*****22*22**22*22***22*22**22*222",
            Bitcoin::FullLeftShift16_2 => b"*****22*22**22*22***22*22**22*22*22",
            Bitcoin::FullLeftShift16_4 => b"*****22*22**22*22***22*22**22*22**22*22",
            Bitcoin::FullLeftShift16_8 => b"*****22*22**22*22***22*22**22*22***22*22**22*22",
            Bitcoin::FullLeftShift32_1 => b"*i2",
            Bitcoin::FullLeftShift32_16 => b"*i****22*22**22*22***22*22**22*22",
            Bitcoin::FullLeftShift32_2 => b"*i*22",
            Bitcoin::FullLeftShift32_4 => b"*i**22*22",
            Bitcoin::FullLeftShift32_8 => b"*i***22*22**22*22",
            Bitcoin::FullLeftShift64_1 => b"*l2",
            Bitcoin::FullLeftShift64_16 => b"*l****22*22**22*22***22*22**22*22",
            Bitcoin::FullLeftShift64_2 => b"*l*22",
            Bitcoin::FullLeftShift64_32 => b"*li",
            Bitcoin::FullLeftShift64_4 => b"*l**22*22",
            Bitcoin::FullLeftShift64_8 => b"*l***22*22**22*22",
            Bitcoin::FullLeftShift8_1 => b"****22*22**22*222",
            Bitcoin::FullLeftShift8_2 => b"****22*22**22*22*22",
            Bitcoin::FullLeftShift8_4 => b"****22*22**22*22**22*22",
            Bitcoin::FullMultiply16 => b"l",
            Bitcoin::FullMultiply32 => b"*ll",
            Bitcoin::FullMultiply64 => b"h",
            Bitcoin::FullMultiply8 => b"i",
            Bitcoin::FullRightShift16_1 => b"*2****22*22**22*22***22*22**22*22",
            Bitcoin::FullRightShift16_2 => b"**22****22*22**22*22***22*22**22*22",
            Bitcoin::FullRightShift16_4 => b"***22*22****22*22**22*22***22*22**22*22",
            Bitcoin::FullRightShift16_8 => b"****22*22**22*22****22*22**22*22***22*22**22*22",
            Bitcoin::FullRightShift32_1 => b"*2i",
            Bitcoin::FullRightShift32_16 => b"*****22*22**22*22***22*22**22*22i",
            Bitcoin::FullRightShift32_2 => b"**22i",
            Bitcoin::FullRightShift32_4 => b"***22*22i",
            Bitcoin::FullRightShift32_8 => b"****22*22**22*22i",
            Bitcoin::FullRightShift64_1 => b"*2l",
            Bitcoin::FullRightShift64_16 => b"*****22*22**22*22***22*22**22*22l",
            Bitcoin::FullRightShift64_2 => b"**22l",
            Bitcoin::FullRightShift64_32 => b"*il",
            Bitcoin::FullRightShift64_4 => b"***22*22l",
            Bitcoin::FullRightShift64_8 => b"****22*22**22*22l",
            Bitcoin::FullRightShift8_1 => b"*2***22*22**22*22",
            Bitcoin::FullRightShift8_2 => b"**22***22*22**22*22",
            Bitcoin::FullRightShift8_4 => b"***22*22***22*22**22*22",
            Bitcoin::FullSubtract16 => b"*2i",
            Bitcoin::FullSubtract32 => b"*2l",
            Bitcoin::FullSubtract64 => b"*2*ll",
            Bitcoin::FullSubtract8 => b"*2****22*22**22*22***22*22**22*22",
            Bitcoin::GeIsOnCurve => b"*hh",
            Bitcoin::GeNegate => b"*hh",
            Bitcoin::GejAdd => b"***hhh**hhh",
            Bitcoin::GejDouble => b"**hhh",
            Bitcoin::GejEquiv => b"***hhh**hhh",
            Bitcoin::GejGeAdd => b"***hhh*hh",
            Bitcoin::GejGeAddEx => b"***hhh*hh",
            Bitcoin::GejGeEquiv => b"***hhh*hh",
            Bitcoin::GejInfinity => b"1",
            Bitcoin::GejIsInfinity => b"**hhh",
            Bitcoin::GejIsOnCurve => b"**hhh",
            Bitcoin::GejNegate => b"**hhh",
            Bitcoin::GejNormalize => b"**hhh",
            Bitcoin::GejRescale => b"***hhhh",
            Bitcoin::GejXEquiv => b"*h**hhh",
            Bitcoin::GejYIsOdd => b"**hhh",
            Bitcoin::Generate => b"h",
            Bitcoin::HashToCurve => b"h",
            Bitcoin::High1 => b"1",
            Bitcoin::High16 => b"1",
            Bitcoin::High32 => b"1",
            Bitcoin::High64 => b"1",
            Bitcoin::High8 => b"1",
            Bitcoin::Increment16 => b"****22*22**22*22***22*22**22*22",
            Bitcoin::Increment32 => b"i",
            Bitcoin::Increment64 => b"l",
            Bitcoin::Increment8 => b"***22*22**22*22",
            Bitcoin::InputAnnexHash => b"i",
            Bitcoin::InputAnnexesHash => b"1",
            Bitcoin::InputHash => b"i",
            Bitcoin::InputOutpointsHash => b"1",
            Bitcoin::InputPrevOutpoint => b"i",
            Bitcoin::InputScriptHash => b"i",
            Bitcoin::InputScriptSigHash => b"i",
            Bitcoin::InputScriptSigsHash => b"1",
            Bitcoin::InputScriptsHash => b"1",
            Bitcoin::InputSequence => b"i",
            Bitcoin::InputSequencesHash => b"1",
            Bitcoin::InputUtxoHash => b"i",
            Bitcoin::InputUtxosHash => b"1",
            Bitcoin::InputValue => b"i",
            Bitcoin::InputValuesHash => b"1",
            Bitcoin::InputsHash => b"1",
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
            Bitcoin::LeftExtend16_32 => b"****22*22**22*22***22*22**22*22",
            Bitcoin::LeftExtend16_64 => b"****22*22**22*22***22*22**22*22",
            Bitcoin::LeftExtend1_16 => b"2",
            Bitcoin::LeftExtend1_32 => b"2",
            Bitcoin::LeftExtend1_64 => b"2",
            Bitcoin::LeftExtend1_8 => b"2",
            Bitcoin::LeftExtend32_64 => b"i",
            Bitcoin::LeftExtend8_16 => b"***22*22**22*22",
            Bitcoin::LeftExtend8_32 => b"***22*22**22*22",
            Bitcoin::LeftExtend8_64 => b"***22*22**22*22",
            Bitcoin::LeftPadHigh16_32 => b"****22*22**22*22***22*22**22*22",
            Bitcoin::LeftPadHigh16_64 => b"****22*22**22*22***22*22**22*22",
            Bitcoin::LeftPadHigh1_16 => b"2",
            Bitcoin::LeftPadHigh1_32 => b"2",
            Bitcoin::LeftPadHigh1_64 => b"2",
            Bitcoin::LeftPadHigh1_8 => b"2",
            Bitcoin::LeftPadHigh32_64 => b"i",
            Bitcoin::LeftPadHigh8_16 => b"***22*22**22*22",
            Bitcoin::LeftPadHigh8_32 => b"***22*22**22*22",
            Bitcoin::LeftPadHigh8_64 => b"***22*22**22*22",
            Bitcoin::LeftPadLow16_32 => b"****22*22**22*22***22*22**22*22",
            Bitcoin::LeftPadLow16_64 => b"****22*22**22*22***22*22**22*22",
            Bitcoin::LeftPadLow1_16 => b"2",
            Bitcoin::LeftPadLow1_32 => b"2",
            Bitcoin::LeftPadLow1_64 => b"2",
            Bitcoin::LeftPadLow1_8 => b"2",
            Bitcoin::LeftPadLow32_64 => b"i",
            Bitcoin::LeftPadLow8_16 => b"***22*22**22*22",
            Bitcoin::LeftPadLow8_32 => b"***22*22**22*22",
            Bitcoin::LeftPadLow8_64 => b"***22*22**22*22",
            Bitcoin::LeftRotate16 => b"***22*22****22*22**22*22***22*22**22*22",
            Bitcoin::LeftRotate32 => b"****22*22**22*22i",
            Bitcoin::LeftRotate64 => b"****22*22**22*22l",
            Bitcoin::LeftRotate8 => b"***22*22***22*22**22*22",
            Bitcoin::LeftShift16 => b"***22*22****22*22**22*22***22*22**22*22",
            Bitcoin::LeftShift32 => b"****22*22**22*22i",
            Bitcoin::LeftShift64 => b"****22*22**22*22l",
            Bitcoin::LeftShift8 => b"***22*22***22*22**22*22",
            Bitcoin::LeftShiftWith16 => b"*2***22*22****22*22**22*22***22*22**22*22",
            Bitcoin::LeftShiftWith32 => b"*2****22*22**22*22i",
            Bitcoin::LeftShiftWith64 => b"*2****22*22**22*22l",
            Bitcoin::LeftShiftWith8 => b"*2***22*22***22*22**22*22",
            Bitcoin::Leftmost16_1 => b"****22*22**22*22***22*22**22*22",
            Bitcoin::Leftmost16_2 => b"****22*22**22*22***22*22**22*22",
            Bitcoin::Leftmost16_4 => b"****22*22**22*22***22*22**22*22",
            Bitcoin::Leftmost16_8 => b"****22*22**22*22***22*22**22*22",
            Bitcoin::Leftmost32_1 => b"i",
            Bitcoin::Leftmost32_16 => b"i",
            Bitcoin::Leftmost32_2 => b"i",
            Bitcoin::Leftmost32_4 => b"i",
            Bitcoin::Leftmost32_8 => b"i",
            Bitcoin::Leftmost64_1 => b"l",
            Bitcoin::Leftmost64_16 => b"l",
            Bitcoin::Leftmost64_2 => b"l",
            Bitcoin::Leftmost64_32 => b"l",
            Bitcoin::Leftmost64_4 => b"l",
            Bitcoin::Leftmost64_8 => b"l",
            Bitcoin::Leftmost8_1 => b"***22*22**22*22",
            Bitcoin::Leftmost8_2 => b"***22*22**22*22",
            Bitcoin::Leftmost8_4 => b"***22*22**22*22",
            Bitcoin::LinearCombination1 => b"**h**hhhh",
            Bitcoin::LinearVerify1 => b"***h*hhh*hh",
            Bitcoin::LockTime => b"1",
            Bitcoin::Low1 => b"1",
            Bitcoin::Low16 => b"1",
            Bitcoin::Low32 => b"1",
            Bitcoin::Low64 => b"1",
            Bitcoin::Low8 => b"1",
            Bitcoin::Lt16 => b"i",
            Bitcoin::Lt32 => b"l",
            Bitcoin::Lt64 => b"*ll",
            Bitcoin::Lt8 => b"****22*22**22*22***22*22**22*22",
            Bitcoin::Maj1 => b"*2*22",
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
            Bitcoin::Or1 => b"*22",
            Bitcoin::Or16 => b"i",
            Bitcoin::Or32 => b"l",
            Bitcoin::Or64 => b"*ll",
            Bitcoin::Or8 => b"****22*22**22*22***22*22**22*22",
            Bitcoin::OutpointHash => b"***+1h*+1*ll*+1l*+1i*+1****22*22**22*22***22*22**22*22+1***22*22**22*22*lh*hi",
            Bitcoin::OutputHash => b"i",
            Bitcoin::OutputScriptHash => b"i",
            Bitcoin::OutputScriptsHash => b"1",
            Bitcoin::OutputValue => b"i",
            Bitcoin::OutputValuesHash => b"1",
            Bitcoin::OutputsHash => b"1",
            Bitcoin::ParseLock => b"i",
            Bitcoin::ParseSequence => b"i",
            Bitcoin::PointVerify1 => b"***h*2hh*2h",
            Bitcoin::RightExtend16_32 => b"****22*22**22*22***22*22**22*22",
            Bitcoin::RightExtend16_64 => b"****22*22**22*22***22*22**22*22",
            Bitcoin::RightExtend32_64 => b"i",
            Bitcoin::RightExtend8_16 => b"***22*22**22*22",
            Bitcoin::RightExtend8_32 => b"***22*22**22*22",
            Bitcoin::RightExtend8_64 => b"***22*22**22*22",
            Bitcoin::RightPadHigh16_32 => b"****22*22**22*22***22*22**22*22",
            Bitcoin::RightPadHigh16_64 => b"****22*22**22*22***22*22**22*22",
            Bitcoin::RightPadHigh1_16 => b"2",
            Bitcoin::RightPadHigh1_32 => b"2",
            Bitcoin::RightPadHigh1_64 => b"2",
            Bitcoin::RightPadHigh1_8 => b"2",
            Bitcoin::RightPadHigh32_64 => b"i",
            Bitcoin::RightPadHigh8_16 => b"***22*22**22*22",
            Bitcoin::RightPadHigh8_32 => b"***22*22**22*22",
            Bitcoin::RightPadHigh8_64 => b"***22*22**22*22",
            Bitcoin::RightPadLow16_32 => b"****22*22**22*22***22*22**22*22",
            Bitcoin::RightPadLow16_64 => b"****22*22**22*22***22*22**22*22",
            Bitcoin::RightPadLow1_16 => b"2",
            Bitcoin::RightPadLow1_32 => b"2",
            Bitcoin::RightPadLow1_64 => b"2",
            Bitcoin::RightPadLow1_8 => b"2",
            Bitcoin::RightPadLow32_64 => b"i",
            Bitcoin::RightPadLow8_16 => b"***22*22**22*22",
            Bitcoin::RightPadLow8_32 => b"***22*22**22*22",
            Bitcoin::RightPadLow8_64 => b"***22*22**22*22",
            Bitcoin::RightRotate16 => b"***22*22****22*22**22*22***22*22**22*22",
            Bitcoin::RightRotate32 => b"****22*22**22*22i",
            Bitcoin::RightRotate64 => b"****22*22**22*22l",
            Bitcoin::RightRotate8 => b"***22*22***22*22**22*22",
            Bitcoin::RightShift16 => b"***22*22****22*22**22*22***22*22**22*22",
            Bitcoin::RightShift32 => b"****22*22**22*22i",
            Bitcoin::RightShift64 => b"****22*22**22*22l",
            Bitcoin::RightShift8 => b"***22*22***22*22**22*22",
            Bitcoin::RightShiftWith16 => b"*2***22*22****22*22**22*22***22*22**22*22",
            Bitcoin::RightShiftWith32 => b"*2****22*22**22*22i",
            Bitcoin::RightShiftWith64 => b"*2****22*22**22*22l",
            Bitcoin::RightShiftWith8 => b"*2***22*22***22*22**22*22",
            Bitcoin::Rightmost16_1 => b"****22*22**22*22***22*22**22*22",
            Bitcoin::Rightmost16_2 => b"****22*22**22*22***22*22**22*22",
            Bitcoin::Rightmost16_4 => b"****22*22**22*22***22*22**22*22",
            Bitcoin::Rightmost16_8 => b"****22*22**22*22***22*22**22*22",
            Bitcoin::Rightmost32_1 => b"i",
            Bitcoin::Rightmost32_16 => b"i",
            Bitcoin::Rightmost32_2 => b"i",
            Bitcoin::Rightmost32_4 => b"i",
            Bitcoin::Rightmost32_8 => b"i",
            Bitcoin::Rightmost64_1 => b"l",
            Bitcoin::Rightmost64_16 => b"l",
            Bitcoin::Rightmost64_2 => b"l",
            Bitcoin::Rightmost64_32 => b"l",
            Bitcoin::Rightmost64_4 => b"l",
            Bitcoin::Rightmost64_8 => b"l",
            Bitcoin::Rightmost8_1 => b"***22*22**22*22",
            Bitcoin::Rightmost8_2 => b"***22*22**22*22",
            Bitcoin::Rightmost8_4 => b"***22*22**22*22",
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
            Bitcoin::SigAllHash => b"1",
            Bitcoin::Some1 => b"2",
            Bitcoin::Some16 => b"****22*22**22*22***22*22**22*22",
            Bitcoin::Some32 => b"i",
            Bitcoin::Some64 => b"l",
            Bitcoin::Some8 => b"***22*22**22*22",
            Bitcoin::Subtract16 => b"i",
            Bitcoin::Subtract32 => b"l",
            Bitcoin::Subtract64 => b"*ll",
            Bitcoin::Subtract8 => b"****22*22**22*22***22*22**22*22",
            Bitcoin::Swu => b"h",
            Bitcoin::TapEnvHash => b"1",
            Bitcoin::TapdataInit => b"1",
            Bitcoin::TapleafHash => b"1",
            Bitcoin::TapleafVersion => b"1",
            Bitcoin::Tappath => b"***22*22**22*22",
            Bitcoin::TappathHash => b"1",
            Bitcoin::TotalInputValue => b"1",
            Bitcoin::TotalOutputValue => b"1",
            Bitcoin::TransactionId => b"1",
            Bitcoin::TxHash => b"1",
            Bitcoin::TxIsFinal => b"1",
            Bitcoin::TxLockDistance => b"1",
            Bitcoin::TxLockDuration => b"1",
            Bitcoin::TxLockHeight => b"1",
            Bitcoin::TxLockTime => b"1",
            Bitcoin::Verify => b"2",
            Bitcoin::Version => b"1",
            Bitcoin::Xor1 => b"*22",
            Bitcoin::Xor16 => b"i",
            Bitcoin::Xor32 => b"l",
            Bitcoin::Xor64 => b"*ll",
            Bitcoin::Xor8 => b"****22*22**22*22***22*22**22*22",
            Bitcoin::XorXor1 => b"*2*22",
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
            Bitcoin::And1 => b"2",
            Bitcoin::And16 => b"****22*22**22*22***22*22**22*22",
            Bitcoin::And32 => b"i",
            Bitcoin::And64 => b"l",
            Bitcoin::And8 => b"***22*22**22*22",
            Bitcoin::AnnexHash => b"**+1h*+1*ll*+1l*+1i*+1****22*22**22*22***22*22**22*22+1***22*22**22*22*lh",
            Bitcoin::Bip0340Verify => b"1",
            Bitcoin::BuildTapbranch => b"h",
            Bitcoin::BuildTapleafSimplicity => b"h",
            Bitcoin::BuildTaptweak => b"h",
            Bitcoin::Ch1 => b"2",
            Bitcoin::Ch16 => b"****22*22**22*22***22*22**22*22",
            Bitcoin::Ch32 => b"i",
            Bitcoin::Ch64 => b"l",
            Bitcoin::Ch8 => b"***22*22**22*22",
            Bitcoin::CheckLockDistance => b"1",
            Bitcoin::CheckLockDuration => b"1",
            Bitcoin::CheckLockHeight => b"1",
            Bitcoin::CheckLockTime => b"1",
            Bitcoin::CheckSigVerify => b"1",
            Bitcoin::Complement1 => b"2",
            Bitcoin::Complement16 => b"****22*22**22*22***22*22**22*22",
            Bitcoin::Complement32 => b"i",
            Bitcoin::Complement64 => b"l",
            Bitcoin::Complement8 => b"***22*22**22*22",
            Bitcoin::CurrentAnnexHash => b"+1h",
            Bitcoin::CurrentIndex => b"i",
            Bitcoin::CurrentPrevOutpoint => b"*hi",
            Bitcoin::CurrentScriptHash => b"h",
            Bitcoin::CurrentScriptSigHash => b"h",
            Bitcoin::CurrentSequence => b"i",
            Bitcoin::CurrentValue => b"l",
            Bitcoin::Decompress => b"+1*hh",
            Bitcoin::Decrement16 => b"*2****22*22**22*22***22*22**22*22",
            Bitcoin::Decrement32 => b"*2i",
            Bitcoin::Decrement64 => b"*2l",
            Bitcoin::Decrement8 => b"*2***22*22**22*22",
            Bitcoin::DivMod128_64 => b"*ll",
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
            Bitcoin::Eq1 => b"2",
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
            Bitcoin::Fee => b"l",
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
            Bitcoin::FullLeftShift16_1 => b"*2****22*22**22*22***22*22**22*22",
            Bitcoin::FullLeftShift16_2 => b"**22****22*22**22*22***22*22**22*22",
            Bitcoin::FullLeftShift16_4 => b"***22*22****22*22**22*22***22*22**22*22",
            Bitcoin::FullLeftShift16_8 => b"****22*22**22*22****22*22**22*22***22*22**22*22",
            Bitcoin::FullLeftShift32_1 => b"*2i",
            Bitcoin::FullLeftShift32_16 => b"*****22*22**22*22***22*22**22*22i",
            Bitcoin::FullLeftShift32_2 => b"**22i",
            Bitcoin::FullLeftShift32_4 => b"***22*22i",
            Bitcoin::FullLeftShift32_8 => b"****22*22**22*22i",
            Bitcoin::FullLeftShift64_1 => b"*2l",
            Bitcoin::FullLeftShift64_16 => b"*****22*22**22*22***22*22**22*22l",
            Bitcoin::FullLeftShift64_2 => b"**22l",
            Bitcoin::FullLeftShift64_32 => b"*il",
            Bitcoin::FullLeftShift64_4 => b"***22*22l",
            Bitcoin::FullLeftShift64_8 => b"****22*22**22*22l",
            Bitcoin::FullLeftShift8_1 => b"*2***22*22**22*22",
            Bitcoin::FullLeftShift8_2 => b"**22***22*22**22*22",
            Bitcoin::FullLeftShift8_4 => b"***22*22***22*22**22*22",
            Bitcoin::FullMultiply16 => b"i",
            Bitcoin::FullMultiply32 => b"l",
            Bitcoin::FullMultiply64 => b"*ll",
            Bitcoin::FullMultiply8 => b"****22*22**22*22***22*22**22*22",
            Bitcoin::FullRightShift16_1 => b"*****22*22**22*22***22*22**22*222",
            Bitcoin::FullRightShift16_2 => b"*****22*22**22*22***22*22**22*22*22",
            Bitcoin::FullRightShift16_4 => b"*****22*22**22*22***22*22**22*22**22*22",
            Bitcoin::FullRightShift16_8 => b"*****22*22**22*22***22*22**22*22***22*22**22*22",
            Bitcoin::FullRightShift32_1 => b"*i2",
            Bitcoin::FullRightShift32_16 => b"*i****22*22**22*22***22*22**22*22",
            Bitcoin::FullRightShift32_2 => b"*i*22",
            Bitcoin::FullRightShift32_4 => b"*i**22*22",
            Bitcoin::FullRightShift32_8 => b"*i***22*22**22*22",
            Bitcoin::FullRightShift64_1 => b"*l2",
            Bitcoin::FullRightShift64_16 => b"*l****22*22**22*22***22*22**22*22",
            Bitcoin::FullRightShift64_2 => b"*l*22",
            Bitcoin::FullRightShift64_32 => b"*li",
            Bitcoin::FullRightShift64_4 => b"*l**22*22",
            Bitcoin::FullRightShift64_8 => b"*l***22*22**22*22",
            Bitcoin::FullRightShift8_1 => b"****22*22**22*222",
            Bitcoin::FullRightShift8_2 => b"****22*22**22*22*22",
            Bitcoin::FullRightShift8_4 => b"****22*22**22*22**22*22",
            Bitcoin::FullSubtract16 => b"*2****22*22**22*22***22*22**22*22",
            Bitcoin::FullSubtract32 => b"*2i",
            Bitcoin::FullSubtract64 => b"*2l",
            Bitcoin::FullSubtract8 => b"*2***22*22**22*22",
            Bitcoin::GeIsOnCurve => b"2",
            Bitcoin::GeNegate => b"*hh",
            Bitcoin::GejAdd => b"**hhh",
            Bitcoin::GejDouble => b"**hhh",
            Bitcoin::GejEquiv => b"2",
            Bitcoin::GejGeAdd => b"**hhh",
            Bitcoin::GejGeAddEx => b"*h**hhh",
            Bitcoin::GejGeEquiv => b"2",
            Bitcoin::GejInfinity => b"**hhh",
            Bitcoin::GejIsInfinity => b"2",
            Bitcoin::GejIsOnCurve => b"2",
            Bitcoin::GejNegate => b"**hhh",
            Bitcoin::GejNormalize => b"+1*hh",
            Bitcoin::GejRescale => b"**hhh",
            Bitcoin::GejXEquiv => b"2",
            Bitcoin::GejYIsOdd => b"2",
            Bitcoin::Generate => b"**hhh",
            Bitcoin::HashToCurve => b"*hh",
            Bitcoin::High1 => b"2",
            Bitcoin::High16 => b"****22*22**22*22***22*22**22*22",
            Bitcoin::High32 => b"i",
            Bitcoin::High64 => b"l",
            Bitcoin::High8 => b"***22*22**22*22",
            Bitcoin::Increment16 => b"*2****22*22**22*22***22*22**22*22",
            Bitcoin::Increment32 => b"*2i",
            Bitcoin::Increment64 => b"*2l",
            Bitcoin::Increment8 => b"*2***22*22**22*22",
            Bitcoin::InputAnnexHash => b"+1+1h",
            Bitcoin::InputAnnexesHash => b"h",
            Bitcoin::InputHash => b"+1h",
            Bitcoin::InputOutpointsHash => b"h",
            Bitcoin::InputPrevOutpoint => b"+1*hi",
            Bitcoin::InputScriptHash => b"+1h",
            Bitcoin::InputScriptSigHash => b"+1h",
            Bitcoin::InputScriptSigsHash => b"h",
            Bitcoin::InputScriptsHash => b"h",
            Bitcoin::InputSequence => b"+1i",
            Bitcoin::InputSequencesHash => b"h",
            Bitcoin::InputUtxoHash => b"+1h",
            Bitcoin::InputUtxosHash => b"h",
            Bitcoin::InputValue => b"+1l",
            Bitcoin::InputValuesHash => b"h",
            Bitcoin::InputsHash => b"h",
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
            Bitcoin::LeftExtend16_32 => b"i",
            Bitcoin::LeftExtend16_64 => b"l",
            Bitcoin::LeftExtend1_16 => b"****22*22**22*22***22*22**22*22",
            Bitcoin::LeftExtend1_32 => b"i",
            Bitcoin::LeftExtend1_64 => b"l",
            Bitcoin::LeftExtend1_8 => b"***22*22**22*22",
            Bitcoin::LeftExtend32_64 => b"l",
            Bitcoin::LeftExtend8_16 => b"****22*22**22*22***22*22**22*22",
            Bitcoin::LeftExtend8_32 => b"i",
            Bitcoin::LeftExtend8_64 => b"l",
            Bitcoin::LeftPadHigh16_32 => b"i",
            Bitcoin::LeftPadHigh16_64 => b"l",
            Bitcoin::LeftPadHigh1_16 => b"****22*22**22*22***22*22**22*22",
            Bitcoin::LeftPadHigh1_32 => b"i",
            Bitcoin::LeftPadHigh1_64 => b"l",
            Bitcoin::LeftPadHigh1_8 => b"***22*22**22*22",
            Bitcoin::LeftPadHigh32_64 => b"l",
            Bitcoin::LeftPadHigh8_16 => b"****22*22**22*22***22*22**22*22",
            Bitcoin::LeftPadHigh8_32 => b"i",
            Bitcoin::LeftPadHigh8_64 => b"l",
            Bitcoin::LeftPadLow16_32 => b"i",
            Bitcoin::LeftPadLow16_64 => b"l",
            Bitcoin::LeftPadLow1_16 => b"****22*22**22*22***22*22**22*22",
            Bitcoin::LeftPadLow1_32 => b"i",
            Bitcoin::LeftPadLow1_64 => b"l",
            Bitcoin::LeftPadLow1_8 => b"***22*22**22*22",
            Bitcoin::LeftPadLow32_64 => b"l",
            Bitcoin::LeftPadLow8_16 => b"****22*22**22*22***22*22**22*22",
            Bitcoin::LeftPadLow8_32 => b"i",
            Bitcoin::LeftPadLow8_64 => b"l",
            Bitcoin::LeftRotate16 => b"****22*22**22*22***22*22**22*22",
            Bitcoin::LeftRotate32 => b"i",
            Bitcoin::LeftRotate64 => b"l",
            Bitcoin::LeftRotate8 => b"***22*22**22*22",
            Bitcoin::LeftShift16 => b"****22*22**22*22***22*22**22*22",
            Bitcoin::LeftShift32 => b"i",
            Bitcoin::LeftShift64 => b"l",
            Bitcoin::LeftShift8 => b"***22*22**22*22",
            Bitcoin::LeftShiftWith16 => b"****22*22**22*22***22*22**22*22",
            Bitcoin::LeftShiftWith32 => b"i",
            Bitcoin::LeftShiftWith64 => b"l",
            Bitcoin::LeftShiftWith8 => b"***22*22**22*22",
            Bitcoin::Leftmost16_1 => b"2",
            Bitcoin::Leftmost16_2 => b"*22",
            Bitcoin::Leftmost16_4 => b"**22*22",
            Bitcoin::Leftmost16_8 => b"***22*22**22*22",
            Bitcoin::Leftmost32_1 => b"2",
            Bitcoin::Leftmost32_16 => b"****22*22**22*22***22*22**22*22",
            Bitcoin::Leftmost32_2 => b"*22",
            Bitcoin::Leftmost32_4 => b"**22*22",
            Bitcoin::Leftmost32_8 => b"***22*22**22*22",
            Bitcoin::Leftmost64_1 => b"2",
            Bitcoin::Leftmost64_16 => b"****22*22**22*22***22*22**22*22",
            Bitcoin::Leftmost64_2 => b"*22",
            Bitcoin::Leftmost64_32 => b"i",
            Bitcoin::Leftmost64_4 => b"**22*22",
            Bitcoin::Leftmost64_8 => b"***22*22**22*22",
            Bitcoin::Leftmost8_1 => b"2",
            Bitcoin::Leftmost8_2 => b"*22",
            Bitcoin::Leftmost8_4 => b"**22*22",
            Bitcoin::LinearCombination1 => b"**hhh",
            Bitcoin::LinearVerify1 => b"1",
            Bitcoin::LockTime => b"i",
            Bitcoin::Low1 => b"2",
            Bitcoin::Low16 => b"****22*22**22*22***22*22**22*22",
            Bitcoin::Low32 => b"i",
            Bitcoin::Low64 => b"l",
            Bitcoin::Low8 => b"***22*22**22*22",
            Bitcoin::Lt16 => b"2",
            Bitcoin::Lt32 => b"2",
            Bitcoin::Lt64 => b"2",
            Bitcoin::Lt8 => b"2",
            Bitcoin::Maj1 => b"2",
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
            Bitcoin::Or1 => b"2",
            Bitcoin::Or16 => b"****22*22**22*22***22*22**22*22",
            Bitcoin::Or32 => b"i",
            Bitcoin::Or64 => b"l",
            Bitcoin::Or8 => b"***22*22**22*22",
            Bitcoin::OutpointHash => b"**+1h*+1*ll*+1l*+1i*+1****22*22**22*22***22*22**22*22+1***22*22**22*22*lh",
            Bitcoin::OutputHash => b"+1h",
            Bitcoin::OutputScriptHash => b"+1h",
            Bitcoin::OutputScriptsHash => b"h",
            Bitcoin::OutputValue => b"+1l",
            Bitcoin::OutputValuesHash => b"h",
            Bitcoin::OutputsHash => b"h",
            Bitcoin::ParseLock => b"+ii",
            Bitcoin::ParseSequence => b"+1+****22*22**22*22***22*22**22*22****22*22**22*22***22*22**22*22",
            Bitcoin::PointVerify1 => b"1",
            Bitcoin::RightExtend16_32 => b"i",
            Bitcoin::RightExtend16_64 => b"l",
            Bitcoin::RightExtend32_64 => b"l",
            Bitcoin::RightExtend8_16 => b"****22*22**22*22***22*22**22*22",
            Bitcoin::RightExtend8_32 => b"i",
            Bitcoin::RightExtend8_64 => b"l",
            Bitcoin::RightPadHigh16_32 => b"i",
            Bitcoin::RightPadHigh16_64 => b"l",
            Bitcoin::RightPadHigh1_16 => b"****22*22**22*22***22*22**22*22",
            Bitcoin::RightPadHigh1_32 => b"i",
            Bitcoin::RightPadHigh1_64 => b"l",
            Bitcoin::RightPadHigh1_8 => b"***22*22**22*22",
            Bitcoin::RightPadHigh32_64 => b"l",
            Bitcoin::RightPadHigh8_16 => b"****22*22**22*22***22*22**22*22",
            Bitcoin::RightPadHigh8_32 => b"i",
            Bitcoin::RightPadHigh8_64 => b"l",
            Bitcoin::RightPadLow16_32 => b"i",
            Bitcoin::RightPadLow16_64 => b"l",
            Bitcoin::RightPadLow1_16 => b"****22*22**22*22***22*22**22*22",
            Bitcoin::RightPadLow1_32 => b"i",
            Bitcoin::RightPadLow1_64 => b"l",
            Bitcoin::RightPadLow1_8 => b"***22*22**22*22",
            Bitcoin::RightPadLow32_64 => b"l",
            Bitcoin::RightPadLow8_16 => b"****22*22**22*22***22*22**22*22",
            Bitcoin::RightPadLow8_32 => b"i",
            Bitcoin::RightPadLow8_64 => b"l",
            Bitcoin::RightRotate16 => b"****22*22**22*22***22*22**22*22",
            Bitcoin::RightRotate32 => b"i",
            Bitcoin::RightRotate64 => b"l",
            Bitcoin::RightRotate8 => b"***22*22**22*22",
            Bitcoin::RightShift16 => b"****22*22**22*22***22*22**22*22",
            Bitcoin::RightShift32 => b"i",
            Bitcoin::RightShift64 => b"l",
            Bitcoin::RightShift8 => b"***22*22**22*22",
            Bitcoin::RightShiftWith16 => b"****22*22**22*22***22*22**22*22",
            Bitcoin::RightShiftWith32 => b"i",
            Bitcoin::RightShiftWith64 => b"l",
            Bitcoin::RightShiftWith8 => b"***22*22**22*22",
            Bitcoin::Rightmost16_1 => b"2",
            Bitcoin::Rightmost16_2 => b"*22",
            Bitcoin::Rightmost16_4 => b"**22*22",
            Bitcoin::Rightmost16_8 => b"***22*22**22*22",
            Bitcoin::Rightmost32_1 => b"2",
            Bitcoin::Rightmost32_16 => b"****22*22**22*22***22*22**22*22",
            Bitcoin::Rightmost32_2 => b"*22",
            Bitcoin::Rightmost32_4 => b"**22*22",
            Bitcoin::Rightmost32_8 => b"***22*22**22*22",
            Bitcoin::Rightmost64_1 => b"2",
            Bitcoin::Rightmost64_16 => b"****22*22**22*22***22*22**22*22",
            Bitcoin::Rightmost64_2 => b"*22",
            Bitcoin::Rightmost64_32 => b"i",
            Bitcoin::Rightmost64_4 => b"**22*22",
            Bitcoin::Rightmost64_8 => b"***22*22**22*22",
            Bitcoin::Rightmost8_1 => b"2",
            Bitcoin::Rightmost8_2 => b"*22",
            Bitcoin::Rightmost8_4 => b"**22*22",
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
            Bitcoin::SigAllHash => b"h",
            Bitcoin::Some1 => b"2",
            Bitcoin::Some16 => b"2",
            Bitcoin::Some32 => b"2",
            Bitcoin::Some64 => b"2",
            Bitcoin::Some8 => b"2",
            Bitcoin::Subtract16 => b"*2****22*22**22*22***22*22**22*22",
            Bitcoin::Subtract32 => b"*2i",
            Bitcoin::Subtract64 => b"*2l",
            Bitcoin::Subtract8 => b"*2***22*22**22*22",
            Bitcoin::Swu => b"*hh",
            Bitcoin::TapEnvHash => b"h",
            Bitcoin::TapdataInit => b"**+1h*+1*ll*+1l*+1i*+1****22*22**22*22***22*22**22*22+1***22*22**22*22*lh",
            Bitcoin::TapleafHash => b"h",
            Bitcoin::TapleafVersion => b"***22*22**22*22",
            Bitcoin::Tappath => b"+1h",
            Bitcoin::TappathHash => b"h",
            Bitcoin::TotalInputValue => b"l",
            Bitcoin::TotalOutputValue => b"l",
            Bitcoin::TransactionId => b"h",
            Bitcoin::TxHash => b"h",
            Bitcoin::TxIsFinal => b"2",
            Bitcoin::TxLockDistance => b"****22*22**22*22***22*22**22*22",
            Bitcoin::TxLockDuration => b"****22*22**22*22***22*22**22*22",
            Bitcoin::TxLockHeight => b"i",
            Bitcoin::TxLockTime => b"i",
            Bitcoin::Verify => b"1",
            Bitcoin::Version => b"i",
            Bitcoin::Xor1 => b"2",
            Bitcoin::Xor16 => b"****22*22**22*22***22*22**22*22",
            Bitcoin::Xor32 => b"i",
            Bitcoin::Xor64 => b"l",
            Bitcoin::Xor8 => b"***22*22**22*22",
            Bitcoin::XorXor1 => b"2",
            Bitcoin::XorXor16 => b"****22*22**22*22***22*22**22*22",
            Bitcoin::XorXor32 => b"i",
            Bitcoin::XorXor64 => b"l",
            Bitcoin::XorXor8 => b"***22*22**22*22",
        };

        TypeName(name)
    }

    fn encode(&self, w: &mut BitWriter<&mut dyn Write>) -> std::io::Result<usize> {
        let (n, len) = match self {
            Bitcoin::Verify => (0, 3),
            Bitcoin::Low1 => (8, 6),
            Bitcoin::Low8 => (37, 8),
            Bitcoin::Low16 => (304, 11),
            Bitcoin::Low32 => (305, 11),
            Bitcoin::Low64 => (306, 11),
            Bitcoin::High1 => (10, 6),
            Bitcoin::High8 => (45, 8),
            Bitcoin::High16 => (368, 11),
            Bitcoin::High32 => (369, 11),
            Bitcoin::High64 => (370, 11),
            Bitcoin::Complement1 => (96, 9),
            Bitcoin::Complement8 => (389, 11),
            Bitcoin::Complement16 => (3120, 14),
            Bitcoin::Complement32 => (3121, 14),
            Bitcoin::Complement64 => (3122, 14),
            Bitcoin::And1 => (98, 9),
            Bitcoin::And8 => (397, 11),
            Bitcoin::And16 => (3184, 14),
            Bitcoin::And32 => (3185, 14),
            Bitcoin::And64 => (3186, 14),
            Bitcoin::Or1 => (100, 9),
            Bitcoin::Or8 => (405, 11),
            Bitcoin::Or16 => (3248, 14),
            Bitcoin::Or32 => (3249, 14),
            Bitcoin::Or64 => (3250, 14),
            Bitcoin::Xor1 => (102, 9),
            Bitcoin::Xor8 => (413, 11),
            Bitcoin::Xor16 => (3312, 14),
            Bitcoin::Xor32 => (3313, 14),
            Bitcoin::Xor64 => (3314, 14),
            Bitcoin::Maj1 => (208, 10),
            Bitcoin::Maj8 => (837, 12),
            Bitcoin::Maj16 => (6704, 15),
            Bitcoin::Maj32 => (6705, 15),
            Bitcoin::Maj64 => (6706, 15),
            Bitcoin::XorXor1 => (210, 10),
            Bitcoin::XorXor8 => (845, 12),
            Bitcoin::XorXor16 => (6768, 15),
            Bitcoin::XorXor32 => (6769, 15),
            Bitcoin::XorXor64 => (6770, 15),
            Bitcoin::Ch1 => (212, 10),
            Bitcoin::Ch8 => (853, 12),
            Bitcoin::Ch16 => (6832, 15),
            Bitcoin::Ch32 => (6833, 15),
            Bitcoin::Ch64 => (6834, 15),
            Bitcoin::Some1 => (214, 10),
            Bitcoin::Some8 => (861, 12),
            Bitcoin::Some16 => (6896, 15),
            Bitcoin::Some32 => (6897, 15),
            Bitcoin::Some64 => (6898, 15),
            Bitcoin::All8 => (869, 12),
            Bitcoin::All16 => (6960, 15),
            Bitcoin::All32 => (6961, 15),
            Bitcoin::All64 => (6962, 15),
            Bitcoin::Eq1 => (218, 10),
            Bitcoin::Eq8 => (877, 12),
            Bitcoin::Eq16 => (7024, 15),
            Bitcoin::Eq32 => (7025, 15),
            Bitcoin::Eq64 => (7026, 15),
            Bitcoin::Eq256 => (14056, 16),
            Bitcoin::FullLeftShift8_1 => (1765, 13),
            Bitcoin::FullLeftShift16_1 => (14128, 16),
            Bitcoin::FullLeftShift32_1 => (14129, 16),
            Bitcoin::FullLeftShift64_1 => (14130, 16),
            Bitcoin::FullLeftShift8_2 => (7076, 15),
            Bitcoin::FullLeftShift16_2 => (7077, 15),
            Bitcoin::FullLeftShift32_2 => (56624, 18),
            Bitcoin::FullLeftShift64_2 => (56625, 18),
            Bitcoin::FullLeftShift8_4 => (1770, 13),
            Bitcoin::FullLeftShift16_4 => (7084, 15),
            Bitcoin::FullLeftShift32_4 => (7085, 15),
            Bitcoin::FullLeftShift64_4 => (56688, 18),
            Bitcoin::FullLeftShift16_8 => (14176, 16),
            Bitcoin::FullLeftShift32_8 => (56708, 18),
            Bitcoin::FullLeftShift64_8 => (56709, 18),
            Bitcoin::FullLeftShift32_16 => (14178, 16),
            Bitcoin::FullLeftShift64_16 => (56716, 18),
            Bitcoin::FullLeftShift64_32 => (14180, 16),
            Bitcoin::FullRightShift8_1 => (1781, 13),
            Bitcoin::FullRightShift16_1 => (14256, 16),
            Bitcoin::FullRightShift32_1 => (14257, 16),
            Bitcoin::FullRightShift64_1 => (14258, 16),
            Bitcoin::FullRightShift8_2 => (7140, 15),
            Bitcoin::FullRightShift16_2 => (7141, 15),
            Bitcoin::FullRightShift32_2 => (57136, 18),
            Bitcoin::FullRightShift64_2 => (57137, 18),
            Bitcoin::FullRightShift8_4 => (1786, 13),
            Bitcoin::FullRightShift16_4 => (7148, 15),
            Bitcoin::FullRightShift32_4 => (7149, 15),
            Bitcoin::FullRightShift64_4 => (57200, 18),
            Bitcoin::FullRightShift16_8 => (14304, 16),
            Bitcoin::FullRightShift32_8 => (57220, 18),
            Bitcoin::FullRightShift64_8 => (57221, 18),
            Bitcoin::FullRightShift32_16 => (14306, 16),
            Bitcoin::FullRightShift64_16 => (57228, 18),
            Bitcoin::FullRightShift64_32 => (14308, 16),
            Bitcoin::Leftmost8_1 => (28677, 17),
            Bitcoin::Leftmost16_1 => (229424, 20),
            Bitcoin::Leftmost32_1 => (229425, 20),
            Bitcoin::Leftmost64_1 => (229426, 20),
            Bitcoin::Leftmost8_2 => (114724, 19),
            Bitcoin::Leftmost16_2 => (114725, 19),
            Bitcoin::Leftmost32_2 => (917808, 22),
            Bitcoin::Leftmost64_2 => (917809, 22),
            Bitcoin::Leftmost8_4 => (28682, 17),
            Bitcoin::Leftmost16_4 => (114732, 19),
            Bitcoin::Leftmost32_4 => (114733, 19),
            Bitcoin::Leftmost64_4 => (917872, 22),
            Bitcoin::Leftmost16_8 => (229472, 20),
            Bitcoin::Leftmost32_8 => (917892, 22),
            Bitcoin::Leftmost64_8 => (917893, 22),
            Bitcoin::Leftmost32_16 => (229474, 20),
            Bitcoin::Leftmost64_16 => (917900, 22),
            Bitcoin::Leftmost64_32 => (229476, 20),
            Bitcoin::Rightmost8_1 => (28693, 17),
            Bitcoin::Rightmost16_1 => (229552, 20),
            Bitcoin::Rightmost32_1 => (229553, 20),
            Bitcoin::Rightmost64_1 => (229554, 20),
            Bitcoin::Rightmost8_2 => (114788, 19),
            Bitcoin::Rightmost16_2 => (114789, 19),
            Bitcoin::Rightmost32_2 => (918320, 22),
            Bitcoin::Rightmost64_2 => (918321, 22),
            Bitcoin::Rightmost8_4 => (28698, 17),
            Bitcoin::Rightmost16_4 => (114796, 19),
            Bitcoin::Rightmost32_4 => (114797, 19),
            Bitcoin::Rightmost64_4 => (918384, 22),
            Bitcoin::Rightmost16_8 => (229600, 20),
            Bitcoin::Rightmost32_8 => (918404, 22),
            Bitcoin::Rightmost64_8 => (918405, 22),
            Bitcoin::Rightmost32_16 => (229602, 20),
            Bitcoin::Rightmost64_16 => (918412, 22),
            Bitcoin::Rightmost64_32 => (229604, 20),
            Bitcoin::LeftPadLow1_8 => (28709, 17),
            Bitcoin::LeftPadLow1_16 => (229680, 20),
            Bitcoin::LeftPadLow1_32 => (229681, 20),
            Bitcoin::LeftPadLow1_64 => (229682, 20),
            Bitcoin::LeftPadLow8_16 => (229728, 20),
            Bitcoin::LeftPadLow8_32 => (918916, 22),
            Bitcoin::LeftPadLow8_64 => (918917, 22),
            Bitcoin::LeftPadLow16_32 => (229730, 20),
            Bitcoin::LeftPadLow16_64 => (918924, 22),
            Bitcoin::LeftPadLow32_64 => (229732, 20),
            Bitcoin::LeftPadHigh1_8 => (28725, 17),
            Bitcoin::LeftPadHigh1_16 => (229808, 20),
            Bitcoin::LeftPadHigh1_32 => (229809, 20),
            Bitcoin::LeftPadHigh1_64 => (229810, 20),
            Bitcoin::LeftPadHigh8_16 => (229856, 20),
            Bitcoin::LeftPadHigh8_32 => (919428, 22),
            Bitcoin::LeftPadHigh8_64 => (919429, 22),
            Bitcoin::LeftPadHigh16_32 => (229858, 20),
            Bitcoin::LeftPadHigh16_64 => (919436, 22),
            Bitcoin::LeftPadHigh32_64 => (229860, 20),
            Bitcoin::LeftExtend1_8 => (28741, 17),
            Bitcoin::LeftExtend1_16 => (229936, 20),
            Bitcoin::LeftExtend1_32 => (229937, 20),
            Bitcoin::LeftExtend1_64 => (229938, 20),
            Bitcoin::LeftExtend8_16 => (229984, 20),
            Bitcoin::LeftExtend8_32 => (919940, 22),
            Bitcoin::LeftExtend8_64 => (919941, 22),
            Bitcoin::LeftExtend16_32 => (229986, 20),
            Bitcoin::LeftExtend16_64 => (919948, 22),
            Bitcoin::LeftExtend32_64 => (229988, 20),
            Bitcoin::RightPadLow1_8 => (28757, 17),
            Bitcoin::RightPadLow1_16 => (230064, 20),
            Bitcoin::RightPadLow1_32 => (230065, 20),
            Bitcoin::RightPadLow1_64 => (230066, 20),
            Bitcoin::RightPadLow8_16 => (230112, 20),
            Bitcoin::RightPadLow8_32 => (920452, 22),
            Bitcoin::RightPadLow8_64 => (920453, 22),
            Bitcoin::RightPadLow16_32 => (230114, 20),
            Bitcoin::RightPadLow16_64 => (920460, 22),
            Bitcoin::RightPadLow32_64 => (230116, 20),
            Bitcoin::RightPadHigh1_8 => (28773, 17),
            Bitcoin::RightPadHigh1_16 => (230192, 20),
            Bitcoin::RightPadHigh1_32 => (230193, 20),
            Bitcoin::RightPadHigh1_64 => (230194, 20),
            Bitcoin::RightPadHigh8_16 => (230240, 20),
            Bitcoin::RightPadHigh8_32 => (920964, 22),
            Bitcoin::RightPadHigh8_64 => (920965, 22),
            Bitcoin::RightPadHigh16_32 => (230242, 20),
            Bitcoin::RightPadHigh16_64 => (920972, 22),
            Bitcoin::RightPadHigh32_64 => (230244, 20),
            Bitcoin::RightExtend8_16 => (230368, 20),
            Bitcoin::RightExtend8_32 => (921476, 22),
            Bitcoin::RightExtend8_64 => (921477, 22),
            Bitcoin::RightExtend16_32 => (230370, 20),
            Bitcoin::RightExtend16_64 => (921484, 22),
            Bitcoin::RightExtend32_64 => (230372, 20),
            Bitcoin::LeftShiftWith8 => (14405, 16),
            Bitcoin::LeftShiftWith16 => (115248, 19),
            Bitcoin::LeftShiftWith32 => (115249, 19),
            Bitcoin::LeftShiftWith64 => (115250, 19),
            Bitcoin::RightShiftWith8 => (14413, 16),
            Bitcoin::RightShiftWith16 => (115312, 19),
            Bitcoin::RightShiftWith32 => (115313, 19),
            Bitcoin::RightShiftWith64 => (115314, 19),
            Bitcoin::LeftShift8 => (14421, 16),
            Bitcoin::LeftShift16 => (115376, 19),
            Bitcoin::LeftShift32 => (115377, 19),
            Bitcoin::LeftShift64 => (115378, 19),
            Bitcoin::RightShift8 => (14429, 16),
            Bitcoin::RightShift16 => (115440, 19),
            Bitcoin::RightShift32 => (115441, 19),
            Bitcoin::RightShift64 => (115442, 19),
            Bitcoin::LeftRotate8 => (14437, 16),
            Bitcoin::LeftRotate16 => (115504, 19),
            Bitcoin::LeftRotate32 => (115505, 19),
            Bitcoin::LeftRotate64 => (115506, 19),
            Bitcoin::RightRotate8 => (14445, 16),
            Bitcoin::RightRotate16 => (115568, 19),
            Bitcoin::RightRotate32 => (115569, 19),
            Bitcoin::RightRotate64 => (115570, 19),
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
            Bitcoin::DivMod128_64 => (639346, 21),
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
            Bitcoin::GejEquiv => (100097, 18),
            Bitcoin::GejGeEquiv => (100098, 18),
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
            Bitcoin::HashToCurve => (200238, 19),
            Bitcoin::Swu => (200239, 19),
            Bitcoin::CheckSigVerify => (98, 8),
            Bitcoin::Bip0340Verify => (396, 10),
            Bitcoin::ParseLock => (102, 8),
            Bitcoin::ParseSequence => (412, 10),
            Bitcoin::TapdataInit => (413, 10),
            Bitcoin::SigAllHash => (4, 3),
            Bitcoin::TxHash => (20, 5),
            Bitcoin::TapEnvHash => (21, 5),
            Bitcoin::OutputsHash => (176, 8),
            Bitcoin::InputsHash => (177, 8),
            Bitcoin::InputUtxosHash => (178, 8),
            Bitcoin::OutputHash => (179, 8),
            Bitcoin::OutputValuesHash => (360, 9),
            Bitcoin::OutputScriptsHash => (361, 9),
            Bitcoin::InputHash => (362, 9),
            Bitcoin::InputOutpointsHash => (363, 9),
            Bitcoin::InputSequencesHash => (364, 9),
            Bitcoin::InputAnnexesHash => (365, 9),
            Bitcoin::InputScriptSigsHash => (366, 9),
            Bitcoin::InputUtxoHash => (367, 9),
            Bitcoin::InputValuesHash => (5888, 13),
            Bitcoin::InputScriptsHash => (5889, 13),
            Bitcoin::TapleafHash => (5890, 13),
            Bitcoin::TappathHash => (5891, 13),
            Bitcoin::OutpointHash => (5892, 13),
            Bitcoin::AnnexHash => (5893, 13),
            Bitcoin::BuildTapleafSimplicity => (5894, 13),
            Bitcoin::BuildTapbranch => (5895, 13),
            Bitcoin::BuildTaptweak => (5896, 13),
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
            Bitcoin::Fee => (883, 10),
            Bitcoin::OutputValue => (1768, 11),
            Bitcoin::OutputScriptHash => (1769, 11),
            Bitcoin::TotalOutputValue => (1770, 11),
            Bitcoin::CurrentPrevOutpoint => (1771, 11),
            Bitcoin::CurrentValue => (1772, 11),
            Bitcoin::CurrentScriptHash => (1773, 11),
            Bitcoin::CurrentSequence => (1774, 11),
            Bitcoin::CurrentAnnexHash => (1775, 11),
            Bitcoin::CurrentScriptSigHash => (28416, 15),
            Bitcoin::InputPrevOutpoint => (28417, 15),
            Bitcoin::InputValue => (28418, 15),
            Bitcoin::InputScriptHash => (28419, 15),
            Bitcoin::InputSequence => (28420, 15),
            Bitcoin::InputAnnexHash => (28421, 15),
            Bitcoin::InputScriptSigHash => (28422, 15),
            Bitcoin::TotalInputValue => (28423, 15),
            Bitcoin::TapleafVersion => (28424, 15),
            Bitcoin::Tappath => (28425, 15),
            Bitcoin::Version => (28426, 15),
            Bitcoin::TransactionId => (28427, 15),
        };

        w.write_bits_be(n, len)
    }

    fn decode<I: Iterator<Item = u8>>(bits: &mut BitIter<I>) -> Result<Self, decode::Error> where Self: Sized {
        decode_bits!(bits, {
            0 => {
                0 => {
                    0 => {Bitcoin::Verify},
                    1 => {
                        0 => {
                            0 => {
                                0 => {Bitcoin::Low1},
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
                                0 => {Bitcoin::High1},
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
                                            0 => {Bitcoin::Complement1},
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
                                            0 => {Bitcoin::And1},
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
                                            0 => {Bitcoin::Or1},
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
                                            0 => {Bitcoin::Xor1},
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
                                                0 => {Bitcoin::Maj1},
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
                                                0 => {Bitcoin::XorXor1},
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
                                                0 => {Bitcoin::Ch1},
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
                                                0 => {Bitcoin::Some1},
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
                                                0 => {Bitcoin::Eq1},
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
                                        1 => {
                                            0 => {
                                                0 => {
                                                    0 => {},
                                                    1 => {
                                                        0 => {
                                                            0 => {},
                                                            1 => {Bitcoin::FullLeftShift8_1}
                                                        },
                                                        1 => {
                                                            0 => {
                                                                0 => {
                                                                    0 => {
                                                                        0 => {Bitcoin::FullLeftShift16_1},
                                                                        1 => {Bitcoin::FullLeftShift32_1}
                                                                    },
                                                                    1 => {
                                                                        0 => {Bitcoin::FullLeftShift64_1},
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
                                                                    0 => {Bitcoin::FullLeftShift8_2},
                                                                    1 => {Bitcoin::FullLeftShift16_2}
                                                                },
                                                                1 => {
                                                                    0 => {
                                                                        0 => {
                                                                            0 => {
                                                                                0 => {Bitcoin::FullLeftShift32_2},
                                                                                1 => {Bitcoin::FullLeftShift64_2}
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
                                                            0 => {Bitcoin::FullLeftShift8_4},
                                                            1 => {
                                                                0 => {
                                                                    0 => {Bitcoin::FullLeftShift16_4},
                                                                    1 => {Bitcoin::FullLeftShift32_4}
                                                                },
                                                                1 => {
                                                                    0 => {
                                                                        0 => {
                                                                            0 => {
                                                                                0 => {Bitcoin::FullLeftShift64_4},
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
                                                                        0 => {Bitcoin::FullLeftShift16_8},
                                                                        1 => {
                                                                            0 => {
                                                                                0 => {Bitcoin::FullLeftShift32_8},
                                                                                1 => {Bitcoin::FullLeftShift64_8}
                                                                            },
                                                                            1 => {}
                                                                        }
                                                                    },
                                                                    1 => {
                                                                        0 => {Bitcoin::FullLeftShift32_16},
                                                                        1 => {
                                                                            0 => {
                                                                                0 => {Bitcoin::FullLeftShift64_16},
                                                                                1 => {}
                                                                            },
                                                                            1 => {}
                                                                        }
                                                                    }
                                                                },
                                                                1 => {
                                                                    0 => {
                                                                        0 => {Bitcoin::FullLeftShift64_32},
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
                                                            1 => {Bitcoin::FullRightShift8_1}
                                                        },
                                                        1 => {
                                                            0 => {
                                                                0 => {
                                                                    0 => {
                                                                        0 => {Bitcoin::FullRightShift16_1},
                                                                        1 => {Bitcoin::FullRightShift32_1}
                                                                    },
                                                                    1 => {
                                                                        0 => {Bitcoin::FullRightShift64_1},
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
                                                                    0 => {Bitcoin::FullRightShift8_2},
                                                                    1 => {Bitcoin::FullRightShift16_2}
                                                                },
                                                                1 => {
                                                                    0 => {
                                                                        0 => {
                                                                            0 => {
                                                                                0 => {Bitcoin::FullRightShift32_2},
                                                                                1 => {Bitcoin::FullRightShift64_2}
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
                                                            0 => {Bitcoin::FullRightShift8_4},
                                                            1 => {
                                                                0 => {
                                                                    0 => {Bitcoin::FullRightShift16_4},
                                                                    1 => {Bitcoin::FullRightShift32_4}
                                                                },
                                                                1 => {
                                                                    0 => {
                                                                        0 => {
                                                                            0 => {
                                                                                0 => {Bitcoin::FullRightShift64_4},
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
                                                                        0 => {Bitcoin::FullRightShift16_8},
                                                                        1 => {
                                                                            0 => {
                                                                                0 => {Bitcoin::FullRightShift32_8},
                                                                                1 => {Bitcoin::FullRightShift64_8}
                                                                            },
                                                                            1 => {}
                                                                        }
                                                                    },
                                                                    1 => {
                                                                        0 => {Bitcoin::FullRightShift32_16},
                                                                        1 => {
                                                                            0 => {
                                                                                0 => {Bitcoin::FullRightShift64_16},
                                                                                1 => {}
                                                                            },
                                                                            1 => {}
                                                                        }
                                                                    }
                                                                },
                                                                1 => {
                                                                    0 => {
                                                                        0 => {Bitcoin::FullRightShift64_32},
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
                                                                            1 => {Bitcoin::Leftmost8_1}
                                                                        },
                                                                        1 => {
                                                                            0 => {
                                                                                0 => {
                                                                                    0 => {
                                                                                        0 => {Bitcoin::Leftmost16_1},
                                                                                        1 => {Bitcoin::Leftmost32_1}
                                                                                    },
                                                                                    1 => {
                                                                                        0 => {Bitcoin::Leftmost64_1},
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
                                                                                    0 => {Bitcoin::Leftmost8_2},
                                                                                    1 => {Bitcoin::Leftmost16_2}
                                                                                },
                                                                                1 => {
                                                                                    0 => {
                                                                                        0 => {
                                                                                            0 => {
                                                                                                0 => {Bitcoin::Leftmost32_2},
                                                                                                1 => {Bitcoin::Leftmost64_2}
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
                                                                            0 => {Bitcoin::Leftmost8_4},
                                                                            1 => {
                                                                                0 => {
                                                                                    0 => {Bitcoin::Leftmost16_4},
                                                                                    1 => {Bitcoin::Leftmost32_4}
                                                                                },
                                                                                1 => {
                                                                                    0 => {
                                                                                        0 => {
                                                                                            0 => {
                                                                                                0 => {Bitcoin::Leftmost64_4},
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
                                                                                        0 => {Bitcoin::Leftmost16_8},
                                                                                        1 => {
                                                                                            0 => {
                                                                                                0 => {Bitcoin::Leftmost32_8},
                                                                                                1 => {Bitcoin::Leftmost64_8}
                                                                                            },
                                                                                            1 => {}
                                                                                        }
                                                                                    },
                                                                                    1 => {
                                                                                        0 => {Bitcoin::Leftmost32_16},
                                                                                        1 => {
                                                                                            0 => {
                                                                                                0 => {Bitcoin::Leftmost64_16},
                                                                                                1 => {}
                                                                                            },
                                                                                            1 => {}
                                                                                        }
                                                                                    }
                                                                                },
                                                                                1 => {
                                                                                    0 => {
                                                                                        0 => {Bitcoin::Leftmost64_32},
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
                                                                            1 => {Bitcoin::Rightmost8_1}
                                                                        },
                                                                        1 => {
                                                                            0 => {
                                                                                0 => {
                                                                                    0 => {
                                                                                        0 => {Bitcoin::Rightmost16_1},
                                                                                        1 => {Bitcoin::Rightmost32_1}
                                                                                    },
                                                                                    1 => {
                                                                                        0 => {Bitcoin::Rightmost64_1},
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
                                                                                    0 => {Bitcoin::Rightmost8_2},
                                                                                    1 => {Bitcoin::Rightmost16_2}
                                                                                },
                                                                                1 => {
                                                                                    0 => {
                                                                                        0 => {
                                                                                            0 => {
                                                                                                0 => {Bitcoin::Rightmost32_2},
                                                                                                1 => {Bitcoin::Rightmost64_2}
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
                                                                            0 => {Bitcoin::Rightmost8_4},
                                                                            1 => {
                                                                                0 => {
                                                                                    0 => {Bitcoin::Rightmost16_4},
                                                                                    1 => {Bitcoin::Rightmost32_4}
                                                                                },
                                                                                1 => {
                                                                                    0 => {
                                                                                        0 => {
                                                                                            0 => {
                                                                                                0 => {Bitcoin::Rightmost64_4},
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
                                                                                        0 => {Bitcoin::Rightmost16_8},
                                                                                        1 => {
                                                                                            0 => {
                                                                                                0 => {Bitcoin::Rightmost32_8},
                                                                                                1 => {Bitcoin::Rightmost64_8}
                                                                                            },
                                                                                            1 => {}
                                                                                        }
                                                                                    },
                                                                                    1 => {
                                                                                        0 => {Bitcoin::Rightmost32_16},
                                                                                        1 => {
                                                                                            0 => {
                                                                                                0 => {Bitcoin::Rightmost64_16},
                                                                                                1 => {}
                                                                                            },
                                                                                            1 => {}
                                                                                        }
                                                                                    }
                                                                                },
                                                                                1 => {
                                                                                    0 => {
                                                                                        0 => {Bitcoin::Rightmost64_32},
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
                                                                            1 => {Bitcoin::LeftPadLow1_8}
                                                                        },
                                                                        1 => {
                                                                            0 => {
                                                                                0 => {
                                                                                    0 => {
                                                                                        0 => {Bitcoin::LeftPadLow1_16},
                                                                                        1 => {Bitcoin::LeftPadLow1_32}
                                                                                    },
                                                                                    1 => {
                                                                                        0 => {Bitcoin::LeftPadLow1_64},
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
                                                                                        0 => {Bitcoin::LeftPadLow8_16},
                                                                                        1 => {
                                                                                            0 => {
                                                                                                0 => {Bitcoin::LeftPadLow8_32},
                                                                                                1 => {Bitcoin::LeftPadLow8_64}
                                                                                            },
                                                                                            1 => {}
                                                                                        }
                                                                                    },
                                                                                    1 => {
                                                                                        0 => {Bitcoin::LeftPadLow16_32},
                                                                                        1 => {
                                                                                            0 => {
                                                                                                0 => {Bitcoin::LeftPadLow16_64},
                                                                                                1 => {}
                                                                                            },
                                                                                            1 => {}
                                                                                        }
                                                                                    }
                                                                                },
                                                                                1 => {
                                                                                    0 => {
                                                                                        0 => {Bitcoin::LeftPadLow32_64},
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
                                                                            1 => {Bitcoin::LeftPadHigh1_8}
                                                                        },
                                                                        1 => {
                                                                            0 => {
                                                                                0 => {
                                                                                    0 => {
                                                                                        0 => {Bitcoin::LeftPadHigh1_16},
                                                                                        1 => {Bitcoin::LeftPadHigh1_32}
                                                                                    },
                                                                                    1 => {
                                                                                        0 => {Bitcoin::LeftPadHigh1_64},
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
                                                                                        0 => {Bitcoin::LeftPadHigh8_16},
                                                                                        1 => {
                                                                                            0 => {
                                                                                                0 => {Bitcoin::LeftPadHigh8_32},
                                                                                                1 => {Bitcoin::LeftPadHigh8_64}
                                                                                            },
                                                                                            1 => {}
                                                                                        }
                                                                                    },
                                                                                    1 => {
                                                                                        0 => {Bitcoin::LeftPadHigh16_32},
                                                                                        1 => {
                                                                                            0 => {
                                                                                                0 => {Bitcoin::LeftPadHigh16_64},
                                                                                                1 => {}
                                                                                            },
                                                                                            1 => {}
                                                                                        }
                                                                                    }
                                                                                },
                                                                                1 => {
                                                                                    0 => {
                                                                                        0 => {Bitcoin::LeftPadHigh32_64},
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
                                                                            1 => {Bitcoin::LeftExtend1_8}
                                                                        },
                                                                        1 => {
                                                                            0 => {
                                                                                0 => {
                                                                                    0 => {
                                                                                        0 => {Bitcoin::LeftExtend1_16},
                                                                                        1 => {Bitcoin::LeftExtend1_32}
                                                                                    },
                                                                                    1 => {
                                                                                        0 => {Bitcoin::LeftExtend1_64},
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
                                                                                        0 => {Bitcoin::LeftExtend8_16},
                                                                                        1 => {
                                                                                            0 => {
                                                                                                0 => {Bitcoin::LeftExtend8_32},
                                                                                                1 => {Bitcoin::LeftExtend8_64}
                                                                                            },
                                                                                            1 => {}
                                                                                        }
                                                                                    },
                                                                                    1 => {
                                                                                        0 => {Bitcoin::LeftExtend16_32},
                                                                                        1 => {
                                                                                            0 => {
                                                                                                0 => {Bitcoin::LeftExtend16_64},
                                                                                                1 => {}
                                                                                            },
                                                                                            1 => {}
                                                                                        }
                                                                                    }
                                                                                },
                                                                                1 => {
                                                                                    0 => {
                                                                                        0 => {Bitcoin::LeftExtend32_64},
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
                                                                            1 => {Bitcoin::RightPadLow1_8}
                                                                        },
                                                                        1 => {
                                                                            0 => {
                                                                                0 => {
                                                                                    0 => {
                                                                                        0 => {Bitcoin::RightPadLow1_16},
                                                                                        1 => {Bitcoin::RightPadLow1_32}
                                                                                    },
                                                                                    1 => {
                                                                                        0 => {Bitcoin::RightPadLow1_64},
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
                                                                                        0 => {Bitcoin::RightPadLow8_16},
                                                                                        1 => {
                                                                                            0 => {
                                                                                                0 => {Bitcoin::RightPadLow8_32},
                                                                                                1 => {Bitcoin::RightPadLow8_64}
                                                                                            },
                                                                                            1 => {}
                                                                                        }
                                                                                    },
                                                                                    1 => {
                                                                                        0 => {Bitcoin::RightPadLow16_32},
                                                                                        1 => {
                                                                                            0 => {
                                                                                                0 => {Bitcoin::RightPadLow16_64},
                                                                                                1 => {}
                                                                                            },
                                                                                            1 => {}
                                                                                        }
                                                                                    }
                                                                                },
                                                                                1 => {
                                                                                    0 => {
                                                                                        0 => {Bitcoin::RightPadLow32_64},
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
                                                                            1 => {Bitcoin::RightPadHigh1_8}
                                                                        },
                                                                        1 => {
                                                                            0 => {
                                                                                0 => {
                                                                                    0 => {
                                                                                        0 => {Bitcoin::RightPadHigh1_16},
                                                                                        1 => {Bitcoin::RightPadHigh1_32}
                                                                                    },
                                                                                    1 => {
                                                                                        0 => {Bitcoin::RightPadHigh1_64},
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
                                                                                        0 => {Bitcoin::RightPadHigh8_16},
                                                                                        1 => {
                                                                                            0 => {
                                                                                                0 => {Bitcoin::RightPadHigh8_32},
                                                                                                1 => {Bitcoin::RightPadHigh8_64}
                                                                                            },
                                                                                            1 => {}
                                                                                        }
                                                                                    },
                                                                                    1 => {
                                                                                        0 => {Bitcoin::RightPadHigh16_32},
                                                                                        1 => {
                                                                                            0 => {
                                                                                                0 => {Bitcoin::RightPadHigh16_64},
                                                                                                1 => {}
                                                                                            },
                                                                                            1 => {}
                                                                                        }
                                                                                    }
                                                                                },
                                                                                1 => {
                                                                                    0 => {
                                                                                        0 => {Bitcoin::RightPadHigh32_64},
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
                                                                                        0 => {Bitcoin::RightExtend8_16},
                                                                                        1 => {
                                                                                            0 => {
                                                                                                0 => {Bitcoin::RightExtend8_32},
                                                                                                1 => {Bitcoin::RightExtend8_64}
                                                                                            },
                                                                                            1 => {}
                                                                                        }
                                                                                    },
                                                                                    1 => {
                                                                                        0 => {Bitcoin::RightExtend16_32},
                                                                                        1 => {
                                                                                            0 => {
                                                                                                0 => {Bitcoin::RightExtend16_64},
                                                                                                1 => {}
                                                                                            },
                                                                                            1 => {}
                                                                                        }
                                                                                    }
                                                                                },
                                                                                1 => {
                                                                                    0 => {
                                                                                        0 => {Bitcoin::RightExtend32_64},
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
                                                                        1 => {Bitcoin::LeftShiftWith8}
                                                                    },
                                                                    1 => {
                                                                        0 => {
                                                                            0 => {
                                                                                0 => {
                                                                                    0 => {Bitcoin::LeftShiftWith16},
                                                                                    1 => {Bitcoin::LeftShiftWith32}
                                                                                },
                                                                                1 => {
                                                                                    0 => {Bitcoin::LeftShiftWith64},
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
                                                                        1 => {Bitcoin::RightShiftWith8}
                                                                    },
                                                                    1 => {
                                                                        0 => {
                                                                            0 => {
                                                                                0 => {
                                                                                    0 => {Bitcoin::RightShiftWith16},
                                                                                    1 => {Bitcoin::RightShiftWith32}
                                                                                },
                                                                                1 => {
                                                                                    0 => {Bitcoin::RightShiftWith64},
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
                                                                        1 => {Bitcoin::LeftShift8}
                                                                    },
                                                                    1 => {
                                                                        0 => {
                                                                            0 => {
                                                                                0 => {
                                                                                    0 => {Bitcoin::LeftShift16},
                                                                                    1 => {Bitcoin::LeftShift32}
                                                                                },
                                                                                1 => {
                                                                                    0 => {Bitcoin::LeftShift64},
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
                                                                        1 => {Bitcoin::RightShift8}
                                                                    },
                                                                    1 => {
                                                                        0 => {
                                                                            0 => {
                                                                                0 => {
                                                                                    0 => {Bitcoin::RightShift16},
                                                                                    1 => {Bitcoin::RightShift32}
                                                                                },
                                                                                1 => {
                                                                                    0 => {Bitcoin::RightShift64},
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
                                                                        1 => {Bitcoin::LeftRotate8}
                                                                    },
                                                                    1 => {
                                                                        0 => {
                                                                            0 => {
                                                                                0 => {
                                                                                    0 => {Bitcoin::LeftRotate16},
                                                                                    1 => {Bitcoin::LeftRotate32}
                                                                                },
                                                                                1 => {
                                                                                    0 => {Bitcoin::LeftRotate64},
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
                                                                        1 => {Bitcoin::RightRotate8}
                                                                    },
                                                                    1 => {
                                                                        0 => {
                                                                            0 => {
                                                                                0 => {
                                                                                    0 => {Bitcoin::RightRotate16},
                                                                                    1 => {Bitcoin::RightRotate32}
                                                                                },
                                                                                1 => {
                                                                                    0 => {Bitcoin::RightRotate64},
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
                                                                    1 => {
                                                                        0 => {},
                                                                        1 => {
                                                                            0 => {},
                                                                            1 => {
                                                                                0 => {
                                                                                    0 => {
                                                                                        0 => {},
                                                                                        1 => {
                                                                                            0 => {Bitcoin::DivMod128_64},
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
                                                                                1 => {Bitcoin::GejEquiv}
                                                                            },
                                                                            1 => {
                                                                                0 => {Bitcoin::GejGeEquiv},
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
                                                                                1 => {
                                                                                    0 => {Bitcoin::HashToCurve},
                                                                                    1 => {Bitcoin::Swu}
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
                                                1 => {Bitcoin::TapdataInit}
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
                0 => {
                    0 => {Bitcoin::SigAllHash},
                    1 => {
                        0 => {
                            0 => {Bitcoin::TxHash},
                            1 => {Bitcoin::TapEnvHash}
                        },
                        1 => {
                            0 => {
                                0 => {
                                    0 => {
                                        0 => {Bitcoin::OutputsHash},
                                        1 => {Bitcoin::InputsHash}
                                    },
                                    1 => {
                                        0 => {Bitcoin::InputUtxosHash},
                                        1 => {Bitcoin::OutputHash}
                                    }
                                },
                                1 => {
                                    0 => {
                                        0 => {
                                            0 => {Bitcoin::OutputValuesHash},
                                            1 => {Bitcoin::OutputScriptsHash}
                                        },
                                        1 => {
                                            0 => {Bitcoin::InputHash},
                                            1 => {Bitcoin::InputOutpointsHash}
                                        }
                                    },
                                    1 => {
                                        0 => {
                                            0 => {Bitcoin::InputSequencesHash},
                                            1 => {Bitcoin::InputAnnexesHash}
                                        },
                                        1 => {
                                            0 => {Bitcoin::InputScriptSigsHash},
                                            1 => {Bitcoin::InputUtxoHash}
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
                                                            0 => {Bitcoin::InputValuesHash},
                                                            1 => {Bitcoin::InputScriptsHash}
                                                        },
                                                        1 => {
                                                            0 => {Bitcoin::TapleafHash},
                                                            1 => {Bitcoin::TappathHash}
                                                        }
                                                    },
                                                    1 => {
                                                        0 => {
                                                            0 => {Bitcoin::OutpointHash},
                                                            1 => {Bitcoin::AnnexHash}
                                                        },
                                                        1 => {
                                                            0 => {Bitcoin::BuildTapleafSimplicity},
                                                            1 => {Bitcoin::BuildTapbranch}
                                                        }
                                                    }
                                                },
                                                1 => {
                                                    0 => {
                                                        0 => {
                                                            0 => {Bitcoin::BuildTaptweak},
                                                            1 => {}
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
                                                1 => {Bitcoin::Fee}
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
                                                    1 => {Bitcoin::CurrentScriptHash}
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
                                                                    1 => {Bitcoin::InputScriptHash}
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
                                                                    1 => {Bitcoin::Tappath}
                                                                },
                                                                1 => {
                                                                    0 => {Bitcoin::Version},
                                                                    1 => {Bitcoin::TransactionId}
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

    fn cost(&self) -> Cost {
        match self {
            Bitcoin::Add16 => Cost::from_milliweight(80),
            Bitcoin::Add32 => Cost::from_milliweight(92),
            Bitcoin::Add64 => Cost::from_milliweight(105),
            Bitcoin::Add8 => Cost::from_milliweight(97),
            Bitcoin::All16 => Cost::from_milliweight(60),
            Bitcoin::All32 => Cost::from_milliweight(62),
            Bitcoin::All64 => Cost::from_milliweight(63),
            Bitcoin::All8 => Cost::from_milliweight(50),
            Bitcoin::And1 => Cost::from_milliweight(77),
            Bitcoin::And16 => Cost::from_milliweight(83),
            Bitcoin::And32 => Cost::from_milliweight(77),
            Bitcoin::And64 => Cost::from_milliweight(78),
            Bitcoin::And8 => Cost::from_milliweight(98),
            Bitcoin::AnnexHash => Cost::from_milliweight(1491),
            Bitcoin::Bip0340Verify => Cost::from_milliweight(49421),
            Bitcoin::BuildTapbranch => Cost::from_milliweight(2554),
            Bitcoin::BuildTapleafSimplicity => Cost::from_milliweight(1927),
            Bitcoin::BuildTaptweak => Cost::from_milliweight(77780),
            Bitcoin::CheckLockDistance => Cost::from_milliweight(84),
            Bitcoin::CheckLockDuration => Cost::from_milliweight(78),
            Bitcoin::CheckLockHeight => Cost::from_milliweight(108),
            Bitcoin::CheckLockTime => Cost::from_milliweight(123),
            Bitcoin::CheckSigVerify => Cost::from_milliweight(50000),
            Bitcoin::Ch1 => Cost::from_milliweight(50),
            Bitcoin::Ch16 => Cost::from_milliweight(83),
            Bitcoin::Ch32 => Cost::from_milliweight(69),
            Bitcoin::Ch64 => Cost::from_milliweight(78),
            Bitcoin::Ch8 => Cost::from_milliweight(86),
            Bitcoin::Complement1 => Cost::from_milliweight(51),
            Bitcoin::Complement16 => Cost::from_milliweight(86),
            Bitcoin::Complement32 => Cost::from_milliweight(58),
            Bitcoin::Complement64 => Cost::from_milliweight(64),
            Bitcoin::Complement8 => Cost::from_milliweight(62),
            Bitcoin::CurrentAnnexHash => Cost::from_milliweight(74),
            Bitcoin::CurrentIndex => Cost::from_milliweight(66),
            Bitcoin::CurrentPrevOutpoint => Cost::from_milliweight(130),
            Bitcoin::CurrentScriptHash => Cost::from_milliweight(127),
            Bitcoin::CurrentScriptSigHash => Cost::from_milliweight(126),
            Bitcoin::CurrentSequence => Cost::from_milliweight(66),
            Bitcoin::CurrentValue => Cost::from_milliweight(85),
            Bitcoin::Decompress => Cost::from_milliweight(10495),
            Bitcoin::Decrement16 => Cost::from_milliweight(58),
            Bitcoin::Decrement32 => Cost::from_milliweight(57),
            Bitcoin::Decrement64 => Cost::from_milliweight(79),
            Bitcoin::Decrement8 => Cost::from_milliweight(77),
            Bitcoin::Divides16 => Cost::from_milliweight(84),
            Bitcoin::Divides32 => Cost::from_milliweight(80),
            Bitcoin::Divides64 => Cost::from_milliweight(67),
            Bitcoin::Divides8 => Cost::from_milliweight(73),
            Bitcoin::Divide16 => Cost::from_milliweight(85),
            Bitcoin::Divide32 => Cost::from_milliweight(82),
            Bitcoin::Divide64 => Cost::from_milliweight(81),
            Bitcoin::Divide8 => Cost::from_milliweight(85),
            Bitcoin::DivMod128_64 => Cost::from_milliweight(169),
            Bitcoin::DivMod16 => Cost::from_milliweight(92),
            Bitcoin::DivMod32 => Cost::from_milliweight(90),
            Bitcoin::DivMod64 => Cost::from_milliweight(82),
            Bitcoin::DivMod8 => Cost::from_milliweight(91),
            Bitcoin::Eq1 => Cost::from_milliweight(63),
            Bitcoin::Eq16 => Cost::from_milliweight(68),
            Bitcoin::Eq256 => Cost::from_milliweight(188),
            Bitcoin::Eq32 => Cost::from_milliweight(74),
            Bitcoin::Eq64 => Cost::from_milliweight(82),
            Bitcoin::Eq8 => Cost::from_milliweight(76),
            Bitcoin::Fee => Cost::from_milliweight(65),
            Bitcoin::FeAdd => Cost::from_milliweight(777),
            Bitcoin::FeInvert => Cost::from_milliweight(3237),
            Bitcoin::FeIsOdd => Cost::from_milliweight(313),
            Bitcoin::FeIsZero => Cost::from_milliweight(277),
            Bitcoin::FeMultiply => Cost::from_milliweight(813),
            Bitcoin::FeMultiplyBeta => Cost::from_milliweight(607),
            Bitcoin::FeNegate => Cost::from_milliweight(541),
            Bitcoin::FeNormalize => Cost::from_milliweight(656),
            Bitcoin::FeSquare => Cost::from_milliweight(570),
            Bitcoin::FeSquareRoot => Cost::from_milliweight(10162),
            Bitcoin::FullAdd16 => Cost::from_milliweight(106),
            Bitcoin::FullAdd32 => Cost::from_milliweight(96),
            Bitcoin::FullAdd64 => Cost::from_milliweight(93),
            Bitcoin::FullAdd8 => Cost::from_milliweight(131),
            Bitcoin::FullDecrement16 => Cost::from_milliweight(60),
            Bitcoin::FullDecrement32 => Cost::from_milliweight(71),
            Bitcoin::FullDecrement64 => Cost::from_milliweight(71),
            Bitcoin::FullDecrement8 => Cost::from_milliweight(68),
            Bitcoin::FullIncrement16 => Cost::from_milliweight(70),
            Bitcoin::FullIncrement32 => Cost::from_milliweight(57),
            Bitcoin::FullIncrement64 => Cost::from_milliweight(68),
            Bitcoin::FullIncrement8 => Cost::from_milliweight(73),
            Bitcoin::FullLeftShift16_1 => Cost::from_milliweight(76),
            Bitcoin::FullLeftShift16_2 => Cost::from_milliweight(59),
            Bitcoin::FullLeftShift16_4 => Cost::from_milliweight(68),
            Bitcoin::FullLeftShift16_8 => Cost::from_milliweight(68),
            Bitcoin::FullLeftShift32_1 => Cost::from_milliweight(58),
            Bitcoin::FullLeftShift32_16 => Cost::from_milliweight(52),
            Bitcoin::FullLeftShift32_2 => Cost::from_milliweight(73),
            Bitcoin::FullLeftShift32_4 => Cost::from_milliweight(59),
            Bitcoin::FullLeftShift32_8 => Cost::from_milliweight(60),
            Bitcoin::FullLeftShift64_1 => Cost::from_milliweight(74),
            Bitcoin::FullLeftShift64_16 => Cost::from_milliweight(69),
            Bitcoin::FullLeftShift64_2 => Cost::from_milliweight(70),
            Bitcoin::FullLeftShift64_32 => Cost::from_milliweight(73),
            Bitcoin::FullLeftShift64_4 => Cost::from_milliweight(66),
            Bitcoin::FullLeftShift64_8 => Cost::from_milliweight(68),
            Bitcoin::FullLeftShift8_1 => Cost::from_milliweight(60),
            Bitcoin::FullLeftShift8_2 => Cost::from_milliweight(64),
            Bitcoin::FullLeftShift8_4 => Cost::from_milliweight(72),
            Bitcoin::FullMultiply16 => Cost::from_milliweight(99),
            Bitcoin::FullMultiply32 => Cost::from_milliweight(87),
            Bitcoin::FullMultiply64 => Cost::from_milliweight(103),
            Bitcoin::FullMultiply8 => Cost::from_milliweight(95),
            Bitcoin::FullRightShift16_1 => Cost::from_milliweight(55),
            Bitcoin::FullRightShift16_2 => Cost::from_milliweight(60),
            Bitcoin::FullRightShift16_4 => Cost::from_milliweight(64),
            Bitcoin::FullRightShift16_8 => Cost::from_milliweight(55),
            Bitcoin::FullRightShift32_1 => Cost::from_milliweight(49),
            Bitcoin::FullRightShift32_16 => Cost::from_milliweight(48),
            Bitcoin::FullRightShift32_2 => Cost::from_milliweight(66),
            Bitcoin::FullRightShift32_4 => Cost::from_milliweight(49),
            Bitcoin::FullRightShift32_8 => Cost::from_milliweight(66),
            Bitcoin::FullRightShift64_1 => Cost::from_milliweight(60),
            Bitcoin::FullRightShift64_16 => Cost::from_milliweight(73),
            Bitcoin::FullRightShift64_2 => Cost::from_milliweight(76),
            Bitcoin::FullRightShift64_32 => Cost::from_milliweight(73),
            Bitcoin::FullRightShift64_4 => Cost::from_milliweight(56),
            Bitcoin::FullRightShift64_8 => Cost::from_milliweight(68),
            Bitcoin::FullRightShift8_1 => Cost::from_milliweight(59),
            Bitcoin::FullRightShift8_2 => Cost::from_milliweight(49),
            Bitcoin::FullRightShift8_4 => Cost::from_milliweight(51),
            Bitcoin::FullSubtract16 => Cost::from_milliweight(99),
            Bitcoin::FullSubtract32 => Cost::from_milliweight(92),
            Bitcoin::FullSubtract64 => Cost::from_milliweight(109),
            Bitcoin::FullSubtract8 => Cost::from_milliweight(106),
            Bitcoin::GejAdd => Cost::from_milliweight(3000),
            Bitcoin::GejDouble => Cost::from_milliweight(1862),
            Bitcoin::GejEquiv => Cost::from_milliweight(2376),
            Bitcoin::GejGeAdd => Cost::from_milliweight(2609),
            Bitcoin::GejGeAddEx => Cost::from_milliweight(2860),
            Bitcoin::GejGeEquiv => Cost::from_milliweight(1823),
            Bitcoin::GejInfinity => Cost::from_milliweight(765),
            Bitcoin::GejIsInfinity => Cost::from_milliweight(701),
            Bitcoin::GejIsOnCurve => Cost::from_milliweight(1039),
            Bitcoin::GejNegate => Cost::from_milliweight(1549),
            Bitcoin::GejNormalize => Cost::from_milliweight(4184),
            Bitcoin::GejRescale => Cost::from_milliweight(2011),
            Bitcoin::GejXEquiv => Cost::from_milliweight(1103),
            Bitcoin::GejYIsOdd => Cost::from_milliweight(3702),
            Bitcoin::Generate => Cost::from_milliweight(49851),
            Bitcoin::GeIsOnCurve => Cost::from_milliweight(688),
            Bitcoin::GeNegate => Cost::from_milliweight(1071),
            Bitcoin::HashToCurve => Cost::from_milliweight(69844),
            Bitcoin::High1 => Cost::from_milliweight(42),
            Bitcoin::High16 => Cost::from_milliweight(50),
            Bitcoin::High32 => Cost::from_milliweight(64),
            Bitcoin::High64 => Cost::from_milliweight(52),
            Bitcoin::High8 => Cost::from_milliweight(59),
            Bitcoin::Increment16 => Cost::from_milliweight(56),
            Bitcoin::Increment32 => Cost::from_milliweight(73),
            Bitcoin::Increment64 => Cost::from_milliweight(64),
            Bitcoin::Increment8 => Cost::from_milliweight(69),
            Bitcoin::InputsHash => Cost::from_milliweight(122),
            Bitcoin::InputAnnexesHash => Cost::from_milliweight(124),
            Bitcoin::InputAnnexHash => Cost::from_milliweight(77),
            Bitcoin::InputHash => Cost::from_milliweight(832),
            Bitcoin::InputOutpointsHash => Cost::from_milliweight(126),
            Bitcoin::InputPrevOutpoint => Cost::from_milliweight(140),
            Bitcoin::InputScriptsHash => Cost::from_milliweight(129),
            Bitcoin::InputScriptHash => Cost::from_milliweight(143),
            Bitcoin::InputScriptSigsHash => Cost::from_milliweight(127),
            Bitcoin::InputScriptSigHash => Cost::from_milliweight(137),
            Bitcoin::InputSequence => Cost::from_milliweight(78),
            Bitcoin::InputSequencesHash => Cost::from_milliweight(125),
            Bitcoin::InputUtxosHash => Cost::from_milliweight(122),
            Bitcoin::InputUtxoHash => Cost::from_milliweight(824),
            Bitcoin::InputValue => Cost::from_milliweight(81),
            Bitcoin::InputValuesHash => Cost::from_milliweight(126),
            Bitcoin::InternalKey => Cost::from_milliweight(124),
            Bitcoin::IsOne16 => Cost::from_milliweight(64),
            Bitcoin::IsOne32 => Cost::from_milliweight(64),
            Bitcoin::IsOne64 => Cost::from_milliweight(66),
            Bitcoin::IsOne8 => Cost::from_milliweight(47),
            Bitcoin::IsZero16 => Cost::from_milliweight(52),
            Bitcoin::IsZero32 => Cost::from_milliweight(58),
            Bitcoin::IsZero64 => Cost::from_milliweight(68),
            Bitcoin::IsZero8 => Cost::from_milliweight(59),
            Bitcoin::Leftmost16_1 => Cost::from_milliweight(68),
            Bitcoin::Leftmost16_2 => Cost::from_milliweight(58),
            Bitcoin::Leftmost16_4 => Cost::from_milliweight(51),
            Bitcoin::Leftmost16_8 => Cost::from_milliweight(62),
            Bitcoin::Leftmost32_1 => Cost::from_milliweight(53),
            Bitcoin::Leftmost32_16 => Cost::from_milliweight(63),
            Bitcoin::Leftmost32_2 => Cost::from_milliweight(62),
            Bitcoin::Leftmost32_4 => Cost::from_milliweight(61),
            Bitcoin::Leftmost32_8 => Cost::from_milliweight(60),
            Bitcoin::Leftmost64_1 => Cost::from_milliweight(65),
            Bitcoin::Leftmost64_16 => Cost::from_milliweight(62),
            Bitcoin::Leftmost64_2 => Cost::from_milliweight(61),
            Bitcoin::Leftmost64_32 => Cost::from_milliweight(77),
            Bitcoin::Leftmost64_4 => Cost::from_milliweight(80),
            Bitcoin::Leftmost64_8 => Cost::from_milliweight(54),
            Bitcoin::Leftmost8_1 => Cost::from_milliweight(54),
            Bitcoin::Leftmost8_2 => Cost::from_milliweight(71),
            Bitcoin::Leftmost8_4 => Cost::from_milliweight(65),
            Bitcoin::LeftExtend16_32 => Cost::from_milliweight(72),
            Bitcoin::LeftExtend16_64 => Cost::from_milliweight(69),
            Bitcoin::LeftExtend1_16 => Cost::from_milliweight(50),
            Bitcoin::LeftExtend1_32 => Cost::from_milliweight(48),
            Bitcoin::LeftExtend1_64 => Cost::from_milliweight(49),
            Bitcoin::LeftExtend1_8 => Cost::from_milliweight(46),
            Bitcoin::LeftExtend32_64 => Cost::from_milliweight(69),
            Bitcoin::LeftExtend8_16 => Cost::from_milliweight(58),
            Bitcoin::LeftExtend8_32 => Cost::from_milliweight(86),
            Bitcoin::LeftExtend8_64 => Cost::from_milliweight(98),
            Bitcoin::LeftPadHigh16_32 => Cost::from_milliweight(71),
            Bitcoin::LeftPadHigh16_64 => Cost::from_milliweight(82),
            Bitcoin::LeftPadHigh1_16 => Cost::from_milliweight(106),
            Bitcoin::LeftPadHigh1_32 => Cost::from_milliweight(220),
            Bitcoin::LeftPadHigh1_64 => Cost::from_milliweight(302),
            Bitcoin::LeftPadHigh1_8 => Cost::from_milliweight(73),
            Bitcoin::LeftPadHigh32_64 => Cost::from_milliweight(69),
            Bitcoin::LeftPadHigh8_16 => Cost::from_milliweight(65),
            Bitcoin::LeftPadHigh8_32 => Cost::from_milliweight(105),
            Bitcoin::LeftPadHigh8_64 => Cost::from_milliweight(113),
            Bitcoin::LeftPadLow16_32 => Cost::from_milliweight(65),
            Bitcoin::LeftPadLow16_64 => Cost::from_milliweight(68),
            Bitcoin::LeftPadLow1_16 => Cost::from_milliweight(59),
            Bitcoin::LeftPadLow1_32 => Cost::from_milliweight(47),
            Bitcoin::LeftPadLow1_64 => Cost::from_milliweight(46),
            Bitcoin::LeftPadLow1_8 => Cost::from_milliweight(48),
            Bitcoin::LeftPadLow32_64 => Cost::from_milliweight(62),
            Bitcoin::LeftPadLow8_16 => Cost::from_milliweight(56),
            Bitcoin::LeftPadLow8_32 => Cost::from_milliweight(75),
            Bitcoin::LeftPadLow8_64 => Cost::from_milliweight(116),
            Bitcoin::LeftRotate16 => Cost::from_milliweight(88),
            Bitcoin::LeftRotate32 => Cost::from_milliweight(62),
            Bitcoin::LeftRotate64 => Cost::from_milliweight(68),
            Bitcoin::LeftRotate8 => Cost::from_milliweight(66),
            Bitcoin::LeftShift16 => Cost::from_milliweight(109),
            Bitcoin::LeftShift32 => Cost::from_milliweight(79),
            Bitcoin::LeftShift64 => Cost::from_milliweight(70),
            Bitcoin::LeftShift8 => Cost::from_milliweight(72),
            Bitcoin::LeftShiftWith16 => Cost::from_milliweight(72),
            Bitcoin::LeftShiftWith32 => Cost::from_milliweight(87),
            Bitcoin::LeftShiftWith64 => Cost::from_milliweight(97),
            Bitcoin::LeftShiftWith8 => Cost::from_milliweight(104),
            Bitcoin::Le16 => Cost::from_milliweight(83),
            Bitcoin::Le32 => Cost::from_milliweight(99),
            Bitcoin::Le64 => Cost::from_milliweight(79),
            Bitcoin::Le8 => Cost::from_milliweight(93),
            Bitcoin::LinearCombination1 => Cost::from_milliweight(85743),
            Bitcoin::LinearVerify1 => Cost::from_milliweight(43579),
            Bitcoin::LockTime => Cost::from_milliweight(66),
            Bitcoin::Low1 => Cost::from_milliweight(40),
            Bitcoin::Low16 => Cost::from_milliweight(60),
            Bitcoin::Low32 => Cost::from_milliweight(52),
            Bitcoin::Low64 => Cost::from_milliweight(50),
            Bitcoin::Low8 => Cost::from_milliweight(45),
            Bitcoin::Lt16 => Cost::from_milliweight(83),
            Bitcoin::Lt32 => Cost::from_milliweight(89),
            Bitcoin::Lt64 => Cost::from_milliweight(71),
            Bitcoin::Lt8 => Cost::from_milliweight(86),
            Bitcoin::Maj1 => Cost::from_milliweight(54),
            Bitcoin::Maj16 => Cost::from_milliweight(85),
            Bitcoin::Maj32 => Cost::from_milliweight(73),
            Bitcoin::Maj64 => Cost::from_milliweight(79),
            Bitcoin::Maj8 => Cost::from_milliweight(64),
            Bitcoin::Max16 => Cost::from_milliweight(80),
            Bitcoin::Max32 => Cost::from_milliweight(70),
            Bitcoin::Max64 => Cost::from_milliweight(75),
            Bitcoin::Max8 => Cost::from_milliweight(79),
            Bitcoin::Median16 => Cost::from_milliweight(80),
            Bitcoin::Median32 => Cost::from_milliweight(77),
            Bitcoin::Median64 => Cost::from_milliweight(89),
            Bitcoin::Median8 => Cost::from_milliweight(77),
            Bitcoin::Min16 => Cost::from_milliweight(83),
            Bitcoin::Min32 => Cost::from_milliweight(96),
            Bitcoin::Min64 => Cost::from_milliweight(82),
            Bitcoin::Min8 => Cost::from_milliweight(78),
            Bitcoin::Modulo16 => Cost::from_milliweight(85),
            Bitcoin::Modulo32 => Cost::from_milliweight(81),
            Bitcoin::Modulo64 => Cost::from_milliweight(71),
            Bitcoin::Modulo8 => Cost::from_milliweight(85),
            Bitcoin::Multiply16 => Cost::from_milliweight(79),
            Bitcoin::Multiply32 => Cost::from_milliweight(78),
            Bitcoin::Multiply64 => Cost::from_milliweight(72),
            Bitcoin::Multiply8 => Cost::from_milliweight(79),
            Bitcoin::Negate16 => Cost::from_milliweight(69),
            Bitcoin::Negate32 => Cost::from_milliweight(56),
            Bitcoin::Negate64 => Cost::from_milliweight(56),
            Bitcoin::Negate8 => Cost::from_milliweight(69),
            Bitcoin::NumInputs => Cost::from_milliweight(74),
            Bitcoin::NumOutputs => Cost::from_milliweight(68),
            Bitcoin::One16 => Cost::from_milliweight(45),
            Bitcoin::One32 => Cost::from_milliweight(45),
            Bitcoin::One64 => Cost::from_milliweight(45),
            Bitcoin::One8 => Cost::from_milliweight(46),
            Bitcoin::Or1 => Cost::from_milliweight(56),
            Bitcoin::Or16 => Cost::from_milliweight(78),
            Bitcoin::Or32 => Cost::from_milliweight(80),
            Bitcoin::Or64 => Cost::from_milliweight(71),
            Bitcoin::Or8 => Cost::from_milliweight(81),
            Bitcoin::OutpointHash => Cost::from_milliweight(1788),
            Bitcoin::OutputsHash => Cost::from_milliweight(117),
            Bitcoin::OutputHash => Cost::from_milliweight(822),
            Bitcoin::OutputScriptsHash => Cost::from_milliweight(123),
            Bitcoin::OutputScriptHash => Cost::from_milliweight(135),
            Bitcoin::OutputValue => Cost::from_milliweight(82),
            Bitcoin::OutputValuesHash => Cost::from_milliweight(119),
            Bitcoin::ParseLock => Cost::from_milliweight(82),
            Bitcoin::ParseSequence => Cost::from_milliweight(93),
            Bitcoin::PointVerify1 => Cost::from_milliweight(41394),
            Bitcoin::Rightmost16_1 => Cost::from_milliweight(70),
            Bitcoin::Rightmost16_2 => Cost::from_milliweight(65),
            Bitcoin::Rightmost16_4 => Cost::from_milliweight(72),
            Bitcoin::Rightmost16_8 => Cost::from_milliweight(69),
            Bitcoin::Rightmost32_1 => Cost::from_milliweight(70),
            Bitcoin::Rightmost32_16 => Cost::from_milliweight(56),
            Bitcoin::Rightmost32_2 => Cost::from_milliweight(74),
            Bitcoin::Rightmost32_4 => Cost::from_milliweight(57),
            Bitcoin::Rightmost32_8 => Cost::from_milliweight(55),
            Bitcoin::Rightmost64_1 => Cost::from_milliweight(61),
            Bitcoin::Rightmost64_16 => Cost::from_milliweight(63),
            Bitcoin::Rightmost64_2 => Cost::from_milliweight(65),
            Bitcoin::Rightmost64_32 => Cost::from_milliweight(64),
            Bitcoin::Rightmost64_4 => Cost::from_milliweight(57),
            Bitcoin::Rightmost64_8 => Cost::from_milliweight(49),
            Bitcoin::Rightmost8_1 => Cost::from_milliweight(65),
            Bitcoin::Rightmost8_2 => Cost::from_milliweight(63),
            Bitcoin::Rightmost8_4 => Cost::from_milliweight(56),
            Bitcoin::RightExtend16_32 => Cost::from_milliweight(73),
            Bitcoin::RightExtend16_64 => Cost::from_milliweight(70),
            Bitcoin::RightExtend32_64 => Cost::from_milliweight(62),
            Bitcoin::RightExtend8_16 => Cost::from_milliweight(63),
            Bitcoin::RightExtend8_32 => Cost::from_milliweight(69),
            Bitcoin::RightExtend8_64 => Cost::from_milliweight(141),
            Bitcoin::RightPadHigh16_32 => Cost::from_milliweight(66),
            Bitcoin::RightPadHigh16_64 => Cost::from_milliweight(81),
            Bitcoin::RightPadHigh1_16 => Cost::from_milliweight(114),
            Bitcoin::RightPadHigh1_32 => Cost::from_milliweight(220),
            Bitcoin::RightPadHigh1_64 => Cost::from_milliweight(313),
            Bitcoin::RightPadHigh1_8 => Cost::from_milliweight(73),
            Bitcoin::RightPadHigh32_64 => Cost::from_milliweight(62),
            Bitcoin::RightPadHigh8_16 => Cost::from_milliweight(75),
            Bitcoin::RightPadHigh8_32 => Cost::from_milliweight(81),
            Bitcoin::RightPadHigh8_64 => Cost::from_milliweight(118),
            Bitcoin::RightPadLow16_32 => Cost::from_milliweight(62),
            Bitcoin::RightPadLow16_64 => Cost::from_milliweight(98),
            Bitcoin::RightPadLow1_16 => Cost::from_milliweight(60),
            Bitcoin::RightPadLow1_32 => Cost::from_milliweight(47),
            Bitcoin::RightPadLow1_64 => Cost::from_milliweight(57),
            Bitcoin::RightPadLow1_8 => Cost::from_milliweight(48),
            Bitcoin::RightPadLow32_64 => Cost::from_milliweight(74),
            Bitcoin::RightPadLow8_16 => Cost::from_milliweight(62),
            Bitcoin::RightPadLow8_32 => Cost::from_milliweight(69),
            Bitcoin::RightPadLow8_64 => Cost::from_milliweight(98),
            Bitcoin::RightRotate16 => Cost::from_milliweight(67),
            Bitcoin::RightRotate32 => Cost::from_milliweight(77),
            Bitcoin::RightRotate64 => Cost::from_milliweight(64),
            Bitcoin::RightRotate8 => Cost::from_milliweight(72),
            Bitcoin::RightShift16 => Cost::from_milliweight(60),
            Bitcoin::RightShift32 => Cost::from_milliweight(69),
            Bitcoin::RightShift64 => Cost::from_milliweight(68),
            Bitcoin::RightShift8 => Cost::from_milliweight(63),
            Bitcoin::RightShiftWith16 => Cost::from_milliweight(83),
            Bitcoin::RightShiftWith32 => Cost::from_milliweight(78),
            Bitcoin::RightShiftWith64 => Cost::from_milliweight(72),
            Bitcoin::RightShiftWith8 => Cost::from_milliweight(71),
            Bitcoin::ScalarAdd => Cost::from_milliweight(778),
            Bitcoin::ScalarInvert => Cost::from_milliweight(3178),
            Bitcoin::ScalarIsZero => Cost::from_milliweight(271),
            Bitcoin::ScalarMultiply => Cost::from_milliweight(793),
            Bitcoin::ScalarMultiplyLambda => Cost::from_milliweight(567),
            Bitcoin::ScalarNegate => Cost::from_milliweight(516),
            Bitcoin::ScalarNormalize => Cost::from_milliweight(500),
            Bitcoin::ScalarSquare => Cost::from_milliweight(571),
            Bitcoin::Scale => Cost::from_milliweight(73548),
            Bitcoin::ScriptCMR => Cost::from_milliweight(122),
            Bitcoin::Sha256Block => Cost::from_milliweight(765),
            Bitcoin::Sha256Ctx8Add1 => Cost::from_milliweight(664),
            Bitcoin::Sha256Ctx8Add128 => Cost::from_milliweight(1778),
            Bitcoin::Sha256Ctx8Add16 => Cost::from_milliweight(781),
            Bitcoin::Sha256Ctx8Add2 => Cost::from_milliweight(674),
            Bitcoin::Sha256Ctx8Add256 => Cost::from_milliweight(2894),
            Bitcoin::Sha256Ctx8Add32 => Cost::from_milliweight(928),
            Bitcoin::Sha256Ctx8Add4 => Cost::from_milliweight(656),
            Bitcoin::Sha256Ctx8Add512 => Cost::from_milliweight(5161),
            Bitcoin::Sha256Ctx8Add64 => Cost::from_milliweight(1220),
            Bitcoin::Sha256Ctx8Add8 => Cost::from_milliweight(694),
            Bitcoin::Sha256Ctx8AddBuffer511 => Cost::from_milliweight(5137),
            Bitcoin::Sha256Ctx8Finalize => Cost::from_milliweight(833),
            Bitcoin::Sha256Ctx8Init => Cost::from_milliweight(123),
            Bitcoin::Sha256Iv => Cost::from_milliweight(92),
            Bitcoin::SigAllHash => Cost::from_milliweight(120),
            Bitcoin::Some1 => Cost::from_milliweight(60),
            Bitcoin::Some16 => Cost::from_milliweight(52),
            Bitcoin::Some32 => Cost::from_milliweight(49),
            Bitcoin::Some64 => Cost::from_milliweight(62),
            Bitcoin::Some8 => Cost::from_milliweight(57),
            Bitcoin::Subtract16 => Cost::from_milliweight(93),
            Bitcoin::Subtract32 => Cost::from_milliweight(87),
            Bitcoin::Subtract64 => Cost::from_milliweight(125),
            Bitcoin::Subtract8 => Cost::from_milliweight(96),
            Bitcoin::Swu => Cost::from_milliweight(32780),
            Bitcoin::TapdataInit => Cost::from_milliweight(1233),
            Bitcoin::TapleafHash => Cost::from_milliweight(116),
            Bitcoin::TapleafVersion => Cost::from_milliweight(66),
            Bitcoin::Tappath => Cost::from_milliweight(76),
            Bitcoin::TappathHash => Cost::from_milliweight(123),
            Bitcoin::TapEnvHash => Cost::from_milliweight(120),
            Bitcoin::TotalInputValue => Cost::from_milliweight(69),
            Bitcoin::TotalOutputValue => Cost::from_milliweight(71),
            Bitcoin::TransactionId => Cost::from_milliweight(122),
            Bitcoin::TxHash => Cost::from_milliweight(119),
            Bitcoin::TxIsFinal => Cost::from_milliweight(62),
            Bitcoin::TxLockDistance => Cost::from_milliweight(72),
            Bitcoin::TxLockDuration => Cost::from_milliweight(66),
            Bitcoin::TxLockHeight => Cost::from_milliweight(72),
            Bitcoin::TxLockTime => Cost::from_milliweight(72),
            Bitcoin::Verify => Cost::from_milliweight(44),
            Bitcoin::Version => Cost::from_milliweight(78),
            Bitcoin::Xor1 => Cost::from_milliweight(60),
            Bitcoin::Xor16 => Cost::from_milliweight(73),
            Bitcoin::Xor32 => Cost::from_milliweight(77),
            Bitcoin::Xor64 => Cost::from_milliweight(68),
            Bitcoin::Xor8 => Cost::from_milliweight(80),
            Bitcoin::XorXor1 => Cost::from_milliweight(50),
            Bitcoin::XorXor16 => Cost::from_milliweight(82),
            Bitcoin::XorXor32 => Cost::from_milliweight(82),
            Bitcoin::XorXor64 => Cost::from_milliweight(80),
            Bitcoin::XorXor8 => Cost::from_milliweight(86),
        }
    }

    fn parse(s: &str) -> Result<Self, crate::Error> where Self: Sized {
        str::FromStr::from_str(s)
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
            Bitcoin::And1 => f.write_str("and_1"),
            Bitcoin::And16 => f.write_str("and_16"),
            Bitcoin::And32 => f.write_str("and_32"),
            Bitcoin::And64 => f.write_str("and_64"),
            Bitcoin::And8 => f.write_str("and_8"),
            Bitcoin::AnnexHash => f.write_str("annex_hash"),
            Bitcoin::Bip0340Verify => f.write_str("bip_0340_verify"),
            Bitcoin::BuildTapbranch => f.write_str("build_tapbranch"),
            Bitcoin::BuildTapleafSimplicity => f.write_str("build_tapleaf_simplicity"),
            Bitcoin::BuildTaptweak => f.write_str("build_taptweak"),
            Bitcoin::Ch1 => f.write_str("ch_1"),
            Bitcoin::Ch16 => f.write_str("ch_16"),
            Bitcoin::Ch32 => f.write_str("ch_32"),
            Bitcoin::Ch64 => f.write_str("ch_64"),
            Bitcoin::Ch8 => f.write_str("ch_8"),
            Bitcoin::CheckLockDistance => f.write_str("check_lock_distance"),
            Bitcoin::CheckLockDuration => f.write_str("check_lock_duration"),
            Bitcoin::CheckLockHeight => f.write_str("check_lock_height"),
            Bitcoin::CheckLockTime => f.write_str("check_lock_time"),
            Bitcoin::CheckSigVerify => f.write_str("check_sig_verify"),
            Bitcoin::Complement1 => f.write_str("complement_1"),
            Bitcoin::Complement16 => f.write_str("complement_16"),
            Bitcoin::Complement32 => f.write_str("complement_32"),
            Bitcoin::Complement64 => f.write_str("complement_64"),
            Bitcoin::Complement8 => f.write_str("complement_8"),
            Bitcoin::CurrentAnnexHash => f.write_str("current_annex_hash"),
            Bitcoin::CurrentIndex => f.write_str("current_index"),
            Bitcoin::CurrentPrevOutpoint => f.write_str("current_prev_outpoint"),
            Bitcoin::CurrentScriptHash => f.write_str("current_script_hash"),
            Bitcoin::CurrentScriptSigHash => f.write_str("current_script_sig_hash"),
            Bitcoin::CurrentSequence => f.write_str("current_sequence"),
            Bitcoin::CurrentValue => f.write_str("current_value"),
            Bitcoin::Decompress => f.write_str("decompress"),
            Bitcoin::Decrement16 => f.write_str("decrement_16"),
            Bitcoin::Decrement32 => f.write_str("decrement_32"),
            Bitcoin::Decrement64 => f.write_str("decrement_64"),
            Bitcoin::Decrement8 => f.write_str("decrement_8"),
            Bitcoin::DivMod128_64 => f.write_str("div_mod_128_64"),
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
            Bitcoin::Eq1 => f.write_str("eq_1"),
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
            Bitcoin::Fee => f.write_str("fee"),
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
            Bitcoin::FullLeftShift16_1 => f.write_str("full_left_shift_16_1"),
            Bitcoin::FullLeftShift16_2 => f.write_str("full_left_shift_16_2"),
            Bitcoin::FullLeftShift16_4 => f.write_str("full_left_shift_16_4"),
            Bitcoin::FullLeftShift16_8 => f.write_str("full_left_shift_16_8"),
            Bitcoin::FullLeftShift32_1 => f.write_str("full_left_shift_32_1"),
            Bitcoin::FullLeftShift32_16 => f.write_str("full_left_shift_32_16"),
            Bitcoin::FullLeftShift32_2 => f.write_str("full_left_shift_32_2"),
            Bitcoin::FullLeftShift32_4 => f.write_str("full_left_shift_32_4"),
            Bitcoin::FullLeftShift32_8 => f.write_str("full_left_shift_32_8"),
            Bitcoin::FullLeftShift64_1 => f.write_str("full_left_shift_64_1"),
            Bitcoin::FullLeftShift64_16 => f.write_str("full_left_shift_64_16"),
            Bitcoin::FullLeftShift64_2 => f.write_str("full_left_shift_64_2"),
            Bitcoin::FullLeftShift64_32 => f.write_str("full_left_shift_64_32"),
            Bitcoin::FullLeftShift64_4 => f.write_str("full_left_shift_64_4"),
            Bitcoin::FullLeftShift64_8 => f.write_str("full_left_shift_64_8"),
            Bitcoin::FullLeftShift8_1 => f.write_str("full_left_shift_8_1"),
            Bitcoin::FullLeftShift8_2 => f.write_str("full_left_shift_8_2"),
            Bitcoin::FullLeftShift8_4 => f.write_str("full_left_shift_8_4"),
            Bitcoin::FullMultiply16 => f.write_str("full_multiply_16"),
            Bitcoin::FullMultiply32 => f.write_str("full_multiply_32"),
            Bitcoin::FullMultiply64 => f.write_str("full_multiply_64"),
            Bitcoin::FullMultiply8 => f.write_str("full_multiply_8"),
            Bitcoin::FullRightShift16_1 => f.write_str("full_right_shift_16_1"),
            Bitcoin::FullRightShift16_2 => f.write_str("full_right_shift_16_2"),
            Bitcoin::FullRightShift16_4 => f.write_str("full_right_shift_16_4"),
            Bitcoin::FullRightShift16_8 => f.write_str("full_right_shift_16_8"),
            Bitcoin::FullRightShift32_1 => f.write_str("full_right_shift_32_1"),
            Bitcoin::FullRightShift32_16 => f.write_str("full_right_shift_32_16"),
            Bitcoin::FullRightShift32_2 => f.write_str("full_right_shift_32_2"),
            Bitcoin::FullRightShift32_4 => f.write_str("full_right_shift_32_4"),
            Bitcoin::FullRightShift32_8 => f.write_str("full_right_shift_32_8"),
            Bitcoin::FullRightShift64_1 => f.write_str("full_right_shift_64_1"),
            Bitcoin::FullRightShift64_16 => f.write_str("full_right_shift_64_16"),
            Bitcoin::FullRightShift64_2 => f.write_str("full_right_shift_64_2"),
            Bitcoin::FullRightShift64_32 => f.write_str("full_right_shift_64_32"),
            Bitcoin::FullRightShift64_4 => f.write_str("full_right_shift_64_4"),
            Bitcoin::FullRightShift64_8 => f.write_str("full_right_shift_64_8"),
            Bitcoin::FullRightShift8_1 => f.write_str("full_right_shift_8_1"),
            Bitcoin::FullRightShift8_2 => f.write_str("full_right_shift_8_2"),
            Bitcoin::FullRightShift8_4 => f.write_str("full_right_shift_8_4"),
            Bitcoin::FullSubtract16 => f.write_str("full_subtract_16"),
            Bitcoin::FullSubtract32 => f.write_str("full_subtract_32"),
            Bitcoin::FullSubtract64 => f.write_str("full_subtract_64"),
            Bitcoin::FullSubtract8 => f.write_str("full_subtract_8"),
            Bitcoin::GeIsOnCurve => f.write_str("ge_is_on_curve"),
            Bitcoin::GeNegate => f.write_str("ge_negate"),
            Bitcoin::GejAdd => f.write_str("gej_add"),
            Bitcoin::GejDouble => f.write_str("gej_double"),
            Bitcoin::GejEquiv => f.write_str("gej_equiv"),
            Bitcoin::GejGeAdd => f.write_str("gej_ge_add"),
            Bitcoin::GejGeAddEx => f.write_str("gej_ge_add_ex"),
            Bitcoin::GejGeEquiv => f.write_str("gej_ge_equiv"),
            Bitcoin::GejInfinity => f.write_str("gej_infinity"),
            Bitcoin::GejIsInfinity => f.write_str("gej_is_infinity"),
            Bitcoin::GejIsOnCurve => f.write_str("gej_is_on_curve"),
            Bitcoin::GejNegate => f.write_str("gej_negate"),
            Bitcoin::GejNormalize => f.write_str("gej_normalize"),
            Bitcoin::GejRescale => f.write_str("gej_rescale"),
            Bitcoin::GejXEquiv => f.write_str("gej_x_equiv"),
            Bitcoin::GejYIsOdd => f.write_str("gej_y_is_odd"),
            Bitcoin::Generate => f.write_str("generate"),
            Bitcoin::HashToCurve => f.write_str("hash_to_curve"),
            Bitcoin::High1 => f.write_str("high_1"),
            Bitcoin::High16 => f.write_str("high_16"),
            Bitcoin::High32 => f.write_str("high_32"),
            Bitcoin::High64 => f.write_str("high_64"),
            Bitcoin::High8 => f.write_str("high_8"),
            Bitcoin::Increment16 => f.write_str("increment_16"),
            Bitcoin::Increment32 => f.write_str("increment_32"),
            Bitcoin::Increment64 => f.write_str("increment_64"),
            Bitcoin::Increment8 => f.write_str("increment_8"),
            Bitcoin::InputAnnexHash => f.write_str("input_annex_hash"),
            Bitcoin::InputAnnexesHash => f.write_str("input_annexes_hash"),
            Bitcoin::InputHash => f.write_str("input_hash"),
            Bitcoin::InputOutpointsHash => f.write_str("input_outpoints_hash"),
            Bitcoin::InputPrevOutpoint => f.write_str("input_prev_outpoint"),
            Bitcoin::InputScriptHash => f.write_str("input_script_hash"),
            Bitcoin::InputScriptSigHash => f.write_str("input_script_sig_hash"),
            Bitcoin::InputScriptSigsHash => f.write_str("input_script_sigs_hash"),
            Bitcoin::InputScriptsHash => f.write_str("input_scripts_hash"),
            Bitcoin::InputSequence => f.write_str("input_sequence"),
            Bitcoin::InputSequencesHash => f.write_str("input_sequences_hash"),
            Bitcoin::InputUtxoHash => f.write_str("input_utxo_hash"),
            Bitcoin::InputUtxosHash => f.write_str("input_utxos_hash"),
            Bitcoin::InputValue => f.write_str("input_value"),
            Bitcoin::InputValuesHash => f.write_str("input_values_hash"),
            Bitcoin::InputsHash => f.write_str("inputs_hash"),
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
            Bitcoin::LeftExtend16_32 => f.write_str("left_extend_16_32"),
            Bitcoin::LeftExtend16_64 => f.write_str("left_extend_16_64"),
            Bitcoin::LeftExtend1_16 => f.write_str("left_extend_1_16"),
            Bitcoin::LeftExtend1_32 => f.write_str("left_extend_1_32"),
            Bitcoin::LeftExtend1_64 => f.write_str("left_extend_1_64"),
            Bitcoin::LeftExtend1_8 => f.write_str("left_extend_1_8"),
            Bitcoin::LeftExtend32_64 => f.write_str("left_extend_32_64"),
            Bitcoin::LeftExtend8_16 => f.write_str("left_extend_8_16"),
            Bitcoin::LeftExtend8_32 => f.write_str("left_extend_8_32"),
            Bitcoin::LeftExtend8_64 => f.write_str("left_extend_8_64"),
            Bitcoin::LeftPadHigh16_32 => f.write_str("left_pad_high_16_32"),
            Bitcoin::LeftPadHigh16_64 => f.write_str("left_pad_high_16_64"),
            Bitcoin::LeftPadHigh1_16 => f.write_str("left_pad_high_1_16"),
            Bitcoin::LeftPadHigh1_32 => f.write_str("left_pad_high_1_32"),
            Bitcoin::LeftPadHigh1_64 => f.write_str("left_pad_high_1_64"),
            Bitcoin::LeftPadHigh1_8 => f.write_str("left_pad_high_1_8"),
            Bitcoin::LeftPadHigh32_64 => f.write_str("left_pad_high_32_64"),
            Bitcoin::LeftPadHigh8_16 => f.write_str("left_pad_high_8_16"),
            Bitcoin::LeftPadHigh8_32 => f.write_str("left_pad_high_8_32"),
            Bitcoin::LeftPadHigh8_64 => f.write_str("left_pad_high_8_64"),
            Bitcoin::LeftPadLow16_32 => f.write_str("left_pad_low_16_32"),
            Bitcoin::LeftPadLow16_64 => f.write_str("left_pad_low_16_64"),
            Bitcoin::LeftPadLow1_16 => f.write_str("left_pad_low_1_16"),
            Bitcoin::LeftPadLow1_32 => f.write_str("left_pad_low_1_32"),
            Bitcoin::LeftPadLow1_64 => f.write_str("left_pad_low_1_64"),
            Bitcoin::LeftPadLow1_8 => f.write_str("left_pad_low_1_8"),
            Bitcoin::LeftPadLow32_64 => f.write_str("left_pad_low_32_64"),
            Bitcoin::LeftPadLow8_16 => f.write_str("left_pad_low_8_16"),
            Bitcoin::LeftPadLow8_32 => f.write_str("left_pad_low_8_32"),
            Bitcoin::LeftPadLow8_64 => f.write_str("left_pad_low_8_64"),
            Bitcoin::LeftRotate16 => f.write_str("left_rotate_16"),
            Bitcoin::LeftRotate32 => f.write_str("left_rotate_32"),
            Bitcoin::LeftRotate64 => f.write_str("left_rotate_64"),
            Bitcoin::LeftRotate8 => f.write_str("left_rotate_8"),
            Bitcoin::LeftShift16 => f.write_str("left_shift_16"),
            Bitcoin::LeftShift32 => f.write_str("left_shift_32"),
            Bitcoin::LeftShift64 => f.write_str("left_shift_64"),
            Bitcoin::LeftShift8 => f.write_str("left_shift_8"),
            Bitcoin::LeftShiftWith16 => f.write_str("left_shift_with_16"),
            Bitcoin::LeftShiftWith32 => f.write_str("left_shift_with_32"),
            Bitcoin::LeftShiftWith64 => f.write_str("left_shift_with_64"),
            Bitcoin::LeftShiftWith8 => f.write_str("left_shift_with_8"),
            Bitcoin::Leftmost16_1 => f.write_str("leftmost_16_1"),
            Bitcoin::Leftmost16_2 => f.write_str("leftmost_16_2"),
            Bitcoin::Leftmost16_4 => f.write_str("leftmost_16_4"),
            Bitcoin::Leftmost16_8 => f.write_str("leftmost_16_8"),
            Bitcoin::Leftmost32_1 => f.write_str("leftmost_32_1"),
            Bitcoin::Leftmost32_16 => f.write_str("leftmost_32_16"),
            Bitcoin::Leftmost32_2 => f.write_str("leftmost_32_2"),
            Bitcoin::Leftmost32_4 => f.write_str("leftmost_32_4"),
            Bitcoin::Leftmost32_8 => f.write_str("leftmost_32_8"),
            Bitcoin::Leftmost64_1 => f.write_str("leftmost_64_1"),
            Bitcoin::Leftmost64_16 => f.write_str("leftmost_64_16"),
            Bitcoin::Leftmost64_2 => f.write_str("leftmost_64_2"),
            Bitcoin::Leftmost64_32 => f.write_str("leftmost_64_32"),
            Bitcoin::Leftmost64_4 => f.write_str("leftmost_64_4"),
            Bitcoin::Leftmost64_8 => f.write_str("leftmost_64_8"),
            Bitcoin::Leftmost8_1 => f.write_str("leftmost_8_1"),
            Bitcoin::Leftmost8_2 => f.write_str("leftmost_8_2"),
            Bitcoin::Leftmost8_4 => f.write_str("leftmost_8_4"),
            Bitcoin::LinearCombination1 => f.write_str("linear_combination_1"),
            Bitcoin::LinearVerify1 => f.write_str("linear_verify_1"),
            Bitcoin::LockTime => f.write_str("lock_time"),
            Bitcoin::Low1 => f.write_str("low_1"),
            Bitcoin::Low16 => f.write_str("low_16"),
            Bitcoin::Low32 => f.write_str("low_32"),
            Bitcoin::Low64 => f.write_str("low_64"),
            Bitcoin::Low8 => f.write_str("low_8"),
            Bitcoin::Lt16 => f.write_str("lt_16"),
            Bitcoin::Lt32 => f.write_str("lt_32"),
            Bitcoin::Lt64 => f.write_str("lt_64"),
            Bitcoin::Lt8 => f.write_str("lt_8"),
            Bitcoin::Maj1 => f.write_str("maj_1"),
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
            Bitcoin::Or1 => f.write_str("or_1"),
            Bitcoin::Or16 => f.write_str("or_16"),
            Bitcoin::Or32 => f.write_str("or_32"),
            Bitcoin::Or64 => f.write_str("or_64"),
            Bitcoin::Or8 => f.write_str("or_8"),
            Bitcoin::OutpointHash => f.write_str("outpoint_hash"),
            Bitcoin::OutputHash => f.write_str("output_hash"),
            Bitcoin::OutputScriptHash => f.write_str("output_script_hash"),
            Bitcoin::OutputScriptsHash => f.write_str("output_scripts_hash"),
            Bitcoin::OutputValue => f.write_str("output_value"),
            Bitcoin::OutputValuesHash => f.write_str("output_values_hash"),
            Bitcoin::OutputsHash => f.write_str("outputs_hash"),
            Bitcoin::ParseLock => f.write_str("parse_lock"),
            Bitcoin::ParseSequence => f.write_str("parse_sequence"),
            Bitcoin::PointVerify1 => f.write_str("point_verify_1"),
            Bitcoin::RightExtend16_32 => f.write_str("right_extend_16_32"),
            Bitcoin::RightExtend16_64 => f.write_str("right_extend_16_64"),
            Bitcoin::RightExtend32_64 => f.write_str("right_extend_32_64"),
            Bitcoin::RightExtend8_16 => f.write_str("right_extend_8_16"),
            Bitcoin::RightExtend8_32 => f.write_str("right_extend_8_32"),
            Bitcoin::RightExtend8_64 => f.write_str("right_extend_8_64"),
            Bitcoin::RightPadHigh16_32 => f.write_str("right_pad_high_16_32"),
            Bitcoin::RightPadHigh16_64 => f.write_str("right_pad_high_16_64"),
            Bitcoin::RightPadHigh1_16 => f.write_str("right_pad_high_1_16"),
            Bitcoin::RightPadHigh1_32 => f.write_str("right_pad_high_1_32"),
            Bitcoin::RightPadHigh1_64 => f.write_str("right_pad_high_1_64"),
            Bitcoin::RightPadHigh1_8 => f.write_str("right_pad_high_1_8"),
            Bitcoin::RightPadHigh32_64 => f.write_str("right_pad_high_32_64"),
            Bitcoin::RightPadHigh8_16 => f.write_str("right_pad_high_8_16"),
            Bitcoin::RightPadHigh8_32 => f.write_str("right_pad_high_8_32"),
            Bitcoin::RightPadHigh8_64 => f.write_str("right_pad_high_8_64"),
            Bitcoin::RightPadLow16_32 => f.write_str("right_pad_low_16_32"),
            Bitcoin::RightPadLow16_64 => f.write_str("right_pad_low_16_64"),
            Bitcoin::RightPadLow1_16 => f.write_str("right_pad_low_1_16"),
            Bitcoin::RightPadLow1_32 => f.write_str("right_pad_low_1_32"),
            Bitcoin::RightPadLow1_64 => f.write_str("right_pad_low_1_64"),
            Bitcoin::RightPadLow1_8 => f.write_str("right_pad_low_1_8"),
            Bitcoin::RightPadLow32_64 => f.write_str("right_pad_low_32_64"),
            Bitcoin::RightPadLow8_16 => f.write_str("right_pad_low_8_16"),
            Bitcoin::RightPadLow8_32 => f.write_str("right_pad_low_8_32"),
            Bitcoin::RightPadLow8_64 => f.write_str("right_pad_low_8_64"),
            Bitcoin::RightRotate16 => f.write_str("right_rotate_16"),
            Bitcoin::RightRotate32 => f.write_str("right_rotate_32"),
            Bitcoin::RightRotate64 => f.write_str("right_rotate_64"),
            Bitcoin::RightRotate8 => f.write_str("right_rotate_8"),
            Bitcoin::RightShift16 => f.write_str("right_shift_16"),
            Bitcoin::RightShift32 => f.write_str("right_shift_32"),
            Bitcoin::RightShift64 => f.write_str("right_shift_64"),
            Bitcoin::RightShift8 => f.write_str("right_shift_8"),
            Bitcoin::RightShiftWith16 => f.write_str("right_shift_with_16"),
            Bitcoin::RightShiftWith32 => f.write_str("right_shift_with_32"),
            Bitcoin::RightShiftWith64 => f.write_str("right_shift_with_64"),
            Bitcoin::RightShiftWith8 => f.write_str("right_shift_with_8"),
            Bitcoin::Rightmost16_1 => f.write_str("rightmost_16_1"),
            Bitcoin::Rightmost16_2 => f.write_str("rightmost_16_2"),
            Bitcoin::Rightmost16_4 => f.write_str("rightmost_16_4"),
            Bitcoin::Rightmost16_8 => f.write_str("rightmost_16_8"),
            Bitcoin::Rightmost32_1 => f.write_str("rightmost_32_1"),
            Bitcoin::Rightmost32_16 => f.write_str("rightmost_32_16"),
            Bitcoin::Rightmost32_2 => f.write_str("rightmost_32_2"),
            Bitcoin::Rightmost32_4 => f.write_str("rightmost_32_4"),
            Bitcoin::Rightmost32_8 => f.write_str("rightmost_32_8"),
            Bitcoin::Rightmost64_1 => f.write_str("rightmost_64_1"),
            Bitcoin::Rightmost64_16 => f.write_str("rightmost_64_16"),
            Bitcoin::Rightmost64_2 => f.write_str("rightmost_64_2"),
            Bitcoin::Rightmost64_32 => f.write_str("rightmost_64_32"),
            Bitcoin::Rightmost64_4 => f.write_str("rightmost_64_4"),
            Bitcoin::Rightmost64_8 => f.write_str("rightmost_64_8"),
            Bitcoin::Rightmost8_1 => f.write_str("rightmost_8_1"),
            Bitcoin::Rightmost8_2 => f.write_str("rightmost_8_2"),
            Bitcoin::Rightmost8_4 => f.write_str("rightmost_8_4"),
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
            Bitcoin::SigAllHash => f.write_str("sig_all_hash"),
            Bitcoin::Some1 => f.write_str("some_1"),
            Bitcoin::Some16 => f.write_str("some_16"),
            Bitcoin::Some32 => f.write_str("some_32"),
            Bitcoin::Some64 => f.write_str("some_64"),
            Bitcoin::Some8 => f.write_str("some_8"),
            Bitcoin::Subtract16 => f.write_str("subtract_16"),
            Bitcoin::Subtract32 => f.write_str("subtract_32"),
            Bitcoin::Subtract64 => f.write_str("subtract_64"),
            Bitcoin::Subtract8 => f.write_str("subtract_8"),
            Bitcoin::Swu => f.write_str("swu"),
            Bitcoin::TapEnvHash => f.write_str("tap_env_hash"),
            Bitcoin::TapdataInit => f.write_str("tapdata_init"),
            Bitcoin::TapleafHash => f.write_str("tapleaf_hash"),
            Bitcoin::TapleafVersion => f.write_str("tapleaf_version"),
            Bitcoin::Tappath => f.write_str("tappath"),
            Bitcoin::TappathHash => f.write_str("tappath_hash"),
            Bitcoin::TotalInputValue => f.write_str("total_input_value"),
            Bitcoin::TotalOutputValue => f.write_str("total_output_value"),
            Bitcoin::TransactionId => f.write_str("transaction_id"),
            Bitcoin::TxHash => f.write_str("tx_hash"),
            Bitcoin::TxIsFinal => f.write_str("tx_is_final"),
            Bitcoin::TxLockDistance => f.write_str("tx_lock_distance"),
            Bitcoin::TxLockDuration => f.write_str("tx_lock_duration"),
            Bitcoin::TxLockHeight => f.write_str("tx_lock_height"),
            Bitcoin::TxLockTime => f.write_str("tx_lock_time"),
            Bitcoin::Verify => f.write_str("verify"),
            Bitcoin::Version => f.write_str("version"),
            Bitcoin::Xor1 => f.write_str("xor_1"),
            Bitcoin::Xor16 => f.write_str("xor_16"),
            Bitcoin::Xor32 => f.write_str("xor_32"),
            Bitcoin::Xor64 => f.write_str("xor_64"),
            Bitcoin::Xor8 => f.write_str("xor_8"),
            Bitcoin::XorXor1 => f.write_str("xor_xor_1"),
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
            "and_1" => Ok(Bitcoin::And1),
            "and_16" => Ok(Bitcoin::And16),
            "and_32" => Ok(Bitcoin::And32),
            "and_64" => Ok(Bitcoin::And64),
            "and_8" => Ok(Bitcoin::And8),
            "annex_hash" => Ok(Bitcoin::AnnexHash),
            "bip_0340_verify" => Ok(Bitcoin::Bip0340Verify),
            "build_tapbranch" => Ok(Bitcoin::BuildTapbranch),
            "build_tapleaf_simplicity" => Ok(Bitcoin::BuildTapleafSimplicity),
            "build_taptweak" => Ok(Bitcoin::BuildTaptweak),
            "ch_1" => Ok(Bitcoin::Ch1),
            "ch_16" => Ok(Bitcoin::Ch16),
            "ch_32" => Ok(Bitcoin::Ch32),
            "ch_64" => Ok(Bitcoin::Ch64),
            "ch_8" => Ok(Bitcoin::Ch8),
            "check_lock_distance" => Ok(Bitcoin::CheckLockDistance),
            "check_lock_duration" => Ok(Bitcoin::CheckLockDuration),
            "check_lock_height" => Ok(Bitcoin::CheckLockHeight),
            "check_lock_time" => Ok(Bitcoin::CheckLockTime),
            "check_sig_verify" => Ok(Bitcoin::CheckSigVerify),
            "complement_1" => Ok(Bitcoin::Complement1),
            "complement_16" => Ok(Bitcoin::Complement16),
            "complement_32" => Ok(Bitcoin::Complement32),
            "complement_64" => Ok(Bitcoin::Complement64),
            "complement_8" => Ok(Bitcoin::Complement8),
            "current_annex_hash" => Ok(Bitcoin::CurrentAnnexHash),
            "current_index" => Ok(Bitcoin::CurrentIndex),
            "current_prev_outpoint" => Ok(Bitcoin::CurrentPrevOutpoint),
            "current_script_hash" => Ok(Bitcoin::CurrentScriptHash),
            "current_script_sig_hash" => Ok(Bitcoin::CurrentScriptSigHash),
            "current_sequence" => Ok(Bitcoin::CurrentSequence),
            "current_value" => Ok(Bitcoin::CurrentValue),
            "decompress" => Ok(Bitcoin::Decompress),
            "decrement_16" => Ok(Bitcoin::Decrement16),
            "decrement_32" => Ok(Bitcoin::Decrement32),
            "decrement_64" => Ok(Bitcoin::Decrement64),
            "decrement_8" => Ok(Bitcoin::Decrement8),
            "div_mod_128_64" => Ok(Bitcoin::DivMod128_64),
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
            "eq_1" => Ok(Bitcoin::Eq1),
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
            "fee" => Ok(Bitcoin::Fee),
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
            "full_left_shift_16_1" => Ok(Bitcoin::FullLeftShift16_1),
            "full_left_shift_16_2" => Ok(Bitcoin::FullLeftShift16_2),
            "full_left_shift_16_4" => Ok(Bitcoin::FullLeftShift16_4),
            "full_left_shift_16_8" => Ok(Bitcoin::FullLeftShift16_8),
            "full_left_shift_32_1" => Ok(Bitcoin::FullLeftShift32_1),
            "full_left_shift_32_16" => Ok(Bitcoin::FullLeftShift32_16),
            "full_left_shift_32_2" => Ok(Bitcoin::FullLeftShift32_2),
            "full_left_shift_32_4" => Ok(Bitcoin::FullLeftShift32_4),
            "full_left_shift_32_8" => Ok(Bitcoin::FullLeftShift32_8),
            "full_left_shift_64_1" => Ok(Bitcoin::FullLeftShift64_1),
            "full_left_shift_64_16" => Ok(Bitcoin::FullLeftShift64_16),
            "full_left_shift_64_2" => Ok(Bitcoin::FullLeftShift64_2),
            "full_left_shift_64_32" => Ok(Bitcoin::FullLeftShift64_32),
            "full_left_shift_64_4" => Ok(Bitcoin::FullLeftShift64_4),
            "full_left_shift_64_8" => Ok(Bitcoin::FullLeftShift64_8),
            "full_left_shift_8_1" => Ok(Bitcoin::FullLeftShift8_1),
            "full_left_shift_8_2" => Ok(Bitcoin::FullLeftShift8_2),
            "full_left_shift_8_4" => Ok(Bitcoin::FullLeftShift8_4),
            "full_multiply_16" => Ok(Bitcoin::FullMultiply16),
            "full_multiply_32" => Ok(Bitcoin::FullMultiply32),
            "full_multiply_64" => Ok(Bitcoin::FullMultiply64),
            "full_multiply_8" => Ok(Bitcoin::FullMultiply8),
            "full_right_shift_16_1" => Ok(Bitcoin::FullRightShift16_1),
            "full_right_shift_16_2" => Ok(Bitcoin::FullRightShift16_2),
            "full_right_shift_16_4" => Ok(Bitcoin::FullRightShift16_4),
            "full_right_shift_16_8" => Ok(Bitcoin::FullRightShift16_8),
            "full_right_shift_32_1" => Ok(Bitcoin::FullRightShift32_1),
            "full_right_shift_32_16" => Ok(Bitcoin::FullRightShift32_16),
            "full_right_shift_32_2" => Ok(Bitcoin::FullRightShift32_2),
            "full_right_shift_32_4" => Ok(Bitcoin::FullRightShift32_4),
            "full_right_shift_32_8" => Ok(Bitcoin::FullRightShift32_8),
            "full_right_shift_64_1" => Ok(Bitcoin::FullRightShift64_1),
            "full_right_shift_64_16" => Ok(Bitcoin::FullRightShift64_16),
            "full_right_shift_64_2" => Ok(Bitcoin::FullRightShift64_2),
            "full_right_shift_64_32" => Ok(Bitcoin::FullRightShift64_32),
            "full_right_shift_64_4" => Ok(Bitcoin::FullRightShift64_4),
            "full_right_shift_64_8" => Ok(Bitcoin::FullRightShift64_8),
            "full_right_shift_8_1" => Ok(Bitcoin::FullRightShift8_1),
            "full_right_shift_8_2" => Ok(Bitcoin::FullRightShift8_2),
            "full_right_shift_8_4" => Ok(Bitcoin::FullRightShift8_4),
            "full_subtract_16" => Ok(Bitcoin::FullSubtract16),
            "full_subtract_32" => Ok(Bitcoin::FullSubtract32),
            "full_subtract_64" => Ok(Bitcoin::FullSubtract64),
            "full_subtract_8" => Ok(Bitcoin::FullSubtract8),
            "ge_is_on_curve" => Ok(Bitcoin::GeIsOnCurve),
            "ge_negate" => Ok(Bitcoin::GeNegate),
            "gej_add" => Ok(Bitcoin::GejAdd),
            "gej_double" => Ok(Bitcoin::GejDouble),
            "gej_equiv" => Ok(Bitcoin::GejEquiv),
            "gej_ge_add" => Ok(Bitcoin::GejGeAdd),
            "gej_ge_add_ex" => Ok(Bitcoin::GejGeAddEx),
            "gej_ge_equiv" => Ok(Bitcoin::GejGeEquiv),
            "gej_infinity" => Ok(Bitcoin::GejInfinity),
            "gej_is_infinity" => Ok(Bitcoin::GejIsInfinity),
            "gej_is_on_curve" => Ok(Bitcoin::GejIsOnCurve),
            "gej_negate" => Ok(Bitcoin::GejNegate),
            "gej_normalize" => Ok(Bitcoin::GejNormalize),
            "gej_rescale" => Ok(Bitcoin::GejRescale),
            "gej_x_equiv" => Ok(Bitcoin::GejXEquiv),
            "gej_y_is_odd" => Ok(Bitcoin::GejYIsOdd),
            "generate" => Ok(Bitcoin::Generate),
            "hash_to_curve" => Ok(Bitcoin::HashToCurve),
            "high_1" => Ok(Bitcoin::High1),
            "high_16" => Ok(Bitcoin::High16),
            "high_32" => Ok(Bitcoin::High32),
            "high_64" => Ok(Bitcoin::High64),
            "high_8" => Ok(Bitcoin::High8),
            "increment_16" => Ok(Bitcoin::Increment16),
            "increment_32" => Ok(Bitcoin::Increment32),
            "increment_64" => Ok(Bitcoin::Increment64),
            "increment_8" => Ok(Bitcoin::Increment8),
            "input_annex_hash" => Ok(Bitcoin::InputAnnexHash),
            "input_annexes_hash" => Ok(Bitcoin::InputAnnexesHash),
            "input_hash" => Ok(Bitcoin::InputHash),
            "input_outpoints_hash" => Ok(Bitcoin::InputOutpointsHash),
            "input_prev_outpoint" => Ok(Bitcoin::InputPrevOutpoint),
            "input_script_hash" => Ok(Bitcoin::InputScriptHash),
            "input_script_sig_hash" => Ok(Bitcoin::InputScriptSigHash),
            "input_script_sigs_hash" => Ok(Bitcoin::InputScriptSigsHash),
            "input_scripts_hash" => Ok(Bitcoin::InputScriptsHash),
            "input_sequence" => Ok(Bitcoin::InputSequence),
            "input_sequences_hash" => Ok(Bitcoin::InputSequencesHash),
            "input_utxo_hash" => Ok(Bitcoin::InputUtxoHash),
            "input_utxos_hash" => Ok(Bitcoin::InputUtxosHash),
            "input_value" => Ok(Bitcoin::InputValue),
            "input_values_hash" => Ok(Bitcoin::InputValuesHash),
            "inputs_hash" => Ok(Bitcoin::InputsHash),
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
            "left_extend_16_32" => Ok(Bitcoin::LeftExtend16_32),
            "left_extend_16_64" => Ok(Bitcoin::LeftExtend16_64),
            "left_extend_1_16" => Ok(Bitcoin::LeftExtend1_16),
            "left_extend_1_32" => Ok(Bitcoin::LeftExtend1_32),
            "left_extend_1_64" => Ok(Bitcoin::LeftExtend1_64),
            "left_extend_1_8" => Ok(Bitcoin::LeftExtend1_8),
            "left_extend_32_64" => Ok(Bitcoin::LeftExtend32_64),
            "left_extend_8_16" => Ok(Bitcoin::LeftExtend8_16),
            "left_extend_8_32" => Ok(Bitcoin::LeftExtend8_32),
            "left_extend_8_64" => Ok(Bitcoin::LeftExtend8_64),
            "left_pad_high_16_32" => Ok(Bitcoin::LeftPadHigh16_32),
            "left_pad_high_16_64" => Ok(Bitcoin::LeftPadHigh16_64),
            "left_pad_high_1_16" => Ok(Bitcoin::LeftPadHigh1_16),
            "left_pad_high_1_32" => Ok(Bitcoin::LeftPadHigh1_32),
            "left_pad_high_1_64" => Ok(Bitcoin::LeftPadHigh1_64),
            "left_pad_high_1_8" => Ok(Bitcoin::LeftPadHigh1_8),
            "left_pad_high_32_64" => Ok(Bitcoin::LeftPadHigh32_64),
            "left_pad_high_8_16" => Ok(Bitcoin::LeftPadHigh8_16),
            "left_pad_high_8_32" => Ok(Bitcoin::LeftPadHigh8_32),
            "left_pad_high_8_64" => Ok(Bitcoin::LeftPadHigh8_64),
            "left_pad_low_16_32" => Ok(Bitcoin::LeftPadLow16_32),
            "left_pad_low_16_64" => Ok(Bitcoin::LeftPadLow16_64),
            "left_pad_low_1_16" => Ok(Bitcoin::LeftPadLow1_16),
            "left_pad_low_1_32" => Ok(Bitcoin::LeftPadLow1_32),
            "left_pad_low_1_64" => Ok(Bitcoin::LeftPadLow1_64),
            "left_pad_low_1_8" => Ok(Bitcoin::LeftPadLow1_8),
            "left_pad_low_32_64" => Ok(Bitcoin::LeftPadLow32_64),
            "left_pad_low_8_16" => Ok(Bitcoin::LeftPadLow8_16),
            "left_pad_low_8_32" => Ok(Bitcoin::LeftPadLow8_32),
            "left_pad_low_8_64" => Ok(Bitcoin::LeftPadLow8_64),
            "left_rotate_16" => Ok(Bitcoin::LeftRotate16),
            "left_rotate_32" => Ok(Bitcoin::LeftRotate32),
            "left_rotate_64" => Ok(Bitcoin::LeftRotate64),
            "left_rotate_8" => Ok(Bitcoin::LeftRotate8),
            "left_shift_16" => Ok(Bitcoin::LeftShift16),
            "left_shift_32" => Ok(Bitcoin::LeftShift32),
            "left_shift_64" => Ok(Bitcoin::LeftShift64),
            "left_shift_8" => Ok(Bitcoin::LeftShift8),
            "left_shift_with_16" => Ok(Bitcoin::LeftShiftWith16),
            "left_shift_with_32" => Ok(Bitcoin::LeftShiftWith32),
            "left_shift_with_64" => Ok(Bitcoin::LeftShiftWith64),
            "left_shift_with_8" => Ok(Bitcoin::LeftShiftWith8),
            "leftmost_16_1" => Ok(Bitcoin::Leftmost16_1),
            "leftmost_16_2" => Ok(Bitcoin::Leftmost16_2),
            "leftmost_16_4" => Ok(Bitcoin::Leftmost16_4),
            "leftmost_16_8" => Ok(Bitcoin::Leftmost16_8),
            "leftmost_32_1" => Ok(Bitcoin::Leftmost32_1),
            "leftmost_32_16" => Ok(Bitcoin::Leftmost32_16),
            "leftmost_32_2" => Ok(Bitcoin::Leftmost32_2),
            "leftmost_32_4" => Ok(Bitcoin::Leftmost32_4),
            "leftmost_32_8" => Ok(Bitcoin::Leftmost32_8),
            "leftmost_64_1" => Ok(Bitcoin::Leftmost64_1),
            "leftmost_64_16" => Ok(Bitcoin::Leftmost64_16),
            "leftmost_64_2" => Ok(Bitcoin::Leftmost64_2),
            "leftmost_64_32" => Ok(Bitcoin::Leftmost64_32),
            "leftmost_64_4" => Ok(Bitcoin::Leftmost64_4),
            "leftmost_64_8" => Ok(Bitcoin::Leftmost64_8),
            "leftmost_8_1" => Ok(Bitcoin::Leftmost8_1),
            "leftmost_8_2" => Ok(Bitcoin::Leftmost8_2),
            "leftmost_8_4" => Ok(Bitcoin::Leftmost8_4),
            "linear_combination_1" => Ok(Bitcoin::LinearCombination1),
            "linear_verify_1" => Ok(Bitcoin::LinearVerify1),
            "lock_time" => Ok(Bitcoin::LockTime),
            "low_1" => Ok(Bitcoin::Low1),
            "low_16" => Ok(Bitcoin::Low16),
            "low_32" => Ok(Bitcoin::Low32),
            "low_64" => Ok(Bitcoin::Low64),
            "low_8" => Ok(Bitcoin::Low8),
            "lt_16" => Ok(Bitcoin::Lt16),
            "lt_32" => Ok(Bitcoin::Lt32),
            "lt_64" => Ok(Bitcoin::Lt64),
            "lt_8" => Ok(Bitcoin::Lt8),
            "maj_1" => Ok(Bitcoin::Maj1),
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
            "or_1" => Ok(Bitcoin::Or1),
            "or_16" => Ok(Bitcoin::Or16),
            "or_32" => Ok(Bitcoin::Or32),
            "or_64" => Ok(Bitcoin::Or64),
            "or_8" => Ok(Bitcoin::Or8),
            "outpoint_hash" => Ok(Bitcoin::OutpointHash),
            "output_hash" => Ok(Bitcoin::OutputHash),
            "output_script_hash" => Ok(Bitcoin::OutputScriptHash),
            "output_scripts_hash" => Ok(Bitcoin::OutputScriptsHash),
            "output_value" => Ok(Bitcoin::OutputValue),
            "output_values_hash" => Ok(Bitcoin::OutputValuesHash),
            "outputs_hash" => Ok(Bitcoin::OutputsHash),
            "parse_lock" => Ok(Bitcoin::ParseLock),
            "parse_sequence" => Ok(Bitcoin::ParseSequence),
            "point_verify_1" => Ok(Bitcoin::PointVerify1),
            "right_extend_16_32" => Ok(Bitcoin::RightExtend16_32),
            "right_extend_16_64" => Ok(Bitcoin::RightExtend16_64),
            "right_extend_32_64" => Ok(Bitcoin::RightExtend32_64),
            "right_extend_8_16" => Ok(Bitcoin::RightExtend8_16),
            "right_extend_8_32" => Ok(Bitcoin::RightExtend8_32),
            "right_extend_8_64" => Ok(Bitcoin::RightExtend8_64),
            "right_pad_high_16_32" => Ok(Bitcoin::RightPadHigh16_32),
            "right_pad_high_16_64" => Ok(Bitcoin::RightPadHigh16_64),
            "right_pad_high_1_16" => Ok(Bitcoin::RightPadHigh1_16),
            "right_pad_high_1_32" => Ok(Bitcoin::RightPadHigh1_32),
            "right_pad_high_1_64" => Ok(Bitcoin::RightPadHigh1_64),
            "right_pad_high_1_8" => Ok(Bitcoin::RightPadHigh1_8),
            "right_pad_high_32_64" => Ok(Bitcoin::RightPadHigh32_64),
            "right_pad_high_8_16" => Ok(Bitcoin::RightPadHigh8_16),
            "right_pad_high_8_32" => Ok(Bitcoin::RightPadHigh8_32),
            "right_pad_high_8_64" => Ok(Bitcoin::RightPadHigh8_64),
            "right_pad_low_16_32" => Ok(Bitcoin::RightPadLow16_32),
            "right_pad_low_16_64" => Ok(Bitcoin::RightPadLow16_64),
            "right_pad_low_1_16" => Ok(Bitcoin::RightPadLow1_16),
            "right_pad_low_1_32" => Ok(Bitcoin::RightPadLow1_32),
            "right_pad_low_1_64" => Ok(Bitcoin::RightPadLow1_64),
            "right_pad_low_1_8" => Ok(Bitcoin::RightPadLow1_8),
            "right_pad_low_32_64" => Ok(Bitcoin::RightPadLow32_64),
            "right_pad_low_8_16" => Ok(Bitcoin::RightPadLow8_16),
            "right_pad_low_8_32" => Ok(Bitcoin::RightPadLow8_32),
            "right_pad_low_8_64" => Ok(Bitcoin::RightPadLow8_64),
            "right_rotate_16" => Ok(Bitcoin::RightRotate16),
            "right_rotate_32" => Ok(Bitcoin::RightRotate32),
            "right_rotate_64" => Ok(Bitcoin::RightRotate64),
            "right_rotate_8" => Ok(Bitcoin::RightRotate8),
            "right_shift_16" => Ok(Bitcoin::RightShift16),
            "right_shift_32" => Ok(Bitcoin::RightShift32),
            "right_shift_64" => Ok(Bitcoin::RightShift64),
            "right_shift_8" => Ok(Bitcoin::RightShift8),
            "right_shift_with_16" => Ok(Bitcoin::RightShiftWith16),
            "right_shift_with_32" => Ok(Bitcoin::RightShiftWith32),
            "right_shift_with_64" => Ok(Bitcoin::RightShiftWith64),
            "right_shift_with_8" => Ok(Bitcoin::RightShiftWith8),
            "rightmost_16_1" => Ok(Bitcoin::Rightmost16_1),
            "rightmost_16_2" => Ok(Bitcoin::Rightmost16_2),
            "rightmost_16_4" => Ok(Bitcoin::Rightmost16_4),
            "rightmost_16_8" => Ok(Bitcoin::Rightmost16_8),
            "rightmost_32_1" => Ok(Bitcoin::Rightmost32_1),
            "rightmost_32_16" => Ok(Bitcoin::Rightmost32_16),
            "rightmost_32_2" => Ok(Bitcoin::Rightmost32_2),
            "rightmost_32_4" => Ok(Bitcoin::Rightmost32_4),
            "rightmost_32_8" => Ok(Bitcoin::Rightmost32_8),
            "rightmost_64_1" => Ok(Bitcoin::Rightmost64_1),
            "rightmost_64_16" => Ok(Bitcoin::Rightmost64_16),
            "rightmost_64_2" => Ok(Bitcoin::Rightmost64_2),
            "rightmost_64_32" => Ok(Bitcoin::Rightmost64_32),
            "rightmost_64_4" => Ok(Bitcoin::Rightmost64_4),
            "rightmost_64_8" => Ok(Bitcoin::Rightmost64_8),
            "rightmost_8_1" => Ok(Bitcoin::Rightmost8_1),
            "rightmost_8_2" => Ok(Bitcoin::Rightmost8_2),
            "rightmost_8_4" => Ok(Bitcoin::Rightmost8_4),
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
            "sig_all_hash" => Ok(Bitcoin::SigAllHash),
            "some_1" => Ok(Bitcoin::Some1),
            "some_16" => Ok(Bitcoin::Some16),
            "some_32" => Ok(Bitcoin::Some32),
            "some_64" => Ok(Bitcoin::Some64),
            "some_8" => Ok(Bitcoin::Some8),
            "subtract_16" => Ok(Bitcoin::Subtract16),
            "subtract_32" => Ok(Bitcoin::Subtract32),
            "subtract_64" => Ok(Bitcoin::Subtract64),
            "subtract_8" => Ok(Bitcoin::Subtract8),
            "swu" => Ok(Bitcoin::Swu),
            "tap_env_hash" => Ok(Bitcoin::TapEnvHash),
            "tapdata_init" => Ok(Bitcoin::TapdataInit),
            "tapleaf_hash" => Ok(Bitcoin::TapleafHash),
            "tapleaf_version" => Ok(Bitcoin::TapleafVersion),
            "tappath" => Ok(Bitcoin::Tappath),
            "tappath_hash" => Ok(Bitcoin::TappathHash),
            "total_input_value" => Ok(Bitcoin::TotalInputValue),
            "total_output_value" => Ok(Bitcoin::TotalOutputValue),
            "transaction_id" => Ok(Bitcoin::TransactionId),
            "tx_hash" => Ok(Bitcoin::TxHash),
            "tx_is_final" => Ok(Bitcoin::TxIsFinal),
            "tx_lock_distance" => Ok(Bitcoin::TxLockDistance),
            "tx_lock_duration" => Ok(Bitcoin::TxLockDuration),
            "tx_lock_height" => Ok(Bitcoin::TxLockHeight),
            "tx_lock_time" => Ok(Bitcoin::TxLockTime),
            "verify" => Ok(Bitcoin::Verify),
            "version" => Ok(Bitcoin::Version),
            "xor_1" => Ok(Bitcoin::Xor1),
            "xor_16" => Ok(Bitcoin::Xor16),
            "xor_32" => Ok(Bitcoin::Xor32),
            "xor_64" => Ok(Bitcoin::Xor64),
            "xor_8" => Ok(Bitcoin::Xor8),
            "xor_xor_1" => Ok(Bitcoin::XorXor1),
            "xor_xor_16" => Ok(Bitcoin::XorXor16),
            "xor_xor_32" => Ok(Bitcoin::XorXor32),
            "xor_xor_64" => Ok(Bitcoin::XorXor64),
            "xor_xor_8" => Ok(Bitcoin::XorXor8),
            x => Err(crate::Error::InvalidJetName(x.to_owned())),
        }
    }
}

pub(crate) fn c_jet_ptr(jet: &Bitcoin) -> fn(&mut CFrameItem, CFrameItem, &c_bitcoin::CTxEnv) -> bool {
    match jet {
        Bitcoin::All8 => simplicity_sys::c_jets::jets_wrapper::all_8,
        Bitcoin::Eq256 => simplicity_sys::c_jets::jets_wrapper::eq_256,
        Bitcoin::Sha256Ctx8Add1 => simplicity_sys::c_jets::jets_wrapper::sha_256_ctx_8_add_1,
        Bitcoin::Sha256Ctx8Add2 => simplicity_sys::c_jets::jets_wrapper::sha_256_ctx_8_add_2,
        Bitcoin::Sha256Ctx8Add4 => simplicity_sys::c_jets::jets_wrapper::sha_256_ctx_8_add_4,
        Bitcoin::Sha256Ctx8Add8 => simplicity_sys::c_jets::jets_wrapper::sha_256_ctx_8_add_8,
        Bitcoin::Sha256Ctx8Add16 => simplicity_sys::c_jets::jets_wrapper::sha_256_ctx_8_add_16,
        Bitcoin::Sha256Ctx8Add32 => simplicity_sys::c_jets::jets_wrapper::sha_256_ctx_8_add_32,
        Bitcoin::Sha256Ctx8Add64 => simplicity_sys::c_jets::jets_wrapper::sha_256_ctx_8_add_64,
        Bitcoin::Sha256Ctx8Add128 => simplicity_sys::c_jets::jets_wrapper::sha_256_ctx_8_add_128,
        Bitcoin::Sha256Ctx8Add256 => simplicity_sys::c_jets::jets_wrapper::sha_256_ctx_8_add_256,
        Bitcoin::Sha256Ctx8AddBuffer511 => simplicity_sys::c_jets::jets_wrapper::sha_256_ctx_8_add_buffer_511,
        Bitcoin::Sha256Ctx8Finalize => simplicity_sys::c_jets::jets_wrapper::sha_256_ctx_8_finalize,
        Bitcoin::Sha256Ctx8Init => simplicity_sys::c_jets::jets_wrapper::sha_256_ctx_8_init,
        Bitcoin::Sha256Iv => simplicity_sys::c_jets::jets_wrapper::sha_256_iv,
        Bitcoin::Sha256Block => simplicity_sys::c_jets::jets_wrapper::sha_256_block,
        Bitcoin::Verify => simplicity_sys::c_jets::jets_wrapper::verify,
        // bip_0340_verify ignores its environment (the C implementation lives
        // in the env-free jets-secp256k1.c), so it can use the shared wrapper
        // that passes a null environment, exactly like the elements backend.
        Bitcoin::Bip0340Verify => simplicity_sys::c_jets::jets_wrapper::bip_0340_verify,
        // Bitcoin jets whose C implementations read the transaction
        // environment (`const txEnv* env`). The wrappers in the bitcoin c_env
        // module forward the Rust `CTxEnv` reference to the C functions.
        Bitcoin::SigAllHash => c_bitcoin::sig_all_hash,
        Bitcoin::CheckLockHeight => c_bitcoin::check_lock_height,
        Bitcoin::CheckLockDistance => c_bitcoin::check_lock_distance,
        Bitcoin::CheckLockDuration => c_bitcoin::check_lock_duration,
        Bitcoin::CurrentIndex => c_bitcoin::current_index,
        Bitcoin::NumInputs => c_bitcoin::num_inputs,
        Bitcoin::TxHash => c_bitcoin::tx_hash,
        Bitcoin::TapEnvHash => c_bitcoin::tap_env_hash,
        // Pure jets (no environment needed)
        Bitcoin::Add16 => simplicity_sys::c_jets::jets_wrapper::add_16,
        Bitcoin::Add32 => simplicity_sys::c_jets::jets_wrapper::add_32,
        Bitcoin::Add64 => simplicity_sys::c_jets::jets_wrapper::add_64,
        Bitcoin::Add8 => simplicity_sys::c_jets::jets_wrapper::add_8,
        Bitcoin::All16 => simplicity_sys::c_jets::jets_wrapper::all_16,
        Bitcoin::All32 => simplicity_sys::c_jets::jets_wrapper::all_32,
        Bitcoin::All64 => simplicity_sys::c_jets::jets_wrapper::all_64,
        Bitcoin::And1 => simplicity_sys::c_jets::jets_wrapper::and_1,
        Bitcoin::And16 => simplicity_sys::c_jets::jets_wrapper::and_16,
        Bitcoin::And32 => simplicity_sys::c_jets::jets_wrapper::and_32,
        Bitcoin::And64 => simplicity_sys::c_jets::jets_wrapper::and_64,
        Bitcoin::And8 => simplicity_sys::c_jets::jets_wrapper::and_8,
        Bitcoin::Ch1 => simplicity_sys::c_jets::jets_wrapper::ch_1,
        Bitcoin::Ch16 => simplicity_sys::c_jets::jets_wrapper::ch_16,
        Bitcoin::Ch32 => simplicity_sys::c_jets::jets_wrapper::ch_32,
        Bitcoin::Ch64 => simplicity_sys::c_jets::jets_wrapper::ch_64,
        Bitcoin::Ch8 => simplicity_sys::c_jets::jets_wrapper::ch_8,
        Bitcoin::CheckSigVerify => simplicity_sys::c_jets::jets_wrapper::check_sig_verify,
        Bitcoin::Complement1 => simplicity_sys::c_jets::jets_wrapper::complement_1,
        Bitcoin::Complement16 => simplicity_sys::c_jets::jets_wrapper::complement_16,
        Bitcoin::Complement32 => simplicity_sys::c_jets::jets_wrapper::complement_32,
        Bitcoin::Complement64 => simplicity_sys::c_jets::jets_wrapper::complement_64,
        Bitcoin::Complement8 => simplicity_sys::c_jets::jets_wrapper::complement_8,
        Bitcoin::Decompress => simplicity_sys::c_jets::jets_wrapper::decompress,
        Bitcoin::Decrement16 => simplicity_sys::c_jets::jets_wrapper::decrement_16,
        Bitcoin::Decrement32 => simplicity_sys::c_jets::jets_wrapper::decrement_32,
        Bitcoin::Decrement64 => simplicity_sys::c_jets::jets_wrapper::decrement_64,
        Bitcoin::Decrement8 => simplicity_sys::c_jets::jets_wrapper::decrement_8,
        Bitcoin::DivMod128_64 => simplicity_sys::c_jets::jets_wrapper::div_mod_128_64,
        Bitcoin::DivMod16 => simplicity_sys::c_jets::jets_wrapper::div_mod_16,
        Bitcoin::DivMod32 => simplicity_sys::c_jets::jets_wrapper::div_mod_32,
        Bitcoin::DivMod64 => simplicity_sys::c_jets::jets_wrapper::div_mod_64,
        Bitcoin::DivMod8 => simplicity_sys::c_jets::jets_wrapper::div_mod_8,
        Bitcoin::Divide16 => simplicity_sys::c_jets::jets_wrapper::divide_16,
        Bitcoin::Divide32 => simplicity_sys::c_jets::jets_wrapper::divide_32,
        Bitcoin::Divide64 => simplicity_sys::c_jets::jets_wrapper::divide_64,
        Bitcoin::Divide8 => simplicity_sys::c_jets::jets_wrapper::divide_8,
        Bitcoin::Divides16 => simplicity_sys::c_jets::jets_wrapper::divides_16,
        Bitcoin::Divides32 => simplicity_sys::c_jets::jets_wrapper::divides_32,
        Bitcoin::Divides64 => simplicity_sys::c_jets::jets_wrapper::divides_64,
        Bitcoin::Divides8 => simplicity_sys::c_jets::jets_wrapper::divides_8,
        Bitcoin::Eq1 => simplicity_sys::c_jets::jets_wrapper::eq_1,
        Bitcoin::Eq16 => simplicity_sys::c_jets::jets_wrapper::eq_16,
        Bitcoin::Eq32 => simplicity_sys::c_jets::jets_wrapper::eq_32,
        Bitcoin::Eq64 => simplicity_sys::c_jets::jets_wrapper::eq_64,
        Bitcoin::Eq8 => simplicity_sys::c_jets::jets_wrapper::eq_8,
        Bitcoin::FeAdd => simplicity_sys::c_jets::jets_wrapper::fe_add,
        Bitcoin::FeInvert => simplicity_sys::c_jets::jets_wrapper::fe_invert,
        Bitcoin::FeIsOdd => simplicity_sys::c_jets::jets_wrapper::fe_is_odd,
        Bitcoin::FeIsZero => simplicity_sys::c_jets::jets_wrapper::fe_is_zero,
        Bitcoin::FeMultiply => simplicity_sys::c_jets::jets_wrapper::fe_multiply,
        Bitcoin::FeMultiplyBeta => simplicity_sys::c_jets::jets_wrapper::fe_multiply_beta,
        Bitcoin::FeNegate => simplicity_sys::c_jets::jets_wrapper::fe_negate,
        Bitcoin::FeNormalize => simplicity_sys::c_jets::jets_wrapper::fe_normalize,
        Bitcoin::FeSquare => simplicity_sys::c_jets::jets_wrapper::fe_square,
        Bitcoin::FeSquareRoot => simplicity_sys::c_jets::jets_wrapper::fe_square_root,
        Bitcoin::FullAdd16 => simplicity_sys::c_jets::jets_wrapper::full_add_16,
        Bitcoin::FullAdd32 => simplicity_sys::c_jets::jets_wrapper::full_add_32,
        Bitcoin::FullAdd64 => simplicity_sys::c_jets::jets_wrapper::full_add_64,
        Bitcoin::FullAdd8 => simplicity_sys::c_jets::jets_wrapper::full_add_8,
        Bitcoin::FullDecrement16 => simplicity_sys::c_jets::jets_wrapper::full_decrement_16,
        Bitcoin::FullDecrement32 => simplicity_sys::c_jets::jets_wrapper::full_decrement_32,
        Bitcoin::FullDecrement64 => simplicity_sys::c_jets::jets_wrapper::full_decrement_64,
        Bitcoin::FullDecrement8 => simplicity_sys::c_jets::jets_wrapper::full_decrement_8,
        Bitcoin::FullIncrement16 => simplicity_sys::c_jets::jets_wrapper::full_increment_16,
        Bitcoin::FullIncrement32 => simplicity_sys::c_jets::jets_wrapper::full_increment_32,
        Bitcoin::FullIncrement64 => simplicity_sys::c_jets::jets_wrapper::full_increment_64,
        Bitcoin::FullIncrement8 => simplicity_sys::c_jets::jets_wrapper::full_increment_8,
        Bitcoin::FullLeftShift16_1 => simplicity_sys::c_jets::jets_wrapper::full_left_shift_16_1,
        Bitcoin::FullLeftShift16_2 => simplicity_sys::c_jets::jets_wrapper::full_left_shift_16_2,
        Bitcoin::FullLeftShift16_4 => simplicity_sys::c_jets::jets_wrapper::full_left_shift_16_4,
        Bitcoin::FullLeftShift16_8 => simplicity_sys::c_jets::jets_wrapper::full_left_shift_16_8,
        Bitcoin::FullLeftShift32_1 => simplicity_sys::c_jets::jets_wrapper::full_left_shift_32_1,
        Bitcoin::FullLeftShift32_16 => simplicity_sys::c_jets::jets_wrapper::full_left_shift_32_16,
        Bitcoin::FullLeftShift32_2 => simplicity_sys::c_jets::jets_wrapper::full_left_shift_32_2,
        Bitcoin::FullLeftShift32_4 => simplicity_sys::c_jets::jets_wrapper::full_left_shift_32_4,
        Bitcoin::FullLeftShift32_8 => simplicity_sys::c_jets::jets_wrapper::full_left_shift_32_8,
        Bitcoin::FullLeftShift64_1 => simplicity_sys::c_jets::jets_wrapper::full_left_shift_64_1,
        Bitcoin::FullLeftShift64_16 => simplicity_sys::c_jets::jets_wrapper::full_left_shift_64_16,
        Bitcoin::FullLeftShift64_2 => simplicity_sys::c_jets::jets_wrapper::full_left_shift_64_2,
        Bitcoin::FullLeftShift64_32 => simplicity_sys::c_jets::jets_wrapper::full_left_shift_64_32,
        Bitcoin::FullLeftShift64_4 => simplicity_sys::c_jets::jets_wrapper::full_left_shift_64_4,
        Bitcoin::FullLeftShift64_8 => simplicity_sys::c_jets::jets_wrapper::full_left_shift_64_8,
        Bitcoin::FullLeftShift8_1 => simplicity_sys::c_jets::jets_wrapper::full_left_shift_8_1,
        Bitcoin::FullLeftShift8_2 => simplicity_sys::c_jets::jets_wrapper::full_left_shift_8_2,
        Bitcoin::FullLeftShift8_4 => simplicity_sys::c_jets::jets_wrapper::full_left_shift_8_4,
        Bitcoin::FullMultiply16 => simplicity_sys::c_jets::jets_wrapper::full_multiply_16,
        Bitcoin::FullMultiply32 => simplicity_sys::c_jets::jets_wrapper::full_multiply_32,
        Bitcoin::FullMultiply64 => simplicity_sys::c_jets::jets_wrapper::full_multiply_64,
        Bitcoin::FullMultiply8 => simplicity_sys::c_jets::jets_wrapper::full_multiply_8,
        Bitcoin::FullRightShift16_1 => simplicity_sys::c_jets::jets_wrapper::full_right_shift_16_1,
        Bitcoin::FullRightShift16_2 => simplicity_sys::c_jets::jets_wrapper::full_right_shift_16_2,
        Bitcoin::FullRightShift16_4 => simplicity_sys::c_jets::jets_wrapper::full_right_shift_16_4,
        Bitcoin::FullRightShift16_8 => simplicity_sys::c_jets::jets_wrapper::full_right_shift_16_8,
        Bitcoin::FullRightShift32_1 => simplicity_sys::c_jets::jets_wrapper::full_right_shift_32_1,
        Bitcoin::FullRightShift32_16 => simplicity_sys::c_jets::jets_wrapper::full_right_shift_32_16,
        Bitcoin::FullRightShift32_2 => simplicity_sys::c_jets::jets_wrapper::full_right_shift_32_2,
        Bitcoin::FullRightShift32_4 => simplicity_sys::c_jets::jets_wrapper::full_right_shift_32_4,
        Bitcoin::FullRightShift32_8 => simplicity_sys::c_jets::jets_wrapper::full_right_shift_32_8,
        Bitcoin::FullRightShift64_1 => simplicity_sys::c_jets::jets_wrapper::full_right_shift_64_1,
        Bitcoin::FullRightShift64_16 => simplicity_sys::c_jets::jets_wrapper::full_right_shift_64_16,
        Bitcoin::FullRightShift64_2 => simplicity_sys::c_jets::jets_wrapper::full_right_shift_64_2,
        Bitcoin::FullRightShift64_32 => simplicity_sys::c_jets::jets_wrapper::full_right_shift_64_32,
        Bitcoin::FullRightShift64_4 => simplicity_sys::c_jets::jets_wrapper::full_right_shift_64_4,
        Bitcoin::FullRightShift64_8 => simplicity_sys::c_jets::jets_wrapper::full_right_shift_64_8,
        Bitcoin::FullRightShift8_1 => simplicity_sys::c_jets::jets_wrapper::full_right_shift_8_1,
        Bitcoin::FullRightShift8_2 => simplicity_sys::c_jets::jets_wrapper::full_right_shift_8_2,
        Bitcoin::FullRightShift8_4 => simplicity_sys::c_jets::jets_wrapper::full_right_shift_8_4,
        Bitcoin::FullSubtract16 => simplicity_sys::c_jets::jets_wrapper::full_subtract_16,
        Bitcoin::FullSubtract32 => simplicity_sys::c_jets::jets_wrapper::full_subtract_32,
        Bitcoin::FullSubtract64 => simplicity_sys::c_jets::jets_wrapper::full_subtract_64,
        Bitcoin::FullSubtract8 => simplicity_sys::c_jets::jets_wrapper::full_subtract_8,
        Bitcoin::GeIsOnCurve => simplicity_sys::c_jets::jets_wrapper::ge_is_on_curve,
        Bitcoin::GeNegate => simplicity_sys::c_jets::jets_wrapper::ge_negate,
        Bitcoin::GejAdd => simplicity_sys::c_jets::jets_wrapper::gej_add,
        Bitcoin::GejDouble => simplicity_sys::c_jets::jets_wrapper::gej_double,
        Bitcoin::GejEquiv => simplicity_sys::c_jets::jets_wrapper::gej_equiv,
        Bitcoin::GejGeAdd => simplicity_sys::c_jets::jets_wrapper::gej_ge_add,
        Bitcoin::GejGeAddEx => simplicity_sys::c_jets::jets_wrapper::gej_ge_add_ex,
        Bitcoin::GejGeEquiv => simplicity_sys::c_jets::jets_wrapper::gej_ge_equiv,
        Bitcoin::GejInfinity => simplicity_sys::c_jets::jets_wrapper::gej_infinity,
        Bitcoin::GejIsInfinity => simplicity_sys::c_jets::jets_wrapper::gej_is_infinity,
        Bitcoin::GejIsOnCurve => simplicity_sys::c_jets::jets_wrapper::gej_is_on_curve,
        Bitcoin::GejNegate => simplicity_sys::c_jets::jets_wrapper::gej_negate,
        Bitcoin::GejNormalize => simplicity_sys::c_jets::jets_wrapper::gej_normalize,
        Bitcoin::GejRescale => simplicity_sys::c_jets::jets_wrapper::gej_rescale,
        Bitcoin::GejXEquiv => simplicity_sys::c_jets::jets_wrapper::gej_x_equiv,
        Bitcoin::GejYIsOdd => simplicity_sys::c_jets::jets_wrapper::gej_y_is_odd,
        Bitcoin::Generate => simplicity_sys::c_jets::jets_wrapper::generate,
        Bitcoin::HashToCurve => simplicity_sys::c_jets::jets_wrapper::hash_to_curve,
        Bitcoin::High1 => simplicity_sys::c_jets::jets_wrapper::high_1,
        Bitcoin::High16 => simplicity_sys::c_jets::jets_wrapper::high_16,
        Bitcoin::High32 => simplicity_sys::c_jets::jets_wrapper::high_32,
        Bitcoin::High64 => simplicity_sys::c_jets::jets_wrapper::high_64,
        Bitcoin::High8 => simplicity_sys::c_jets::jets_wrapper::high_8,
        Bitcoin::Increment16 => simplicity_sys::c_jets::jets_wrapper::increment_16,
        Bitcoin::Increment32 => simplicity_sys::c_jets::jets_wrapper::increment_32,
        Bitcoin::Increment64 => simplicity_sys::c_jets::jets_wrapper::increment_64,
        Bitcoin::Increment8 => simplicity_sys::c_jets::jets_wrapper::increment_8,
        Bitcoin::IsOne16 => simplicity_sys::c_jets::jets_wrapper::is_one_16,
        Bitcoin::IsOne32 => simplicity_sys::c_jets::jets_wrapper::is_one_32,
        Bitcoin::IsOne64 => simplicity_sys::c_jets::jets_wrapper::is_one_64,
        Bitcoin::IsOne8 => simplicity_sys::c_jets::jets_wrapper::is_one_8,
        Bitcoin::IsZero16 => simplicity_sys::c_jets::jets_wrapper::is_zero_16,
        Bitcoin::IsZero32 => simplicity_sys::c_jets::jets_wrapper::is_zero_32,
        Bitcoin::IsZero64 => simplicity_sys::c_jets::jets_wrapper::is_zero_64,
        Bitcoin::IsZero8 => simplicity_sys::c_jets::jets_wrapper::is_zero_8,
        Bitcoin::Le16 => simplicity_sys::c_jets::jets_wrapper::le_16,
        Bitcoin::Le32 => simplicity_sys::c_jets::jets_wrapper::le_32,
        Bitcoin::Le64 => simplicity_sys::c_jets::jets_wrapper::le_64,
        Bitcoin::Le8 => simplicity_sys::c_jets::jets_wrapper::le_8,
        Bitcoin::LeftExtend16_32 => simplicity_sys::c_jets::jets_wrapper::left_extend_16_32,
        Bitcoin::LeftExtend16_64 => simplicity_sys::c_jets::jets_wrapper::left_extend_16_64,
        Bitcoin::LeftExtend1_16 => simplicity_sys::c_jets::jets_wrapper::left_extend_1_16,
        Bitcoin::LeftExtend1_32 => simplicity_sys::c_jets::jets_wrapper::left_extend_1_32,
        Bitcoin::LeftExtend1_64 => simplicity_sys::c_jets::jets_wrapper::left_extend_1_64,
        Bitcoin::LeftExtend1_8 => simplicity_sys::c_jets::jets_wrapper::left_extend_1_8,
        Bitcoin::LeftExtend32_64 => simplicity_sys::c_jets::jets_wrapper::left_extend_32_64,
        Bitcoin::LeftExtend8_16 => simplicity_sys::c_jets::jets_wrapper::left_extend_8_16,
        Bitcoin::LeftExtend8_32 => simplicity_sys::c_jets::jets_wrapper::left_extend_8_32,
        Bitcoin::LeftExtend8_64 => simplicity_sys::c_jets::jets_wrapper::left_extend_8_64,
        Bitcoin::LeftPadHigh16_32 => simplicity_sys::c_jets::jets_wrapper::left_pad_high_16_32,
        Bitcoin::LeftPadHigh16_64 => simplicity_sys::c_jets::jets_wrapper::left_pad_high_16_64,
        Bitcoin::LeftPadHigh1_16 => simplicity_sys::c_jets::jets_wrapper::left_pad_high_1_16,
        Bitcoin::LeftPadHigh1_32 => simplicity_sys::c_jets::jets_wrapper::left_pad_high_1_32,
        Bitcoin::LeftPadHigh1_64 => simplicity_sys::c_jets::jets_wrapper::left_pad_high_1_64,
        Bitcoin::LeftPadHigh1_8 => simplicity_sys::c_jets::jets_wrapper::left_pad_high_1_8,
        Bitcoin::LeftPadHigh32_64 => simplicity_sys::c_jets::jets_wrapper::left_pad_high_32_64,
        Bitcoin::LeftPadHigh8_16 => simplicity_sys::c_jets::jets_wrapper::left_pad_high_8_16,
        Bitcoin::LeftPadHigh8_32 => simplicity_sys::c_jets::jets_wrapper::left_pad_high_8_32,
        Bitcoin::LeftPadHigh8_64 => simplicity_sys::c_jets::jets_wrapper::left_pad_high_8_64,
        Bitcoin::LeftPadLow16_32 => simplicity_sys::c_jets::jets_wrapper::left_pad_low_16_32,
        Bitcoin::LeftPadLow16_64 => simplicity_sys::c_jets::jets_wrapper::left_pad_low_16_64,
        Bitcoin::LeftPadLow1_16 => simplicity_sys::c_jets::jets_wrapper::left_pad_low_1_16,
        Bitcoin::LeftPadLow1_32 => simplicity_sys::c_jets::jets_wrapper::left_pad_low_1_32,
        Bitcoin::LeftPadLow1_64 => simplicity_sys::c_jets::jets_wrapper::left_pad_low_1_64,
        Bitcoin::LeftPadLow1_8 => simplicity_sys::c_jets::jets_wrapper::left_pad_low_1_8,
        Bitcoin::LeftPadLow32_64 => simplicity_sys::c_jets::jets_wrapper::left_pad_low_32_64,
        Bitcoin::LeftPadLow8_16 => simplicity_sys::c_jets::jets_wrapper::left_pad_low_8_16,
        Bitcoin::LeftPadLow8_32 => simplicity_sys::c_jets::jets_wrapper::left_pad_low_8_32,
        Bitcoin::LeftPadLow8_64 => simplicity_sys::c_jets::jets_wrapper::left_pad_low_8_64,
        Bitcoin::LeftRotate16 => simplicity_sys::c_jets::jets_wrapper::left_rotate_16,
        Bitcoin::LeftRotate32 => simplicity_sys::c_jets::jets_wrapper::left_rotate_32,
        Bitcoin::LeftRotate64 => simplicity_sys::c_jets::jets_wrapper::left_rotate_64,
        Bitcoin::LeftRotate8 => simplicity_sys::c_jets::jets_wrapper::left_rotate_8,
        Bitcoin::LeftShift16 => simplicity_sys::c_jets::jets_wrapper::left_shift_16,
        Bitcoin::LeftShift32 => simplicity_sys::c_jets::jets_wrapper::left_shift_32,
        Bitcoin::LeftShift64 => simplicity_sys::c_jets::jets_wrapper::left_shift_64,
        Bitcoin::LeftShift8 => simplicity_sys::c_jets::jets_wrapper::left_shift_8,
        Bitcoin::LeftShiftWith16 => simplicity_sys::c_jets::jets_wrapper::left_shift_with_16,
        Bitcoin::LeftShiftWith32 => simplicity_sys::c_jets::jets_wrapper::left_shift_with_32,
        Bitcoin::LeftShiftWith64 => simplicity_sys::c_jets::jets_wrapper::left_shift_with_64,
        Bitcoin::LeftShiftWith8 => simplicity_sys::c_jets::jets_wrapper::left_shift_with_8,
        Bitcoin::Leftmost16_1 => simplicity_sys::c_jets::jets_wrapper::leftmost_16_1,
        Bitcoin::Leftmost16_2 => simplicity_sys::c_jets::jets_wrapper::leftmost_16_2,
        Bitcoin::Leftmost16_4 => simplicity_sys::c_jets::jets_wrapper::leftmost_16_4,
        Bitcoin::Leftmost16_8 => simplicity_sys::c_jets::jets_wrapper::leftmost_16_8,
        Bitcoin::Leftmost32_1 => simplicity_sys::c_jets::jets_wrapper::leftmost_32_1,
        Bitcoin::Leftmost32_16 => simplicity_sys::c_jets::jets_wrapper::leftmost_32_16,
        Bitcoin::Leftmost32_2 => simplicity_sys::c_jets::jets_wrapper::leftmost_32_2,
        Bitcoin::Leftmost32_4 => simplicity_sys::c_jets::jets_wrapper::leftmost_32_4,
        Bitcoin::Leftmost32_8 => simplicity_sys::c_jets::jets_wrapper::leftmost_32_8,
        Bitcoin::Leftmost64_1 => simplicity_sys::c_jets::jets_wrapper::leftmost_64_1,
        Bitcoin::Leftmost64_16 => simplicity_sys::c_jets::jets_wrapper::leftmost_64_16,
        Bitcoin::Leftmost64_2 => simplicity_sys::c_jets::jets_wrapper::leftmost_64_2,
        Bitcoin::Leftmost64_32 => simplicity_sys::c_jets::jets_wrapper::leftmost_64_32,
        Bitcoin::Leftmost64_4 => simplicity_sys::c_jets::jets_wrapper::leftmost_64_4,
        Bitcoin::Leftmost64_8 => simplicity_sys::c_jets::jets_wrapper::leftmost_64_8,
        Bitcoin::Leftmost8_1 => simplicity_sys::c_jets::jets_wrapper::leftmost_8_1,
        Bitcoin::Leftmost8_2 => simplicity_sys::c_jets::jets_wrapper::leftmost_8_2,
        Bitcoin::Leftmost8_4 => simplicity_sys::c_jets::jets_wrapper::leftmost_8_4,
        Bitcoin::LinearCombination1 => simplicity_sys::c_jets::jets_wrapper::linear_combination_1,
        Bitcoin::LinearVerify1 => simplicity_sys::c_jets::jets_wrapper::linear_verify_1,
        Bitcoin::Low1 => simplicity_sys::c_jets::jets_wrapper::low_1,
        Bitcoin::Low16 => simplicity_sys::c_jets::jets_wrapper::low_16,
        Bitcoin::Low32 => simplicity_sys::c_jets::jets_wrapper::low_32,
        Bitcoin::Low64 => simplicity_sys::c_jets::jets_wrapper::low_64,
        Bitcoin::Low8 => simplicity_sys::c_jets::jets_wrapper::low_8,
        Bitcoin::Lt16 => simplicity_sys::c_jets::jets_wrapper::lt_16,
        Bitcoin::Lt32 => simplicity_sys::c_jets::jets_wrapper::lt_32,
        Bitcoin::Lt64 => simplicity_sys::c_jets::jets_wrapper::lt_64,
        Bitcoin::Lt8 => simplicity_sys::c_jets::jets_wrapper::lt_8,
        Bitcoin::Maj1 => simplicity_sys::c_jets::jets_wrapper::maj_1,
        Bitcoin::Maj16 => simplicity_sys::c_jets::jets_wrapper::maj_16,
        Bitcoin::Maj32 => simplicity_sys::c_jets::jets_wrapper::maj_32,
        Bitcoin::Maj64 => simplicity_sys::c_jets::jets_wrapper::maj_64,
        Bitcoin::Maj8 => simplicity_sys::c_jets::jets_wrapper::maj_8,
        Bitcoin::Max16 => simplicity_sys::c_jets::jets_wrapper::max_16,
        Bitcoin::Max32 => simplicity_sys::c_jets::jets_wrapper::max_32,
        Bitcoin::Max64 => simplicity_sys::c_jets::jets_wrapper::max_64,
        Bitcoin::Max8 => simplicity_sys::c_jets::jets_wrapper::max_8,
        Bitcoin::Median16 => simplicity_sys::c_jets::jets_wrapper::median_16,
        Bitcoin::Median32 => simplicity_sys::c_jets::jets_wrapper::median_32,
        Bitcoin::Median64 => simplicity_sys::c_jets::jets_wrapper::median_64,
        Bitcoin::Median8 => simplicity_sys::c_jets::jets_wrapper::median_8,
        Bitcoin::Min16 => simplicity_sys::c_jets::jets_wrapper::min_16,
        Bitcoin::Min32 => simplicity_sys::c_jets::jets_wrapper::min_32,
        Bitcoin::Min64 => simplicity_sys::c_jets::jets_wrapper::min_64,
        Bitcoin::Min8 => simplicity_sys::c_jets::jets_wrapper::min_8,
        Bitcoin::Modulo16 => simplicity_sys::c_jets::jets_wrapper::modulo_16,
        Bitcoin::Modulo32 => simplicity_sys::c_jets::jets_wrapper::modulo_32,
        Bitcoin::Modulo64 => simplicity_sys::c_jets::jets_wrapper::modulo_64,
        Bitcoin::Modulo8 => simplicity_sys::c_jets::jets_wrapper::modulo_8,
        Bitcoin::Multiply16 => simplicity_sys::c_jets::jets_wrapper::multiply_16,
        Bitcoin::Multiply32 => simplicity_sys::c_jets::jets_wrapper::multiply_32,
        Bitcoin::Multiply64 => simplicity_sys::c_jets::jets_wrapper::multiply_64,
        Bitcoin::Multiply8 => simplicity_sys::c_jets::jets_wrapper::multiply_8,
        Bitcoin::Negate16 => simplicity_sys::c_jets::jets_wrapper::negate_16,
        Bitcoin::Negate32 => simplicity_sys::c_jets::jets_wrapper::negate_32,
        Bitcoin::Negate64 => simplicity_sys::c_jets::jets_wrapper::negate_64,
        Bitcoin::Negate8 => simplicity_sys::c_jets::jets_wrapper::negate_8,
        Bitcoin::One16 => simplicity_sys::c_jets::jets_wrapper::one_16,
        Bitcoin::One32 => simplicity_sys::c_jets::jets_wrapper::one_32,
        Bitcoin::One64 => simplicity_sys::c_jets::jets_wrapper::one_64,
        Bitcoin::One8 => simplicity_sys::c_jets::jets_wrapper::one_8,
        Bitcoin::Or1 => simplicity_sys::c_jets::jets_wrapper::or_1,
        Bitcoin::Or16 => simplicity_sys::c_jets::jets_wrapper::or_16,
        Bitcoin::Or32 => simplicity_sys::c_jets::jets_wrapper::or_32,
        Bitcoin::Or64 => simplicity_sys::c_jets::jets_wrapper::or_64,
        Bitcoin::Or8 => simplicity_sys::c_jets::jets_wrapper::or_8,
        Bitcoin::ParseLock => simplicity_sys::c_jets::jets_wrapper::parse_lock,
        Bitcoin::ParseSequence => simplicity_sys::c_jets::jets_wrapper::parse_sequence,
        Bitcoin::PointVerify1 => simplicity_sys::c_jets::jets_wrapper::point_verify_1,
        Bitcoin::RightExtend16_32 => simplicity_sys::c_jets::jets_wrapper::right_extend_16_32,
        Bitcoin::RightExtend16_64 => simplicity_sys::c_jets::jets_wrapper::right_extend_16_64,
        Bitcoin::RightExtend32_64 => simplicity_sys::c_jets::jets_wrapper::right_extend_32_64,
        Bitcoin::RightExtend8_16 => simplicity_sys::c_jets::jets_wrapper::right_extend_8_16,
        Bitcoin::RightExtend8_32 => simplicity_sys::c_jets::jets_wrapper::right_extend_8_32,
        Bitcoin::RightExtend8_64 => simplicity_sys::c_jets::jets_wrapper::right_extend_8_64,
        Bitcoin::RightPadHigh16_32 => simplicity_sys::c_jets::jets_wrapper::right_pad_high_16_32,
        Bitcoin::RightPadHigh16_64 => simplicity_sys::c_jets::jets_wrapper::right_pad_high_16_64,
        Bitcoin::RightPadHigh1_16 => simplicity_sys::c_jets::jets_wrapper::right_pad_high_1_16,
        Bitcoin::RightPadHigh1_32 => simplicity_sys::c_jets::jets_wrapper::right_pad_high_1_32,
        Bitcoin::RightPadHigh1_64 => simplicity_sys::c_jets::jets_wrapper::right_pad_high_1_64,
        Bitcoin::RightPadHigh1_8 => simplicity_sys::c_jets::jets_wrapper::right_pad_high_1_8,
        Bitcoin::RightPadHigh32_64 => simplicity_sys::c_jets::jets_wrapper::right_pad_high_32_64,
        Bitcoin::RightPadHigh8_16 => simplicity_sys::c_jets::jets_wrapper::right_pad_high_8_16,
        Bitcoin::RightPadHigh8_32 => simplicity_sys::c_jets::jets_wrapper::right_pad_high_8_32,
        Bitcoin::RightPadHigh8_64 => simplicity_sys::c_jets::jets_wrapper::right_pad_high_8_64,
        Bitcoin::RightPadLow16_32 => simplicity_sys::c_jets::jets_wrapper::right_pad_low_16_32,
        Bitcoin::RightPadLow16_64 => simplicity_sys::c_jets::jets_wrapper::right_pad_low_16_64,
        Bitcoin::RightPadLow1_16 => simplicity_sys::c_jets::jets_wrapper::right_pad_low_1_16,
        Bitcoin::RightPadLow1_32 => simplicity_sys::c_jets::jets_wrapper::right_pad_low_1_32,
        Bitcoin::RightPadLow1_64 => simplicity_sys::c_jets::jets_wrapper::right_pad_low_1_64,
        Bitcoin::RightPadLow1_8 => simplicity_sys::c_jets::jets_wrapper::right_pad_low_1_8,
        Bitcoin::RightPadLow32_64 => simplicity_sys::c_jets::jets_wrapper::right_pad_low_32_64,
        Bitcoin::RightPadLow8_16 => simplicity_sys::c_jets::jets_wrapper::right_pad_low_8_16,
        Bitcoin::RightPadLow8_32 => simplicity_sys::c_jets::jets_wrapper::right_pad_low_8_32,
        Bitcoin::RightPadLow8_64 => simplicity_sys::c_jets::jets_wrapper::right_pad_low_8_64,
        Bitcoin::RightRotate16 => simplicity_sys::c_jets::jets_wrapper::right_rotate_16,
        Bitcoin::RightRotate32 => simplicity_sys::c_jets::jets_wrapper::right_rotate_32,
        Bitcoin::RightRotate64 => simplicity_sys::c_jets::jets_wrapper::right_rotate_64,
        Bitcoin::RightRotate8 => simplicity_sys::c_jets::jets_wrapper::right_rotate_8,
        Bitcoin::RightShift16 => simplicity_sys::c_jets::jets_wrapper::right_shift_16,
        Bitcoin::RightShift32 => simplicity_sys::c_jets::jets_wrapper::right_shift_32,
        Bitcoin::RightShift64 => simplicity_sys::c_jets::jets_wrapper::right_shift_64,
        Bitcoin::RightShift8 => simplicity_sys::c_jets::jets_wrapper::right_shift_8,
        Bitcoin::RightShiftWith16 => simplicity_sys::c_jets::jets_wrapper::right_shift_with_16,
        Bitcoin::RightShiftWith32 => simplicity_sys::c_jets::jets_wrapper::right_shift_with_32,
        Bitcoin::RightShiftWith64 => simplicity_sys::c_jets::jets_wrapper::right_shift_with_64,
        Bitcoin::RightShiftWith8 => simplicity_sys::c_jets::jets_wrapper::right_shift_with_8,
        Bitcoin::Rightmost16_1 => simplicity_sys::c_jets::jets_wrapper::rightmost_16_1,
        Bitcoin::Rightmost16_2 => simplicity_sys::c_jets::jets_wrapper::rightmost_16_2,
        Bitcoin::Rightmost16_4 => simplicity_sys::c_jets::jets_wrapper::rightmost_16_4,
        Bitcoin::Rightmost16_8 => simplicity_sys::c_jets::jets_wrapper::rightmost_16_8,
        Bitcoin::Rightmost32_1 => simplicity_sys::c_jets::jets_wrapper::rightmost_32_1,
        Bitcoin::Rightmost32_16 => simplicity_sys::c_jets::jets_wrapper::rightmost_32_16,
        Bitcoin::Rightmost32_2 => simplicity_sys::c_jets::jets_wrapper::rightmost_32_2,
        Bitcoin::Rightmost32_4 => simplicity_sys::c_jets::jets_wrapper::rightmost_32_4,
        Bitcoin::Rightmost32_8 => simplicity_sys::c_jets::jets_wrapper::rightmost_32_8,
        Bitcoin::Rightmost64_1 => simplicity_sys::c_jets::jets_wrapper::rightmost_64_1,
        Bitcoin::Rightmost64_16 => simplicity_sys::c_jets::jets_wrapper::rightmost_64_16,
        Bitcoin::Rightmost64_2 => simplicity_sys::c_jets::jets_wrapper::rightmost_64_2,
        Bitcoin::Rightmost64_32 => simplicity_sys::c_jets::jets_wrapper::rightmost_64_32,
        Bitcoin::Rightmost64_4 => simplicity_sys::c_jets::jets_wrapper::rightmost_64_4,
        Bitcoin::Rightmost64_8 => simplicity_sys::c_jets::jets_wrapper::rightmost_64_8,
        Bitcoin::Rightmost8_1 => simplicity_sys::c_jets::jets_wrapper::rightmost_8_1,
        Bitcoin::Rightmost8_2 => simplicity_sys::c_jets::jets_wrapper::rightmost_8_2,
        Bitcoin::Rightmost8_4 => simplicity_sys::c_jets::jets_wrapper::rightmost_8_4,
        Bitcoin::ScalarAdd => simplicity_sys::c_jets::jets_wrapper::scalar_add,
        Bitcoin::ScalarInvert => simplicity_sys::c_jets::jets_wrapper::scalar_invert,
        Bitcoin::ScalarIsZero => simplicity_sys::c_jets::jets_wrapper::scalar_is_zero,
        Bitcoin::ScalarMultiply => simplicity_sys::c_jets::jets_wrapper::scalar_multiply,
        Bitcoin::ScalarMultiplyLambda => simplicity_sys::c_jets::jets_wrapper::scalar_multiply_lambda,
        Bitcoin::ScalarNegate => simplicity_sys::c_jets::jets_wrapper::scalar_negate,
        Bitcoin::ScalarNormalize => simplicity_sys::c_jets::jets_wrapper::scalar_normalize,
        Bitcoin::ScalarSquare => simplicity_sys::c_jets::jets_wrapper::scalar_square,
        Bitcoin::Scale => simplicity_sys::c_jets::jets_wrapper::scale,
        Bitcoin::Sha256Ctx8Add512 => simplicity_sys::c_jets::jets_wrapper::sha_256_ctx_8_add_512,
        Bitcoin::Some1 => simplicity_sys::c_jets::jets_wrapper::some_1,
        Bitcoin::Some16 => simplicity_sys::c_jets::jets_wrapper::some_16,
        Bitcoin::Some32 => simplicity_sys::c_jets::jets_wrapper::some_32,
        Bitcoin::Some64 => simplicity_sys::c_jets::jets_wrapper::some_64,
        Bitcoin::Some8 => simplicity_sys::c_jets::jets_wrapper::some_8,
        Bitcoin::Subtract16 => simplicity_sys::c_jets::jets_wrapper::subtract_16,
        Bitcoin::Subtract32 => simplicity_sys::c_jets::jets_wrapper::subtract_32,
        Bitcoin::Subtract64 => simplicity_sys::c_jets::jets_wrapper::subtract_64,
        Bitcoin::Subtract8 => simplicity_sys::c_jets::jets_wrapper::subtract_8,
        Bitcoin::Swu => simplicity_sys::c_jets::jets_wrapper::swu,
        Bitcoin::TapdataInit => simplicity_sys::c_jets::jets_wrapper::tapdata_init,
        Bitcoin::Xor1 => simplicity_sys::c_jets::jets_wrapper::xor_1,
        Bitcoin::Xor16 => simplicity_sys::c_jets::jets_wrapper::xor_16,
        Bitcoin::Xor32 => simplicity_sys::c_jets::jets_wrapper::xor_32,
        Bitcoin::Xor64 => simplicity_sys::c_jets::jets_wrapper::xor_64,
        Bitcoin::Xor8 => simplicity_sys::c_jets::jets_wrapper::xor_8,
        Bitcoin::XorXor1 => simplicity_sys::c_jets::jets_wrapper::xor_xor_1,
        Bitcoin::XorXor16 => simplicity_sys::c_jets::jets_wrapper::xor_xor_16,
        Bitcoin::XorXor32 => simplicity_sys::c_jets::jets_wrapper::xor_xor_32,
        Bitcoin::XorXor64 => simplicity_sys::c_jets::jets_wrapper::xor_xor_64,
        Bitcoin::XorXor8 => simplicity_sys::c_jets::jets_wrapper::xor_xor_8,
        // Remaining env jets
        Bitcoin::AnnexHash => c_bitcoin::annex_hash,
        Bitcoin::BuildTapbranch => c_bitcoin::build_tapbranch,
        Bitcoin::BuildTapleafSimplicity => c_bitcoin::build_tapleaf_simplicity,
        Bitcoin::BuildTaptweak => c_bitcoin::build_taptweak,
        Bitcoin::CheckLockTime => c_bitcoin::check_lock_time,
        Bitcoin::CurrentAnnexHash => c_bitcoin::current_annex_hash,
        Bitcoin::CurrentPrevOutpoint => c_bitcoin::current_prev_outpoint,
        Bitcoin::CurrentScriptHash => c_bitcoin::current_script_hash,
        Bitcoin::CurrentScriptSigHash => c_bitcoin::current_script_sig_hash,
        Bitcoin::CurrentSequence => c_bitcoin::current_sequence,
        Bitcoin::CurrentValue => c_bitcoin::current_value,
        Bitcoin::Fee => c_bitcoin::fee,
        Bitcoin::InputAnnexHash => c_bitcoin::input_annex_hash,
        Bitcoin::InputAnnexesHash => c_bitcoin::input_annexes_hash,
        Bitcoin::InputHash => c_bitcoin::input_hash,
        Bitcoin::InputOutpointsHash => c_bitcoin::input_outpoints_hash,
        Bitcoin::InputPrevOutpoint => c_bitcoin::input_prev_outpoint,
        Bitcoin::InputScriptHash => c_bitcoin::input_script_hash,
        Bitcoin::InputScriptSigHash => c_bitcoin::input_script_sig_hash,
        Bitcoin::InputScriptSigsHash => c_bitcoin::input_script_sigs_hash,
        Bitcoin::InputScriptsHash => c_bitcoin::input_scripts_hash,
        Bitcoin::InputSequence => c_bitcoin::input_sequence,
        Bitcoin::InputSequencesHash => c_bitcoin::input_sequences_hash,
        Bitcoin::InputUtxoHash => c_bitcoin::input_utxo_hash,
        Bitcoin::InputUtxosHash => c_bitcoin::input_utxos_hash,
        Bitcoin::InputValue => c_bitcoin::input_value,
        Bitcoin::InputValuesHash => c_bitcoin::input_values_hash,
        Bitcoin::InputsHash => c_bitcoin::inputs_hash,
        Bitcoin::InternalKey => c_bitcoin::internal_key,
        Bitcoin::LockTime => c_bitcoin::lock_time,
        Bitcoin::NumOutputs => c_bitcoin::num_outputs,
        Bitcoin::OutpointHash => c_bitcoin::outpoint_hash,
        Bitcoin::OutputHash => c_bitcoin::output_hash,
        Bitcoin::OutputScriptHash => c_bitcoin::output_script_hash,
        Bitcoin::OutputScriptsHash => c_bitcoin::output_scripts_hash,
        Bitcoin::OutputValue => c_bitcoin::output_value,
        Bitcoin::OutputValuesHash => c_bitcoin::output_values_hash,
        Bitcoin::OutputsHash => c_bitcoin::outputs_hash,
        Bitcoin::ScriptCMR => c_bitcoin::script_cmr,
        Bitcoin::TapleafHash => c_bitcoin::tapleaf_hash,
        Bitcoin::TapleafVersion => c_bitcoin::tapleaf_version,
        Bitcoin::Tappath => c_bitcoin::tappath,
        Bitcoin::TappathHash => c_bitcoin::tappath_hash,
        Bitcoin::TotalInputValue => c_bitcoin::total_input_value,
        Bitcoin::TotalOutputValue => c_bitcoin::total_output_value,
        Bitcoin::TransactionId => c_bitcoin::transaction_id,
        Bitcoin::TxIsFinal => c_bitcoin::tx_is_final,
        Bitcoin::TxLockDistance => c_bitcoin::tx_lock_distance,
        Bitcoin::TxLockDuration => c_bitcoin::tx_lock_duration,
        Bitcoin::TxLockHeight => c_bitcoin::tx_lock_height,
        Bitcoin::TxLockTime => c_bitcoin::tx_lock_time,
        Bitcoin::Version => c_bitcoin::version,
    }
}


#[cfg(test)]
mod bitcoin_cmr_tests {
    use super::*;
    use crate::merkle::cmr::Cmr;

    #[test]
    fn cmr_matches_spec() {
        assert_eq!(Bitcoin::Add16.cmr(), Cmr::from_byte_array([0x49, 0x42, 0x5a, 0x86, 0xe2, 0x0a, 0x67, 0x6d, 0x8b, 0x87, 0xe3, 0xc1, 0xa9, 0xb8, 0xea, 0x6e, 0xc7, 0x5d, 0x85, 0x9c, 0x12, 0xc5, 0x1b, 0xcb, 0x7f, 0xa9, 0xf9, 0x69, 0x12, 0xc3, 0x49, 0xcf]));
        assert_eq!(Bitcoin::Add32.cmr(), Cmr::from_byte_array([0x46, 0x68, 0xcd, 0x55, 0xe8, 0xd1, 0x59, 0x19, 0x53, 0x32, 0x70, 0x14, 0xec, 0x64, 0xc8, 0xe7, 0xd5, 0x2b, 0x86, 0xb5, 0x3e, 0x11, 0xc0, 0x14, 0x57, 0xea, 0xf2, 0xc3, 0xd3, 0xce, 0xbf, 0x9f]));
        assert_eq!(Bitcoin::Add64.cmr(), Cmr::from_byte_array([0xbe, 0x2b, 0x75, 0x19, 0x30, 0x3a, 0x67, 0xee, 0xa6, 0xb4, 0x82, 0x95, 0x0e, 0xda, 0x83, 0x43, 0x5e, 0x1d, 0xe8, 0x55, 0x9c, 0x39, 0x4a, 0x23, 0x62, 0x22, 0xff, 0x5b, 0xf0, 0x89, 0xd3, 0x46]));
        assert_eq!(Bitcoin::Add8.cmr(), Cmr::from_byte_array([0xdf, 0xa1, 0x79, 0xad, 0xf4, 0x55, 0x0b, 0x28, 0x48, 0x73, 0xbf, 0x30, 0x12, 0x3e, 0x0d, 0x4e, 0x54, 0x06, 0x9b, 0x08, 0x58, 0x34, 0xce, 0x56, 0x58, 0x15, 0xef, 0x7e, 0x45, 0x78, 0x4a, 0xcb]));
        assert_eq!(Bitcoin::All16.cmr(), Cmr::from_byte_array([0x24, 0xf4, 0x82, 0xa5, 0x13, 0xd3, 0x33, 0x62, 0x01, 0x5d, 0x28, 0xdf, 0x4b, 0xb6, 0xc3, 0xee, 0x08, 0xab, 0x8a, 0xfb, 0xbd, 0x25, 0x57, 0x1f, 0x0e, 0xa8, 0x9d, 0x8c, 0xab, 0xa3, 0x14, 0x04]));
        assert_eq!(Bitcoin::All32.cmr(), Cmr::from_byte_array([0xa7, 0x16, 0x52, 0x2d, 0x0f, 0x37, 0x87, 0xc8, 0xb4, 0xd5, 0x07, 0x64, 0x7f, 0x1f, 0x80, 0x7b, 0x67, 0xf3, 0x20, 0xd6, 0xeb, 0x67, 0xb8, 0x4b, 0x60, 0x9c, 0xec, 0x1d, 0x2f, 0x12, 0x21, 0x8a]));
        assert_eq!(Bitcoin::All64.cmr(), Cmr::from_byte_array([0x7a, 0xee, 0xfe, 0x2e, 0xce, 0x24, 0xba, 0xb3, 0x7c, 0x6e, 0x54, 0x30, 0xee, 0xd4, 0x19, 0xfc, 0xd5, 0xf0, 0x37, 0x91, 0x2d, 0x17, 0x70, 0xcb, 0x7d, 0x65, 0x20, 0xdc, 0xe5, 0x25, 0x29, 0x1a]));
        assert_eq!(Bitcoin::All8.cmr(), Cmr::from_byte_array([0x46, 0x37, 0xf4, 0x0e, 0x5f, 0x47, 0x26, 0xb0, 0x05, 0x70, 0x76, 0x5a, 0xc7, 0x94, 0xe2, 0x9e, 0xd1, 0xbb, 0x26, 0x55, 0xff, 0xc4, 0x12, 0xb2, 0xdc, 0x41, 0x25, 0x8e, 0x41, 0xaa, 0xc6, 0x24]));
        assert_eq!(Bitcoin::And1.cmr(), Cmr::from_byte_array([0x10, 0x68, 0x4d, 0x0d, 0xd7, 0x2c, 0xb0, 0xa8, 0x26, 0xa8, 0x63, 0x83, 0x4e, 0x01, 0x1f, 0x50, 0xfa, 0x0d, 0x55, 0x8b, 0xa7, 0x7d, 0x6b, 0x9f, 0x49, 0xa1, 0xac, 0x22, 0x90, 0x2a, 0x6a, 0xd0]));
        assert_eq!(Bitcoin::And16.cmr(), Cmr::from_byte_array([0x37, 0x3c, 0x73, 0x0f, 0xad, 0x3e, 0x88, 0x47, 0x99, 0x1a, 0xa4, 0x17, 0xd9, 0xf0, 0x80, 0xee, 0x1c, 0xb8, 0x8a, 0x7f, 0x72, 0x06, 0xf3, 0xfa, 0x84, 0x0b, 0x19, 0x50, 0x77, 0x61, 0xfb, 0x23]));
        assert_eq!(Bitcoin::And32.cmr(), Cmr::from_byte_array([0x13, 0xb0, 0x2c, 0x4c, 0x60, 0xae, 0x6e, 0xa4, 0x91, 0x16, 0x16, 0x49, 0xac, 0xf9, 0xa4, 0x7a, 0x70, 0x25, 0xaf, 0x84, 0x7d, 0x5f, 0x58, 0x1e, 0x6f, 0x1c, 0xcc, 0xfb, 0x21, 0xd3, 0x00, 0x1d]));
        assert_eq!(Bitcoin::And64.cmr(), Cmr::from_byte_array([0x92, 0x18, 0x55, 0x35, 0xd4, 0x50, 0x54, 0x07, 0xde, 0xa3, 0xc8, 0xa6, 0x08, 0x26, 0xed, 0xe6, 0x4a, 0x8f, 0xbb, 0x3d, 0xb4, 0x86, 0xd5, 0x6f, 0x64, 0x2d, 0x21, 0x7c, 0x29, 0xcb, 0xd7, 0x95]));
        assert_eq!(Bitcoin::And8.cmr(), Cmr::from_byte_array([0x26, 0x9a, 0x1b, 0x44, 0x62, 0x66, 0xf8, 0xf4, 0xa4, 0xa3, 0x8f, 0xa7, 0xe7, 0xe3, 0x91, 0x82, 0xf1, 0x52, 0x14, 0x36, 0x14, 0x2b, 0xad, 0xed, 0xf3, 0xaa, 0x63, 0xfb, 0x2f, 0x17, 0x2d, 0x2f]));
        assert_eq!(Bitcoin::AnnexHash.cmr(), Cmr::from_byte_array([0x51, 0xfa, 0x19, 0x13, 0xa6, 0x29, 0x73, 0x48, 0x4a, 0x70, 0x0a, 0xf3, 0xff, 0x93, 0x26, 0x94, 0xd3, 0x89, 0x0a, 0xe0, 0xad, 0x87, 0xa4, 0x55, 0xda, 0xf4, 0x90, 0x6f, 0x22, 0x4a, 0x48, 0xd5]));
        assert_eq!(Bitcoin::Bip0340Verify.cmr(), Cmr::from_byte_array([0x49, 0x15, 0x65, 0xfe, 0x23, 0xa7, 0xbd, 0xc1, 0x84, 0x2b, 0xe7, 0x49, 0x50, 0x93, 0x37, 0xf9, 0x68, 0x90, 0xd5, 0xb3, 0x58, 0xb3, 0x65, 0x20, 0x90, 0xda, 0x55, 0x66, 0x54, 0xe2, 0x95, 0x49]));
        assert_eq!(Bitcoin::BuildTapbranch.cmr(), Cmr::from_byte_array([0xef, 0x8e, 0x92, 0x91, 0x9b, 0x26, 0x08, 0xea, 0x6b, 0x5b, 0xcd, 0xd3, 0x9c, 0x51, 0x78, 0xe5, 0x46, 0x24, 0x95, 0x53, 0x91, 0x4b, 0x0f, 0x8d, 0x33, 0x73, 0x5d, 0x28, 0x86, 0xe1, 0xa5, 0xe8]));
        assert_eq!(Bitcoin::BuildTapleafSimplicity.cmr(), Cmr::from_byte_array([0x22, 0x41, 0x19, 0x09, 0x98, 0x41, 0x47, 0xde, 0x8f, 0x5a, 0x04, 0x28, 0x35, 0x8b, 0x47, 0x16, 0xdb, 0x08, 0x76, 0x64, 0xa7, 0x28, 0x56, 0x08, 0x52, 0xb0, 0xe6, 0x16, 0xeb, 0xc6, 0x2d, 0x30]));
        assert_eq!(Bitcoin::BuildTaptweak.cmr(), Cmr::from_byte_array([0xf1, 0x92, 0xdb, 0x17, 0x06, 0x60, 0x8d, 0xef, 0x16, 0x5d, 0xbd, 0xda, 0x72, 0xa3, 0x8c, 0x88, 0x82, 0xcb, 0x36, 0xc6, 0xda, 0x47, 0x07, 0x0d, 0x8a, 0x5f, 0x58, 0x96, 0xfb, 0xda, 0x84, 0x34]));
        assert_eq!(Bitcoin::CheckLockDistance.cmr(), Cmr::from_byte_array([0x38, 0xfd, 0xf7, 0xdd, 0x28, 0x53, 0x86, 0x70, 0xfb, 0x34, 0xc1, 0xbf, 0xe7, 0x2f, 0x17, 0xe2, 0xca, 0x57, 0x84, 0xf8, 0x7f, 0xed, 0x88, 0xea, 0xb5, 0x84, 0x79, 0x2b, 0x39, 0x74, 0xbd, 0x18]));
        assert_eq!(Bitcoin::CheckLockDuration.cmr(), Cmr::from_byte_array([0x77, 0x50, 0x38, 0x32, 0x6e, 0xae, 0x25, 0xc7, 0x20, 0x9b, 0x24, 0x43, 0x06, 0xea, 0xa9, 0xf9, 0x20, 0x4c, 0x7e, 0xce, 0x2d, 0xd6, 0x3b, 0x45, 0x2e, 0x10, 0x01, 0x7f, 0xa4, 0xea, 0x53, 0xcf]));
        assert_eq!(Bitcoin::CheckLockHeight.cmr(), Cmr::from_byte_array([0xb9, 0x0f, 0x15, 0x1f, 0x45, 0xb4, 0xeb, 0x37, 0x21, 0x06, 0x21, 0xf9, 0x70, 0x0a, 0x36, 0xc8, 0xc5, 0x04, 0xbe, 0xe0, 0x67, 0x77, 0x11, 0x63, 0xa8, 0xb8, 0x3a, 0x77, 0x18, 0xe6, 0x68, 0x6a]));
        assert_eq!(Bitcoin::CheckLockTime.cmr(), Cmr::from_byte_array([0xa4, 0x51, 0xeb, 0xb2, 0x25, 0xd6, 0xb1, 0x33, 0xa5, 0xe6, 0x35, 0x39, 0x78, 0x00, 0xd4, 0x87, 0xb3, 0x96, 0x8b, 0x0d, 0xe5, 0x5c, 0x96, 0xeb, 0x82, 0xf2, 0x90, 0xec, 0xff, 0x9c, 0x25, 0x90]));
        assert_eq!(Bitcoin::CheckSigVerify.cmr(), Cmr::from_byte_array([0xb5, 0x80, 0x15, 0x54, 0x6d, 0x28, 0x52, 0x66, 0x5d, 0xd2, 0x1b, 0xf1, 0x12, 0x66, 0x26, 0x70, 0x20, 0xfa, 0x5e, 0x27, 0x50, 0x01, 0xdd, 0x46, 0x18, 0xfa, 0x41, 0x56, 0x25, 0x95, 0x2e, 0x68]));
        assert_eq!(Bitcoin::Ch1.cmr(), Cmr::from_byte_array([0x73, 0xb2, 0xa9, 0x81, 0xd7, 0x21, 0x98, 0x6f, 0x8c, 0xde, 0xd6, 0x97, 0xe0, 0x63, 0x05, 0xd4, 0x58, 0x54, 0x10, 0x2d, 0xff, 0x20, 0xc0, 0xe5, 0xb9, 0x8a, 0xe1, 0x76, 0x23, 0x2f, 0xf2, 0x5b]));
        assert_eq!(Bitcoin::Ch16.cmr(), Cmr::from_byte_array([0x78, 0xde, 0x46, 0x5e, 0x61, 0xd9, 0xa5, 0x0f, 0x78, 0x25, 0x2f, 0xf4, 0xab, 0x23, 0xc9, 0xe6, 0x3a, 0xe6, 0x8c, 0x76, 0x9d, 0x36, 0x66, 0x12, 0x71, 0x20, 0x7d, 0xc6, 0x93, 0xf4, 0x69, 0xb4]));
        assert_eq!(Bitcoin::Ch32.cmr(), Cmr::from_byte_array([0xed, 0x93, 0xbe, 0xf1, 0xf6, 0x6e, 0x3a, 0x75, 0xe6, 0x12, 0x06, 0x02, 0xec, 0xee, 0x67, 0x40, 0x65, 0x3e, 0x7b, 0xd4, 0x6e, 0x07, 0xeb, 0x77, 0x14, 0x4e, 0xf1, 0xbb, 0x2c, 0x9d, 0xe5, 0x3d]));
        assert_eq!(Bitcoin::Ch64.cmr(), Cmr::from_byte_array([0xce, 0xd0, 0x07, 0x9b, 0x0b, 0xd1, 0xcc, 0x00, 0x20, 0x9a, 0x7c, 0xbc, 0x23, 0xf1, 0x3d, 0xfd, 0x20, 0x28, 0x08, 0xf0, 0xf5, 0x25, 0x7d, 0x8a, 0x50, 0xac, 0x54, 0x3e, 0x64, 0xee, 0x3a, 0x05]));
        assert_eq!(Bitcoin::Ch8.cmr(), Cmr::from_byte_array([0xc7, 0x07, 0xca, 0x72, 0x3e, 0x24, 0xf6, 0xb2, 0x5b, 0xf3, 0x94, 0xa9, 0x9a, 0x4d, 0x75, 0xe8, 0x13, 0x79, 0xb4, 0x67, 0x84, 0x38, 0xac, 0x78, 0x9d, 0xee, 0x18, 0x8e, 0xdc, 0xce, 0x75, 0xfa]));
        assert_eq!(Bitcoin::Complement1.cmr(), Cmr::from_byte_array([0x1b, 0xcf, 0xae, 0x13, 0xd5, 0xd2, 0x37, 0xa0, 0xbb, 0x9b, 0x1d, 0x75, 0x32, 0x04, 0x74, 0x62, 0xb2, 0x76, 0x90, 0xde, 0x5c, 0xac, 0x0e, 0x20, 0x19, 0x29, 0x89, 0x64, 0x57, 0x93, 0x40, 0x60]));
        assert_eq!(Bitcoin::Complement16.cmr(), Cmr::from_byte_array([0x81, 0xad, 0x4d, 0x2c, 0x3d, 0x16, 0xbf, 0x34, 0x0a, 0xf3, 0x88, 0x6d, 0x35, 0x5c, 0xc5, 0xbd, 0x1d, 0x59, 0x67, 0xe1, 0x6a, 0xce, 0x92, 0x4f, 0x19, 0xec, 0xf7, 0xd4, 0x86, 0xd6, 0xc7, 0xe9]));
        assert_eq!(Bitcoin::Complement32.cmr(), Cmr::from_byte_array([0x13, 0x74, 0x2c, 0x18, 0x04, 0xa9, 0x6e, 0x6c, 0x03, 0x95, 0x28, 0xbf, 0xd0, 0x3b, 0x8c, 0xf2, 0xb4, 0x62, 0x52, 0x6b, 0xb1, 0x81, 0xa3, 0xd8, 0xb4, 0x32, 0xf9, 0x9a, 0xc4, 0xf5, 0xa7, 0xef]));
        assert_eq!(Bitcoin::Complement64.cmr(), Cmr::from_byte_array([0x65, 0xb7, 0xbd, 0x09, 0x36, 0x39, 0xc5, 0x6d, 0xa2, 0x85, 0xce, 0xfa, 0x2d, 0x04, 0x64, 0x64, 0x5e, 0x14, 0xdd, 0x13, 0x64, 0x2f, 0x34, 0x95, 0x7d, 0x47, 0x37, 0xbd, 0x52, 0xfa, 0xc5, 0x88]));
        assert_eq!(Bitcoin::Complement8.cmr(), Cmr::from_byte_array([0x95, 0x4b, 0x70, 0xdc, 0xec, 0x53, 0x9e, 0x6b, 0x67, 0xdf, 0xfe, 0xc5, 0x3c, 0xf2, 0x4a, 0x66, 0x99, 0x39, 0x60, 0x8b, 0x22, 0x3f, 0x5f, 0x8b, 0x6d, 0x12, 0x9d, 0xaa, 0x48, 0xca, 0x1c, 0xf0]));
        assert_eq!(Bitcoin::CurrentAnnexHash.cmr(), Cmr::from_byte_array([0xce, 0xd9, 0x02, 0x2e, 0xdc, 0x69, 0x24, 0x1f, 0xe7, 0x07, 0x49, 0xa7, 0xf5, 0xd4, 0x89, 0xc3, 0x13, 0x5e, 0xe8, 0xc9, 0x6f, 0xe6, 0x4c, 0x44, 0x64, 0x40, 0x1f, 0x98, 0x51, 0xd7, 0xa1, 0x7d]));
        assert_eq!(Bitcoin::CurrentIndex.cmr(), Cmr::from_byte_array([0x0e, 0x8c, 0x96, 0x4c, 0x2f, 0x2b, 0x34, 0x90, 0x36, 0x2f, 0x3b, 0xbc, 0x74, 0x83, 0xde, 0xa3, 0x7f, 0xda, 0x81, 0x0b, 0x69, 0x31, 0x4f, 0xf6, 0x64, 0xfe, 0xa0, 0xe3, 0x27, 0x08, 0xec, 0x8f]));
        assert_eq!(Bitcoin::CurrentPrevOutpoint.cmr(), Cmr::from_byte_array([0x64, 0x43, 0x39, 0x1b, 0x34, 0x40, 0x86, 0x84, 0x6d, 0x5a, 0x17, 0xa6, 0x2e, 0x3e, 0x06, 0x28, 0x2a, 0xd6, 0x96, 0x2c, 0x4d, 0xfc, 0xed, 0x2f, 0x6a, 0x83, 0xd0, 0xdf, 0xbf, 0x6a, 0x5c, 0x54]));
        assert_eq!(Bitcoin::CurrentScriptHash.cmr(), Cmr::from_byte_array([0x23, 0x49, 0x8d, 0xd6, 0x64, 0x5e, 0xd1, 0x38, 0xb3, 0x44, 0x93, 0x7c, 0xf6, 0x54, 0xaa, 0xff, 0xa6, 0x27, 0xf8, 0x5a, 0x47, 0xca, 0xa6, 0x89, 0x54, 0xf1, 0x3f, 0x4c, 0x6a, 0x4d, 0xc7, 0x72]));
        assert_eq!(Bitcoin::CurrentScriptSigHash.cmr(), Cmr::from_byte_array([0x34, 0xa3, 0xc5, 0x5d, 0x14, 0x73, 0x74, 0xd8, 0xf3, 0xa1, 0x76, 0x1b, 0xba, 0xb2, 0x86, 0x96, 0x84, 0x55, 0x40, 0x4c, 0x2c, 0xa0, 0x3f, 0x69, 0x39, 0x3f, 0x63, 0x39, 0xd0, 0xf5, 0x8b, 0x59]));
        assert_eq!(Bitcoin::CurrentSequence.cmr(), Cmr::from_byte_array([0xc4, 0x9f, 0x76, 0xac, 0x79, 0xf8, 0xf1, 0x5c, 0x40, 0x9b, 0xa8, 0x15, 0xc1, 0x6e, 0xdc, 0xb8, 0xd1, 0x1e, 0x9a, 0x07, 0x56, 0x5c, 0x8e, 0x09, 0xb6, 0x3e, 0x7f, 0xdf, 0x31, 0x03, 0x00, 0x9f]));
        assert_eq!(Bitcoin::CurrentValue.cmr(), Cmr::from_byte_array([0x91, 0xb9, 0x6e, 0x82, 0x9e, 0x3b, 0x49, 0x72, 0xb0, 0xcb, 0x09, 0x1a, 0x0a, 0x90, 0x4b, 0xa4, 0x11, 0x33, 0x8a, 0xbf, 0xc0, 0x8d, 0xa7, 0x86, 0xd5, 0xa8, 0x4f, 0x04, 0x9b, 0x5b, 0xa3, 0xb8]));
        assert_eq!(Bitcoin::Decompress.cmr(), Cmr::from_byte_array([0x89, 0x00, 0x56, 0xdf, 0x82, 0x8a, 0x76, 0x6e, 0xe9, 0xf6, 0x56, 0x07, 0x22, 0x1e, 0x89, 0x46, 0xfa, 0x77, 0xc2, 0x56, 0xbb, 0x96, 0xe2, 0x31, 0xe1, 0x94, 0xd3, 0x00, 0x8c, 0xf3, 0x56, 0xf6]));
        assert_eq!(Bitcoin::Decrement16.cmr(), Cmr::from_byte_array([0x35, 0xfd, 0xa3, 0x8b, 0x67, 0x2c, 0x38, 0x31, 0xd8, 0xca, 0x11, 0xa4, 0xf3, 0xa9, 0x59, 0x62, 0x22, 0x52, 0x9e, 0xb1, 0xc1, 0x5f, 0x8c, 0x70, 0x50, 0x13, 0x97, 0x7d, 0x7d, 0xfb, 0x5d, 0x8b]));
        assert_eq!(Bitcoin::Decrement32.cmr(), Cmr::from_byte_array([0x3b, 0x2b, 0x19, 0x39, 0x55, 0x22, 0x84, 0xf6, 0x14, 0x69, 0x4b, 0xa1, 0x8d, 0xce, 0x70, 0xce, 0xe4, 0x76, 0xff, 0x42, 0xdc, 0xd0, 0x89, 0xe1, 0xa3, 0xc0, 0xa4, 0x2b, 0xeb, 0xd1, 0x08, 0xf6]));
        assert_eq!(Bitcoin::Decrement64.cmr(), Cmr::from_byte_array([0x7e, 0xf7, 0xbd, 0xd3, 0x5d, 0xb6, 0x85, 0xae, 0x99, 0x05, 0x53, 0x37, 0x35, 0xa2, 0xc7, 0xa7, 0xcc, 0xbc, 0x17, 0x08, 0xae, 0x63, 0x6f, 0x93, 0x1b, 0x5c, 0xe0, 0x26, 0xe5, 0xa1, 0x7f, 0xed]));
        assert_eq!(Bitcoin::Decrement8.cmr(), Cmr::from_byte_array([0xe3, 0x64, 0xf2, 0xe5, 0xc0, 0x8a, 0xe0, 0x11, 0x8e, 0xbe, 0x99, 0x3e, 0x8b, 0x3c, 0x95, 0x8c, 0x2b, 0xcc, 0x60, 0x62, 0xa3, 0x3b, 0xaa, 0xb9, 0x28, 0xc0, 0x4b, 0x3e, 0xc9, 0x32, 0xf5, 0x1b]));
        assert_eq!(Bitcoin::Divides16.cmr(), Cmr::from_byte_array([0x10, 0xbb, 0x18, 0x18, 0x0e, 0xab, 0x5b, 0xad, 0xdc, 0x16, 0x5d, 0x03, 0x37, 0xc4, 0xad, 0xa0, 0x88, 0xe1, 0x57, 0xb1, 0xaa, 0x67, 0x83, 0x34, 0x2a, 0x45, 0x20, 0xa3, 0x24, 0xdd, 0x9d, 0x2b]));
        assert_eq!(Bitcoin::Divides32.cmr(), Cmr::from_byte_array([0xf5, 0xe8, 0xe7, 0x8c, 0x82, 0x76, 0x9a, 0x48, 0xc9, 0x10, 0x3e, 0x44, 0xdd, 0xb4, 0x7f, 0x84, 0x1d, 0x76, 0x93, 0xb0, 0x41, 0x9e, 0x5e, 0x7d, 0xa4, 0xe6, 0x8b, 0x78, 0xb2, 0x37, 0xa5, 0x72]));
        assert_eq!(Bitcoin::Divides64.cmr(), Cmr::from_byte_array([0x9e, 0xbd, 0x55, 0xfa, 0xe4, 0x18, 0x88, 0x5e, 0xea, 0x04, 0xc3, 0xcd, 0xff, 0xf5, 0x31, 0xb7, 0xd7, 0x14, 0xd0, 0x59, 0x4f, 0xa7, 0xda, 0x87, 0xeb, 0x65, 0x55, 0xd3, 0x6b, 0x95, 0x3d, 0xb2]));
        assert_eq!(Bitcoin::Divides8.cmr(), Cmr::from_byte_array([0xa2, 0x36, 0xbc, 0x3e, 0x5c, 0xf4, 0xd2, 0x56, 0x40, 0x8b, 0xa3, 0x8c, 0x1e, 0xae, 0xe7, 0x36, 0x9a, 0x9c, 0x40, 0x2f, 0x74, 0xbc, 0xd1, 0xc8, 0x02, 0xf9, 0x09, 0x4f, 0xbf, 0x36, 0x80, 0x3d]));
        assert_eq!(Bitcoin::Divide16.cmr(), Cmr::from_byte_array([0x52, 0xab, 0xfe, 0xf1, 0x79, 0x75, 0x4c, 0x90, 0xf9, 0xa4, 0x26, 0x0f, 0x32, 0x3a, 0x8c, 0xa4, 0x95, 0x15, 0x92, 0x90, 0x2b, 0x8e, 0xcb, 0xd6, 0x4b, 0xa4, 0x26, 0x56, 0xfa, 0xc0, 0x59, 0x68]));
        assert_eq!(Bitcoin::Divide32.cmr(), Cmr::from_byte_array([0x4a, 0x8a, 0xe5, 0x35, 0x44, 0xe1, 0x47, 0xed, 0x02, 0x25, 0x04, 0x23, 0x79, 0x34, 0xcc, 0x25, 0x44, 0x79, 0xbc, 0xf9, 0x3d, 0xe1, 0xe1, 0x97, 0x4d, 0xda, 0xb3, 0xbb, 0x51, 0x6e, 0x60, 0x6c]));
        assert_eq!(Bitcoin::Divide64.cmr(), Cmr::from_byte_array([0xd7, 0x02, 0x5d, 0x05, 0xad, 0xfa, 0xe6, 0x6b, 0x47, 0x10, 0xd0, 0xff, 0x1e, 0x87, 0xe8, 0x28, 0x15, 0x57, 0x3e, 0x9c, 0xb6, 0x31, 0xb4, 0xc7, 0xd1, 0x3d, 0x2f, 0x1b, 0xe4, 0xdd, 0x26, 0xd2]));
        assert_eq!(Bitcoin::Divide8.cmr(), Cmr::from_byte_array([0x40, 0xcd, 0x1d, 0xac, 0xea, 0x24, 0x66, 0x9b, 0x6a, 0x58, 0x9b, 0x61, 0x47, 0x54, 0x74, 0xaf, 0x31, 0xd1, 0x4f, 0x8d, 0x46, 0x87, 0x70, 0x84, 0x52, 0xd3, 0xdf, 0x37, 0x30, 0x25, 0x31, 0x26]));
        assert_eq!(Bitcoin::DivMod128_64.cmr(), Cmr::from_byte_array([0x9a, 0x94, 0x43, 0xa2, 0xb5, 0x41, 0xe2, 0x9f, 0x27, 0x2f, 0xfd, 0x56, 0x7d, 0x1b, 0xf7, 0x42, 0xd6, 0x8c, 0xcb, 0xe9, 0x53, 0x8a, 0x87, 0x29, 0x1b, 0x0c, 0xa6, 0x38, 0x15, 0x63, 0xac, 0x2c]));
        assert_eq!(Bitcoin::DivMod16.cmr(), Cmr::from_byte_array([0x39, 0xbc, 0xb5, 0xc0, 0x1d, 0xc1, 0x80, 0x5c, 0x49, 0x19, 0x89, 0x5c, 0xb5, 0x9e, 0x8f, 0x3b, 0x41, 0x44, 0x67, 0x17, 0xf7, 0xff, 0x48, 0xfd, 0xc9, 0x37, 0xdd, 0x03, 0x80, 0x24, 0xa0, 0x8a]));
        assert_eq!(Bitcoin::DivMod32.cmr(), Cmr::from_byte_array([0xfb, 0x12, 0x02, 0xf4, 0xe8, 0x66, 0x3a, 0x87, 0xf5, 0x68, 0x99, 0x2a, 0x18, 0x50, 0x24, 0xc7, 0x0b, 0x4f, 0x07, 0x9f, 0xbe, 0x95, 0x30, 0x01, 0x0f, 0x6d, 0xb2, 0x84, 0x21, 0x8a, 0xf6, 0xcd]));
        assert_eq!(Bitcoin::DivMod64.cmr(), Cmr::from_byte_array([0x67, 0x64, 0xdf, 0x5e, 0x2a, 0xa0, 0x30, 0x32, 0x6e, 0xe5, 0x44, 0xc6, 0xe5, 0x3f, 0xf3, 0x8e, 0xf0, 0xb2, 0x85, 0x17, 0x91, 0x5e, 0xec, 0x65, 0xc7, 0x2e, 0xa5, 0x7a, 0x12, 0x98, 0x28, 0xeb]));
        assert_eq!(Bitcoin::DivMod8.cmr(), Cmr::from_byte_array([0xd3, 0x00, 0x24, 0x4e, 0x48, 0x0d, 0xd9, 0x74, 0x12, 0x13, 0xe4, 0xcb, 0x0e, 0xba, 0x83, 0x6d, 0x30, 0x59, 0xe7, 0x78, 0xb8, 0x12, 0x2f, 0x78, 0x90, 0x03, 0x26, 0x73, 0x73, 0x9c, 0x6a, 0x2c]));
        assert_eq!(Bitcoin::Eq1.cmr(), Cmr::from_byte_array([0x65, 0x49, 0xf9, 0x86, 0x20, 0x3a, 0x64, 0x97, 0x35, 0x6e, 0x43, 0x2b, 0x2a, 0xa1, 0x60, 0xd6, 0xee, 0x87, 0x0b, 0x11, 0x19, 0x08, 0x65, 0xbd, 0x36, 0xa4, 0x7c, 0xb0, 0x47, 0x04, 0x33, 0xa5]));
        assert_eq!(Bitcoin::Eq16.cmr(), Cmr::from_byte_array([0x0c, 0x54, 0x02, 0xb0, 0xad, 0xc8, 0xfc, 0x65, 0x70, 0x1b, 0xb7, 0x5b, 0x32, 0x54, 0xc8, 0x35, 0xf8, 0xfe, 0xc1, 0x30, 0x81, 0xcd, 0x35, 0xe1, 0x32, 0x8f, 0x2b, 0xd7, 0xdb, 0xd2, 0x3f, 0xa6]));
        assert_eq!(Bitcoin::Eq256.cmr(), Cmr::from_byte_array([0x26, 0x0e, 0x1d, 0x13, 0x6d, 0xd7, 0x44, 0xfc, 0xb0, 0x50, 0x7a, 0x2d, 0x27, 0x70, 0x27, 0xa7, 0x72, 0x43, 0x54, 0xeb, 0x17, 0x6b, 0x2f, 0xbf, 0x31, 0xc6, 0xc7, 0xd7, 0xfb, 0x3e, 0xcd, 0x6f]));
        assert_eq!(Bitcoin::Eq32.cmr(), Cmr::from_byte_array([0xf5, 0xd6, 0xed, 0xc8, 0xb6, 0x16, 0x4e, 0x12, 0x5b, 0xbb, 0xef, 0x08, 0xc9, 0xe0, 0x8a, 0x1e, 0x6f, 0xd4, 0x92, 0xf5, 0xbd, 0xca, 0x6f, 0xdc, 0x8b, 0x5f, 0x5a, 0x6f, 0x05, 0xc5, 0xab, 0x96]));
        assert_eq!(Bitcoin::Eq64.cmr(), Cmr::from_byte_array([0x1f, 0x93, 0xac, 0xb8, 0x09, 0x2f, 0xa0, 0x6d, 0xea, 0xf3, 0xc3, 0x87, 0xf5, 0x4a, 0x18, 0xff, 0xea, 0xa6, 0x9a, 0x47, 0xa6, 0xf5, 0xca, 0xf4, 0xae, 0x49, 0x7e, 0x5c, 0xc2, 0xb3, 0x6c, 0x43]));
        assert_eq!(Bitcoin::Eq8.cmr(), Cmr::from_byte_array([0xd7, 0x52, 0xfa, 0x7f, 0x51, 0x47, 0x30, 0x14, 0xeb, 0xb6, 0x9e, 0x1e, 0x1d, 0x2c, 0x86, 0xd5, 0x11, 0x48, 0xb6, 0xba, 0xa0, 0x21, 0x37, 0xa4, 0x8f, 0x62, 0xd5, 0x7e, 0xaf, 0x8d, 0xf1, 0xcd]));
        assert_eq!(Bitcoin::Fee.cmr(), Cmr::from_byte_array([0xfb, 0x9c, 0xa9, 0x39, 0x81, 0x67, 0x3d, 0x1d, 0x23, 0xae, 0xd2, 0x4d, 0x61, 0x2c, 0x1f, 0x5d, 0xc7, 0xcd, 0x49, 0xf8, 0x6d, 0x34, 0x8e, 0x67, 0xa1, 0x5b, 0xc4, 0xa7, 0x13, 0x0a, 0xe1, 0x85]));
        assert_eq!(Bitcoin::FeAdd.cmr(), Cmr::from_byte_array([0xa6, 0xc9, 0x0e, 0x02, 0xfd, 0xe4, 0xee, 0x6e, 0xef, 0x66, 0x67, 0x37, 0x49, 0x2e, 0x14, 0xaf, 0xc8, 0x76, 0x25, 0x04, 0x97, 0x4a, 0xf5, 0xd5, 0x47, 0x2b, 0xb9, 0x43, 0x3a, 0xd2, 0xd2, 0x94]));
        assert_eq!(Bitcoin::FeInvert.cmr(), Cmr::from_byte_array([0x7c, 0x4a, 0xba, 0xce, 0x33, 0xc7, 0x2b, 0x3b, 0xe1, 0xfd, 0x0e, 0xe3, 0x9f, 0xc6, 0xcb, 0x3e, 0xe5, 0xc8, 0xf1, 0x1e, 0xf2, 0x19, 0x98, 0xc0, 0x60, 0x2b, 0x52, 0x15, 0xaa, 0x2a, 0x75, 0xc2]));
        assert_eq!(Bitcoin::FeIsOdd.cmr(), Cmr::from_byte_array([0x30, 0xf5, 0x17, 0x1f, 0x58, 0xf1, 0x08, 0x9d, 0x5d, 0xcf, 0xb6, 0xe6, 0x68, 0x3f, 0x5a, 0xde, 0x98, 0x4c, 0x07, 0x99, 0x76, 0x3c, 0xa7, 0x38, 0x3f, 0x75, 0xdf, 0x1c, 0xa0, 0x81, 0x3e, 0xfe]));
        assert_eq!(Bitcoin::FeIsZero.cmr(), Cmr::from_byte_array([0xb0, 0xb7, 0x4d, 0x86, 0x51, 0xff, 0x55, 0x7c, 0xa9, 0x60, 0x44, 0xdd, 0x97, 0x28, 0x13, 0x38, 0xa8, 0xf7, 0xd3, 0xac, 0xb3, 0x84, 0x7d, 0x03, 0xac, 0xbf, 0x3d, 0x32, 0xd9, 0x6f, 0xae, 0x55]));
        assert_eq!(Bitcoin::FeMultiply.cmr(), Cmr::from_byte_array([0x50, 0x6b, 0x93, 0x19, 0xc1, 0x7a, 0x14, 0xa9, 0x46, 0x9d, 0x46, 0x27, 0x61, 0xa3, 0x30, 0x3a, 0xb4, 0x7d, 0xdb, 0x3a, 0x30, 0x79, 0xfb, 0xa3, 0x40, 0x73, 0xaa, 0x55, 0x42, 0x16, 0xa3, 0x88]));
        assert_eq!(Bitcoin::FeMultiplyBeta.cmr(), Cmr::from_byte_array([0x6e, 0x18, 0x0e, 0xea, 0xbe, 0x84, 0x22, 0xb7, 0x99, 0x68, 0xe7, 0x11, 0xdd, 0x00, 0xa4, 0xb6, 0x57, 0x8b, 0xb2, 0x75, 0xbe, 0xf4, 0x7f, 0xe5, 0xff, 0x96, 0x8f, 0x14, 0x72, 0xd7, 0x6f, 0x2a]));
        assert_eq!(Bitcoin::FeNegate.cmr(), Cmr::from_byte_array([0xd4, 0x37, 0xea, 0x00, 0x33, 0x98, 0x80, 0xb3, 0x83, 0xd8, 0x5f, 0xb2, 0xae, 0xaf, 0x20, 0x1b, 0xbe, 0x8f, 0xfc, 0x83, 0x70, 0x50, 0x62, 0xf9, 0xc9, 0x68, 0x59, 0x0d, 0x5d, 0xb3, 0x37, 0xf6]));
        assert_eq!(Bitcoin::FeNormalize.cmr(), Cmr::from_byte_array([0xec, 0x0c, 0x3d, 0xd9, 0xc5, 0x28, 0x63, 0x64, 0x78, 0xbe, 0xc0, 0xe1, 0x60, 0xe5, 0x0a, 0xd9, 0xbf, 0x45, 0x2c, 0x5b, 0x6f, 0x84, 0xe9, 0x40, 0xe1, 0x65, 0x84, 0xeb, 0x08, 0x5a, 0xce, 0x38]));
        assert_eq!(Bitcoin::FeSquare.cmr(), Cmr::from_byte_array([0xb9, 0x04, 0x77, 0x2d, 0x74, 0xa1, 0x85, 0xb8, 0x28, 0xeb, 0x15, 0x47, 0x28, 0xd2, 0x49, 0xc5, 0x08, 0x47, 0x11, 0xe9, 0xa1, 0x83, 0x2b, 0x89, 0xca, 0xf2, 0xaf, 0x59, 0xf9, 0x60, 0xe1, 0x18]));
        assert_eq!(Bitcoin::FeSquareRoot.cmr(), Cmr::from_byte_array([0x16, 0xfb, 0x9a, 0xce, 0xbe, 0x8b, 0x5b, 0x87, 0xf2, 0xea, 0x7d, 0xb6, 0xaa, 0x3a, 0x2a, 0xf8, 0x8c, 0xa2, 0xb5, 0x8f, 0x02, 0xcd, 0xc8, 0x7e, 0x7c, 0xe6, 0xbe, 0x0c, 0x1f, 0xfc, 0xe0, 0x14]));
        assert_eq!(Bitcoin::FullAdd16.cmr(), Cmr::from_byte_array([0xc5, 0x03, 0xb0, 0x78, 0xdd, 0xe3, 0x99, 0xc6, 0x3a, 0xc4, 0xa2, 0x32, 0xbd, 0x2a, 0x32, 0x9b, 0x04, 0x30, 0x8c, 0x75, 0xea, 0xec, 0x53, 0xa2, 0xf8, 0x89, 0xb8, 0xdf, 0x0d, 0x03, 0x34, 0x72]));
        assert_eq!(Bitcoin::FullAdd32.cmr(), Cmr::from_byte_array([0xa7, 0xaf, 0xd0, 0x40, 0xfc, 0xb0, 0xb2, 0xf2, 0x71, 0x90, 0x78, 0x1a, 0xe5, 0x3a, 0x6c, 0xca, 0x00, 0xe9, 0xfe, 0x59, 0x53, 0x11, 0x15, 0xc2, 0x58, 0xcc, 0xb6, 0x9d, 0x3b, 0xe5, 0xa2, 0x13]));
        assert_eq!(Bitcoin::FullAdd64.cmr(), Cmr::from_byte_array([0x80, 0xa3, 0xef, 0x6c, 0xdb, 0x84, 0xae, 0x7c, 0x8d, 0xbc, 0xf3, 0xa1, 0x84, 0x24, 0x84, 0xc0, 0x98, 0xdf, 0x6f, 0x19, 0x42, 0x7a, 0x5a, 0x4a, 0xdf, 0xe7, 0x6c, 0xd5, 0xff, 0x28, 0x36, 0xca]));
        assert_eq!(Bitcoin::FullAdd8.cmr(), Cmr::from_byte_array([0x4b, 0x90, 0x76, 0xb8, 0xc1, 0xad, 0x56, 0xc9, 0xdb, 0x6b, 0xb3, 0xba, 0xf5, 0x93, 0x89, 0x54, 0x46, 0xce, 0x61, 0xc7, 0x4f, 0x79, 0x7e, 0xb8, 0xb2, 0x30, 0xd2, 0x05, 0x42, 0x1c, 0x96, 0x17]));
        assert_eq!(Bitcoin::FullDecrement16.cmr(), Cmr::from_byte_array([0xfb, 0xa3, 0xc9, 0x78, 0x6e, 0xa3, 0x07, 0xf6, 0xd8, 0x54, 0x34, 0xfd, 0xa2, 0x56, 0x24, 0x82, 0x43, 0xa0, 0x0b, 0xac, 0x9a, 0x53, 0x53, 0xb6, 0x1e, 0xd3, 0x9c, 0x60, 0x55, 0xb6, 0x93, 0xb0]));
        assert_eq!(Bitcoin::FullDecrement32.cmr(), Cmr::from_byte_array([0x62, 0x3d, 0x21, 0xd0, 0x46, 0x79, 0x22, 0xc0, 0x01, 0xc5, 0x65, 0x68, 0x61, 0xd0, 0xdd, 0xb8, 0x60, 0xc0, 0xc9, 0xa8, 0x6b, 0xd4, 0xcf, 0xdc, 0x37, 0xa1, 0x4c, 0x14, 0x06, 0xe3, 0x44, 0x6e]));
        assert_eq!(Bitcoin::FullDecrement64.cmr(), Cmr::from_byte_array([0x14, 0x8b, 0x3e, 0xe1, 0xf7, 0x49, 0xea, 0x0b, 0xfb, 0xa7, 0x63, 0xbe, 0xe9, 0x99, 0xa2, 0x96, 0x77, 0x45, 0x6e, 0xae, 0x9e, 0xf5, 0x3a, 0xd8, 0x78, 0xf8, 0xb6, 0x14, 0x94, 0xf0, 0x8f, 0x00]));
        assert_eq!(Bitcoin::FullDecrement8.cmr(), Cmr::from_byte_array([0xb4, 0x1a, 0xfe, 0x97, 0x4e, 0xaa, 0x11, 0x82, 0xac, 0x46, 0x10, 0x52, 0x1e, 0x28, 0x27, 0x81, 0x31, 0x8c, 0xe2, 0x95, 0xa3, 0xf2, 0x3f, 0x0b, 0x87, 0x6a, 0xe2, 0x69, 0x67, 0x3f, 0xb1, 0xdf]));
        assert_eq!(Bitcoin::FullIncrement16.cmr(), Cmr::from_byte_array([0xa6, 0x8e, 0xcc, 0xdb, 0x9e, 0xad, 0x29, 0x26, 0xc3, 0xe4, 0x5b, 0x4b, 0xae, 0x43, 0x1c, 0xc4, 0x66, 0xd5, 0x8b, 0x8f, 0xac, 0xc9, 0x5a, 0x1b, 0x48, 0x44, 0xb9, 0x12, 0xdf, 0x56, 0x76, 0xdf]));
        assert_eq!(Bitcoin::FullIncrement32.cmr(), Cmr::from_byte_array([0xd0, 0xeb, 0x0e, 0x94, 0xa5, 0xc2, 0x57, 0x13, 0xeb, 0x94, 0x4c, 0xad, 0x4d, 0x70, 0x1c, 0x6a, 0x96, 0x88, 0x09, 0xbc, 0x1a, 0xf9, 0x03, 0xfd, 0xb1, 0x1e, 0x6f, 0xad, 0x0b, 0xb3, 0x1b, 0x10]));
        assert_eq!(Bitcoin::FullIncrement64.cmr(), Cmr::from_byte_array([0xc0, 0x03, 0xd2, 0xe9, 0xb0, 0xa5, 0x10, 0xc2, 0xdd, 0x78, 0x3e, 0x7d, 0x64, 0xeb, 0x87, 0xb3, 0x38, 0x55, 0xd3, 0x29, 0x90, 0xdf, 0xc2, 0x86, 0x26, 0x6e, 0x47, 0x8d, 0xa4, 0xe7, 0x47, 0x91]));
        assert_eq!(Bitcoin::FullIncrement8.cmr(), Cmr::from_byte_array([0x0b, 0xea, 0x24, 0x29, 0x18, 0xf2, 0xdd, 0x17, 0x64, 0x77, 0x78, 0x11, 0xe4, 0x44, 0x28, 0x63, 0x93, 0x52, 0x25, 0xb0, 0xf8, 0xb2, 0x39, 0xc2, 0x37, 0x52, 0xf9, 0xd8, 0x53, 0x92, 0xa1, 0x39]));
        assert_eq!(Bitcoin::FullLeftShift16_1.cmr(), Cmr::from_byte_array([0xb3, 0x66, 0xa8, 0x16, 0x92, 0x2f, 0xc4, 0x55, 0x01, 0x0f, 0xe8, 0x8a, 0x5f, 0x6a, 0x5c, 0xf2, 0xce, 0xa9, 0x17, 0xe1, 0x2b, 0xd1, 0x40, 0xae, 0x6d, 0x43, 0xb6, 0x41, 0xe5, 0x7f, 0x42, 0xb3]));
        assert_eq!(Bitcoin::FullLeftShift16_2.cmr(), Cmr::from_byte_array([0x27, 0x96, 0x0d, 0x0d, 0xf2, 0xfb, 0xbc, 0x38, 0x99, 0x3d, 0x86, 0xfb, 0x8f, 0x0c, 0xd2, 0xc3, 0x43, 0x4e, 0xdb, 0x11, 0x04, 0x82, 0x13, 0xc1, 0x41, 0x18, 0x93, 0xca, 0x99, 0x33, 0xb2, 0xee]));
        assert_eq!(Bitcoin::FullLeftShift16_4.cmr(), Cmr::from_byte_array([0x65, 0x51, 0x37, 0xbe, 0xc5, 0xc0, 0x36, 0x8f, 0x29, 0xbc, 0x99, 0x2c, 0x88, 0x42, 0x1a, 0x15, 0x98, 0x56, 0x40, 0x39, 0x7b, 0x61, 0x7f, 0xc4, 0x8d, 0x33, 0x21, 0x0f, 0xc0, 0x05, 0x3a, 0xd1]));
        assert_eq!(Bitcoin::FullLeftShift16_8.cmr(), Cmr::from_byte_array([0x16, 0x8f, 0x57, 0x6a, 0xa5, 0x6e, 0xa4, 0x7e, 0x07, 0x06, 0x46, 0xe7, 0x88, 0x96, 0xbe, 0xb2, 0x49, 0x8b, 0x1a, 0xe6, 0xb1, 0xff, 0x9c, 0x78, 0x62, 0x70, 0xe9, 0x55, 0x65, 0x84, 0x19, 0x29]));
        assert_eq!(Bitcoin::FullLeftShift32_1.cmr(), Cmr::from_byte_array([0xd7, 0xcd, 0x52, 0x24, 0x49, 0x11, 0x8e, 0x81, 0x00, 0xa7, 0x66, 0x2f, 0x4d, 0xf0, 0x39, 0xf8, 0xca, 0xeb, 0xf4, 0x33, 0xeb, 0x03, 0x9e, 0xdc, 0x42, 0xe8, 0x82, 0x37, 0x92, 0xcc, 0xea, 0x8a]));
        assert_eq!(Bitcoin::FullLeftShift32_16.cmr(), Cmr::from_byte_array([0x8b, 0xd8, 0x0d, 0x4d, 0x2f, 0x8b, 0x22, 0x46, 0xc1, 0x23, 0x15, 0xc4, 0x28, 0x41, 0xb4, 0xe4, 0x0a, 0x71, 0xae, 0x76, 0x96, 0x6a, 0x08, 0x95, 0x4d, 0x66, 0x6b, 0x86, 0x32, 0x86, 0x74, 0x37]));
        assert_eq!(Bitcoin::FullLeftShift32_2.cmr(), Cmr::from_byte_array([0x13, 0x06, 0x3d, 0x62, 0x93, 0x83, 0x29, 0x31, 0x1f, 0xb7, 0xda, 0xbb, 0x15, 0xc3, 0xfe, 0x58, 0xc2, 0x88, 0x76, 0x83, 0x00, 0x97, 0xec, 0xc6, 0xbf, 0xdd, 0x48, 0x0b, 0xe1, 0x98, 0x81, 0x46]));
        assert_eq!(Bitcoin::FullLeftShift32_4.cmr(), Cmr::from_byte_array([0x25, 0xa1, 0xb5, 0xdd, 0xe5, 0xdb, 0x28, 0x4e, 0x8a, 0x88, 0x21, 0x26, 0x7c, 0x26, 0x53, 0x01, 0x14, 0xbb, 0xe6, 0x71, 0xcf, 0xaf, 0xb4, 0x4a, 0x60, 0xd7, 0x50, 0x27, 0x67, 0xdb, 0x78, 0x2e]));
        assert_eq!(Bitcoin::FullLeftShift32_8.cmr(), Cmr::from_byte_array([0xce, 0x54, 0x70, 0xaf, 0xbf, 0xad, 0xbf, 0xba, 0x68, 0xf9, 0xb2, 0xd5, 0xb0, 0x64, 0x5a, 0x44, 0x08, 0xbe, 0x6f, 0x85, 0xa6, 0x9c, 0x7f, 0x09, 0xd0, 0x96, 0x45, 0x53, 0x68, 0x04, 0x87, 0x64]));
        assert_eq!(Bitcoin::FullLeftShift64_1.cmr(), Cmr::from_byte_array([0x05, 0x1f, 0x36, 0x05, 0x86, 0xc3, 0x79, 0xac, 0x2c, 0xe3, 0x99, 0xcb, 0xeb, 0x68, 0x7e, 0x77, 0x53, 0xb1, 0x5d, 0x73, 0x03, 0xdd, 0x31, 0x6c, 0xbd, 0x12, 0x30, 0x12, 0x08, 0x7c, 0xc6, 0x6f]));
        assert_eq!(Bitcoin::FullLeftShift64_16.cmr(), Cmr::from_byte_array([0xb2, 0x48, 0xbe, 0x4d, 0xfc, 0xb8, 0x8c, 0x5d, 0x89, 0xb1, 0xca, 0x61, 0x86, 0xa0, 0x41, 0xe9, 0x02, 0xb4, 0xc8, 0xa6, 0x2b, 0xb1, 0x6e, 0x09, 0xfe, 0x15, 0x61, 0x6e, 0x0e, 0x3a, 0xbd, 0x6d]));
        assert_eq!(Bitcoin::FullLeftShift64_2.cmr(), Cmr::from_byte_array([0x34, 0xbb, 0x51, 0x62, 0x6b, 0x1d, 0x6b, 0x89, 0x7a, 0xbc, 0x15, 0x5d, 0x03, 0x4f, 0xe0, 0x66, 0x3a, 0x0e, 0xc0, 0xfd, 0x8f, 0x64, 0x0e, 0x5f, 0xe1, 0xbf, 0x3c, 0xb7, 0x67, 0x0a, 0x29, 0x25]));
        assert_eq!(Bitcoin::FullLeftShift64_32.cmr(), Cmr::from_byte_array([0x9d, 0xac, 0x8c, 0xd7, 0xfd, 0x8b, 0x48, 0x88, 0x9e, 0x55, 0xc5, 0xaa, 0x12, 0xfe, 0x97, 0xb7, 0x29, 0xfe, 0xbc, 0x04, 0x1a, 0x9f, 0xff, 0x44, 0xc4, 0xd9, 0xb6, 0xf1, 0xe0, 0x7e, 0xb4, 0x42]));
        assert_eq!(Bitcoin::FullLeftShift64_4.cmr(), Cmr::from_byte_array([0x94, 0xb7, 0x3d, 0xad, 0xc3, 0xee, 0xeb, 0x2e, 0xe4, 0xa4, 0xd4, 0x44, 0xdd, 0x0f, 0x72, 0xac, 0x30, 0x62, 0x01, 0xf2, 0xff, 0xcf, 0x71, 0x4b, 0x8e, 0xbe, 0x79, 0x82, 0x74, 0x4c, 0x0c, 0x7e]));
        assert_eq!(Bitcoin::FullLeftShift64_8.cmr(), Cmr::from_byte_array([0x0e, 0xf7, 0x14, 0x75, 0x7e, 0xcf, 0x11, 0xca, 0x3c, 0x73, 0xce, 0x25, 0xef, 0x24, 0xee, 0x72, 0x95, 0xdd, 0x41, 0x71, 0xcc, 0x16, 0x28, 0x7f, 0xe6, 0x97, 0x1b, 0xd6, 0x7b, 0x47, 0x8b, 0x46]));
        assert_eq!(Bitcoin::FullLeftShift8_1.cmr(), Cmr::from_byte_array([0x9b, 0xfa, 0x48, 0xb7, 0xad, 0x51, 0x00, 0x91, 0xba, 0x60, 0x85, 0x44, 0x62, 0xd8, 0x59, 0xef, 0xd1, 0xa2, 0xaa, 0x18, 0x73, 0x1b, 0x5f, 0x0c, 0x9e, 0x2b, 0xa4, 0xd8, 0x9d, 0x3a, 0xa8, 0x43]));
        assert_eq!(Bitcoin::FullLeftShift8_2.cmr(), Cmr::from_byte_array([0x79, 0x7c, 0x20, 0x87, 0x01, 0xb2, 0xa4, 0xe1, 0x04, 0x9e, 0x83, 0xd6, 0x95, 0xf5, 0x54, 0xb9, 0x84, 0xf5, 0xdb, 0x28, 0x21, 0x52, 0x55, 0x58, 0x7c, 0x37, 0x34, 0x21, 0x51, 0xb7, 0x24, 0x1d]));
        assert_eq!(Bitcoin::FullLeftShift8_4.cmr(), Cmr::from_byte_array([0x37, 0xbd, 0xac, 0x91, 0x53, 0x8f, 0x22, 0x19, 0xcb, 0x89, 0xdf, 0x0e, 0xf9, 0xf1, 0x97, 0xcd, 0x68, 0x03, 0x1f, 0x27, 0x67, 0xe8, 0x94, 0xf0, 0x01, 0xc2, 0x6f, 0xff, 0x5e, 0xeb, 0x58, 0xcd]));
        assert_eq!(Bitcoin::FullMultiply16.cmr(), Cmr::from_byte_array([0x09, 0xba, 0xff, 0x92, 0x1e, 0x9f, 0x14, 0xd1, 0x20, 0x8d, 0x1d, 0xd8, 0x26, 0x4c, 0xf1, 0xf3, 0xb8, 0x54, 0xc9, 0xaf, 0x21, 0xf7, 0x78, 0xb2, 0xb5, 0x5a, 0x8a, 0x42, 0x6d, 0xfe, 0x89, 0x28]));
        assert_eq!(Bitcoin::FullMultiply32.cmr(), Cmr::from_byte_array([0x10, 0xc4, 0xb8, 0xc4, 0xc0, 0xac, 0xd9, 0x73, 0x90, 0xf8, 0x5c, 0xb3, 0xf5, 0xff, 0xe3, 0x6a, 0x29, 0x20, 0x37, 0xc1, 0x90, 0xee, 0xba, 0xb3, 0xe9, 0x89, 0x34, 0xfe, 0x93, 0xb2, 0xed, 0x90]));
        assert_eq!(Bitcoin::FullMultiply64.cmr(), Cmr::from_byte_array([0x2d, 0xb1, 0x9d, 0xba, 0x90, 0xef, 0x86, 0x7b, 0x5a, 0x3e, 0x91, 0x4b, 0x89, 0xfd, 0xa2, 0xda, 0x63, 0x7c, 0xa8, 0x0c, 0x42, 0x67, 0xe1, 0x98, 0x18, 0x37, 0xee, 0x3c, 0x6f, 0xe3, 0xda, 0xf5]));
        assert_eq!(Bitcoin::FullMultiply8.cmr(), Cmr::from_byte_array([0x7f, 0x46, 0xee, 0x72, 0x84, 0xf3, 0x9e, 0x73, 0x42, 0x75, 0xa2, 0x50, 0x9a, 0x0b, 0x73, 0x7e, 0xd9, 0x39, 0x11, 0x5f, 0x02, 0x19, 0xa5, 0x74, 0xd4, 0x69, 0xcd, 0x30, 0xb8, 0x19, 0xef, 0xe3]));
        assert_eq!(Bitcoin::FullRightShift16_1.cmr(), Cmr::from_byte_array([0x7e, 0xbe, 0x0c, 0x66, 0xc3, 0xc7, 0xdc, 0x16, 0xa5, 0x46, 0x9e, 0x91, 0x79, 0x09, 0x84, 0x17, 0xac, 0x5f, 0x20, 0xa3, 0x9c, 0xc4, 0x1a, 0xc3, 0x82, 0xfb, 0x1d, 0xbd, 0x98, 0xe8, 0xe3, 0x0f]));
        assert_eq!(Bitcoin::FullRightShift16_2.cmr(), Cmr::from_byte_array([0x8d, 0xb0, 0xc2, 0x16, 0x19, 0xc6, 0x2d, 0x63, 0xd4, 0xc2, 0x7b, 0xfc, 0xf6, 0x47, 0xd7, 0x09, 0xce, 0x37, 0xbe, 0xd0, 0x57, 0x18, 0xe9, 0x3e, 0x45, 0x15, 0xe2, 0x9e, 0xf3, 0x73, 0x0c, 0xf4]));
        assert_eq!(Bitcoin::FullRightShift16_4.cmr(), Cmr::from_byte_array([0x5c, 0x74, 0xb1, 0x32, 0x06, 0x31, 0x79, 0x17, 0xe0, 0x70, 0xe5, 0xfc, 0x1c, 0x82, 0xf4, 0xc5, 0xc2, 0xfb, 0xe9, 0xf3, 0x1b, 0x81, 0x29, 0x46, 0xba, 0x23, 0x0d, 0x8c, 0x94, 0xd4, 0x06, 0x16]));
        assert_eq!(Bitcoin::FullRightShift16_8.cmr(), Cmr::from_byte_array([0x11, 0x05, 0x81, 0x8a, 0xc9, 0x48, 0xd7, 0xbb, 0x63, 0x47, 0x07, 0xe6, 0x9d, 0xbf, 0x1f, 0x67, 0x90, 0x58, 0xa1, 0x3d, 0x35, 0xfa, 0xc2, 0xa6, 0x4d, 0xf9, 0x72, 0x62, 0xf2, 0x42, 0xb6, 0x3b]));
        assert_eq!(Bitcoin::FullRightShift32_1.cmr(), Cmr::from_byte_array([0x9b, 0x42, 0xc8, 0xf3, 0x3b, 0xc5, 0x75, 0x0e, 0x2a, 0x83, 0xaa, 0xdb, 0xf2, 0x9c, 0xc7, 0xfc, 0xb9, 0x50, 0xfe, 0x5a, 0x40, 0xaa, 0x0e, 0xc5, 0x24, 0x52, 0xe5, 0x33, 0xf8, 0x25, 0xa1, 0x15]));
        assert_eq!(Bitcoin::FullRightShift32_16.cmr(), Cmr::from_byte_array([0x0a, 0xe5, 0x65, 0x9c, 0x2f, 0xa7, 0x57, 0x94, 0x78, 0xeb, 0xd5, 0x7c, 0x4c, 0x98, 0xae, 0xe7, 0x77, 0x01, 0x56, 0x45, 0xb2, 0x84, 0x31, 0x81, 0x64, 0xfc, 0xbd, 0x30, 0x65, 0xfc, 0x87, 0x3c]));
        assert_eq!(Bitcoin::FullRightShift32_2.cmr(), Cmr::from_byte_array([0x57, 0xfb, 0x1c, 0x03, 0xc2, 0xeb, 0x17, 0xf6, 0x23, 0x47, 0x87, 0x34, 0xfd, 0x69, 0x37, 0xf9, 0xe3, 0xef, 0x02, 0x7c, 0x15, 0x60, 0x03, 0x8f, 0xa6, 0x06, 0x69, 0x05, 0x17, 0x89, 0xe3, 0x68]));
        assert_eq!(Bitcoin::FullRightShift32_4.cmr(), Cmr::from_byte_array([0x85, 0x82, 0xcd, 0xfa, 0x74, 0xef, 0x46, 0x6b, 0x81, 0x27, 0xb1, 0x97, 0x88, 0x13, 0x45, 0x93, 0x99, 0x8e, 0x49, 0x69, 0x00, 0xb3, 0x8f, 0x0f, 0x3d, 0x37, 0x58, 0x18, 0xd6, 0x73, 0x45, 0x1e]));
        assert_eq!(Bitcoin::FullRightShift32_8.cmr(), Cmr::from_byte_array([0xd9, 0x05, 0x93, 0x2e, 0xbf, 0xca, 0x2a, 0x38, 0x61, 0x9d, 0x80, 0x7e, 0x28, 0xff, 0x2e, 0x0d, 0x3b, 0xe0, 0x8a, 0x26, 0x06, 0x76, 0xd2, 0x57, 0xef, 0xa0, 0x40, 0xc3, 0x05, 0xaa, 0xdc, 0x33]));
        assert_eq!(Bitcoin::FullRightShift64_1.cmr(), Cmr::from_byte_array([0x3c, 0x15, 0x20, 0x9b, 0x99, 0xd2, 0x84, 0x5e, 0x22, 0x5e, 0x14, 0xe1, 0xe9, 0xe5, 0xe6, 0xa4, 0x87, 0x8b, 0xc8, 0xce, 0xa3, 0xf9, 0xf3, 0x6b, 0x8b, 0x53, 0x5a, 0xc6, 0x83, 0xe2, 0x9d, 0x00]));
        assert_eq!(Bitcoin::FullRightShift64_16.cmr(), Cmr::from_byte_array([0x02, 0x85, 0x25, 0x7b, 0x09, 0x0d, 0x8d, 0xa1, 0x28, 0xef, 0x64, 0xa8, 0x0c, 0x8d, 0x16, 0xfd, 0xc3, 0xbf, 0x5c, 0xe5, 0x0f, 0xcd, 0x56, 0xfe, 0xc5, 0xf9, 0x02, 0x55, 0xd9, 0xc8, 0xdf, 0x47]));
        assert_eq!(Bitcoin::FullRightShift64_2.cmr(), Cmr::from_byte_array([0x7e, 0xc2, 0xdd, 0x65, 0xc9, 0xe0, 0x13, 0xe3, 0xe4, 0xce, 0x90, 0xfb, 0xeb, 0x3f, 0xb1, 0xc7, 0x8c, 0xcc, 0x5d, 0x2a, 0x7d, 0x26, 0xd8, 0xaf, 0x77, 0xf9, 0x9d, 0xe8, 0x4c, 0xf7, 0x29, 0x73]));
        assert_eq!(Bitcoin::FullRightShift64_32.cmr(), Cmr::from_byte_array([0x35, 0x6f, 0x7d, 0xd4, 0x6b, 0xa3, 0x3f, 0x84, 0xb0, 0x66, 0x72, 0xfd, 0xe9, 0xa2, 0x97, 0x2e, 0x80, 0xf3, 0xea, 0x96, 0x5a, 0xe8, 0xbc, 0x0b, 0xff, 0x67, 0xaa, 0x2f, 0x69, 0xf1, 0x0b, 0x56]));
        assert_eq!(Bitcoin::FullRightShift64_4.cmr(), Cmr::from_byte_array([0x05, 0x46, 0x4a, 0x33, 0x35, 0xaf, 0xbb, 0x09, 0xd0, 0x46, 0x82, 0x8a, 0x92, 0x2c, 0x4d, 0xa0, 0xec, 0xee, 0xb1, 0x09, 0x77, 0xe4, 0x68, 0x01, 0xc9, 0x3c, 0xdd, 0x66, 0x8f, 0x22, 0xee, 0x63]));
        assert_eq!(Bitcoin::FullRightShift64_8.cmr(), Cmr::from_byte_array([0x70, 0x17, 0x2e, 0x1a, 0x69, 0x48, 0xbf, 0x40, 0x12, 0x0e, 0x68, 0xfb, 0x8b, 0x4b, 0x23, 0xbc, 0x35, 0x5a, 0x12, 0x00, 0x2c, 0xcc, 0x1d, 0xb6, 0x47, 0xc8, 0x9b, 0x12, 0xd1, 0x0e, 0xc5, 0x06]));
        assert_eq!(Bitcoin::FullRightShift8_1.cmr(), Cmr::from_byte_array([0x56, 0x69, 0xdb, 0xfc, 0xc6, 0x33, 0xec, 0x0b, 0xdf, 0x59, 0xe2, 0x2f, 0x03, 0xed, 0x4b, 0x64, 0x19, 0x20, 0x95, 0xf5, 0xdf, 0x20, 0xff, 0xc1, 0x2d, 0xd9, 0x0d, 0x7c, 0xda, 0x11, 0x37, 0x4f]));
        assert_eq!(Bitcoin::FullRightShift8_2.cmr(), Cmr::from_byte_array([0x1f, 0x94, 0x43, 0x61, 0x09, 0xdf, 0x52, 0xb3, 0x45, 0xfa, 0x3a, 0x89, 0xac, 0x2a, 0x49, 0xed, 0xc9, 0xd2, 0x85, 0xf2, 0x1f, 0x45, 0xed, 0x11, 0xd7, 0x75, 0xf7, 0xf7, 0xf3, 0x9d, 0x3e, 0x8f]));
        assert_eq!(Bitcoin::FullRightShift8_4.cmr(), Cmr::from_byte_array([0x71, 0x46, 0x98, 0xa2, 0x76, 0x84, 0xb5, 0xba, 0xa6, 0xb6, 0x48, 0x0e, 0xe3, 0xb2, 0x57, 0xcb, 0xb7, 0xcd, 0xab, 0x74, 0x72, 0xf3, 0x71, 0xa6, 0x27, 0x06, 0x18, 0xc0, 0xab, 0x12, 0x90, 0x8b]));
        assert_eq!(Bitcoin::FullSubtract16.cmr(), Cmr::from_byte_array([0x40, 0x09, 0x61, 0x52, 0xb5, 0x4e, 0x74, 0x25, 0x45, 0x55, 0xa6, 0x5d, 0xcc, 0xc6, 0x29, 0xdf, 0x57, 0xb9, 0x79, 0xc8, 0x47, 0x00, 0x54, 0x50, 0x36, 0xfe, 0x19, 0x0a, 0x6a, 0xf3, 0xd3, 0x8a]));
        assert_eq!(Bitcoin::FullSubtract32.cmr(), Cmr::from_byte_array([0xe7, 0x93, 0x0d, 0x64, 0x35, 0xa9, 0x68, 0x0b, 0xef, 0xb4, 0x9d, 0xb7, 0xd8, 0x7c, 0x2f, 0x50, 0xaf, 0xd4, 0x6d, 0x98, 0x88, 0x0d, 0xed, 0x50, 0xe5, 0x05, 0x5f, 0xa3, 0x09, 0xe1, 0xaf, 0xca]));
        assert_eq!(Bitcoin::FullSubtract64.cmr(), Cmr::from_byte_array([0xff, 0x28, 0x1d, 0xf8, 0xc4, 0x2a, 0x31, 0x59, 0xd9, 0xff, 0xa9, 0x25, 0x16, 0xca, 0x89, 0x3e, 0x23, 0xb0, 0xeb, 0x93, 0x8b, 0x4c, 0xb0, 0xb3, 0xf1, 0x34, 0x46, 0x8e, 0x9f, 0x4e, 0xbc, 0x46]));
        assert_eq!(Bitcoin::FullSubtract8.cmr(), Cmr::from_byte_array([0x7e, 0x3d, 0xcf, 0xe4, 0x56, 0xae, 0x3c, 0x5c, 0x87, 0xde, 0xbf, 0x04, 0x71, 0x89, 0xc2, 0x74, 0x82, 0xa4, 0xff, 0x4e, 0x8c, 0xfd, 0x1f, 0x17, 0x30, 0xc8, 0x7d, 0x2b, 0x7b, 0xff, 0x73, 0xba]));
        assert_eq!(Bitcoin::GejAdd.cmr(), Cmr::from_byte_array([0x45, 0xba, 0x7f, 0x3d, 0x1e, 0x1e, 0x6d, 0x34, 0x9f, 0xcf, 0x86, 0x98, 0x7b, 0x0e, 0x7f, 0x7a, 0xce, 0x66, 0x2e, 0x82, 0x20, 0x1d, 0x35, 0x02, 0x60, 0x45, 0x4e, 0x2f, 0xfe, 0xec, 0xb5, 0x4d]));
        assert_eq!(Bitcoin::GejDouble.cmr(), Cmr::from_byte_array([0x23, 0xe9, 0x78, 0xf3, 0x41, 0x54, 0x11, 0x9b, 0xde, 0xfc, 0x5d, 0x13, 0xfc, 0xfd, 0x0a, 0x34, 0xa7, 0x5e, 0x37, 0x26, 0xd6, 0xcb, 0x25, 0x81, 0x33, 0x70, 0xad, 0x7d, 0x9d, 0xe5, 0xe1, 0x33]));
        assert_eq!(Bitcoin::GejEquiv.cmr(), Cmr::from_byte_array([0xb9, 0x4b, 0x2a, 0xac, 0x73, 0xa6, 0x7f, 0x44, 0x95, 0x85, 0x99, 0x13, 0x4d, 0xe2, 0x30, 0x17, 0x9e, 0x9d, 0x6b, 0xb6, 0x47, 0xfd, 0x06, 0x11, 0x15, 0x8a, 0xab, 0xa7, 0x0b, 0x73, 0xe4, 0x00]));
        assert_eq!(Bitcoin::GejGeAdd.cmr(), Cmr::from_byte_array([0xf1, 0x16, 0x0b, 0x6f, 0x5e, 0xe2, 0xc5, 0x82, 0xe4, 0x95, 0x66, 0xe6, 0xc3, 0x86, 0xb3, 0x80, 0x94, 0xab, 0xc1, 0xa7, 0x18, 0x2d, 0x33, 0xa1, 0x50, 0x1f, 0xa2, 0xaa, 0xf0, 0x0a, 0xf3, 0xea]));
        assert_eq!(Bitcoin::GejGeAddEx.cmr(), Cmr::from_byte_array([0xc3, 0xd7, 0x34, 0x7f, 0xfe, 0x2d, 0x9c, 0x83, 0x9a, 0xac, 0x56, 0x7e, 0x29, 0x98, 0xe0, 0x16, 0xaf, 0x39, 0x4e, 0x2a, 0x19, 0x29, 0x31, 0x4b, 0x52, 0xe3, 0x1e, 0xed, 0x67, 0x8e, 0x30, 0xbf]));
        assert_eq!(Bitcoin::GejGeEquiv.cmr(), Cmr::from_byte_array([0x27, 0xc2, 0x99, 0x69, 0x13, 0x9f, 0x8d, 0x57, 0xed, 0xc9, 0x89, 0x5c, 0x30, 0x40, 0x3d, 0xf0, 0x15, 0xc5, 0x0c, 0xe7, 0x21, 0xc3, 0x81, 0xfb, 0x19, 0x7c, 0x0c, 0x04, 0x03, 0xf1, 0xdb, 0x0c]));
        assert_eq!(Bitcoin::GejInfinity.cmr(), Cmr::from_byte_array([0xaa, 0xfb, 0x93, 0x80, 0xd6, 0x1a, 0x7f, 0x14, 0x78, 0x46, 0x80, 0x6b, 0x2c, 0xc3, 0x74, 0xfb, 0xe8, 0x2d, 0xd1, 0xae, 0xd4, 0x85, 0xb9, 0x8a, 0x0f, 0x16, 0x4b, 0x3a, 0x54, 0xc2, 0xc0, 0xb0]));
        assert_eq!(Bitcoin::GejIsInfinity.cmr(), Cmr::from_byte_array([0xdb, 0x49, 0x5f, 0xd1, 0x31, 0x42, 0xe9, 0xb3, 0x37, 0x63, 0xfc, 0x6d, 0x48, 0xd2, 0xfb, 0x0e, 0x71, 0xb0, 0xd9, 0xd9, 0x9b, 0xd7, 0x26, 0xf4, 0x7a, 0xd1, 0x3f, 0xc5, 0x56, 0x06, 0x70, 0xa2]));
        assert_eq!(Bitcoin::GejIsOnCurve.cmr(), Cmr::from_byte_array([0xbf, 0x4c, 0xa1, 0x3f, 0xf2, 0x12, 0xe3, 0x4b, 0xf1, 0x7d, 0x90, 0xc1, 0x2e, 0x45, 0x3d, 0x08, 0xac, 0x7d, 0xaa, 0x4a, 0x47, 0xd5, 0x7e, 0x85, 0xb4, 0x3f, 0x2d, 0x43, 0x66, 0xd4, 0x3d, 0xda]));
        assert_eq!(Bitcoin::GejNegate.cmr(), Cmr::from_byte_array([0x01, 0xbd, 0x1a, 0x35, 0x1f, 0xb8, 0x16, 0x4c, 0x81, 0x3d, 0x91, 0x6d, 0x07, 0x77, 0x49, 0x99, 0x6b, 0x7d, 0xb1, 0x18, 0xd3, 0x15, 0x86, 0xca, 0x9d, 0x75, 0xe7, 0x56, 0x35, 0x18, 0xf4, 0x54]));
        assert_eq!(Bitcoin::GejNormalize.cmr(), Cmr::from_byte_array([0xec, 0x59, 0x7d, 0x17, 0xe2, 0xef, 0xb6, 0xd2, 0xa0, 0x02, 0xd5, 0x0e, 0x67, 0x75, 0x27, 0xd3, 0xd4, 0xa2, 0x90, 0x7a, 0x11, 0x9d, 0x68, 0xf1, 0x22, 0x84, 0xb9, 0xa1, 0xb0, 0xd2, 0x30, 0x3a]));
        assert_eq!(Bitcoin::GejRescale.cmr(), Cmr::from_byte_array([0x29, 0x77, 0xd9, 0x53, 0xef, 0x7a, 0x11, 0x56, 0xce, 0xc6, 0xdb, 0x2d, 0xc2, 0x92, 0x54, 0x12, 0x75, 0xcb, 0xc8, 0x2f, 0xb8, 0x29, 0xfd, 0x67, 0x1b, 0x97, 0x2e, 0x89, 0xeb, 0xed, 0x0c, 0x24]));
        assert_eq!(Bitcoin::GejXEquiv.cmr(), Cmr::from_byte_array([0xf9, 0xf1, 0x89, 0xfc, 0x00, 0xb6, 0x1f, 0x72, 0xf1, 0x0b, 0xaa, 0xa2, 0x1b, 0xcd, 0x88, 0xe5, 0xd2, 0x2e, 0x0a, 0xa9, 0xb7, 0x50, 0x9a, 0xe1, 0x62, 0xa1, 0x83, 0xa4, 0xb6, 0x64, 0xa4, 0xaf]));
        assert_eq!(Bitcoin::GejYIsOdd.cmr(), Cmr::from_byte_array([0x9e, 0xb6, 0xe4, 0x53, 0x5f, 0xb6, 0x9b, 0xf6, 0x09, 0x91, 0x65, 0x99, 0xf1, 0x34, 0x5a, 0xd7, 0x73, 0x5d, 0xa3, 0xf3, 0x94, 0x8d, 0x06, 0x86, 0x90, 0x8e, 0x44, 0xf4, 0x5b, 0x2f, 0xf6, 0x0c]));
        assert_eq!(Bitcoin::Generate.cmr(), Cmr::from_byte_array([0x14, 0x88, 0x85, 0xac, 0x73, 0x81, 0x31, 0x13, 0xc5, 0x23, 0xe8, 0x09, 0xbe, 0xa4, 0x7f, 0xfd, 0x8b, 0x1d, 0xaf, 0x37, 0x8d, 0x9d, 0xd5, 0x4b, 0xf9, 0x66, 0xcc, 0xb8, 0x83, 0xb1, 0xa9, 0x84]));
        assert_eq!(Bitcoin::GeIsOnCurve.cmr(), Cmr::from_byte_array([0x7d, 0x44, 0x87, 0x19, 0xf5, 0xf9, 0x57, 0x2b, 0xf5, 0x40, 0x2e, 0x12, 0xd1, 0x93, 0xaf, 0xf6, 0x77, 0x48, 0x2d, 0x66, 0xff, 0x3d, 0xcf, 0x27, 0x48, 0xf2, 0x5c, 0x6b, 0x73, 0x77, 0x02, 0x8c]));
        assert_eq!(Bitcoin::GeNegate.cmr(), Cmr::from_byte_array([0x3d, 0x2c, 0x8d, 0xe4, 0xc7, 0x01, 0x5f, 0xd3, 0x13, 0x26, 0x95, 0xfd, 0x66, 0xdf, 0xcf, 0x0f, 0x17, 0x78, 0xc7, 0x91, 0x85, 0x26, 0x8e, 0x9f, 0xae, 0x77, 0x89, 0xda, 0x53, 0x8e, 0xca, 0x59]));
        assert_eq!(Bitcoin::HashToCurve.cmr(), Cmr::from_byte_array([0xef, 0x4f, 0x54, 0x8b, 0x3c, 0x6c, 0x75, 0x17, 0x5f, 0x2c, 0xe2, 0xd1, 0x99, 0x3b, 0x2d, 0x19, 0x9b, 0xeb, 0x16, 0xc0, 0xa1, 0x40, 0x17, 0x5c, 0x48, 0xa1, 0x27, 0x7e, 0xfc, 0x43, 0xa9, 0x9b]));
        assert_eq!(Bitcoin::High1.cmr(), Cmr::from_byte_array([0xb1, 0x09, 0xcf, 0x1c, 0xce, 0x35, 0xf7, 0xe9, 0xb6, 0x49, 0x67, 0x1a, 0x9b, 0x45, 0xdb, 0xc2, 0x40, 0x99, 0xa7, 0x13, 0xae, 0xb9, 0xa8, 0x9c, 0xc4, 0xcf, 0x6e, 0xf6, 0xed, 0x8b, 0x30, 0x8b]));
        assert_eq!(Bitcoin::High16.cmr(), Cmr::from_byte_array([0x03, 0x5d, 0xad, 0xd9, 0xd7, 0xbf, 0x74, 0x33, 0x64, 0x45, 0xe7, 0x1d, 0xdc, 0x4d, 0x82, 0x02, 0x24, 0xff, 0x7e, 0x38, 0xe0, 0xb8, 0xd5, 0x2b, 0xec, 0x97, 0x29, 0xb5, 0x72, 0xb5, 0x31, 0xf9]));
        assert_eq!(Bitcoin::High32.cmr(), Cmr::from_byte_array([0xc5, 0xf1, 0xdf, 0x0d, 0x64, 0xa2, 0x73, 0x7a, 0x63, 0x1b, 0x3a, 0xae, 0x8f, 0x26, 0x0e, 0x8b, 0x8d, 0xc1, 0x95, 0x7b, 0xd0, 0x92, 0x91, 0x1b, 0x91, 0xd2, 0x07, 0x8a, 0xd2, 0x1e, 0x41, 0x8a]));
        assert_eq!(Bitcoin::High64.cmr(), Cmr::from_byte_array([0xa3, 0x12, 0x63, 0x3e, 0x0a, 0x23, 0x05, 0xe6, 0x9b, 0x3f, 0x34, 0x1d, 0x91, 0xd6, 0x83, 0xdd, 0x94, 0x19, 0x6a, 0x2f, 0x90, 0x05, 0xc9, 0xb1, 0x87, 0x2a, 0x2c, 0x15, 0xad, 0x46, 0xcf, 0x17]));
        assert_eq!(Bitcoin::High8.cmr(), Cmr::from_byte_array([0xcb, 0xd7, 0x8d, 0x50, 0xaf, 0x77, 0x99, 0x85, 0x5a, 0xdc, 0x49, 0x03, 0xdb, 0xbe, 0xfc, 0x13, 0x45, 0xd5, 0x14, 0x84, 0xf0, 0x3d, 0x3c, 0x75, 0x5c, 0xaa, 0xa5, 0xca, 0xa9, 0x7d, 0x4a, 0x14]));
        assert_eq!(Bitcoin::Increment16.cmr(), Cmr::from_byte_array([0x86, 0x77, 0x49, 0x49, 0x39, 0xb2, 0x7b, 0x86, 0xcb, 0x5a, 0x8c, 0x7f, 0x81, 0x72, 0xad, 0x55, 0x50, 0x95, 0x31, 0xc9, 0xb0, 0xe1, 0x1e, 0x99, 0x75, 0x7e, 0x29, 0x6c, 0xc3, 0xc7, 0xc1, 0x92]));
        assert_eq!(Bitcoin::Increment32.cmr(), Cmr::from_byte_array([0x6b, 0xdb, 0xab, 0x7c, 0xfc, 0x16, 0xc5, 0x03, 0x36, 0x3c, 0x2f, 0x07, 0x7e, 0x02, 0xc3, 0x35, 0xda, 0x40, 0x61, 0x75, 0xd1, 0x92, 0xfb, 0xef, 0x50, 0xc0, 0x7f, 0xc2, 0x79, 0xb3, 0xf4, 0x0c]));
        assert_eq!(Bitcoin::Increment64.cmr(), Cmr::from_byte_array([0x20, 0xe7, 0x5e, 0x71, 0x7c, 0xb7, 0x6d, 0x46, 0x95, 0x56, 0x4f, 0x7c, 0x20, 0x22, 0x1b, 0x7a, 0x01, 0x43, 0x13, 0x87, 0x38, 0xf1, 0x51, 0xaa, 0x19, 0x5e, 0xb1, 0x70, 0xec, 0x13, 0xc0, 0x49]));
        assert_eq!(Bitcoin::Increment8.cmr(), Cmr::from_byte_array([0x5f, 0x4e, 0x05, 0x6e, 0xf4, 0xed, 0x8d, 0x68, 0xbf, 0x91, 0x1f, 0xc5, 0xcb, 0x69, 0x03, 0x7e, 0xbf, 0x6c, 0x92, 0x21, 0x73, 0x43, 0xa8, 0x90, 0x5d, 0x38, 0xc4, 0x32, 0xc1, 0x83, 0x23, 0x3c]));
        assert_eq!(Bitcoin::InputsHash.cmr(), Cmr::from_byte_array([0xab, 0xbf, 0xe1, 0xc7, 0xd1, 0x15, 0xc4, 0x19, 0x1f, 0x50, 0x48, 0x39, 0xf9, 0x8c, 0x3f, 0x20, 0x42, 0x2b, 0x84, 0xe7, 0xfa, 0x14, 0xda, 0x14, 0x02, 0x89, 0x6c, 0x4d, 0x98, 0xbf, 0xa8, 0xd8]));
        assert_eq!(Bitcoin::InputAnnexesHash.cmr(), Cmr::from_byte_array([0x89, 0xb6, 0x02, 0x70, 0x44, 0x14, 0x1f, 0x20, 0x65, 0xb6, 0xf2, 0x36, 0xcf, 0xcc, 0x13, 0xb9, 0x68, 0x48, 0x5e, 0x00, 0x74, 0x6b, 0x78, 0x59, 0x28, 0x69, 0x03, 0xc6, 0x8c, 0x7f, 0x88, 0x0d]));
        assert_eq!(Bitcoin::InputAnnexHash.cmr(), Cmr::from_byte_array([0x94, 0x5b, 0x14, 0x7e, 0x5f, 0x0e, 0xdb, 0x83, 0x2d, 0x62, 0x34, 0x8c, 0xae, 0xea, 0xc2, 0x24, 0x56, 0xee, 0xe9, 0x44, 0x65, 0x37, 0x6d, 0xbf, 0x59, 0x6b, 0x9d, 0x62, 0x98, 0x5b, 0x01, 0xb6]));
        assert_eq!(Bitcoin::InputHash.cmr(), Cmr::from_byte_array([0x3a, 0xd2, 0x22, 0x31, 0xd6, 0xb9, 0xdf, 0xf6, 0xa0, 0xb4, 0xdb, 0xcf, 0xf0, 0x44, 0xe1, 0x1c, 0x08, 0x2e, 0x04, 0x68, 0x4e, 0x73, 0xce, 0x95, 0xc2, 0x05, 0xe0, 0xc4, 0x49, 0x6e, 0x5e, 0xca]));
        assert_eq!(Bitcoin::InputOutpointsHash.cmr(), Cmr::from_byte_array([0x17, 0x59, 0xc6, 0xf4, 0x70, 0xb9, 0xab, 0xa6, 0x2a, 0x31, 0x79, 0x1e, 0xa0, 0x10, 0x56, 0xe3, 0x60, 0x8b, 0xdf, 0x22, 0xf5, 0xdd, 0x43, 0xbf, 0x7d, 0xb0, 0x0e, 0xa5, 0x82, 0x3f, 0x7c, 0xce]));
        assert_eq!(Bitcoin::InputPrevOutpoint.cmr(), Cmr::from_byte_array([0x5d, 0x17, 0x12, 0x42, 0x09, 0xee, 0x05, 0x21, 0x24, 0xe5, 0x52, 0x38, 0xe0, 0xb6, 0xa6, 0xfe, 0x85, 0xa4, 0x86, 0x88, 0xc1, 0xe1, 0x5e, 0x61, 0x81, 0xde, 0x94, 0xf7, 0x8d, 0xb0, 0x40, 0x18]));
        assert_eq!(Bitcoin::InputScriptsHash.cmr(), Cmr::from_byte_array([0xf1, 0x77, 0x67, 0x1d, 0x60, 0x96, 0x0e, 0x46, 0x3a, 0x9a, 0x8f, 0x7e, 0x1f, 0x52, 0x13, 0xaa, 0xba, 0x91, 0x22, 0xd2, 0xdb, 0x75, 0xfe, 0x91, 0xf8, 0xf1, 0xcf, 0x91, 0xbe, 0x00, 0x19, 0x07]));
        assert_eq!(Bitcoin::InputScriptHash.cmr(), Cmr::from_byte_array([0x05, 0x62, 0x41, 0x2f, 0x02, 0x1d, 0xa4, 0x81, 0x9b, 0x01, 0x9c, 0x8c, 0xd8, 0x87, 0xd5, 0x30, 0x49, 0x2b, 0x9f, 0x94, 0x2c, 0x4e, 0xbf, 0x21, 0xae, 0x39, 0xed, 0x32, 0x33, 0x0d, 0x7c, 0xfe]));
        assert_eq!(Bitcoin::InputScriptSigsHash.cmr(), Cmr::from_byte_array([0xe7, 0x29, 0xc5, 0xf5, 0x8a, 0x93, 0x89, 0x08, 0x1a, 0x5c, 0xa8, 0x45, 0xf7, 0x6c, 0xb9, 0x80, 0xf0, 0x85, 0x99, 0xce, 0xb3, 0xc7, 0xd7, 0xea, 0xe1, 0x14, 0x5b, 0x53, 0xa6, 0x0a, 0xe3, 0xd5]));
        assert_eq!(Bitcoin::InputScriptSigHash.cmr(), Cmr::from_byte_array([0x1c, 0x7e, 0xfb, 0x37, 0xcb, 0xc4, 0xb9, 0x96, 0xd9, 0xb8, 0xc6, 0x17, 0x08, 0xe6, 0x65, 0x73, 0xce, 0x87, 0xc1, 0xf0, 0xaa, 0x05, 0x22, 0xd0, 0x65, 0xba, 0x90, 0x25, 0x05, 0x6e, 0x3f, 0x80]));
        assert_eq!(Bitcoin::InputSequence.cmr(), Cmr::from_byte_array([0x5d, 0xd1, 0x7f, 0xe1, 0x55, 0x0f, 0x48, 0xf7, 0xba, 0xed, 0x4d, 0x06, 0x80, 0x08, 0xd4, 0xa1, 0xff, 0x98, 0xcb, 0xeb, 0xe2, 0x54, 0x1c, 0x5d, 0xc7, 0x7a, 0xc5, 0x3a, 0xd8, 0x3f, 0xa7, 0x79]));
        assert_eq!(Bitcoin::InputSequencesHash.cmr(), Cmr::from_byte_array([0x55, 0x97, 0x3d, 0x64, 0x43, 0x45, 0x74, 0x72, 0x81, 0x37, 0x73, 0x64, 0xba, 0xd2, 0xa8, 0x0b, 0x75, 0x8f, 0x45, 0x66, 0x60, 0xc1, 0x8d, 0xe5, 0xc0, 0x1a, 0x38, 0x82, 0x07, 0x1d, 0x50, 0xe8]));
        assert_eq!(Bitcoin::InputUtxosHash.cmr(), Cmr::from_byte_array([0xd6, 0xf9, 0x0c, 0xd1, 0x04, 0xe1, 0xa5, 0xc6, 0x1a, 0x4b, 0x50, 0x00, 0xad, 0x9a, 0xba, 0x8d, 0x43, 0x00, 0x4b, 0xf9, 0x43, 0xdf, 0x32, 0x5f, 0xa6, 0x36, 0xd1, 0xa2, 0x2b, 0xec, 0xa0, 0xcb]));
        assert_eq!(Bitcoin::InputUtxoHash.cmr(), Cmr::from_byte_array([0x93, 0x1e, 0x4e, 0x95, 0xe0, 0xa6, 0x47, 0x60, 0x27, 0x6e, 0x91, 0xb5, 0xdc, 0x74, 0x67, 0x80, 0xd0, 0x69, 0x7d, 0x0a, 0xf5, 0xaa, 0xf5, 0xbb, 0xc8, 0x1d, 0xbe, 0xb3, 0x98, 0x59, 0x6a, 0xbb]));
        assert_eq!(Bitcoin::InputValue.cmr(), Cmr::from_byte_array([0x7d, 0x3c, 0x3f, 0x95, 0x5b, 0x2c, 0xf0, 0xd0, 0xd1, 0x28, 0x0a, 0x1b, 0xb1, 0x20, 0x46, 0x92, 0x92, 0xd1, 0x32, 0x9c, 0x83, 0xa9, 0xc2, 0xff, 0x7e, 0x7e, 0x1e, 0xb3, 0xf6, 0x97, 0x83, 0xa3]));
        assert_eq!(Bitcoin::InputValuesHash.cmr(), Cmr::from_byte_array([0x29, 0x83, 0x9e, 0xad, 0x0e, 0xb0, 0x3f, 0xe4, 0x65, 0x42, 0xe3, 0x6d, 0x71, 0xe9, 0xe6, 0xaf, 0xdf, 0x96, 0x93, 0x01, 0x53, 0x3d, 0x74, 0xee, 0x09, 0x9b, 0x12, 0x66, 0xa2, 0x50, 0x55, 0x2c]));
        assert_eq!(Bitcoin::InternalKey.cmr(), Cmr::from_byte_array([0x37, 0x48, 0x36, 0x99, 0x28, 0x10, 0x02, 0x2f, 0x88, 0xe0, 0x14, 0x5b, 0xca, 0xd7, 0x7f, 0x4a, 0x84, 0x91, 0xfa, 0x80, 0x83, 0xcb, 0x51, 0xc3, 0x01, 0xfc, 0xf7, 0xa1, 0x34, 0x78, 0xc2, 0xcc]));
        assert_eq!(Bitcoin::IsOne16.cmr(), Cmr::from_byte_array([0x1b, 0xd3, 0xa2, 0x53, 0xdb, 0x24, 0x3f, 0xca, 0x45, 0x53, 0x37, 0x99, 0xfe, 0x91, 0x48, 0x38, 0xc3, 0x8e, 0x38, 0x06, 0xb1, 0x2b, 0xd7, 0xe8, 0x5c, 0xa7, 0x12, 0x07, 0xa8, 0x84, 0x62, 0xb0]));
        assert_eq!(Bitcoin::IsOne32.cmr(), Cmr::from_byte_array([0x78, 0xb1, 0xba, 0xe0, 0x99, 0xec, 0x9c, 0x59, 0xcb, 0xf4, 0x12, 0x62, 0x51, 0xc1, 0xe9, 0x67, 0x41, 0xb3, 0x50, 0xd5, 0x63, 0xbd, 0x74, 0xd5, 0x44, 0x18, 0xba, 0x78, 0xeb, 0xea, 0x25, 0xbf]));
        assert_eq!(Bitcoin::IsOne64.cmr(), Cmr::from_byte_array([0x81, 0x7b, 0x95, 0xa5, 0x39, 0x5e, 0xfb, 0xec, 0xbb, 0x85, 0x15, 0xa5, 0x5b, 0x3f, 0xfe, 0x1a, 0x4d, 0x7b, 0xac, 0x6e, 0x23, 0xdb, 0xca, 0x54, 0xad, 0x60, 0x66, 0x66, 0x2f, 0x20, 0x2b, 0x93]));
        assert_eq!(Bitcoin::IsOne8.cmr(), Cmr::from_byte_array([0xf6, 0x92, 0x54, 0x91, 0xd3, 0x4b, 0x37, 0x74, 0x2c, 0xb0, 0x8d, 0xec, 0x19, 0x3e, 0xe5, 0x12, 0x5f, 0x93, 0x3c, 0xad, 0xcc, 0x23, 0x2a, 0xed, 0xee, 0xdb, 0x57, 0x2d, 0x12, 0x60, 0xff, 0xd5]));
        assert_eq!(Bitcoin::IsZero16.cmr(), Cmr::from_byte_array([0x1b, 0xa7, 0x21, 0x3b, 0x58, 0x8b, 0xe0, 0x92, 0xb4, 0x46, 0x59, 0x9c, 0x2a, 0x60, 0xff, 0x54, 0x67, 0x13, 0x6a, 0x79, 0x75, 0x99, 0x61, 0x0b, 0xd7, 0xa5, 0xf1, 0x78, 0x04, 0xe3, 0x2a, 0x2c]));
        assert_eq!(Bitcoin::IsZero32.cmr(), Cmr::from_byte_array([0x5e, 0xbf, 0x14, 0x66, 0x93, 0xf0, 0xe2, 0xd2, 0xf9, 0x36, 0x1b, 0x47, 0x6d, 0xba, 0x34, 0x85, 0x8b, 0x83, 0x2d, 0x66, 0xfa, 0xcf, 0x71, 0x3b, 0xfb, 0x32, 0xc3, 0xbb, 0x8d, 0xb9, 0xee, 0xbf]));
        assert_eq!(Bitcoin::IsZero64.cmr(), Cmr::from_byte_array([0x19, 0xab, 0x9a, 0xc0, 0xcf, 0x42, 0x66, 0x82, 0x19, 0xba, 0x6c, 0xb8, 0x97, 0xe4, 0x87, 0xfe, 0x36, 0x80, 0x93, 0x7f, 0xff, 0xa8, 0xd2, 0x03, 0x51, 0x1d, 0xb7, 0x5d, 0xbb, 0x10, 0xc7, 0xe5]));
        assert_eq!(Bitcoin::IsZero8.cmr(), Cmr::from_byte_array([0x8e, 0xff, 0x62, 0x08, 0x44, 0x07, 0xe9, 0xaf, 0xd5, 0x40, 0xf3, 0x18, 0xf6, 0x6b, 0xcf, 0x31, 0xdf, 0x1d, 0x42, 0xa5, 0xc1, 0x61, 0xca, 0xe3, 0x5a, 0x29, 0x48, 0x18, 0x0c, 0xa2, 0xaa, 0x2e]));
        assert_eq!(Bitcoin::Leftmost16_1.cmr(), Cmr::from_byte_array([0x5b, 0xff, 0x4c, 0xb5, 0x58, 0x76, 0x05, 0xd5, 0xfd, 0x05, 0x9d, 0x77, 0x33, 0x49, 0x0d, 0x7d, 0xd2, 0x2d, 0x27, 0x8b, 0x59, 0x9e, 0x06, 0xd3, 0xb5, 0xdb, 0x6d, 0x79, 0xf3, 0xc9, 0x23, 0xbd]));
        assert_eq!(Bitcoin::Leftmost16_2.cmr(), Cmr::from_byte_array([0x53, 0x6d, 0xb4, 0x86, 0xb1, 0x22, 0x27, 0xe5, 0xb0, 0x9d, 0x6f, 0xeb, 0xd2, 0x77, 0x6b, 0x1a, 0xbb, 0xc6, 0x74, 0x99, 0x96, 0xaa, 0x78, 0x3e, 0xd7, 0xe5, 0x37, 0x44, 0x6b, 0xbf, 0x15, 0x1b]));
        assert_eq!(Bitcoin::Leftmost16_4.cmr(), Cmr::from_byte_array([0xf2, 0x32, 0x13, 0x67, 0x49, 0x6d, 0x1a, 0x77, 0xee, 0xa0, 0x5e, 0x95, 0xe3, 0xb8, 0x07, 0xd3, 0xba, 0x5f, 0x05, 0x13, 0x6c, 0xe0, 0x91, 0x2a, 0xe7, 0x17, 0xc8, 0x3a, 0x02, 0x61, 0xb2, 0xe1]));
        assert_eq!(Bitcoin::Leftmost16_8.cmr(), Cmr::from_byte_array([0x24, 0x14, 0x8e, 0xf3, 0x0a, 0xd4, 0x3e, 0xbe, 0xc5, 0x63, 0x72, 0x83, 0x22, 0xc3, 0xce, 0x11, 0x79, 0xae, 0xd7, 0xa7, 0x82, 0x16, 0xd7, 0x99, 0x88, 0x8b, 0xf1, 0x8b, 0x39, 0x57, 0x06, 0x71]));
        assert_eq!(Bitcoin::Leftmost32_1.cmr(), Cmr::from_byte_array([0xb9, 0x2e, 0x15, 0xec, 0x5d, 0xa0, 0x7e, 0xe8, 0xed, 0x39, 0x7c, 0xb9, 0xf6, 0x0a, 0x4c, 0x5d, 0xa8, 0x38, 0x62, 0x93, 0x1a, 0x90, 0x73, 0x59, 0xd2, 0x7c, 0xae, 0xb6, 0x0e, 0x60, 0xef, 0x8a]));
        assert_eq!(Bitcoin::Leftmost32_16.cmr(), Cmr::from_byte_array([0xad, 0xb0, 0x27, 0xb2, 0x06, 0x56, 0x73, 0x58, 0x53, 0x26, 0xc0, 0x1c, 0x3b, 0xe2, 0xfa, 0xeb, 0x38, 0x63, 0x49, 0xe2, 0x90, 0x09, 0xb6, 0x57, 0x6e, 0xe5, 0x3a, 0x85, 0x55, 0x12, 0xcc, 0x67]));
        assert_eq!(Bitcoin::Leftmost32_2.cmr(), Cmr::from_byte_array([0xb7, 0x5b, 0x31, 0xc5, 0x59, 0x12, 0x3d, 0x3d, 0x63, 0x35, 0x98, 0x59, 0x32, 0xb8, 0xb1, 0xb2, 0x66, 0x4e, 0xe5, 0x97, 0xaf, 0xb1, 0x5f, 0xd1, 0xa4, 0x99, 0xd0, 0x07, 0xcf, 0xf2, 0x75, 0x5c]));
        assert_eq!(Bitcoin::Leftmost32_4.cmr(), Cmr::from_byte_array([0xcb, 0x75, 0x7e, 0x47, 0x1e, 0x9d, 0x9a, 0x40, 0x77, 0x1d, 0xd1, 0xcf, 0x3c, 0x1b, 0xf5, 0xd2, 0x3c, 0x17, 0xed, 0x68, 0xcd, 0xbd, 0xb2, 0x2d, 0xad, 0xa1, 0x7a, 0x73, 0xa7, 0xb4, 0x07, 0xb2]));
        assert_eq!(Bitcoin::Leftmost32_8.cmr(), Cmr::from_byte_array([0xbf, 0xc5, 0x34, 0xb4, 0x9e, 0x06, 0x00, 0x6e, 0x19, 0xf3, 0xb6, 0x8e, 0x0a, 0x02, 0x39, 0x1c, 0x14, 0x9f, 0x9a, 0x34, 0xf4, 0x3e, 0xe3, 0x6b, 0x9f, 0x1d, 0x79, 0xa7, 0x9c, 0x9a, 0x9e, 0x4d]));
        assert_eq!(Bitcoin::Leftmost64_1.cmr(), Cmr::from_byte_array([0x1b, 0x1d, 0x4e, 0x92, 0x38, 0x4b, 0x8b, 0x15, 0x9b, 0xa0, 0xd8, 0x06, 0x55, 0x8b, 0x54, 0x94, 0xe3, 0x61, 0x4e, 0xed, 0xe0, 0x3c, 0x94, 0x6c, 0xea, 0xf1, 0x41, 0xf3, 0x6f, 0x01, 0xc7, 0x9b]));
        assert_eq!(Bitcoin::Leftmost64_16.cmr(), Cmr::from_byte_array([0x0d, 0xeb, 0xdc, 0x1a, 0xa0, 0x43, 0x30, 0x34, 0x42, 0xe1, 0x8f, 0xe0, 0x3d, 0x8a, 0x99, 0xd2, 0xbe, 0x6b, 0xb8, 0xa8, 0x69, 0x1a, 0xba, 0x19, 0x56, 0x62, 0x59, 0xe3, 0x67, 0x60, 0xf7, 0xf9]));
        assert_eq!(Bitcoin::Leftmost64_2.cmr(), Cmr::from_byte_array([0x83, 0x9e, 0xcf, 0xa3, 0x18, 0x70, 0x5c, 0x25, 0x3d, 0x0c, 0x52, 0xff, 0x27, 0xb9, 0x04, 0x64, 0x92, 0x3d, 0x8c, 0x0e, 0x55, 0xa8, 0x2c, 0x0d, 0x16, 0x24, 0x02, 0x39, 0x7f, 0x36, 0x53, 0x78]));
        assert_eq!(Bitcoin::Leftmost64_32.cmr(), Cmr::from_byte_array([0x92, 0x91, 0x97, 0xa9, 0x64, 0x28, 0x61, 0xa7, 0x7b, 0xd6, 0x62, 0x58, 0x05, 0x11, 0x97, 0xbe, 0x86, 0xff, 0x08, 0xe6, 0x28, 0xe3, 0x0f, 0x7e, 0xfc, 0xbd, 0x2c, 0x4d, 0xfe, 0xcf, 0x9b, 0xdd]));
        assert_eq!(Bitcoin::Leftmost64_4.cmr(), Cmr::from_byte_array([0x02, 0xbd, 0x16, 0x45, 0xd5, 0x75, 0xf0, 0x4b, 0x3c, 0xbb, 0xaa, 0x6d, 0x8c, 0xa9, 0x86, 0xef, 0x1c, 0x8c, 0xd0, 0xff, 0xe1, 0x65, 0x89, 0x03, 0x93, 0x9d, 0xb7, 0x64, 0x56, 0x2a, 0x26, 0x47]));
        assert_eq!(Bitcoin::Leftmost64_8.cmr(), Cmr::from_byte_array([0x35, 0x58, 0xb3, 0x1b, 0x3b, 0x6e, 0x8f, 0x9a, 0x28, 0x8f, 0xdc, 0x72, 0xf2, 0x46, 0x02, 0xbe, 0x05, 0x58, 0x19, 0x10, 0x71, 0xa5, 0x4a, 0x99, 0xfa, 0x03, 0xa0, 0x25, 0x34, 0xf8, 0x80, 0x05]));
        assert_eq!(Bitcoin::Leftmost8_1.cmr(), Cmr::from_byte_array([0x28, 0x65, 0xef, 0xd4, 0x29, 0x83, 0xcb, 0xe3, 0xf8, 0x16, 0x37, 0x3a, 0xb8, 0xa8, 0x82, 0xf1, 0x83, 0x17, 0x19, 0x4d, 0xc1, 0xab, 0xa3, 0x8d, 0xa0, 0x30, 0x4b, 0x8c, 0x14, 0x4b, 0x1d, 0xa4]));
        assert_eq!(Bitcoin::Leftmost8_2.cmr(), Cmr::from_byte_array([0x51, 0x96, 0x4c, 0xb0, 0x74, 0x05, 0xa8, 0xd2, 0x3d, 0x21, 0x87, 0x74, 0x1a, 0x9e, 0xd3, 0x04, 0xbc, 0xb4, 0x69, 0xd9, 0xac, 0x9f, 0x5d, 0x92, 0x55, 0x82, 0x5c, 0xfd, 0xa3, 0xda, 0x07, 0xc0]));
        assert_eq!(Bitcoin::Leftmost8_4.cmr(), Cmr::from_byte_array([0x88, 0x3c, 0x94, 0xf8, 0xa2, 0x6c, 0xda, 0xb7, 0xbc, 0x5c, 0xd6, 0x31, 0xe5, 0x22, 0x55, 0xa8, 0x5e, 0xf6, 0xe0, 0x70, 0x76, 0x64, 0x57, 0xf6, 0x32, 0x1e, 0x2c, 0xcb, 0x11, 0x9d, 0x9b, 0x2b]));
        assert_eq!(Bitcoin::LeftExtend16_32.cmr(), Cmr::from_byte_array([0x28, 0x99, 0x97, 0xfb, 0xa1, 0xfa, 0xe7, 0xec, 0x1c, 0x45, 0x31, 0xc5, 0x0b, 0xbf, 0x86, 0x71, 0xb8, 0x97, 0x13, 0x9b, 0xdd, 0x3a, 0xad, 0x97, 0xa3, 0x76, 0x39, 0x57, 0x4a, 0x04, 0x7c, 0x80]));
        assert_eq!(Bitcoin::LeftExtend16_64.cmr(), Cmr::from_byte_array([0x5d, 0xff, 0x21, 0xf6, 0xe6, 0x12, 0x47, 0x75, 0xc5, 0x78, 0xea, 0xf4, 0x85, 0x5c, 0x0b, 0x01, 0x64, 0xf7, 0x87, 0x9b, 0x17, 0x60, 0xf9, 0x02, 0x7c, 0xb5, 0x0f, 0x7b, 0x5a, 0xcb, 0x49, 0x18]));
        assert_eq!(Bitcoin::LeftExtend1_16.cmr(), Cmr::from_byte_array([0x8c, 0x87, 0xd7, 0x56, 0xd1, 0x4b, 0xd3, 0xd9, 0xa7, 0x86, 0x90, 0x81, 0x29, 0x12, 0xb8, 0x94, 0x29, 0xc0, 0x17, 0x1a, 0x41, 0x10, 0x3a, 0x58, 0xc6, 0xe9, 0xf2, 0x25, 0x14, 0x1a, 0x02, 0x22]));
        assert_eq!(Bitcoin::LeftExtend1_32.cmr(), Cmr::from_byte_array([0xc8, 0xf1, 0x54, 0xd4, 0x6d, 0x2e, 0x78, 0x95, 0xda, 0x1b, 0x33, 0xc2, 0xb3, 0x15, 0xe6, 0xd4, 0xd4, 0x85, 0x1d, 0xde, 0xe2, 0x8a, 0xef, 0x8b, 0x70, 0x70, 0x90, 0x61, 0x6b, 0xc7, 0xee, 0xa0]));
        assert_eq!(Bitcoin::LeftExtend1_64.cmr(), Cmr::from_byte_array([0xa3, 0x40, 0x4d, 0xf6, 0x8c, 0xc9, 0x20, 0x75, 0x4c, 0x6e, 0x18, 0x47, 0x20, 0x7d, 0xb3, 0x84, 0x5d, 0x11, 0xc7, 0x49, 0x09, 0xd0, 0x7c, 0xa8, 0x2a, 0xd1, 0xf1, 0xcc, 0x67, 0xbf, 0x3a, 0x9b]));
        assert_eq!(Bitcoin::LeftExtend1_8.cmr(), Cmr::from_byte_array([0x3b, 0xca, 0x33, 0x97, 0xb8, 0x3c, 0x27, 0xf3, 0x63, 0x16, 0xf8, 0xb8, 0xb3, 0x03, 0x35, 0x0a, 0xfe, 0x8b, 0xa0, 0x07, 0x8f, 0x77, 0xf1, 0xd4, 0x2a, 0x9b, 0x78, 0x92, 0xb2, 0xa4, 0xdb, 0xee]));
        assert_eq!(Bitcoin::LeftExtend32_64.cmr(), Cmr::from_byte_array([0x42, 0xcb, 0xeb, 0x01, 0xfe, 0x7a, 0x3a, 0x6d, 0xd3, 0x31, 0x1d, 0xb3, 0x36, 0x5f, 0x91, 0xe5, 0xc1, 0x18, 0xc7, 0xe4, 0x1f, 0x03, 0xaa, 0xe7, 0xb2, 0x83, 0xde, 0x6b, 0xb9, 0x05, 0x3e, 0x6b]));
        assert_eq!(Bitcoin::LeftExtend8_16.cmr(), Cmr::from_byte_array([0x9a, 0x57, 0xc9, 0x6a, 0xf5, 0x71, 0x48, 0x96, 0xb7, 0x24, 0xde, 0x45, 0xeb, 0x9f, 0xe9, 0x7d, 0x73, 0x69, 0x7d, 0xe6, 0x2e, 0x8d, 0xad, 0x78, 0x71, 0xeb, 0x58, 0xf5, 0x81, 0xa0, 0x11, 0xbb]));
        assert_eq!(Bitcoin::LeftExtend8_32.cmr(), Cmr::from_byte_array([0xd6, 0x24, 0xbd, 0x40, 0x40, 0x76, 0x3c, 0xb1, 0x3c, 0xca, 0xd4, 0x98, 0xf5, 0x3d, 0x38, 0xc1, 0x12, 0xf1, 0x92, 0x95, 0x68, 0x26, 0xda, 0xfe, 0xc9, 0xac, 0x91, 0x65, 0x79, 0x2b, 0x34, 0x7a]));
        assert_eq!(Bitcoin::LeftExtend8_64.cmr(), Cmr::from_byte_array([0x9d, 0xc4, 0xa2, 0x05, 0x4d, 0x5d, 0x26, 0x34, 0x2a, 0xc5, 0x90, 0xb6, 0x67, 0xf1, 0xb0, 0x1d, 0xf5, 0x4f, 0xd0, 0xcd, 0xaa, 0x40, 0x5e, 0xf8, 0xcb, 0xb7, 0x6f, 0xd8, 0xf9, 0xb0, 0x0e, 0xe5]));
        assert_eq!(Bitcoin::LeftPadHigh16_32.cmr(), Cmr::from_byte_array([0x05, 0x45, 0xc4, 0xb5, 0x8f, 0x00, 0x4a, 0x21, 0xe7, 0xf1, 0x29, 0xa4, 0xc0, 0x51, 0x89, 0x97, 0x17, 0x14, 0xca, 0xa2, 0xd9, 0x1d, 0x1d, 0xfd, 0x5f, 0xad, 0x3e, 0x63, 0x24, 0x49, 0x94, 0x28]));
        assert_eq!(Bitcoin::LeftPadHigh16_64.cmr(), Cmr::from_byte_array([0x1c, 0x61, 0xd0, 0x3d, 0x49, 0x3b, 0xbd, 0x05, 0x82, 0x22, 0x59, 0xd1, 0x73, 0x0a, 0x8d, 0x7a, 0x5f, 0x55, 0xb0, 0xba, 0x2a, 0x93, 0x91, 0xa6, 0xc8, 0x88, 0x1e, 0xb4, 0x75, 0x04, 0xaf, 0xfd]));
        assert_eq!(Bitcoin::LeftPadHigh1_16.cmr(), Cmr::from_byte_array([0x56, 0xfd, 0xf5, 0x4f, 0x1f, 0xcd, 0x19, 0x82, 0x5e, 0x7c, 0x3b, 0x79, 0x06, 0x15, 0xc1, 0xd3, 0xfe, 0x82, 0x88, 0x6c, 0x74, 0x7b, 0xc4, 0x87, 0x59, 0x87, 0xf5, 0x05, 0x16, 0x94, 0x5f, 0xb3]));
        assert_eq!(Bitcoin::LeftPadHigh1_32.cmr(), Cmr::from_byte_array([0xdb, 0x33, 0x05, 0x9a, 0xbe, 0x2d, 0x43, 0x2d, 0x67, 0xf4, 0x2b, 0x1e, 0x94, 0x27, 0x56, 0xdc, 0xa6, 0xcd, 0xe6, 0x37, 0x85, 0xe5, 0xbd, 0x43, 0x0d, 0xc8, 0xf4, 0xae, 0xfc, 0x31, 0xb8, 0xdf]));
        assert_eq!(Bitcoin::LeftPadHigh1_64.cmr(), Cmr::from_byte_array([0x1d, 0x66, 0x9c, 0x1f, 0xa5, 0xfd, 0x3e, 0xf6, 0x6e, 0xb4, 0xae, 0xf6, 0x18, 0x6e, 0x3e, 0xc1, 0x36, 0xee, 0x75, 0x84, 0x10, 0xdf, 0x3e, 0xde, 0xbb, 0x31, 0xbf, 0x26, 0xd4, 0x56, 0x20, 0x51]));
        assert_eq!(Bitcoin::LeftPadHigh1_8.cmr(), Cmr::from_byte_array([0x9a, 0x1b, 0xad, 0x3d, 0x8a, 0xb9, 0x00, 0x30, 0x3d, 0xa2, 0x02, 0xf0, 0xf4, 0x49, 0xf0, 0xb7, 0xe6, 0x79, 0x5c, 0x2a, 0x7c, 0x12, 0x17, 0x18, 0x80, 0x0a, 0xc4, 0x0c, 0x87, 0xd8, 0x27, 0x29]));
        assert_eq!(Bitcoin::LeftPadHigh32_64.cmr(), Cmr::from_byte_array([0x39, 0x20, 0xcc, 0x4b, 0x33, 0xba, 0xf7, 0xef, 0xa5, 0xca, 0xf9, 0xe7, 0x80, 0x01, 0x44, 0x67, 0x06, 0xf6, 0xe4, 0xe8, 0x26, 0x56, 0x74, 0x05, 0x7e, 0xed, 0x87, 0x17, 0x78, 0x08, 0x9e, 0x94]));
        assert_eq!(Bitcoin::LeftPadHigh8_16.cmr(), Cmr::from_byte_array([0x75, 0x2e, 0x29, 0xf2, 0xfe, 0x2b, 0xec, 0xc3, 0xf6, 0x62, 0x90, 0xfe, 0x44, 0xe1, 0xae, 0xb3, 0x78, 0x41, 0x80, 0xdd, 0x90, 0x5e, 0x19, 0x62, 0x4e, 0x19, 0x5f, 0x21, 0x6c, 0x07, 0xc5, 0x7c]));
        assert_eq!(Bitcoin::LeftPadHigh8_32.cmr(), Cmr::from_byte_array([0xbe, 0xe8, 0x8f, 0x1c, 0x8c, 0x30, 0x63, 0x4c, 0x6e, 0x95, 0xca, 0xcc, 0x0e, 0x9a, 0xdd, 0x49, 0x41, 0x32, 0x21, 0xfd, 0xab, 0xbd, 0x8d, 0x4c, 0x0a, 0xcc, 0xf1, 0xca, 0xe2, 0xd2, 0xa7, 0x78]));
        assert_eq!(Bitcoin::LeftPadHigh8_64.cmr(), Cmr::from_byte_array([0x39, 0x23, 0x87, 0xf6, 0xdc, 0x04, 0xbf, 0xc5, 0x4d, 0xd4, 0xa2, 0x81, 0x19, 0xc8, 0x1d, 0x15, 0xd7, 0xa5, 0x80, 0x9b, 0xbf, 0x62, 0xfc, 0xc2, 0x7d, 0xc5, 0x5c, 0xf8, 0x2e, 0x9e, 0x5e, 0xe6]));
        assert_eq!(Bitcoin::LeftPadLow16_32.cmr(), Cmr::from_byte_array([0x4f, 0xfd, 0x6c, 0xb3, 0x40, 0x23, 0x05, 0x82, 0x1d, 0xd8, 0x99, 0x70, 0xd7, 0x22, 0xd1, 0xc1, 0x3f, 0x1f, 0xf7, 0x73, 0x9f, 0xd5, 0xf3, 0x4b, 0xa1, 0x6c, 0x73, 0x65, 0x3b, 0x04, 0x47, 0x18]));
        assert_eq!(Bitcoin::LeftPadLow16_64.cmr(), Cmr::from_byte_array([0xbe, 0x3e, 0xb8, 0x5c, 0x5f, 0x19, 0x91, 0x53, 0xfb, 0x1c, 0x46, 0x13, 0x5c, 0x04, 0xfa, 0xcf, 0xdb, 0xc6, 0xf1, 0xb7, 0x8c, 0x2b, 0xb7, 0xae, 0x75, 0xf1, 0x55, 0xbc, 0x3e, 0xa0, 0x8a, 0x8b]));
        assert_eq!(Bitcoin::LeftPadLow1_16.cmr(), Cmr::from_byte_array([0xdd, 0xd0, 0x15, 0x3e, 0xf3, 0x12, 0xf2, 0x8d, 0x64, 0x2c, 0xd9, 0x4c, 0xb3, 0x6f, 0x32, 0x97, 0x75, 0xb0, 0x0d, 0xa8, 0x8f, 0xcc, 0xc4, 0xce, 0xa1, 0xba, 0xe8, 0x9b, 0xad, 0x13, 0xbe, 0x6b]));
        assert_eq!(Bitcoin::LeftPadLow1_32.cmr(), Cmr::from_byte_array([0xbc, 0x9d, 0x31, 0x14, 0x35, 0x46, 0x7b, 0xc0, 0x8b, 0x10, 0x08, 0xe5, 0x47, 0xaa, 0x7a, 0x07, 0xe8, 0x3b, 0x15, 0x14, 0x68, 0x61, 0xa9, 0xe9, 0xb5, 0x41, 0x3b, 0xe3, 0x1b, 0x82, 0xb6, 0xb5]));
        assert_eq!(Bitcoin::LeftPadLow1_64.cmr(), Cmr::from_byte_array([0x8b, 0xc6, 0x2f, 0x93, 0x60, 0x89, 0x4e, 0x48, 0xa4, 0x73, 0x2c, 0x95, 0x76, 0x9c, 0x8f, 0xaa, 0xe9, 0x56, 0x8f, 0x9d, 0xe8, 0xe8, 0xa2, 0x00, 0x83, 0x6b, 0xd4, 0xe5, 0x0b, 0x02, 0xcd, 0x84]));
        assert_eq!(Bitcoin::LeftPadLow1_8.cmr(), Cmr::from_byte_array([0xf6, 0x6c, 0xd7, 0xa4, 0x2b, 0x32, 0x0f, 0x97, 0xc1, 0x9f, 0x2d, 0x54, 0x16, 0xcd, 0xe0, 0x87, 0x25, 0x3a, 0x27, 0x91, 0x29, 0x65, 0xd5, 0x5b, 0x65, 0x71, 0x2a, 0xd8, 0x09, 0xb8, 0x3c, 0xfd]));
        assert_eq!(Bitcoin::LeftPadLow32_64.cmr(), Cmr::from_byte_array([0xa3, 0x3a, 0x07, 0xb9, 0xbc, 0xf9, 0x45, 0xf6, 0x4f, 0x07, 0x2b, 0x8b, 0x9c, 0x91, 0x48, 0x39, 0xa5, 0x85, 0xbf, 0xa9, 0xf3, 0x42, 0x5b, 0x14, 0x77, 0x54, 0xab, 0x55, 0xa8, 0xba, 0x6c, 0x0f]));
        assert_eq!(Bitcoin::LeftPadLow8_16.cmr(), Cmr::from_byte_array([0x2a, 0x51, 0x6a, 0x79, 0x3f, 0x97, 0xc4, 0x5f, 0xea, 0xeb, 0xb1, 0xcc, 0x96, 0x1a, 0x15, 0x6d, 0x80, 0x35, 0x49, 0x28, 0x79, 0x78, 0x9d, 0x6e, 0xdc, 0x9b, 0x57, 0xe7, 0x2f, 0x11, 0xe5, 0xb5]));
        assert_eq!(Bitcoin::LeftPadLow8_32.cmr(), Cmr::from_byte_array([0x1a, 0xa2, 0xe4, 0xd0, 0x4b, 0xd6, 0x90, 0x55, 0x12, 0x3d, 0xd6, 0xaa, 0xfe, 0x27, 0xf5, 0xf7, 0xf4, 0x7c, 0x3b, 0x30, 0x90, 0xc3, 0xa8, 0x27, 0x29, 0x73, 0xfe, 0x2f, 0x75, 0x16, 0x5a, 0x5d]));
        assert_eq!(Bitcoin::LeftPadLow8_64.cmr(), Cmr::from_byte_array([0xb6, 0x52, 0xe0, 0xae, 0xdd, 0x0f, 0x4f, 0x66, 0xf6, 0xa1, 0xcd, 0x4b, 0xeb, 0xf8, 0x75, 0xff, 0x7b, 0xbb, 0x2d, 0xd9, 0x9b, 0x06, 0x5b, 0x2d, 0xb5, 0xb5, 0xb5, 0x90, 0x53, 0x61, 0x61, 0x4d]));
        assert_eq!(Bitcoin::LeftRotate16.cmr(), Cmr::from_byte_array([0x8a, 0x12, 0xff, 0x6a, 0x4b, 0xf2, 0x37, 0x15, 0xdd, 0x3b, 0x76, 0x6b, 0x99, 0x67, 0xc7, 0x15, 0x8b, 0xf3, 0xed, 0x74, 0xb3, 0xdc, 0xe7, 0x30, 0xaf, 0xfc, 0xf4, 0x66, 0x16, 0x47, 0x8e, 0xcb]));
        assert_eq!(Bitcoin::LeftRotate32.cmr(), Cmr::from_byte_array([0x2f, 0xcb, 0x52, 0x17, 0x2f, 0xd4, 0x9c, 0x36, 0x21, 0x7d, 0xea, 0xe0, 0xc2, 0x37, 0x14, 0x32, 0x1f, 0x69, 0xf5, 0xf1, 0x3f, 0x6e, 0x94, 0xb2, 0xbd, 0xfe, 0x4b, 0x74, 0x88, 0x69, 0x7f, 0xd5]));
        assert_eq!(Bitcoin::LeftRotate64.cmr(), Cmr::from_byte_array([0x72, 0xcc, 0xd6, 0xc4, 0xe5, 0xfd, 0xf6, 0x8a, 0xd3, 0x3b, 0x6d, 0x58, 0xfb, 0x37, 0x2b, 0xe4, 0xf1, 0xb8, 0x0e, 0xef, 0x70, 0x1f, 0x9d, 0xb7, 0xe5, 0xed, 0x85, 0x9b, 0x96, 0xb3, 0x62, 0x09]));
        assert_eq!(Bitcoin::LeftRotate8.cmr(), Cmr::from_byte_array([0x1a, 0xae, 0xc9, 0xf3, 0xb7, 0x5d, 0x89, 0xf8, 0x2a, 0x64, 0x98, 0x45, 0x8c, 0x44, 0x83, 0xcb, 0x9a, 0x78, 0x44, 0x89, 0x05, 0xf3, 0xbb, 0x39, 0xfc, 0x08, 0x3f, 0x14, 0xdd, 0xcc, 0xdc, 0x9b]));
        assert_eq!(Bitcoin::LeftShift16.cmr(), Cmr::from_byte_array([0x37, 0xac, 0x63, 0x87, 0x21, 0xab, 0x09, 0x7a, 0x96, 0x02, 0xba, 0x4d, 0xc9, 0x2e, 0x19, 0xb5, 0xa1, 0x85, 0xb2, 0x32, 0x9f, 0x1a, 0xa6, 0x00, 0xcb, 0x9c, 0x15, 0x61, 0x5a, 0x00, 0x81, 0xf8]));
        assert_eq!(Bitcoin::LeftShift32.cmr(), Cmr::from_byte_array([0x8e, 0x3c, 0x47, 0x3b, 0x28, 0x67, 0xf1, 0x54, 0x73, 0xb3, 0x63, 0x2d, 0xbf, 0xdd, 0x99, 0x77, 0x55, 0x51, 0xef, 0x5f, 0x9d, 0xba, 0x47, 0x5e, 0x9c, 0xf0, 0x90, 0x75, 0x80, 0x70, 0xf0, 0xbf]));
        assert_eq!(Bitcoin::LeftShift64.cmr(), Cmr::from_byte_array([0x50, 0x49, 0xf4, 0x04, 0xd1, 0x73, 0x29, 0x9a, 0x3a, 0xee, 0x04, 0xcb, 0xc2, 0x46, 0x2c, 0xb3, 0x4c, 0x80, 0x69, 0xc1, 0xb6, 0xdb, 0x7f, 0xed, 0x0e, 0x38, 0x8f, 0xf6, 0xd4, 0x67, 0xa0, 0x86]));
        assert_eq!(Bitcoin::LeftShift8.cmr(), Cmr::from_byte_array([0x83, 0x2f, 0x63, 0x6e, 0x63, 0x44, 0x6c, 0xef, 0xba, 0x8d, 0xf3, 0xa4, 0x6e, 0xfb, 0xb3, 0x61, 0x59, 0xc1, 0x88, 0x54, 0x56, 0x77, 0x68, 0xad, 0xc9, 0xb8, 0xdb, 0x8a, 0x07, 0x49, 0x2a, 0x58]));
        assert_eq!(Bitcoin::LeftShiftWith16.cmr(), Cmr::from_byte_array([0xe6, 0x47, 0x62, 0xb1, 0xc5, 0xe6, 0x14, 0x4a, 0x71, 0x81, 0xea, 0xaf, 0x4d, 0xd9, 0xd9, 0xb3, 0xaa, 0x43, 0xaa, 0xd9, 0x55, 0x15, 0x81, 0x98, 0xee, 0x20, 0x90, 0xeb, 0xd9, 0xe4, 0xbb, 0x0d]));
        assert_eq!(Bitcoin::LeftShiftWith32.cmr(), Cmr::from_byte_array([0x64, 0x76, 0xba, 0x89, 0x95, 0xf8, 0x3b, 0x5e, 0xe1, 0xeb, 0xc2, 0x2c, 0xb4, 0x16, 0xf5, 0x58, 0x15, 0x7f, 0x2e, 0x57, 0x69, 0x9a, 0x5c, 0xaf, 0x84, 0x29, 0x1f, 0xf3, 0xfc, 0x14, 0x83, 0xc1]));
        assert_eq!(Bitcoin::LeftShiftWith64.cmr(), Cmr::from_byte_array([0x06, 0xb8, 0xfe, 0x67, 0xcf, 0xc5, 0x86, 0x32, 0x23, 0x97, 0xaf, 0x02, 0x4f, 0xde, 0x29, 0x11, 0xf7, 0xae, 0x87, 0xa0, 0x6a, 0xbc, 0x6c, 0x59, 0x30, 0x93, 0x40, 0x97, 0x15, 0x69, 0x1c, 0x19]));
        assert_eq!(Bitcoin::LeftShiftWith8.cmr(), Cmr::from_byte_array([0xb1, 0xac, 0x9c, 0x68, 0x23, 0x58, 0xc4, 0x5b, 0xab, 0xf4, 0x06, 0x95, 0x56, 0xfe, 0x6e, 0x37, 0x5b, 0x45, 0x54, 0xde, 0x9e, 0x10, 0xc5, 0x91, 0xc1, 0x48, 0x39, 0x84, 0x47, 0xac, 0x18, 0x0e]));
        assert_eq!(Bitcoin::Le16.cmr(), Cmr::from_byte_array([0x01, 0x67, 0x05, 0xa7, 0xd7, 0xdc, 0xe1, 0xaf, 0xc6, 0x3e, 0xab, 0x84, 0x20, 0x3f, 0x5f, 0x42, 0xd6, 0xb6, 0xbb, 0xad, 0x75, 0xce, 0xe3, 0x8c, 0xec, 0x5a, 0x51, 0x5b, 0x59, 0x97, 0x48, 0x9f]));
        assert_eq!(Bitcoin::Le32.cmr(), Cmr::from_byte_array([0x53, 0x51, 0xfc, 0x5d, 0xeb, 0xe5, 0xb2, 0x98, 0xad, 0x70, 0x57, 0xe4, 0xa5, 0xa7, 0x6a, 0x3b, 0x9c, 0x65, 0x8a, 0xcd, 0xe7, 0xd1, 0xbb, 0x52, 0xe5, 0x88, 0x9c, 0xa1, 0xe3, 0x8f, 0x5e, 0xfb]));
        assert_eq!(Bitcoin::Le64.cmr(), Cmr::from_byte_array([0xae, 0x2d, 0xe1, 0xe0, 0xcf, 0x73, 0x0d, 0x1d, 0xcc, 0x96, 0xd7, 0xcc, 0xfe, 0x71, 0x16, 0x8a, 0x24, 0x0d, 0xea, 0xf8, 0x04, 0x61, 0x5a, 0x7b, 0xa9, 0x20, 0xdc, 0x16, 0xfd, 0x6e, 0xa4, 0x5f]));
        assert_eq!(Bitcoin::Le8.cmr(), Cmr::from_byte_array([0xaf, 0x29, 0xf6, 0x16, 0x8e, 0xbd, 0xc0, 0x9e, 0xfb, 0xe0, 0xe6, 0x39, 0xcb, 0x75, 0x0b, 0x12, 0x05, 0x78, 0x8f, 0x90, 0x21, 0xd6, 0x66, 0xef, 0xce, 0xfe, 0x13, 0xf1, 0x2f, 0x96, 0x71, 0xf0]));
        assert_eq!(Bitcoin::LinearCombination1.cmr(), Cmr::from_byte_array([0x34, 0x10, 0xa9, 0xee, 0x33, 0x3d, 0xf8, 0xc8, 0xa0, 0x1c, 0x14, 0x11, 0x5b, 0x54, 0x43, 0x27, 0xe3, 0x24, 0xe2, 0x87, 0xaa, 0x11, 0x07, 0xe0, 0x19, 0x55, 0xbd, 0x20, 0x50, 0x6e, 0xa9, 0x87]));
        assert_eq!(Bitcoin::LinearVerify1.cmr(), Cmr::from_byte_array([0xdc, 0x66, 0xd3, 0x31, 0xc1, 0x7f, 0x3f, 0xdd, 0xa3, 0x99, 0x46, 0x98, 0x1b, 0x39, 0xb3, 0x57, 0xd0, 0x55, 0x5c, 0x35, 0x62, 0xec, 0xae, 0x02, 0xaa, 0x2d, 0xad, 0x16, 0x3e, 0x6c, 0x9a, 0x2e]));
        assert_eq!(Bitcoin::LockTime.cmr(), Cmr::from_byte_array([0x9a, 0xe0, 0xac, 0xc3, 0x7b, 0xc2, 0x04, 0x47, 0x79, 0xb0, 0x7c, 0x3d, 0x46, 0x02, 0xa5, 0xfd, 0xe8, 0xbc, 0x33, 0xf8, 0x79, 0xf6, 0x6b, 0x73, 0x9b, 0x10, 0xf0, 0x1a, 0xeb, 0x11, 0x54, 0xec]));
        assert_eq!(Bitcoin::Low1.cmr(), Cmr::from_byte_array([0xfe, 0x62, 0x14, 0xf9, 0x67, 0x15, 0x6d, 0xcd, 0xe6, 0xdd, 0x49, 0xfd, 0xc5, 0x5e, 0xfb, 0x86, 0x50, 0x69, 0xfe, 0xab, 0xff, 0xf0, 0xfe, 0x93, 0x1d, 0xba, 0x85, 0x31, 0x34, 0xee, 0xd1, 0x30]));
        assert_eq!(Bitcoin::Low16.cmr(), Cmr::from_byte_array([0x74, 0x93, 0xcf, 0x69, 0x8a, 0x48, 0x82, 0xe5, 0xc3, 0x57, 0x9d, 0x06, 0x51, 0x8e, 0x7e, 0xca, 0x2b, 0x84, 0x28, 0xf6, 0x2e, 0x2b, 0x51, 0x38, 0x02, 0xab, 0xe6, 0x22, 0x17, 0x0c, 0x20, 0xfe]));
        assert_eq!(Bitcoin::Low32.cmr(), Cmr::from_byte_array([0x36, 0x2d, 0x66, 0xa4, 0xf0, 0xae, 0xb9, 0x65, 0x84, 0xa5, 0x67, 0x57, 0x82, 0x71, 0xb1, 0xf7, 0xbb, 0xfc, 0xc2, 0xde, 0x0d, 0xcf, 0x95, 0x79, 0x6b, 0x6f, 0x7a, 0x82, 0x6b, 0x2a, 0x8a, 0xf7]));
        assert_eq!(Bitcoin::Low64.cmr(), Cmr::from_byte_array([0x97, 0x33, 0x23, 0xbc, 0x2b, 0x92, 0xe4, 0x28, 0x04, 0xd2, 0xe4, 0xf5, 0x8b, 0x86, 0xf6, 0x5b, 0x56, 0xf9, 0x1d, 0xee, 0xb4, 0x81, 0x0e, 0xab, 0x8a, 0x1d, 0xed, 0xa9, 0x69, 0x7a, 0x08, 0x72]));
        assert_eq!(Bitcoin::Low8.cmr(), Cmr::from_byte_array([0xcd, 0x1a, 0x85, 0x58, 0xef, 0x99, 0xa3, 0x22, 0x60, 0x21, 0x7a, 0x76, 0x49, 0xff, 0x51, 0x40, 0xda, 0x69, 0xda, 0x70, 0x06, 0x72, 0x69, 0x0b, 0x27, 0x91, 0x7b, 0x07, 0xd7, 0xc1, 0x4c, 0x67]));
        assert_eq!(Bitcoin::Lt16.cmr(), Cmr::from_byte_array([0x04, 0xac, 0xa8, 0x7e, 0x3e, 0x17, 0xf8, 0x05, 0xa2, 0x1c, 0xf2, 0x91, 0x7a, 0xee, 0x99, 0x57, 0xb9, 0x50, 0xb2, 0xdb, 0x5d, 0x7a, 0xe5, 0xc8, 0x26, 0xd4, 0xac, 0x2e, 0xc9, 0x7b, 0x5a, 0x52]));
        assert_eq!(Bitcoin::Lt32.cmr(), Cmr::from_byte_array([0x23, 0xa0, 0xa5, 0xc1, 0x97, 0x74, 0x7e, 0x3a, 0x95, 0x79, 0xe9, 0x0e, 0x0f, 0x22, 0xf8, 0x4a, 0x29, 0xbf, 0xb5, 0xf0, 0x7b, 0x84, 0xb5, 0x9b, 0x26, 0x68, 0x8a, 0x0c, 0xd5, 0x9d, 0xfe, 0xbd]));
        assert_eq!(Bitcoin::Lt64.cmr(), Cmr::from_byte_array([0xd2, 0x99, 0x90, 0x1c, 0x7b, 0x5b, 0x3a, 0x59, 0xff, 0xc8, 0xdd, 0x09, 0x54, 0x5a, 0x32, 0x38, 0x24, 0xb7, 0x79, 0xa9, 0x9b, 0x2d, 0x1a, 0x2f, 0x87, 0x45, 0x2d, 0x9e, 0x4b, 0xef, 0xaf, 0x30]));
        assert_eq!(Bitcoin::Lt8.cmr(), Cmr::from_byte_array([0xdd, 0x94, 0x41, 0x3b, 0x52, 0x9c, 0x29, 0x8c, 0x16, 0x96, 0xe9, 0xfb, 0x08, 0xe6, 0x67, 0x67, 0xb3, 0xf8, 0x33, 0x7a, 0xc0, 0x2e, 0x44, 0xb0, 0x68, 0xe9, 0x40, 0x14, 0xf7, 0xc4, 0x1f, 0x2a]));
        assert_eq!(Bitcoin::Maj1.cmr(), Cmr::from_byte_array([0x0e, 0x6f, 0xb4, 0x0f, 0xe3, 0x1a, 0x3a, 0x52, 0x6b, 0x44, 0xcf, 0x0b, 0x7c, 0x79, 0x36, 0xc7, 0x77, 0xcb, 0xba, 0x89, 0x65, 0xa7, 0x25, 0x52, 0x32, 0xa7, 0xcf, 0x53, 0xa9, 0x22, 0x88, 0x5a]));
        assert_eq!(Bitcoin::Maj16.cmr(), Cmr::from_byte_array([0x38, 0x66, 0x9c, 0xe5, 0xe1, 0xe1, 0x71, 0x47, 0x54, 0x00, 0x73, 0x1b, 0xee, 0xb6, 0x0b, 0xca, 0xfa, 0xd6, 0x66, 0x04, 0xc9, 0x39, 0x40, 0x16, 0x0c, 0xd7, 0x12, 0x88, 0x35, 0x55, 0x93, 0x42]));
        assert_eq!(Bitcoin::Maj32.cmr(), Cmr::from_byte_array([0x55, 0x54, 0x34, 0x9b, 0x58, 0x4f, 0x5c, 0x38, 0x72, 0xc7, 0xf4, 0xf2, 0x57, 0x82, 0x9e, 0x2a, 0xe8, 0x22, 0xd8, 0x23, 0x42, 0x4c, 0xeb, 0x95, 0x98, 0xf0, 0x83, 0x18, 0x58, 0x6a, 0x88, 0x07]));
        assert_eq!(Bitcoin::Maj64.cmr(), Cmr::from_byte_array([0x73, 0x49, 0x03, 0xba, 0xef, 0xb7, 0x1d, 0x5e, 0xa4, 0x16, 0x48, 0xff, 0x43, 0xee, 0xe6, 0x98, 0x94, 0xe0, 0x63, 0xb3, 0x88, 0xea, 0x42, 0x2f, 0x96, 0xae, 0xde, 0x19, 0x3c, 0xea, 0xb8, 0x39]));
        assert_eq!(Bitcoin::Maj8.cmr(), Cmr::from_byte_array([0xba, 0x47, 0xa3, 0x99, 0xdc, 0x94, 0x35, 0xe1, 0x8e, 0x08, 0x0a, 0x4e, 0x18, 0xaf, 0x7c, 0x65, 0x7f, 0xd3, 0x9f, 0x7c, 0xe7, 0xd6, 0x05, 0x2e, 0x46, 0x90, 0x23, 0x11, 0xb0, 0x78, 0xd5, 0x85]));
        assert_eq!(Bitcoin::Max16.cmr(), Cmr::from_byte_array([0xaa, 0x55, 0x23, 0x74, 0x6c, 0xab, 0xfa, 0xf5, 0x66, 0x8e, 0x9e, 0x07, 0x37, 0xe5, 0x6b, 0x06, 0x06, 0x22, 0x51, 0xd7, 0xe8, 0x0a, 0xb9, 0xb9, 0x10, 0x6d, 0x8f, 0x17, 0x2d, 0xc8, 0x4d, 0xd6]));
        assert_eq!(Bitcoin::Max32.cmr(), Cmr::from_byte_array([0x69, 0x22, 0x96, 0x5d, 0x14, 0x43, 0x45, 0xc9, 0x13, 0xec, 0xb3, 0x0b, 0x5e, 0xd4, 0x7e, 0x88, 0xda, 0xe3, 0x5c, 0x12, 0x21, 0xf2, 0x6a, 0xa9, 0x2d, 0xd5, 0xa5, 0xf6, 0x15, 0xdb, 0xdb, 0x53]));
        assert_eq!(Bitcoin::Max64.cmr(), Cmr::from_byte_array([0x8a, 0x9b, 0xe9, 0x07, 0xb6, 0xa4, 0xc3, 0x0a, 0xbc, 0xc0, 0xf2, 0x2d, 0x01, 0x30, 0x74, 0xc2, 0xd5, 0x6b, 0xb0, 0x81, 0xf2, 0x62, 0x18, 0x57, 0xd5, 0x38, 0xcc, 0x97, 0x13, 0x1e, 0x44, 0x09]));
        assert_eq!(Bitcoin::Max8.cmr(), Cmr::from_byte_array([0xb4, 0xbf, 0x93, 0x23, 0x40, 0x22, 0xe8, 0x60, 0xfe, 0x76, 0xc0, 0xb5, 0x36, 0x0e, 0x8b, 0x36, 0xff, 0x81, 0xee, 0x67, 0x05, 0xb5, 0x93, 0xac, 0xdf, 0x65, 0x5a, 0xc6, 0xe6, 0xd7, 0xae, 0xba]));
        assert_eq!(Bitcoin::Median16.cmr(), Cmr::from_byte_array([0x17, 0xe2, 0xe8, 0x7f, 0x07, 0x60, 0xf4, 0xfb, 0x3c, 0x9f, 0xd0, 0xbe, 0xd0, 0x00, 0xd7, 0x39, 0x73, 0xab, 0x60, 0xf5, 0xe6, 0xc2, 0xc1, 0xfa, 0xb1, 0x7f, 0x9b, 0x23, 0xee, 0x6a, 0xca, 0x48]));
        assert_eq!(Bitcoin::Median32.cmr(), Cmr::from_byte_array([0x11, 0x60, 0xae, 0x8e, 0xa8, 0xd3, 0x0f, 0x9a, 0x22, 0x33, 0xc4, 0x8e, 0x73, 0x12, 0x40, 0xf8, 0x44, 0x93, 0xb8, 0x28, 0xb5, 0x57, 0x93, 0xe2, 0xf4, 0x04, 0x2a, 0x19, 0x82, 0xac, 0x26, 0xa5]));
        assert_eq!(Bitcoin::Median64.cmr(), Cmr::from_byte_array([0xc8, 0x73, 0x73, 0x64, 0x9e, 0x7e, 0x40, 0x50, 0xbb, 0x73, 0x33, 0x7e, 0x08, 0xeb, 0x5d, 0xe4, 0x52, 0x28, 0xab, 0x86, 0xad, 0x4e, 0x1f, 0x41, 0x91, 0xe5, 0x20, 0x2a, 0xa6, 0xaf, 0xa0, 0xc5]));
        assert_eq!(Bitcoin::Median8.cmr(), Cmr::from_byte_array([0xc3, 0xb4, 0xe0, 0x89, 0x8a, 0x21, 0xbd, 0xe9, 0x4d, 0xae, 0xd3, 0x7a, 0x20, 0xad, 0xf9, 0x0c, 0x8b, 0xe5, 0x69, 0x1a, 0x03, 0xb6, 0xa1, 0xe5, 0x56, 0x38, 0x5d, 0x42, 0xeb, 0x19, 0x02, 0x2b]));
        assert_eq!(Bitcoin::Min16.cmr(), Cmr::from_byte_array([0x5f, 0xd0, 0x05, 0x1e, 0xdb, 0x37, 0x19, 0xa6, 0x45, 0xb2, 0x72, 0xa0, 0x21, 0x08, 0xef, 0xbb, 0x3d, 0x9b, 0xc0, 0xf6, 0x06, 0x21, 0xbf, 0x5a, 0x5b, 0xab, 0xe1, 0x16, 0xd5, 0x55, 0xd5, 0x78]));
        assert_eq!(Bitcoin::Min32.cmr(), Cmr::from_byte_array([0xd8, 0x07, 0x82, 0xa2, 0xb5, 0xd8, 0x6a, 0xb6, 0xb9, 0xc9, 0xc3, 0xfb, 0x77, 0x8a, 0x34, 0x73, 0xf6, 0x00, 0xb1, 0x85, 0xfe, 0x19, 0x25, 0xee, 0x9f, 0xc2, 0xe8, 0x77, 0x7e, 0xd2, 0x66, 0x01]));
        assert_eq!(Bitcoin::Min64.cmr(), Cmr::from_byte_array([0xc5, 0xc0, 0x9d, 0x50, 0x13, 0x38, 0xe9, 0xa5, 0x12, 0xcf, 0x89, 0x76, 0xca, 0x4b, 0x32, 0xb9, 0x24, 0x80, 0xbe, 0xf6, 0xae, 0xb2, 0x9d, 0x36, 0xd5, 0x90, 0xd3, 0x5b, 0xf9, 0xf9, 0xec, 0xe1]));
        assert_eq!(Bitcoin::Min8.cmr(), Cmr::from_byte_array([0x81, 0xd2, 0x1e, 0x12, 0x81, 0x42, 0x38, 0x81, 0x80, 0x2c, 0x0e, 0x0c, 0x7d, 0x22, 0xbd, 0x34, 0xd2, 0x6b, 0xd1, 0x2a, 0x4c, 0x4f, 0x1b, 0x70, 0x68, 0xe7, 0xe1, 0x83, 0x82, 0x08, 0x48, 0xe9]));
        assert_eq!(Bitcoin::Modulo16.cmr(), Cmr::from_byte_array([0xb6, 0xb8, 0x7c, 0xfa, 0xb6, 0x7e, 0x55, 0x19, 0xf1, 0xc9, 0x98, 0xda, 0x47, 0x94, 0x37, 0xbb, 0x79, 0xe6, 0x74, 0xf7, 0x15, 0xe9, 0xa2, 0xe5, 0x38, 0xee, 0xc5, 0xec, 0x18, 0xe1, 0x8e, 0xa5]));
        assert_eq!(Bitcoin::Modulo32.cmr(), Cmr::from_byte_array([0x8d, 0x48, 0x6e, 0x83, 0x16, 0x54, 0xf3, 0x8a, 0x32, 0xda, 0x35, 0xeb, 0x7b, 0xb6, 0x55, 0xa6, 0xed, 0x69, 0x4d, 0xbf, 0xa0, 0x58, 0x95, 0x7d, 0x9f, 0x5c, 0xbf, 0xcc, 0x57, 0x92, 0xc6, 0x5b]));
        assert_eq!(Bitcoin::Modulo64.cmr(), Cmr::from_byte_array([0x14, 0xdf, 0x20, 0xd9, 0x3d, 0xfd, 0xef, 0xe2, 0x55, 0x9b, 0xac, 0x50, 0xed, 0x38, 0x19, 0x3b, 0xd7, 0x8b, 0xd6, 0x3f, 0x92, 0x9d, 0x86, 0xfb, 0x4f, 0x29, 0xa7, 0xc5, 0xaf, 0x32, 0x42, 0xad]));
        assert_eq!(Bitcoin::Modulo8.cmr(), Cmr::from_byte_array([0x2c, 0x75, 0x8a, 0x7c, 0x0f, 0x59, 0xe8, 0x00, 0xe9, 0x4f, 0x3d, 0xc5, 0xa0, 0x01, 0xbf, 0x8e, 0xd9, 0x43, 0x5f, 0x75, 0xa2, 0xd9, 0x69, 0x30, 0xc5, 0x7e, 0xaa, 0xb0, 0xcd, 0x80, 0xaf, 0x5c]));
        assert_eq!(Bitcoin::Multiply16.cmr(), Cmr::from_byte_array([0x75, 0xbd, 0x41, 0xf2, 0xd2, 0xb3, 0x39, 0xf0, 0x69, 0xbf, 0xdf, 0xd8, 0x02, 0xd6, 0x1e, 0x6c, 0xa8, 0xe3, 0xba, 0xd6, 0xfb, 0x6d, 0x95, 0xb6, 0x72, 0x09, 0x5b, 0x93, 0x34, 0x5f, 0x04, 0x7f]));
        assert_eq!(Bitcoin::Multiply32.cmr(), Cmr::from_byte_array([0x84, 0xcb, 0xe6, 0xce, 0x87, 0x03, 0x79, 0x92, 0x13, 0x87, 0x7c, 0x1b, 0xd5, 0x05, 0xc7, 0x64, 0x34, 0x33, 0x69, 0x00, 0x2e, 0x50, 0x2c, 0x43, 0xd9, 0x7f, 0x3d, 0x57, 0x77, 0x2d, 0x6c, 0x87]));
        assert_eq!(Bitcoin::Multiply64.cmr(), Cmr::from_byte_array([0x92, 0x98, 0x7b, 0x80, 0x1b, 0x92, 0xf6, 0x79, 0xeb, 0x96, 0x13, 0x68, 0x84, 0x44, 0xa1, 0x78, 0x87, 0x50, 0xa8, 0x50, 0x6e, 0x03, 0xa9, 0x21, 0x8c, 0x21, 0xec, 0xc7, 0x20, 0x82, 0xdc, 0x6a]));
        assert_eq!(Bitcoin::Multiply8.cmr(), Cmr::from_byte_array([0x76, 0x4c, 0xab, 0x71, 0xdb, 0x94, 0x59, 0xa7, 0x69, 0x6d, 0x94, 0x4a, 0x50, 0x09, 0x5b, 0x1a, 0xeb, 0xdf, 0xd9, 0x28, 0x4b, 0xdb, 0x74, 0x96, 0xa7, 0xb3, 0x02, 0x41, 0xcc, 0xba, 0x3e, 0xce]));
        assert_eq!(Bitcoin::Negate16.cmr(), Cmr::from_byte_array([0xe7, 0x60, 0xee, 0x40, 0x29, 0xc3, 0x4f, 0x89, 0x74, 0x06, 0xff, 0xde, 0xa5, 0x55, 0x84, 0x86, 0x62, 0xe8, 0x9c, 0x98, 0x3e, 0x60, 0x70, 0xbd, 0x02, 0x72, 0xad, 0x0f, 0xa3, 0x42, 0xef, 0xa3]));
        assert_eq!(Bitcoin::Negate32.cmr(), Cmr::from_byte_array([0x84, 0x95, 0xb7, 0x40, 0x09, 0xad, 0x07, 0xc9, 0x30, 0x2a, 0x25, 0xae, 0x56, 0xc3, 0xe9, 0x73, 0x3f, 0x00, 0xc2, 0xba, 0xa4, 0x10, 0xea, 0xc4, 0xa5, 0x8e, 0x75, 0xdb, 0x83, 0xaf, 0x1d, 0x22]));
        assert_eq!(Bitcoin::Negate64.cmr(), Cmr::from_byte_array([0x34, 0xe8, 0x9f, 0xaf, 0x34, 0x5a, 0xfd, 0x5e, 0x7b, 0x29, 0x00, 0x14, 0x52, 0xfc, 0x5f, 0xc2, 0xe3, 0x78, 0x3a, 0xf7, 0xf2, 0x10, 0x16, 0x43, 0xbd, 0x76, 0x70, 0x6a, 0x6f, 0xc3, 0xf3, 0x6a]));
        assert_eq!(Bitcoin::Negate8.cmr(), Cmr::from_byte_array([0xe8, 0x1b, 0xe0, 0xb1, 0x5c, 0x67, 0x1a, 0xb8, 0xdf, 0x1f, 0x48, 0x69, 0xc5, 0x7f, 0x11, 0x11, 0x18, 0xcb, 0x66, 0x83, 0x54, 0x97, 0x5c, 0x63, 0x66, 0xec, 0xb2, 0xb8, 0xbb, 0x7c, 0x15, 0xcf]));
        assert_eq!(Bitcoin::NumInputs.cmr(), Cmr::from_byte_array([0x5c, 0x5a, 0xc4, 0xff, 0x6d, 0xa5, 0x6c, 0xb3, 0x72, 0xb2, 0x32, 0x66, 0x6e, 0x83, 0x34, 0xb9, 0xe2, 0xcf, 0xb0, 0xdc, 0xb4, 0x18, 0xf1, 0x61, 0xbf, 0xf1, 0x49, 0xe8, 0x4e, 0xc9, 0x2c, 0x3e]));
        assert_eq!(Bitcoin::NumOutputs.cmr(), Cmr::from_byte_array([0x98, 0xa1, 0xcc, 0xa7, 0x05, 0xdf, 0xcf, 0xaf, 0xd3, 0xa6, 0x9e, 0x9a, 0xdc, 0x05, 0xba, 0x47, 0xe1, 0xfe, 0xfa, 0x6a, 0x29, 0xf3, 0x42, 0x86, 0x20, 0x48, 0xe4, 0x96, 0x86, 0x48, 0xc3, 0xd7]));
        assert_eq!(Bitcoin::One16.cmr(), Cmr::from_byte_array([0x2e, 0x5e, 0x3d, 0x95, 0xe4, 0x53, 0x16, 0x88, 0x8e, 0x4f, 0x37, 0x09, 0xef, 0x83, 0x2b, 0x9f, 0xd9, 0xe1, 0x5f, 0x30, 0x71, 0x9b, 0xf5, 0x5f, 0xc2, 0xe0, 0xe0, 0x9a, 0x36, 0x57, 0xd8, 0x82]));
        assert_eq!(Bitcoin::One32.cmr(), Cmr::from_byte_array([0x06, 0x42, 0x6b, 0x85, 0x3c, 0x1b, 0xcb, 0x33, 0x8a, 0xed, 0xbe, 0x1f, 0x89, 0xa6, 0xd9, 0xb7, 0xa3, 0xda, 0x03, 0x8c, 0xd0, 0x0a, 0x44, 0x71, 0x18, 0x36, 0x93, 0x49, 0x66, 0x9e, 0x29, 0x76]));
        assert_eq!(Bitcoin::One64.cmr(), Cmr::from_byte_array([0xab, 0x1d, 0x2c, 0xd9, 0x96, 0x78, 0xda, 0x3c, 0x12, 0x8d, 0x39, 0xad, 0x9f, 0xe6, 0xff, 0xa9, 0x55, 0xc1, 0x6e, 0x5e, 0xf2, 0xc2, 0x5b, 0xb4, 0x31, 0x83, 0x15, 0x59, 0x69, 0x51, 0xf4, 0x27]));
        assert_eq!(Bitcoin::One8.cmr(), Cmr::from_byte_array([0x3c, 0xc5, 0xf5, 0x23, 0xd6, 0xa6, 0x35, 0x5d, 0xc9, 0x24, 0xee, 0x0a, 0xc1, 0xf5, 0xfe, 0x2c, 0x52, 0x12, 0x75, 0xe3, 0xaa, 0x9f, 0x21, 0xd3, 0x1b, 0x08, 0x2d, 0xb2, 0xac, 0x23, 0x0d, 0x9d]));
        assert_eq!(Bitcoin::Or1.cmr(), Cmr::from_byte_array([0xc4, 0x65, 0x96, 0x43, 0x69, 0xfc, 0xa2, 0x09, 0x7f, 0x83, 0x53, 0x0c, 0x87, 0xbc, 0xbc, 0x90, 0xc3, 0x06, 0x57, 0x9d, 0x9f, 0x3b, 0xfe, 0xdd, 0xf4, 0xa1, 0x72, 0xa4, 0xea, 0x0b, 0x58, 0xec]));
        assert_eq!(Bitcoin::Or16.cmr(), Cmr::from_byte_array([0x5a, 0x98, 0x5e, 0x04, 0x3b, 0x85, 0x27, 0x3b, 0x90, 0xf9, 0x0e, 0x20, 0xf8, 0x2b, 0x75, 0x32, 0x33, 0x51, 0xcf, 0x2a, 0x4e, 0x62, 0xa7, 0xf9, 0xcb, 0x2f, 0x05, 0x96, 0x40, 0x2e, 0x9e, 0x28]));
        assert_eq!(Bitcoin::Or32.cmr(), Cmr::from_byte_array([0x35, 0x52, 0x38, 0x3a, 0x57, 0xff, 0xb4, 0x8d, 0x63, 0xa0, 0x33, 0x7a, 0xf0, 0xdd, 0x6e, 0xfa, 0xb6, 0xb4, 0x6c, 0x5d, 0xe1, 0x72, 0x0e, 0x42, 0x0b, 0xdd, 0x1c, 0x82, 0x27, 0x6b, 0xc9, 0xa9]));
        assert_eq!(Bitcoin::Or64.cmr(), Cmr::from_byte_array([0x51, 0xa1, 0x73, 0xda, 0xdc, 0xa0, 0x1a, 0xc6, 0xf6, 0x2e, 0x75, 0xd5, 0xcd, 0x35, 0x22, 0xf0, 0x9f, 0xde, 0x62, 0xb1, 0x15, 0x13, 0xe0, 0x68, 0x42, 0x28, 0x52, 0xa4, 0x91, 0x67, 0xb6, 0x06]));
        assert_eq!(Bitcoin::Or8.cmr(), Cmr::from_byte_array([0x79, 0xef, 0xbd, 0xcb, 0x53, 0x7b, 0xeb, 0xcb, 0x18, 0x8d, 0x11, 0x16, 0xb7, 0x8a, 0x10, 0x9b, 0xff, 0xbc, 0x2a, 0x6c, 0xe3, 0xd1, 0xf8, 0x70, 0x15, 0x4a, 0x79, 0x56, 0x09, 0x1b, 0x34, 0x2f]));
        assert_eq!(Bitcoin::OutpointHash.cmr(), Cmr::from_byte_array([0x3a, 0x1a, 0xe9, 0x0e, 0x16, 0x7f, 0xb4, 0x0d, 0x6e, 0x13, 0xb4, 0x51, 0xad, 0x67, 0x41, 0x0d, 0x8d, 0xd9, 0x91, 0xc8, 0x7d, 0x6a, 0x4a, 0x59, 0xcc, 0x76, 0xc6, 0x3f, 0x3b, 0x9e, 0x5e, 0x56]));
        assert_eq!(Bitcoin::OutputsHash.cmr(), Cmr::from_byte_array([0xf2, 0xeb, 0x6d, 0x0f, 0x01, 0x8e, 0x6f, 0x15, 0xe3, 0x5b, 0xaa, 0x82, 0xe5, 0x7e, 0x14, 0xfe, 0x34, 0x37, 0x96, 0xf2, 0x19, 0x68, 0x26, 0xbe, 0xd7, 0xc7, 0x87, 0x55, 0x98, 0xd6, 0x64, 0x1d]));
        assert_eq!(Bitcoin::OutputHash.cmr(), Cmr::from_byte_array([0x91, 0x21, 0x1f, 0xc6, 0x01, 0x1a, 0x64, 0x93, 0x00, 0xc6, 0xbe, 0xe9, 0x4f, 0xdd, 0x48, 0xa9, 0x7f, 0xa2, 0xa9, 0xb6, 0xf2, 0x84, 0xbe, 0x01, 0x5d, 0x46, 0x2d, 0x17, 0xde, 0x66, 0x4a, 0xc3]));
        assert_eq!(Bitcoin::OutputScriptsHash.cmr(), Cmr::from_byte_array([0xff, 0x20, 0xbc, 0x43, 0x65, 0xe7, 0x17, 0x07, 0x57, 0x1c, 0x6e, 0x17, 0x38, 0xe1, 0xed, 0x32, 0x6f, 0x7c, 0x35, 0x1d, 0xe1, 0x30, 0x22, 0xae, 0xa3, 0xd6, 0x40, 0x6b, 0x8a, 0xee, 0x8e, 0x3b]));
        assert_eq!(Bitcoin::OutputScriptHash.cmr(), Cmr::from_byte_array([0xbd, 0xfd, 0xb2, 0x31, 0xf4, 0xf1, 0xa6, 0x2c, 0x9d, 0x7b, 0x03, 0x93, 0x1e, 0x7f, 0x19, 0xa4, 0x54, 0x6a, 0xf2, 0x34, 0x75, 0x4c, 0xbf, 0x70, 0x05, 0x9f, 0xdd, 0x42, 0xbb, 0xbc, 0x41, 0x26]));
        assert_eq!(Bitcoin::OutputValue.cmr(), Cmr::from_byte_array([0x93, 0x36, 0x43, 0xb6, 0xc5, 0xa6, 0x22, 0x0a, 0xbb, 0xca, 0x6f, 0x35, 0x09, 0xfe, 0xff, 0x6d, 0x13, 0xef, 0xa6, 0xc9, 0xfa, 0xe9, 0x59, 0x24, 0x57, 0x53, 0x64, 0xf2, 0xb1, 0x64, 0xd2, 0xbc]));
        assert_eq!(Bitcoin::OutputValuesHash.cmr(), Cmr::from_byte_array([0x22, 0x89, 0x93, 0x79, 0x00, 0x57, 0x06, 0x6f, 0x20, 0x16, 0x97, 0x1d, 0xf5, 0x5e, 0x6f, 0x67, 0xd2, 0x52, 0xef, 0xb6, 0xda, 0xab, 0xd0, 0xfc, 0x56, 0x6a, 0x8d, 0x21, 0x56, 0xef, 0xbb, 0xfc]));
        assert_eq!(Bitcoin::ParseLock.cmr(), Cmr::from_byte_array([0x3d, 0xb8, 0x45, 0x35, 0xfa, 0x3d, 0x90, 0xef, 0x0b, 0x58, 0x1e, 0x22, 0xb6, 0x1d, 0x21, 0x27, 0x84, 0x4b, 0x21, 0x16, 0xe8, 0x4f, 0x81, 0x4a, 0x5c, 0xba, 0xc5, 0x2d, 0xf5, 0x15, 0xf2, 0xd2]));
        assert_eq!(Bitcoin::ParseSequence.cmr(), Cmr::from_byte_array([0x38, 0xb2, 0x53, 0x3f, 0x5f, 0xed, 0xe8, 0x69, 0xba, 0xa1, 0x70, 0x69, 0x83, 0xdf, 0x4c, 0x89, 0xd6, 0x2d, 0x5f, 0x90, 0x80, 0x0b, 0x47, 0xea, 0xb2, 0x11, 0x13, 0x31, 0x1a, 0x5a, 0xae, 0xc9]));
        assert_eq!(Bitcoin::PointVerify1.cmr(), Cmr::from_byte_array([0xbe, 0x2a, 0x98, 0x90, 0xf1, 0xd5, 0xb6, 0x15, 0x14, 0x7f, 0x82, 0x41, 0xe0, 0x60, 0x9b, 0x5c, 0xac, 0x01, 0xec, 0xe0, 0xa3, 0xf9, 0x23, 0x68, 0x67, 0xb2, 0xbf, 0xde, 0xa1, 0xb8, 0x04, 0x4e]));
        assert_eq!(Bitcoin::Rightmost16_1.cmr(), Cmr::from_byte_array([0x3f, 0x3c, 0x43, 0x46, 0x87, 0x17, 0x42, 0x26, 0x5e, 0x87, 0xf0, 0x01, 0xb4, 0x6d, 0xe7, 0xd1, 0x98, 0x75, 0x1b, 0x34, 0xfa, 0xa1, 0x80, 0x18, 0xde, 0x60, 0xc8, 0x46, 0x8d, 0x9b, 0x98, 0xa4]));
        assert_eq!(Bitcoin::Rightmost16_2.cmr(), Cmr::from_byte_array([0xc1, 0x8b, 0x9f, 0xdd, 0x34, 0x0a, 0x26, 0x7a, 0xc1, 0x6d, 0x4f, 0x39, 0xee, 0x75, 0x43, 0x56, 0x52, 0xaa, 0xca, 0x52, 0x56, 0x50, 0xb5, 0x1a, 0x45, 0x87, 0x98, 0x04, 0x8e, 0x62, 0x7d, 0x51]));
        assert_eq!(Bitcoin::Rightmost16_4.cmr(), Cmr::from_byte_array([0xc6, 0xc5, 0x3f, 0xa7, 0x1e, 0x23, 0x0c, 0xf0, 0x58, 0x51, 0x58, 0xf4, 0x70, 0x58, 0x8b, 0xac, 0x5c, 0x51, 0x8f, 0x84, 0xf9, 0xfc, 0x23, 0x86, 0x52, 0xf1, 0x75, 0xfb, 0x6e, 0xa1, 0x8c, 0x11]));
        assert_eq!(Bitcoin::Rightmost16_8.cmr(), Cmr::from_byte_array([0xee, 0x76, 0x9c, 0x1c, 0xc8, 0xa3, 0xfd, 0xd1, 0x83, 0x8f, 0xc9, 0xf0, 0x49, 0x0c, 0xe7, 0x03, 0x93, 0xfd, 0x91, 0xba, 0x3c, 0xbd, 0x4a, 0xbd, 0x08, 0x64, 0x9f, 0xb9, 0xc4, 0x43, 0x11, 0xbd]));
        assert_eq!(Bitcoin::Rightmost32_1.cmr(), Cmr::from_byte_array([0x1c, 0x44, 0x23, 0x69, 0xfb, 0x81, 0xf6, 0x11, 0xd3, 0x28, 0x01, 0x0b, 0x86, 0x4b, 0xcc, 0xb7, 0xf3, 0x5e, 0xd4, 0x77, 0xdf, 0xa3, 0x85, 0x55, 0x74, 0xc1, 0x35, 0x64, 0xcd, 0xbd, 0xb8, 0x60]));
        assert_eq!(Bitcoin::Rightmost32_16.cmr(), Cmr::from_byte_array([0xad, 0xd2, 0xc3, 0x39, 0x0d, 0x9a, 0xf7, 0xc2, 0x4a, 0x15, 0x9a, 0x37, 0xd6, 0x9d, 0x44, 0x84, 0xd2, 0xc2, 0x4a, 0x2c, 0xb5, 0xb0, 0xeb, 0x2d, 0x3c, 0x49, 0x3d, 0x98, 0x12, 0xac, 0xfd, 0x74]));
        assert_eq!(Bitcoin::Rightmost32_2.cmr(), Cmr::from_byte_array([0x00, 0xb8, 0x81, 0x5a, 0xd7, 0x42, 0x3d, 0xd5, 0x8c, 0xb9, 0x8b, 0xe8, 0x2c, 0xad, 0x26, 0x67, 0x5c, 0x3b, 0xf5, 0x4a, 0x0b, 0xed, 0xba, 0xde, 0x34, 0x64, 0xb4, 0xfe, 0x5a, 0x4e, 0x8c, 0xe6]));
        assert_eq!(Bitcoin::Rightmost32_4.cmr(), Cmr::from_byte_array([0x84, 0xfa, 0x5a, 0x54, 0xf7, 0x72, 0x9f, 0x9d, 0x68, 0x99, 0x4b, 0xea, 0xb9, 0x3a, 0xe7, 0x9b, 0x8c, 0x4a, 0x10, 0xd5, 0xb7, 0xae, 0x97, 0x27, 0xaa, 0x17, 0x16, 0xe5, 0x7d, 0x03, 0x3b, 0x74]));
        assert_eq!(Bitcoin::Rightmost32_8.cmr(), Cmr::from_byte_array([0x7d, 0x38, 0x05, 0xd3, 0xc7, 0x8c, 0x4e, 0xea, 0x91, 0xe3, 0xd3, 0x5e, 0xfd, 0xd4, 0x7e, 0xed, 0xd4, 0x21, 0xaf, 0x84, 0xd2, 0x19, 0x10, 0x32, 0x93, 0x32, 0xa0, 0xb5, 0x48, 0x7f, 0xab, 0x63]));
        assert_eq!(Bitcoin::Rightmost64_1.cmr(), Cmr::from_byte_array([0xd3, 0xb1, 0x64, 0xc5, 0xdc, 0x66, 0xcc, 0x7e, 0xf9, 0x23, 0x4f, 0xed, 0xe4, 0xdc, 0x7f, 0x0d, 0xa5, 0xcd, 0x71, 0xc1, 0xc1, 0xd4, 0xca, 0xd6, 0x0f, 0xb4, 0xec, 0x57, 0x3e, 0x2b, 0x8a, 0x75]));
        assert_eq!(Bitcoin::Rightmost64_16.cmr(), Cmr::from_byte_array([0xea, 0xe4, 0x34, 0x78, 0xf9, 0xf2, 0xf4, 0x52, 0xef, 0xac, 0x15, 0xee, 0xe6, 0x0f, 0x8b, 0x52, 0x53, 0xd8, 0x0a, 0x2d, 0x32, 0x12, 0x9b, 0x4e, 0x5b, 0xa3, 0x83, 0x00, 0xad, 0x98, 0x52, 0xfd]));
        assert_eq!(Bitcoin::Rightmost64_2.cmr(), Cmr::from_byte_array([0x9c, 0xd4, 0xa9, 0x8b, 0xbd, 0xb8, 0xa3, 0x35, 0x85, 0xc0, 0x0f, 0x47, 0xd6, 0xad, 0xab, 0x7a, 0xf5, 0x42, 0x86, 0xfb, 0x8a, 0xe6, 0x0f, 0x72, 0x30, 0x11, 0xfb, 0x84, 0xc0, 0xee, 0x78, 0xf9]));
        assert_eq!(Bitcoin::Rightmost64_32.cmr(), Cmr::from_byte_array([0x7f, 0x24, 0x20, 0xae, 0x5b, 0x0f, 0x5a, 0x3f, 0x6f, 0x2e, 0x60, 0xb6, 0x1f, 0x8a, 0x41, 0x5c, 0x08, 0x8b, 0x94, 0xb2, 0x1c, 0x1a, 0x62, 0xa3, 0xfd, 0xaa, 0xc7, 0x49, 0xdb, 0xdf, 0x4c, 0x71]));
        assert_eq!(Bitcoin::Rightmost64_4.cmr(), Cmr::from_byte_array([0xe2, 0x65, 0x55, 0x2a, 0x24, 0xfb, 0xcd, 0xec, 0x05, 0x83, 0xd7, 0x18, 0x3e, 0x48, 0xeb, 0xc2, 0xff, 0x6d, 0x31, 0x65, 0x57, 0xba, 0xc5, 0x91, 0x5c, 0x03, 0xcb, 0x23, 0x35, 0xd2, 0x32, 0x95]));
        assert_eq!(Bitcoin::Rightmost64_8.cmr(), Cmr::from_byte_array([0x98, 0xcd, 0x95, 0xf9, 0x5d, 0x46, 0x64, 0x1b, 0x04, 0x9e, 0x77, 0xbf, 0x90, 0xee, 0xa5, 0x98, 0xad, 0xf2, 0x9e, 0xe5, 0x00, 0xe6, 0x50, 0x72, 0x87, 0x54, 0x8b, 0xb1, 0xcd, 0xaf, 0x78, 0x4d]));
        assert_eq!(Bitcoin::Rightmost8_1.cmr(), Cmr::from_byte_array([0x08, 0x76, 0xfc, 0xd4, 0x69, 0x85, 0x91, 0xf3, 0x31, 0x91, 0x01, 0x57, 0x4c, 0xe1, 0x53, 0xfc, 0xdf, 0xe9, 0x4f, 0x58, 0x1a, 0xac, 0x5e, 0x75, 0xf3, 0xcd, 0x74, 0x46, 0xdf, 0x56, 0xf3, 0xc7]));
        assert_eq!(Bitcoin::Rightmost8_2.cmr(), Cmr::from_byte_array([0xb9, 0xf7, 0xb2, 0x90, 0xaf, 0xe7, 0xf1, 0x89, 0xe3, 0x2a, 0xeb, 0xf2, 0xcc, 0x4d, 0xdc, 0xa9, 0x6b, 0xb0, 0x07, 0x64, 0xc7, 0xbe, 0x28, 0x87, 0xdc, 0xe0, 0x54, 0xd0, 0x9e, 0x38, 0xc3, 0x53]));
        assert_eq!(Bitcoin::Rightmost8_4.cmr(), Cmr::from_byte_array([0xf2, 0x8e, 0x9a, 0xf5, 0xaf, 0x4c, 0x9c, 0xca, 0x4b, 0x43, 0xcc, 0x6a, 0xdf, 0x9d, 0x9d, 0x8d, 0x16, 0x9c, 0x87, 0xc5, 0x55, 0x9f, 0x9f, 0x3c, 0xca, 0xc8, 0xf2, 0x35, 0x2b, 0x62, 0x9f, 0x18]));
        assert_eq!(Bitcoin::RightExtend16_32.cmr(), Cmr::from_byte_array([0xdb, 0xf1, 0x8d, 0x87, 0xa7, 0x89, 0x21, 0x39, 0xa3, 0x88, 0xe9, 0xa9, 0x83, 0xc4, 0x89, 0x92, 0xac, 0x35, 0xa8, 0x45, 0x56, 0xee, 0x0d, 0xef, 0xc1, 0xda, 0xdf, 0x0c, 0x5f, 0x47, 0x1a, 0x26]));
        assert_eq!(Bitcoin::RightExtend16_64.cmr(), Cmr::from_byte_array([0xd0, 0x11, 0xac, 0xc7, 0x94, 0xe3, 0xc4, 0x78, 0x9a, 0xcc, 0xd0, 0xd5, 0xfe, 0x49, 0x97, 0xd3, 0x34, 0xd9, 0x1f, 0x08, 0x31, 0xa1, 0xeb, 0x35, 0x04, 0xb4, 0xcb, 0x2d, 0xdf, 0x47, 0x97, 0xaf]));
        assert_eq!(Bitcoin::RightExtend32_64.cmr(), Cmr::from_byte_array([0xa5, 0xaa, 0x5d, 0xb1, 0xe5, 0x35, 0xe7, 0x23, 0x2a, 0xd3, 0x6d, 0xaf, 0xba, 0x6d, 0x5a, 0x20, 0x0d, 0x54, 0xeb, 0x85, 0x3b, 0x75, 0xdc, 0x70, 0xa5, 0x94, 0xed, 0x64, 0xaa, 0x6b, 0xd9, 0xab]));
        assert_eq!(Bitcoin::RightExtend8_16.cmr(), Cmr::from_byte_array([0x81, 0x06, 0xd5, 0x8a, 0x80, 0x66, 0xee, 0x6e, 0x15, 0xe5, 0x5c, 0xa5, 0x2c, 0xb7, 0xaf, 0xd8, 0xe3, 0x27, 0x75, 0x87, 0xbf, 0xd7, 0xde, 0xc0, 0xbe, 0x37, 0xd4, 0x06, 0x74, 0x2a, 0x39, 0x31]));
        assert_eq!(Bitcoin::RightExtend8_32.cmr(), Cmr::from_byte_array([0xdf, 0xa4, 0xba, 0xfa, 0x43, 0x2a, 0x53, 0x38, 0xd3, 0x74, 0xde, 0xb6, 0xb7, 0x24, 0xb7, 0xf6, 0xea, 0xe5, 0x58, 0x61, 0xfe, 0x73, 0x1d, 0x43, 0x04, 0x8a, 0xa3, 0x04, 0xd1, 0xf7, 0xf9, 0xa2]));
        assert_eq!(Bitcoin::RightExtend8_64.cmr(), Cmr::from_byte_array([0x62, 0x0a, 0x37, 0x03, 0x8b, 0x6f, 0xa1, 0x27, 0x49, 0x5f, 0x0b, 0x46, 0x49, 0x6f, 0x64, 0x35, 0xdd, 0x2d, 0xad, 0x7e, 0xf0, 0xc0, 0xfd, 0x2c, 0xd6, 0x5f, 0x54, 0xdc, 0x18, 0x5e, 0x99, 0x7b]));
        assert_eq!(Bitcoin::RightPadHigh16_32.cmr(), Cmr::from_byte_array([0x2b, 0x6a, 0xbc, 0x38, 0x32, 0x1a, 0x7c, 0x54, 0x2f, 0xb1, 0x69, 0x74, 0x62, 0x1c, 0xed, 0x80, 0x88, 0x0d, 0xb5, 0x19, 0xbb, 0x48, 0x60, 0x93, 0x42, 0x6e, 0x8c, 0xe1, 0x8e, 0x01, 0x69, 0xb1]));
        assert_eq!(Bitcoin::RightPadHigh16_64.cmr(), Cmr::from_byte_array([0xad, 0x90, 0xd8, 0xff, 0xa5, 0x74, 0x50, 0xb3, 0xb5, 0xe9, 0x09, 0x62, 0x25, 0x34, 0x9e, 0xd8, 0xf0, 0x72, 0xe1, 0x01, 0x72, 0x93, 0xf3, 0x92, 0xef, 0x85, 0x4e, 0x03, 0x19, 0xab, 0xc9, 0x34]));
        assert_eq!(Bitcoin::RightPadHigh1_16.cmr(), Cmr::from_byte_array([0x28, 0x81, 0x58, 0xb1, 0xc9, 0x10, 0x87, 0x7b, 0x7e, 0xea, 0x3d, 0xfc, 0xf2, 0xb2, 0xb7, 0x88, 0x92, 0x28, 0x08, 0xb6, 0xd6, 0xfa, 0x75, 0xf8, 0x96, 0x77, 0x19, 0x04, 0x8b, 0x14, 0x12, 0x49]));
        assert_eq!(Bitcoin::RightPadHigh1_32.cmr(), Cmr::from_byte_array([0xee, 0x2a, 0xd7, 0x7f, 0x66, 0x8d, 0x3d, 0x6a, 0x2e, 0x68, 0x50, 0x6e, 0x49, 0x04, 0xcf, 0x50, 0xa0, 0x84, 0x60, 0xe1, 0xd2, 0xb8, 0x6a, 0x81, 0xe1, 0x4e, 0x41, 0xf8, 0xda, 0x4c, 0xdd, 0xf2]));
        assert_eq!(Bitcoin::RightPadHigh1_64.cmr(), Cmr::from_byte_array([0x3d, 0x6a, 0x7f, 0xe6, 0x9a, 0x11, 0x64, 0x2a, 0xce, 0xd6, 0x84, 0x2b, 0x89, 0xaa, 0x1b, 0xb8, 0x41, 0x3e, 0x39, 0x90, 0x63, 0xcc, 0x16, 0x78, 0x6a, 0xf7, 0xc0, 0x33, 0xda, 0xd5, 0x8b, 0x95]));
        assert_eq!(Bitcoin::RightPadHigh1_8.cmr(), Cmr::from_byte_array([0x28, 0x44, 0xbd, 0xfd, 0x6a, 0xba, 0x29, 0xdf, 0x03, 0xf9, 0x3a, 0xa6, 0xae, 0xb2, 0x1c, 0x06, 0x40, 0x28, 0xdb, 0x05, 0xff, 0x77, 0xd8, 0xd9, 0x1c, 0xfd, 0xcd, 0xef, 0xb1, 0x90, 0xc5, 0xbd]));
        assert_eq!(Bitcoin::RightPadHigh32_64.cmr(), Cmr::from_byte_array([0xb4, 0x32, 0xe5, 0x32, 0x1a, 0xe1, 0x71, 0x4c, 0xe1, 0x95, 0x29, 0xd8, 0x5f, 0x24, 0xff, 0x89, 0x87, 0x91, 0x0e, 0xbc, 0xf0, 0x15, 0xf8, 0x7f, 0x15, 0xbb, 0xed, 0x55, 0xf0, 0xa0, 0xe8, 0x92]));
        assert_eq!(Bitcoin::RightPadHigh8_16.cmr(), Cmr::from_byte_array([0x6f, 0x2d, 0x96, 0xc9, 0x54, 0x13, 0xca, 0x9a, 0xa8, 0xcc, 0x55, 0x0f, 0x25, 0x73, 0xe1, 0x66, 0x99, 0x56, 0xd6, 0x07, 0x69, 0x2c, 0xf1, 0xca, 0x6d, 0xc7, 0x6d, 0x2f, 0x2b, 0x4a, 0x3a, 0xc8]));
        assert_eq!(Bitcoin::RightPadHigh8_32.cmr(), Cmr::from_byte_array([0xdf, 0x2c, 0x7f, 0x92, 0x99, 0x00, 0xa4, 0x49, 0x01, 0xe6, 0xff, 0x65, 0x27, 0x6a, 0x95, 0x1a, 0xeb, 0x95, 0xdf, 0x25, 0x0b, 0x13, 0x97, 0x14, 0xd4, 0x19, 0x54, 0x04, 0xd7, 0x78, 0x98, 0xed]));
        assert_eq!(Bitcoin::RightPadHigh8_64.cmr(), Cmr::from_byte_array([0x79, 0xc0, 0x1d, 0xa3, 0xe6, 0x0b, 0x9c, 0x69, 0x35, 0xce, 0x3e, 0x15, 0x98, 0xb1, 0x78, 0x40, 0xaf, 0x82, 0xdc, 0xb0, 0xdd, 0xc6, 0x3a, 0xef, 0x4a, 0x06, 0xe7, 0xf9, 0xca, 0x5d, 0x27, 0x41]));
        assert_eq!(Bitcoin::RightPadLow16_32.cmr(), Cmr::from_byte_array([0x6f, 0x20, 0x10, 0x27, 0xcc, 0x75, 0x98, 0x02, 0x30, 0xa0, 0x70, 0x85, 0x9c, 0x3e, 0x38, 0x02, 0x36, 0xa1, 0xcb, 0x10, 0xe6, 0x1a, 0x01, 0xaa, 0x1f, 0x6d, 0x23, 0x1d, 0x15, 0x14, 0x2f, 0x25]));
        assert_eq!(Bitcoin::RightPadLow16_64.cmr(), Cmr::from_byte_array([0xb8, 0x6e, 0x1f, 0x0b, 0xfe, 0xc6, 0x55, 0x98, 0xd0, 0xa3, 0xd1, 0xec, 0x96, 0x03, 0x05, 0xb9, 0x67, 0x45, 0x67, 0x3e, 0x1b, 0x16, 0xbf, 0x32, 0x7a, 0x71, 0x68, 0x05, 0x83, 0xd7, 0x1d, 0x90]));
        assert_eq!(Bitcoin::RightPadLow1_16.cmr(), Cmr::from_byte_array([0x05, 0x2a, 0x64, 0x99, 0xc9, 0x3e, 0xe6, 0xbc, 0x1a, 0xe6, 0x57, 0xf8, 0x5f, 0xd4, 0xd4, 0xfe, 0x67, 0x7a, 0xbc, 0xee, 0x54, 0x0d, 0x13, 0x40, 0x33, 0x54, 0x2e, 0x9a, 0xb6, 0x0a, 0x63, 0xdd]));
        assert_eq!(Bitcoin::RightPadLow1_32.cmr(), Cmr::from_byte_array([0x5b, 0x70, 0xd4, 0x28, 0x96, 0x0e, 0x95, 0xcc, 0x40, 0xd5, 0x18, 0x46, 0xf5, 0x3a, 0x4d, 0x0a, 0x35, 0xc9, 0x01, 0x5d, 0x15, 0x00, 0xb6, 0xbc, 0x84, 0x9b, 0x72, 0x83, 0x5e, 0x2b, 0xd4, 0x40]));
        assert_eq!(Bitcoin::RightPadLow1_64.cmr(), Cmr::from_byte_array([0x44, 0xef, 0xeb, 0x87, 0xca, 0x2a, 0xd7, 0xfd, 0x4b, 0x73, 0xf1, 0x63, 0x07, 0xc7, 0xf0, 0x59, 0x02, 0x65, 0x6f, 0x35, 0x09, 0x0f, 0xb0, 0xa4, 0x32, 0x6c, 0x64, 0x89, 0x88, 0xae, 0x1d, 0x39]));
        assert_eq!(Bitcoin::RightPadLow1_8.cmr(), Cmr::from_byte_array([0x93, 0x40, 0x39, 0x8b, 0xcc, 0x8e, 0xa8, 0x3e, 0xc8, 0x40, 0xbe, 0x72, 0x9d, 0xbb, 0x8b, 0x81, 0x20, 0x78, 0x24, 0xee, 0x87, 0x5d, 0x15, 0x82, 0x59, 0xd6, 0xda, 0xd2, 0x0a, 0x83, 0x93, 0x0c]));
        assert_eq!(Bitcoin::RightPadLow32_64.cmr(), Cmr::from_byte_array([0x69, 0x3e, 0x28, 0x10, 0x1e, 0x04, 0xfd, 0xa4, 0x3b, 0x97, 0xe6, 0x11, 0xf0, 0xfe, 0x98, 0x00, 0x0e, 0x14, 0x30, 0x2e, 0x5d, 0xcd, 0x6e, 0xd6, 0x5e, 0xee, 0x42, 0xe3, 0x40, 0x14, 0x24, 0x2f]));
        assert_eq!(Bitcoin::RightPadLow8_16.cmr(), Cmr::from_byte_array([0x09, 0x6b, 0x25, 0xc3, 0xc8, 0x41, 0x5f, 0x04, 0xd8, 0x83, 0x27, 0x43, 0xeb, 0x2f, 0x84, 0x56, 0xd5, 0xf0, 0xa6, 0x44, 0x91, 0x3d, 0x3e, 0xc5, 0x9d, 0x34, 0xf4, 0x55, 0x25, 0x01, 0xfa, 0x20]));
        assert_eq!(Bitcoin::RightPadLow8_32.cmr(), Cmr::from_byte_array([0xfc, 0x7f, 0x57, 0x22, 0xa6, 0x2a, 0xa2, 0x20, 0x18, 0xcc, 0x81, 0xcd, 0x00, 0xa9, 0x32, 0x6c, 0x7f, 0xe9, 0xc6, 0x3a, 0xbc, 0xe2, 0xbd, 0xa4, 0xc0, 0xe6, 0x6a, 0x3f, 0x47, 0xc6, 0x7c, 0x53]));
        assert_eq!(Bitcoin::RightPadLow8_64.cmr(), Cmr::from_byte_array([0xa5, 0xbb, 0x7d, 0x5e, 0xfc, 0xa0, 0xe4, 0x8d, 0x9d, 0x80, 0xc5, 0x02, 0x71, 0x15, 0xb4, 0x85, 0x78, 0x10, 0x51, 0xe0, 0xef, 0x46, 0xe4, 0xd6, 0x08, 0x31, 0x7a, 0x1c, 0x42, 0x61, 0xbc, 0x46]));
        assert_eq!(Bitcoin::RightRotate16.cmr(), Cmr::from_byte_array([0x48, 0x2e, 0xa7, 0xe1, 0x21, 0x45, 0x01, 0xd9, 0x3c, 0x9a, 0xd1, 0x6f, 0xa8, 0xb9, 0x7b, 0xf5, 0xb3, 0x84, 0xfc, 0x2b, 0x54, 0x78, 0x9b, 0x8c, 0xd9, 0xe7, 0x84, 0xcc, 0xd0, 0xeb, 0x9d, 0x57]));
        assert_eq!(Bitcoin::RightRotate32.cmr(), Cmr::from_byte_array([0x09, 0x41, 0xb6, 0xee, 0xea, 0x9a, 0xf8, 0x19, 0x5b, 0x02, 0x8a, 0xfc, 0x0b, 0xd2, 0xa5, 0x34, 0x21, 0x8b, 0xf9, 0x0d, 0x1a, 0x0e, 0x37, 0x3d, 0x74, 0x74, 0x18, 0x54, 0x0b, 0x72, 0x6d, 0x73]));
        assert_eq!(Bitcoin::RightRotate64.cmr(), Cmr::from_byte_array([0x44, 0x4d, 0xbb, 0xc3, 0xdd, 0x2a, 0x11, 0xa5, 0xc7, 0xb0, 0x43, 0x9f, 0xdb, 0xa9, 0x9a, 0xc7, 0x4a, 0x11, 0xb8, 0xee, 0xb2, 0xdb, 0x30, 0x1e, 0x24, 0x3e, 0xa8, 0x91, 0x22, 0x90, 0x71, 0x52]));
        assert_eq!(Bitcoin::RightRotate8.cmr(), Cmr::from_byte_array([0x72, 0x65, 0xa3, 0x0c, 0x2e, 0x83, 0x6e, 0x65, 0x54, 0x4a, 0xba, 0x91, 0x1b, 0x64, 0xd1, 0x8f, 0xa6, 0x9b, 0x17, 0x65, 0x45, 0x85, 0x6c, 0x77, 0xc4, 0xf0, 0xd7, 0x6f, 0xc3, 0xf5, 0x83, 0x51]));
        assert_eq!(Bitcoin::RightShift16.cmr(), Cmr::from_byte_array([0xcd, 0x57, 0xa3, 0xd3, 0xab, 0x2d, 0x92, 0xd4, 0xf0, 0x86, 0x55, 0x04, 0x3a, 0x8b, 0x8b, 0xb6, 0x73, 0x89, 0x81, 0xfa, 0xe6, 0xda, 0x01, 0x34, 0xb4, 0xde, 0xda, 0xce, 0x5f, 0x00, 0x88, 0x60]));
        assert_eq!(Bitcoin::RightShift32.cmr(), Cmr::from_byte_array([0xd6, 0xb3, 0x26, 0xb1, 0xa3, 0x23, 0x57, 0xa3, 0x32, 0x80, 0x7d, 0x3f, 0xa1, 0xb1, 0x56, 0xc2, 0x8b, 0x16, 0x22, 0xf7, 0x38, 0xde, 0xf1, 0x26, 0x81, 0x46, 0x7f, 0x34, 0x9b, 0xd3, 0x49, 0x4b]));
        assert_eq!(Bitcoin::RightShift64.cmr(), Cmr::from_byte_array([0xb2, 0x09, 0x5f, 0x2d, 0x47, 0x33, 0x5d, 0x5f, 0x98, 0xc8, 0x54, 0x34, 0xa2, 0xfa, 0xf5, 0xb0, 0xf7, 0x5c, 0xf8, 0x99, 0x01, 0x2a, 0x34, 0xbb, 0xcd, 0x0a, 0x14, 0xcb, 0xed, 0xb6, 0x11, 0x07]));
        assert_eq!(Bitcoin::RightShift8.cmr(), Cmr::from_byte_array([0x4b, 0x2b, 0x1a, 0xa2, 0xef, 0x73, 0x21, 0x73, 0x17, 0x0d, 0x62, 0x1a, 0x38, 0xde, 0xb2, 0x61, 0xe4, 0x73, 0xc0, 0x7c, 0x55, 0x8b, 0x05, 0x5a, 0x25, 0xa8, 0x6e, 0x4e, 0x32, 0x1a, 0xfc, 0x04]));
        assert_eq!(Bitcoin::RightShiftWith16.cmr(), Cmr::from_byte_array([0x14, 0xb7, 0x76, 0x85, 0x47, 0xb3, 0xd3, 0xf4, 0x7e, 0xe5, 0xc2, 0xb8, 0x0d, 0x9b, 0xda, 0xe2, 0xae, 0xc1, 0xf9, 0xc6, 0x59, 0x4e, 0xd3, 0x12, 0x7b, 0x12, 0x64, 0x5a, 0xdc, 0xf5, 0x97, 0x54]));
        assert_eq!(Bitcoin::RightShiftWith32.cmr(), Cmr::from_byte_array([0x32, 0x7b, 0x6e, 0x98, 0xa6, 0xfd, 0x34, 0x0c, 0x60, 0xcf, 0x83, 0xaa, 0x64, 0x99, 0x33, 0x11, 0x4c, 0xb8, 0xd8, 0x4f, 0x59, 0x0e, 0x01, 0x21, 0x3a, 0x26, 0x10, 0x01, 0x2b, 0x46, 0x07, 0xea]));
        assert_eq!(Bitcoin::RightShiftWith64.cmr(), Cmr::from_byte_array([0x06, 0x2f, 0xa7, 0x4a, 0xf3, 0x47, 0x6e, 0x59, 0x38, 0x7b, 0xe0, 0x8e, 0x69, 0x49, 0xa0, 0x05, 0x43, 0xbc, 0x84, 0xa2, 0xb6, 0x89, 0xea, 0x39, 0xad, 0x6e, 0xed, 0x7f, 0x75, 0x67, 0x85, 0xd4]));
        assert_eq!(Bitcoin::RightShiftWith8.cmr(), Cmr::from_byte_array([0x14, 0x1b, 0xe4, 0x7e, 0x96, 0x7b, 0x2f, 0xd7, 0xc7, 0x12, 0x6c, 0x5a, 0xdf, 0x2d, 0xfe, 0x47, 0x31, 0x5b, 0xbc, 0x10, 0x53, 0xbb, 0xe6, 0x05, 0xb3, 0x88, 0x98, 0xdb, 0xed, 0x49, 0xf2, 0x27]));
        assert_eq!(Bitcoin::ScalarAdd.cmr(), Cmr::from_byte_array([0x11, 0xdd, 0xbe, 0xba, 0xeb, 0xf4, 0x21, 0x80, 0xa0, 0xb7, 0xed, 0xdf, 0xfd, 0xc4, 0x8e, 0xc7, 0x51, 0x13, 0x30, 0xfb, 0x33, 0x15, 0xfa, 0x65, 0xd5, 0x8a, 0xff, 0x66, 0xb9, 0xca, 0xf2, 0xd4]));
        assert_eq!(Bitcoin::ScalarInvert.cmr(), Cmr::from_byte_array([0xa6, 0x39, 0x27, 0x25, 0xbb, 0x2d, 0xad, 0xbb, 0x1e, 0x76, 0xdf, 0x2d, 0xec, 0x57, 0xdf, 0x55, 0xc3, 0xfc, 0xc5, 0x77, 0x3b, 0x62, 0x21, 0x8a, 0xec, 0x55, 0xa7, 0x5e, 0x14, 0xf3, 0xd6, 0x0d]));
        assert_eq!(Bitcoin::ScalarIsZero.cmr(), Cmr::from_byte_array([0xf7, 0x5e, 0xda, 0x06, 0xce, 0x6a, 0xf0, 0x9f, 0xae, 0x37, 0xdb, 0x4e, 0x62, 0x25, 0xe6, 0xa8, 0xac, 0x86, 0xa2, 0x36, 0x37, 0x62, 0x7d, 0x62, 0x64, 0x09, 0x19, 0x0f, 0xf3, 0xb3, 0x9d, 0x90]));
        assert_eq!(Bitcoin::ScalarMultiply.cmr(), Cmr::from_byte_array([0x4a, 0x61, 0x67, 0x2a, 0xce, 0xc4, 0x88, 0x77, 0x56, 0xde, 0x1d, 0xb6, 0x04, 0x21, 0xa1, 0x2b, 0x90, 0x1a, 0x85, 0x8a, 0x6e, 0xe6, 0x35, 0x2e, 0x55, 0x9d, 0x4c, 0xe5, 0x97, 0x33, 0x52, 0xbe]));
        assert_eq!(Bitcoin::ScalarMultiplyLambda.cmr(), Cmr::from_byte_array([0x49, 0xea, 0x9c, 0x3f, 0xb1, 0xd8, 0xff, 0x52, 0xd2, 0xdb, 0x03, 0x46, 0x9f, 0xdf, 0xe8, 0x50, 0x50, 0x3f, 0xdd, 0xeb, 0x45, 0xe1, 0x6d, 0x26, 0xe8, 0x92, 0x8a, 0xdd, 0x25, 0x87, 0x0e, 0x91]));
        assert_eq!(Bitcoin::ScalarNegate.cmr(), Cmr::from_byte_array([0x1d, 0xbf, 0x8b, 0x49, 0x1e, 0xc6, 0x65, 0x80, 0x3f, 0x63, 0x33, 0x30, 0xd3, 0xff, 0xb0, 0xe7, 0x81, 0xe6, 0x7c, 0x18, 0x01, 0xac, 0x9d, 0x49, 0xbb, 0xf4, 0x35, 0x89, 0xab, 0xf7, 0x82, 0xbf]));
        assert_eq!(Bitcoin::ScalarNormalize.cmr(), Cmr::from_byte_array([0x46, 0x33, 0x18, 0x0e, 0xa0, 0x2c, 0x4d, 0xf7, 0x81, 0x9d, 0x3d, 0x54, 0xa4, 0x01, 0x73, 0x4f, 0x96, 0x5b, 0x31, 0xac, 0xc7, 0x84, 0x05, 0x4e, 0xbf, 0xb7, 0x31, 0x68, 0x16, 0xb0, 0x29, 0xec]));
        assert_eq!(Bitcoin::ScalarSquare.cmr(), Cmr::from_byte_array([0x8a, 0x27, 0x9e, 0x6f, 0x61, 0x3a, 0xa9, 0xe9, 0x34, 0xf2, 0xf2, 0xa3, 0x43, 0xc0, 0xd3, 0x29, 0x1c, 0x36, 0x70, 0xe2, 0x97, 0xdd, 0xae, 0x20, 0x52, 0x9e, 0x82, 0x50, 0x69, 0xef, 0xea, 0x0e]));
        assert_eq!(Bitcoin::Scale.cmr(), Cmr::from_byte_array([0x12, 0x6e, 0x22, 0x12, 0x5b, 0xac, 0x80, 0xb9, 0x9b, 0x7b, 0x73, 0x43, 0xb4, 0xe5, 0xe5, 0x86, 0x60, 0x82, 0x16, 0x10, 0x5d, 0x4d, 0xe6, 0xf7, 0x94, 0xad, 0xd3, 0x4e, 0x23, 0xb1, 0x95, 0xca]));
        assert_eq!(Bitcoin::ScriptCMR.cmr(), Cmr::from_byte_array([0xa8, 0xa4, 0xa6, 0x22, 0x10, 0xb5, 0xe4, 0x95, 0x0e, 0x25, 0x34, 0x24, 0x7c, 0x74, 0x11, 0xd1, 0xc8, 0xff, 0x22, 0x86, 0x5b, 0x54, 0x56, 0xbb, 0xb2, 0x16, 0x38, 0xe9, 0x14, 0xf5, 0xe5, 0x28]));
        assert_eq!(Bitcoin::Sha256Block.cmr(), Cmr::from_byte_array([0x45, 0x35, 0xf3, 0xe1, 0xab, 0x9f, 0x1b, 0x75, 0x7a, 0x06, 0x91, 0x37, 0xe1, 0xd5, 0xb1, 0xca, 0xad, 0x8e, 0x31, 0xf7, 0x8d, 0xc5, 0xfb, 0xd0, 0x73, 0x46, 0x49, 0xf9, 0x40, 0xa7, 0xfc, 0x96]));
        assert_eq!(Bitcoin::Sha256Ctx8Add1.cmr(), Cmr::from_byte_array([0x9a, 0x47, 0x11, 0xb8, 0xc5, 0x69, 0x0e, 0x58, 0x7e, 0x5f, 0x79, 0xe6, 0x8d, 0x6e, 0xca, 0x04, 0x74, 0x58, 0xaa, 0x63, 0xb8, 0xbc, 0x9e, 0xe5, 0x68, 0x08, 0x6a, 0x4a, 0x1b, 0x56, 0xd8, 0x34]));
        assert_eq!(Bitcoin::Sha256Ctx8Add128.cmr(), Cmr::from_byte_array([0x1c, 0xb1, 0xdb, 0x8a, 0x05, 0x5b, 0x31, 0x97, 0xac, 0xf0, 0xf0, 0x8c, 0xe9, 0xc6, 0x35, 0xad, 0xd6, 0x95, 0xb6, 0x0f, 0x23, 0x4b, 0x18, 0xe0, 0xb3, 0x23, 0xc9, 0x37, 0xb0, 0x38, 0x5a, 0xea]));
        assert_eq!(Bitcoin::Sha256Ctx8Add16.cmr(), Cmr::from_byte_array([0xe0, 0x84, 0x54, 0x75, 0xeb, 0xb9, 0x01, 0x40, 0xfa, 0x4e, 0x01, 0xaf, 0x8a, 0x94, 0x35, 0x99, 0x1a, 0xd8, 0x7a, 0xf9, 0x8c, 0x08, 0xae, 0xce, 0x11, 0x0e, 0x99, 0xcb, 0xce, 0xcd, 0xee, 0x79]));
        assert_eq!(Bitcoin::Sha256Ctx8Add2.cmr(), Cmr::from_byte_array([0x7d, 0x69, 0x13, 0x8f, 0x1c, 0x94, 0x2b, 0xee, 0x2f, 0xdf, 0x60, 0x0c, 0xe4, 0x4b, 0x36, 0xff, 0x97, 0x83, 0x9d, 0xc2, 0xbb, 0xda, 0xfb, 0xd5, 0xfa, 0xb4, 0xdf, 0xbc, 0x3c, 0x97, 0x6f, 0x29]));
        assert_eq!(Bitcoin::Sha256Ctx8Add256.cmr(), Cmr::from_byte_array([0x4f, 0x5c, 0x29, 0xd5, 0x36, 0x86, 0xc0, 0x60, 0x62, 0xb3, 0x83, 0x24, 0xf8, 0xaf, 0xf1, 0x7e, 0xc5, 0x56, 0xa2, 0x95, 0xff, 0x09, 0x8b, 0x10, 0xe7, 0x05, 0xdd, 0x22, 0xe1, 0x3b, 0xc3, 0xc9]));
        assert_eq!(Bitcoin::Sha256Ctx8Add32.cmr(), Cmr::from_byte_array([0xd5, 0x7b, 0x67, 0xb1, 0x74, 0xe7, 0x8e, 0x38, 0xf9, 0xbc, 0xa8, 0xe0, 0x7a, 0xdd, 0x61, 0xc7, 0x53, 0xe2, 0xc1, 0x56, 0xd8, 0xe9, 0x83, 0x2a, 0xa6, 0x62, 0x04, 0x55, 0x00, 0xf5, 0x1a, 0x80]));
        assert_eq!(Bitcoin::Sha256Ctx8Add4.cmr(), Cmr::from_byte_array([0x95, 0xda, 0x32, 0x99, 0x3f, 0x5c, 0x7d, 0x00, 0x83, 0x06, 0x4c, 0xdf, 0xf1, 0xbe, 0xc3, 0xb9, 0x36, 0xc6, 0x38, 0x33, 0x7a, 0xde, 0xc5, 0x47, 0x48, 0x7a, 0xf2, 0x32, 0xd6, 0x9f, 0xdf, 0x65]));
        assert_eq!(Bitcoin::Sha256Ctx8Add512.cmr(), Cmr::from_byte_array([0x4a, 0xcb, 0x16, 0x3a, 0xa4, 0x8f, 0x09, 0xd5, 0xf2, 0x6d, 0x2b, 0x2a, 0xb1, 0x88, 0xa6, 0xc6, 0xb6, 0xc4, 0xae, 0xdf, 0x23, 0xc9, 0x19, 0x00, 0x1c, 0x02, 0xee, 0x15, 0xb3, 0x37, 0xa9, 0x6e]));
        assert_eq!(Bitcoin::Sha256Ctx8Add64.cmr(), Cmr::from_byte_array([0x52, 0xe5, 0x3e, 0xc5, 0x77, 0x0f, 0x9b, 0xe4, 0x06, 0x9a, 0xee, 0xfc, 0xb2, 0x13, 0x22, 0xb1, 0x3a, 0xb6, 0xe3, 0x94, 0x1f, 0xdc, 0x2c, 0x85, 0xf4, 0xb4, 0x1b, 0xe6, 0x7d, 0x38, 0xea, 0x7e]));
        assert_eq!(Bitcoin::Sha256Ctx8Add8.cmr(), Cmr::from_byte_array([0xc2, 0x6b, 0x28, 0xaf, 0xe5, 0xe8, 0x66, 0xd8, 0x46, 0x16, 0x81, 0x4d, 0x1a, 0x13, 0xfb, 0x86, 0x30, 0xb9, 0xe8, 0x4e, 0x5d, 0x78, 0x15, 0x56, 0xc6, 0xd8, 0x23, 0x6e, 0xfb, 0x45, 0xdf, 0xf9]));
        assert_eq!(Bitcoin::Sha256Ctx8AddBuffer511.cmr(), Cmr::from_byte_array([0xad, 0x69, 0x90, 0x46, 0x48, 0xa8, 0x23, 0x8d, 0x00, 0xd8, 0x51, 0x63, 0xfc, 0xe8, 0x19, 0x63, 0xa0, 0x04, 0x7a, 0xb5, 0x82, 0xbe, 0x97, 0xa4, 0x14, 0x00, 0x65, 0x59, 0x79, 0xcf, 0xdd, 0x28]));
        assert_eq!(Bitcoin::Sha256Ctx8Finalize.cmr(), Cmr::from_byte_array([0x8e, 0x45, 0xbd, 0xc3, 0x87, 0xd4, 0xed, 0xfa, 0x73, 0x35, 0x25, 0xf3, 0xab, 0x19, 0xe4, 0x2b, 0x58, 0xec, 0xb1, 0xb5, 0xf6, 0xdc, 0xcf, 0x94, 0xed, 0xbf, 0x59, 0x95, 0x8a, 0xe3, 0xe1, 0x16]));
        assert_eq!(Bitcoin::Sha256Ctx8Init.cmr(), Cmr::from_byte_array([0x63, 0x5f, 0x64, 0x05, 0x84, 0x86, 0x85, 0xc0, 0x11, 0xfe, 0xbd, 0x41, 0xfa, 0xac, 0x87, 0x4b, 0xbb, 0xf5, 0xb2, 0x4d, 0x5f, 0xb1, 0x2f, 0xed, 0xbc, 0xb6, 0xcb, 0xff, 0x95, 0xa0, 0xf3, 0x66]));
        assert_eq!(Bitcoin::Sha256Iv.cmr(), Cmr::from_byte_array([0x12, 0xe4, 0x59, 0x37, 0x51, 0xc9, 0x46, 0x3b, 0x56, 0x25, 0x03, 0xc1, 0x40, 0xd7, 0x8b, 0x3b, 0x75, 0x7a, 0x1f, 0x4f, 0x16, 0x32, 0x1d, 0x28, 0x62, 0xd3, 0x25, 0x43, 0x85, 0x38, 0x97, 0x1b]));
        assert_eq!(Bitcoin::SigAllHash.cmr(), Cmr::from_byte_array([0x09, 0x78, 0xb9, 0xe5, 0x0b, 0x9e, 0x8e, 0x09, 0x8b, 0x27, 0xf2, 0xb8, 0xb5, 0x9d, 0xe5, 0x4f, 0x62, 0xba, 0x7c, 0x13, 0x33, 0xdf, 0x3b, 0xed, 0x22, 0x1e, 0x26, 0x62, 0x68, 0x05, 0xbc, 0x55]));
        assert_eq!(Bitcoin::Some1.cmr(), Cmr::from_byte_array([0x15, 0xca, 0x4e, 0x4b, 0x82, 0xc2, 0xf9, 0x1b, 0x9a, 0x79, 0x29, 0x92, 0xcd, 0xc1, 0xb2, 0x92, 0xab, 0x86, 0xa2, 0xd2, 0x93, 0x9c, 0x9a, 0x64, 0xb5, 0x0b, 0xe6, 0x0b, 0xda, 0x6a, 0xb4, 0xca]));
        assert_eq!(Bitcoin::Some16.cmr(), Cmr::from_byte_array([0xa9, 0xdf, 0xbb, 0xea, 0xb5, 0x9d, 0xf7, 0x2a, 0x45, 0xfc, 0x3f, 0xc7, 0xac, 0x58, 0x1e, 0xc8, 0xda, 0x71, 0x3f, 0x2f, 0x81, 0x03, 0xf7, 0x87, 0xaa, 0x1c, 0xee, 0x4e, 0x0b, 0xa6, 0x48, 0x66]));
        assert_eq!(Bitcoin::Some32.cmr(), Cmr::from_byte_array([0x46, 0x33, 0xa3, 0x97, 0x74, 0x2e, 0xf4, 0x82, 0xbe, 0x2f, 0xa3, 0xfb, 0x64, 0x10, 0xec, 0x79, 0xc3, 0x73, 0x83, 0x65, 0x69, 0xfb, 0xbc, 0xb1, 0xf9, 0x48, 0xec, 0x32, 0x48, 0x73, 0x78, 0xb7]));
        assert_eq!(Bitcoin::Some64.cmr(), Cmr::from_byte_array([0x1d, 0xc2, 0x45, 0xac, 0x6f, 0x5b, 0x42, 0x2b, 0xd1, 0x88, 0x6e, 0xf5, 0x14, 0x4c, 0x4d, 0xc7, 0x2c, 0x96, 0x73, 0x15, 0x59, 0x66, 0x07, 0x6c, 0xd8, 0x39, 0x68, 0x1d, 0x9e, 0xc7, 0xf8, 0xf5]));
        assert_eq!(Bitcoin::Some8.cmr(), Cmr::from_byte_array([0x33, 0xaf, 0xb9, 0xc6, 0x45, 0x4e, 0x59, 0x0e, 0xc1, 0x3e, 0xd7, 0x5e, 0x1b, 0x7d, 0x9c, 0x3a, 0x3d, 0xe6, 0x75, 0x2b, 0xcc, 0x7c, 0x1d, 0x4c, 0xb3, 0x63, 0xfa, 0x51, 0x82, 0x8b, 0xcb, 0x74]));
        assert_eq!(Bitcoin::Subtract16.cmr(), Cmr::from_byte_array([0x4e, 0x06, 0xec, 0x31, 0x37, 0x62, 0x22, 0xe2, 0x5e, 0x27, 0xd0, 0x15, 0x9d, 0xc1, 0xc0, 0x71, 0x4a, 0x44, 0xca, 0x6a, 0xac, 0xf9, 0x50, 0x5c, 0xaa, 0xd2, 0x80, 0xe9, 0x73, 0xfb, 0x5c, 0xab]));
        assert_eq!(Bitcoin::Subtract32.cmr(), Cmr::from_byte_array([0xb9, 0xc0, 0xf3, 0x6e, 0x75, 0x22, 0xa8, 0xd9, 0x49, 0x05, 0x0d, 0x51, 0x6a, 0x05, 0xce, 0x20, 0x3a, 0x1f, 0x9a, 0x9e, 0x37, 0x2f, 0xd2, 0x63, 0xde, 0x38, 0xb0, 0xe9, 0x03, 0x13, 0x41, 0x98]));
        assert_eq!(Bitcoin::Subtract64.cmr(), Cmr::from_byte_array([0x1c, 0xdb, 0x5c, 0x74, 0xad, 0xd1, 0x02, 0xf5, 0x0f, 0x93, 0x8e, 0xd8, 0x86, 0xf4, 0x96, 0xe5, 0xba, 0xb2, 0x75, 0x5c, 0x3c, 0x48, 0x4e, 0x88, 0x87, 0x90, 0x3d, 0x2f, 0x6a, 0x57, 0xf3, 0xaa]));
        assert_eq!(Bitcoin::Subtract8.cmr(), Cmr::from_byte_array([0x4f, 0x21, 0x17, 0xa0, 0xe8, 0x10, 0x59, 0xff, 0x0c, 0xd6, 0x4d, 0x84, 0x88, 0x65, 0x42, 0xe5, 0x75, 0xea, 0x8d, 0x6e, 0xc0, 0x31, 0x08, 0xfd, 0x0b, 0x50, 0x8b, 0x39, 0x20, 0x8c, 0xd0, 0xef]));
        assert_eq!(Bitcoin::Swu.cmr(), Cmr::from_byte_array([0x00, 0xf5, 0x1f, 0x4f, 0x4b, 0xec, 0xe7, 0x90, 0x03, 0xec, 0xad, 0x48, 0x1a, 0x12, 0x5a, 0xf7, 0x17, 0x6e, 0x4d, 0xe9, 0x8c, 0x33, 0x92, 0x42, 0x5c, 0xb9, 0x14, 0x66, 0x26, 0xc1, 0x3b, 0x3b]));
        assert_eq!(Bitcoin::TapdataInit.cmr(), Cmr::from_byte_array([0xa4, 0xd0, 0x22, 0xef, 0x5c, 0xf4, 0x67, 0xbc, 0xa0, 0x32, 0x5e, 0x46, 0x3f, 0xca, 0xce, 0x7c, 0xbd, 0xd6, 0x4f, 0xf8, 0xf7, 0x1c, 0x5c, 0x7f, 0x63, 0xe6, 0x07, 0x84, 0xaa, 0x0a, 0xc4, 0x86]));
        assert_eq!(Bitcoin::TapleafHash.cmr(), Cmr::from_byte_array([0x0c, 0x07, 0x16, 0xfe, 0x5d, 0x97, 0x8e, 0xa8, 0xe0, 0xc7, 0x5a, 0xdc, 0x82, 0x10, 0xd6, 0x60, 0x06, 0x2e, 0x3d, 0xa0, 0x6f, 0x1a, 0x66, 0x61, 0x31, 0x79, 0x27, 0xd3, 0xb8, 0x4b, 0x50, 0x73]));
        assert_eq!(Bitcoin::TapleafVersion.cmr(), Cmr::from_byte_array([0xe2, 0xec, 0xb1, 0xcb, 0x0e, 0xd4, 0xed, 0x93, 0x48, 0x35, 0x45, 0xfd, 0x8a, 0x62, 0xf8, 0xaa, 0x10, 0x17, 0x53, 0x49, 0xff, 0xcc, 0x5a, 0xd3, 0xde, 0x7f, 0x34, 0x84, 0xea, 0x1f, 0x10, 0x3f]));
        assert_eq!(Bitcoin::Tappath.cmr(), Cmr::from_byte_array([0x99, 0xe8, 0x21, 0x1e, 0x8c, 0x1b, 0xe6, 0xd9, 0xca, 0x98, 0xd3, 0xd4, 0x3d, 0x91, 0x42, 0x56, 0x7e, 0x06, 0xa8, 0x40, 0x22, 0x39, 0x33, 0x03, 0xfa, 0xb0, 0xb5, 0x7d, 0x39, 0x40, 0x74, 0xe8]));
        assert_eq!(Bitcoin::TappathHash.cmr(), Cmr::from_byte_array([0x02, 0x11, 0x54, 0x6d, 0x07, 0x78, 0xe7, 0x87, 0x14, 0x1e, 0xce, 0x65, 0xdf, 0x6c, 0xd1, 0xdb, 0x31, 0x38, 0x8f, 0xc1, 0x42, 0x19, 0x68, 0xc8, 0xcf, 0xd8, 0xd7, 0x59, 0x26, 0xdb, 0x47, 0xb3]));
        assert_eq!(Bitcoin::TapEnvHash.cmr(), Cmr::from_byte_array([0x19, 0xd9, 0x94, 0x4c, 0x4d, 0x45, 0x7c, 0x70, 0xba, 0xbc, 0x45, 0xcc, 0xcb, 0xd5, 0x73, 0xb3, 0x9a, 0x51, 0xd0, 0xc9, 0x91, 0x15, 0xb4, 0x12, 0x78, 0x3b, 0x49, 0x00, 0x82, 0xfd, 0x0e, 0x58]));
        assert_eq!(Bitcoin::TotalInputValue.cmr(), Cmr::from_byte_array([0x81, 0x28, 0x7a, 0x15, 0x89, 0xdd, 0xc2, 0x17, 0x99, 0x26, 0xb7, 0x06, 0x5e, 0x9b, 0x26, 0x77, 0xb7, 0xfb, 0x09, 0x9b, 0x95, 0xf9, 0x47, 0xa7, 0xdd, 0x59, 0x0c, 0xdd, 0x4c, 0xcf, 0x4a, 0x56]));
        assert_eq!(Bitcoin::TotalOutputValue.cmr(), Cmr::from_byte_array([0xba, 0x03, 0x2f, 0x3e, 0x62, 0xf8, 0xfc, 0xb0, 0x4b, 0x04, 0x29, 0xa2, 0x53, 0xb5, 0xec, 0x57, 0xe4, 0xc8, 0x7a, 0xe2, 0xe9, 0x51, 0xe1, 0x45, 0xd7, 0x65, 0x0e, 0x7f, 0x0c, 0x63, 0xe5, 0x55]));
        assert_eq!(Bitcoin::TransactionId.cmr(), Cmr::from_byte_array([0x5c, 0x49, 0xea, 0x98, 0x6b, 0x32, 0x8b, 0xe2, 0xa4, 0xe0, 0xb3, 0x15, 0xb6, 0xb3, 0xfe, 0xf2, 0x3c, 0x1f, 0x68, 0x56, 0xbb, 0xc3, 0xb0, 0x56, 0xc9, 0x9c, 0xf5, 0x9f, 0xbd, 0x54, 0x6e, 0x65]));
        assert_eq!(Bitcoin::TxHash.cmr(), Cmr::from_byte_array([0x54, 0xe5, 0x3c, 0x99, 0x93, 0xab, 0xd5, 0x5d, 0x1f, 0x85, 0x23, 0xd2, 0xbb, 0x21, 0x7b, 0x32, 0xe6, 0xfe, 0x86, 0x1f, 0x84, 0xc9, 0x86, 0xb7, 0xee, 0x8b, 0xdc, 0x68, 0x81, 0x06, 0x87, 0x4a]));
        assert_eq!(Bitcoin::TxIsFinal.cmr(), Cmr::from_byte_array([0x7b, 0x0e, 0x4f, 0x4c, 0xa8, 0xe5, 0xaf, 0x61, 0xa1, 0xd3, 0x45, 0x4e, 0x11, 0xef, 0x9a, 0xb6, 0x88, 0x70, 0x61, 0x21, 0x1c, 0x00, 0x90, 0xeb, 0xa9, 0x55, 0x3d, 0xa2, 0xe4, 0x5d, 0x84, 0x73]));
        assert_eq!(Bitcoin::TxLockDistance.cmr(), Cmr::from_byte_array([0xb6, 0xfb, 0xaa, 0xc2, 0x10, 0xa3, 0x06, 0xbe, 0x8b, 0x58, 0xd0, 0xd7, 0xb1, 0x56, 0x3f, 0x62, 0x23, 0x36, 0xd2, 0xae, 0xb5, 0x6c, 0x39, 0x3d, 0x27, 0x64, 0x45, 0xa9, 0xa2, 0x2c, 0xc4, 0xa7]));
        assert_eq!(Bitcoin::TxLockDuration.cmr(), Cmr::from_byte_array([0x53, 0x57, 0x28, 0x18, 0xe7, 0xe5, 0xb9, 0x8f, 0x96, 0x8c, 0x9d, 0xa7, 0xdf, 0x50, 0x90, 0xd9, 0x82, 0x6f, 0x9b, 0xcf, 0x84, 0xb6, 0x36, 0x39, 0x5e, 0xea, 0x32, 0x1b, 0x69, 0x09, 0xec, 0xe9]));
        assert_eq!(Bitcoin::TxLockHeight.cmr(), Cmr::from_byte_array([0x44, 0x9f, 0x61, 0xdf, 0x1a, 0x7b, 0xad, 0x8d, 0x9e, 0x0a, 0x96, 0x67, 0x14, 0x22, 0x7e, 0x57, 0x10, 0x07, 0xf9, 0x3c, 0x51, 0x7b, 0x80, 0x5d, 0x61, 0x65, 0x10, 0xe8, 0xff, 0x62, 0x17, 0x2b]));
        assert_eq!(Bitcoin::TxLockTime.cmr(), Cmr::from_byte_array([0x31, 0xdf, 0x36, 0x3f, 0x17, 0xb2, 0xcf, 0xc9, 0x7a, 0x1f, 0x93, 0x72, 0xc1, 0x4a, 0x32, 0x58, 0x64, 0xd7, 0xcb, 0x13, 0xaa, 0x8d, 0x52, 0x15, 0xfb, 0x25, 0x3d, 0x33, 0x10, 0x91, 0x77, 0x62]));
        assert_eq!(Bitcoin::Verify.cmr(), Cmr::from_byte_array([0xcd, 0xca, 0x2a, 0x05, 0xe5, 0x2c, 0xef, 0xa5, 0x9d, 0xc7, 0xa5, 0xb0, 0xda, 0xe2, 0x20, 0x98, 0xfb, 0x89, 0x6e, 0x39, 0x13, 0xbf, 0xdd, 0x44, 0x6b, 0x59, 0x4e, 0x1f, 0x92, 0x50, 0x78, 0x3e]));
        assert_eq!(Bitcoin::Version.cmr(), Cmr::from_byte_array([0x83, 0x73, 0x58, 0x64, 0x00, 0xb6, 0x79, 0x0b, 0x46, 0xab, 0x04, 0x10, 0x52, 0x3c, 0xf0, 0x1e, 0xb7, 0x4d, 0x10, 0xfa, 0xf4, 0x8a, 0x3a, 0xcc, 0x86, 0xc4, 0xc5, 0x1d, 0x06, 0xa5, 0x2c, 0x49]));
        assert_eq!(Bitcoin::Xor1.cmr(), Cmr::from_byte_array([0x8c, 0x4e, 0x4e, 0x6e, 0xbf, 0x46, 0x30, 0xb2, 0x9b, 0x5a, 0x57, 0xea, 0x79, 0xf0, 0xc9, 0xaf, 0x6b, 0xff, 0x54, 0xc4, 0xd2, 0xd7, 0x69, 0xbf, 0x51, 0x59, 0x47, 0x74, 0xa5, 0x2b, 0x99, 0xc9]));
        assert_eq!(Bitcoin::Xor16.cmr(), Cmr::from_byte_array([0xd9, 0xf0, 0xaf, 0x3f, 0xe3, 0xfd, 0x24, 0x7c, 0x1d, 0xf3, 0x4a, 0x25, 0x27, 0x13, 0xb2, 0xe9, 0x33, 0xa9, 0x45, 0xa5, 0x67, 0x19, 0x48, 0x7f, 0x8e, 0xd7, 0xf5, 0x63, 0xea, 0x86, 0x1a, 0xb5]));
        assert_eq!(Bitcoin::Xor32.cmr(), Cmr::from_byte_array([0xd5, 0xae, 0x27, 0x12, 0xed, 0xea, 0xf6, 0x76, 0x52, 0x0f, 0xa3, 0xba, 0x0f, 0x40, 0xbf, 0x4a, 0x16, 0x57, 0x43, 0x7e, 0xff, 0xbd, 0x99, 0x86, 0xd0, 0x6a, 0xe8, 0x1b, 0x29, 0xa4, 0xf9, 0x8c]));
        assert_eq!(Bitcoin::Xor64.cmr(), Cmr::from_byte_array([0xc4, 0xdf, 0x1c, 0xcf, 0x33, 0x3e, 0xde, 0xbd, 0xd4, 0x0d, 0xea, 0x9a, 0x0e, 0x6c, 0xbb, 0x83, 0x06, 0x31, 0xe8, 0x3a, 0x94, 0xbb, 0x77, 0x9f, 0xe6, 0x00, 0x7b, 0xc6, 0xcb, 0x53, 0xa5, 0x44]));
        assert_eq!(Bitcoin::Xor8.cmr(), Cmr::from_byte_array([0x4a, 0xb1, 0x4a, 0x81, 0x4a, 0x39, 0x52, 0x8a, 0x80, 0xfd, 0xb4, 0x30, 0x58, 0x9b, 0xa4, 0x50, 0x10, 0x4b, 0x9c, 0x72, 0x09, 0xaa, 0x2f, 0xe2, 0x85, 0xcd, 0x60, 0xc0, 0x90, 0x43, 0x11, 0x4a]));
        assert_eq!(Bitcoin::XorXor1.cmr(), Cmr::from_byte_array([0x18, 0xb9, 0x44, 0x6a, 0x41, 0x66, 0xa3, 0xfe, 0xe2, 0xbc, 0xb2, 0x54, 0x5b, 0xb9, 0x01, 0x18, 0xdc, 0xf0, 0xe8, 0xf8, 0x86, 0xa1, 0x07, 0x6d, 0x4c, 0x38, 0x60, 0x06, 0x0c, 0xde, 0x1a, 0x51]));
        assert_eq!(Bitcoin::XorXor16.cmr(), Cmr::from_byte_array([0x94, 0x6c, 0xde, 0x87, 0x2e, 0x30, 0xe6, 0x50, 0x9d, 0xaf, 0xf4, 0x05, 0xf0, 0xe0, 0xfe, 0xfe, 0x27, 0x55, 0x47, 0xb4, 0x0e, 0xb2, 0x03, 0x84, 0xaf, 0xe9, 0xa8, 0x63, 0x60, 0xfc, 0x80, 0xef]));
        assert_eq!(Bitcoin::XorXor32.cmr(), Cmr::from_byte_array([0x65, 0x27, 0xdf, 0x67, 0xa5, 0x0d, 0x14, 0x8d, 0xb4, 0xfc, 0x8f, 0xee, 0xc7, 0x84, 0x55, 0x64, 0x99, 0xa8, 0xc7, 0xf0, 0xfa, 0x7d, 0x28, 0xe6, 0x27, 0x8e, 0x99, 0x7f, 0x49, 0x59, 0xbe, 0x39]));
        assert_eq!(Bitcoin::XorXor64.cmr(), Cmr::from_byte_array([0xf1, 0x62, 0xf9, 0xe6, 0x56, 0x63, 0xa6, 0x9a, 0xc5, 0xf9, 0x2a, 0x5e, 0xb5, 0x2c, 0x03, 0x32, 0x39, 0x2e, 0xdd, 0x1e, 0xd1, 0xba, 0x35, 0x5e, 0x6f, 0x19, 0x40, 0x6e, 0xab, 0xe3, 0xf6, 0xed]));
        assert_eq!(Bitcoin::XorXor8.cmr(), Cmr::from_byte_array([0xe0, 0x6d, 0x69, 0x4c, 0x5b, 0x40, 0x7d, 0xda, 0xd7, 0xaa, 0x1f, 0x88, 0x07, 0x16, 0xbc, 0xb7, 0x0a, 0xcd, 0xba, 0x75, 0x85, 0xca, 0x40, 0x09, 0x9a, 0x0a, 0x0a, 0x61, 0xf3, 0xad, 0x2d, 0xb5]));
    }

    #[test]
    fn cost_matches_spec() {
        assert_eq!(Bitcoin::Add16.cost(), Cost::from_milliweight(80));
        assert_eq!(Bitcoin::AnnexHash.cost(), Cost::from_milliweight(1491));
        assert_eq!(Bitcoin::Bip0340Verify.cost(), Cost::from_milliweight(49421));
        assert_eq!(Bitcoin::CheckSigVerify.cost(), Cost::from_milliweight(50000));
        assert_eq!(Bitcoin::Sha256Block.cost(), Cost::from_milliweight(765));
        assert_eq!(Bitcoin::Verify.cost(), Cost::from_milliweight(44));
        assert_eq!(Bitcoin::FullLeftShift64_32.cost(), Cost::from_milliweight(73));
        assert_eq!(Bitcoin::ScalarMultiply.cost(), Cost::from_milliweight(793));
    }
}
