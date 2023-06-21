/* This file has been automatically generated. */

use crate::jet::type_name::TypeName;
use crate::jet::Jet;
use crate::merkle::cmr::Cmr;
use crate::decode_bits;
use crate::{decode, BitIter, BitWriter};
use bitcoin_hashes::sha256::Midstate;
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
    GenesisBlockHash,
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
    InputOutpointsHash,
    InputPegin,
    InputPrevOutpoint,
    InputScriptHash,
    InputScriptSigHash,
    InputScriptSigsHash,
    InputScriptsHash,
    InputSequence,
    InputSequencesHash,
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
    IssuanceRangeProofsHash,
    IssuanceToken,
    IssuanceTokenAmount,
    IssuanceTokenAmountsHash,
    IssuanceTokenProof,
    IssuancesHash,
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
    NewIssuanceContract,
    NonceHash,
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
    OutpointHash,
    OutputAmount,
    OutputAmountsHash,
    OutputAsset,
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
    Some16,
    Some32,
    Some64,
    Some8,
    Subtract16,
    Subtract32,
    Subtract64,
    Subtract8,
    TapEnvHash,
    Tapbranch,
    TapbranchHash,
    TapleafHash,
    TapleafVersion,
    TxHash,
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

impl Jet for Elements {

    type Environment = ElementsEnv;
    type CJetEnvironment = CElementsTxEnv;

    fn c_jet_env<'env>(&self, env: &'env Self::Environment) -> &'env Self::CJetEnvironment {
        env.c_tx_env()
    }

    fn cmr(&self) -> Cmr {
        let bytes = match self {
            Elements::Add16 => [
                0x55, 0xcf, 0x8c, 0xe6, 0x6e, 0x27, 0x5d, 0x01, 0xaf, 0xc1, 0xfb, 0x35, 0x10, 0xd0,
                0xb5, 0x3f, 0xab, 0x54, 0xa4, 0x78, 0x2a, 0x3a, 0x48, 0xdd, 0xac, 0x49, 0x4e, 0x96,
                0x72, 0xfa, 0x04, 0x5d,
            ],
            Elements::Add32 => [
                0xfa, 0x87, 0x4e, 0x40, 0x62, 0x72, 0xc9, 0x0c, 0xf0, 0xb8, 0x7b, 0x1e, 0x7b, 0x89,
                0xf6, 0x61, 0xf3, 0x34, 0x1b, 0x0d, 0x61, 0x63, 0x5c, 0xf6, 0xf5, 0xa8, 0xa2, 0xa5,
                0x32, 0x88, 0xcf, 0xe4,
            ],
            Elements::Add64 => [
                0x12, 0x56, 0xbc, 0x57, 0x26, 0xcd, 0xb9, 0x54, 0x58, 0x21, 0xd3, 0xd9, 0x6e, 0x7f,
                0xc7, 0x32, 0x86, 0xd7, 0xdb, 0x23, 0xae, 0x65, 0x17, 0x10, 0xff, 0x05, 0xfc, 0xc0,
                0xa9, 0xb7, 0xc3, 0xe9,
            ],
            Elements::Add8 => [
                0xd9, 0x4b, 0x8f, 0xb6, 0xb6, 0x02, 0x5d, 0x8d, 0x02, 0x9d, 0xf1, 0xb0, 0xa5, 0x26,
                0xa4, 0x2b, 0x3e, 0xae, 0x94, 0xc8, 0xa9, 0x93, 0x6c, 0xe7, 0xe2, 0xa3, 0xb0, 0x00,
                0x2e, 0x7f, 0x44, 0x5d,
            ],
            Elements::All16 => [
                0xc2, 0x62, 0x4c, 0x7f, 0x52, 0x9d, 0x7f, 0x8e, 0x7c, 0xb9, 0x69, 0x24, 0xdf, 0x5c,
                0x48, 0xda, 0x2b, 0xf2, 0xd9, 0x27, 0x96, 0x66, 0x2b, 0x0c, 0x76, 0x78, 0x21, 0xe1,
                0x9e, 0x7a, 0x36, 0x00,
            ],
            Elements::All32 => [
                0x4c, 0x06, 0x0c, 0x8b, 0xd9, 0x70, 0x24, 0x41, 0x11, 0x74, 0x65, 0x60, 0xdd, 0x55,
                0xfb, 0xcd, 0x61, 0x04, 0xae, 0x34, 0x35, 0xaa, 0x43, 0x14, 0xe4, 0xac, 0x0e, 0x05,
                0x31, 0x20, 0x5e, 0x20,
            ],
            Elements::All64 => [
                0xf5, 0x2f, 0x9f, 0xeb, 0xc7, 0x11, 0x3b, 0xb1, 0xca, 0x14, 0xe3, 0xd2, 0xff, 0x9c,
                0x16, 0x1a, 0x59, 0x8d, 0xe5, 0xaf, 0xe9, 0x28, 0xc6, 0x91, 0x35, 0x5b, 0x9e, 0x86,
                0xbe, 0x13, 0x12, 0x67,
            ],
            Elements::All8 => [
                0xe5, 0x1e, 0x0e, 0x23, 0x93, 0x31, 0x5a, 0x99, 0xbf, 0xdb, 0x33, 0x58, 0x4a, 0x4e,
                0xb6, 0xcd, 0x6a, 0x3d, 0x03, 0xf1, 0xa7, 0x60, 0xea, 0xa0, 0x58, 0xb1, 0x4d, 0x33,
                0xd2, 0xd3, 0xf8, 0x2a,
            ],
            Elements::And16 => [
                0x25, 0x17, 0x21, 0x39, 0x33, 0x59, 0x3c, 0x62, 0x72, 0xd8, 0xaa, 0x0c, 0x1d, 0x99,
                0x6a, 0x3a, 0x1c, 0x56, 0xf8, 0x0d, 0xd3, 0x5b, 0x05, 0x44, 0x96, 0xf9, 0xc6, 0x37,
                0xf6, 0x9f, 0xf9, 0x21,
            ],
            Elements::And32 => [
                0x4f, 0x9d, 0x32, 0x6e, 0x9e, 0x51, 0x51, 0x52, 0x32, 0xf8, 0x47, 0x52, 0x33, 0xf7,
                0x50, 0xc0, 0xa5, 0x72, 0x12, 0x55, 0x80, 0x6d, 0x3d, 0x36, 0xb6, 0x3f, 0xfc, 0x14,
                0x99, 0x72, 0x51, 0x59,
            ],
            Elements::And64 => [
                0xdf, 0x3a, 0x07, 0xb2, 0x40, 0xeb, 0x8c, 0x53, 0x33, 0xf7, 0xb0, 0x80, 0x3d, 0x81,
                0xc8, 0x75, 0x69, 0xef, 0x59, 0x97, 0xdb, 0x56, 0xac, 0x37, 0xad, 0x63, 0x05, 0x9f,
                0x0c, 0xc0, 0x77, 0x9c,
            ],
            Elements::And8 => [
                0x14, 0x76, 0x9a, 0x23, 0xfc, 0xac, 0x95, 0x46, 0x22, 0x9d, 0x4c, 0x2e, 0xf3, 0xc5,
                0xd7, 0x79, 0xea, 0xf6, 0x24, 0x11, 0xfa, 0x4c, 0xa3, 0xa0, 0x6d, 0x60, 0xc4, 0x17,
                0xf2, 0x5b, 0x3e, 0x30,
            ],
            Elements::AnnexHash => [
                0x75, 0xb1, 0xd1, 0x0e, 0x8c, 0xfe, 0x48, 0xb1, 0xf3, 0x7c, 0xe0, 0xf0, 0x3e, 0xbd,
                0xe8, 0x40, 0x2c, 0xae, 0x19, 0xfe, 0xad, 0xc0, 0x1a, 0xec, 0x28, 0xd6, 0x5d, 0x2b,
                0x64, 0x45, 0xe7, 0x97,
            ],
            Elements::AssetAmountHash => [
                0x2b, 0xc0, 0x1a, 0xde, 0xd5, 0xd2, 0x61, 0x23, 0xd6, 0x89, 0x4a, 0x1f, 0xcb, 0x46,
                0xaa, 0x8b, 0x52, 0xa6, 0x0c, 0x0c, 0x64, 0xec, 0x2f, 0x34, 0x4f, 0xb6, 0x0b, 0x39,
                0xbf, 0x19, 0x42, 0x44,
            ],
            Elements::Bip0340Verify => [
                0xaf, 0x92, 0x4c, 0xbe, 0xc9, 0x41, 0xfd, 0x9b, 0x9e, 0x82, 0x93, 0xef, 0x90, 0x3e,
                0x23, 0x7b, 0x01, 0xdb, 0xcb, 0x34, 0x2b, 0x96, 0x10, 0xbe, 0xf9, 0x5f, 0x59, 0xb8,
                0x13, 0x6e, 0xf8, 0x04,
            ],
            Elements::BuildTapbranch => [
                0x7d, 0xe0, 0xe3, 0xc1, 0x53, 0xa8, 0x60, 0xa0, 0x92, 0x01, 0xc7, 0xe5, 0x94, 0x73,
                0xf0, 0x3a, 0xe2, 0x9a, 0xea, 0x64, 0xfb, 0x18, 0xe8, 0x79, 0xa9, 0x57, 0x67, 0xd7,
                0xa9, 0xb4, 0x94, 0xd0,
            ],
            Elements::BuildTapleafSimplicity => [
                0x1e, 0x61, 0x07, 0x28, 0x01, 0xf5, 0x3d, 0xe6, 0xc0, 0x09, 0xf4, 0xf4, 0x1b, 0xdf,
                0x49, 0xf0, 0xb3, 0x02, 0xed, 0xc7, 0x4c, 0xbf, 0x94, 0x25, 0x28, 0xf2, 0xaa, 0x19,
                0x5a, 0xc0, 0x4d, 0x50,
            ],
            Elements::CalculateAsset => [
                0xe2, 0x88, 0xfd, 0x8c, 0x56, 0xa1, 0x06, 0x16, 0x13, 0x19, 0x67, 0x2b, 0x38, 0x5a,
                0x42, 0x9e, 0x4d, 0x04, 0x3f, 0x83, 0xd2, 0x9d, 0x03, 0x9a, 0x44, 0xc3, 0xdf, 0x9c,
                0xc0, 0xab, 0x55, 0x4c,
            ],
            Elements::CalculateConfidentialToken => [
                0x82, 0xe1, 0x35, 0xbc, 0x20, 0x94, 0x06, 0xfd, 0x56, 0x8e, 0xe7, 0x7c, 0xde, 0x68,
                0x77, 0xe9, 0xcc, 0x8e, 0x4c, 0x2b, 0x5a, 0x69, 0x8d, 0x5d, 0x3f, 0xb1, 0x21, 0x08,
                0x11, 0xc8, 0x11, 0xca,
            ],
            Elements::CalculateExplicitToken => [
                0xd7, 0x2b, 0x39, 0x77, 0x21, 0xf7, 0xed, 0x4c, 0xab, 0x9c, 0x0f, 0xf0, 0xd7, 0xce,
                0xd4, 0x5a, 0xf9, 0x54, 0x9c, 0x27, 0xee, 0x23, 0x26, 0xf6, 0x1d, 0xc8, 0x7b, 0x74,
                0x22, 0x0e, 0x9c, 0x46,
            ],
            Elements::CalculateIssuanceEntropy => [
                0xe8, 0x06, 0x4f, 0x3f, 0x65, 0x87, 0x72, 0x52, 0xac, 0x16, 0x1a, 0xaa, 0x14, 0xd8,
                0xea, 0x60, 0x77, 0x98, 0x9c, 0xf5, 0x55, 0x49, 0xe5, 0xb4, 0x09, 0x07, 0x66, 0xa5,
                0xf6, 0x92, 0xf9, 0xdd,
            ],
            Elements::Ch16 => [
                0x72, 0x3b, 0x96, 0x0c, 0x67, 0x34, 0xa9, 0x88, 0x32, 0x29, 0x88, 0x63, 0x28, 0x48,
                0x5d, 0xb7, 0x18, 0x9d, 0x0b, 0x3c, 0xec, 0x24, 0xc8, 0xa0, 0xb2, 0x88, 0xa4, 0x59,
                0x78, 0x67, 0xbb, 0x9c,
            ],
            Elements::Ch32 => [
                0xf1, 0x47, 0x82, 0xaf, 0x48, 0xdb, 0x5e, 0xe8, 0x42, 0x4d, 0xf8, 0x4d, 0xb5, 0x5e,
                0xcd, 0x83, 0x42, 0x0e, 0x4a, 0xde, 0x1e, 0x43, 0x11, 0x9a, 0xcb, 0x0a, 0x06, 0x5f,
                0x86, 0x67, 0x50, 0x7d,
            ],
            Elements::Ch64 => [
                0xb1, 0x80, 0x0e, 0xac, 0x70, 0x03, 0x8b, 0xdc, 0x3e, 0xe9, 0xdd, 0x14, 0x75, 0xc1,
                0x1f, 0x5a, 0x3b, 0x40, 0xb3, 0xc4, 0x0a, 0xd6, 0xc8, 0xe6, 0x67, 0x48, 0x77, 0x43,
                0x09, 0xc2, 0xc2, 0xac,
            ],
            Elements::Ch8 => [
                0x5b, 0x0b, 0x12, 0x35, 0x72, 0x0b, 0xbf, 0x0b, 0x66, 0x6f, 0x35, 0xe6, 0x78, 0x3a,
                0x6a, 0xb6, 0xb7, 0x84, 0x8c, 0x9d, 0xa4, 0x79, 0x44, 0xbc, 0x91, 0x4f, 0x05, 0x60,
                0x38, 0x95, 0x1c, 0xf8,
            ],
            Elements::CheckLockDistance => [
                0xb9, 0x2f, 0xc8, 0x7f, 0x29, 0xfe, 0x08, 0x01, 0xe8, 0xec, 0x7b, 0x0b, 0x5c, 0x9b,
                0xe7, 0x1e, 0x3a, 0x17, 0xf9, 0x29, 0xbb, 0x80, 0x2e, 0x05, 0x3b, 0xe0, 0x26, 0x1b,
                0x7c, 0x50, 0x3f, 0xad,
            ],
            Elements::CheckLockDuration => [
                0xa3, 0x71, 0xe3, 0x7d, 0x44, 0x12, 0xb6, 0x99, 0xf0, 0x20, 0x63, 0x28, 0x94, 0x49,
                0x0a, 0x38, 0x3e, 0x7c, 0xd6, 0xe4, 0x70, 0x18, 0x2e, 0xe3, 0x92, 0x88, 0x9d, 0x64,
                0x05, 0xb3, 0xad, 0xf0,
            ],
            Elements::CheckLockHeight => [
                0xcd, 0x3d, 0xb0, 0xcb, 0x9c, 0x21, 0xf5, 0x8a, 0x18, 0xea, 0x6c, 0xc4, 0x85, 0xa7,
                0x80, 0x26, 0x44, 0xa1, 0xf7, 0xd7, 0xb8, 0x5e, 0xc1, 0x6f, 0x85, 0x5b, 0x1a, 0xbe,
                0xb3, 0xc7, 0xbf, 0x31,
            ],
            Elements::CheckLockTime => [
                0xab, 0x2c, 0x94, 0x1b, 0xb3, 0xde, 0xf1, 0x16, 0xa5, 0x92, 0xb5, 0xbe, 0x8e, 0x5f,
                0xcb, 0x3a, 0x74, 0xfc, 0x96, 0x1c, 0x82, 0xfb, 0x19, 0xfe, 0x79, 0x68, 0x0b, 0x53,
                0xd5, 0x21, 0x32, 0x0b,
            ],
            Elements::CheckSigVerify => [
                0x29, 0x74, 0x59, 0xd8, 0x37, 0x3f, 0x87, 0x27, 0xda, 0x9f, 0x92, 0xda, 0xea, 0xb7,
                0x19, 0x6b, 0xed, 0x0b, 0xd5, 0x23, 0x64, 0x40, 0x55, 0x21, 0x44, 0x55, 0x72, 0x3a,
                0xba, 0xc0, 0xb0, 0x85,
            ],
            Elements::Complement16 => [
                0x9d, 0x56, 0x15, 0xf9, 0x07, 0x22, 0xd4, 0xae, 0xfb, 0xd8, 0xdd, 0x09, 0xc6, 0x82,
                0x88, 0x43, 0x68, 0xa8, 0x8f, 0x47, 0x11, 0xf6, 0x2c, 0xfe, 0x02, 0x9c, 0x9f, 0x07,
                0x29, 0xa2, 0x73, 0x44,
            ],
            Elements::Complement32 => [
                0x43, 0x60, 0xc5, 0xd2, 0xa4, 0x57, 0xd4, 0x43, 0x21, 0x9e, 0x02, 0x67, 0x74, 0xe6,
                0x8d, 0xee, 0x23, 0xef, 0x6e, 0xc7, 0xa9, 0x13, 0x72, 0x18, 0x6c, 0x9d, 0x46, 0x03,
                0x94, 0xc1, 0x51, 0x9b,
            ],
            Elements::Complement64 => [
                0x24, 0x13, 0x58, 0x82, 0xf7, 0x56, 0x03, 0xd0, 0x69, 0x81, 0x34, 0x90, 0x2e, 0xb5,
                0x0e, 0x14, 0xa9, 0xa4, 0xa7, 0x9f, 0x48, 0xe7, 0x77, 0xa6, 0x97, 0xfc, 0x02, 0xd6,
                0x4c, 0x15, 0x65, 0x89,
            ],
            Elements::Complement8 => [
                0x85, 0x8b, 0x61, 0xb0, 0x7e, 0x3d, 0x4e, 0xf1, 0x43, 0x4e, 0xc4, 0xf4, 0xae, 0xc3,
                0x0d, 0x29, 0x59, 0x9b, 0x8a, 0xc3, 0x83, 0x70, 0xe5, 0x8a, 0xfa, 0x47, 0x7b, 0x1e,
                0xd3, 0x9d, 0x6b, 0x47,
            ],
            Elements::CurrentAmount => [
                0x22, 0x49, 0xb6, 0x1c, 0x45, 0x60, 0x79, 0xb3, 0x72, 0xb1, 0xbc, 0x12, 0xd6, 0x25,
                0x77, 0x2d, 0x7d, 0xc8, 0xe7, 0xd8, 0xc8, 0x43, 0xd3, 0x1c, 0xd3, 0x8d, 0xc1, 0xd0,
                0xea, 0xff, 0xd3, 0x88,
            ],
            Elements::CurrentAnnexHash => [
                0x2a, 0x06, 0xfc, 0xc4, 0x65, 0x72, 0xe5, 0xb3, 0x86, 0x3e, 0xdb, 0x47, 0xcc, 0x77,
                0xa9, 0x80, 0xb7, 0x15, 0xd4, 0x07, 0xad, 0xf8, 0xc6, 0x25, 0x77, 0xe1, 0xbb, 0x74,
                0x56, 0xa2, 0xa8, 0x19,
            ],
            Elements::CurrentAsset => [
                0xd0, 0x39, 0x89, 0x5e, 0x43, 0x8d, 0x0f, 0xca, 0xf6, 0x25, 0x21, 0xf3, 0x85, 0x63,
                0x4e, 0xef, 0x7a, 0x6e, 0x71, 0x68, 0x9c, 0x9e, 0xb1, 0x5e, 0x5c, 0xd6, 0x3e, 0xfe,
                0x5b, 0xa8, 0xd9, 0x24,
            ],
            Elements::CurrentIndex => [
                0x60, 0x06, 0x9b, 0x2d, 0x6a, 0xa0, 0x6f, 0x1e, 0xf2, 0x40, 0x19, 0xb3, 0xb8, 0x4e,
                0xfb, 0x19, 0x06, 0xac, 0x10, 0x04, 0x94, 0x76, 0xe5, 0xc8, 0x0a, 0xc9, 0xaa, 0xf5,
                0x0f, 0xd1, 0xdd, 0xed,
            ],
            Elements::CurrentIssuanceAssetAmount => [
                0x1c, 0x3c, 0x68, 0x64, 0xdd, 0x31, 0x78, 0x7f, 0x25, 0x72, 0x1c, 0x46, 0x59, 0x43,
                0x48, 0x38, 0xa0, 0x7f, 0x46, 0x0e, 0x51, 0x2a, 0x48, 0x8f, 0x0e, 0x5d, 0xa4, 0xf9,
                0xae, 0x58, 0x41, 0x8d,
            ],
            Elements::CurrentIssuanceAssetProof => [
                0x52, 0xf8, 0x9e, 0xd9, 0x96, 0x5f, 0xf1, 0x16, 0xb3, 0x73, 0x21, 0x22, 0xa3, 0xfb,
                0x9b, 0x8a, 0x3b, 0x5d, 0xda, 0x59, 0xb3, 0x43, 0x2d, 0x77, 0x71, 0x7a, 0x20, 0x60,
                0x8a, 0x39, 0x4c, 0xa4,
            ],
            Elements::CurrentIssuanceTokenAmount => [
                0x8a, 0x97, 0x84, 0xd9, 0xc0, 0x0d, 0xdb, 0x63, 0x47, 0x49, 0x65, 0xee, 0x28, 0x01,
                0x5c, 0x86, 0x8e, 0xbe, 0xb7, 0x35, 0x5d, 0x22, 0xe7, 0x95, 0x6c, 0x7e, 0xa4, 0xdb,
                0x97, 0xbc, 0x6b, 0xb4,
            ],
            Elements::CurrentIssuanceTokenProof => [
                0xeb, 0x8e, 0x14, 0x90, 0x9b, 0xf0, 0x97, 0x1d, 0xe3, 0xb1, 0x4e, 0x46, 0xaf, 0xee,
                0x81, 0x27, 0x94, 0x46, 0x98, 0xd6, 0xb2, 0x86, 0xaf, 0x00, 0xe4, 0x46, 0xc2, 0xc8,
                0xc1, 0xfe, 0xd1, 0x21,
            ],
            Elements::CurrentNewIssuanceContract => [
                0x15, 0x3e, 0xfc, 0x0d, 0x8a, 0x73, 0x93, 0x29, 0xc8, 0xdd, 0x37, 0x31, 0xea, 0x3d,
                0x06, 0xab, 0x26, 0xd4, 0xdf, 0x7c, 0xa1, 0x8d, 0x88, 0x15, 0x64, 0x5e, 0x6c, 0xae,
                0x4f, 0x1e, 0x4b, 0x72,
            ],
            Elements::CurrentPegin => [
                0xdf, 0x11, 0x9d, 0x08, 0x35, 0xc1, 0xb2, 0x4a, 0xcb, 0xe8, 0xbc, 0x00, 0x80, 0x72,
                0x05, 0x6a, 0xa6, 0x86, 0x34, 0x01, 0x9a, 0x89, 0x21, 0x74, 0x17, 0x30, 0x01, 0x3e,
                0x76, 0xc1, 0x08, 0xf1,
            ],
            Elements::CurrentPrevOutpoint => [
                0x14, 0xad, 0xdf, 0x9d, 0x58, 0x71, 0x69, 0x31, 0x77, 0x59, 0x10, 0xc7, 0xc9, 0x83,
                0xf1, 0xd4, 0xf0, 0xa0, 0xaf, 0x95, 0x1f, 0x3b, 0x0b, 0x3f, 0x3d, 0x2c, 0xbe, 0x62,
                0x95, 0xc1, 0x1d, 0x5e,
            ],
            Elements::CurrentReissuanceBlinding => [
                0xe7, 0x92, 0x72, 0xff, 0x85, 0xeb, 0x34, 0x34, 0x9f, 0xdb, 0xcd, 0x28, 0x9d, 0x5a,
                0xb3, 0x49, 0x12, 0xd4, 0x74, 0x52, 0x50, 0xbd, 0x70, 0x37, 0x65, 0x67, 0x09, 0x5a,
                0x97, 0xa0, 0xb5, 0x43,
            ],
            Elements::CurrentReissuanceEntropy => [
                0x6a, 0xba, 0xba, 0x99, 0x9a, 0xbe, 0x66, 0x62, 0xcc, 0x76, 0x8c, 0x90, 0x05, 0x81,
                0xa3, 0x47, 0xf7, 0x1e, 0xad, 0x4d, 0x68, 0x09, 0x13, 0x96, 0xe3, 0xe7, 0x56, 0xd6,
                0x6b, 0xd5, 0x5e, 0x1e,
            ],
            Elements::CurrentScriptHash => [
                0xff, 0x09, 0x38, 0x23, 0x75, 0xa9, 0x7b, 0x54, 0xf9, 0xad, 0x1c, 0x17, 0xfe, 0xc2,
                0x00, 0x02, 0x68, 0xe4, 0xd4, 0x8e, 0x3f, 0xef, 0x2f, 0xaf, 0x25, 0x3f, 0x27, 0xfb,
                0x80, 0x1f, 0x8d, 0xab,
            ],
            Elements::CurrentScriptSigHash => [
                0xb3, 0x79, 0x4a, 0x09, 0xbf, 0x86, 0x86, 0x84, 0x52, 0x79, 0xb6, 0xff, 0xb2, 0x36,
                0x82, 0x9c, 0x5d, 0x3c, 0x81, 0x02, 0xb6, 0xcd, 0x44, 0x6e, 0xbd, 0x02, 0x14, 0xd8,
                0xd9, 0x43, 0x81, 0x68,
            ],
            Elements::CurrentSequence => [
                0xdb, 0x71, 0x95, 0xf6, 0x90, 0x19, 0xe0, 0xaf, 0xa2, 0x64, 0x40, 0xc5, 0x8e, 0x97,
                0x3d, 0x42, 0xc5, 0x69, 0x9e, 0xf7, 0x07, 0x32, 0xa8, 0x45, 0x51, 0x04, 0x9c, 0x50,
                0x5f, 0x83, 0xaa, 0x43,
            ],
            Elements::Decompress => [
                0x59, 0x85, 0xb0, 0x05, 0xf2, 0x01, 0x34, 0x2d, 0x90, 0x97, 0x49, 0x27, 0x30, 0x25,
                0x7d, 0x68, 0x3e, 0xe5, 0x4f, 0x0d, 0x6b, 0xfb, 0x68, 0xfa, 0xe1, 0x82, 0x1d, 0x6a,
                0x01, 0xfe, 0xa2, 0x8c,
            ],
            Elements::Decrement16 => [
                0x88, 0x73, 0xfb, 0x6a, 0x44, 0x7d, 0xe2, 0x86, 0xcd, 0x6d, 0x64, 0xb4, 0x92, 0xd8,
                0xa1, 0xd3, 0xd7, 0xcd, 0xdf, 0xbb, 0xe6, 0x75, 0xbf, 0xf5, 0xc5, 0x6e, 0xba, 0x40,
                0xf6, 0x90, 0xdb, 0xd3,
            ],
            Elements::Decrement32 => [
                0xca, 0x2b, 0xc9, 0xbd, 0x09, 0xc3, 0x61, 0x30, 0xc5, 0x2e, 0x4c, 0xfa, 0x7d, 0xf6,
                0xd1, 0x08, 0x30, 0x61, 0x66, 0x04, 0x19, 0x16, 0x3d, 0x0a, 0x96, 0xb6, 0x69, 0xa0,
                0x73, 0x96, 0x6a, 0xdd,
            ],
            Elements::Decrement64 => [
                0xdb, 0xc8, 0xe7, 0x57, 0xbc, 0x06, 0x96, 0x24, 0xb0, 0x27, 0xa9, 0xf7, 0x6f, 0x0a,
                0xe7, 0x9f, 0xa2, 0x0e, 0x48, 0xb8, 0x91, 0x0f, 0x7a, 0xb4, 0xa2, 0xb1, 0x62, 0x27,
                0xd2, 0xb4, 0xcc, 0xac,
            ],
            Elements::Decrement8 => [
                0xd6, 0xec, 0x5f, 0xba, 0x81, 0xdc, 0xbd, 0xd4, 0x4a, 0x1b, 0x3a, 0xb1, 0x94, 0xad,
                0xd7, 0xf5, 0x42, 0xb1, 0x70, 0xca, 0xde, 0xb4, 0x65, 0xdd, 0x2f, 0x31, 0x92, 0x84,
                0x58, 0x1a, 0x76, 0xd6,
            ],
            Elements::DivMod16 => [
                0x66, 0xee, 0x6d, 0x63, 0x76, 0x96, 0x76, 0x0e, 0xf0, 0x90, 0x3a, 0x4e, 0xe9, 0xda,
                0x77, 0x15, 0xcb, 0xf9, 0x04, 0xaa, 0x9f, 0x59, 0xd8, 0x4e, 0xcc, 0xb3, 0x5f, 0x90,
                0x47, 0x5d, 0x37, 0xbe,
            ],
            Elements::DivMod32 => [
                0x6c, 0x6a, 0xf7, 0x62, 0x0b, 0x1d, 0x71, 0x8b, 0x1e, 0x09, 0x13, 0x30, 0x72, 0xb2,
                0xf2, 0x04, 0x35, 0x8d, 0xbe, 0x58, 0x94, 0xd3, 0xa2, 0xc6, 0x18, 0xc2, 0x59, 0x3e,
                0x62, 0xf0, 0x06, 0xcc,
            ],
            Elements::DivMod64 => [
                0x24, 0x39, 0x58, 0x0d, 0xda, 0x93, 0x4d, 0x44, 0x98, 0xfc, 0x41, 0x77, 0x4f, 0x29,
                0x4d, 0xe5, 0x93, 0xf4, 0x4c, 0x63, 0x2f, 0x84, 0xe7, 0x21, 0xa8, 0xd5, 0xbb, 0xde,
                0x34, 0x5c, 0xce, 0x40,
            ],
            Elements::DivMod8 => [
                0x06, 0xfb, 0x2a, 0x19, 0x7c, 0x16, 0x1d, 0x0a, 0x77, 0xfc, 0x72, 0x6d, 0x3b, 0xaa,
                0x85, 0x7e, 0x27, 0xf8, 0x69, 0x1b, 0xcc, 0xe2, 0xb4, 0x73, 0xd4, 0xd1, 0x40, 0x8c,
                0x17, 0xe0, 0x32, 0x1d,
            ],
            Elements::Divide16 => [
                0x70, 0x1e, 0xf0, 0x5d, 0x05, 0xfe, 0x59, 0x43, 0x54, 0xb2, 0xef, 0x79, 0x6f, 0x4c,
                0xa4, 0x52, 0x76, 0xda, 0xac, 0x15, 0x25, 0x92, 0x76, 0x3e, 0x2a, 0x1f, 0x6e, 0x19,
                0x67, 0x25, 0x76, 0xbc,
            ],
            Elements::Divide32 => [
                0xf8, 0x83, 0xb1, 0xc6, 0x4d, 0x04, 0x40, 0x31, 0xd9, 0x9e, 0x04, 0x67, 0x42, 0x54,
                0xfe, 0x2b, 0xb3, 0x44, 0xbc, 0xdd, 0x54, 0x38, 0xd6, 0xea, 0x01, 0x45, 0x8d, 0x9e,
                0x24, 0x97, 0x70, 0x2a,
            ],
            Elements::Divide64 => [
                0xaa, 0x84, 0x4c, 0xfe, 0x9a, 0x83, 0xc5, 0x26, 0xd5, 0xfb, 0x05, 0x82, 0x0d, 0x1a,
                0x6f, 0x95, 0x3e, 0xb5, 0x0c, 0x85, 0x6c, 0xda, 0x50, 0x7d, 0xb9, 0xbe, 0x5b, 0x7c,
                0x90, 0xab, 0x2d, 0x14,
            ],
            Elements::Divide8 => [
                0xbb, 0x39, 0x21, 0x8f, 0x0b, 0x2d, 0x69, 0x30, 0xa6, 0xe4, 0x17, 0x66, 0xfd, 0x52,
                0x18, 0xfa, 0xd1, 0x81, 0x0d, 0x17, 0x6a, 0xba, 0x80, 0x1a, 0x26, 0x38, 0x0d, 0x35,
                0x1a, 0x2d, 0x79, 0xb1,
            ],
            Elements::Divides16 => [
                0x0e, 0x98, 0xa5, 0x3a, 0x62, 0x66, 0xf4, 0xc6, 0x0b, 0xdc, 0xaf, 0x62, 0x0d, 0xbc,
                0xa7, 0x6d, 0xb3, 0xcc, 0x1b, 0x59, 0x8a, 0x9c, 0x66, 0x7a, 0xd1, 0xda, 0x86, 0xb5,
                0xc9, 0xc0, 0xe9, 0x12,
            ],
            Elements::Divides32 => [
                0x6e, 0xd2, 0x1a, 0xbf, 0x02, 0x2c, 0x2b, 0xa0, 0x20, 0xe9, 0x1f, 0x0d, 0x09, 0x59,
                0x3b, 0xa5, 0x5f, 0x74, 0x26, 0x26, 0x29, 0x14, 0xe9, 0x55, 0xe1, 0x79, 0x1a, 0x58,
                0x3b, 0x31, 0xc0, 0x7d,
            ],
            Elements::Divides64 => [
                0x39, 0x71, 0xea, 0x23, 0x01, 0xba, 0xbb, 0x7b, 0xb5, 0x4f, 0x5f, 0x48, 0x33, 0xc6,
                0x3c, 0xaa, 0x52, 0x79, 0xd4, 0xc2, 0xb6, 0x54, 0x59, 0x2f, 0xbb, 0x50, 0x6e, 0xb2,
                0x03, 0xb0, 0x06, 0x15,
            ],
            Elements::Divides8 => [
                0x77, 0x63, 0x5c, 0x01, 0x95, 0x71, 0x9b, 0x99, 0x0f, 0xeb, 0x14, 0x31, 0xb1, 0x24,
                0x4e, 0xb7, 0x55, 0xae, 0xe9, 0xba, 0xce, 0xe4, 0xb2, 0x0c, 0x12, 0xbe, 0x8e, 0x23,
                0x9d, 0x12, 0xf7, 0xdc,
            ],
            Elements::Eq16 => [
                0x08, 0x1d, 0x4c, 0x58, 0x0a, 0x51, 0x56, 0x85, 0xa7, 0x00, 0xcb, 0x47, 0xa5, 0x43,
                0x33, 0x04, 0x49, 0x0a, 0xdd, 0x10, 0x1c, 0xbc, 0x07, 0xc3, 0xc4, 0x6e, 0xfb, 0x19,
                0x6b, 0x75, 0xfe, 0x55,
            ],
            Elements::Eq256 => [
                0x56, 0x83, 0x53, 0x14, 0x63, 0x05, 0x0b, 0x4f, 0x99, 0x26, 0xd0, 0xe8, 0xff, 0x64,
                0x26, 0x3a, 0x15, 0xce, 0xb1, 0xce, 0xa6, 0x54, 0x12, 0x63, 0x7c, 0x23, 0x5f, 0x46,
                0xd7, 0xbe, 0x66, 0x05,
            ],
            Elements::Eq32 => [
                0x73, 0x87, 0xd2, 0x79, 0x8a, 0x02, 0x10, 0x7e, 0x2c, 0x57, 0x36, 0x71, 0x27, 0xf3,
                0xf3, 0x6f, 0xc9, 0xf1, 0x97, 0x0d, 0xaf, 0xba, 0x80, 0xf9, 0x34, 0x0c, 0x16, 0x8b,
                0x50, 0xf7, 0xa4, 0xe2,
            ],
            Elements::Eq64 => [
                0x05, 0x72, 0xe1, 0x6b, 0xab, 0x59, 0xd8, 0x47, 0x5f, 0xda, 0x6b, 0xfc, 0xbd, 0x49,
                0x6f, 0xe7, 0x53, 0x41, 0x07, 0xf6, 0x29, 0x1d, 0xd7, 0xee, 0xdb, 0xfb, 0xf2, 0x8b,
                0x76, 0x0a, 0xa4, 0x1a,
            ],
            Elements::Eq8 => [
                0xd4, 0x25, 0x79, 0xe4, 0xe7, 0xaf, 0x6d, 0xaa, 0x39, 0xa5, 0x4e, 0x54, 0x27, 0x47,
                0xd4, 0x19, 0x5c, 0x2a, 0xb4, 0x8c, 0x1f, 0xdd, 0x7b, 0x87, 0x32, 0xe7, 0xa5, 0x2f,
                0xed, 0x40, 0x75, 0x99,
            ],
            Elements::FeAdd => [
                0xdf, 0x5c, 0x03, 0x4b, 0x64, 0x59, 0xde, 0xd2, 0xf2, 0x07, 0xb5, 0xe4, 0x0f, 0x02,
                0xe5, 0x08, 0x76, 0xee, 0x6b, 0x25, 0x0c, 0xdd, 0x8c, 0x94, 0x9f, 0xbb, 0x78, 0x3b,
                0x5e, 0x90, 0x44, 0xda,
            ],
            Elements::FeInvert => [
                0xc1, 0x22, 0x09, 0x8f, 0x95, 0x59, 0x4b, 0x72, 0x1d, 0x3a, 0x51, 0xa3, 0x1d, 0xc1,
                0xb2, 0xc4, 0x0d, 0xec, 0xcb, 0x9b, 0x3c, 0xdc, 0x9a, 0xb9, 0x0e, 0x56, 0x1a, 0xc2,
                0x13, 0x41, 0xbc, 0xe2,
            ],
            Elements::FeIsOdd => [
                0xaf, 0xa5, 0x6c, 0x6b, 0x60, 0xc5, 0x2a, 0x39, 0xe5, 0xa1, 0x61, 0x0e, 0xfb, 0x8e,
                0x1a, 0xe5, 0x6f, 0x72, 0x03, 0x72, 0x01, 0xca, 0xc6, 0x04, 0x37, 0xb6, 0x75, 0x8a,
                0x74, 0x46, 0x9b, 0x2a,
            ],
            Elements::FeIsZero => [
                0x51, 0x1a, 0xc5, 0xb8, 0x35, 0x9c, 0x20, 0x02, 0x08, 0x0a, 0x16, 0xa1, 0x18, 0x40,
                0xab, 0x1a, 0xe1, 0x5b, 0x60, 0x15, 0x73, 0x72, 0x67, 0x7b, 0x39, 0xa9, 0x2f, 0xd7,
                0xcf, 0x3e, 0x8e, 0xb3,
            ],
            Elements::FeMultiply => [
                0x4d, 0xb0, 0xb0, 0xf2, 0xa2, 0xa8, 0x21, 0x38, 0x7a, 0x7d, 0xab, 0x9c, 0x5c, 0x0c,
                0xe8, 0xeb, 0xde, 0x14, 0xd0, 0x39, 0x25, 0xb5, 0x6d, 0x7f, 0x30, 0x9a, 0x5e, 0x2d,
                0x09, 0x9b, 0xf9, 0xab,
            ],
            Elements::FeMultiplyBeta => [
                0x45, 0x32, 0xc7, 0x30, 0x5f, 0x04, 0xbb, 0x13, 0x0f, 0xef, 0x0c, 0x4e, 0x16, 0x7b,
                0x9a, 0xc0, 0xcc, 0x07, 0xbc, 0xfc, 0x66, 0xdd, 0x2d, 0x0c, 0x67, 0x8e, 0xd9, 0x13,
                0xae, 0xe2, 0x1d, 0x64,
            ],
            Elements::FeNegate => [
                0x3b, 0x1c, 0x7d, 0x40, 0x36, 0x1c, 0x5b, 0x03, 0x73, 0x77, 0x1e, 0xab, 0xb0, 0x66,
                0x28, 0x9f, 0x29, 0x1f, 0x76, 0x65, 0x25, 0xeb, 0x25, 0xab, 0x42, 0xd1, 0x8d, 0xfa,
                0x21, 0x96, 0xda, 0x40,
            ],
            Elements::FeNormalize => [
                0xe0, 0x89, 0xae, 0x03, 0x97, 0x27, 0x38, 0xe5, 0x8c, 0xad, 0x4f, 0x74, 0x26, 0xe2,
                0x63, 0xfa, 0x5e, 0x2c, 0x6c, 0x4b, 0x58, 0x69, 0xef, 0x96, 0x9b, 0xec, 0x86, 0xfc,
                0xb7, 0xad, 0x32, 0x05,
            ],
            Elements::FeSquare => [
                0x1f, 0x1f, 0x57, 0xf8, 0x8b, 0x28, 0x73, 0xf4, 0xdf, 0x41, 0x63, 0xa6, 0x8f, 0x53,
                0x98, 0xb1, 0x68, 0x2a, 0x91, 0x1b, 0xeb, 0x4d, 0x6d, 0x24, 0xe8, 0xca, 0x27, 0x1e,
                0x35, 0xba, 0xe8, 0xdb,
            ],
            Elements::FeSquareRoot => [
                0xb0, 0xea, 0x63, 0x8c, 0xf2, 0x50, 0xe2, 0xa2, 0x8d, 0xcf, 0xc8, 0xfc, 0x04, 0x09,
                0x40, 0x13, 0x10, 0x21, 0x81, 0xf8, 0x16, 0xfe, 0xfb, 0xc6, 0xcb, 0x45, 0x1b, 0xe7,
                0x1e, 0xe8, 0xa8, 0x05,
            ],
            Elements::FullAdd16 => [
                0x67, 0xc5, 0x7c, 0x82, 0x46, 0x18, 0x60, 0xc0, 0x22, 0x3a, 0x42, 0x7d, 0xde, 0x99,
                0xe5, 0x12, 0x75, 0x3b, 0x0e, 0xc5, 0x2a, 0x05, 0xdc, 0xbe, 0x71, 0x8e, 0xf3, 0x0a,
                0x26, 0x84, 0xb6, 0x81,
            ],
            Elements::FullAdd32 => [
                0x73, 0x9f, 0x5b, 0x1e, 0x9b, 0x40, 0x8b, 0x36, 0x3c, 0xda, 0xd6, 0xbf, 0x00, 0xa8,
                0xbf, 0xf5, 0x22, 0xb3, 0xd5, 0x38, 0x02, 0xea, 0xad, 0xd8, 0x94, 0xa0, 0x10, 0x7d,
                0x20, 0x3a, 0x46, 0x00,
            ],
            Elements::FullAdd64 => [
                0x70, 0xe1, 0xee, 0xa5, 0x3b, 0xc9, 0xb0, 0x1b, 0x06, 0x76, 0xb5, 0xea, 0x97, 0x36,
                0x74, 0xd6, 0xf3, 0xc3, 0x56, 0x74, 0x32, 0xf2, 0x64, 0xbc, 0x60, 0xb7, 0xae, 0x1f,
                0xf6, 0x9a, 0x40, 0x2a,
            ],
            Elements::FullAdd8 => [
                0x37, 0xff, 0x9d, 0x08, 0x4d, 0xd0, 0x51, 0x7f, 0xb3, 0x89, 0x7d, 0xe0, 0x86, 0x38,
                0x1a, 0x9c, 0x2c, 0x72, 0x44, 0x0e, 0xe1, 0xba, 0xc3, 0x8d, 0xb7, 0xf4, 0xf7, 0x4a,
                0x50, 0x1d, 0x54, 0xdd,
            ],
            Elements::FullDecrement16 => [
                0x61, 0xe2, 0xe8, 0x5a, 0xb8, 0xc7, 0xe4, 0xc2, 0xe9, 0xdc, 0x6e, 0xb2, 0xf0, 0x60,
                0x74, 0x9a, 0x03, 0x35, 0x41, 0x2f, 0x9f, 0x3a, 0xc8, 0xfd, 0x68, 0xf2, 0x56, 0x15,
                0xff, 0x06, 0x7e, 0x61,
            ],
            Elements::FullDecrement32 => [
                0x8b, 0xe3, 0x11, 0x0b, 0xda, 0xe8, 0xd6, 0x5f, 0x08, 0x33, 0x19, 0x95, 0xf5, 0x84,
                0xbf, 0x36, 0x02, 0x36, 0xa1, 0x4c, 0x61, 0xad, 0x2f, 0xa6, 0xe7, 0xae, 0xac, 0x60,
                0x9b, 0xd4, 0xdb, 0xfa,
            ],
            Elements::FullDecrement64 => [
                0xd0, 0x03, 0x87, 0x12, 0x19, 0x99, 0x14, 0x9c, 0x3a, 0x7b, 0xd2, 0x56, 0xce, 0x8b,
                0xd0, 0x2d, 0x71, 0x9f, 0x3d, 0x5c, 0x58, 0xf1, 0x0a, 0xb6, 0x75, 0x7c, 0xc0, 0x23,
                0xd5, 0x5d, 0xbe, 0x1b,
            ],
            Elements::FullDecrement8 => [
                0xe8, 0xcc, 0x36, 0x06, 0xb1, 0xdd, 0x7f, 0xcf, 0x64, 0xc9, 0x91, 0xb0, 0xce, 0xb9,
                0xc6, 0x14, 0x6c, 0xcc, 0xd0, 0x89, 0x49, 0x89, 0xd8, 0xd9, 0xab, 0x97, 0xd2, 0x74,
                0x3e, 0xa1, 0xa5, 0xac,
            ],
            Elements::FullIncrement16 => [
                0xce, 0xaf, 0x13, 0xe7, 0x8e, 0x3a, 0x30, 0x75, 0xd1, 0x17, 0x48, 0x0c, 0x7b, 0x72,
                0xc2, 0xba, 0xd4, 0x54, 0x8c, 0xcc, 0xfa, 0xf4, 0x8e, 0xa1, 0xeb, 0x67, 0x7f, 0x78,
                0x0e, 0x5c, 0x00, 0x30,
            ],
            Elements::FullIncrement32 => [
                0x7e, 0x83, 0x12, 0x88, 0x87, 0x14, 0x66, 0x43, 0x44, 0x5a, 0xf4, 0xca, 0xbb, 0x1d,
                0xa5, 0x0e, 0x82, 0xda, 0xeb, 0x34, 0xb7, 0xd9, 0xa2, 0x42, 0xcd, 0x00, 0xde, 0x5f,
                0xbb, 0xea, 0x51, 0x82,
            ],
            Elements::FullIncrement64 => [
                0x93, 0xac, 0xb5, 0x83, 0x1a, 0x59, 0xb2, 0x8a, 0xe4, 0x38, 0x87, 0x6e, 0x31, 0x02,
                0xf9, 0x19, 0x42, 0x7b, 0x82, 0x72, 0x63, 0x0a, 0x8e, 0xd7, 0x85, 0x92, 0xef, 0x54,
                0x3c, 0xfb, 0x65, 0xf6,
            ],
            Elements::FullIncrement8 => [
                0x92, 0x1a, 0xaa, 0xa8, 0xca, 0xc8, 0x20, 0x63, 0xfb, 0xfa, 0xc0, 0x51, 0x17, 0xe4,
                0xa9, 0x3e, 0xb2, 0xf0, 0xb9, 0x2d, 0x83, 0x56, 0x21, 0x99, 0xeb, 0xdb, 0x8a, 0xfc,
                0x16, 0xb1, 0xd5, 0x2c,
            ],
            Elements::FullMultiply16 => [
                0x15, 0x97, 0x38, 0x6d, 0xa7, 0xf0, 0xd5, 0x56, 0x99, 0x66, 0xd1, 0x71, 0x76, 0xea,
                0xe3, 0x78, 0x1b, 0xec, 0x8f, 0xb4, 0xfd, 0xe5, 0xc8, 0xc7, 0x12, 0xda, 0xea, 0xa8,
                0x6b, 0xd8, 0x3f, 0x37,
            ],
            Elements::FullMultiply32 => [
                0xe8, 0xb8, 0x81, 0xa1, 0x26, 0xe2, 0xc0, 0x4e, 0xbd, 0x8d, 0xc7, 0x8d, 0x8e, 0x24,
                0xfd, 0x92, 0xcc, 0x82, 0xba, 0x0c, 0x41, 0xe1, 0xa4, 0x03, 0xe6, 0x43, 0x13, 0x34,
                0xd1, 0x3c, 0x4e, 0xc2,
            ],
            Elements::FullMultiply64 => [
                0xdc, 0xcc, 0x7e, 0x3c, 0x39, 0x7c, 0x15, 0x0a, 0xfd, 0xf3, 0x83, 0xbe, 0xd8, 0x9e,
                0x95, 0xe5, 0x05, 0x94, 0xb2, 0x85, 0xb8, 0xc7, 0xd6, 0x24, 0xd1, 0xb9, 0xe0, 0x59,
                0xe9, 0xf7, 0x23, 0x0a,
            ],
            Elements::FullMultiply8 => [
                0x20, 0x18, 0x33, 0x05, 0x30, 0x45, 0x46, 0x19, 0xc6, 0x4f, 0x5a, 0x10, 0x9e, 0x7b,
                0x3c, 0xce, 0x81, 0x79, 0x79, 0xdb, 0x53, 0x52, 0x18, 0x9e, 0x31, 0x97, 0xb0, 0x51,
                0x99, 0xc7, 0x4d, 0xe9,
            ],
            Elements::FullSubtract16 => [
                0x09, 0x19, 0xce, 0x69, 0x18, 0x98, 0xd2, 0x7c, 0x6f, 0xc4, 0x4c, 0x13, 0x70, 0x64,
                0x52, 0x4e, 0x41, 0x31, 0x46, 0xd1, 0xce, 0x20, 0x17, 0x32, 0xc9, 0x75, 0xb0, 0xd5,
                0x47, 0xef, 0xfa, 0x7c,
            ],
            Elements::FullSubtract32 => [
                0x81, 0x0b, 0xd2, 0x82, 0x67, 0x28, 0x23, 0x55, 0xee, 0x69, 0x48, 0x20, 0x71, 0x3e,
                0x68, 0xd0, 0x67, 0xbb, 0xd8, 0xb8, 0x1e, 0xd9, 0x13, 0x8c, 0x33, 0xe5, 0x16, 0x49,
                0x73, 0xa5, 0x62, 0x84,
            ],
            Elements::FullSubtract64 => [
                0xc8, 0x67, 0x88, 0x5f, 0xb9, 0x15, 0x4f, 0x6e, 0x4a, 0x68, 0xb8, 0x16, 0x4e, 0xca,
                0xe1, 0xd6, 0x9f, 0xeb, 0x4f, 0x4d, 0xc6, 0xc2, 0xe1, 0xe9, 0x72, 0xaf, 0x8a, 0xf8,
                0xc6, 0x33, 0x47, 0xf1,
            ],
            Elements::FullSubtract8 => [
                0xb9, 0xf1, 0xb9, 0xb8, 0xd1, 0x0f, 0xab, 0xbb, 0xb7, 0xdf, 0x46, 0xc8, 0x75, 0x86,
                0x2f, 0x7e, 0x7e, 0x70, 0x14, 0x08, 0x85, 0x60, 0xb0, 0xda, 0x45, 0x48, 0x0a, 0x6c,
                0x7c, 0x12, 0x59, 0xa2,
            ],
            Elements::GeIsOnCurve => [
                0x36, 0xd0, 0x5a, 0x72, 0x9a, 0x79, 0xb0, 0x37, 0xb1, 0x0b, 0x3a, 0xb2, 0xb2, 0xc2,
                0xbf, 0xea, 0x03, 0xa0, 0x1a, 0x18, 0xe0, 0xea, 0xf6, 0x5c, 0x6f, 0x9b, 0xc7, 0x45,
                0xc7, 0xc2, 0xe6, 0xfa,
            ],
            Elements::GeNegate => [
                0x52, 0x01, 0xd4, 0x5c, 0xc1, 0xde, 0xb3, 0xe2, 0xb9, 0x49, 0xfe, 0x66, 0x0a, 0x07,
                0xc5, 0xce, 0x69, 0x44, 0x3d, 0xbe, 0xc6, 0x7c, 0xc7, 0x6b, 0x24, 0x25, 0xff, 0x32,
                0xe8, 0x52, 0xe5, 0xb4,
            ],
            Elements::GejAdd => [
                0x28, 0x54, 0x85, 0xc4, 0x70, 0x84, 0x49, 0x25, 0x10, 0x37, 0x3d, 0xf4, 0x3d, 0xf5,
                0x34, 0x07, 0xac, 0xec, 0x8f, 0xb1, 0xbd, 0x01, 0x03, 0x80, 0x89, 0x7b, 0x51, 0x7c,
                0x39, 0xcd, 0x63, 0x19,
            ],
            Elements::GejDouble => [
                0x71, 0x07, 0x74, 0x58, 0x57, 0x75, 0xf9, 0x1f, 0x4c, 0xe5, 0x78, 0xad, 0x8d, 0x1e,
                0x64, 0x45, 0x41, 0xe2, 0x1f, 0xc6, 0xc8, 0x10, 0xab, 0xdb, 0x3b, 0x3e, 0xd2, 0x11,
                0x5e, 0x39, 0xcd, 0xae,
            ],
            Elements::GejGeAdd => [
                0x7d, 0x7f, 0x42, 0x6e, 0x42, 0x45, 0x8e, 0x45, 0x77, 0x12, 0x91, 0xcc, 0x9e, 0x60,
                0x7e, 0x67, 0x26, 0x7a, 0x38, 0x85, 0xad, 0xbe, 0xbd, 0xc3, 0x69, 0xdf, 0x59, 0x66,
                0x32, 0x20, 0xbe, 0xfb,
            ],
            Elements::GejGeAddEx => [
                0xcd, 0xda, 0xe7, 0x8d, 0x33, 0xa2, 0x21, 0x28, 0xbc, 0x2f, 0x72, 0xa6, 0x02, 0xe0,
                0x06, 0x6f, 0x63, 0xfe, 0x18, 0x62, 0x57, 0xea, 0x34, 0x8c, 0x2b, 0xb1, 0xf7, 0xe9,
                0xbf, 0x9b, 0x0d, 0x73,
            ],
            Elements::GejInfinity => [
                0x95, 0xbd, 0x3c, 0xe9, 0x3f, 0x0c, 0x8e, 0x95, 0xaa, 0x2e, 0x60, 0xa9, 0xe8, 0x26,
                0x57, 0xfd, 0x98, 0x0f, 0x3f, 0x27, 0x78, 0x11, 0xfb, 0x6d, 0x39, 0xd1, 0xff, 0x12,
                0x1f, 0x3f, 0x8a, 0x14,
            ],
            Elements::GejIsInfinity => [
                0x63, 0x58, 0xd3, 0xdd, 0xc7, 0xab, 0x52, 0xfd, 0x6a, 0xd6, 0x36, 0xad, 0xf9, 0xb9,
                0xf3, 0x7e, 0xaf, 0x79, 0x6f, 0x89, 0xb2, 0xd9, 0xbf, 0xba, 0x97, 0xab, 0xee, 0x3f,
                0x32, 0x7b, 0x30, 0xa4,
            ],
            Elements::GejIsOnCurve => [
                0x6b, 0xcc, 0x65, 0xcf, 0xed, 0x04, 0x39, 0xf1, 0x11, 0xee, 0x5f, 0x5b, 0x5b, 0x47,
                0x91, 0xe1, 0xe4, 0xad, 0x7f, 0xf3, 0x69, 0x51, 0xa2, 0x33, 0x1b, 0x18, 0xf9, 0x7f,
                0x7a, 0xf9, 0x13, 0x98,
            ],
            Elements::GejNegate => [
                0x0a, 0xe3, 0x66, 0x32, 0x36, 0x47, 0x83, 0x1a, 0x3f, 0x1c, 0x8e, 0x51, 0xf1, 0xf6,
                0xc1, 0x7e, 0xec, 0x93, 0xcf, 0x53, 0x95, 0xa2, 0x76, 0x44, 0x0a, 0x24, 0x6d, 0xeb,
                0xb2, 0xef, 0xc3, 0x91,
            ],
            Elements::GejNormalize => [
                0xcb, 0x5a, 0x52, 0xa3, 0x24, 0x29, 0x5b, 0xda, 0xd4, 0x2d, 0x0f, 0xb0, 0x1f, 0x67,
                0xaa, 0xdf, 0x6e, 0x10, 0xe0, 0xb3, 0xd1, 0x8c, 0x93, 0x24, 0xc3, 0xa1, 0x2a, 0x05,
                0xb4, 0xfe, 0x1d, 0x64,
            ],
            Elements::GejRescale => [
                0xe1, 0x42, 0x78, 0x60, 0xb5, 0x1b, 0xa7, 0xe7, 0x07, 0x2d, 0x75, 0x69, 0xc5, 0x5b,
                0x83, 0x39, 0xf8, 0x5b, 0x60, 0x22, 0x75, 0x0c, 0x6a, 0xf5, 0xaf, 0x2a, 0x72, 0xf1,
                0xb7, 0xb6, 0xba, 0xee,
            ],
            Elements::GejXEquiv => [
                0xa7, 0xee, 0x48, 0xd5, 0xfb, 0x92, 0x6d, 0x41, 0xf7, 0x0b, 0xbb, 0x0f, 0x06, 0x82,
                0x13, 0x75, 0x34, 0x3f, 0x0d, 0x2f, 0x0e, 0x5b, 0x19, 0x91, 0x55, 0x39, 0xd9, 0x10,
                0x1b, 0x8d, 0x84, 0x7a,
            ],
            Elements::GejYIsOdd => [
                0xa2, 0xd7, 0x2b, 0x1e, 0x83, 0x88, 0xa1, 0x75, 0x54, 0x27, 0xa7, 0xb2, 0x55, 0x67,
                0x68, 0x31, 0xca, 0x76, 0xea, 0xdd, 0xa2, 0x82, 0xf9, 0x7a, 0x34, 0x3f, 0x0e, 0xb1,
                0x55, 0xf3, 0x50, 0x46,
            ],
            Elements::Generate => [
                0x0e, 0x91, 0xf4, 0x55, 0x7c, 0xb7, 0xd4, 0xc3, 0xbb, 0xf3, 0xf2, 0xd0, 0x74, 0xdd,
                0x69, 0x46, 0x42, 0x3a, 0x3b, 0x4f, 0xac, 0xb5, 0x7a, 0x00, 0xca, 0xe4, 0x3f, 0xd6,
                0xa7, 0x35, 0x2a, 0x13,
            ],
            Elements::GenesisBlockHash => [
                0x59, 0x4c, 0xcf, 0xb4, 0xb5, 0xfa, 0x3c, 0x0d, 0x3a, 0x89, 0xdd, 0x83, 0xb7, 0x4c,
                0x75, 0x43, 0x2e, 0x55, 0x57, 0xd8, 0x80, 0xfb, 0xa8, 0xaf, 0xa4, 0xc8, 0x07, 0x07,
                0x99, 0x4a, 0x94, 0x94,
            ],
            Elements::High16 => [
                0xae, 0x53, 0xd2, 0xd0, 0xaa, 0x16, 0x23, 0xb3, 0xa3, 0xc8, 0xb5, 0x53, 0x82, 0x6f,
                0xde, 0xf4, 0x3f, 0xf5, 0x9e, 0xa7, 0xc4, 0xa0, 0x18, 0x5c, 0xf3, 0x4c, 0x16, 0x45,
                0x08, 0xd8, 0xbf, 0xbc,
            ],
            Elements::High32 => [
                0x64, 0xe7, 0xc5, 0x45, 0x3c, 0x3f, 0x8c, 0x63, 0x6c, 0x59, 0x61, 0xf4, 0x8a, 0x57,
                0xb0, 0xec, 0x7b, 0xde, 0xad, 0xed, 0xa4, 0xb3, 0x53, 0x11, 0x86, 0x7b, 0xde, 0x1c,
                0xf7, 0xc1, 0xc1, 0xd8,
            ],
            Elements::High64 => [
                0xfd, 0xbf, 0xad, 0x11, 0x2a, 0xff, 0x8b, 0xd0, 0x21, 0xab, 0xee, 0xa8, 0x68, 0x88,
                0xdd, 0x28, 0xba, 0x90, 0xb1, 0xd6, 0x36, 0x3b, 0x39, 0x8a, 0x09, 0xdf, 0x9d, 0x5f,
                0x40, 0xe8, 0x06, 0x08,
            ],
            Elements::High8 => [
                0x2d, 0x12, 0x02, 0xea, 0x63, 0x3d, 0x2d, 0x65, 0xf4, 0x5e, 0xaf, 0x80, 0xea, 0x03,
                0xe5, 0xcc, 0x3d, 0x7d, 0x9a, 0xe4, 0xa4, 0xb3, 0x71, 0x21, 0x8a, 0x01, 0x60, 0xa6,
                0x93, 0x10, 0x4f, 0x69,
            ],
            Elements::Increment16 => [
                0xef, 0x9f, 0x63, 0x27, 0x4d, 0x9f, 0xc0, 0x71, 0x0a, 0xba, 0x34, 0x2f, 0xe7, 0xca,
                0x00, 0xd9, 0x12, 0xbf, 0x27, 0x71, 0xa9, 0x40, 0xbd, 0xd1, 0x27, 0x2d, 0x28, 0x9b,
                0x70, 0x43, 0x00, 0x44,
            ],
            Elements::Increment32 => [
                0x42, 0x57, 0xa7, 0x52, 0xd0, 0x89, 0x5a, 0x75, 0x31, 0xa5, 0x21, 0x14, 0x15, 0x53,
                0xd3, 0x30, 0x90, 0xb2, 0x78, 0xd0, 0xbb, 0x79, 0xd1, 0x8b, 0x53, 0xcf, 0x75, 0x3c,
                0x76, 0x40, 0x2a, 0x0e,
            ],
            Elements::Increment64 => [
                0xb5, 0x25, 0xe5, 0x54, 0x40, 0x19, 0x3d, 0xc3, 0xd5, 0x1d, 0x9f, 0xc6, 0xb1, 0xf3,
                0xf2, 0x91, 0xf1, 0xa2, 0x77, 0x6e, 0x99, 0xa2, 0x10, 0xc9, 0xfe, 0x33, 0x36, 0x77,
                0xd9, 0x68, 0x50, 0xf2,
            ],
            Elements::Increment8 => [
                0xb5, 0x30, 0x23, 0x1a, 0x83, 0x32, 0x0a, 0x13, 0x0f, 0x41, 0x89, 0xf1, 0xef, 0xee,
                0x5a, 0xb2, 0x91, 0x41, 0xdc, 0xe4, 0xc3, 0x14, 0x91, 0x19, 0xcb, 0x21, 0xb9, 0xcb,
                0x06, 0xce, 0x6f, 0xa1,
            ],
            Elements::InputAmount => [
                0x53, 0x23, 0x44, 0x1b, 0x79, 0x86, 0x7d, 0xfd, 0xf7, 0x5f, 0x59, 0xf3, 0x53, 0x2d,
                0xc5, 0x16, 0xde, 0xfa, 0x91, 0x45, 0xaa, 0xfb, 0x99, 0x51, 0xae, 0x89, 0xe5, 0xb3,
                0xb3, 0x0b, 0xe2, 0x3e,
            ],
            Elements::InputAmountsHash => [
                0x53, 0x46, 0xaa, 0xf0, 0x3a, 0xc9, 0xca, 0xf2, 0xf6, 0x8c, 0xfc, 0xda, 0xde, 0xc1,
                0x7c, 0x19, 0xeb, 0xaf, 0xf0, 0x3d, 0xaa, 0x15, 0xae, 0x74, 0x15, 0x71, 0x61, 0x1b,
                0x08, 0x95, 0x04, 0x4b,
            ],
            Elements::InputAnnexHash => [
                0x3d, 0xf9, 0x1f, 0x8e, 0xe5, 0x88, 0xaf, 0xf8, 0x36, 0xc4, 0xad, 0x9f, 0x3b, 0x98,
                0x40, 0xe7, 0x79, 0xc4, 0x02, 0x2b, 0x7a, 0x09, 0xcb, 0x3d, 0xea, 0xeb, 0x44, 0xd8,
                0x46, 0x8d, 0xd0, 0xc9,
            ],
            Elements::InputAnnexesHash => [
                0x82, 0x74, 0x90, 0xbf, 0x96, 0xc1, 0xf8, 0x3b, 0xaa, 0xf7, 0xd8, 0x2c, 0x7e, 0xef,
                0x45, 0x3f, 0xb7, 0x49, 0x3e, 0x68, 0xb9, 0xa5, 0xb4, 0x98, 0xc2, 0xdd, 0x99, 0x83,
                0xb6, 0x38, 0xa1, 0x76,
            ],
            Elements::InputAsset => [
                0x0f, 0x89, 0x05, 0x7d, 0xff, 0x8f, 0x5b, 0x3d, 0x43, 0xe7, 0xb0, 0x7a, 0x75, 0xb8,
                0xc3, 0x2a, 0xc7, 0x17, 0x77, 0x36, 0x6f, 0xc8, 0x2d, 0x3c, 0xc3, 0x6d, 0x62, 0x85,
                0x08, 0x38, 0x4a, 0x93,
            ],
            Elements::InputOutpointsHash => [
                0x12, 0x0b, 0x26, 0x61, 0x5e, 0x03, 0x73, 0xcc, 0xb1, 0x73, 0xd7, 0x8e, 0xf3, 0x7e,
                0x1c, 0x83, 0x5e, 0xb8, 0xcf, 0x60, 0x20, 0x19, 0xb8, 0xbf, 0xf6, 0x23, 0xa1, 0x34,
                0x28, 0xac, 0x63, 0xd1,
            ],
            Elements::InputPegin => [
                0x68, 0x4c, 0x9b, 0x1a, 0x71, 0xfd, 0x07, 0xd3, 0x54, 0xc0, 0xeb, 0x24, 0x1f, 0xaf,
                0xf7, 0x47, 0x60, 0x16, 0x77, 0xbd, 0xbd, 0x02, 0x87, 0xde, 0x97, 0x0d, 0xf3, 0x86,
                0xc2, 0x0d, 0xde, 0xa9,
            ],
            Elements::InputPrevOutpoint => [
                0x8a, 0xf8, 0x20, 0x69, 0xe2, 0xd3, 0x2b, 0x3c, 0xb2, 0x98, 0xfa, 0xb7, 0x54, 0x8b,
                0x40, 0x3c, 0xe0, 0x58, 0xcc, 0x84, 0x98, 0x69, 0x95, 0x60, 0xac, 0x3a, 0x9f, 0x55,
                0xaa, 0x51, 0x62, 0x7d,
            ],
            Elements::InputScriptHash => [
                0x6d, 0xf2, 0x54, 0x7f, 0x30, 0x43, 0xf1, 0x1f, 0xe6, 0xfb, 0xcd, 0x0b, 0x68, 0x26,
                0x12, 0xf9, 0xf6, 0x4d, 0x83, 0x6c, 0x2a, 0xe7, 0x46, 0x9d, 0xf5, 0x42, 0x86, 0x52,
                0x64, 0x16, 0x20, 0x76,
            ],
            Elements::InputScriptSigHash => [
                0x87, 0x25, 0xa1, 0x70, 0x7f, 0x60, 0x56, 0x86, 0xd2, 0x29, 0x60, 0x8c, 0x13, 0x6d,
                0x83, 0x2a, 0x57, 0x04, 0x2a, 0xfd, 0xcb, 0xe1, 0xa7, 0x06, 0x68, 0x92, 0xcf, 0xc2,
                0x93, 0x5b, 0x84, 0x44,
            ],
            Elements::InputScriptSigsHash => [
                0xb1, 0xbf, 0xda, 0xe8, 0xf6, 0xec, 0x88, 0xaf, 0x83, 0xaa, 0x75, 0x76, 0xc4, 0x92,
                0xf5, 0x25, 0x37, 0x48, 0xe8, 0xd4, 0x1f, 0x67, 0x1f, 0x9a, 0xf0, 0x44, 0x87, 0x13,
                0x90, 0x80, 0x0d, 0xbd,
            ],
            Elements::InputScriptsHash => [
                0x32, 0xc6, 0xb2, 0x24, 0x15, 0x9f, 0x2c, 0x83, 0x35, 0xf2, 0x0a, 0x95, 0x7b, 0x05,
                0x38, 0x85, 0xa4, 0x39, 0x92, 0x4a, 0x2f, 0xb4, 0x4e, 0x13, 0x13, 0x80, 0x0a, 0x54,
                0xdb, 0x11, 0x2d, 0x61,
            ],
            Elements::InputSequence => [
                0xdb, 0x52, 0x5e, 0x1c, 0x10, 0x71, 0xd1, 0x8c, 0x53, 0x10, 0x1f, 0x6e, 0xec, 0xd9,
                0x87, 0x0c, 0x3d, 0x5f, 0x12, 0x59, 0xe3, 0x56, 0x65, 0xa6, 0x30, 0x7a, 0x8b, 0x64,
                0xce, 0x0e, 0xda, 0x2f,
            ],
            Elements::InputSequencesHash => [
                0xec, 0x8e, 0xd3, 0x5a, 0x80, 0xed, 0xde, 0xef, 0x6d, 0xa0, 0xa0, 0x06, 0xcd, 0x6a,
                0x0e, 0x27, 0x50, 0x10, 0xec, 0x0f, 0x6f, 0xc1, 0xb5, 0xec, 0x99, 0x69, 0xdd, 0x0e,
                0x1a, 0xbe, 0x60, 0x7c,
            ],
            Elements::InputUtxosHash => [
                0x39, 0xdf, 0x75, 0x78, 0x39, 0x75, 0x60, 0xd5, 0x23, 0x2f, 0xad, 0xfa, 0x05, 0x8f,
                0x78, 0x0b, 0x44, 0x22, 0x61, 0x29, 0xcb, 0xe4, 0xda, 0xe5, 0x40, 0xd1, 0x3c, 0x42,
                0x20, 0xd8, 0x9f, 0xec,
            ],
            Elements::InputsHash => [
                0x16, 0x05, 0x3a, 0x9a, 0xd3, 0x1d, 0x50, 0xec, 0x05, 0x04, 0x95, 0xb0, 0xdb, 0x9e,
                0x04, 0x03, 0x45, 0xdf, 0x24, 0xd5, 0x69, 0x32, 0xe2, 0x07, 0xd2, 0x7a, 0x83, 0x2b,
                0x6a, 0x68, 0x01, 0xa8,
            ],
            Elements::InternalKey => [
                0xe6, 0x06, 0x8a, 0x12, 0xba, 0x8a, 0x89, 0x80, 0xac, 0x2e, 0xaf, 0xa4, 0xbc, 0x0c,
                0xa0, 0xfa, 0x73, 0x29, 0xdb, 0x6a, 0xb8, 0x45, 0x53, 0x70, 0x38, 0x78, 0xfd, 0xa6,
                0x53, 0xc6, 0x99, 0x64,
            ],
            Elements::IsOne16 => [
                0xec, 0xc5, 0x6a, 0x7c, 0x8d, 0xae, 0x7f, 0xb0, 0x19, 0xb9, 0xf5, 0x49, 0xf8, 0x3a,
                0x5d, 0xa1, 0x21, 0xf3, 0x74, 0xd8, 0xd4, 0x5b, 0xed, 0x50, 0x77, 0x8e, 0xa6, 0xcf,
                0xa9, 0xbb, 0x9e, 0xa0,
            ],
            Elements::IsOne32 => [
                0x3f, 0x3b, 0xc4, 0x6c, 0x8c, 0x53, 0x86, 0x0c, 0x49, 0xc4, 0x70, 0x28, 0x11, 0x10,
                0x90, 0x89, 0xa2, 0xfe, 0xd0, 0xac, 0xdc, 0x3e, 0x54, 0x3b, 0x8f, 0x4c, 0x79, 0x62,
                0x65, 0x5f, 0x7f, 0x93,
            ],
            Elements::IsOne64 => [
                0xe7, 0xb9, 0xc6, 0x90, 0x1f, 0x20, 0xd9, 0x8c, 0x65, 0xd3, 0xe7, 0x8a, 0xd1, 0x71,
                0x61, 0x99, 0x93, 0x90, 0x40, 0xd4, 0xe3, 0x97, 0x80, 0xb4, 0x43, 0x44, 0xab, 0x80,
                0xbb, 0x5f, 0x72, 0x3c,
            ],
            Elements::IsOne8 => [
                0x6a, 0xa1, 0x40, 0xd2, 0xf0, 0xe5, 0xdd, 0x66, 0xc1, 0x21, 0x06, 0x81, 0x62, 0xef,
                0xc5, 0x79, 0xe5, 0x40, 0xf3, 0xac, 0x1c, 0x83, 0x23, 0xd1, 0xa9, 0x22, 0x6f, 0x0e,
                0xb3, 0x2f, 0x84, 0x66,
            ],
            Elements::IsZero16 => [
                0x05, 0x83, 0x68, 0xc8, 0x85, 0xf9, 0x58, 0x18, 0x16, 0x76, 0xde, 0x6d, 0x4d, 0x31,
                0xca, 0xbf, 0x3f, 0x71, 0x42, 0xee, 0xa4, 0x75, 0x21, 0x28, 0x01, 0xeb, 0xe3, 0x73,
                0x79, 0x45, 0x6c, 0x2c,
            ],
            Elements::IsZero32 => [
                0x3b, 0x73, 0x91, 0x81, 0x8f, 0x80, 0xcb, 0xd2, 0x2e, 0xd7, 0xe5, 0xe8, 0x52, 0x3c,
                0x82, 0x0f, 0xa0, 0xdd, 0xc1, 0xa6, 0xc1, 0x8b, 0x00, 0x86, 0xd6, 0x53, 0xff, 0xc3,
                0x7e, 0xa7, 0xa1, 0xac,
            ],
            Elements::IsZero64 => [
                0xf8, 0x19, 0x13, 0xaf, 0xc6, 0x09, 0xde, 0x33, 0x40, 0xa9, 0xf6, 0x7e, 0x83, 0x92,
                0x7b, 0x57, 0x22, 0x32, 0xeb, 0x51, 0x08, 0xc0, 0x27, 0x8c, 0xbc, 0x21, 0x47, 0x61,
                0x92, 0xe0, 0x3c, 0xc7,
            ],
            Elements::IsZero8 => [
                0xfd, 0x9a, 0xc0, 0x8b, 0x59, 0x5b, 0x1e, 0xcb, 0x84, 0x8d, 0xe2, 0x0f, 0xf1, 0x6b,
                0x7c, 0xb2, 0x7a, 0x77, 0x0a, 0x8a, 0x91, 0x8c, 0x01, 0x8b, 0x40, 0x25, 0xf3, 0x6a,
                0xa0, 0xc7, 0x5e, 0x32,
            ],
            Elements::Issuance => [
                0xf7, 0xb6, 0xe3, 0xc4, 0x5b, 0x16, 0x13, 0x9c, 0x65, 0xae, 0x40, 0x3d, 0xa6, 0x02,
                0x52, 0xae, 0xf4, 0x36, 0x2a, 0xc6, 0xb1, 0x03, 0x80, 0x64, 0x19, 0x6d, 0xb4, 0xf6,
                0xcf, 0x34, 0x2f, 0x1b,
            ],
            Elements::IssuanceAsset => [
                0xfb, 0x2b, 0x25, 0x9a, 0x09, 0xb3, 0xf3, 0xb1, 0xe4, 0x31, 0x4e, 0xec, 0x9d, 0x18,
                0xa0, 0xa1, 0xf7, 0x34, 0xf9, 0xa7, 0x5f, 0x58, 0x6e, 0xdc, 0xcc, 0xcf, 0xb9, 0xd8,
                0x34, 0x43, 0xcc, 0xe7,
            ],
            Elements::IssuanceAssetAmount => [
                0xf1, 0xe9, 0xbd, 0xec, 0x38, 0x2e, 0xde, 0x64, 0x22, 0x6e, 0xeb, 0xcd, 0x9a, 0x44,
                0x64, 0x88, 0xc7, 0x0c, 0x61, 0x90, 0xe2, 0xb8, 0x8d, 0xb3, 0x23, 0x03, 0x94, 0x7e,
                0x5f, 0xe5, 0x9e, 0x1e,
            ],
            Elements::IssuanceAssetAmountsHash => [
                0xb4, 0xef, 0xff, 0xbb, 0x92, 0xf0, 0xdc, 0x9c, 0x91, 0x89, 0x34, 0xf2, 0x9d, 0xd6,
                0xbc, 0x97, 0xe8, 0x3a, 0x11, 0x27, 0xa1, 0xb2, 0x41, 0x59, 0xf8, 0x39, 0x8a, 0xc2,
                0x89, 0x90, 0xc7, 0x62,
            ],
            Elements::IssuanceAssetProof => [
                0xfe, 0xaa, 0x9e, 0xa6, 0x84, 0x66, 0xf4, 0x76, 0xb4, 0xde, 0x35, 0x61, 0x61, 0xc7,
                0x07, 0x29, 0xa7, 0xa4, 0x76, 0xa6, 0x57, 0x6e, 0xfe, 0x66, 0xe7, 0x25, 0x03, 0xef,
                0xf3, 0xea, 0xf8, 0xbb,
            ],
            Elements::IssuanceBlindingEntropyHash => [
                0x26, 0xf4, 0xcc, 0xa0, 0xb7, 0xd4, 0x66, 0xdb, 0x9b, 0x2c, 0x1f, 0x7c, 0xd9, 0x2e,
                0x66, 0x4d, 0x13, 0x21, 0x87, 0xc2, 0xe2, 0xcd, 0x79, 0x9b, 0x2f, 0xb0, 0x1a, 0xae,
                0x6d, 0x9b, 0xce, 0x10,
            ],
            Elements::IssuanceEntropy => [
                0x48, 0x65, 0x4e, 0xb9, 0xd5, 0x84, 0xd7, 0xf8, 0xb8, 0xc7, 0x7d, 0x94, 0x29, 0x83,
                0x69, 0xd6, 0x9b, 0x24, 0x41, 0xd7, 0x5f, 0xa6, 0x90, 0x56, 0xd8, 0x6b, 0x1e, 0xf6,
                0x71, 0xf7, 0xb2, 0xb3,
            ],
            Elements::IssuanceRangeProofsHash => [
                0x5c, 0xea, 0x38, 0x39, 0x68, 0xf9, 0xfb, 0x17, 0x45, 0x7e, 0x56, 0x42, 0x60, 0xf2,
                0x6f, 0xe0, 0xb7, 0x7e, 0x66, 0xd3, 0x20, 0xb8, 0x58, 0x38, 0x25, 0x80, 0xaf, 0x5c,
                0x97, 0x8c, 0x82, 0x15,
            ],
            Elements::IssuanceToken => [
                0x85, 0xe9, 0x59, 0x1c, 0xf6, 0x42, 0xe7, 0x8d, 0x5e, 0x31, 0x4b, 0x3a, 0xfc, 0x01,
                0x68, 0x69, 0x2d, 0xbb, 0x1f, 0x21, 0x07, 0xe5, 0x16, 0xb3, 0xd4, 0x09, 0x79, 0xc3,
                0x1e, 0xc0, 0x8c, 0x4b,
            ],
            Elements::IssuanceTokenAmount => [
                0x8c, 0x69, 0xf3, 0x79, 0x7a, 0xf5, 0xd5, 0x00, 0x9f, 0x27, 0x06, 0xe2, 0xfb, 0xbc,
                0x5f, 0x6d, 0x21, 0xdf, 0xa3, 0x9d, 0xa9, 0xf7, 0x10, 0x1b, 0xdb, 0x5c, 0x9a, 0x0b,
                0xdc, 0x2e, 0xfb, 0xd5,
            ],
            Elements::IssuanceTokenAmountsHash => [
                0xd0, 0x22, 0x39, 0xae, 0xeb, 0x45, 0xb4, 0xb1, 0x12, 0xd4, 0xcf, 0x03, 0xaf, 0xe1,
                0x28, 0x99, 0x56, 0x66, 0x6d, 0x0d, 0xa0, 0x61, 0x44, 0x60, 0xa1, 0xcb, 0x34, 0x2d,
                0x1c, 0x83, 0x93, 0x13,
            ],
            Elements::IssuanceTokenProof => [
                0xb9, 0x76, 0xfd, 0xda, 0x11, 0x8a, 0x63, 0x39, 0x5f, 0x7f, 0x6f, 0x75, 0x9d, 0x85,
                0x54, 0x02, 0xc2, 0xf0, 0xa0, 0xdc, 0x25, 0x66, 0x03, 0x74, 0xd3, 0xb5, 0xf3, 0x2e,
                0x45, 0x26, 0x53, 0x51,
            ],
            Elements::IssuancesHash => [
                0x97, 0xb7, 0x24, 0x2b, 0x32, 0x66, 0xa1, 0x43, 0x12, 0xb0, 0xb6, 0xab, 0xb4, 0x1a,
                0x53, 0x41, 0xdd, 0xa8, 0xef, 0xd4, 0xee, 0xd4, 0x3b, 0xa8, 0x58, 0x17, 0x51, 0x42,
                0x77, 0x4f, 0x04, 0x86,
            ],
            Elements::Le16 => [
                0x90, 0x0c, 0x66, 0x5e, 0xbe, 0x88, 0x4d, 0x50, 0x71, 0xc3, 0x2b, 0x3a, 0x0b, 0x6b,
                0x65, 0xa1, 0x2c, 0xba, 0xb6, 0xf0, 0x82, 0x48, 0x0e, 0xfa, 0x06, 0x94, 0x5a, 0x06,
                0x5f, 0xe1, 0x58, 0x66,
            ],
            Elements::Le32 => [
                0xef, 0x40, 0xfc, 0x7c, 0x21, 0x58, 0x56, 0xbb, 0x7e, 0x87, 0x41, 0x4a, 0x39, 0x5b,
                0x4f, 0x02, 0xe2, 0xc4, 0xbb, 0x16, 0x7c, 0x85, 0x26, 0xed, 0x43, 0x10, 0x5d, 0x56,
                0x42, 0xb0, 0xd6, 0xb6,
            ],
            Elements::Le64 => [
                0xf9, 0x1b, 0x28, 0x86, 0xb9, 0xda, 0x94, 0xf7, 0xe7, 0x97, 0xb7, 0x58, 0x14, 0xcf,
                0xee, 0xb2, 0x8c, 0xe0, 0x73, 0xe2, 0x14, 0xe5, 0xa5, 0x5f, 0x70, 0x19, 0xb6, 0x97,
                0x8b, 0x0d, 0x67, 0x42,
            ],
            Elements::Le8 => [
                0x8c, 0xfb, 0x5e, 0x97, 0x85, 0x64, 0x45, 0x45, 0xcc, 0x42, 0xeb, 0x6a, 0x14, 0xb7,
                0xe9, 0x58, 0x27, 0x94, 0x99, 0x00, 0xaa, 0xa3, 0xc5, 0xad, 0xc8, 0x02, 0x4a, 0xf3,
                0xc2, 0xa3, 0x1b, 0x44,
            ],
            Elements::LinearCombination1 => [
                0x95, 0x07, 0x86, 0xef, 0xa6, 0x5a, 0x71, 0x22, 0xe2, 0x55, 0x4c, 0x6f, 0xb5, 0x51,
                0x24, 0xf9, 0xe5, 0xac, 0xd8, 0x2c, 0x29, 0x81, 0x7a, 0xff, 0xc1, 0x9f, 0xc7, 0xa9,
                0x27, 0xd3, 0xa0, 0x70,
            ],
            Elements::LinearVerify1 => [
                0x63, 0x55, 0x71, 0xb1, 0x27, 0xc0, 0x15, 0x65, 0x7c, 0x1b, 0xfb, 0x1d, 0x92, 0x67,
                0xbb, 0x84, 0x6a, 0x7b, 0xf9, 0x49, 0x75, 0x07, 0xae, 0xa6, 0x65, 0x37, 0x35, 0x74,
                0x08, 0xe7, 0x11, 0xa3,
            ],
            Elements::LockTime => [
                0xf2, 0xa3, 0x71, 0xaa, 0x25, 0x17, 0x29, 0xbf, 0xbb, 0xc8, 0xb5, 0x10, 0x7d, 0xbc,
                0x99, 0xe1, 0x50, 0xbf, 0x22, 0xc1, 0x2e, 0x80, 0xc8, 0xbb, 0x93, 0x08, 0x06, 0x7f,
                0xa7, 0xfc, 0x21, 0x61,
            ],
            Elements::Low16 => [
                0xb3, 0x27, 0xae, 0x29, 0x99, 0xed, 0x5f, 0x59, 0x42, 0x15, 0x84, 0xfd, 0x53, 0x7f,
                0x99, 0xc5, 0xfa, 0x10, 0x27, 0x1e, 0x53, 0xe1, 0x33, 0x2b, 0x1f, 0x53, 0x46, 0x35,
                0x54, 0x0d, 0xc8, 0x4d,
            ],
            Elements::Low32 => [
                0x3a, 0x13, 0x27, 0x6c, 0x60, 0x97, 0xd2, 0x72, 0xdb, 0x13, 0x98, 0xc1, 0xd0, 0x3e,
                0x13, 0x02, 0x2e, 0x72, 0x9d, 0x73, 0xce, 0x50, 0xf8, 0xb2, 0xe5, 0xb2, 0xaa, 0xf9,
                0x93, 0x6a, 0x2b, 0xe1,
            ],
            Elements::Low64 => [
                0x06, 0x19, 0xc9, 0x95, 0x66, 0xf8, 0xd3, 0xdc, 0x82, 0x6e, 0x8e, 0xf9, 0x67, 0x65,
                0x70, 0x19, 0x1d, 0xbc, 0xe1, 0xa0, 0x2e, 0x74, 0xd9, 0x22, 0xb1, 0x12, 0x0c, 0xb7,
                0x2d, 0x38, 0x51, 0x3b,
            ],
            Elements::Low8 => [
                0xaa, 0xb5, 0xa1, 0xe3, 0xe6, 0xf0, 0x27, 0x47, 0xd5, 0xb6, 0x48, 0x97, 0xfe, 0x37,
                0xf2, 0xf7, 0x37, 0xca, 0x2b, 0xe4, 0xdc, 0x9c, 0x85, 0xa6, 0xe8, 0x04, 0xad, 0x5e,
                0x95, 0xed, 0x08, 0x33,
            ],
            Elements::Lt16 => [
                0x26, 0x7a, 0xb9, 0xe8, 0xde, 0xee, 0xfa, 0x01, 0xbd, 0xd4, 0xd9, 0xfc, 0x52, 0x4b,
                0xda, 0x2c, 0x4a, 0xe5, 0xb9, 0x5e, 0x77, 0x25, 0xab, 0x97, 0xaa, 0x04, 0x22, 0x01,
                0x3e, 0x86, 0xa4, 0xae,
            ],
            Elements::Lt32 => [
                0xc3, 0x1a, 0x47, 0x16, 0x1a, 0x1b, 0x5f, 0x77, 0xe2, 0x79, 0xfd, 0x22, 0xba, 0x9a,
                0xb0, 0xc3, 0xe1, 0x9c, 0x77, 0x19, 0xa0, 0x0c, 0xfe, 0xe6, 0x03, 0x6d, 0xb6, 0x8e,
                0x1b, 0x9e, 0xc6, 0xdf,
            ],
            Elements::Lt64 => [
                0x58, 0x6e, 0x2b, 0x1f, 0x0b, 0x0c, 0xfa, 0x80, 0x0f, 0x83, 0x1d, 0x35, 0xed, 0x1f,
                0xdb, 0x96, 0x4f, 0xeb, 0x47, 0xbc, 0x62, 0xf7, 0xc0, 0xd4, 0x16, 0xde, 0x3f, 0xd8,
                0x3b, 0xb2, 0x56, 0x42,
            ],
            Elements::Lt8 => [
                0xaf, 0xe9, 0x89, 0xbf, 0xd8, 0xba, 0x4d, 0xc1, 0x06, 0xd2, 0xe3, 0xb0, 0xb3, 0x0b,
                0x1d, 0x9b, 0x5e, 0x69, 0x16, 0xcc, 0xce, 0xbc, 0xd9, 0x87, 0x40, 0xb6, 0xa9, 0xc3,
                0xb9, 0x03, 0xb6, 0x26,
            ],
            Elements::Maj16 => [
                0xd4, 0x47, 0x91, 0x1b, 0x6f, 0xbd, 0x54, 0x7d, 0x43, 0x05, 0x66, 0xbc, 0xe5, 0x21,
                0xcc, 0xf2, 0x5d, 0xff, 0x0f, 0xf5, 0x2a, 0xe7, 0xab, 0x01, 0x0b, 0x69, 0x5a, 0xe0,
                0x87, 0xec, 0x1b, 0x71,
            ],
            Elements::Maj32 => [
                0x66, 0x57, 0x11, 0xe6, 0x27, 0x0b, 0xa2, 0x67, 0x56, 0xc0, 0xc1, 0x85, 0xb7, 0x01,
                0x17, 0xa1, 0x29, 0x6f, 0xf6, 0xc2, 0x9e, 0x42, 0x6a, 0xa9, 0x55, 0xf6, 0x14, 0x95,
                0x85, 0x28, 0x6c, 0x49,
            ],
            Elements::Maj64 => [
                0x69, 0xf4, 0xce, 0xd9, 0xc7, 0x12, 0x73, 0x6d, 0x0d, 0x8e, 0xe1, 0xc4, 0xdc, 0xda,
                0x4f, 0xf5, 0x7f, 0x24, 0xeb, 0x31, 0x42, 0xf9, 0xe0, 0x68, 0x48, 0xde, 0x60, 0xbe,
                0x57, 0x5c, 0x91, 0x49,
            ],
            Elements::Maj8 => [
                0x6b, 0xcb, 0xfe, 0x9c, 0xf8, 0xc1, 0x32, 0x41, 0x11, 0x77, 0xba, 0x84, 0xf4, 0x0f,
                0xc8, 0x4e, 0x0b, 0x8c, 0x16, 0x44, 0x24, 0x50, 0x5f, 0xdd, 0x3b, 0xdf, 0x27, 0xf7,
                0x2a, 0x0d, 0xf7, 0xf6,
            ],
            Elements::Max16 => [
                0x19, 0x34, 0x49, 0xe0, 0xb8, 0x45, 0xbe, 0x48, 0xbb, 0xb1, 0x13, 0x24, 0xba, 0xc6,
                0x34, 0x45, 0x1a, 0x53, 0xbf, 0x03, 0x12, 0xcb, 0x0e, 0x8b, 0x71, 0xb1, 0xd4, 0x63,
                0x67, 0x56, 0x92, 0xfd,
            ],
            Elements::Max32 => [
                0xee, 0x99, 0xdd, 0x4c, 0x3d, 0xe4, 0x96, 0xa2, 0x74, 0xce, 0x3c, 0x06, 0x50, 0xf8,
                0x1c, 0x1e, 0xd9, 0xce, 0xa8, 0x5b, 0xaa, 0xf2, 0x9c, 0x21, 0x33, 0x33, 0x70, 0x98,
                0x4c, 0xa2, 0x53, 0x5c,
            ],
            Elements::Max64 => [
                0xfc, 0x75, 0xa5, 0xd0, 0xc2, 0xde, 0x6f, 0xc6, 0xc3, 0xf3, 0x52, 0x5c, 0x6e, 0x8a,
                0x89, 0x38, 0x35, 0xaa, 0x9f, 0x7a, 0x8d, 0x3d, 0x35, 0x18, 0x1d, 0x0a, 0x58, 0x1b,
                0x27, 0x23, 0xee, 0x4e,
            ],
            Elements::Max8 => [
                0xcf, 0xcf, 0x17, 0x20, 0x71, 0xf0, 0x3c, 0xa6, 0x18, 0xcc, 0xd3, 0x58, 0x21, 0x06,
                0x3d, 0x4e, 0xc1, 0x3c, 0xcc, 0x6d, 0x73, 0xdf, 0x99, 0x46, 0xe9, 0xcd, 0xb7, 0x77,
                0xb1, 0x1f, 0xb4, 0x37,
            ],
            Elements::Median16 => [
                0x04, 0xb1, 0x02, 0xc6, 0xbf, 0x51, 0x7d, 0x0d, 0x58, 0x90, 0xd5, 0x5e, 0x0d, 0x1a,
                0x0c, 0x23, 0x95, 0x40, 0x82, 0x0a, 0x0c, 0xde, 0x26, 0x20, 0xd5, 0x2c, 0x60, 0x2c,
                0x65, 0x56, 0xaf, 0x66,
            ],
            Elements::Median32 => [
                0x77, 0x9f, 0xeb, 0xff, 0x10, 0xbb, 0x16, 0x40, 0xa7, 0x6e, 0xdb, 0x05, 0x10, 0x58,
                0xd5, 0x8f, 0xe7, 0xc3, 0x79, 0xbb, 0x8b, 0x73, 0xd2, 0xe2, 0xb1, 0xd7, 0xc2, 0x0d,
                0x26, 0x82, 0xc5, 0x0d,
            ],
            Elements::Median64 => [
                0xdb, 0x62, 0xcc, 0x65, 0xbe, 0x87, 0xa9, 0x07, 0xb6, 0x87, 0xbc, 0x32, 0x36, 0xf7,
                0x5b, 0x63, 0xa7, 0xb7, 0x3d, 0x75, 0x64, 0x9e, 0xb6, 0x54, 0xeb, 0xac, 0x19, 0x68,
                0x6c, 0x32, 0x62, 0x8e,
            ],
            Elements::Median8 => [
                0x89, 0x77, 0x37, 0x02, 0x27, 0x41, 0x9e, 0xc2, 0x26, 0xba, 0x4a, 0xd4, 0x5f, 0x44,
                0x06, 0x4f, 0x82, 0xf9, 0x73, 0x03, 0x94, 0xcb, 0x79, 0x2d, 0xc5, 0x43, 0xa6, 0x5b,
                0xca, 0xae, 0x7b, 0x09,
            ],
            Elements::Min16 => [
                0x09, 0x38, 0x26, 0x1d, 0xf2, 0xa5, 0xa0, 0xde, 0x39, 0x71, 0x26, 0xe7, 0x65, 0x41,
                0xe2, 0x16, 0x6b, 0x24, 0x2a, 0xe8, 0x87, 0x57, 0x0d, 0xba, 0xcf, 0x9a, 0x3b, 0x6f,
                0x98, 0x21, 0x33, 0xd9,
            ],
            Elements::Min32 => [
                0xa2, 0xb8, 0x5a, 0xe1, 0x70, 0xc9, 0x9f, 0x8f, 0x46, 0xa0, 0xf8, 0xf8, 0xc2, 0x1b,
                0xa6, 0xe1, 0x41, 0x9b, 0x69, 0x1d, 0x46, 0xef, 0x6b, 0x28, 0x73, 0xdf, 0x96, 0xdb,
                0xb9, 0x85, 0x89, 0x00,
            ],
            Elements::Min64 => [
                0x3e, 0xa4, 0x86, 0x19, 0x3d, 0x0d, 0x1a, 0xff, 0x0c, 0xc6, 0xea, 0xab, 0x18, 0x31,
                0xf5, 0xf2, 0x5c, 0x30, 0xe7, 0x8d, 0x7e, 0x25, 0xb0, 0xb0, 0xc2, 0x78, 0x1e, 0x1e,
                0xca, 0xbf, 0x46, 0x2e,
            ],
            Elements::Min8 => [
                0xe9, 0xb4, 0xe5, 0x41, 0xe3, 0x0d, 0xd5, 0x18, 0x37, 0xa2, 0x38, 0x8c, 0xb0, 0x6c,
                0x3d, 0xe9, 0x05, 0x68, 0xb4, 0xe8, 0xfe, 0x24, 0x02, 0x01, 0xfd, 0xa8, 0x33, 0xae,
                0x57, 0x89, 0x91, 0x64,
            ],
            Elements::Modulo16 => [
                0x6d, 0xcf, 0xa1, 0x9b, 0xf6, 0x5f, 0x47, 0x8d, 0x30, 0x6f, 0x11, 0x3f, 0xbb, 0x64,
                0xb3, 0x3a, 0xe2, 0x1e, 0x99, 0xc0, 0xe7, 0x5c, 0x46, 0xa9, 0xd8, 0x7d, 0xbd, 0xaf,
                0x72, 0x9e, 0xc8, 0xa4,
            ],
            Elements::Modulo32 => [
                0x33, 0x2a, 0xd0, 0x58, 0x62, 0x6c, 0x5e, 0x58, 0x81, 0xc7, 0x4a, 0xab, 0x45, 0x82,
                0x02, 0x08, 0xef, 0x3b, 0x3e, 0xe5, 0xc0, 0x8a, 0x0d, 0xe2, 0x87, 0x12, 0x45, 0x92,
                0x11, 0x18, 0x21, 0xec,
            ],
            Elements::Modulo64 => [
                0x1f, 0x5d, 0x0b, 0x25, 0x5f, 0xe6, 0xd8, 0x3e, 0x33, 0xd2, 0x01, 0x9a, 0x2c, 0x66,
                0xec, 0xe9, 0x75, 0x0c, 0x02, 0x07, 0x3a, 0x10, 0xdf, 0x2a, 0x57, 0x3b, 0xc0, 0xe9,
                0x02, 0xbc, 0x59, 0x1b,
            ],
            Elements::Modulo8 => [
                0xfc, 0x66, 0x85, 0xa7, 0x70, 0x98, 0xb1, 0x09, 0x29, 0xc6, 0xa2, 0xba, 0x8c, 0x97,
                0xb9, 0xa0, 0xbc, 0x64, 0xb1, 0xf2, 0x0a, 0xe7, 0x8d, 0x53, 0x9f, 0x59, 0x2b, 0xe9,
                0x72, 0x95, 0x07, 0x9d,
            ],
            Elements::Multiply16 => [
                0x53, 0xcb, 0x58, 0x3e, 0xde, 0xe8, 0xbf, 0x8d, 0x65, 0xdb, 0xa9, 0x95, 0x65, 0x70,
                0x8a, 0x94, 0x75, 0xa7, 0xc3, 0x75, 0x3f, 0x09, 0x83, 0xe4, 0x86, 0x1f, 0x22, 0xdf,
                0xb6, 0x6a, 0x3d, 0x58,
            ],
            Elements::Multiply32 => [
                0x0f, 0x2d, 0x9f, 0xf6, 0x5b, 0xa4, 0x94, 0x2c, 0x5f, 0xfa, 0x4d, 0x8e, 0x6e, 0x1f,
                0x5d, 0x91, 0x0c, 0x1a, 0xbb, 0xfe, 0x24, 0xd8, 0x65, 0xa0, 0x2e, 0x06, 0x67, 0x20,
                0xe7, 0xa4, 0xcb, 0x66,
            ],
            Elements::Multiply64 => [
                0x6a, 0x6d, 0x34, 0xfe, 0xcc, 0xf3, 0xc6, 0xcd, 0xa6, 0x35, 0x5a, 0x41, 0xd0, 0x74,
                0xe3, 0x18, 0xf9, 0x1a, 0xc0, 0x98, 0x21, 0xce, 0x0a, 0x49, 0xe2, 0x52, 0x15, 0xa9,
                0xce, 0x8b, 0x41, 0x57,
            ],
            Elements::Multiply8 => [
                0x4e, 0x55, 0x2a, 0xfc, 0x5d, 0xdb, 0xfe, 0x21, 0xf8, 0x0e, 0x69, 0xff, 0xf6, 0xb2,
                0x19, 0x43, 0xd0, 0x99, 0x43, 0x91, 0x83, 0x39, 0x82, 0xef, 0xda, 0xdb, 0x86, 0x7d,
                0x06, 0x16, 0x75, 0x48,
            ],
            Elements::Negate16 => [
                0x10, 0xdd, 0xc6, 0x5d, 0x21, 0xf5, 0xad, 0x08, 0x2d, 0x70, 0xe9, 0xaf, 0x9b, 0xe4,
                0x6d, 0xc5, 0xcb, 0x5e, 0xe3, 0xd6, 0x16, 0xa8, 0x3b, 0x61, 0xf6, 0xd1, 0xb4, 0x45,
                0xf7, 0x6f, 0xc2, 0x21,
            ],
            Elements::Negate32 => [
                0x30, 0x00, 0x9b, 0x3c, 0x58, 0x8c, 0x0d, 0xee, 0x3e, 0x5b, 0xc9, 0xa4, 0x1b, 0x2d,
                0x0b, 0xb0, 0x8c, 0x0a, 0xf0, 0xd9, 0xb4, 0x2c, 0x88, 0xa5, 0xb7, 0x07, 0xe3, 0x6c,
                0x56, 0x48, 0x2c, 0x9b,
            ],
            Elements::Negate64 => [
                0xa0, 0xc2, 0xa1, 0x66, 0x13, 0xa1, 0xfc, 0xbc, 0x13, 0x93, 0xca, 0x07, 0x0b, 0xb5,
                0x58, 0xf6, 0x57, 0x65, 0x9b, 0x07, 0xf0, 0xb4, 0x3e, 0x61, 0xce, 0x9c, 0x62, 0x7e,
                0xd9, 0xd7, 0xdb, 0x63,
            ],
            Elements::Negate8 => [
                0xcf, 0xb4, 0x38, 0xb3, 0x07, 0x88, 0xd7, 0xda, 0x99, 0xf7, 0x87, 0xf2, 0xfa, 0x89,
                0x33, 0x3e, 0xde, 0x1c, 0xf6, 0x3b, 0x84, 0x78, 0x02, 0x26, 0x24, 0xd6, 0x15, 0x90,
                0xad, 0x50, 0xa7, 0x37,
            ],
            Elements::NewIssuanceContract => [
                0x8c, 0x01, 0xb1, 0x5f, 0x01, 0xa9, 0x94, 0xcc, 0xca, 0x7b, 0xb1, 0xf1, 0x47, 0xe9,
                0x38, 0xd3, 0x6c, 0x94, 0xd3, 0xba, 0x04, 0x38, 0x67, 0x9c, 0x28, 0x03, 0x9d, 0x33,
                0x6d, 0xc6, 0xa6, 0x1f,
            ],
            Elements::NonceHash => [
                0x94, 0xe5, 0xa0, 0x3d, 0xd4, 0xb8, 0x75, 0x40, 0x99, 0x5f, 0x73, 0xd8, 0x78, 0x5c,
                0x56, 0x21, 0x20, 0x90, 0x50, 0x61, 0x6e, 0x76, 0x50, 0x07, 0xc4, 0x8b, 0x8f, 0x25,
                0xc5, 0x63, 0xd9, 0xf0,
            ],
            Elements::NumInputs => [
                0x8c, 0x2e, 0x09, 0x71, 0xf3, 0x81, 0x8f, 0x39, 0x9b, 0xdf, 0x41, 0x64, 0x7c, 0x54,
                0x91, 0xfe, 0xe9, 0xaa, 0x10, 0x1e, 0xe1, 0x83, 0x2a, 0xb4, 0xb4, 0xa2, 0x4c, 0x54,
                0x1e, 0x09, 0x79, 0xd5,
            ],
            Elements::NumOutputs => [
                0x44, 0x71, 0x65, 0xa3, 0x76, 0xe3, 0x66, 0x8a, 0x5a, 0x22, 0x8b, 0xbb, 0xcd, 0x41,
                0xab, 0x74, 0xb8, 0xb5, 0x1b, 0x95, 0x61, 0x19, 0x0e, 0x34, 0xe8, 0xf6, 0x68, 0xc3,
                0x95, 0x6b, 0x94, 0x07,
            ],
            Elements::One16 => [
                0xef, 0x62, 0xcf, 0x64, 0xfe, 0x05, 0x88, 0x46, 0x57, 0x80, 0xe6, 0x00, 0xd2, 0x8c,
                0x83, 0x14, 0x75, 0x1c, 0x74, 0xf5, 0xd2, 0x41, 0x7f, 0xaf, 0x7b, 0x58, 0x72, 0x9a,
                0x79, 0xa4, 0x03, 0xfa,
            ],
            Elements::One32 => [
                0x0a, 0x81, 0x86, 0xf9, 0xd6, 0x6a, 0xfb, 0xcc, 0xb1, 0x53, 0xe8, 0x9e, 0x5d, 0x21,
                0x53, 0x0f, 0x0d, 0x66, 0x01, 0xfa, 0x35, 0x9c, 0x4a, 0x30, 0x58, 0xe9, 0x09, 0xbc,
                0xef, 0xfe, 0x20, 0x7d,
            ],
            Elements::One64 => [
                0x96, 0x4d, 0xf8, 0xba, 0xb0, 0x50, 0x82, 0x11, 0x74, 0x67, 0x44, 0x76, 0x12, 0x31,
                0x7e, 0x1c, 0x64, 0x53, 0x8d, 0xe6, 0xc9, 0xb2, 0x5d, 0xd6, 0xb1, 0x18, 0x60, 0xdf,
                0x36, 0x73, 0x94, 0x25,
            ],
            Elements::One8 => [
                0xd3, 0xdb, 0xa3, 0x01, 0x75, 0x38, 0x9b, 0xf1, 0x33, 0x05, 0x15, 0x6a, 0xf9, 0xd6,
                0x9c, 0x6b, 0xa2, 0x04, 0xd3, 0xb2, 0x69, 0xbf, 0x8e, 0x2b, 0x90, 0x38, 0xe0, 0x63,
                0x50, 0x3a, 0xe5, 0xcf,
            ],
            Elements::Or16 => [
                0x01, 0xfb, 0x5f, 0x5a, 0xe2, 0xaa, 0xbb, 0x88, 0x93, 0x52, 0x32, 0x88, 0x89, 0xbc,
                0x69, 0xc0, 0xe5, 0x2e, 0xeb, 0x96, 0x71, 0x5b, 0x18, 0x9e, 0x33, 0x38, 0x04, 0xf0,
                0xa4, 0x92, 0x4c, 0x9c,
            ],
            Elements::Or32 => [
                0xb2, 0x75, 0x8f, 0x51, 0x14, 0x49, 0xf2, 0xaa, 0x77, 0x66, 0x43, 0xd8, 0xe0, 0x4b,
                0xfb, 0x6e, 0x10, 0xb4, 0x59, 0x1d, 0xba, 0x91, 0x9e, 0x1c, 0x19, 0x4f, 0xc8, 0x9b,
                0xb0, 0x45, 0xfd, 0x29,
            ],
            Elements::Or64 => [
                0xa4, 0x0d, 0x0b, 0x13, 0x84, 0xa3, 0xe0, 0xb0, 0xad, 0xf5, 0x93, 0x6b, 0x78, 0x27,
                0x9e, 0x52, 0x3f, 0xe5, 0x5b, 0xeb, 0x3f, 0xe4, 0x26, 0x68, 0x3b, 0xc8, 0x7e, 0xc9,
                0x9b, 0xbe, 0x27, 0x9a,
            ],
            Elements::Or8 => [
                0xc6, 0x03, 0xc1, 0xe8, 0x1b, 0x5a, 0x2e, 0x8e, 0x4c, 0xbf, 0x81, 0xd0, 0x59, 0xa5,
                0xa0, 0xc7, 0xc4, 0xd9, 0x1a, 0x17, 0x92, 0x8c, 0x9d, 0x5c, 0xae, 0xbf, 0x67, 0xb8,
                0x6d, 0x00, 0x34, 0xc3,
            ],
            Elements::OutpointHash => [
                0xb3, 0x0d, 0x98, 0x92, 0x05, 0x0c, 0x76, 0xbd, 0xef, 0xf8, 0xa6, 0x7f, 0xd8, 0xc3,
                0xed, 0x60, 0x52, 0x62, 0x0f, 0x2f, 0x87, 0xdb, 0x34, 0x7d, 0x0f, 0xe6, 0x18, 0x77,
                0x55, 0x0f, 0x0b, 0x57,
            ],
            Elements::OutputAmount => [
                0xcb, 0x06, 0x28, 0x22, 0x9d, 0x2c, 0xb3, 0xea, 0x72, 0xae, 0xc0, 0x0b, 0x69, 0xcc,
                0xd0, 0x43, 0x9c, 0x45, 0x06, 0xbe, 0xef, 0x7b, 0xb6, 0x28, 0xaf, 0x5d, 0x19, 0x7f,
                0xe5, 0x42, 0x0f, 0x72,
            ],
            Elements::OutputAmountsHash => [
                0x76, 0x55, 0x79, 0x1b, 0xc5, 0x0e, 0x75, 0x77, 0xdf, 0xc6, 0x52, 0xc2, 0x10, 0xf2,
                0xaf, 0xb0, 0x31, 0x36, 0x9b, 0x80, 0xa3, 0xbb, 0x38, 0xd2, 0x99, 0x44, 0x5e, 0x0b,
                0xdb, 0x84, 0x37, 0x5b,
            ],
            Elements::OutputAsset => [
                0x20, 0xc5, 0x45, 0x82, 0x85, 0x01, 0x67, 0xc7, 0x5a, 0xfc, 0x3e, 0x68, 0xf4, 0xd6,
                0x5b, 0x2d, 0x37, 0x5a, 0x8a, 0x89, 0x9c, 0x13, 0xd1, 0x1e, 0xa8, 0xdc, 0x91, 0xcf,
                0x55, 0x59, 0x5d, 0x3c,
            ],
            Elements::OutputNonce => [
                0xbc, 0x8e, 0x1c, 0x22, 0x2d, 0x1c, 0x8f, 0x83, 0x92, 0xc2, 0xc5, 0xc0, 0xb8, 0x4f,
                0x19, 0x19, 0xe6, 0x8a, 0x81, 0xe4, 0x87, 0x45, 0x5e, 0xbe, 0x60, 0xe4, 0x52, 0xdf,
                0xad, 0x60, 0x94, 0x88,
            ],
            Elements::OutputNoncesHash => [
                0x23, 0x23, 0x36, 0x19, 0x96, 0xf9, 0xf9, 0xb5, 0xd6, 0x0b, 0xa0, 0x2c, 0xf4, 0x9b,
                0x53, 0xa3, 0xc2, 0xc4, 0x02, 0xd1, 0x43, 0x98, 0x5c, 0x67, 0x59, 0x3b, 0x22, 0xed,
                0x05, 0xc9, 0x02, 0xf6,
            ],
            Elements::OutputNullDatum => [
                0xab, 0x6a, 0x96, 0xb6, 0xf0, 0xde, 0x07, 0x4b, 0x85, 0x74, 0x09, 0xad, 0xdc, 0x10,
                0x7a, 0x91, 0x9e, 0xa7, 0x9d, 0xf8, 0x64, 0xcb, 0xac, 0x8a, 0x8b, 0xd2, 0x67, 0xb4,
                0x47, 0x72, 0x8d, 0xd0,
            ],
            Elements::OutputRangeProof => [
                0x45, 0xa8, 0xb7, 0x46, 0xa0, 0x1f, 0x59, 0xcf, 0x18, 0x0c, 0xe4, 0xf3, 0x2c, 0x57,
                0xf1, 0xda, 0xe8, 0xf1, 0x6a, 0x75, 0xf9, 0x94, 0xf1, 0x3a, 0x7b, 0x83, 0xf8, 0xf4,
                0xfc, 0x43, 0x74, 0xab,
            ],
            Elements::OutputRangeProofsHash => [
                0x34, 0x22, 0x55, 0x5b, 0xe1, 0x2d, 0xda, 0x7e, 0xc1, 0x95, 0x36, 0x01, 0x1a, 0x01,
                0xac, 0xfd, 0xf0, 0x90, 0x5b, 0x39, 0xbe, 0x47, 0x58, 0x01, 0x5c, 0x05, 0xd2, 0xfd,
                0xe6, 0x16, 0x4c, 0xf1,
            ],
            Elements::OutputScriptHash => [
                0x88, 0x37, 0x8e, 0x35, 0x4b, 0x46, 0xb2, 0x72, 0xe5, 0xcf, 0xb0, 0xbd, 0x97, 0x7f,
                0x92, 0xf3, 0x09, 0xf1, 0x0d, 0xc2, 0xe0, 0xc5, 0xaa, 0xf8, 0xd9, 0xbe, 0x81, 0xf5,
                0xe0, 0x23, 0x8d, 0x44,
            ],
            Elements::OutputScriptsHash => [
                0x4d, 0x12, 0x8c, 0xbc, 0xfd, 0xb9, 0xe0, 0x9c, 0xd7, 0xc3, 0x59, 0x9d, 0x1f, 0x29,
                0xd7, 0x5a, 0xe7, 0x18, 0xdb, 0xf3, 0xe9, 0x3d, 0x5d, 0xbb, 0xaf, 0x8d, 0x6e, 0x11,
                0x2d, 0x58, 0x36, 0x2f,
            ],
            Elements::OutputSurjectionProof => [
                0x51, 0x6c, 0x4f, 0xb6, 0x9a, 0x1d, 0x40, 0xae, 0xed, 0xdd, 0x9a, 0x13, 0x25, 0x00,
                0x1d, 0x14, 0x5b, 0x5b, 0x45, 0x2d, 0x8c, 0x6f, 0x6f, 0x37, 0x0e, 0x04, 0x71, 0xff,
                0x30, 0x10, 0x71, 0xc6,
            ],
            Elements::OutputSurjectionProofsHash => [
                0xcf, 0xc0, 0x6c, 0x2f, 0x71, 0x26, 0x38, 0xa7, 0x64, 0x74, 0xaf, 0xa3, 0x43, 0xc1,
                0x25, 0xc3, 0xda, 0xfc, 0xcc, 0xea, 0x7c, 0x14, 0x7a, 0xc6, 0x79, 0xb9, 0xf3, 0xb6,
                0xcc, 0x0a, 0x61, 0x84,
            ],
            Elements::OutputsHash => [
                0xdc, 0xb5, 0x38, 0x03, 0xf0, 0x99, 0xef, 0xec, 0xa0, 0x56, 0x4b, 0x5b, 0x25, 0x97,
                0x12, 0x69, 0x28, 0x14, 0x06, 0x9c, 0x46, 0xdd, 0xa0, 0x31, 0x5b, 0xc2, 0x67, 0x8e,
                0xc2, 0x90, 0x0b, 0xc1,
            ],
            Elements::ParseLock => [
                0x48, 0x71, 0xcb, 0xae, 0x52, 0x78, 0x66, 0xba, 0x12, 0x50, 0x81, 0xdf, 0x03, 0x75,
                0x1e, 0x23, 0x1a, 0x61, 0x28, 0x22, 0xbd, 0x45, 0xde, 0x3b, 0xcf, 0xe6, 0xa8, 0xc5,
                0x14, 0xe2, 0xe7, 0x43,
            ],
            Elements::ParseSequence => [
                0xfc, 0xf5, 0xd5, 0xcf, 0x69, 0x08, 0x93, 0x5e, 0x58, 0x5e, 0x9a, 0xfc, 0xc0, 0x07,
                0x17, 0xfd, 0xb1, 0xc5, 0x48, 0x79, 0x5b, 0x23, 0xf0, 0x8b, 0xb8, 0x13, 0x83, 0x17,
                0x7d, 0x93, 0xca, 0xd9,
            ],
            Elements::PointVerify1 => [
                0x6a, 0x08, 0x9d, 0x61, 0xca, 0x20, 0x0a, 0x42, 0x58, 0xe8, 0xb5, 0xb4, 0xfe, 0x5c,
                0x08, 0xd5, 0x74, 0x85, 0x62, 0x49, 0x8d, 0x75, 0xf6, 0xc6, 0x26, 0x09, 0xbb, 0x68,
                0xc9, 0x8b, 0x40, 0x7c,
            ],
            Elements::ReissuanceBlinding => [
                0xb9, 0xaf, 0x17, 0x12, 0xcb, 0x93, 0x10, 0x7f, 0x23, 0x36, 0x7c, 0x21, 0x7f, 0x3f,
                0x09, 0x95, 0x78, 0xd7, 0xf9, 0x1f, 0xe3, 0xe6, 0x94, 0x3c, 0xd6, 0x28, 0x61, 0x62,
                0x30, 0xcb, 0xf6, 0xc7,
            ],
            Elements::ReissuanceEntropy => [
                0x9b, 0xa4, 0x9e, 0x89, 0x0c, 0xf5, 0x1e, 0x18, 0x24, 0x3c, 0xab, 0x94, 0xf5, 0xad,
                0xea, 0xe7, 0x40, 0x75, 0x6c, 0xc9, 0x5c, 0x1a, 0x91, 0x75, 0x4a, 0x6d, 0x29, 0x02,
                0x47, 0x4a, 0x20, 0xda,
            ],
            Elements::ScalarAdd => [
                0xa2, 0x3a, 0xa2, 0xc3, 0x3d, 0xd1, 0xef, 0xf2, 0x49, 0xa1, 0x7c, 0x91, 0xc9, 0x9a,
                0x15, 0x1d, 0x30, 0x87, 0xdc, 0x7c, 0x45, 0x91, 0xcf, 0x94, 0xc5, 0xc6, 0xab, 0xc1,
                0xba, 0x6d, 0x1c, 0xec,
            ],
            Elements::ScalarInvert => [
                0xf2, 0x2d, 0xf7, 0xd7, 0x5e, 0xfd, 0xea, 0x1c, 0xfc, 0x53, 0xc5, 0x70, 0xcb, 0x8b,
                0x12, 0xa2, 0xd3, 0x41, 0x06, 0x7c, 0xb5, 0xfe, 0x41, 0xe9, 0x9d, 0xbd, 0x5f, 0xb6,
                0x32, 0x82, 0xe5, 0x1e,
            ],
            Elements::ScalarIsZero => [
                0x41, 0x62, 0xf9, 0x33, 0x27, 0x03, 0xd0, 0x1e, 0x8d, 0xa6, 0x36, 0x86, 0xe8, 0x57,
                0xf2, 0x8c, 0x19, 0x82, 0x03, 0x10, 0x72, 0x2e, 0x09, 0x6b, 0xb9, 0x7f, 0xb4, 0x61,
                0xaf, 0xf5, 0x62, 0x0c,
            ],
            Elements::ScalarMultiply => [
                0x72, 0xcb, 0x04, 0x36, 0xf6, 0xf1, 0x47, 0x8c, 0xcf, 0x8c, 0x54, 0x56, 0x98, 0xc6,
                0x74, 0xdf, 0x03, 0xc3, 0x9a, 0x9d, 0x2d, 0xab, 0xa1, 0x31, 0x50, 0x33, 0x46, 0xef,
                0x30, 0xd5, 0x8c, 0x26,
            ],
            Elements::ScalarMultiplyLambda => [
                0x2b, 0x60, 0x03, 0x37, 0x64, 0xce, 0xec, 0x76, 0x23, 0xaf, 0x54, 0xd3, 0x86, 0xf6,
                0x87, 0xce, 0x30, 0x03, 0x25, 0x70, 0xb5, 0x6e, 0x9d, 0x5c, 0x31, 0x64, 0x8f, 0xbf,
                0x65, 0xe0, 0x14, 0x27,
            ],
            Elements::ScalarNegate => [
                0x64, 0x5a, 0xe5, 0x73, 0xd5, 0xd6, 0x8b, 0x2b, 0x1a, 0xfe, 0xac, 0xe9, 0x06, 0x39,
                0xbd, 0x3a, 0x28, 0x9e, 0xeb, 0x82, 0x61, 0xa6, 0xb4, 0x72, 0x4c, 0x23, 0xca, 0xa9,
                0x1b, 0x36, 0x59, 0x3c,
            ],
            Elements::ScalarNormalize => [
                0xdd, 0x37, 0x65, 0xa5, 0x16, 0x61, 0xcf, 0x66, 0x06, 0x92, 0x2f, 0x61, 0x9f, 0xca,
                0x64, 0x58, 0x52, 0x9d, 0x1e, 0x92, 0x56, 0x09, 0x83, 0x46, 0x92, 0x22, 0x47, 0xbd,
                0x04, 0x1b, 0xc1, 0x1f,
            ],
            Elements::ScalarSquare => [
                0x6b, 0xff, 0xd4, 0x35, 0x3f, 0x64, 0xfe, 0x9d, 0xf8, 0xcf, 0xcb, 0x58, 0x6d, 0xfd,
                0xd0, 0x04, 0x4c, 0xb0, 0x3a, 0x6d, 0xe9, 0x13, 0x7d, 0x4c, 0xe9, 0x94, 0xff, 0x9d,
                0xa1, 0x35, 0xff, 0xe4,
            ],
            Elements::Scale => [
                0x22, 0x9c, 0x9f, 0xaf, 0xad, 0xd9, 0x74, 0x5e, 0x00, 0xd1, 0x08, 0xb8, 0x2b, 0x83,
                0x62, 0x05, 0x9e, 0x83, 0x57, 0x0d, 0xfc, 0x36, 0xcb, 0x1a, 0x2c, 0xe9, 0xc5, 0xc2,
                0xd9, 0x13, 0xc6, 0x44,
            ],
            Elements::ScriptCMR => [
                0x8a, 0x5f, 0xf5, 0xe2, 0x03, 0xc9, 0x2e, 0x9b, 0x59, 0x7d, 0x2e, 0x67, 0x23, 0x10,
                0x22, 0x07, 0xa1, 0x19, 0x56, 0x25, 0xbf, 0x49, 0xf8, 0xdb, 0x84, 0x3a, 0x9f, 0xf1,
                0xe2, 0x15, 0x54, 0xb8,
            ],
            Elements::Sha256Block => [
                0x45, 0xb2, 0x78, 0x86, 0xa5, 0xee, 0xd4, 0x09, 0xef, 0x05, 0xb5, 0x30, 0xc7, 0x0e,
                0x36, 0x62, 0xfa, 0xee, 0x43, 0x12, 0xd3, 0x75, 0x9d, 0xf2, 0x41, 0x44, 0x98, 0x36,
                0x1b, 0x51, 0x17, 0xae,
            ],
            Elements::Sha256Ctx8Add1 => [
                0x41, 0x66, 0x54, 0x90, 0xd6, 0xed, 0xb4, 0xc5, 0xd5, 0x28, 0x45, 0x68, 0x2d, 0x7c,
                0xbd, 0x1e, 0x6c, 0x07, 0x45, 0xfa, 0xfc, 0xc0, 0x92, 0xf3, 0xef, 0x1b, 0x6f, 0xe9,
                0xcc, 0xb4, 0x2b, 0x59,
            ],
            Elements::Sha256Ctx8Add128 => [
                0xed, 0x92, 0xb8, 0x91, 0x9a, 0xbb, 0x89, 0x1c, 0x0f, 0xd2, 0x0f, 0xfe, 0xa6, 0xca,
                0xae, 0xa1, 0xbd, 0x79, 0xb9, 0xe3, 0xa8, 0x2a, 0xfe, 0x2c, 0xc0, 0xce, 0x47, 0x30,
                0x86, 0xf2, 0xae, 0x2a,
            ],
            Elements::Sha256Ctx8Add16 => [
                0x1a, 0xd8, 0xc2, 0x19, 0xf9, 0x71, 0xa4, 0xae, 0x29, 0xc2, 0x69, 0x95, 0x97, 0x15,
                0x0a, 0x11, 0x5b, 0xae, 0x2c, 0x09, 0xf3, 0xbc, 0x5f, 0xd8, 0x79, 0x50, 0x21, 0x63,
                0xee, 0xdd, 0x7b, 0x8d,
            ],
            Elements::Sha256Ctx8Add2 => [
                0x25, 0xab, 0x7a, 0xa9, 0xf7, 0xf8, 0x57, 0xbd, 0x26, 0x89, 0x5b, 0x89, 0x0d, 0xf3,
                0x33, 0x16, 0xd7, 0x0f, 0x70, 0xb0, 0xc2, 0x68, 0xea, 0x3e, 0xc0, 0x19, 0xe0, 0x3e,
                0x50, 0xc5, 0xbd, 0x7e,
            ],
            Elements::Sha256Ctx8Add256 => [
                0x10, 0xde, 0x9d, 0xa4, 0xe8, 0xf7, 0xa2, 0x91, 0x80, 0x84, 0x37, 0x91, 0x2e, 0x24,
                0xd2, 0x48, 0xe8, 0x4d, 0xac, 0xd7, 0x49, 0x68, 0x25, 0xd3, 0xca, 0xc5, 0x06, 0x72,
                0x3c, 0x21, 0x89, 0x1f,
            ],
            Elements::Sha256Ctx8Add32 => [
                0x69, 0x56, 0xf8, 0x07, 0xb5, 0x6e, 0xd6, 0x64, 0x07, 0xed, 0xe0, 0xbf, 0xff, 0xb6,
                0x18, 0x69, 0x9f, 0xc5, 0x78, 0x02, 0xbe, 0xd3, 0x44, 0xb6, 0x5e, 0xae, 0xf1, 0x39,
                0xec, 0x69, 0xef, 0x0c,
            ],
            Elements::Sha256Ctx8Add4 => [
                0x77, 0xd6, 0xf4, 0xb1, 0x3f, 0x2c, 0x5c, 0x42, 0x5f, 0x45, 0xfc, 0xf1, 0x86, 0x9f,
                0x9a, 0xce, 0x8c, 0xc8, 0x6b, 0xd4, 0x90, 0xe7, 0xb3, 0x14, 0x13, 0x4e, 0xf5, 0x99,
                0xbb, 0xf2, 0x21, 0xba,
            ],
            Elements::Sha256Ctx8Add512 => [
                0xa8, 0x76, 0x0d, 0xf3, 0x76, 0xf0, 0x7e, 0x76, 0x2d, 0xf6, 0x55, 0x9f, 0x69, 0xb1,
                0x17, 0x61, 0x38, 0x3b, 0x3f, 0xf6, 0xd8, 0x31, 0xfb, 0x5b, 0x54, 0x2b, 0x91, 0x72,
                0xfa, 0x34, 0xb9, 0x35,
            ],
            Elements::Sha256Ctx8Add64 => [
                0x80, 0x48, 0xd4, 0x90, 0x5c, 0x14, 0xb1, 0x82, 0xd8, 0xf8, 0x3c, 0x81, 0x8d, 0x03,
                0x22, 0xc2, 0xd1, 0xe4, 0x79, 0x50, 0x88, 0x98, 0x1a, 0xc7, 0xab, 0x99, 0xc8, 0x7b,
                0x12, 0xc6, 0xeb, 0x88,
            ],
            Elements::Sha256Ctx8Add8 => [
                0xe5, 0xc8, 0x39, 0xa5, 0xa3, 0x29, 0x6c, 0xb3, 0xcd, 0x38, 0x29, 0x7f, 0x92, 0xe2,
                0x63, 0x63, 0x0a, 0x6f, 0x29, 0x94, 0xc9, 0xbb, 0xbd, 0x2a, 0xfa, 0x7d, 0xa8, 0x3b,
                0x1e, 0xdc, 0xe8, 0xfb,
            ],
            Elements::Sha256Ctx8AddBuffer511 => [
                0x11, 0xb1, 0x3d, 0x73, 0x30, 0xae, 0x18, 0xe9, 0x19, 0x7b, 0x92, 0x1b, 0x81, 0x25,
                0x75, 0x2f, 0x8f, 0xd9, 0xb5, 0x19, 0x72, 0x04, 0xda, 0x5a, 0x7e, 0xa9, 0x24, 0xec,
                0x07, 0x2c, 0xf5, 0xfe,
            ],
            Elements::Sha256Ctx8Finalize => [
                0xe8, 0xc2, 0x70, 0xdf, 0x3d, 0x70, 0xa3, 0x95, 0x65, 0xd2, 0xc6, 0x45, 0x3d, 0x68,
                0xe6, 0x1f, 0x45, 0x97, 0x83, 0xf1, 0x9a, 0xa4, 0xf6, 0x2d, 0xb5, 0x6c, 0x9a, 0x1e,
                0xc1, 0x12, 0x5d, 0x6a,
            ],
            Elements::Sha256Ctx8Init => [
                0x6e, 0xcf, 0xda, 0x8c, 0x9b, 0xe3, 0xe8, 0x34, 0xc5, 0xad, 0x10, 0xcc, 0xf2, 0x5d,
                0xb1, 0x5e, 0xce, 0x9b, 0xa2, 0xb3, 0x73, 0xe8, 0x5c, 0xa1, 0x81, 0xe7, 0x37, 0x39,
                0x8d, 0xa6, 0xfc, 0x80,
            ],
            Elements::Sha256Iv => [
                0x9a, 0xdb, 0x29, 0x4a, 0xa5, 0x63, 0x9a, 0x79, 0x29, 0xac, 0xc5, 0xea, 0x85, 0x8b,
                0x2a, 0x89, 0xb5, 0xc5, 0xbf, 0xc1, 0xd0, 0x93, 0x6f, 0x89, 0x32, 0xae, 0x6b, 0xb2,
                0x9b, 0x3e, 0xe6, 0x69,
            ],
            Elements::SigAllHash => [
                0x99, 0x02, 0xbc, 0x0f, 0xf1, 0xf5, 0x9e, 0xd5, 0xd5, 0x92, 0x25, 0x8b, 0x7a, 0xa8,
                0x91, 0xa0, 0x35, 0x67, 0xc1, 0x3b, 0xf9, 0xf6, 0x79, 0x89, 0xf8, 0xd8, 0xb4, 0x63,
                0xf3, 0x86, 0x3f, 0x65,
            ],
            Elements::Some16 => [
                0xd9, 0x80, 0x12, 0xfd, 0x09, 0x90, 0xa1, 0x70, 0x4f, 0x92, 0x80, 0x3e, 0xfc, 0x14,
                0xa9, 0x56, 0xed, 0x3a, 0xd3, 0x7b, 0x3e, 0xfc, 0x40, 0x39, 0x19, 0xaf, 0x1e, 0x20,
                0x8d, 0x0b, 0x33, 0x20,
            ],
            Elements::Some32 => [
                0x7a, 0x24, 0x81, 0x51, 0xb7, 0xba, 0x67, 0x00, 0x30, 0xd0, 0x90, 0xf0, 0xa0, 0x0f,
                0x42, 0x3b, 0xbe, 0x15, 0x32, 0x76, 0xb3, 0xa7, 0xa3, 0x03, 0x33, 0x44, 0x9c, 0x67,
                0x60, 0x89, 0x6f, 0xab,
            ],
            Elements::Some64 => [
                0xa3, 0xa0, 0x6c, 0x9b, 0xc4, 0x78, 0xc0, 0xd4, 0x25, 0xcb, 0x26, 0x4e, 0xa9, 0xde,
                0x3e, 0x13, 0x8a, 0x7a, 0x13, 0x84, 0x9d, 0xf0, 0x61, 0x69, 0xe8, 0x5b, 0x50, 0x36,
                0xfe, 0x97, 0x69, 0x78,
            ],
            Elements::Some8 => [
                0xd6, 0xe3, 0x30, 0x68, 0x8d, 0x02, 0x75, 0x32, 0x09, 0x8a, 0x6e, 0x19, 0xdc, 0x41,
                0x7a, 0x97, 0xd9, 0xde, 0xb0, 0x5a, 0x86, 0xd1, 0xf9, 0xc0, 0xc3, 0xaa, 0xca, 0x74,
                0xeb, 0xc9, 0x58, 0x28,
            ],
            Elements::Subtract16 => [
                0x14, 0x23, 0x14, 0xe6, 0x74, 0x26, 0x8d, 0x2f, 0xb6, 0x57, 0xef, 0x84, 0x49, 0x85,
                0x38, 0x92, 0xee, 0xf4, 0x61, 0x40, 0x89, 0x6b, 0x72, 0x0e, 0x46, 0xe8, 0xb0, 0x0d,
                0x6d, 0x50, 0xaf, 0x54,
            ],
            Elements::Subtract32 => [
                0x4c, 0x45, 0x9b, 0x19, 0x16, 0xe3, 0x1d, 0x51, 0xcc, 0x03, 0xe6, 0x11, 0x58, 0x41,
                0xfb, 0x1f, 0x92, 0x33, 0xff, 0x2b, 0x38, 0xd8, 0xe7, 0x11, 0xe9, 0xe0, 0x16, 0xcf,
                0x89, 0x65, 0x31, 0xf6,
            ],
            Elements::Subtract64 => [
                0x6b, 0x35, 0x3c, 0xa0, 0x35, 0xbf, 0x5a, 0x64, 0xae, 0x2c, 0x2d, 0x4e, 0x77, 0x05,
                0xdf, 0xed, 0x8b, 0x06, 0xaa, 0x58, 0x9c, 0xa1, 0x2c, 0xc9, 0x23, 0xfd, 0x72, 0x4c,
                0xcb, 0xd1, 0x31, 0xed,
            ],
            Elements::Subtract8 => [
                0x22, 0xd2, 0x4d, 0x97, 0xa4, 0x01, 0x2b, 0x0a, 0x57, 0x78, 0x7d, 0x72, 0x94, 0xd4,
                0xd9, 0xdb, 0x9d, 0x2b, 0x85, 0xba, 0x89, 0xbb, 0xc8, 0xc1, 0x24, 0x8f, 0xc8, 0x3a,
                0xfd, 0x26, 0x47, 0x17,
            ],
            Elements::TapEnvHash => [
                0xa5, 0x3b, 0x8e, 0xbd, 0xe6, 0xed, 0x47, 0xbd, 0xaf, 0x72, 0xa4, 0xe1, 0x39, 0xbb,
                0xf4, 0xe1, 0x75, 0xcf, 0x69, 0xa8, 0x5d, 0xf9, 0x7f, 0x9f, 0x28, 0xc4, 0xa7, 0x9f,
                0x2f, 0x42, 0x77, 0x37,
            ],
            Elements::Tapbranch => [
                0x22, 0x51, 0x6d, 0xa6, 0x8f, 0x11, 0x08, 0x93, 0x5c, 0xb3, 0xf1, 0xc5, 0x8f, 0xa0,
                0xa4, 0xfd, 0x3b, 0x00, 0x4f, 0x81, 0xf5, 0x6d, 0x62, 0xf2, 0x93, 0x88, 0xd4, 0x20,
                0xe2, 0x95, 0x49, 0x91,
            ],
            Elements::TapbranchHash => [
                0x8f, 0x90, 0x86, 0x4a, 0x37, 0x61, 0x9d, 0xc0, 0xb6, 0x7c, 0x8b, 0x52, 0xe7, 0xd7,
                0xce, 0xeb, 0xd3, 0xd2, 0x5f, 0xf8, 0xca, 0xe7, 0xe7, 0xf2, 0xb1, 0x63, 0xfc, 0xaa,
                0x24, 0x86, 0x7e, 0x32,
            ],
            Elements::TapleafHash => [
                0xbc, 0xe8, 0xd0, 0x62, 0xd4, 0x81, 0xe8, 0x13, 0x44, 0x01, 0xdc, 0xe3, 0x1a, 0xe7,
                0x42, 0xac, 0xa3, 0xc2, 0x0f, 0x93, 0xec, 0x64, 0x0f, 0x90, 0x69, 0xab, 0x68, 0x3c,
                0x16, 0x73, 0x45, 0xda,
            ],
            Elements::TapleafVersion => [
                0xfe, 0x50, 0x3a, 0x57, 0x73, 0x0c, 0x66, 0xfc, 0xcd, 0x61, 0xeb, 0x5d, 0xf6, 0x3e,
                0x4d, 0xaa, 0x0c, 0x3e, 0x13, 0xde, 0x14, 0x34, 0xa2, 0x3d, 0x0f, 0xeb, 0x7c, 0x0d,
                0x72, 0x84, 0x4e, 0x54,
            ],
            Elements::TxHash => [
                0xb7, 0xd4, 0x66, 0x72, 0x64, 0xee, 0xcd, 0xa3, 0x60, 0x19, 0xf4, 0xe3, 0x26, 0xd3,
                0x93, 0x27, 0x70, 0x1b, 0x24, 0x88, 0x4e, 0x72, 0xdf, 0xb4, 0x81, 0x0f, 0x71, 0xc4,
                0xf1, 0x78, 0x2e, 0x7a,
            ],
            Elements::TxIsFinal => [
                0x7a, 0x65, 0xd5, 0xa6, 0x05, 0xd4, 0x70, 0x87, 0xb4, 0x7f, 0xfc, 0x6c, 0xff, 0x03,
                0x70, 0x44, 0xda, 0x70, 0x95, 0xb6, 0xe1, 0x90, 0xb4, 0x1a, 0x68, 0x67, 0x20, 0x71,
                0x9a, 0x5f, 0xba, 0xef,
            ],
            Elements::TxLockDistance => [
                0xec, 0xaf, 0x88, 0x85, 0x06, 0xca, 0xd0, 0x0a, 0xc9, 0x71, 0x14, 0x6b, 0x4a, 0x89,
                0xf5, 0xaa, 0xba, 0xbd, 0xf5, 0x83, 0x2e, 0x2c, 0xaf, 0xd9, 0xfc, 0x9a, 0x81, 0x7d,
                0x00, 0xb1, 0x85, 0xb8,
            ],
            Elements::TxLockDuration => [
                0xa1, 0x99, 0xb2, 0x6d, 0xf4, 0x49, 0x42, 0x88, 0xc5, 0xb5, 0x33, 0xbd, 0x39, 0x08,
                0x18, 0x46, 0x68, 0x1d, 0x2b, 0xe9, 0x81, 0x95, 0xfa, 0xfe, 0x80, 0x7c, 0x70, 0x72,
                0x72, 0x32, 0x05, 0x71,
            ],
            Elements::TxLockHeight => [
                0x56, 0xdf, 0x87, 0xac, 0x8a, 0x85, 0x7c, 0xa0, 0x6f, 0x87, 0x40, 0x66, 0xe4, 0x1b,
                0x05, 0x41, 0x63, 0x6b, 0x68, 0xc0, 0x68, 0xda, 0xd0, 0x9f, 0xa9, 0x72, 0x3a, 0xc7,
                0x68, 0x37, 0x65, 0x84,
            ],
            Elements::TxLockTime => [
                0xb8, 0xda, 0x79, 0x11, 0x8d, 0x5b, 0x43, 0x66, 0x4f, 0x51, 0xa1, 0x35, 0x58, 0xf8,
                0x43, 0x7a, 0x75, 0xec, 0x88, 0x6f, 0x11, 0x3f, 0x55, 0x59, 0x60, 0xd6, 0xe2, 0x8d,
                0x2b, 0x61, 0x48, 0x35,
            ],
            Elements::Verify => [
                0x02, 0x0e, 0x84, 0x01, 0x30, 0x30, 0xec, 0x69, 0xd9, 0xa9, 0x3f, 0xec, 0x71, 0x10,
                0xe7, 0x27, 0xea, 0xd5, 0x12, 0x88, 0x5f, 0xa3, 0xc5, 0x72, 0xd8, 0xcf, 0xc3, 0x47,
                0x2c, 0xa5, 0xc8, 0xe8,
            ],
            Elements::Version => [
                0x8f, 0xdf, 0x01, 0xa1, 0x85, 0x6c, 0xa9, 0x5d, 0xc1, 0x4d, 0x13, 0x8d, 0x4b, 0x7f,
                0xd0, 0x84, 0x3e, 0xa1, 0x11, 0x9e, 0x5f, 0x81, 0xcf, 0x4d, 0x43, 0xc8, 0x0a, 0x1f,
                0xef, 0x88, 0x83, 0x14,
            ],
            Elements::Xor16 => [
                0xa6, 0x10, 0x14, 0x40, 0x5b, 0xa1, 0x4a, 0x9b, 0xd4, 0x7d, 0x3f, 0x8f, 0x38, 0x73,
                0x19, 0x76, 0x5d, 0x34, 0xf0, 0x7e, 0xe2, 0x82, 0x58, 0xe1, 0x63, 0xa8, 0xac, 0x1f,
                0x03, 0x74, 0x87, 0xd2,
            ],
            Elements::Xor32 => [
                0x14, 0x6d, 0x01, 0x72, 0x51, 0x76, 0xeb, 0x83, 0x20, 0x0a, 0x70, 0xcc, 0x4b, 0x79,
                0x3c, 0xe9, 0x52, 0x5c, 0xfe, 0x28, 0x99, 0x44, 0xd4, 0xbd, 0xf2, 0xf1, 0xc2, 0x9f,
                0x89, 0x0a, 0x86, 0xf9,
            ],
            Elements::Xor64 => [
                0x7c, 0x89, 0x81, 0xc7, 0x6f, 0xeb, 0x14, 0x88, 0xe3, 0x86, 0x18, 0x06, 0xe8, 0x23,
                0x36, 0x39, 0x52, 0x56, 0xb5, 0xcb, 0xa4, 0x45, 0xab, 0xf4, 0x8f, 0xec, 0x54, 0x0d,
                0xd3, 0xf9, 0x43, 0x80,
            ],
            Elements::Xor8 => [
                0x4e, 0x79, 0x06, 0x51, 0x60, 0xc4, 0x53, 0x68, 0xa0, 0xf6, 0xa7, 0x60, 0xd3, 0x1d,
                0x9e, 0xeb, 0x0b, 0x92, 0xb3, 0x32, 0x74, 0x14, 0x72, 0x37, 0x10, 0x57, 0xad, 0x04,
                0x79, 0x02, 0xd3, 0xda,
            ],
            Elements::XorXor16 => [
                0x5b, 0x1c, 0xd2, 0x87, 0x52, 0x72, 0x90, 0x29, 0x6e, 0xba, 0x6d, 0xff, 0xc5, 0x64,
                0x53, 0x6d, 0x4f, 0x7a, 0x04, 0x12, 0x12, 0xaf, 0x8c, 0x27, 0x1c, 0xb5, 0xb1, 0x62,
                0xc2, 0x0b, 0x87, 0x1d,
            ],
            Elements::XorXor32 => [
                0x4f, 0x96, 0xb4, 0xf5, 0x65, 0x5d, 0xd3, 0xc7, 0x59, 0xfe, 0x0a, 0x4c, 0x38, 0xd1,
                0x07, 0x94, 0x79, 0xbe, 0xc4, 0xed, 0x99, 0x25, 0x29, 0xff, 0x1a, 0x07, 0x59, 0x24,
                0xbb, 0x1c, 0x63, 0xc1,
            ],
            Elements::XorXor64 => [
                0x35, 0x00, 0x87, 0xbf, 0xdc, 0x11, 0x9d, 0x8c, 0x85, 0x77, 0xe9, 0xa2, 0xf5, 0x23,
                0x9e, 0x28, 0xfe, 0xcc, 0x43, 0x0f, 0x4c, 0x2d, 0x41, 0xc0, 0x18, 0x7d, 0x26, 0x3f,
                0x79, 0x66, 0x79, 0x6c,
            ],
            Elements::XorXor8 => [
                0xdc, 0x4a, 0x35, 0x81, 0xee, 0xf9, 0xf3, 0xa4, 0x0e, 0xd1, 0x2f, 0xac, 0x10, 0x91,
                0x5c, 0xc6, 0x60, 0x43, 0x20, 0xc0, 0xde, 0xc9, 0x4f, 0x19, 0xe5, 0x85, 0x11, 0x27,
                0xf9, 0x4f, 0x2d, 0x36,
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
            Elements::Ch16 => b"*****22*22**22*22***22*22**22*22i",
            Elements::Ch32 => b"*il",
            Elements::Ch64 => b"*l*ll",
            Elements::Ch8 => b"****22*22**22*22****22*22**22*22***22*22**22*22",
            Elements::CheckLockDistance => b"****22*22**22*22***22*22**22*22",
            Elements::CheckLockDuration => b"****22*22**22*22***22*22**22*22",
            Elements::CheckLockHeight => b"i",
            Elements::CheckLockTime => b"i",
            Elements::CheckSigVerify => b"**h*hh*hh",
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
            Elements::FullMultiply16 => b"l",
            Elements::FullMultiply32 => b"*ll",
            Elements::FullMultiply64 => b"h",
            Elements::FullMultiply8 => b"i",
            Elements::FullSubtract16 => b"*2i",
            Elements::FullSubtract32 => b"*2l",
            Elements::FullSubtract64 => b"*2*ll",
            Elements::FullSubtract8 => b"*2****22*22**22*22***22*22**22*22",
            Elements::GeIsOnCurve => b"*hh",
            Elements::GeNegate => b"*hh",
            Elements::GejAdd => b"***hhh**hhh",
            Elements::GejDouble => b"**hhh",
            Elements::GejGeAdd => b"***hhh*hh",
            Elements::GejGeAddEx => b"***hhh*hh",
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
            Elements::InputOutpointsHash => b"1",
            Elements::InputPegin => b"i",
            Elements::InputPrevOutpoint => b"i",
            Elements::InputScriptHash => b"i",
            Elements::InputScriptSigHash => b"i",
            Elements::InputScriptSigsHash => b"1",
            Elements::InputScriptsHash => b"1",
            Elements::InputSequence => b"i",
            Elements::InputSequencesHash => b"1",
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
            Elements::IssuanceRangeProofsHash => b"1",
            Elements::IssuanceToken => b"i",
            Elements::IssuanceTokenAmount => b"i",
            Elements::IssuanceTokenAmountsHash => b"1",
            Elements::IssuanceTokenProof => b"i",
            Elements::IssuancesHash => b"1",
            Elements::Le16 => b"i",
            Elements::Le32 => b"l",
            Elements::Le64 => b"*ll",
            Elements::Le8 => b"****22*22**22*22***22*22**22*22",
            Elements::LinearCombination1 => b"**h**hhhh",
            Elements::LinearVerify1 => b"***h*hhh*hh",
            Elements::LockTime => b"1",
            Elements::Low16 => b"1",
            Elements::Low32 => b"1",
            Elements::Low64 => b"1",
            Elements::Low8 => b"1",
            Elements::Lt16 => b"i",
            Elements::Lt32 => b"l",
            Elements::Lt64 => b"*ll",
            Elements::Lt8 => b"****22*22**22*22***22*22**22*22",
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
            Elements::Or16 => b"i",
            Elements::Or32 => b"l",
            Elements::Or64 => b"*ll",
            Elements::Or8 => b"****22*22**22*22***22*22**22*22",
            Elements::OutpointHash => b"***+1h*+1*ll*+1l*+1i*+1****22*22**22*22***22*22**22*22+1***22*22**22*22*lh*+1h*hi",
            Elements::OutputAmount => b"i",
            Elements::OutputAmountsHash => b"1",
            Elements::OutputAsset => b"i",
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
            Elements::Some16 => b"****22*22**22*22***22*22**22*22",
            Elements::Some32 => b"i",
            Elements::Some64 => b"l",
            Elements::Some8 => b"***22*22**22*22",
            Elements::Subtract16 => b"i",
            Elements::Subtract32 => b"l",
            Elements::Subtract64 => b"*ll",
            Elements::Subtract8 => b"****22*22**22*22***22*22**22*22",
            Elements::TapEnvHash => b"1",
            Elements::Tapbranch => b"***22*22**22*22",
            Elements::TapbranchHash => b"1",
            Elements::TapleafHash => b"1",
            Elements::TapleafVersion => b"1",
            Elements::TxHash => b"1",
            Elements::TxIsFinal => b"1",
            Elements::TxLockDistance => b"1",
            Elements::TxLockDuration => b"1",
            Elements::TxLockHeight => b"1",
            Elements::TxLockTime => b"1",
            Elements::Verify => b"2",
            Elements::Version => b"1",
            Elements::Xor16 => b"i",
            Elements::Xor32 => b"l",
            Elements::Xor64 => b"*ll",
            Elements::Xor8 => b"****22*22**22*22***22*22**22*22",
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
            Elements::Ch16 => b"****22*22**22*22***22*22**22*22",
            Elements::Ch32 => b"i",
            Elements::Ch64 => b"l",
            Elements::Ch8 => b"***22*22**22*22",
            Elements::CheckLockDistance => b"1",
            Elements::CheckLockDuration => b"1",
            Elements::CheckLockHeight => b"1",
            Elements::CheckLockTime => b"1",
            Elements::CheckSigVerify => b"1",
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
            Elements::FullMultiply16 => b"i",
            Elements::FullMultiply32 => b"l",
            Elements::FullMultiply64 => b"*ll",
            Elements::FullMultiply8 => b"****22*22**22*22***22*22**22*22",
            Elements::FullSubtract16 => b"*2****22*22**22*22***22*22**22*22",
            Elements::FullSubtract32 => b"*2i",
            Elements::FullSubtract64 => b"*2l",
            Elements::FullSubtract8 => b"*2***22*22**22*22",
            Elements::GeIsOnCurve => b"2",
            Elements::GeNegate => b"*hh",
            Elements::GejAdd => b"**hhh",
            Elements::GejDouble => b"**hhh",
            Elements::GejGeAdd => b"**hhh",
            Elements::GejGeAddEx => b"*h**hhh",
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
            Elements::InputOutpointsHash => b"h",
            Elements::InputPegin => b"+1+1h",
            Elements::InputPrevOutpoint => b"+1*hi",
            Elements::InputScriptHash => b"+1h",
            Elements::InputScriptSigHash => b"+1h",
            Elements::InputScriptSigsHash => b"h",
            Elements::InputScriptsHash => b"h",
            Elements::InputSequence => b"+1i",
            Elements::InputSequencesHash => b"h",
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
            Elements::IssuanceRangeProofsHash => b"h",
            Elements::IssuanceToken => b"+1+1h",
            Elements::IssuanceTokenAmount => b"+1+1+*2hl",
            Elements::IssuanceTokenAmountsHash => b"h",
            Elements::IssuanceTokenProof => b"+1h",
            Elements::IssuancesHash => b"h",
            Elements::Le16 => b"2",
            Elements::Le32 => b"2",
            Elements::Le64 => b"2",
            Elements::Le8 => b"2",
            Elements::LinearCombination1 => b"**hhh",
            Elements::LinearVerify1 => b"1",
            Elements::LockTime => b"i",
            Elements::Low16 => b"****22*22**22*22***22*22**22*22",
            Elements::Low32 => b"i",
            Elements::Low64 => b"l",
            Elements::Low8 => b"***22*22**22*22",
            Elements::Lt16 => b"2",
            Elements::Lt32 => b"2",
            Elements::Lt64 => b"2",
            Elements::Lt8 => b"2",
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
            Elements::Or16 => b"****22*22**22*22***22*22**22*22",
            Elements::Or32 => b"i",
            Elements::Or64 => b"l",
            Elements::Or8 => b"***22*22**22*22",
            Elements::OutpointHash => b"**+1h*+1*ll*+1l*+1i*+1****22*22**22*22***22*22**22*22+1***22*22**22*22*lh",
            Elements::OutputAmount => b"+1*+*2hh+*2hl",
            Elements::OutputAmountsHash => b"h",
            Elements::OutputAsset => b"+1+*2hh",
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
            Elements::Some16 => b"2",
            Elements::Some32 => b"2",
            Elements::Some64 => b"2",
            Elements::Some8 => b"2",
            Elements::Subtract16 => b"*2****22*22**22*22***22*22**22*22",
            Elements::Subtract32 => b"*2i",
            Elements::Subtract64 => b"*2l",
            Elements::Subtract8 => b"*2***22*22**22*22",
            Elements::TapEnvHash => b"h",
            Elements::Tapbranch => b"+1h",
            Elements::TapbranchHash => b"h",
            Elements::TapleafHash => b"h",
            Elements::TapleafVersion => b"***22*22**22*22",
            Elements::TxHash => b"h",
            Elements::TxIsFinal => b"2",
            Elements::TxLockDistance => b"****22*22**22*22***22*22**22*22",
            Elements::TxLockDuration => b"****22*22**22*22***22*22**22*22",
            Elements::TxLockHeight => b"i",
            Elements::TxLockTime => b"i",
            Elements::Verify => b"1",
            Elements::Version => b"i",
            Elements::Xor16 => b"****22*22**22*22***22*22**22*22",
            Elements::Xor32 => b"i",
            Elements::Xor64 => b"l",
            Elements::Xor8 => b"***22*22**22*22",
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
            Elements::Low8 => (37, 8),
            Elements::Low16 => (304, 11),
            Elements::Low32 => (305, 11),
            Elements::Low64 => (306, 11),
            Elements::High8 => (45, 8),
            Elements::High16 => (368, 11),
            Elements::High32 => (369, 11),
            Elements::High64 => (370, 11),
            Elements::Complement8 => (389, 11),
            Elements::Complement16 => (3120, 14),
            Elements::Complement32 => (3121, 14),
            Elements::Complement64 => (3122, 14),
            Elements::And8 => (397, 11),
            Elements::And16 => (3184, 14),
            Elements::And32 => (3185, 14),
            Elements::And64 => (3186, 14),
            Elements::Or8 => (405, 11),
            Elements::Or16 => (3248, 14),
            Elements::Or32 => (3249, 14),
            Elements::Or64 => (3250, 14),
            Elements::Xor8 => (413, 11),
            Elements::Xor16 => (3312, 14),
            Elements::Xor32 => (3313, 14),
            Elements::Xor64 => (3314, 14),
            Elements::Maj8 => (837, 12),
            Elements::Maj16 => (6704, 15),
            Elements::Maj32 => (6705, 15),
            Elements::Maj64 => (6706, 15),
            Elements::XorXor8 => (845, 12),
            Elements::XorXor16 => (6768, 15),
            Elements::XorXor32 => (6769, 15),
            Elements::XorXor64 => (6770, 15),
            Elements::Ch8 => (853, 12),
            Elements::Ch16 => (6832, 15),
            Elements::Ch32 => (6833, 15),
            Elements::Ch64 => (6834, 15),
            Elements::Some8 => (861, 12),
            Elements::Some16 => (6896, 15),
            Elements::Some32 => (6897, 15),
            Elements::Some64 => (6898, 15),
            Elements::All8 => (869, 12),
            Elements::All16 => (6960, 15),
            Elements::All32 => (6961, 15),
            Elements::All64 => (6962, 15),
            Elements::Eq8 => (877, 12),
            Elements::Eq16 => (7024, 15),
            Elements::Eq32 => (7025, 15),
            Elements::Eq64 => (7026, 15),
            Elements::Eq256 => (14056, 16),
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
            Elements::CheckSigVerify => (98, 8),
            Elements::Bip0340Verify => (396, 10),
            Elements::ParseLock => (102, 8),
            Elements::ParseSequence => (412, 10),
            Elements::SigAllHash => (4, 3),
            Elements::TxHash => (20, 5),
            Elements::TapEnvHash => (21, 5),
            Elements::InputsHash => (176, 8),
            Elements::OutputsHash => (177, 8),
            Elements::IssuancesHash => (178, 8),
            Elements::InputUtxosHash => (179, 8),
            Elements::OutputAmountsHash => (360, 9),
            Elements::OutputScriptsHash => (361, 9),
            Elements::OutputNoncesHash => (362, 9),
            Elements::OutputRangeProofsHash => (363, 9),
            Elements::OutputSurjectionProofsHash => (364, 9),
            Elements::InputOutpointsHash => (365, 9),
            Elements::InputSequencesHash => (366, 9),
            Elements::InputAnnexesHash => (367, 9),
            Elements::InputScriptSigsHash => (5888, 13),
            Elements::IssuanceAssetAmountsHash => (5889, 13),
            Elements::IssuanceTokenAmountsHash => (5890, 13),
            Elements::IssuanceRangeProofsHash => (5891, 13),
            Elements::IssuanceBlindingEntropyHash => (5892, 13),
            Elements::InputAmountsHash => (5893, 13),
            Elements::InputScriptsHash => (5894, 13),
            Elements::TapleafHash => (5895, 13),
            Elements::TapbranchHash => (5896, 13),
            Elements::OutpointHash => (5897, 13),
            Elements::AssetAmountHash => (5898, 13),
            Elements::NonceHash => (5899, 13),
            Elements::AnnexHash => (5900, 13),
            Elements::BuildTapleafSimplicity => (5901, 13),
            Elements::BuildTapbranch => (5902, 13),
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
            Elements::OutputSurjectionProof => (14444, 14),
            Elements::OutputRangeProof => (14445, 14),
            Elements::CurrentPegin => (14447, 14),
            Elements::CurrentPrevOutpoint => (231168, 18),
            Elements::CurrentAsset => (231169, 18),
            Elements::CurrentAmount => (231170, 18),
            Elements::CurrentScriptHash => (231171, 18),
            Elements::CurrentSequence => (231172, 18),
            Elements::CurrentAnnexHash => (231173, 18),
            Elements::CurrentScriptSigHash => (231174, 18),
            Elements::CurrentReissuanceBlinding => (231175, 18),
            Elements::CurrentNewIssuanceContract => (231176, 18),
            Elements::CurrentReissuanceEntropy => (231177, 18),
            Elements::CurrentIssuanceAssetAmount => (231178, 18),
            Elements::CurrentIssuanceTokenAmount => (231179, 18),
            Elements::CurrentIssuanceAssetProof => (231180, 18),
            Elements::CurrentIssuanceTokenProof => (231181, 18),
            Elements::InputPegin => (231182, 18),
            Elements::InputPrevOutpoint => (231183, 18),
            Elements::InputAsset => (462368, 19),
            Elements::InputAmount => (462369, 19),
            Elements::InputScriptHash => (462370, 19),
            Elements::InputSequence => (462371, 19),
            Elements::InputAnnexHash => (462372, 19),
            Elements::InputScriptSigHash => (462373, 19),
            Elements::ReissuanceBlinding => (462374, 19),
            Elements::NewIssuanceContract => (462375, 19),
            Elements::ReissuanceEntropy => (462376, 19),
            Elements::IssuanceAssetAmount => (462377, 19),
            Elements::IssuanceTokenAmount => (462378, 19),
            Elements::IssuanceAssetProof => (462379, 19),
            Elements::IssuanceTokenProof => (462380, 19),
            Elements::TapleafVersion => (462381, 19),
            Elements::Tapbranch => (462382, 19),
            Elements::Version => (462383, 19),
            Elements::GenesisBlockHash => (462384, 19),
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
                                0 => {},
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
                                0 => {},
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
                                            0 => {},
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
                                            0 => {},
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
                                            0 => {},
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
                                            0 => {},
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
                                                0 => {},
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
                                                0 => {},
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
                                                0 => {},
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
                                                0 => {},
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
                                                0 => {},
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
                                                                    1 => {}
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
                                                                                1 => {}
                                                                            },
                                                                            1 => {
                                                                                0 => {},
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
                                        0 => {Elements::InputsHash},
                                        1 => {Elements::OutputsHash}
                                    },
                                    1 => {
                                        0 => {Elements::IssuancesHash},
                                        1 => {Elements::InputUtxosHash}
                                    }
                                },
                                1 => {
                                    0 => {
                                        0 => {
                                            0 => {Elements::OutputAmountsHash},
                                            1 => {Elements::OutputScriptsHash}
                                        },
                                        1 => {
                                            0 => {Elements::OutputNoncesHash},
                                            1 => {Elements::OutputRangeProofsHash}
                                        }
                                    },
                                    1 => {
                                        0 => {
                                            0 => {Elements::OutputSurjectionProofsHash},
                                            1 => {Elements::InputOutpointsHash}
                                        },
                                        1 => {
                                            0 => {Elements::InputSequencesHash},
                                            1 => {Elements::InputAnnexesHash}
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
                                                            0 => {Elements::InputScriptSigsHash},
                                                            1 => {Elements::IssuanceAssetAmountsHash}
                                                        },
                                                        1 => {
                                                            0 => {Elements::IssuanceTokenAmountsHash},
                                                            1 => {Elements::IssuanceRangeProofsHash}
                                                        }
                                                    },
                                                    1 => {
                                                        0 => {
                                                            0 => {Elements::IssuanceBlindingEntropyHash},
                                                            1 => {Elements::InputAmountsHash}
                                                        },
                                                        1 => {
                                                            0 => {Elements::InputScriptsHash},
                                                            1 => {Elements::TapleafHash}
                                                        }
                                                    }
                                                },
                                                1 => {
                                                    0 => {
                                                        0 => {
                                                            0 => {Elements::TapbranchHash},
                                                            1 => {Elements::OutpointHash}
                                                        },
                                                        1 => {
                                                            0 => {Elements::AssetAmountHash},
                                                            1 => {Elements::NonceHash}
                                                        }
                                                    },
                                                    1 => {
                                                        0 => {
                                                            0 => {Elements::AnnexHash},
                                                            1 => {Elements::BuildTapleafSimplicity}
                                                        },
                                                        1 => {
                                                            0 => {Elements::BuildTapbranch},
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
                                                                0 => {Elements::OutputSurjectionProof},
                                                                1 => {Elements::OutputRangeProof}
                                                            },
                                                            1 => {
                                                                0 => {},
                                                                1 => {Elements::CurrentPegin}
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
                                                                                0 => {Elements::CurrentPrevOutpoint},
                                                                                1 => {Elements::CurrentAsset}
                                                                            },
                                                                            1 => {
                                                                                0 => {Elements::CurrentAmount},
                                                                                1 => {Elements::CurrentScriptHash}
                                                                            }
                                                                        },
                                                                        1 => {
                                                                            0 => {
                                                                                0 => {Elements::CurrentSequence},
                                                                                1 => {Elements::CurrentAnnexHash}
                                                                            },
                                                                            1 => {
                                                                                0 => {Elements::CurrentScriptSigHash},
                                                                                1 => {Elements::CurrentReissuanceBlinding}
                                                                            }
                                                                        }
                                                                    },
                                                                    1 => {
                                                                        0 => {
                                                                            0 => {
                                                                                0 => {Elements::CurrentNewIssuanceContract},
                                                                                1 => {Elements::CurrentReissuanceEntropy}
                                                                            },
                                                                            1 => {
                                                                                0 => {Elements::CurrentIssuanceAssetAmount},
                                                                                1 => {Elements::CurrentIssuanceTokenAmount}
                                                                            }
                                                                        },
                                                                        1 => {
                                                                            0 => {
                                                                                0 => {Elements::CurrentIssuanceAssetProof},
                                                                                1 => {Elements::CurrentIssuanceTokenProof}
                                                                            },
                                                                            1 => {
                                                                                0 => {Elements::InputPegin},
                                                                                1 => {Elements::InputPrevOutpoint}
                                                                            }
                                                                        }
                                                                    }
                                                                },
                                                                1 => {
                                                                    0 => {
                                                                        0 => {
                                                                            0 => {
                                                                                0 => {
                                                                                    0 => {Elements::InputAsset},
                                                                                    1 => {Elements::InputAmount}
                                                                                },
                                                                                1 => {
                                                                                    0 => {Elements::InputScriptHash},
                                                                                    1 => {Elements::InputSequence}
                                                                                }
                                                                            },
                                                                            1 => {
                                                                                0 => {
                                                                                    0 => {Elements::InputAnnexHash},
                                                                                    1 => {Elements::InputScriptSigHash}
                                                                                },
                                                                                1 => {
                                                                                    0 => {Elements::ReissuanceBlinding},
                                                                                    1 => {Elements::NewIssuanceContract}
                                                                                }
                                                                            }
                                                                        },
                                                                        1 => {
                                                                            0 => {
                                                                                0 => {
                                                                                    0 => {Elements::ReissuanceEntropy},
                                                                                    1 => {Elements::IssuanceAssetAmount}
                                                                                },
                                                                                1 => {
                                                                                    0 => {Elements::IssuanceTokenAmount},
                                                                                    1 => {Elements::IssuanceAssetProof}
                                                                                }
                                                                            },
                                                                            1 => {
                                                                                0 => {
                                                                                    0 => {Elements::IssuanceTokenProof},
                                                                                    1 => {Elements::TapleafVersion}
                                                                                },
                                                                                1 => {
                                                                                    0 => {Elements::Tapbranch},
                                                                                    1 => {Elements::Version}
                                                                                }
                                                                            }
                                                                        }
                                                                    },
                                                                    1 => {
                                                                        0 => {
                                                                            0 => {
                                                                                0 => {
                                                                                    0 => {Elements::GenesisBlockHash},
                                                                                    1 => {}
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
            Elements::Ch16 => &simplicity_sys::c_jets::jets_wrapper::ch_16,
            Elements::Ch32 => &simplicity_sys::c_jets::jets_wrapper::ch_32,
            Elements::Ch64 => &simplicity_sys::c_jets::jets_wrapper::ch_64,
            Elements::Ch8 => &simplicity_sys::c_jets::jets_wrapper::ch_8,
            Elements::CheckLockDistance => &simplicity_sys::c_jets::jets_wrapper::check_lock_distance,
            Elements::CheckLockDuration => &simplicity_sys::c_jets::jets_wrapper::check_lock_duration,
            Elements::CheckLockHeight => &simplicity_sys::c_jets::jets_wrapper::check_lock_height,
            Elements::CheckLockTime => &simplicity_sys::c_jets::jets_wrapper::check_lock_time,
            Elements::CheckSigVerify => &simplicity_sys::c_jets::jets_wrapper::check_sig_verify,
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
            Elements::FullMultiply16 => &simplicity_sys::c_jets::jets_wrapper::full_multiply_16,
            Elements::FullMultiply32 => &simplicity_sys::c_jets::jets_wrapper::full_multiply_32,
            Elements::FullMultiply64 => &simplicity_sys::c_jets::jets_wrapper::full_multiply_64,
            Elements::FullMultiply8 => &simplicity_sys::c_jets::jets_wrapper::full_multiply_8,
            Elements::FullSubtract16 => &simplicity_sys::c_jets::jets_wrapper::full_subtract_16,
            Elements::FullSubtract32 => &simplicity_sys::c_jets::jets_wrapper::full_subtract_32,
            Elements::FullSubtract64 => &simplicity_sys::c_jets::jets_wrapper::full_subtract_64,
            Elements::FullSubtract8 => &simplicity_sys::c_jets::jets_wrapper::full_subtract_8,
            Elements::GeIsOnCurve => &simplicity_sys::c_jets::jets_wrapper::ge_is_on_curve,
            Elements::GeNegate => &simplicity_sys::c_jets::jets_wrapper::ge_negate,
            Elements::GejAdd => &simplicity_sys::c_jets::jets_wrapper::gej_add,
            Elements::GejDouble => &simplicity_sys::c_jets::jets_wrapper::gej_double,
            Elements::GejGeAdd => &simplicity_sys::c_jets::jets_wrapper::gej_ge_add,
            Elements::GejGeAddEx => &simplicity_sys::c_jets::jets_wrapper::gej_ge_add_ex,
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
            Elements::InputOutpointsHash => &simplicity_sys::c_jets::jets_wrapper::input_outpoints_hash,
            Elements::InputPegin => &simplicity_sys::c_jets::jets_wrapper::input_pegin,
            Elements::InputPrevOutpoint => &simplicity_sys::c_jets::jets_wrapper::input_prev_outpoint,
            Elements::InputScriptHash => &simplicity_sys::c_jets::jets_wrapper::input_script_hash,
            Elements::InputScriptSigHash => &simplicity_sys::c_jets::jets_wrapper::input_script_sig_hash,
            Elements::InputScriptSigsHash => &simplicity_sys::c_jets::jets_wrapper::input_script_sigs_hash,
            Elements::InputScriptsHash => &simplicity_sys::c_jets::jets_wrapper::input_scripts_hash,
            Elements::InputSequence => &simplicity_sys::c_jets::jets_wrapper::input_sequence,
            Elements::InputSequencesHash => &simplicity_sys::c_jets::jets_wrapper::input_sequences_hash,
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
            Elements::IssuanceRangeProofsHash => &simplicity_sys::c_jets::jets_wrapper::issuance_range_proofs_hash,
            Elements::IssuanceToken => &simplicity_sys::c_jets::jets_wrapper::issuance_token,
            Elements::IssuanceTokenAmount => &simplicity_sys::c_jets::jets_wrapper::issuance_token_amount,
            Elements::IssuanceTokenAmountsHash => &simplicity_sys::c_jets::jets_wrapper::issuance_token_amounts_hash,
            Elements::IssuanceTokenProof => &simplicity_sys::c_jets::jets_wrapper::issuance_token_proof,
            Elements::IssuancesHash => &simplicity_sys::c_jets::jets_wrapper::issuances_hash,
            Elements::Le16 => &simplicity_sys::c_jets::jets_wrapper::le_16,
            Elements::Le32 => &simplicity_sys::c_jets::jets_wrapper::le_32,
            Elements::Le64 => &simplicity_sys::c_jets::jets_wrapper::le_64,
            Elements::Le8 => &simplicity_sys::c_jets::jets_wrapper::le_8,
            Elements::LinearCombination1 => &simplicity_sys::c_jets::jets_wrapper::linear_combination_1,
            Elements::LinearVerify1 => &simplicity_sys::c_jets::jets_wrapper::linear_verify_1,
            Elements::LockTime => &simplicity_sys::c_jets::jets_wrapper::lock_time,
            Elements::Low16 => &simplicity_sys::c_jets::jets_wrapper::low_16,
            Elements::Low32 => &simplicity_sys::c_jets::jets_wrapper::low_32,
            Elements::Low64 => &simplicity_sys::c_jets::jets_wrapper::low_64,
            Elements::Low8 => &simplicity_sys::c_jets::jets_wrapper::low_8,
            Elements::Lt16 => &simplicity_sys::c_jets::jets_wrapper::lt_16,
            Elements::Lt32 => &simplicity_sys::c_jets::jets_wrapper::lt_32,
            Elements::Lt64 => &simplicity_sys::c_jets::jets_wrapper::lt_64,
            Elements::Lt8 => &simplicity_sys::c_jets::jets_wrapper::lt_8,
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
            Elements::Or16 => &simplicity_sys::c_jets::jets_wrapper::or_16,
            Elements::Or32 => &simplicity_sys::c_jets::jets_wrapper::or_32,
            Elements::Or64 => &simplicity_sys::c_jets::jets_wrapper::or_64,
            Elements::Or8 => &simplicity_sys::c_jets::jets_wrapper::or_8,
            Elements::OutpointHash => &simplicity_sys::c_jets::jets_wrapper::outpoint_hash,
            Elements::OutputAmount => &simplicity_sys::c_jets::jets_wrapper::output_amount,
            Elements::OutputAmountsHash => &simplicity_sys::c_jets::jets_wrapper::output_amounts_hash,
            Elements::OutputAsset => &simplicity_sys::c_jets::jets_wrapper::output_asset,
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
            Elements::Some16 => &simplicity_sys::c_jets::jets_wrapper::some_16,
            Elements::Some32 => &simplicity_sys::c_jets::jets_wrapper::some_32,
            Elements::Some64 => &simplicity_sys::c_jets::jets_wrapper::some_64,
            Elements::Some8 => &simplicity_sys::c_jets::jets_wrapper::some_8,
            Elements::Subtract16 => &simplicity_sys::c_jets::jets_wrapper::subtract_16,
            Elements::Subtract32 => &simplicity_sys::c_jets::jets_wrapper::subtract_32,
            Elements::Subtract64 => &simplicity_sys::c_jets::jets_wrapper::subtract_64,
            Elements::Subtract8 => &simplicity_sys::c_jets::jets_wrapper::subtract_8,
            Elements::TapEnvHash => &simplicity_sys::c_jets::jets_wrapper::tap_env_hash,
            Elements::Tapbranch => &simplicity_sys::c_jets::jets_wrapper::tapbranch,
            Elements::TapbranchHash => &simplicity_sys::c_jets::jets_wrapper::tapbranch_hash,
            Elements::TapleafHash => &simplicity_sys::c_jets::jets_wrapper::tapleaf_hash,
            Elements::TapleafVersion => &simplicity_sys::c_jets::jets_wrapper::tapleaf_version,
            Elements::TxHash => &simplicity_sys::c_jets::jets_wrapper::tx_hash,
            Elements::TxIsFinal => &simplicity_sys::c_jets::jets_wrapper::tx_is_final,
            Elements::TxLockDistance => &simplicity_sys::c_jets::jets_wrapper::tx_lock_distance,
            Elements::TxLockDuration => &simplicity_sys::c_jets::jets_wrapper::tx_lock_duration,
            Elements::TxLockHeight => &simplicity_sys::c_jets::jets_wrapper::tx_lock_height,
            Elements::TxLockTime => &simplicity_sys::c_jets::jets_wrapper::tx_lock_time,
            Elements::Verify => &simplicity_sys::c_jets::jets_wrapper::verify,
            Elements::Version => &simplicity_sys::c_jets::jets_wrapper::version,
            Elements::Xor16 => &simplicity_sys::c_jets::jets_wrapper::xor_16,
            Elements::Xor32 => &simplicity_sys::c_jets::jets_wrapper::xor_32,
            Elements::Xor64 => &simplicity_sys::c_jets::jets_wrapper::xor_64,
            Elements::Xor8 => &simplicity_sys::c_jets::jets_wrapper::xor_8,
            Elements::XorXor16 => &simplicity_sys::c_jets::jets_wrapper::xor_xor_16,
            Elements::XorXor32 => &simplicity_sys::c_jets::jets_wrapper::xor_xor_32,
            Elements::XorXor64 => &simplicity_sys::c_jets::jets_wrapper::xor_xor_64,
            Elements::XorXor8 => &simplicity_sys::c_jets::jets_wrapper::xor_xor_8,
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
            Elements::Ch16 => f.write_str("ch_16"),
            Elements::Ch32 => f.write_str("ch_32"),
            Elements::Ch64 => f.write_str("ch_64"),
            Elements::Ch8 => f.write_str("ch_8"),
            Elements::CheckLockDistance => f.write_str("check_lock_distance"),
            Elements::CheckLockDuration => f.write_str("check_lock_duration"),
            Elements::CheckLockHeight => f.write_str("check_lock_height"),
            Elements::CheckLockTime => f.write_str("check_lock_time"),
            Elements::CheckSigVerify => f.write_str("check_sig_verify"),
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
            Elements::FullMultiply16 => f.write_str("full_multiply_16"),
            Elements::FullMultiply32 => f.write_str("full_multiply_32"),
            Elements::FullMultiply64 => f.write_str("full_multiply_64"),
            Elements::FullMultiply8 => f.write_str("full_multiply_8"),
            Elements::FullSubtract16 => f.write_str("full_subtract_16"),
            Elements::FullSubtract32 => f.write_str("full_subtract_32"),
            Elements::FullSubtract64 => f.write_str("full_subtract_64"),
            Elements::FullSubtract8 => f.write_str("full_subtract_8"),
            Elements::GeIsOnCurve => f.write_str("ge_is_on_curve"),
            Elements::GeNegate => f.write_str("ge_negate"),
            Elements::GejAdd => f.write_str("gej_add"),
            Elements::GejDouble => f.write_str("gej_double"),
            Elements::GejGeAdd => f.write_str("gej_ge_add"),
            Elements::GejGeAddEx => f.write_str("gej_ge_add_ex"),
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
            Elements::InputOutpointsHash => f.write_str("input_outpoints_hash"),
            Elements::InputPegin => f.write_str("input_pegin"),
            Elements::InputPrevOutpoint => f.write_str("input_prev_outpoint"),
            Elements::InputScriptHash => f.write_str("input_script_hash"),
            Elements::InputScriptSigHash => f.write_str("input_script_sig_hash"),
            Elements::InputScriptSigsHash => f.write_str("input_script_sigs_hash"),
            Elements::InputScriptsHash => f.write_str("input_scripts_hash"),
            Elements::InputSequence => f.write_str("input_sequence"),
            Elements::InputSequencesHash => f.write_str("input_sequences_hash"),
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
            Elements::IssuanceRangeProofsHash => f.write_str("issuance_range_proofs_hash"),
            Elements::IssuanceToken => f.write_str("issuance_token"),
            Elements::IssuanceTokenAmount => f.write_str("issuance_token_amount"),
            Elements::IssuanceTokenAmountsHash => f.write_str("issuance_token_amounts_hash"),
            Elements::IssuanceTokenProof => f.write_str("issuance_token_proof"),
            Elements::IssuancesHash => f.write_str("issuances_hash"),
            Elements::Le16 => f.write_str("le_16"),
            Elements::Le32 => f.write_str("le_32"),
            Elements::Le64 => f.write_str("le_64"),
            Elements::Le8 => f.write_str("le_8"),
            Elements::LinearCombination1 => f.write_str("linear_combination_1"),
            Elements::LinearVerify1 => f.write_str("linear_verify_1"),
            Elements::LockTime => f.write_str("lock_time"),
            Elements::Low16 => f.write_str("low_16"),
            Elements::Low32 => f.write_str("low_32"),
            Elements::Low64 => f.write_str("low_64"),
            Elements::Low8 => f.write_str("low_8"),
            Elements::Lt16 => f.write_str("lt_16"),
            Elements::Lt32 => f.write_str("lt_32"),
            Elements::Lt64 => f.write_str("lt_64"),
            Elements::Lt8 => f.write_str("lt_8"),
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
            Elements::Or16 => f.write_str("or_16"),
            Elements::Or32 => f.write_str("or_32"),
            Elements::Or64 => f.write_str("or_64"),
            Elements::Or8 => f.write_str("or_8"),
            Elements::OutpointHash => f.write_str("outpoint_hash"),
            Elements::OutputAmount => f.write_str("output_amount"),
            Elements::OutputAmountsHash => f.write_str("output_amounts_hash"),
            Elements::OutputAsset => f.write_str("output_asset"),
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
            Elements::Some16 => f.write_str("some_16"),
            Elements::Some32 => f.write_str("some_32"),
            Elements::Some64 => f.write_str("some_64"),
            Elements::Some8 => f.write_str("some_8"),
            Elements::Subtract16 => f.write_str("subtract_16"),
            Elements::Subtract32 => f.write_str("subtract_32"),
            Elements::Subtract64 => f.write_str("subtract_64"),
            Elements::Subtract8 => f.write_str("subtract_8"),
            Elements::TapEnvHash => f.write_str("tap_env_hash"),
            Elements::Tapbranch => f.write_str("tapbranch"),
            Elements::TapbranchHash => f.write_str("tapbranch_hash"),
            Elements::TapleafHash => f.write_str("tapleaf_hash"),
            Elements::TapleafVersion => f.write_str("tapleaf_version"),
            Elements::TxHash => f.write_str("tx_hash"),
            Elements::TxIsFinal => f.write_str("tx_is_final"),
            Elements::TxLockDistance => f.write_str("tx_lock_distance"),
            Elements::TxLockDuration => f.write_str("tx_lock_duration"),
            Elements::TxLockHeight => f.write_str("tx_lock_height"),
            Elements::TxLockTime => f.write_str("tx_lock_time"),
            Elements::Verify => f.write_str("verify"),
            Elements::Version => f.write_str("version"),
            Elements::Xor16 => f.write_str("xor_16"),
            Elements::Xor32 => f.write_str("xor_32"),
            Elements::Xor64 => f.write_str("xor_64"),
            Elements::Xor8 => f.write_str("xor_8"),
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
            "ch_16" => Ok(Elements::Ch16),
            "ch_32" => Ok(Elements::Ch32),
            "ch_64" => Ok(Elements::Ch64),
            "ch_8" => Ok(Elements::Ch8),
            "check_lock_distance" => Ok(Elements::CheckLockDistance),
            "check_lock_duration" => Ok(Elements::CheckLockDuration),
            "check_lock_height" => Ok(Elements::CheckLockHeight),
            "check_lock_time" => Ok(Elements::CheckLockTime),
            "check_sig_verify" => Ok(Elements::CheckSigVerify),
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
            "full_multiply_16" => Ok(Elements::FullMultiply16),
            "full_multiply_32" => Ok(Elements::FullMultiply32),
            "full_multiply_64" => Ok(Elements::FullMultiply64),
            "full_multiply_8" => Ok(Elements::FullMultiply8),
            "full_subtract_16" => Ok(Elements::FullSubtract16),
            "full_subtract_32" => Ok(Elements::FullSubtract32),
            "full_subtract_64" => Ok(Elements::FullSubtract64),
            "full_subtract_8" => Ok(Elements::FullSubtract8),
            "ge_is_on_curve" => Ok(Elements::GeIsOnCurve),
            "ge_negate" => Ok(Elements::GeNegate),
            "gej_add" => Ok(Elements::GejAdd),
            "gej_double" => Ok(Elements::GejDouble),
            "gej_ge_add" => Ok(Elements::GejGeAdd),
            "gej_ge_add_ex" => Ok(Elements::GejGeAddEx),
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
            "input_outpoints_hash" => Ok(Elements::InputOutpointsHash),
            "input_pegin" => Ok(Elements::InputPegin),
            "input_prev_outpoint" => Ok(Elements::InputPrevOutpoint),
            "input_script_hash" => Ok(Elements::InputScriptHash),
            "input_script_sig_hash" => Ok(Elements::InputScriptSigHash),
            "input_script_sigs_hash" => Ok(Elements::InputScriptSigsHash),
            "input_scripts_hash" => Ok(Elements::InputScriptsHash),
            "input_sequence" => Ok(Elements::InputSequence),
            "input_sequences_hash" => Ok(Elements::InputSequencesHash),
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
            "issuance_range_proofs_hash" => Ok(Elements::IssuanceRangeProofsHash),
            "issuance_token" => Ok(Elements::IssuanceToken),
            "issuance_token_amount" => Ok(Elements::IssuanceTokenAmount),
            "issuance_token_amounts_hash" => Ok(Elements::IssuanceTokenAmountsHash),
            "issuance_token_proof" => Ok(Elements::IssuanceTokenProof),
            "issuances_hash" => Ok(Elements::IssuancesHash),
            "le_16" => Ok(Elements::Le16),
            "le_32" => Ok(Elements::Le32),
            "le_64" => Ok(Elements::Le64),
            "le_8" => Ok(Elements::Le8),
            "linear_combination_1" => Ok(Elements::LinearCombination1),
            "linear_verify_1" => Ok(Elements::LinearVerify1),
            "lock_time" => Ok(Elements::LockTime),
            "low_16" => Ok(Elements::Low16),
            "low_32" => Ok(Elements::Low32),
            "low_64" => Ok(Elements::Low64),
            "low_8" => Ok(Elements::Low8),
            "lt_16" => Ok(Elements::Lt16),
            "lt_32" => Ok(Elements::Lt32),
            "lt_64" => Ok(Elements::Lt64),
            "lt_8" => Ok(Elements::Lt8),
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
            "or_16" => Ok(Elements::Or16),
            "or_32" => Ok(Elements::Or32),
            "or_64" => Ok(Elements::Or64),
            "or_8" => Ok(Elements::Or8),
            "outpoint_hash" => Ok(Elements::OutpointHash),
            "output_amount" => Ok(Elements::OutputAmount),
            "output_amounts_hash" => Ok(Elements::OutputAmountsHash),
            "output_asset" => Ok(Elements::OutputAsset),
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
            "some_16" => Ok(Elements::Some16),
            "some_32" => Ok(Elements::Some32),
            "some_64" => Ok(Elements::Some64),
            "some_8" => Ok(Elements::Some8),
            "subtract_16" => Ok(Elements::Subtract16),
            "subtract_32" => Ok(Elements::Subtract32),
            "subtract_64" => Ok(Elements::Subtract64),
            "subtract_8" => Ok(Elements::Subtract8),
            "tap_env_hash" => Ok(Elements::TapEnvHash),
            "tapbranch" => Ok(Elements::Tapbranch),
            "tapbranch_hash" => Ok(Elements::TapbranchHash),
            "tapleaf_hash" => Ok(Elements::TapleafHash),
            "tapleaf_version" => Ok(Elements::TapleafVersion),
            "tx_hash" => Ok(Elements::TxHash),
            "tx_is_final" => Ok(Elements::TxIsFinal),
            "tx_lock_distance" => Ok(Elements::TxLockDistance),
            "tx_lock_duration" => Ok(Elements::TxLockDuration),
            "tx_lock_height" => Ok(Elements::TxLockHeight),
            "tx_lock_time" => Ok(Elements::TxLockTime),
            "verify" => Ok(Elements::Verify),
            "version" => Ok(Elements::Version),
            "xor_16" => Ok(Elements::Xor16),
            "xor_32" => Ok(Elements::Xor32),
            "xor_64" => Ok(Elements::Xor64),
            "xor_8" => Ok(Elements::Xor8),
            "xor_xor_16" => Ok(Elements::XorXor16),
            "xor_xor_32" => Ok(Elements::XorXor32),
            "xor_xor_64" => Ok(Elements::XorXor64),
            "xor_xor_8" => Ok(Elements::XorXor8),
            x => Err(crate::Error::InvalidJetName(x.to_owned())),
        }
    }
}
