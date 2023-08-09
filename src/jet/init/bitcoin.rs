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
    TapleafVersion,
    Tappath,
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
        unimplemented!("Bitcoin jet CMRs weights have not yet been implemented.")
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
            Bitcoin::TapleafVersion => b"1",
            Bitcoin::Tappath => b"***22*22**22*22",
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
            Bitcoin::TapleafVersion => b"***22*22**22*22",
            Bitcoin::Tappath => b"+1h",
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
            Bitcoin::Tappath => (28425, 15),
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
                                                                    1 => {Bitcoin::Tappath}
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
            Bitcoin::TapleafVersion => f.write_str("tapleaf_version"),
            Bitcoin::Tappath => f.write_str("tappath"),
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
            "tapleaf_version" => Ok(Bitcoin::TapleafVersion),
            "tappath" => Ok(Bitcoin::Tappath),
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
