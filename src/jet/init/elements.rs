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
use crate::jet::elements::ElementsEnv;
use simplicity_sys::CElementsTxEnv;

/// Elements jet family
#[derive(Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Debug, Hash)]
pub enum Elements {
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
    AssetAmountHash,
    Bip0340Verify,
    BuildTapbranch,
    BuildTapleafSimplicity,
    CalculateAsset,
    CalculateConfidentialToken,
    CalculateExplicitToken,
    CalculateIssuanceEntropy,
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
    CurrentAmount,
    CurrentAnnexHash,
    CurrentAsset,
    CurrentIndex,
    CurrentIssuanceAssetAmount,
    CurrentIssuanceAssetProof,
    CurrentIssuanceTokenAmount,
    CurrentIssuanceTokenProof,
    CurrentNewIssuanceContract,
    CurrentPegin,
    CurrentPrevOutpoint,
    CurrentReissuanceBlinding,
    CurrentReissuanceEntropy,
    CurrentScriptHash,
    CurrentScriptSigHash,
    CurrentSequence,
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
    GenesisBlockHash,
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
    InputAmount,
    InputAmountsHash,
    InputAnnexHash,
    InputAnnexesHash,
    InputAsset,
    InputHash,
    InputOutpointsHash,
    InputPegin,
    InputPrevOutpoint,
    InputScriptHash,
    InputScriptSigHash,
    InputScriptSigsHash,
    InputScriptsHash,
    InputSequence,
    InputSequencesHash,
    InputUtxoHash,
    InputUtxosHash,
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
    Issuance,
    IssuanceAsset,
    IssuanceAssetAmount,
    IssuanceAssetAmountsHash,
    IssuanceAssetProof,
    IssuanceBlindingEntropyHash,
    IssuanceEntropy,
    IssuanceHash,
    IssuanceRangeProofsHash,
    IssuanceToken,
    IssuanceTokenAmount,
    IssuanceTokenAmountsHash,
    IssuanceTokenProof,
    IssuancesHash,
    LbtcAsset,
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
    NewIssuanceContract,
    NonceHash,
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
    OutputAmount,
    OutputAmountsHash,
    OutputAsset,
    OutputHash,
    OutputIsFee,
    OutputNonce,
    OutputNoncesHash,
    OutputNullDatum,
    OutputRangeProof,
    OutputRangeProofsHash,
    OutputScriptHash,
    OutputScriptsHash,
    OutputSurjectionProof,
    OutputSurjectionProofsHash,
    OutputsHash,
    ParseLock,
    ParseSequence,
    PointVerify1,
    ReissuanceBlinding,
    ReissuanceEntropy,
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
    TapleafHash,
    TapleafVersion,
    Tappath,
    TappathHash,
    TotalFee,
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

impl Jet for Elements {

    type Environment = ElementsEnv<std::sync::Arc<elements::Transaction>>;
    type CJetEnvironment = CElementsTxEnv;

    fn c_jet_env(env: &Self::Environment) -> &Self::CJetEnvironment {
        env.c_tx_env()
    }

    fn cmr(&self) -> Cmr {
        let bytes = match self {
            Elements::Add16 => [
                0x90, 0xe5, 0xc3, 0x02, 0xc2, 0xa1, 0x02, 0xf7, 0x6c, 0xc2, 0x0b, 0xcd, 0xdd, 0xc0,
                0x62, 0x2b, 0xf3, 0xb8, 0xa5, 0x44, 0x6a, 0xb6, 0x8d, 0x1b, 0x2a, 0x03, 0x39, 0x5e,
                0x6f, 0xae, 0x92, 0x50,
            ],
            Elements::Add32 => [
                0x6d, 0x78, 0x30, 0xd6, 0x5e, 0x69, 0xf2, 0x7d, 0x55, 0x1c, 0x4b, 0xb3, 0x71, 0x8d,
                0xa9, 0x35, 0x22, 0x2f, 0xae, 0xc4, 0xaf, 0x7f, 0x49, 0xe9, 0x5e, 0x98, 0x33, 0xed,
                0x2f, 0x84, 0xc1, 0x06,
            ],
            Elements::Add64 => [
                0x9e, 0xee, 0xbe, 0xa8, 0x47, 0x38, 0x8e, 0x14, 0xc7, 0xc9, 0xe7, 0xca, 0x14, 0x7c,
                0x70, 0x71, 0x4d, 0xe1, 0x44, 0xeb, 0xab, 0x39, 0x8a, 0x53, 0xf4, 0xf5, 0x8a, 0x34,
                0x93, 0x0c, 0xc4, 0x5c,
            ],
            Elements::Add8 => [
                0x37, 0x59, 0xb0, 0x40, 0x6a, 0xa2, 0xc4, 0xe1, 0x15, 0x26, 0x21, 0xe0, 0x11, 0xe4,
                0xef, 0xdb, 0x55, 0x5c, 0x6a, 0xb6, 0xd9, 0x1a, 0xe9, 0xeb, 0xf1, 0xd6, 0x31, 0x5b,
                0x43, 0x82, 0xa6, 0xc4,
            ],
            Elements::All16 => [
                0x3e, 0x7e, 0x95, 0x3d, 0x28, 0xa6, 0xdc, 0x5e, 0xd7, 0xdf, 0xbd, 0x4c, 0x63, 0xf2,
                0xe0, 0x80, 0x47, 0xc7, 0x1b, 0xb9, 0xc4, 0x37, 0xe8, 0xc7, 0x1e, 0xe0, 0x14, 0x7a,
                0x60, 0xf7, 0x8b, 0xee,
            ],
            Elements::All32 => [
                0x5d, 0xd0, 0x85, 0x16, 0x26, 0x03, 0x6c, 0xb6, 0xa4, 0x2a, 0xa9, 0x52, 0x93, 0xdb,
                0xb8, 0xf3, 0x6e, 0xfe, 0x5e, 0x0e, 0xa5, 0x7a, 0x31, 0x08, 0x27, 0xfa, 0x6d, 0x79,
                0x2f, 0x46, 0xe0, 0xc1,
            ],
            Elements::All64 => [
                0xba, 0xd0, 0x2f, 0xd1, 0x58, 0x90, 0x8a, 0x1b, 0xe0, 0x68, 0x72, 0x5e, 0xb0, 0x52,
                0x76, 0xde, 0xfa, 0x41, 0x12, 0x07, 0xd0, 0xc8, 0xa9, 0x1b, 0xbe, 0x6f, 0xa5, 0x7d,
                0x2c, 0xda, 0x29, 0xdf,
            ],
            Elements::All8 => [
                0xdc, 0x09, 0x88, 0x54, 0xdd, 0x06, 0xae, 0x1e, 0x6e, 0x3e, 0x73, 0xa4, 0xae, 0x94,
                0xd0, 0xb2, 0xac, 0xce, 0x5c, 0xb3, 0xec, 0xc1, 0x2e, 0x8c, 0xb8, 0x16, 0x7f, 0x7b,
                0x6e, 0xaa, 0x40, 0x69,
            ],
            Elements::And1 => [
                0x2d, 0xc2, 0xdb, 0xcc, 0x69, 0xc1, 0x2c, 0x78, 0x30, 0xdf, 0x11, 0x70, 0xd9, 0xe9,
                0x3a, 0x35, 0xd8, 0x28, 0x4c, 0xc8, 0x15, 0x91, 0x6a, 0xeb, 0x3b, 0x1b, 0x95, 0xef,
                0xda, 0xa9, 0x2c, 0x26,
            ],
            Elements::And16 => [
                0x4f, 0x42, 0xc6, 0x88, 0xcc, 0x3c, 0xeb, 0x76, 0x1f, 0x66, 0x2a, 0x0b, 0x8e, 0x77,
                0xb1, 0x96, 0x42, 0xda, 0xb5, 0x40, 0x7d, 0x9a, 0x87, 0x3c, 0xd9, 0xc1, 0x86, 0x09,
                0x97, 0x7e, 0xbd, 0x91,
            ],
            Elements::And32 => [
                0x16, 0x80, 0xcf, 0xd2, 0x64, 0x3a, 0xd3, 0x89, 0x6b, 0xfb, 0x45, 0xe9, 0x6f, 0xc6,
                0x2d, 0xc2, 0xbb, 0x0c, 0xc0, 0xac, 0x48, 0x6f, 0xa8, 0x33, 0x30, 0xa8, 0x89, 0x0c,
                0x09, 0xee, 0x96, 0x9b,
            ],
            Elements::And64 => [
                0xa2, 0xdd, 0x3d, 0x49, 0xf1, 0x2c, 0xd4, 0x42, 0xce, 0xa7, 0xc8, 0x32, 0x38, 0x68,
                0xbe, 0x3c, 0xe6, 0xe9, 0xaf, 0x67, 0x6f, 0x72, 0x51, 0x10, 0xf3, 0xce, 0x49, 0x4e,
                0xdd, 0x6a, 0x5a, 0x4a,
            ],
            Elements::And8 => [
                0x27, 0x55, 0x41, 0x78, 0x19, 0x86, 0x58, 0xf5, 0xbf, 0xb5, 0xb0, 0x41, 0x30, 0x89,
                0x87, 0x42, 0xbf, 0x82, 0xdb, 0x60, 0x39, 0xdc, 0xbb, 0x17, 0x26, 0x6d, 0xfe, 0x03,
                0x0c, 0xc1, 0x20, 0x75,
            ],
            Elements::AnnexHash => [
                0xca, 0x1b, 0xed, 0xa0, 0x4f, 0x1f, 0xe8, 0xcb, 0xb0, 0x8c, 0xd6, 0x9f, 0x39, 0x59,
                0xd4, 0x42, 0xfe, 0x5a, 0xb0, 0x15, 0xc6, 0x92, 0x3e, 0x17, 0x1d, 0x8d, 0x06, 0x86,
                0xef, 0x31, 0x48, 0x8f,
            ],
            Elements::AssetAmountHash => [
                0x4f, 0xf7, 0xe7, 0x3c, 0xe7, 0xfb, 0xbf, 0xa1, 0xc5, 0x77, 0xe6, 0xfe, 0x1d, 0xab,
                0x70, 0xb8, 0xf1, 0x0b, 0xb4, 0xdf, 0x31, 0x9c, 0x0f, 0x6d, 0x67, 0x99, 0x4f, 0x6f,
                0xdb, 0x22, 0x42, 0x58,
            ],
            Elements::Bip0340Verify => [
                0x3a, 0x19, 0x6e, 0xc1, 0x9d, 0x8f, 0x3f, 0xcc, 0x67, 0xb0, 0xad, 0x8f, 0x61, 0x1e,
                0x76, 0x2d, 0xc1, 0x18, 0x9a, 0x72, 0x2f, 0xaa, 0x74, 0x5b, 0x69, 0x4d, 0x29, 0x2c,
                0x47, 0x37, 0x3f, 0x53,
            ],
            Elements::BuildTapbranch => [
                0xd4, 0xb3, 0x13, 0x12, 0x78, 0x6c, 0xb3, 0x3b, 0x43, 0xb0, 0x62, 0xd4, 0x8e, 0x0b,
                0xe0, 0x39, 0xc7, 0xde, 0x2b, 0x8d, 0x67, 0xce, 0x9e, 0xea, 0x5f, 0x52, 0x25, 0xf1,
                0x97, 0xef, 0x77, 0x74,
            ],
            Elements::BuildTapleafSimplicity => [
                0x13, 0x6f, 0x16, 0xe4, 0x4a, 0xe5, 0x5b, 0x7e, 0x63, 0x30, 0xd6, 0x56, 0xa2, 0x82,
                0xcb, 0xe5, 0xaf, 0x96, 0x6e, 0x74, 0x1f, 0x57, 0x6c, 0x9d, 0x0a, 0xcd, 0x77, 0x49,
                0x46, 0x7f, 0xd5, 0x05,
            ],
            Elements::CalculateAsset => [
                0x2e, 0xaa, 0xe2, 0x4d, 0xcc, 0xce, 0x3c, 0x94, 0x4b, 0xeb, 0x8e, 0x38, 0xac, 0x6a,
                0x52, 0xea, 0x43, 0x44, 0x20, 0xfb, 0x56, 0xa9, 0xa9, 0xb4, 0xfb, 0x84, 0x15, 0x83,
                0x48, 0x08, 0x74, 0x21,
            ],
            Elements::CalculateConfidentialToken => [
                0x3c, 0x5b, 0xf0, 0x8a, 0xba, 0x5a, 0xa3, 0x3e, 0xba, 0x87, 0xef, 0xb0, 0x99, 0xbd,
                0xf1, 0xc4, 0x64, 0xe7, 0x72, 0x2a, 0xec, 0xe1, 0xa2, 0x21, 0x3d, 0xc8, 0xeb, 0x25,
                0x54, 0xf0, 0xb5, 0x6b,
            ],
            Elements::CalculateExplicitToken => [
                0xbe, 0xf4, 0x4e, 0x2e, 0x52, 0xcc, 0xbf, 0xba, 0xc8, 0x8f, 0xd4, 0xd0, 0xbd, 0xad,
                0x51, 0xa4, 0x69, 0xb4, 0x84, 0xc7, 0x58, 0x5b, 0x78, 0x25, 0x8a, 0xd4, 0xa7, 0x9b,
                0xc9, 0xe4, 0x80, 0x3b,
            ],
            Elements::CalculateIssuanceEntropy => [
                0xca, 0x16, 0x2c, 0x8a, 0xab, 0x8f, 0x94, 0xdd, 0xe2, 0x08, 0xdd, 0x87, 0x13, 0x06,
                0xb4, 0xcc, 0x91, 0x7e, 0xf2, 0xd8, 0x1b, 0x8d, 0xf7, 0x5f, 0xb2, 0x3a, 0x91, 0x6e,
                0xb3, 0xbe, 0x19, 0x7e,
            ],
            Elements::Ch1 => [
                0xc2, 0x32, 0x36, 0x12, 0x4d, 0xa0, 0x1f, 0x3d, 0x8e, 0xb7, 0x42, 0xc2, 0xed, 0x47,
                0x95, 0x3f, 0x66, 0xc8, 0xb0, 0x84, 0xd9, 0x5a, 0x10, 0xc6, 0x0c, 0xae, 0x69, 0xba,
                0x98, 0x42, 0xae, 0x96,
            ],
            Elements::Ch16 => [
                0xdc, 0xcd, 0xe6, 0xa9, 0x54, 0x58, 0x75, 0xc5, 0xcb, 0x46, 0xe7, 0x2c, 0x7b, 0x04,
                0xce, 0xeb, 0x92, 0x0c, 0x20, 0x3d, 0x1c, 0x04, 0x2a, 0xec, 0x91, 0x24, 0x1d, 0xbe,
                0xca, 0x23, 0xf4, 0x35,
            ],
            Elements::Ch32 => [
                0x8c, 0x0a, 0x87, 0xee, 0x8e, 0x1d, 0xfc, 0xa3, 0x4b, 0xdf, 0xf4, 0x21, 0x2b, 0xa1,
                0xaf, 0x75, 0x44, 0xd2, 0xde, 0x6f, 0x4a, 0xcb, 0x76, 0x18, 0x08, 0x23, 0x1a, 0x5f,
                0x57, 0x78, 0x8e, 0x62,
            ],
            Elements::Ch64 => [
                0x5d, 0x5a, 0xe5, 0x7d, 0x76, 0x47, 0xf0, 0x1e, 0xc3, 0xfd, 0x79, 0x7e, 0xd8, 0x9d,
                0x62, 0xbe, 0x5f, 0xe5, 0x85, 0xfb, 0xb9, 0xfd, 0xb6, 0xc3, 0x20, 0x8b, 0x2c, 0x08,
                0x57, 0x9b, 0x9e, 0x2c,
            ],
            Elements::Ch8 => [
                0xdf, 0xc5, 0x78, 0xf2, 0x5c, 0xb3, 0x14, 0x02, 0xe2, 0x2a, 0x81, 0x0e, 0x98, 0x95,
                0x0a, 0xe1, 0x7b, 0x19, 0x08, 0x96, 0x36, 0x4c, 0xde, 0xe8, 0xee, 0xa3, 0x00, 0xcf,
                0x79, 0xd8, 0x78, 0x01,
            ],
            Elements::CheckLockDistance => [
                0xc5, 0x10, 0xcd, 0x9a, 0xf3, 0xea, 0xd1, 0x71, 0xe1, 0x89, 0xf9, 0xe1, 0x0c, 0x95,
                0x46, 0x3d, 0x33, 0x1b, 0x16, 0x73, 0x94, 0x1d, 0xe3, 0x84, 0xe8, 0x46, 0xab, 0x71,
                0x57, 0x53, 0x8e, 0xe6,
            ],
            Elements::CheckLockDuration => [
                0x4a, 0x64, 0x70, 0x0c, 0x73, 0x0c, 0x63, 0x51, 0x94, 0xcc, 0xed, 0x20, 0x47, 0x1b,
                0x0a, 0x69, 0xc6, 0xde, 0xa2, 0x71, 0xb0, 0x4a, 0x41, 0xaa, 0x8c, 0x76, 0x43, 0x7b,
                0x74, 0x62, 0x8d, 0xdb,
            ],
            Elements::CheckLockHeight => [
                0x00, 0xdd, 0x53, 0x16, 0x8f, 0x95, 0x17, 0x73, 0x81, 0x8c, 0x1e, 0xa6, 0x97, 0x97,
                0xed, 0x00, 0xcc, 0xcd, 0x37, 0x62, 0xd2, 0xed, 0xb6, 0xd9, 0x5f, 0xeb, 0x76, 0x70,
                0x98, 0x05, 0xd4, 0x9f,
            ],
            Elements::CheckLockTime => [
                0xde, 0xa6, 0xdf, 0x2a, 0x19, 0x80, 0x02, 0x8f, 0x42, 0x3b, 0x6a, 0x49, 0x04, 0x2e,
                0x1e, 0xe5, 0x36, 0x17, 0x60, 0xe9, 0x42, 0xf0, 0xea, 0x68, 0xf9, 0xbf, 0x9d, 0xca,
                0xab, 0x57, 0xde, 0x53,
            ],
            Elements::CheckSigVerify => [
                0xa5, 0x16, 0xdd, 0x5c, 0x9c, 0xab, 0xd0, 0x41, 0x12, 0x0e, 0xa3, 0x68, 0xab, 0x4c,
                0xcd, 0x92, 0x31, 0x25, 0x5e, 0xf5, 0x79, 0x96, 0x85, 0x58, 0xc1, 0xaa, 0x6a, 0x21,
                0x2f, 0x16, 0x83, 0x29,
            ],
            Elements::Complement1 => [
                0x7e, 0x71, 0x48, 0x13, 0x2e, 0x28, 0x92, 0x82, 0x3f, 0xcf, 0x2a, 0x26, 0xc6, 0x22,
                0x0b, 0xee, 0x03, 0x9f, 0xf6, 0xc5, 0xd7, 0xc1, 0xb4, 0xe4, 0xca, 0x21, 0x3a, 0xd8,
                0x37, 0xab, 0x47, 0x74,
            ],
            Elements::Complement16 => [
                0x7f, 0x16, 0xd1, 0x24, 0x15, 0x99, 0x87, 0x68, 0x34, 0x7f, 0x43, 0xa9, 0xb9, 0x1c,
                0x89, 0x87, 0x59, 0x1d, 0xfe, 0xc1, 0xcf, 0x78, 0x50, 0xb4, 0x23, 0xfe, 0xa3, 0x3a,
                0x37, 0x26, 0xf4, 0x63,
            ],
            Elements::Complement32 => [
                0xe6, 0x5a, 0x4f, 0x75, 0x54, 0x11, 0xb4, 0x80, 0xb8, 0x05, 0x85, 0xfd, 0xc0, 0x2e,
                0xe2, 0x4c, 0xbb, 0xd9, 0x50, 0x2a, 0x48, 0xa7, 0x34, 0x7a, 0x71, 0x02, 0x94, 0xe7,
                0x82, 0xac, 0xff, 0xba,
            ],
            Elements::Complement64 => [
                0xd5, 0xc6, 0x57, 0x13, 0x8a, 0x6c, 0xf5, 0x53, 0x22, 0xc3, 0x6e, 0x6f, 0x3c, 0x4a,
                0x0f, 0x9f, 0xb6, 0x39, 0x88, 0xa1, 0x12, 0x0d, 0x6e, 0xeb, 0x03, 0xfc, 0x42, 0x7c,
                0xad, 0x97, 0x18, 0x69,
            ],
            Elements::Complement8 => [
                0x1f, 0x5c, 0x6a, 0xb4, 0x1d, 0x40, 0x6b, 0xad, 0xfc, 0x9d, 0x5a, 0x2c, 0x76, 0xbe,
                0xf7, 0xe3, 0x28, 0x7a, 0xab, 0xee, 0x19, 0xd0, 0x85, 0x0f, 0x09, 0x38, 0x98, 0x46,
                0xea, 0xbf, 0x45, 0x8e,
            ],
            Elements::CurrentAmount => [
                0x76, 0x31, 0x94, 0x4b, 0x93, 0x6d, 0x0b, 0x7f, 0xa9, 0x2a, 0x86, 0x8d, 0xd7, 0x6e,
                0x4d, 0x47, 0x29, 0x36, 0xcf, 0x42, 0x5c, 0xd8, 0x6e, 0x5b, 0x90, 0x93, 0x6f, 0xbd,
                0x8d, 0x54, 0xe2, 0x02,
            ],
            Elements::CurrentAnnexHash => [
                0x45, 0xca, 0x82, 0x94, 0xef, 0x4d, 0x61, 0x20, 0xe9, 0xef, 0x59, 0xeb, 0xeb, 0x8c,
                0x51, 0xd2, 0xc4, 0xe8, 0x16, 0xf5, 0x2d, 0x14, 0x8f, 0x64, 0x25, 0x0e, 0xb8, 0xa3,
                0xaa, 0x05, 0x0e, 0x18,
            ],
            Elements::CurrentAsset => [
                0x38, 0xf1, 0x4f, 0x83, 0xc6, 0xa0, 0x47, 0x4c, 0x3b, 0xc2, 0x2c, 0xab, 0x54, 0x0f,
                0xaf, 0x66, 0xbb, 0x06, 0xd6, 0x7e, 0xa1, 0x2c, 0x00, 0x48, 0xf4, 0x75, 0x98, 0x0d,
                0x5c, 0x2b, 0x7c, 0x5e,
            ],
            Elements::CurrentIndex => [
                0x75, 0x50, 0x22, 0xed, 0xc3, 0xfa, 0xb4, 0xbc, 0x65, 0x6e, 0xed, 0xc4, 0x61, 0x7e,
                0xa3, 0x7d, 0x13, 0x2b, 0x2c, 0x46, 0x17, 0x40, 0xa6, 0x2d, 0xca, 0xd4, 0x72, 0xea,
                0x23, 0xde, 0x62, 0x9e,
            ],
            Elements::CurrentIssuanceAssetAmount => [
                0xea, 0xc7, 0x4d, 0x4c, 0x7d, 0xe4, 0x35, 0x14, 0x16, 0x4f, 0xe5, 0xae, 0x76, 0x01,
                0x60, 0x45, 0xa6, 0x83, 0x58, 0xcf, 0x1c, 0x74, 0x24, 0x1b, 0x15, 0x2e, 0xd0, 0x63,
                0xac, 0x9f, 0x5e, 0x29,
            ],
            Elements::CurrentIssuanceAssetProof => [
                0x36, 0x2d, 0xfa, 0x80, 0xda, 0x84, 0x10, 0xc5, 0xf5, 0x06, 0x66, 0x47, 0xab, 0x64,
                0x2f, 0x0d, 0x6b, 0x72, 0x86, 0xac, 0x26, 0x08, 0xb9, 0x54, 0xc3, 0xa1, 0x5c, 0xdf,
                0x0f, 0x55, 0x55, 0x5e,
            ],
            Elements::CurrentIssuanceTokenAmount => [
                0x85, 0x92, 0x41, 0xc0, 0x4b, 0x8f, 0x0d, 0xc7, 0x30, 0x72, 0xae, 0xd7, 0x83, 0x80,
                0x67, 0xfa, 0x70, 0xa4, 0xca, 0x52, 0x16, 0xf3, 0x97, 0x13, 0xf1, 0x2f, 0x5f, 0xc5,
                0xa4, 0xae, 0xad, 0x7d,
            ],
            Elements::CurrentIssuanceTokenProof => [
                0x1b, 0x7e, 0xa1, 0x6f, 0x03, 0x2f, 0x37, 0xdc, 0x2a, 0xe1, 0x03, 0xe6, 0x96, 0x2c,
                0x5a, 0x90, 0x42, 0xaf, 0x29, 0x5b, 0x4a, 0x70, 0x3a, 0x3c, 0x84, 0x2e, 0x62, 0x95,
                0x15, 0xad, 0x24, 0xcb,
            ],
            Elements::CurrentNewIssuanceContract => [
                0x0b, 0xaa, 0x87, 0x35, 0xa3, 0x02, 0x90, 0x84, 0x85, 0x7c, 0xc9, 0x74, 0x9b, 0xbc,
                0x7f, 0x19, 0x85, 0x1b, 0xc7, 0x8f, 0xbe, 0xcd, 0xce, 0x1f, 0x0d, 0xd6, 0x75, 0x7b,
                0x7b, 0x8f, 0x3f, 0x30,
            ],
            Elements::CurrentPegin => [
                0x98, 0x07, 0xac, 0x5e, 0xb0, 0x7b, 0xa2, 0x4f, 0xb0, 0x21, 0xb7, 0x4a, 0xe6, 0x87,
                0x4f, 0x3b, 0xc9, 0xb3, 0x93, 0xc6, 0x6c, 0x4f, 0x5d, 0x22, 0xf0, 0x55, 0x46, 0xe2,
                0xea, 0x08, 0xd7, 0x60,
            ],
            Elements::CurrentPrevOutpoint => [
                0x08, 0x77, 0xb3, 0x6f, 0x65, 0x6c, 0x24, 0x89, 0x13, 0x25, 0x44, 0x15, 0x92, 0xdf,
                0x56, 0xe2, 0xbb, 0xec, 0xf2, 0x4d, 0x0a, 0xcd, 0x0b, 0x78, 0xfc, 0x94, 0x38, 0xb7,
                0x77, 0x3a, 0x9e, 0xb4,
            ],
            Elements::CurrentReissuanceBlinding => [
                0xc8, 0xd3, 0xe8, 0x08, 0x87, 0x50, 0x5a, 0xa7, 0xf5, 0x83, 0xac, 0x6b, 0xe4, 0x71,
                0x82, 0xec, 0xbc, 0x68, 0x5c, 0xc3, 0xc4, 0xa6, 0xac, 0xaa, 0x8e, 0x2c, 0x13, 0x29,
                0xe3, 0x5d, 0x29, 0x30,
            ],
            Elements::CurrentReissuanceEntropy => [
                0x4f, 0x24, 0x10, 0x95, 0x40, 0x1c, 0xee, 0xa8, 0x31, 0x2d, 0x2f, 0x42, 0xbe, 0xae,
                0x8d, 0x88, 0x0a, 0xcf, 0xe1, 0x89, 0x33, 0x63, 0x0d, 0x7b, 0x82, 0x16, 0xc5, 0x37,
                0x62, 0x3e, 0x2a, 0xf7,
            ],
            Elements::CurrentScriptHash => [
                0xcd, 0x5e, 0xa7, 0xfb, 0x09, 0x10, 0xf7, 0x6e, 0x76, 0xc6, 0xea, 0x68, 0xd0, 0xe2,
                0xca, 0xd1, 0x34, 0xcb, 0xb6, 0x42, 0x82, 0x3b, 0xc4, 0x3c, 0x04, 0x23, 0x85, 0x2f,
                0x37, 0xb7, 0xe2, 0x5a,
            ],
            Elements::CurrentScriptSigHash => [
                0x17, 0xad, 0x01, 0xca, 0xe2, 0x93, 0x42, 0x7b, 0xe2, 0x50, 0x5a, 0xf2, 0xfe, 0x4c,
                0x3e, 0x65, 0x12, 0x5f, 0xc3, 0xdd, 0xe2, 0x7a, 0x11, 0x43, 0x78, 0x8f, 0xa1, 0x36,
                0x0f, 0x1c, 0x37, 0x51,
            ],
            Elements::CurrentSequence => [
                0x79, 0xce, 0x41, 0xee, 0x05, 0xc9, 0x73, 0x35, 0xdc, 0x73, 0x51, 0x5c, 0x29, 0x34,
                0xc9, 0x1c, 0x2e, 0x2d, 0xca, 0x6e, 0x73, 0xc3, 0x35, 0xea, 0x4f, 0x43, 0xe0, 0xfa,
                0x88, 0xde, 0x00, 0xf6,
            ],
            Elements::Decompress => [
                0xa4, 0x3c, 0x39, 0x26, 0x7f, 0xac, 0xd1, 0xe2, 0xb6, 0xd4, 0x2c, 0x76, 0x15, 0xad,
                0x3f, 0x28, 0x68, 0x63, 0x75, 0x72, 0xf4, 0x6e, 0x18, 0x79, 0x4e, 0x34, 0x75, 0xb9,
                0xb2, 0xea, 0xef, 0x84,
            ],
            Elements::Decrement16 => [
                0x19, 0x6b, 0x2d, 0xc6, 0x32, 0xe6, 0xc5, 0xd0, 0x94, 0xff, 0x9d, 0x34, 0xa2, 0x09,
                0x2b, 0x80, 0x5e, 0x4b, 0x94, 0xd0, 0x36, 0xba, 0x48, 0x26, 0xac, 0xef, 0xed, 0x60,
                0x3a, 0x48, 0x9d, 0xc7,
            ],
            Elements::Decrement32 => [
                0x29, 0xb3, 0x15, 0x92, 0x66, 0xe9, 0xf6, 0x08, 0x80, 0x88, 0x4f, 0x6e, 0xb2, 0x81,
                0x6f, 0x19, 0xa2, 0x07, 0x29, 0x1c, 0x82, 0x9f, 0xd1, 0x39, 0xfb, 0x0d, 0x78, 0xa3,
                0x9e, 0x08, 0xd0, 0xc5,
            ],
            Elements::Decrement64 => [
                0x67, 0x17, 0x5a, 0x67, 0x6d, 0xc1, 0x78, 0x0a, 0x20, 0xfa, 0xf5, 0xe5, 0xaa, 0xd5,
                0x82, 0xec, 0x79, 0x7b, 0xae, 0xa5, 0x5f, 0x29, 0x05, 0xec, 0xad, 0x2a, 0xaa, 0x1a,
                0xe7, 0x6b, 0xe9, 0x43,
            ],
            Elements::Decrement8 => [
                0x2b, 0x59, 0x11, 0xe6, 0x67, 0xe6, 0x96, 0xf9, 0xcd, 0x03, 0xe9, 0xdf, 0x7d, 0x40,
                0x0a, 0x55, 0x45, 0xe7, 0xe9, 0x74, 0x4a, 0xfb, 0xf9, 0x2e, 0x0b, 0xa8, 0x7f, 0x6c,
                0x7c, 0xc4, 0x42, 0x87,
            ],
            Elements::DivMod128_64 => [
                0x1e, 0xe3, 0xe6, 0x0f, 0x5c, 0x9e, 0x4f, 0x0d, 0x3e, 0xc9, 0xeb, 0xea, 0x36, 0x98,
                0xec, 0xa8, 0xbd, 0x32, 0xac, 0x16, 0x7e, 0x69, 0xb2, 0x96, 0xab, 0x9c, 0x18, 0x36,
                0xb6, 0x10, 0x1d, 0x04,
            ],
            Elements::DivMod16 => [
                0x15, 0x85, 0xd1, 0xbf, 0xe9, 0x52, 0xf7, 0x1b, 0x1c, 0xfe, 0xbb, 0xef, 0x29, 0x2c,
                0xed, 0x56, 0x60, 0x91, 0x57, 0x23, 0x05, 0x18, 0x27, 0x20, 0x62, 0x5a, 0x1a, 0xc0,
                0x08, 0x81, 0xa4, 0x89,
            ],
            Elements::DivMod32 => [
                0xba, 0xb7, 0xbc, 0x9a, 0x9f, 0x64, 0x4e, 0xdc, 0xba, 0x73, 0xe3, 0x30, 0xc1, 0xa5,
                0x05, 0x43, 0x67, 0xf5, 0xfb, 0x86, 0x78, 0xc1, 0x23, 0x6a, 0xb6, 0xe7, 0xf4, 0xd7,
                0x18, 0x60, 0xe0, 0x02,
            ],
            Elements::DivMod64 => [
                0x8d, 0xb4, 0x66, 0x4d, 0x99, 0xe8, 0xc8, 0x78, 0xef, 0x31, 0xeb, 0xf1, 0xac, 0xa6,
                0xa8, 0x76, 0x4b, 0x58, 0x49, 0x5d, 0xa0, 0xec, 0xc0, 0x32, 0xb3, 0xc9, 0xb4, 0x89,
                0xaa, 0xa0, 0xff, 0x90,
            ],
            Elements::DivMod8 => [
                0x25, 0xf3, 0x69, 0x4b, 0x75, 0x2e, 0x4d, 0xf1, 0xb0, 0xcc, 0x8c, 0x51, 0xbf, 0x99,
                0x17, 0xef, 0x6e, 0x11, 0xc5, 0x2e, 0xa7, 0x77, 0x35, 0x3e, 0x95, 0x07, 0xe8, 0x19,
                0xa8, 0x06, 0x62, 0x95,
            ],
            Elements::Divide16 => [
                0x2d, 0xc3, 0xe9, 0xa9, 0xc6, 0x1d, 0xc8, 0x8d, 0xd7, 0x44, 0xf1, 0x6b, 0xe1, 0xae,
                0xd4, 0xb7, 0x2a, 0xb4, 0x79, 0x49, 0x0e, 0x41, 0x42, 0x26, 0x58, 0xb9, 0x72, 0x19,
                0xdf, 0x49, 0x95, 0x7c,
            ],
            Elements::Divide32 => [
                0xf6, 0xdc, 0xf4, 0xbd, 0x8d, 0x9b, 0x5e, 0xff, 0x75, 0x1e, 0x9b, 0x01, 0x47, 0xa9,
                0x8e, 0x0f, 0xc4, 0xc8, 0x87, 0x2c, 0x1a, 0xb7, 0x82, 0xd1, 0xeb, 0xec, 0x63, 0x83,
                0xa0, 0x9b, 0x78, 0x0a,
            ],
            Elements::Divide64 => [
                0x8a, 0x94, 0x51, 0x8b, 0x9b, 0x76, 0x17, 0x9a, 0x9d, 0x5c, 0x1e, 0x31, 0x6e, 0x69,
                0x00, 0x62, 0x7f, 0x5b, 0x6e, 0x3e, 0xcc, 0x66, 0x8f, 0xe2, 0xd0, 0xc5, 0xd5, 0x62,
                0x38, 0xe3, 0x3e, 0xa3,
            ],
            Elements::Divide8 => [
                0xd7, 0x16, 0x00, 0x0c, 0xe7, 0xe5, 0x72, 0x78, 0x84, 0xa6, 0xd7, 0x97, 0x6a, 0x32,
                0xc0, 0xf6, 0xbd, 0xb8, 0x81, 0xd0, 0x0f, 0xe8, 0xdb, 0xb2, 0x8d, 0xea, 0x37, 0x01,
                0x03, 0x76, 0x9e, 0x2d,
            ],
            Elements::Divides16 => [
                0x8f, 0x41, 0x84, 0xa5, 0x82, 0x67, 0x4e, 0x00, 0x3b, 0x37, 0x40, 0x1b, 0x13, 0xc9,
                0xb8, 0xc3, 0x7c, 0x68, 0x3b, 0x88, 0xdf, 0x63, 0x3a, 0x22, 0x71, 0x9a, 0x23, 0xe1,
                0x94, 0xbd, 0xa6, 0xf3,
            ],
            Elements::Divides32 => [
                0xe8, 0x9e, 0xcd, 0xc3, 0x7c, 0xd1, 0x04, 0x66, 0xb0, 0x0f, 0x01, 0x37, 0xe2, 0x0b,
                0x09, 0xb9, 0xc5, 0x94, 0x8d, 0x07, 0x31, 0x2d, 0x05, 0x3f, 0xed, 0x63, 0xc4, 0x91,
                0xbb, 0x5c, 0x4c, 0x6a,
            ],
            Elements::Divides64 => [
                0xb4, 0x31, 0x98, 0xd5, 0x74, 0x41, 0x90, 0x57, 0xe7, 0x6b, 0xc0, 0x69, 0xd7, 0xd3,
                0xe2, 0xcb, 0x04, 0xf7, 0x07, 0x2b, 0x80, 0x69, 0x6d, 0x3d, 0xb9, 0x2c, 0x8a, 0x72,
                0xc7, 0x8a, 0x5c, 0xfc,
            ],
            Elements::Divides8 => [
                0x3f, 0xc3, 0xb9, 0xb4, 0x96, 0x8e, 0x16, 0x74, 0xda, 0x9e, 0xc4, 0x57, 0xb1, 0xca,
                0x8a, 0x66, 0x29, 0xa3, 0x7b, 0x4b, 0xef, 0xdc, 0xd7, 0xcc, 0x55, 0x88, 0x60, 0x80,
                0xae, 0xf6, 0x8a, 0xf8,
            ],
            Elements::Eq1 => [
                0x42, 0x49, 0xc5, 0xbd, 0xec, 0x54, 0x0a, 0x06, 0x95, 0x7d, 0xcd, 0xab, 0x04, 0x5a,
                0x44, 0x5e, 0xb9, 0x18, 0x91, 0xc0, 0x6c, 0x3f, 0x8f, 0x96, 0xc8, 0x88, 0xe9, 0xdd,
                0xd2, 0xfb, 0xaa, 0x28,
            ],
            Elements::Eq16 => [
                0xab, 0x31, 0xa7, 0x97, 0x4a, 0xcb, 0xf7, 0x2a, 0xb2, 0xf2, 0x1b, 0xf5, 0x3f, 0xec,
                0x34, 0x6a, 0x28, 0xe6, 0xe6, 0x5e, 0x4c, 0x05, 0xe3, 0xe7, 0x84, 0x3e, 0x14, 0x73,
                0xb1, 0xfd, 0x3f, 0xf9,
            ],
            Elements::Eq256 => [
                0xf7, 0xdf, 0x29, 0xed, 0x2d, 0x00, 0x78, 0x22, 0x7e, 0x71, 0x9b, 0x7d, 0x13, 0x37,
                0xdb, 0xea, 0x3a, 0x71, 0x0f, 0x58, 0x4a, 0x4b, 0x07, 0xf0, 0xf2, 0x65, 0x4a, 0x44,
                0x65, 0x54, 0xf2, 0xbe,
            ],
            Elements::Eq32 => [
                0x76, 0x79, 0x1f, 0x0d, 0xdd, 0xab, 0x9d, 0xc7, 0x7a, 0x14, 0xa6, 0x8c, 0xc6, 0x77,
                0x87, 0xc9, 0x84, 0x49, 0x3f, 0xae, 0xbb, 0xdb, 0xb6, 0xd6, 0xd4, 0x93, 0x35, 0x2e,
                0x11, 0x47, 0x20, 0xd5,
            ],
            Elements::Eq64 => [
                0x01, 0x1b, 0x5f, 0x4c, 0xb3, 0x1f, 0x34, 0xba, 0x15, 0x8a, 0x68, 0x22, 0x3b, 0x34,
                0x44, 0x5e, 0x93, 0xf3, 0xdc, 0x18, 0x91, 0xb5, 0x33, 0x90, 0x04, 0x92, 0x3d, 0xcf,
                0xab, 0xe0, 0x34, 0xc1,
            ],
            Elements::Eq8 => [
                0x0f, 0x8b, 0x1f, 0xde, 0x19, 0x49, 0xbe, 0xbd, 0x47, 0xd2, 0x55, 0x4a, 0x48, 0x07,
                0xfd, 0x8d, 0xca, 0xef, 0x87, 0x30, 0x18, 0x38, 0xd6, 0x0e, 0x84, 0xc7, 0x38, 0x14,
                0x42, 0xda, 0x7e, 0xce,
            ],
            Elements::FeAdd => [
                0xa5, 0xe9, 0x21, 0xd4, 0xd2, 0x95, 0x9a, 0xf7, 0x83, 0x3d, 0xf9, 0x30, 0x83, 0x68,
                0x98, 0x57, 0x32, 0xab, 0x0f, 0x33, 0xba, 0xd8, 0xaa, 0x34, 0x5b, 0x39, 0xf3, 0x03,
                0x03, 0x2b, 0xe7, 0xae,
            ],
            Elements::FeInvert => [
                0x21, 0xf4, 0x29, 0x56, 0x9d, 0x78, 0xe0, 0x9f, 0xb2, 0x90, 0x13, 0xef, 0x8a, 0x16,
                0xc7, 0x55, 0x0e, 0xdc, 0x4e, 0x19, 0x05, 0x25, 0x71, 0x5a, 0xbf, 0xb0, 0xc1, 0x9b,
                0x21, 0xcf, 0xd9, 0xc2,
            ],
            Elements::FeIsOdd => [
                0x21, 0xd3, 0x5d, 0x0a, 0x65, 0x7f, 0x5d, 0xef, 0xac, 0xde, 0x6d, 0xa3, 0xfd, 0xb4,
                0x8d, 0x67, 0x56, 0xf3, 0x06, 0x90, 0xb4, 0x02, 0x70, 0xf0, 0x10, 0x17, 0xe7, 0xe3,
                0x48, 0xd7, 0x58, 0x5d,
            ],
            Elements::FeIsZero => [
                0xf5, 0x71, 0x62, 0x82, 0xba, 0x0d, 0x26, 0xbd, 0x06, 0x71, 0xcd, 0xfe, 0x1f, 0x11,
                0x8f, 0x45, 0xba, 0xe7, 0x9d, 0x10, 0x9a, 0x6d, 0x9f, 0x39, 0x7a, 0x4c, 0x45, 0x54,
                0x6f, 0x62, 0xaf, 0x00,
            ],
            Elements::FeMultiply => [
                0xc1, 0x76, 0x96, 0x9c, 0x33, 0x7f, 0xbc, 0x6b, 0x5a, 0xc8, 0xed, 0x3e, 0xa2, 0x8a,
                0xb3, 0xfd, 0x94, 0x78, 0x20, 0x64, 0x31, 0x95, 0xd7, 0x3b, 0x86, 0x20, 0x18, 0xf7,
                0x08, 0x48, 0x7d, 0x23,
            ],
            Elements::FeMultiplyBeta => [
                0xc0, 0xfc, 0x76, 0x66, 0x8d, 0x99, 0xd4, 0x07, 0x3f, 0x39, 0x93, 0x44, 0x3a, 0x68,
                0x98, 0x0b, 0xa9, 0x6e, 0x97, 0x60, 0x1e, 0xd1, 0xa9, 0x19, 0xc4, 0xa2, 0x33, 0x05,
                0x7b, 0x66, 0xfe, 0x23,
            ],
            Elements::FeNegate => [
                0xd6, 0xef, 0x8f, 0xc2, 0x27, 0x5d, 0xdf, 0x70, 0x58, 0xb5, 0xe4, 0x99, 0xf1, 0x0d,
                0xd2, 0x84, 0x66, 0xf5, 0x7b, 0x72, 0x53, 0xa5, 0x44, 0x07, 0x79, 0x8a, 0x47, 0x72,
                0x59, 0x21, 0x84, 0xb9,
            ],
            Elements::FeNormalize => [
                0xf4, 0x98, 0xbc, 0xdf, 0xf3, 0x50, 0x67, 0x88, 0x1b, 0xd9, 0x6e, 0xe1, 0xc9, 0x1f,
                0x43, 0x67, 0x1e, 0x79, 0x18, 0x6e, 0x70, 0x0e, 0xc4, 0xda, 0x2d, 0xc0, 0x29, 0x6a,
                0x2d, 0xea, 0x91, 0xc7,
            ],
            Elements::FeSquare => [
                0x16, 0x8a, 0x11, 0xb1, 0x9c, 0x0f, 0xb9, 0x7f, 0x10, 0x99, 0xf8, 0x63, 0xa4, 0xc8,
                0xe2, 0xf8, 0x12, 0x42, 0x5f, 0xbd, 0x3d, 0x01, 0xbd, 0xed, 0x3f, 0x46, 0x0c, 0xb3,
                0x59, 0xbf, 0xdd, 0x89,
            ],
            Elements::FeSquareRoot => [
                0x3b, 0x46, 0xcd, 0xe3, 0x06, 0x74, 0xb7, 0x77, 0x16, 0xf2, 0xc8, 0xf6, 0x14, 0xad,
                0xbd, 0x16, 0x61, 0x98, 0x5d, 0x82, 0xc5, 0x53, 0x04, 0xd5, 0x6e, 0x53, 0xef, 0x5c,
                0x74, 0xc8, 0x3e, 0x83,
            ],
            Elements::FullAdd16 => [
                0xf4, 0x54, 0xa6, 0x32, 0xfd, 0x19, 0x28, 0xcd, 0x6f, 0x07, 0x0d, 0xf6, 0x80, 0x14,
                0x28, 0x8e, 0x97, 0xb3, 0xe5, 0x82, 0xe7, 0xf0, 0x3e, 0x98, 0xed, 0x32, 0x34, 0xb6,
                0x28, 0x2d, 0x02, 0x94,
            ],
            Elements::FullAdd32 => [
                0x96, 0x06, 0xb9, 0xe6, 0x54, 0x1e, 0x03, 0x56, 0xed, 0x76, 0x08, 0xd5, 0xbc, 0x6f,
                0x9d, 0x4e, 0xf6, 0x8e, 0x0a, 0x3d, 0x23, 0x51, 0x68, 0xd9, 0xe7, 0x5c, 0xae, 0x66,
                0x52, 0xdd, 0x06, 0x34,
            ],
            Elements::FullAdd64 => [
                0xf0, 0xf7, 0x95, 0x32, 0x00, 0x1c, 0x02, 0xaf, 0xac, 0x65, 0x0a, 0x62, 0x80, 0x19,
                0x32, 0xaf, 0xc2, 0xc1, 0xb2, 0x86, 0xa3, 0x1f, 0xef, 0x7a, 0xe0, 0x8a, 0x47, 0xea,
                0x1e, 0x71, 0x38, 0x7d,
            ],
            Elements::FullAdd8 => [
                0xcd, 0xd0, 0x80, 0xfd, 0x86, 0x12, 0xe7, 0xd1, 0x4a, 0xda, 0x34, 0x91, 0x64, 0xa6,
                0xd5, 0xaf, 0x60, 0x54, 0xd6, 0x73, 0x77, 0xe1, 0x76, 0x65, 0xc4, 0x72, 0x20, 0x40,
                0x28, 0xbc, 0x21, 0x25,
            ],
            Elements::FullDecrement16 => [
                0xa7, 0xd1, 0xb9, 0xa6, 0x09, 0xdc, 0xd3, 0x67, 0x66, 0xa1, 0x67, 0xa3, 0x7a, 0xa0,
                0xe6, 0x49, 0xfb, 0xdd, 0x75, 0xf6, 0x46, 0x91, 0xa9, 0xfb, 0x6b, 0xf1, 0x66, 0x3d,
                0x32, 0x92, 0x82, 0xa8,
            ],
            Elements::FullDecrement32 => [
                0xcb, 0xcf, 0xd7, 0xca, 0x73, 0x72, 0xb2, 0xb1, 0x80, 0x00, 0x8e, 0x51, 0x58, 0x98,
                0x80, 0xd3, 0xb5, 0x39, 0x9b, 0xd9, 0xa8, 0xdd, 0xb9, 0xbc, 0xda, 0x61, 0x32, 0xc2,
                0x2c, 0x77, 0x7e, 0x0f,
            ],
            Elements::FullDecrement64 => [
                0x42, 0xdf, 0x92, 0xd2, 0xe7, 0xf4, 0x2e, 0x57, 0x9b, 0xd4, 0xc4, 0x30, 0x8c, 0xd2,
                0xdb, 0x6e, 0x76, 0x77, 0x46, 0x95, 0xd6, 0xa5, 0x69, 0x50, 0xc5, 0xcf, 0x92, 0x03,
                0x00, 0x2d, 0x41, 0xaa,
            ],
            Elements::FullDecrement8 => [
                0xff, 0x58, 0xdd, 0x01, 0x62, 0xbc, 0xe9, 0xfb, 0x73, 0xed, 0x01, 0x5b, 0x7e, 0x5d,
                0x3d, 0x1d, 0xc1, 0x1b, 0x50, 0x5c, 0x93, 0x2d, 0x49, 0x0c, 0xaa, 0xdc, 0x1b, 0x5d,
                0x89, 0x3d, 0x1f, 0xad,
            ],
            Elements::FullIncrement16 => [
                0x54, 0x53, 0x60, 0x8e, 0xef, 0xf9, 0x67, 0xaf, 0xef, 0x4a, 0xe5, 0x83, 0x1a, 0xa7,
                0xa9, 0xfc, 0x75, 0xc2, 0xba, 0x57, 0xe1, 0xac, 0x41, 0x2a, 0xb5, 0x46, 0x4d, 0x4d,
                0xe9, 0x3a, 0xf4, 0xf9,
            ],
            Elements::FullIncrement32 => [
                0x36, 0x32, 0x33, 0x21, 0x58, 0xf9, 0x0b, 0xa4, 0xdf, 0x62, 0x3f, 0x08, 0x68, 0x08,
                0xd1, 0xd3, 0x39, 0xf3, 0x6d, 0x9a, 0x6f, 0x92, 0x27, 0x23, 0x34, 0x16, 0xf2, 0x34,
                0xcd, 0xd2, 0x5d, 0x98,
            ],
            Elements::FullIncrement64 => [
                0x06, 0x37, 0xfc, 0xc5, 0x52, 0xf6, 0x2a, 0x8a, 0x08, 0xc6, 0xd0, 0x70, 0x66, 0x7e,
                0xd8, 0x0e, 0x47, 0xba, 0x70, 0x7c, 0x12, 0x0c, 0x4d, 0x65, 0x66, 0x04, 0xa3, 0x43,
                0x3b, 0xfc, 0x40, 0xa6,
            ],
            Elements::FullIncrement8 => [
                0x29, 0x15, 0x24, 0xe1, 0x70, 0xd5, 0x23, 0xfd, 0x35, 0xc2, 0x57, 0x7b, 0x8e, 0xa3,
                0xf4, 0x47, 0x8e, 0xc6, 0x32, 0x06, 0xc9, 0x8f, 0xb5, 0xcc, 0xcf, 0x77, 0xc2, 0x09,
                0xbe, 0xbd, 0x29, 0xef,
            ],
            Elements::FullLeftShift16_1 => [
                0x74, 0x0e, 0x23, 0x81, 0x1b, 0x3e, 0x62, 0xd4, 0x91, 0x51, 0x0f, 0xc9, 0xed, 0xc4,
                0xcb, 0x0a, 0x0e, 0xec, 0xfa, 0xdf, 0xd2, 0x1b, 0xb2, 0x7f, 0x33, 0xe2, 0x20, 0xb1,
                0xd8, 0x7d, 0x14, 0xd7,
            ],
            Elements::FullLeftShift16_2 => [
                0xc2, 0x6e, 0x8a, 0xc3, 0xff, 0x9c, 0xc5, 0x18, 0x71, 0x9d, 0x4d, 0x7f, 0xd1, 0x49,
                0xd8, 0x02, 0xf2, 0x3f, 0x0b, 0x02, 0x49, 0x99, 0xed, 0x5d, 0xaf, 0x36, 0x92, 0x10,
                0xac, 0xbe, 0x33, 0x45,
            ],
            Elements::FullLeftShift16_4 => [
                0x5e, 0x57, 0x0c, 0xbe, 0x4c, 0x7c, 0xa9, 0x4b, 0xe0, 0xfc, 0x7b, 0x3e, 0xe5, 0x79,
                0xbd, 0xd7, 0x84, 0x26, 0xf0, 0xb7, 0x67, 0xf4, 0x85, 0x17, 0x17, 0xbb, 0xfe, 0xae,
                0xde, 0x91, 0xfe, 0x30,
            ],
            Elements::FullLeftShift16_8 => [
                0x65, 0xad, 0xc5, 0x53, 0x48, 0x38, 0x3b, 0x28, 0xe8, 0x79, 0x7f, 0x81, 0xa9, 0x28,
                0x2d, 0x91, 0x1b, 0x3f, 0x8f, 0xa6, 0x13, 0x92, 0x72, 0x51, 0xd8, 0x8e, 0x0c, 0x38,
                0xb0, 0x29, 0xb7, 0x05,
            ],
            Elements::FullLeftShift32_1 => [
                0x72, 0xe0, 0x10, 0x4e, 0xfa, 0xf1, 0xde, 0xe4, 0x11, 0x98, 0xec, 0x3b, 0x79, 0x03,
                0x73, 0xf6, 0x48, 0xf1, 0x3f, 0x5e, 0xe0, 0x65, 0x52, 0xfb, 0x02, 0x0b, 0xaf, 0xb5,
                0x84, 0x97, 0xc2, 0x5c,
            ],
            Elements::FullLeftShift32_16 => [
                0xb9, 0xfb, 0x21, 0x69, 0x90, 0x8d, 0x91, 0x44, 0xcc, 0x73, 0xe6, 0x8f, 0x75, 0x35,
                0x36, 0xf4, 0x3c, 0xb2, 0xb7, 0x4c, 0xb6, 0x2c, 0x64, 0x08, 0x81, 0x06, 0x70, 0xde,
                0x84, 0xab, 0x09, 0xbd,
            ],
            Elements::FullLeftShift32_2 => [
                0x11, 0xef, 0xdb, 0x81, 0xb0, 0xc4, 0xde, 0xda, 0x4d, 0x4f, 0x98, 0x47, 0x5d, 0x78,
                0x78, 0xef, 0xa3, 0x38, 0x69, 0x4f, 0xa0, 0xfd, 0x61, 0x3e, 0x12, 0x93, 0x22, 0x5a,
                0x4f, 0x46, 0x2f, 0x7c,
            ],
            Elements::FullLeftShift32_4 => [
                0x77, 0xe3, 0x99, 0xd7, 0xd8, 0x3f, 0x7d, 0x11, 0x44, 0x99, 0x1d, 0xaf, 0xa3, 0xcc,
                0x98, 0x11, 0xc1, 0x63, 0x2c, 0x29, 0xe4, 0x93, 0xa8, 0xaf, 0x98, 0xe9, 0x8f, 0xbc,
                0x1d, 0x63, 0x5f, 0xb4,
            ],
            Elements::FullLeftShift32_8 => [
                0xba, 0x66, 0x4c, 0xb1, 0xc4, 0x2e, 0xda, 0x17, 0x91, 0x91, 0xeb, 0xc2, 0xa1, 0x10,
                0x39, 0x6d, 0xae, 0x58, 0xf9, 0x06, 0xa6, 0x41, 0x06, 0xb3, 0x06, 0x67, 0x79, 0x0a,
                0xc2, 0xf2, 0x38, 0x2d,
            ],
            Elements::FullLeftShift64_1 => [
                0x79, 0xd3, 0x8f, 0xe0, 0x75, 0x83, 0x9b, 0x22, 0x7c, 0xff, 0xd9, 0x2a, 0x8c, 0xdb,
                0x5c, 0x8c, 0x35, 0x22, 0xbc, 0xb4, 0xd1, 0xe0, 0x3b, 0xee, 0xb6, 0xdb, 0x6a, 0xb6,
                0x4e, 0xd4, 0x72, 0x1f,
            ],
            Elements::FullLeftShift64_16 => [
                0x21, 0x43, 0x56, 0x62, 0x45, 0xf5, 0xa1, 0xb9, 0xdf, 0xeb, 0x0c, 0x75, 0x87, 0x8e,
                0x21, 0xdb, 0xe1, 0x38, 0x04, 0xc2, 0x69, 0x35, 0xee, 0x47, 0xca, 0xc9, 0xad, 0x82,
                0x2d, 0x6d, 0xed, 0xb2,
            ],
            Elements::FullLeftShift64_2 => [
                0x9c, 0x92, 0x16, 0x49, 0x15, 0xaf, 0x0b, 0x15, 0x4e, 0x1d, 0xf5, 0x64, 0xd4, 0xdc,
                0x9b, 0xe9, 0x80, 0xb3, 0x98, 0x83, 0x5c, 0x99, 0x88, 0xbb, 0xb1, 0x08, 0xd0, 0xcd,
                0x81, 0x45, 0xb3, 0x30,
            ],
            Elements::FullLeftShift64_32 => [
                0xd0, 0xd0, 0x16, 0xe9, 0xc7, 0x8c, 0xd1, 0x12, 0xb4, 0xdd, 0x91, 0xa8, 0x35, 0x9f,
                0x80, 0x5c, 0x68, 0x41, 0x5b, 0x85, 0x7a, 0x79, 0x9b, 0x00, 0x39, 0x49, 0x54, 0xdc,
                0xd2, 0x90, 0xac, 0xbc,
            ],
            Elements::FullLeftShift64_4 => [
                0x0f, 0x1f, 0x7d, 0x37, 0x4e, 0x82, 0x86, 0x8d, 0x71, 0xe7, 0xe7, 0xc0, 0x32, 0x21,
                0xb1, 0x50, 0x59, 0x4b, 0x63, 0x04, 0x45, 0xb1, 0xb1, 0x63, 0x56, 0xcf, 0x35, 0x45,
                0xbd, 0x93, 0x92, 0x63,
            ],
            Elements::FullLeftShift64_8 => [
                0xad, 0x7b, 0x44, 0x38, 0xb7, 0x3f, 0x6f, 0x9e, 0x42, 0xf6, 0x4c, 0x70, 0x53, 0x04,
                0x75, 0xee, 0x08, 0x93, 0x6e, 0x47, 0x63, 0xe5, 0xb7, 0x3e, 0xa4, 0xbc, 0x83, 0x83,
                0xa2, 0xb9, 0x63, 0xd5,
            ],
            Elements::FullLeftShift8_1 => [
                0x21, 0x13, 0x68, 0x1a, 0x11, 0x62, 0x4e, 0x60, 0x60, 0x30, 0xc4, 0x70, 0xd6, 0x8f,
                0x60, 0x61, 0x23, 0x2f, 0x71, 0xcf, 0xab, 0xc5, 0x05, 0x71, 0x92, 0xc6, 0xc8, 0xbd,
                0x1d, 0x73, 0xb7, 0xe1,
            ],
            Elements::FullLeftShift8_2 => [
                0x36, 0x83, 0x68, 0xc9, 0x4b, 0x04, 0x0e, 0x81, 0xb9, 0x48, 0xd7, 0x37, 0xc1, 0x93,
                0xc0, 0x42, 0x83, 0xec, 0x80, 0xa2, 0x8f, 0xd3, 0xa0, 0x21, 0xb0, 0xb8, 0xc1, 0xab,
                0xcf, 0x5e, 0xdc, 0xd3,
            ],
            Elements::FullLeftShift8_4 => [
                0x8f, 0x85, 0x4d, 0x58, 0xf9, 0x68, 0xb4, 0xbe, 0x3b, 0x20, 0x21, 0xfb, 0x22, 0x14,
                0x2d, 0xd3, 0xe6, 0x8a, 0xa8, 0x19, 0x7b, 0x54, 0x75, 0xb7, 0x05, 0x0b, 0x02, 0xe1,
                0xe5, 0xca, 0xee, 0x47,
            ],
            Elements::FullMultiply16 => [
                0x32, 0xcf, 0x7f, 0x50, 0x89, 0x4e, 0xa2, 0xc4, 0x61, 0xa0, 0x54, 0x66, 0xbb, 0xfa,
                0x1e, 0x4e, 0x1b, 0x04, 0x99, 0x57, 0x52, 0x3f, 0x64, 0x93, 0x7a, 0x8b, 0x54, 0x27,
                0x3d, 0xd3, 0x1b, 0x37,
            ],
            Elements::FullMultiply32 => [
                0xde, 0xa1, 0xaf, 0xc6, 0xfd, 0x54, 0x6c, 0x75, 0xe0, 0xb2, 0xd8, 0xe4, 0x18, 0xf2,
                0x61, 0x79, 0xd6, 0xdb, 0xe9, 0x05, 0x8b, 0x07, 0x9a, 0xa9, 0xab, 0x80, 0xea, 0xa6,
                0xc0, 0x5f, 0x39, 0xcb,
            ],
            Elements::FullMultiply64 => [
                0x81, 0x3d, 0x74, 0xd6, 0xc3, 0x06, 0x4c, 0xf7, 0xc5, 0xdb, 0x2d, 0xda, 0x96, 0x4e,
                0xd0, 0xe2, 0xd5, 0xa2, 0x49, 0x1b, 0x89, 0x43, 0x29, 0x21, 0x92, 0x37, 0xcb, 0x1a,
                0x91, 0xee, 0x09, 0x34,
            ],
            Elements::FullMultiply8 => [
                0xf7, 0xf3, 0x9d, 0x95, 0xda, 0xb5, 0x73, 0x08, 0x52, 0xe9, 0xcc, 0x7e, 0x74, 0xc0,
                0x74, 0x3b, 0x8f, 0xb3, 0xf7, 0x54, 0x87, 0x12, 0x0b, 0xa3, 0x26, 0xff, 0x60, 0x0a,
                0xd8, 0xb1, 0xf3, 0xe6,
            ],
            Elements::FullRightShift16_1 => [
                0xb8, 0x07, 0x44, 0x23, 0xe6, 0x74, 0x8a, 0x6a, 0xa5, 0x4e, 0xc5, 0x74, 0x1f, 0xee,
                0xf2, 0x5a, 0x26, 0x2f, 0xde, 0xcb, 0xfc, 0xe3, 0x91, 0x24, 0xe6, 0x10, 0x23, 0x8a,
                0x3b, 0x0a, 0x23, 0xfc,
            ],
            Elements::FullRightShift16_2 => [
                0x3f, 0xcf, 0x98, 0x5e, 0xe0, 0xc7, 0x2c, 0xa4, 0x1d, 0xdf, 0x6c, 0x89, 0xd0, 0xf0,
                0xf6, 0x9d, 0x50, 0x65, 0x87, 0x6e, 0x3b, 0x60, 0x20, 0xec, 0xc9, 0xbf, 0x05, 0x9e,
                0x8f, 0x97, 0x19, 0xc6,
            ],
            Elements::FullRightShift16_4 => [
                0xa3, 0x0c, 0x7c, 0x29, 0xd0, 0xee, 0xac, 0x29, 0x52, 0x58, 0xb2, 0xb6, 0x1d, 0x0b,
                0x54, 0x13, 0x46, 0xf4, 0x07, 0xc0, 0x84, 0x8d, 0x44, 0x8e, 0x13, 0xe9, 0x77, 0x4c,
                0x1c, 0x96, 0x96, 0x79,
            ],
            Elements::FullRightShift16_8 => [
                0x5b, 0x88, 0x08, 0xca, 0xda, 0x55, 0x87, 0xb3, 0x6d, 0x1a, 0x6f, 0xad, 0x66, 0xae,
                0x4d, 0xa0, 0x8d, 0x41, 0x23, 0x64, 0x4c, 0x0b, 0xdd, 0x59, 0x77, 0x2a, 0x70, 0xaa,
                0x74, 0x32, 0xe7, 0x15,
            ],
            Elements::FullRightShift32_1 => [
                0x32, 0xaf, 0xd0, 0xef, 0x94, 0xdf, 0x51, 0xb7, 0xd3, 0x5c, 0x00, 0xe5, 0x61, 0xa8,
                0x39, 0x0c, 0x5c, 0xf5, 0x0f, 0x93, 0x0b, 0x30, 0xd7, 0x86, 0x88, 0x04, 0xb5, 0x80,
                0x49, 0x37, 0x58, 0x40,
            ],
            Elements::FullRightShift32_16 => [
                0x44, 0xd1, 0x79, 0xa8, 0x90, 0xf7, 0x81, 0x2f, 0x15, 0x13, 0x31, 0xb5, 0x5f, 0xc0,
                0x7e, 0xb4, 0xe4, 0xd7, 0x81, 0x4e, 0xb6, 0x83, 0xda, 0x28, 0x8f, 0x8f, 0xe7, 0xcd,
                0x55, 0xb4, 0x39, 0x06,
            ],
            Elements::FullRightShift32_2 => [
                0x33, 0xc6, 0x61, 0xdf, 0x3a, 0x32, 0xca, 0xe5, 0x5b, 0x52, 0xa5, 0xf2, 0x63, 0x21,
                0x54, 0xcc, 0x85, 0xb6, 0x59, 0x13, 0x87, 0xbc, 0x2b, 0x34, 0x83, 0x30, 0xc8, 0x70,
                0xa6, 0xf6, 0x70, 0x6f,
            ],
            Elements::FullRightShift32_4 => [
                0xe4, 0xbe, 0xbf, 0x16, 0x93, 0x5f, 0x67, 0xbe, 0x7d, 0x8c, 0x86, 0xbc, 0x58, 0x8a,
                0xdb, 0xcf, 0x8e, 0x59, 0x75, 0x39, 0x25, 0x7f, 0xdd, 0xab, 0x9f, 0xb0, 0x43, 0x72,
                0xc7, 0x70, 0x12, 0xd3,
            ],
            Elements::FullRightShift32_8 => [
                0xab, 0xcf, 0xfb, 0x08, 0x4a, 0x23, 0x96, 0x42, 0x16, 0xd5, 0x62, 0x73, 0x30, 0x5c,
                0x0c, 0x8b, 0x03, 0xbd, 0xab, 0xda, 0xd6, 0x9f, 0xf7, 0xe9, 0x42, 0xf0, 0xd2, 0xcf,
                0x08, 0x0f, 0xeb, 0xcc,
            ],
            Elements::FullRightShift64_1 => [
                0x37, 0x68, 0x82, 0x60, 0xc5, 0x3a, 0xf0, 0x6b, 0x85, 0x6d, 0x90, 0x22, 0xca, 0x5d,
                0x87, 0xf8, 0xa6, 0x87, 0xee, 0x53, 0xfa, 0xca, 0x18, 0x66, 0xec, 0x84, 0x2a, 0x7c,
                0x89, 0x0a, 0x4b, 0x70,
            ],
            Elements::FullRightShift64_16 => [
                0x41, 0x7b, 0xfb, 0x71, 0x5a, 0x20, 0xb1, 0x0d, 0x48, 0x81, 0xf5, 0xc3, 0x49, 0x6c,
                0x63, 0xef, 0xee, 0x4a, 0xb5, 0x00, 0x3d, 0xfd, 0x0a, 0x16, 0xb8, 0x5f, 0x94, 0xf8,
                0xe5, 0xb0, 0x66, 0x7c,
            ],
            Elements::FullRightShift64_2 => [
                0xce, 0xca, 0x25, 0x67, 0xb9, 0x1a, 0x63, 0xe9, 0xca, 0x44, 0x03, 0x5e, 0xb5, 0x9e,
                0x2f, 0x22, 0xd8, 0x1e, 0x37, 0xe1, 0x96, 0x59, 0x5a, 0x74, 0x8c, 0xea, 0x4a, 0x46,
                0x84, 0xa2, 0x15, 0xb0,
            ],
            Elements::FullRightShift64_32 => [
                0x03, 0x96, 0x99, 0x37, 0x84, 0x02, 0x3d, 0x47, 0xe8, 0x51, 0x4b, 0x45, 0x92, 0x98,
                0x19, 0x8d, 0x33, 0xbd, 0x71, 0xe6, 0xf7, 0x56, 0xd0, 0x8e, 0xdf, 0x46, 0x2a, 0x8f,
                0x62, 0xa2, 0x1b, 0x80,
            ],
            Elements::FullRightShift64_4 => [
                0xde, 0xe4, 0xda, 0xd6, 0x7a, 0x5d, 0xdc, 0xc3, 0x5d, 0xa1, 0xa7, 0x90, 0x63, 0xca,
                0x97, 0x5f, 0x81, 0x34, 0xc8, 0xea, 0xc5, 0x6a, 0x9f, 0x55, 0x5d, 0x2b, 0x0e, 0x13,
                0xda, 0x10, 0x99, 0x4d,
            ],
            Elements::FullRightShift64_8 => [
                0x9c, 0xd7, 0x78, 0x03, 0xfc, 0x38, 0x9c, 0x94, 0xff, 0xf2, 0x86, 0xda, 0x0b, 0x37,
                0x4b, 0x89, 0xfe, 0xeb, 0x3d, 0xaa, 0x38, 0xce, 0x67, 0xca, 0xb0, 0x22, 0x0d, 0xab,
                0xee, 0xfe, 0x23, 0xa2,
            ],
            Elements::FullRightShift8_1 => [
                0xee, 0x23, 0xff, 0xf0, 0x7d, 0xe5, 0x3c, 0xc3, 0x71, 0x09, 0xa4, 0x7f, 0x9f, 0xde,
                0x3c, 0x74, 0x44, 0x7a, 0xe8, 0x31, 0xce, 0xe9, 0xac, 0x4d, 0xb7, 0x90, 0xcd, 0xe8,
                0xb1, 0x53, 0x23, 0xb2,
            ],
            Elements::FullRightShift8_2 => [
                0x25, 0xe1, 0xde, 0xa1, 0x08, 0xc5, 0xf8, 0x9c, 0xce, 0x5b, 0x3d, 0x5b, 0x0e, 0x07,
                0x92, 0xbe, 0x37, 0x90, 0x1a, 0x5a, 0x65, 0xde, 0xf9, 0x04, 0xdd, 0x51, 0x71, 0x0a,
                0x35, 0x5a, 0xb5, 0x5f,
            ],
            Elements::FullRightShift8_4 => [
                0xd7, 0xf0, 0xa8, 0x3c, 0x41, 0x04, 0x54, 0x3e, 0xc7, 0x5b, 0x5e, 0xe7, 0x5b, 0xf5,
                0xf7, 0x91, 0x5d, 0x65, 0xfa, 0x50, 0xc2, 0x09, 0x5d, 0xe2, 0xa3, 0x56, 0x70, 0xa5,
                0x05, 0xbe, 0x12, 0x9a,
            ],
            Elements::FullSubtract16 => [
                0x95, 0xea, 0x5e, 0x54, 0xc5, 0x60, 0x3f, 0x2f, 0x78, 0xac, 0xf6, 0xb8, 0xa8, 0x7a,
                0x63, 0xb3, 0xac, 0xc7, 0xb6, 0x5f, 0x2b, 0x87, 0xb6, 0x90, 0x4b, 0x98, 0x30, 0xfa,
                0x91, 0x21, 0x2c, 0x8c,
            ],
            Elements::FullSubtract32 => [
                0x32, 0x96, 0x35, 0x26, 0xfc, 0x60, 0x89, 0xf5, 0xec, 0x7d, 0xa5, 0x84, 0xfb, 0xee,
                0x32, 0x37, 0x63, 0x1c, 0x9b, 0x12, 0x81, 0xf6, 0xf4, 0x46, 0xd9, 0x9c, 0x9b, 0x50,
                0xc8, 0x0c, 0x76, 0xdb,
            ],
            Elements::FullSubtract64 => [
                0x15, 0xd4, 0x8a, 0x43, 0x24, 0x79, 0xb4, 0x51, 0xa6, 0xe2, 0xc1, 0x8f, 0x43, 0xfb,
                0x0d, 0xfc, 0x9a, 0xeb, 0x6f, 0xce, 0x04, 0x03, 0x50, 0x27, 0xb0, 0x33, 0xaa, 0x99,
                0xfb, 0x14, 0x34, 0x47,
            ],
            Elements::FullSubtract8 => [
                0x2b, 0xd4, 0xf0, 0xb3, 0xa0, 0xa7, 0x58, 0xac, 0x39, 0xf1, 0x58, 0x1b, 0x2c, 0x34,
                0xd7, 0xf4, 0x14, 0xdb, 0x4d, 0x8e, 0x1b, 0xc1, 0x19, 0xd7, 0xf8, 0x92, 0x91, 0x35,
                0x48, 0x0e, 0x9a, 0xff,
            ],
            Elements::GeIsOnCurve => [
                0x05, 0x72, 0xae, 0xf3, 0x63, 0x00, 0x50, 0x8c, 0x96, 0xed, 0xa4, 0xa4, 0xfe, 0xc8,
                0x57, 0x9d, 0x8d, 0x8a, 0x43, 0xbd, 0x54, 0x0c, 0xb6, 0xdd, 0xec, 0xc8, 0x2a, 0x49,
                0xe6, 0x68, 0xf7, 0xde,
            ],
            Elements::GeNegate => [
                0x90, 0xf3, 0x3d, 0x91, 0x0f, 0x85, 0x8f, 0x9b, 0x3b, 0x84, 0x0d, 0xdb, 0xc0, 0x3d,
                0x8a, 0x39, 0xaf, 0x81, 0x02, 0x14, 0x85, 0x3b, 0xad, 0x3c, 0x90, 0x5f, 0x18, 0x29,
                0x6f, 0xdf, 0xcb, 0x58,
            ],
            Elements::GejAdd => [
                0x00, 0x87, 0x1c, 0x07, 0x66, 0x28, 0x6a, 0x50, 0x0a, 0xce, 0xa0, 0x5f, 0x7a, 0x1f,
                0xd9, 0x8c, 0x3e, 0x52, 0x46, 0xc1, 0x94, 0x67, 0x71, 0xe4, 0x73, 0x8e, 0x75, 0x82,
                0x09, 0x1c, 0xca, 0x0d,
            ],
            Elements::GejDouble => [
                0x5f, 0xc4, 0x80, 0xc5, 0x05, 0xb6, 0x7d, 0xdd, 0xb9, 0xce, 0x84, 0xb0, 0xa8, 0xee,
                0x5a, 0x7e, 0x4e, 0xf0, 0xe9, 0x36, 0x59, 0xd4, 0x45, 0x21, 0xe6, 0xa6, 0x64, 0xac,
                0x5d, 0x65, 0x56, 0xf8,
            ],
            Elements::GejEquiv => [
                0xe8, 0x7f, 0x98, 0xd9, 0x1d, 0x9e, 0xbd, 0xec, 0x16, 0x7a, 0xba, 0x00, 0xf8, 0xe8,
                0xfc, 0x6d, 0xab, 0x80, 0x79, 0xc7, 0x6b, 0x4d, 0x9c, 0x3d, 0x88, 0xf1, 0xf8, 0x4c,
                0x8e, 0x6c, 0xd2, 0x74,
            ],
            Elements::GejGeAdd => [
                0xa2, 0x4d, 0x04, 0x9f, 0xf1, 0xe4, 0xcf, 0x37, 0x84, 0xc1, 0xb7, 0xd6, 0xd1, 0xba,
                0x09, 0xfd, 0x17, 0xfe, 0xcf, 0xeb, 0x55, 0x80, 0xea, 0xb5, 0xf1, 0x1e, 0x8f, 0x8e,
                0xb9, 0xd9, 0xad, 0xef,
            ],
            Elements::GejGeAddEx => [
                0x1e, 0x0e, 0x26, 0xd0, 0xb2, 0x9a, 0xe6, 0x3d, 0x41, 0xe6, 0x76, 0x7e, 0x01, 0x6e,
                0x7e, 0x24, 0x86, 0xe4, 0xf5, 0xd8, 0xdc, 0x2c, 0xf6, 0x65, 0x02, 0x22, 0x03, 0x16,
                0x40, 0xf1, 0x73, 0x3a,
            ],
            Elements::GejGeEquiv => [
                0x3b, 0x51, 0xda, 0xcd, 0x29, 0xff, 0x5c, 0xd3, 0xc3, 0x20, 0x45, 0x5f, 0xc3, 0xfa,
                0x1a, 0xe9, 0x61, 0x21, 0x29, 0xaa, 0x8c, 0x8e, 0x23, 0x74, 0x60, 0xc3, 0xca, 0x2a,
                0xd5, 0x4e, 0x8f, 0x58,
            ],
            Elements::GejInfinity => [
                0x88, 0xd4, 0x64, 0x2c, 0xfc, 0x2b, 0x52, 0xd0, 0x90, 0xce, 0x6e, 0x89, 0x5c, 0x20,
                0xda, 0x2e, 0xfb, 0x0d, 0xf6, 0xfe, 0x84, 0xf2, 0x27, 0x22, 0xbc, 0x46, 0x11, 0x1c,
                0xc6, 0xbe, 0x5c, 0xda,
            ],
            Elements::GejIsInfinity => [
                0x80, 0xf5, 0x28, 0xe5, 0xd8, 0x56, 0x72, 0xdc, 0x8d, 0x9c, 0x26, 0x4f, 0x67, 0xc7,
                0xb7, 0x27, 0x00, 0xfa, 0xad, 0x89, 0x97, 0x2a, 0x7e, 0x1d, 0x27, 0xd0, 0x49, 0xc9,
                0x47, 0x4b, 0x6c, 0xd9,
            ],
            Elements::GejIsOnCurve => [
                0x70, 0xce, 0x4f, 0xfc, 0xe2, 0x49, 0x7c, 0xc6, 0x2f, 0x17, 0x0c, 0x57, 0x14, 0xff,
                0x2c, 0xfe, 0xce, 0x90, 0xb4, 0xcb, 0x89, 0xa6, 0xa2, 0x2f, 0xac, 0x26, 0xb1, 0xb5,
                0xc6, 0x6f, 0xaa, 0x10,
            ],
            Elements::GejNegate => [
                0xad, 0x53, 0xf1, 0x79, 0x3f, 0xe0, 0x7b, 0x8d, 0x67, 0x2d, 0x9f, 0x7b, 0x07, 0x41,
                0xe9, 0xed, 0x61, 0x55, 0x7e, 0xff, 0x5b, 0x72, 0x96, 0xc5, 0x68, 0xe0, 0x9b, 0x3d,
                0x19, 0xcf, 0x71, 0x24,
            ],
            Elements::GejNormalize => [
                0x03, 0x33, 0xdf, 0x98, 0xa8, 0x06, 0x0c, 0x93, 0x15, 0xc5, 0xfd, 0xb8, 0x3e, 0xbf,
                0xfe, 0x34, 0x07, 0xbf, 0x9c, 0x33, 0x6b, 0xf4, 0xbb, 0x92, 0x2d, 0xf5, 0x85, 0x88,
                0x7e, 0x1a, 0xcd, 0xc7,
            ],
            Elements::GejRescale => [
                0xf0, 0x0a, 0xd1, 0x8f, 0xb4, 0x92, 0x8c, 0xfa, 0xd0, 0x2c, 0x5b, 0x9d, 0x8b, 0x6f,
                0xd4, 0xb0, 0x5d, 0x7c, 0xb5, 0x49, 0xee, 0x65, 0x98, 0x4d, 0x02, 0x2a, 0x6d, 0xf9,
                0x87, 0x12, 0xb6, 0xd9,
            ],
            Elements::GejXEquiv => [
                0xe3, 0x1c, 0x0f, 0x2c, 0x5d, 0x08, 0x13, 0x9b, 0x4f, 0xeb, 0x09, 0x85, 0x2c, 0x06,
                0xb6, 0xaa, 0x00, 0xb1, 0xd1, 0x3e, 0x62, 0xba, 0xbd, 0x99, 0x82, 0x82, 0x12, 0xdc,
                0xff, 0x82, 0x21, 0x7c,
            ],
            Elements::GejYIsOdd => [
                0xe3, 0x4c, 0x86, 0x7d, 0xe1, 0x6b, 0x2f, 0x65, 0x61, 0x09, 0xa7, 0x38, 0x72, 0xb0,
                0xb5, 0xba, 0x55, 0xca, 0x3c, 0x2d, 0xbe, 0xa9, 0xc2, 0xc6, 0xe4, 0xcb, 0x19, 0xad,
                0x18, 0xc0, 0x6f, 0x56,
            ],
            Elements::Generate => [
                0xa3, 0xc5, 0x5b, 0xef, 0x32, 0xa3, 0x50, 0xd9, 0x0d, 0x5c, 0x3d, 0xac, 0x24, 0x76,
                0x7a, 0x03, 0x86, 0x7f, 0xaf, 0x7a, 0x73, 0x27, 0x77, 0x03, 0x89, 0x5a, 0x27, 0xcb,
                0x6b, 0x44, 0x25, 0x2d,
            ],
            Elements::GenesisBlockHash => [
                0x33, 0x6f, 0xf7, 0x3d, 0x00, 0x2f, 0x37, 0x54, 0x9c, 0x48, 0x1f, 0xd4, 0xaa, 0xf1,
                0x5a, 0x58, 0x60, 0x77, 0x9d, 0xf2, 0x83, 0x34, 0x48, 0x71, 0x41, 0xf4, 0x09, 0xb4,
                0xb2, 0x9e, 0xfb, 0x65,
            ],
            Elements::HashToCurve => [
                0x4d, 0xc3, 0xeb, 0x6a, 0x6f, 0xbe, 0x37, 0x4b, 0xa7, 0x69, 0x64, 0x72, 0x63, 0xa6,
                0x95, 0xa4, 0x5d, 0x09, 0x21, 0x13, 0x8a, 0x17, 0xf0, 0x03, 0xc4, 0xab, 0xb7, 0x58,
                0xd8, 0x38, 0xb1, 0x33,
            ],
            Elements::High1 => [
                0x97, 0xa1, 0x43, 0xf0, 0x4c, 0xb6, 0x03, 0xf6, 0x5f, 0x84, 0xa8, 0x0d, 0x31, 0xc3,
                0x36, 0x4f, 0x8f, 0xda, 0x22, 0x97, 0x3a, 0x9a, 0xe6, 0x95, 0xa5, 0x81, 0x89, 0xc3,
                0x14, 0x63, 0xa8, 0xbf,
            ],
            Elements::High16 => [
                0x62, 0x10, 0xac, 0x71, 0x36, 0x58, 0x6c, 0x73, 0xa0, 0x9c, 0x94, 0x21, 0xa4, 0x0e,
                0x30, 0x8c, 0x44, 0x91, 0xea, 0xce, 0x9b, 0x5b, 0x36, 0x95, 0xd6, 0x1f, 0x4c, 0x81,
                0x96, 0xa6, 0x9d, 0xc8,
            ],
            Elements::High32 => [
                0x71, 0x94, 0x24, 0xb1, 0xac, 0xd3, 0x5b, 0x13, 0x73, 0x58, 0x06, 0x90, 0xa7, 0xec,
                0x0b, 0x8f, 0xb4, 0x86, 0x14, 0x5c, 0x9c, 0xde, 0x72, 0x8d, 0xa7, 0x98, 0x46, 0x93,
                0xe9, 0x5f, 0xc7, 0xc0,
            ],
            Elements::High64 => [
                0x8c, 0x5d, 0x44, 0x09, 0x34, 0xdf, 0xdc, 0xf2, 0x75, 0xa2, 0x1c, 0xf0, 0x87, 0xac,
                0x12, 0x7d, 0xa7, 0x57, 0x15, 0xb5, 0xda, 0xc9, 0xc6, 0x93, 0xc2, 0xaf, 0xd7, 0xc7,
                0x18, 0xdc, 0x0f, 0xfb,
            ],
            Elements::High8 => [
                0x3a, 0x5c, 0xe0, 0x0e, 0x15, 0xe3, 0x18, 0x08, 0x51, 0xc2, 0x00, 0x21, 0x1f, 0x1c,
                0x82, 0xda, 0xa3, 0x3e, 0xc8, 0x76, 0x38, 0x24, 0x8a, 0x4b, 0xf1, 0x13, 0x40, 0x7c,
                0x6b, 0x16, 0xac, 0x4f,
            ],
            Elements::Increment16 => [
                0x80, 0xf6, 0xcb, 0xbc, 0x09, 0xb7, 0x8c, 0xea, 0x76, 0xc8, 0x13, 0x90, 0xd3, 0xed,
                0x98, 0x9b, 0x70, 0xe4, 0x39, 0x16, 0x1e, 0xff, 0xaf, 0x9a, 0x62, 0xc6, 0x4b, 0x1b,
                0x95, 0x9c, 0xd0, 0x30,
            ],
            Elements::Increment32 => [
                0x5a, 0x96, 0x3c, 0xa4, 0xad, 0xa6, 0x61, 0x9a, 0x80, 0x53, 0x46, 0xe8, 0x09, 0x95,
                0x03, 0xe4, 0x78, 0x25, 0xbe, 0x5c, 0xf3, 0xc9, 0xa8, 0x9b, 0xfd, 0xbe, 0x2f, 0x19,
                0x1c, 0x32, 0xa2, 0xe0,
            ],
            Elements::Increment64 => [
                0x86, 0x57, 0x52, 0x36, 0x83, 0xc2, 0xa0, 0x5c, 0x0e, 0x09, 0x98, 0xcd, 0xa7, 0xb9,
                0x4e, 0x8b, 0x03, 0x65, 0xee, 0x83, 0x22, 0xa1, 0x4c, 0x5a, 0x37, 0x22, 0xb5, 0x15,
                0xbb, 0x68, 0x1f, 0x74,
            ],
            Elements::Increment8 => [
                0xd1, 0xdb, 0x8a, 0x9e, 0xce, 0xd1, 0x7e, 0x21, 0x99, 0x65, 0x26, 0xbc, 0x73, 0xbe,
                0x8e, 0x57, 0x98, 0x68, 0x8d, 0xa3, 0xc2, 0xef, 0x8b, 0x8d, 0x5b, 0xa2, 0x55, 0xd1,
                0x1e, 0xd8, 0x18, 0x3e,
            ],
            Elements::InputAmount => [
                0xbe, 0x74, 0x0f, 0x91, 0xe4, 0xd7, 0xb4, 0xb7, 0xe6, 0x2c, 0xd1, 0x4a, 0x61, 0xcc,
                0x00, 0xa2, 0x4c, 0x3a, 0x1f, 0xb7, 0x6c, 0x62, 0xf7, 0x7b, 0x9c, 0x1f, 0x92, 0xa2,
                0x77, 0xf5, 0x5f, 0xf0,
            ],
            Elements::InputAmountsHash => [
                0x5a, 0x06, 0xd0, 0xb9, 0xf1, 0x1a, 0x19, 0xfd, 0x32, 0xe8, 0x3e, 0x9d, 0x28, 0x11,
                0x1b, 0xd2, 0x5f, 0x46, 0x52, 0x8b, 0x65, 0x40, 0x45, 0x04, 0xc4, 0xc6, 0xe6, 0x8e,
                0x64, 0x5b, 0xb0, 0x4e,
            ],
            Elements::InputAnnexHash => [
                0x21, 0x76, 0x9e, 0x4f, 0x92, 0x82, 0x5d, 0x7a, 0x82, 0x15, 0x5e, 0x10, 0x2e, 0x10,
                0x60, 0x5a, 0x82, 0x85, 0x53, 0x0f, 0xfd, 0xf7, 0x5b, 0xaa, 0x2a, 0x8c, 0xde, 0xd3,
                0x62, 0x8d, 0xb3, 0x16,
            ],
            Elements::InputAnnexesHash => [
                0x26, 0x0d, 0xc7, 0x1d, 0x41, 0x4e, 0xb8, 0x59, 0xc4, 0xa2, 0xcb, 0x33, 0xca, 0x74,
                0x06, 0xcf, 0x06, 0x7b, 0xc3, 0xc2, 0x1c, 0x33, 0x99, 0x03, 0x5f, 0xeb, 0xaf, 0xd4,
                0x0e, 0x90, 0x29, 0xc9,
            ],
            Elements::InputAsset => [
                0x60, 0x72, 0xd8, 0x1b, 0x12, 0x01, 0x5a, 0xb2, 0x97, 0xef, 0xa1, 0xe8, 0xa7, 0x71,
                0x38, 0x04, 0x26, 0xef, 0x0a, 0x12, 0xf9, 0x0d, 0x61, 0xe1, 0xd4, 0x19, 0xd0, 0x82,
                0xe6, 0x7b, 0x75, 0x65,
            ],
            Elements::InputHash => [
                0xb5, 0xd0, 0xd7, 0x77, 0x85, 0x1e, 0x89, 0xd1, 0x41, 0xd3, 0xcd, 0xa1, 0xb0, 0xf2,
                0x51, 0xe9, 0xc3, 0x10, 0x11, 0xd1, 0xb0, 0x1c, 0x29, 0x39, 0x77, 0x11, 0x96, 0x62,
                0xc3, 0x49, 0x95, 0x9e,
            ],
            Elements::InputOutpointsHash => [
                0xda, 0xd2, 0x60, 0x93, 0x0c, 0x5f, 0x4d, 0x23, 0x8d, 0x83, 0x56, 0x5a, 0x67, 0x57,
                0xe1, 0x47, 0x00, 0x30, 0xcb, 0x29, 0xe0, 0xfd, 0x1e, 0x18, 0xfc, 0x46, 0xed, 0xf3,
                0x1e, 0x65, 0x05, 0x57,
            ],
            Elements::InputPegin => [
                0x98, 0xf0, 0x57, 0x5a, 0x0a, 0x97, 0x8d, 0x26, 0x62, 0x6b, 0x79, 0xc8, 0x42, 0x4b,
                0x4a, 0x5d, 0x16, 0xd9, 0x42, 0x44, 0xca, 0x60, 0x6c, 0x8a, 0x63, 0xbe, 0xe7, 0xe4,
                0xb0, 0x50, 0x91, 0x22,
            ],
            Elements::InputPrevOutpoint => [
                0x7a, 0xd2, 0x6d, 0x34, 0x37, 0xc5, 0xae, 0xba, 0x3b, 0x3d, 0x6b, 0x54, 0x12, 0x8d,
                0x4d, 0x5e, 0x9b, 0x99, 0xe9, 0xed, 0x50, 0xfe, 0x7e, 0x07, 0x67, 0x4c, 0xc7, 0x34,
                0x80, 0x55, 0xd8, 0x95,
            ],
            Elements::InputScriptHash => [
                0xa0, 0x04, 0xc2, 0x3c, 0x28, 0xe3, 0xc9, 0x38, 0x9c, 0x2a, 0x7d, 0xe1, 0xf5, 0x09,
                0x13, 0x64, 0x3a, 0x7e, 0xc2, 0x59, 0x77, 0xa3, 0x1b, 0xd9, 0x12, 0xe1, 0x48, 0xd6,
                0x7b, 0x82, 0xb6, 0xda,
            ],
            Elements::InputScriptSigHash => [
                0x25, 0xa5, 0x9e, 0x2a, 0xed, 0x6b, 0xe1, 0x4c, 0xc0, 0xb0, 0x8f, 0x91, 0xf8, 0x1c,
                0xe3, 0x71, 0xe3, 0x96, 0xfe, 0x38, 0xd7, 0x19, 0xc3, 0x3d, 0x38, 0x50, 0x28, 0x66,
                0x8a, 0xe2, 0xeb, 0x87,
            ],
            Elements::InputScriptSigsHash => [
                0xc4, 0x7d, 0xc3, 0x3e, 0x1f, 0x78, 0x02, 0x63, 0x49, 0x28, 0xf1, 0xf5, 0xd6, 0x38,
                0x55, 0x8b, 0x8f, 0x41, 0x7e, 0xb2, 0xbf, 0xf6, 0x9f, 0x54, 0x2e, 0x8c, 0xeb, 0x98,
                0xbe, 0x1f, 0x02, 0xcd,
            ],
            Elements::InputScriptsHash => [
                0xcf, 0xa0, 0x56, 0xe1, 0x24, 0x9e, 0x8e, 0xb1, 0xcd, 0xb7, 0xfe, 0xe1, 0x23, 0x83,
                0xef, 0xa6, 0x62, 0x02, 0x8e, 0x7a, 0xdf, 0xce, 0xfa, 0x4f, 0x8a, 0xa5, 0x19, 0xf6,
                0x51, 0xca, 0xb9, 0xbe,
            ],
            Elements::InputSequence => [
                0x9f, 0x7a, 0x66, 0x94, 0xe5, 0x99, 0x13, 0x43, 0x57, 0xc5, 0xee, 0x0b, 0x1e, 0xe6,
                0xd0, 0xae, 0x5f, 0x95, 0x4d, 0x5c, 0xd1, 0xb1, 0x0e, 0x09, 0x20, 0x19, 0x7a, 0x33,
                0xec, 0x87, 0x7a, 0x4c,
            ],
            Elements::InputSequencesHash => [
                0x4b, 0x8b, 0x17, 0xe2, 0x5e, 0x91, 0x81, 0x56, 0xd5, 0x62, 0x55, 0x86, 0x01, 0x9a,
                0xa0, 0xf7, 0x6b, 0x79, 0xa5, 0x3a, 0x24, 0x49, 0x5d, 0xe3, 0xe3, 0x9c, 0x48, 0xbb,
                0x03, 0x72, 0x2e, 0xda,
            ],
            Elements::InputUtxoHash => [
                0xd6, 0x45, 0x3a, 0xee, 0xb2, 0xb1, 0x8b, 0x4c, 0x1b, 0xb0, 0x2b, 0x33, 0x51, 0xce,
                0x51, 0x35, 0x45, 0xcb, 0x7f, 0x10, 0x62, 0xf2, 0xa3, 0xfa, 0xc6, 0x80, 0xd0, 0xbb,
                0xca, 0xc6, 0x3e, 0x86,
            ],
            Elements::InputUtxosHash => [
                0xde, 0x24, 0x82, 0x5f, 0xdb, 0x9d, 0x56, 0xda, 0x36, 0x38, 0xbd, 0x48, 0x00, 0x05,
                0xc3, 0xdb, 0x57, 0x4f, 0xdf, 0x11, 0xc1, 0x53, 0x03, 0xb2, 0x42, 0xe2, 0x9b, 0xdb,
                0xc5, 0x1d, 0xf3, 0x89,
            ],
            Elements::InputsHash => [
                0x56, 0x78, 0x27, 0x5e, 0xde, 0x87, 0x21, 0xf1, 0xab, 0xda, 0x5c, 0xb4, 0x60, 0xb3,
                0x02, 0x38, 0x11, 0xd2, 0x3e, 0x47, 0xcc, 0x1f, 0x11, 0xc1, 0xf4, 0x45, 0x80, 0x35,
                0xcd, 0xc3, 0xd7, 0xec,
            ],
            Elements::InternalKey => [
                0x6e, 0x04, 0x87, 0x32, 0x5d, 0xf1, 0xac, 0x05, 0xd2, 0xd7, 0xde, 0xcb, 0xee, 0x3f,
                0xc1, 0xf5, 0xd6, 0x3d, 0xf0, 0x47, 0xf6, 0x9c, 0x22, 0xb4, 0xd9, 0x3b, 0xc4, 0x7c,
                0xba, 0xb4, 0x49, 0x41,
            ],
            Elements::IsOne16 => [
                0x87, 0x7c, 0xd0, 0x01, 0xd5, 0xe7, 0xb7, 0x4e, 0xdd, 0x1a, 0x4a, 0x5c, 0x96, 0x56,
                0x41, 0xd4, 0xfb, 0x53, 0x68, 0x2d, 0x7f, 0xef, 0xae, 0x50, 0xb5, 0x14, 0x12, 0x18,
                0xc0, 0x4b, 0xe5, 0xaf,
            ],
            Elements::IsOne32 => [
                0x83, 0x35, 0x4e, 0x97, 0xd6, 0x14, 0x60, 0x0a, 0x49, 0x89, 0x4e, 0xc2, 0xc9, 0xd1,
                0x98, 0x0f, 0x9c, 0x4c, 0x92, 0x8c, 0x15, 0x61, 0xee, 0xca, 0xc9, 0x9c, 0x16, 0x81,
                0x9c, 0x2b, 0x07, 0x91,
            ],
            Elements::IsOne64 => [
                0xb3, 0x55, 0xf6, 0x02, 0xec, 0x76, 0xc4, 0xc4, 0xce, 0x70, 0x77, 0x20, 0xfd, 0x54,
                0x34, 0x32, 0x22, 0xa7, 0xc8, 0xcf, 0xca, 0x43, 0x94, 0x51, 0xf2, 0x3f, 0x98, 0x54,
                0x39, 0x50, 0x78, 0xba,
            ],
            Elements::IsOne8 => [
                0xf2, 0x45, 0x9c, 0xf3, 0x5b, 0x97, 0x20, 0x29, 0xfb, 0xb2, 0x2e, 0x82, 0x19, 0x1a,
                0xce, 0x11, 0x73, 0x2b, 0x1e, 0x08, 0x34, 0x6e, 0x21, 0xcf, 0x0d, 0x30, 0x5b, 0x41,
                0xfa, 0xe8, 0x79, 0x90,
            ],
            Elements::IsZero16 => [
                0x6e, 0xd0, 0x33, 0x25, 0x6f, 0xe6, 0x45, 0xb5, 0xec, 0x3f, 0x59, 0x50, 0x8c, 0x60,
                0x19, 0x2a, 0xc8, 0x76, 0x30, 0x08, 0x91, 0x5c, 0x81, 0x15, 0xe9, 0x29, 0x33, 0x0c,
                0x0f, 0xcd, 0x48, 0x38,
            ],
            Elements::IsZero32 => [
                0x1a, 0xf3, 0x58, 0xa0, 0x6b, 0xe9, 0x3a, 0xc6, 0xf3, 0x7c, 0xbb, 0x7d, 0x25, 0x4d,
                0x7b, 0xf9, 0xd7, 0x18, 0x77, 0x38, 0xba, 0xf1, 0xf2, 0x5d, 0x0b, 0x67, 0xc9, 0xff,
                0xe0, 0x0f, 0x6a, 0x62,
            ],
            Elements::IsZero64 => [
                0xc1, 0x10, 0x0c, 0xfc, 0x16, 0x20, 0x3c, 0xa4, 0x44, 0xf1, 0x60, 0x82, 0x64, 0x5b,
                0x72, 0x4d, 0x3e, 0xcd, 0x23, 0xdf, 0x5d, 0x0c, 0xcf, 0x91, 0xf3, 0x5c, 0x5d, 0x9b,
                0x5a, 0x02, 0xa2, 0xf2,
            ],
            Elements::IsZero8 => [
                0x4d, 0x58, 0x68, 0x0d, 0x8e, 0x1e, 0x86, 0x18, 0xc4, 0x3c, 0xe5, 0x25, 0xf7, 0x86,
                0x61, 0xa3, 0x01, 0x48, 0x67, 0x58, 0x61, 0xa6, 0x12, 0xd1, 0xbf, 0xcf, 0xe3, 0xbb,
                0x5f, 0xa4, 0xca, 0x95,
            ],
            Elements::Issuance => [
                0x1d, 0xd0, 0x33, 0xd3, 0x2c, 0xd8, 0xe8, 0x90, 0x2f, 0x5e, 0xcb, 0x18, 0x33, 0x85,
                0xec, 0x2b, 0x0c, 0x27, 0xda, 0x38, 0xae, 0x8e, 0x7b, 0x59, 0x3f, 0x68, 0xf9, 0xb1,
                0xa4, 0xe6, 0x20, 0x51,
            ],
            Elements::IssuanceAsset => [
                0x7a, 0x36, 0x99, 0xd0, 0x71, 0x03, 0x0b, 0x58, 0xd9, 0xf4, 0x1a, 0xe1, 0x1c, 0x63,
                0x7b, 0xac, 0x97, 0x78, 0x49, 0xf6, 0x9a, 0x04, 0x87, 0xb1, 0x9d, 0x23, 0x36, 0xd1,
                0xa7, 0xc6, 0xb7, 0x05,
            ],
            Elements::IssuanceAssetAmount => [
                0x75, 0xbd, 0x99, 0x55, 0x9c, 0x02, 0x2d, 0xbf, 0x5d, 0x5c, 0xf9, 0x51, 0xcb, 0x33,
                0xea, 0x37, 0x05, 0x05, 0x8b, 0xf8, 0x1f, 0xbc, 0x74, 0xd5, 0x9b, 0xbf, 0xba, 0x9e,
                0x37, 0xa1, 0xbe, 0x0d,
            ],
            Elements::IssuanceAssetAmountsHash => [
                0x8c, 0x09, 0x0d, 0x1a, 0x32, 0xb3, 0x2b, 0x27, 0x1f, 0xa3, 0x76, 0xa3, 0xba, 0x30,
                0x36, 0x16, 0x8c, 0x0f, 0x62, 0xf9, 0xa4, 0x46, 0xac, 0xe2, 0x9b, 0x35, 0x45, 0xc9,
                0xe5, 0x1f, 0x3e, 0x47,
            ],
            Elements::IssuanceAssetProof => [
                0xc0, 0x48, 0x20, 0x71, 0x1d, 0xc2, 0x5a, 0xb0, 0x61, 0xda, 0x6d, 0xe8, 0x32, 0x78,
                0x8d, 0x6d, 0xbf, 0xb5, 0x4c, 0xfe, 0xea, 0xfd, 0xdb, 0x7e, 0x1e, 0xae, 0x60, 0x85,
                0x0a, 0x76, 0x6e, 0xe9,
            ],
            Elements::IssuanceBlindingEntropyHash => [
                0x3c, 0x42, 0x9f, 0xa6, 0x28, 0xe1, 0x2c, 0xcf, 0x9f, 0x16, 0x7a, 0x07, 0xe2, 0x6f,
                0x70, 0xaf, 0x36, 0xcb, 0x34, 0x59, 0x96, 0x4d, 0xbc, 0x14, 0x2b, 0x52, 0x4d, 0x40,
                0x9d, 0xb1, 0x69, 0x9b,
            ],
            Elements::IssuanceEntropy => [
                0x61, 0xf3, 0x14, 0x55, 0xd9, 0x98, 0x28, 0x01, 0x25, 0xdf, 0x98, 0xe5, 0x98, 0x3d,
                0x72, 0xf5, 0xfb, 0xfd, 0xd6, 0x5c, 0x63, 0x69, 0xa0, 0xe7, 0x8d, 0x07, 0x70, 0x2f,
                0x28, 0xe2, 0xff, 0x5e,
            ],
            Elements::IssuanceHash => [
                0x3a, 0x7c, 0x8e, 0xa2, 0xde, 0x57, 0x98, 0x79, 0x27, 0x5b, 0xf1, 0xbe, 0xa7, 0xbe,
                0x52, 0x79, 0xe7, 0x03, 0xa3, 0x6d, 0xc5, 0xf4, 0xe4, 0x88, 0x53, 0xa3, 0xaf, 0x9b,
                0x0c, 0x50, 0x1b, 0x66,
            ],
            Elements::IssuanceRangeProofsHash => [
                0xe0, 0x5a, 0x29, 0xd1, 0x71, 0x61, 0x10, 0x5e, 0x3b, 0x52, 0x21, 0xf4, 0xeb, 0xae,
                0x70, 0x2b, 0x0c, 0x4c, 0x72, 0xb6, 0xcb, 0x70, 0xdf, 0xde, 0xe2, 0xdc, 0x60, 0x7b,
                0xa8, 0xa8, 0xcf, 0x72,
            ],
            Elements::IssuanceToken => [
                0x0c, 0x7e, 0x9b, 0x99, 0xf9, 0xec, 0x40, 0xf6, 0x70, 0x20, 0xf8, 0x3e, 0x73, 0x67,
                0xdb, 0x45, 0x88, 0x03, 0x8e, 0xcd, 0x1b, 0x31, 0x24, 0xb9, 0x5b, 0xe9, 0x82, 0x67,
                0xd5, 0xf3, 0x48, 0xc8,
            ],
            Elements::IssuanceTokenAmount => [
                0x95, 0x54, 0xcc, 0xd2, 0x41, 0x7e, 0xd6, 0xb3, 0x75, 0xb1, 0x78, 0x83, 0xf7, 0xe9,
                0x85, 0x57, 0xe4, 0x48, 0xd8, 0x04, 0xcc, 0xec, 0x8d, 0xc6, 0xa3, 0xa3, 0x8c, 0x60,
                0xf2, 0xf4, 0x09, 0x7a,
            ],
            Elements::IssuanceTokenAmountsHash => [
                0x19, 0x5e, 0x76, 0x66, 0x1a, 0x78, 0xbc, 0x69, 0x1e, 0xf0, 0x13, 0x9e, 0x1f, 0x36,
                0x6a, 0x96, 0x6f, 0x75, 0x58, 0x4a, 0xb7, 0x76, 0x8d, 0xc7, 0x8f, 0xff, 0xab, 0xde,
                0x15, 0x56, 0x01, 0xf6,
            ],
            Elements::IssuanceTokenProof => [
                0x9b, 0xd4, 0x5c, 0x77, 0x4d, 0x41, 0xd2, 0x08, 0x96, 0x8d, 0x9a, 0xde, 0x7d, 0x28,
                0x0d, 0xeb, 0x05, 0xc2, 0x69, 0x9a, 0xb2, 0xb8, 0x70, 0xeb, 0xe6, 0xf4, 0x4a, 0xc3,
                0xb5, 0x2e, 0xc8, 0xfe,
            ],
            Elements::IssuancesHash => [
                0x9a, 0xb9, 0xfd, 0x03, 0xff, 0x1c, 0x69, 0x58, 0x61, 0x74, 0xbc, 0x26, 0xa8, 0x87,
                0x64, 0xf0, 0xdf, 0x9c, 0xc7, 0xdd, 0x6e, 0x61, 0xf3, 0xdc, 0x3e, 0x42, 0x66, 0x60,
                0xf0, 0x91, 0x22, 0x30,
            ],
            Elements::LbtcAsset => [
                0x49, 0x5d, 0xd0, 0xa1, 0xcb, 0x39, 0xee, 0x84, 0xff, 0x45, 0xe1, 0xdf, 0x3d, 0xb4,
                0xb6, 0x9d, 0xe2, 0x7c, 0x37, 0xc3, 0x6d, 0x69, 0xb3, 0x12, 0x97, 0x6c, 0x43, 0xf5,
                0x0e, 0x5d, 0x94, 0x1a,
            ],
            Elements::Le16 => [
                0xd6, 0x17, 0xfe, 0xea, 0xfd, 0x6f, 0xfc, 0x23, 0xfe, 0xff, 0xbe, 0x70, 0x12, 0xce,
                0x3a, 0x03, 0x02, 0xd4, 0xd1, 0x11, 0x66, 0x58, 0x22, 0xb3, 0x04, 0xb7, 0x9a, 0xdb,
                0xcc, 0x9a, 0x16, 0xd7,
            ],
            Elements::Le32 => [
                0x45, 0x74, 0x5c, 0x5b, 0xc7, 0xf8, 0x97, 0x8f, 0x85, 0xb1, 0xb1, 0x4d, 0x49, 0x4a,
                0xf2, 0x1a, 0x8a, 0x51, 0xcc, 0xd8, 0x7f, 0x3f, 0xed, 0xe9, 0x59, 0x74, 0x95, 0x91,
                0x32, 0xae, 0xf9, 0xce,
            ],
            Elements::Le64 => [
                0x3a, 0x8f, 0x08, 0x70, 0x90, 0x0a, 0xf7, 0x1d, 0x42, 0x42, 0xfe, 0x26, 0xad, 0x4d,
                0xfb, 0xed, 0x92, 0xf3, 0x0b, 0xb7, 0x9b, 0x72, 0x73, 0x7d, 0xbc, 0xab, 0x9a, 0xc5,
                0xc0, 0x70, 0xab, 0xe5,
            ],
            Elements::Le8 => [
                0x5e, 0xcc, 0x9b, 0x33, 0x25, 0x44, 0x67, 0x49, 0xaf, 0xa4, 0x09, 0x65, 0xea, 0x21,
                0xe0, 0x11, 0x18, 0xfb, 0x8c, 0x1a, 0xdc, 0xdc, 0x11, 0x21, 0x97, 0x93, 0xcb, 0x2a,
                0xfc, 0x4a, 0x7e, 0x8c,
            ],
            Elements::LeftExtend16_32 => [
                0x8c, 0x99, 0x04, 0x35, 0xb1, 0x35, 0xde, 0x74, 0x57, 0xc2, 0x69, 0x0d, 0x2d, 0xc8,
                0x74, 0x4a, 0x50, 0x66, 0x41, 0xb8, 0x81, 0xf4, 0x1e, 0x5c, 0x17, 0x02, 0x77, 0x65,
                0xc3, 0x52, 0xdd, 0xcb,
            ],
            Elements::LeftExtend16_64 => [
                0x9b, 0x9f, 0xf9, 0xcc, 0x27, 0x13, 0x93, 0x19, 0xb2, 0x24, 0xb7, 0xb2, 0xb8, 0x16,
                0xc9, 0x13, 0xa5, 0x68, 0xbf, 0xd0, 0x0b, 0xe1, 0xf3, 0x83, 0xc0, 0x26, 0xbc, 0xff,
                0xe9, 0xbf, 0xe7, 0x12,
            ],
            Elements::LeftExtend1_16 => [
                0xb8, 0xff, 0x8d, 0xc1, 0xa0, 0x4c, 0xa7, 0x16, 0x49, 0x17, 0xf4, 0xc4, 0x50, 0x68,
                0x8c, 0x83, 0xdd, 0x41, 0x6c, 0xef, 0x7b, 0x0f, 0xab, 0xdd, 0x16, 0x92, 0xfa, 0xe6,
                0xbf, 0xf7, 0xb4, 0xa6,
            ],
            Elements::LeftExtend1_32 => [
                0x22, 0x53, 0xa4, 0x52, 0xb9, 0x99, 0x02, 0xab, 0xcf, 0x15, 0x49, 0x6d, 0xf1, 0x9d,
                0x31, 0x12, 0xa1, 0xce, 0xf5, 0x9b, 0x9a, 0xdc, 0xee, 0x20, 0x6c, 0x0d, 0x8d, 0xce,
                0xa6, 0x28, 0xd0, 0x73,
            ],
            Elements::LeftExtend1_64 => [
                0xc8, 0x59, 0x9c, 0x85, 0x75, 0xed, 0xb7, 0xc2, 0x60, 0x40, 0x2e, 0xf2, 0xf2, 0x6d,
                0xd4, 0x91, 0xcb, 0x5e, 0x4d, 0x38, 0x18, 0xff, 0x2e, 0x95, 0x85, 0xc8, 0xd3, 0xe7,
                0x81, 0x2d, 0xb5, 0xaa,
            ],
            Elements::LeftExtend1_8 => [
                0xcf, 0xe0, 0x22, 0x00, 0x5f, 0x6b, 0xad, 0x4b, 0x25, 0xb5, 0x1e, 0x9e, 0xbe, 0x92,
                0x94, 0x24, 0x37, 0x3f, 0xf1, 0x97, 0xce, 0xca, 0x62, 0xb9, 0xe0, 0x69, 0xab, 0x08,
                0xda, 0x9f, 0x38, 0xf2,
            ],
            Elements::LeftExtend32_64 => [
                0x3f, 0x66, 0x84, 0x04, 0x7e, 0xda, 0xfb, 0x76, 0xf1, 0x1f, 0xbf, 0x59, 0x54, 0x99,
                0xc5, 0xab, 0xa9, 0xc4, 0xba, 0x55, 0xca, 0xb5, 0x87, 0xcd, 0xfe, 0xba, 0xd4, 0x57,
                0xb5, 0x5b, 0xf5, 0xbb,
            ],
            Elements::LeftExtend8_16 => [
                0xdb, 0xd5, 0x5f, 0x41, 0x70, 0x15, 0x38, 0x53, 0xba, 0xd0, 0xe0, 0x84, 0xf9, 0xe1,
                0xa7, 0xe7, 0x5a, 0x7a, 0x0d, 0xc9, 0xd8, 0x92, 0x07, 0x75, 0x57, 0x5b, 0x0d, 0x48,
                0xe5, 0x07, 0xaf, 0x2f,
            ],
            Elements::LeftExtend8_32 => [
                0x5b, 0x5b, 0x45, 0x67, 0x6a, 0x75, 0x03, 0xee, 0xd0, 0x85, 0x57, 0x38, 0x69, 0xdb,
                0xc5, 0x80, 0x0b, 0x35, 0x02, 0x9d, 0x14, 0x02, 0x90, 0xac, 0x20, 0x89, 0x14, 0x89,
                0xbd, 0x2a, 0xa7, 0xd5,
            ],
            Elements::LeftExtend8_64 => [
                0x7b, 0x9e, 0xa1, 0x48, 0xa3, 0x0f, 0x2a, 0xf4, 0xd4, 0x00, 0x1d, 0x4f, 0x25, 0xb0,
                0xbf, 0x4f, 0xdd, 0x67, 0xc7, 0xd0, 0xf1, 0x34, 0xd7, 0xef, 0x3f, 0x67, 0x8f, 0x72,
                0x19, 0x00, 0x2b, 0xcf,
            ],
            Elements::LeftPadHigh16_32 => [
                0xe8, 0xc2, 0xd8, 0x5a, 0x7b, 0x7b, 0x2a, 0x8e, 0xbb, 0x5b, 0x0f, 0x21, 0x2f, 0xc8,
                0x45, 0x0d, 0xc1, 0xd3, 0xa4, 0x68, 0x22, 0xfb, 0x21, 0xe8, 0x6e, 0x3f, 0xee, 0x02,
                0x0a, 0xf9, 0x73, 0x8f,
            ],
            Elements::LeftPadHigh16_64 => [
                0x61, 0x3b, 0x85, 0xd2, 0xa7, 0x51, 0xb3, 0xe5, 0x1f, 0xbc, 0x59, 0xa1, 0xde, 0xdd,
                0x1f, 0xc7, 0x93, 0x36, 0x5e, 0x40, 0x71, 0xdc, 0x1e, 0x01, 0x41, 0x08, 0xd8, 0x92,
                0x0d, 0x41, 0xd4, 0x70,
            ],
            Elements::LeftPadHigh1_16 => [
                0xe8, 0x2b, 0x00, 0xef, 0xd7, 0xdc, 0xc3, 0x4e, 0x96, 0xe8, 0xf3, 0xe5, 0x1e, 0xad,
                0x12, 0xd3, 0x84, 0x5f, 0x6c, 0x77, 0x85, 0xe4, 0x43, 0x8b, 0x05, 0x45, 0x7d, 0x95,
                0x32, 0x6e, 0x59, 0xe3,
            ],
            Elements::LeftPadHigh1_32 => [
                0x88, 0x58, 0x5b, 0x8c, 0x45, 0x92, 0xd5, 0xd0, 0x83, 0xdf, 0xf6, 0x8e, 0xb5, 0xc2,
                0x30, 0xd0, 0x6c, 0x37, 0x99, 0x5a, 0x6f, 0xed, 0xe2, 0x2c, 0x26, 0xe6, 0x61, 0xa7,
                0x51, 0x6e, 0x5b, 0xf4,
            ],
            Elements::LeftPadHigh1_64 => [
                0xc2, 0x7e, 0x3d, 0x02, 0x94, 0x92, 0x20, 0x58, 0x89, 0x0d, 0x5c, 0x8b, 0x67, 0x6c,
                0x1d, 0xae, 0xfd, 0xde, 0x65, 0x31, 0xe8, 0xae, 0x6d, 0x79, 0x37, 0x53, 0x92, 0x72,
                0x3e, 0xad, 0x9c, 0x03,
            ],
            Elements::LeftPadHigh1_8 => [
                0x14, 0x3e, 0x12, 0x39, 0x2b, 0x8f, 0x2e, 0x73, 0x53, 0x71, 0xfe, 0xd0, 0xb4, 0xb6,
                0xd6, 0x23, 0xff, 0xa4, 0xf6, 0x60, 0xe5, 0x39, 0x0c, 0x00, 0xe5, 0x67, 0xcf, 0x21,
                0xa1, 0xc9, 0x20, 0x78,
            ],
            Elements::LeftPadHigh32_64 => [
                0xf4, 0x84, 0x7c, 0x67, 0x56, 0x67, 0xb3, 0xc6, 0xa8, 0x90, 0xfd, 0x5a, 0x6a, 0xdb,
                0x09, 0x27, 0x55, 0x7e, 0xe8, 0x0c, 0xd6, 0xe6, 0x5a, 0xf9, 0xb1, 0xb4, 0x72, 0x95,
                0x72, 0xcd, 0x86, 0x61,
            ],
            Elements::LeftPadHigh8_16 => [
                0x76, 0x18, 0x05, 0x8a, 0x4e, 0x08, 0xeb, 0x52, 0x43, 0xda, 0xd2, 0x05, 0xcc, 0x7e,
                0x8d, 0x25, 0x47, 0x38, 0x0d, 0xa0, 0x5e, 0xc5, 0x41, 0x1e, 0xfc, 0x37, 0x2b, 0xaa,
                0x2b, 0xb1, 0x2d, 0xa6,
            ],
            Elements::LeftPadHigh8_32 => [
                0x3d, 0xfe, 0xd5, 0xc0, 0xa9, 0xb2, 0x6b, 0x8f, 0x4a, 0x8e, 0xab, 0xd6, 0xfb, 0xed,
                0x87, 0xbe, 0x45, 0x0d, 0xe7, 0x95, 0xf5, 0x41, 0x95, 0x3e, 0xec, 0x16, 0xa6, 0xd5,
                0xaa, 0x82, 0x5a, 0x56,
            ],
            Elements::LeftPadHigh8_64 => [
                0xce, 0x7c, 0x40, 0x7e, 0x4b, 0x56, 0x17, 0x21, 0xd6, 0x6c, 0x1b, 0xb0, 0xd1, 0x7e,
                0xe8, 0x41, 0xb4, 0xd5, 0x04, 0xb4, 0xc4, 0xc0, 0x72, 0x00, 0x22, 0x37, 0x14, 0x0b,
                0x89, 0x09, 0x76, 0x9f,
            ],
            Elements::LeftPadLow16_32 => [
                0xa2, 0x8c, 0x79, 0x24, 0xa4, 0x7b, 0x46, 0xec, 0x73, 0xbc, 0xff, 0x6e, 0x13, 0x28,
                0xd4, 0x39, 0xaa, 0x90, 0x3a, 0xcf, 0x10, 0x3e, 0x9a, 0xa8, 0x0b, 0x65, 0xba, 0x76,
                0xd2, 0xa0, 0x8e, 0x75,
            ],
            Elements::LeftPadLow16_64 => [
                0xb6, 0x25, 0x8a, 0xcd, 0xa2, 0x11, 0x0e, 0xd9, 0x8c, 0x17, 0xbc, 0xa8, 0x27, 0x12,
                0xe3, 0xea, 0x60, 0x86, 0x6f, 0x7d, 0x40, 0x04, 0xc8, 0x3e, 0x8a, 0xe5, 0x24, 0xb7,
                0xba, 0x44, 0x00, 0x8b,
            ],
            Elements::LeftPadLow1_16 => [
                0x1e, 0x1f, 0x6c, 0xc4, 0x24, 0x64, 0x83, 0x75, 0x49, 0xb9, 0x7d, 0x30, 0x7e, 0x28,
                0xa9, 0xc2, 0x36, 0x80, 0x91, 0x4c, 0xd8, 0x6d, 0x65, 0xc3, 0x04, 0x67, 0x93, 0x12,
                0x7b, 0x54, 0xfe, 0x82,
            ],
            Elements::LeftPadLow1_32 => [
                0xc3, 0x38, 0xa1, 0x95, 0x5e, 0x99, 0x82, 0x0d, 0x0e, 0xd3, 0x1a, 0x5a, 0xfe, 0xdd,
                0x13, 0x58, 0xc1, 0x74, 0x44, 0x02, 0x3e, 0x3f, 0x2b, 0x47, 0x33, 0xd9, 0xf6, 0x8c,
                0xb7, 0xb4, 0x0c, 0xd9,
            ],
            Elements::LeftPadLow1_64 => [
                0x68, 0x9e, 0xd5, 0x69, 0xc2, 0x01, 0x52, 0x1e, 0xc1, 0x95, 0x4f, 0x0d, 0xc7, 0xd2,
                0x12, 0x8e, 0x46, 0x5a, 0x52, 0x04, 0x99, 0x19, 0x05, 0x49, 0x85, 0x8d, 0xe9, 0xed,
                0x23, 0x1a, 0x5d, 0x69,
            ],
            Elements::LeftPadLow1_8 => [
                0x5b, 0x51, 0x90, 0x53, 0xfd, 0x2b, 0xb7, 0x58, 0x47, 0x3a, 0xf8, 0xe3, 0x91, 0x0b,
                0xae, 0xf3, 0x3c, 0xc8, 0x01, 0xc0, 0xb1, 0x42, 0x0a, 0xaf, 0x81, 0x4a, 0x7e, 0x72,
                0x54, 0xea, 0x78, 0xf0,
            ],
            Elements::LeftPadLow32_64 => [
                0xac, 0x00, 0xa3, 0x4f, 0xb6, 0xa5, 0x8e, 0x57, 0xad, 0x22, 0x39, 0x50, 0x0e, 0x65,
                0x71, 0x37, 0x5d, 0xfd, 0xa0, 0xce, 0xa1, 0x17, 0x5f, 0xe9, 0x9d, 0x87, 0x5c, 0xd8,
                0x71, 0x81, 0x05, 0xe9,
            ],
            Elements::LeftPadLow8_16 => [
                0xb6, 0xf6, 0x33, 0x4c, 0xc6, 0x60, 0xe3, 0x06, 0x9f, 0x7e, 0x14, 0x37, 0xa1, 0x94,
                0x3f, 0x61, 0x0f, 0xc5, 0xa5, 0xab, 0x8a, 0xa5, 0x10, 0x5b, 0xfc, 0xec, 0xd3, 0xda,
                0x0c, 0x59, 0x63, 0x3c,
            ],
            Elements::LeftPadLow8_32 => [
                0x8a, 0xee, 0x9f, 0x42, 0xda, 0x9b, 0x84, 0x2e, 0x41, 0x18, 0x95, 0x96, 0x59, 0x47,
                0x56, 0xba, 0xd9, 0xba, 0xb2, 0x95, 0x3d, 0xea, 0xe6, 0x68, 0x56, 0xb9, 0xcb, 0x60,
                0x1d, 0x7a, 0xc5, 0x48,
            ],
            Elements::LeftPadLow8_64 => [
                0x61, 0x1d, 0x64, 0xce, 0x94, 0xf8, 0x82, 0xfd, 0xa2, 0x4c, 0x97, 0xad, 0xd1, 0x90,
                0x54, 0x21, 0x2f, 0x46, 0xb3, 0xb9, 0x8e, 0xf2, 0xae, 0x22, 0x79, 0x36, 0x45, 0x39,
                0xb9, 0x3e, 0x2b, 0x8b,
            ],
            Elements::LeftRotate16 => [
                0x25, 0xe2, 0xd4, 0xec, 0xc0, 0x3f, 0x87, 0x65, 0x5e, 0x96, 0x5b, 0x35, 0x7d, 0x6f,
                0xd0, 0xc2, 0xea, 0x36, 0xd1, 0x12, 0x06, 0x8c, 0x96, 0x33, 0x39, 0xde, 0x46, 0x7e,
                0x5c, 0x8e, 0x7d, 0x8e,
            ],
            Elements::LeftRotate32 => [
                0x2d, 0x41, 0x89, 0xb3, 0x12, 0xe8, 0xce, 0xda, 0xaa, 0x38, 0x53, 0xa4, 0x5a, 0x12,
                0x98, 0x6e, 0xe2, 0x62, 0xfb, 0x60, 0x5f, 0x0d, 0x59, 0x2d, 0xcb, 0xb9, 0x61, 0x8f,
                0xe6, 0x7a, 0x25, 0x0b,
            ],
            Elements::LeftRotate64 => [
                0xb8, 0xe6, 0x8e, 0x0a, 0xd6, 0x82, 0xb9, 0x67, 0xf2, 0x4c, 0x09, 0x84, 0xf7, 0xd5,
                0xf8, 0x09, 0xa2, 0x85, 0x97, 0xa0, 0x43, 0x46, 0x18, 0xc8, 0x94, 0x9f, 0xa8, 0x08,
                0xe3, 0xbe, 0x76, 0x14,
            ],
            Elements::LeftRotate8 => [
                0x95, 0x6a, 0x65, 0x3a, 0xe0, 0xb8, 0xf8, 0xc3, 0xf2, 0x9f, 0xd8, 0xf3, 0x31, 0x19,
                0x16, 0x8f, 0xcb, 0xe6, 0x4f, 0x5d, 0x76, 0x5f, 0xa9, 0xff, 0x6b, 0x8e, 0x3b, 0x0d,
                0x96, 0x1a, 0x16, 0x29,
            ],
            Elements::LeftShift16 => [
                0x9b, 0xbc, 0xe2, 0x9e, 0x69, 0x5b, 0xe2, 0xe4, 0x83, 0x0c, 0x7a, 0x93, 0xa0, 0xd2,
                0x15, 0x1b, 0x66, 0x4f, 0xc2, 0x72, 0x06, 0xee, 0xd7, 0xe5, 0x0f, 0xce, 0xf6, 0x02,
                0xd3, 0x45, 0xce, 0x0d,
            ],
            Elements::LeftShift32 => [
                0xf9, 0xc4, 0xbf, 0x07, 0xd3, 0xe7, 0x2e, 0x85, 0xb1, 0xd4, 0x55, 0xf7, 0x34, 0xcf,
                0x1b, 0x11, 0xbe, 0xa5, 0x8e, 0x25, 0x3b, 0x85, 0x4a, 0x1a, 0x09, 0x7b, 0xab, 0x1e,
                0xc2, 0xc6, 0x2e, 0x1f,
            ],
            Elements::LeftShift64 => [
                0x8c, 0xfa, 0x74, 0x1a, 0xa4, 0x1d, 0x82, 0x8a, 0x41, 0x08, 0x3b, 0xb7, 0xcb, 0xdd,
                0x1f, 0x4e, 0xda, 0x5d, 0xcc, 0xac, 0x52, 0x9b, 0x24, 0x7d, 0x18, 0x84, 0x95, 0xb4,
                0x9b, 0xb3, 0x8c, 0x2b,
            ],
            Elements::LeftShift8 => [
                0x95, 0x66, 0x3e, 0x07, 0x7c, 0xad, 0xca, 0x31, 0xb9, 0x59, 0x6b, 0x09, 0x70, 0x6c,
                0xdb, 0x4f, 0xa7, 0x03, 0x87, 0x0f, 0x79, 0x2a, 0x46, 0x35, 0x85, 0x2b, 0x5e, 0x24,
                0x69, 0xe6, 0xfd, 0xba,
            ],
            Elements::LeftShiftWith16 => [
                0x62, 0x14, 0xc4, 0x56, 0x25, 0xd7, 0x04, 0xce, 0xc9, 0x87, 0xb7, 0x96, 0x67, 0x6f,
                0x15, 0x66, 0x1a, 0x6b, 0xf5, 0xdc, 0x0f, 0x6a, 0x51, 0xcb, 0x86, 0x5a, 0x0e, 0x71,
                0xd6, 0x6f, 0xbf, 0x95,
            ],
            Elements::LeftShiftWith32 => [
                0x1b, 0x45, 0x2f, 0xc7, 0xab, 0x5c, 0x71, 0x47, 0x45, 0x4a, 0xf4, 0xd5, 0x59, 0x54,
                0x81, 0xff, 0xac, 0x42, 0xde, 0xa1, 0x06, 0x03, 0x2b, 0x3b, 0x9f, 0x37, 0x5b, 0xed,
                0xcd, 0xa6, 0xf4, 0xd6,
            ],
            Elements::LeftShiftWith64 => [
                0xc3, 0x8b, 0x02, 0xab, 0xcf, 0xf5, 0x14, 0xd9, 0x61, 0x91, 0xa7, 0xfe, 0xfb, 0xa1,
                0xac, 0x16, 0xe9, 0xc1, 0x50, 0xa1, 0x8c, 0xe1, 0xc5, 0xbc, 0xf0, 0x9d, 0x67, 0x55,
                0xe0, 0x36, 0x99, 0x05,
            ],
            Elements::LeftShiftWith8 => [
                0x21, 0x7a, 0xd6, 0xdc, 0x12, 0x92, 0xaa, 0x42, 0xdb, 0xd8, 0x4d, 0xbd, 0x97, 0x1c,
                0x11, 0x8f, 0x02, 0xa9, 0x74, 0x0a, 0x7c, 0xb5, 0x66, 0x1e, 0x90, 0xd4, 0x2d, 0xd5,
                0xca, 0x8c, 0xa4, 0xd9,
            ],
            Elements::Leftmost16_1 => [
                0x3e, 0xa8, 0x9e, 0x43, 0x20, 0x77, 0x94, 0x0d, 0x0b, 0xbf, 0x9e, 0xd2, 0xcf, 0x16,
                0xba, 0x63, 0x11, 0x10, 0xe7, 0xab, 0x9f, 0x19, 0xee, 0xf3, 0xea, 0x92, 0x5a, 0x69,
                0x9f, 0x60, 0xc6, 0x0c,
            ],
            Elements::Leftmost16_2 => [
                0x5c, 0xe2, 0xbd, 0x7a, 0xc3, 0x5a, 0x7c, 0x33, 0x73, 0xc3, 0xdd, 0x60, 0x7f, 0x48,
                0xe5, 0xd4, 0xc7, 0xaa, 0xa6, 0xc6, 0x9f, 0xc4, 0x93, 0x0e, 0xca, 0x14, 0x04, 0x9f,
                0x5d, 0x39, 0xff, 0xab,
            ],
            Elements::Leftmost16_4 => [
                0x10, 0x12, 0xa1, 0x39, 0x3e, 0xd0, 0xf9, 0x1d, 0x75, 0xad, 0x59, 0x12, 0x28, 0x53,
                0x89, 0x3a, 0x7f, 0x25, 0xcd, 0x35, 0xc8, 0x03, 0x6c, 0x7f, 0xa1, 0x95, 0x68, 0x2c,
                0xa1, 0x45, 0x8c, 0x4a,
            ],
            Elements::Leftmost16_8 => [
                0xcc, 0xd3, 0x1e, 0x9e, 0xb1, 0xa1, 0xbb, 0xde, 0x55, 0x5c, 0x0f, 0x73, 0x1a, 0xf2,
                0xd3, 0xd4, 0xff, 0x53, 0x88, 0xfa, 0x14, 0x61, 0x82, 0x6a, 0xa9, 0xc8, 0x93, 0x42,
                0x42, 0xac, 0x75, 0x3f,
            ],
            Elements::Leftmost32_1 => [
                0xb7, 0x14, 0xad, 0x74, 0xae, 0x04, 0x5a, 0xf7, 0x56, 0x80, 0x77, 0x8a, 0x03, 0x27,
                0x61, 0xa4, 0xc7, 0x26, 0xd7, 0xb6, 0xd9, 0x77, 0xbc, 0x93, 0xa4, 0x12, 0x56, 0x54,
                0x3c, 0xae, 0x8d, 0x3d,
            ],
            Elements::Leftmost32_16 => [
                0x1b, 0x20, 0x63, 0x4f, 0xb4, 0x3e, 0xb8, 0x3a, 0x96, 0x8c, 0x3c, 0x81, 0xc0, 0x08,
                0x7c, 0x63, 0xd5, 0xd4, 0xf8, 0xca, 0xcd, 0xbd, 0x3e, 0x0e, 0x9f, 0x9a, 0x3d, 0x75,
                0x91, 0xc3, 0xef, 0x62,
            ],
            Elements::Leftmost32_2 => [
                0x75, 0x2d, 0xda, 0x08, 0xe4, 0x0f, 0xae, 0xa0, 0xf6, 0xc4, 0xee, 0x3d, 0x34, 0x4b,
                0x7c, 0x4e, 0xa1, 0x1b, 0x97, 0x1d, 0xce, 0xc5, 0x55, 0x92, 0xb8, 0x22, 0xee, 0x56,
                0x27, 0x1c, 0xa5, 0xdf,
            ],
            Elements::Leftmost32_4 => [
                0x44, 0xe9, 0xf7, 0x79, 0xa0, 0x29, 0xec, 0xfc, 0x97, 0x62, 0xb8, 0xb6, 0xcb, 0xaf,
                0x09, 0x22, 0xd9, 0x35, 0xfe, 0xa5, 0x15, 0x0a, 0x54, 0x6a, 0x5f, 0xc1, 0xfd, 0xb8,
                0xb9, 0x53, 0x41, 0x34,
            ],
            Elements::Leftmost32_8 => [
                0x54, 0x80, 0x1e, 0xb5, 0xe7, 0x78, 0xcf, 0x6c, 0xda, 0x95, 0xcc, 0xf5, 0x70, 0x28,
                0x6d, 0x81, 0x6d, 0x3a, 0x1f, 0xf1, 0xdd, 0x39, 0xdb, 0x5a, 0xb6, 0x13, 0x6f, 0x0e,
                0xc3, 0xb7, 0x2d, 0xc6,
            ],
            Elements::Leftmost64_1 => [
                0xb3, 0x16, 0xaf, 0x24, 0xc8, 0x6b, 0x39, 0x61, 0x3d, 0x4f, 0xd1, 0xb3, 0x92, 0x6a,
                0x84, 0x13, 0x0e, 0xb7, 0xab, 0x12, 0xfd, 0xef, 0x62, 0x33, 0x17, 0xab, 0x48, 0xf7,
                0x7c, 0xb6, 0x21, 0x45,
            ],
            Elements::Leftmost64_16 => [
                0x8c, 0xf8, 0x54, 0x81, 0xe0, 0xf0, 0x08, 0x38, 0xb5, 0x23, 0x9b, 0xbf, 0xad, 0x13,
                0x82, 0xf0, 0x7b, 0xd0, 0x3c, 0x12, 0x1d, 0x5d, 0x8a, 0xaf, 0xa6, 0xd9, 0x83, 0x41,
                0x6d, 0xe4, 0x5c, 0x32,
            ],
            Elements::Leftmost64_2 => [
                0xda, 0x40, 0xc8, 0x9b, 0x15, 0xc9, 0xe8, 0x6b, 0x02, 0x8c, 0xe9, 0xec, 0x07, 0xb7,
                0xf6, 0x99, 0x5a, 0x5d, 0xdd, 0xa4, 0x85, 0x0a, 0x91, 0xaf, 0x8c, 0x60, 0xe0, 0x2b,
                0xf9, 0x91, 0xfb, 0x0c,
            ],
            Elements::Leftmost64_32 => [
                0x95, 0xf4, 0x6d, 0xb9, 0xd9, 0x06, 0xf0, 0x50, 0x53, 0x45, 0x5e, 0x95, 0x34, 0xeb,
                0x9b, 0x08, 0xb0, 0x9e, 0x38, 0xbc, 0x0f, 0xc6, 0x98, 0xa1, 0x6f, 0x4b, 0x2a, 0x62,
                0x71, 0x07, 0x59, 0xd1,
            ],
            Elements::Leftmost64_4 => [
                0xf5, 0x01, 0xf9, 0x05, 0xfb, 0x8b, 0xab, 0xa1, 0xa7, 0xe8, 0xa6, 0xbf, 0x68, 0xd3,
                0xae, 0x6a, 0x0a, 0xdd, 0x91, 0x95, 0x1b, 0x56, 0x62, 0x9d, 0x59, 0xf4, 0x28, 0x73,
                0x9e, 0x7e, 0x41, 0xa2,
            ],
            Elements::Leftmost64_8 => [
                0xb3, 0x7f, 0x0b, 0xa2, 0xfc, 0xbd, 0x4a, 0xe3, 0x31, 0x6a, 0x4f, 0xe4, 0xf5, 0x8a,
                0xa1, 0xa5, 0x41, 0x74, 0x0c, 0xde, 0x60, 0xed, 0x87, 0xf3, 0x38, 0x62, 0xa2, 0xff,
                0xec, 0xad, 0x44, 0x2f,
            ],
            Elements::Leftmost8_1 => [
                0x5f, 0xdc, 0xfa, 0x9b, 0x9a, 0x4b, 0x65, 0xc7, 0x20, 0x74, 0x71, 0xe5, 0x33, 0x92,
                0x8d, 0x6a, 0x24, 0xf4, 0xb6, 0xff, 0x9b, 0x34, 0x5e, 0xf7, 0x61, 0xb1, 0x48, 0x0a,
                0x8a, 0x05, 0xe3, 0xd7,
            ],
            Elements::Leftmost8_2 => [
                0x62, 0x42, 0x21, 0xe9, 0xf8, 0xa9, 0x16, 0x91, 0x26, 0xc7, 0x33, 0x47, 0x96, 0x48,
                0xc7, 0x3b, 0x68, 0xc6, 0xb8, 0xeb, 0xbb, 0x60, 0xc7, 0x2a, 0xf1, 0xe6, 0xfc, 0x65,
                0xe7, 0xd3, 0x07, 0x23,
            ],
            Elements::Leftmost8_4 => [
                0x1a, 0x48, 0x43, 0xc4, 0x08, 0xe1, 0xd4, 0x6c, 0x9c, 0x93, 0x89, 0x46, 0x34, 0x49,
                0x5f, 0x8a, 0xd6, 0xa6, 0x80, 0xe3, 0x2d, 0xd6, 0xf2, 0x5b, 0xa1, 0x9d, 0xbc, 0x60,
                0xa6, 0x0d, 0x18, 0x97,
            ],
            Elements::LinearCombination1 => [
                0x00, 0xbe, 0xf1, 0x44, 0xda, 0x3f, 0x51, 0x63, 0x31, 0x8c, 0x01, 0xeb, 0x66, 0xcc,
                0x68, 0x1f, 0x29, 0xcc, 0xb9, 0x87, 0xea, 0x2a, 0x88, 0xd0, 0x83, 0x34, 0x5a, 0x1c,
                0xaa, 0x08, 0x2c, 0xe3,
            ],
            Elements::LinearVerify1 => [
                0x0c, 0x9f, 0xae, 0x64, 0xa6, 0x4c, 0xd6, 0x3c, 0xa8, 0x5b, 0xeb, 0xa7, 0x5c, 0x9f,
                0x2c, 0x6e, 0x85, 0x34, 0x3b, 0x74, 0xf2, 0x86, 0x34, 0xea, 0x85, 0xf7, 0x0f, 0xc3,
                0x41, 0xcc, 0xaf, 0xf3,
            ],
            Elements::LockTime => [
                0xee, 0x87, 0xa0, 0x25, 0xd7, 0x33, 0x8a, 0x3c, 0x8f, 0x25, 0x1b, 0xe4, 0x19, 0xef,
                0xb7, 0xee, 0x71, 0x77, 0xcb, 0x73, 0x8f, 0x42, 0xe8, 0xc0, 0x03, 0xfc, 0x46, 0x58,
                0xcc, 0x0e, 0x5b, 0x8b,
            ],
            Elements::Low1 => [
                0xdb, 0x4a, 0x42, 0x4a, 0x20, 0xae, 0xef, 0xa4, 0xe7, 0x42, 0xd5, 0x1d, 0x84, 0x92,
                0x92, 0x18, 0xcb, 0xf7, 0x34, 0x72, 0x61, 0x76, 0xdc, 0x4f, 0xf9, 0xf8, 0xbf, 0x13,
                0xde, 0x10, 0xca, 0x2b,
            ],
            Elements::Low16 => [
                0xa1, 0x14, 0xe9, 0x58, 0x0d, 0xe0, 0x7d, 0x8b, 0x07, 0x7e, 0xb8, 0x89, 0x98, 0x75,
                0x5a, 0x0a, 0x62, 0xbf, 0xe0, 0x85, 0xfb, 0x23, 0x40, 0x4c, 0xd1, 0xe8, 0x78, 0x68,
                0xcd, 0x56, 0xd5, 0xbd,
            ],
            Elements::Low32 => [
                0xa7, 0x17, 0x61, 0x0e, 0x8c, 0x57, 0x71, 0x25, 0x51, 0x40, 0xd6, 0x20, 0x7f, 0xff,
                0x3b, 0xdd, 0x34, 0x64, 0xac, 0xff, 0x59, 0x98, 0xe1, 0x29, 0xaf, 0x8b, 0x9f, 0x4c,
                0x0e, 0x21, 0xb2, 0x3d,
            ],
            Elements::Low64 => [
                0x9a, 0x6f, 0x4a, 0x0a, 0xd2, 0xfb, 0x72, 0xc5, 0x79, 0x42, 0x4c, 0x72, 0xe1, 0x7d,
                0xaa, 0xe6, 0x2b, 0x36, 0x66, 0x95, 0xcd, 0x1a, 0x26, 0x85, 0xfb, 0x0a, 0xb0, 0x2e,
                0x4c, 0x2c, 0xc0, 0xf5,
            ],
            Elements::Low8 => [
                0x69, 0xc9, 0xb9, 0xca, 0xb9, 0xf4, 0x4c, 0xff, 0xae, 0xdf, 0x1c, 0x84, 0x31, 0x46,
                0xbc, 0xc0, 0xb0, 0x3b, 0x0f, 0x0c, 0x13, 0x48, 0x66, 0xaf, 0xd2, 0x3a, 0x61, 0x4f,
                0x01, 0xa7, 0x0a, 0x24,
            ],
            Elements::Lt16 => [
                0x4f, 0x46, 0x5a, 0x49, 0xa9, 0x63, 0xac, 0xce, 0x93, 0xf6, 0xb6, 0xf8, 0x23, 0xeb,
                0x94, 0x72, 0xb4, 0xcc, 0x21, 0xf6, 0xe5, 0x8b, 0x76, 0x57, 0x05, 0x08, 0xba, 0xbd,
                0xf4, 0x4a, 0x8c, 0x97,
            ],
            Elements::Lt32 => [
                0xd3, 0x42, 0x1d, 0xfc, 0x84, 0x67, 0x1c, 0xd7, 0x44, 0x50, 0x82, 0xa9, 0x86, 0xdf,
                0x16, 0x28, 0xed, 0xca, 0xcb, 0xf4, 0x29, 0xa3, 0xc8, 0x09, 0xd2, 0x36, 0x54, 0x97,
                0xe1, 0x17, 0x0b, 0xfd,
            ],
            Elements::Lt64 => [
                0x7c, 0x67, 0xaa, 0x89, 0x93, 0xfc, 0xda, 0x59, 0x30, 0xf7, 0x9d, 0xb2, 0x00, 0x8a,
                0xa6, 0xd7, 0xbe, 0x5f, 0x3b, 0xed, 0x5a, 0xca, 0x5e, 0x03, 0x2d, 0xcc, 0xcf, 0x0b,
                0xe8, 0x4f, 0x62, 0xd1,
            ],
            Elements::Lt8 => [
                0x98, 0x76, 0x3f, 0x78, 0x21, 0x69, 0x09, 0x54, 0xcf, 0x50, 0x81, 0x02, 0x09, 0xdf,
                0x6e, 0x15, 0x57, 0x03, 0x16, 0xbb, 0xa8, 0x9f, 0xfa, 0x9a, 0xe5, 0x56, 0xa9, 0x15,
                0xf3, 0x7b, 0x64, 0x0b,
            ],
            Elements::Maj1 => [
                0xae, 0x8b, 0x91, 0x2e, 0x3a, 0xd4, 0x7f, 0x68, 0x8b, 0xbb, 0x46, 0xc8, 0xcb, 0x6d,
                0x53, 0x33, 0x69, 0xf5, 0x10, 0x9a, 0x27, 0x30, 0x47, 0x1e, 0xab, 0x6e, 0xfe, 0x98,
                0xe9, 0xea, 0x5e, 0x78,
            ],
            Elements::Maj16 => [
                0xf5, 0xa4, 0x1d, 0xa0, 0x37, 0x7f, 0xe6, 0x88, 0xac, 0x2f, 0xcd, 0xf3, 0x5b, 0x6b,
                0x7a, 0x47, 0x2e, 0x78, 0xea, 0x69, 0xfd, 0x2b, 0x17, 0xf7, 0x56, 0x34, 0x2b, 0xaa,
                0x1f, 0x8b, 0x9f, 0xdd,
            ],
            Elements::Maj32 => [
                0xf1, 0xd9, 0x3b, 0x04, 0x6b, 0x02, 0x85, 0xf6, 0xe5, 0x20, 0x46, 0x7f, 0x26, 0x5f,
                0x6a, 0x6a, 0xe3, 0x49, 0x1f, 0x16, 0x78, 0xc8, 0xa8, 0x26, 0xa0, 0x09, 0x9b, 0x9c,
                0xd4, 0x15, 0x99, 0xd4,
            ],
            Elements::Maj64 => [
                0x5e, 0x99, 0x6c, 0x51, 0x51, 0xd7, 0xac, 0xee, 0x4a, 0x7d, 0x9e, 0x22, 0x95, 0xef,
                0x8f, 0x2c, 0x75, 0x54, 0x88, 0x84, 0x4b, 0xfd, 0xe2, 0x5d, 0x5a, 0xcd, 0xfe, 0xa2,
                0x98, 0x44, 0x43, 0x5a,
            ],
            Elements::Maj8 => [
                0xab, 0x6c, 0x4c, 0x41, 0xab, 0x67, 0xeb, 0xf7, 0xea, 0x60, 0x87, 0x0c, 0x9b, 0x2d,
                0x93, 0x10, 0x01, 0x74, 0x29, 0x61, 0x8d, 0x2a, 0xb9, 0x02, 0x68, 0xb5, 0xc5, 0xc2,
                0xb5, 0x37, 0x7f, 0x3b,
            ],
            Elements::Max16 => [
                0xcd, 0xdd, 0x81, 0x69, 0x55, 0x22, 0x97, 0x1b, 0x52, 0x9e, 0xb0, 0xbb, 0x53, 0x81,
                0x8f, 0xe2, 0x28, 0x58, 0xac, 0x6b, 0x03, 0x7c, 0x38, 0x10, 0xec, 0x26, 0x8b, 0x53,
                0xb8, 0x17, 0xa3, 0x8d,
            ],
            Elements::Max32 => [
                0xd9, 0x16, 0xf3, 0x4b, 0xd7, 0x85, 0xb9, 0x38, 0x08, 0xb6, 0x7c, 0x62, 0x28, 0x4f,
                0x94, 0xaa, 0x5d, 0xf4, 0x79, 0x19, 0x59, 0xb4, 0x9b, 0x85, 0x82, 0xb0, 0x58, 0x2b,
                0x82, 0x85, 0x38, 0x83,
            ],
            Elements::Max64 => [
                0x99, 0xef, 0x5a, 0xbf, 0xf7, 0x87, 0x00, 0xbd, 0x93, 0xd1, 0xc9, 0x77, 0xfd, 0x68,
                0x6f, 0x31, 0x21, 0x0f, 0xd1, 0xce, 0x1d, 0x09, 0x4d, 0x23, 0x04, 0x8c, 0x02, 0x6c,
                0x19, 0xfc, 0x98, 0x4a,
            ],
            Elements::Max8 => [
                0x3a, 0xcb, 0xf7, 0x73, 0x3f, 0xd8, 0xef, 0x4f, 0x51, 0x96, 0x7a, 0xa2, 0x45, 0x55,
                0x8b, 0x28, 0x37, 0x4d, 0x54, 0xc6, 0x32, 0xa9, 0xf4, 0x6a, 0xab, 0x6b, 0x2b, 0x1e,
                0xc2, 0xd5, 0x13, 0x9a,
            ],
            Elements::Median16 => [
                0xd6, 0x46, 0x3e, 0x89, 0xdf, 0xe2, 0xc3, 0x34, 0xb0, 0x1f, 0x90, 0x97, 0xb0, 0x1a,
                0x75, 0xc4, 0x75, 0xfb, 0x0f, 0x0f, 0x16, 0x7c, 0xb6, 0xb7, 0x49, 0x76, 0xf1, 0x97,
                0xbd, 0x4c, 0x76, 0xe1,
            ],
            Elements::Median32 => [
                0x01, 0xdc, 0xa1, 0xed, 0xe5, 0xb2, 0x9d, 0x88, 0x48, 0x0b, 0xc4, 0xdc, 0x43, 0xbe,
                0x4e, 0x04, 0xf2, 0xd2, 0x09, 0xee, 0x32, 0xf6, 0xcf, 0x3e, 0xc7, 0x05, 0x0d, 0xa1,
                0x07, 0x35, 0x90, 0x63,
            ],
            Elements::Median64 => [
                0x70, 0x7a, 0x6f, 0xa8, 0xc1, 0x54, 0x95, 0xa2, 0xc4, 0x82, 0x22, 0x63, 0xdd, 0xce,
                0xa5, 0xe0, 0xf0, 0xe8, 0x50, 0xa9, 0x4b, 0x1c, 0xf4, 0xec, 0xc0, 0x65, 0x2f, 0xaf,
                0xee, 0xf1, 0x30, 0xe9,
            ],
            Elements::Median8 => [
                0x26, 0xbf, 0xcb, 0x7b, 0xab, 0xcf, 0x26, 0xc9, 0x7a, 0x84, 0x16, 0x75, 0x45, 0x74,
                0x62, 0x76, 0x59, 0x27, 0x9a, 0xf8, 0x09, 0x55, 0x48, 0x82, 0x40, 0x0e, 0xf8, 0x74,
                0xdf, 0x1c, 0x1a, 0x31,
            ],
            Elements::Min16 => [
                0x63, 0xbb, 0xc0, 0xe5, 0xdb, 0x1c, 0xe0, 0x80, 0x97, 0xd3, 0xf5, 0xbc, 0xc5, 0xbe,
                0x1d, 0x72, 0x09, 0x0b, 0xab, 0x4a, 0x87, 0xc1, 0x0c, 0x80, 0x05, 0x49, 0xca, 0x84,
                0xef, 0xbf, 0xe5, 0xe3,
            ],
            Elements::Min32 => [
                0xaf, 0x4d, 0x61, 0x84, 0x2f, 0x47, 0x90, 0x32, 0x5c, 0x61, 0x45, 0x56, 0xc7, 0x77,
                0x6e, 0xea, 0x45, 0x75, 0x13, 0xca, 0x42, 0x2f, 0x30, 0x5b, 0x46, 0x3f, 0x8f, 0x7d,
                0x8b, 0xda, 0x0c, 0x3c,
            ],
            Elements::Min64 => [
                0xf0, 0xa3, 0x17, 0x0f, 0xb3, 0x06, 0x50, 0x90, 0x71, 0xb2, 0x14, 0x32, 0x30, 0x9c,
                0x00, 0x86, 0x5d, 0x8f, 0xb6, 0x75, 0x2d, 0xe6, 0xec, 0x41, 0x0a, 0xbb, 0x6c, 0xe3,
                0x5b, 0x4f, 0xfa, 0x9c,
            ],
            Elements::Min8 => [
                0x8a, 0xa7, 0x1d, 0x09, 0x3e, 0x83, 0x8a, 0xd3, 0x6a, 0xa6, 0xfe, 0x16, 0xfd, 0x1d,
                0xde, 0x9f, 0xb5, 0x77, 0x2a, 0xc4, 0x5d, 0x6b, 0xb8, 0xe4, 0x2c, 0x88, 0x83, 0xa2,
                0xa9, 0xf1, 0x00, 0xe2,
            ],
            Elements::Modulo16 => [
                0x0e, 0xa8, 0x98, 0x59, 0x79, 0x20, 0x9d, 0x31, 0x94, 0xc5, 0x59, 0x12, 0xf2, 0x1a,
                0x65, 0x77, 0xda, 0x30, 0xc5, 0x17, 0xe0, 0x08, 0xf8, 0x51, 0xc3, 0x52, 0x08, 0x5b,
                0xb0, 0x67, 0xa0, 0xf1,
            ],
            Elements::Modulo32 => [
                0x22, 0x3f, 0xb5, 0x83, 0x2b, 0x95, 0x59, 0xca, 0x86, 0x51, 0x72, 0x6a, 0xb7, 0x95,
                0x6e, 0x58, 0xe5, 0x3a, 0x40, 0x2d, 0xf9, 0x60, 0x34, 0x86, 0x18, 0x18, 0x07, 0xa2,
                0xfc, 0xae, 0xd4, 0x87,
            ],
            Elements::Modulo64 => [
                0x82, 0x59, 0xa2, 0xe3, 0xb0, 0x78, 0x9c, 0x9d, 0x31, 0xea, 0xbc, 0x49, 0x39, 0xd3,
                0x1b, 0xc5, 0x25, 0x4c, 0x56, 0x65, 0xc4, 0x12, 0x05, 0x2c, 0x12, 0xd7, 0x2e, 0xcd,
                0xfb, 0xed, 0x52, 0x25,
            ],
            Elements::Modulo8 => [
                0xf8, 0xfd, 0x7c, 0x21, 0xc1, 0x5e, 0xbc, 0x5a, 0x04, 0xb3, 0xd3, 0xbc, 0x0a, 0xd6,
                0x47, 0xe2, 0x1d, 0x89, 0xf8, 0x00, 0x8d, 0xdd, 0xdf, 0xfe, 0xbb, 0x00, 0x3e, 0x35,
                0x52, 0xbd, 0xb5, 0x70,
            ],
            Elements::Multiply16 => [
                0xd6, 0x6b, 0x0a, 0xd2, 0x1c, 0xdb, 0xe6, 0x01, 0x4f, 0x26, 0x19, 0xf4, 0x40, 0xd9,
                0x35, 0xf6, 0x42, 0xe9, 0x7c, 0x59, 0x4e, 0x3b, 0x44, 0x1f, 0x01, 0x98, 0x0e, 0xea,
                0x08, 0xc4, 0x85, 0x91,
            ],
            Elements::Multiply32 => [
                0xc6, 0x7f, 0x65, 0xaf, 0xae, 0x79, 0x10, 0x8c, 0xe5, 0xf7, 0x24, 0xf4, 0x01, 0x81,
                0x7a, 0xa1, 0xb4, 0xa7, 0x60, 0x8a, 0x4e, 0x18, 0xee, 0x74, 0x82, 0x6b, 0x37, 0xf7,
                0x94, 0x9c, 0x77, 0xd6,
            ],
            Elements::Multiply64 => [
                0x16, 0xac, 0x08, 0xfe, 0xc9, 0xd9, 0x66, 0x52, 0x7f, 0x80, 0xc3, 0x9c, 0x64, 0xa4,
                0xfc, 0x10, 0x82, 0xb1, 0x50, 0xbd, 0xbb, 0x05, 0xf7, 0xf6, 0xc4, 0xb9, 0xa0, 0x0e,
                0xbd, 0x89, 0x4d, 0xba,
            ],
            Elements::Multiply8 => [
                0xce, 0xc3, 0xd5, 0xc3, 0x2d, 0xee, 0x35, 0x2e, 0x75, 0x4f, 0x14, 0x1c, 0x02, 0xef,
                0x2b, 0x60, 0xf8, 0x6e, 0x27, 0xd2, 0xcb, 0xe2, 0x73, 0x26, 0x06, 0x0d, 0xec, 0x9e,
                0x7b, 0xcc, 0x20, 0x6b,
            ],
            Elements::Negate16 => [
                0x81, 0xf3, 0xef, 0x08, 0x2b, 0x19, 0x9d, 0x2e, 0x47, 0x45, 0x35, 0xfb, 0x84, 0x28,
                0x2f, 0x21, 0x11, 0x1c, 0x1a, 0x04, 0x97, 0xa9, 0x16, 0x95, 0xe7, 0x2c, 0x4d, 0x51,
                0xaf, 0x1c, 0xb0, 0x57,
            ],
            Elements::Negate32 => [
                0xba, 0x17, 0x3b, 0xde, 0xf2, 0x4e, 0x63, 0x7f, 0x4a, 0x5a, 0x0c, 0x6f, 0xed, 0x4b,
                0xbc, 0x22, 0xc7, 0xd0, 0x8b, 0xcc, 0x20, 0xfd, 0xca, 0x61, 0x70, 0x36, 0x3d, 0x25,
                0x26, 0x95, 0x58, 0x36,
            ],
            Elements::Negate64 => [
                0xe7, 0x13, 0x27, 0x2d, 0x6d, 0x9a, 0x6d, 0x69, 0x6c, 0x46, 0x88, 0x9d, 0xfb, 0x27,
                0x04, 0x75, 0xe9, 0x23, 0x8d, 0x17, 0x72, 0xab, 0xf9, 0x16, 0x5b, 0x09, 0x3c, 0x79,
                0x98, 0xee, 0x80, 0x70,
            ],
            Elements::Negate8 => [
                0x8b, 0xe8, 0x71, 0x39, 0x3e, 0x18, 0xc0, 0x63, 0x69, 0x16, 0x1f, 0xb1, 0xc0, 0xad,
                0x2b, 0x92, 0x06, 0x22, 0x97, 0x4e, 0x3f, 0xdf, 0xca, 0x90, 0x56, 0x7e, 0x6d, 0xa4,
                0x29, 0xfd, 0x98, 0x42,
            ],
            Elements::NewIssuanceContract => [
                0x59, 0xe2, 0xa7, 0x4c, 0x99, 0x7a, 0x07, 0xaa, 0xfd, 0x5c, 0x28, 0x0e, 0x46, 0x49,
                0xe7, 0xaf, 0x2b, 0x3c, 0xc4, 0xdc, 0x9b, 0x72, 0xa8, 0x1c, 0x72, 0x25, 0x48, 0xff,
                0x8c, 0x13, 0x13, 0xbf,
            ],
            Elements::NonceHash => [
                0xca, 0x4a, 0x66, 0xd7, 0xf8, 0xe9, 0x82, 0xa9, 0xee, 0x04, 0x25, 0x04, 0x0d, 0x2b,
                0x0d, 0x1d, 0x3d, 0xc6, 0x20, 0x7f, 0x93, 0x85, 0x64, 0x58, 0xe2, 0x4e, 0x7a, 0x26,
                0xe9, 0x5c, 0x56, 0xf8,
            ],
            Elements::NumInputs => [
                0xd7, 0xb4, 0x66, 0x28, 0x7e, 0xf4, 0xe5, 0x9c, 0x02, 0xbe, 0x33, 0x89, 0xea, 0xfb,
                0x43, 0x47, 0xb7, 0xd1, 0xa2, 0xa1, 0x5c, 0x14, 0x49, 0x18, 0xb7, 0xad, 0x34, 0x43,
                0xb1, 0x4f, 0xae, 0x03,
            ],
            Elements::NumOutputs => [
                0x67, 0xdf, 0x4d, 0xeb, 0x3f, 0xf0, 0xc2, 0x6a, 0xe3, 0x12, 0x30, 0x47, 0xcc, 0x73,
                0x4a, 0xe3, 0xb6, 0x1f, 0xfc, 0xef, 0x51, 0x1f, 0xcf, 0x42, 0x81, 0x3c, 0x3f, 0x48,
                0x19, 0x20, 0xee, 0x48,
            ],
            Elements::One16 => [
                0x76, 0x29, 0x79, 0x2b, 0xa1, 0xa4, 0x41, 0x14, 0xe5, 0x9e, 0x64, 0xb5, 0x1d, 0x18,
                0x7a, 0xf6, 0xd9, 0x65, 0xd9, 0x66, 0x20, 0xdc, 0xa2, 0x62, 0x75, 0x07, 0xb7, 0xd3,
                0x88, 0xd9, 0x35, 0x3c,
            ],
            Elements::One32 => [
                0x0b, 0xa0, 0x04, 0xce, 0xa2, 0x50, 0xfe, 0x95, 0x3a, 0xc7, 0x4e, 0x6e, 0xcd, 0x36,
                0x20, 0xe8, 0x02, 0x84, 0x1f, 0xda, 0x79, 0x52, 0x08, 0xde, 0xc6, 0x6d, 0x62, 0x6e,
                0x06, 0xaa, 0x29, 0xb2,
            ],
            Elements::One64 => [
                0xba, 0x34, 0x78, 0xc1, 0x08, 0x74, 0x0a, 0x83, 0xf0, 0xca, 0x0e, 0xae, 0x86, 0xc3,
                0x1b, 0x4a, 0xfa, 0xc8, 0x30, 0xdf, 0x09, 0x34, 0x67, 0xcc, 0x08, 0xea, 0x1c, 0x04,
                0x15, 0xef, 0xef, 0x6d,
            ],
            Elements::One8 => [
                0x15, 0xca, 0x77, 0xa4, 0xb7, 0x02, 0x25, 0x68, 0x37, 0xf9, 0x0f, 0xf7, 0x8c, 0xa7,
                0x74, 0x0a, 0x40, 0xfe, 0xce, 0x71, 0x91, 0x30, 0x1d, 0x00, 0xe5, 0x17, 0xd8, 0xd3,
                0x4f, 0x46, 0xc2, 0x50,
            ],
            Elements::Or1 => [
                0x93, 0x52, 0x22, 0x30, 0x17, 0x00, 0x98, 0x7d, 0xe1, 0x2c, 0xb4, 0x26, 0x17, 0x21,
                0x81, 0x53, 0xfd, 0x7c, 0xcd, 0x63, 0x17, 0x4a, 0x17, 0x49, 0xfc, 0x88, 0x0c, 0x39,
                0xe3, 0xe7, 0x23, 0x9c,
            ],
            Elements::Or16 => [
                0xdf, 0xea, 0xd0, 0xba, 0x93, 0xe4, 0x91, 0x55, 0xc4, 0x0c, 0xb3, 0x72, 0xca, 0x5e,
                0xf6, 0x17, 0x97, 0x41, 0xc6, 0x1f, 0x2b, 0x3c, 0xc2, 0x79, 0x7e, 0xf1, 0x62, 0xc8,
                0xd2, 0xfc, 0x1f, 0x9a,
            ],
            Elements::Or32 => [
                0x67, 0xac, 0x69, 0x45, 0xcd, 0xc0, 0x06, 0xd2, 0x5e, 0x5b, 0xbe, 0x6c, 0x4f, 0xe8,
                0x1c, 0xa1, 0x67, 0x41, 0xff, 0xab, 0x3d, 0x23, 0x96, 0x0e, 0xeb, 0x49, 0x85, 0x0f,
                0x92, 0x73, 0x2c, 0xbb,
            ],
            Elements::Or64 => [
                0x1e, 0xb9, 0x52, 0xe4, 0x61, 0x16, 0xe2, 0x71, 0xdc, 0x48, 0x9e, 0x67, 0x22, 0xa9,
                0x01, 0x85, 0xeb, 0xf5, 0xfb, 0x77, 0x5b, 0x77, 0x81, 0x60, 0x6d, 0xbf, 0x5e, 0x89,
                0xcc, 0xd7, 0xc2, 0x50,
            ],
            Elements::Or8 => [
                0x37, 0x8f, 0x7a, 0xbe, 0x8b, 0x08, 0x1f, 0xaf, 0x5b, 0x3a, 0x25, 0x78, 0xef, 0x19,
                0x79, 0xfe, 0x80, 0xbc, 0xcb, 0x07, 0x91, 0x15, 0x6a, 0x49, 0x3f, 0x8a, 0x3f, 0x6e,
                0x0b, 0xb2, 0xfc, 0x84,
            ],
            Elements::OutpointHash => [
                0xe3, 0x1d, 0x33, 0x9d, 0x86, 0xe6, 0xd7, 0x6b, 0xad, 0xc3, 0xd6, 0xd6, 0xcf, 0x1b,
                0xe7, 0x06, 0x53, 0xe0, 0x34, 0x42, 0x73, 0x44, 0x9c, 0xec, 0xf5, 0x21, 0xd3, 0x66,
                0x2b, 0xd8, 0xfc, 0x2b,
            ],
            Elements::OutputAmount => [
                0x52, 0xdb, 0x68, 0xc6, 0xc7, 0xbe, 0x32, 0xbf, 0xaa, 0x42, 0x69, 0x70, 0xeb, 0x41,
                0x97, 0xce, 0x1c, 0xe3, 0xf4, 0x53, 0x0e, 0x1a, 0x2c, 0xff, 0x8e, 0x17, 0xdb, 0x47,
                0x22, 0x92, 0x07, 0xb3,
            ],
            Elements::OutputAmountsHash => [
                0xbc, 0xcb, 0xee, 0x78, 0x1e, 0xe2, 0x97, 0xfa, 0xbf, 0xbe, 0x38, 0xc0, 0x07, 0x9a,
                0xd3, 0x74, 0x25, 0x8c, 0x0b, 0x1e, 0xbe, 0x22, 0x34, 0x57, 0x09, 0x9a, 0x28, 0x4a,
                0x1b, 0x43, 0x38, 0xa2,
            ],
            Elements::OutputAsset => [
                0xca, 0x89, 0xb5, 0xc4, 0x5a, 0x9a, 0x24, 0x7d, 0x39, 0x08, 0x6c, 0xdf, 0xbe, 0x2b,
                0xa1, 0xdf, 0x02, 0x5e, 0x37, 0xdb, 0x9b, 0x6a, 0x2a, 0xbb, 0x74, 0xd9, 0x70, 0x66,
                0xa6, 0x9b, 0xf3, 0xa0,
            ],
            Elements::OutputHash => [
                0x78, 0x75, 0xcc, 0x33, 0x2c, 0xf1, 0xb4, 0x9c, 0xdc, 0xf1, 0x1e, 0x70, 0x18, 0xdc,
                0x81, 0x19, 0x3e, 0xe4, 0xb1, 0x9e, 0xa1, 0x42, 0xfc, 0x0a, 0xe7, 0xdb, 0xdd, 0xb3,
                0x36, 0xe2, 0xfb, 0xfa,
            ],
            Elements::OutputIsFee => [
                0xf7, 0x0e, 0xb0, 0x68, 0xd2, 0x00, 0xb7, 0xbc, 0x5a, 0xaa, 0xab, 0xa1, 0x29, 0x28,
                0xe8, 0x19, 0x2b, 0x29, 0x48, 0xcd, 0xe7, 0xe8, 0xf3, 0x14, 0x1c, 0x6b, 0x43, 0x11,
                0x38, 0xe5, 0xde, 0x3c,
            ],
            Elements::OutputNonce => [
                0xed, 0xab, 0x12, 0x41, 0xd2, 0x97, 0xf2, 0x74, 0x7a, 0xd3, 0x78, 0xa8, 0x1a, 0x56,
                0x14, 0x3d, 0x74, 0x1b, 0xc4, 0xee, 0x4c, 0x30, 0xb3, 0x2f, 0x8b, 0xc1, 0xae, 0xb5,
                0x2c, 0xc4, 0x81, 0x4c,
            ],
            Elements::OutputNoncesHash => [
                0xf0, 0x7f, 0xe7, 0xe9, 0x73, 0x96, 0x86, 0x5d, 0x50, 0xc1, 0xac, 0x94, 0x77, 0xad,
                0x9f, 0x95, 0x06, 0x2f, 0x11, 0xed, 0x01, 0xc9, 0x7d, 0xe7, 0xfa, 0x4a, 0xff, 0xa0,
                0x8c, 0xd1, 0x05, 0x31,
            ],
            Elements::OutputNullDatum => [
                0x96, 0x4c, 0x4d, 0xee, 0x8f, 0xe4, 0xa8, 0x66, 0xcc, 0x3e, 0xa4, 0x94, 0x78, 0x84,
                0x81, 0x4c, 0xcc, 0xd6, 0x02, 0x69, 0x2e, 0x37, 0x8b, 0xa0, 0xe9, 0x35, 0xfa, 0x03,
                0xf9, 0x5e, 0x3d, 0xe1,
            ],
            Elements::OutputRangeProof => [
                0xa0, 0x04, 0x18, 0xe4, 0xab, 0xc4, 0xbe, 0xd5, 0x37, 0x3f, 0xc2, 0x22, 0xce, 0x1f,
                0x96, 0x0b, 0x64, 0x35, 0xea, 0xa8, 0xf1, 0x4f, 0xe6, 0x6c, 0xa4, 0x2c, 0x52, 0xdd,
                0x9a, 0xe2, 0x26, 0xb8,
            ],
            Elements::OutputRangeProofsHash => [
                0xeb, 0xe6, 0x26, 0xe2, 0x85, 0x4e, 0x70, 0x00, 0x75, 0xea, 0xde, 0xee, 0xad, 0x5f,
                0xe0, 0xbd, 0xd6, 0x9c, 0x1b, 0x0f, 0xad, 0xf1, 0x08, 0xd3, 0x68, 0xc1, 0x29, 0x3f,
                0x86, 0x58, 0x5f, 0xb1,
            ],
            Elements::OutputScriptHash => [
                0xec, 0xf6, 0x73, 0xe5, 0xfc, 0xaf, 0x7b, 0x7e, 0x6e, 0x37, 0xb6, 0x28, 0x09, 0x5e,
                0x5a, 0x9e, 0xf9, 0x81, 0xe6, 0xc4, 0xc3, 0x39, 0xeb, 0xdf, 0x3e, 0xb1, 0xe6, 0xbf,
                0x45, 0x05, 0xf5, 0x34,
            ],
            Elements::OutputScriptsHash => [
                0xc0, 0x48, 0x9e, 0x87, 0x8c, 0xd8, 0x9c, 0xa5, 0x10, 0x9a, 0x53, 0x16, 0x7a, 0x61,
                0xa8, 0x45, 0x4e, 0xd6, 0xdc, 0x2a, 0xce, 0xf9, 0x19, 0x1a, 0xd5, 0x3b, 0xbf, 0xb3,
                0xb0, 0xea, 0x72, 0xdf,
            ],
            Elements::OutputSurjectionProof => [
                0x1d, 0xe8, 0xad, 0xdb, 0xf8, 0xb6, 0x97, 0xc2, 0xb3, 0xd1, 0x92, 0xbd, 0xe2, 0xc2,
                0xc5, 0x5f, 0x5e, 0x67, 0x09, 0xc5, 0xbd, 0x86, 0x31, 0xe3, 0x8d, 0x83, 0x78, 0xdd,
                0x3d, 0xce, 0x49, 0xcc,
            ],
            Elements::OutputSurjectionProofsHash => [
                0x74, 0x3e, 0xb2, 0xd9, 0x8e, 0x2c, 0x69, 0xc4, 0x03, 0xab, 0xa3, 0xc3, 0x3d, 0xcb,
                0x83, 0x51, 0xde, 0x82, 0x24, 0xc9, 0xe0, 0x4d, 0xa5, 0x07, 0x1c, 0x8d, 0xfd, 0x7a,
                0x93, 0x50, 0x70, 0xac,
            ],
            Elements::OutputsHash => [
                0x12, 0xaf, 0x1e, 0x5c, 0x32, 0xda, 0x75, 0x12, 0xfc, 0xd4, 0x32, 0x59, 0xd1, 0xbc,
                0x84, 0x72, 0xa9, 0x15, 0xce, 0x21, 0xc6, 0xd3, 0x06, 0xe4, 0x85, 0xf1, 0xdb, 0xbc,
                0x07, 0xa1, 0xcf, 0xdb,
            ],
            Elements::ParseLock => [
                0x45, 0x71, 0xca, 0x04, 0x52, 0x57, 0x7c, 0xfa, 0xf8, 0xa8, 0x5f, 0x5e, 0x32, 0x9d,
                0x60, 0x2c, 0x8d, 0xca, 0xd1, 0x27, 0x6a, 0x97, 0x4e, 0x2c, 0x75, 0xd2, 0xec, 0x04,
                0x65, 0x7b, 0xc4, 0x25,
            ],
            Elements::ParseSequence => [
                0x55, 0xc9, 0x03, 0x66, 0x0c, 0x8c, 0x92, 0xb1, 0x42, 0x9f, 0x63, 0xe2, 0x74, 0x71,
                0xac, 0xd5, 0x9c, 0x3a, 0x26, 0xaa, 0x7a, 0x59, 0x22, 0x1d, 0x7a, 0x6d, 0x52, 0xb6,
                0x2e, 0xba, 0xf9, 0x4a,
            ],
            Elements::PointVerify1 => [
                0xb6, 0x00, 0x9c, 0xec, 0x8b, 0xe8, 0xb1, 0x4a, 0x1d, 0x31, 0x32, 0x99, 0xd6, 0x67,
                0x61, 0xd6, 0x59, 0xad, 0xff, 0xf8, 0x23, 0x28, 0x99, 0xab, 0x7d, 0xb4, 0x09, 0x7f,
                0x52, 0xc7, 0xd0, 0xbe,
            ],
            Elements::ReissuanceBlinding => [
                0x0b, 0x9a, 0x0a, 0xaf, 0xed, 0xb1, 0x02, 0xa1, 0x64, 0x06, 0x14, 0x19, 0xa7, 0x89,
                0xea, 0x84, 0x32, 0x6c, 0xd5, 0x89, 0xf1, 0x7c, 0x0e, 0xa0, 0x19, 0x2d, 0xd6, 0x37,
                0x2e, 0xcb, 0x5e, 0xbf,
            ],
            Elements::ReissuanceEntropy => [
                0xce, 0x84, 0xf9, 0x83, 0xa3, 0xbd, 0x33, 0x94, 0x3d, 0x4d, 0xcb, 0x44, 0x86, 0x1c,
                0x4a, 0x69, 0xa2, 0xa5, 0xe8, 0xa5, 0x22, 0xb5, 0xd5, 0xeb, 0xf2, 0x51, 0x5f, 0x51,
                0x1a, 0xc6, 0xa9, 0xb2,
            ],
            Elements::RightExtend16_32 => [
                0x36, 0x42, 0x3c, 0x16, 0xd4, 0x8d, 0x6c, 0x7c, 0x91, 0xed, 0x44, 0x16, 0x11, 0xbe,
                0x30, 0x72, 0xdf, 0xa5, 0xdd, 0x38, 0xe4, 0xd2, 0x7d, 0xa8, 0xda, 0xed, 0x29, 0x78,
                0x8f, 0xc9, 0x52, 0x08,
            ],
            Elements::RightExtend16_64 => [
                0x4b, 0x8a, 0x47, 0xb9, 0x06, 0x70, 0x73, 0xa1, 0xfb, 0x68, 0x30, 0x0f, 0xac, 0xd6,
                0xc5, 0x06, 0x98, 0x90, 0xab, 0xdb, 0x7e, 0xaa, 0xcb, 0x62, 0x2a, 0xd7, 0x30, 0x9a,
                0x87, 0xf4, 0xd3, 0x4d,
            ],
            Elements::RightExtend32_64 => [
                0xdd, 0x6a, 0xf1, 0xc8, 0x01, 0xd2, 0x6c, 0x0b, 0x2e, 0xdf, 0x83, 0xce, 0x67, 0xb1,
                0x72, 0xdf, 0x67, 0x57, 0xd0, 0x7f, 0xb7, 0xc8, 0x54, 0x68, 0x6f, 0x42, 0xe5, 0x76,
                0x8a, 0xdc, 0xc9, 0xe7,
            ],
            Elements::RightExtend8_16 => [
                0x1d, 0xe2, 0x01, 0xa8, 0x64, 0x70, 0xa0, 0x2b, 0x2d, 0xfe, 0x48, 0xc6, 0x6a, 0xfe,
                0x06, 0x73, 0x5b, 0x47, 0x5e, 0x88, 0xd3, 0x25, 0xcb, 0xf1, 0x60, 0x42, 0xa9, 0x10,
                0x24, 0xd2, 0xbe, 0xd9,
            ],
            Elements::RightExtend8_32 => [
                0x7e, 0x9c, 0x5c, 0xb3, 0x54, 0x19, 0xab, 0x06, 0xe1, 0x22, 0x00, 0x23, 0x10, 0x2b,
                0xe4, 0x6a, 0xb6, 0xd9, 0x69, 0x95, 0xc4, 0x23, 0xc6, 0xb1, 0x4b, 0x9a, 0x66, 0x02,
                0x8a, 0xec, 0x5d, 0x75,
            ],
            Elements::RightExtend8_64 => [
                0x49, 0xd2, 0x46, 0xc2, 0xa6, 0x1c, 0xd3, 0x9d, 0x78, 0x20, 0xdc, 0xd7, 0x5e, 0xee,
                0x84, 0x7b, 0xf0, 0x57, 0xc0, 0x1a, 0x63, 0xa3, 0xac, 0xbc, 0xc9, 0x46, 0x3e, 0x44,
                0xbc, 0x1e, 0x0b, 0x6c,
            ],
            Elements::RightPadHigh16_32 => [
                0xfe, 0x90, 0x1f, 0xb4, 0xf6, 0xeb, 0xdc, 0x4e, 0xa2, 0x96, 0x19, 0x98, 0x99, 0x22,
                0xb8, 0x0f, 0xa9, 0xce, 0x24, 0x12, 0x87, 0xfa, 0x54, 0x08, 0x64, 0x36, 0x2c, 0xcc,
                0xe9, 0xf5, 0x4b, 0x3b,
            ],
            Elements::RightPadHigh16_64 => [
                0xda, 0x90, 0xad, 0xd3, 0x10, 0x67, 0xcc, 0xfd, 0xbe, 0xe4, 0xcb, 0xfb, 0x21, 0xde,
                0x8e, 0x6a, 0xa4, 0xf9, 0x3e, 0x00, 0x22, 0x00, 0x71, 0x1f, 0x99, 0x84, 0xaf, 0x6f,
                0xc0, 0x1e, 0x27, 0x00,
            ],
            Elements::RightPadHigh1_16 => [
                0xe4, 0xcf, 0x11, 0x6c, 0x08, 0x80, 0xf7, 0x3f, 0x99, 0x52, 0xf7, 0x00, 0x81, 0x78,
                0x84, 0x98, 0xe5, 0x08, 0x4c, 0xbb, 0x72, 0xcf, 0x84, 0x1b, 0xcd, 0x91, 0x67, 0xa6,
                0xee, 0xa2, 0x64, 0xdc,
            ],
            Elements::RightPadHigh1_32 => [
                0x12, 0x76, 0x03, 0x6b, 0xb9, 0x4c, 0xfd, 0x92, 0x0a, 0xb7, 0x31, 0x64, 0x3b, 0x76,
                0xb1, 0x19, 0x72, 0xdd, 0x26, 0x54, 0x38, 0x53, 0x44, 0x4e, 0x18, 0xd7, 0xf6, 0x3f,
                0xca, 0xc0, 0x91, 0xa3,
            ],
            Elements::RightPadHigh1_64 => [
                0x38, 0xc9, 0x99, 0x80, 0xb1, 0xa9, 0x98, 0x10, 0x51, 0x11, 0xc5, 0x6b, 0xf8, 0x24,
                0x65, 0x09, 0x65, 0xa5, 0x09, 0xc4, 0x7e, 0x1c, 0x76, 0xd9, 0x00, 0x75, 0x0a, 0x1f,
                0xee, 0x45, 0xc9, 0x64,
            ],
            Elements::RightPadHigh1_8 => [
                0xca, 0x72, 0xce, 0xed, 0x2d, 0x98, 0xdc, 0xcd, 0x81, 0xaa, 0x21, 0xf0, 0xba, 0x21,
                0xd1, 0xa0, 0x87, 0xb6, 0xf2, 0x52, 0x07, 0xc2, 0x4a, 0x58, 0x0a, 0xda, 0x7e, 0x60,
                0x5f, 0x79, 0x82, 0xdf,
            ],
            Elements::RightPadHigh32_64 => [
                0x17, 0xeb, 0x59, 0x11, 0xf8, 0x54, 0x95, 0x76, 0x68, 0xee, 0xf4, 0x63, 0xb0, 0xcb,
                0xae, 0x72, 0x08, 0x52, 0x91, 0x34, 0xef, 0x5e, 0x56, 0xcd, 0x33, 0xfb, 0xbc, 0x29,
                0xc2, 0x8b, 0xbe, 0x92,
            ],
            Elements::RightPadHigh8_16 => [
                0xd2, 0x6f, 0x0c, 0xc5, 0xb2, 0x61, 0xeb, 0x83, 0x0e, 0x02, 0xdf, 0x12, 0xcc, 0x57,
                0x44, 0x25, 0x9b, 0x4a, 0x43, 0xd9, 0x75, 0xbd, 0x2e, 0x3d, 0x7c, 0x78, 0x28, 0x11,
                0x76, 0x1f, 0xf1, 0xd1,
            ],
            Elements::RightPadHigh8_32 => [
                0xbd, 0x2e, 0x5c, 0x92, 0x60, 0xbf, 0x6f, 0x32, 0x4d, 0x2b, 0x1f, 0x40, 0xcb, 0xb1,
                0x22, 0x40, 0x2f, 0x30, 0xd5, 0x2f, 0x64, 0x34, 0xe3, 0x9f, 0x8a, 0x09, 0xb8, 0x39,
                0x7b, 0xc3, 0x2e, 0x94,
            ],
            Elements::RightPadHigh8_64 => [
                0x94, 0x1b, 0xf4, 0x42, 0xdb, 0xcf, 0x4f, 0x20, 0x04, 0xa4, 0xb1, 0x8b, 0xee, 0xb2,
                0xad, 0xac, 0x9f, 0x20, 0x9f, 0xea, 0x4c, 0x4b, 0xd4, 0x8c, 0xed, 0xe8, 0xda, 0xfa,
                0xcf, 0x88, 0x43, 0xb7,
            ],
            Elements::RightPadLow16_32 => [
                0xbb, 0x38, 0x7c, 0x29, 0x2d, 0x59, 0xd7, 0x13, 0xad, 0x76, 0xf6, 0xce, 0xd5, 0xb5,
                0x96, 0xcf, 0xd8, 0x38, 0x58, 0x92, 0x4f, 0x72, 0x5f, 0x7d, 0x11, 0x6b, 0x28, 0x07,
                0x58, 0x21, 0x92, 0x5a,
            ],
            Elements::RightPadLow16_64 => [
                0x02, 0x32, 0x32, 0x6e, 0xe1, 0xb2, 0x06, 0xad, 0x26, 0x34, 0x9b, 0x55, 0x3d, 0x7f,
                0x24, 0x62, 0x28, 0x73, 0x20, 0xd6, 0x30, 0xe4, 0x29, 0x32, 0x07, 0x40, 0xcb, 0xd3,
                0xeb, 0x4e, 0xf9, 0xbe,
            ],
            Elements::RightPadLow1_16 => [
                0xd9, 0x13, 0xf6, 0x02, 0xb3, 0x59, 0x58, 0xd5, 0x2a, 0xbb, 0x20, 0xb0, 0x2c, 0xe6,
                0x89, 0x61, 0x6f, 0xfa, 0x66, 0xe0, 0x2d, 0x73, 0x86, 0x7d, 0x29, 0x18, 0x1e, 0x11,
                0x93, 0xc9, 0xd2, 0x43,
            ],
            Elements::RightPadLow1_32 => [
                0x6b, 0x40, 0x33, 0xd9, 0xfc, 0x6c, 0x87, 0x6b, 0x2e, 0x75, 0xd5, 0x82, 0xbb, 0x9b,
                0x3c, 0x04, 0xfa, 0x29, 0xdf, 0xb2, 0x2c, 0x9e, 0x1a, 0x48, 0x8e, 0x83, 0x7c, 0x2f,
                0x39, 0xaa, 0x61, 0x60,
            ],
            Elements::RightPadLow1_64 => [
                0x4e, 0x2b, 0x20, 0xdd, 0x9d, 0x91, 0x85, 0x7a, 0x49, 0xc8, 0x20, 0xd0, 0x6f, 0x43,
                0x5d, 0xd3, 0xca, 0x79, 0x1f, 0x17, 0x7e, 0xea, 0xf3, 0x4a, 0xec, 0x36, 0xc4, 0x54,
                0x19, 0xd1, 0x69, 0x65,
            ],
            Elements::RightPadLow1_8 => [
                0x24, 0xee, 0xe4, 0x51, 0xb2, 0x6b, 0xa3, 0x9d, 0x6b, 0xcc, 0x58, 0x8b, 0x72, 0x0f,
                0xaf, 0x22, 0x32, 0x76, 0x79, 0x12, 0xf6, 0x7d, 0xb3, 0x29, 0x06, 0x0d, 0x90, 0xb7,
                0x14, 0x17, 0xb6, 0xc3,
            ],
            Elements::RightPadLow32_64 => [
                0x52, 0xfb, 0x8b, 0xbc, 0xef, 0x90, 0x32, 0x31, 0xa5, 0xb7, 0x67, 0x91, 0xe4, 0x65,
                0x2b, 0x38, 0xbe, 0xd8, 0x97, 0x7f, 0x5d, 0xab, 0x17, 0x95, 0x55, 0x99, 0x8d, 0xb2,
                0x4d, 0x1d, 0x7c, 0x98,
            ],
            Elements::RightPadLow8_16 => [
                0x17, 0x19, 0xb2, 0x79, 0x74, 0xe8, 0x43, 0x80, 0x50, 0x88, 0x25, 0x30, 0xa1, 0xa4,
                0x2e, 0xd7, 0xab, 0x3c, 0xa2, 0x8d, 0x25, 0x4a, 0xdc, 0x37, 0xfe, 0x56, 0x66, 0xfd,
                0x2f, 0x70, 0xb4, 0xe4,
            ],
            Elements::RightPadLow8_32 => [
                0xee, 0x2a, 0x82, 0x30, 0xf2, 0x83, 0xdc, 0x08, 0x3b, 0x8e, 0x19, 0x44, 0x8b, 0xa3,
                0x24, 0x97, 0xe9, 0x31, 0x8b, 0x4e, 0x9e, 0x1b, 0xd4, 0xeb, 0xe1, 0xbe, 0xc5, 0x24,
                0x47, 0x6a, 0xb8, 0x6d,
            ],
            Elements::RightPadLow8_64 => [
                0x97, 0xda, 0x90, 0xd8, 0x42, 0x8e, 0x6b, 0x94, 0xe6, 0xc1, 0x35, 0x14, 0x60, 0xdc,
                0x01, 0x12, 0x3e, 0x47, 0x9c, 0x4a, 0xaf, 0xbb, 0xd1, 0x4c, 0x78, 0xad, 0x2f, 0xad,
                0x0a, 0x89, 0x5e, 0xf3,
            ],
            Elements::RightRotate16 => [
                0xee, 0x7d, 0x1c, 0x1f, 0x3d, 0x82, 0xda, 0x56, 0x81, 0xdd, 0x8b, 0x50, 0x69, 0xd5,
                0x37, 0xd8, 0x9f, 0x22, 0x93, 0xaa, 0x60, 0x53, 0x32, 0xce, 0x10, 0xc1, 0xc4, 0x22,
                0x4a, 0x53, 0xce, 0xea,
            ],
            Elements::RightRotate32 => [
                0x89, 0x2a, 0x28, 0xdb, 0x32, 0x4c, 0xd9, 0x3c, 0xf7, 0xf6, 0x9c, 0x30, 0x72, 0xa7,
                0xb2, 0x22, 0xb8, 0x8c, 0x81, 0x8e, 0xe0, 0xe5, 0xa1, 0xb8, 0x97, 0xe5, 0x0c, 0x58,
                0x1f, 0x2a, 0x29, 0x62,
            ],
            Elements::RightRotate64 => [
                0x64, 0x31, 0x4f, 0xf1, 0x90, 0x40, 0xa3, 0x76, 0xf9, 0xfc, 0xf0, 0x2e, 0x75, 0x74,
                0x14, 0x9c, 0x12, 0x3f, 0x99, 0xc3, 0x90, 0x71, 0xcd, 0x37, 0x85, 0x1f, 0x8f, 0x8c,
                0xdf, 0x0e, 0xed, 0x42,
            ],
            Elements::RightRotate8 => [
                0x15, 0x81, 0xe0, 0xca, 0x09, 0xf1, 0x36, 0x84, 0xfe, 0x31, 0x35, 0xc1, 0xc6, 0xb6,
                0xf9, 0xc4, 0x89, 0xd7, 0xdd, 0x1e, 0xf0, 0xa5, 0xf7, 0x70, 0x83, 0xbb, 0x0e, 0xd0,
                0x0b, 0x4d, 0xf2, 0x8f,
            ],
            Elements::RightShift16 => [
                0x5b, 0x4e, 0xc4, 0x62, 0xd4, 0xe2, 0xed, 0x89, 0xff, 0xe3, 0xfd, 0x40, 0x59, 0x32,
                0xc7, 0x97, 0x80, 0x28, 0x61, 0x20, 0x3e, 0xcb, 0x61, 0xd5, 0xb5, 0x9a, 0x73, 0xb0,
                0xfb, 0xfc, 0x4e, 0x84,
            ],
            Elements::RightShift32 => [
                0xb2, 0x86, 0x1a, 0x48, 0xb2, 0x05, 0x41, 0x76, 0x91, 0xb6, 0x34, 0x7f, 0xe7, 0x5e,
                0xbe, 0xa5, 0x45, 0x60, 0xcf, 0x81, 0x38, 0x14, 0xac, 0x31, 0x63, 0x91, 0x70, 0xdb,
                0x92, 0xb9, 0x47, 0xd6,
            ],
            Elements::RightShift64 => [
                0xd3, 0x39, 0x42, 0xbf, 0x18, 0x61, 0x8a, 0x10, 0x4a, 0x57, 0x07, 0x54, 0x7f, 0x78,
                0xab, 0x72, 0x94, 0x1f, 0x4e, 0xe8, 0x13, 0x21, 0x6c, 0x0c, 0xe5, 0x20, 0xf3, 0x56,
                0x60, 0xfd, 0xbf, 0x81,
            ],
            Elements::RightShift8 => [
                0x73, 0x79, 0x12, 0xae, 0x32, 0x32, 0x50, 0xc0, 0x4e, 0x51, 0x6e, 0x39, 0x66, 0xce,
                0x94, 0x7e, 0x65, 0x32, 0x6f, 0x47, 0x46, 0x8a, 0xc9, 0x31, 0xc1, 0x63, 0xc3, 0xb0,
                0x2d, 0xe4, 0x12, 0x45,
            ],
            Elements::RightShiftWith16 => [
                0x1e, 0x18, 0x1c, 0x33, 0x16, 0x93, 0x59, 0x4c, 0x6e, 0x0e, 0x8f, 0xde, 0xb4, 0x0a,
                0x81, 0xa3, 0xaf, 0x8f, 0x56, 0xb7, 0xa5, 0x60, 0xde, 0x64, 0x41, 0x30, 0x3f, 0x65,
                0xf4, 0xfc, 0x93, 0x7c,
            ],
            Elements::RightShiftWith32 => [
                0x69, 0xdb, 0xe1, 0x90, 0xd7, 0x2d, 0x77, 0xd0, 0xd0, 0xdc, 0xf3, 0x25, 0xde, 0x96,
                0x59, 0x22, 0x14, 0x58, 0x1f, 0x11, 0xe9, 0xed, 0xca, 0x93, 0xe2, 0xf9, 0x28, 0x48,
                0x2b, 0x5e, 0x77, 0xa7,
            ],
            Elements::RightShiftWith64 => [
                0x2d, 0x0a, 0xb8, 0x83, 0x04, 0x69, 0x28, 0x0e, 0x2a, 0x28, 0x99, 0x3c, 0x5a, 0x05,
                0xf5, 0x6b, 0x91, 0xa8, 0xae, 0xb0, 0x34, 0xcc, 0xeb, 0xe0, 0x9c, 0x50, 0xf1, 0x3e,
                0xa7, 0x8d, 0xda, 0xfc,
            ],
            Elements::RightShiftWith8 => [
                0x1b, 0xdb, 0xdc, 0x8d, 0x8b, 0x74, 0x9b, 0xa3, 0xda, 0x75, 0x75, 0x58, 0x7d, 0x99,
                0x93, 0x00, 0x72, 0x60, 0x3f, 0x27, 0x5f, 0x7b, 0xd2, 0xf3, 0x24, 0xa3, 0x49, 0x51,
                0xd4, 0x46, 0x1b, 0x21,
            ],
            Elements::Rightmost16_1 => [
                0xe1, 0x29, 0xa8, 0xae, 0x88, 0x0f, 0x51, 0xca, 0x2a, 0x94, 0xdb, 0x44, 0xed, 0xec,
                0xa1, 0xc3, 0xa7, 0x66, 0xb7, 0x3e, 0x98, 0x97, 0x0b, 0x11, 0x98, 0xad, 0xe2, 0x16,
                0xae, 0x69, 0xcd, 0x2d,
            ],
            Elements::Rightmost16_2 => [
                0x8d, 0x0f, 0x68, 0xda, 0xdf, 0x54, 0x6c, 0x5e, 0xd3, 0x6f, 0x34, 0x70, 0x58, 0x02,
                0xb0, 0xce, 0x83, 0x9a, 0x63, 0xe5, 0x74, 0x49, 0x77, 0x85, 0x24, 0x30, 0x08, 0xab,
                0x42, 0x7e, 0x45, 0x6b,
            ],
            Elements::Rightmost16_4 => [
                0xb0, 0xd4, 0x13, 0x95, 0x41, 0xec, 0xab, 0x2c, 0x16, 0xfc, 0x1a, 0x87, 0x98, 0x9b,
                0xdd, 0x04, 0x53, 0x22, 0xef, 0xb1, 0xe7, 0x0b, 0xc1, 0xf7, 0xb0, 0x4d, 0x43, 0xb2,
                0x8b, 0xb3, 0x49, 0xff,
            ],
            Elements::Rightmost16_8 => [
                0x0f, 0x03, 0xfa, 0x0f, 0xa6, 0xce, 0xb5, 0x5d, 0xf9, 0x9b, 0x20, 0xd9, 0xef, 0xcf,
                0x37, 0x10, 0xa7, 0x08, 0xa2, 0x84, 0xa9, 0x5c, 0x33, 0x4c, 0x1d, 0xa3, 0xcb, 0xfe,
                0x02, 0xfb, 0x94, 0x67,
            ],
            Elements::Rightmost32_1 => [
                0x8f, 0x5e, 0x52, 0x63, 0xbb, 0x8e, 0xf8, 0x00, 0xc9, 0x9d, 0x0c, 0x23, 0xfc, 0xba,
                0xa3, 0x19, 0x8a, 0x6a, 0xbd, 0xf0, 0x08, 0x58, 0x1e, 0x8c, 0x89, 0x10, 0x52, 0xb4,
                0x0c, 0xa7, 0xf7, 0xa4,
            ],
            Elements::Rightmost32_16 => [
                0xb9, 0xa2, 0x3e, 0x1b, 0xf7, 0xc6, 0x81, 0x43, 0x51, 0x30, 0x74, 0xc9, 0x39, 0xbd,
                0x73, 0xc9, 0xbf, 0x8e, 0xb5, 0xaa, 0xce, 0x84, 0x15, 0xff, 0x01, 0x02, 0x2f, 0xca,
                0x65, 0xb3, 0xa3, 0x42,
            ],
            Elements::Rightmost32_2 => [
                0xab, 0xf3, 0x23, 0x8d, 0x3c, 0xbf, 0x0b, 0xf3, 0x5a, 0x83, 0x96, 0x1f, 0xb9, 0xf9,
                0x04, 0xb5, 0x6d, 0x3a, 0x9e, 0x0e, 0x35, 0xc8, 0x9d, 0xf8, 0x72, 0xc9, 0xc9, 0x38,
                0xd3, 0x44, 0xa5, 0x4a,
            ],
            Elements::Rightmost32_4 => [
                0xf7, 0xee, 0xd2, 0xec, 0x80, 0x59, 0x06, 0xfe, 0xb3, 0xac, 0x27, 0xf2, 0xde, 0xe5,
                0x3b, 0x58, 0xc3, 0xb1, 0x3e, 0x40, 0xe2, 0xbc, 0x3e, 0x8b, 0x10, 0x63, 0x2e, 0xd9,
                0xc0, 0xe7, 0xca, 0x5f,
            ],
            Elements::Rightmost32_8 => [
                0xf3, 0xe4, 0x39, 0xed, 0x98, 0x83, 0xc6, 0xa6, 0xb9, 0x07, 0x20, 0x53, 0x2e, 0xb4,
                0xe0, 0x43, 0xe8, 0x9a, 0x35, 0xf0, 0xb5, 0x29, 0x5f, 0xd5, 0x02, 0xa0, 0xb0, 0xb2,
                0x43, 0x6b, 0xd2, 0x13,
            ],
            Elements::Rightmost64_1 => [
                0xc9, 0x6b, 0xe3, 0xe3, 0x35, 0x48, 0x25, 0x8e, 0x30, 0x71, 0x7b, 0x30, 0x81, 0x7e,
                0x44, 0x0f, 0x0a, 0xf4, 0xb1, 0x89, 0x0e, 0xdf, 0xcf, 0x7f, 0xdc, 0xb3, 0x9c, 0xb9,
                0xef, 0xff, 0x47, 0x1d,
            ],
            Elements::Rightmost64_16 => [
                0x5d, 0x55, 0x5f, 0x83, 0xe4, 0x80, 0x87, 0xdb, 0x0c, 0x41, 0x5d, 0xad, 0x17, 0xf0,
                0x81, 0xd4, 0xf6, 0xb7, 0x60, 0xe9, 0x95, 0xf2, 0x72, 0xbb, 0xb6, 0xe4, 0xcb, 0x42,
                0xd0, 0xf5, 0x03, 0x25,
            ],
            Elements::Rightmost64_2 => [
                0xa9, 0xcb, 0x13, 0x43, 0xdb, 0xd5, 0x22, 0xb9, 0x1b, 0x64, 0x82, 0xe4, 0xba, 0xe6,
                0x2b, 0x0e, 0x5f, 0x82, 0x98, 0x68, 0x7e, 0x64, 0x23, 0x33, 0x5c, 0x6d, 0xf5, 0x06,
                0xdc, 0x42, 0x5b, 0x90,
            ],
            Elements::Rightmost64_32 => [
                0x47, 0x33, 0xb1, 0x92, 0x59, 0x80, 0x09, 0x64, 0x99, 0xb7, 0x87, 0x7c, 0x04, 0xe0,
                0x01, 0xba, 0xd3, 0x32, 0x5b, 0x2e, 0xca, 0xb3, 0x48, 0xe5, 0xad, 0xd7, 0x20, 0xd0,
                0x7b, 0x1b, 0x4a, 0x3a,
            ],
            Elements::Rightmost64_4 => [
                0x89, 0xda, 0xf7, 0xbe, 0x2c, 0xde, 0x58, 0xf0, 0x4e, 0x8d, 0xee, 0x58, 0xa4, 0x39,
                0x10, 0x91, 0x2c, 0x09, 0x6e, 0x95, 0xe1, 0x46, 0xc1, 0x9b, 0x00, 0xf5, 0x4f, 0xe8,
                0x74, 0x70, 0x07, 0x40,
            ],
            Elements::Rightmost64_8 => [
                0x1d, 0xfb, 0x2b, 0xef, 0x4c, 0xae, 0x45, 0x07, 0x92, 0x27, 0x08, 0xe5, 0xa5, 0x70,
                0x99, 0x49, 0x3f, 0xbe, 0x21, 0x15, 0x98, 0xee, 0xc0, 0xbf, 0xe0, 0xe7, 0x7b, 0x3d,
                0x41, 0xec, 0x89, 0xab,
            ],
            Elements::Rightmost8_1 => [
                0xce, 0xab, 0xd5, 0xca, 0x9f, 0xd9, 0x16, 0x2f, 0x99, 0x5e, 0x37, 0x35, 0x77, 0x04,
                0x7a, 0xa4, 0xba, 0x71, 0xf8, 0x07, 0xc7, 0x11, 0xf6, 0x0b, 0x08, 0xeb, 0x6a, 0x1c,
                0xfc, 0x38, 0x1c, 0x9c,
            ],
            Elements::Rightmost8_2 => [
                0x39, 0xb2, 0xf0, 0x37, 0xb6, 0xa0, 0x81, 0x86, 0x11, 0x50, 0x65, 0xf3, 0x85, 0x05,
                0x7a, 0xf3, 0xde, 0x3b, 0x9f, 0x0a, 0x9b, 0xda, 0x68, 0x33, 0x71, 0x46, 0x22, 0x59,
                0x41, 0x30, 0x28, 0xec,
            ],
            Elements::Rightmost8_4 => [
                0xa7, 0xa9, 0x49, 0x49, 0x0d, 0x1a, 0x00, 0xde, 0xfe, 0x5f, 0x61, 0x51, 0x29, 0x23,
                0x85, 0x0f, 0x51, 0xe3, 0x47, 0xc0, 0x6a, 0x8d, 0x76, 0xa0, 0xcd, 0xab, 0x87, 0xee,
                0xe2, 0x9a, 0x5d, 0xef,
            ],
            Elements::ScalarAdd => [
                0x4e, 0xe9, 0xa9, 0x6c, 0xef, 0x49, 0x6c, 0xf4, 0xa8, 0xfc, 0x4e, 0x8a, 0x8b, 0xc0,
                0xd1, 0x59, 0xca, 0x5f, 0xfb, 0x87, 0x53, 0x64, 0x3a, 0x8a, 0xdf, 0x63, 0x8a, 0xe8,
                0x9b, 0xbb, 0xb3, 0x45,
            ],
            Elements::ScalarInvert => [
                0x12, 0xb8, 0x55, 0xe5, 0xeb, 0xaa, 0x7f, 0x8b, 0xb4, 0x4f, 0xee, 0x26, 0x16, 0xa0,
                0x51, 0xad, 0x00, 0x49, 0x9f, 0x9d, 0xf2, 0xa2, 0xad, 0xf7, 0x99, 0x73, 0xe9, 0xdb,
                0x81, 0x85, 0x75, 0x9c,
            ],
            Elements::ScalarIsZero => [
                0x4d, 0x25, 0x28, 0x03, 0x45, 0x7b, 0x83, 0xb8, 0x5b, 0x98, 0x7f, 0x04, 0x87, 0x33,
                0xfb, 0xee, 0xde, 0xaa, 0x8d, 0x25, 0x9d, 0x32, 0x05, 0x07, 0x45, 0x00, 0x19, 0xc6,
                0x22, 0x03, 0x4f, 0x26,
            ],
            Elements::ScalarMultiply => [
                0x87, 0x56, 0xf2, 0xdc, 0x31, 0x0c, 0xde, 0xb6, 0x40, 0x45, 0xc4, 0x4c, 0x23, 0x66,
                0xe1, 0x4b, 0xc1, 0xfa, 0xfa, 0x17, 0x15, 0x9f, 0x2d, 0x7b, 0x48, 0x9b, 0xd9, 0x45,
                0x3e, 0xe3, 0x7e, 0xa0,
            ],
            Elements::ScalarMultiplyLambda => [
                0x2b, 0x31, 0xd3, 0x9e, 0xc4, 0xff, 0x37, 0x23, 0x1a, 0x1b, 0x3e, 0xbe, 0x75, 0x9d,
                0x41, 0xe0, 0xf5, 0xce, 0x34, 0x49, 0x2d, 0x4b, 0xd3, 0xc2, 0x09, 0x88, 0xc3, 0xf2,
                0xf7, 0xc5, 0x3e, 0xdc,
            ],
            Elements::ScalarNegate => [
                0xfc, 0x2e, 0xd1, 0x87, 0x50, 0xa2, 0x21, 0x81, 0xaf, 0x5b, 0x81, 0x41, 0x96, 0x92,
                0x73, 0xca, 0xaf, 0x72, 0xcc, 0x11, 0x31, 0xe1, 0x08, 0x2c, 0xf0, 0x08, 0xf5, 0xca,
                0x09, 0x09, 0xc2, 0x16,
            ],
            Elements::ScalarNormalize => [
                0x79, 0x7c, 0xff, 0xa8, 0x08, 0x59, 0xca, 0xb7, 0xcd, 0xbf, 0x3b, 0x9d, 0xe6, 0xe0,
                0xa8, 0xb7, 0x91, 0x48, 0x4e, 0xaa, 0xba, 0xcb, 0xdf, 0xba, 0xeb, 0x01, 0xe2, 0x38,
                0x95, 0xe9, 0x61, 0x99,
            ],
            Elements::ScalarSquare => [
                0x77, 0xb4, 0x3c, 0x60, 0x38, 0xad, 0x80, 0xb4, 0x6d, 0x3a, 0x76, 0xe2, 0x12, 0xb3,
                0xa8, 0xc0, 0xd2, 0xf0, 0x63, 0x07, 0xbc, 0x45, 0x6e, 0x40, 0xb5, 0xd6, 0xf4, 0xa3,
                0xa5, 0x0e, 0x26, 0x4d,
            ],
            Elements::Scale => [
                0x57, 0x4c, 0xe7, 0x60, 0x24, 0xa5, 0xf0, 0x11, 0xa2, 0xd0, 0xbc, 0xeb, 0xb0, 0xf8,
                0x1a, 0x15, 0xe0, 0xf9, 0xd0, 0x6b, 0x34, 0xf2, 0x09, 0x94, 0x33, 0xcb, 0x11, 0x4f,
                0x53, 0x46, 0x8d, 0x4f,
            ],
            Elements::ScriptCMR => [
                0xb8, 0xad, 0x37, 0x23, 0x76, 0x99, 0x30, 0x1f, 0xb3, 0x3e, 0x69, 0x28, 0xa4, 0x10,
                0x1f, 0x93, 0x16, 0x9a, 0xd1, 0xf4, 0x12, 0x8b, 0x9e, 0x1e, 0x86, 0xbd, 0xbb, 0x6c,
                0xdc, 0x27, 0x4c, 0x38,
            ],
            Elements::Sha256Block => [
                0x94, 0xa3, 0x6a, 0x40, 0x83, 0x30, 0x9e, 0x0b, 0x86, 0xde, 0x77, 0xd0, 0xfb, 0x48,
                0xd9, 0xd3, 0x31, 0xe2, 0xd2, 0xf1, 0x67, 0x74, 0x0b, 0x60, 0x6e, 0x60, 0x57, 0x4a,
                0xf4, 0x38, 0xcd, 0x86,
            ],
            Elements::Sha256Ctx8Add1 => [
                0x8a, 0x1d, 0x25, 0x70, 0x87, 0xb3, 0x2c, 0xcd, 0xc3, 0x32, 0x00, 0x37, 0x4e, 0x6e,
                0x95, 0xc8, 0x75, 0xa0, 0x5e, 0x54, 0x81, 0x22, 0x32, 0x3f, 0x6b, 0x7a, 0xb9, 0xc0,
                0x7e, 0xb9, 0xb6, 0xee,
            ],
            Elements::Sha256Ctx8Add128 => [
                0xe7, 0x77, 0x2c, 0xb9, 0xc0, 0xec, 0x42, 0x1a, 0xb8, 0xa7, 0x89, 0xd4, 0x5c, 0xd6,
                0x46, 0x61, 0xf4, 0x33, 0xdd, 0x7d, 0x3d, 0x2c, 0x94, 0xdc, 0x1f, 0x1c, 0x4f, 0x3a,
                0xf8, 0xc7, 0x80, 0xab,
            ],
            Elements::Sha256Ctx8Add16 => [
                0xf0, 0xf4, 0xcf, 0x99, 0xad, 0x58, 0xa0, 0x38, 0x5e, 0x16, 0xb8, 0x7d, 0xbf, 0x32,
                0x71, 0xc4, 0x75, 0x24, 0xde, 0xfd, 0x78, 0xf1, 0x1b, 0xbc, 0x74, 0x71, 0xfe, 0x4d,
                0xa9, 0x4a, 0xeb, 0xad,
            ],
            Elements::Sha256Ctx8Add2 => [
                0x79, 0x98, 0xc7, 0xc2, 0xbd, 0x81, 0x4b, 0x0c, 0x0f, 0x40, 0x3f, 0x58, 0xc5, 0x76,
                0xea, 0x56, 0x40, 0x7d, 0x25, 0xee, 0x22, 0x9f, 0xae, 0x62, 0x5b, 0xca, 0xb0, 0xc6,
                0x20, 0xa2, 0xa2, 0x9c,
            ],
            Elements::Sha256Ctx8Add256 => [
                0xa8, 0x74, 0x99, 0x61, 0xb3, 0x1a, 0xfe, 0x2f, 0xb5, 0x53, 0xb7, 0x0b, 0x4c, 0xea,
                0x78, 0x7d, 0xca, 0x47, 0x25, 0x84, 0x54, 0xd5, 0x83, 0xdc, 0x45, 0xa0, 0x78, 0x0d,
                0x5e, 0x2a, 0x2a, 0x50,
            ],
            Elements::Sha256Ctx8Add32 => [
                0xf2, 0x20, 0x68, 0xb7, 0x76, 0xa3, 0x78, 0x7f, 0x9d, 0x52, 0xec, 0x2a, 0x95, 0x91,
                0x95, 0x1f, 0x8a, 0x73, 0xf0, 0x09, 0x60, 0x09, 0x12, 0x35, 0x8a, 0x2a, 0x1e, 0x15,
                0x86, 0x4e, 0x80, 0xb2,
            ],
            Elements::Sha256Ctx8Add4 => [
                0x40, 0xe6, 0x6d, 0xec, 0xa1, 0x32, 0xf5, 0xae, 0x0c, 0x54, 0x93, 0x7b, 0x95, 0xcc,
                0xac, 0xa1, 0x12, 0x67, 0xa4, 0xad, 0xca, 0x70, 0x28, 0x87, 0xb6, 0xe4, 0x08, 0xed,
                0x76, 0x15, 0x30, 0xbc,
            ],
            Elements::Sha256Ctx8Add512 => [
                0x5a, 0x96, 0xee, 0x66, 0x8b, 0x52, 0xf6, 0x3f, 0x7b, 0xdb, 0xaa, 0xe3, 0xcc, 0x38,
                0xe3, 0x93, 0xef, 0x24, 0xb3, 0xa9, 0x67, 0x85, 0x6f, 0x2c, 0xd8, 0x9d, 0x83, 0xba,
                0x4c, 0xcb, 0xda, 0x8d,
            ],
            Elements::Sha256Ctx8Add64 => [
                0x36, 0xed, 0xe6, 0xb6, 0x8a, 0xb6, 0xe4, 0xdb, 0x31, 0xef, 0xc7, 0xa7, 0xea, 0xe7,
                0xec, 0x3f, 0x7c, 0xee, 0xad, 0x93, 0x91, 0x10, 0x3b, 0x8d, 0xcb, 0x40, 0x60, 0x91,
                0x57, 0xfb, 0x87, 0x02,
            ],
            Elements::Sha256Ctx8Add8 => [
                0xac, 0x1f, 0xf0, 0xfb, 0x0c, 0xf7, 0x1d, 0x71, 0x13, 0xdb, 0x42, 0xaa, 0x1d, 0xdb,
                0x89, 0x55, 0x96, 0xe6, 0x45, 0xd7, 0x63, 0xb8, 0xdf, 0x5e, 0x87, 0x25, 0xf5, 0x14,
                0x4c, 0x39, 0x95, 0x88,
            ],
            Elements::Sha256Ctx8AddBuffer511 => [
                0x69, 0x01, 0xac, 0x0e, 0x30, 0xfd, 0x59, 0xce, 0xc8, 0x79, 0xd1, 0x69, 0x3b, 0x26,
                0x79, 0x59, 0x35, 0x69, 0x07, 0x84, 0x4b, 0x57, 0x7e, 0xdc, 0x3c, 0xe0, 0xe3, 0xf7,
                0x14, 0xa8, 0xef, 0x31,
            ],
            Elements::Sha256Ctx8Finalize => [
                0x84, 0x61, 0x59, 0x54, 0x00, 0x7e, 0xd8, 0x23, 0xd6, 0x05, 0x46, 0xd7, 0x5b, 0x04,
                0xb9, 0x09, 0xbc, 0x90, 0x92, 0x06, 0x37, 0x14, 0x73, 0xda, 0xc7, 0x0e, 0x12, 0x68,
                0x04, 0x77, 0x08, 0xcb,
            ],
            Elements::Sha256Ctx8Init => [
                0x6a, 0xdd, 0xa7, 0xd7, 0x33, 0x9f, 0x7d, 0xce, 0x4d, 0x62, 0xff, 0x82, 0x28, 0x16,
                0xda, 0x8d, 0xf5, 0x6a, 0xf6, 0x24, 0x3f, 0xa0, 0x73, 0xa2, 0x5c, 0x4c, 0x7c, 0xb5,
                0x7e, 0x01, 0x12, 0xb3,
            ],
            Elements::Sha256Iv => [
                0x6a, 0x18, 0xe6, 0xe7, 0x64, 0xaf, 0x80, 0x0d, 0xc6, 0xfa, 0xeb, 0x07, 0x54, 0xbf,
                0x0b, 0x17, 0x32, 0x9f, 0x98, 0x28, 0x1d, 0x13, 0xf5, 0x15, 0x77, 0x00, 0xfa, 0x6a,
                0x1d, 0x6d, 0x5d, 0x42,
            ],
            Elements::SigAllHash => [
                0x0d, 0xc3, 0xf1, 0x0b, 0xbb, 0xbf, 0x9a, 0xa4, 0x91, 0xbb, 0x88, 0x14, 0x63, 0x1d,
                0x3b, 0x6d, 0x41, 0x95, 0xf8, 0x2c, 0x97, 0x73, 0xbb, 0xcb, 0x2d, 0xa3, 0xef, 0x0c,
                0xf0, 0xcd, 0x94, 0xd2,
            ],
            Elements::Some1 => [
                0x0b, 0x9c, 0xb7, 0xb4, 0x7d, 0xeb, 0x4f, 0x9d, 0x95, 0xd5, 0xc0, 0x20, 0x00, 0x1f,
                0xd0, 0x09, 0xa2, 0xf1, 0x0c, 0xe5, 0xd9, 0x18, 0xd8, 0x18, 0x1e, 0x25, 0x93, 0x15,
                0xfe, 0x8e, 0xac, 0x53,
            ],
            Elements::Some16 => [
                0x30, 0xd8, 0x14, 0xff, 0xb4, 0x92, 0x78, 0xb4, 0x25, 0x00, 0x7b, 0x9d, 0xe2, 0x79,
                0xf7, 0x6f, 0x4a, 0x6d, 0xa4, 0xc0, 0x34, 0x63, 0x4a, 0xbb, 0x87, 0x11, 0x0e, 0xcb,
                0xea, 0x2c, 0xe4, 0x29,
            ],
            Elements::Some32 => [
                0x69, 0x27, 0x91, 0x90, 0x3b, 0xe7, 0xd9, 0xe4, 0xc5, 0x47, 0x72, 0xba, 0x88, 0xa4,
                0x86, 0x15, 0x46, 0x36, 0x12, 0x18, 0xdd, 0x8a, 0x26, 0xce, 0xed, 0x69, 0x9b, 0xcf,
                0x77, 0xc0, 0x99, 0x09,
            ],
            Elements::Some64 => [
                0xfa, 0x9b, 0x01, 0x60, 0xc7, 0x27, 0x15, 0xff, 0xfd, 0x1d, 0x94, 0xda, 0x19, 0x97,
                0x88, 0x28, 0x09, 0xb4, 0x62, 0xbb, 0x14, 0x4a, 0xce, 0xcb, 0x43, 0x05, 0x44, 0x68,
                0xe6, 0x37, 0x86, 0xb5,
            ],
            Elements::Some8 => [
                0x0b, 0xd7, 0xab, 0x43, 0x80, 0xc8, 0xf4, 0x5a, 0xca, 0x7c, 0xac, 0x97, 0x28, 0x86,
                0xce, 0xef, 0x23, 0xba, 0x84, 0x21, 0x0c, 0x5c, 0x4d, 0x96, 0x9b, 0x1f, 0x59, 0xb1,
                0x83, 0x2c, 0x1d, 0x36,
            ],
            Elements::Subtract16 => [
                0x0c, 0xd3, 0xab, 0x73, 0xe5, 0xce, 0x2a, 0x44, 0xf2, 0xd1, 0xc3, 0x32, 0xa1, 0xed,
                0x5a, 0xef, 0x5b, 0xcb, 0x60, 0x4f, 0x72, 0x1b, 0x15, 0xb8, 0x01, 0x0d, 0xd5, 0x4f,
                0x40, 0xc6, 0xac, 0xa7,
            ],
            Elements::Subtract32 => [
                0xe0, 0xde, 0x68, 0x76, 0x25, 0x9d, 0x5a, 0x00, 0x4b, 0x30, 0x16, 0xd3, 0x58, 0x15,
                0xdb, 0x41, 0xbc, 0xec, 0xdb, 0xfa, 0x18, 0xd3, 0x7d, 0x99, 0x20, 0x4e, 0x49, 0x57,
                0xad, 0x2c, 0x4d, 0x0e,
            ],
            Elements::Subtract64 => [
                0xff, 0xe6, 0x73, 0xee, 0x8e, 0xdc, 0x72, 0x9a, 0x47, 0xec, 0xed, 0x3a, 0x67, 0x7f,
                0x85, 0xb1, 0xda, 0xe7, 0x14, 0xa5, 0x10, 0x0c, 0x49, 0x49, 0x73, 0xab, 0xcb, 0x32,
                0x59, 0xa3, 0xc7, 0x56,
            ],
            Elements::Subtract8 => [
                0xbd, 0xa5, 0x98, 0x13, 0x96, 0x47, 0x8e, 0xb3, 0x6f, 0x85, 0x92, 0xa7, 0x50, 0x9f,
                0xa4, 0x87, 0x7c, 0x50, 0xb2, 0xbf, 0x91, 0x65, 0xe5, 0xb7, 0x96, 0x35, 0xbf, 0x8b,
                0xcb, 0x84, 0xd4, 0x42,
            ],
            Elements::Swu => [
                0x69, 0x45, 0x9b, 0x6d, 0xd1, 0x0a, 0x63, 0xdf, 0x37, 0xde, 0xdf, 0x18, 0x6c, 0xb4,
                0xd2, 0x08, 0xd8, 0xc0, 0x39, 0x61, 0x1c, 0xec, 0xdd, 0x09, 0xcf, 0xe0, 0x7a, 0xc9,
                0x1d, 0x01, 0x42, 0x2f,
            ],
            Elements::TapEnvHash => [
                0x59, 0xe0, 0x4e, 0xee, 0x36, 0x88, 0x3d, 0xd5, 0xe7, 0xc2, 0x69, 0x5d, 0x7c, 0x99,
                0x38, 0xc0, 0x6d, 0x98, 0x3f, 0xba, 0x88, 0xdc, 0x2a, 0x04, 0x84, 0x3a, 0xb8, 0x0b,
                0x68, 0xf1, 0x99, 0xec,
            ],
            Elements::TapleafHash => [
                0xea, 0x81, 0x56, 0x58, 0xff, 0x9a, 0x27, 0x16, 0x42, 0x0d, 0x45, 0x91, 0x5d, 0x66,
                0x07, 0x0b, 0xe4, 0x11, 0x6c, 0x1c, 0x70, 0x7e, 0x26, 0x94, 0xd3, 0x66, 0x58, 0x45,
                0x20, 0xc2, 0x24, 0x69,
            ],
            Elements::TapleafVersion => [
                0x30, 0x3e, 0xf4, 0xde, 0x61, 0x8e, 0x7f, 0x08, 0x67, 0x79, 0x48, 0x28, 0x2d, 0xd4,
                0x2d, 0x57, 0xd6, 0x44, 0xc4, 0x1c, 0x18, 0xc1, 0x1c, 0x13, 0x0a, 0x1b, 0xe5, 0xa6,
                0xe2, 0x54, 0xe3, 0xe9,
            ],
            Elements::Tappath => [
                0x5f, 0xd7, 0x1c, 0x6f, 0xf6, 0x7b, 0xa4, 0x26, 0x3c, 0x82, 0x56, 0xfe, 0xce, 0xe1,
                0xe2, 0xec, 0x1d, 0xcf, 0x45, 0xa8, 0xb3, 0x84, 0x50, 0x62, 0x70, 0x15, 0x17, 0x80,
                0xaa, 0xcf, 0x1c, 0x32,
            ],
            Elements::TappathHash => [
                0x62, 0xaf, 0x75, 0xf4, 0xf3, 0x7c, 0x0f, 0xf2, 0x75, 0x37, 0xe0, 0x54, 0x3a, 0x13,
                0x48, 0x02, 0x66, 0xa5, 0xad, 0x12, 0xce, 0x53, 0xdf, 0x1f, 0xba, 0xc1, 0x11, 0xea,
                0xa9, 0x3d, 0x27, 0x7c,
            ],
            Elements::TotalFee => [
                0xcc, 0x69, 0xd2, 0x4c, 0x5d, 0xd8, 0x52, 0xcd, 0xef, 0xca, 0xa7, 0x4d, 0x31, 0x40,
                0x46, 0x7f, 0xbe, 0x8d, 0x08, 0xa7, 0x70, 0x94, 0x22, 0xee, 0xc2, 0x8e, 0xd6, 0x32,
                0xb2, 0x8a, 0xba, 0x63,
            ],
            Elements::TransactionId => [
                0xab, 0x4b, 0xf4, 0x78, 0x2d, 0x35, 0x8c, 0xd3, 0x1f, 0x22, 0xe0, 0x66, 0x64, 0x16,
                0x70, 0x78, 0x45, 0x7c, 0x76, 0xe3, 0xfa, 0x8e, 0x42, 0x1c, 0x79, 0x29, 0xde, 0x11,
                0x31, 0x0b, 0x07, 0xd9,
            ],
            Elements::TxHash => [
                0xb7, 0xe0, 0xa2, 0xdc, 0x5c, 0xa1, 0x1c, 0xe5, 0x82, 0xa7, 0xaa, 0x29, 0x9a, 0x51,
                0xb2, 0x3e, 0x66, 0xef, 0xe6, 0x23, 0x60, 0x2a, 0x7b, 0x7d, 0x43, 0x4e, 0xec, 0x51,
                0xaf, 0x24, 0x5a, 0x07,
            ],
            Elements::TxIsFinal => [
                0xeb, 0x8e, 0x83, 0xd1, 0xe0, 0x2c, 0x3c, 0xe7, 0xaf, 0x2d, 0x28, 0xf0, 0xce, 0xc0,
                0x43, 0x14, 0x80, 0xdc, 0x2f, 0x38, 0x85, 0xc9, 0x91, 0x3a, 0x50, 0x72, 0x7e, 0x43,
                0xd0, 0xbd, 0xaf, 0x88,
            ],
            Elements::TxLockDistance => [
                0xa3, 0x78, 0x93, 0xc9, 0x7c, 0x9b, 0x95, 0x0e, 0x0f, 0x7f, 0x10, 0x45, 0xc8, 0xd8,
                0x47, 0x3a, 0xc8, 0x33, 0x6a, 0xae, 0xe9, 0xea, 0x1f, 0xad, 0x6b, 0x2a, 0x40, 0x61,
                0x72, 0x91, 0x49, 0xc6,
            ],
            Elements::TxLockDuration => [
                0x4d, 0x69, 0x87, 0xdd, 0x45, 0xf7, 0x8d, 0x5e, 0x43, 0xf9, 0x38, 0x51, 0x64, 0xa0,
                0xbd, 0x08, 0x80, 0x75, 0x2a, 0xb8, 0x11, 0xb3, 0x8b, 0xd2, 0x97, 0xda, 0x00, 0x16,
                0x15, 0x26, 0xa9, 0xca,
            ],
            Elements::TxLockHeight => [
                0x16, 0xab, 0x83, 0x0f, 0xac, 0xe5, 0x1f, 0xe4, 0xe1, 0x80, 0x89, 0xec, 0xa1, 0xb5,
                0x52, 0x34, 0x36, 0xaa, 0x45, 0xef, 0x84, 0x13, 0x21, 0x82, 0xc7, 0xcc, 0x65, 0x83,
                0x6f, 0xd1, 0x34, 0x2c,
            ],
            Elements::TxLockTime => [
                0xaf, 0x11, 0x2f, 0xc8, 0xee, 0xd3, 0xc3, 0x3b, 0xc7, 0xb5, 0xea, 0x04, 0xcc, 0xab,
                0x86, 0x99, 0x19, 0x53, 0xf2, 0xa2, 0xaa, 0xf3, 0xd8, 0x7d, 0x6c, 0x2c, 0x43, 0x14,
                0x3d, 0x42, 0x6f, 0x56,
            ],
            Elements::Verify => [
                0xf1, 0x1c, 0x94, 0x81, 0xe7, 0x18, 0x63, 0xa2, 0x44, 0x53, 0xc3, 0xe2, 0x13, 0x04,
                0x64, 0x99, 0xa0, 0x3a, 0x9a, 0x0f, 0x99, 0x3b, 0xe3, 0xc4, 0x8e, 0x9d, 0x1f, 0x40,
                0x5d, 0x97, 0x94, 0x7c,
            ],
            Elements::Version => [
                0x22, 0x21, 0x53, 0x00, 0x9f, 0x08, 0x63, 0xda, 0x5c, 0xec, 0x8f, 0xfa, 0xa4, 0xa0,
                0x17, 0xd8, 0xe4, 0xff, 0x25, 0x66, 0xfa, 0x9c, 0x03, 0xe7, 0x43, 0x1a, 0x09, 0x02,
                0xd5, 0x6a, 0x24, 0xdc,
            ],
            Elements::Xor1 => [
                0x77, 0xb7, 0x14, 0xe6, 0x89, 0xc9, 0xd6, 0xa4, 0x8f, 0xd1, 0xad, 0xd8, 0x65, 0x22,
                0x82, 0x3d, 0xeb, 0xc7, 0x0d, 0xf6, 0xa7, 0xfe, 0x4b, 0xf2, 0xb8, 0x5d, 0xe5, 0x49,
                0xe0, 0xcd, 0x0a, 0x05,
            ],
            Elements::Xor16 => [
                0xca, 0x36, 0x35, 0x51, 0x35, 0xa8, 0x6a, 0x11, 0x68, 0x6c, 0x01, 0xaa, 0x35, 0xf2,
                0x5b, 0x97, 0xfa, 0xee, 0xda, 0xbf, 0xde, 0xc8, 0xdf, 0x08, 0xd2, 0xc0, 0xf6, 0x65,
                0x08, 0x33, 0xf9, 0x3f,
            ],
            Elements::Xor32 => [
                0xff, 0xe2, 0xc8, 0xee, 0x96, 0xd5, 0x57, 0x97, 0x81, 0xc4, 0x36, 0x62, 0x88, 0xd9,
                0x50, 0x71, 0x85, 0xe4, 0x61, 0xcc, 0xc4, 0x0a, 0x45, 0xbb, 0xcc, 0x55, 0x94, 0x89,
                0xd3, 0xc6, 0x96, 0x12,
            ],
            Elements::Xor64 => [
                0x5b, 0x3b, 0xb7, 0xb1, 0x8c, 0x70, 0x98, 0xf4, 0xe5, 0xcd, 0x14, 0x97, 0x22, 0xac,
                0x73, 0x09, 0xce, 0x66, 0xa3, 0xe2, 0x19, 0xc6, 0x1b, 0x33, 0x3f, 0x31, 0x33, 0x58,
                0x3a, 0x7b, 0x57, 0x2d,
            ],
            Elements::Xor8 => [
                0xff, 0x3e, 0x52, 0x62, 0x02, 0xff, 0x89, 0xcf, 0xf7, 0xbb, 0xe7, 0x0f, 0xdb, 0xf9,
                0xf9, 0x7d, 0x23, 0xc1, 0x2f, 0x6e, 0x2b, 0xb8, 0xbb, 0xe8, 0x30, 0x4a, 0xc7, 0x0f,
                0x61, 0xc1, 0xcf, 0x2c,
            ],
            Elements::XorXor1 => [
                0x22, 0x52, 0xa9, 0x86, 0x08, 0xd2, 0x0b, 0xd4, 0x11, 0x31, 0x7a, 0x20, 0x15, 0xc1,
                0x56, 0x98, 0x70, 0xa6, 0x2c, 0x95, 0x3a, 0x61, 0x65, 0xfb, 0xe9, 0x77, 0xb4, 0x0d,
                0x6c, 0xce, 0xa4, 0x95,
            ],
            Elements::XorXor16 => [
                0xa1, 0xf2, 0xd6, 0x33, 0xbf, 0x98, 0x89, 0xa0, 0x8a, 0x42, 0x51, 0x2a, 0x78, 0x93,
                0xa6, 0x79, 0x9d, 0xc4, 0x7a, 0xa8, 0x29, 0xff, 0x8f, 0x57, 0x7c, 0x5b, 0xc9, 0x75,
                0x66, 0xc4, 0xd3, 0xfe,
            ],
            Elements::XorXor32 => [
                0x0d, 0x5f, 0xf5, 0x81, 0x23, 0xba, 0x0d, 0xae, 0x3b, 0x32, 0x40, 0xa6, 0x31, 0x05,
                0x2a, 0xf2, 0xe8, 0x7b, 0x52, 0xb6, 0x37, 0xa2, 0xcb, 0xd3, 0x37, 0xd2, 0x25, 0x93,
                0x70, 0x62, 0x87, 0x41,
            ],
            Elements::XorXor64 => [
                0x78, 0x3f, 0x49, 0xa1, 0x9d, 0x4f, 0x4a, 0xae, 0x4d, 0x3c, 0x1d, 0x6c, 0xcf, 0x83,
                0x15, 0x6d, 0xc5, 0x5d, 0x0b, 0x5c, 0x08, 0xcf, 0x59, 0x23, 0x36, 0x58, 0x4c, 0xb1,
                0x31, 0x67, 0xc6, 0xca,
            ],
            Elements::XorXor8 => [
                0x83, 0xa9, 0x80, 0xcc, 0x61, 0x06, 0x85, 0x24, 0x88, 0x10, 0x5d, 0x3c, 0xee, 0x10,
                0xf3, 0x51, 0x13, 0xb8, 0xc9, 0xf7, 0x46, 0x64, 0xe7, 0xce, 0x6d, 0x4e, 0xc0, 0x91,
                0x2b, 0xc2, 0x9b, 0xc7,
            ],
        };

        Cmr(Midstate(bytes))
    }

    fn source_ty(&self) -> TypeName {
        let name: &'static [u8] = match self {
            Elements::Add16 => b"i",
            Elements::Add32 => b"l",
            Elements::Add64 => b"*ll",
            Elements::Add8 => b"****22*22**22*22***22*22**22*22",
            Elements::All16 => b"****22*22**22*22***22*22**22*22",
            Elements::All32 => b"i",
            Elements::All64 => b"l",
            Elements::All8 => b"***22*22**22*22",
            Elements::And1 => b"*22",
            Elements::And16 => b"i",
            Elements::And32 => b"l",
            Elements::And64 => b"*ll",
            Elements::And8 => b"****22*22**22*22***22*22**22*22",
            Elements::AnnexHash => b"***+1h*+1*ll*+1l*+1i*+1****22*22**22*22***22*22**22*22+1***22*22**22*22*lh+1h",
            Elements::AssetAmountHash => b"***+1h*+1*ll*+1l*+1i*+1****22*22**22*22***22*22**22*22+1***22*22**22*22*lh*+*2hh+*2hl",
            Elements::Bip0340Verify => b"**hh*hh",
            Elements::BuildTapbranch => b"*hh",
            Elements::BuildTapleafSimplicity => b"h",
            Elements::CalculateAsset => b"h",
            Elements::CalculateConfidentialToken => b"h",
            Elements::CalculateExplicitToken => b"h",
            Elements::CalculateIssuanceEntropy => b"**hih",
            Elements::Ch1 => b"*2*22",
            Elements::Ch16 => b"*****22*22**22*22***22*22**22*22i",
            Elements::Ch32 => b"*il",
            Elements::Ch64 => b"*l*ll",
            Elements::Ch8 => b"****22*22**22*22****22*22**22*22***22*22**22*22",
            Elements::CheckLockDistance => b"****22*22**22*22***22*22**22*22",
            Elements::CheckLockDuration => b"****22*22**22*22***22*22**22*22",
            Elements::CheckLockHeight => b"i",
            Elements::CheckLockTime => b"i",
            Elements::CheckSigVerify => b"**h*hh*hh",
            Elements::Complement1 => b"2",
            Elements::Complement16 => b"****22*22**22*22***22*22**22*22",
            Elements::Complement32 => b"i",
            Elements::Complement64 => b"l",
            Elements::Complement8 => b"***22*22**22*22",
            Elements::CurrentAmount => b"1",
            Elements::CurrentAnnexHash => b"1",
            Elements::CurrentAsset => b"1",
            Elements::CurrentIndex => b"1",
            Elements::CurrentIssuanceAssetAmount => b"1",
            Elements::CurrentIssuanceAssetProof => b"1",
            Elements::CurrentIssuanceTokenAmount => b"1",
            Elements::CurrentIssuanceTokenProof => b"1",
            Elements::CurrentNewIssuanceContract => b"1",
            Elements::CurrentPegin => b"1",
            Elements::CurrentPrevOutpoint => b"1",
            Elements::CurrentReissuanceBlinding => b"1",
            Elements::CurrentReissuanceEntropy => b"1",
            Elements::CurrentScriptHash => b"1",
            Elements::CurrentScriptSigHash => b"1",
            Elements::CurrentSequence => b"1",
            Elements::Decompress => b"*2h",
            Elements::Decrement16 => b"****22*22**22*22***22*22**22*22",
            Elements::Decrement32 => b"i",
            Elements::Decrement64 => b"l",
            Elements::Decrement8 => b"***22*22**22*22",
            Elements::DivMod128_64 => b"**lll",
            Elements::DivMod16 => b"i",
            Elements::DivMod32 => b"l",
            Elements::DivMod64 => b"*ll",
            Elements::DivMod8 => b"****22*22**22*22***22*22**22*22",
            Elements::Divide16 => b"i",
            Elements::Divide32 => b"l",
            Elements::Divide64 => b"*ll",
            Elements::Divide8 => b"****22*22**22*22***22*22**22*22",
            Elements::Divides16 => b"i",
            Elements::Divides32 => b"l",
            Elements::Divides64 => b"*ll",
            Elements::Divides8 => b"****22*22**22*22***22*22**22*22",
            Elements::Eq1 => b"*22",
            Elements::Eq16 => b"i",
            Elements::Eq256 => b"*hh",
            Elements::Eq32 => b"l",
            Elements::Eq64 => b"*ll",
            Elements::Eq8 => b"****22*22**22*22***22*22**22*22",
            Elements::FeAdd => b"*hh",
            Elements::FeInvert => b"h",
            Elements::FeIsOdd => b"h",
            Elements::FeIsZero => b"h",
            Elements::FeMultiply => b"*hh",
            Elements::FeMultiplyBeta => b"h",
            Elements::FeNegate => b"h",
            Elements::FeNormalize => b"h",
            Elements::FeSquare => b"h",
            Elements::FeSquareRoot => b"h",
            Elements::FullAdd16 => b"*2i",
            Elements::FullAdd32 => b"*2l",
            Elements::FullAdd64 => b"*2*ll",
            Elements::FullAdd8 => b"*2****22*22**22*22***22*22**22*22",
            Elements::FullDecrement16 => b"*2****22*22**22*22***22*22**22*22",
            Elements::FullDecrement32 => b"*2i",
            Elements::FullDecrement64 => b"*2l",
            Elements::FullDecrement8 => b"*2***22*22**22*22",
            Elements::FullIncrement16 => b"*2****22*22**22*22***22*22**22*22",
            Elements::FullIncrement32 => b"*2i",
            Elements::FullIncrement64 => b"*2l",
            Elements::FullIncrement8 => b"*2***22*22**22*22",
            Elements::FullLeftShift16_1 => b"*****22*22**22*22***22*22**22*222",
            Elements::FullLeftShift16_2 => b"*****22*22**22*22***22*22**22*22*22",
            Elements::FullLeftShift16_4 => b"*****22*22**22*22***22*22**22*22**22*22",
            Elements::FullLeftShift16_8 => b"*****22*22**22*22***22*22**22*22***22*22**22*22",
            Elements::FullLeftShift32_1 => b"*i2",
            Elements::FullLeftShift32_16 => b"*i****22*22**22*22***22*22**22*22",
            Elements::FullLeftShift32_2 => b"*i*22",
            Elements::FullLeftShift32_4 => b"*i**22*22",
            Elements::FullLeftShift32_8 => b"*i***22*22**22*22",
            Elements::FullLeftShift64_1 => b"*l2",
            Elements::FullLeftShift64_16 => b"*l****22*22**22*22***22*22**22*22",
            Elements::FullLeftShift64_2 => b"*l*22",
            Elements::FullLeftShift64_32 => b"*li",
            Elements::FullLeftShift64_4 => b"*l**22*22",
            Elements::FullLeftShift64_8 => b"*l***22*22**22*22",
            Elements::FullLeftShift8_1 => b"****22*22**22*222",
            Elements::FullLeftShift8_2 => b"****22*22**22*22*22",
            Elements::FullLeftShift8_4 => b"****22*22**22*22**22*22",
            Elements::FullMultiply16 => b"l",
            Elements::FullMultiply32 => b"*ll",
            Elements::FullMultiply64 => b"h",
            Elements::FullMultiply8 => b"i",
            Elements::FullRightShift16_1 => b"*2****22*22**22*22***22*22**22*22",
            Elements::FullRightShift16_2 => b"**22****22*22**22*22***22*22**22*22",
            Elements::FullRightShift16_4 => b"***22*22****22*22**22*22***22*22**22*22",
            Elements::FullRightShift16_8 => b"****22*22**22*22****22*22**22*22***22*22**22*22",
            Elements::FullRightShift32_1 => b"*2i",
            Elements::FullRightShift32_16 => b"*****22*22**22*22***22*22**22*22i",
            Elements::FullRightShift32_2 => b"**22i",
            Elements::FullRightShift32_4 => b"***22*22i",
            Elements::FullRightShift32_8 => b"****22*22**22*22i",
            Elements::FullRightShift64_1 => b"*2l",
            Elements::FullRightShift64_16 => b"*****22*22**22*22***22*22**22*22l",
            Elements::FullRightShift64_2 => b"**22l",
            Elements::FullRightShift64_32 => b"*il",
            Elements::FullRightShift64_4 => b"***22*22l",
            Elements::FullRightShift64_8 => b"****22*22**22*22l",
            Elements::FullRightShift8_1 => b"*2***22*22**22*22",
            Elements::FullRightShift8_2 => b"**22***22*22**22*22",
            Elements::FullRightShift8_4 => b"***22*22***22*22**22*22",
            Elements::FullSubtract16 => b"*2i",
            Elements::FullSubtract32 => b"*2l",
            Elements::FullSubtract64 => b"*2*ll",
            Elements::FullSubtract8 => b"*2****22*22**22*22***22*22**22*22",
            Elements::GeIsOnCurve => b"*hh",
            Elements::GeNegate => b"*hh",
            Elements::GejAdd => b"***hhh**hhh",
            Elements::GejDouble => b"**hhh",
            Elements::GejEquiv => b"***hhh**hhh",
            Elements::GejGeAdd => b"***hhh*hh",
            Elements::GejGeAddEx => b"***hhh*hh",
            Elements::GejGeEquiv => b"***hhh*hh",
            Elements::GejInfinity => b"1",
            Elements::GejIsInfinity => b"**hhh",
            Elements::GejIsOnCurve => b"**hhh",
            Elements::GejNegate => b"**hhh",
            Elements::GejNormalize => b"**hhh",
            Elements::GejRescale => b"***hhhh",
            Elements::GejXEquiv => b"*h**hhh",
            Elements::GejYIsOdd => b"**hhh",
            Elements::Generate => b"h",
            Elements::GenesisBlockHash => b"1",
            Elements::HashToCurve => b"h",
            Elements::High1 => b"1",
            Elements::High16 => b"1",
            Elements::High32 => b"1",
            Elements::High64 => b"1",
            Elements::High8 => b"1",
            Elements::Increment16 => b"****22*22**22*22***22*22**22*22",
            Elements::Increment32 => b"i",
            Elements::Increment64 => b"l",
            Elements::Increment8 => b"***22*22**22*22",
            Elements::InputAmount => b"i",
            Elements::InputAmountsHash => b"1",
            Elements::InputAnnexHash => b"i",
            Elements::InputAnnexesHash => b"1",
            Elements::InputAsset => b"i",
            Elements::InputHash => b"i",
            Elements::InputOutpointsHash => b"1",
            Elements::InputPegin => b"i",
            Elements::InputPrevOutpoint => b"i",
            Elements::InputScriptHash => b"i",
            Elements::InputScriptSigHash => b"i",
            Elements::InputScriptSigsHash => b"1",
            Elements::InputScriptsHash => b"1",
            Elements::InputSequence => b"i",
            Elements::InputSequencesHash => b"1",
            Elements::InputUtxoHash => b"i",
            Elements::InputUtxosHash => b"1",
            Elements::InputsHash => b"1",
            Elements::InternalKey => b"1",
            Elements::IsOne16 => b"****22*22**22*22***22*22**22*22",
            Elements::IsOne32 => b"i",
            Elements::IsOne64 => b"l",
            Elements::IsOne8 => b"***22*22**22*22",
            Elements::IsZero16 => b"****22*22**22*22***22*22**22*22",
            Elements::IsZero32 => b"i",
            Elements::IsZero64 => b"l",
            Elements::IsZero8 => b"***22*22**22*22",
            Elements::Issuance => b"i",
            Elements::IssuanceAsset => b"i",
            Elements::IssuanceAssetAmount => b"i",
            Elements::IssuanceAssetAmountsHash => b"1",
            Elements::IssuanceAssetProof => b"i",
            Elements::IssuanceBlindingEntropyHash => b"1",
            Elements::IssuanceEntropy => b"i",
            Elements::IssuanceHash => b"i",
            Elements::IssuanceRangeProofsHash => b"1",
            Elements::IssuanceToken => b"i",
            Elements::IssuanceTokenAmount => b"i",
            Elements::IssuanceTokenAmountsHash => b"1",
            Elements::IssuanceTokenProof => b"i",
            Elements::IssuancesHash => b"1",
            Elements::LbtcAsset => b"1",
            Elements::Le16 => b"i",
            Elements::Le32 => b"l",
            Elements::Le64 => b"*ll",
            Elements::Le8 => b"****22*22**22*22***22*22**22*22",
            Elements::LeftExtend16_32 => b"****22*22**22*22***22*22**22*22",
            Elements::LeftExtend16_64 => b"****22*22**22*22***22*22**22*22",
            Elements::LeftExtend1_16 => b"2",
            Elements::LeftExtend1_32 => b"2",
            Elements::LeftExtend1_64 => b"2",
            Elements::LeftExtend1_8 => b"2",
            Elements::LeftExtend32_64 => b"i",
            Elements::LeftExtend8_16 => b"***22*22**22*22",
            Elements::LeftExtend8_32 => b"***22*22**22*22",
            Elements::LeftExtend8_64 => b"***22*22**22*22",
            Elements::LeftPadHigh16_32 => b"****22*22**22*22***22*22**22*22",
            Elements::LeftPadHigh16_64 => b"****22*22**22*22***22*22**22*22",
            Elements::LeftPadHigh1_16 => b"2",
            Elements::LeftPadHigh1_32 => b"2",
            Elements::LeftPadHigh1_64 => b"2",
            Elements::LeftPadHigh1_8 => b"2",
            Elements::LeftPadHigh32_64 => b"i",
            Elements::LeftPadHigh8_16 => b"***22*22**22*22",
            Elements::LeftPadHigh8_32 => b"***22*22**22*22",
            Elements::LeftPadHigh8_64 => b"***22*22**22*22",
            Elements::LeftPadLow16_32 => b"****22*22**22*22***22*22**22*22",
            Elements::LeftPadLow16_64 => b"****22*22**22*22***22*22**22*22",
            Elements::LeftPadLow1_16 => b"2",
            Elements::LeftPadLow1_32 => b"2",
            Elements::LeftPadLow1_64 => b"2",
            Elements::LeftPadLow1_8 => b"2",
            Elements::LeftPadLow32_64 => b"i",
            Elements::LeftPadLow8_16 => b"***22*22**22*22",
            Elements::LeftPadLow8_32 => b"***22*22**22*22",
            Elements::LeftPadLow8_64 => b"***22*22**22*22",
            Elements::LeftRotate16 => b"***22*22****22*22**22*22***22*22**22*22",
            Elements::LeftRotate32 => b"****22*22**22*22i",
            Elements::LeftRotate64 => b"****22*22**22*22l",
            Elements::LeftRotate8 => b"***22*22***22*22**22*22",
            Elements::LeftShift16 => b"***22*22****22*22**22*22***22*22**22*22",
            Elements::LeftShift32 => b"****22*22**22*22i",
            Elements::LeftShift64 => b"****22*22**22*22l",
            Elements::LeftShift8 => b"***22*22***22*22**22*22",
            Elements::LeftShiftWith16 => b"*2***22*22****22*22**22*22***22*22**22*22",
            Elements::LeftShiftWith32 => b"*2****22*22**22*22i",
            Elements::LeftShiftWith64 => b"*2****22*22**22*22l",
            Elements::LeftShiftWith8 => b"*2***22*22***22*22**22*22",
            Elements::Leftmost16_1 => b"****22*22**22*22***22*22**22*22",
            Elements::Leftmost16_2 => b"****22*22**22*22***22*22**22*22",
            Elements::Leftmost16_4 => b"****22*22**22*22***22*22**22*22",
            Elements::Leftmost16_8 => b"****22*22**22*22***22*22**22*22",
            Elements::Leftmost32_1 => b"i",
            Elements::Leftmost32_16 => b"i",
            Elements::Leftmost32_2 => b"i",
            Elements::Leftmost32_4 => b"i",
            Elements::Leftmost32_8 => b"i",
            Elements::Leftmost64_1 => b"l",
            Elements::Leftmost64_16 => b"l",
            Elements::Leftmost64_2 => b"l",
            Elements::Leftmost64_32 => b"l",
            Elements::Leftmost64_4 => b"l",
            Elements::Leftmost64_8 => b"l",
            Elements::Leftmost8_1 => b"***22*22**22*22",
            Elements::Leftmost8_2 => b"***22*22**22*22",
            Elements::Leftmost8_4 => b"***22*22**22*22",
            Elements::LinearCombination1 => b"**h**hhhh",
            Elements::LinearVerify1 => b"***h*hhh*hh",
            Elements::LockTime => b"1",
            Elements::Low1 => b"1",
            Elements::Low16 => b"1",
            Elements::Low32 => b"1",
            Elements::Low64 => b"1",
            Elements::Low8 => b"1",
            Elements::Lt16 => b"i",
            Elements::Lt32 => b"l",
            Elements::Lt64 => b"*ll",
            Elements::Lt8 => b"****22*22**22*22***22*22**22*22",
            Elements::Maj1 => b"*2*22",
            Elements::Maj16 => b"*****22*22**22*22***22*22**22*22i",
            Elements::Maj32 => b"*il",
            Elements::Maj64 => b"*l*ll",
            Elements::Maj8 => b"****22*22**22*22****22*22**22*22***22*22**22*22",
            Elements::Max16 => b"i",
            Elements::Max32 => b"l",
            Elements::Max64 => b"*ll",
            Elements::Max8 => b"****22*22**22*22***22*22**22*22",
            Elements::Median16 => b"*****22*22**22*22***22*22**22*22i",
            Elements::Median32 => b"*il",
            Elements::Median64 => b"*l*ll",
            Elements::Median8 => b"****22*22**22*22****22*22**22*22***22*22**22*22",
            Elements::Min16 => b"i",
            Elements::Min32 => b"l",
            Elements::Min64 => b"*ll",
            Elements::Min8 => b"****22*22**22*22***22*22**22*22",
            Elements::Modulo16 => b"i",
            Elements::Modulo32 => b"l",
            Elements::Modulo64 => b"*ll",
            Elements::Modulo8 => b"****22*22**22*22***22*22**22*22",
            Elements::Multiply16 => b"i",
            Elements::Multiply32 => b"l",
            Elements::Multiply64 => b"*ll",
            Elements::Multiply8 => b"****22*22**22*22***22*22**22*22",
            Elements::Negate16 => b"****22*22**22*22***22*22**22*22",
            Elements::Negate32 => b"i",
            Elements::Negate64 => b"l",
            Elements::Negate8 => b"***22*22**22*22",
            Elements::NewIssuanceContract => b"i",
            Elements::NonceHash => b"***+1h*+1*ll*+1l*+1i*+1****22*22**22*22***22*22**22*22+1***22*22**22*22*lh+1+*2hh",
            Elements::NumInputs => b"1",
            Elements::NumOutputs => b"1",
            Elements::One16 => b"1",
            Elements::One32 => b"1",
            Elements::One64 => b"1",
            Elements::One8 => b"1",
            Elements::Or1 => b"*22",
            Elements::Or16 => b"i",
            Elements::Or32 => b"l",
            Elements::Or64 => b"*ll",
            Elements::Or8 => b"****22*22**22*22***22*22**22*22",
            Elements::OutpointHash => b"***+1h*+1*ll*+1l*+1i*+1****22*22**22*22***22*22**22*22+1***22*22**22*22*lh*+1h*hi",
            Elements::OutputAmount => b"i",
            Elements::OutputAmountsHash => b"1",
            Elements::OutputAsset => b"i",
            Elements::OutputHash => b"i",
            Elements::OutputIsFee => b"i",
            Elements::OutputNonce => b"i",
            Elements::OutputNoncesHash => b"1",
            Elements::OutputNullDatum => b"l",
            Elements::OutputRangeProof => b"i",
            Elements::OutputRangeProofsHash => b"1",
            Elements::OutputScriptHash => b"i",
            Elements::OutputScriptsHash => b"1",
            Elements::OutputSurjectionProof => b"i",
            Elements::OutputSurjectionProofsHash => b"1",
            Elements::OutputsHash => b"1",
            Elements::ParseLock => b"i",
            Elements::ParseSequence => b"i",
            Elements::PointVerify1 => b"***h*2hh*2h",
            Elements::ReissuanceBlinding => b"i",
            Elements::ReissuanceEntropy => b"i",
            Elements::RightExtend16_32 => b"****22*22**22*22***22*22**22*22",
            Elements::RightExtend16_64 => b"****22*22**22*22***22*22**22*22",
            Elements::RightExtend32_64 => b"i",
            Elements::RightExtend8_16 => b"***22*22**22*22",
            Elements::RightExtend8_32 => b"***22*22**22*22",
            Elements::RightExtend8_64 => b"***22*22**22*22",
            Elements::RightPadHigh16_32 => b"****22*22**22*22***22*22**22*22",
            Elements::RightPadHigh16_64 => b"****22*22**22*22***22*22**22*22",
            Elements::RightPadHigh1_16 => b"2",
            Elements::RightPadHigh1_32 => b"2",
            Elements::RightPadHigh1_64 => b"2",
            Elements::RightPadHigh1_8 => b"2",
            Elements::RightPadHigh32_64 => b"i",
            Elements::RightPadHigh8_16 => b"***22*22**22*22",
            Elements::RightPadHigh8_32 => b"***22*22**22*22",
            Elements::RightPadHigh8_64 => b"***22*22**22*22",
            Elements::RightPadLow16_32 => b"****22*22**22*22***22*22**22*22",
            Elements::RightPadLow16_64 => b"****22*22**22*22***22*22**22*22",
            Elements::RightPadLow1_16 => b"2",
            Elements::RightPadLow1_32 => b"2",
            Elements::RightPadLow1_64 => b"2",
            Elements::RightPadLow1_8 => b"2",
            Elements::RightPadLow32_64 => b"i",
            Elements::RightPadLow8_16 => b"***22*22**22*22",
            Elements::RightPadLow8_32 => b"***22*22**22*22",
            Elements::RightPadLow8_64 => b"***22*22**22*22",
            Elements::RightRotate16 => b"***22*22****22*22**22*22***22*22**22*22",
            Elements::RightRotate32 => b"****22*22**22*22i",
            Elements::RightRotate64 => b"****22*22**22*22l",
            Elements::RightRotate8 => b"***22*22***22*22**22*22",
            Elements::RightShift16 => b"***22*22****22*22**22*22***22*22**22*22",
            Elements::RightShift32 => b"****22*22**22*22i",
            Elements::RightShift64 => b"****22*22**22*22l",
            Elements::RightShift8 => b"***22*22***22*22**22*22",
            Elements::RightShiftWith16 => b"*2***22*22****22*22**22*22***22*22**22*22",
            Elements::RightShiftWith32 => b"*2****22*22**22*22i",
            Elements::RightShiftWith64 => b"*2****22*22**22*22l",
            Elements::RightShiftWith8 => b"*2***22*22***22*22**22*22",
            Elements::Rightmost16_1 => b"****22*22**22*22***22*22**22*22",
            Elements::Rightmost16_2 => b"****22*22**22*22***22*22**22*22",
            Elements::Rightmost16_4 => b"****22*22**22*22***22*22**22*22",
            Elements::Rightmost16_8 => b"****22*22**22*22***22*22**22*22",
            Elements::Rightmost32_1 => b"i",
            Elements::Rightmost32_16 => b"i",
            Elements::Rightmost32_2 => b"i",
            Elements::Rightmost32_4 => b"i",
            Elements::Rightmost32_8 => b"i",
            Elements::Rightmost64_1 => b"l",
            Elements::Rightmost64_16 => b"l",
            Elements::Rightmost64_2 => b"l",
            Elements::Rightmost64_32 => b"l",
            Elements::Rightmost64_4 => b"l",
            Elements::Rightmost64_8 => b"l",
            Elements::Rightmost8_1 => b"***22*22**22*22",
            Elements::Rightmost8_2 => b"***22*22**22*22",
            Elements::Rightmost8_4 => b"***22*22**22*22",
            Elements::ScalarAdd => b"*hh",
            Elements::ScalarInvert => b"h",
            Elements::ScalarIsZero => b"h",
            Elements::ScalarMultiply => b"*hh",
            Elements::ScalarMultiplyLambda => b"h",
            Elements::ScalarNegate => b"h",
            Elements::ScalarNormalize => b"h",
            Elements::ScalarSquare => b"h",
            Elements::Scale => b"*h**hhh",
            Elements::ScriptCMR => b"1",
            Elements::Sha256Block => b"*h*hh",
            Elements::Sha256Ctx8Add1 => b"***+1h*+1*ll*+1l*+1i*+1****22*22**22*22***22*22**22*22+1***22*22**22*22*lh***22*22**22*22",
            Elements::Sha256Ctx8Add128 => b"***+1h*+1*ll*+1l*+1i*+1****22*22**22*22***22*22**22*22+1***22*22**22*22*lh**hh*hh",
            Elements::Sha256Ctx8Add16 => b"***+1h*+1*ll*+1l*+1i*+1****22*22**22*22***22*22**22*22+1***22*22**22*22*lh*ll",
            Elements::Sha256Ctx8Add2 => b"***+1h*+1*ll*+1l*+1i*+1****22*22**22*22***22*22**22*22+1***22*22**22*22*lh****22*22**22*22***22*22**22*22",
            Elements::Sha256Ctx8Add256 => b"***+1h*+1*ll*+1l*+1i*+1****22*22**22*22***22*22**22*22+1***22*22**22*22*lh***hh*hh**hh*hh",
            Elements::Sha256Ctx8Add32 => b"***+1h*+1*ll*+1l*+1i*+1****22*22**22*22***22*22**22*22+1***22*22**22*22*lhh",
            Elements::Sha256Ctx8Add4 => b"***+1h*+1*ll*+1l*+1i*+1****22*22**22*22***22*22**22*22+1***22*22**22*22*lhi",
            Elements::Sha256Ctx8Add512 => b"***+1h*+1*ll*+1l*+1i*+1****22*22**22*22***22*22**22*22+1***22*22**22*22*lh****hh*hh**hh*hh***hh*hh**hh*hh",
            Elements::Sha256Ctx8Add64 => b"***+1h*+1*ll*+1l*+1i*+1****22*22**22*22***22*22**22*22+1***22*22**22*22*lh*hh",
            Elements::Sha256Ctx8Add8 => b"***+1h*+1*ll*+1l*+1i*+1****22*22**22*22***22*22**22*22+1***22*22**22*22*lhl",
            Elements::Sha256Ctx8AddBuffer511 => b"***+1h*+1*ll*+1l*+1i*+1****22*22**22*22***22*22**22*22+1***22*22**22*22*lh*+1***hh*hh**hh*hh*+1**hh*hh*+1*hh*+1h*+1*ll*+1l*+1i*+1****22*22**22*22***22*22**22*22+1***22*22**22*22",
            Elements::Sha256Ctx8Finalize => b"**+1h*+1*ll*+1l*+1i*+1****22*22**22*22***22*22**22*22+1***22*22**22*22*lh",
            Elements::Sha256Ctx8Init => b"1",
            Elements::Sha256Iv => b"1",
            Elements::SigAllHash => b"1",
            Elements::Some1 => b"2",
            Elements::Some16 => b"****22*22**22*22***22*22**22*22",
            Elements::Some32 => b"i",
            Elements::Some64 => b"l",
            Elements::Some8 => b"***22*22**22*22",
            Elements::Subtract16 => b"i",
            Elements::Subtract32 => b"l",
            Elements::Subtract64 => b"*ll",
            Elements::Subtract8 => b"****22*22**22*22***22*22**22*22",
            Elements::Swu => b"h",
            Elements::TapEnvHash => b"1",
            Elements::TapleafHash => b"1",
            Elements::TapleafVersion => b"1",
            Elements::Tappath => b"***22*22**22*22",
            Elements::TappathHash => b"1",
            Elements::TotalFee => b"h",
            Elements::TransactionId => b"1",
            Elements::TxHash => b"1",
            Elements::TxIsFinal => b"1",
            Elements::TxLockDistance => b"1",
            Elements::TxLockDuration => b"1",
            Elements::TxLockHeight => b"1",
            Elements::TxLockTime => b"1",
            Elements::Verify => b"2",
            Elements::Version => b"1",
            Elements::Xor1 => b"*22",
            Elements::Xor16 => b"i",
            Elements::Xor32 => b"l",
            Elements::Xor64 => b"*ll",
            Elements::Xor8 => b"****22*22**22*22***22*22**22*22",
            Elements::XorXor1 => b"*2*22",
            Elements::XorXor16 => b"*****22*22**22*22***22*22**22*22i",
            Elements::XorXor32 => b"*il",
            Elements::XorXor64 => b"*l*ll",
            Elements::XorXor8 => b"****22*22**22*22****22*22**22*22***22*22**22*22",
        };

        TypeName(name)
    }

    fn target_ty(&self) -> TypeName {
        let name: &'static [u8] = match self {
            Elements::Add16 => b"*2****22*22**22*22***22*22**22*22",
            Elements::Add32 => b"*2i",
            Elements::Add64 => b"*2l",
            Elements::Add8 => b"*2***22*22**22*22",
            Elements::All16 => b"2",
            Elements::All32 => b"2",
            Elements::All64 => b"2",
            Elements::All8 => b"2",
            Elements::And1 => b"2",
            Elements::And16 => b"****22*22**22*22***22*22**22*22",
            Elements::And32 => b"i",
            Elements::And64 => b"l",
            Elements::And8 => b"***22*22**22*22",
            Elements::AnnexHash => b"**+1h*+1*ll*+1l*+1i*+1****22*22**22*22***22*22**22*22+1***22*22**22*22*lh",
            Elements::AssetAmountHash => b"**+1h*+1*ll*+1l*+1i*+1****22*22**22*22***22*22**22*22+1***22*22**22*22*lh",
            Elements::Bip0340Verify => b"1",
            Elements::BuildTapbranch => b"h",
            Elements::BuildTapleafSimplicity => b"h",
            Elements::CalculateAsset => b"h",
            Elements::CalculateConfidentialToken => b"h",
            Elements::CalculateExplicitToken => b"h",
            Elements::CalculateIssuanceEntropy => b"h",
            Elements::Ch1 => b"2",
            Elements::Ch16 => b"****22*22**22*22***22*22**22*22",
            Elements::Ch32 => b"i",
            Elements::Ch64 => b"l",
            Elements::Ch8 => b"***22*22**22*22",
            Elements::CheckLockDistance => b"1",
            Elements::CheckLockDuration => b"1",
            Elements::CheckLockHeight => b"1",
            Elements::CheckLockTime => b"1",
            Elements::CheckSigVerify => b"1",
            Elements::Complement1 => b"2",
            Elements::Complement16 => b"****22*22**22*22***22*22**22*22",
            Elements::Complement32 => b"i",
            Elements::Complement64 => b"l",
            Elements::Complement8 => b"***22*22**22*22",
            Elements::CurrentAmount => b"*+*2hh+*2hl",
            Elements::CurrentAnnexHash => b"+1h",
            Elements::CurrentAsset => b"+*2hh",
            Elements::CurrentIndex => b"i",
            Elements::CurrentIssuanceAssetAmount => b"+1+*2hl",
            Elements::CurrentIssuanceAssetProof => b"h",
            Elements::CurrentIssuanceTokenAmount => b"+1+*2hl",
            Elements::CurrentIssuanceTokenProof => b"h",
            Elements::CurrentNewIssuanceContract => b"+1h",
            Elements::CurrentPegin => b"+1h",
            Elements::CurrentPrevOutpoint => b"*hi",
            Elements::CurrentReissuanceBlinding => b"+1h",
            Elements::CurrentReissuanceEntropy => b"+1h",
            Elements::CurrentScriptHash => b"h",
            Elements::CurrentScriptSigHash => b"h",
            Elements::CurrentSequence => b"i",
            Elements::Decompress => b"+1*hh",
            Elements::Decrement16 => b"*2****22*22**22*22***22*22**22*22",
            Elements::Decrement32 => b"*2i",
            Elements::Decrement64 => b"*2l",
            Elements::Decrement8 => b"*2***22*22**22*22",
            Elements::DivMod128_64 => b"*ll",
            Elements::DivMod16 => b"i",
            Elements::DivMod32 => b"l",
            Elements::DivMod64 => b"*ll",
            Elements::DivMod8 => b"****22*22**22*22***22*22**22*22",
            Elements::Divide16 => b"****22*22**22*22***22*22**22*22",
            Elements::Divide32 => b"i",
            Elements::Divide64 => b"l",
            Elements::Divide8 => b"***22*22**22*22",
            Elements::Divides16 => b"2",
            Elements::Divides32 => b"2",
            Elements::Divides64 => b"2",
            Elements::Divides8 => b"2",
            Elements::Eq1 => b"2",
            Elements::Eq16 => b"2",
            Elements::Eq256 => b"2",
            Elements::Eq32 => b"2",
            Elements::Eq64 => b"2",
            Elements::Eq8 => b"2",
            Elements::FeAdd => b"h",
            Elements::FeInvert => b"h",
            Elements::FeIsOdd => b"2",
            Elements::FeIsZero => b"2",
            Elements::FeMultiply => b"h",
            Elements::FeMultiplyBeta => b"h",
            Elements::FeNegate => b"h",
            Elements::FeNormalize => b"h",
            Elements::FeSquare => b"h",
            Elements::FeSquareRoot => b"+1h",
            Elements::FullAdd16 => b"*2****22*22**22*22***22*22**22*22",
            Elements::FullAdd32 => b"*2i",
            Elements::FullAdd64 => b"*2l",
            Elements::FullAdd8 => b"*2***22*22**22*22",
            Elements::FullDecrement16 => b"*2****22*22**22*22***22*22**22*22",
            Elements::FullDecrement32 => b"*2i",
            Elements::FullDecrement64 => b"*2l",
            Elements::FullDecrement8 => b"*2***22*22**22*22",
            Elements::FullIncrement16 => b"*2****22*22**22*22***22*22**22*22",
            Elements::FullIncrement32 => b"*2i",
            Elements::FullIncrement64 => b"*2l",
            Elements::FullIncrement8 => b"*2***22*22**22*22",
            Elements::FullLeftShift16_1 => b"*2****22*22**22*22***22*22**22*22",
            Elements::FullLeftShift16_2 => b"**22****22*22**22*22***22*22**22*22",
            Elements::FullLeftShift16_4 => b"***22*22****22*22**22*22***22*22**22*22",
            Elements::FullLeftShift16_8 => b"****22*22**22*22****22*22**22*22***22*22**22*22",
            Elements::FullLeftShift32_1 => b"*2i",
            Elements::FullLeftShift32_16 => b"*****22*22**22*22***22*22**22*22i",
            Elements::FullLeftShift32_2 => b"**22i",
            Elements::FullLeftShift32_4 => b"***22*22i",
            Elements::FullLeftShift32_8 => b"****22*22**22*22i",
            Elements::FullLeftShift64_1 => b"*2l",
            Elements::FullLeftShift64_16 => b"*****22*22**22*22***22*22**22*22l",
            Elements::FullLeftShift64_2 => b"**22l",
            Elements::FullLeftShift64_32 => b"*il",
            Elements::FullLeftShift64_4 => b"***22*22l",
            Elements::FullLeftShift64_8 => b"****22*22**22*22l",
            Elements::FullLeftShift8_1 => b"*2***22*22**22*22",
            Elements::FullLeftShift8_2 => b"**22***22*22**22*22",
            Elements::FullLeftShift8_4 => b"***22*22***22*22**22*22",
            Elements::FullMultiply16 => b"i",
            Elements::FullMultiply32 => b"l",
            Elements::FullMultiply64 => b"*ll",
            Elements::FullMultiply8 => b"****22*22**22*22***22*22**22*22",
            Elements::FullRightShift16_1 => b"*****22*22**22*22***22*22**22*222",
            Elements::FullRightShift16_2 => b"*****22*22**22*22***22*22**22*22*22",
            Elements::FullRightShift16_4 => b"*****22*22**22*22***22*22**22*22**22*22",
            Elements::FullRightShift16_8 => b"*****22*22**22*22***22*22**22*22***22*22**22*22",
            Elements::FullRightShift32_1 => b"*i2",
            Elements::FullRightShift32_16 => b"*i****22*22**22*22***22*22**22*22",
            Elements::FullRightShift32_2 => b"*i*22",
            Elements::FullRightShift32_4 => b"*i**22*22",
            Elements::FullRightShift32_8 => b"*i***22*22**22*22",
            Elements::FullRightShift64_1 => b"*l2",
            Elements::FullRightShift64_16 => b"*l****22*22**22*22***22*22**22*22",
            Elements::FullRightShift64_2 => b"*l*22",
            Elements::FullRightShift64_32 => b"*li",
            Elements::FullRightShift64_4 => b"*l**22*22",
            Elements::FullRightShift64_8 => b"*l***22*22**22*22",
            Elements::FullRightShift8_1 => b"****22*22**22*222",
            Elements::FullRightShift8_2 => b"****22*22**22*22*22",
            Elements::FullRightShift8_4 => b"****22*22**22*22**22*22",
            Elements::FullSubtract16 => b"*2****22*22**22*22***22*22**22*22",
            Elements::FullSubtract32 => b"*2i",
            Elements::FullSubtract64 => b"*2l",
            Elements::FullSubtract8 => b"*2***22*22**22*22",
            Elements::GeIsOnCurve => b"2",
            Elements::GeNegate => b"*hh",
            Elements::GejAdd => b"**hhh",
            Elements::GejDouble => b"**hhh",
            Elements::GejEquiv => b"2",
            Elements::GejGeAdd => b"**hhh",
            Elements::GejGeAddEx => b"*h**hhh",
            Elements::GejGeEquiv => b"2",
            Elements::GejInfinity => b"**hhh",
            Elements::GejIsInfinity => b"2",
            Elements::GejIsOnCurve => b"2",
            Elements::GejNegate => b"**hhh",
            Elements::GejNormalize => b"+1*hh",
            Elements::GejRescale => b"**hhh",
            Elements::GejXEquiv => b"2",
            Elements::GejYIsOdd => b"2",
            Elements::Generate => b"**hhh",
            Elements::GenesisBlockHash => b"h",
            Elements::HashToCurve => b"*hh",
            Elements::High1 => b"2",
            Elements::High16 => b"****22*22**22*22***22*22**22*22",
            Elements::High32 => b"i",
            Elements::High64 => b"l",
            Elements::High8 => b"***22*22**22*22",
            Elements::Increment16 => b"*2****22*22**22*22***22*22**22*22",
            Elements::Increment32 => b"*2i",
            Elements::Increment64 => b"*2l",
            Elements::Increment8 => b"*2***22*22**22*22",
            Elements::InputAmount => b"+1*+*2hh+*2hl",
            Elements::InputAmountsHash => b"h",
            Elements::InputAnnexHash => b"+1+1h",
            Elements::InputAnnexesHash => b"h",
            Elements::InputAsset => b"+1+*2hh",
            Elements::InputHash => b"+1h",
            Elements::InputOutpointsHash => b"h",
            Elements::InputPegin => b"+1+1h",
            Elements::InputPrevOutpoint => b"+1*hi",
            Elements::InputScriptHash => b"+1h",
            Elements::InputScriptSigHash => b"+1h",
            Elements::InputScriptSigsHash => b"h",
            Elements::InputScriptsHash => b"h",
            Elements::InputSequence => b"+1i",
            Elements::InputSequencesHash => b"h",
            Elements::InputUtxoHash => b"+1h",
            Elements::InputUtxosHash => b"h",
            Elements::InputsHash => b"h",
            Elements::InternalKey => b"h",
            Elements::IsOne16 => b"2",
            Elements::IsOne32 => b"2",
            Elements::IsOne64 => b"2",
            Elements::IsOne8 => b"2",
            Elements::IsZero16 => b"2",
            Elements::IsZero32 => b"2",
            Elements::IsZero64 => b"2",
            Elements::IsZero8 => b"2",
            Elements::Issuance => b"+1+12",
            Elements::IssuanceAsset => b"+1+1h",
            Elements::IssuanceAssetAmount => b"+1+1+*2hl",
            Elements::IssuanceAssetAmountsHash => b"h",
            Elements::IssuanceAssetProof => b"+1h",
            Elements::IssuanceBlindingEntropyHash => b"h",
            Elements::IssuanceEntropy => b"+1+1h",
            Elements::IssuanceHash => b"+1h",
            Elements::IssuanceRangeProofsHash => b"h",
            Elements::IssuanceToken => b"+1+1h",
            Elements::IssuanceTokenAmount => b"+1+1+*2hl",
            Elements::IssuanceTokenAmountsHash => b"h",
            Elements::IssuanceTokenProof => b"+1h",
            Elements::IssuancesHash => b"h",
            Elements::LbtcAsset => b"h",
            Elements::Le16 => b"2",
            Elements::Le32 => b"2",
            Elements::Le64 => b"2",
            Elements::Le8 => b"2",
            Elements::LeftExtend16_32 => b"i",
            Elements::LeftExtend16_64 => b"l",
            Elements::LeftExtend1_16 => b"****22*22**22*22***22*22**22*22",
            Elements::LeftExtend1_32 => b"i",
            Elements::LeftExtend1_64 => b"l",
            Elements::LeftExtend1_8 => b"***22*22**22*22",
            Elements::LeftExtend32_64 => b"l",
            Elements::LeftExtend8_16 => b"****22*22**22*22***22*22**22*22",
            Elements::LeftExtend8_32 => b"i",
            Elements::LeftExtend8_64 => b"l",
            Elements::LeftPadHigh16_32 => b"i",
            Elements::LeftPadHigh16_64 => b"l",
            Elements::LeftPadHigh1_16 => b"****22*22**22*22***22*22**22*22",
            Elements::LeftPadHigh1_32 => b"i",
            Elements::LeftPadHigh1_64 => b"l",
            Elements::LeftPadHigh1_8 => b"***22*22**22*22",
            Elements::LeftPadHigh32_64 => b"l",
            Elements::LeftPadHigh8_16 => b"****22*22**22*22***22*22**22*22",
            Elements::LeftPadHigh8_32 => b"i",
            Elements::LeftPadHigh8_64 => b"l",
            Elements::LeftPadLow16_32 => b"i",
            Elements::LeftPadLow16_64 => b"l",
            Elements::LeftPadLow1_16 => b"****22*22**22*22***22*22**22*22",
            Elements::LeftPadLow1_32 => b"i",
            Elements::LeftPadLow1_64 => b"l",
            Elements::LeftPadLow1_8 => b"***22*22**22*22",
            Elements::LeftPadLow32_64 => b"l",
            Elements::LeftPadLow8_16 => b"****22*22**22*22***22*22**22*22",
            Elements::LeftPadLow8_32 => b"i",
            Elements::LeftPadLow8_64 => b"l",
            Elements::LeftRotate16 => b"****22*22**22*22***22*22**22*22",
            Elements::LeftRotate32 => b"i",
            Elements::LeftRotate64 => b"l",
            Elements::LeftRotate8 => b"***22*22**22*22",
            Elements::LeftShift16 => b"****22*22**22*22***22*22**22*22",
            Elements::LeftShift32 => b"i",
            Elements::LeftShift64 => b"l",
            Elements::LeftShift8 => b"***22*22**22*22",
            Elements::LeftShiftWith16 => b"****22*22**22*22***22*22**22*22",
            Elements::LeftShiftWith32 => b"i",
            Elements::LeftShiftWith64 => b"l",
            Elements::LeftShiftWith8 => b"***22*22**22*22",
            Elements::Leftmost16_1 => b"2",
            Elements::Leftmost16_2 => b"*22",
            Elements::Leftmost16_4 => b"**22*22",
            Elements::Leftmost16_8 => b"***22*22**22*22",
            Elements::Leftmost32_1 => b"2",
            Elements::Leftmost32_16 => b"****22*22**22*22***22*22**22*22",
            Elements::Leftmost32_2 => b"*22",
            Elements::Leftmost32_4 => b"**22*22",
            Elements::Leftmost32_8 => b"***22*22**22*22",
            Elements::Leftmost64_1 => b"2",
            Elements::Leftmost64_16 => b"****22*22**22*22***22*22**22*22",
            Elements::Leftmost64_2 => b"*22",
            Elements::Leftmost64_32 => b"i",
            Elements::Leftmost64_4 => b"**22*22",
            Elements::Leftmost64_8 => b"***22*22**22*22",
            Elements::Leftmost8_1 => b"2",
            Elements::Leftmost8_2 => b"*22",
            Elements::Leftmost8_4 => b"**22*22",
            Elements::LinearCombination1 => b"**hhh",
            Elements::LinearVerify1 => b"1",
            Elements::LockTime => b"i",
            Elements::Low1 => b"2",
            Elements::Low16 => b"****22*22**22*22***22*22**22*22",
            Elements::Low32 => b"i",
            Elements::Low64 => b"l",
            Elements::Low8 => b"***22*22**22*22",
            Elements::Lt16 => b"2",
            Elements::Lt32 => b"2",
            Elements::Lt64 => b"2",
            Elements::Lt8 => b"2",
            Elements::Maj1 => b"2",
            Elements::Maj16 => b"****22*22**22*22***22*22**22*22",
            Elements::Maj32 => b"i",
            Elements::Maj64 => b"l",
            Elements::Maj8 => b"***22*22**22*22",
            Elements::Max16 => b"****22*22**22*22***22*22**22*22",
            Elements::Max32 => b"i",
            Elements::Max64 => b"l",
            Elements::Max8 => b"***22*22**22*22",
            Elements::Median16 => b"****22*22**22*22***22*22**22*22",
            Elements::Median32 => b"i",
            Elements::Median64 => b"l",
            Elements::Median8 => b"***22*22**22*22",
            Elements::Min16 => b"****22*22**22*22***22*22**22*22",
            Elements::Min32 => b"i",
            Elements::Min64 => b"l",
            Elements::Min8 => b"***22*22**22*22",
            Elements::Modulo16 => b"****22*22**22*22***22*22**22*22",
            Elements::Modulo32 => b"i",
            Elements::Modulo64 => b"l",
            Elements::Modulo8 => b"***22*22**22*22",
            Elements::Multiply16 => b"i",
            Elements::Multiply32 => b"l",
            Elements::Multiply64 => b"*ll",
            Elements::Multiply8 => b"****22*22**22*22***22*22**22*22",
            Elements::Negate16 => b"*2****22*22**22*22***22*22**22*22",
            Elements::Negate32 => b"*2i",
            Elements::Negate64 => b"*2l",
            Elements::Negate8 => b"*2***22*22**22*22",
            Elements::NewIssuanceContract => b"+1+1h",
            Elements::NonceHash => b"**+1h*+1*ll*+1l*+1i*+1****22*22**22*22***22*22**22*22+1***22*22**22*22*lh",
            Elements::NumInputs => b"i",
            Elements::NumOutputs => b"i",
            Elements::One16 => b"****22*22**22*22***22*22**22*22",
            Elements::One32 => b"i",
            Elements::One64 => b"l",
            Elements::One8 => b"***22*22**22*22",
            Elements::Or1 => b"2",
            Elements::Or16 => b"****22*22**22*22***22*22**22*22",
            Elements::Or32 => b"i",
            Elements::Or64 => b"l",
            Elements::Or8 => b"***22*22**22*22",
            Elements::OutpointHash => b"**+1h*+1*ll*+1l*+1i*+1****22*22**22*22***22*22**22*22+1***22*22**22*22*lh",
            Elements::OutputAmount => b"+1*+*2hh+*2hl",
            Elements::OutputAmountsHash => b"h",
            Elements::OutputAsset => b"+1+*2hh",
            Elements::OutputHash => b"+1h",
            Elements::OutputIsFee => b"+12",
            Elements::OutputNonce => b"+1+1+*2hh",
            Elements::OutputNoncesHash => b"h",
            Elements::OutputNullDatum => b"+1+1+**22h+2**22*22",
            Elements::OutputRangeProof => b"+1h",
            Elements::OutputRangeProofsHash => b"h",
            Elements::OutputScriptHash => b"+1h",
            Elements::OutputScriptsHash => b"h",
            Elements::OutputSurjectionProof => b"+1h",
            Elements::OutputSurjectionProofsHash => b"h",
            Elements::OutputsHash => b"h",
            Elements::ParseLock => b"+ii",
            Elements::ParseSequence => b"+1+****22*22**22*22***22*22**22*22****22*22**22*22***22*22**22*22",
            Elements::PointVerify1 => b"1",
            Elements::ReissuanceBlinding => b"+1+1h",
            Elements::ReissuanceEntropy => b"+1+1h",
            Elements::RightExtend16_32 => b"i",
            Elements::RightExtend16_64 => b"l",
            Elements::RightExtend32_64 => b"l",
            Elements::RightExtend8_16 => b"****22*22**22*22***22*22**22*22",
            Elements::RightExtend8_32 => b"i",
            Elements::RightExtend8_64 => b"l",
            Elements::RightPadHigh16_32 => b"i",
            Elements::RightPadHigh16_64 => b"l",
            Elements::RightPadHigh1_16 => b"****22*22**22*22***22*22**22*22",
            Elements::RightPadHigh1_32 => b"i",
            Elements::RightPadHigh1_64 => b"l",
            Elements::RightPadHigh1_8 => b"***22*22**22*22",
            Elements::RightPadHigh32_64 => b"l",
            Elements::RightPadHigh8_16 => b"****22*22**22*22***22*22**22*22",
            Elements::RightPadHigh8_32 => b"i",
            Elements::RightPadHigh8_64 => b"l",
            Elements::RightPadLow16_32 => b"i",
            Elements::RightPadLow16_64 => b"l",
            Elements::RightPadLow1_16 => b"****22*22**22*22***22*22**22*22",
            Elements::RightPadLow1_32 => b"i",
            Elements::RightPadLow1_64 => b"l",
            Elements::RightPadLow1_8 => b"***22*22**22*22",
            Elements::RightPadLow32_64 => b"l",
            Elements::RightPadLow8_16 => b"****22*22**22*22***22*22**22*22",
            Elements::RightPadLow8_32 => b"i",
            Elements::RightPadLow8_64 => b"l",
            Elements::RightRotate16 => b"****22*22**22*22***22*22**22*22",
            Elements::RightRotate32 => b"i",
            Elements::RightRotate64 => b"l",
            Elements::RightRotate8 => b"***22*22**22*22",
            Elements::RightShift16 => b"****22*22**22*22***22*22**22*22",
            Elements::RightShift32 => b"i",
            Elements::RightShift64 => b"l",
            Elements::RightShift8 => b"***22*22**22*22",
            Elements::RightShiftWith16 => b"****22*22**22*22***22*22**22*22",
            Elements::RightShiftWith32 => b"i",
            Elements::RightShiftWith64 => b"l",
            Elements::RightShiftWith8 => b"***22*22**22*22",
            Elements::Rightmost16_1 => b"2",
            Elements::Rightmost16_2 => b"*22",
            Elements::Rightmost16_4 => b"**22*22",
            Elements::Rightmost16_8 => b"***22*22**22*22",
            Elements::Rightmost32_1 => b"2",
            Elements::Rightmost32_16 => b"****22*22**22*22***22*22**22*22",
            Elements::Rightmost32_2 => b"*22",
            Elements::Rightmost32_4 => b"**22*22",
            Elements::Rightmost32_8 => b"***22*22**22*22",
            Elements::Rightmost64_1 => b"2",
            Elements::Rightmost64_16 => b"****22*22**22*22***22*22**22*22",
            Elements::Rightmost64_2 => b"*22",
            Elements::Rightmost64_32 => b"i",
            Elements::Rightmost64_4 => b"**22*22",
            Elements::Rightmost64_8 => b"***22*22**22*22",
            Elements::Rightmost8_1 => b"2",
            Elements::Rightmost8_2 => b"*22",
            Elements::Rightmost8_4 => b"**22*22",
            Elements::ScalarAdd => b"h",
            Elements::ScalarInvert => b"h",
            Elements::ScalarIsZero => b"2",
            Elements::ScalarMultiply => b"h",
            Elements::ScalarMultiplyLambda => b"h",
            Elements::ScalarNegate => b"h",
            Elements::ScalarNormalize => b"h",
            Elements::ScalarSquare => b"h",
            Elements::Scale => b"**hhh",
            Elements::ScriptCMR => b"h",
            Elements::Sha256Block => b"h",
            Elements::Sha256Ctx8Add1 => b"**+1h*+1*ll*+1l*+1i*+1****22*22**22*22***22*22**22*22+1***22*22**22*22*lh",
            Elements::Sha256Ctx8Add128 => b"**+1h*+1*ll*+1l*+1i*+1****22*22**22*22***22*22**22*22+1***22*22**22*22*lh",
            Elements::Sha256Ctx8Add16 => b"**+1h*+1*ll*+1l*+1i*+1****22*22**22*22***22*22**22*22+1***22*22**22*22*lh",
            Elements::Sha256Ctx8Add2 => b"**+1h*+1*ll*+1l*+1i*+1****22*22**22*22***22*22**22*22+1***22*22**22*22*lh",
            Elements::Sha256Ctx8Add256 => b"**+1h*+1*ll*+1l*+1i*+1****22*22**22*22***22*22**22*22+1***22*22**22*22*lh",
            Elements::Sha256Ctx8Add32 => b"**+1h*+1*ll*+1l*+1i*+1****22*22**22*22***22*22**22*22+1***22*22**22*22*lh",
            Elements::Sha256Ctx8Add4 => b"**+1h*+1*ll*+1l*+1i*+1****22*22**22*22***22*22**22*22+1***22*22**22*22*lh",
            Elements::Sha256Ctx8Add512 => b"**+1h*+1*ll*+1l*+1i*+1****22*22**22*22***22*22**22*22+1***22*22**22*22*lh",
            Elements::Sha256Ctx8Add64 => b"**+1h*+1*ll*+1l*+1i*+1****22*22**22*22***22*22**22*22+1***22*22**22*22*lh",
            Elements::Sha256Ctx8Add8 => b"**+1h*+1*ll*+1l*+1i*+1****22*22**22*22***22*22**22*22+1***22*22**22*22*lh",
            Elements::Sha256Ctx8AddBuffer511 => b"**+1h*+1*ll*+1l*+1i*+1****22*22**22*22***22*22**22*22+1***22*22**22*22*lh",
            Elements::Sha256Ctx8Finalize => b"h",
            Elements::Sha256Ctx8Init => b"**+1h*+1*ll*+1l*+1i*+1****22*22**22*22***22*22**22*22+1***22*22**22*22*lh",
            Elements::Sha256Iv => b"h",
            Elements::SigAllHash => b"h",
            Elements::Some1 => b"2",
            Elements::Some16 => b"2",
            Elements::Some32 => b"2",
            Elements::Some64 => b"2",
            Elements::Some8 => b"2",
            Elements::Subtract16 => b"*2****22*22**22*22***22*22**22*22",
            Elements::Subtract32 => b"*2i",
            Elements::Subtract64 => b"*2l",
            Elements::Subtract8 => b"*2***22*22**22*22",
            Elements::Swu => b"*hh",
            Elements::TapEnvHash => b"h",
            Elements::TapleafHash => b"h",
            Elements::TapleafVersion => b"***22*22**22*22",
            Elements::Tappath => b"+1h",
            Elements::TappathHash => b"h",
            Elements::TotalFee => b"l",
            Elements::TransactionId => b"h",
            Elements::TxHash => b"h",
            Elements::TxIsFinal => b"2",
            Elements::TxLockDistance => b"****22*22**22*22***22*22**22*22",
            Elements::TxLockDuration => b"****22*22**22*22***22*22**22*22",
            Elements::TxLockHeight => b"i",
            Elements::TxLockTime => b"i",
            Elements::Verify => b"1",
            Elements::Version => b"i",
            Elements::Xor1 => b"2",
            Elements::Xor16 => b"****22*22**22*22***22*22**22*22",
            Elements::Xor32 => b"i",
            Elements::Xor64 => b"l",
            Elements::Xor8 => b"***22*22**22*22",
            Elements::XorXor1 => b"2",
            Elements::XorXor16 => b"****22*22**22*22***22*22**22*22",
            Elements::XorXor32 => b"i",
            Elements::XorXor64 => b"l",
            Elements::XorXor8 => b"***22*22**22*22",
        };

        TypeName(name)
    }

    fn encode<W: Write>(&self, w: &mut BitWriter<W>) -> std::io::Result<usize> {
        let (n, len) = match self {
            Elements::Verify => (0, 3),
            Elements::Low1 => (8, 6),
            Elements::Low8 => (37, 8),
            Elements::Low16 => (304, 11),
            Elements::Low32 => (305, 11),
            Elements::Low64 => (306, 11),
            Elements::High1 => (10, 6),
            Elements::High8 => (45, 8),
            Elements::High16 => (368, 11),
            Elements::High32 => (369, 11),
            Elements::High64 => (370, 11),
            Elements::Complement1 => (96, 9),
            Elements::Complement8 => (389, 11),
            Elements::Complement16 => (3120, 14),
            Elements::Complement32 => (3121, 14),
            Elements::Complement64 => (3122, 14),
            Elements::And1 => (98, 9),
            Elements::And8 => (397, 11),
            Elements::And16 => (3184, 14),
            Elements::And32 => (3185, 14),
            Elements::And64 => (3186, 14),
            Elements::Or1 => (100, 9),
            Elements::Or8 => (405, 11),
            Elements::Or16 => (3248, 14),
            Elements::Or32 => (3249, 14),
            Elements::Or64 => (3250, 14),
            Elements::Xor1 => (102, 9),
            Elements::Xor8 => (413, 11),
            Elements::Xor16 => (3312, 14),
            Elements::Xor32 => (3313, 14),
            Elements::Xor64 => (3314, 14),
            Elements::Maj1 => (208, 10),
            Elements::Maj8 => (837, 12),
            Elements::Maj16 => (6704, 15),
            Elements::Maj32 => (6705, 15),
            Elements::Maj64 => (6706, 15),
            Elements::XorXor1 => (210, 10),
            Elements::XorXor8 => (845, 12),
            Elements::XorXor16 => (6768, 15),
            Elements::XorXor32 => (6769, 15),
            Elements::XorXor64 => (6770, 15),
            Elements::Ch1 => (212, 10),
            Elements::Ch8 => (853, 12),
            Elements::Ch16 => (6832, 15),
            Elements::Ch32 => (6833, 15),
            Elements::Ch64 => (6834, 15),
            Elements::Some1 => (214, 10),
            Elements::Some8 => (861, 12),
            Elements::Some16 => (6896, 15),
            Elements::Some32 => (6897, 15),
            Elements::Some64 => (6898, 15),
            Elements::All8 => (869, 12),
            Elements::All16 => (6960, 15),
            Elements::All32 => (6961, 15),
            Elements::All64 => (6962, 15),
            Elements::Eq1 => (218, 10),
            Elements::Eq8 => (877, 12),
            Elements::Eq16 => (7024, 15),
            Elements::Eq32 => (7025, 15),
            Elements::Eq64 => (7026, 15),
            Elements::Eq256 => (14056, 16),
            Elements::FullLeftShift8_1 => (1765, 13),
            Elements::FullLeftShift16_1 => (14128, 16),
            Elements::FullLeftShift32_1 => (14129, 16),
            Elements::FullLeftShift64_1 => (14130, 16),
            Elements::FullLeftShift8_2 => (7076, 15),
            Elements::FullLeftShift16_2 => (7077, 15),
            Elements::FullLeftShift32_2 => (56624, 18),
            Elements::FullLeftShift64_2 => (56625, 18),
            Elements::FullLeftShift8_4 => (1770, 13),
            Elements::FullLeftShift16_4 => (7084, 15),
            Elements::FullLeftShift32_4 => (7085, 15),
            Elements::FullLeftShift64_4 => (56688, 18),
            Elements::FullLeftShift16_8 => (14176, 16),
            Elements::FullLeftShift32_8 => (56708, 18),
            Elements::FullLeftShift64_8 => (56709, 18),
            Elements::FullLeftShift32_16 => (14178, 16),
            Elements::FullLeftShift64_16 => (56716, 18),
            Elements::FullLeftShift64_32 => (14180, 16),
            Elements::FullRightShift8_1 => (1781, 13),
            Elements::FullRightShift16_1 => (14256, 16),
            Elements::FullRightShift32_1 => (14257, 16),
            Elements::FullRightShift64_1 => (14258, 16),
            Elements::FullRightShift8_2 => (7140, 15),
            Elements::FullRightShift16_2 => (7141, 15),
            Elements::FullRightShift32_2 => (57136, 18),
            Elements::FullRightShift64_2 => (57137, 18),
            Elements::FullRightShift8_4 => (1786, 13),
            Elements::FullRightShift16_4 => (7148, 15),
            Elements::FullRightShift32_4 => (7149, 15),
            Elements::FullRightShift64_4 => (57200, 18),
            Elements::FullRightShift16_8 => (14304, 16),
            Elements::FullRightShift32_8 => (57220, 18),
            Elements::FullRightShift64_8 => (57221, 18),
            Elements::FullRightShift32_16 => (14306, 16),
            Elements::FullRightShift64_16 => (57228, 18),
            Elements::FullRightShift64_32 => (14308, 16),
            Elements::Leftmost8_1 => (28677, 17),
            Elements::Leftmost16_1 => (229424, 20),
            Elements::Leftmost32_1 => (229425, 20),
            Elements::Leftmost64_1 => (229426, 20),
            Elements::Leftmost8_2 => (114724, 19),
            Elements::Leftmost16_2 => (114725, 19),
            Elements::Leftmost32_2 => (917808, 22),
            Elements::Leftmost64_2 => (917809, 22),
            Elements::Leftmost8_4 => (28682, 17),
            Elements::Leftmost16_4 => (114732, 19),
            Elements::Leftmost32_4 => (114733, 19),
            Elements::Leftmost64_4 => (917872, 22),
            Elements::Leftmost16_8 => (229472, 20),
            Elements::Leftmost32_8 => (917892, 22),
            Elements::Leftmost64_8 => (917893, 22),
            Elements::Leftmost32_16 => (229474, 20),
            Elements::Leftmost64_16 => (917900, 22),
            Elements::Leftmost64_32 => (229476, 20),
            Elements::Rightmost8_1 => (28693, 17),
            Elements::Rightmost16_1 => (229552, 20),
            Elements::Rightmost32_1 => (229553, 20),
            Elements::Rightmost64_1 => (229554, 20),
            Elements::Rightmost8_2 => (114788, 19),
            Elements::Rightmost16_2 => (114789, 19),
            Elements::Rightmost32_2 => (918320, 22),
            Elements::Rightmost64_2 => (918321, 22),
            Elements::Rightmost8_4 => (28698, 17),
            Elements::Rightmost16_4 => (114796, 19),
            Elements::Rightmost32_4 => (114797, 19),
            Elements::Rightmost64_4 => (918384, 22),
            Elements::Rightmost16_8 => (229600, 20),
            Elements::Rightmost32_8 => (918404, 22),
            Elements::Rightmost64_8 => (918405, 22),
            Elements::Rightmost32_16 => (229602, 20),
            Elements::Rightmost64_16 => (918412, 22),
            Elements::Rightmost64_32 => (229604, 20),
            Elements::LeftPadLow1_8 => (28709, 17),
            Elements::LeftPadLow1_16 => (229680, 20),
            Elements::LeftPadLow1_32 => (229681, 20),
            Elements::LeftPadLow1_64 => (229682, 20),
            Elements::LeftPadLow8_16 => (229728, 20),
            Elements::LeftPadLow8_32 => (918916, 22),
            Elements::LeftPadLow8_64 => (918917, 22),
            Elements::LeftPadLow16_32 => (229730, 20),
            Elements::LeftPadLow16_64 => (918924, 22),
            Elements::LeftPadLow32_64 => (229732, 20),
            Elements::LeftPadHigh1_8 => (28725, 17),
            Elements::LeftPadHigh1_16 => (229808, 20),
            Elements::LeftPadHigh1_32 => (229809, 20),
            Elements::LeftPadHigh1_64 => (229810, 20),
            Elements::LeftPadHigh8_16 => (229856, 20),
            Elements::LeftPadHigh8_32 => (919428, 22),
            Elements::LeftPadHigh8_64 => (919429, 22),
            Elements::LeftPadHigh16_32 => (229858, 20),
            Elements::LeftPadHigh16_64 => (919436, 22),
            Elements::LeftPadHigh32_64 => (229860, 20),
            Elements::LeftExtend1_8 => (28741, 17),
            Elements::LeftExtend1_16 => (229936, 20),
            Elements::LeftExtend1_32 => (229937, 20),
            Elements::LeftExtend1_64 => (229938, 20),
            Elements::LeftExtend8_16 => (229984, 20),
            Elements::LeftExtend8_32 => (919940, 22),
            Elements::LeftExtend8_64 => (919941, 22),
            Elements::LeftExtend16_32 => (229986, 20),
            Elements::LeftExtend16_64 => (919948, 22),
            Elements::LeftExtend32_64 => (229988, 20),
            Elements::RightPadLow1_8 => (28757, 17),
            Elements::RightPadLow1_16 => (230064, 20),
            Elements::RightPadLow1_32 => (230065, 20),
            Elements::RightPadLow1_64 => (230066, 20),
            Elements::RightPadLow8_16 => (230112, 20),
            Elements::RightPadLow8_32 => (920452, 22),
            Elements::RightPadLow8_64 => (920453, 22),
            Elements::RightPadLow16_32 => (230114, 20),
            Elements::RightPadLow16_64 => (920460, 22),
            Elements::RightPadLow32_64 => (230116, 20),
            Elements::RightPadHigh1_8 => (28773, 17),
            Elements::RightPadHigh1_16 => (230192, 20),
            Elements::RightPadHigh1_32 => (230193, 20),
            Elements::RightPadHigh1_64 => (230194, 20),
            Elements::RightPadHigh8_16 => (230240, 20),
            Elements::RightPadHigh8_32 => (920964, 22),
            Elements::RightPadHigh8_64 => (920965, 22),
            Elements::RightPadHigh16_32 => (230242, 20),
            Elements::RightPadHigh16_64 => (920972, 22),
            Elements::RightPadHigh32_64 => (230244, 20),
            Elements::RightExtend8_16 => (230368, 20),
            Elements::RightExtend8_32 => (921476, 22),
            Elements::RightExtend8_64 => (921477, 22),
            Elements::RightExtend16_32 => (230370, 20),
            Elements::RightExtend16_64 => (921484, 22),
            Elements::RightExtend32_64 => (230372, 20),
            Elements::LeftShiftWith8 => (14405, 16),
            Elements::LeftShiftWith16 => (115248, 19),
            Elements::LeftShiftWith32 => (115249, 19),
            Elements::LeftShiftWith64 => (115250, 19),
            Elements::RightShiftWith8 => (14413, 16),
            Elements::RightShiftWith16 => (115312, 19),
            Elements::RightShiftWith32 => (115313, 19),
            Elements::RightShiftWith64 => (115314, 19),
            Elements::LeftShift8 => (14421, 16),
            Elements::LeftShift16 => (115376, 19),
            Elements::LeftShift32 => (115377, 19),
            Elements::LeftShift64 => (115378, 19),
            Elements::RightShift8 => (14429, 16),
            Elements::RightShift16 => (115440, 19),
            Elements::RightShift32 => (115441, 19),
            Elements::RightShift64 => (115442, 19),
            Elements::LeftRotate8 => (14437, 16),
            Elements::LeftRotate16 => (115504, 19),
            Elements::LeftRotate32 => (115505, 19),
            Elements::LeftRotate64 => (115506, 19),
            Elements::RightRotate8 => (14445, 16),
            Elements::RightRotate16 => (115568, 19),
            Elements::RightRotate32 => (115569, 19),
            Elements::RightRotate64 => (115570, 19),
            Elements::One8 => (69, 8),
            Elements::One16 => (560, 11),
            Elements::One32 => (561, 11),
            Elements::One64 => (562, 11),
            Elements::FullAdd8 => (293, 10),
            Elements::FullAdd16 => (2352, 13),
            Elements::FullAdd32 => (2353, 13),
            Elements::FullAdd64 => (2354, 13),
            Elements::Add8 => (301, 10),
            Elements::Add16 => (2416, 13),
            Elements::Add32 => (2417, 13),
            Elements::Add64 => (2418, 13),
            Elements::FullIncrement8 => (2437, 13),
            Elements::FullIncrement16 => (19504, 16),
            Elements::FullIncrement32 => (19505, 16),
            Elements::FullIncrement64 => (19506, 16),
            Elements::Increment8 => (2445, 13),
            Elements::Increment16 => (19568, 16),
            Elements::Increment32 => (19569, 16),
            Elements::Increment64 => (19570, 16),
            Elements::FullSubtract8 => (2461, 13),
            Elements::FullSubtract16 => (19696, 16),
            Elements::FullSubtract32 => (19697, 16),
            Elements::FullSubtract64 => (19698, 16),
            Elements::Subtract8 => (4933, 14),
            Elements::Subtract16 => (39472, 17),
            Elements::Subtract32 => (39473, 17),
            Elements::Subtract64 => (39474, 17),
            Elements::Negate8 => (4941, 14),
            Elements::Negate16 => (39536, 17),
            Elements::Negate32 => (39537, 17),
            Elements::Negate64 => (39538, 17),
            Elements::FullDecrement8 => (4949, 14),
            Elements::FullDecrement16 => (39600, 17),
            Elements::FullDecrement32 => (39601, 17),
            Elements::FullDecrement64 => (39602, 17),
            Elements::Decrement8 => (4957, 14),
            Elements::Decrement16 => (39664, 17),
            Elements::Decrement32 => (39665, 17),
            Elements::Decrement64 => (39666, 17),
            Elements::FullMultiply8 => (4965, 14),
            Elements::FullMultiply16 => (39728, 17),
            Elements::FullMultiply32 => (39729, 17),
            Elements::FullMultiply64 => (39730, 17),
            Elements::Multiply8 => (4973, 14),
            Elements::Multiply16 => (39792, 17),
            Elements::Multiply32 => (39793, 17),
            Elements::Multiply64 => (39794, 17),
            Elements::IsZero8 => (4981, 14),
            Elements::IsZero16 => (39856, 17),
            Elements::IsZero32 => (39857, 17),
            Elements::IsZero64 => (39858, 17),
            Elements::IsOne8 => (4989, 14),
            Elements::IsOne16 => (39920, 17),
            Elements::IsOne32 => (39921, 17),
            Elements::IsOne64 => (39922, 17),
            Elements::Le8 => (79877, 18),
            Elements::Le16 => (639024, 21),
            Elements::Le32 => (639025, 21),
            Elements::Le64 => (639026, 21),
            Elements::Lt8 => (79885, 18),
            Elements::Lt16 => (639088, 21),
            Elements::Lt32 => (639089, 21),
            Elements::Lt64 => (639090, 21),
            Elements::Min8 => (79893, 18),
            Elements::Min16 => (639152, 21),
            Elements::Min32 => (639153, 21),
            Elements::Min64 => (639154, 21),
            Elements::Max8 => (79901, 18),
            Elements::Max16 => (639216, 21),
            Elements::Max32 => (639217, 21),
            Elements::Max64 => (639218, 21),
            Elements::Median8 => (79909, 18),
            Elements::Median16 => (639280, 21),
            Elements::Median32 => (639281, 21),
            Elements::Median64 => (639282, 21),
            Elements::DivMod128_64 => (639346, 21),
            Elements::DivMod8 => (79925, 18),
            Elements::DivMod16 => (639408, 21),
            Elements::DivMod32 => (639409, 21),
            Elements::DivMod64 => (639410, 21),
            Elements::Divide8 => (79933, 18),
            Elements::Divide16 => (639472, 21),
            Elements::Divide32 => (639473, 21),
            Elements::Divide64 => (639474, 21),
            Elements::Modulo8 => (79941, 18),
            Elements::Modulo16 => (639536, 21),
            Elements::Modulo32 => (639537, 21),
            Elements::Modulo64 => (639538, 21),
            Elements::Divides8 => (79949, 18),
            Elements::Divides16 => (639600, 21),
            Elements::Divides32 => (639601, 21),
            Elements::Divides64 => (639602, 21),
            Elements::Sha256Block => (20, 6),
            Elements::Sha256Iv => (84, 8),
            Elements::Sha256Ctx8Add1 => (170, 9),
            Elements::Sha256Ctx8Add2 => (684, 11),
            Elements::Sha256Ctx8Add4 => (685, 11),
            Elements::Sha256Ctx8Add8 => (5488, 14),
            Elements::Sha256Ctx8Add16 => (5489, 14),
            Elements::Sha256Ctx8Add32 => (5490, 14),
            Elements::Sha256Ctx8Add64 => (5491, 14),
            Elements::Sha256Ctx8Add128 => (10984, 15),
            Elements::Sha256Ctx8Add256 => (10985, 15),
            Elements::Sha256Ctx8Add512 => (10986, 15),
            Elements::Sha256Ctx8AddBuffer511 => (688, 11),
            Elements::Sha256Ctx8Finalize => (689, 11),
            Elements::Sha256Ctx8Init => (690, 11),
            Elements::PointVerify1 => (192, 9),
            Elements::Decompress => (388, 10),
            Elements::LinearVerify1 => (778, 11),
            Elements::LinearCombination1 => (6240, 14),
            Elements::Scale => (3121, 13),
            Elements::Generate => (3122, 13),
            Elements::GejInfinity => (3123, 13),
            Elements::GejNormalize => (6248, 14),
            Elements::GejNegate => (6249, 14),
            Elements::GeNegate => (6250, 14),
            Elements::GejDouble => (6251, 14),
            Elements::GejAdd => (6252, 14),
            Elements::GejGeAddEx => (6253, 14),
            Elements::GejGeAdd => (6254, 14),
            Elements::GejRescale => (6255, 14),
            Elements::GejIsInfinity => (100096, 18),
            Elements::GejEquiv => (100097, 18),
            Elements::GejGeEquiv => (100098, 18),
            Elements::GejXEquiv => (100099, 18),
            Elements::GejYIsOdd => (100100, 18),
            Elements::GejIsOnCurve => (100101, 18),
            Elements::GeIsOnCurve => (100102, 18),
            Elements::ScalarNormalize => (100103, 18),
            Elements::ScalarNegate => (100104, 18),
            Elements::ScalarAdd => (100105, 18),
            Elements::ScalarSquare => (100106, 18),
            Elements::ScalarMultiply => (100107, 18),
            Elements::ScalarMultiplyLambda => (100108, 18),
            Elements::ScalarInvert => (100109, 18),
            Elements::ScalarIsZero => (100110, 18),
            Elements::FeNormalize => (200227, 19),
            Elements::FeNegate => (200228, 19),
            Elements::FeAdd => (200229, 19),
            Elements::FeSquare => (200230, 19),
            Elements::FeMultiply => (200231, 19),
            Elements::FeMultiplyBeta => (200232, 19),
            Elements::FeInvert => (200233, 19),
            Elements::FeSquareRoot => (200234, 19),
            Elements::FeIsZero => (200235, 19),
            Elements::FeIsOdd => (200236, 19),
            Elements::HashToCurve => (200238, 19),
            Elements::Swu => (200239, 19),
            Elements::CheckSigVerify => (98, 8),
            Elements::Bip0340Verify => (396, 10),
            Elements::ParseLock => (102, 8),
            Elements::ParseSequence => (412, 10),
            Elements::SigAllHash => (4, 3),
            Elements::TxHash => (20, 5),
            Elements::TapEnvHash => (21, 5),
            Elements::OutputsHash => (176, 8),
            Elements::InputsHash => (177, 8),
            Elements::IssuancesHash => (178, 8),
            Elements::InputUtxosHash => (179, 8),
            Elements::OutputHash => (360, 9),
            Elements::OutputAmountsHash => (361, 9),
            Elements::OutputScriptsHash => (362, 9),
            Elements::OutputNoncesHash => (363, 9),
            Elements::OutputRangeProofsHash => (364, 9),
            Elements::OutputSurjectionProofsHash => (365, 9),
            Elements::InputHash => (366, 9),
            Elements::InputOutpointsHash => (367, 9),
            Elements::InputSequencesHash => (5888, 13),
            Elements::InputAnnexesHash => (5889, 13),
            Elements::InputScriptSigsHash => (5890, 13),
            Elements::IssuanceHash => (5891, 13),
            Elements::IssuanceAssetAmountsHash => (5892, 13),
            Elements::IssuanceTokenAmountsHash => (5893, 13),
            Elements::IssuanceRangeProofsHash => (5894, 13),
            Elements::IssuanceBlindingEntropyHash => (5895, 13),
            Elements::InputUtxoHash => (5896, 13),
            Elements::InputAmountsHash => (5897, 13),
            Elements::InputScriptsHash => (5898, 13),
            Elements::TapleafHash => (5899, 13),
            Elements::TappathHash => (5900, 13),
            Elements::OutpointHash => (5901, 13),
            Elements::AssetAmountHash => (5902, 13),
            Elements::NonceHash => (5903, 13),
            Elements::AnnexHash => (11808, 14),
            Elements::BuildTapleafSimplicity => (11809, 14),
            Elements::BuildTapbranch => (11810, 14),
            Elements::CheckLockHeight => (24, 5),
            Elements::CheckLockTime => (100, 7),
            Elements::CheckLockDistance => (101, 7),
            Elements::CheckLockDuration => (816, 10),
            Elements::TxLockHeight => (817, 10),
            Elements::TxLockTime => (818, 10),
            Elements::TxLockDistance => (819, 10),
            Elements::TxLockDuration => (1640, 11),
            Elements::TxIsFinal => (1641, 11),
            Elements::Issuance => (26, 5),
            Elements::IssuanceAsset => (108, 7),
            Elements::IssuanceToken => (109, 7),
            Elements::IssuanceEntropy => (880, 10),
            Elements::CalculateIssuanceEntropy => (881, 10),
            Elements::CalculateAsset => (882, 10),
            Elements::CalculateExplicitToken => (883, 10),
            Elements::CalculateConfidentialToken => (1768, 11),
            Elements::LbtcAsset => (1769, 11),
            Elements::ScriptCMR => (224, 8),
            Elements::InternalKey => (900, 10),
            Elements::CurrentIndex => (901, 10),
            Elements::NumInputs => (7216, 13),
            Elements::NumOutputs => (7217, 13),
            Elements::LockTime => (7218, 13),
            Elements::OutputAsset => (7219, 13),
            Elements::OutputAmount => (14440, 14),
            Elements::OutputNonce => (14441, 14),
            Elements::OutputScriptHash => (14442, 14),
            Elements::OutputNullDatum => (14443, 14),
            Elements::OutputIsFee => (14444, 14),
            Elements::OutputSurjectionProof => (14445, 14),
            Elements::OutputRangeProof => (14446, 14),
            Elements::TotalFee => (14447, 14),
            Elements::CurrentPegin => (231168, 18),
            Elements::CurrentPrevOutpoint => (231169, 18),
            Elements::CurrentAsset => (231170, 18),
            Elements::CurrentAmount => (231171, 18),
            Elements::CurrentScriptHash => (231172, 18),
            Elements::CurrentSequence => (231173, 18),
            Elements::CurrentAnnexHash => (231174, 18),
            Elements::CurrentScriptSigHash => (231175, 18),
            Elements::CurrentReissuanceBlinding => (231176, 18),
            Elements::CurrentNewIssuanceContract => (231177, 18),
            Elements::CurrentReissuanceEntropy => (231178, 18),
            Elements::CurrentIssuanceAssetAmount => (231179, 18),
            Elements::CurrentIssuanceTokenAmount => (231180, 18),
            Elements::CurrentIssuanceAssetProof => (231181, 18),
            Elements::CurrentIssuanceTokenProof => (231182, 18),
            Elements::InputPegin => (231183, 18),
            Elements::InputPrevOutpoint => (462368, 19),
            Elements::InputAsset => (462369, 19),
            Elements::InputAmount => (462370, 19),
            Elements::InputScriptHash => (462371, 19),
            Elements::InputSequence => (462372, 19),
            Elements::InputAnnexHash => (462373, 19),
            Elements::InputScriptSigHash => (462374, 19),
            Elements::ReissuanceBlinding => (462375, 19),
            Elements::NewIssuanceContract => (462376, 19),
            Elements::ReissuanceEntropy => (462377, 19),
            Elements::IssuanceAssetAmount => (462378, 19),
            Elements::IssuanceTokenAmount => (462379, 19),
            Elements::IssuanceAssetProof => (462380, 19),
            Elements::IssuanceTokenProof => (462381, 19),
            Elements::TapleafVersion => (462382, 19),
            Elements::Tappath => (462383, 19),
            Elements::Version => (462384, 19),
            Elements::GenesisBlockHash => (462385, 19),
            Elements::TransactionId => (462386, 19),
        };

        w.write_bits_be(n, len)
    }

    fn decode<I: Iterator<Item = u8>>(bits: &mut BitIter<I>) -> Result<Self, decode::Error> {
        decode_bits!(bits, {
            0 => {
                0 => {
                    0 => {Elements::Verify},
                    1 => {
                        0 => {
                            0 => {
                                0 => {Elements::Low1},
                                1 => {
                                    0 => {
                                        0 => {},
                                        1 => {Elements::Low8}
                                    },
                                    1 => {
                                        0 => {
                                            0 => {
                                                0 => {
                                                    0 => {Elements::Low16},
                                                    1 => {Elements::Low32}
                                                },
                                                1 => {
                                                    0 => {Elements::Low64},
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
                                0 => {Elements::High1},
                                1 => {
                                    0 => {
                                        0 => {},
                                        1 => {Elements::High8}
                                    },
                                    1 => {
                                        0 => {
                                            0 => {
                                                0 => {
                                                    0 => {Elements::High16},
                                                    1 => {Elements::High32}
                                                },
                                                1 => {
                                                    0 => {Elements::High64},
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
                                            0 => {Elements::Complement1},
                                            1 => {
                                                0 => {
                                                    0 => {},
                                                    1 => {Elements::Complement8}
                                                },
                                                1 => {
                                                    0 => {
                                                        0 => {
                                                            0 => {
                                                                0 => {Elements::Complement16},
                                                                1 => {Elements::Complement32}
                                                            },
                                                            1 => {
                                                                0 => {Elements::Complement64},
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
                                            0 => {Elements::And1},
                                            1 => {
                                                0 => {
                                                    0 => {},
                                                    1 => {Elements::And8}
                                                },
                                                1 => {
                                                    0 => {
                                                        0 => {
                                                            0 => {
                                                                0 => {Elements::And16},
                                                                1 => {Elements::And32}
                                                            },
                                                            1 => {
                                                                0 => {Elements::And64},
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
                                            0 => {Elements::Or1},
                                            1 => {
                                                0 => {
                                                    0 => {},
                                                    1 => {Elements::Or8}
                                                },
                                                1 => {
                                                    0 => {
                                                        0 => {
                                                            0 => {
                                                                0 => {Elements::Or16},
                                                                1 => {Elements::Or32}
                                                            },
                                                            1 => {
                                                                0 => {Elements::Or64},
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
                                            0 => {Elements::Xor1},
                                            1 => {
                                                0 => {
                                                    0 => {},
                                                    1 => {Elements::Xor8}
                                                },
                                                1 => {
                                                    0 => {
                                                        0 => {
                                                            0 => {
                                                                0 => {Elements::Xor16},
                                                                1 => {Elements::Xor32}
                                                            },
                                                            1 => {
                                                                0 => {Elements::Xor64},
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
                                                0 => {Elements::Maj1},
                                                1 => {
                                                    0 => {
                                                        0 => {},
                                                        1 => {Elements::Maj8}
                                                    },
                                                    1 => {
                                                        0 => {
                                                            0 => {
                                                                0 => {
                                                                    0 => {Elements::Maj16},
                                                                    1 => {Elements::Maj32}
                                                                },
                                                                1 => {
                                                                    0 => {Elements::Maj64},
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
                                                0 => {Elements::XorXor1},
                                                1 => {
                                                    0 => {
                                                        0 => {},
                                                        1 => {Elements::XorXor8}
                                                    },
                                                    1 => {
                                                        0 => {
                                                            0 => {
                                                                0 => {
                                                                    0 => {Elements::XorXor16},
                                                                    1 => {Elements::XorXor32}
                                                                },
                                                                1 => {
                                                                    0 => {Elements::XorXor64},
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
                                                0 => {Elements::Ch1},
                                                1 => {
                                                    0 => {
                                                        0 => {},
                                                        1 => {Elements::Ch8}
                                                    },
                                                    1 => {
                                                        0 => {
                                                            0 => {
                                                                0 => {
                                                                    0 => {Elements::Ch16},
                                                                    1 => {Elements::Ch32}
                                                                },
                                                                1 => {
                                                                    0 => {Elements::Ch64},
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
                                                0 => {Elements::Some1},
                                                1 => {
                                                    0 => {
                                                        0 => {},
                                                        1 => {Elements::Some8}
                                                    },
                                                    1 => {
                                                        0 => {
                                                            0 => {
                                                                0 => {
                                                                    0 => {Elements::Some16},
                                                                    1 => {Elements::Some32}
                                                                },
                                                                1 => {
                                                                    0 => {Elements::Some64},
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
                                                        1 => {Elements::All8}
                                                    },
                                                    1 => {
                                                        0 => {
                                                            0 => {
                                                                0 => {
                                                                    0 => {Elements::All16},
                                                                    1 => {Elements::All32}
                                                                },
                                                                1 => {
                                                                    0 => {Elements::All64},
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
                                                0 => {Elements::Eq1},
                                                1 => {
                                                    0 => {
                                                        0 => {},
                                                        1 => {Elements::Eq8}
                                                    },
                                                    1 => {
                                                        0 => {
                                                            0 => {
                                                                0 => {
                                                                    0 => {Elements::Eq16},
                                                                    1 => {Elements::Eq32}
                                                                },
                                                                1 => {
                                                                    0 => {Elements::Eq64},
                                                                    1 => {}
                                                                }
                                                            },
                                                            1 => {
                                                                0 => {
                                                                    0 => {
                                                                        0 => {Elements::Eq256},
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
                                                            1 => {Elements::FullLeftShift8_1}
                                                        },
                                                        1 => {
                                                            0 => {
                                                                0 => {
                                                                    0 => {
                                                                        0 => {Elements::FullLeftShift16_1},
                                                                        1 => {Elements::FullLeftShift32_1}
                                                                    },
                                                                    1 => {
                                                                        0 => {Elements::FullLeftShift64_1},
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
                                                                    0 => {Elements::FullLeftShift8_2},
                                                                    1 => {Elements::FullLeftShift16_2}
                                                                },
                                                                1 => {
                                                                    0 => {
                                                                        0 => {
                                                                            0 => {
                                                                                0 => {Elements::FullLeftShift32_2},
                                                                                1 => {Elements::FullLeftShift64_2}
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
                                                            0 => {Elements::FullLeftShift8_4},
                                                            1 => {
                                                                0 => {
                                                                    0 => {Elements::FullLeftShift16_4},
                                                                    1 => {Elements::FullLeftShift32_4}
                                                                },
                                                                1 => {
                                                                    0 => {
                                                                        0 => {
                                                                            0 => {
                                                                                0 => {Elements::FullLeftShift64_4},
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
                                                                        0 => {Elements::FullLeftShift16_8},
                                                                        1 => {
                                                                            0 => {
                                                                                0 => {Elements::FullLeftShift32_8},
                                                                                1 => {Elements::FullLeftShift64_8}
                                                                            },
                                                                            1 => {}
                                                                        }
                                                                    },
                                                                    1 => {
                                                                        0 => {Elements::FullLeftShift32_16},
                                                                        1 => {
                                                                            0 => {
                                                                                0 => {Elements::FullLeftShift64_16},
                                                                                1 => {}
                                                                            },
                                                                            1 => {}
                                                                        }
                                                                    }
                                                                },
                                                                1 => {
                                                                    0 => {
                                                                        0 => {Elements::FullLeftShift64_32},
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
                                                            1 => {Elements::FullRightShift8_1}
                                                        },
                                                        1 => {
                                                            0 => {
                                                                0 => {
                                                                    0 => {
                                                                        0 => {Elements::FullRightShift16_1},
                                                                        1 => {Elements::FullRightShift32_1}
                                                                    },
                                                                    1 => {
                                                                        0 => {Elements::FullRightShift64_1},
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
                                                                    0 => {Elements::FullRightShift8_2},
                                                                    1 => {Elements::FullRightShift16_2}
                                                                },
                                                                1 => {
                                                                    0 => {
                                                                        0 => {
                                                                            0 => {
                                                                                0 => {Elements::FullRightShift32_2},
                                                                                1 => {Elements::FullRightShift64_2}
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
                                                            0 => {Elements::FullRightShift8_4},
                                                            1 => {
                                                                0 => {
                                                                    0 => {Elements::FullRightShift16_4},
                                                                    1 => {Elements::FullRightShift32_4}
                                                                },
                                                                1 => {
                                                                    0 => {
                                                                        0 => {
                                                                            0 => {
                                                                                0 => {Elements::FullRightShift64_4},
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
                                                                        0 => {Elements::FullRightShift16_8},
                                                                        1 => {
                                                                            0 => {
                                                                                0 => {Elements::FullRightShift32_8},
                                                                                1 => {Elements::FullRightShift64_8}
                                                                            },
                                                                            1 => {}
                                                                        }
                                                                    },
                                                                    1 => {
                                                                        0 => {Elements::FullRightShift32_16},
                                                                        1 => {
                                                                            0 => {
                                                                                0 => {Elements::FullRightShift64_16},
                                                                                1 => {}
                                                                            },
                                                                            1 => {}
                                                                        }
                                                                    }
                                                                },
                                                                1 => {
                                                                    0 => {
                                                                        0 => {Elements::FullRightShift64_32},
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
                                                                            1 => {Elements::Leftmost8_1}
                                                                        },
                                                                        1 => {
                                                                            0 => {
                                                                                0 => {
                                                                                    0 => {
                                                                                        0 => {Elements::Leftmost16_1},
                                                                                        1 => {Elements::Leftmost32_1}
                                                                                    },
                                                                                    1 => {
                                                                                        0 => {Elements::Leftmost64_1},
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
                                                                                    0 => {Elements::Leftmost8_2},
                                                                                    1 => {Elements::Leftmost16_2}
                                                                                },
                                                                                1 => {
                                                                                    0 => {
                                                                                        0 => {
                                                                                            0 => {
                                                                                                0 => {Elements::Leftmost32_2},
                                                                                                1 => {Elements::Leftmost64_2}
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
                                                                            0 => {Elements::Leftmost8_4},
                                                                            1 => {
                                                                                0 => {
                                                                                    0 => {Elements::Leftmost16_4},
                                                                                    1 => {Elements::Leftmost32_4}
                                                                                },
                                                                                1 => {
                                                                                    0 => {
                                                                                        0 => {
                                                                                            0 => {
                                                                                                0 => {Elements::Leftmost64_4},
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
                                                                                        0 => {Elements::Leftmost16_8},
                                                                                        1 => {
                                                                                            0 => {
                                                                                                0 => {Elements::Leftmost32_8},
                                                                                                1 => {Elements::Leftmost64_8}
                                                                                            },
                                                                                            1 => {}
                                                                                        }
                                                                                    },
                                                                                    1 => {
                                                                                        0 => {Elements::Leftmost32_16},
                                                                                        1 => {
                                                                                            0 => {
                                                                                                0 => {Elements::Leftmost64_16},
                                                                                                1 => {}
                                                                                            },
                                                                                            1 => {}
                                                                                        }
                                                                                    }
                                                                                },
                                                                                1 => {
                                                                                    0 => {
                                                                                        0 => {Elements::Leftmost64_32},
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
                                                                            1 => {Elements::Rightmost8_1}
                                                                        },
                                                                        1 => {
                                                                            0 => {
                                                                                0 => {
                                                                                    0 => {
                                                                                        0 => {Elements::Rightmost16_1},
                                                                                        1 => {Elements::Rightmost32_1}
                                                                                    },
                                                                                    1 => {
                                                                                        0 => {Elements::Rightmost64_1},
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
                                                                                    0 => {Elements::Rightmost8_2},
                                                                                    1 => {Elements::Rightmost16_2}
                                                                                },
                                                                                1 => {
                                                                                    0 => {
                                                                                        0 => {
                                                                                            0 => {
                                                                                                0 => {Elements::Rightmost32_2},
                                                                                                1 => {Elements::Rightmost64_2}
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
                                                                            0 => {Elements::Rightmost8_4},
                                                                            1 => {
                                                                                0 => {
                                                                                    0 => {Elements::Rightmost16_4},
                                                                                    1 => {Elements::Rightmost32_4}
                                                                                },
                                                                                1 => {
                                                                                    0 => {
                                                                                        0 => {
                                                                                            0 => {
                                                                                                0 => {Elements::Rightmost64_4},
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
                                                                                        0 => {Elements::Rightmost16_8},
                                                                                        1 => {
                                                                                            0 => {
                                                                                                0 => {Elements::Rightmost32_8},
                                                                                                1 => {Elements::Rightmost64_8}
                                                                                            },
                                                                                            1 => {}
                                                                                        }
                                                                                    },
                                                                                    1 => {
                                                                                        0 => {Elements::Rightmost32_16},
                                                                                        1 => {
                                                                                            0 => {
                                                                                                0 => {Elements::Rightmost64_16},
                                                                                                1 => {}
                                                                                            },
                                                                                            1 => {}
                                                                                        }
                                                                                    }
                                                                                },
                                                                                1 => {
                                                                                    0 => {
                                                                                        0 => {Elements::Rightmost64_32},
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
                                                                            1 => {Elements::LeftPadLow1_8}
                                                                        },
                                                                        1 => {
                                                                            0 => {
                                                                                0 => {
                                                                                    0 => {
                                                                                        0 => {Elements::LeftPadLow1_16},
                                                                                        1 => {Elements::LeftPadLow1_32}
                                                                                    },
                                                                                    1 => {
                                                                                        0 => {Elements::LeftPadLow1_64},
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
                                                                                        0 => {Elements::LeftPadLow8_16},
                                                                                        1 => {
                                                                                            0 => {
                                                                                                0 => {Elements::LeftPadLow8_32},
                                                                                                1 => {Elements::LeftPadLow8_64}
                                                                                            },
                                                                                            1 => {}
                                                                                        }
                                                                                    },
                                                                                    1 => {
                                                                                        0 => {Elements::LeftPadLow16_32},
                                                                                        1 => {
                                                                                            0 => {
                                                                                                0 => {Elements::LeftPadLow16_64},
                                                                                                1 => {}
                                                                                            },
                                                                                            1 => {}
                                                                                        }
                                                                                    }
                                                                                },
                                                                                1 => {
                                                                                    0 => {
                                                                                        0 => {Elements::LeftPadLow32_64},
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
                                                                            1 => {Elements::LeftPadHigh1_8}
                                                                        },
                                                                        1 => {
                                                                            0 => {
                                                                                0 => {
                                                                                    0 => {
                                                                                        0 => {Elements::LeftPadHigh1_16},
                                                                                        1 => {Elements::LeftPadHigh1_32}
                                                                                    },
                                                                                    1 => {
                                                                                        0 => {Elements::LeftPadHigh1_64},
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
                                                                                        0 => {Elements::LeftPadHigh8_16},
                                                                                        1 => {
                                                                                            0 => {
                                                                                                0 => {Elements::LeftPadHigh8_32},
                                                                                                1 => {Elements::LeftPadHigh8_64}
                                                                                            },
                                                                                            1 => {}
                                                                                        }
                                                                                    },
                                                                                    1 => {
                                                                                        0 => {Elements::LeftPadHigh16_32},
                                                                                        1 => {
                                                                                            0 => {
                                                                                                0 => {Elements::LeftPadHigh16_64},
                                                                                                1 => {}
                                                                                            },
                                                                                            1 => {}
                                                                                        }
                                                                                    }
                                                                                },
                                                                                1 => {
                                                                                    0 => {
                                                                                        0 => {Elements::LeftPadHigh32_64},
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
                                                                            1 => {Elements::LeftExtend1_8}
                                                                        },
                                                                        1 => {
                                                                            0 => {
                                                                                0 => {
                                                                                    0 => {
                                                                                        0 => {Elements::LeftExtend1_16},
                                                                                        1 => {Elements::LeftExtend1_32}
                                                                                    },
                                                                                    1 => {
                                                                                        0 => {Elements::LeftExtend1_64},
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
                                                                                        0 => {Elements::LeftExtend8_16},
                                                                                        1 => {
                                                                                            0 => {
                                                                                                0 => {Elements::LeftExtend8_32},
                                                                                                1 => {Elements::LeftExtend8_64}
                                                                                            },
                                                                                            1 => {}
                                                                                        }
                                                                                    },
                                                                                    1 => {
                                                                                        0 => {Elements::LeftExtend16_32},
                                                                                        1 => {
                                                                                            0 => {
                                                                                                0 => {Elements::LeftExtend16_64},
                                                                                                1 => {}
                                                                                            },
                                                                                            1 => {}
                                                                                        }
                                                                                    }
                                                                                },
                                                                                1 => {
                                                                                    0 => {
                                                                                        0 => {Elements::LeftExtend32_64},
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
                                                                            1 => {Elements::RightPadLow1_8}
                                                                        },
                                                                        1 => {
                                                                            0 => {
                                                                                0 => {
                                                                                    0 => {
                                                                                        0 => {Elements::RightPadLow1_16},
                                                                                        1 => {Elements::RightPadLow1_32}
                                                                                    },
                                                                                    1 => {
                                                                                        0 => {Elements::RightPadLow1_64},
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
                                                                                        0 => {Elements::RightPadLow8_16},
                                                                                        1 => {
                                                                                            0 => {
                                                                                                0 => {Elements::RightPadLow8_32},
                                                                                                1 => {Elements::RightPadLow8_64}
                                                                                            },
                                                                                            1 => {}
                                                                                        }
                                                                                    },
                                                                                    1 => {
                                                                                        0 => {Elements::RightPadLow16_32},
                                                                                        1 => {
                                                                                            0 => {
                                                                                                0 => {Elements::RightPadLow16_64},
                                                                                                1 => {}
                                                                                            },
                                                                                            1 => {}
                                                                                        }
                                                                                    }
                                                                                },
                                                                                1 => {
                                                                                    0 => {
                                                                                        0 => {Elements::RightPadLow32_64},
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
                                                                            1 => {Elements::RightPadHigh1_8}
                                                                        },
                                                                        1 => {
                                                                            0 => {
                                                                                0 => {
                                                                                    0 => {
                                                                                        0 => {Elements::RightPadHigh1_16},
                                                                                        1 => {Elements::RightPadHigh1_32}
                                                                                    },
                                                                                    1 => {
                                                                                        0 => {Elements::RightPadHigh1_64},
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
                                                                                        0 => {Elements::RightPadHigh8_16},
                                                                                        1 => {
                                                                                            0 => {
                                                                                                0 => {Elements::RightPadHigh8_32},
                                                                                                1 => {Elements::RightPadHigh8_64}
                                                                                            },
                                                                                            1 => {}
                                                                                        }
                                                                                    },
                                                                                    1 => {
                                                                                        0 => {Elements::RightPadHigh16_32},
                                                                                        1 => {
                                                                                            0 => {
                                                                                                0 => {Elements::RightPadHigh16_64},
                                                                                                1 => {}
                                                                                            },
                                                                                            1 => {}
                                                                                        }
                                                                                    }
                                                                                },
                                                                                1 => {
                                                                                    0 => {
                                                                                        0 => {Elements::RightPadHigh32_64},
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
                                                                                        0 => {Elements::RightExtend8_16},
                                                                                        1 => {
                                                                                            0 => {
                                                                                                0 => {Elements::RightExtend8_32},
                                                                                                1 => {Elements::RightExtend8_64}
                                                                                            },
                                                                                            1 => {}
                                                                                        }
                                                                                    },
                                                                                    1 => {
                                                                                        0 => {Elements::RightExtend16_32},
                                                                                        1 => {
                                                                                            0 => {
                                                                                                0 => {Elements::RightExtend16_64},
                                                                                                1 => {}
                                                                                            },
                                                                                            1 => {}
                                                                                        }
                                                                                    }
                                                                                },
                                                                                1 => {
                                                                                    0 => {
                                                                                        0 => {Elements::RightExtend32_64},
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
                                                                        1 => {Elements::LeftShiftWith8}
                                                                    },
                                                                    1 => {
                                                                        0 => {
                                                                            0 => {
                                                                                0 => {
                                                                                    0 => {Elements::LeftShiftWith16},
                                                                                    1 => {Elements::LeftShiftWith32}
                                                                                },
                                                                                1 => {
                                                                                    0 => {Elements::LeftShiftWith64},
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
                                                                        1 => {Elements::RightShiftWith8}
                                                                    },
                                                                    1 => {
                                                                        0 => {
                                                                            0 => {
                                                                                0 => {
                                                                                    0 => {Elements::RightShiftWith16},
                                                                                    1 => {Elements::RightShiftWith32}
                                                                                },
                                                                                1 => {
                                                                                    0 => {Elements::RightShiftWith64},
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
                                                                        1 => {Elements::LeftShift8}
                                                                    },
                                                                    1 => {
                                                                        0 => {
                                                                            0 => {
                                                                                0 => {
                                                                                    0 => {Elements::LeftShift16},
                                                                                    1 => {Elements::LeftShift32}
                                                                                },
                                                                                1 => {
                                                                                    0 => {Elements::LeftShift64},
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
                                                                        1 => {Elements::RightShift8}
                                                                    },
                                                                    1 => {
                                                                        0 => {
                                                                            0 => {
                                                                                0 => {
                                                                                    0 => {Elements::RightShift16},
                                                                                    1 => {Elements::RightShift32}
                                                                                },
                                                                                1 => {
                                                                                    0 => {Elements::RightShift64},
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
                                                                        1 => {Elements::LeftRotate8}
                                                                    },
                                                                    1 => {
                                                                        0 => {
                                                                            0 => {
                                                                                0 => {
                                                                                    0 => {Elements::LeftRotate16},
                                                                                    1 => {Elements::LeftRotate32}
                                                                                },
                                                                                1 => {
                                                                                    0 => {Elements::LeftRotate64},
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
                                                                        1 => {Elements::RightRotate8}
                                                                    },
                                                                    1 => {
                                                                        0 => {
                                                                            0 => {
                                                                                0 => {
                                                                                    0 => {Elements::RightRotate16},
                                                                                    1 => {Elements::RightRotate32}
                                                                                },
                                                                                1 => {
                                                                                    0 => {Elements::RightRotate64},
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
                                        1 => {Elements::One8}
                                    },
                                    1 => {
                                        0 => {
                                            0 => {
                                                0 => {
                                                    0 => {Elements::One16},
                                                    1 => {Elements::One32}
                                                },
                                                1 => {
                                                    0 => {Elements::One64},
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
                                                1 => {Elements::FullAdd8}
                                            },
                                            1 => {
                                                0 => {
                                                    0 => {
                                                        0 => {
                                                            0 => {Elements::FullAdd16},
                                                            1 => {Elements::FullAdd32}
                                                        },
                                                        1 => {
                                                            0 => {Elements::FullAdd64},
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
                                                1 => {Elements::Add8}
                                            },
                                            1 => {
                                                0 => {
                                                    0 => {
                                                        0 => {
                                                            0 => {Elements::Add16},
                                                            1 => {Elements::Add32}
                                                        },
                                                        1 => {
                                                            0 => {Elements::Add64},
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
                                                            1 => {Elements::FullIncrement8}
                                                        },
                                                        1 => {
                                                            0 => {
                                                                0 => {
                                                                    0 => {
                                                                        0 => {Elements::FullIncrement16},
                                                                        1 => {Elements::FullIncrement32}
                                                                    },
                                                                    1 => {
                                                                        0 => {Elements::FullIncrement64},
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
                                                            1 => {Elements::Increment8}
                                                        },
                                                        1 => {
                                                            0 => {
                                                                0 => {
                                                                    0 => {
                                                                        0 => {Elements::Increment16},
                                                                        1 => {Elements::Increment32}
                                                                    },
                                                                    1 => {
                                                                        0 => {Elements::Increment64},
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
                                                            1 => {Elements::FullSubtract8}
                                                        },
                                                        1 => {
                                                            0 => {
                                                                0 => {
                                                                    0 => {
                                                                        0 => {Elements::FullSubtract16},
                                                                        1 => {Elements::FullSubtract32}
                                                                    },
                                                                    1 => {
                                                                        0 => {Elements::FullSubtract64},
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
                                                                1 => {Elements::Subtract8}
                                                            },
                                                            1 => {
                                                                0 => {
                                                                    0 => {
                                                                        0 => {
                                                                            0 => {Elements::Subtract16},
                                                                            1 => {Elements::Subtract32}
                                                                        },
                                                                        1 => {
                                                                            0 => {Elements::Subtract64},
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
                                                                1 => {Elements::Negate8}
                                                            },
                                                            1 => {
                                                                0 => {
                                                                    0 => {
                                                                        0 => {
                                                                            0 => {Elements::Negate16},
                                                                            1 => {Elements::Negate32}
                                                                        },
                                                                        1 => {
                                                                            0 => {Elements::Negate64},
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
                                                                1 => {Elements::FullDecrement8}
                                                            },
                                                            1 => {
                                                                0 => {
                                                                    0 => {
                                                                        0 => {
                                                                            0 => {Elements::FullDecrement16},
                                                                            1 => {Elements::FullDecrement32}
                                                                        },
                                                                        1 => {
                                                                            0 => {Elements::FullDecrement64},
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
                                                                1 => {Elements::Decrement8}
                                                            },
                                                            1 => {
                                                                0 => {
                                                                    0 => {
                                                                        0 => {
                                                                            0 => {Elements::Decrement16},
                                                                            1 => {Elements::Decrement32}
                                                                        },
                                                                        1 => {
                                                                            0 => {Elements::Decrement64},
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
                                                                1 => {Elements::FullMultiply8}
                                                            },
                                                            1 => {
                                                                0 => {
                                                                    0 => {
                                                                        0 => {
                                                                            0 => {Elements::FullMultiply16},
                                                                            1 => {Elements::FullMultiply32}
                                                                        },
                                                                        1 => {
                                                                            0 => {Elements::FullMultiply64},
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
                                                                1 => {Elements::Multiply8}
                                                            },
                                                            1 => {
                                                                0 => {
                                                                    0 => {
                                                                        0 => {
                                                                            0 => {Elements::Multiply16},
                                                                            1 => {Elements::Multiply32}
                                                                        },
                                                                        1 => {
                                                                            0 => {Elements::Multiply64},
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
                                                                1 => {Elements::IsZero8}
                                                            },
                                                            1 => {
                                                                0 => {
                                                                    0 => {
                                                                        0 => {
                                                                            0 => {Elements::IsZero16},
                                                                            1 => {Elements::IsZero32}
                                                                        },
                                                                        1 => {
                                                                            0 => {Elements::IsZero64},
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
                                                                1 => {Elements::IsOne8}
                                                            },
                                                            1 => {
                                                                0 => {
                                                                    0 => {
                                                                        0 => {
                                                                            0 => {Elements::IsOne16},
                                                                            1 => {Elements::IsOne32}
                                                                        },
                                                                        1 => {
                                                                            0 => {Elements::IsOne64},
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
                                                                                1 => {Elements::Le8}
                                                                            },
                                                                            1 => {
                                                                                0 => {
                                                                                    0 => {
                                                                                        0 => {
                                                                                            0 => {Elements::Le16},
                                                                                            1 => {Elements::Le32}
                                                                                        },
                                                                                        1 => {
                                                                                            0 => {Elements::Le64},
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
                                                                                1 => {Elements::Lt8}
                                                                            },
                                                                            1 => {
                                                                                0 => {
                                                                                    0 => {
                                                                                        0 => {
                                                                                            0 => {Elements::Lt16},
                                                                                            1 => {Elements::Lt32}
                                                                                        },
                                                                                        1 => {
                                                                                            0 => {Elements::Lt64},
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
                                                                                1 => {Elements::Min8}
                                                                            },
                                                                            1 => {
                                                                                0 => {
                                                                                    0 => {
                                                                                        0 => {
                                                                                            0 => {Elements::Min16},
                                                                                            1 => {Elements::Min32}
                                                                                        },
                                                                                        1 => {
                                                                                            0 => {Elements::Min64},
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
                                                                                1 => {Elements::Max8}
                                                                            },
                                                                            1 => {
                                                                                0 => {
                                                                                    0 => {
                                                                                        0 => {
                                                                                            0 => {Elements::Max16},
                                                                                            1 => {Elements::Max32}
                                                                                        },
                                                                                        1 => {
                                                                                            0 => {Elements::Max64},
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
                                                                                1 => {Elements::Median8}
                                                                            },
                                                                            1 => {
                                                                                0 => {
                                                                                    0 => {
                                                                                        0 => {
                                                                                            0 => {Elements::Median16},
                                                                                            1 => {Elements::Median32}
                                                                                        },
                                                                                        1 => {
                                                                                            0 => {Elements::Median64},
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
                                                                                            0 => {Elements::DivMod128_64},
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
                                                                                1 => {Elements::DivMod8}
                                                                            },
                                                                            1 => {
                                                                                0 => {
                                                                                    0 => {
                                                                                        0 => {
                                                                                            0 => {Elements::DivMod16},
                                                                                            1 => {Elements::DivMod32}
                                                                                        },
                                                                                        1 => {
                                                                                            0 => {Elements::DivMod64},
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
                                                                                1 => {Elements::Divide8}
                                                                            },
                                                                            1 => {
                                                                                0 => {
                                                                                    0 => {
                                                                                        0 => {
                                                                                            0 => {Elements::Divide16},
                                                                                            1 => {Elements::Divide32}
                                                                                        },
                                                                                        1 => {
                                                                                            0 => {Elements::Divide64},
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
                                                                                1 => {Elements::Modulo8}
                                                                            },
                                                                            1 => {
                                                                                0 => {
                                                                                    0 => {
                                                                                        0 => {
                                                                                            0 => {Elements::Modulo16},
                                                                                            1 => {Elements::Modulo32}
                                                                                        },
                                                                                        1 => {
                                                                                            0 => {Elements::Modulo64},
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
                                                                                1 => {Elements::Divides8}
                                                                            },
                                                                            1 => {
                                                                                0 => {
                                                                                    0 => {
                                                                                        0 => {
                                                                                            0 => {Elements::Divides16},
                                                                                            1 => {Elements::Divides32}
                                                                                        },
                                                                                        1 => {
                                                                                            0 => {Elements::Divides64},
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
                                0 => {Elements::Sha256Block},
                                1 => {
                                    0 => {
                                        0 => {Elements::Sha256Iv},
                                        1 => {
                                            0 => {Elements::Sha256Ctx8Add1},
                                            1 => {
                                                0 => {
                                                    0 => {Elements::Sha256Ctx8Add2},
                                                    1 => {Elements::Sha256Ctx8Add4}
                                                },
                                                1 => {
                                                    0 => {
                                                        0 => {
                                                            0 => {
                                                                0 => {Elements::Sha256Ctx8Add8},
                                                                1 => {Elements::Sha256Ctx8Add16}
                                                            },
                                                            1 => {
                                                                0 => {Elements::Sha256Ctx8Add32},
                                                                1 => {Elements::Sha256Ctx8Add64}
                                                            }
                                                        },
                                                        1 => {
                                                            0 => {
                                                                0 => {
                                                                    0 => {Elements::Sha256Ctx8Add128},
                                                                    1 => {Elements::Sha256Ctx8Add256}
                                                                },
                                                                1 => {
                                                                    0 => {Elements::Sha256Ctx8Add512},
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
                                                    0 => {Elements::Sha256Ctx8AddBuffer511},
                                                    1 => {Elements::Sha256Ctx8Finalize}
                                                },
                                                1 => {
                                                    0 => {Elements::Sha256Ctx8Init},
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
                                            0 => {Elements::PointVerify1},
                                            1 => {}
                                        },
                                        1 => {
                                            0 => {
                                                0 => {Elements::Decompress},
                                                1 => {
                                                    0 => {Elements::LinearVerify1},
                                                    1 => {}
                                                }
                                            },
                                            1 => {
                                                0 => {
                                                    0 => {
                                                        0 => {
                                                            0 => {
                                                                0 => {Elements::LinearCombination1},
                                                                1 => {}
                                                            },
                                                            1 => {Elements::Scale}
                                                        },
                                                        1 => {
                                                            0 => {Elements::Generate},
                                                            1 => {Elements::GejInfinity}
                                                        }
                                                    },
                                                    1 => {
                                                        0 => {
                                                            0 => {
                                                                0 => {Elements::GejNormalize},
                                                                1 => {Elements::GejNegate}
                                                            },
                                                            1 => {
                                                                0 => {Elements::GeNegate},
                                                                1 => {Elements::GejDouble}
                                                            }
                                                        },
                                                        1 => {
                                                            0 => {
                                                                0 => {Elements::GejAdd},
                                                                1 => {Elements::GejGeAddEx}
                                                            },
                                                            1 => {
                                                                0 => {Elements::GejGeAdd},
                                                                1 => {Elements::GejRescale}
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
                                                                                0 => {Elements::GejIsInfinity},
                                                                                1 => {Elements::GejEquiv}
                                                                            },
                                                                            1 => {
                                                                                0 => {Elements::GejGeEquiv},
                                                                                1 => {Elements::GejXEquiv}
                                                                            }
                                                                        },
                                                                        1 => {
                                                                            0 => {
                                                                                0 => {Elements::GejYIsOdd},
                                                                                1 => {Elements::GejIsOnCurve}
                                                                            },
                                                                            1 => {
                                                                                0 => {Elements::GeIsOnCurve},
                                                                                1 => {Elements::ScalarNormalize}
                                                                            }
                                                                        }
                                                                    },
                                                                    1 => {
                                                                        0 => {
                                                                            0 => {
                                                                                0 => {Elements::ScalarNegate},
                                                                                1 => {Elements::ScalarAdd}
                                                                            },
                                                                            1 => {
                                                                                0 => {Elements::ScalarSquare},
                                                                                1 => {Elements::ScalarMultiply}
                                                                            }
                                                                        },
                                                                        1 => {
                                                                            0 => {
                                                                                0 => {Elements::ScalarMultiplyLambda},
                                                                                1 => {Elements::ScalarInvert}
                                                                            },
                                                                            1 => {
                                                                                0 => {Elements::ScalarIsZero},
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
                                                                                    1 => {Elements::FeNormalize}
                                                                                }
                                                                            },
                                                                            1 => {
                                                                                0 => {
                                                                                    0 => {Elements::FeNegate},
                                                                                    1 => {Elements::FeAdd}
                                                                                },
                                                                                1 => {
                                                                                    0 => {Elements::FeSquare},
                                                                                    1 => {Elements::FeMultiply}
                                                                                }
                                                                            }
                                                                        },
                                                                        1 => {
                                                                            0 => {
                                                                                0 => {
                                                                                    0 => {Elements::FeMultiplyBeta},
                                                                                    1 => {Elements::FeInvert}
                                                                                },
                                                                                1 => {
                                                                                    0 => {Elements::FeSquareRoot},
                                                                                    1 => {Elements::FeIsZero}
                                                                                }
                                                                            },
                                                                            1 => {
                                                                                0 => {
                                                                                    0 => {Elements::FeIsOdd},
                                                                                    1 => {}
                                                                                },
                                                                                1 => {
                                                                                    0 => {Elements::HashToCurve},
                                                                                    1 => {Elements::Swu}
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
                                        0 => {Elements::CheckSigVerify},
                                        1 => {
                                            0 => {
                                                0 => {Elements::Bip0340Verify},
                                                1 => {}
                                            },
                                            1 => {}
                                        }
                                    }
                                },
                                1 => {
                                    0 => {},
                                    1 => {
                                        0 => {Elements::ParseLock},
                                        1 => {
                                            0 => {
                                                0 => {Elements::ParseSequence},
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
                0 => {
                    0 => {Elements::SigAllHash},
                    1 => {
                        0 => {
                            0 => {Elements::TxHash},
                            1 => {Elements::TapEnvHash}
                        },
                        1 => {
                            0 => {
                                0 => {
                                    0 => {
                                        0 => {Elements::OutputsHash},
                                        1 => {Elements::InputsHash}
                                    },
                                    1 => {
                                        0 => {Elements::IssuancesHash},
                                        1 => {Elements::InputUtxosHash}
                                    }
                                },
                                1 => {
                                    0 => {
                                        0 => {
                                            0 => {Elements::OutputHash},
                                            1 => {Elements::OutputAmountsHash}
                                        },
                                        1 => {
                                            0 => {Elements::OutputScriptsHash},
                                            1 => {Elements::OutputNoncesHash}
                                        }
                                    },
                                    1 => {
                                        0 => {
                                            0 => {Elements::OutputRangeProofsHash},
                                            1 => {Elements::OutputSurjectionProofsHash}
                                        },
                                        1 => {
                                            0 => {Elements::InputHash},
                                            1 => {Elements::InputOutpointsHash}
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
                                                            0 => {Elements::InputSequencesHash},
                                                            1 => {Elements::InputAnnexesHash}
                                                        },
                                                        1 => {
                                                            0 => {Elements::InputScriptSigsHash},
                                                            1 => {Elements::IssuanceHash}
                                                        }
                                                    },
                                                    1 => {
                                                        0 => {
                                                            0 => {Elements::IssuanceAssetAmountsHash},
                                                            1 => {Elements::IssuanceTokenAmountsHash}
                                                        },
                                                        1 => {
                                                            0 => {Elements::IssuanceRangeProofsHash},
                                                            1 => {Elements::IssuanceBlindingEntropyHash}
                                                        }
                                                    }
                                                },
                                                1 => {
                                                    0 => {
                                                        0 => {
                                                            0 => {Elements::InputUtxoHash},
                                                            1 => {Elements::InputAmountsHash}
                                                        },
                                                        1 => {
                                                            0 => {Elements::InputScriptsHash},
                                                            1 => {Elements::TapleafHash}
                                                        }
                                                    },
                                                    1 => {
                                                        0 => {
                                                            0 => {Elements::TappathHash},
                                                            1 => {Elements::OutpointHash}
                                                        },
                                                        1 => {
                                                            0 => {Elements::AssetAmountHash},
                                                            1 => {Elements::NonceHash}
                                                        }
                                                    }
                                                }
                                            },
                                            1 => {
                                                0 => {
                                                    0 => {
                                                        0 => {
                                                            0 => {
                                                                0 => {Elements::AnnexHash},
                                                                1 => {Elements::BuildTapleafSimplicity}
                                                            },
                                                            1 => {
                                                                0 => {Elements::BuildTapbranch},
                                                                1 => {}
                                                            }
                                                        },
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
                            }
                        }
                    }
                },
                1 => {
                    0 => {
                        0 => {
                            0 => {Elements::CheckLockHeight},
                            1 => {
                                0 => {
                                    0 => {Elements::CheckLockTime},
                                    1 => {Elements::CheckLockDistance}
                                },
                                1 => {
                                    0 => {
                                        0 => {
                                            0 => {
                                                0 => {Elements::CheckLockDuration},
                                                1 => {Elements::TxLockHeight}
                                            },
                                            1 => {
                                                0 => {Elements::TxLockTime},
                                                1 => {Elements::TxLockDistance}
                                            }
                                        },
                                        1 => {
                                            0 => {
                                                0 => {
                                                    0 => {Elements::TxLockDuration},
                                                    1 => {Elements::TxIsFinal}
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
                            0 => {Elements::Issuance},
                            1 => {
                                0 => {
                                    0 => {Elements::IssuanceAsset},
                                    1 => {Elements::IssuanceToken}
                                },
                                1 => {
                                    0 => {
                                        0 => {
                                            0 => {
                                                0 => {Elements::IssuanceEntropy},
                                                1 => {Elements::CalculateIssuanceEntropy}
                                            },
                                            1 => {
                                                0 => {Elements::CalculateAsset},
                                                1 => {Elements::CalculateExplicitToken}
                                            }
                                        },
                                        1 => {
                                            0 => {
                                                0 => {
                                                    0 => {Elements::CalculateConfidentialToken},
                                                    1 => {Elements::LbtcAsset}
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
                                0 => {
                                    0 => {
                                        0 => {Elements::ScriptCMR},
                                        1 => {
                                            0 => {
                                                0 => {Elements::InternalKey},
                                                1 => {Elements::CurrentIndex}
                                            },
                                            1 => {
                                                0 => {
                                                    0 => {
                                                        0 => {
                                                            0 => {Elements::NumInputs},
                                                            1 => {Elements::NumOutputs}
                                                        },
                                                        1 => {
                                                            0 => {Elements::LockTime},
                                                            1 => {Elements::OutputAsset}
                                                        }
                                                    },
                                                    1 => {
                                                        0 => {
                                                            0 => {
                                                                0 => {Elements::OutputAmount},
                                                                1 => {Elements::OutputNonce}
                                                            },
                                                            1 => {
                                                                0 => {Elements::OutputScriptHash},
                                                                1 => {Elements::OutputNullDatum}
                                                            }
                                                        },
                                                        1 => {
                                                            0 => {
                                                                0 => {Elements::OutputIsFee},
                                                                1 => {Elements::OutputSurjectionProof}
                                                            },
                                                            1 => {
                                                                0 => {Elements::OutputRangeProof},
                                                                1 => {Elements::TotalFee}
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
                                                                                0 => {Elements::CurrentPegin},
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
                                                                                0 => {Elements::CurrentAnnexHash},
                                                                                1 => {Elements::CurrentScriptSigHash}
                                                                            }
                                                                        }
                                                                    },
                                                                    1 => {
                                                                        0 => {
                                                                            0 => {
                                                                                0 => {Elements::CurrentReissuanceBlinding},
                                                                                1 => {Elements::CurrentNewIssuanceContract}
                                                                            },
                                                                            1 => {
                                                                                0 => {Elements::CurrentReissuanceEntropy},
                                                                                1 => {Elements::CurrentIssuanceAssetAmount}
                                                                            }
                                                                        },
                                                                        1 => {
                                                                            0 => {
                                                                                0 => {Elements::CurrentIssuanceTokenAmount},
                                                                                1 => {Elements::CurrentIssuanceAssetProof}
                                                                            },
                                                                            1 => {
                                                                                0 => {Elements::CurrentIssuanceTokenProof},
                                                                                1 => {Elements::InputPegin}
                                                                            }
                                                                        }
                                                                    }
                                                                },
                                                                1 => {
                                                                    0 => {
                                                                        0 => {
                                                                            0 => {
                                                                                0 => {
                                                                                    0 => {Elements::InputPrevOutpoint},
                                                                                    1 => {Elements::InputAsset}
                                                                                },
                                                                                1 => {
                                                                                    0 => {Elements::InputAmount},
                                                                                    1 => {Elements::InputScriptHash}
                                                                                }
                                                                            },
                                                                            1 => {
                                                                                0 => {
                                                                                    0 => {Elements::InputSequence},
                                                                                    1 => {Elements::InputAnnexHash}
                                                                                },
                                                                                1 => {
                                                                                    0 => {Elements::InputScriptSigHash},
                                                                                    1 => {Elements::ReissuanceBlinding}
                                                                                }
                                                                            }
                                                                        },
                                                                        1 => {
                                                                            0 => {
                                                                                0 => {
                                                                                    0 => {Elements::NewIssuanceContract},
                                                                                    1 => {Elements::ReissuanceEntropy}
                                                                                },
                                                                                1 => {
                                                                                    0 => {Elements::IssuanceAssetAmount},
                                                                                    1 => {Elements::IssuanceTokenAmount}
                                                                                }
                                                                            },
                                                                            1 => {
                                                                                0 => {
                                                                                    0 => {Elements::IssuanceAssetProof},
                                                                                    1 => {Elements::IssuanceTokenProof}
                                                                                },
                                                                                1 => {
                                                                                    0 => {Elements::TapleafVersion},
                                                                                    1 => {Elements::Tappath}
                                                                                }
                                                                            }
                                                                        }
                                                                    },
                                                                    1 => {
                                                                        0 => {
                                                                            0 => {
                                                                                0 => {
                                                                                    0 => {Elements::Version},
                                                                                    1 => {Elements::GenesisBlockHash}
                                                                                },
                                                                                1 => {
                                                                                    0 => {Elements::TransactionId},
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
                            },
                            1 => {}
                        },
                        1 => {}
                    }
                }
            }
        })
    }

    fn c_jet_ptr(&self) -> &dyn Fn(&mut CFrameItem, CFrameItem, &Self::CJetEnvironment) -> bool {
        match self {
            Elements::Add16 => &simplicity_sys::c_jets::jets_wrapper::add_16,
            Elements::Add32 => &simplicity_sys::c_jets::jets_wrapper::add_32,
            Elements::Add64 => &simplicity_sys::c_jets::jets_wrapper::add_64,
            Elements::Add8 => &simplicity_sys::c_jets::jets_wrapper::add_8,
            Elements::All16 => &simplicity_sys::c_jets::jets_wrapper::all_16,
            Elements::All32 => &simplicity_sys::c_jets::jets_wrapper::all_32,
            Elements::All64 => &simplicity_sys::c_jets::jets_wrapper::all_64,
            Elements::All8 => &simplicity_sys::c_jets::jets_wrapper::all_8,
            Elements::And1 => &simplicity_sys::c_jets::jets_wrapper::and_1,
            Elements::And16 => &simplicity_sys::c_jets::jets_wrapper::and_16,
            Elements::And32 => &simplicity_sys::c_jets::jets_wrapper::and_32,
            Elements::And64 => &simplicity_sys::c_jets::jets_wrapper::and_64,
            Elements::And8 => &simplicity_sys::c_jets::jets_wrapper::and_8,
            Elements::AnnexHash => &simplicity_sys::c_jets::jets_wrapper::annex_hash,
            Elements::AssetAmountHash => &simplicity_sys::c_jets::jets_wrapper::asset_amount_hash,
            Elements::Bip0340Verify => &simplicity_sys::c_jets::jets_wrapper::bip_0340_verify,
            Elements::BuildTapbranch => &simplicity_sys::c_jets::jets_wrapper::build_tapbranch,
            Elements::BuildTapleafSimplicity => &simplicity_sys::c_jets::jets_wrapper::build_tapleaf_simplicity,
            Elements::CalculateAsset => &simplicity_sys::c_jets::jets_wrapper::calculate_asset,
            Elements::CalculateConfidentialToken => &simplicity_sys::c_jets::jets_wrapper::calculate_confidential_token,
            Elements::CalculateExplicitToken => &simplicity_sys::c_jets::jets_wrapper::calculate_explicit_token,
            Elements::CalculateIssuanceEntropy => &simplicity_sys::c_jets::jets_wrapper::calculate_issuance_entropy,
            Elements::Ch1 => &simplicity_sys::c_jets::jets_wrapper::ch_1,
            Elements::Ch16 => &simplicity_sys::c_jets::jets_wrapper::ch_16,
            Elements::Ch32 => &simplicity_sys::c_jets::jets_wrapper::ch_32,
            Elements::Ch64 => &simplicity_sys::c_jets::jets_wrapper::ch_64,
            Elements::Ch8 => &simplicity_sys::c_jets::jets_wrapper::ch_8,
            Elements::CheckLockDistance => &simplicity_sys::c_jets::jets_wrapper::check_lock_distance,
            Elements::CheckLockDuration => &simplicity_sys::c_jets::jets_wrapper::check_lock_duration,
            Elements::CheckLockHeight => &simplicity_sys::c_jets::jets_wrapper::check_lock_height,
            Elements::CheckLockTime => &simplicity_sys::c_jets::jets_wrapper::check_lock_time,
            Elements::CheckSigVerify => &simplicity_sys::c_jets::jets_wrapper::check_sig_verify,
            Elements::Complement1 => &simplicity_sys::c_jets::jets_wrapper::complement_1,
            Elements::Complement16 => &simplicity_sys::c_jets::jets_wrapper::complement_16,
            Elements::Complement32 => &simplicity_sys::c_jets::jets_wrapper::complement_32,
            Elements::Complement64 => &simplicity_sys::c_jets::jets_wrapper::complement_64,
            Elements::Complement8 => &simplicity_sys::c_jets::jets_wrapper::complement_8,
            Elements::CurrentAmount => &simplicity_sys::c_jets::jets_wrapper::current_amount,
            Elements::CurrentAnnexHash => &simplicity_sys::c_jets::jets_wrapper::current_annex_hash,
            Elements::CurrentAsset => &simplicity_sys::c_jets::jets_wrapper::current_asset,
            Elements::CurrentIndex => &simplicity_sys::c_jets::jets_wrapper::current_index,
            Elements::CurrentIssuanceAssetAmount => &simplicity_sys::c_jets::jets_wrapper::current_issuance_asset_amount,
            Elements::CurrentIssuanceAssetProof => &simplicity_sys::c_jets::jets_wrapper::current_issuance_asset_proof,
            Elements::CurrentIssuanceTokenAmount => &simplicity_sys::c_jets::jets_wrapper::current_issuance_token_amount,
            Elements::CurrentIssuanceTokenProof => &simplicity_sys::c_jets::jets_wrapper::current_issuance_token_proof,
            Elements::CurrentNewIssuanceContract => &simplicity_sys::c_jets::jets_wrapper::current_new_issuance_contract,
            Elements::CurrentPegin => &simplicity_sys::c_jets::jets_wrapper::current_pegin,
            Elements::CurrentPrevOutpoint => &simplicity_sys::c_jets::jets_wrapper::current_prev_outpoint,
            Elements::CurrentReissuanceBlinding => &simplicity_sys::c_jets::jets_wrapper::current_reissuance_blinding,
            Elements::CurrentReissuanceEntropy => &simplicity_sys::c_jets::jets_wrapper::current_reissuance_entropy,
            Elements::CurrentScriptHash => &simplicity_sys::c_jets::jets_wrapper::current_script_hash,
            Elements::CurrentScriptSigHash => &simplicity_sys::c_jets::jets_wrapper::current_script_sig_hash,
            Elements::CurrentSequence => &simplicity_sys::c_jets::jets_wrapper::current_sequence,
            Elements::Decompress => &simplicity_sys::c_jets::jets_wrapper::decompress,
            Elements::Decrement16 => &simplicity_sys::c_jets::jets_wrapper::decrement_16,
            Elements::Decrement32 => &simplicity_sys::c_jets::jets_wrapper::decrement_32,
            Elements::Decrement64 => &simplicity_sys::c_jets::jets_wrapper::decrement_64,
            Elements::Decrement8 => &simplicity_sys::c_jets::jets_wrapper::decrement_8,
            Elements::DivMod128_64 => &simplicity_sys::c_jets::jets_wrapper::div_mod_128_64,
            Elements::DivMod16 => &simplicity_sys::c_jets::jets_wrapper::div_mod_16,
            Elements::DivMod32 => &simplicity_sys::c_jets::jets_wrapper::div_mod_32,
            Elements::DivMod64 => &simplicity_sys::c_jets::jets_wrapper::div_mod_64,
            Elements::DivMod8 => &simplicity_sys::c_jets::jets_wrapper::div_mod_8,
            Elements::Divide16 => &simplicity_sys::c_jets::jets_wrapper::divide_16,
            Elements::Divide32 => &simplicity_sys::c_jets::jets_wrapper::divide_32,
            Elements::Divide64 => &simplicity_sys::c_jets::jets_wrapper::divide_64,
            Elements::Divide8 => &simplicity_sys::c_jets::jets_wrapper::divide_8,
            Elements::Divides16 => &simplicity_sys::c_jets::jets_wrapper::divides_16,
            Elements::Divides32 => &simplicity_sys::c_jets::jets_wrapper::divides_32,
            Elements::Divides64 => &simplicity_sys::c_jets::jets_wrapper::divides_64,
            Elements::Divides8 => &simplicity_sys::c_jets::jets_wrapper::divides_8,
            Elements::Eq1 => &simplicity_sys::c_jets::jets_wrapper::eq_1,
            Elements::Eq16 => &simplicity_sys::c_jets::jets_wrapper::eq_16,
            Elements::Eq256 => &simplicity_sys::c_jets::jets_wrapper::eq_256,
            Elements::Eq32 => &simplicity_sys::c_jets::jets_wrapper::eq_32,
            Elements::Eq64 => &simplicity_sys::c_jets::jets_wrapper::eq_64,
            Elements::Eq8 => &simplicity_sys::c_jets::jets_wrapper::eq_8,
            Elements::FeAdd => &simplicity_sys::c_jets::jets_wrapper::fe_add,
            Elements::FeInvert => &simplicity_sys::c_jets::jets_wrapper::fe_invert,
            Elements::FeIsOdd => &simplicity_sys::c_jets::jets_wrapper::fe_is_odd,
            Elements::FeIsZero => &simplicity_sys::c_jets::jets_wrapper::fe_is_zero,
            Elements::FeMultiply => &simplicity_sys::c_jets::jets_wrapper::fe_multiply,
            Elements::FeMultiplyBeta => &simplicity_sys::c_jets::jets_wrapper::fe_multiply_beta,
            Elements::FeNegate => &simplicity_sys::c_jets::jets_wrapper::fe_negate,
            Elements::FeNormalize => &simplicity_sys::c_jets::jets_wrapper::fe_normalize,
            Elements::FeSquare => &simplicity_sys::c_jets::jets_wrapper::fe_square,
            Elements::FeSquareRoot => &simplicity_sys::c_jets::jets_wrapper::fe_square_root,
            Elements::FullAdd16 => &simplicity_sys::c_jets::jets_wrapper::full_add_16,
            Elements::FullAdd32 => &simplicity_sys::c_jets::jets_wrapper::full_add_32,
            Elements::FullAdd64 => &simplicity_sys::c_jets::jets_wrapper::full_add_64,
            Elements::FullAdd8 => &simplicity_sys::c_jets::jets_wrapper::full_add_8,
            Elements::FullDecrement16 => &simplicity_sys::c_jets::jets_wrapper::full_decrement_16,
            Elements::FullDecrement32 => &simplicity_sys::c_jets::jets_wrapper::full_decrement_32,
            Elements::FullDecrement64 => &simplicity_sys::c_jets::jets_wrapper::full_decrement_64,
            Elements::FullDecrement8 => &simplicity_sys::c_jets::jets_wrapper::full_decrement_8,
            Elements::FullIncrement16 => &simplicity_sys::c_jets::jets_wrapper::full_increment_16,
            Elements::FullIncrement32 => &simplicity_sys::c_jets::jets_wrapper::full_increment_32,
            Elements::FullIncrement64 => &simplicity_sys::c_jets::jets_wrapper::full_increment_64,
            Elements::FullIncrement8 => &simplicity_sys::c_jets::jets_wrapper::full_increment_8,
            Elements::FullLeftShift16_1 => &simplicity_sys::c_jets::jets_wrapper::full_left_shift_16_1,
            Elements::FullLeftShift16_2 => &simplicity_sys::c_jets::jets_wrapper::full_left_shift_16_2,
            Elements::FullLeftShift16_4 => &simplicity_sys::c_jets::jets_wrapper::full_left_shift_16_4,
            Elements::FullLeftShift16_8 => &simplicity_sys::c_jets::jets_wrapper::full_left_shift_16_8,
            Elements::FullLeftShift32_1 => &simplicity_sys::c_jets::jets_wrapper::full_left_shift_32_1,
            Elements::FullLeftShift32_16 => &simplicity_sys::c_jets::jets_wrapper::full_left_shift_32_16,
            Elements::FullLeftShift32_2 => &simplicity_sys::c_jets::jets_wrapper::full_left_shift_32_2,
            Elements::FullLeftShift32_4 => &simplicity_sys::c_jets::jets_wrapper::full_left_shift_32_4,
            Elements::FullLeftShift32_8 => &simplicity_sys::c_jets::jets_wrapper::full_left_shift_32_8,
            Elements::FullLeftShift64_1 => &simplicity_sys::c_jets::jets_wrapper::full_left_shift_64_1,
            Elements::FullLeftShift64_16 => &simplicity_sys::c_jets::jets_wrapper::full_left_shift_64_16,
            Elements::FullLeftShift64_2 => &simplicity_sys::c_jets::jets_wrapper::full_left_shift_64_2,
            Elements::FullLeftShift64_32 => &simplicity_sys::c_jets::jets_wrapper::full_left_shift_64_32,
            Elements::FullLeftShift64_4 => &simplicity_sys::c_jets::jets_wrapper::full_left_shift_64_4,
            Elements::FullLeftShift64_8 => &simplicity_sys::c_jets::jets_wrapper::full_left_shift_64_8,
            Elements::FullLeftShift8_1 => &simplicity_sys::c_jets::jets_wrapper::full_left_shift_8_1,
            Elements::FullLeftShift8_2 => &simplicity_sys::c_jets::jets_wrapper::full_left_shift_8_2,
            Elements::FullLeftShift8_4 => &simplicity_sys::c_jets::jets_wrapper::full_left_shift_8_4,
            Elements::FullMultiply16 => &simplicity_sys::c_jets::jets_wrapper::full_multiply_16,
            Elements::FullMultiply32 => &simplicity_sys::c_jets::jets_wrapper::full_multiply_32,
            Elements::FullMultiply64 => &simplicity_sys::c_jets::jets_wrapper::full_multiply_64,
            Elements::FullMultiply8 => &simplicity_sys::c_jets::jets_wrapper::full_multiply_8,
            Elements::FullRightShift16_1 => &simplicity_sys::c_jets::jets_wrapper::full_right_shift_16_1,
            Elements::FullRightShift16_2 => &simplicity_sys::c_jets::jets_wrapper::full_right_shift_16_2,
            Elements::FullRightShift16_4 => &simplicity_sys::c_jets::jets_wrapper::full_right_shift_16_4,
            Elements::FullRightShift16_8 => &simplicity_sys::c_jets::jets_wrapper::full_right_shift_16_8,
            Elements::FullRightShift32_1 => &simplicity_sys::c_jets::jets_wrapper::full_right_shift_32_1,
            Elements::FullRightShift32_16 => &simplicity_sys::c_jets::jets_wrapper::full_right_shift_32_16,
            Elements::FullRightShift32_2 => &simplicity_sys::c_jets::jets_wrapper::full_right_shift_32_2,
            Elements::FullRightShift32_4 => &simplicity_sys::c_jets::jets_wrapper::full_right_shift_32_4,
            Elements::FullRightShift32_8 => &simplicity_sys::c_jets::jets_wrapper::full_right_shift_32_8,
            Elements::FullRightShift64_1 => &simplicity_sys::c_jets::jets_wrapper::full_right_shift_64_1,
            Elements::FullRightShift64_16 => &simplicity_sys::c_jets::jets_wrapper::full_right_shift_64_16,
            Elements::FullRightShift64_2 => &simplicity_sys::c_jets::jets_wrapper::full_right_shift_64_2,
            Elements::FullRightShift64_32 => &simplicity_sys::c_jets::jets_wrapper::full_right_shift_64_32,
            Elements::FullRightShift64_4 => &simplicity_sys::c_jets::jets_wrapper::full_right_shift_64_4,
            Elements::FullRightShift64_8 => &simplicity_sys::c_jets::jets_wrapper::full_right_shift_64_8,
            Elements::FullRightShift8_1 => &simplicity_sys::c_jets::jets_wrapper::full_right_shift_8_1,
            Elements::FullRightShift8_2 => &simplicity_sys::c_jets::jets_wrapper::full_right_shift_8_2,
            Elements::FullRightShift8_4 => &simplicity_sys::c_jets::jets_wrapper::full_right_shift_8_4,
            Elements::FullSubtract16 => &simplicity_sys::c_jets::jets_wrapper::full_subtract_16,
            Elements::FullSubtract32 => &simplicity_sys::c_jets::jets_wrapper::full_subtract_32,
            Elements::FullSubtract64 => &simplicity_sys::c_jets::jets_wrapper::full_subtract_64,
            Elements::FullSubtract8 => &simplicity_sys::c_jets::jets_wrapper::full_subtract_8,
            Elements::GeIsOnCurve => &simplicity_sys::c_jets::jets_wrapper::ge_is_on_curve,
            Elements::GeNegate => &simplicity_sys::c_jets::jets_wrapper::ge_negate,
            Elements::GejAdd => &simplicity_sys::c_jets::jets_wrapper::gej_add,
            Elements::GejDouble => &simplicity_sys::c_jets::jets_wrapper::gej_double,
            Elements::GejEquiv => &simplicity_sys::c_jets::jets_wrapper::gej_equiv,
            Elements::GejGeAdd => &simplicity_sys::c_jets::jets_wrapper::gej_ge_add,
            Elements::GejGeAddEx => &simplicity_sys::c_jets::jets_wrapper::gej_ge_add_ex,
            Elements::GejGeEquiv => &simplicity_sys::c_jets::jets_wrapper::gej_ge_equiv,
            Elements::GejInfinity => &simplicity_sys::c_jets::jets_wrapper::gej_infinity,
            Elements::GejIsInfinity => &simplicity_sys::c_jets::jets_wrapper::gej_is_infinity,
            Elements::GejIsOnCurve => &simplicity_sys::c_jets::jets_wrapper::gej_is_on_curve,
            Elements::GejNegate => &simplicity_sys::c_jets::jets_wrapper::gej_negate,
            Elements::GejNormalize => &simplicity_sys::c_jets::jets_wrapper::gej_normalize,
            Elements::GejRescale => &simplicity_sys::c_jets::jets_wrapper::gej_rescale,
            Elements::GejXEquiv => &simplicity_sys::c_jets::jets_wrapper::gej_x_equiv,
            Elements::GejYIsOdd => &simplicity_sys::c_jets::jets_wrapper::gej_y_is_odd,
            Elements::Generate => &simplicity_sys::c_jets::jets_wrapper::generate,
            Elements::GenesisBlockHash => &simplicity_sys::c_jets::jets_wrapper::genesis_block_hash,
            Elements::HashToCurve => &simplicity_sys::c_jets::jets_wrapper::hash_to_curve,
            Elements::High1 => &simplicity_sys::c_jets::jets_wrapper::high_1,
            Elements::High16 => &simplicity_sys::c_jets::jets_wrapper::high_16,
            Elements::High32 => &simplicity_sys::c_jets::jets_wrapper::high_32,
            Elements::High64 => &simplicity_sys::c_jets::jets_wrapper::high_64,
            Elements::High8 => &simplicity_sys::c_jets::jets_wrapper::high_8,
            Elements::Increment16 => &simplicity_sys::c_jets::jets_wrapper::increment_16,
            Elements::Increment32 => &simplicity_sys::c_jets::jets_wrapper::increment_32,
            Elements::Increment64 => &simplicity_sys::c_jets::jets_wrapper::increment_64,
            Elements::Increment8 => &simplicity_sys::c_jets::jets_wrapper::increment_8,
            Elements::InputAmount => &simplicity_sys::c_jets::jets_wrapper::input_amount,
            Elements::InputAmountsHash => &simplicity_sys::c_jets::jets_wrapper::input_amounts_hash,
            Elements::InputAnnexHash => &simplicity_sys::c_jets::jets_wrapper::input_annex_hash,
            Elements::InputAnnexesHash => &simplicity_sys::c_jets::jets_wrapper::input_annexes_hash,
            Elements::InputAsset => &simplicity_sys::c_jets::jets_wrapper::input_asset,
            Elements::InputHash => &simplicity_sys::c_jets::jets_wrapper::input_hash,
            Elements::InputOutpointsHash => &simplicity_sys::c_jets::jets_wrapper::input_outpoints_hash,
            Elements::InputPegin => &simplicity_sys::c_jets::jets_wrapper::input_pegin,
            Elements::InputPrevOutpoint => &simplicity_sys::c_jets::jets_wrapper::input_prev_outpoint,
            Elements::InputScriptHash => &simplicity_sys::c_jets::jets_wrapper::input_script_hash,
            Elements::InputScriptSigHash => &simplicity_sys::c_jets::jets_wrapper::input_script_sig_hash,
            Elements::InputScriptSigsHash => &simplicity_sys::c_jets::jets_wrapper::input_script_sigs_hash,
            Elements::InputScriptsHash => &simplicity_sys::c_jets::jets_wrapper::input_scripts_hash,
            Elements::InputSequence => &simplicity_sys::c_jets::jets_wrapper::input_sequence,
            Elements::InputSequencesHash => &simplicity_sys::c_jets::jets_wrapper::input_sequences_hash,
            Elements::InputUtxoHash => &simplicity_sys::c_jets::jets_wrapper::input_utxo_hash,
            Elements::InputUtxosHash => &simplicity_sys::c_jets::jets_wrapper::input_utxos_hash,
            Elements::InputsHash => &simplicity_sys::c_jets::jets_wrapper::inputs_hash,
            Elements::InternalKey => &simplicity_sys::c_jets::jets_wrapper::internal_key,
            Elements::IsOne16 => &simplicity_sys::c_jets::jets_wrapper::is_one_16,
            Elements::IsOne32 => &simplicity_sys::c_jets::jets_wrapper::is_one_32,
            Elements::IsOne64 => &simplicity_sys::c_jets::jets_wrapper::is_one_64,
            Elements::IsOne8 => &simplicity_sys::c_jets::jets_wrapper::is_one_8,
            Elements::IsZero16 => &simplicity_sys::c_jets::jets_wrapper::is_zero_16,
            Elements::IsZero32 => &simplicity_sys::c_jets::jets_wrapper::is_zero_32,
            Elements::IsZero64 => &simplicity_sys::c_jets::jets_wrapper::is_zero_64,
            Elements::IsZero8 => &simplicity_sys::c_jets::jets_wrapper::is_zero_8,
            Elements::Issuance => &simplicity_sys::c_jets::jets_wrapper::issuance,
            Elements::IssuanceAsset => &simplicity_sys::c_jets::jets_wrapper::issuance_asset,
            Elements::IssuanceAssetAmount => &simplicity_sys::c_jets::jets_wrapper::issuance_asset_amount,
            Elements::IssuanceAssetAmountsHash => &simplicity_sys::c_jets::jets_wrapper::issuance_asset_amounts_hash,
            Elements::IssuanceAssetProof => &simplicity_sys::c_jets::jets_wrapper::issuance_asset_proof,
            Elements::IssuanceBlindingEntropyHash => &simplicity_sys::c_jets::jets_wrapper::issuance_blinding_entropy_hash,
            Elements::IssuanceEntropy => &simplicity_sys::c_jets::jets_wrapper::issuance_entropy,
            Elements::IssuanceHash => &simplicity_sys::c_jets::jets_wrapper::issuance_hash,
            Elements::IssuanceRangeProofsHash => &simplicity_sys::c_jets::jets_wrapper::issuance_range_proofs_hash,
            Elements::IssuanceToken => &simplicity_sys::c_jets::jets_wrapper::issuance_token,
            Elements::IssuanceTokenAmount => &simplicity_sys::c_jets::jets_wrapper::issuance_token_amount,
            Elements::IssuanceTokenAmountsHash => &simplicity_sys::c_jets::jets_wrapper::issuance_token_amounts_hash,
            Elements::IssuanceTokenProof => &simplicity_sys::c_jets::jets_wrapper::issuance_token_proof,
            Elements::IssuancesHash => &simplicity_sys::c_jets::jets_wrapper::issuances_hash,
            Elements::LbtcAsset => &simplicity_sys::c_jets::jets_wrapper::lbtc_asset,
            Elements::Le16 => &simplicity_sys::c_jets::jets_wrapper::le_16,
            Elements::Le32 => &simplicity_sys::c_jets::jets_wrapper::le_32,
            Elements::Le64 => &simplicity_sys::c_jets::jets_wrapper::le_64,
            Elements::Le8 => &simplicity_sys::c_jets::jets_wrapper::le_8,
            Elements::LeftExtend16_32 => &simplicity_sys::c_jets::jets_wrapper::left_extend_16_32,
            Elements::LeftExtend16_64 => &simplicity_sys::c_jets::jets_wrapper::left_extend_16_64,
            Elements::LeftExtend1_16 => &simplicity_sys::c_jets::jets_wrapper::left_extend_1_16,
            Elements::LeftExtend1_32 => &simplicity_sys::c_jets::jets_wrapper::left_extend_1_32,
            Elements::LeftExtend1_64 => &simplicity_sys::c_jets::jets_wrapper::left_extend_1_64,
            Elements::LeftExtend1_8 => &simplicity_sys::c_jets::jets_wrapper::left_extend_1_8,
            Elements::LeftExtend32_64 => &simplicity_sys::c_jets::jets_wrapper::left_extend_32_64,
            Elements::LeftExtend8_16 => &simplicity_sys::c_jets::jets_wrapper::left_extend_8_16,
            Elements::LeftExtend8_32 => &simplicity_sys::c_jets::jets_wrapper::left_extend_8_32,
            Elements::LeftExtend8_64 => &simplicity_sys::c_jets::jets_wrapper::left_extend_8_64,
            Elements::LeftPadHigh16_32 => &simplicity_sys::c_jets::jets_wrapper::left_pad_high_16_32,
            Elements::LeftPadHigh16_64 => &simplicity_sys::c_jets::jets_wrapper::left_pad_high_16_64,
            Elements::LeftPadHigh1_16 => &simplicity_sys::c_jets::jets_wrapper::left_pad_high_1_16,
            Elements::LeftPadHigh1_32 => &simplicity_sys::c_jets::jets_wrapper::left_pad_high_1_32,
            Elements::LeftPadHigh1_64 => &simplicity_sys::c_jets::jets_wrapper::left_pad_high_1_64,
            Elements::LeftPadHigh1_8 => &simplicity_sys::c_jets::jets_wrapper::left_pad_high_1_8,
            Elements::LeftPadHigh32_64 => &simplicity_sys::c_jets::jets_wrapper::left_pad_high_32_64,
            Elements::LeftPadHigh8_16 => &simplicity_sys::c_jets::jets_wrapper::left_pad_high_8_16,
            Elements::LeftPadHigh8_32 => &simplicity_sys::c_jets::jets_wrapper::left_pad_high_8_32,
            Elements::LeftPadHigh8_64 => &simplicity_sys::c_jets::jets_wrapper::left_pad_high_8_64,
            Elements::LeftPadLow16_32 => &simplicity_sys::c_jets::jets_wrapper::left_pad_low_16_32,
            Elements::LeftPadLow16_64 => &simplicity_sys::c_jets::jets_wrapper::left_pad_low_16_64,
            Elements::LeftPadLow1_16 => &simplicity_sys::c_jets::jets_wrapper::left_pad_low_1_16,
            Elements::LeftPadLow1_32 => &simplicity_sys::c_jets::jets_wrapper::left_pad_low_1_32,
            Elements::LeftPadLow1_64 => &simplicity_sys::c_jets::jets_wrapper::left_pad_low_1_64,
            Elements::LeftPadLow1_8 => &simplicity_sys::c_jets::jets_wrapper::left_pad_low_1_8,
            Elements::LeftPadLow32_64 => &simplicity_sys::c_jets::jets_wrapper::left_pad_low_32_64,
            Elements::LeftPadLow8_16 => &simplicity_sys::c_jets::jets_wrapper::left_pad_low_8_16,
            Elements::LeftPadLow8_32 => &simplicity_sys::c_jets::jets_wrapper::left_pad_low_8_32,
            Elements::LeftPadLow8_64 => &simplicity_sys::c_jets::jets_wrapper::left_pad_low_8_64,
            Elements::LeftRotate16 => &simplicity_sys::c_jets::jets_wrapper::left_rotate_16,
            Elements::LeftRotate32 => &simplicity_sys::c_jets::jets_wrapper::left_rotate_32,
            Elements::LeftRotate64 => &simplicity_sys::c_jets::jets_wrapper::left_rotate_64,
            Elements::LeftRotate8 => &simplicity_sys::c_jets::jets_wrapper::left_rotate_8,
            Elements::LeftShift16 => &simplicity_sys::c_jets::jets_wrapper::left_shift_16,
            Elements::LeftShift32 => &simplicity_sys::c_jets::jets_wrapper::left_shift_32,
            Elements::LeftShift64 => &simplicity_sys::c_jets::jets_wrapper::left_shift_64,
            Elements::LeftShift8 => &simplicity_sys::c_jets::jets_wrapper::left_shift_8,
            Elements::LeftShiftWith16 => &simplicity_sys::c_jets::jets_wrapper::left_shift_with_16,
            Elements::LeftShiftWith32 => &simplicity_sys::c_jets::jets_wrapper::left_shift_with_32,
            Elements::LeftShiftWith64 => &simplicity_sys::c_jets::jets_wrapper::left_shift_with_64,
            Elements::LeftShiftWith8 => &simplicity_sys::c_jets::jets_wrapper::left_shift_with_8,
            Elements::Leftmost16_1 => &simplicity_sys::c_jets::jets_wrapper::leftmost_16_1,
            Elements::Leftmost16_2 => &simplicity_sys::c_jets::jets_wrapper::leftmost_16_2,
            Elements::Leftmost16_4 => &simplicity_sys::c_jets::jets_wrapper::leftmost_16_4,
            Elements::Leftmost16_8 => &simplicity_sys::c_jets::jets_wrapper::leftmost_16_8,
            Elements::Leftmost32_1 => &simplicity_sys::c_jets::jets_wrapper::leftmost_32_1,
            Elements::Leftmost32_16 => &simplicity_sys::c_jets::jets_wrapper::leftmost_32_16,
            Elements::Leftmost32_2 => &simplicity_sys::c_jets::jets_wrapper::leftmost_32_2,
            Elements::Leftmost32_4 => &simplicity_sys::c_jets::jets_wrapper::leftmost_32_4,
            Elements::Leftmost32_8 => &simplicity_sys::c_jets::jets_wrapper::leftmost_32_8,
            Elements::Leftmost64_1 => &simplicity_sys::c_jets::jets_wrapper::leftmost_64_1,
            Elements::Leftmost64_16 => &simplicity_sys::c_jets::jets_wrapper::leftmost_64_16,
            Elements::Leftmost64_2 => &simplicity_sys::c_jets::jets_wrapper::leftmost_64_2,
            Elements::Leftmost64_32 => &simplicity_sys::c_jets::jets_wrapper::leftmost_64_32,
            Elements::Leftmost64_4 => &simplicity_sys::c_jets::jets_wrapper::leftmost_64_4,
            Elements::Leftmost64_8 => &simplicity_sys::c_jets::jets_wrapper::leftmost_64_8,
            Elements::Leftmost8_1 => &simplicity_sys::c_jets::jets_wrapper::leftmost_8_1,
            Elements::Leftmost8_2 => &simplicity_sys::c_jets::jets_wrapper::leftmost_8_2,
            Elements::Leftmost8_4 => &simplicity_sys::c_jets::jets_wrapper::leftmost_8_4,
            Elements::LinearCombination1 => &simplicity_sys::c_jets::jets_wrapper::linear_combination_1,
            Elements::LinearVerify1 => &simplicity_sys::c_jets::jets_wrapper::linear_verify_1,
            Elements::LockTime => &simplicity_sys::c_jets::jets_wrapper::lock_time,
            Elements::Low1 => &simplicity_sys::c_jets::jets_wrapper::low_1,
            Elements::Low16 => &simplicity_sys::c_jets::jets_wrapper::low_16,
            Elements::Low32 => &simplicity_sys::c_jets::jets_wrapper::low_32,
            Elements::Low64 => &simplicity_sys::c_jets::jets_wrapper::low_64,
            Elements::Low8 => &simplicity_sys::c_jets::jets_wrapper::low_8,
            Elements::Lt16 => &simplicity_sys::c_jets::jets_wrapper::lt_16,
            Elements::Lt32 => &simplicity_sys::c_jets::jets_wrapper::lt_32,
            Elements::Lt64 => &simplicity_sys::c_jets::jets_wrapper::lt_64,
            Elements::Lt8 => &simplicity_sys::c_jets::jets_wrapper::lt_8,
            Elements::Maj1 => &simplicity_sys::c_jets::jets_wrapper::maj_1,
            Elements::Maj16 => &simplicity_sys::c_jets::jets_wrapper::maj_16,
            Elements::Maj32 => &simplicity_sys::c_jets::jets_wrapper::maj_32,
            Elements::Maj64 => &simplicity_sys::c_jets::jets_wrapper::maj_64,
            Elements::Maj8 => &simplicity_sys::c_jets::jets_wrapper::maj_8,
            Elements::Max16 => &simplicity_sys::c_jets::jets_wrapper::max_16,
            Elements::Max32 => &simplicity_sys::c_jets::jets_wrapper::max_32,
            Elements::Max64 => &simplicity_sys::c_jets::jets_wrapper::max_64,
            Elements::Max8 => &simplicity_sys::c_jets::jets_wrapper::max_8,
            Elements::Median16 => &simplicity_sys::c_jets::jets_wrapper::median_16,
            Elements::Median32 => &simplicity_sys::c_jets::jets_wrapper::median_32,
            Elements::Median64 => &simplicity_sys::c_jets::jets_wrapper::median_64,
            Elements::Median8 => &simplicity_sys::c_jets::jets_wrapper::median_8,
            Elements::Min16 => &simplicity_sys::c_jets::jets_wrapper::min_16,
            Elements::Min32 => &simplicity_sys::c_jets::jets_wrapper::min_32,
            Elements::Min64 => &simplicity_sys::c_jets::jets_wrapper::min_64,
            Elements::Min8 => &simplicity_sys::c_jets::jets_wrapper::min_8,
            Elements::Modulo16 => &simplicity_sys::c_jets::jets_wrapper::modulo_16,
            Elements::Modulo32 => &simplicity_sys::c_jets::jets_wrapper::modulo_32,
            Elements::Modulo64 => &simplicity_sys::c_jets::jets_wrapper::modulo_64,
            Elements::Modulo8 => &simplicity_sys::c_jets::jets_wrapper::modulo_8,
            Elements::Multiply16 => &simplicity_sys::c_jets::jets_wrapper::multiply_16,
            Elements::Multiply32 => &simplicity_sys::c_jets::jets_wrapper::multiply_32,
            Elements::Multiply64 => &simplicity_sys::c_jets::jets_wrapper::multiply_64,
            Elements::Multiply8 => &simplicity_sys::c_jets::jets_wrapper::multiply_8,
            Elements::Negate16 => &simplicity_sys::c_jets::jets_wrapper::negate_16,
            Elements::Negate32 => &simplicity_sys::c_jets::jets_wrapper::negate_32,
            Elements::Negate64 => &simplicity_sys::c_jets::jets_wrapper::negate_64,
            Elements::Negate8 => &simplicity_sys::c_jets::jets_wrapper::negate_8,
            Elements::NewIssuanceContract => &simplicity_sys::c_jets::jets_wrapper::new_issuance_contract,
            Elements::NonceHash => &simplicity_sys::c_jets::jets_wrapper::nonce_hash,
            Elements::NumInputs => &simplicity_sys::c_jets::jets_wrapper::num_inputs,
            Elements::NumOutputs => &simplicity_sys::c_jets::jets_wrapper::num_outputs,
            Elements::One16 => &simplicity_sys::c_jets::jets_wrapper::one_16,
            Elements::One32 => &simplicity_sys::c_jets::jets_wrapper::one_32,
            Elements::One64 => &simplicity_sys::c_jets::jets_wrapper::one_64,
            Elements::One8 => &simplicity_sys::c_jets::jets_wrapper::one_8,
            Elements::Or1 => &simplicity_sys::c_jets::jets_wrapper::or_1,
            Elements::Or16 => &simplicity_sys::c_jets::jets_wrapper::or_16,
            Elements::Or32 => &simplicity_sys::c_jets::jets_wrapper::or_32,
            Elements::Or64 => &simplicity_sys::c_jets::jets_wrapper::or_64,
            Elements::Or8 => &simplicity_sys::c_jets::jets_wrapper::or_8,
            Elements::OutpointHash => &simplicity_sys::c_jets::jets_wrapper::outpoint_hash,
            Elements::OutputAmount => &simplicity_sys::c_jets::jets_wrapper::output_amount,
            Elements::OutputAmountsHash => &simplicity_sys::c_jets::jets_wrapper::output_amounts_hash,
            Elements::OutputAsset => &simplicity_sys::c_jets::jets_wrapper::output_asset,
            Elements::OutputHash => &simplicity_sys::c_jets::jets_wrapper::output_hash,
            Elements::OutputIsFee => &simplicity_sys::c_jets::jets_wrapper::output_is_fee,
            Elements::OutputNonce => &simplicity_sys::c_jets::jets_wrapper::output_nonce,
            Elements::OutputNoncesHash => &simplicity_sys::c_jets::jets_wrapper::output_nonces_hash,
            Elements::OutputNullDatum => &simplicity_sys::c_jets::jets_wrapper::output_null_datum,
            Elements::OutputRangeProof => &simplicity_sys::c_jets::jets_wrapper::output_range_proof,
            Elements::OutputRangeProofsHash => &simplicity_sys::c_jets::jets_wrapper::output_range_proofs_hash,
            Elements::OutputScriptHash => &simplicity_sys::c_jets::jets_wrapper::output_script_hash,
            Elements::OutputScriptsHash => &simplicity_sys::c_jets::jets_wrapper::output_scripts_hash,
            Elements::OutputSurjectionProof => &simplicity_sys::c_jets::jets_wrapper::output_surjection_proof,
            Elements::OutputSurjectionProofsHash => &simplicity_sys::c_jets::jets_wrapper::output_surjection_proofs_hash,
            Elements::OutputsHash => &simplicity_sys::c_jets::jets_wrapper::outputs_hash,
            Elements::ParseLock => &simplicity_sys::c_jets::jets_wrapper::parse_lock,
            Elements::ParseSequence => &simplicity_sys::c_jets::jets_wrapper::parse_sequence,
            Elements::PointVerify1 => &simplicity_sys::c_jets::jets_wrapper::point_verify_1,
            Elements::ReissuanceBlinding => &simplicity_sys::c_jets::jets_wrapper::reissuance_blinding,
            Elements::ReissuanceEntropy => &simplicity_sys::c_jets::jets_wrapper::reissuance_entropy,
            Elements::RightExtend16_32 => &simplicity_sys::c_jets::jets_wrapper::right_extend_16_32,
            Elements::RightExtend16_64 => &simplicity_sys::c_jets::jets_wrapper::right_extend_16_64,
            Elements::RightExtend32_64 => &simplicity_sys::c_jets::jets_wrapper::right_extend_32_64,
            Elements::RightExtend8_16 => &simplicity_sys::c_jets::jets_wrapper::right_extend_8_16,
            Elements::RightExtend8_32 => &simplicity_sys::c_jets::jets_wrapper::right_extend_8_32,
            Elements::RightExtend8_64 => &simplicity_sys::c_jets::jets_wrapper::right_extend_8_64,
            Elements::RightPadHigh16_32 => &simplicity_sys::c_jets::jets_wrapper::right_pad_high_16_32,
            Elements::RightPadHigh16_64 => &simplicity_sys::c_jets::jets_wrapper::right_pad_high_16_64,
            Elements::RightPadHigh1_16 => &simplicity_sys::c_jets::jets_wrapper::right_pad_high_1_16,
            Elements::RightPadHigh1_32 => &simplicity_sys::c_jets::jets_wrapper::right_pad_high_1_32,
            Elements::RightPadHigh1_64 => &simplicity_sys::c_jets::jets_wrapper::right_pad_high_1_64,
            Elements::RightPadHigh1_8 => &simplicity_sys::c_jets::jets_wrapper::right_pad_high_1_8,
            Elements::RightPadHigh32_64 => &simplicity_sys::c_jets::jets_wrapper::right_pad_high_32_64,
            Elements::RightPadHigh8_16 => &simplicity_sys::c_jets::jets_wrapper::right_pad_high_8_16,
            Elements::RightPadHigh8_32 => &simplicity_sys::c_jets::jets_wrapper::right_pad_high_8_32,
            Elements::RightPadHigh8_64 => &simplicity_sys::c_jets::jets_wrapper::right_pad_high_8_64,
            Elements::RightPadLow16_32 => &simplicity_sys::c_jets::jets_wrapper::right_pad_low_16_32,
            Elements::RightPadLow16_64 => &simplicity_sys::c_jets::jets_wrapper::right_pad_low_16_64,
            Elements::RightPadLow1_16 => &simplicity_sys::c_jets::jets_wrapper::right_pad_low_1_16,
            Elements::RightPadLow1_32 => &simplicity_sys::c_jets::jets_wrapper::right_pad_low_1_32,
            Elements::RightPadLow1_64 => &simplicity_sys::c_jets::jets_wrapper::right_pad_low_1_64,
            Elements::RightPadLow1_8 => &simplicity_sys::c_jets::jets_wrapper::right_pad_low_1_8,
            Elements::RightPadLow32_64 => &simplicity_sys::c_jets::jets_wrapper::right_pad_low_32_64,
            Elements::RightPadLow8_16 => &simplicity_sys::c_jets::jets_wrapper::right_pad_low_8_16,
            Elements::RightPadLow8_32 => &simplicity_sys::c_jets::jets_wrapper::right_pad_low_8_32,
            Elements::RightPadLow8_64 => &simplicity_sys::c_jets::jets_wrapper::right_pad_low_8_64,
            Elements::RightRotate16 => &simplicity_sys::c_jets::jets_wrapper::right_rotate_16,
            Elements::RightRotate32 => &simplicity_sys::c_jets::jets_wrapper::right_rotate_32,
            Elements::RightRotate64 => &simplicity_sys::c_jets::jets_wrapper::right_rotate_64,
            Elements::RightRotate8 => &simplicity_sys::c_jets::jets_wrapper::right_rotate_8,
            Elements::RightShift16 => &simplicity_sys::c_jets::jets_wrapper::right_shift_16,
            Elements::RightShift32 => &simplicity_sys::c_jets::jets_wrapper::right_shift_32,
            Elements::RightShift64 => &simplicity_sys::c_jets::jets_wrapper::right_shift_64,
            Elements::RightShift8 => &simplicity_sys::c_jets::jets_wrapper::right_shift_8,
            Elements::RightShiftWith16 => &simplicity_sys::c_jets::jets_wrapper::right_shift_with_16,
            Elements::RightShiftWith32 => &simplicity_sys::c_jets::jets_wrapper::right_shift_with_32,
            Elements::RightShiftWith64 => &simplicity_sys::c_jets::jets_wrapper::right_shift_with_64,
            Elements::RightShiftWith8 => &simplicity_sys::c_jets::jets_wrapper::right_shift_with_8,
            Elements::Rightmost16_1 => &simplicity_sys::c_jets::jets_wrapper::rightmost_16_1,
            Elements::Rightmost16_2 => &simplicity_sys::c_jets::jets_wrapper::rightmost_16_2,
            Elements::Rightmost16_4 => &simplicity_sys::c_jets::jets_wrapper::rightmost_16_4,
            Elements::Rightmost16_8 => &simplicity_sys::c_jets::jets_wrapper::rightmost_16_8,
            Elements::Rightmost32_1 => &simplicity_sys::c_jets::jets_wrapper::rightmost_32_1,
            Elements::Rightmost32_16 => &simplicity_sys::c_jets::jets_wrapper::rightmost_32_16,
            Elements::Rightmost32_2 => &simplicity_sys::c_jets::jets_wrapper::rightmost_32_2,
            Elements::Rightmost32_4 => &simplicity_sys::c_jets::jets_wrapper::rightmost_32_4,
            Elements::Rightmost32_8 => &simplicity_sys::c_jets::jets_wrapper::rightmost_32_8,
            Elements::Rightmost64_1 => &simplicity_sys::c_jets::jets_wrapper::rightmost_64_1,
            Elements::Rightmost64_16 => &simplicity_sys::c_jets::jets_wrapper::rightmost_64_16,
            Elements::Rightmost64_2 => &simplicity_sys::c_jets::jets_wrapper::rightmost_64_2,
            Elements::Rightmost64_32 => &simplicity_sys::c_jets::jets_wrapper::rightmost_64_32,
            Elements::Rightmost64_4 => &simplicity_sys::c_jets::jets_wrapper::rightmost_64_4,
            Elements::Rightmost64_8 => &simplicity_sys::c_jets::jets_wrapper::rightmost_64_8,
            Elements::Rightmost8_1 => &simplicity_sys::c_jets::jets_wrapper::rightmost_8_1,
            Elements::Rightmost8_2 => &simplicity_sys::c_jets::jets_wrapper::rightmost_8_2,
            Elements::Rightmost8_4 => &simplicity_sys::c_jets::jets_wrapper::rightmost_8_4,
            Elements::ScalarAdd => &simplicity_sys::c_jets::jets_wrapper::scalar_add,
            Elements::ScalarInvert => &simplicity_sys::c_jets::jets_wrapper::scalar_invert,
            Elements::ScalarIsZero => &simplicity_sys::c_jets::jets_wrapper::scalar_is_zero,
            Elements::ScalarMultiply => &simplicity_sys::c_jets::jets_wrapper::scalar_multiply,
            Elements::ScalarMultiplyLambda => &simplicity_sys::c_jets::jets_wrapper::scalar_multiply_lambda,
            Elements::ScalarNegate => &simplicity_sys::c_jets::jets_wrapper::scalar_negate,
            Elements::ScalarNormalize => &simplicity_sys::c_jets::jets_wrapper::scalar_normalize,
            Elements::ScalarSquare => &simplicity_sys::c_jets::jets_wrapper::scalar_square,
            Elements::Scale => &simplicity_sys::c_jets::jets_wrapper::scale,
            Elements::ScriptCMR => &simplicity_sys::c_jets::jets_wrapper::script_cmr,
            Elements::Sha256Block => &simplicity_sys::c_jets::jets_wrapper::sha_256_block,
            Elements::Sha256Ctx8Add1 => &simplicity_sys::c_jets::jets_wrapper::sha_256_ctx_8_add_1,
            Elements::Sha256Ctx8Add128 => &simplicity_sys::c_jets::jets_wrapper::sha_256_ctx_8_add_128,
            Elements::Sha256Ctx8Add16 => &simplicity_sys::c_jets::jets_wrapper::sha_256_ctx_8_add_16,
            Elements::Sha256Ctx8Add2 => &simplicity_sys::c_jets::jets_wrapper::sha_256_ctx_8_add_2,
            Elements::Sha256Ctx8Add256 => &simplicity_sys::c_jets::jets_wrapper::sha_256_ctx_8_add_256,
            Elements::Sha256Ctx8Add32 => &simplicity_sys::c_jets::jets_wrapper::sha_256_ctx_8_add_32,
            Elements::Sha256Ctx8Add4 => &simplicity_sys::c_jets::jets_wrapper::sha_256_ctx_8_add_4,
            Elements::Sha256Ctx8Add512 => &simplicity_sys::c_jets::jets_wrapper::sha_256_ctx_8_add_512,
            Elements::Sha256Ctx8Add64 => &simplicity_sys::c_jets::jets_wrapper::sha_256_ctx_8_add_64,
            Elements::Sha256Ctx8Add8 => &simplicity_sys::c_jets::jets_wrapper::sha_256_ctx_8_add_8,
            Elements::Sha256Ctx8AddBuffer511 => &simplicity_sys::c_jets::jets_wrapper::sha_256_ctx_8_add_buffer_511,
            Elements::Sha256Ctx8Finalize => &simplicity_sys::c_jets::jets_wrapper::sha_256_ctx_8_finalize,
            Elements::Sha256Ctx8Init => &simplicity_sys::c_jets::jets_wrapper::sha_256_ctx_8_init,
            Elements::Sha256Iv => &simplicity_sys::c_jets::jets_wrapper::sha_256_iv,
            Elements::SigAllHash => &simplicity_sys::c_jets::jets_wrapper::sig_all_hash,
            Elements::Some1 => &simplicity_sys::c_jets::jets_wrapper::some_1,
            Elements::Some16 => &simplicity_sys::c_jets::jets_wrapper::some_16,
            Elements::Some32 => &simplicity_sys::c_jets::jets_wrapper::some_32,
            Elements::Some64 => &simplicity_sys::c_jets::jets_wrapper::some_64,
            Elements::Some8 => &simplicity_sys::c_jets::jets_wrapper::some_8,
            Elements::Subtract16 => &simplicity_sys::c_jets::jets_wrapper::subtract_16,
            Elements::Subtract32 => &simplicity_sys::c_jets::jets_wrapper::subtract_32,
            Elements::Subtract64 => &simplicity_sys::c_jets::jets_wrapper::subtract_64,
            Elements::Subtract8 => &simplicity_sys::c_jets::jets_wrapper::subtract_8,
            Elements::Swu => &simplicity_sys::c_jets::jets_wrapper::swu,
            Elements::TapEnvHash => &simplicity_sys::c_jets::jets_wrapper::tap_env_hash,
            Elements::TapleafHash => &simplicity_sys::c_jets::jets_wrapper::tapleaf_hash,
            Elements::TapleafVersion => &simplicity_sys::c_jets::jets_wrapper::tapleaf_version,
            Elements::Tappath => &simplicity_sys::c_jets::jets_wrapper::tappath,
            Elements::TappathHash => &simplicity_sys::c_jets::jets_wrapper::tappath_hash,
            Elements::TotalFee => &simplicity_sys::c_jets::jets_wrapper::total_fee,
            Elements::TransactionId => &simplicity_sys::c_jets::jets_wrapper::transaction_id,
            Elements::TxHash => &simplicity_sys::c_jets::jets_wrapper::tx_hash,
            Elements::TxIsFinal => &simplicity_sys::c_jets::jets_wrapper::tx_is_final,
            Elements::TxLockDistance => &simplicity_sys::c_jets::jets_wrapper::tx_lock_distance,
            Elements::TxLockDuration => &simplicity_sys::c_jets::jets_wrapper::tx_lock_duration,
            Elements::TxLockHeight => &simplicity_sys::c_jets::jets_wrapper::tx_lock_height,
            Elements::TxLockTime => &simplicity_sys::c_jets::jets_wrapper::tx_lock_time,
            Elements::Verify => &simplicity_sys::c_jets::jets_wrapper::verify,
            Elements::Version => &simplicity_sys::c_jets::jets_wrapper::version,
            Elements::Xor1 => &simplicity_sys::c_jets::jets_wrapper::xor_1,
            Elements::Xor16 => &simplicity_sys::c_jets::jets_wrapper::xor_16,
            Elements::Xor32 => &simplicity_sys::c_jets::jets_wrapper::xor_32,
            Elements::Xor64 => &simplicity_sys::c_jets::jets_wrapper::xor_64,
            Elements::Xor8 => &simplicity_sys::c_jets::jets_wrapper::xor_8,
            Elements::XorXor1 => &simplicity_sys::c_jets::jets_wrapper::xor_xor_1,
            Elements::XorXor16 => &simplicity_sys::c_jets::jets_wrapper::xor_xor_16,
            Elements::XorXor32 => &simplicity_sys::c_jets::jets_wrapper::xor_xor_32,
            Elements::XorXor64 => &simplicity_sys::c_jets::jets_wrapper::xor_xor_64,
            Elements::XorXor8 => &simplicity_sys::c_jets::jets_wrapper::xor_xor_8,
        }
    }

    fn cost(&self) -> Cost {
        match self {
            Elements::Add16 => Cost::from_milliweight(226),
            Elements::Add32 => Cost::from_milliweight(183),
            Elements::Add64 => Cost::from_milliweight(221),
            Elements::Add8 => Cost::from_milliweight(150),
            Elements::All16 => Cost::from_milliweight(110),
            Elements::All32 => Cost::from_milliweight(136),
            Elements::All64 => Cost::from_milliweight(165),
            Elements::All8 => Cost::from_milliweight(113),
            Elements::And1 => Cost::from_milliweight(159),
            Elements::And16 => Cost::from_milliweight(195),
            Elements::And32 => Cost::from_milliweight(175),
            Elements::And64 => Cost::from_milliweight(221),
            Elements::And8 => Cost::from_milliweight(159),
            Elements::AnnexHash => Cost::from_milliweight(3065),
            Elements::AssetAmountHash => Cost::from_milliweight(3177),
            Elements::Bip0340Verify => Cost::from_milliweight(49671),
            Elements::BuildTapbranch => Cost::from_milliweight(4804),
            Elements::BuildTapleafSimplicity => Cost::from_milliweight(3670),
            Elements::CalculateAsset => Cost::from_milliweight(1516),
            Elements::CalculateConfidentialToken => Cost::from_milliweight(1727),
            Elements::CalculateExplicitToken => Cost::from_milliweight(1610),
            Elements::CalculateIssuanceEntropy => Cost::from_milliweight(3453),
            Elements::Ch1 => Cost::from_milliweight(240),
            Elements::Ch16 => Cost::from_milliweight(245),
            Elements::Ch32 => Cost::from_milliweight(238),
            Elements::Ch64 => Cost::from_milliweight(274),
            Elements::Ch8 => Cost::from_milliweight(240),
            Elements::CheckLockDistance => Cost::from_milliweight(198),
            Elements::CheckLockDuration => Cost::from_milliweight(201),
            Elements::CheckLockHeight => Cost::from_milliweight(239),
            Elements::CheckLockTime => Cost::from_milliweight(232),
            Elements::CheckSigVerify => Cost::from_milliweight(50000),
            Elements::Complement1 => Cost::from_milliweight(139),
            Elements::Complement16 => Cost::from_milliweight(146),
            Elements::Complement32 => Cost::from_milliweight(161),
            Elements::Complement64 => Cost::from_milliweight(174),
            Elements::Complement8 => Cost::from_milliweight(139),
            Elements::CurrentAmount => Cost::from_milliweight(400),
            Elements::CurrentAnnexHash => Cost::from_milliweight(137),
            Elements::CurrentAsset => Cost::from_milliweight(320),
            Elements::CurrentIndex => Cost::from_milliweight(137),
            Elements::CurrentIssuanceAssetAmount => Cost::from_milliweight(317),
            Elements::CurrentIssuanceAssetProof => Cost::from_milliweight(301),
            Elements::CurrentIssuanceTokenAmount => Cost::from_milliweight(326),
            Elements::CurrentIssuanceTokenProof => Cost::from_milliweight(288),
            Elements::CurrentNewIssuanceContract => Cost::from_milliweight(293),
            Elements::CurrentPegin => Cost::from_milliweight(303),
            Elements::CurrentPrevOutpoint => Cost::from_milliweight(312),
            Elements::CurrentReissuanceBlinding => Cost::from_milliweight(148),
            Elements::CurrentReissuanceEntropy => Cost::from_milliweight(139),
            Elements::CurrentScriptHash => Cost::from_milliweight(305),
            Elements::CurrentScriptSigHash => Cost::from_milliweight(298),
            Elements::CurrentSequence => Cost::from_milliweight(138),
            Elements::Decompress => Cost::from_milliweight(10956),
            Elements::Decrement16 => Cost::from_milliweight(116),
            Elements::Decrement32 => Cost::from_milliweight(159),
            Elements::Decrement64 => Cost::from_milliweight(160),
            Elements::Decrement8 => Cost::from_milliweight(195),
            Elements::DivMod128_64 => Cost::from_milliweight(220),
            Elements::DivMod16 => Cost::from_milliweight(223),
            Elements::DivMod32 => Cost::from_milliweight(198),
            Elements::DivMod64 => Cost::from_milliweight(220),
            Elements::DivMod8 => Cost::from_milliweight(141),
            Elements::Divide16 => Cost::from_milliweight(188),
            Elements::Divide32 => Cost::from_milliweight(225),
            Elements::Divide64 => Cost::from_milliweight(202),
            Elements::Divide8 => Cost::from_milliweight(125),
            Elements::Divides16 => Cost::from_milliweight(173),
            Elements::Divides32 => Cost::from_milliweight(175),
            Elements::Divides64 => Cost::from_milliweight(246),
            Elements::Divides8 => Cost::from_milliweight(142),
            Elements::Eq1 => Cost::from_milliweight(120),
            Elements::Eq16 => Cost::from_milliweight(174),
            Elements::Eq256 => Cost::from_milliweight(431),
            Elements::Eq32 => Cost::from_milliweight(233),
            Elements::Eq64 => Cost::from_milliweight(202),
            Elements::Eq8 => Cost::from_milliweight(120),
            Elements::FeAdd => Cost::from_milliweight(908),
            Elements::FeInvert => Cost::from_milliweight(3375),
            Elements::FeIsOdd => Cost::from_milliweight(544),
            Elements::FeIsZero => Cost::from_milliweight(521),
            Elements::FeMultiply => Cost::from_milliweight(975),
            Elements::FeMultiplyBeta => Cost::from_milliweight(824),
            Elements::FeNegate => Cost::from_milliweight(846),
            Elements::FeNormalize => Cost::from_milliweight(813),
            Elements::FeSquare => Cost::from_milliweight(829),
            Elements::FeSquareRoot => Cost::from_milliweight(10698),
            Elements::FullAdd16 => Cost::from_milliweight(193),
            Elements::FullAdd32 => Cost::from_milliweight(197),
            Elements::FullAdd64 => Cost::from_milliweight(225),
            Elements::FullAdd8 => Cost::from_milliweight(190),
            Elements::FullDecrement16 => Cost::from_milliweight(107),
            Elements::FullDecrement32 => Cost::from_milliweight(153),
            Elements::FullDecrement64 => Cost::from_milliweight(151),
            Elements::FullDecrement8 => Cost::from_milliweight(218),
            Elements::FullIncrement16 => Cost::from_milliweight(108),
            Elements::FullIncrement32 => Cost::from_milliweight(171),
            Elements::FullIncrement64 => Cost::from_milliweight(161),
            Elements::FullIncrement8 => Cost::from_milliweight(204),
            Elements::FullLeftShift16_1 => Cost::from_milliweight(150),
            Elements::FullLeftShift16_2 => Cost::from_milliweight(150),
            Elements::FullLeftShift16_4 => Cost::from_milliweight(150),
            Elements::FullLeftShift16_8 => Cost::from_milliweight(150),
            Elements::FullLeftShift32_1 => Cost::from_milliweight(150),
            Elements::FullLeftShift32_16 => Cost::from_milliweight(150),
            Elements::FullLeftShift32_2 => Cost::from_milliweight(150),
            Elements::FullLeftShift32_4 => Cost::from_milliweight(150),
            Elements::FullLeftShift32_8 => Cost::from_milliweight(150),
            Elements::FullLeftShift64_1 => Cost::from_milliweight(150),
            Elements::FullLeftShift64_16 => Cost::from_milliweight(150),
            Elements::FullLeftShift64_2 => Cost::from_milliweight(150),
            Elements::FullLeftShift64_32 => Cost::from_milliweight(150),
            Elements::FullLeftShift64_4 => Cost::from_milliweight(150),
            Elements::FullLeftShift64_8 => Cost::from_milliweight(150),
            Elements::FullLeftShift8_1 => Cost::from_milliweight(150),
            Elements::FullLeftShift8_2 => Cost::from_milliweight(150),
            Elements::FullLeftShift8_4 => Cost::from_milliweight(150),
            Elements::FullMultiply16 => Cost::from_milliweight(208),
            Elements::FullMultiply32 => Cost::from_milliweight(213),
            Elements::FullMultiply64 => Cost::from_milliweight(209),
            Elements::FullMultiply8 => Cost::from_milliweight(190),
            Elements::FullRightShift16_1 => Cost::from_milliweight(150),
            Elements::FullRightShift16_2 => Cost::from_milliweight(150),
            Elements::FullRightShift16_4 => Cost::from_milliweight(150),
            Elements::FullRightShift16_8 => Cost::from_milliweight(150),
            Elements::FullRightShift32_1 => Cost::from_milliweight(150),
            Elements::FullRightShift32_16 => Cost::from_milliweight(150),
            Elements::FullRightShift32_2 => Cost::from_milliweight(150),
            Elements::FullRightShift32_4 => Cost::from_milliweight(150),
            Elements::FullRightShift32_8 => Cost::from_milliweight(150),
            Elements::FullRightShift64_1 => Cost::from_milliweight(150),
            Elements::FullRightShift64_16 => Cost::from_milliweight(150),
            Elements::FullRightShift64_2 => Cost::from_milliweight(150),
            Elements::FullRightShift64_32 => Cost::from_milliweight(150),
            Elements::FullRightShift64_4 => Cost::from_milliweight(150),
            Elements::FullRightShift64_8 => Cost::from_milliweight(150),
            Elements::FullRightShift8_1 => Cost::from_milliweight(150),
            Elements::FullRightShift8_2 => Cost::from_milliweight(150),
            Elements::FullRightShift8_4 => Cost::from_milliweight(150),
            Elements::FullSubtract16 => Cost::from_milliweight(201),
            Elements::FullSubtract32 => Cost::from_milliweight(170),
            Elements::FullSubtract64 => Cost::from_milliweight(231),
            Elements::FullSubtract8 => Cost::from_milliweight(141),
            Elements::GeIsOnCurve => Cost::from_milliweight(763),
            Elements::GeNegate => Cost::from_milliweight(1278),
            Elements::GejAdd => Cost::from_milliweight(3292),
            Elements::GejDouble => Cost::from_milliweight(2103),
            Elements::GejEquiv => Cost::from_milliweight(1270),
            Elements::GejGeAdd => Cost::from_milliweight(2890),
            Elements::GejGeAddEx => Cost::from_milliweight(3114),
            Elements::GejGeEquiv => Cost::from_milliweight(1270),
            Elements::GejInfinity => Cost::from_milliweight(971),
            Elements::GejIsInfinity => Cost::from_milliweight(923),
            Elements::GejIsOnCurve => Cost::from_milliweight(1106),
            Elements::GejNegate => Cost::from_milliweight(1823),
            Elements::GejNormalize => Cost::from_milliweight(4337),
            Elements::GejRescale => Cost::from_milliweight(2315),
            Elements::GejXEquiv => Cost::from_milliweight(1270),
            Elements::GejYIsOdd => Cost::from_milliweight(3665),
            Elements::Generate => Cost::from_milliweight(51706),
            Elements::GenesisBlockHash => Cost::from_milliweight(395),
            Elements::HashToCurve => Cost::from_milliweight(10956),
            Elements::High1 => Cost::from_milliweight(169),
            Elements::High16 => Cost::from_milliweight(159),
            Elements::High32 => Cost::from_milliweight(121),
            Elements::High64 => Cost::from_milliweight(110),
            Elements::High8 => Cost::from_milliweight(169),
            Elements::Increment16 => Cost::from_milliweight(129),
            Elements::Increment32 => Cost::from_milliweight(195),
            Elements::Increment64 => Cost::from_milliweight(187),
            Elements::Increment8 => Cost::from_milliweight(155),
            Elements::InputAmount => Cost::from_milliweight(359),
            Elements::InputAmountsHash => Cost::from_milliweight(280),
            Elements::InputAnnexHash => Cost::from_milliweight(122),
            Elements::InputAnnexesHash => Cost::from_milliweight(290),
            Elements::InputAsset => Cost::from_milliweight(220),
            Elements::InputHash => Cost::from_milliweight(289),
            Elements::InputOutpointsHash => Cost::from_milliweight(408),
            Elements::InputPegin => Cost::from_milliweight(235),
            Elements::InputPrevOutpoint => Cost::from_milliweight(326),
            Elements::InputScriptHash => Cost::from_milliweight(285),
            Elements::InputScriptSigHash => Cost::from_milliweight(231),
            Elements::InputScriptSigsHash => Cost::from_milliweight(283),
            Elements::InputScriptsHash => Cost::from_milliweight(280),
            Elements::InputSequence => Cost::from_milliweight(144),
            Elements::InputSequencesHash => Cost::from_milliweight(292),
            Elements::InputUtxoHash => Cost::from_milliweight(409),
            Elements::InputUtxosHash => Cost::from_milliweight(409),
            Elements::InputsHash => Cost::from_milliweight(289),
            Elements::InternalKey => Cost::from_milliweight(272),
            Elements::IsOne16 => Cost::from_milliweight(117),
            Elements::IsOne32 => Cost::from_milliweight(136),
            Elements::IsOne64 => Cost::from_milliweight(163),
            Elements::IsOne8 => Cost::from_milliweight(160),
            Elements::IsZero16 => Cost::from_milliweight(143),
            Elements::IsZero32 => Cost::from_milliweight(135),
            Elements::IsZero64 => Cost::from_milliweight(136),
            Elements::IsZero8 => Cost::from_milliweight(163),
            Elements::Issuance => Cost::from_milliweight(116),
            Elements::IssuanceAsset => Cost::from_milliweight(241),
            Elements::IssuanceAssetAmount => Cost::from_milliweight(261),
            Elements::IssuanceAssetAmountsHash => Cost::from_milliweight(276),
            Elements::IssuanceAssetProof => Cost::from_milliweight(229),
            Elements::IssuanceBlindingEntropyHash => Cost::from_milliweight(278),
            Elements::IssuanceEntropy => Cost::from_milliweight(437),
            Elements::IssuanceHash => Cost::from_milliweight(281),
            Elements::IssuanceRangeProofsHash => Cost::from_milliweight(280),
            Elements::IssuanceToken => Cost::from_milliweight(253),
            Elements::IssuanceTokenAmount => Cost::from_milliweight(240),
            Elements::IssuanceTokenAmountsHash => Cost::from_milliweight(289),
            Elements::IssuanceTokenProof => Cost::from_milliweight(233),
            Elements::IssuancesHash => Cost::from_milliweight(281),
            Elements::LbtcAsset => Cost::from_milliweight(3065),
            Elements::Le16 => Cost::from_milliweight(166),
            Elements::Le32 => Cost::from_milliweight(216),
            Elements::Le64 => Cost::from_milliweight(173),
            Elements::Le8 => Cost::from_milliweight(143),
            Elements::LeftExtend16_32 => Cost::from_milliweight(150),
            Elements::LeftExtend16_64 => Cost::from_milliweight(150),
            Elements::LeftExtend1_16 => Cost::from_milliweight(150),
            Elements::LeftExtend1_32 => Cost::from_milliweight(150),
            Elements::LeftExtend1_64 => Cost::from_milliweight(150),
            Elements::LeftExtend1_8 => Cost::from_milliweight(150),
            Elements::LeftExtend32_64 => Cost::from_milliweight(150),
            Elements::LeftExtend8_16 => Cost::from_milliweight(150),
            Elements::LeftExtend8_32 => Cost::from_milliweight(150),
            Elements::LeftExtend8_64 => Cost::from_milliweight(150),
            Elements::LeftPadHigh16_32 => Cost::from_milliweight(150),
            Elements::LeftPadHigh16_64 => Cost::from_milliweight(150),
            Elements::LeftPadHigh1_16 => Cost::from_milliweight(150),
            Elements::LeftPadHigh1_32 => Cost::from_milliweight(150),
            Elements::LeftPadHigh1_64 => Cost::from_milliweight(150),
            Elements::LeftPadHigh1_8 => Cost::from_milliweight(150),
            Elements::LeftPadHigh32_64 => Cost::from_milliweight(150),
            Elements::LeftPadHigh8_16 => Cost::from_milliweight(150),
            Elements::LeftPadHigh8_32 => Cost::from_milliweight(150),
            Elements::LeftPadHigh8_64 => Cost::from_milliweight(150),
            Elements::LeftPadLow16_32 => Cost::from_milliweight(150),
            Elements::LeftPadLow16_64 => Cost::from_milliweight(150),
            Elements::LeftPadLow1_16 => Cost::from_milliweight(150),
            Elements::LeftPadLow1_32 => Cost::from_milliweight(150),
            Elements::LeftPadLow1_64 => Cost::from_milliweight(150),
            Elements::LeftPadLow1_8 => Cost::from_milliweight(150),
            Elements::LeftPadLow32_64 => Cost::from_milliweight(150),
            Elements::LeftPadLow8_16 => Cost::from_milliweight(150),
            Elements::LeftPadLow8_32 => Cost::from_milliweight(150),
            Elements::LeftPadLow8_64 => Cost::from_milliweight(150),
            Elements::LeftRotate16 => Cost::from_milliweight(150),
            Elements::LeftRotate32 => Cost::from_milliweight(150),
            Elements::LeftRotate64 => Cost::from_milliweight(150),
            Elements::LeftRotate8 => Cost::from_milliweight(150),
            Elements::LeftShift16 => Cost::from_milliweight(150),
            Elements::LeftShift32 => Cost::from_milliweight(150),
            Elements::LeftShift64 => Cost::from_milliweight(150),
            Elements::LeftShift8 => Cost::from_milliweight(150),
            Elements::LeftShiftWith16 => Cost::from_milliweight(150),
            Elements::LeftShiftWith32 => Cost::from_milliweight(150),
            Elements::LeftShiftWith64 => Cost::from_milliweight(150),
            Elements::LeftShiftWith8 => Cost::from_milliweight(150),
            Elements::Leftmost16_1 => Cost::from_milliweight(150),
            Elements::Leftmost16_2 => Cost::from_milliweight(150),
            Elements::Leftmost16_4 => Cost::from_milliweight(150),
            Elements::Leftmost16_8 => Cost::from_milliweight(150),
            Elements::Leftmost32_1 => Cost::from_milliweight(150),
            Elements::Leftmost32_16 => Cost::from_milliweight(150),
            Elements::Leftmost32_2 => Cost::from_milliweight(150),
            Elements::Leftmost32_4 => Cost::from_milliweight(150),
            Elements::Leftmost32_8 => Cost::from_milliweight(150),
            Elements::Leftmost64_1 => Cost::from_milliweight(150),
            Elements::Leftmost64_16 => Cost::from_milliweight(150),
            Elements::Leftmost64_2 => Cost::from_milliweight(150),
            Elements::Leftmost64_32 => Cost::from_milliweight(150),
            Elements::Leftmost64_4 => Cost::from_milliweight(150),
            Elements::Leftmost64_8 => Cost::from_milliweight(150),
            Elements::Leftmost8_1 => Cost::from_milliweight(150),
            Elements::Leftmost8_2 => Cost::from_milliweight(150),
            Elements::Leftmost8_4 => Cost::from_milliweight(150),
            Elements::LinearCombination1 => Cost::from_milliweight(86722),
            Elements::LinearVerify1 => Cost::from_milliweight(43063),
            Elements::LockTime => Cost::from_milliweight(132),
            Elements::Low1 => Cost::from_milliweight(173),
            Elements::Low16 => Cost::from_milliweight(172),
            Elements::Low32 => Cost::from_milliweight(170),
            Elements::Low64 => Cost::from_milliweight(162),
            Elements::Low8 => Cost::from_milliweight(173),
            Elements::Lt16 => Cost::from_milliweight(188),
            Elements::Lt32 => Cost::from_milliweight(215),
            Elements::Lt64 => Cost::from_milliweight(195),
            Elements::Lt8 => Cost::from_milliweight(130),
            Elements::Maj1 => Cost::from_milliweight(241),
            Elements::Maj16 => Cost::from_milliweight(273),
            Elements::Maj32 => Cost::from_milliweight(289),
            Elements::Maj64 => Cost::from_milliweight(293),
            Elements::Maj8 => Cost::from_milliweight(241),
            Elements::Max16 => Cost::from_milliweight(164),
            Elements::Max32 => Cost::from_milliweight(162),
            Elements::Max64 => Cost::from_milliweight(193),
            Elements::Max8 => Cost::from_milliweight(142),
            Elements::Median16 => Cost::from_milliweight(270),
            Elements::Median32 => Cost::from_milliweight(256),
            Elements::Median64 => Cost::from_milliweight(336),
            Elements::Median8 => Cost::from_milliweight(256),
            Elements::Min16 => Cost::from_milliweight(164),
            Elements::Min32 => Cost::from_milliweight(181),
            Elements::Min64 => Cost::from_milliweight(150),
            Elements::Min8 => Cost::from_milliweight(135),
            Elements::Modulo16 => Cost::from_milliweight(188),
            Elements::Modulo32 => Cost::from_milliweight(207),
            Elements::Modulo64 => Cost::from_milliweight(191),
            Elements::Modulo8 => Cost::from_milliweight(158),
            Elements::Multiply16 => Cost::from_milliweight(154),
            Elements::Multiply32 => Cost::from_milliweight(165),
            Elements::Multiply64 => Cost::from_milliweight(185),
            Elements::Multiply8 => Cost::from_milliweight(126),
            Elements::Negate16 => Cost::from_milliweight(121),
            Elements::Negate32 => Cost::from_milliweight(185),
            Elements::Negate64 => Cost::from_milliweight(162),
            Elements::Negate8 => Cost::from_milliweight(152),
            Elements::NewIssuanceContract => Cost::from_milliweight(214),
            Elements::NonceHash => Cost::from_milliweight(2727),
            Elements::NumInputs => Cost::from_milliweight(139),
            Elements::NumOutputs => Cost::from_milliweight(134),
            Elements::One16 => Cost::from_milliweight(126),
            Elements::One32 => Cost::from_milliweight(122),
            Elements::One64 => Cost::from_milliweight(123),
            Elements::One8 => Cost::from_milliweight(127),
            Elements::Or1 => Cost::from_milliweight(147),
            Elements::Or16 => Cost::from_milliweight(204),
            Elements::Or32 => Cost::from_milliweight(197),
            Elements::Or64 => Cost::from_milliweight(214),
            Elements::Or8 => Cost::from_milliweight(147),
            Elements::OutpointHash => Cost::from_milliweight(5256),
            Elements::OutputAmount => Cost::from_milliweight(709),
            Elements::OutputAmountsHash => Cost::from_milliweight(404),
            Elements::OutputAsset => Cost::from_milliweight(621),
            Elements::OutputHash => Cost::from_milliweight(296),
            Elements::OutputIsFee => Cost::from_milliweight(270),
            Elements::OutputNonce => Cost::from_milliweight(396),
            Elements::OutputNoncesHash => Cost::from_milliweight(387),
            Elements::OutputNullDatum => Cost::from_milliweight(249),
            Elements::OutputRangeProof => Cost::from_milliweight(452),
            Elements::OutputRangeProofsHash => Cost::from_milliweight(397),
            Elements::OutputScriptHash => Cost::from_milliweight(437),
            Elements::OutputScriptsHash => Cost::from_milliweight(416),
            Elements::OutputSurjectionProof => Cost::from_milliweight(453),
            Elements::OutputSurjectionProofsHash => Cost::from_milliweight(419),
            Elements::OutputsHash => Cost::from_milliweight(296),
            Elements::ParseLock => Cost::from_milliweight(177),
            Elements::ParseSequence => Cost::from_milliweight(261),
            Elements::PointVerify1 => Cost::from_milliweight(50604),
            Elements::ReissuanceBlinding => Cost::from_milliweight(153),
            Elements::ReissuanceEntropy => Cost::from_milliweight(126),
            Elements::RightExtend16_32 => Cost::from_milliweight(150),
            Elements::RightExtend16_64 => Cost::from_milliweight(150),
            Elements::RightExtend32_64 => Cost::from_milliweight(150),
            Elements::RightExtend8_16 => Cost::from_milliweight(150),
            Elements::RightExtend8_32 => Cost::from_milliweight(150),
            Elements::RightExtend8_64 => Cost::from_milliweight(150),
            Elements::RightPadHigh16_32 => Cost::from_milliweight(150),
            Elements::RightPadHigh16_64 => Cost::from_milliweight(150),
            Elements::RightPadHigh1_16 => Cost::from_milliweight(150),
            Elements::RightPadHigh1_32 => Cost::from_milliweight(150),
            Elements::RightPadHigh1_64 => Cost::from_milliweight(150),
            Elements::RightPadHigh1_8 => Cost::from_milliweight(150),
            Elements::RightPadHigh32_64 => Cost::from_milliweight(150),
            Elements::RightPadHigh8_16 => Cost::from_milliweight(150),
            Elements::RightPadHigh8_32 => Cost::from_milliweight(150),
            Elements::RightPadHigh8_64 => Cost::from_milliweight(150),
            Elements::RightPadLow16_32 => Cost::from_milliweight(150),
            Elements::RightPadLow16_64 => Cost::from_milliweight(150),
            Elements::RightPadLow1_16 => Cost::from_milliweight(150),
            Elements::RightPadLow1_32 => Cost::from_milliweight(150),
            Elements::RightPadLow1_64 => Cost::from_milliweight(150),
            Elements::RightPadLow1_8 => Cost::from_milliweight(150),
            Elements::RightPadLow32_64 => Cost::from_milliweight(150),
            Elements::RightPadLow8_16 => Cost::from_milliweight(150),
            Elements::RightPadLow8_32 => Cost::from_milliweight(150),
            Elements::RightPadLow8_64 => Cost::from_milliweight(150),
            Elements::RightRotate16 => Cost::from_milliweight(150),
            Elements::RightRotate32 => Cost::from_milliweight(150),
            Elements::RightRotate64 => Cost::from_milliweight(150),
            Elements::RightRotate8 => Cost::from_milliweight(150),
            Elements::RightShift16 => Cost::from_milliweight(150),
            Elements::RightShift32 => Cost::from_milliweight(150),
            Elements::RightShift64 => Cost::from_milliweight(150),
            Elements::RightShift8 => Cost::from_milliweight(150),
            Elements::RightShiftWith16 => Cost::from_milliweight(150),
            Elements::RightShiftWith32 => Cost::from_milliweight(150),
            Elements::RightShiftWith64 => Cost::from_milliweight(150),
            Elements::RightShiftWith8 => Cost::from_milliweight(150),
            Elements::Rightmost16_1 => Cost::from_milliweight(150),
            Elements::Rightmost16_2 => Cost::from_milliweight(150),
            Elements::Rightmost16_4 => Cost::from_milliweight(150),
            Elements::Rightmost16_8 => Cost::from_milliweight(150),
            Elements::Rightmost32_1 => Cost::from_milliweight(150),
            Elements::Rightmost32_16 => Cost::from_milliweight(150),
            Elements::Rightmost32_2 => Cost::from_milliweight(150),
            Elements::Rightmost32_4 => Cost::from_milliweight(150),
            Elements::Rightmost32_8 => Cost::from_milliweight(150),
            Elements::Rightmost64_1 => Cost::from_milliweight(150),
            Elements::Rightmost64_16 => Cost::from_milliweight(150),
            Elements::Rightmost64_2 => Cost::from_milliweight(150),
            Elements::Rightmost64_32 => Cost::from_milliweight(150),
            Elements::Rightmost64_4 => Cost::from_milliweight(150),
            Elements::Rightmost64_8 => Cost::from_milliweight(150),
            Elements::Rightmost8_1 => Cost::from_milliweight(150),
            Elements::Rightmost8_2 => Cost::from_milliweight(150),
            Elements::Rightmost8_4 => Cost::from_milliweight(150),
            Elements::ScalarAdd => Cost::from_milliweight(962),
            Elements::ScalarInvert => Cost::from_milliweight(4025),
            Elements::ScalarIsZero => Cost::from_milliweight(569),
            Elements::ScalarMultiply => Cost::from_milliweight(1230),
            Elements::ScalarMultiplyLambda => Cost::from_milliweight(984),
            Elements::ScalarNegate => Cost::from_milliweight(851),
            Elements::ScalarNormalize => Cost::from_milliweight(808),
            Elements::ScalarSquare => Cost::from_milliweight(947),
            Elements::Scale => Cost::from_milliweight(75377),
            Elements::ScriptCMR => Cost::from_milliweight(281),
            Elements::Sha256Block => Cost::from_milliweight(986),
            Elements::Sha256Ctx8Add1 => Cost::from_milliweight(1600),
            Elements::Sha256Ctx8Add128 => Cost::from_milliweight(3921),
            Elements::Sha256Ctx8Add16 => Cost::from_milliweight(2275),
            Elements::Sha256Ctx8Add2 => Cost::from_milliweight(3772),
            Elements::Sha256Ctx8Add256 => Cost::from_milliweight(6211),
            Elements::Sha256Ctx8Add32 => Cost::from_milliweight(4599),
            Elements::Sha256Ctx8Add4 => Cost::from_milliweight(3515),
            Elements::Sha256Ctx8Add512 => Cost::from_milliweight(10936),
            Elements::Sha256Ctx8Add64 => Cost::from_milliweight(3111),
            Elements::Sha256Ctx8Add8 => Cost::from_milliweight(1625),
            Elements::Sha256Ctx8AddBuffer511 => Cost::from_milliweight(14290),
            Elements::Sha256Ctx8Finalize => Cost::from_milliweight(2111),
            Elements::Sha256Ctx8Init => Cost::from_milliweight(184),
            Elements::Sha256Iv => Cost::from_milliweight(129),
            Elements::SigAllHash => Cost::from_milliweight(265),
            Elements::Some1 => Cost::from_milliweight(104),
            Elements::Some16 => Cost::from_milliweight(129),
            Elements::Some32 => Cost::from_milliweight(183),
            Elements::Some64 => Cost::from_milliweight(139),
            Elements::Some8 => Cost::from_milliweight(104),
            Elements::Subtract16 => Cost::from_milliweight(237),
            Elements::Subtract32 => Cost::from_milliweight(186),
            Elements::Subtract64 => Cost::from_milliweight(315),
            Elements::Subtract8 => Cost::from_milliweight(149),
            Elements::Swu => Cost::from_milliweight(10956),
            Elements::TapEnvHash => Cost::from_milliweight(301),
            Elements::TapleafHash => Cost::from_milliweight(279),
            Elements::TapleafVersion => Cost::from_milliweight(141),
            Elements::Tappath => Cost::from_milliweight(143),
            Elements::TappathHash => Cost::from_milliweight(282),
            Elements::TotalFee => Cost::from_milliweight(397),
            Elements::TransactionId => Cost::from_milliweight(281),
            Elements::TxHash => Cost::from_milliweight(290),
            Elements::TxIsFinal => Cost::from_milliweight(128),
            Elements::TxLockDistance => Cost::from_milliweight(133),
            Elements::TxLockDuration => Cost::from_milliweight(146),
            Elements::TxLockHeight => Cost::from_milliweight(132),
            Elements::TxLockTime => Cost::from_milliweight(136),
            Elements::Verify => Cost::from_milliweight(144),
            Elements::Version => Cost::from_milliweight(188),
            Elements::Xor1 => Cost::from_milliweight(135),
            Elements::Xor16 => Cost::from_milliweight(188),
            Elements::Xor32 => Cost::from_milliweight(204),
            Elements::Xor64 => Cost::from_milliweight(207),
            Elements::Xor8 => Cost::from_milliweight(135),
            Elements::XorXor1 => Cost::from_milliweight(258),
            Elements::XorXor16 => Cost::from_milliweight(235),
            Elements::XorXor32 => Cost::from_milliweight(251),
            Elements::XorXor64 => Cost::from_milliweight(285),
            Elements::XorXor8 => Cost::from_milliweight(258),
        }
    }
}

impl fmt::Display for Elements {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Elements::Add16 => f.write_str("add_16"),
            Elements::Add32 => f.write_str("add_32"),
            Elements::Add64 => f.write_str("add_64"),
            Elements::Add8 => f.write_str("add_8"),
            Elements::All16 => f.write_str("all_16"),
            Elements::All32 => f.write_str("all_32"),
            Elements::All64 => f.write_str("all_64"),
            Elements::All8 => f.write_str("all_8"),
            Elements::And1 => f.write_str("and_1"),
            Elements::And16 => f.write_str("and_16"),
            Elements::And32 => f.write_str("and_32"),
            Elements::And64 => f.write_str("and_64"),
            Elements::And8 => f.write_str("and_8"),
            Elements::AnnexHash => f.write_str("annex_hash"),
            Elements::AssetAmountHash => f.write_str("asset_amount_hash"),
            Elements::Bip0340Verify => f.write_str("bip_0340_verify"),
            Elements::BuildTapbranch => f.write_str("build_tapbranch"),
            Elements::BuildTapleafSimplicity => f.write_str("build_tapleaf_simplicity"),
            Elements::CalculateAsset => f.write_str("calculate_asset"),
            Elements::CalculateConfidentialToken => f.write_str("calculate_confidential_token"),
            Elements::CalculateExplicitToken => f.write_str("calculate_explicit_token"),
            Elements::CalculateIssuanceEntropy => f.write_str("calculate_issuance_entropy"),
            Elements::Ch1 => f.write_str("ch_1"),
            Elements::Ch16 => f.write_str("ch_16"),
            Elements::Ch32 => f.write_str("ch_32"),
            Elements::Ch64 => f.write_str("ch_64"),
            Elements::Ch8 => f.write_str("ch_8"),
            Elements::CheckLockDistance => f.write_str("check_lock_distance"),
            Elements::CheckLockDuration => f.write_str("check_lock_duration"),
            Elements::CheckLockHeight => f.write_str("check_lock_height"),
            Elements::CheckLockTime => f.write_str("check_lock_time"),
            Elements::CheckSigVerify => f.write_str("check_sig_verify"),
            Elements::Complement1 => f.write_str("complement_1"),
            Elements::Complement16 => f.write_str("complement_16"),
            Elements::Complement32 => f.write_str("complement_32"),
            Elements::Complement64 => f.write_str("complement_64"),
            Elements::Complement8 => f.write_str("complement_8"),
            Elements::CurrentAmount => f.write_str("current_amount"),
            Elements::CurrentAnnexHash => f.write_str("current_annex_hash"),
            Elements::CurrentAsset => f.write_str("current_asset"),
            Elements::CurrentIndex => f.write_str("current_index"),
            Elements::CurrentIssuanceAssetAmount => f.write_str("current_issuance_asset_amount"),
            Elements::CurrentIssuanceAssetProof => f.write_str("current_issuance_asset_proof"),
            Elements::CurrentIssuanceTokenAmount => f.write_str("current_issuance_token_amount"),
            Elements::CurrentIssuanceTokenProof => f.write_str("current_issuance_token_proof"),
            Elements::CurrentNewIssuanceContract => f.write_str("current_new_issuance_contract"),
            Elements::CurrentPegin => f.write_str("current_pegin"),
            Elements::CurrentPrevOutpoint => f.write_str("current_prev_outpoint"),
            Elements::CurrentReissuanceBlinding => f.write_str("current_reissuance_blinding"),
            Elements::CurrentReissuanceEntropy => f.write_str("current_reissuance_entropy"),
            Elements::CurrentScriptHash => f.write_str("current_script_hash"),
            Elements::CurrentScriptSigHash => f.write_str("current_script_sig_hash"),
            Elements::CurrentSequence => f.write_str("current_sequence"),
            Elements::Decompress => f.write_str("decompress"),
            Elements::Decrement16 => f.write_str("decrement_16"),
            Elements::Decrement32 => f.write_str("decrement_32"),
            Elements::Decrement64 => f.write_str("decrement_64"),
            Elements::Decrement8 => f.write_str("decrement_8"),
            Elements::DivMod128_64 => f.write_str("div_mod_128_64"),
            Elements::DivMod16 => f.write_str("div_mod_16"),
            Elements::DivMod32 => f.write_str("div_mod_32"),
            Elements::DivMod64 => f.write_str("div_mod_64"),
            Elements::DivMod8 => f.write_str("div_mod_8"),
            Elements::Divide16 => f.write_str("divide_16"),
            Elements::Divide32 => f.write_str("divide_32"),
            Elements::Divide64 => f.write_str("divide_64"),
            Elements::Divide8 => f.write_str("divide_8"),
            Elements::Divides16 => f.write_str("divides_16"),
            Elements::Divides32 => f.write_str("divides_32"),
            Elements::Divides64 => f.write_str("divides_64"),
            Elements::Divides8 => f.write_str("divides_8"),
            Elements::Eq1 => f.write_str("eq_1"),
            Elements::Eq16 => f.write_str("eq_16"),
            Elements::Eq256 => f.write_str("eq_256"),
            Elements::Eq32 => f.write_str("eq_32"),
            Elements::Eq64 => f.write_str("eq_64"),
            Elements::Eq8 => f.write_str("eq_8"),
            Elements::FeAdd => f.write_str("fe_add"),
            Elements::FeInvert => f.write_str("fe_invert"),
            Elements::FeIsOdd => f.write_str("fe_is_odd"),
            Elements::FeIsZero => f.write_str("fe_is_zero"),
            Elements::FeMultiply => f.write_str("fe_multiply"),
            Elements::FeMultiplyBeta => f.write_str("fe_multiply_beta"),
            Elements::FeNegate => f.write_str("fe_negate"),
            Elements::FeNormalize => f.write_str("fe_normalize"),
            Elements::FeSquare => f.write_str("fe_square"),
            Elements::FeSquareRoot => f.write_str("fe_square_root"),
            Elements::FullAdd16 => f.write_str("full_add_16"),
            Elements::FullAdd32 => f.write_str("full_add_32"),
            Elements::FullAdd64 => f.write_str("full_add_64"),
            Elements::FullAdd8 => f.write_str("full_add_8"),
            Elements::FullDecrement16 => f.write_str("full_decrement_16"),
            Elements::FullDecrement32 => f.write_str("full_decrement_32"),
            Elements::FullDecrement64 => f.write_str("full_decrement_64"),
            Elements::FullDecrement8 => f.write_str("full_decrement_8"),
            Elements::FullIncrement16 => f.write_str("full_increment_16"),
            Elements::FullIncrement32 => f.write_str("full_increment_32"),
            Elements::FullIncrement64 => f.write_str("full_increment_64"),
            Elements::FullIncrement8 => f.write_str("full_increment_8"),
            Elements::FullLeftShift16_1 => f.write_str("full_left_shift_16_1"),
            Elements::FullLeftShift16_2 => f.write_str("full_left_shift_16_2"),
            Elements::FullLeftShift16_4 => f.write_str("full_left_shift_16_4"),
            Elements::FullLeftShift16_8 => f.write_str("full_left_shift_16_8"),
            Elements::FullLeftShift32_1 => f.write_str("full_left_shift_32_1"),
            Elements::FullLeftShift32_16 => f.write_str("full_left_shift_32_16"),
            Elements::FullLeftShift32_2 => f.write_str("full_left_shift_32_2"),
            Elements::FullLeftShift32_4 => f.write_str("full_left_shift_32_4"),
            Elements::FullLeftShift32_8 => f.write_str("full_left_shift_32_8"),
            Elements::FullLeftShift64_1 => f.write_str("full_left_shift_64_1"),
            Elements::FullLeftShift64_16 => f.write_str("full_left_shift_64_16"),
            Elements::FullLeftShift64_2 => f.write_str("full_left_shift_64_2"),
            Elements::FullLeftShift64_32 => f.write_str("full_left_shift_64_32"),
            Elements::FullLeftShift64_4 => f.write_str("full_left_shift_64_4"),
            Elements::FullLeftShift64_8 => f.write_str("full_left_shift_64_8"),
            Elements::FullLeftShift8_1 => f.write_str("full_left_shift_8_1"),
            Elements::FullLeftShift8_2 => f.write_str("full_left_shift_8_2"),
            Elements::FullLeftShift8_4 => f.write_str("full_left_shift_8_4"),
            Elements::FullMultiply16 => f.write_str("full_multiply_16"),
            Elements::FullMultiply32 => f.write_str("full_multiply_32"),
            Elements::FullMultiply64 => f.write_str("full_multiply_64"),
            Elements::FullMultiply8 => f.write_str("full_multiply_8"),
            Elements::FullRightShift16_1 => f.write_str("full_right_shift_16_1"),
            Elements::FullRightShift16_2 => f.write_str("full_right_shift_16_2"),
            Elements::FullRightShift16_4 => f.write_str("full_right_shift_16_4"),
            Elements::FullRightShift16_8 => f.write_str("full_right_shift_16_8"),
            Elements::FullRightShift32_1 => f.write_str("full_right_shift_32_1"),
            Elements::FullRightShift32_16 => f.write_str("full_right_shift_32_16"),
            Elements::FullRightShift32_2 => f.write_str("full_right_shift_32_2"),
            Elements::FullRightShift32_4 => f.write_str("full_right_shift_32_4"),
            Elements::FullRightShift32_8 => f.write_str("full_right_shift_32_8"),
            Elements::FullRightShift64_1 => f.write_str("full_right_shift_64_1"),
            Elements::FullRightShift64_16 => f.write_str("full_right_shift_64_16"),
            Elements::FullRightShift64_2 => f.write_str("full_right_shift_64_2"),
            Elements::FullRightShift64_32 => f.write_str("full_right_shift_64_32"),
            Elements::FullRightShift64_4 => f.write_str("full_right_shift_64_4"),
            Elements::FullRightShift64_8 => f.write_str("full_right_shift_64_8"),
            Elements::FullRightShift8_1 => f.write_str("full_right_shift_8_1"),
            Elements::FullRightShift8_2 => f.write_str("full_right_shift_8_2"),
            Elements::FullRightShift8_4 => f.write_str("full_right_shift_8_4"),
            Elements::FullSubtract16 => f.write_str("full_subtract_16"),
            Elements::FullSubtract32 => f.write_str("full_subtract_32"),
            Elements::FullSubtract64 => f.write_str("full_subtract_64"),
            Elements::FullSubtract8 => f.write_str("full_subtract_8"),
            Elements::GeIsOnCurve => f.write_str("ge_is_on_curve"),
            Elements::GeNegate => f.write_str("ge_negate"),
            Elements::GejAdd => f.write_str("gej_add"),
            Elements::GejDouble => f.write_str("gej_double"),
            Elements::GejEquiv => f.write_str("gej_equiv"),
            Elements::GejGeAdd => f.write_str("gej_ge_add"),
            Elements::GejGeAddEx => f.write_str("gej_ge_add_ex"),
            Elements::GejGeEquiv => f.write_str("gej_ge_equiv"),
            Elements::GejInfinity => f.write_str("gej_infinity"),
            Elements::GejIsInfinity => f.write_str("gej_is_infinity"),
            Elements::GejIsOnCurve => f.write_str("gej_is_on_curve"),
            Elements::GejNegate => f.write_str("gej_negate"),
            Elements::GejNormalize => f.write_str("gej_normalize"),
            Elements::GejRescale => f.write_str("gej_rescale"),
            Elements::GejXEquiv => f.write_str("gej_x_equiv"),
            Elements::GejYIsOdd => f.write_str("gej_y_is_odd"),
            Elements::Generate => f.write_str("generate"),
            Elements::GenesisBlockHash => f.write_str("genesis_block_hash"),
            Elements::HashToCurve => f.write_str("hash_to_curve"),
            Elements::High1 => f.write_str("high_1"),
            Elements::High16 => f.write_str("high_16"),
            Elements::High32 => f.write_str("high_32"),
            Elements::High64 => f.write_str("high_64"),
            Elements::High8 => f.write_str("high_8"),
            Elements::Increment16 => f.write_str("increment_16"),
            Elements::Increment32 => f.write_str("increment_32"),
            Elements::Increment64 => f.write_str("increment_64"),
            Elements::Increment8 => f.write_str("increment_8"),
            Elements::InputAmount => f.write_str("input_amount"),
            Elements::InputAmountsHash => f.write_str("input_amounts_hash"),
            Elements::InputAnnexHash => f.write_str("input_annex_hash"),
            Elements::InputAnnexesHash => f.write_str("input_annexes_hash"),
            Elements::InputAsset => f.write_str("input_asset"),
            Elements::InputHash => f.write_str("input_hash"),
            Elements::InputOutpointsHash => f.write_str("input_outpoints_hash"),
            Elements::InputPegin => f.write_str("input_pegin"),
            Elements::InputPrevOutpoint => f.write_str("input_prev_outpoint"),
            Elements::InputScriptHash => f.write_str("input_script_hash"),
            Elements::InputScriptSigHash => f.write_str("input_script_sig_hash"),
            Elements::InputScriptSigsHash => f.write_str("input_script_sigs_hash"),
            Elements::InputScriptsHash => f.write_str("input_scripts_hash"),
            Elements::InputSequence => f.write_str("input_sequence"),
            Elements::InputSequencesHash => f.write_str("input_sequences_hash"),
            Elements::InputUtxoHash => f.write_str("input_utxo_hash"),
            Elements::InputUtxosHash => f.write_str("input_utxos_hash"),
            Elements::InputsHash => f.write_str("inputs_hash"),
            Elements::InternalKey => f.write_str("internal_key"),
            Elements::IsOne16 => f.write_str("is_one_16"),
            Elements::IsOne32 => f.write_str("is_one_32"),
            Elements::IsOne64 => f.write_str("is_one_64"),
            Elements::IsOne8 => f.write_str("is_one_8"),
            Elements::IsZero16 => f.write_str("is_zero_16"),
            Elements::IsZero32 => f.write_str("is_zero_32"),
            Elements::IsZero64 => f.write_str("is_zero_64"),
            Elements::IsZero8 => f.write_str("is_zero_8"),
            Elements::Issuance => f.write_str("issuance"),
            Elements::IssuanceAsset => f.write_str("issuance_asset"),
            Elements::IssuanceAssetAmount => f.write_str("issuance_asset_amount"),
            Elements::IssuanceAssetAmountsHash => f.write_str("issuance_asset_amounts_hash"),
            Elements::IssuanceAssetProof => f.write_str("issuance_asset_proof"),
            Elements::IssuanceBlindingEntropyHash => f.write_str("issuance_blinding_entropy_hash"),
            Elements::IssuanceEntropy => f.write_str("issuance_entropy"),
            Elements::IssuanceHash => f.write_str("issuance_hash"),
            Elements::IssuanceRangeProofsHash => f.write_str("issuance_range_proofs_hash"),
            Elements::IssuanceToken => f.write_str("issuance_token"),
            Elements::IssuanceTokenAmount => f.write_str("issuance_token_amount"),
            Elements::IssuanceTokenAmountsHash => f.write_str("issuance_token_amounts_hash"),
            Elements::IssuanceTokenProof => f.write_str("issuance_token_proof"),
            Elements::IssuancesHash => f.write_str("issuances_hash"),
            Elements::LbtcAsset => f.write_str("lbtc_asset"),
            Elements::Le16 => f.write_str("le_16"),
            Elements::Le32 => f.write_str("le_32"),
            Elements::Le64 => f.write_str("le_64"),
            Elements::Le8 => f.write_str("le_8"),
            Elements::LeftExtend16_32 => f.write_str("left_extend_16_32"),
            Elements::LeftExtend16_64 => f.write_str("left_extend_16_64"),
            Elements::LeftExtend1_16 => f.write_str("left_extend_1_16"),
            Elements::LeftExtend1_32 => f.write_str("left_extend_1_32"),
            Elements::LeftExtend1_64 => f.write_str("left_extend_1_64"),
            Elements::LeftExtend1_8 => f.write_str("left_extend_1_8"),
            Elements::LeftExtend32_64 => f.write_str("left_extend_32_64"),
            Elements::LeftExtend8_16 => f.write_str("left_extend_8_16"),
            Elements::LeftExtend8_32 => f.write_str("left_extend_8_32"),
            Elements::LeftExtend8_64 => f.write_str("left_extend_8_64"),
            Elements::LeftPadHigh16_32 => f.write_str("left_pad_high_16_32"),
            Elements::LeftPadHigh16_64 => f.write_str("left_pad_high_16_64"),
            Elements::LeftPadHigh1_16 => f.write_str("left_pad_high_1_16"),
            Elements::LeftPadHigh1_32 => f.write_str("left_pad_high_1_32"),
            Elements::LeftPadHigh1_64 => f.write_str("left_pad_high_1_64"),
            Elements::LeftPadHigh1_8 => f.write_str("left_pad_high_1_8"),
            Elements::LeftPadHigh32_64 => f.write_str("left_pad_high_32_64"),
            Elements::LeftPadHigh8_16 => f.write_str("left_pad_high_8_16"),
            Elements::LeftPadHigh8_32 => f.write_str("left_pad_high_8_32"),
            Elements::LeftPadHigh8_64 => f.write_str("left_pad_high_8_64"),
            Elements::LeftPadLow16_32 => f.write_str("left_pad_low_16_32"),
            Elements::LeftPadLow16_64 => f.write_str("left_pad_low_16_64"),
            Elements::LeftPadLow1_16 => f.write_str("left_pad_low_1_16"),
            Elements::LeftPadLow1_32 => f.write_str("left_pad_low_1_32"),
            Elements::LeftPadLow1_64 => f.write_str("left_pad_low_1_64"),
            Elements::LeftPadLow1_8 => f.write_str("left_pad_low_1_8"),
            Elements::LeftPadLow32_64 => f.write_str("left_pad_low_32_64"),
            Elements::LeftPadLow8_16 => f.write_str("left_pad_low_8_16"),
            Elements::LeftPadLow8_32 => f.write_str("left_pad_low_8_32"),
            Elements::LeftPadLow8_64 => f.write_str("left_pad_low_8_64"),
            Elements::LeftRotate16 => f.write_str("left_rotate_16"),
            Elements::LeftRotate32 => f.write_str("left_rotate_32"),
            Elements::LeftRotate64 => f.write_str("left_rotate_64"),
            Elements::LeftRotate8 => f.write_str("left_rotate_8"),
            Elements::LeftShift16 => f.write_str("left_shift_16"),
            Elements::LeftShift32 => f.write_str("left_shift_32"),
            Elements::LeftShift64 => f.write_str("left_shift_64"),
            Elements::LeftShift8 => f.write_str("left_shift_8"),
            Elements::LeftShiftWith16 => f.write_str("left_shift_with_16"),
            Elements::LeftShiftWith32 => f.write_str("left_shift_with_32"),
            Elements::LeftShiftWith64 => f.write_str("left_shift_with_64"),
            Elements::LeftShiftWith8 => f.write_str("left_shift_with_8"),
            Elements::Leftmost16_1 => f.write_str("leftmost_16_1"),
            Elements::Leftmost16_2 => f.write_str("leftmost_16_2"),
            Elements::Leftmost16_4 => f.write_str("leftmost_16_4"),
            Elements::Leftmost16_8 => f.write_str("leftmost_16_8"),
            Elements::Leftmost32_1 => f.write_str("leftmost_32_1"),
            Elements::Leftmost32_16 => f.write_str("leftmost_32_16"),
            Elements::Leftmost32_2 => f.write_str("leftmost_32_2"),
            Elements::Leftmost32_4 => f.write_str("leftmost_32_4"),
            Elements::Leftmost32_8 => f.write_str("leftmost_32_8"),
            Elements::Leftmost64_1 => f.write_str("leftmost_64_1"),
            Elements::Leftmost64_16 => f.write_str("leftmost_64_16"),
            Elements::Leftmost64_2 => f.write_str("leftmost_64_2"),
            Elements::Leftmost64_32 => f.write_str("leftmost_64_32"),
            Elements::Leftmost64_4 => f.write_str("leftmost_64_4"),
            Elements::Leftmost64_8 => f.write_str("leftmost_64_8"),
            Elements::Leftmost8_1 => f.write_str("leftmost_8_1"),
            Elements::Leftmost8_2 => f.write_str("leftmost_8_2"),
            Elements::Leftmost8_4 => f.write_str("leftmost_8_4"),
            Elements::LinearCombination1 => f.write_str("linear_combination_1"),
            Elements::LinearVerify1 => f.write_str("linear_verify_1"),
            Elements::LockTime => f.write_str("lock_time"),
            Elements::Low1 => f.write_str("low_1"),
            Elements::Low16 => f.write_str("low_16"),
            Elements::Low32 => f.write_str("low_32"),
            Elements::Low64 => f.write_str("low_64"),
            Elements::Low8 => f.write_str("low_8"),
            Elements::Lt16 => f.write_str("lt_16"),
            Elements::Lt32 => f.write_str("lt_32"),
            Elements::Lt64 => f.write_str("lt_64"),
            Elements::Lt8 => f.write_str("lt_8"),
            Elements::Maj1 => f.write_str("maj_1"),
            Elements::Maj16 => f.write_str("maj_16"),
            Elements::Maj32 => f.write_str("maj_32"),
            Elements::Maj64 => f.write_str("maj_64"),
            Elements::Maj8 => f.write_str("maj_8"),
            Elements::Max16 => f.write_str("max_16"),
            Elements::Max32 => f.write_str("max_32"),
            Elements::Max64 => f.write_str("max_64"),
            Elements::Max8 => f.write_str("max_8"),
            Elements::Median16 => f.write_str("median_16"),
            Elements::Median32 => f.write_str("median_32"),
            Elements::Median64 => f.write_str("median_64"),
            Elements::Median8 => f.write_str("median_8"),
            Elements::Min16 => f.write_str("min_16"),
            Elements::Min32 => f.write_str("min_32"),
            Elements::Min64 => f.write_str("min_64"),
            Elements::Min8 => f.write_str("min_8"),
            Elements::Modulo16 => f.write_str("modulo_16"),
            Elements::Modulo32 => f.write_str("modulo_32"),
            Elements::Modulo64 => f.write_str("modulo_64"),
            Elements::Modulo8 => f.write_str("modulo_8"),
            Elements::Multiply16 => f.write_str("multiply_16"),
            Elements::Multiply32 => f.write_str("multiply_32"),
            Elements::Multiply64 => f.write_str("multiply_64"),
            Elements::Multiply8 => f.write_str("multiply_8"),
            Elements::Negate16 => f.write_str("negate_16"),
            Elements::Negate32 => f.write_str("negate_32"),
            Elements::Negate64 => f.write_str("negate_64"),
            Elements::Negate8 => f.write_str("negate_8"),
            Elements::NewIssuanceContract => f.write_str("new_issuance_contract"),
            Elements::NonceHash => f.write_str("nonce_hash"),
            Elements::NumInputs => f.write_str("num_inputs"),
            Elements::NumOutputs => f.write_str("num_outputs"),
            Elements::One16 => f.write_str("one_16"),
            Elements::One32 => f.write_str("one_32"),
            Elements::One64 => f.write_str("one_64"),
            Elements::One8 => f.write_str("one_8"),
            Elements::Or1 => f.write_str("or_1"),
            Elements::Or16 => f.write_str("or_16"),
            Elements::Or32 => f.write_str("or_32"),
            Elements::Or64 => f.write_str("or_64"),
            Elements::Or8 => f.write_str("or_8"),
            Elements::OutpointHash => f.write_str("outpoint_hash"),
            Elements::OutputAmount => f.write_str("output_amount"),
            Elements::OutputAmountsHash => f.write_str("output_amounts_hash"),
            Elements::OutputAsset => f.write_str("output_asset"),
            Elements::OutputHash => f.write_str("output_hash"),
            Elements::OutputIsFee => f.write_str("output_is_fee"),
            Elements::OutputNonce => f.write_str("output_nonce"),
            Elements::OutputNoncesHash => f.write_str("output_nonces_hash"),
            Elements::OutputNullDatum => f.write_str("output_null_datum"),
            Elements::OutputRangeProof => f.write_str("output_range_proof"),
            Elements::OutputRangeProofsHash => f.write_str("output_range_proofs_hash"),
            Elements::OutputScriptHash => f.write_str("output_script_hash"),
            Elements::OutputScriptsHash => f.write_str("output_scripts_hash"),
            Elements::OutputSurjectionProof => f.write_str("output_surjection_proof"),
            Elements::OutputSurjectionProofsHash => f.write_str("output_surjection_proofs_hash"),
            Elements::OutputsHash => f.write_str("outputs_hash"),
            Elements::ParseLock => f.write_str("parse_lock"),
            Elements::ParseSequence => f.write_str("parse_sequence"),
            Elements::PointVerify1 => f.write_str("point_verify_1"),
            Elements::ReissuanceBlinding => f.write_str("reissuance_blinding"),
            Elements::ReissuanceEntropy => f.write_str("reissuance_entropy"),
            Elements::RightExtend16_32 => f.write_str("right_extend_16_32"),
            Elements::RightExtend16_64 => f.write_str("right_extend_16_64"),
            Elements::RightExtend32_64 => f.write_str("right_extend_32_64"),
            Elements::RightExtend8_16 => f.write_str("right_extend_8_16"),
            Elements::RightExtend8_32 => f.write_str("right_extend_8_32"),
            Elements::RightExtend8_64 => f.write_str("right_extend_8_64"),
            Elements::RightPadHigh16_32 => f.write_str("right_pad_high_16_32"),
            Elements::RightPadHigh16_64 => f.write_str("right_pad_high_16_64"),
            Elements::RightPadHigh1_16 => f.write_str("right_pad_high_1_16"),
            Elements::RightPadHigh1_32 => f.write_str("right_pad_high_1_32"),
            Elements::RightPadHigh1_64 => f.write_str("right_pad_high_1_64"),
            Elements::RightPadHigh1_8 => f.write_str("right_pad_high_1_8"),
            Elements::RightPadHigh32_64 => f.write_str("right_pad_high_32_64"),
            Elements::RightPadHigh8_16 => f.write_str("right_pad_high_8_16"),
            Elements::RightPadHigh8_32 => f.write_str("right_pad_high_8_32"),
            Elements::RightPadHigh8_64 => f.write_str("right_pad_high_8_64"),
            Elements::RightPadLow16_32 => f.write_str("right_pad_low_16_32"),
            Elements::RightPadLow16_64 => f.write_str("right_pad_low_16_64"),
            Elements::RightPadLow1_16 => f.write_str("right_pad_low_1_16"),
            Elements::RightPadLow1_32 => f.write_str("right_pad_low_1_32"),
            Elements::RightPadLow1_64 => f.write_str("right_pad_low_1_64"),
            Elements::RightPadLow1_8 => f.write_str("right_pad_low_1_8"),
            Elements::RightPadLow32_64 => f.write_str("right_pad_low_32_64"),
            Elements::RightPadLow8_16 => f.write_str("right_pad_low_8_16"),
            Elements::RightPadLow8_32 => f.write_str("right_pad_low_8_32"),
            Elements::RightPadLow8_64 => f.write_str("right_pad_low_8_64"),
            Elements::RightRotate16 => f.write_str("right_rotate_16"),
            Elements::RightRotate32 => f.write_str("right_rotate_32"),
            Elements::RightRotate64 => f.write_str("right_rotate_64"),
            Elements::RightRotate8 => f.write_str("right_rotate_8"),
            Elements::RightShift16 => f.write_str("right_shift_16"),
            Elements::RightShift32 => f.write_str("right_shift_32"),
            Elements::RightShift64 => f.write_str("right_shift_64"),
            Elements::RightShift8 => f.write_str("right_shift_8"),
            Elements::RightShiftWith16 => f.write_str("right_shift_with_16"),
            Elements::RightShiftWith32 => f.write_str("right_shift_with_32"),
            Elements::RightShiftWith64 => f.write_str("right_shift_with_64"),
            Elements::RightShiftWith8 => f.write_str("right_shift_with_8"),
            Elements::Rightmost16_1 => f.write_str("rightmost_16_1"),
            Elements::Rightmost16_2 => f.write_str("rightmost_16_2"),
            Elements::Rightmost16_4 => f.write_str("rightmost_16_4"),
            Elements::Rightmost16_8 => f.write_str("rightmost_16_8"),
            Elements::Rightmost32_1 => f.write_str("rightmost_32_1"),
            Elements::Rightmost32_16 => f.write_str("rightmost_32_16"),
            Elements::Rightmost32_2 => f.write_str("rightmost_32_2"),
            Elements::Rightmost32_4 => f.write_str("rightmost_32_4"),
            Elements::Rightmost32_8 => f.write_str("rightmost_32_8"),
            Elements::Rightmost64_1 => f.write_str("rightmost_64_1"),
            Elements::Rightmost64_16 => f.write_str("rightmost_64_16"),
            Elements::Rightmost64_2 => f.write_str("rightmost_64_2"),
            Elements::Rightmost64_32 => f.write_str("rightmost_64_32"),
            Elements::Rightmost64_4 => f.write_str("rightmost_64_4"),
            Elements::Rightmost64_8 => f.write_str("rightmost_64_8"),
            Elements::Rightmost8_1 => f.write_str("rightmost_8_1"),
            Elements::Rightmost8_2 => f.write_str("rightmost_8_2"),
            Elements::Rightmost8_4 => f.write_str("rightmost_8_4"),
            Elements::ScalarAdd => f.write_str("scalar_add"),
            Elements::ScalarInvert => f.write_str("scalar_invert"),
            Elements::ScalarIsZero => f.write_str("scalar_is_zero"),
            Elements::ScalarMultiply => f.write_str("scalar_multiply"),
            Elements::ScalarMultiplyLambda => f.write_str("scalar_multiply_lambda"),
            Elements::ScalarNegate => f.write_str("scalar_negate"),
            Elements::ScalarNormalize => f.write_str("scalar_normalize"),
            Elements::ScalarSquare => f.write_str("scalar_square"),
            Elements::Scale => f.write_str("scale"),
            Elements::ScriptCMR => f.write_str("script_cmr"),
            Elements::Sha256Block => f.write_str("sha_256_block"),
            Elements::Sha256Ctx8Add1 => f.write_str("sha_256_ctx_8_add_1"),
            Elements::Sha256Ctx8Add128 => f.write_str("sha_256_ctx_8_add_128"),
            Elements::Sha256Ctx8Add16 => f.write_str("sha_256_ctx_8_add_16"),
            Elements::Sha256Ctx8Add2 => f.write_str("sha_256_ctx_8_add_2"),
            Elements::Sha256Ctx8Add256 => f.write_str("sha_256_ctx_8_add_256"),
            Elements::Sha256Ctx8Add32 => f.write_str("sha_256_ctx_8_add_32"),
            Elements::Sha256Ctx8Add4 => f.write_str("sha_256_ctx_8_add_4"),
            Elements::Sha256Ctx8Add512 => f.write_str("sha_256_ctx_8_add_512"),
            Elements::Sha256Ctx8Add64 => f.write_str("sha_256_ctx_8_add_64"),
            Elements::Sha256Ctx8Add8 => f.write_str("sha_256_ctx_8_add_8"),
            Elements::Sha256Ctx8AddBuffer511 => f.write_str("sha_256_ctx_8_add_buffer_511"),
            Elements::Sha256Ctx8Finalize => f.write_str("sha_256_ctx_8_finalize"),
            Elements::Sha256Ctx8Init => f.write_str("sha_256_ctx_8_init"),
            Elements::Sha256Iv => f.write_str("sha_256_iv"),
            Elements::SigAllHash => f.write_str("sig_all_hash"),
            Elements::Some1 => f.write_str("some_1"),
            Elements::Some16 => f.write_str("some_16"),
            Elements::Some32 => f.write_str("some_32"),
            Elements::Some64 => f.write_str("some_64"),
            Elements::Some8 => f.write_str("some_8"),
            Elements::Subtract16 => f.write_str("subtract_16"),
            Elements::Subtract32 => f.write_str("subtract_32"),
            Elements::Subtract64 => f.write_str("subtract_64"),
            Elements::Subtract8 => f.write_str("subtract_8"),
            Elements::Swu => f.write_str("swu"),
            Elements::TapEnvHash => f.write_str("tap_env_hash"),
            Elements::TapleafHash => f.write_str("tapleaf_hash"),
            Elements::TapleafVersion => f.write_str("tapleaf_version"),
            Elements::Tappath => f.write_str("tappath"),
            Elements::TappathHash => f.write_str("tappath_hash"),
            Elements::TotalFee => f.write_str("total_fee"),
            Elements::TransactionId => f.write_str("transaction_id"),
            Elements::TxHash => f.write_str("tx_hash"),
            Elements::TxIsFinal => f.write_str("tx_is_final"),
            Elements::TxLockDistance => f.write_str("tx_lock_distance"),
            Elements::TxLockDuration => f.write_str("tx_lock_duration"),
            Elements::TxLockHeight => f.write_str("tx_lock_height"),
            Elements::TxLockTime => f.write_str("tx_lock_time"),
            Elements::Verify => f.write_str("verify"),
            Elements::Version => f.write_str("version"),
            Elements::Xor1 => f.write_str("xor_1"),
            Elements::Xor16 => f.write_str("xor_16"),
            Elements::Xor32 => f.write_str("xor_32"),
            Elements::Xor64 => f.write_str("xor_64"),
            Elements::Xor8 => f.write_str("xor_8"),
            Elements::XorXor1 => f.write_str("xor_xor_1"),
            Elements::XorXor16 => f.write_str("xor_xor_16"),
            Elements::XorXor32 => f.write_str("xor_xor_32"),
            Elements::XorXor64 => f.write_str("xor_xor_64"),
            Elements::XorXor8 => f.write_str("xor_xor_8"),
        }
    }
}

impl str::FromStr for Elements {
    type Err = crate::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "add_16" => Ok(Elements::Add16),
            "add_32" => Ok(Elements::Add32),
            "add_64" => Ok(Elements::Add64),
            "add_8" => Ok(Elements::Add8),
            "all_16" => Ok(Elements::All16),
            "all_32" => Ok(Elements::All32),
            "all_64" => Ok(Elements::All64),
            "all_8" => Ok(Elements::All8),
            "and_1" => Ok(Elements::And1),
            "and_16" => Ok(Elements::And16),
            "and_32" => Ok(Elements::And32),
            "and_64" => Ok(Elements::And64),
            "and_8" => Ok(Elements::And8),
            "annex_hash" => Ok(Elements::AnnexHash),
            "asset_amount_hash" => Ok(Elements::AssetAmountHash),
            "bip_0340_verify" => Ok(Elements::Bip0340Verify),
            "build_tapbranch" => Ok(Elements::BuildTapbranch),
            "build_tapleaf_simplicity" => Ok(Elements::BuildTapleafSimplicity),
            "calculate_asset" => Ok(Elements::CalculateAsset),
            "calculate_confidential_token" => Ok(Elements::CalculateConfidentialToken),
            "calculate_explicit_token" => Ok(Elements::CalculateExplicitToken),
            "calculate_issuance_entropy" => Ok(Elements::CalculateIssuanceEntropy),
            "ch_1" => Ok(Elements::Ch1),
            "ch_16" => Ok(Elements::Ch16),
            "ch_32" => Ok(Elements::Ch32),
            "ch_64" => Ok(Elements::Ch64),
            "ch_8" => Ok(Elements::Ch8),
            "check_lock_distance" => Ok(Elements::CheckLockDistance),
            "check_lock_duration" => Ok(Elements::CheckLockDuration),
            "check_lock_height" => Ok(Elements::CheckLockHeight),
            "check_lock_time" => Ok(Elements::CheckLockTime),
            "check_sig_verify" => Ok(Elements::CheckSigVerify),
            "complement_1" => Ok(Elements::Complement1),
            "complement_16" => Ok(Elements::Complement16),
            "complement_32" => Ok(Elements::Complement32),
            "complement_64" => Ok(Elements::Complement64),
            "complement_8" => Ok(Elements::Complement8),
            "current_amount" => Ok(Elements::CurrentAmount),
            "current_annex_hash" => Ok(Elements::CurrentAnnexHash),
            "current_asset" => Ok(Elements::CurrentAsset),
            "current_index" => Ok(Elements::CurrentIndex),
            "current_issuance_asset_amount" => Ok(Elements::CurrentIssuanceAssetAmount),
            "current_issuance_asset_proof" => Ok(Elements::CurrentIssuanceAssetProof),
            "current_issuance_token_amount" => Ok(Elements::CurrentIssuanceTokenAmount),
            "current_issuance_token_proof" => Ok(Elements::CurrentIssuanceTokenProof),
            "current_new_issuance_contract" => Ok(Elements::CurrentNewIssuanceContract),
            "current_pegin" => Ok(Elements::CurrentPegin),
            "current_prev_outpoint" => Ok(Elements::CurrentPrevOutpoint),
            "current_reissuance_blinding" => Ok(Elements::CurrentReissuanceBlinding),
            "current_reissuance_entropy" => Ok(Elements::CurrentReissuanceEntropy),
            "current_script_hash" => Ok(Elements::CurrentScriptHash),
            "current_script_sig_hash" => Ok(Elements::CurrentScriptSigHash),
            "current_sequence" => Ok(Elements::CurrentSequence),
            "decompress" => Ok(Elements::Decompress),
            "decrement_16" => Ok(Elements::Decrement16),
            "decrement_32" => Ok(Elements::Decrement32),
            "decrement_64" => Ok(Elements::Decrement64),
            "decrement_8" => Ok(Elements::Decrement8),
            "div_mod_128_64" => Ok(Elements::DivMod128_64),
            "div_mod_16" => Ok(Elements::DivMod16),
            "div_mod_32" => Ok(Elements::DivMod32),
            "div_mod_64" => Ok(Elements::DivMod64),
            "div_mod_8" => Ok(Elements::DivMod8),
            "divide_16" => Ok(Elements::Divide16),
            "divide_32" => Ok(Elements::Divide32),
            "divide_64" => Ok(Elements::Divide64),
            "divide_8" => Ok(Elements::Divide8),
            "divides_16" => Ok(Elements::Divides16),
            "divides_32" => Ok(Elements::Divides32),
            "divides_64" => Ok(Elements::Divides64),
            "divides_8" => Ok(Elements::Divides8),
            "eq_1" => Ok(Elements::Eq1),
            "eq_16" => Ok(Elements::Eq16),
            "eq_256" => Ok(Elements::Eq256),
            "eq_32" => Ok(Elements::Eq32),
            "eq_64" => Ok(Elements::Eq64),
            "eq_8" => Ok(Elements::Eq8),
            "fe_add" => Ok(Elements::FeAdd),
            "fe_invert" => Ok(Elements::FeInvert),
            "fe_is_odd" => Ok(Elements::FeIsOdd),
            "fe_is_zero" => Ok(Elements::FeIsZero),
            "fe_multiply" => Ok(Elements::FeMultiply),
            "fe_multiply_beta" => Ok(Elements::FeMultiplyBeta),
            "fe_negate" => Ok(Elements::FeNegate),
            "fe_normalize" => Ok(Elements::FeNormalize),
            "fe_square" => Ok(Elements::FeSquare),
            "fe_square_root" => Ok(Elements::FeSquareRoot),
            "full_add_16" => Ok(Elements::FullAdd16),
            "full_add_32" => Ok(Elements::FullAdd32),
            "full_add_64" => Ok(Elements::FullAdd64),
            "full_add_8" => Ok(Elements::FullAdd8),
            "full_decrement_16" => Ok(Elements::FullDecrement16),
            "full_decrement_32" => Ok(Elements::FullDecrement32),
            "full_decrement_64" => Ok(Elements::FullDecrement64),
            "full_decrement_8" => Ok(Elements::FullDecrement8),
            "full_increment_16" => Ok(Elements::FullIncrement16),
            "full_increment_32" => Ok(Elements::FullIncrement32),
            "full_increment_64" => Ok(Elements::FullIncrement64),
            "full_increment_8" => Ok(Elements::FullIncrement8),
            "full_left_shift_16_1" => Ok(Elements::FullLeftShift16_1),
            "full_left_shift_16_2" => Ok(Elements::FullLeftShift16_2),
            "full_left_shift_16_4" => Ok(Elements::FullLeftShift16_4),
            "full_left_shift_16_8" => Ok(Elements::FullLeftShift16_8),
            "full_left_shift_32_1" => Ok(Elements::FullLeftShift32_1),
            "full_left_shift_32_16" => Ok(Elements::FullLeftShift32_16),
            "full_left_shift_32_2" => Ok(Elements::FullLeftShift32_2),
            "full_left_shift_32_4" => Ok(Elements::FullLeftShift32_4),
            "full_left_shift_32_8" => Ok(Elements::FullLeftShift32_8),
            "full_left_shift_64_1" => Ok(Elements::FullLeftShift64_1),
            "full_left_shift_64_16" => Ok(Elements::FullLeftShift64_16),
            "full_left_shift_64_2" => Ok(Elements::FullLeftShift64_2),
            "full_left_shift_64_32" => Ok(Elements::FullLeftShift64_32),
            "full_left_shift_64_4" => Ok(Elements::FullLeftShift64_4),
            "full_left_shift_64_8" => Ok(Elements::FullLeftShift64_8),
            "full_left_shift_8_1" => Ok(Elements::FullLeftShift8_1),
            "full_left_shift_8_2" => Ok(Elements::FullLeftShift8_2),
            "full_left_shift_8_4" => Ok(Elements::FullLeftShift8_4),
            "full_multiply_16" => Ok(Elements::FullMultiply16),
            "full_multiply_32" => Ok(Elements::FullMultiply32),
            "full_multiply_64" => Ok(Elements::FullMultiply64),
            "full_multiply_8" => Ok(Elements::FullMultiply8),
            "full_right_shift_16_1" => Ok(Elements::FullRightShift16_1),
            "full_right_shift_16_2" => Ok(Elements::FullRightShift16_2),
            "full_right_shift_16_4" => Ok(Elements::FullRightShift16_4),
            "full_right_shift_16_8" => Ok(Elements::FullRightShift16_8),
            "full_right_shift_32_1" => Ok(Elements::FullRightShift32_1),
            "full_right_shift_32_16" => Ok(Elements::FullRightShift32_16),
            "full_right_shift_32_2" => Ok(Elements::FullRightShift32_2),
            "full_right_shift_32_4" => Ok(Elements::FullRightShift32_4),
            "full_right_shift_32_8" => Ok(Elements::FullRightShift32_8),
            "full_right_shift_64_1" => Ok(Elements::FullRightShift64_1),
            "full_right_shift_64_16" => Ok(Elements::FullRightShift64_16),
            "full_right_shift_64_2" => Ok(Elements::FullRightShift64_2),
            "full_right_shift_64_32" => Ok(Elements::FullRightShift64_32),
            "full_right_shift_64_4" => Ok(Elements::FullRightShift64_4),
            "full_right_shift_64_8" => Ok(Elements::FullRightShift64_8),
            "full_right_shift_8_1" => Ok(Elements::FullRightShift8_1),
            "full_right_shift_8_2" => Ok(Elements::FullRightShift8_2),
            "full_right_shift_8_4" => Ok(Elements::FullRightShift8_4),
            "full_subtract_16" => Ok(Elements::FullSubtract16),
            "full_subtract_32" => Ok(Elements::FullSubtract32),
            "full_subtract_64" => Ok(Elements::FullSubtract64),
            "full_subtract_8" => Ok(Elements::FullSubtract8),
            "ge_is_on_curve" => Ok(Elements::GeIsOnCurve),
            "ge_negate" => Ok(Elements::GeNegate),
            "gej_add" => Ok(Elements::GejAdd),
            "gej_double" => Ok(Elements::GejDouble),
            "gej_equiv" => Ok(Elements::GejEquiv),
            "gej_ge_add" => Ok(Elements::GejGeAdd),
            "gej_ge_add_ex" => Ok(Elements::GejGeAddEx),
            "gej_ge_equiv" => Ok(Elements::GejGeEquiv),
            "gej_infinity" => Ok(Elements::GejInfinity),
            "gej_is_infinity" => Ok(Elements::GejIsInfinity),
            "gej_is_on_curve" => Ok(Elements::GejIsOnCurve),
            "gej_negate" => Ok(Elements::GejNegate),
            "gej_normalize" => Ok(Elements::GejNormalize),
            "gej_rescale" => Ok(Elements::GejRescale),
            "gej_x_equiv" => Ok(Elements::GejXEquiv),
            "gej_y_is_odd" => Ok(Elements::GejYIsOdd),
            "generate" => Ok(Elements::Generate),
            "genesis_block_hash" => Ok(Elements::GenesisBlockHash),
            "hash_to_curve" => Ok(Elements::HashToCurve),
            "high_1" => Ok(Elements::High1),
            "high_16" => Ok(Elements::High16),
            "high_32" => Ok(Elements::High32),
            "high_64" => Ok(Elements::High64),
            "high_8" => Ok(Elements::High8),
            "increment_16" => Ok(Elements::Increment16),
            "increment_32" => Ok(Elements::Increment32),
            "increment_64" => Ok(Elements::Increment64),
            "increment_8" => Ok(Elements::Increment8),
            "input_amount" => Ok(Elements::InputAmount),
            "input_amounts_hash" => Ok(Elements::InputAmountsHash),
            "input_annex_hash" => Ok(Elements::InputAnnexHash),
            "input_annexes_hash" => Ok(Elements::InputAnnexesHash),
            "input_asset" => Ok(Elements::InputAsset),
            "input_hash" => Ok(Elements::InputHash),
            "input_outpoints_hash" => Ok(Elements::InputOutpointsHash),
            "input_pegin" => Ok(Elements::InputPegin),
            "input_prev_outpoint" => Ok(Elements::InputPrevOutpoint),
            "input_script_hash" => Ok(Elements::InputScriptHash),
            "input_script_sig_hash" => Ok(Elements::InputScriptSigHash),
            "input_script_sigs_hash" => Ok(Elements::InputScriptSigsHash),
            "input_scripts_hash" => Ok(Elements::InputScriptsHash),
            "input_sequence" => Ok(Elements::InputSequence),
            "input_sequences_hash" => Ok(Elements::InputSequencesHash),
            "input_utxo_hash" => Ok(Elements::InputUtxoHash),
            "input_utxos_hash" => Ok(Elements::InputUtxosHash),
            "inputs_hash" => Ok(Elements::InputsHash),
            "internal_key" => Ok(Elements::InternalKey),
            "is_one_16" => Ok(Elements::IsOne16),
            "is_one_32" => Ok(Elements::IsOne32),
            "is_one_64" => Ok(Elements::IsOne64),
            "is_one_8" => Ok(Elements::IsOne8),
            "is_zero_16" => Ok(Elements::IsZero16),
            "is_zero_32" => Ok(Elements::IsZero32),
            "is_zero_64" => Ok(Elements::IsZero64),
            "is_zero_8" => Ok(Elements::IsZero8),
            "issuance" => Ok(Elements::Issuance),
            "issuance_asset" => Ok(Elements::IssuanceAsset),
            "issuance_asset_amount" => Ok(Elements::IssuanceAssetAmount),
            "issuance_asset_amounts_hash" => Ok(Elements::IssuanceAssetAmountsHash),
            "issuance_asset_proof" => Ok(Elements::IssuanceAssetProof),
            "issuance_blinding_entropy_hash" => Ok(Elements::IssuanceBlindingEntropyHash),
            "issuance_entropy" => Ok(Elements::IssuanceEntropy),
            "issuance_hash" => Ok(Elements::IssuanceHash),
            "issuance_range_proofs_hash" => Ok(Elements::IssuanceRangeProofsHash),
            "issuance_token" => Ok(Elements::IssuanceToken),
            "issuance_token_amount" => Ok(Elements::IssuanceTokenAmount),
            "issuance_token_amounts_hash" => Ok(Elements::IssuanceTokenAmountsHash),
            "issuance_token_proof" => Ok(Elements::IssuanceTokenProof),
            "issuances_hash" => Ok(Elements::IssuancesHash),
            "lbtc_asset" => Ok(Elements::LbtcAsset),
            "le_16" => Ok(Elements::Le16),
            "le_32" => Ok(Elements::Le32),
            "le_64" => Ok(Elements::Le64),
            "le_8" => Ok(Elements::Le8),
            "left_extend_16_32" => Ok(Elements::LeftExtend16_32),
            "left_extend_16_64" => Ok(Elements::LeftExtend16_64),
            "left_extend_1_16" => Ok(Elements::LeftExtend1_16),
            "left_extend_1_32" => Ok(Elements::LeftExtend1_32),
            "left_extend_1_64" => Ok(Elements::LeftExtend1_64),
            "left_extend_1_8" => Ok(Elements::LeftExtend1_8),
            "left_extend_32_64" => Ok(Elements::LeftExtend32_64),
            "left_extend_8_16" => Ok(Elements::LeftExtend8_16),
            "left_extend_8_32" => Ok(Elements::LeftExtend8_32),
            "left_extend_8_64" => Ok(Elements::LeftExtend8_64),
            "left_pad_high_16_32" => Ok(Elements::LeftPadHigh16_32),
            "left_pad_high_16_64" => Ok(Elements::LeftPadHigh16_64),
            "left_pad_high_1_16" => Ok(Elements::LeftPadHigh1_16),
            "left_pad_high_1_32" => Ok(Elements::LeftPadHigh1_32),
            "left_pad_high_1_64" => Ok(Elements::LeftPadHigh1_64),
            "left_pad_high_1_8" => Ok(Elements::LeftPadHigh1_8),
            "left_pad_high_32_64" => Ok(Elements::LeftPadHigh32_64),
            "left_pad_high_8_16" => Ok(Elements::LeftPadHigh8_16),
            "left_pad_high_8_32" => Ok(Elements::LeftPadHigh8_32),
            "left_pad_high_8_64" => Ok(Elements::LeftPadHigh8_64),
            "left_pad_low_16_32" => Ok(Elements::LeftPadLow16_32),
            "left_pad_low_16_64" => Ok(Elements::LeftPadLow16_64),
            "left_pad_low_1_16" => Ok(Elements::LeftPadLow1_16),
            "left_pad_low_1_32" => Ok(Elements::LeftPadLow1_32),
            "left_pad_low_1_64" => Ok(Elements::LeftPadLow1_64),
            "left_pad_low_1_8" => Ok(Elements::LeftPadLow1_8),
            "left_pad_low_32_64" => Ok(Elements::LeftPadLow32_64),
            "left_pad_low_8_16" => Ok(Elements::LeftPadLow8_16),
            "left_pad_low_8_32" => Ok(Elements::LeftPadLow8_32),
            "left_pad_low_8_64" => Ok(Elements::LeftPadLow8_64),
            "left_rotate_16" => Ok(Elements::LeftRotate16),
            "left_rotate_32" => Ok(Elements::LeftRotate32),
            "left_rotate_64" => Ok(Elements::LeftRotate64),
            "left_rotate_8" => Ok(Elements::LeftRotate8),
            "left_shift_16" => Ok(Elements::LeftShift16),
            "left_shift_32" => Ok(Elements::LeftShift32),
            "left_shift_64" => Ok(Elements::LeftShift64),
            "left_shift_8" => Ok(Elements::LeftShift8),
            "left_shift_with_16" => Ok(Elements::LeftShiftWith16),
            "left_shift_with_32" => Ok(Elements::LeftShiftWith32),
            "left_shift_with_64" => Ok(Elements::LeftShiftWith64),
            "left_shift_with_8" => Ok(Elements::LeftShiftWith8),
            "leftmost_16_1" => Ok(Elements::Leftmost16_1),
            "leftmost_16_2" => Ok(Elements::Leftmost16_2),
            "leftmost_16_4" => Ok(Elements::Leftmost16_4),
            "leftmost_16_8" => Ok(Elements::Leftmost16_8),
            "leftmost_32_1" => Ok(Elements::Leftmost32_1),
            "leftmost_32_16" => Ok(Elements::Leftmost32_16),
            "leftmost_32_2" => Ok(Elements::Leftmost32_2),
            "leftmost_32_4" => Ok(Elements::Leftmost32_4),
            "leftmost_32_8" => Ok(Elements::Leftmost32_8),
            "leftmost_64_1" => Ok(Elements::Leftmost64_1),
            "leftmost_64_16" => Ok(Elements::Leftmost64_16),
            "leftmost_64_2" => Ok(Elements::Leftmost64_2),
            "leftmost_64_32" => Ok(Elements::Leftmost64_32),
            "leftmost_64_4" => Ok(Elements::Leftmost64_4),
            "leftmost_64_8" => Ok(Elements::Leftmost64_8),
            "leftmost_8_1" => Ok(Elements::Leftmost8_1),
            "leftmost_8_2" => Ok(Elements::Leftmost8_2),
            "leftmost_8_4" => Ok(Elements::Leftmost8_4),
            "linear_combination_1" => Ok(Elements::LinearCombination1),
            "linear_verify_1" => Ok(Elements::LinearVerify1),
            "lock_time" => Ok(Elements::LockTime),
            "low_1" => Ok(Elements::Low1),
            "low_16" => Ok(Elements::Low16),
            "low_32" => Ok(Elements::Low32),
            "low_64" => Ok(Elements::Low64),
            "low_8" => Ok(Elements::Low8),
            "lt_16" => Ok(Elements::Lt16),
            "lt_32" => Ok(Elements::Lt32),
            "lt_64" => Ok(Elements::Lt64),
            "lt_8" => Ok(Elements::Lt8),
            "maj_1" => Ok(Elements::Maj1),
            "maj_16" => Ok(Elements::Maj16),
            "maj_32" => Ok(Elements::Maj32),
            "maj_64" => Ok(Elements::Maj64),
            "maj_8" => Ok(Elements::Maj8),
            "max_16" => Ok(Elements::Max16),
            "max_32" => Ok(Elements::Max32),
            "max_64" => Ok(Elements::Max64),
            "max_8" => Ok(Elements::Max8),
            "median_16" => Ok(Elements::Median16),
            "median_32" => Ok(Elements::Median32),
            "median_64" => Ok(Elements::Median64),
            "median_8" => Ok(Elements::Median8),
            "min_16" => Ok(Elements::Min16),
            "min_32" => Ok(Elements::Min32),
            "min_64" => Ok(Elements::Min64),
            "min_8" => Ok(Elements::Min8),
            "modulo_16" => Ok(Elements::Modulo16),
            "modulo_32" => Ok(Elements::Modulo32),
            "modulo_64" => Ok(Elements::Modulo64),
            "modulo_8" => Ok(Elements::Modulo8),
            "multiply_16" => Ok(Elements::Multiply16),
            "multiply_32" => Ok(Elements::Multiply32),
            "multiply_64" => Ok(Elements::Multiply64),
            "multiply_8" => Ok(Elements::Multiply8),
            "negate_16" => Ok(Elements::Negate16),
            "negate_32" => Ok(Elements::Negate32),
            "negate_64" => Ok(Elements::Negate64),
            "negate_8" => Ok(Elements::Negate8),
            "new_issuance_contract" => Ok(Elements::NewIssuanceContract),
            "nonce_hash" => Ok(Elements::NonceHash),
            "num_inputs" => Ok(Elements::NumInputs),
            "num_outputs" => Ok(Elements::NumOutputs),
            "one_16" => Ok(Elements::One16),
            "one_32" => Ok(Elements::One32),
            "one_64" => Ok(Elements::One64),
            "one_8" => Ok(Elements::One8),
            "or_1" => Ok(Elements::Or1),
            "or_16" => Ok(Elements::Or16),
            "or_32" => Ok(Elements::Or32),
            "or_64" => Ok(Elements::Or64),
            "or_8" => Ok(Elements::Or8),
            "outpoint_hash" => Ok(Elements::OutpointHash),
            "output_amount" => Ok(Elements::OutputAmount),
            "output_amounts_hash" => Ok(Elements::OutputAmountsHash),
            "output_asset" => Ok(Elements::OutputAsset),
            "output_hash" => Ok(Elements::OutputHash),
            "output_is_fee" => Ok(Elements::OutputIsFee),
            "output_nonce" => Ok(Elements::OutputNonce),
            "output_nonces_hash" => Ok(Elements::OutputNoncesHash),
            "output_null_datum" => Ok(Elements::OutputNullDatum),
            "output_range_proof" => Ok(Elements::OutputRangeProof),
            "output_range_proofs_hash" => Ok(Elements::OutputRangeProofsHash),
            "output_script_hash" => Ok(Elements::OutputScriptHash),
            "output_scripts_hash" => Ok(Elements::OutputScriptsHash),
            "output_surjection_proof" => Ok(Elements::OutputSurjectionProof),
            "output_surjection_proofs_hash" => Ok(Elements::OutputSurjectionProofsHash),
            "outputs_hash" => Ok(Elements::OutputsHash),
            "parse_lock" => Ok(Elements::ParseLock),
            "parse_sequence" => Ok(Elements::ParseSequence),
            "point_verify_1" => Ok(Elements::PointVerify1),
            "reissuance_blinding" => Ok(Elements::ReissuanceBlinding),
            "reissuance_entropy" => Ok(Elements::ReissuanceEntropy),
            "right_extend_16_32" => Ok(Elements::RightExtend16_32),
            "right_extend_16_64" => Ok(Elements::RightExtend16_64),
            "right_extend_32_64" => Ok(Elements::RightExtend32_64),
            "right_extend_8_16" => Ok(Elements::RightExtend8_16),
            "right_extend_8_32" => Ok(Elements::RightExtend8_32),
            "right_extend_8_64" => Ok(Elements::RightExtend8_64),
            "right_pad_high_16_32" => Ok(Elements::RightPadHigh16_32),
            "right_pad_high_16_64" => Ok(Elements::RightPadHigh16_64),
            "right_pad_high_1_16" => Ok(Elements::RightPadHigh1_16),
            "right_pad_high_1_32" => Ok(Elements::RightPadHigh1_32),
            "right_pad_high_1_64" => Ok(Elements::RightPadHigh1_64),
            "right_pad_high_1_8" => Ok(Elements::RightPadHigh1_8),
            "right_pad_high_32_64" => Ok(Elements::RightPadHigh32_64),
            "right_pad_high_8_16" => Ok(Elements::RightPadHigh8_16),
            "right_pad_high_8_32" => Ok(Elements::RightPadHigh8_32),
            "right_pad_high_8_64" => Ok(Elements::RightPadHigh8_64),
            "right_pad_low_16_32" => Ok(Elements::RightPadLow16_32),
            "right_pad_low_16_64" => Ok(Elements::RightPadLow16_64),
            "right_pad_low_1_16" => Ok(Elements::RightPadLow1_16),
            "right_pad_low_1_32" => Ok(Elements::RightPadLow1_32),
            "right_pad_low_1_64" => Ok(Elements::RightPadLow1_64),
            "right_pad_low_1_8" => Ok(Elements::RightPadLow1_8),
            "right_pad_low_32_64" => Ok(Elements::RightPadLow32_64),
            "right_pad_low_8_16" => Ok(Elements::RightPadLow8_16),
            "right_pad_low_8_32" => Ok(Elements::RightPadLow8_32),
            "right_pad_low_8_64" => Ok(Elements::RightPadLow8_64),
            "right_rotate_16" => Ok(Elements::RightRotate16),
            "right_rotate_32" => Ok(Elements::RightRotate32),
            "right_rotate_64" => Ok(Elements::RightRotate64),
            "right_rotate_8" => Ok(Elements::RightRotate8),
            "right_shift_16" => Ok(Elements::RightShift16),
            "right_shift_32" => Ok(Elements::RightShift32),
            "right_shift_64" => Ok(Elements::RightShift64),
            "right_shift_8" => Ok(Elements::RightShift8),
            "right_shift_with_16" => Ok(Elements::RightShiftWith16),
            "right_shift_with_32" => Ok(Elements::RightShiftWith32),
            "right_shift_with_64" => Ok(Elements::RightShiftWith64),
            "right_shift_with_8" => Ok(Elements::RightShiftWith8),
            "rightmost_16_1" => Ok(Elements::Rightmost16_1),
            "rightmost_16_2" => Ok(Elements::Rightmost16_2),
            "rightmost_16_4" => Ok(Elements::Rightmost16_4),
            "rightmost_16_8" => Ok(Elements::Rightmost16_8),
            "rightmost_32_1" => Ok(Elements::Rightmost32_1),
            "rightmost_32_16" => Ok(Elements::Rightmost32_16),
            "rightmost_32_2" => Ok(Elements::Rightmost32_2),
            "rightmost_32_4" => Ok(Elements::Rightmost32_4),
            "rightmost_32_8" => Ok(Elements::Rightmost32_8),
            "rightmost_64_1" => Ok(Elements::Rightmost64_1),
            "rightmost_64_16" => Ok(Elements::Rightmost64_16),
            "rightmost_64_2" => Ok(Elements::Rightmost64_2),
            "rightmost_64_32" => Ok(Elements::Rightmost64_32),
            "rightmost_64_4" => Ok(Elements::Rightmost64_4),
            "rightmost_64_8" => Ok(Elements::Rightmost64_8),
            "rightmost_8_1" => Ok(Elements::Rightmost8_1),
            "rightmost_8_2" => Ok(Elements::Rightmost8_2),
            "rightmost_8_4" => Ok(Elements::Rightmost8_4),
            "scalar_add" => Ok(Elements::ScalarAdd),
            "scalar_invert" => Ok(Elements::ScalarInvert),
            "scalar_is_zero" => Ok(Elements::ScalarIsZero),
            "scalar_multiply" => Ok(Elements::ScalarMultiply),
            "scalar_multiply_lambda" => Ok(Elements::ScalarMultiplyLambda),
            "scalar_negate" => Ok(Elements::ScalarNegate),
            "scalar_normalize" => Ok(Elements::ScalarNormalize),
            "scalar_square" => Ok(Elements::ScalarSquare),
            "scale" => Ok(Elements::Scale),
            "script_cmr" => Ok(Elements::ScriptCMR),
            "sha_256_block" => Ok(Elements::Sha256Block),
            "sha_256_ctx_8_add_1" => Ok(Elements::Sha256Ctx8Add1),
            "sha_256_ctx_8_add_128" => Ok(Elements::Sha256Ctx8Add128),
            "sha_256_ctx_8_add_16" => Ok(Elements::Sha256Ctx8Add16),
            "sha_256_ctx_8_add_2" => Ok(Elements::Sha256Ctx8Add2),
            "sha_256_ctx_8_add_256" => Ok(Elements::Sha256Ctx8Add256),
            "sha_256_ctx_8_add_32" => Ok(Elements::Sha256Ctx8Add32),
            "sha_256_ctx_8_add_4" => Ok(Elements::Sha256Ctx8Add4),
            "sha_256_ctx_8_add_512" => Ok(Elements::Sha256Ctx8Add512),
            "sha_256_ctx_8_add_64" => Ok(Elements::Sha256Ctx8Add64),
            "sha_256_ctx_8_add_8" => Ok(Elements::Sha256Ctx8Add8),
            "sha_256_ctx_8_add_buffer_511" => Ok(Elements::Sha256Ctx8AddBuffer511),
            "sha_256_ctx_8_finalize" => Ok(Elements::Sha256Ctx8Finalize),
            "sha_256_ctx_8_init" => Ok(Elements::Sha256Ctx8Init),
            "sha_256_iv" => Ok(Elements::Sha256Iv),
            "sig_all_hash" => Ok(Elements::SigAllHash),
            "some_1" => Ok(Elements::Some1),
            "some_16" => Ok(Elements::Some16),
            "some_32" => Ok(Elements::Some32),
            "some_64" => Ok(Elements::Some64),
            "some_8" => Ok(Elements::Some8),
            "subtract_16" => Ok(Elements::Subtract16),
            "subtract_32" => Ok(Elements::Subtract32),
            "subtract_64" => Ok(Elements::Subtract64),
            "subtract_8" => Ok(Elements::Subtract8),
            "swu" => Ok(Elements::Swu),
            "tap_env_hash" => Ok(Elements::TapEnvHash),
            "tapleaf_hash" => Ok(Elements::TapleafHash),
            "tapleaf_version" => Ok(Elements::TapleafVersion),
            "tappath" => Ok(Elements::Tappath),
            "tappath_hash" => Ok(Elements::TappathHash),
            "total_fee" => Ok(Elements::TotalFee),
            "transaction_id" => Ok(Elements::TransactionId),
            "tx_hash" => Ok(Elements::TxHash),
            "tx_is_final" => Ok(Elements::TxIsFinal),
            "tx_lock_distance" => Ok(Elements::TxLockDistance),
            "tx_lock_duration" => Ok(Elements::TxLockDuration),
            "tx_lock_height" => Ok(Elements::TxLockHeight),
            "tx_lock_time" => Ok(Elements::TxLockTime),
            "verify" => Ok(Elements::Verify),
            "version" => Ok(Elements::Version),
            "xor_1" => Ok(Elements::Xor1),
            "xor_16" => Ok(Elements::Xor16),
            "xor_32" => Ok(Elements::Xor32),
            "xor_64" => Ok(Elements::Xor64),
            "xor_8" => Ok(Elements::Xor8),
            "xor_xor_1" => Ok(Elements::XorXor1),
            "xor_xor_16" => Ok(Elements::XorXor16),
            "xor_xor_32" => Ok(Elements::XorXor32),
            "xor_xor_64" => Ok(Elements::XorXor64),
            "xor_xor_8" => Ok(Elements::XorXor8),
            x => Err(crate::Error::InvalidJetName(x.to_owned())),
        }
    }
}
