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
use std::sync::Arc;

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
    TapleafHash,
    TapleafVersion,
    Tappath,
    TappathHash,
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

    type Environment = ElementsEnv<Arc<elements::Transaction>>;
    type CJetEnvironment = CElementsTxEnv;

    fn c_jet_env<'env>(&self, env: &'env Self::Environment) -> &'env Self::CJetEnvironment {
        env.c_tx_env()
    }

    fn cmr(&self) -> Cmr {
        let bytes = match self {
            Elements::Add16 => [
                0xab, 0x71, 0x0d, 0x45, 0xbd, 0xda, 0x1e, 0xe2, 0x4e, 0xb1, 0x8f, 0x64, 0xb9, 0x55,
                0xac, 0xc2, 0x7f, 0x07, 0x63, 0x9e, 0x22, 0xa0, 0x29, 0x05, 0x38, 0x12, 0xb6, 0xf5,
                0xac, 0xd0, 0x0c, 0x05,
            ],
            Elements::Add32 => [
                0x91, 0xed, 0x4c, 0x3d, 0x0d, 0x94, 0xd1, 0x11, 0xc7, 0x2b, 0xd4, 0x31, 0xc0, 0xac,
                0xbb, 0x0d, 0xe1, 0xc5, 0x89, 0x95, 0x64, 0x0b, 0x58, 0xcf, 0x70, 0xd3, 0x7e, 0xad,
                0xb7, 0x40, 0x10, 0x1a,
            ],
            Elements::Add64 => [
                0xa5, 0xe6, 0x55, 0x6a, 0x21, 0xcc, 0xe1, 0x05, 0xe9, 0x94, 0xe1, 0x65, 0xa4, 0x03,
                0x65, 0x7e, 0xea, 0x7e, 0xef, 0x36, 0x1b, 0x10, 0x41, 0x04, 0x5f, 0xf3, 0x26, 0xa3,
                0x27, 0x02, 0xaa, 0xfe,
            ],
            Elements::Add8 => [
                0x51, 0xe5, 0xaf, 0x2c, 0x99, 0xe0, 0xd4, 0xd3, 0x93, 0x43, 0x0b, 0x3e, 0xd3, 0x2d,
                0xb0, 0x83, 0xa9, 0xa0, 0x20, 0x64, 0x6d, 0x36, 0x64, 0x4e, 0x54, 0x70, 0xbb, 0x9d,
                0xb9, 0xfd, 0x0e, 0x22,
            ],
            Elements::All16 => [
                0xcc, 0xc0, 0x06, 0x86, 0x73, 0xe3, 0x0d, 0x34, 0x42, 0xe6, 0x16, 0x48, 0x7a, 0x4b,
                0x53, 0xa1, 0x95, 0xf5, 0xc2, 0xcf, 0x13, 0x89, 0x1c, 0xc6, 0x3f, 0x7e, 0x86, 0x2b,
                0xa0, 0xd9, 0x29, 0xa8,
            ],
            Elements::All32 => [
                0xb2, 0xc0, 0xd7, 0x7f, 0xeb, 0xc6, 0xf1, 0x73, 0xb1, 0x42, 0xc2, 0xb6, 0x2b, 0xb8,
                0xf5, 0xfd, 0xbd, 0x37, 0x9a, 0x1d, 0x7d, 0xdf, 0x89, 0x51, 0x33, 0x60, 0xa0, 0x2e,
                0xeb, 0x41, 0x94, 0x00,
            ],
            Elements::All64 => [
                0x1c, 0x9a, 0x3a, 0x89, 0x5a, 0x18, 0xe1, 0xe7, 0x81, 0x82, 0x13, 0x0d, 0xdf, 0xc8,
                0x97, 0x2a, 0x28, 0xae, 0x45, 0xe4, 0xb4, 0xea, 0xfd, 0x1b, 0xcf, 0xed, 0x5d, 0xd6,
                0x09, 0x06, 0x57, 0xff,
            ],
            Elements::All8 => [
                0xd7, 0xf5, 0x0e, 0xc5, 0x65, 0x03, 0xb3, 0x29, 0x96, 0x95, 0x9d, 0xe1, 0xad, 0x48,
                0x67, 0xb8, 0xb0, 0x59, 0xd9, 0x6a, 0xc8, 0xcc, 0x4e, 0x1d, 0x4f, 0x89, 0x60, 0x7f,
                0x06, 0x0e, 0xd1, 0x46,
            ],
            Elements::And16 => [
                0x92, 0x2f, 0x86, 0x4b, 0xa8, 0xdc, 0x2b, 0x6b, 0x9d, 0x31, 0x63, 0x4d, 0x31, 0x16,
                0xcb, 0x38, 0xd1, 0xae, 0x0d, 0x7c, 0x21, 0x50, 0xea, 0x1d, 0x4a, 0xec, 0x1f, 0xa3,
                0x93, 0xac, 0xf6, 0xf2,
            ],
            Elements::And32 => [
                0xe8, 0xad, 0x05, 0xb4, 0x6c, 0xf6, 0x2c, 0x0d, 0x48, 0x45, 0x42, 0xa6, 0xbb, 0x43,
                0xf9, 0xaa, 0xdf, 0x0e, 0x14, 0x8b, 0xd3, 0x45, 0xe6, 0x62, 0x51, 0x17, 0xab, 0x61,
                0xed, 0xea, 0x0b, 0xd5,
            ],
            Elements::And64 => [
                0x79, 0x71, 0xf1, 0x8c, 0x00, 0xcf, 0xf1, 0x66, 0xa0, 0xd2, 0x1b, 0x7a, 0x18, 0xf0,
                0xed, 0xc8, 0x0e, 0xc7, 0xd3, 0x92, 0x71, 0xe4, 0x13, 0xef, 0x08, 0x25, 0xe4, 0x97,
                0xb0, 0xca, 0x2d, 0x1c,
            ],
            Elements::And8 => [
                0x58, 0x33, 0x65, 0x36, 0x0b, 0xec, 0xef, 0x53, 0xc3, 0x19, 0x0d, 0xcd, 0xa5, 0x45,
                0x16, 0x75, 0xd7, 0x20, 0xdf, 0x45, 0xb0, 0x67, 0x43, 0x40, 0x8b, 0xcd, 0x66, 0x43,
                0xae, 0x35, 0x80, 0xdb,
            ],
            Elements::AnnexHash => [
                0xb6, 0x9c, 0x9e, 0x4c, 0xd5, 0x1f, 0x22, 0xc2, 0xa1, 0x91, 0x33, 0xef, 0x32, 0x95,
                0xb8, 0xe4, 0xe5, 0xd7, 0xdd, 0x39, 0x91, 0x10, 0x08, 0xaa, 0x6a, 0x5e, 0x6d, 0x8f,
                0x17, 0x22, 0xea, 0x1d,
            ],
            Elements::AssetAmountHash => [
                0xf7, 0x17, 0x99, 0xb8, 0x37, 0x37, 0x78, 0xe1, 0xd9, 0x1e, 0x16, 0x1a, 0xd4, 0xf4,
                0x35, 0x09, 0x06, 0x76, 0x05, 0xc2, 0xe7, 0x7a, 0x31, 0x61, 0xfc, 0xc3, 0x58, 0xee,
                0xfc, 0xef, 0x41, 0x57,
            ],
            Elements::Bip0340Verify => [
                0xea, 0x0f, 0x81, 0xfa, 0x0a, 0xc2, 0xe2, 0x03, 0x62, 0xcb, 0x09, 0x61, 0x2d, 0x19,
                0x9f, 0xcc, 0x45, 0xbe, 0x91, 0xb0, 0xd5, 0x8d, 0x87, 0x82, 0x25, 0x72, 0xe9, 0xec,
                0xe2, 0x4e, 0x10, 0xc1,
            ],
            Elements::BuildTapbranch => [
                0x33, 0x7f, 0xf4, 0x2d, 0xe0, 0x41, 0xb6, 0x02, 0xd7, 0x57, 0x64, 0xce, 0xa4, 0x82,
                0x19, 0xec, 0x2a, 0xaf, 0x7e, 0x8e, 0x5b, 0xc0, 0x35, 0x08, 0x6a, 0x00, 0x69, 0x01,
                0xf2, 0x79, 0x54, 0x03,
            ],
            Elements::BuildTapleafSimplicity => [
                0x3a, 0x62, 0xc7, 0x64, 0xf6, 0xda, 0x8a, 0xde, 0x19, 0x08, 0x6b, 0xfb, 0x75, 0x75,
                0xc2, 0x31, 0x47, 0x40, 0x07, 0xf0, 0x6a, 0xb4, 0xf4, 0x0a, 0x58, 0x93, 0x85, 0x44,
                0x92, 0x48, 0x34, 0xd2,
            ],
            Elements::CalculateAsset => [
                0x59, 0x1a, 0xb7, 0xa5, 0x1e, 0x14, 0x69, 0xd1, 0x3b, 0x55, 0x46, 0xe5, 0xcf, 0x3d,
                0x24, 0x55, 0xae, 0xa9, 0x51, 0x3a, 0x82, 0x2c, 0xdb, 0x12, 0xd2, 0xcd, 0x1c, 0x9d,
                0x33, 0x0f, 0x0f, 0x7b,
            ],
            Elements::CalculateConfidentialToken => [
                0xda, 0x2a, 0x88, 0x68, 0xfd, 0xa8, 0xfc, 0xd1, 0xb5, 0x30, 0x49, 0x20, 0xbd, 0xec,
                0xd6, 0x69, 0x08, 0x5f, 0x85, 0x39, 0x7d, 0x39, 0xbf, 0x1f, 0xff, 0x8a, 0xa5, 0x7d,
                0xf8, 0xa3, 0x1a, 0xb4,
            ],
            Elements::CalculateExplicitToken => [
                0x9b, 0x41, 0x98, 0x28, 0xae, 0xa6, 0xac, 0x84, 0x80, 0x2b, 0xe1, 0x28, 0x83, 0x49,
                0x1f, 0xe4, 0x9b, 0x07, 0x6c, 0x25, 0x81, 0x2b, 0xde, 0xd8, 0xba, 0x38, 0x74, 0xbf,
                0x95, 0x60, 0xaa, 0xe6,
            ],
            Elements::CalculateIssuanceEntropy => [
                0x1d, 0xf7, 0xec, 0x05, 0xe1, 0xa9, 0x8d, 0xd3, 0x7b, 0x6d, 0x8c, 0x48, 0x0a, 0x21,
                0xbc, 0xd0, 0x43, 0x19, 0xe7, 0x50, 0x53, 0x93, 0xb6, 0xf6, 0x45, 0x7b, 0x50, 0xc7,
                0x78, 0xf7, 0x05, 0xa3,
            ],
            Elements::Ch16 => [
                0xc6, 0xdd, 0x71, 0x70, 0xc5, 0xf6, 0x72, 0x68, 0x31, 0x7e, 0x19, 0x5e, 0x8b, 0x30,
                0xea, 0xf3, 0x4c, 0x23, 0xb0, 0x0c, 0x5a, 0xb9, 0xc3, 0x1e, 0xed, 0x33, 0xe5, 0x2d,
                0xbf, 0x00, 0xf5, 0x12,
            ],
            Elements::Ch32 => [
                0x51, 0xbb, 0x48, 0x45, 0x66, 0x4f, 0x60, 0x1d, 0xa4, 0x0d, 0xaf, 0x5e, 0x2a, 0x6f,
                0x0e, 0x1b, 0x4f, 0xed, 0x70, 0x5c, 0xb0, 0x00, 0x60, 0x59, 0x35, 0x8a, 0x06, 0xd6,
                0x74, 0x3e, 0xba, 0xb9,
            ],
            Elements::Ch64 => [
                0x16, 0x87, 0x09, 0x97, 0xe0, 0xfb, 0x1f, 0xbe, 0xfe, 0x81, 0xcc, 0x21, 0x6d, 0x72,
                0x4d, 0x6f, 0x44, 0xb3, 0x16, 0x3b, 0x33, 0x31, 0x78, 0xff, 0x36, 0x06, 0xc5, 0x67,
                0x24, 0x82, 0x96, 0x57,
            ],
            Elements::Ch8 => [
                0x1f, 0x6e, 0x22, 0x5b, 0x36, 0x9a, 0x75, 0x89, 0x0b, 0xd5, 0xd7, 0xa6, 0xca, 0x40,
                0xf4, 0x83, 0x47, 0xf1, 0xb1, 0x94, 0xf5, 0x14, 0x7a, 0x1b, 0x1b, 0x10, 0xf8, 0x60,
                0xf9, 0x35, 0x22, 0xa8,
            ],
            Elements::CheckLockDistance => [
                0xbe, 0x2d, 0xaf, 0x31, 0xa9, 0x27, 0xea, 0xb2, 0x5c, 0xd2, 0x96, 0x15, 0x31, 0xd5,
                0x7f, 0x5a, 0x75, 0xd6, 0x07, 0x82, 0xaa, 0x96, 0x56, 0x28, 0x23, 0x74, 0x66, 0x87,
                0x2a, 0xc9, 0xbe, 0x58,
            ],
            Elements::CheckLockDuration => [
                0x3a, 0xfa, 0x73, 0x01, 0xeb, 0x46, 0xa1, 0x30, 0x53, 0x41, 0x1a, 0x2a, 0xc4, 0xbe,
                0x68, 0x08, 0x5a, 0x45, 0xb3, 0xfb, 0x8d, 0x64, 0xa0, 0x75, 0x1a, 0xf8, 0x69, 0x80,
                0x1b, 0xe5, 0x7f, 0xf2,
            ],
            Elements::CheckLockHeight => [
                0x07, 0x69, 0xe0, 0x02, 0xa0, 0xc6, 0x78, 0xf8, 0xb3, 0x10, 0x63, 0x81, 0x0c, 0x0b,
                0x20, 0xbc, 0xe2, 0xa2, 0x4d, 0x63, 0xdd, 0x03, 0xc8, 0xf1, 0xa5, 0xa5, 0x99, 0x17,
                0xbc, 0x73, 0x56, 0xee,
            ],
            Elements::CheckLockTime => [
                0xbc, 0x20, 0x35, 0x52, 0x70, 0x92, 0x43, 0xf2, 0x6d, 0x0f, 0xba, 0xdc, 0xba, 0x27,
                0x9b, 0xf0, 0x2e, 0x4c, 0x42, 0x95, 0xb5, 0x5e, 0x47, 0x68, 0x49, 0x70, 0xcb, 0xff,
                0xbb, 0xd2, 0x59, 0x9e,
            ],
            Elements::CheckSigVerify => [
                0xe8, 0xfe, 0xb5, 0x34, 0x34, 0xe2, 0xbd, 0xb5, 0xcf, 0xe6, 0xa9, 0x1f, 0xa8, 0xf9,
                0xe1, 0x77, 0xf9, 0x41, 0xa6, 0x7c, 0xc6, 0xce, 0xd8, 0x69, 0x74, 0x6f, 0x1a, 0x8e,
                0xb6, 0x50, 0x6f, 0x76,
            ],
            Elements::Complement16 => [
                0x35, 0x3d, 0x1a, 0x7f, 0x4d, 0x54, 0x79, 0x55, 0x42, 0x29, 0xc7, 0x43, 0xed, 0xa6,
                0x5b, 0x6a, 0xe2, 0x01, 0x9a, 0x36, 0x51, 0x96, 0x9b, 0xd6, 0x52, 0xa1, 0x42, 0xc3,
                0x19, 0x04, 0xdc, 0xba,
            ],
            Elements::Complement32 => [
                0x29, 0xe5, 0x39, 0xb4, 0xf8, 0x60, 0x4e, 0x79, 0xe7, 0x5c, 0x17, 0x90, 0x88, 0xeb,
                0x73, 0x08, 0xeb, 0x10, 0x8c, 0xc6, 0xe3, 0x34, 0xc5, 0x09, 0x81, 0xc8, 0x8b, 0x70,
                0x4f, 0x1c, 0x7c, 0x5b,
            ],
            Elements::Complement64 => [
                0x16, 0x28, 0x87, 0x3a, 0x31, 0xd5, 0xaa, 0xcf, 0x55, 0x78, 0xba, 0x26, 0xc5, 0x8b,
                0xb7, 0x95, 0x3b, 0xf6, 0x2b, 0x87, 0x1a, 0x15, 0x98, 0x77, 0xb1, 0x48, 0x84, 0x0c,
                0x22, 0xe8, 0x59, 0x8d,
            ],
            Elements::Complement8 => [
                0x13, 0xd1, 0xc5, 0x24, 0xc9, 0xf6, 0xe8, 0x8f, 0xda, 0x60, 0xa8, 0x9f, 0x41, 0xb8,
                0x2d, 0xa7, 0xbd, 0x98, 0x8d, 0x98, 0xf9, 0x0f, 0x6b, 0xfa, 0x69, 0x74, 0x16, 0x7d,
                0xe7, 0x31, 0x4c, 0x30,
            ],
            Elements::CurrentAmount => [
                0x59, 0xaf, 0x46, 0xd9, 0xd0, 0xde, 0xab, 0x49, 0xc4, 0xff, 0x2b, 0xdb, 0xfc, 0xc5,
                0x6a, 0xb8, 0xb3, 0xce, 0xee, 0xd4, 0x22, 0x3f, 0x7c, 0x94, 0x80, 0x3f, 0xf1, 0xb3,
                0xd1, 0xf5, 0x72, 0x20,
            ],
            Elements::CurrentAnnexHash => [
                0xba, 0x5e, 0x70, 0x41, 0xff, 0x93, 0xcb, 0x3c, 0x00, 0xa5, 0x68, 0x40, 0x7c, 0xe9,
                0x65, 0x75, 0x7b, 0x6b, 0x52, 0x0c, 0x45, 0x24, 0x84, 0xd7, 0x77, 0x78, 0x49, 0x7b,
                0x57, 0xb0, 0x1c, 0x1a,
            ],
            Elements::CurrentAsset => [
                0xf0, 0xc3, 0xed, 0xdc, 0x28, 0xe2, 0x24, 0xc3, 0xc0, 0xa0, 0x78, 0x89, 0x0d, 0x28,
                0x16, 0x9e, 0xa9, 0xe1, 0x87, 0x91, 0xc1, 0xa6, 0xf7, 0xa8, 0xad, 0xe8, 0xe8, 0x11,
                0xc5, 0x0b, 0x7d, 0x89,
            ],
            Elements::CurrentIndex => [
                0x31, 0xff, 0xeb, 0xcb, 0x63, 0x3e, 0x5b, 0xe9, 0x2c, 0x3d, 0x4d, 0x7c, 0x28, 0x58,
                0x9d, 0x39, 0x0d, 0xa5, 0x13, 0xf9, 0x8d, 0xf2, 0x8b, 0xc4, 0xac, 0xc3, 0x1b, 0x2e,
                0x6d, 0xfa, 0xd5, 0x0e,
            ],
            Elements::CurrentIssuanceAssetAmount => [
                0xad, 0x56, 0x7b, 0xe3, 0x63, 0x83, 0x65, 0xe5, 0xcf, 0x93, 0xd8, 0xd8, 0x96, 0x57,
                0x15, 0x0f, 0xd8, 0x5c, 0xfe, 0x58, 0x78, 0x69, 0x91, 0x1b, 0x0d, 0xbf, 0x26, 0x4f,
                0xb2, 0xf9, 0x44, 0xa4,
            ],
            Elements::CurrentIssuanceAssetProof => [
                0xad, 0x2c, 0xdb, 0x8f, 0x66, 0xf9, 0xe2, 0x15, 0x8e, 0xe7, 0xb6, 0xd3, 0x76, 0x15,
                0xa0, 0x18, 0x1f, 0xaf, 0x9b, 0x5b, 0x96, 0xb6, 0xc4, 0xe7, 0x03, 0x1c, 0x77, 0x6d,
                0xf9, 0xb5, 0x40, 0x8f,
            ],
            Elements::CurrentIssuanceTokenAmount => [
                0x03, 0xf2, 0x11, 0xcf, 0x6f, 0x2a, 0x0f, 0xfb, 0xf0, 0x48, 0x35, 0x73, 0xee, 0xc6,
                0x04, 0xb4, 0x2e, 0x9d, 0x7b, 0xfb, 0x55, 0x29, 0xbc, 0xdf, 0x4a, 0x9b, 0x0b, 0x72,
                0x0d, 0xba, 0x31, 0xbe,
            ],
            Elements::CurrentIssuanceTokenProof => [
                0x5b, 0x27, 0x8a, 0x3f, 0x6a, 0x6e, 0xbe, 0xd0, 0xd5, 0x7a, 0xc9, 0x2c, 0xb8, 0x3f,
                0xca, 0x37, 0x12, 0xb6, 0x16, 0xb9, 0x99, 0xf9, 0x08, 0xa9, 0x83, 0xec, 0xce, 0x43,
                0x85, 0xdd, 0x6a, 0xad,
            ],
            Elements::CurrentNewIssuanceContract => [
                0xbe, 0xfb, 0x45, 0xe9, 0x49, 0x25, 0xb6, 0x36, 0x21, 0x7c, 0xc3, 0x94, 0x63, 0x0e,
                0x97, 0x92, 0x8f, 0x0c, 0xe6, 0xf9, 0x64, 0x54, 0x9a, 0x61, 0x3c, 0x19, 0x6f, 0x53,
                0x3c, 0x77, 0x8f, 0xa5,
            ],
            Elements::CurrentPegin => [
                0xfc, 0xbc, 0xcf, 0x45, 0x0b, 0x64, 0xd4, 0x29, 0x05, 0x29, 0x37, 0xd0, 0x66, 0x5d,
                0xd2, 0xe4, 0xd3, 0x54, 0xb8, 0xae, 0xf4, 0xfb, 0x0a, 0xce, 0xfd, 0xe8, 0x27, 0xb4,
                0x32, 0xd2, 0x70, 0x2b,
            ],
            Elements::CurrentPrevOutpoint => [
                0xcc, 0xaa, 0x96, 0x49, 0xd5, 0x82, 0xff, 0x08, 0x64, 0x9e, 0xf5, 0x15, 0x29, 0x11,
                0xae, 0x10, 0x73, 0xe7, 0x11, 0xc0, 0x22, 0x40, 0x2b, 0xf2, 0xde, 0xab, 0x57, 0x0a,
                0xb5, 0x0e, 0x4b, 0xdc,
            ],
            Elements::CurrentReissuanceBlinding => [
                0x74, 0x89, 0xa0, 0x9c, 0x2b, 0x06, 0x46, 0xb4, 0xbf, 0x39, 0x2c, 0x6f, 0x16, 0x46,
                0x65, 0x4c, 0x5f, 0xea, 0x6b, 0x67, 0xc6, 0x35, 0x2b, 0x60, 0xc0, 0x86, 0x4f, 0x45,
                0x8f, 0x51, 0x36, 0xba,
            ],
            Elements::CurrentReissuanceEntropy => [
                0xa4, 0x7b, 0x82, 0x46, 0x89, 0x51, 0x1e, 0x5e, 0x65, 0x54, 0x38, 0xf6, 0xab, 0x2c,
                0x07, 0x52, 0x14, 0x12, 0x8c, 0xd7, 0x7f, 0x05, 0x93, 0x5c, 0x13, 0xca, 0xd4, 0x01,
                0xb7, 0xd2, 0x96, 0x52,
            ],
            Elements::CurrentScriptHash => [
                0x0e, 0x15, 0x21, 0x32, 0x6a, 0xe0, 0x8a, 0x41, 0xaa, 0xc6, 0x65, 0x06, 0xd2, 0x3e,
                0x79, 0x78, 0xee, 0xa3, 0xcd, 0x99, 0x93, 0xc0, 0xf1, 0xdf, 0xf2, 0x4d, 0xad, 0x7c,
                0x01, 0x34, 0x70, 0x10,
            ],
            Elements::CurrentScriptSigHash => [
                0x44, 0xfb, 0xc7, 0x5c, 0xe3, 0x0a, 0x75, 0xa2, 0xd5, 0x43, 0x06, 0x0f, 0x73, 0xf6,
                0xec, 0x62, 0xe9, 0x75, 0x2f, 0x44, 0x31, 0x17, 0x38, 0xb1, 0xa9, 0xb2, 0xb8, 0x77,
                0xd2, 0xcd, 0x78, 0x6c,
            ],
            Elements::CurrentSequence => [
                0x5e, 0x5b, 0x4d, 0xa3, 0x3e, 0xc2, 0x56, 0xdc, 0xa6, 0x85, 0x78, 0x96, 0xcc, 0xaf,
                0x67, 0x1d, 0x58, 0xc0, 0xa8, 0x0a, 0xf0, 0xd1, 0xbf, 0x85, 0x52, 0x48, 0x9d, 0x45,
                0x46, 0xd8, 0x15, 0xc7,
            ],
            Elements::Decompress => [
                0x41, 0x9c, 0x87, 0xe5, 0xe9, 0x03, 0x4b, 0x5d, 0x45, 0x3f, 0xa6, 0x68, 0x19, 0xdb,
                0x5a, 0x36, 0xa4, 0xa5, 0x84, 0x35, 0x68, 0x83, 0x3e, 0x85, 0x70, 0xec, 0x77, 0x29,
                0xb1, 0x0a, 0x40, 0xa5,
            ],
            Elements::Decrement16 => [
                0xb4, 0x83, 0xb1, 0xe9, 0xd9, 0xef, 0x74, 0x10, 0x68, 0x33, 0xf6, 0x6f, 0x68, 0x0c,
                0x22, 0x6e, 0xf6, 0x98, 0x22, 0x3c, 0x26, 0xcf, 0x8d, 0xe5, 0x72, 0xed, 0x54, 0x0f,
                0x2c, 0x12, 0x6d, 0xaf,
            ],
            Elements::Decrement32 => [
                0x82, 0x66, 0xc0, 0x7d, 0x1c, 0xbf, 0x8f, 0x65, 0xa3, 0xeb, 0x8f, 0xd3, 0x2e, 0xf6,
                0x06, 0x01, 0xce, 0x28, 0x9f, 0xfb, 0x03, 0xae, 0x3e, 0x37, 0x5b, 0x9c, 0x76, 0x10,
                0x35, 0x7f, 0xe9, 0xdf,
            ],
            Elements::Decrement64 => [
                0x6e, 0xe6, 0xd1, 0xa4, 0x96, 0x5d, 0x37, 0x85, 0xbf, 0xf0, 0xa3, 0xe1, 0xe1, 0x69,
                0x0b, 0x16, 0xfd, 0xce, 0x2b, 0x32, 0x61, 0xc7, 0x77, 0x19, 0xaf, 0xb6, 0x9c, 0x8f,
                0xab, 0x4b, 0x9a, 0x5a,
            ],
            Elements::Decrement8 => [
                0xa7, 0x5f, 0xe6, 0x22, 0x50, 0x99, 0x3f, 0xb4, 0xd5, 0x37, 0xae, 0x86, 0xf3, 0x01,
                0x45, 0x19, 0x24, 0x66, 0xdf, 0x31, 0x27, 0xd5, 0xac, 0x2d, 0x42, 0x0a, 0x7b, 0x71,
                0x74, 0x98, 0xb5, 0x48,
            ],
            Elements::DivMod16 => [
                0x07, 0x1a, 0x9f, 0xc0, 0xa5, 0x34, 0xe7, 0x52, 0x49, 0x28, 0x74, 0xa9, 0xdc, 0x1e,
                0xc5, 0x31, 0xd0, 0x7d, 0x0c, 0x22, 0x09, 0x95, 0xce, 0xd1, 0x22, 0x7a, 0x1a, 0xc1,
                0xe3, 0xe7, 0xf1, 0xa7,
            ],
            Elements::DivMod32 => [
                0x93, 0x7f, 0x39, 0xaa, 0x8e, 0x26, 0xa7, 0x66, 0x52, 0xed, 0x51, 0xcd, 0xb7, 0xd4,
                0x89, 0xb9, 0x33, 0x27, 0x3f, 0x27, 0x5b, 0xde, 0x68, 0xf5, 0x4e, 0x59, 0xb4, 0x9b,
                0x26, 0x6b, 0x6f, 0x8c,
            ],
            Elements::DivMod64 => [
                0xa2, 0x68, 0x17, 0x27, 0x26, 0x40, 0x28, 0x56, 0x53, 0x71, 0x39, 0xcd, 0x0a, 0x23,
                0xfe, 0x34, 0x03, 0xb7, 0x65, 0xef, 0x74, 0xb5, 0x77, 0xdc, 0x61, 0xfd, 0xb2, 0x0d,
                0x4d, 0x7a, 0xc4, 0xdf,
            ],
            Elements::DivMod8 => [
                0xbf, 0x7c, 0xaf, 0x50, 0xce, 0xae, 0x7b, 0x12, 0xb3, 0x8d, 0x82, 0xc9, 0x3e, 0x46,
                0x01, 0x93, 0x69, 0x41, 0xf6, 0x05, 0x73, 0xdf, 0x98, 0xe7, 0x2a, 0x9c, 0x5a, 0x13,
                0x49, 0x07, 0x39, 0xd2,
            ],
            Elements::Divide16 => [
                0x50, 0x79, 0x73, 0x13, 0xf7, 0x6d, 0xd6, 0x55, 0x85, 0xef, 0x82, 0x8c, 0xe0, 0x1c,
                0xdc, 0x13, 0x2a, 0x48, 0x9c, 0xda, 0xff, 0x43, 0x49, 0x86, 0xdd, 0x89, 0x99, 0xdb,
                0xde, 0xaa, 0x61, 0x7b,
            ],
            Elements::Divide32 => [
                0x4d, 0xf2, 0xa1, 0x25, 0x5f, 0x07, 0x1f, 0x8a, 0x2b, 0xfc, 0x9a, 0x80, 0x5e, 0xdf,
                0x99, 0xd3, 0xc8, 0x90, 0xf5, 0xb1, 0xc4, 0xb8, 0x1a, 0x8f, 0xa6, 0xa9, 0x6c, 0x95,
                0xce, 0xa1, 0xa3, 0xfb,
            ],
            Elements::Divide64 => [
                0xba, 0x62, 0xd1, 0x87, 0xb9, 0x60, 0x24, 0x30, 0xd0, 0x00, 0x1d, 0x91, 0xd1, 0x22,
                0x3f, 0xed, 0x79, 0x28, 0xd3, 0x20, 0x95, 0xae, 0xfc, 0x57, 0x22, 0xe4, 0xb3, 0x0c,
                0xe6, 0xac, 0x1c, 0xf4,
            ],
            Elements::Divide8 => [
                0xcd, 0x0e, 0xb6, 0xf3, 0x2f, 0x77, 0xe2, 0xd9, 0x78, 0x08, 0x06, 0x4e, 0x96, 0x1d,
                0xab, 0x46, 0xee, 0x3e, 0x26, 0x47, 0xb2, 0x35, 0x7a, 0x7b, 0x58, 0x92, 0xd3, 0x6f,
                0xbc, 0x23, 0x2d, 0xc0,
            ],
            Elements::Divides16 => [
                0x0a, 0xd6, 0x0f, 0x11, 0xf0, 0x12, 0x3e, 0x30, 0x4a, 0x9b, 0x77, 0x72, 0x5c, 0xa6,
                0x5e, 0x70, 0xb7, 0x10, 0x6b, 0x4d, 0x05, 0x64, 0x4a, 0xc6, 0xd8, 0x45, 0x2a, 0x56,
                0xe7, 0xfb, 0x6f, 0x90,
            ],
            Elements::Divides32 => [
                0xd6, 0x5a, 0x47, 0x93, 0x65, 0xd8, 0xef, 0x47, 0xe4, 0x57, 0x01, 0x6c, 0xf3, 0xe4,
                0xc1, 0x27, 0x89, 0x10, 0x36, 0x68, 0x54, 0x92, 0xcb, 0x85, 0xf8, 0xcc, 0x2c, 0xc1,
                0x77, 0x17, 0x11, 0x13,
            ],
            Elements::Divides64 => [
                0xb7, 0x3d, 0xdb, 0xb9, 0x32, 0xc8, 0xb8, 0x60, 0xe8, 0x2e, 0x99, 0x2a, 0xcc, 0x72,
                0x59, 0xa7, 0x8c, 0x0f, 0xdd, 0x32, 0x00, 0x90, 0x36, 0xef, 0x65, 0xc4, 0x76, 0x08,
                0x36, 0x8b, 0x03, 0xc8,
            ],
            Elements::Divides8 => [
                0xe4, 0x24, 0x0f, 0x08, 0x24, 0xdf, 0x3a, 0xce, 0x65, 0x09, 0x36, 0x26, 0x00, 0x3a,
                0x6b, 0x97, 0x5d, 0x27, 0xfe, 0x89, 0xaf, 0x19, 0x99, 0x21, 0x4d, 0x43, 0x8b, 0x05,
                0xf1, 0xa1, 0x59, 0xed,
            ],
            Elements::Eq16 => [
                0x05, 0xdf, 0xef, 0xd7, 0x4a, 0x26, 0xab, 0x79, 0x89, 0xa8, 0x50, 0x93, 0x31, 0xa0,
                0x3f, 0x33, 0x40, 0x18, 0x21, 0xa0, 0x1a, 0x75, 0x04, 0xe8, 0x09, 0xb1, 0x14, 0x12,
                0x82, 0x26, 0xd6, 0xfd,
            ],
            Elements::Eq256 => [
                0xf5, 0x27, 0x49, 0xf1, 0xf2, 0xfc, 0xda, 0xaf, 0xa3, 0x62, 0x54, 0xf1, 0x63, 0xe6,
                0x70, 0x46, 0xe4, 0x88, 0x1e, 0xed, 0xca, 0xfd, 0x17, 0x01, 0x5e, 0x98, 0xe0, 0xcc,
                0x14, 0x41, 0xa5, 0x44,
            ],
            Elements::Eq32 => [
                0xd4, 0x9e, 0xc0, 0x1c, 0x58, 0xa1, 0xd9, 0x55, 0xca, 0xeb, 0x56, 0xf7, 0xd7, 0xb0,
                0xf3, 0x9f, 0x18, 0x19, 0xa9, 0xd2, 0xbd, 0xcd, 0x34, 0xcb, 0x10, 0x8b, 0x63, 0xe3,
                0x12, 0x2e, 0x04, 0xc6,
            ],
            Elements::Eq64 => [
                0xf1, 0x96, 0x89, 0xad, 0x94, 0x7e, 0x24, 0x55, 0x89, 0xa4, 0xe1, 0x87, 0x1a, 0x17,
                0x52, 0xc6, 0x66, 0x66, 0x6b, 0x43, 0x42, 0x79, 0x09, 0xfb, 0x3a, 0xcd, 0x95, 0xf0,
                0x8b, 0xc6, 0x93, 0x0a,
            ],
            Elements::Eq8 => [
                0x7b, 0x7e, 0xb8, 0x29, 0x73, 0x8d, 0x38, 0x3c, 0xc2, 0x5f, 0xbd, 0x59, 0xc7, 0xc6,
                0x13, 0xbb, 0x96, 0x55, 0x7d, 0x82, 0xf5, 0x91, 0x4f, 0x81, 0x42, 0x1d, 0xac, 0x18,
                0xe8, 0x01, 0x53, 0x38,
            ],
            Elements::FeAdd => [
                0x9f, 0x0f, 0xa0, 0xf8, 0x4d, 0x29, 0xd2, 0xc3, 0x6d, 0x98, 0xba, 0xaa, 0xdd, 0x1a,
                0xad, 0x0c, 0x9b, 0xd4, 0xe1, 0xae, 0x7d, 0xe9, 0x86, 0x92, 0xdd, 0x2a, 0x9c, 0x00,
                0xf4, 0x05, 0x03, 0xd4,
            ],
            Elements::FeInvert => [
                0x01, 0x48, 0xa6, 0x06, 0x5a, 0x45, 0xb7, 0xe9, 0x56, 0x4b, 0x29, 0x09, 0x14, 0x07,
                0x3a, 0xad, 0xbb, 0x89, 0x5d, 0xdb, 0xf6, 0xff, 0x93, 0xf4, 0x5e, 0x55, 0x2c, 0x54,
                0x9d, 0x2e, 0x99, 0x00,
            ],
            Elements::FeIsOdd => [
                0xd6, 0xb7, 0xd5, 0x44, 0xe0, 0x7d, 0xf3, 0x2a, 0x14, 0x0d, 0x43, 0x15, 0x4f, 0x49,
                0x03, 0xd7, 0x5c, 0x20, 0x1d, 0x80, 0x52, 0x64, 0xf2, 0xa4, 0xfb, 0x8c, 0x10, 0xe4,
                0x16, 0x49, 0x0c, 0x04,
            ],
            Elements::FeIsZero => [
                0xaa, 0x0d, 0x4d, 0xc7, 0xe1, 0xd9, 0xf0, 0x2a, 0x38, 0xd1, 0xd3, 0x65, 0x8e, 0x1a,
                0xf7, 0xfe, 0x9c, 0x29, 0xdf, 0xba, 0xd0, 0xa0, 0x94, 0x38, 0x1a, 0xe7, 0xe5, 0xcd,
                0x63, 0x4b, 0x81, 0xe3,
            ],
            Elements::FeMultiply => [
                0xd9, 0xc7, 0x4e, 0xb3, 0x9b, 0x8b, 0xad, 0x28, 0x21, 0xfd, 0xed, 0xfb, 0x77, 0x1f,
                0xbf, 0xf4, 0xc9, 0x05, 0x63, 0x1a, 0xbc, 0x50, 0xf2, 0x7c, 0xbf, 0x54, 0x78, 0x43,
                0xdd, 0x02, 0x56, 0x0d,
            ],
            Elements::FeMultiplyBeta => [
                0x2f, 0x46, 0xce, 0xda, 0xf4, 0xae, 0x91, 0xf5, 0x29, 0x8c, 0xeb, 0x33, 0x04, 0xca,
                0x9d, 0xe8, 0xcc, 0xc4, 0x16, 0xe6, 0x7b, 0xa6, 0xf6, 0xa6, 0x06, 0xa6, 0x55, 0xb9,
                0x27, 0x92, 0x9a, 0x0f,
            ],
            Elements::FeNegate => [
                0x6d, 0x39, 0xca, 0xe2, 0x6d, 0x28, 0x08, 0x7b, 0x4a, 0x97, 0xca, 0xe2, 0xd6, 0x47,
                0x90, 0xf4, 0xad, 0xe2, 0x58, 0x95, 0xc0, 0xa3, 0x7d, 0x31, 0x25, 0x83, 0x4a, 0x13,
                0xc7, 0x67, 0x0a, 0xce,
            ],
            Elements::FeNormalize => [
                0x38, 0xc6, 0xbe, 0x1d, 0xeb, 0xb3, 0xfa, 0xfd, 0xaa, 0x31, 0x8f, 0xba, 0xe4, 0xf3,
                0xab, 0xca, 0x3e, 0x5e, 0x11, 0x7b, 0x01, 0x75, 0x6a, 0x56, 0x7d, 0xf2, 0x94, 0x0d,
                0xe5, 0x63, 0x86, 0x6c,
            ],
            Elements::FeSquare => [
                0xfb, 0xc7, 0x62, 0x56, 0x79, 0xfe, 0xac, 0x0a, 0xaf, 0x53, 0xc2, 0x9b, 0xdd, 0x9e,
                0x8a, 0xc6, 0x1d, 0x0e, 0xf2, 0xb8, 0x0e, 0x49, 0x3f, 0x3a, 0x60, 0x23, 0x84, 0xf8,
                0x31, 0x8b, 0x68, 0xbd,
            ],
            Elements::FeSquareRoot => [
                0x86, 0xb3, 0xa1, 0x33, 0x34, 0x54, 0x1b, 0x43, 0x9e, 0xf2, 0x41, 0x15, 0x6f, 0x34,
                0xde, 0xf6, 0x6d, 0x32, 0xba, 0xaa, 0xf6, 0xc6, 0xca, 0x4f, 0x69, 0x21, 0x58, 0x0c,
                0x32, 0xe8, 0x49, 0x18,
            ],
            Elements::FullAdd16 => [
                0xb6, 0x4c, 0x43, 0x31, 0x5b, 0x94, 0x56, 0xed, 0x80, 0x70, 0xc1, 0xc8, 0x29, 0xb7,
                0x38, 0x2d, 0xc8, 0x5e, 0xc0, 0xde, 0x75, 0x4d, 0x4d, 0x9c, 0xde, 0xd3, 0xce, 0x69,
                0x8c, 0x9a, 0x10, 0x7b,
            ],
            Elements::FullAdd32 => [
                0x91, 0x85, 0x39, 0x4d, 0xb2, 0xbe, 0xb6, 0x4d, 0xd0, 0x8f, 0x83, 0x20, 0x52, 0xf7,
                0x6f, 0xb0, 0xe0, 0x86, 0xcc, 0x5e, 0xa4, 0xed, 0x7a, 0xc8, 0x5d, 0x22, 0x20, 0x0d,
                0xdb, 0x74, 0xab, 0x26,
            ],
            Elements::FullAdd64 => [
                0xd3, 0x8a, 0x1e, 0x79, 0x3d, 0x39, 0xc4, 0xda, 0xbb, 0x30, 0x54, 0x44, 0xb7, 0x07,
                0x8c, 0x09, 0x24, 0x73, 0x94, 0xa0, 0xcc, 0x56, 0x19, 0x27, 0x58, 0x53, 0x02, 0xb6,
                0x84, 0xa5, 0x43, 0xb5,
            ],
            Elements::FullAdd8 => [
                0x99, 0x3f, 0x63, 0x34, 0x4f, 0x9c, 0x4e, 0xd3, 0xaf, 0x29, 0x1d, 0x71, 0xf6, 0x1f,
                0xc4, 0x09, 0x47, 0x50, 0x68, 0x21, 0xef, 0xf7, 0x00, 0xe5, 0x21, 0xc1, 0xec, 0xbb,
                0x32, 0xdf, 0xcc, 0x45,
            ],
            Elements::FullDecrement16 => [
                0x46, 0xf2, 0x73, 0x40, 0x31, 0x03, 0x8e, 0xa6, 0xf0, 0xe1, 0xda, 0x7f, 0x9f, 0x04,
                0xcc, 0xa7, 0x77, 0xe6, 0xc6, 0x9f, 0xff, 0xc1, 0x10, 0x85, 0x15, 0x86, 0x8d, 0x51,
                0xed, 0xc8, 0xac, 0xcb,
            ],
            Elements::FullDecrement32 => [
                0x22, 0xab, 0xbf, 0x3d, 0x6a, 0xcf, 0x0b, 0x8c, 0xb7, 0xf5, 0x8f, 0x9c, 0x7e, 0x92,
                0x7a, 0xa1, 0xa1, 0xbb, 0x8f, 0xd0, 0xb9, 0x96, 0x23, 0xa1, 0x10, 0x04, 0xac, 0x53,
                0x65, 0x56, 0xab, 0x02,
            ],
            Elements::FullDecrement64 => [
                0xd5, 0x39, 0xaa, 0xe2, 0x25, 0x27, 0xd4, 0x61, 0x35, 0x46, 0x1e, 0x40, 0x59, 0x07,
                0x1e, 0x9d, 0x5b, 0x2f, 0xad, 0xfc, 0xa1, 0x3f, 0xb0, 0x2a, 0xb5, 0x72, 0x3e, 0x8a,
                0x91, 0x6a, 0x61, 0x5c,
            ],
            Elements::FullDecrement8 => [
                0x3f, 0x63, 0x6f, 0x40, 0x16, 0x9d, 0x07, 0x2f, 0x36, 0x82, 0x0b, 0x1f, 0x3f, 0x39,
                0xc1, 0x74, 0x86, 0x24, 0xd9, 0xd5, 0xfb, 0x95, 0x7a, 0x8d, 0x49, 0x44, 0xf4, 0xfd,
                0x0d, 0x48, 0x59, 0x7a,
            ],
            Elements::FullIncrement16 => [
                0x99, 0x64, 0x22, 0xaf, 0xc5, 0x2e, 0x28, 0xb1, 0x25, 0xaa, 0x70, 0xb2, 0xd0, 0x7c,
                0xa4, 0x61, 0x32, 0x2f, 0xab, 0x6a, 0x45, 0xbb, 0x76, 0x9d, 0xc1, 0x2d, 0xb2, 0xfa,
                0x9a, 0xf9, 0xbd, 0xbb,
            ],
            Elements::FullIncrement32 => [
                0x1a, 0x56, 0x55, 0xfa, 0xe2, 0xaf, 0x0a, 0xe6, 0xa6, 0xac, 0xb5, 0x87, 0xa8, 0x6c,
                0x72, 0xbb, 0x30, 0x88, 0xd7, 0x65, 0x6d, 0x25, 0x30, 0x95, 0x13, 0x7c, 0x9e, 0xd0,
                0x44, 0x89, 0x20, 0xbd,
            ],
            Elements::FullIncrement64 => [
                0x80, 0x7d, 0xc6, 0x66, 0xd3, 0xb1, 0xd3, 0xfd, 0x65, 0xa4, 0x03, 0x73, 0xf6, 0xdf,
                0xc6, 0xef, 0x64, 0x19, 0xfa, 0x93, 0x06, 0x5c, 0x1a, 0x90, 0x00, 0xde, 0xf7, 0x54,
                0x2c, 0x8e, 0xe8, 0xed,
            ],
            Elements::FullIncrement8 => [
                0x1f, 0xfb, 0x9a, 0x76, 0x2c, 0x99, 0xcb, 0x4d, 0x52, 0x8c, 0xba, 0x4d, 0xc6, 0x1f,
                0x13, 0x4f, 0xdb, 0x41, 0x07, 0xe1, 0xf4, 0x9b, 0x07, 0x6a, 0x57, 0x70, 0xf3, 0x91,
                0x95, 0x95, 0x46, 0x6c,
            ],
            Elements::FullMultiply16 => [
                0x8f, 0x46, 0xfa, 0xa9, 0xec, 0x05, 0xe3, 0xd3, 0xae, 0xee, 0x3e, 0xc4, 0x92, 0xec,
                0x53, 0x4c, 0xde, 0x6f, 0xc0, 0x33, 0x61, 0xe6, 0x19, 0x99, 0x4e, 0x8d, 0x04, 0x73,
                0xd6, 0xb0, 0x9e, 0x5d,
            ],
            Elements::FullMultiply32 => [
                0x6c, 0x60, 0x02, 0x1f, 0x3b, 0x6f, 0x33, 0xe0, 0x8c, 0xa4, 0x2e, 0x4c, 0x7e, 0x5a,
                0xcf, 0xf4, 0xcc, 0x7e, 0x0d, 0x69, 0x86, 0xdf, 0xb3, 0x55, 0xab, 0xe0, 0x56, 0x17,
                0x57, 0xf7, 0x25, 0xcb,
            ],
            Elements::FullMultiply64 => [
                0x05, 0xb2, 0x7c, 0xe5, 0x8a, 0xb0, 0x86, 0xdd, 0xb2, 0x5a, 0x1c, 0xf9, 0xe3, 0x09,
                0x1f, 0xdf, 0x23, 0x21, 0x4c, 0x54, 0x31, 0x50, 0x40, 0x32, 0x4d, 0x4f, 0x6f, 0x10,
                0x95, 0x3e, 0xd2, 0x45,
            ],
            Elements::FullMultiply8 => [
                0xcc, 0x22, 0xf3, 0x08, 0x18, 0xeb, 0x2d, 0x7b, 0xe5, 0x11, 0x42, 0xe6, 0xb2, 0xc4,
                0xeb, 0x64, 0xad, 0x7c, 0x83, 0xd5, 0xc1, 0x95, 0xcd, 0xdf, 0x58, 0x1b, 0xd9, 0x75,
                0x62, 0x38, 0x14, 0x6b,
            ],
            Elements::FullSubtract16 => [
                0x41, 0x6d, 0x74, 0x7b, 0xf0, 0x18, 0x85, 0xb0, 0x9e, 0xcb, 0x8f, 0x05, 0x7e, 0x6b,
                0x15, 0x40, 0xe5, 0x42, 0xfc, 0xde, 0x49, 0xa7, 0x74, 0x61, 0xc0, 0x5a, 0xc7, 0x30,
                0x84, 0x5b, 0xdf, 0x8e,
            ],
            Elements::FullSubtract32 => [
                0xb1, 0xbc, 0x9d, 0xf5, 0x70, 0x27, 0x52, 0xb4, 0xb3, 0x05, 0x12, 0xaa, 0xef, 0x54,
                0x9c, 0x6b, 0x0e, 0x82, 0x47, 0x71, 0x09, 0x0b, 0x94, 0xef, 0x96, 0x89, 0xbd, 0x4a,
                0xe3, 0xab, 0xa8, 0x81,
            ],
            Elements::FullSubtract64 => [
                0xc2, 0x44, 0xb5, 0x36, 0xd5, 0x3c, 0x79, 0xa2, 0xd4, 0x5c, 0xdf, 0x8b, 0x56, 0x1b,
                0xf5, 0x10, 0x22, 0xe5, 0xd5, 0x81, 0x98, 0x49, 0x50, 0x78, 0x47, 0x78, 0xde, 0x9e,
                0x6a, 0x90, 0x05, 0x2f,
            ],
            Elements::FullSubtract8 => [
                0x98, 0xa6, 0x0f, 0x3a, 0x31, 0x98, 0xca, 0x68, 0x60, 0x2a, 0xa0, 0xa5, 0x23, 0xad,
                0xc1, 0x77, 0xc3, 0x66, 0xe8, 0x03, 0x31, 0x9e, 0xee, 0x88, 0x16, 0x0e, 0xa3, 0xb0,
                0xef, 0xd7, 0x10, 0x4c,
            ],
            Elements::GeIsOnCurve => [
                0xf3, 0xcd, 0xa5, 0x4a, 0x99, 0xb7, 0x01, 0x1b, 0xd4, 0x88, 0xd3, 0x1e, 0xca, 0xfc,
                0xac, 0x34, 0x7a, 0x35, 0x7b, 0x4d, 0xeb, 0x79, 0x2b, 0x8c, 0xfa, 0x3d, 0x00, 0xc7,
                0xb1, 0x09, 0x7e, 0x4c,
            ],
            Elements::GeNegate => [
                0xa9, 0xfe, 0x3b, 0xe6, 0x70, 0xd1, 0x5c, 0x78, 0x99, 0x70, 0xcc, 0xf7, 0x6c, 0xa8,
                0x69, 0xf8, 0x0b, 0xbb, 0x77, 0x2b, 0x6e, 0xa0, 0xae, 0x85, 0x18, 0xd3, 0xa8, 0xd6,
                0xf8, 0x83, 0xa6, 0x42,
            ],
            Elements::GejAdd => [
                0x93, 0x35, 0x13, 0x59, 0x9e, 0x03, 0x6b, 0x4f, 0xb8, 0xfc, 0x94, 0xcc, 0x8c, 0xee,
                0x43, 0x7d, 0xa1, 0x0f, 0x22, 0xcb, 0xf8, 0x91, 0x0e, 0x48, 0xa5, 0x34, 0x7f, 0xaa,
                0x36, 0x3b, 0x40, 0x4c,
            ],
            Elements::GejDouble => [
                0x08, 0xa1, 0xce, 0x0a, 0x24, 0x79, 0xc6, 0xd1, 0x42, 0x45, 0x99, 0xfd, 0x6b, 0xa9,
                0x1e, 0xd3, 0xb5, 0xb9, 0x56, 0x23, 0x8b, 0xa1, 0x27, 0x6d, 0xf5, 0x1f, 0x06, 0x9b,
                0xb2, 0x47, 0xb1, 0x8d,
            ],
            Elements::GejGeAdd => [
                0xfd, 0x6a, 0x62, 0xe5, 0xee, 0x48, 0x6d, 0x81, 0x6d, 0x7d, 0x50, 0x78, 0x3a, 0xe6,
                0xc3, 0x99, 0x76, 0xbc, 0x7a, 0x3e, 0x19, 0x67, 0xe8, 0x4a, 0x3e, 0x3a, 0x1a, 0xdc,
                0x64, 0x5e, 0x45, 0xf8,
            ],
            Elements::GejGeAddEx => [
                0xfd, 0x09, 0xfb, 0xc0, 0xa6, 0xac, 0xa5, 0x78, 0x76, 0x61, 0xda, 0x11, 0xee, 0xe9,
                0x3d, 0x4f, 0xbb, 0xd7, 0x4d, 0xe1, 0x1c, 0xf5, 0x31, 0x6d, 0xb1, 0x46, 0x42, 0x3e,
                0xf8, 0x90, 0xf1, 0x18,
            ],
            Elements::GejInfinity => [
                0x73, 0x8f, 0xf0, 0x80, 0xff, 0x20, 0x81, 0x15, 0x4e, 0x96, 0xcb, 0x1b, 0xad, 0x2a,
                0x74, 0xf6, 0xe8, 0xea, 0xe3, 0x38, 0xcd, 0x56, 0x11, 0x9c, 0x5b, 0x15, 0x60, 0x43,
                0x90, 0x89, 0xf8, 0xf1,
            ],
            Elements::GejIsInfinity => [
                0x68, 0xe7, 0xf0, 0x52, 0xf0, 0xc5, 0xc9, 0xe2, 0x6d, 0xcc, 0x37, 0x27, 0xb8, 0x0b,
                0x00, 0xa4, 0x7a, 0xaa, 0xa3, 0x69, 0xb3, 0xa9, 0x6d, 0x18, 0x7e, 0xb8, 0x0a, 0x5c,
                0xaa, 0xc2, 0x9d, 0x98,
            ],
            Elements::GejIsOnCurve => [
                0x5c, 0x64, 0xd0, 0x6a, 0x8e, 0x63, 0x11, 0x33, 0xfe, 0x42, 0x5d, 0xd1, 0xac, 0x35,
                0x4e, 0x4e, 0x83, 0x44, 0xdc, 0x2a, 0x21, 0x94, 0x82, 0x75, 0x9e, 0xeb, 0x55, 0x7d,
                0x8f, 0x3e, 0x87, 0xb4,
            ],
            Elements::GejNegate => [
                0x67, 0x5e, 0xeb, 0xfe, 0x53, 0x7f, 0x27, 0x74, 0xea, 0xaf, 0xcb, 0x9f, 0x0a, 0xa4,
                0xe4, 0x87, 0x95, 0x76, 0x5c, 0x18, 0x6e, 0x25, 0x02, 0xb9, 0x7d, 0xc3, 0x0d, 0xbd,
                0x22, 0x5d, 0x14, 0xf5,
            ],
            Elements::GejNormalize => [
                0x7c, 0x89, 0x63, 0x42, 0x1e, 0xf4, 0x6d, 0x02, 0xe8, 0xd5, 0x76, 0xd6, 0xbd, 0x06,
                0x0b, 0xe8, 0x8a, 0xf5, 0xd3, 0xa3, 0xd8, 0xa7, 0xa9, 0x4c, 0x99, 0x75, 0x82, 0x97,
                0xd6, 0x06, 0x4b, 0xd8,
            ],
            Elements::GejRescale => [
                0x02, 0x58, 0xf6, 0xb5, 0x8e, 0x2d, 0x53, 0x60, 0x36, 0xe7, 0xae, 0x59, 0xc5, 0x45,
                0x09, 0x61, 0xb4, 0xe8, 0x2e, 0x09, 0x5f, 0x3b, 0xb7, 0x82, 0x7c, 0xfb, 0xba, 0x59,
                0x19, 0x72, 0x41, 0x44,
            ],
            Elements::GejXEquiv => [
                0xb7, 0x1a, 0x68, 0xc6, 0x1b, 0xea, 0x01, 0x41, 0x55, 0x4b, 0x9b, 0xec, 0x06, 0x97,
                0xe9, 0xce, 0x9e, 0x44, 0x7c, 0x2c, 0x94, 0xc2, 0xf9, 0x3f, 0xbf, 0x10, 0xf6, 0x6c,
                0x50, 0xf2, 0x25, 0x6e,
            ],
            Elements::GejYIsOdd => [
                0x82, 0xdf, 0x24, 0x26, 0xe2, 0xcc, 0xc6, 0xf5, 0xae, 0x2b, 0x4a, 0xc2, 0x25, 0xe2,
                0xa9, 0x4c, 0xf4, 0x9c, 0xbe, 0x62, 0xe6, 0x4d, 0xf4, 0xa9, 0x33, 0xc1, 0x16, 0xae,
                0xb9, 0x93, 0xe8, 0x3d,
            ],
            Elements::Generate => [
                0x90, 0xa3, 0x4b, 0xdd, 0x0b, 0xee, 0x48, 0xf8, 0x56, 0xd8, 0xcc, 0xf6, 0x98, 0x1a,
                0x72, 0x64, 0x8e, 0x3b, 0x51, 0x10, 0x9b, 0x3b, 0xb2, 0x55, 0x20, 0x78, 0x14, 0x10,
                0x57, 0x7b, 0xfc, 0x81,
            ],
            Elements::GenesisBlockHash => [
                0xb2, 0xa8, 0xfc, 0x83, 0x99, 0x8f, 0x35, 0x8a, 0x0e, 0x0f, 0x49, 0x3f, 0x4e, 0xe8,
                0x54, 0x22, 0xf6, 0x48, 0x70, 0xee, 0x8b, 0x20, 0x9b, 0x45, 0x81, 0xce, 0xe8, 0x5d,
                0xff, 0xab, 0x5e, 0x83,
            ],
            Elements::High16 => [
                0x1f, 0x52, 0x46, 0x4d, 0xfa, 0x38, 0x74, 0xf6, 0xda, 0x23, 0x3b, 0x46, 0x88, 0x79,
                0x93, 0x77, 0xd8, 0x47, 0x32, 0x65, 0x8c, 0xda, 0x1b, 0x43, 0x71, 0x49, 0x84, 0x82,
                0x59, 0xcb, 0xd5, 0x30,
            ],
            Elements::High32 => [
                0x60, 0xe0, 0xaa, 0x26, 0xc8, 0xd7, 0x30, 0x5c, 0x44, 0x14, 0xb4, 0x5d, 0xce, 0x3b,
                0x29, 0xf1, 0x07, 0x3e, 0xc1, 0x87, 0x62, 0x60, 0x82, 0xbe, 0x37, 0x72, 0x34, 0xcf,
                0xd5, 0x58, 0xc9, 0x72,
            ],
            Elements::High64 => [
                0xaf, 0x1a, 0xd2, 0xbd, 0x95, 0xca, 0x2f, 0x4e, 0x48, 0x62, 0xd4, 0xa3, 0x9b, 0x8a,
                0x4b, 0x79, 0x80, 0x55, 0x01, 0xbf, 0x8a, 0x6d, 0x40, 0x7e, 0x97, 0x2c, 0xf0, 0x06,
                0x61, 0xc3, 0xa3, 0x76,
            ],
            Elements::High8 => [
                0x3b, 0x30, 0xa5, 0x2e, 0x98, 0x3e, 0xb1, 0xb8, 0x61, 0x3f, 0x34, 0x80, 0x1b, 0x04,
                0x64, 0x49, 0xa5, 0xe1, 0xff, 0x32, 0x27, 0xb0, 0x96, 0xb9, 0x3c, 0xa4, 0xad, 0x9b,
                0x3f, 0x4b, 0xab, 0x82,
            ],
            Elements::Increment16 => [
                0x7f, 0xcd, 0x35, 0x7e, 0x97, 0x74, 0xb6, 0x02, 0x8c, 0x27, 0xfb, 0xd4, 0x48, 0x1c,
                0x28, 0x22, 0xb7, 0x45, 0xcf, 0x2c, 0xb2, 0xab, 0x8d, 0xcd, 0xc7, 0x07, 0x9f, 0x22,
                0x5c, 0xb1, 0x5d, 0x6d,
            ],
            Elements::Increment32 => [
                0x10, 0x26, 0x81, 0x25, 0xcb, 0x54, 0xb1, 0x73, 0x9e, 0x02, 0x9f, 0x20, 0x4d, 0x6e,
                0xf0, 0xf7, 0x21, 0x68, 0x72, 0xfd, 0x04, 0x5e, 0xef, 0x08, 0x96, 0x1a, 0x79, 0xde,
                0xf8, 0x89, 0xc6, 0x9c,
            ],
            Elements::Increment64 => [
                0x72, 0x8b, 0xec, 0x59, 0xca, 0x50, 0xc2, 0x33, 0xd5, 0xb3, 0xda, 0x1d, 0x81, 0x7c,
                0xf8, 0x3f, 0x11, 0x12, 0xe2, 0xba, 0xda, 0xed, 0x0f, 0xa8, 0xd4, 0xfc, 0xf1, 0x22,
                0xc0, 0x06, 0x29, 0x9c,
            ],
            Elements::Increment8 => [
                0xef, 0xc2, 0x4a, 0x38, 0xa6, 0xab, 0x37, 0x3c, 0xa7, 0xdf, 0x08, 0xf9, 0xe4, 0x3c,
                0xca, 0x03, 0xcf, 0xd0, 0x42, 0x82, 0x72, 0x37, 0xf5, 0x1e, 0x4c, 0x70, 0x7f, 0xb1,
                0x66, 0xf0, 0xad, 0x25,
            ],
            Elements::InputAmount => [
                0xa6, 0x77, 0x9c, 0x5a, 0x1d, 0xa8, 0x95, 0xfb, 0x09, 0xe3, 0xa7, 0x6a, 0x01, 0x6e,
                0xd0, 0x31, 0x4c, 0x22, 0x90, 0x53, 0x84, 0x52, 0xd6, 0x02, 0x7d, 0x94, 0x0f, 0x7b,
                0x51, 0x27, 0x98, 0x79,
            ],
            Elements::InputAmountsHash => [
                0xe7, 0xea, 0xfc, 0xf7, 0x9d, 0x53, 0xf5, 0x1c, 0x96, 0x27, 0x79, 0x3e, 0x71, 0x49,
                0x13, 0x6b, 0x02, 0x60, 0x21, 0x1c, 0x50, 0x91, 0xa6, 0x14, 0x94, 0x7d, 0x58, 0x8d,
                0xc5, 0xf9, 0xc9, 0xe6,
            ],
            Elements::InputAnnexHash => [
                0xa6, 0x9f, 0xf0, 0x18, 0x2d, 0x9e, 0x56, 0x5f, 0x35, 0xaa, 0x98, 0xd5, 0x18, 0xf2,
                0xb5, 0xb2, 0xab, 0x41, 0xb8, 0x2a, 0xe0, 0xcc, 0x61, 0x19, 0x4d, 0x09, 0x9f, 0x90,
                0xad, 0x96, 0xff, 0x7b,
            ],
            Elements::InputAnnexesHash => [
                0xcb, 0x25, 0x45, 0x6d, 0x25, 0xa1, 0x47, 0x85, 0x6a, 0x97, 0x1c, 0x48, 0x31, 0xfe,
                0x57, 0x02, 0x72, 0x78, 0xb9, 0xa6, 0x00, 0xfd, 0x13, 0xf8, 0xa4, 0x37, 0xb4, 0xc4,
                0x52, 0x23, 0x96, 0xee,
            ],
            Elements::InputAsset => [
                0x26, 0x6c, 0xb0, 0x1c, 0x76, 0x40, 0x09, 0x72, 0x20, 0x0c, 0x92, 0xba, 0xf8, 0x93,
                0x85, 0x33, 0xfc, 0x4d, 0x65, 0xda, 0x95, 0xc7, 0x44, 0x59, 0x7a, 0x93, 0xd9, 0xd3,
                0x99, 0x8e, 0x10, 0xe7,
            ],
            Elements::InputOutpointsHash => [
                0x4e, 0xee, 0xfa, 0xff, 0x11, 0x8c, 0x77, 0x74, 0xf7, 0xaf, 0xc5, 0xb4, 0x5f, 0x90,
                0xff, 0x8e, 0xb1, 0x6e, 0xeb, 0x63, 0x71, 0x3c, 0x22, 0x98, 0xd5, 0xc8, 0x79, 0x9f,
                0xb8, 0x53, 0x3b, 0x27,
            ],
            Elements::InputPegin => [
                0xb5, 0x07, 0x96, 0x96, 0xda, 0x2b, 0x5b, 0xe3, 0x72, 0x83, 0xbe, 0x5d, 0x33, 0x58,
                0x8f, 0x27, 0x4f, 0xc8, 0x36, 0xcf, 0xf7, 0xc2, 0xf9, 0x7f, 0x99, 0x33, 0x43, 0x86,
                0x46, 0x66, 0xc4, 0xfd,
            ],
            Elements::InputPrevOutpoint => [
                0x86, 0x40, 0x6b, 0x2c, 0xfd, 0xe4, 0xd9, 0x5e, 0x97, 0xff, 0x9d, 0x67, 0x50, 0x21,
                0x37, 0xb9, 0x89, 0x11, 0x76, 0x69, 0x60, 0x13, 0x49, 0xfd, 0xba, 0x5f, 0x04, 0xe6,
                0xf1, 0x49, 0x3a, 0x0d,
            ],
            Elements::InputScriptHash => [
                0x17, 0x20, 0xac, 0xab, 0x93, 0x83, 0x67, 0x88, 0xd0, 0x40, 0x53, 0xf2, 0x75, 0xfd,
                0x27, 0x37, 0xa4, 0xf5, 0xc7, 0xa7, 0x67, 0x83, 0x5d, 0x3c, 0x5b, 0xad, 0x52, 0x2b,
                0x4c, 0x05, 0x2e, 0x60,
            ],
            Elements::InputScriptSigHash => [
                0xc2, 0x6f, 0xa3, 0x7c, 0xef, 0x29, 0x5b, 0xf8, 0xa3, 0x62, 0xc4, 0xc8, 0x00, 0x7d,
                0xf6, 0x88, 0x9a, 0xb7, 0x89, 0x68, 0xba, 0x43, 0xa1, 0xb3, 0xf7, 0x80, 0x06, 0xce,
                0x44, 0xe4, 0x4c, 0xcb,
            ],
            Elements::InputScriptSigsHash => [
                0xf7, 0x92, 0x89, 0x2e, 0x01, 0x6c, 0xdf, 0x32, 0x49, 0xb9, 0xba, 0x66, 0x9a, 0xd2,
                0x3e, 0xb6, 0x8d, 0xb7, 0x0d, 0xdd, 0x21, 0xc4, 0x8d, 0xdf, 0x10, 0xf0, 0x63, 0x5f,
                0x2f, 0xde, 0xa0, 0x08,
            ],
            Elements::InputScriptsHash => [
                0x8d, 0x4a, 0xd3, 0xc0, 0x71, 0xf3, 0x0d, 0x6c, 0x9e, 0x71, 0x4d, 0x4a, 0x21, 0xf6,
                0x90, 0x01, 0xaa, 0x34, 0x94, 0xbd, 0x3f, 0x4e, 0x47, 0x5e, 0xf6, 0xcb, 0x88, 0x69,
                0x6c, 0x6d, 0xa1, 0x97,
            ],
            Elements::InputSequence => [
                0xf9, 0x71, 0xdb, 0xfe, 0xef, 0x44, 0x74, 0x50, 0x0a, 0x2d, 0x3a, 0x3d, 0x36, 0xd2,
                0xdb, 0xa8, 0x62, 0xae, 0x87, 0x2e, 0xb3, 0xb0, 0xe3, 0x56, 0xf7, 0xc1, 0x34, 0xe6,
                0xd2, 0x55, 0xf5, 0xc4,
            ],
            Elements::InputSequencesHash => [
                0x4a, 0x82, 0xb8, 0x7d, 0xc3, 0xec, 0xf1, 0x84, 0x82, 0xa5, 0x03, 0x8c, 0xef, 0x15,
                0x94, 0xa0, 0xd9, 0x5c, 0xae, 0x73, 0x22, 0xd1, 0xa7, 0x12, 0x44, 0x64, 0xab, 0x9c,
                0x3c, 0xb9, 0xad, 0x33,
            ],
            Elements::InputUtxosHash => [
                0x43, 0x8d, 0xa1, 0xec, 0x43, 0x78, 0xff, 0x37, 0x3d, 0xba, 0xda, 0xc7, 0xde, 0xc2,
                0x9a, 0x53, 0xa2, 0xc8, 0x91, 0x9f, 0x0a, 0x36, 0xac, 0xa1, 0x1e, 0x20, 0x13, 0xdb,
                0x27, 0x1c, 0xe8, 0xbd,
            ],
            Elements::InputsHash => [
                0x96, 0x84, 0x35, 0x8e, 0x5d, 0xd2, 0x45, 0xc6, 0xa8, 0x09, 0xef, 0x31, 0x2a, 0x99,
                0x69, 0x5d, 0x6e, 0xa2, 0x40, 0x36, 0x08, 0x78, 0x64, 0x8a, 0xc1, 0xf4, 0xa5, 0x72,
                0xa4, 0xfb, 0x5e, 0x93,
            ],
            Elements::InternalKey => [
                0xb3, 0x16, 0xdc, 0x70, 0xd9, 0x06, 0x02, 0x0e, 0x2d, 0x6b, 0x5b, 0xd6, 0x66, 0x69,
                0x4a, 0xb7, 0x39, 0x40, 0x54, 0x13, 0x27, 0x22, 0x37, 0xdb, 0x28, 0xda, 0xe1, 0x42,
                0x8b, 0xef, 0x65, 0xb8,
            ],
            Elements::IsOne16 => [
                0x7c, 0xb3, 0xd4, 0xed, 0xbf, 0x46, 0x40, 0xd2, 0xe9, 0xc7, 0x03, 0xc6, 0x8a, 0xcb,
                0xbf, 0xc4, 0x52, 0xef, 0xd6, 0x0e, 0xb1, 0xc2, 0xe0, 0x4f, 0x18, 0x7e, 0x7d, 0x70,
                0xed, 0x45, 0x7a, 0x39,
            ],
            Elements::IsOne32 => [
                0x91, 0xd7, 0x4e, 0x95, 0xef, 0x74, 0x4e, 0x01, 0xe5, 0x6d, 0xf6, 0xac, 0x36, 0xdf,
                0x18, 0x39, 0xab, 0x6c, 0x60, 0x8c, 0x7a, 0xb0, 0xaf, 0xc1, 0xfc, 0xc8, 0x7e, 0xb5,
                0x0a, 0x27, 0xd7, 0x99,
            ],
            Elements::IsOne64 => [
                0x7d, 0xdf, 0xc1, 0x3a, 0x05, 0x43, 0x39, 0x8e, 0x8f, 0x0e, 0x07, 0xe6, 0xca, 0x05,
                0x44, 0xb5, 0xff, 0x0a, 0x3e, 0xe8, 0x38, 0x25, 0xb2, 0x18, 0xb3, 0xb8, 0xb0, 0xe4,
                0x2e, 0x96, 0x52, 0xa9,
            ],
            Elements::IsOne8 => [
                0xbc, 0xd4, 0x16, 0x8e, 0x02, 0x83, 0x2d, 0xcc, 0xc9, 0xbf, 0xb5, 0x7f, 0x9a, 0x94,
                0xd2, 0x2c, 0xc0, 0x2d, 0xc7, 0x4b, 0xf4, 0x58, 0xc8, 0x17, 0x3e, 0xa3, 0x9e, 0x9e,
                0x16, 0xe6, 0x08, 0x34,
            ],
            Elements::IsZero16 => [
                0xb5, 0x16, 0x84, 0x8c, 0x3f, 0x28, 0x2c, 0xa7, 0xa8, 0x0b, 0x19, 0x50, 0x5e, 0x8f,
                0xd1, 0x38, 0x80, 0xc9, 0x93, 0xa9, 0x4d, 0xa5, 0x9b, 0xe7, 0x72, 0x32, 0x52, 0xbc,
                0x62, 0xa3, 0xc9, 0x24,
            ],
            Elements::IsZero32 => [
                0x69, 0x91, 0xf0, 0x26, 0xaf, 0x70, 0x7d, 0xd9, 0x54, 0x1d, 0xe6, 0x25, 0x67, 0x0f,
                0x83, 0x20, 0x80, 0x0d, 0x5b, 0xbd, 0x48, 0x04, 0x7d, 0x44, 0xc8, 0xee, 0x0d, 0xb1,
                0x33, 0x91, 0x19, 0x36,
            ],
            Elements::IsZero64 => [
                0xe2, 0xf8, 0x90, 0xc6, 0x7d, 0x35, 0xdc, 0xe2, 0x4f, 0x7b, 0x2c, 0xa6, 0x09, 0xaf,
                0x8b, 0xd2, 0x35, 0xbe, 0x6c, 0xd6, 0x99, 0x97, 0x0a, 0x43, 0xc6, 0x9b, 0xbb, 0xe2,
                0xf2, 0x1d, 0x15, 0x9c,
            ],
            Elements::IsZero8 => [
                0xa9, 0xc8, 0xc4, 0x77, 0xad, 0xd8, 0xa9, 0xf5, 0x58, 0x52, 0xc5, 0x1c, 0xe1, 0xfb,
                0xeb, 0x9a, 0x4c, 0x50, 0xc8, 0xf0, 0xb6, 0x3a, 0x55, 0xe8, 0x92, 0x40, 0x09, 0xa8,
                0xf5, 0x9d, 0x3c, 0x69,
            ],
            Elements::Issuance => [
                0xf8, 0x45, 0x15, 0xcf, 0x83, 0xee, 0xcb, 0xf0, 0xfe, 0xf6, 0xdc, 0xc1, 0xcd, 0x4e,
                0xb8, 0xe7, 0x93, 0x6e, 0x55, 0xb9, 0xbd, 0x55, 0x7f, 0x25, 0x63, 0x75, 0xae, 0xee,
                0xf5, 0xdb, 0x6a, 0x2d,
            ],
            Elements::IssuanceAsset => [
                0xa2, 0xc2, 0xff, 0xe6, 0x2b, 0x7f, 0x0d, 0x1e, 0xed, 0xa0, 0x47, 0xa5, 0x94, 0xe1,
                0x7e, 0xd2, 0x50, 0xa0, 0xf1, 0xce, 0x80, 0x72, 0xe6, 0x6c, 0x8c, 0x2c, 0x66, 0x6a,
                0xe1, 0xd1, 0x7d, 0x5b,
            ],
            Elements::IssuanceAssetAmount => [
                0x77, 0x93, 0x38, 0x19, 0xa4, 0xc0, 0x91, 0x04, 0x3f, 0xa4, 0xa2, 0xf7, 0x0f, 0x6a,
                0x85, 0x62, 0x7a, 0x64, 0x4b, 0xb2, 0x72, 0x81, 0xdd, 0x37, 0x68, 0x29, 0xac, 0xfc,
                0xc6, 0x0f, 0x8a, 0x68,
            ],
            Elements::IssuanceAssetAmountsHash => [
                0x76, 0xce, 0x74, 0x7b, 0x3b, 0x4f, 0xd4, 0x4e, 0x81, 0x51, 0x19, 0x6c, 0x89, 0xcd,
                0x38, 0x5d, 0x31, 0x02, 0x97, 0x5f, 0x01, 0x36, 0x1e, 0x77, 0xae, 0x40, 0xdc, 0x6e,
                0x6b, 0x6a, 0x0c, 0x13,
            ],
            Elements::IssuanceAssetProof => [
                0xa5, 0x31, 0x72, 0x39, 0x75, 0xb2, 0x08, 0x52, 0x00, 0x16, 0x9a, 0x79, 0xa3, 0x3a,
                0xb5, 0xe2, 0xae, 0x29, 0x4e, 0x8b, 0xf3, 0x51, 0xec, 0x17, 0x49, 0xf5, 0x30, 0x7d,
                0x06, 0xfc, 0x3c, 0xc8,
            ],
            Elements::IssuanceBlindingEntropyHash => [
                0x3b, 0x4b, 0x13, 0xf3, 0x2c, 0x82, 0xd2, 0xe6, 0x2f, 0x8c, 0x1d, 0x65, 0xad, 0xd8,
                0xfc, 0xa5, 0x74, 0xa5, 0x1b, 0x1e, 0xb7, 0xe1, 0x97, 0x65, 0x72, 0xe5, 0x7a, 0x46,
                0x51, 0xa6, 0x83, 0x7b,
            ],
            Elements::IssuanceEntropy => [
                0xa6, 0x16, 0xda, 0xef, 0x37, 0x38, 0x50, 0x50, 0x86, 0x6e, 0xaf, 0x5a, 0x01, 0x34,
                0x7a, 0x9b, 0xaa, 0x84, 0xf1, 0xa2, 0xf1, 0xa6, 0xe5, 0xc8, 0x99, 0x77, 0x0b, 0x00,
                0x0a, 0xfe, 0xce, 0xe3,
            ],
            Elements::IssuanceRangeProofsHash => [
                0x77, 0xba, 0x39, 0xd3, 0xfa, 0x89, 0x48, 0xab, 0x9b, 0xee, 0x09, 0x3d, 0x2b, 0xed,
                0xb7, 0x5b, 0x12, 0xd3, 0xfe, 0x7c, 0x91, 0x68, 0xdb, 0x4d, 0xf0, 0xf2, 0x09, 0x37,
                0x96, 0x94, 0xa9, 0x4f,
            ],
            Elements::IssuanceToken => [
                0x7a, 0x73, 0xd4, 0x42, 0xc6, 0x67, 0x1f, 0x7a, 0x04, 0x3d, 0xe2, 0x08, 0x88, 0x9c,
                0xfc, 0x13, 0x13, 0x05, 0x5f, 0x11, 0x98, 0x62, 0x55, 0x90, 0x84, 0x40, 0x23, 0xda,
                0xcf, 0x14, 0x8f, 0xa8,
            ],
            Elements::IssuanceTokenAmount => [
                0x5f, 0xc5, 0x56, 0x82, 0x70, 0x7a, 0xcd, 0x42, 0xff, 0x24, 0x66, 0xda, 0x71, 0x34,
                0x60, 0x82, 0xb0, 0x45, 0x61, 0x6f, 0x5f, 0xcc, 0x2b, 0x4b, 0x30, 0x57, 0x81, 0xae,
                0x22, 0xeb, 0x03, 0x4e,
            ],
            Elements::IssuanceTokenAmountsHash => [
                0xb0, 0x70, 0x4a, 0x69, 0xce, 0x0a, 0x35, 0x90, 0xc4, 0xa1, 0xcd, 0xc0, 0x87, 0x2c,
                0x53, 0xbc, 0x9e, 0xf1, 0x69, 0xe6, 0x1b, 0x3f, 0xa3, 0x7e, 0x59, 0x0d, 0xc8, 0xea,
                0x07, 0xd6, 0xdb, 0x02,
            ],
            Elements::IssuanceTokenProof => [
                0x3f, 0xf5, 0xe7, 0x2e, 0x0c, 0x53, 0x9e, 0xe0, 0x9d, 0xcb, 0x8d, 0xaf, 0xc0, 0x94,
                0x1c, 0x68, 0x6e, 0xd4, 0x44, 0x0c, 0x84, 0x73, 0xcf, 0x03, 0x0a, 0xd6, 0xab, 0x91,
                0xec, 0xc2, 0xc5, 0x30,
            ],
            Elements::IssuancesHash => [
                0x34, 0x98, 0x37, 0x5d, 0x90, 0x5b, 0x60, 0x7c, 0xd9, 0x0a, 0x9f, 0xa1, 0xb3, 0xdc,
                0x8a, 0x87, 0xda, 0x31, 0xc1, 0x03, 0x0a, 0x2a, 0x27, 0xee, 0xa9, 0x9b, 0xb0, 0x54,
                0x1e, 0xef, 0x4b, 0xbc,
            ],
            Elements::Le16 => [
                0xd0, 0x26, 0x9d, 0x47, 0xd2, 0x62, 0xcc, 0xb6, 0x7e, 0x39, 0xee, 0xb8, 0x8f, 0x7e,
                0xe0, 0xec, 0xd6, 0xf1, 0xf9, 0x64, 0x10, 0x25, 0x2d, 0x59, 0xce, 0x35, 0xcb, 0x87,
                0x1d, 0x09, 0xca, 0x64,
            ],
            Elements::Le32 => [
                0x2c, 0xea, 0x91, 0x87, 0x17, 0x3a, 0x50, 0x70, 0x4e, 0x93, 0xa6, 0x31, 0xd9, 0xcf,
                0xb2, 0x6a, 0xde, 0x50, 0xf6, 0xf1, 0x9d, 0xf4, 0xc3, 0x59, 0x15, 0xf4, 0x9d, 0x22,
                0xa3, 0xea, 0xd4, 0xd5,
            ],
            Elements::Le64 => [
                0x09, 0x2b, 0xb9, 0x2e, 0x9d, 0x8b, 0xe5, 0xf3, 0xcd, 0x88, 0x45, 0x9e, 0xca, 0x90,
                0x9e, 0x2a, 0x49, 0xaf, 0x7a, 0xc9, 0xf0, 0xdf, 0x11, 0x59, 0xc1, 0xac, 0x1a, 0xc9,
                0x1b, 0xc5, 0x1b, 0x0c,
            ],
            Elements::Le8 => [
                0x02, 0x7d, 0x93, 0xd3, 0xc2, 0x47, 0xc3, 0x90, 0xe4, 0x5f, 0xf3, 0x24, 0x08, 0xcc,
                0x49, 0xa5, 0xc1, 0x7d, 0x55, 0xb0, 0x49, 0x31, 0xda, 0xcc, 0x03, 0x0a, 0x3a, 0x50,
                0x4a, 0x0e, 0x3c, 0x75,
            ],
            Elements::LinearCombination1 => [
                0x69, 0x6c, 0x5f, 0x28, 0xf9, 0xe7, 0x59, 0xd9, 0x6c, 0x2a, 0x0f, 0x99, 0x03, 0x79,
                0xfc, 0xcf, 0x08, 0x42, 0x65, 0xf6, 0x33, 0x93, 0x26, 0x1d, 0x72, 0x49, 0xbb, 0xa3,
                0x1d, 0xfa, 0x24, 0x41,
            ],
            Elements::LinearVerify1 => [
                0x9d, 0x09, 0x52, 0xbf, 0x99, 0x97, 0xd8, 0x51, 0xa8, 0xcb, 0xbf, 0x8b, 0xdf, 0x2d,
                0x5e, 0xf4, 0x10, 0x86, 0x45, 0xcd, 0xdf, 0xf3, 0xf4, 0xf0, 0x0d, 0x38, 0x24, 0xdd,
                0x10, 0x72, 0x75, 0x2d,
            ],
            Elements::LockTime => [
                0x72, 0x7e, 0x4a, 0x3e, 0xf6, 0xf5, 0xbd, 0x8a, 0xb1, 0x5b, 0xa2, 0x70, 0xe4, 0x44,
                0xe5, 0x3e, 0xa5, 0xf1, 0xfb, 0x37, 0xbc, 0xf1, 0xb4, 0x76, 0x0e, 0xdf, 0xb3, 0xcb,
                0x82, 0xef, 0x31, 0x73,
            ],
            Elements::Low16 => [
                0xe3, 0x85, 0x5b, 0xb4, 0x64, 0x5a, 0x08, 0x25, 0xea, 0xae, 0x04, 0x12, 0x51, 0x5a,
                0x8b, 0x15, 0xa3, 0x7b, 0x30, 0x5d, 0xed, 0x6f, 0x11, 0x9a, 0x3f, 0x44, 0x25, 0x31,
                0x29, 0x7a, 0xb6, 0x9c,
            ],
            Elements::Low32 => [
                0x75, 0xd2, 0xfb, 0x6b, 0xe6, 0x3b, 0x8b, 0x2c, 0x35, 0x21, 0x12, 0x3b, 0x5b, 0xa7,
                0x7f, 0x78, 0x1b, 0xe9, 0xff, 0x1c, 0x1a, 0x75, 0xa8, 0xe2, 0xce, 0xef, 0x62, 0x06,
                0xfb, 0x73, 0x45, 0x89,
            ],
            Elements::Low64 => [
                0xea, 0x12, 0x49, 0xb3, 0xe2, 0x2e, 0x95, 0xf3, 0xc6, 0xff, 0x59, 0x1e, 0x87, 0x2c,
                0x03, 0xb6, 0xf1, 0xb4, 0xa1, 0xd8, 0xc0, 0x45, 0xc5, 0xa3, 0xba, 0xc7, 0xd9, 0x21,
                0x27, 0x28, 0x15, 0xb6,
            ],
            Elements::Low8 => [
                0x53, 0xe0, 0x0b, 0x91, 0x24, 0xd9, 0x60, 0x14, 0x7a, 0xb5, 0x72, 0x44, 0x5d, 0x37,
                0x3b, 0x42, 0x0a, 0x22, 0x21, 0x44, 0x81, 0xbd, 0xec, 0x5e, 0x6a, 0x3f, 0x05, 0xd5,
                0xab, 0x8e, 0x19, 0x84,
            ],
            Elements::Lt16 => [
                0x11, 0xb4, 0xc0, 0x33, 0x26, 0x30, 0xbc, 0xc6, 0xc6, 0x04, 0x4c, 0x3f, 0x88, 0x7b,
                0x2c, 0x50, 0x39, 0x74, 0x3f, 0x8a, 0xd9, 0x08, 0x97, 0x8e, 0x7b, 0xa3, 0x0a, 0x18,
                0xf0, 0x5f, 0xec, 0xaf,
            ],
            Elements::Lt32 => [
                0x04, 0x20, 0x22, 0x45, 0xd7, 0x59, 0x0d, 0x5d, 0x9c, 0x68, 0xb8, 0x4f, 0x26, 0x0f,
                0xd3, 0x6f, 0x0f, 0xce, 0xb6, 0x2a, 0xea, 0x20, 0x4d, 0xd7, 0xc5, 0x73, 0x6b, 0xf6,
                0xb9, 0x41, 0xb7, 0x65,
            ],
            Elements::Lt64 => [
                0xe8, 0x6f, 0x1e, 0xda, 0x2e, 0x39, 0xef, 0x08, 0x7e, 0x3f, 0x03, 0x16, 0x62, 0xb8,
                0xb1, 0xda, 0x8d, 0xf1, 0xa3, 0x84, 0x31, 0xe3, 0x1b, 0xdd, 0x56, 0x7f, 0x26, 0x7f,
                0xa0, 0x88, 0xe3, 0x38,
            ],
            Elements::Lt8 => [
                0xa0, 0x3e, 0x3c, 0xb8, 0x4b, 0xa0, 0xc6, 0xd5, 0xc9, 0x77, 0x93, 0x92, 0xe5, 0x8f,
                0x89, 0xa7, 0x60, 0x1c, 0xd5, 0x25, 0x49, 0xde, 0x6d, 0x75, 0xff, 0x71, 0x55, 0xb1,
                0xf1, 0x0a, 0x2e, 0x48,
            ],
            Elements::Maj16 => [
                0x29, 0x87, 0x64, 0xb7, 0xe8, 0xfb, 0x49, 0xb5, 0xd9, 0x55, 0x9b, 0x70, 0x90, 0x1f,
                0x1a, 0x17, 0x96, 0x56, 0x86, 0x4e, 0xef, 0xf3, 0xe2, 0x69, 0x21, 0x12, 0x6f, 0x70,
                0x30, 0xf1, 0x4a, 0x65,
            ],
            Elements::Maj32 => [
                0xb8, 0xc1, 0x7d, 0xea, 0x76, 0xe2, 0x3b, 0x5d, 0x71, 0x5c, 0xb0, 0x0f, 0x90, 0x11,
                0x39, 0x30, 0xfe, 0xb9, 0x1e, 0x86, 0xc8, 0x39, 0x41, 0x1c, 0x18, 0xe9, 0xac, 0x86,
                0xfc, 0x82, 0x78, 0xc7,
            ],
            Elements::Maj64 => [
                0x01, 0xd8, 0x5a, 0xb4, 0xbb, 0xc8, 0x9a, 0xc3, 0xa6, 0xf4, 0xce, 0x55, 0x64, 0xe1,
                0xa8, 0x42, 0xd7, 0x30, 0x31, 0x26, 0x8c, 0x9e, 0xb9, 0x8d, 0xf9, 0x3e, 0x12, 0x09,
                0x42, 0x0b, 0x4c, 0x0f,
            ],
            Elements::Maj8 => [
                0x83, 0xae, 0x95, 0xcb, 0x13, 0x96, 0xc2, 0xc8, 0x14, 0xb0, 0x19, 0xf6, 0x10, 0x37,
                0x1c, 0xb1, 0x8f, 0x14, 0xb0, 0xf0, 0x6e, 0xad, 0xc2, 0x85, 0xfa, 0xec, 0x3a, 0x4d,
                0x5b, 0x22, 0x9a, 0xe1,
            ],
            Elements::Max16 => [
                0xa1, 0x7d, 0x86, 0xd1, 0xdb, 0xb4, 0x2f, 0x02, 0xd6, 0xf7, 0x7d, 0x73, 0x51, 0x31,
                0xb7, 0xa5, 0x95, 0x86, 0x1d, 0x24, 0x95, 0x5e, 0x98, 0x58, 0x89, 0x4b, 0x04, 0x76,
                0xee, 0x74, 0x68, 0xe3,
            ],
            Elements::Max32 => [
                0x2d, 0x8d, 0xb7, 0xe8, 0xda, 0x9e, 0x71, 0x13, 0xf3, 0x38, 0xa3, 0x2f, 0x50, 0xe4,
                0xb0, 0xfe, 0x55, 0xab, 0x7e, 0xb3, 0xcf, 0x4a, 0x07, 0x74, 0xa8, 0x99, 0x4e, 0x81,
                0x84, 0xf9, 0x98, 0x39,
            ],
            Elements::Max64 => [
                0x87, 0xe7, 0xbb, 0x87, 0xb8, 0xd3, 0x99, 0x51, 0x6e, 0x23, 0x9f, 0x5f, 0x80, 0xeb,
                0xfe, 0x2e, 0x4e, 0x83, 0x90, 0x46, 0x97, 0x5f, 0xac, 0x45, 0x31, 0xcf, 0xaf, 0x84,
                0xb2, 0x02, 0xff, 0x89,
            ],
            Elements::Max8 => [
                0xfa, 0xe5, 0x3c, 0x8c, 0x42, 0x1b, 0xd1, 0x5c, 0x6e, 0x31, 0x1f, 0xd4, 0xb1, 0x07,
                0xf6, 0x4e, 0x7a, 0x53, 0xe2, 0xa3, 0xdc, 0x72, 0xab, 0x0c, 0xc1, 0x46, 0x59, 0xa4,
                0x64, 0x6a, 0x05, 0x7b,
            ],
            Elements::Median16 => [
                0x1c, 0x9e, 0xb3, 0x76, 0x95, 0xd4, 0x2f, 0x70, 0x71, 0x93, 0xd4, 0x87, 0xe2, 0x58,
                0xa1, 0x46, 0x84, 0xa3, 0x9b, 0x4d, 0x83, 0x5c, 0x6b, 0xd3, 0x4e, 0x33, 0xf7, 0x0c,
                0x00, 0x55, 0xc1, 0xc6,
            ],
            Elements::Median32 => [
                0x43, 0x66, 0x91, 0x97, 0x15, 0x50, 0x21, 0xbe, 0x47, 0xd6, 0x7b, 0x01, 0xae, 0x70,
                0xc1, 0x25, 0xa7, 0x4b, 0x27, 0x5c, 0x46, 0xf4, 0xf6, 0x0c, 0xd2, 0xe7, 0x54, 0xa5,
                0x69, 0x75, 0x58, 0xdc,
            ],
            Elements::Median64 => [
                0x18, 0x29, 0x1e, 0xa2, 0xe0, 0x88, 0x4f, 0x11, 0x29, 0xeb, 0x68, 0x1f, 0x68, 0x9b,
                0xd7, 0x07, 0x00, 0xdb, 0x9e, 0x3f, 0xa4, 0xec, 0x0d, 0x3f, 0xd1, 0x84, 0x60, 0x5a,
                0x96, 0x43, 0xed, 0xb2,
            ],
            Elements::Median8 => [
                0x01, 0xd8, 0x19, 0xb8, 0x33, 0xbf, 0x99, 0x73, 0x6b, 0x1a, 0xd1, 0xfa, 0xdb, 0x9e,
                0x8d, 0xf6, 0xfb, 0x94, 0xbd, 0xf6, 0x26, 0x7e, 0x16, 0x5b, 0xfb, 0x80, 0x85, 0xc4,
                0xd2, 0x71, 0x70, 0xe3,
            ],
            Elements::Min16 => [
                0x6f, 0x65, 0xef, 0x43, 0x92, 0xf4, 0x55, 0x4a, 0x1b, 0x90, 0xa6, 0x2e, 0xb4, 0xbb,
                0x35, 0x7e, 0x46, 0x8d, 0x5e, 0x85, 0xe7, 0x3b, 0x01, 0x83, 0xfb, 0x36, 0x6a, 0x75,
                0xaf, 0x0d, 0x53, 0x02,
            ],
            Elements::Min32 => [
                0x24, 0xd0, 0xd0, 0x83, 0x63, 0xe5, 0xcb, 0xda, 0xa9, 0xe0, 0xe0, 0xd2, 0x3f, 0x6d,
                0x43, 0x20, 0xa0, 0xc4, 0x91, 0x59, 0x36, 0xc5, 0x12, 0x6a, 0xc1, 0x70, 0x74, 0x5a,
                0xe0, 0x73, 0x6b, 0x3e,
            ],
            Elements::Min64 => [
                0xde, 0x6c, 0x8c, 0x3f, 0xaf, 0x26, 0x59, 0xfd, 0xf5, 0x2d, 0x44, 0xea, 0xe4, 0xd3,
                0x83, 0x54, 0xad, 0x09, 0x75, 0x58, 0xff, 0x8f, 0xac, 0xe2, 0xe4, 0xa5, 0x37, 0x17,
                0x38, 0xb4, 0x73, 0xfb,
            ],
            Elements::Min8 => [
                0xcb, 0x1e, 0x4f, 0x64, 0xd8, 0x34, 0x48, 0x59, 0x0d, 0xbc, 0x5a, 0xd2, 0x56, 0x88,
                0x46, 0x11, 0xb1, 0x15, 0x6b, 0x9a, 0xd2, 0x35, 0x44, 0x0f, 0x44, 0xe6, 0x60, 0x29,
                0x4e, 0xff, 0xfc, 0x2c,
            ],
            Elements::Modulo16 => [
                0xb6, 0x4f, 0xe4, 0xab, 0xf6, 0x84, 0xbc, 0x0c, 0xb4, 0x1c, 0x29, 0xce, 0x9d, 0xc2,
                0xc5, 0xd8, 0xef, 0xd5, 0x2c, 0x46, 0x3d, 0x08, 0xcb, 0x88, 0xcf, 0x6c, 0x84, 0x88,
                0x90, 0xe2, 0x80, 0xcd,
            ],
            Elements::Modulo32 => [
                0x34, 0xda, 0x46, 0x60, 0xae, 0xea, 0xfe, 0x01, 0x52, 0x9d, 0x8c, 0xe9, 0xe1, 0x52,
                0x84, 0x23, 0xcb, 0x87, 0x03, 0x24, 0x2e, 0x67, 0x2c, 0x61, 0x09, 0xfe, 0xc5, 0xc1,
                0x16, 0x79, 0x50, 0x6d,
            ],
            Elements::Modulo64 => [
                0xcb, 0x1c, 0x3c, 0x88, 0x45, 0x2b, 0x06, 0xaf, 0xa9, 0x7f, 0x37, 0xf9, 0x55, 0x3d,
                0xbd, 0x28, 0x9b, 0xda, 0x35, 0x3a, 0x8b, 0xd1, 0xdb, 0x52, 0x73, 0xe4, 0xa3, 0xcb,
                0xc1, 0x8f, 0x63, 0xcd,
            ],
            Elements::Modulo8 => [
                0xc9, 0x77, 0xb5, 0x79, 0xa7, 0x93, 0x13, 0xd2, 0x1a, 0x0a, 0x7c, 0xf5, 0x5d, 0xbf,
                0x77, 0xee, 0xa3, 0x24, 0x23, 0xb1, 0xeb, 0xf8, 0xdd, 0x48, 0x97, 0x0b, 0x4e, 0xb5,
                0x0a, 0xc2, 0x96, 0xde,
            ],
            Elements::Multiply16 => [
                0xc9, 0xe1, 0xcb, 0x83, 0xb9, 0x0e, 0xd4, 0xef, 0xea, 0x73, 0xcd, 0x85, 0xe7, 0x5f,
                0x9d, 0x04, 0x23, 0xfe, 0xd3, 0x12, 0x6f, 0x64, 0x71, 0xe4, 0x45, 0x75, 0xc7, 0xf9,
                0x6c, 0x49, 0x4c, 0xb1,
            ],
            Elements::Multiply32 => [
                0x2a, 0x82, 0xb6, 0x2f, 0x5f, 0xb9, 0x45, 0xc0, 0x4e, 0xb7, 0x8a, 0x48, 0x9c, 0x28,
                0xae, 0x2f, 0xa1, 0x37, 0x6c, 0x49, 0x64, 0xd9, 0xe2, 0x06, 0x26, 0x2d, 0x13, 0xb0,
                0xd5, 0x16, 0xc5, 0x4a,
            ],
            Elements::Multiply64 => [
                0x41, 0x36, 0xa3, 0x38, 0xa8, 0x2e, 0xdd, 0x39, 0xbf, 0x9f, 0x69, 0xac, 0xad, 0x06,
                0xa6, 0x29, 0xef, 0x5e, 0xea, 0xdc, 0xdc, 0xd0, 0xa5, 0x7f, 0xca, 0x2e, 0xf2, 0x45,
                0x96, 0x6d, 0x89, 0x2e,
            ],
            Elements::Multiply8 => [
                0x25, 0x82, 0x99, 0x3d, 0x80, 0xb9, 0x8e, 0xd1, 0xe0, 0x6d, 0x30, 0x13, 0xb9, 0x3c,
                0x41, 0x2c, 0xc2, 0x34, 0x3a, 0x79, 0xc9, 0xa7, 0x25, 0x38, 0x4c, 0xa1, 0x18, 0x11,
                0xd0, 0xb9, 0x08, 0x0b,
            ],
            Elements::Negate16 => [
                0xdf, 0xb1, 0x32, 0x60, 0x3d, 0x54, 0x1a, 0x98, 0x94, 0x8e, 0x16, 0x2f, 0xc9, 0x58,
                0xb0, 0x55, 0xb4, 0x9e, 0x60, 0x06, 0x85, 0x0b, 0xe6, 0x67, 0xad, 0x0c, 0x94, 0xbf,
                0xa4, 0x51, 0x21, 0x55,
            ],
            Elements::Negate32 => [
                0xd8, 0x1e, 0xb9, 0xd8, 0x8a, 0x19, 0xd9, 0x14, 0x03, 0x0e, 0x31, 0x86, 0x38, 0xef,
                0xd8, 0x53, 0x5b, 0xd9, 0x65, 0xe9, 0x51, 0x36, 0xfc, 0xa4, 0x98, 0xd2, 0x1d, 0x30,
                0x55, 0x30, 0x27, 0x08,
            ],
            Elements::Negate64 => [
                0x38, 0xd7, 0x4e, 0x41, 0xe4, 0x67, 0x04, 0x44, 0xb9, 0xe7, 0x86, 0x80, 0x2e, 0x17,
                0x43, 0x43, 0x88, 0x46, 0x22, 0x0a, 0x60, 0xac, 0xcd, 0xd9, 0xc6, 0x62, 0x25, 0xc4,
                0x7e, 0x47, 0x05, 0x53,
            ],
            Elements::Negate8 => [
                0xbc, 0xc2, 0x52, 0xb8, 0x13, 0xf2, 0x19, 0x7e, 0xbd, 0x25, 0xa9, 0xcb, 0x91, 0x3f,
                0x26, 0x97, 0xcd, 0xd0, 0x8a, 0x61, 0x68, 0x93, 0x6a, 0x0c, 0x65, 0xfc, 0x89, 0x7e,
                0x33, 0xbb, 0xfd, 0xfc,
            ],
            Elements::NewIssuanceContract => [
                0x87, 0x52, 0xac, 0x81, 0x51, 0x6d, 0xd0, 0x80, 0x9d, 0x9e, 0x6b, 0x71, 0xe3, 0x52,
                0xfc, 0x4b, 0xbe, 0x0a, 0x2c, 0x26, 0xc3, 0x67, 0x54, 0xdd, 0x48, 0xba, 0xcc, 0xc6,
                0x42, 0x41, 0x38, 0x26,
            ],
            Elements::NonceHash => [
                0x35, 0x88, 0xc8, 0xe3, 0xd0, 0xb3, 0x13, 0xbc, 0xef, 0x26, 0xc2, 0x31, 0xcc, 0xda,
                0x6a, 0xe1, 0x4c, 0xbd, 0xbc, 0x95, 0xd7, 0x27, 0x22, 0x45, 0x10, 0x58, 0x92, 0x00,
                0x06, 0x85, 0xf7, 0x22,
            ],
            Elements::NumInputs => [
                0x09, 0x02, 0x8c, 0x91, 0xe9, 0x45, 0xb6, 0x7c, 0x69, 0xac, 0xac, 0x89, 0xbb, 0x9d,
                0x66, 0x0d, 0x7f, 0xa2, 0x1a, 0xc7, 0x48, 0x17, 0x78, 0x63, 0x40, 0xdf, 0x08, 0x64,
                0x03, 0xd8, 0x5e, 0xbd,
            ],
            Elements::NumOutputs => [
                0x43, 0xd5, 0x59, 0x73, 0xe9, 0x64, 0xc0, 0x63, 0x0e, 0x75, 0x2e, 0x29, 0x15, 0x69,
                0x61, 0xd3, 0x02, 0x76, 0x61, 0x7d, 0x45, 0x36, 0x2b, 0x2a, 0x79, 0x50, 0xfb, 0x28,
                0x54, 0xea, 0x0d, 0x24,
            ],
            Elements::One16 => [
                0xf6, 0xa1, 0x4c, 0x50, 0x61, 0x90, 0xcc, 0x5d, 0xbd, 0xfb, 0x24, 0x92, 0x02, 0x7a,
                0x4d, 0xe3, 0x79, 0xdd, 0x2d, 0x57, 0x85, 0xe0, 0x51, 0x2d, 0x43, 0xfa, 0xeb, 0x1c,
                0xd2, 0x51, 0x28, 0x14,
            ],
            Elements::One32 => [
                0xc4, 0x2b, 0x70, 0xb5, 0x2a, 0xa3, 0x97, 0x8c, 0x28, 0x82, 0xb8, 0xc2, 0x32, 0x36,
                0x4b, 0xe8, 0xf4, 0x8e, 0xd7, 0x3d, 0x33, 0x08, 0x6b, 0xe7, 0x2c, 0x38, 0xf1, 0xc3,
                0x66, 0xf4, 0x35, 0x02,
            ],
            Elements::One64 => [
                0x40, 0xc3, 0x17, 0xbc, 0x23, 0xce, 0x5a, 0x48, 0x6a, 0xe3, 0x25, 0x05, 0xa3, 0x8a,
                0xd1, 0x7a, 0x84, 0x47, 0x9a, 0x10, 0x86, 0x71, 0x5b, 0x92, 0xac, 0x29, 0x60, 0xbd,
                0xfe, 0x0e, 0xdb, 0xc9,
            ],
            Elements::One8 => [
                0xc0, 0xd3, 0x51, 0xba, 0x41, 0x26, 0xb6, 0x0e, 0x88, 0x8d, 0xcc, 0x1e, 0x59, 0xf1,
                0x23, 0x98, 0xb4, 0xd8, 0x5e, 0xfd, 0x18, 0xfe, 0x3d, 0xa4, 0x74, 0xa2, 0x96, 0x6b,
                0x04, 0xa6, 0x13, 0x17,
            ],
            Elements::Or16 => [
                0x58, 0xd7, 0x09, 0xe8, 0xaa, 0xf4, 0x12, 0xff, 0x27, 0x99, 0x54, 0x7a, 0xc9, 0x14,
                0x4a, 0x79, 0x77, 0xce, 0x49, 0xcc, 0x8e, 0x95, 0x63, 0xd4, 0x5d, 0x68, 0x00, 0xad,
                0x5e, 0x4e, 0xff, 0x2e,
            ],
            Elements::Or32 => [
                0x8e, 0x03, 0x78, 0xd2, 0x41, 0xc1, 0x42, 0x37, 0xd2, 0x18, 0xa4, 0x8f, 0xfd, 0x04,
                0x51, 0x63, 0x4d, 0xe5, 0x63, 0xa4, 0xd4, 0xba, 0x7c, 0x7a, 0xc8, 0xc0, 0x42, 0x8d,
                0x33, 0xd1, 0x67, 0x8e,
            ],
            Elements::Or64 => [
                0xc0, 0x3a, 0xbc, 0x73, 0x13, 0xff, 0xbd, 0x09, 0xc8, 0x69, 0xb4, 0xa8, 0xfd, 0x61,
                0x0c, 0x94, 0x12, 0xd0, 0x5a, 0x94, 0x26, 0x09, 0xaf, 0xaf, 0xbc, 0x8b, 0xa6, 0x07,
                0xb4, 0x5d, 0xb8, 0xd8,
            ],
            Elements::Or8 => [
                0x0a, 0x28, 0xc7, 0xf0, 0xd0, 0x94, 0x42, 0x58, 0xb1, 0xa0, 0x49, 0x00, 0xf0, 0x33,
                0x82, 0x07, 0x31, 0xef, 0xea, 0xb6, 0x44, 0x10, 0x2c, 0x7a, 0xa5, 0x45, 0xaa, 0x32,
                0x76, 0xbd, 0x22, 0x73,
            ],
            Elements::OutpointHash => [
                0x61, 0x87, 0x79, 0x14, 0x0a, 0xa7, 0xc3, 0x2f, 0x8c, 0xe4, 0x0e, 0x11, 0x06, 0xec,
                0x27, 0xe8, 0x4e, 0xbf, 0x70, 0x5a, 0x7f, 0x1a, 0xea, 0x08, 0x72, 0x6a, 0x91, 0xe5,
                0x89, 0x43, 0xd7, 0x22,
            ],
            Elements::OutputAmount => [
                0x2a, 0x43, 0x8d, 0x50, 0x8b, 0xb9, 0x3c, 0xfc, 0x86, 0xcd, 0x13, 0xc7, 0x56, 0x76,
                0xf9, 0xdd, 0x9f, 0xd8, 0x42, 0x85, 0x45, 0xbe, 0xf2, 0xe7, 0xe8, 0x57, 0x8b, 0xc3,
                0x81, 0x26, 0x42, 0x98,
            ],
            Elements::OutputAmountsHash => [
                0xf3, 0x1a, 0xde, 0x56, 0xdc, 0x65, 0x4c, 0xd9, 0x62, 0x7e, 0x5f, 0xb8, 0xdf, 0xe7,
                0x79, 0xfa, 0x9e, 0x45, 0x0d, 0x69, 0xcc, 0x84, 0x91, 0x49, 0xee, 0xab, 0xe7, 0xad,
                0x1b, 0x24, 0x9d, 0x50,
            ],
            Elements::OutputAsset => [
                0x5a, 0x9f, 0x3c, 0xbc, 0xc3, 0x63, 0x93, 0x59, 0x3d, 0xb8, 0x0d, 0x0c, 0xa4, 0xbd,
                0xe4, 0x55, 0x5a, 0x7d, 0x7c, 0xeb, 0xbd, 0x0a, 0xbd, 0xcf, 0x73, 0xaf, 0x04, 0x26,
                0x4d, 0x76, 0x57, 0xd5,
            ],
            Elements::OutputNonce => [
                0xb3, 0x95, 0x4e, 0x56, 0xe7, 0xba, 0x56, 0xe7, 0xe4, 0xad, 0x21, 0x2a, 0x68, 0x74,
                0xb0, 0x5e, 0xd5, 0x3b, 0xbf, 0x05, 0x48, 0x7d, 0x60, 0xda, 0x3b, 0xfe, 0x3c, 0xbb,
                0x74, 0x7a, 0x9e, 0x15,
            ],
            Elements::OutputNoncesHash => [
                0x3d, 0x36, 0x6f, 0x30, 0xec, 0x75, 0x55, 0x68, 0xbc, 0x0e, 0x1d, 0x69, 0xa9, 0xf9,
                0xa8, 0xaf, 0x98, 0xed, 0x8a, 0xd5, 0x99, 0xaf, 0xb3, 0x49, 0x53, 0xf1, 0xa1, 0x3e,
                0x6e, 0xd0, 0x3e, 0x42,
            ],
            Elements::OutputNullDatum => [
                0x3d, 0x8c, 0x4c, 0x85, 0x25, 0xad, 0x23, 0x8f, 0xb6, 0x18, 0xa0, 0xd0, 0x4a, 0x6b,
                0xc3, 0x6e, 0xe1, 0x4a, 0x99, 0x08, 0x8c, 0xb5, 0x58, 0xdb, 0x2f, 0xd2, 0xac, 0x9e,
                0x56, 0x73, 0xb6, 0x7b,
            ],
            Elements::OutputRangeProof => [
                0x97, 0xe5, 0xdb, 0xf2, 0x5d, 0x00, 0x49, 0xe4, 0x23, 0x88, 0x96, 0x7d, 0x44, 0x59,
                0xb6, 0xbb, 0xb2, 0x12, 0x8d, 0x1b, 0x7f, 0xd2, 0x3c, 0x6e, 0xf7, 0xaa, 0x9f, 0x8d,
                0xfc, 0xac, 0xd5, 0x24,
            ],
            Elements::OutputRangeProofsHash => [
                0x1e, 0xa0, 0x53, 0xe5, 0xc8, 0xac, 0xee, 0xab, 0x93, 0xaa, 0x33, 0xf7, 0x75, 0x42,
                0xd2, 0xd6, 0x82, 0x9c, 0x38, 0x62, 0x9a, 0x4b, 0xe4, 0x64, 0xf1, 0x0c, 0x8a, 0x56,
                0x9f, 0xae, 0xb8, 0x4c,
            ],
            Elements::OutputScriptHash => [
                0xd7, 0x2b, 0x68, 0x5e, 0x74, 0xc7, 0x0d, 0x0e, 0x56, 0x2e, 0xc0, 0xf0, 0x4f, 0xf0,
                0x62, 0x98, 0x61, 0xa5, 0xa2, 0x12, 0xdf, 0xa8, 0x3c, 0xf8, 0x8b, 0x0f, 0xf7, 0x33,
                0x81, 0xe6, 0x9e, 0xa1,
            ],
            Elements::OutputScriptsHash => [
                0xb3, 0x99, 0xb2, 0xdc, 0x2a, 0xfa, 0x85, 0x60, 0xf8, 0x82, 0xd9, 0xd5, 0xc8, 0x34,
                0xb6, 0x85, 0xef, 0x95, 0xe5, 0x12, 0x0d, 0xeb, 0x8c, 0xb7, 0x94, 0x4b, 0x64, 0x8b,
                0x57, 0xe5, 0xaf, 0xf5,
            ],
            Elements::OutputSurjectionProof => [
                0x7c, 0x76, 0x34, 0x0a, 0x17, 0x48, 0xf4, 0x61, 0x89, 0xd0, 0xe9, 0x9d, 0x5e, 0x72,
                0x52, 0x93, 0x40, 0x94, 0x74, 0xbb, 0x45, 0x22, 0xab, 0xc2, 0xd9, 0x25, 0x5f, 0xa4,
                0x55, 0x48, 0xde, 0x6f,
            ],
            Elements::OutputSurjectionProofsHash => [
                0x6a, 0x8e, 0x53, 0xab, 0x36, 0x59, 0xfb, 0x36, 0x86, 0x44, 0x11, 0xbe, 0xcd, 0x0c,
                0xc6, 0x8c, 0xea, 0x87, 0xb6, 0x97, 0x80, 0x61, 0x9e, 0x9d, 0x33, 0xf3, 0x2c, 0x2b,
                0x4b, 0xfc, 0x9e, 0x82,
            ],
            Elements::OutputsHash => [
                0x6e, 0x59, 0xe2, 0x3e, 0x42, 0xb5, 0xc1, 0x19, 0xc2, 0x87, 0xd1, 0xc0, 0x5e, 0xa3,
                0x40, 0x9b, 0x8c, 0xc8, 0x63, 0x21, 0x45, 0x60, 0x29, 0xbb, 0x6e, 0x31, 0x58, 0x39,
                0x64, 0x1a, 0xaf, 0x50,
            ],
            Elements::ParseLock => [
                0x13, 0x83, 0xd3, 0x28, 0xa4, 0x0b, 0xc6, 0x34, 0x21, 0x74, 0xba, 0xaf, 0x9a, 0x39,
                0x09, 0x32, 0xc1, 0xfe, 0x4f, 0x2f, 0x4a, 0x8e, 0xc7, 0xfd, 0x21, 0x8b, 0xab, 0xf1,
                0x80, 0x5f, 0xb7, 0xe5,
            ],
            Elements::ParseSequence => [
                0xc9, 0x61, 0x77, 0x69, 0x10, 0x6d, 0x45, 0xa3, 0x70, 0xf2, 0x93, 0xaa, 0x22, 0x2e,
                0xe6, 0xb4, 0x15, 0x02, 0xe2, 0x74, 0xce, 0x41, 0xb9, 0x05, 0x9c, 0x9d, 0xde, 0x89,
                0x73, 0xcd, 0x80, 0xc7,
            ],
            Elements::PointVerify1 => [
                0x86, 0x94, 0x47, 0xdc, 0x82, 0xa9, 0xed, 0xf8, 0xb1, 0x44, 0xa3, 0xee, 0xba, 0xfd,
                0x7a, 0x5d, 0xcb, 0x70, 0xf4, 0x58, 0x3d, 0x95, 0xd7, 0x85, 0xdc, 0x01, 0xfd, 0xb3,
                0x61, 0x48, 0x1e, 0xd5,
            ],
            Elements::ReissuanceBlinding => [
                0x32, 0x80, 0xf4, 0xe8, 0xfa, 0x64, 0xaa, 0x3f, 0xf6, 0x16, 0xd9, 0x3b, 0x6d, 0xd6,
                0x68, 0xe9, 0xa9, 0x69, 0xc0, 0x71, 0xf9, 0xad, 0x39, 0x34, 0x73, 0x85, 0xaa, 0xda,
                0x3f, 0xe0, 0xff, 0xbe,
            ],
            Elements::ReissuanceEntropy => [
                0xf7, 0xf9, 0xa0, 0x85, 0xff, 0xd4, 0x4f, 0xd6, 0x19, 0x9e, 0x16, 0x5e, 0x31, 0x10,
                0x91, 0x0d, 0x41, 0x46, 0x07, 0x00, 0x22, 0xf0, 0x85, 0x04, 0x9e, 0xa0, 0x26, 0x51,
                0xb4, 0x17, 0xdc, 0x98,
            ],
            Elements::ScalarAdd => [
                0x47, 0xfa, 0xd9, 0x4f, 0xfa, 0x74, 0x1b, 0x30, 0x22, 0x34, 0x80, 0x5e, 0x5d, 0xa5,
                0x43, 0xce, 0xcc, 0x8e, 0x0b, 0xf8, 0x61, 0xc6, 0x07, 0xe3, 0xd2, 0x5a, 0x84, 0x2c,
                0xe6, 0x1e, 0x06, 0xda,
            ],
            Elements::ScalarInvert => [
                0x6d, 0x59, 0xed, 0xd6, 0xa8, 0x2f, 0x20, 0x85, 0xa7, 0x9e, 0x82, 0x2b, 0x1c, 0xd0,
                0x21, 0x43, 0x1d, 0x80, 0xe3, 0xf3, 0x63, 0x5b, 0x25, 0x5b, 0xc7, 0x9a, 0xbd, 0x47,
                0x21, 0xf8, 0x2f, 0xdb,
            ],
            Elements::ScalarIsZero => [
                0xb2, 0x18, 0x81, 0xe2, 0xbd, 0xfa, 0x6a, 0xf0, 0x05, 0xf1, 0xbd, 0x1e, 0x36, 0xe0,
                0x42, 0x11, 0x27, 0x48, 0x47, 0x14, 0x77, 0xe2, 0x08, 0x9f, 0xc1, 0xe3, 0x51, 0xc7,
                0x15, 0x02, 0xc5, 0xca,
            ],
            Elements::ScalarMultiply => [
                0xe6, 0xf6, 0xb6, 0x55, 0x01, 0xda, 0x00, 0xb2, 0x7b, 0x1f, 0x4e, 0xfe, 0x31, 0xc2,
                0x71, 0x2f, 0x31, 0xd1, 0xb1, 0x76, 0xa9, 0xcc, 0xdd, 0x0c, 0xf0, 0x0c, 0x58, 0x7a,
                0xf1, 0x2a, 0x05, 0xfd,
            ],
            Elements::ScalarMultiplyLambda => [
                0x4d, 0xdf, 0x8f, 0xcb, 0x58, 0xff, 0xae, 0x7b, 0x3a, 0xa6, 0xbc, 0x52, 0xff, 0xae,
                0x76, 0xad, 0xa3, 0x37, 0x04, 0x5b, 0xa0, 0x09, 0x80, 0xf5, 0x21, 0x58, 0x09, 0x9b,
                0x53, 0x9a, 0x94, 0x2e,
            ],
            Elements::ScalarNegate => [
                0x00, 0x7f, 0xba, 0x3d, 0x50, 0xde, 0xb6, 0x03, 0xdd, 0x23, 0x3e, 0x94, 0x63, 0x11,
                0x7d, 0x49, 0x36, 0x03, 0x3a, 0x50, 0xf6, 0x82, 0x21, 0x90, 0x0c, 0x93, 0x19, 0x53,
                0xf8, 0x2c, 0x16, 0x03,
            ],
            Elements::ScalarNormalize => [
                0xbb, 0xfa, 0x23, 0x5a, 0x95, 0x76, 0xd3, 0xd6, 0xeb, 0x82, 0xf3, 0xc9, 0x44, 0x27,
                0x69, 0x4f, 0xc5, 0xd9, 0x7a, 0x90, 0xd4, 0xc4, 0xee, 0x32, 0xcd, 0x80, 0xd0, 0xdb,
                0xad, 0x99, 0xd9, 0x3a,
            ],
            Elements::ScalarSquare => [
                0xa5, 0xcb, 0x22, 0x26, 0xa0, 0x9f, 0x26, 0xaa, 0x53, 0xf1, 0x54, 0x80, 0x34, 0xcd,
                0x58, 0xcc, 0x63, 0x1b, 0xea, 0x69, 0x72, 0x65, 0x8a, 0x9c, 0x55, 0x69, 0x8c, 0x39,
                0x7f, 0x0a, 0x84, 0x5b,
            ],
            Elements::Scale => [
                0xa7, 0x23, 0x53, 0x8c, 0x2d, 0x33, 0xf7, 0x67, 0x9a, 0xa9, 0x48, 0xdd, 0x96, 0x40,
                0x7c, 0x3d, 0xdd, 0xaf, 0x02, 0xac, 0x27, 0xff, 0x2e, 0xc2, 0x8d, 0x4c, 0xc2, 0x7e,
                0xea, 0xfe, 0x6b, 0xc3,
            ],
            Elements::ScriptCMR => [
                0xb3, 0x44, 0xcd, 0x4c, 0xbb, 0x59, 0x52, 0x86, 0xaf, 0xab, 0x2c, 0x19, 0xe2, 0xf0,
                0xba, 0x06, 0x8b, 0xbe, 0xaf, 0xe5, 0xa3, 0xb1, 0x4f, 0x0e, 0xe5, 0x7a, 0x5d, 0xde,
                0xda, 0x11, 0x1c, 0x45,
            ],
            Elements::Sha256Block => [
                0xd3, 0xe3, 0xc6, 0x9a, 0xed, 0x82, 0x2b, 0x24, 0xf9, 0x46, 0x14, 0xf2, 0xc2, 0xd7,
                0xcb, 0x39, 0x1d, 0x3a, 0x17, 0x17, 0xf1, 0x3d, 0xe9, 0xd5, 0x89, 0xcf, 0x0a, 0x14,
                0x62, 0x46, 0x68, 0xde,
            ],
            Elements::Sha256Ctx8Add1 => [
                0x4a, 0xaf, 0xee, 0x24, 0x56, 0xb5, 0x1e, 0x53, 0x62, 0x2c, 0x9f, 0x82, 0xc3, 0xe4,
                0x6c, 0x91, 0xb2, 0xd7, 0x9b, 0x65, 0x9c, 0x54, 0xf5, 0xfa, 0x67, 0x1b, 0x04, 0xd6,
                0x0a, 0x42, 0xc5, 0x6a,
            ],
            Elements::Sha256Ctx8Add128 => [
                0xae, 0x96, 0x07, 0x80, 0x6c, 0xc3, 0xdd, 0x3a, 0xe4, 0x54, 0x23, 0xbe, 0x24, 0x22,
                0x69, 0x6a, 0xf4, 0xe9, 0xad, 0xfc, 0xc5, 0xac, 0xa8, 0x8e, 0xa7, 0x96, 0xc6, 0xfb,
                0x64, 0xef, 0x24, 0x86,
            ],
            Elements::Sha256Ctx8Add16 => [
                0x35, 0x91, 0x2a, 0x51, 0xd9, 0xba, 0xdb, 0x3f, 0x88, 0x47, 0xed, 0xdc, 0x53, 0xc8,
                0xb2, 0xeb, 0xdb, 0x62, 0xdd, 0x3b, 0x49, 0x7f, 0xe0, 0xb4, 0x77, 0x26, 0xc0, 0xc7,
                0x02, 0x9e, 0xcd, 0x89,
            ],
            Elements::Sha256Ctx8Add2 => [
                0x86, 0xfa, 0xfa, 0x03, 0x8b, 0x9d, 0x86, 0xf7, 0xb5, 0x5e, 0x5a, 0xd7, 0x31, 0x8b,
                0x14, 0x0f, 0x56, 0x28, 0x91, 0xc9, 0x11, 0x4a, 0xd0, 0xca, 0xcb, 0x99, 0x53, 0xc6,
                0xa2, 0xed, 0xb9, 0x94,
            ],
            Elements::Sha256Ctx8Add256 => [
                0xe7, 0xc6, 0x52, 0x1b, 0x4e, 0x3a, 0xdb, 0xbb, 0x34, 0xdf, 0x28, 0x6a, 0x41, 0xdc,
                0xa1, 0x37, 0xf6, 0xa1, 0xc3, 0xee, 0x40, 0x6f, 0xd5, 0x96, 0x28, 0x71, 0x4d, 0xb1,
                0x07, 0x46, 0x62, 0xa5,
            ],
            Elements::Sha256Ctx8Add32 => [
                0x16, 0xfa, 0xdf, 0xc2, 0xfb, 0x9e, 0x73, 0x83, 0x85, 0xbf, 0x08, 0x19, 0x14, 0xaf,
                0xc9, 0x74, 0x30, 0xed, 0xce, 0x32, 0x0e, 0x12, 0x03, 0xac, 0xc2, 0x05, 0x49, 0xdb,
                0x89, 0xa1, 0xc2, 0x9e,
            ],
            Elements::Sha256Ctx8Add4 => [
                0xb7, 0xa3, 0x0a, 0xec, 0x7d, 0x6d, 0x78, 0xad, 0x83, 0x57, 0xa4, 0xc6, 0x2b, 0xe7,
                0xbf, 0x6d, 0xe0, 0x7e, 0x44, 0x01, 0x06, 0xb5, 0x3c, 0xcb, 0xf9, 0xef, 0x41, 0x02,
                0x2f, 0x03, 0xd7, 0xe2,
            ],
            Elements::Sha256Ctx8Add512 => [
                0xb3, 0x19, 0xe2, 0x84, 0x6a, 0xd9, 0xce, 0x46, 0xe8, 0xb4, 0x78, 0xc6, 0x47, 0x00,
                0x7b, 0xbd, 0x4b, 0x3a, 0x1a, 0x45, 0xbd, 0x2e, 0xe4, 0x79, 0x96, 0xfa, 0x0a, 0x2a,
                0xf4, 0x0e, 0xbd, 0x52,
            ],
            Elements::Sha256Ctx8Add64 => [
                0x75, 0x14, 0xd9, 0x08, 0x17, 0x86, 0x02, 0x4d, 0x71, 0x18, 0x99, 0x15, 0xb0, 0x05,
                0xd7, 0xc2, 0x21, 0x74, 0x8e, 0x48, 0x2f, 0xd8, 0x16, 0xb8, 0x81, 0x14, 0x5b, 0xe2,
                0x97, 0x97, 0xb4, 0xdd,
            ],
            Elements::Sha256Ctx8Add8 => [
                0xb0, 0xb7, 0x13, 0x1a, 0x41, 0x12, 0xc6, 0x53, 0x06, 0xe2, 0x2a, 0xd3, 0x54, 0x70,
                0x4e, 0x3e, 0xc2, 0xd7, 0x44, 0x43, 0x4f, 0x4a, 0x4d, 0x6a, 0xe8, 0xbc, 0xfd, 0x35,
                0x39, 0xc1, 0x0a, 0xdf,
            ],
            Elements::Sha256Ctx8AddBuffer511 => [
                0xc2, 0x88, 0xe0, 0xd1, 0xfd, 0xdd, 0xb9, 0xe0, 0xae, 0xf4, 0xfa, 0x05, 0x9c, 0xc8,
                0x83, 0x0e, 0x71, 0x25, 0xa6, 0x10, 0xb6, 0xa3, 0xa7, 0x1e, 0x8a, 0xa4, 0x96, 0x69,
                0x6e, 0xa7, 0x73, 0xfb,
            ],
            Elements::Sha256Ctx8Finalize => [
                0x59, 0x68, 0x67, 0x2b, 0x01, 0xc7, 0x2f, 0x8e, 0x84, 0xfa, 0xc9, 0xd1, 0x7e, 0x1e,
                0x9d, 0xf8, 0xf8, 0x78, 0x6b, 0x4b, 0x47, 0x04, 0x04, 0x4d, 0xd5, 0x7e, 0xaf, 0x77,
                0x36, 0x03, 0x88, 0x8f,
            ],
            Elements::Sha256Ctx8Init => [
                0x79, 0xa0, 0x29, 0x55, 0x61, 0xf6, 0x55, 0x4f, 0x04, 0xcb, 0x94, 0xc9, 0x89, 0x68,
                0xd1, 0xbd, 0x69, 0x66, 0xfc, 0x97, 0x85, 0x53, 0x83, 0x2b, 0x20, 0xf7, 0x5c, 0x2b,
                0x15, 0x56, 0x59, 0xba,
            ],
            Elements::Sha256Iv => [
                0x90, 0x25, 0x1f, 0x75, 0xab, 0xf9, 0x37, 0x8f, 0xfa, 0x81, 0x5e, 0x74, 0x2c, 0x89,
                0x4a, 0x38, 0xb9, 0x42, 0xf2, 0x8c, 0xff, 0xe9, 0xa4, 0x47, 0xca, 0xbb, 0xe6, 0xdd,
                0x2c, 0x2c, 0xd0, 0x26,
            ],
            Elements::SigAllHash => [
                0x36, 0x04, 0xf1, 0x7c, 0x73, 0x71, 0x86, 0x53, 0x12, 0x20, 0xda, 0x2c, 0x7c, 0x19,
                0x28, 0x1d, 0x25, 0x1a, 0x51, 0x0e, 0x73, 0x2d, 0xbc, 0x4a, 0xb5, 0xa5, 0x47, 0x87,
                0x7c, 0xb5, 0xff, 0x18,
            ],
            Elements::Some16 => [
                0xce, 0xcb, 0x74, 0xb8, 0x31, 0x35, 0xf5, 0x34, 0x0b, 0x6d, 0x88, 0xf3, 0xb7, 0x12,
                0xf1, 0x50, 0x4d, 0x2e, 0x50, 0x32, 0x86, 0x7f, 0xa7, 0xb9, 0xa2, 0xc4, 0xb4, 0x24,
                0xd9, 0x65, 0xc7, 0x8e,
            ],
            Elements::Some32 => [
                0x0c, 0xe7, 0xca, 0xb8, 0x0f, 0x07, 0xb1, 0x2d, 0x9c, 0x9b, 0x94, 0xc4, 0x71, 0xd9,
                0x58, 0xa1, 0xe7, 0x24, 0x14, 0xb9, 0x34, 0x8c, 0x1c, 0x53, 0xe0, 0x32, 0x4c, 0x2b,
                0x89, 0xde, 0x26, 0xf8,
            ],
            Elements::Some64 => [
                0x2f, 0xe1, 0xb4, 0x08, 0x34, 0xae, 0x9f, 0x5c, 0xcb, 0xcf, 0xa8, 0xe7, 0x91, 0x9e,
                0xb1, 0x1c, 0x71, 0x46, 0x88, 0x91, 0x71, 0x10, 0xae, 0xea, 0x8d, 0xb7, 0x78, 0xd8,
                0x5d, 0xa4, 0xe6, 0x43,
            ],
            Elements::Some8 => [
                0x33, 0xd9, 0x58, 0xa6, 0x8c, 0x24, 0xf6, 0x3b, 0xe7, 0xb0, 0x06, 0x99, 0x39, 0xcc,
                0xfc, 0x22, 0xb6, 0xa4, 0x34, 0x80, 0x52, 0x5f, 0x78, 0x56, 0x70, 0xed, 0xd1, 0x34,
                0xff, 0x24, 0xf6, 0xd0,
            ],
            Elements::Subtract16 => [
                0x7d, 0x22, 0xf4, 0x99, 0x7a, 0x66, 0xd1, 0x74, 0x31, 0xc2, 0x34, 0x81, 0xf4, 0x0a,
                0x44, 0xd3, 0xb4, 0x5f, 0x9d, 0x8a, 0xbf, 0x54, 0x96, 0x32, 0x54, 0x52, 0x77, 0x1a,
                0x87, 0xc7, 0x7c, 0x6b,
            ],
            Elements::Subtract32 => [
                0x5b, 0x3b, 0xe7, 0xfd, 0x8a, 0xb0, 0x14, 0x31, 0x49, 0xc1, 0xd9, 0xf9, 0xd8, 0x8a,
                0x9d, 0x99, 0xea, 0x70, 0xfd, 0xc0, 0x86, 0xbc, 0xf3, 0x88, 0xbf, 0x9f, 0x4d, 0xd9,
                0xe1, 0x2d, 0x17, 0x3c,
            ],
            Elements::Subtract64 => [
                0x17, 0x65, 0x3e, 0xdb, 0xdd, 0xea, 0x93, 0xa4, 0xc2, 0x0f, 0xf3, 0x59, 0xad, 0xc9,
                0x97, 0x5b, 0x7c, 0x2a, 0xbf, 0x8c, 0xf3, 0x2a, 0x1b, 0xc1, 0x47, 0xe2, 0xc9, 0x00,
                0x25, 0xbc, 0xfd, 0x19,
            ],
            Elements::Subtract8 => [
                0x77, 0x86, 0x49, 0xf8, 0x39, 0x35, 0xcd, 0x3f, 0xfc, 0x04, 0xf6, 0xc6, 0x62, 0x0a,
                0x33, 0x96, 0x5d, 0x0e, 0xf1, 0xdd, 0x17, 0xe7, 0xb5, 0x0f, 0xa4, 0x10, 0x76, 0xd8,
                0xad, 0x87, 0x6b, 0x1d,
            ],
            Elements::TapEnvHash => [
                0x16, 0x34, 0xe5, 0xc7, 0x4c, 0x21, 0x26, 0x24, 0x8d, 0x5d, 0x11, 0x7c, 0x53, 0xee,
                0x4f, 0xce, 0x83, 0xae, 0xa0, 0x76, 0x60, 0x21, 0x96, 0x9b, 0x43, 0x30, 0x59, 0x55,
                0x74, 0x9f, 0x09, 0x25,
            ],
            Elements::TapleafHash => [
                0x35, 0x39, 0xdb, 0x25, 0x09, 0xae, 0xca, 0x7c, 0xca, 0xb8, 0xbd, 0x45, 0x24, 0x60,
                0x50, 0x98, 0x1a, 0xa7, 0x71, 0xb4, 0x94, 0x84, 0x0b, 0xcf, 0x50, 0xad, 0x44, 0x65,
                0xda, 0x81, 0x90, 0x5a,
            ],
            Elements::TapleafVersion => [
                0x7e, 0x08, 0xfd, 0x84, 0x3c, 0xfe, 0x16, 0x56, 0x8e, 0x17, 0x6a, 0xd4, 0xbe, 0x40,
                0x9a, 0xd4, 0x7b, 0x0d, 0x4d, 0x6b, 0x56, 0xf6, 0xa9, 0xe4, 0x85, 0x2a, 0xcd, 0x87,
                0x2c, 0x72, 0x42, 0x06,
            ],
            Elements::Tappath => [
                0xe2, 0x53, 0x91, 0x19, 0x0b, 0xf7, 0xa8, 0x68, 0xb2, 0xeb, 0x62, 0x1b, 0xc3, 0x94,
                0xf6, 0xd4, 0x8a, 0x05, 0xfb, 0xe3, 0x15, 0x75, 0x29, 0xcd, 0x3c, 0xaf, 0xd5, 0x17,
                0x1c, 0x77, 0xe2, 0x13,
            ],
            Elements::TappathHash => [
                0x2e, 0x75, 0x09, 0x95, 0xe6, 0x78, 0x06, 0x61, 0x32, 0x1f, 0x29, 0xf4, 0x77, 0x37,
                0xf3, 0x1b, 0x9f, 0xe4, 0x3d, 0x5c, 0xd4, 0xf2, 0x4a, 0x41, 0x4a, 0x55, 0x37, 0xf7,
                0x37, 0xf6, 0xfc, 0x1d,
            ],
            Elements::TxHash => [
                0x7c, 0x3d, 0x67, 0x76, 0xd2, 0xf6, 0x1f, 0x88, 0x91, 0x0f, 0x0e, 0x3e, 0xef, 0x8a,
                0xed, 0x15, 0x2d, 0xdd, 0xa1, 0x27, 0x83, 0x51, 0xba, 0x82, 0xcc, 0xa5, 0x89, 0xfd,
                0x91, 0xfb, 0x49, 0x93,
            ],
            Elements::TxIsFinal => [
                0x39, 0x1d, 0x9e, 0xa2, 0xc9, 0xe5, 0xf0, 0x3e, 0xd7, 0xad, 0x7b, 0x62, 0x37, 0x2a,
                0x16, 0x57, 0x4c, 0x0d, 0x33, 0x6c, 0x9b, 0x65, 0xb3, 0x04, 0xb8, 0x35, 0x2f, 0xe1,
                0x3a, 0x01, 0x12, 0x47,
            ],
            Elements::TxLockDistance => [
                0x3b, 0x71, 0x08, 0x77, 0xf3, 0x0d, 0x97, 0xc9, 0x3b, 0x9f, 0x3c, 0x72, 0x82, 0xc6,
                0x57, 0x88, 0x94, 0xc2, 0xa2, 0xd4, 0x60, 0x84, 0x87, 0x58, 0x8c, 0x74, 0x2a, 0x9c,
                0x44, 0x76, 0x80, 0x1e,
            ],
            Elements::TxLockDuration => [
                0x63, 0x07, 0xdc, 0x72, 0x0d, 0x7f, 0x13, 0x61, 0xcc, 0x5e, 0x11, 0xdc, 0x69, 0x9a,
                0x66, 0x5a, 0x18, 0x4c, 0xac, 0xce, 0x83, 0x4e, 0xf1, 0x4a, 0xb0, 0xb5, 0xd0, 0x4e,
                0x58, 0xeb, 0x0a, 0xcc,
            ],
            Elements::TxLockHeight => [
                0x4a, 0xfc, 0x96, 0x0e, 0xaa, 0x9d, 0x32, 0x02, 0xec, 0xed, 0xbd, 0xce, 0xcc, 0x83,
                0x1b, 0x69, 0x64, 0xc7, 0x58, 0x97, 0x62, 0x5d, 0x7d, 0x5b, 0x34, 0x33, 0xc0, 0x0b,
                0xb7, 0x96, 0x0c, 0x9b,
            ],
            Elements::TxLockTime => [
                0x95, 0xcf, 0x84, 0xf8, 0x8b, 0xb5, 0x5e, 0x9a, 0xd1, 0xd7, 0xbb, 0x53, 0xc0, 0x89,
                0x01, 0x45, 0x43, 0x2e, 0x9f, 0x45, 0x24, 0x77, 0x65, 0x35, 0xfb, 0xde, 0x94, 0x74,
                0x68, 0x0d, 0xf3, 0x19,
            ],
            Elements::Verify => [
                0xa2, 0x06, 0x23, 0xb6, 0x86, 0xa6, 0xbd, 0xb6, 0x18, 0xb5, 0x0b, 0x4c, 0x5e, 0x8e,
                0xee, 0x06, 0x01, 0xcc, 0x84, 0x01, 0x34, 0xcb, 0x9e, 0x54, 0x80, 0xf7, 0xb6, 0xde,
                0x89, 0x49, 0x48, 0x9e,
            ],
            Elements::Version => [
                0xae, 0xb6, 0x61, 0x97, 0x9a, 0xfa, 0x87, 0x77, 0xf6, 0x0b, 0x14, 0x1e, 0x17, 0x05,
                0xd5, 0xa9, 0xc9, 0x8f, 0x9d, 0xcc, 0x9b, 0xc0, 0x95, 0x05, 0x4c, 0xee, 0xc0, 0x62,
                0xc9, 0xdd, 0xcf, 0xb3,
            ],
            Elements::Xor16 => [
                0xed, 0x59, 0x8a, 0xe3, 0x6d, 0x75, 0x61, 0xe7, 0xaa, 0x6e, 0xdc, 0x8c, 0x88, 0xa2,
                0xc8, 0x86, 0x66, 0xd6, 0x3f, 0xaf, 0x1d, 0xb3, 0x22, 0xde, 0x2a, 0xc6, 0x32, 0x47,
                0x0d, 0x41, 0xf1, 0xfa,
            ],
            Elements::Xor32 => [
                0x0f, 0x1e, 0xa1, 0xed, 0x1f, 0xf2, 0x28, 0x93, 0x8b, 0x66, 0x38, 0x55, 0xd1, 0x50,
                0xc2, 0x8e, 0x7d, 0xfa, 0x0c, 0x17, 0x84, 0x16, 0x18, 0x34, 0x30, 0x68, 0x3f, 0xca,
                0x1e, 0x99, 0xb5, 0xb4,
            ],
            Elements::Xor64 => [
                0x7d, 0x97, 0x32, 0x8d, 0xb2, 0x12, 0xf1, 0xaa, 0x00, 0xd6, 0x15, 0x62, 0x07, 0x5e,
                0x02, 0xc0, 0xfe, 0x49, 0x36, 0x3a, 0x32, 0x0b, 0xf2, 0x07, 0x4b, 0x7b, 0xc3, 0x00,
                0xe2, 0x3a, 0xb2, 0xed,
            ],
            Elements::Xor8 => [
                0x87, 0x6c, 0xd5, 0x80, 0x98, 0x6b, 0xc2, 0x08, 0x5a, 0xc4, 0xf4, 0xeb, 0x5e, 0x02,
                0x1e, 0x05, 0x63, 0x50, 0x2d, 0x1c, 0x97, 0x08, 0x70, 0xf0, 0x3e, 0x80, 0x03, 0xb0,
                0x4f, 0xe6, 0xcf, 0x7d,
            ],
            Elements::XorXor16 => [
                0x25, 0xde, 0xc0, 0x2c, 0x5c, 0xaa, 0xed, 0xa6, 0x6e, 0xa2, 0xab, 0x32, 0xe5, 0xea,
                0x41, 0xb7, 0x68, 0x24, 0x12, 0x21, 0x20, 0xa3, 0xca, 0x0d, 0x9a, 0x4e, 0xd5, 0x0e,
                0x8c, 0x31, 0xc2, 0x45,
            ],
            Elements::XorXor32 => [
                0x1e, 0x00, 0x76, 0xa0, 0x99, 0x93, 0x28, 0xaa, 0xce, 0x84, 0xc4, 0x82, 0x78, 0x0a,
                0xe5, 0x86, 0x93, 0x7d, 0x3e, 0x36, 0x5a, 0x3b, 0x32, 0xee, 0x73, 0xe2, 0xbf, 0x04,
                0xc8, 0x25, 0x37, 0xd2,
            ],
            Elements::XorXor64 => [
                0x2b, 0x14, 0xc3, 0x27, 0xc5, 0xd5, 0x3c, 0xd1, 0x25, 0xe7, 0x42, 0x9f, 0x2b, 0x97,
                0x66, 0x4a, 0x85, 0x77, 0x8c, 0xdf, 0xad, 0x24, 0x53, 0xec, 0xb1, 0x93, 0xb3, 0x5d,
                0x8d, 0x31, 0xac, 0x47,
            ],
            Elements::XorXor8 => [
                0xe2, 0x57, 0xcb, 0x12, 0xa9, 0x63, 0x24, 0x88, 0xa1, 0x5d, 0x49, 0x84, 0xfa, 0x38,
                0x88, 0x90, 0x3a, 0x95, 0x93, 0x86, 0xe8, 0x4f, 0x1c, 0x07, 0xe9, 0xd2, 0x9d, 0xf7,
                0x12, 0x04, 0x70, 0x71,
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
            Elements::TapleafHash => b"1",
            Elements::TapleafVersion => b"1",
            Elements::Tappath => b"***22*22**22*22",
            Elements::TappathHash => b"1",
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
            Elements::TapleafHash => b"h",
            Elements::TapleafVersion => b"***22*22**22*22",
            Elements::Tappath => b"+1h",
            Elements::TappathHash => b"h",
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
            Elements::TappathHash => (5896, 13),
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
            Elements::Tappath => (462382, 19),
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
                                                            0 => {Elements::TappathHash},
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
                                                                                    0 => {Elements::Tappath},
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
            Elements::TapleafHash => &simplicity_sys::c_jets::jets_wrapper::tapleaf_hash,
            Elements::TapleafVersion => &simplicity_sys::c_jets::jets_wrapper::tapleaf_version,
            Elements::Tappath => &simplicity_sys::c_jets::jets_wrapper::tappath,
            Elements::TappathHash => &simplicity_sys::c_jets::jets_wrapper::tappath_hash,
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

    fn cost(&self) -> Cost {
        match self {
            Elements::Add16 => Cost::from_milliweight(133),
            Elements::Add32 => Cost::from_milliweight(133),
            Elements::Add64 => Cost::from_milliweight(133),
            Elements::Add8 => Cost::from_milliweight(133),
            Elements::All16 => Cost::from_milliweight(98),
            Elements::All32 => Cost::from_milliweight(98),
            Elements::All64 => Cost::from_milliweight(98),
            Elements::All8 => Cost::from_milliweight(98),
            Elements::And16 => Cost::from_milliweight(133),
            Elements::And32 => Cost::from_milliweight(133),
            Elements::And64 => Cost::from_milliweight(133),
            Elements::And8 => Cost::from_milliweight(133),
            Elements::AnnexHash => Cost::from_milliweight(465),
            Elements::AssetAmountHash => Cost::from_milliweight(399),
            Elements::Bip0340Verify => Cost::from_milliweight(50735),
            Elements::BuildTapbranch => Cost::from_milliweight(3167),
            Elements::BuildTapleafSimplicity => Cost::from_milliweight(2240),
            Elements::CalculateAsset => Cost::from_milliweight(794),
            Elements::CalculateConfidentialToken => Cost::from_milliweight(774),
            Elements::CalculateExplicitToken => Cost::from_milliweight(769),
            Elements::CalculateIssuanceEntropy => Cost::from_milliweight(3564),
            Elements::Ch16 => Cost::from_milliweight(133),
            Elements::Ch32 => Cost::from_milliweight(133),
            Elements::Ch64 => Cost::from_milliweight(133),
            Elements::Ch8 => Cost::from_milliweight(133),
            Elements::CheckLockDistance => Cost::from_milliweight(168),
            Elements::CheckLockDuration => Cost::from_milliweight(182),
            Elements::CheckLockHeight => Cost::from_milliweight(117),
            Elements::CheckLockTime => Cost::from_milliweight(95),
            Elements::CheckSigVerify => Cost::from_milliweight(50000),
            Elements::Complement16 => Cost::from_milliweight(133),
            Elements::Complement32 => Cost::from_milliweight(133),
            Elements::Complement64 => Cost::from_milliweight(133),
            Elements::Complement8 => Cost::from_milliweight(133),
            Elements::CurrentAmount => Cost::from_milliweight(299),
            Elements::CurrentAnnexHash => Cost::from_milliweight(128),
            Elements::CurrentAsset => Cost::from_milliweight(158),
            Elements::CurrentIndex => Cost::from_milliweight(59),
            Elements::CurrentIssuanceAssetAmount => Cost::from_milliweight(132),
            Elements::CurrentIssuanceAssetProof => Cost::from_milliweight(129),
            Elements::CurrentIssuanceTokenAmount => Cost::from_milliweight(127),
            Elements::CurrentIssuanceTokenProof => Cost::from_milliweight(143),
            Elements::CurrentNewIssuanceContract => Cost::from_milliweight(146),
            Elements::CurrentPegin => Cost::from_milliweight(120),
            Elements::CurrentPrevOutpoint => Cost::from_milliweight(120),
            Elements::CurrentReissuanceBlinding => Cost::from_milliweight(200),
            Elements::CurrentReissuanceEntropy => Cost::from_milliweight(73),
            Elements::CurrentScriptHash => Cost::from_milliweight(188),
            Elements::CurrentScriptSigHash => Cost::from_milliweight(542),
            Elements::CurrentSequence => Cost::from_milliweight(147),
            Elements::Decompress => Cost::from_milliweight(11173),
            Elements::Decrement16 => Cost::from_milliweight(109),
            Elements::Decrement32 => Cost::from_milliweight(109),
            Elements::Decrement64 => Cost::from_milliweight(109),
            Elements::Decrement8 => Cost::from_milliweight(109),
            Elements::DivMod16 => Cost::from_milliweight(138),
            Elements::DivMod32 => Cost::from_milliweight(138),
            Elements::DivMod64 => Cost::from_milliweight(138),
            Elements::DivMod8 => Cost::from_milliweight(138),
            Elements::Divide16 => Cost::from_milliweight(138),
            Elements::Divide32 => Cost::from_milliweight(138),
            Elements::Divide64 => Cost::from_milliweight(138),
            Elements::Divide8 => Cost::from_milliweight(138),
            Elements::Divides16 => Cost::from_milliweight(138),
            Elements::Divides32 => Cost::from_milliweight(138),
            Elements::Divides64 => Cost::from_milliweight(138),
            Elements::Divides8 => Cost::from_milliweight(138),
            Elements::Eq16 => Cost::from_milliweight(69),
            Elements::Eq256 => Cost::from_milliweight(153),
            Elements::Eq32 => Cost::from_milliweight(69),
            Elements::Eq64 => Cost::from_milliweight(69),
            Elements::Eq8 => Cost::from_milliweight(69),
            Elements::FeAdd => Cost::from_milliweight(1408),
            Elements::FeInvert => Cost::from_milliweight(4008),
            Elements::FeIsOdd => Cost::from_milliweight(311),
            Elements::FeIsZero => Cost::from_milliweight(299),
            Elements::FeMultiply => Cost::from_milliweight(1109),
            Elements::FeMultiplyBeta => Cost::from_milliweight(755),
            Elements::FeNegate => Cost::from_milliweight(2187),
            Elements::FeNormalize => Cost::from_milliweight(2672),
            Elements::FeSquare => Cost::from_milliweight(800),
            Elements::FeSquareRoot => Cost::from_milliweight(12716),
            Elements::FullAdd16 => Cost::from_milliweight(93),
            Elements::FullAdd32 => Cost::from_milliweight(93),
            Elements::FullAdd64 => Cost::from_milliweight(93),
            Elements::FullAdd8 => Cost::from_milliweight(93),
            Elements::FullDecrement16 => Cost::from_milliweight(109),
            Elements::FullDecrement32 => Cost::from_milliweight(109),
            Elements::FullDecrement64 => Cost::from_milliweight(109),
            Elements::FullDecrement8 => Cost::from_milliweight(109),
            Elements::FullIncrement16 => Cost::from_milliweight(133),
            Elements::FullIncrement32 => Cost::from_milliweight(133),
            Elements::FullIncrement64 => Cost::from_milliweight(133),
            Elements::FullIncrement8 => Cost::from_milliweight(133),
            Elements::FullMultiply16 => Cost::from_milliweight(138),
            Elements::FullMultiply32 => Cost::from_milliweight(138),
            Elements::FullMultiply64 => Cost::from_milliweight(138),
            Elements::FullMultiply8 => Cost::from_milliweight(138),
            Elements::FullSubtract16 => Cost::from_milliweight(104),
            Elements::FullSubtract32 => Cost::from_milliweight(104),
            Elements::FullSubtract64 => Cost::from_milliweight(104),
            Elements::FullSubtract8 => Cost::from_milliweight(104),
            Elements::GeIsOnCurve => Cost::from_milliweight(768),
            Elements::GeNegate => Cost::from_milliweight(1596),
            Elements::GejAdd => Cost::from_milliweight(3274),
            Elements::GejDouble => Cost::from_milliweight(1899),
            Elements::GejGeAdd => Cost::from_milliweight(3004),
            Elements::GejGeAddEx => Cost::from_milliweight(2981),
            Elements::GejInfinity => Cost::from_milliweight(1042),
            Elements::GejIsInfinity => Cost::from_milliweight(1509),
            Elements::GejIsOnCurve => Cost::from_milliweight(1286),
            Elements::GejNegate => Cost::from_milliweight(1596),
            Elements::GejNormalize => Cost::from_milliweight(5931),
            Elements::GejRescale => Cost::from_milliweight(2462),
            Elements::GejXEquiv => Cost::from_milliweight(1383),
            Elements::GejYIsOdd => Cost::from_milliweight(5506),
            Elements::Generate => Cost::from_milliweight(49026),
            Elements::GenesisBlockHash => Cost::from_milliweight(143),
            Elements::High16 => Cost::from_milliweight(57),
            Elements::High32 => Cost::from_milliweight(57),
            Elements::High64 => Cost::from_milliweight(57),
            Elements::High8 => Cost::from_milliweight(57),
            Elements::Increment16 => Cost::from_milliweight(133),
            Elements::Increment32 => Cost::from_milliweight(133),
            Elements::Increment64 => Cost::from_milliweight(133),
            Elements::Increment8 => Cost::from_milliweight(133),
            Elements::InputAmount => Cost::from_milliweight(185),
            Elements::InputAmountsHash => Cost::from_milliweight(138),
            Elements::InputAnnexHash => Cost::from_milliweight(89),
            Elements::InputAnnexesHash => Cost::from_milliweight(125),
            Elements::InputAsset => Cost::from_milliweight(144),
            Elements::InputOutpointsHash => Cost::from_milliweight(141),
            Elements::InputPegin => Cost::from_milliweight(162),
            Elements::InputPrevOutpoint => Cost::from_milliweight(136),
            Elements::InputScriptHash => Cost::from_milliweight(152),
            Elements::InputScriptSigHash => Cost::from_milliweight(156),
            Elements::InputScriptSigsHash => Cost::from_milliweight(111),
            Elements::InputScriptsHash => Cost::from_milliweight(156),
            Elements::InputSequence => Cost::from_milliweight(89),
            Elements::InputSequencesHash => Cost::from_milliweight(117),
            Elements::InputUtxosHash => Cost::from_milliweight(131),
            Elements::InputsHash => Cost::from_milliweight(114),
            Elements::InternalKey => Cost::from_milliweight(108),
            Elements::IsOne16 => Cost::from_milliweight(98),
            Elements::IsOne32 => Cost::from_milliweight(98),
            Elements::IsOne64 => Cost::from_milliweight(98),
            Elements::IsOne8 => Cost::from_milliweight(98),
            Elements::IsZero16 => Cost::from_milliweight(98),
            Elements::IsZero32 => Cost::from_milliweight(98),
            Elements::IsZero64 => Cost::from_milliweight(98),
            Elements::IsZero8 => Cost::from_milliweight(98),
            Elements::Issuance => Cost::from_milliweight(89),
            Elements::IssuanceAsset => Cost::from_milliweight(149),
            Elements::IssuanceAssetAmount => Cost::from_milliweight(142),
            Elements::IssuanceAssetAmountsHash => Cost::from_milliweight(128),
            Elements::IssuanceAssetProof => Cost::from_milliweight(133),
            Elements::IssuanceBlindingEntropyHash => Cost::from_milliweight(142),
            Elements::IssuanceEntropy => Cost::from_milliweight(165),
            Elements::IssuanceRangeProofsHash => Cost::from_milliweight(140),
            Elements::IssuanceToken => Cost::from_milliweight(176),
            Elements::IssuanceTokenAmount => Cost::from_milliweight(152),
            Elements::IssuanceTokenAmountsHash => Cost::from_milliweight(143),
            Elements::IssuanceTokenProof => Cost::from_milliweight(139),
            Elements::IssuancesHash => Cost::from_milliweight(124),
            Elements::Le16 => Cost::from_milliweight(98),
            Elements::Le32 => Cost::from_milliweight(98),
            Elements::Le64 => Cost::from_milliweight(98),
            Elements::Le8 => Cost::from_milliweight(98),
            Elements::LinearCombination1 => Cost::from_milliweight(83117),
            Elements::LinearVerify1 => Cost::from_milliweight(49791),
            Elements::LockTime => Cost::from_milliweight(59),
            Elements::Low16 => Cost::from_milliweight(57),
            Elements::Low32 => Cost::from_milliweight(57),
            Elements::Low64 => Cost::from_milliweight(57),
            Elements::Low8 => Cost::from_milliweight(57),
            Elements::Lt16 => Cost::from_milliweight(98),
            Elements::Lt32 => Cost::from_milliweight(98),
            Elements::Lt64 => Cost::from_milliweight(98),
            Elements::Lt8 => Cost::from_milliweight(98),
            Elements::Maj16 => Cost::from_milliweight(133),
            Elements::Maj32 => Cost::from_milliweight(133),
            Elements::Maj64 => Cost::from_milliweight(133),
            Elements::Maj8 => Cost::from_milliweight(133),
            Elements::Max16 => Cost::from_milliweight(98),
            Elements::Max32 => Cost::from_milliweight(98),
            Elements::Max64 => Cost::from_milliweight(98),
            Elements::Max8 => Cost::from_milliweight(98),
            Elements::Median16 => Cost::from_milliweight(98),
            Elements::Median32 => Cost::from_milliweight(98),
            Elements::Median64 => Cost::from_milliweight(98),
            Elements::Median8 => Cost::from_milliweight(98),
            Elements::Min16 => Cost::from_milliweight(98),
            Elements::Min32 => Cost::from_milliweight(98),
            Elements::Min64 => Cost::from_milliweight(98),
            Elements::Min8 => Cost::from_milliweight(98),
            Elements::Modulo16 => Cost::from_milliweight(138),
            Elements::Modulo32 => Cost::from_milliweight(138),
            Elements::Modulo64 => Cost::from_milliweight(138),
            Elements::Modulo8 => Cost::from_milliweight(138),
            Elements::Multiply16 => Cost::from_milliweight(87),
            Elements::Multiply32 => Cost::from_milliweight(87),
            Elements::Multiply64 => Cost::from_milliweight(87),
            Elements::Multiply8 => Cost::from_milliweight(87),
            Elements::Negate16 => Cost::from_milliweight(109),
            Elements::Negate32 => Cost::from_milliweight(109),
            Elements::Negate64 => Cost::from_milliweight(109),
            Elements::Negate8 => Cost::from_milliweight(109),
            Elements::NewIssuanceContract => Cost::from_milliweight(192),
            Elements::NonceHash => Cost::from_milliweight(433),
            Elements::NumInputs => Cost::from_milliweight(60),
            Elements::NumOutputs => Cost::from_milliweight(60),
            Elements::One16 => Cost::from_milliweight(53),
            Elements::One32 => Cost::from_milliweight(53),
            Elements::One64 => Cost::from_milliweight(53),
            Elements::One8 => Cost::from_milliweight(53),
            Elements::Or16 => Cost::from_milliweight(133),
            Elements::Or32 => Cost::from_milliweight(133),
            Elements::Or64 => Cost::from_milliweight(133),
            Elements::Or8 => Cost::from_milliweight(133),
            Elements::OutpointHash => Cost::from_milliweight(427),
            Elements::OutputAmount => Cost::from_milliweight(166),
            Elements::OutputAmountsHash => Cost::from_milliweight(127),
            Elements::OutputAsset => Cost::from_milliweight(142),
            Elements::OutputNonce => Cost::from_milliweight(74),
            Elements::OutputNoncesHash => Cost::from_milliweight(111),
            Elements::OutputNullDatum => Cost::from_milliweight(73),
            Elements::OutputRangeProof => Cost::from_milliweight(233),
            Elements::OutputRangeProofsHash => Cost::from_milliweight(111),
            Elements::OutputScriptHash => Cost::from_milliweight(130),
            Elements::OutputScriptsHash => Cost::from_milliweight(122),
            Elements::OutputSurjectionProof => Cost::from_milliweight(127),
            Elements::OutputSurjectionProofsHash => Cost::from_milliweight(163),
            Elements::OutputsHash => Cost::from_milliweight(136),
            Elements::ParseLock => Cost::from_milliweight(84),
            Elements::ParseSequence => Cost::from_milliweight(82),
            Elements::PointVerify1 => Cost::from_milliweight(56153),
            Elements::ReissuanceBlinding => Cost::from_milliweight(82),
            Elements::ReissuanceEntropy => Cost::from_milliweight(179),
            Elements::ScalarAdd => Cost::from_milliweight(862),
            Elements::ScalarInvert => Cost::from_milliweight(3853),
            Elements::ScalarIsZero => Cost::from_milliweight(323),
            Elements::ScalarMultiply => Cost::from_milliweight(1070),
            Elements::ScalarMultiplyLambda => Cost::from_milliweight(736),
            Elements::ScalarNegate => Cost::from_milliweight(726),
            Elements::ScalarNormalize => Cost::from_milliweight(740),
            Elements::ScalarSquare => Cost::from_milliweight(822),
            Elements::Scale => Cost::from_milliweight(75458),
            Elements::ScriptCMR => Cost::from_milliweight(110),
            Elements::Sha256Block => Cost::from_milliweight(1046),
            Elements::Sha256Ctx8Add1 => Cost::from_milliweight(454),
            Elements::Sha256Ctx8Add128 => Cost::from_milliweight(401),
            Elements::Sha256Ctx8Add16 => Cost::from_milliweight(340),
            Elements::Sha256Ctx8Add2 => Cost::from_milliweight(576),
            Elements::Sha256Ctx8Add256 => Cost::from_milliweight(572),
            Elements::Sha256Ctx8Add32 => Cost::from_milliweight(326),
            Elements::Sha256Ctx8Add4 => Cost::from_milliweight(570),
            Elements::Sha256Ctx8Add512 => Cost::from_milliweight(559),
            Elements::Sha256Ctx8Add64 => Cost::from_milliweight(475),
            Elements::Sha256Ctx8Add8 => Cost::from_milliweight(330),
            Elements::Sha256Ctx8AddBuffer511 => Cost::from_milliweight(412),
            Elements::Sha256Ctx8Finalize => Cost::from_milliweight(1205),
            Elements::Sha256Ctx8Init => Cost::from_milliweight(237),
            Elements::Sha256Iv => Cost::from_milliweight(107),
            Elements::SigAllHash => Cost::from_milliweight(135),
            Elements::Some16 => Cost::from_milliweight(98),
            Elements::Some32 => Cost::from_milliweight(98),
            Elements::Some64 => Cost::from_milliweight(98),
            Elements::Some8 => Cost::from_milliweight(98),
            Elements::Subtract16 => Cost::from_milliweight(109),
            Elements::Subtract32 => Cost::from_milliweight(109),
            Elements::Subtract64 => Cost::from_milliweight(109),
            Elements::Subtract8 => Cost::from_milliweight(109),
            Elements::TapEnvHash => Cost::from_milliweight(136),
            Elements::TapleafHash => Cost::from_milliweight(117),
            Elements::TapleafVersion => Cost::from_milliweight(80),
            Elements::Tappath => Cost::from_milliweight(94),
            Elements::TappathHash => Cost::from_milliweight(136),
            Elements::TxHash => Cost::from_milliweight(163),
            Elements::TxIsFinal => Cost::from_milliweight(67),
            Elements::TxLockDistance => Cost::from_milliweight(72),
            Elements::TxLockDuration => Cost::from_milliweight(63),
            Elements::TxLockHeight => Cost::from_milliweight(71),
            Elements::TxLockTime => Cost::from_milliweight(61),
            Elements::Verify => Cost::from_milliweight(45),
            Elements::Version => Cost::from_milliweight(176),
            Elements::Xor16 => Cost::from_milliweight(133),
            Elements::Xor32 => Cost::from_milliweight(133),
            Elements::Xor64 => Cost::from_milliweight(133),
            Elements::Xor8 => Cost::from_milliweight(133),
            Elements::XorXor16 => Cost::from_milliweight(133),
            Elements::XorXor32 => Cost::from_milliweight(133),
            Elements::XorXor64 => Cost::from_milliweight(133),
            Elements::XorXor8 => Cost::from_milliweight(133),
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
            Elements::TapleafHash => f.write_str("tapleaf_hash"),
            Elements::TapleafVersion => f.write_str("tapleaf_version"),
            Elements::Tappath => f.write_str("tappath"),
            Elements::TappathHash => f.write_str("tappath_hash"),
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
            "tapleaf_hash" => Ok(Elements::TapleafHash),
            "tapleaf_version" => Ok(Elements::TapleafVersion),
            "tappath" => Ok(Elements::Tappath),
            "tappath_hash" => Ok(Elements::TappathHash),
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
