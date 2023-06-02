/* This file has been automatically generated. */

use crate::bititer::BitIter;
use crate::bitwriter::BitWriter;
use crate::jet::type_name::TypeName;
use crate::jet::Jet;
use crate::merkle::cmr::Cmr;
use crate::decode_bits;
use crate::Error;
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
    AnnexHash,
    AssetAmountHash,
    Bip0340Verify,
    BuildTapbranch,
    BuildTapleafSimplicity,
    CalculateAsset,
    CalculateConfidentialToken,
    CalculateExplicitToken,
    CalculateIssuanceEntropy,
    CheckLockDistance,
    CheckLockDuration,
    CheckLockHeight,
    CheckLockTime,
    CheckSigVerify,
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
                0x51, 0x7a, 0x52, 0x5b, 0x98, 0xc8, 0x78, 0x60, 0x88, 0x41, 0x82, 0xcf, 0xc0, 0x14,
                0x81, 0x64, 0xaf, 0x35, 0x14, 0x63, 0xe0, 0xfb, 0xac, 0x34, 0x60, 0x65, 0x2b, 0x2f,
                0xb5, 0xed, 0xcf, 0x19,
            ],
            Elements::Add32 => [
                0xe4, 0x04, 0x66, 0xa7, 0xec, 0xf7, 0x1c, 0xe8, 0x62, 0xfb, 0x3c, 0x15, 0x4c, 0x1e,
                0x8f, 0x84, 0x5d, 0x7e, 0x57, 0x07, 0x46, 0x3a, 0x89, 0x45, 0x37, 0xa3, 0x2f, 0xc7,
                0x21, 0x49, 0x00, 0xad,
            ],
            Elements::Add64 => [
                0x81, 0x2e, 0x6e, 0x75, 0x70, 0x12, 0x1f, 0x8c, 0xc3, 0x52, 0xc0, 0x9a, 0x0a, 0xc7,
                0x1a, 0xe6, 0xcc, 0xfe, 0x85, 0x9e, 0x84, 0x97, 0x60, 0xbd, 0x47, 0x8d, 0x29, 0x8a,
                0xea, 0x82, 0xae, 0xb0,
            ],
            Elements::Add8 => [
                0x62, 0x61, 0x3c, 0x74, 0x98, 0xfe, 0x9c, 0x5e, 0x98, 0xa1, 0xf2, 0x4e, 0xe1, 0x07,
                0x0a, 0x02, 0xe9, 0xdc, 0xf2, 0x7f, 0x7f, 0x73, 0x63, 0x8c, 0x0a, 0x8f, 0xce, 0x85,
                0xb7, 0x64, 0x15, 0x1a,
            ],
            Elements::AnnexHash => [
                0xa1, 0xd4, 0xa3, 0x5f, 0x18, 0xfa, 0xa1, 0xbe, 0xec, 0x1d, 0xe6, 0xd8, 0x6e, 0xf5,
                0x40, 0x3e, 0x5a, 0x52, 0x87, 0x78, 0xbc, 0x38, 0xe4, 0xee, 0x95, 0xd7, 0xc1, 0xbf,
                0x5f, 0xe6, 0xde, 0x4b,
            ],
            Elements::AssetAmountHash => [
                0xb7, 0xfa, 0x2d, 0x7a, 0x2e, 0x52, 0x65, 0x58, 0x0b, 0x00, 0xf4, 0xce, 0x23, 0xab,
                0xa6, 0x40, 0x6c, 0xc4, 0x99, 0xca, 0xd1, 0x29, 0x1f, 0x41, 0xf9, 0x1f, 0x8b, 0x4c,
                0xec, 0x77, 0xa2, 0x13,
            ],
            Elements::Bip0340Verify => [
                0xc6, 0x19, 0xa5, 0x79, 0x12, 0xb7, 0x6c, 0x30, 0x5d, 0xc9, 0xe2, 0x62, 0xf6, 0x98,
                0xe2, 0x05, 0x06, 0x1f, 0x46, 0x26, 0xc0, 0x70, 0x04, 0x7b, 0x0e, 0x1b, 0xa9, 0x99,
                0xdb, 0xf1, 0x00, 0x6c,
            ],
            Elements::BuildTapbranch => [
                0x37, 0x27, 0x1a, 0x84, 0x5d, 0xfe, 0x8d, 0xf3, 0x9a, 0xec, 0xce, 0x72, 0xc4, 0xe6,
                0xda, 0xb0, 0xcf, 0x4f, 0x2c, 0xdd, 0xbc, 0xc6, 0x7a, 0x1a, 0xa7, 0xff, 0x4a, 0x50,
                0x03, 0x30, 0x9d, 0xce,
            ],
            Elements::BuildTapleafSimplicity => [
                0x39, 0x4c, 0x1b, 0xf2, 0xfb, 0xe8, 0xba, 0x15, 0x04, 0xba, 0x15, 0xcb, 0xaa, 0xb2,
                0x0c, 0x6c, 0xdc, 0x83, 0x6a, 0x32, 0x5c, 0x31, 0xbf, 0x3c, 0xd9, 0xcc, 0x9e, 0xf1,
                0x2c, 0xc0, 0x09, 0x74,
            ],
            Elements::CalculateAsset => [
                0xb6, 0x25, 0xbd, 0xf7, 0x1f, 0xb2, 0xa7, 0x4b, 0x7d, 0xf8, 0x85, 0x84, 0xfd, 0xa9,
                0x5e, 0x62, 0xd7, 0x6f, 0x6d, 0x76, 0x11, 0xee, 0x22, 0xbd, 0x1a, 0xd0, 0x0a, 0x6d,
                0xc8, 0xd9, 0x33, 0x99,
            ],
            Elements::CalculateConfidentialToken => [
                0x4e, 0x9e, 0xc7, 0x73, 0x2e, 0xba, 0xce, 0xf6, 0x9b, 0xe4, 0x0c, 0xe9, 0x53, 0x95,
                0xc2, 0x21, 0x4a, 0x78, 0x98, 0xbc, 0x68, 0x82, 0x35, 0x52, 0x7d, 0xb7, 0xba, 0xf0,
                0xdd, 0x0b, 0x47, 0xfc,
            ],
            Elements::CalculateExplicitToken => [
                0x36, 0xda, 0xaf, 0xef, 0xfe, 0x74, 0x60, 0x43, 0x12, 0xda, 0xe1, 0x54, 0xb9, 0xe6,
                0xac, 0xb9, 0x64, 0x74, 0x5d, 0xf5, 0x4f, 0xe5, 0x59, 0xf1, 0x7d, 0xc8, 0x13, 0xd3,
                0xd2, 0xf3, 0xd6, 0x65,
            ],
            Elements::CalculateIssuanceEntropy => [
                0x26, 0x1d, 0xb7, 0x3c, 0xf0, 0xad, 0x2c, 0x7b, 0x68, 0x55, 0x58, 0x18, 0xf5, 0x00,
                0x46, 0x7b, 0x20, 0x61, 0xa8, 0xfd, 0x61, 0x4d, 0x43, 0x66, 0xa9, 0x8f, 0x04, 0x20,
                0xe4, 0x0a, 0x26, 0x4b,
            ],
            Elements::CheckLockDistance => [
                0x11, 0x48, 0x6d, 0xf3, 0xda, 0x60, 0x1f, 0xd3, 0x8b, 0x74, 0x06, 0x8c, 0xe0, 0x38,
                0xf2, 0x45, 0xfd, 0xe6, 0x17, 0x56, 0xc6, 0x68, 0x32, 0x34, 0x14, 0x4f, 0x16, 0x3c,
                0xd1, 0xef, 0x2e, 0x8d,
            ],
            Elements::CheckLockDuration => [
                0x83, 0x06, 0x89, 0x17, 0x74, 0x54, 0x9b, 0x14, 0x5b, 0x19, 0x35, 0x66, 0xd5, 0xd2,
                0x22, 0x38, 0x2d, 0x26, 0x0e, 0x5a, 0x15, 0xfb, 0x52, 0xce, 0xd2, 0x3c, 0x26, 0x2b,
                0x17, 0x69, 0x9b, 0xc0,
            ],
            Elements::CheckLockHeight => [
                0x22, 0xa2, 0x9e, 0x57, 0xc6, 0x38, 0x69, 0xe2, 0x2d, 0xb2, 0x36, 0xac, 0x43, 0x47,
                0x41, 0x52, 0xd2, 0xcd, 0xd9, 0x58, 0x48, 0xf3, 0x00, 0x65, 0x1f, 0xf3, 0x3d, 0xa7,
                0xb2, 0x7c, 0x47, 0x44,
            ],
            Elements::CheckLockTime => [
                0xd6, 0x7a, 0xaa, 0xec, 0x7a, 0x42, 0xa6, 0x3b, 0x2e, 0xea, 0x09, 0x50, 0x86, 0x01,
                0xab, 0xfa, 0xf7, 0x48, 0xd1, 0x51, 0xf2, 0xc4, 0x58, 0xf3, 0xdd, 0x2b, 0xad, 0x00,
                0x22, 0x03, 0x88, 0x24,
            ],
            Elements::CheckSigVerify => [
                0xfe, 0x5e, 0x35, 0xd8, 0xa4, 0xa8, 0x46, 0xa1, 0x05, 0xac, 0x4b, 0x03, 0xfc, 0xe4,
                0xa9, 0x7a, 0x55, 0xbc, 0x40, 0xb5, 0x50, 0x4d, 0x9a, 0x60, 0x3c, 0x23, 0x9b, 0x05,
                0x59, 0xe3, 0x11, 0x02,
            ],
            Elements::CurrentAmount => [
                0x6a, 0x95, 0x6b, 0x72, 0x6a, 0xa8, 0x6e, 0x23, 0xbc, 0x28, 0x9a, 0xfa, 0x8e, 0xf7,
                0xd1, 0xde, 0x5e, 0xbc, 0x19, 0x37, 0x61, 0xb4, 0x82, 0xd8, 0x90, 0x3b, 0xff, 0xea,
                0x0a, 0x29, 0x50, 0x93,
            ],
            Elements::CurrentAnnexHash => [
                0xf2, 0xbc, 0xe4, 0x31, 0x72, 0x2d, 0xcc, 0x92, 0x3d, 0xe8, 0x36, 0x7a, 0x8a, 0x74,
                0xb4, 0xda, 0x69, 0xea, 0x4d, 0xf2, 0xe0, 0x9c, 0x98, 0xad, 0x22, 0xfe, 0x5c, 0x1b,
                0xba, 0x89, 0xb6, 0x07,
            ],
            Elements::CurrentAsset => [
                0x94, 0x9d, 0xf4, 0x68, 0x1e, 0xde, 0x99, 0xe7, 0x9b, 0xf5, 0x55, 0x53, 0xd6, 0x71,
                0xc4, 0x09, 0x52, 0x9d, 0xf5, 0x28, 0x14, 0x05, 0x71, 0xd9, 0x6f, 0xfe, 0xbe, 0x64,
                0xb4, 0x7a, 0xb1, 0xdd,
            ],
            Elements::CurrentIndex => [
                0xf3, 0x7c, 0x71, 0xdc, 0xb1, 0x85, 0xce, 0x16, 0x4b, 0x54, 0xed, 0x33, 0x07, 0x62,
                0x37, 0x8a, 0xa7, 0x3f, 0x8c, 0x4e, 0x68, 0xde, 0x4b, 0x5b, 0xa6, 0xb4, 0xbc, 0xf9,
                0x6a, 0x29, 0x33, 0xc5,
            ],
            Elements::CurrentIssuanceAssetAmount => [
                0x13, 0x7b, 0x9e, 0x56, 0xda, 0xa1, 0x99, 0x56, 0x03, 0x04, 0x94, 0x14, 0x2a, 0x30,
                0x16, 0x5a, 0x48, 0x45, 0x32, 0x30, 0x97, 0xf1, 0x47, 0x77, 0x58, 0x50, 0xe7, 0xfe,
                0x67, 0x15, 0xb5, 0x88,
            ],
            Elements::CurrentIssuanceAssetProof => [
                0x2e, 0xb5, 0x04, 0xee, 0xce, 0xc6, 0x44, 0xf3, 0x95, 0xf3, 0x50, 0x84, 0xd9, 0x32,
                0x60, 0xf9, 0x6d, 0x59, 0x15, 0x6b, 0x80, 0x06, 0x54, 0x16, 0x16, 0x40, 0xeb, 0x20,
                0x63, 0x9c, 0x2f, 0xda,
            ],
            Elements::CurrentIssuanceTokenAmount => [
                0x04, 0x15, 0x9c, 0xae, 0x38, 0x1d, 0x2e, 0x04, 0x52, 0x81, 0x23, 0xbb, 0x2d, 0xe2,
                0x8e, 0xa2, 0x97, 0x2b, 0x96, 0x58, 0x6e, 0x7f, 0xf2, 0x4b, 0xe3, 0x98, 0xc1, 0x97,
                0x3b, 0xcb, 0x0a, 0x36,
            ],
            Elements::CurrentIssuanceTokenProof => [
                0x37, 0x53, 0xe3, 0x3e, 0xe2, 0xc7, 0xf2, 0x40, 0xc7, 0x24, 0x2b, 0x7a, 0xf0, 0xec,
                0xe5, 0xce, 0x97, 0xb4, 0x7e, 0xbb, 0xa2, 0x3c, 0x71, 0xcf, 0x3a, 0x27, 0xf3, 0x5c,
                0x74, 0xbb, 0xdd, 0xcd,
            ],
            Elements::CurrentNewIssuanceContract => [
                0x82, 0x48, 0xc1, 0x53, 0x3b, 0x1a, 0x03, 0xc5, 0x97, 0x7f, 0x63, 0x45, 0x65, 0x06,
                0x5e, 0xcb, 0x83, 0x7b, 0x9c, 0x4c, 0x25, 0xb3, 0xa4, 0x1f, 0x54, 0xd7, 0x6d, 0xd5,
                0xd6, 0x73, 0x30, 0x23,
            ],
            Elements::CurrentPegin => [
                0x11, 0xb7, 0xf0, 0xe3, 0x47, 0x19, 0x06, 0xb1, 0xa3, 0x7c, 0xbe, 0xe8, 0x0f, 0x78,
                0xb9, 0xa4, 0x25, 0x63, 0x3f, 0xf8, 0x58, 0xbb, 0xf3, 0x80, 0x23, 0xa7, 0xbb, 0x1d,
                0x8f, 0x11, 0xa7, 0x19,
            ],
            Elements::CurrentPrevOutpoint => [
                0xcf, 0x66, 0xcd, 0x9d, 0x49, 0x5c, 0x9a, 0xbd, 0x97, 0x0f, 0xbd, 0xa7, 0xb6, 0x1b,
                0x3b, 0xba, 0x34, 0x31, 0x06, 0xbf, 0x52, 0x29, 0x54, 0x36, 0x12, 0xcb, 0x20, 0xc5,
                0xd5, 0x05, 0x82, 0xf5,
            ],
            Elements::CurrentReissuanceBlinding => [
                0x9c, 0xc9, 0x1a, 0xe6, 0x23, 0xe9, 0x5f, 0xf9, 0x2b, 0x6b, 0x3a, 0x73, 0xb7, 0x72,
                0xd8, 0x71, 0x1b, 0xdc, 0x00, 0xfc, 0x68, 0xe2, 0x47, 0xb5, 0x14, 0x00, 0x34, 0x1f,
                0xc9, 0x20, 0xb7, 0x05,
            ],
            Elements::CurrentReissuanceEntropy => [
                0x20, 0x68, 0x43, 0x0e, 0xf8, 0x28, 0xf3, 0x51, 0x1e, 0xf4, 0x54, 0x96, 0x31, 0x0f,
                0x93, 0xbb, 0x72, 0x02, 0x6f, 0xa7, 0x9e, 0x30, 0x36, 0x58, 0xad, 0x8f, 0x21, 0xb8,
                0x8a, 0x8a, 0x28, 0xdc,
            ],
            Elements::CurrentScriptHash => [
                0x04, 0x92, 0x99, 0x91, 0xd6, 0xcb, 0x6e, 0xf7, 0xb9, 0x1b, 0x21, 0xd9, 0x6b, 0xbe,
                0x14, 0x4f, 0x7b, 0x97, 0x40, 0x39, 0xac, 0xaf, 0x89, 0xaa, 0x0e, 0xa0, 0xa2, 0x5f,
                0x6f, 0x5e, 0x2f, 0x66,
            ],
            Elements::CurrentScriptSigHash => [
                0x42, 0x78, 0xe5, 0x14, 0xda, 0xdf, 0xf4, 0xe0, 0x11, 0x7c, 0xdc, 0xcd, 0x95, 0xcb,
                0xdc, 0xd1, 0xc3, 0x9a, 0x52, 0xb7, 0xc5, 0x9a, 0xca, 0x19, 0x16, 0x6c, 0xbb, 0x3b,
                0xae, 0xc7, 0x03, 0x5b,
            ],
            Elements::CurrentSequence => [
                0x89, 0x72, 0x85, 0xa4, 0x3e, 0x6f, 0x97, 0xf3, 0x18, 0x61, 0xb6, 0x07, 0x54, 0x87,
                0x5f, 0x6a, 0xb9, 0x7b, 0x0e, 0xbf, 0x28, 0xe4, 0xde, 0x3f, 0xf4, 0x90, 0x7d, 0xe1,
                0x33, 0x04, 0x76, 0x6f,
            ],
            Elements::Decompress => [
                0xab, 0x5a, 0x2d, 0xbc, 0x0f, 0x2c, 0x82, 0x08, 0x6d, 0x2d, 0x46, 0xbb, 0xa5, 0x69,
                0x1f, 0x13, 0x12, 0xba, 0xcc, 0x94, 0x6b, 0x08, 0xef, 0xfb, 0xe7, 0xf8, 0x51, 0x51,
                0x14, 0x1f, 0x7d, 0xcf,
            ],
            Elements::Decrement16 => [
                0x4a, 0x10, 0x94, 0x9e, 0xb5, 0x98, 0xc0, 0x04, 0xbc, 0xae, 0xaf, 0xb8, 0x2f, 0x02,
                0xac, 0x62, 0x06, 0xc2, 0xa0, 0xf2, 0x32, 0xff, 0x95, 0x40, 0xa4, 0xff, 0x02, 0xa5,
                0x54, 0x71, 0xf8, 0xfa,
            ],
            Elements::Decrement32 => [
                0x29, 0x1e, 0xab, 0x34, 0x34, 0xe2, 0x01, 0x77, 0x5e, 0xcb, 0x68, 0x21, 0x5b, 0x54,
                0x03, 0xfe, 0x8c, 0x9e, 0xef, 0xbe, 0x88, 0x0c, 0xb2, 0xe0, 0x96, 0xe7, 0x60, 0x22,
                0xb8, 0x30, 0x1b, 0x59,
            ],
            Elements::Decrement64 => [
                0xe1, 0x8e, 0xce, 0x15, 0xc9, 0x14, 0xaa, 0x0c, 0x22, 0x2d, 0xec, 0x3b, 0x0c, 0xa0,
                0xab, 0x39, 0xcd, 0x81, 0xed, 0x38, 0x25, 0x6c, 0xf8, 0x77, 0x0b, 0x78, 0x17, 0x3b,
                0xb9, 0xc6, 0xb4, 0x87,
            ],
            Elements::Decrement8 => [
                0xd2, 0xaf, 0xfa, 0xd9, 0xae, 0xa4, 0xfd, 0xff, 0x91, 0x6d, 0xbe, 0x5f, 0x90, 0xa6,
                0xf9, 0x3e, 0x2f, 0x4d, 0x30, 0xba, 0x87, 0x3a, 0x52, 0x81, 0xa4, 0x41, 0xd4, 0xa7,
                0x93, 0x70, 0xbe, 0x96,
            ],
            Elements::DivMod16 => [
                0xf5, 0x09, 0x47, 0x22, 0x26, 0xc5, 0x32, 0x5a, 0x4a, 0x41, 0x7d, 0x68, 0xc9, 0x97,
                0xd6, 0x39, 0x5d, 0xe0, 0x99, 0x29, 0xdd, 0xc2, 0x27, 0x1e, 0xc7, 0x28, 0x20, 0x46,
                0xed, 0xbd, 0xdc, 0x29,
            ],
            Elements::DivMod32 => [
                0xa2, 0x90, 0x83, 0x06, 0x91, 0xf7, 0x3a, 0x5f, 0x40, 0x89, 0x18, 0x6f, 0xce, 0x3b,
                0x22, 0xff, 0x4c, 0xc1, 0x0a, 0xee, 0x1a, 0xb2, 0x58, 0xa4, 0xac, 0x51, 0x86, 0x4d,
                0x12, 0xf4, 0xe3, 0x3b,
            ],
            Elements::DivMod64 => [
                0x64, 0x97, 0x16, 0xf1, 0x59, 0x72, 0xf7, 0xb9, 0xc7, 0x0f, 0x9a, 0xd2, 0x2f, 0x65,
                0xb3, 0x91, 0xff, 0x68, 0x52, 0xac, 0xc8, 0x78, 0x37, 0xbb, 0xce, 0xdd, 0x17, 0x1d,
                0xcb, 0x8b, 0xd3, 0x14,
            ],
            Elements::DivMod8 => [
                0xd9, 0xbe, 0x81, 0xb7, 0x53, 0x49, 0xe3, 0xbb, 0x22, 0x21, 0xc4, 0xd4, 0x5a, 0xe0,
                0x4f, 0x8d, 0xf1, 0xbe, 0x4e, 0x7e, 0x76, 0x8b, 0x36, 0x83, 0xb1, 0xa1, 0x55, 0x68,
                0xb8, 0x31, 0xfe, 0xfb,
            ],
            Elements::Divide16 => [
                0xd3, 0x1f, 0xf5, 0x9b, 0x05, 0x84, 0x17, 0x2f, 0xc1, 0xe4, 0xce, 0x0e, 0x5e, 0x89,
                0xbe, 0x12, 0x80, 0x90, 0x8e, 0x49, 0x47, 0x28, 0xee, 0xd7, 0x53, 0xbf, 0xa7, 0x1f,
                0xb6, 0xd8, 0x51, 0x93,
            ],
            Elements::Divide32 => [
                0xab, 0x97, 0x45, 0xca, 0xf9, 0x3e, 0x6b, 0xaa, 0x48, 0xaf, 0x0e, 0xe3, 0xd2, 0xbe,
                0xf3, 0x9d, 0xb8, 0x0f, 0xe5, 0x4d, 0x06, 0xd2, 0xa8, 0xa6, 0x78, 0x75, 0x6d, 0x07,
                0x75, 0x32, 0x5f, 0xc5,
            ],
            Elements::Divide64 => [
                0xae, 0x18, 0xfb, 0xff, 0xb0, 0x4e, 0x5c, 0x0d, 0x9f, 0xe9, 0x73, 0xa4, 0x5d, 0xcc,
                0x9a, 0xcd, 0x0e, 0xdd, 0xe7, 0x74, 0xb8, 0x0a, 0x6d, 0xb6, 0x63, 0x91, 0x23, 0x75,
                0x63, 0x46, 0xde, 0xec,
            ],
            Elements::Divide8 => [
                0x61, 0x37, 0x2c, 0x0c, 0xd4, 0xa7, 0xe4, 0x78, 0x2d, 0x01, 0xd5, 0xb9, 0xce, 0x03,
                0x4d, 0xa9, 0x2c, 0x8a, 0x35, 0x13, 0x84, 0xcd, 0x28, 0xd6, 0x48, 0x48, 0x47, 0x93,
                0xc7, 0x08, 0x66, 0x39,
            ],
            Elements::Divides16 => [
                0xf6, 0x6f, 0x19, 0x64, 0x09, 0x34, 0xa8, 0x25, 0xc4, 0x66, 0x08, 0xcd, 0x06, 0x5d,
                0xca, 0x61, 0x5d, 0x9c, 0xb8, 0x8b, 0x6d, 0x6e, 0xeb, 0xd6, 0x3c, 0xde, 0x22, 0x12,
                0x9b, 0xb4, 0xf6, 0xb4,
            ],
            Elements::Divides32 => [
                0x38, 0x59, 0x74, 0x5d, 0x44, 0x09, 0x64, 0xa8, 0x80, 0x2e, 0xbe, 0x32, 0xb1, 0x0c,
                0x56, 0xf1, 0x86, 0x52, 0xd8, 0xe0, 0xde, 0xd3, 0xc8, 0x8d, 0xe0, 0x32, 0x76, 0xbf,
                0xb3, 0xcf, 0x15, 0x63,
            ],
            Elements::Divides64 => [
                0x38, 0xa5, 0xd0, 0x32, 0xe9, 0x2f, 0x57, 0x8f, 0x4d, 0x39, 0xa3, 0x19, 0x2a, 0xce,
                0x98, 0xc7, 0x50, 0xfa, 0x1c, 0x4b, 0x13, 0x58, 0xa6, 0x0e, 0xec, 0x9a, 0x23, 0xc6,
                0x93, 0x47, 0x7e, 0x54,
            ],
            Elements::Divides8 => [
                0x54, 0x6b, 0x4e, 0x70, 0x78, 0xa7, 0xd4, 0xf2, 0x7b, 0x44, 0xe6, 0x50, 0xdc, 0x29,
                0xb3, 0x61, 0x42, 0x97, 0xb7, 0xea, 0x32, 0xab, 0x9a, 0x94, 0x89, 0xac, 0xcc, 0x81,
                0xfc, 0x8d, 0xb6, 0xcd,
            ],
            Elements::Eq16 => [
                0x9a, 0x5e, 0x8d, 0xb0, 0xdd, 0x21, 0xfb, 0xbc, 0x8c, 0x6a, 0x57, 0xa5, 0xf5, 0x49,
                0x61, 0x83, 0xca, 0xd7, 0x88, 0xed, 0xb8, 0x9c, 0x81, 0x59, 0x9d, 0x77, 0x09, 0xf3,
                0xb1, 0x97, 0x1e, 0x81,
            ],
            Elements::Eq256 => [
                0xd8, 0xbb, 0x63, 0x54, 0x45, 0xb5, 0x6b, 0xc0, 0x6a, 0x33, 0x5e, 0x1d, 0xe8, 0xe5,
                0x56, 0xcc, 0x06, 0xf4, 0x93, 0x08, 0x67, 0x4c, 0xb8, 0x71, 0x6a, 0x5b, 0xa8, 0x1c,
                0xce, 0xd7, 0x54, 0x0e,
            ],
            Elements::Eq32 => [
                0x88, 0xbf, 0xc8, 0x93, 0x7c, 0x11, 0x87, 0x3b, 0x06, 0xe3, 0xd8, 0xaf, 0xa4, 0xf9,
                0xf9, 0xe6, 0x22, 0xdb, 0x93, 0xf4, 0x70, 0x6b, 0xde, 0x36, 0xbc, 0x60, 0x14, 0x03,
                0x34, 0x48, 0xf3, 0x01,
            ],
            Elements::Eq64 => [
                0xdb, 0xd0, 0xa3, 0x5e, 0x1f, 0x78, 0xe6, 0x1a, 0xd6, 0x75, 0xbd, 0xc1, 0x46, 0x29,
                0xc0, 0xd8, 0x32, 0xae, 0x1d, 0xe6, 0x6b, 0xd7, 0x73, 0xaa, 0x5c, 0xb3, 0x15, 0x79,
                0x81, 0x4c, 0x16, 0xba,
            ],
            Elements::Eq8 => [
                0xcb, 0xe1, 0xb7, 0x99, 0xb0, 0x70, 0xeb, 0xad, 0xb0, 0x3d, 0x6a, 0x2e, 0xb3, 0x16,
                0x2c, 0xf5, 0x2c, 0xa0, 0x15, 0xc9, 0xb6, 0xc6, 0x95, 0x90, 0x31, 0x52, 0xc1, 0xe3,
                0x48, 0x7d, 0xca, 0x3d,
            ],
            Elements::FeAdd => [
                0xc0, 0x9d, 0x58, 0xe3, 0x8d, 0xc4, 0xce, 0x1a, 0xcc, 0x09, 0xda, 0xe8, 0xa5, 0x88,
                0x19, 0x08, 0xfe, 0x1e, 0xbc, 0x7f, 0x1f, 0xc7, 0x42, 0xc6, 0x1c, 0xdc, 0x84, 0x93,
                0xf2, 0x33, 0xb9, 0x4a,
            ],
            Elements::FeInvert => [
                0xb6, 0xb1, 0x1b, 0xd6, 0x02, 0x58, 0xd3, 0x26, 0xb1, 0x9b, 0x6f, 0x78, 0x38, 0x7f,
                0xf3, 0xaa, 0xbf, 0x6a, 0x4c, 0x41, 0x9d, 0x0c, 0x5a, 0x8d, 0xda, 0x6b, 0x44, 0x05,
                0x0b, 0xc6, 0xe9, 0xd5,
            ],
            Elements::FeIsOdd => [
                0x4d, 0x21, 0xe4, 0xb5, 0x47, 0x6f, 0xcf, 0x56, 0xa3, 0x69, 0xe5, 0xa1, 0x89, 0xc4,
                0x86, 0xe0, 0x98, 0x7f, 0x93, 0x32, 0xaa, 0xf4, 0xc0, 0xa7, 0x5a, 0xc3, 0xaf, 0x09,
                0xf8, 0x1b, 0x17, 0x09,
            ],
            Elements::FeIsZero => [
                0x52, 0x35, 0x9f, 0x07, 0xed, 0x7a, 0x7c, 0x97, 0x0e, 0x94, 0x7d, 0xc8, 0xc4, 0x99,
                0x40, 0x56, 0x6c, 0x1a, 0x1e, 0x09, 0x5d, 0x03, 0x60, 0x4a, 0xfd, 0xb4, 0x43, 0x3d,
                0xa1, 0x3a, 0x87, 0xec,
            ],
            Elements::FeMultiply => [
                0xac, 0x5a, 0x36, 0x26, 0xb5, 0xfc, 0x2b, 0x5a, 0x20, 0x6a, 0xc1, 0x8f, 0x1b, 0x0f,
                0x9e, 0xca, 0xcb, 0x5c, 0x63, 0x14, 0xc4, 0xef, 0xda, 0x59, 0xe0, 0xfa, 0xd3, 0xa1,
                0xb5, 0x99, 0xa1, 0xbd,
            ],
            Elements::FeMultiplyBeta => [
                0xe7, 0xa6, 0x98, 0xe2, 0x5e, 0xbb, 0xf7, 0x0f, 0x8c, 0xed, 0x31, 0x30, 0xac, 0x04,
                0xd6, 0x74, 0xa9, 0xda, 0x39, 0xe0, 0x61, 0x76, 0x1b, 0xfd, 0x9d, 0x87, 0xd7, 0x94,
                0x89, 0x8f, 0x8a, 0x7a,
            ],
            Elements::FeNegate => [
                0x87, 0x66, 0x58, 0x5d, 0x27, 0xd1, 0x88, 0x63, 0x42, 0x71, 0x44, 0x43, 0x2b, 0xa4,
                0x83, 0xb3, 0x6c, 0xd2, 0xdd, 0x1f, 0x36, 0x18, 0x14, 0x10, 0xac, 0xc7, 0x14, 0x93,
                0x9c, 0x0c, 0xb5, 0x6a,
            ],
            Elements::FeNormalize => [
                0xc0, 0x70, 0xad, 0xba, 0xab, 0x2c, 0x7b, 0xe6, 0xff, 0x57, 0x7a, 0x75, 0x07, 0xaf,
                0xf0, 0xe7, 0x76, 0x57, 0xf3, 0x09, 0xe6, 0x5d, 0x05, 0xfa, 0x23, 0xc1, 0x92, 0x76,
                0xf7, 0x38, 0x52, 0xeb,
            ],
            Elements::FeSquare => [
                0x8e, 0x77, 0xcc, 0x8c, 0x63, 0x69, 0x3a, 0x2a, 0xcd, 0x9a, 0x65, 0x26, 0x6a, 0x02,
                0x89, 0x06, 0xf8, 0x64, 0x21, 0x4a, 0xf6, 0x6b, 0xa5, 0x4c, 0xce, 0x11, 0xac, 0xb0,
                0x37, 0xc9, 0x43, 0x93,
            ],
            Elements::FeSquareRoot => [
                0xf7, 0x71, 0x81, 0x03, 0x30, 0x4c, 0xb4, 0x36, 0x96, 0xbd, 0xf9, 0x2f, 0x61, 0x4c,
                0x83, 0x8d, 0x24, 0xd7, 0xdd, 0x7b, 0xa8, 0xdc, 0x01, 0xab, 0x5c, 0x6a, 0x77, 0x26,
                0x3c, 0x15, 0xf7, 0x29,
            ],
            Elements::FullAdd16 => [
                0xf8, 0xe1, 0x52, 0xa9, 0x0a, 0xd9, 0x2b, 0x82, 0x05, 0xc5, 0x56, 0x98, 0x21, 0xef,
                0x09, 0x79, 0x90, 0xd9, 0xf7, 0xf9, 0x8f, 0x21, 0xca, 0xa8, 0xd9, 0xad, 0x55, 0xb5,
                0xed, 0xbc, 0xb6, 0x4d,
            ],
            Elements::FullAdd32 => [
                0x47, 0x27, 0x36, 0x1e, 0xa0, 0x03, 0xc1, 0xa4, 0x83, 0xe5, 0x75, 0x05, 0xcf, 0x5b,
                0x40, 0x5a, 0x82, 0x27, 0xda, 0x1a, 0xdd, 0xc4, 0x7e, 0x2b, 0x01, 0x6c, 0x2d, 0x09,
                0xbe, 0x04, 0x7f, 0xe8,
            ],
            Elements::FullAdd64 => [
                0x8e, 0x29, 0x53, 0x1e, 0xa3, 0x0a, 0x34, 0xc0, 0x72, 0x97, 0x86, 0x07, 0x7f, 0x5d,
                0xc3, 0x79, 0xcb, 0x22, 0x45, 0xfd, 0xe4, 0x41, 0xdb, 0x16, 0xa6, 0x85, 0x6e, 0x26,
                0x80, 0x3d, 0x26, 0xb9,
            ],
            Elements::FullAdd8 => [
                0x3f, 0xd9, 0x59, 0x05, 0x5a, 0x7b, 0x6f, 0x51, 0xb1, 0x47, 0x44, 0x75, 0x37, 0x66,
                0x02, 0x8f, 0x51, 0x83, 0x1b, 0xbd, 0x7c, 0xf7, 0x0e, 0x1e, 0xcc, 0x70, 0x06, 0x0d,
                0xfc, 0xcf, 0xb6, 0x4c,
            ],
            Elements::FullDecrement16 => [
                0xb8, 0x35, 0xbc, 0xde, 0x81, 0x20, 0x7b, 0x8a, 0x1f, 0x68, 0x6b, 0xb5, 0xad, 0x00,
                0x8d, 0xd7, 0xf2, 0x7f, 0xfb, 0xa2, 0xd9, 0x1f, 0xab, 0x34, 0x63, 0x52, 0xc4, 0x11,
                0x8a, 0x13, 0xba, 0x57,
            ],
            Elements::FullDecrement32 => [
                0x91, 0xde, 0x80, 0x21, 0xe3, 0xae, 0x86, 0x0b, 0xd8, 0x27, 0x04, 0xb2, 0xe5, 0x14,
                0x9d, 0xfe, 0x62, 0xb7, 0x4e, 0x1a, 0x6c, 0x71, 0xdb, 0xf0, 0xcb, 0xea, 0xbb, 0x7b,
                0x9e, 0xb2, 0x6a, 0x2b,
            ],
            Elements::FullDecrement64 => [
                0x9e, 0x85, 0xc0, 0x20, 0xcd, 0x4d, 0xbb, 0x7a, 0x5d, 0xfb, 0x69, 0x7a, 0x44, 0x4c,
                0x06, 0x74, 0x8e, 0x22, 0x87, 0xae, 0x22, 0x37, 0x81, 0x3b, 0x1c, 0x5f, 0xa4, 0xa5,
                0x95, 0x8e, 0x6f, 0x1b,
            ],
            Elements::FullDecrement8 => [
                0x16, 0xdb, 0x3f, 0x6d, 0x60, 0x02, 0xd0, 0x70, 0x51, 0xff, 0xf6, 0xf4, 0x94, 0x04,
                0x66, 0x61, 0xf0, 0x38, 0x79, 0xd6, 0x36, 0x54, 0x67, 0xfc, 0xaa, 0xf9, 0xab, 0xab,
                0xa0, 0xc7, 0xf2, 0x81,
            ],
            Elements::FullIncrement16 => [
                0x08, 0xf3, 0xe2, 0x48, 0x41, 0xfe, 0x94, 0x8d, 0x15, 0x29, 0x76, 0x52, 0x0f, 0x32,
                0xdd, 0x12, 0xf7, 0x5c, 0x38, 0x2a, 0xb9, 0xf5, 0x96, 0x6a, 0x16, 0x2c, 0x05, 0x06,
                0xe2, 0xb3, 0x7b, 0x55,
            ],
            Elements::FullIncrement32 => [
                0xff, 0x69, 0xcd, 0xe4, 0x86, 0xc1, 0x91, 0xf4, 0x26, 0x60, 0x7c, 0xaa, 0xdb, 0x45,
                0xaa, 0x4b, 0x82, 0xd2, 0x1f, 0xc3, 0x5f, 0x99, 0xba, 0x0d, 0x7f, 0x56, 0xa8, 0x89,
                0x4c, 0x24, 0xa2, 0x6c,
            ],
            Elements::FullIncrement64 => [
                0x4f, 0x08, 0x0c, 0x7a, 0xb1, 0x90, 0xbb, 0x41, 0xc2, 0x18, 0x8a, 0x37, 0xa4, 0xb3,
                0xb5, 0x2d, 0x6f, 0x75, 0x66, 0x86, 0x1f, 0x5b, 0x46, 0xa9, 0x80, 0x2d, 0xbe, 0xb1,
                0x42, 0x35, 0x0a, 0xe0,
            ],
            Elements::FullIncrement8 => [
                0xba, 0x53, 0x0e, 0x83, 0x49, 0xb1, 0x3d, 0x90, 0x27, 0xff, 0x14, 0x16, 0xa4, 0x12,
                0x12, 0x1f, 0x9b, 0x6b, 0x5e, 0x3c, 0x62, 0xea, 0xa1, 0x18, 0x2a, 0xd2, 0x44, 0xb0,
                0xf7, 0x0f, 0x12, 0xc5,
            ],
            Elements::FullMultiply16 => [
                0x78, 0x3d, 0x05, 0xcf, 0xb8, 0x4d, 0x0c, 0x91, 0x76, 0x60, 0x38, 0x26, 0xc1, 0x6c,
                0x5c, 0xf7, 0x30, 0xd0, 0x1c, 0x50, 0xa1, 0x69, 0x29, 0x79, 0xf7, 0x92, 0xe0, 0x98,
                0xf6, 0x99, 0x80, 0x12,
            ],
            Elements::FullMultiply32 => [
                0xaa, 0xc2, 0x53, 0x61, 0xe5, 0x98, 0xe3, 0x54, 0x38, 0xb9, 0x18, 0xb5, 0x8f, 0xd2,
                0xce, 0xf4, 0xdb, 0x3c, 0x5d, 0x8c, 0x5e, 0x63, 0xaa, 0x4f, 0x25, 0xe9, 0xce, 0xc0,
                0xcf, 0xd9, 0xdf, 0xb1,
            ],
            Elements::FullMultiply64 => [
                0x4e, 0x7b, 0xe1, 0x50, 0x1c, 0xda, 0x15, 0xae, 0x5c, 0xfb, 0x21, 0xee, 0xec, 0xe0,
                0x66, 0xe1, 0x02, 0x0e, 0x0f, 0xd9, 0xb3, 0xe8, 0x36, 0x5b, 0x68, 0x14, 0x02, 0x70,
                0xf9, 0xbc, 0x69, 0x47,
            ],
            Elements::FullMultiply8 => [
                0xd5, 0xe4, 0xc5, 0x6d, 0xe7, 0x83, 0x94, 0x0d, 0xd6, 0xb6, 0xec, 0xe1, 0x3e, 0xa5,
                0x8e, 0xf7, 0x9e, 0x1b, 0x70, 0x84, 0xba, 0x2c, 0x0c, 0xc5, 0x71, 0x20, 0xb7, 0xc5,
                0xc6, 0x2b, 0x3f, 0xdc,
            ],
            Elements::FullSubtract16 => [
                0xfc, 0x6d, 0x4c, 0xd4, 0xa9, 0x9d, 0xbc, 0x0e, 0x01, 0xa5, 0x1e, 0x3d, 0x98, 0x6d,
                0x6b, 0x04, 0x1c, 0x65, 0x57, 0xb8, 0xfc, 0x2e, 0x8b, 0x8c, 0x2f, 0xb6, 0xd2, 0x29,
                0x2a, 0x31, 0x12, 0x61,
            ],
            Elements::FullSubtract32 => [
                0x6d, 0x96, 0xf6, 0x8a, 0x94, 0x5c, 0x22, 0xe7, 0x62, 0x11, 0x5c, 0x09, 0x42, 0x97,
                0xb1, 0x94, 0xbe, 0xdc, 0x0c, 0xe5, 0xa0, 0xc9, 0x2d, 0xb6, 0x4b, 0x83, 0x0a, 0x18,
                0xb4, 0x4d, 0xf0, 0xd0,
            ],
            Elements::FullSubtract64 => [
                0xcd, 0x12, 0x1b, 0x7f, 0x43, 0x84, 0x0f, 0x96, 0xc4, 0x0a, 0xa4, 0x05, 0x6d, 0xef,
                0x80, 0xd1, 0x77, 0xb6, 0x19, 0xb2, 0x50, 0x5a, 0x94, 0x7a, 0x21, 0x47, 0x55, 0xfa,
                0x83, 0x2c, 0x7f, 0x0c,
            ],
            Elements::FullSubtract8 => [
                0x11, 0x62, 0x66, 0x81, 0x54, 0x66, 0xfe, 0x6e, 0x6d, 0x47, 0x57, 0xf6, 0xb8, 0xad,
                0xe3, 0x34, 0xbe, 0xcd, 0xa9, 0xdb, 0x8c, 0x4e, 0xd7, 0xe3, 0x28, 0x13, 0x1f, 0x36,
                0x98, 0x08, 0x67, 0xf2,
            ],
            Elements::GeIsOnCurve => [
                0xd9, 0xaa, 0x66, 0x06, 0x5c, 0xb0, 0xed, 0x2c, 0x71, 0x68, 0x60, 0x9d, 0xfd, 0x62,
                0xab, 0x64, 0x3a, 0xa8, 0x7c, 0x27, 0xe0, 0xdb, 0xbf, 0x96, 0xf2, 0x91, 0x45, 0x28,
                0xfe, 0xef, 0x52, 0xc5,
            ],
            Elements::GeNegate => [
                0xa4, 0x97, 0xe7, 0x1c, 0x40, 0x3c, 0x4c, 0xe2, 0xb7, 0x81, 0x89, 0x3c, 0xd6, 0x9a,
                0x52, 0x85, 0xea, 0x02, 0xd7, 0xb7, 0xfe, 0x8e, 0xdf, 0xac, 0xe7, 0x8a, 0x20, 0x5a,
                0xad, 0x2e, 0xc0, 0x33,
            ],
            Elements::GejAdd => [
                0x86, 0x0a, 0x61, 0x5b, 0xb2, 0x5d, 0x22, 0xc1, 0x0a, 0x04, 0x48, 0x72, 0xd1, 0xb8,
                0xfb, 0x04, 0x98, 0x25, 0x86, 0x86, 0x28, 0x81, 0x81, 0x9c, 0xed, 0x25, 0xd6, 0x75,
                0xdc, 0x4f, 0x7d, 0xfe,
            ],
            Elements::GejDouble => [
                0x73, 0x2c, 0x6a, 0xb2, 0x5e, 0xab, 0x89, 0x66, 0x8d, 0x0c, 0xe2, 0x1c, 0x5b, 0x36,
                0x50, 0x18, 0x83, 0xb8, 0xdb, 0x67, 0x86, 0x4f, 0xf3, 0x4f, 0x98, 0x1a, 0x56, 0x8e,
                0xb9, 0xc5, 0xe8, 0xf0,
            ],
            Elements::GejGeAdd => [
                0xd2, 0x89, 0x43, 0x14, 0xa4, 0x8e, 0xa3, 0xf7, 0x15, 0x88, 0x91, 0x07, 0x48, 0xb6,
                0x75, 0x3a, 0x9e, 0x53, 0x9f, 0xf2, 0xb3, 0x6d, 0x1b, 0xf0, 0x89, 0x74, 0x93, 0xdc,
                0x14, 0x0a, 0xa3, 0xce,
            ],
            Elements::GejGeAddEx => [
                0x86, 0x64, 0x35, 0x4f, 0x50, 0x56, 0x65, 0xc8, 0xbe, 0x4b, 0x3b, 0xc1, 0xfa, 0x08,
                0x6f, 0x4f, 0x02, 0xf3, 0xaf, 0x69, 0x1d, 0x20, 0x9e, 0x85, 0x7c, 0x6f, 0x61, 0x5b,
                0x0d, 0xc4, 0x80, 0x32,
            ],
            Elements::GejInfinity => [
                0x2d, 0x9d, 0x36, 0xb4, 0x6e, 0xad, 0x02, 0xdb, 0x63, 0xb5, 0x85, 0xdc, 0xa6, 0x7e,
                0x5e, 0x4d, 0xcb, 0x94, 0x05, 0x89, 0xbb, 0x13, 0x3c, 0x99, 0x10, 0x0d, 0x61, 0x7c,
                0x27, 0x12, 0x6e, 0x96,
            ],
            Elements::GejIsInfinity => [
                0xe1, 0x86, 0xf9, 0xbd, 0xbe, 0x49, 0x16, 0xc7, 0x2f, 0x6c, 0x3b, 0xc2, 0xad, 0xf3,
                0xe0, 0x47, 0x22, 0xef, 0x4c, 0xec, 0x29, 0x72, 0x53, 0xe3, 0xec, 0xaa, 0x4e, 0x4c,
                0xc5, 0x51, 0xef, 0x2c,
            ],
            Elements::GejIsOnCurve => [
                0xa8, 0xc8, 0x2e, 0x8b, 0x3a, 0x61, 0x99, 0xda, 0x25, 0xb2, 0xb1, 0x9c, 0xd1, 0x49,
                0xf9, 0xff, 0x4c, 0x3f, 0xdc, 0x0b, 0x00, 0xb2, 0x64, 0x80, 0xc0, 0x00, 0x65, 0x53,
                0xd4, 0x3c, 0x1f, 0x6e,
            ],
            Elements::GejNegate => [
                0x71, 0xee, 0xff, 0xb5, 0xb6, 0x37, 0xaf, 0x51, 0xc4, 0x97, 0x80, 0x02, 0xc2, 0x12,
                0xcd, 0xaf, 0x39, 0x6c, 0xf8, 0xef, 0xca, 0x33, 0xaa, 0xb0, 0xf8, 0x33, 0xf8, 0x1a,
                0x9f, 0xb6, 0xa9, 0x89,
            ],
            Elements::GejNormalize => [
                0xb4, 0x19, 0x98, 0xde, 0x56, 0x4e, 0xf6, 0x4f, 0x63, 0xa4, 0xc9, 0xfa, 0xd1, 0x39,
                0x50, 0x64, 0x83, 0x2e, 0x7d, 0x5c, 0x4c, 0x77, 0x1d, 0x18, 0x0f, 0xce, 0xd2, 0x8d,
                0x8a, 0x76, 0x5b, 0xd6,
            ],
            Elements::GejRescale => [
                0xfa, 0x70, 0xaa, 0x15, 0x3d, 0xab, 0x6b, 0xc9, 0x93, 0x55, 0xc1, 0x0c, 0x61, 0xe5,
                0xbf, 0xcf, 0xa3, 0x97, 0xb3, 0x81, 0xf7, 0xef, 0x59, 0x15, 0x9d, 0x83, 0x79, 0x1a,
                0x2a, 0x6a, 0x58, 0x24,
            ],
            Elements::GejXEquiv => [
                0x65, 0xa8, 0x60, 0xfa, 0x7e, 0x74, 0x60, 0x1c, 0xb6, 0xd8, 0x37, 0x55, 0x3b, 0xa1,
                0x9d, 0x60, 0xc4, 0x77, 0x3c, 0x1c, 0x12, 0xb4, 0xb0, 0xf0, 0x27, 0x8b, 0x45, 0xfb,
                0x23, 0xfc, 0xe9, 0x67,
            ],
            Elements::GejYIsOdd => [
                0xcf, 0x91, 0xc7, 0x1d, 0xa7, 0x13, 0x98, 0xec, 0x8c, 0x64, 0xfd, 0xbf, 0x8f, 0xdc,
                0x91, 0x2e, 0xd5, 0xa8, 0xc1, 0xfa, 0xc6, 0x56, 0x6e, 0x0b, 0x59, 0x13, 0xeb, 0xe9,
                0xc4, 0xb1, 0x06, 0x17,
            ],
            Elements::Generate => [
                0x31, 0x78, 0x42, 0xd3, 0xdd, 0x20, 0x4c, 0x31, 0xc5, 0xd8, 0x34, 0x86, 0x94, 0x0d,
                0x15, 0xbb, 0x6a, 0x05, 0x3e, 0x3d, 0x25, 0x61, 0xee, 0x13, 0x6e, 0xa9, 0x1e, 0x74,
                0x67, 0x74, 0x41, 0xae,
            ],
            Elements::GenesisBlockHash => [
                0x8a, 0x38, 0x03, 0xed, 0x96, 0xeb, 0xf9, 0x6c, 0x5b, 0xc9, 0x9d, 0x7d, 0xdc, 0x31,
                0x87, 0xd7, 0x94, 0x00, 0x41, 0x01, 0x90, 0x56, 0x41, 0x4f, 0x86, 0x96, 0x06, 0x1d,
                0x97, 0x79, 0x13, 0xc2,
            ],
            Elements::Increment16 => [
                0x89, 0xca, 0xc7, 0x0e, 0x2b, 0xf1, 0x4c, 0xd4, 0xd4, 0x32, 0x75, 0xa0, 0x13, 0x5a,
                0x9f, 0xab, 0xc0, 0xeb, 0x5a, 0x33, 0xf6, 0x2d, 0x0f, 0x4f, 0xa4, 0x4a, 0x3c, 0xa7,
                0x3e, 0xda, 0x85, 0x27,
            ],
            Elements::Increment32 => [
                0x44, 0xf6, 0x64, 0x2e, 0x7b, 0x8d, 0xe6, 0x98, 0x7b, 0x5f, 0x1e, 0x9e, 0x2f, 0x2e,
                0x28, 0x4a, 0x4c, 0xcb, 0xbc, 0x3c, 0x75, 0x5f, 0x23, 0x11, 0xc3, 0x4b, 0x50, 0xf9,
                0x4f, 0xa4, 0x48, 0xbe,
            ],
            Elements::Increment64 => [
                0xe0, 0xb2, 0x61, 0x72, 0x28, 0x67, 0x29, 0xf5, 0xcd, 0xaf, 0x25, 0x16, 0x18, 0xb9,
                0x9e, 0x8e, 0xab, 0x93, 0xd8, 0x4a, 0xb9, 0xba, 0x87, 0x03, 0x06, 0xe6, 0x4d, 0xbc,
                0x5e, 0x3e, 0x09, 0x3d,
            ],
            Elements::Increment8 => [
                0x71, 0xd9, 0x4f, 0xdb, 0x37, 0x95, 0x9c, 0x9a, 0x89, 0xcc, 0x1b, 0x79, 0x71, 0x2c,
                0xa1, 0x67, 0xda, 0xea, 0x47, 0xbe, 0xf8, 0x5f, 0x92, 0xd4, 0x06, 0x6b, 0x6e, 0x94,
                0xcc, 0x16, 0x16, 0xb7,
            ],
            Elements::InputAmount => [
                0x6e, 0x0a, 0x61, 0xfa, 0xae, 0xbc, 0x2c, 0xf4, 0x22, 0x0d, 0xea, 0x6c, 0xc5, 0x9c,
                0x46, 0xe8, 0x0c, 0x90, 0xc4, 0x8e, 0x9b, 0xd0, 0xe4, 0xe5, 0xf6, 0x3b, 0x04, 0xe1,
                0xec, 0xdb, 0xbf, 0xdf,
            ],
            Elements::InputAmountsHash => [
                0x1d, 0x82, 0x58, 0x40, 0x8c, 0x82, 0xe6, 0x53, 0x80, 0xea, 0xf3, 0x6f, 0x07, 0x30,
                0x2b, 0xbe, 0x09, 0xc7, 0x7e, 0x79, 0x34, 0x97, 0xf2, 0x10, 0x0f, 0x7a, 0xd4, 0x3e,
                0xed, 0x60, 0x14, 0x09,
            ],
            Elements::InputAnnexHash => [
                0xde, 0x1a, 0x81, 0x80, 0xbc, 0x9b, 0x81, 0x41, 0x4a, 0x86, 0xd3, 0x12, 0x67, 0x1f,
                0x6a, 0x57, 0x13, 0x2f, 0xec, 0x6b, 0xbc, 0xd4, 0x3f, 0x9f, 0xf5, 0xc0, 0x77, 0x37,
                0xea, 0xbe, 0x59, 0x5f,
            ],
            Elements::InputAnnexesHash => [
                0x86, 0xf9, 0xee, 0xda, 0xa3, 0xfc, 0xcf, 0x6a, 0xa4, 0xa5, 0x52, 0xf8, 0x7e, 0xe3,
                0x13, 0x6c, 0xc0, 0x92, 0x31, 0x45, 0x60, 0xd8, 0xed, 0xcc, 0xcd, 0x7f, 0x7a, 0x5e,
                0xaf, 0x9e, 0x3f, 0x98,
            ],
            Elements::InputAsset => [
                0x7d, 0xa2, 0x46, 0xf6, 0xf7, 0xe0, 0x83, 0x17, 0x2e, 0xb7, 0xc7, 0x0d, 0x81, 0xba,
                0xe5, 0x57, 0x2f, 0x2c, 0x10, 0x26, 0x4d, 0x46, 0xf2, 0x5c, 0x68, 0x27, 0xe4, 0x42,
                0xe3, 0x66, 0xe0, 0x18,
            ],
            Elements::InputOutpointsHash => [
                0x36, 0x87, 0x79, 0x67, 0xb2, 0x55, 0x5e, 0x7b, 0xcc, 0x2a, 0xad, 0x10, 0x1f, 0x79,
                0xd5, 0x5d, 0x3b, 0x4e, 0xcf, 0x00, 0xde, 0xb4, 0x56, 0x46, 0x4c, 0x10, 0x9b, 0x55,
                0x79, 0xf4, 0xf9, 0xa9,
            ],
            Elements::InputPegin => [
                0xeb, 0x74, 0xdd, 0x1c, 0xb2, 0x54, 0xb3, 0x08, 0x6f, 0x5c, 0x8d, 0xfa, 0xfd, 0x01,
                0x74, 0x32, 0xf3, 0xc2, 0xe0, 0x30, 0x62, 0xd5, 0x27, 0xd3, 0x22, 0xd4, 0xab, 0xb0,
                0xad, 0x19, 0x56, 0xc3,
            ],
            Elements::InputPrevOutpoint => [
                0x09, 0x1b, 0x59, 0xfe, 0xb2, 0xcc, 0xb3, 0xb0, 0x0f, 0x81, 0x04, 0xca, 0xf7, 0xfa,
                0x16, 0x4d, 0x5f, 0xcb, 0xad, 0x32, 0xe4, 0xbc, 0x73, 0x88, 0x82, 0xc8, 0xcf, 0x55,
                0x10, 0x08, 0xdb, 0xb1,
            ],
            Elements::InputScriptHash => [
                0xe9, 0x6a, 0x7e, 0x33, 0xed, 0xc6, 0xab, 0xa6, 0xf7, 0x6a, 0xeb, 0x5a, 0xb0, 0xcf,
                0x4f, 0x79, 0x59, 0x7c, 0xf7, 0x09, 0x37, 0xb0, 0x96, 0x19, 0x0e, 0x01, 0x3c, 0x24,
                0x6f, 0x6e, 0xa5, 0xa3,
            ],
            Elements::InputScriptSigHash => [
                0xac, 0x6c, 0xf8, 0x52, 0x15, 0xeb, 0xb3, 0x56, 0x10, 0x23, 0x9c, 0x7f, 0xae, 0x9e,
                0x21, 0xd9, 0x8c, 0x47, 0xb5, 0x67, 0x54, 0x86, 0x39, 0x5f, 0xea, 0x9e, 0x89, 0x37,
                0xb4, 0xc0, 0x14, 0x38,
            ],
            Elements::InputScriptSigsHash => [
                0xea, 0x44, 0xc5, 0x50, 0x3e, 0x91, 0xe3, 0x2d, 0x4e, 0x5e, 0xc3, 0x58, 0xd4, 0xa4,
                0x19, 0x02, 0x5d, 0x4d, 0xc1, 0x03, 0x93, 0xe3, 0xc4, 0xd1, 0x03, 0x46, 0x88, 0xad,
                0xed, 0xd4, 0xdd, 0xbc,
            ],
            Elements::InputScriptsHash => [
                0xd8, 0x35, 0x74, 0x3f, 0xb6, 0x8c, 0x9a, 0x82, 0xb5, 0x2e, 0x57, 0x14, 0x40, 0x47,
                0xcf, 0x27, 0x0d, 0xe5, 0xcb, 0xb5, 0x20, 0xf6, 0x5c, 0xf5, 0x2f, 0xd0, 0x0c, 0x4f,
                0x79, 0xed, 0xba, 0x3d,
            ],
            Elements::InputSequence => [
                0x27, 0x43, 0x07, 0x36, 0xfe, 0xd8, 0x57, 0x46, 0x33, 0xa6, 0xde, 0x9a, 0x19, 0x8b,
                0x3c, 0x63, 0xff, 0xf8, 0x0f, 0x18, 0xd8, 0x5a, 0xef, 0x70, 0x6c, 0x64, 0x46, 0x9e,
                0x35, 0xc3, 0x99, 0x89,
            ],
            Elements::InputSequencesHash => [
                0x6c, 0x87, 0x61, 0xa1, 0xd3, 0x90, 0xad, 0x6a, 0xec, 0xb4, 0x46, 0xb1, 0x11, 0x08,
                0xd3, 0xfd, 0x84, 0x84, 0x4c, 0xe8, 0x8c, 0x02, 0x46, 0x2b, 0x16, 0x05, 0x13, 0x49,
                0x1b, 0x6e, 0xbe, 0xfa,
            ],
            Elements::InputUtxosHash => [
                0x82, 0xb4, 0xa8, 0x97, 0xfb, 0xf2, 0x61, 0x95, 0x90, 0x47, 0x02, 0x90, 0xba, 0xe5,
                0x30, 0xb4, 0x3f, 0xab, 0x5e, 0x0b, 0x63, 0xf7, 0xb5, 0xad, 0xdd, 0xb2, 0xab, 0x7d,
                0x2f, 0xc4, 0xec, 0xeb,
            ],
            Elements::InputsHash => [
                0xf0, 0x70, 0xa4, 0xa0, 0x5d, 0x10, 0x95, 0xa0, 0xc9, 0xbe, 0x49, 0xf4, 0xdc, 0xcc,
                0xd6, 0xe6, 0x26, 0xca, 0x3f, 0xd1, 0xf7, 0xc5, 0x1c, 0xad, 0x06, 0xbf, 0x8e, 0x31,
                0xff, 0x91, 0x4d, 0x76,
            ],
            Elements::InternalKey => [
                0xbd, 0x8e, 0x8c, 0x3a, 0xf9, 0xe2, 0x3c, 0xa1, 0x92, 0x52, 0x51, 0xd0, 0xc1, 0xbf,
                0xd8, 0x9b, 0x06, 0xe1, 0xe9, 0xf5, 0x23, 0x88, 0xdd, 0x64, 0x67, 0x87, 0xe1, 0x5b,
                0xe2, 0x16, 0xc7, 0x66,
            ],
            Elements::IsOne16 => [
                0x67, 0x31, 0xde, 0x96, 0x8d, 0x5c, 0xa6, 0xd1, 0x96, 0xd9, 0x0f, 0xb9, 0x3e, 0x23,
                0x2e, 0x20, 0x90, 0x1b, 0x49, 0x34, 0xfd, 0x17, 0xb7, 0x21, 0x14, 0x96, 0x92, 0xf8,
                0xfc, 0xa4, 0x3a, 0x4f,
            ],
            Elements::IsOne32 => [
                0xea, 0x1a, 0x8c, 0xe7, 0x2f, 0x57, 0xbe, 0x4e, 0x29, 0x11, 0xb9, 0x14, 0xe2, 0x06,
                0x47, 0xda, 0xdc, 0xec, 0x87, 0xad, 0x13, 0x5c, 0xdf, 0x64, 0x3b, 0x67, 0x7f, 0xc2,
                0x14, 0x9d, 0xd6, 0x65,
            ],
            Elements::IsOne64 => [
                0xf2, 0x8a, 0xe2, 0x7a, 0x4e, 0x4b, 0xeb, 0x42, 0xeb, 0x71, 0x9c, 0x5e, 0xae, 0xa4,
                0xc1, 0xaf, 0xac, 0x66, 0x8b, 0xdc, 0x08, 0x6a, 0x5f, 0x15, 0x4e, 0xb5, 0x79, 0x13,
                0x4f, 0xc2, 0x06, 0x42,
            ],
            Elements::IsOne8 => [
                0xf1, 0xa2, 0x81, 0xd5, 0x56, 0xbf, 0x41, 0xcd, 0xa8, 0x78, 0x6c, 0x04, 0x53, 0x0e,
                0x32, 0x19, 0xfd, 0x58, 0x2f, 0xba, 0x2d, 0xff, 0x4e, 0x99, 0xbd, 0x27, 0x5d, 0x7f,
                0xf7, 0x9a, 0x45, 0x0f,
            ],
            Elements::IsZero16 => [
                0x58, 0x22, 0x9c, 0x26, 0x64, 0x15, 0xd1, 0x05, 0xcf, 0x22, 0xb4, 0x2a, 0x9e, 0xde,
                0x72, 0x51, 0x48, 0xb3, 0xbc, 0x01, 0x76, 0x47, 0x5d, 0xe6, 0x2a, 0x74, 0x2d, 0x8e,
                0x11, 0xb5, 0x58, 0x39,
            ],
            Elements::IsZero32 => [
                0x3d, 0x03, 0xf7, 0x9a, 0xda, 0x94, 0x9a, 0x47, 0xc4, 0x6e, 0x4e, 0x97, 0x36, 0xb5,
                0x2d, 0x2a, 0xc1, 0x93, 0x57, 0xd6, 0xca, 0xe9, 0x71, 0x0d, 0xd6, 0xde, 0xd8, 0x2d,
                0x12, 0x0a, 0xa5, 0xb5,
            ],
            Elements::IsZero64 => [
                0xb4, 0x39, 0x4c, 0x5d, 0x5f, 0xd1, 0xdf, 0x3b, 0x6d, 0x02, 0x48, 0x95, 0x3f, 0xb2,
                0x61, 0x4c, 0x86, 0x7f, 0x96, 0xee, 0x4e, 0x0b, 0x05, 0x23, 0x0a, 0xe3, 0x52, 0x88,
                0x7f, 0x38, 0x32, 0x46,
            ],
            Elements::IsZero8 => [
                0xdc, 0x3a, 0xae, 0xa2, 0x21, 0x96, 0xcc, 0x94, 0x94, 0xce, 0xe4, 0x24, 0xdc, 0x71,
                0x60, 0xad, 0x52, 0x8c, 0x62, 0xd7, 0x4d, 0xdf, 0x99, 0xd2, 0xdb, 0x9e, 0x7b, 0x5e,
                0x7e, 0xd2, 0xa0, 0x1b,
            ],
            Elements::Issuance => [
                0x3a, 0xa8, 0x19, 0xcf, 0x43, 0xf1, 0x10, 0x61, 0xde, 0x93, 0xac, 0xc5, 0x6f, 0x5d,
                0x20, 0xc3, 0x97, 0xbc, 0xd9, 0x8d, 0xdb, 0xef, 0xcf, 0x19, 0x88, 0xa2, 0x06, 0x02,
                0x33, 0xe6, 0x4c, 0xbb,
            ],
            Elements::IssuanceAsset => [
                0x66, 0x7b, 0x65, 0x49, 0xeb, 0xda, 0x20, 0xad, 0x18, 0x2b, 0x03, 0x7a, 0xa3, 0xf9,
                0x14, 0x11, 0x2b, 0x80, 0x63, 0x54, 0xa5, 0xa2, 0x80, 0xb7, 0xe8, 0x41, 0x32, 0x1b,
                0xd2, 0xfb, 0x81, 0x75,
            ],
            Elements::IssuanceAssetAmount => [
                0x16, 0x96, 0x40, 0xb2, 0x49, 0xc8, 0xf9, 0x44, 0xd5, 0x08, 0x6f, 0x39, 0xcb, 0xfc,
                0x80, 0xda, 0x2b, 0x02, 0xd9, 0x04, 0x1c, 0x52, 0x05, 0xf6, 0x15, 0x6d, 0x5d, 0xee,
                0xd8, 0x47, 0xba, 0xae,
            ],
            Elements::IssuanceAssetAmountsHash => [
                0x38, 0xc9, 0xfb, 0xd2, 0xa3, 0x78, 0xd4, 0xe9, 0x96, 0x96, 0xef, 0xd8, 0xcd, 0x58,
                0xe9, 0xf0, 0x4b, 0xee, 0x1a, 0x5b, 0x1b, 0x56, 0xe9, 0xa2, 0xc5, 0xb0, 0x9a, 0x09,
                0x6d, 0xa5, 0xaf, 0x9d,
            ],
            Elements::IssuanceAssetProof => [
                0x41, 0xb3, 0x37, 0xce, 0x07, 0xad, 0x73, 0xc0, 0x6a, 0x29, 0xf0, 0xc6, 0xa0, 0xfd,
                0xd0, 0x8a, 0x14, 0xf3, 0x33, 0xfd, 0x52, 0x33, 0xaf, 0x84, 0x94, 0xb0, 0x27, 0x3c,
                0x13, 0x01, 0x40, 0xb9,
            ],
            Elements::IssuanceBlindingEntropyHash => [
                0xca, 0xbe, 0x95, 0xaf, 0x5f, 0xa8, 0x78, 0x64, 0x6d, 0xc8, 0xf8, 0x7e, 0xcb, 0xe8,
                0xc4, 0x22, 0x36, 0x99, 0x32, 0x80, 0xd8, 0xa4, 0x99, 0x36, 0xea, 0x1a, 0x88, 0x15,
                0x98, 0x5a, 0x0d, 0x19,
            ],
            Elements::IssuanceEntropy => [
                0x08, 0x5c, 0xa1, 0x03, 0x54, 0xa3, 0xd3, 0xdb, 0xf0, 0x94, 0x19, 0xf7, 0xba, 0x57,
                0x7a, 0xda, 0xf0, 0xec, 0x6c, 0x0b, 0xa2, 0xad, 0x7f, 0xa9, 0x2e, 0x75, 0x24, 0xb7,
                0xda, 0x71, 0xa4, 0x08,
            ],
            Elements::IssuanceRangeProofsHash => [
                0x4a, 0x05, 0x1c, 0xea, 0x1c, 0xd8, 0x88, 0x3d, 0x61, 0xec, 0x6f, 0xba, 0x24, 0x60,
                0xa6, 0x0a, 0xd7, 0xb9, 0x4f, 0x11, 0x0a, 0xc6, 0x59, 0xc1, 0xdb, 0x43, 0xb4, 0x5f,
                0x0c, 0xbe, 0xe2, 0xac,
            ],
            Elements::IssuanceToken => [
                0x2b, 0xdc, 0xff, 0x7b, 0x60, 0x80, 0xf8, 0x09, 0xab, 0x59, 0xb9, 0xb5, 0xe7, 0x58,
                0x5d, 0x1a, 0xc5, 0xa0, 0xc5, 0xc3, 0xe1, 0xd4, 0xbd, 0x33, 0x75, 0xfa, 0xd7, 0x97,
                0x06, 0x3a, 0xae, 0xac,
            ],
            Elements::IssuanceTokenAmount => [
                0x16, 0x13, 0x37, 0x04, 0x12, 0x50, 0x4a, 0x7e, 0xb9, 0x70, 0x15, 0x37, 0x78, 0x34,
                0xf9, 0xd6, 0x54, 0xda, 0x88, 0x2d, 0x6f, 0x79, 0xda, 0x05, 0x65, 0x8a, 0x89, 0xce,
                0xed, 0x36, 0x04, 0x62,
            ],
            Elements::IssuanceTokenAmountsHash => [
                0xc8, 0x29, 0xca, 0x6b, 0xa8, 0xb9, 0x87, 0xfe, 0xd7, 0xee, 0xf4, 0xfe, 0x08, 0x76,
                0xe7, 0x13, 0x4e, 0x09, 0xdc, 0x3c, 0x35, 0xfa, 0x65, 0x72, 0x42, 0x34, 0xb7, 0x0f,
                0x0d, 0x64, 0xea, 0xae,
            ],
            Elements::IssuanceTokenProof => [
                0xdf, 0x95, 0x35, 0xf1, 0x65, 0x23, 0xaa, 0xca, 0xa7, 0x60, 0x59, 0x9f, 0x2e, 0xd8,
                0xee, 0x4a, 0xbf, 0x16, 0xce, 0x97, 0xfd, 0x2d, 0xcb, 0x9c, 0xdd, 0x4f, 0xeb, 0x53,
                0x89, 0x74, 0x8a, 0x67,
            ],
            Elements::IssuancesHash => [
                0xd2, 0xe1, 0x89, 0xaa, 0xc5, 0x9d, 0x16, 0xc8, 0x30, 0xb4, 0x0c, 0x5d, 0x6a, 0x5f,
                0xab, 0x32, 0xe8, 0x62, 0x92, 0xb6, 0x78, 0x58, 0xef, 0xa2, 0x8c, 0x82, 0xf9, 0x73,
                0x27, 0x3c, 0x08, 0x69,
            ],
            Elements::Le16 => [
                0x17, 0x43, 0xd1, 0xbc, 0x39, 0x38, 0xed, 0xc8, 0xdc, 0x65, 0x78, 0xa3, 0x52, 0xc4,
                0xf3, 0xec, 0x1f, 0x82, 0xea, 0xae, 0xfe, 0xa9, 0x33, 0x61, 0x89, 0xb9, 0xe2, 0x5c,
                0x3f, 0x13, 0x29, 0xf4,
            ],
            Elements::Le32 => [
                0xea, 0x77, 0x2c, 0x0b, 0x5a, 0xec, 0xde, 0x7d, 0xc1, 0x6e, 0x3f, 0x1f, 0x95, 0x27,
                0x89, 0xf0, 0x13, 0x70, 0x89, 0x04, 0xce, 0xaa, 0x62, 0xcf, 0xdc, 0xf6, 0x4d, 0x30,
                0xa3, 0x91, 0x17, 0xaf,
            ],
            Elements::Le64 => [
                0x37, 0xb6, 0x54, 0xea, 0xd2, 0x33, 0xc3, 0xef, 0xb8, 0x0b, 0x4f, 0x88, 0xa6, 0x13,
                0xba, 0x70, 0x24, 0xd9, 0x8e, 0xb3, 0x1a, 0x2c, 0x3e, 0xdc, 0x93, 0xb5, 0x1a, 0xa4,
                0x60, 0xbc, 0xf7, 0x83,
            ],
            Elements::Le8 => [
                0xff, 0xdf, 0x7f, 0x6a, 0x8d, 0x3b, 0x2c, 0x1b, 0x06, 0xbd, 0x86, 0x40, 0xbd, 0x98,
                0xda, 0xa8, 0x9d, 0x2f, 0xcc, 0xa3, 0xa8, 0xb0, 0xc5, 0x70, 0x43, 0xea, 0xc2, 0x9a,
                0x05, 0xe7, 0xf2, 0xd3,
            ],
            Elements::LinearCombination1 => [
                0x35, 0xcb, 0xfa, 0x56, 0x1b, 0x44, 0xf4, 0x48, 0xdc, 0x84, 0x3a, 0xa6, 0x05, 0x2a,
                0xdd, 0x1a, 0x4f, 0xf6, 0xfb, 0x56, 0xd6, 0x86, 0xc3, 0x21, 0xc5, 0xf8, 0x3b, 0x55,
                0x25, 0xef, 0xe1, 0x8b,
            ],
            Elements::LinearVerify1 => [
                0x34, 0xae, 0x24, 0x21, 0x03, 0xad, 0x8c, 0xf1, 0x09, 0x82, 0x44, 0x44, 0x13, 0xdd,
                0x0d, 0xcb, 0x57, 0x77, 0x48, 0x67, 0x90, 0xdd, 0x21, 0x98, 0xeb, 0xbb, 0xe6, 0x4a,
                0x5a, 0xef, 0x0a, 0x35,
            ],
            Elements::LockTime => [
                0x67, 0x3a, 0x98, 0x79, 0xc3, 0x37, 0xf7, 0x69, 0x63, 0xbb, 0xb6, 0xcd, 0x9a, 0x42,
                0x4e, 0x69, 0x24, 0x37, 0xc3, 0xb0, 0x2e, 0x8d, 0xb0, 0x12, 0xa2, 0x03, 0x12, 0x5b,
                0xb9, 0x65, 0xe2, 0x07,
            ],
            Elements::Low16 => [
                0x11, 0xd9, 0xe7, 0xcd, 0x36, 0x81, 0x3e, 0x73, 0xa9, 0x47, 0x92, 0xcf, 0xae, 0x99,
                0x63, 0xef, 0x4f, 0x64, 0x64, 0x8d, 0xf4, 0x8a, 0xac, 0xff, 0xe5, 0x7f, 0xb7, 0x0f,
                0x4e, 0x19, 0xac, 0xac,
            ],
            Elements::Low32 => [
                0x96, 0xbe, 0x53, 0x34, 0x5d, 0x52, 0x14, 0xb0, 0x05, 0xc7, 0xfc, 0x5d, 0xe4, 0x0a,
                0x92, 0x62, 0x56, 0x60, 0x41, 0x40, 0x63, 0x35, 0x27, 0xc1, 0xd0, 0x2b, 0xe0, 0xd5,
                0xf6, 0x0c, 0xc2, 0xad,
            ],
            Elements::Low64 => [
                0x61, 0x2a, 0x86, 0xb8, 0x22, 0x44, 0x94, 0xd8, 0x23, 0x0e, 0xe1, 0x16, 0xba, 0x47,
                0x89, 0x46, 0x4c, 0x57, 0xa7, 0xe0, 0x58, 0x52, 0x95, 0xb6, 0x1e, 0xc5, 0xc9, 0x6f,
                0x59, 0x83, 0x77, 0x60,
            ],
            Elements::Low8 => [
                0x43, 0x95, 0xde, 0x24, 0xfd, 0x70, 0xf5, 0xc8, 0x62, 0xfe, 0xe6, 0x8c, 0x6f, 0x67,
                0xd6, 0x17, 0x12, 0xd8, 0xd6, 0x00, 0x3a, 0x38, 0xe1, 0xbd, 0x85, 0xbe, 0x4b, 0x94,
                0x52, 0x69, 0x92, 0xba,
            ],
            Elements::Lt16 => [
                0xf9, 0xd9, 0xee, 0xbe, 0xc9, 0xc2, 0x3b, 0x0b, 0x9e, 0xb8, 0xfd, 0x72, 0x2a, 0x31,
                0x92, 0xcf, 0x7d, 0x28, 0x7d, 0xaa, 0xcf, 0xa5, 0x3e, 0x5d, 0xdc, 0xe7, 0x1b, 0x26,
                0x92, 0x94, 0x0f, 0x1c,
            ],
            Elements::Lt32 => [
                0xc5, 0x23, 0x3b, 0x33, 0x0d, 0xec, 0x92, 0x06, 0x5e, 0x9e, 0xd3, 0x2d, 0x20, 0xd4,
                0xe0, 0xcf, 0xfd, 0x97, 0x56, 0xef, 0x11, 0xc8, 0x43, 0x86, 0x9f, 0xa7, 0x54, 0x37,
                0xef, 0x00, 0xda, 0xaf,
            ],
            Elements::Lt64 => [
                0x3b, 0x8c, 0xbf, 0xda, 0x60, 0x38, 0xef, 0x26, 0x95, 0x06, 0x03, 0xdd, 0xd8, 0x61,
                0x9c, 0x19, 0x7f, 0x8d, 0xbe, 0x4e, 0x15, 0x4c, 0x15, 0x0b, 0x93, 0xd3, 0x8d, 0x0a,
                0xb9, 0x68, 0xf7, 0x67,
            ],
            Elements::Lt8 => [
                0x4e, 0xe0, 0x55, 0x24, 0x23, 0xc4, 0x59, 0x57, 0x5f, 0xca, 0x1b, 0x39, 0xfc, 0xdc,
                0xf0, 0x56, 0x6f, 0xa8, 0x82, 0x1f, 0x14, 0x0c, 0x0a, 0x3c, 0xa4, 0x71, 0x35, 0xbb,
                0x3c, 0xd6, 0xa1, 0xf9,
            ],
            Elements::Max16 => [
                0x13, 0xc1, 0xad, 0xf5, 0x49, 0xc7, 0x73, 0x0b, 0x87, 0x43, 0xf8, 0x95, 0x06, 0x4f,
                0xa4, 0xab, 0xd7, 0x79, 0xf4, 0xb4, 0x1d, 0x6d, 0xf7, 0xc6, 0x33, 0x08, 0x71, 0xa3,
                0x7a, 0xa7, 0x8c, 0xd7,
            ],
            Elements::Max32 => [
                0xc7, 0xf9, 0xaa, 0x5b, 0x7e, 0x5e, 0x03, 0xeb, 0xad, 0x3b, 0xec, 0x46, 0xe7, 0x73,
                0x45, 0x9f, 0xf8, 0x6a, 0x73, 0x9f, 0x9a, 0x09, 0x01, 0x9e, 0xcc, 0x0b, 0x6c, 0x3c,
                0xdf, 0xf4, 0x1a, 0x7b,
            ],
            Elements::Max64 => [
                0x12, 0x7d, 0xd5, 0x9e, 0x06, 0xc5, 0xd6, 0x93, 0x04, 0xeb, 0x92, 0xcf, 0xd6, 0xb4,
                0xf3, 0x65, 0xb7, 0xc2, 0xcd, 0x62, 0x5e, 0x90, 0x45, 0xbe, 0xde, 0xab, 0xf2, 0x31,
                0xc0, 0x14, 0xe8, 0x60,
            ],
            Elements::Max8 => [
                0xaf, 0xe3, 0xc6, 0x8d, 0x17, 0x39, 0x5d, 0xea, 0x61, 0x8e, 0x15, 0xbd, 0x34, 0xae,
                0xad, 0xb8, 0xcb, 0xfb, 0xc8, 0x56, 0x00, 0x71, 0xfb, 0xd1, 0x61, 0x4d, 0xa8, 0xb1,
                0x3f, 0xdf, 0x5d, 0xdd,
            ],
            Elements::Median16 => [
                0xa1, 0xa4, 0x1f, 0x4a, 0x60, 0x17, 0x94, 0x89, 0x9a, 0xa0, 0x50, 0x75, 0x66, 0x3e,
                0x6a, 0xfd, 0xe4, 0xb2, 0x13, 0x82, 0xb2, 0x77, 0x8d, 0xaf, 0x8f, 0x5c, 0x26, 0xeb,
                0xb3, 0xd3, 0xab, 0x53,
            ],
            Elements::Median32 => [
                0xc5, 0xfe, 0x19, 0x50, 0x41, 0x84, 0xdc, 0xa2, 0xd3, 0xe8, 0xd0, 0x86, 0xf2, 0x11,
                0xe6, 0xab, 0x5f, 0x44, 0x6d, 0x6a, 0xcf, 0xc1, 0xe8, 0xfb, 0xa8, 0x12, 0x75, 0x90,
                0xd8, 0xcf, 0xc0, 0x4e,
            ],
            Elements::Median64 => [
                0xf5, 0x32, 0x56, 0x7c, 0x74, 0x17, 0xcb, 0x0d, 0x19, 0xe4, 0x18, 0xd7, 0x2b, 0x31,
                0x9a, 0xf8, 0xa1, 0xce, 0x3c, 0xca, 0x90, 0x3b, 0xa2, 0x6d, 0x3b, 0x8f, 0x98, 0xf2,
                0xd8, 0x31, 0xa6, 0x1d,
            ],
            Elements::Median8 => [
                0x1a, 0x64, 0xb1, 0x35, 0xf4, 0xb3, 0xa8, 0x04, 0xdb, 0xc1, 0x2a, 0x30, 0x59, 0xd9,
                0x54, 0xa1, 0x8c, 0xfb, 0x7f, 0x0b, 0x9f, 0xc5, 0x2e, 0xb9, 0x5f, 0x4a, 0x1c, 0x30,
                0xef, 0x48, 0x6c, 0xad,
            ],
            Elements::Min16 => [
                0x26, 0xc0, 0x2d, 0x7c, 0xff, 0xb8, 0x84, 0xad, 0x54, 0x92, 0x88, 0x2b, 0x4b, 0x96,
                0x62, 0x8e, 0x07, 0x53, 0xeb, 0xa9, 0xf5, 0x13, 0x7f, 0x13, 0x49, 0xe9, 0x43, 0xaf,
                0x70, 0x3a, 0xd4, 0x39,
            ],
            Elements::Min32 => [
                0xc6, 0x18, 0xd5, 0x77, 0xa5, 0xbd, 0x0c, 0xe3, 0xeb, 0x3d, 0xbb, 0xe5, 0x1b, 0xad,
                0x5c, 0x9f, 0x9f, 0x10, 0xce, 0xc4, 0x70, 0x59, 0xcb, 0xb5, 0x82, 0x0f, 0x8a, 0xba,
                0x05, 0x47, 0xa5, 0xca,
            ],
            Elements::Min64 => [
                0xdb, 0x6d, 0xe6, 0x99, 0x42, 0x80, 0x6c, 0xfe, 0xbf, 0x21, 0xae, 0x01, 0x80, 0x67,
                0x3e, 0xd3, 0x72, 0x93, 0x42, 0xc0, 0x53, 0x48, 0x5b, 0x9b, 0x7e, 0xa7, 0xae, 0xaa,
                0x51, 0xb6, 0xbf, 0xd7,
            ],
            Elements::Min8 => [
                0x25, 0xc8, 0xdc, 0x61, 0xf7, 0x3e, 0xfa, 0xe8, 0xd7, 0xdf, 0x91, 0x70, 0xc5, 0xf0,
                0xcb, 0xb9, 0xf7, 0x62, 0x65, 0xa3, 0xad, 0x95, 0xcc, 0x5c, 0x27, 0xe7, 0x28, 0x74,
                0x05, 0xe0, 0x6d, 0x8c,
            ],
            Elements::Modulo16 => [
                0x25, 0x22, 0x08, 0xa3, 0xe2, 0x82, 0x5d, 0x78, 0xa9, 0x49, 0x5d, 0x81, 0x02, 0x5b,
                0x83, 0x99, 0x08, 0xaa, 0x70, 0x54, 0xe9, 0x0b, 0x0d, 0x2d, 0xdb, 0xbb, 0x0c, 0x8d,
                0x84, 0xe5, 0xc8, 0x37,
            ],
            Elements::Modulo32 => [
                0x1a, 0xa0, 0xf2, 0xed, 0x81, 0xe6, 0xaf, 0x95, 0x8d, 0x1a, 0x3b, 0x72, 0xa6, 0xda,
                0x7f, 0x17, 0x38, 0x53, 0x63, 0xf2, 0xd2, 0xbd, 0x20, 0xd2, 0x93, 0x50, 0x6c, 0x0a,
                0x18, 0x58, 0x28, 0xfb,
            ],
            Elements::Modulo64 => [
                0x00, 0xd4, 0x46, 0x3d, 0xec, 0x3c, 0x73, 0x0d, 0x7f, 0xed, 0x1a, 0x23, 0xc6, 0x6b,
                0xc3, 0xf4, 0xa2, 0xc7, 0x16, 0xa0, 0xce, 0x8e, 0x5d, 0x80, 0x50, 0x5b, 0x28, 0xef,
                0xb9, 0x3d, 0xbc, 0x8b,
            ],
            Elements::Modulo8 => [
                0x57, 0x8a, 0xe6, 0x7d, 0xa0, 0x09, 0xf3, 0xfe, 0xc1, 0x0f, 0x45, 0x64, 0x40, 0xaa,
                0xcd, 0x54, 0x16, 0x0a, 0x16, 0xf4, 0x47, 0x5f, 0x05, 0x8f, 0xb3, 0x4b, 0xd2, 0xd3,
                0x75, 0x10, 0xc9, 0x56,
            ],
            Elements::Multiply16 => [
                0x3c, 0xd8, 0x35, 0x22, 0x72, 0xaa, 0xa2, 0xc5, 0x49, 0x65, 0xf0, 0xe8, 0x66, 0xa6,
                0xe0, 0x81, 0xb2, 0x09, 0x84, 0x8c, 0x3b, 0x0a, 0x90, 0x6e, 0xcf, 0x02, 0x64, 0x15,
                0xa6, 0x53, 0x4a, 0xb3,
            ],
            Elements::Multiply32 => [
                0x16, 0x1f, 0xd0, 0x3a, 0x92, 0xc6, 0x21, 0xb3, 0x28, 0x98, 0x49, 0xff, 0x29, 0xad,
                0x81, 0x34, 0x99, 0xd6, 0x3e, 0xd9, 0x73, 0xdb, 0x0e, 0x97, 0x51, 0x78, 0x54, 0x21,
                0xf5, 0x68, 0xd1, 0x8f,
            ],
            Elements::Multiply64 => [
                0x22, 0x25, 0xf1, 0xaf, 0x99, 0xf5, 0x2f, 0xf9, 0x49, 0xea, 0x46, 0xb8, 0xf1, 0xce,
                0xf6, 0x2f, 0x68, 0xaa, 0x3a, 0x42, 0x60, 0x11, 0x2e, 0xc9, 0xcd, 0x74, 0xd7, 0x8e,
                0xbe, 0x1d, 0x15, 0xe7,
            ],
            Elements::Multiply8 => [
                0x80, 0x24, 0x3b, 0x83, 0x65, 0x8c, 0x33, 0x0c, 0x3b, 0xc1, 0x93, 0x9e, 0x45, 0x4f,
                0x53, 0xaa, 0x74, 0xdf, 0x6c, 0xf0, 0xa1, 0x9d, 0x1d, 0x67, 0xf2, 0x14, 0x6c, 0x9c,
                0xbe, 0xe9, 0x51, 0x30,
            ],
            Elements::Negate16 => [
                0x02, 0x69, 0xd7, 0x3f, 0x09, 0x4d, 0x59, 0x41, 0x19, 0x73, 0xea, 0xcd, 0xd5, 0xd3,
                0xd9, 0xe4, 0x7c, 0xab, 0xb8, 0x27, 0x7e, 0x6e, 0xf4, 0x11, 0x15, 0x7a, 0x44, 0x8a,
                0xe1, 0x25, 0x2d, 0x33,
            ],
            Elements::Negate32 => [
                0xf6, 0xb2, 0x1e, 0x3f, 0x59, 0x3e, 0xbd, 0x97, 0xec, 0x16, 0x1f, 0xd4, 0xf8, 0x54,
                0x43, 0xd3, 0x65, 0xc0, 0x23, 0x07, 0x5a, 0x22, 0xb6, 0x8c, 0xa2, 0x6c, 0xf6, 0xb7,
                0x8b, 0xab, 0x94, 0xb0,
            ],
            Elements::Negate64 => [
                0x2b, 0xf4, 0x8c, 0x08, 0xa5, 0x7f, 0x18, 0x36, 0xea, 0x2f, 0x57, 0x22, 0xc0, 0x79,
                0x59, 0xa7, 0xd6, 0x4c, 0xbe, 0xf7, 0x05, 0xdc, 0xc3, 0xca, 0xba, 0x3a, 0x90, 0x05,
                0x03, 0xd4, 0x89, 0x59,
            ],
            Elements::Negate8 => [
                0x25, 0x6d, 0x50, 0x79, 0x8b, 0x7f, 0xe9, 0xe7, 0xcb, 0x16, 0x96, 0xfb, 0x18, 0x83,
                0x19, 0x01, 0xef, 0x88, 0x95, 0x2c, 0xc4, 0x60, 0x6b, 0x68, 0x97, 0x4a, 0x79, 0xca,
                0x8a, 0x8a, 0x00, 0x44,
            ],
            Elements::NewIssuanceContract => [
                0x8d, 0x58, 0xd7, 0x7a, 0x39, 0xe4, 0xf1, 0x2d, 0x60, 0x2e, 0x9c, 0x01, 0x2b, 0x0a,
                0xcf, 0x2a, 0xb0, 0xd9, 0x1c, 0xbb, 0x1f, 0x12, 0x0d, 0x8c, 0x25, 0x4f, 0x97, 0x0c,
                0x3b, 0xe1, 0x55, 0xc9,
            ],
            Elements::NonceHash => [
                0x37, 0x68, 0x39, 0x43, 0x6b, 0x77, 0xf2, 0x68, 0xe7, 0xe7, 0x1d, 0xd6, 0x03, 0xc0,
                0x86, 0x0b, 0x73, 0x47, 0x84, 0x74, 0x6e, 0xce, 0x33, 0x0b, 0xba, 0xab, 0x53, 0x87,
                0x53, 0xb4, 0x11, 0x84,
            ],
            Elements::NumInputs => [
                0x6a, 0x05, 0x8e, 0x23, 0xaf, 0x90, 0x1f, 0x13, 0xe8, 0x02, 0xdb, 0x86, 0x3f, 0x2a,
                0x99, 0xa0, 0xf7, 0xff, 0x42, 0x3b, 0x32, 0x24, 0xad, 0x6e, 0xe0, 0xcb, 0xbe, 0xa3,
                0xe2, 0x72, 0xfa, 0xd2,
            ],
            Elements::NumOutputs => [
                0xba, 0x74, 0xcb, 0x98, 0x91, 0x6c, 0xe0, 0x84, 0xee, 0x95, 0x79, 0xdc, 0xb0, 0xca,
                0x66, 0xce, 0x53, 0x98, 0x24, 0xd3, 0x34, 0x1b, 0xa6, 0xf0, 0x3e, 0x4b, 0x05, 0x5a,
                0xac, 0x4e, 0x48, 0xe6,
            ],
            Elements::One16 => [
                0x8e, 0x67, 0xce, 0x64, 0xee, 0x18, 0xb6, 0x44, 0xc5, 0x5f, 0xd0, 0x7c, 0x71, 0x93,
                0x2c, 0xa6, 0xe0, 0x29, 0x0b, 0xab, 0xef, 0xf9, 0x49, 0x25, 0xdd, 0x6d, 0xf2, 0x3a,
                0x2e, 0xe4, 0xd1, 0xa2,
            ],
            Elements::One32 => [
                0x45, 0x26, 0x09, 0x9d, 0x0f, 0x84, 0x0b, 0xa6, 0xab, 0xe5, 0x5e, 0xd9, 0xf3, 0xa9,
                0xb8, 0xfc, 0xa0, 0x84, 0xf2, 0x51, 0xbf, 0xb2, 0x6d, 0x01, 0x1c, 0xf7, 0xaf, 0x27,
                0xb0, 0xd8, 0x8e, 0x29,
            ],
            Elements::One64 => [
                0xc1, 0xa3, 0x4a, 0x00, 0xe3, 0x1e, 0xe1, 0x55, 0x2e, 0x6d, 0x09, 0xc6, 0x70, 0x77,
                0x2b, 0xcb, 0x18, 0x63, 0xc1, 0x0f, 0x14, 0x82, 0xd0, 0x8c, 0xf0, 0xd9, 0xa1, 0x83,
                0x2c, 0x59, 0xec, 0xf4,
            ],
            Elements::One8 => [
                0xf9, 0x58, 0x76, 0x11, 0x81, 0xef, 0xf2, 0x30, 0xcb, 0xc5, 0x1c, 0xc0, 0xe4, 0x5f,
                0x66, 0x91, 0x1b, 0x32, 0x19, 0x16, 0x0e, 0x62, 0x7f, 0xa8, 0x10, 0xc0, 0x7d, 0xfe,
                0xa0, 0xb9, 0x9d, 0x6a,
            ],
            Elements::OutpointHash => [
                0x4d, 0xf6, 0xf1, 0xbf, 0xc3, 0x0c, 0xd6, 0x9a, 0x2f, 0x26, 0xb4, 0x37, 0x05, 0xfe,
                0x0c, 0x7d, 0xa8, 0xf5, 0xd9, 0x71, 0x1e, 0x00, 0x80, 0x07, 0x0c, 0x18, 0xa2, 0xd2,
                0x6f, 0xe8, 0xb0, 0x48,
            ],
            Elements::OutputAmount => [
                0x1d, 0x89, 0xd5, 0x74, 0x43, 0x88, 0x3c, 0x2f, 0x75, 0x58, 0x77, 0x79, 0x77, 0xc6,
                0x48, 0x00, 0x8d, 0x80, 0x4b, 0xd7, 0x4d, 0x83, 0x28, 0x86, 0xba, 0x78, 0x2a, 0x79,
                0x2a, 0x42, 0xeb, 0x97,
            ],
            Elements::OutputAmountsHash => [
                0x8e, 0x59, 0xda, 0x59, 0x06, 0xab, 0x00, 0x17, 0x0a, 0xaf, 0xc4, 0xf5, 0x51, 0x7f,
                0xaa, 0x97, 0x49, 0x51, 0x98, 0x05, 0x78, 0x75, 0xd0, 0x94, 0x6e, 0x37, 0x01, 0x39,
                0x10, 0x86, 0x8e, 0x9e,
            ],
            Elements::OutputAsset => [
                0x7c, 0x1b, 0xe5, 0x44, 0x01, 0xfa, 0x1f, 0x2d, 0xf5, 0x72, 0xce, 0x8c, 0x24, 0x7f,
                0x58, 0x9d, 0xcb, 0x65, 0x9e, 0x8c, 0x59, 0x76, 0x66, 0x0d, 0x80, 0xc6, 0x61, 0x0a,
                0x19, 0xac, 0x39, 0xa9,
            ],
            Elements::OutputNonce => [
                0xd9, 0x0e, 0x3f, 0x0a, 0x28, 0xb4, 0x9f, 0x81, 0x45, 0xbf, 0x98, 0xc7, 0x5f, 0x16,
                0xdf, 0x34, 0xfc, 0xef, 0xf1, 0xee, 0x9e, 0x2e, 0x81, 0xf9, 0xb6, 0x0f, 0x07, 0xa8,
                0x86, 0xc9, 0x67, 0x66,
            ],
            Elements::OutputNoncesHash => [
                0x90, 0xf5, 0xb0, 0x94, 0xe7, 0xdf, 0xb9, 0xd1, 0xf7, 0x0e, 0x59, 0xec, 0x94, 0xe3,
                0x33, 0x59, 0x76, 0xbb, 0xf3, 0x1e, 0xae, 0x82, 0x7a, 0xb2, 0x3c, 0x47, 0x4a, 0xe1,
                0xf5, 0xaf, 0x7e, 0x5a,
            ],
            Elements::OutputNullDatum => [
                0xc2, 0x3d, 0x25, 0xcc, 0x33, 0xad, 0xb7, 0x41, 0x16, 0x5f, 0x8c, 0xf5, 0x68, 0x44,
                0xaf, 0x57, 0x85, 0xae, 0xdf, 0x4e, 0x4e, 0x28, 0xcd, 0xbc, 0x3b, 0xad, 0x19, 0x33,
                0x29, 0x91, 0x65, 0x7d,
            ],
            Elements::OutputRangeProof => [
                0x4e, 0xb4, 0x09, 0xbf, 0x1d, 0xac, 0x48, 0xca, 0x1c, 0x62, 0xab, 0x42, 0x59, 0xa9,
                0x7c, 0x09, 0x1e, 0x92, 0x7e, 0xbe, 0xbe, 0xdf, 0xf0, 0x69, 0xdb, 0x5d, 0x22, 0x4e,
                0x0d, 0xde, 0x3d, 0xc2,
            ],
            Elements::OutputRangeProofsHash => [
                0xdb, 0xee, 0xee, 0xf7, 0x64, 0x03, 0x60, 0xe2, 0x03, 0xe2, 0x73, 0x43, 0x8f, 0x76,
                0xd2, 0x8e, 0x61, 0xcf, 0x3a, 0x0f, 0xbd, 0xe7, 0xf5, 0x7d, 0x0b, 0x46, 0xf9, 0x36,
                0x7f, 0x6c, 0xd6, 0x9e,
            ],
            Elements::OutputScriptHash => [
                0xfd, 0x0c, 0xeb, 0x21, 0x13, 0x9d, 0x07, 0x77, 0xa4, 0xa6, 0x1f, 0x05, 0xa5, 0xdc,
                0x1f, 0xda, 0xa7, 0x61, 0xf6, 0xfd, 0xf2, 0x54, 0x78, 0x12, 0x2b, 0xe9, 0xe0, 0x60,
                0xea, 0x21, 0x86, 0x35,
            ],
            Elements::OutputScriptsHash => [
                0xbe, 0xdc, 0x17, 0x72, 0x68, 0x3b, 0x82, 0x40, 0xf6, 0x9a, 0xfa, 0x34, 0xd2, 0x80,
                0xea, 0x27, 0xa5, 0x85, 0xd8, 0x21, 0x02, 0xd1, 0x0c, 0xf9, 0x18, 0x20, 0x7a, 0x31,
                0x5e, 0xd5, 0xb2, 0xf0,
            ],
            Elements::OutputSurjectionProof => [
                0x10, 0x99, 0x78, 0x18, 0xdc, 0x55, 0xd3, 0x95, 0xc9, 0xdf, 0x59, 0x97, 0x93, 0xeb,
                0xdb, 0xca, 0xcb, 0x27, 0x7c, 0x70, 0x10, 0x4f, 0x83, 0xa0, 0x04, 0x96, 0x88, 0x67,
                0xe8, 0x94, 0xb6, 0xf9,
            ],
            Elements::OutputSurjectionProofsHash => [
                0xd8, 0x04, 0x6c, 0x13, 0x08, 0x6e, 0x17, 0xb4, 0x73, 0x9e, 0xfc, 0x91, 0xa4, 0x68,
                0xa6, 0xa1, 0x8c, 0x05, 0xf6, 0xe2, 0x6e, 0xc3, 0x05, 0xea, 0x0d, 0xb5, 0xa0, 0x1a,
                0xdf, 0xc0, 0xd3, 0x74,
            ],
            Elements::OutputsHash => [
                0x5a, 0xa4, 0x6e, 0x62, 0x4d, 0xe1, 0x9c, 0xd0, 0xca, 0x3b, 0x11, 0x88, 0xcc, 0x2c,
                0x2b, 0x50, 0x55, 0x12, 0x07, 0xa4, 0xab, 0x2b, 0x16, 0x93, 0x95, 0xe8, 0xb9, 0x53,
                0x13, 0x9d, 0x79, 0x6a,
            ],
            Elements::ParseLock => [
                0xd5, 0x96, 0x9c, 0x25, 0xb5, 0x6e, 0x87, 0xb4, 0xfb, 0x28, 0x80, 0xea, 0xee, 0x90,
                0xcc, 0x94, 0x49, 0xfa, 0xa2, 0xf3, 0xd0, 0x76, 0x0d, 0xe9, 0xfe, 0xe5, 0x65, 0xdf,
                0xcf, 0x6a, 0x16, 0xc6,
            ],
            Elements::ParseSequence => [
                0x04, 0xb9, 0x5b, 0xbe, 0x88, 0x1e, 0x1a, 0x6b, 0x41, 0x9f, 0x82, 0x88, 0x3f, 0xf0,
                0x73, 0xea, 0xdb, 0xc1, 0x4f, 0x2c, 0x8c, 0x56, 0xb3, 0x6f, 0x19, 0xdc, 0xec, 0xac,
                0x27, 0xe7, 0x73, 0x7f,
            ],
            Elements::PointVerify1 => [
                0xef, 0xe2, 0x87, 0xa5, 0xcb, 0xe2, 0x81, 0x8b, 0x11, 0x3d, 0x87, 0x00, 0x22, 0x32,
                0x6e, 0xb3, 0x0a, 0x4b, 0xe0, 0xad, 0xa4, 0x40, 0x23, 0xd5, 0x45, 0xb6, 0xc1, 0x51,
                0xf0, 0xcd, 0xa7, 0x08,
            ],
            Elements::ReissuanceBlinding => [
                0xa2, 0x2d, 0x03, 0x2b, 0xd9, 0x7d, 0xa2, 0xb2, 0x9f, 0xc6, 0x2d, 0x88, 0xe0, 0x49,
                0x79, 0x50, 0x6c, 0x2f, 0x59, 0xd8, 0xfe, 0xc8, 0xad, 0xd8, 0x99, 0xa9, 0xfb, 0x99,
                0x07, 0xf4, 0x9a, 0xa9,
            ],
            Elements::ReissuanceEntropy => [
                0xdd, 0xc1, 0x31, 0x7e, 0xdd, 0xd9, 0xf6, 0xd8, 0xb4, 0x8c, 0xf7, 0x58, 0x99, 0x82,
                0x7b, 0xef, 0xe3, 0xae, 0x5f, 0xfb, 0x4e, 0xa2, 0xdd, 0xfd, 0x25, 0xa9, 0x52, 0x84,
                0x8d, 0x82, 0x79, 0xec,
            ],
            Elements::ScalarAdd => [
                0x67, 0xe4, 0x1f, 0xad, 0x70, 0x45, 0x00, 0xce, 0x97, 0x50, 0x91, 0x32, 0xd4, 0xf6,
                0x97, 0x34, 0x2e, 0x85, 0x83, 0xed, 0x7e, 0x9f, 0x7b, 0xed, 0xb9, 0x95, 0xd3, 0x6c,
                0xf6, 0x5f, 0xf3, 0x2e,
            ],
            Elements::ScalarInvert => [
                0xc0, 0xe2, 0xad, 0x1b, 0xa6, 0xbf, 0xd9, 0x10, 0x44, 0x24, 0xf5, 0x94, 0xd0, 0x07,
                0x3e, 0xa1, 0x99, 0x40, 0x5e, 0x5c, 0xa5, 0xb7, 0x12, 0x83, 0x94, 0xb6, 0x13, 0xb9,
                0xe1, 0xbd, 0x36, 0xfc,
            ],
            Elements::ScalarIsZero => [
                0x38, 0xa4, 0x57, 0xca, 0xb1, 0xc3, 0x0c, 0x51, 0x4e, 0x20, 0xe2, 0x41, 0xd5, 0x84,
                0x65, 0x40, 0x75, 0xec, 0x4d, 0x05, 0x49, 0x6c, 0x7e, 0x0b, 0x1c, 0xe2, 0xde, 0x5e,
                0x2f, 0xc1, 0x99, 0x16,
            ],
            Elements::ScalarMultiply => [
                0x14, 0x51, 0x3c, 0xf4, 0x41, 0x17, 0x9e, 0x62, 0xfb, 0x42, 0x93, 0xbb, 0x35, 0x3e,
                0xe5, 0xbf, 0x01, 0xed, 0xdf, 0x8d, 0x81, 0xce, 0x03, 0x10, 0x06, 0x2f, 0x09, 0xa8,
                0x1d, 0x2f, 0xbc, 0xa8,
            ],
            Elements::ScalarMultiplyLambda => [
                0xf6, 0x24, 0x00, 0xf5, 0xbe, 0x74, 0x00, 0xa7, 0x12, 0xe7, 0x4a, 0x1d, 0xc1, 0xa8,
                0x41, 0xe6, 0x02, 0x4a, 0x96, 0x18, 0x55, 0x1c, 0x33, 0x64, 0xc4, 0xe8, 0x15, 0x6d,
                0x1c, 0xdd, 0xed, 0x83,
            ],
            Elements::ScalarNegate => [
                0x6a, 0x97, 0x6a, 0x67, 0x68, 0xbd, 0x72, 0x8f, 0xe2, 0x10, 0x51, 0x85, 0x1c, 0x60,
                0xeb, 0x25, 0x72, 0xe5, 0xd0, 0x6c, 0x95, 0x56, 0x6d, 0xfa, 0xe9, 0x28, 0x20, 0xc8,
                0x42, 0x4a, 0xaa, 0x4e,
            ],
            Elements::ScalarNormalize => [
                0x90, 0xe0, 0x25, 0x78, 0x96, 0x99, 0x0b, 0xa6, 0xf0, 0xb0, 0x76, 0x54, 0x29, 0x19,
                0xcd, 0x06, 0x4a, 0xc0, 0x8b, 0x27, 0x7f, 0xae, 0x34, 0x79, 0xe4, 0x09, 0x18, 0xeb,
                0x6f, 0x5b, 0x07, 0xd8,
            ],
            Elements::ScalarSquare => [
                0xd6, 0x36, 0xb4, 0x9d, 0xc6, 0xb2, 0x26, 0x6c, 0xce, 0xcb, 0x7b, 0xc0, 0x41, 0x68,
                0x82, 0x3b, 0x2a, 0x5d, 0x7a, 0x1d, 0x2a, 0xc3, 0x43, 0xda, 0x60, 0x54, 0x19, 0xd3,
                0x8d, 0xfd, 0xfd, 0xe0,
            ],
            Elements::Scale => [
                0xac, 0x65, 0xf0, 0xb8, 0x0d, 0x5e, 0xfc, 0xeb, 0xb5, 0x01, 0xe5, 0xe9, 0x62, 0xf4,
                0x4f, 0xb8, 0x14, 0x6b, 0x6d, 0x4e, 0x46, 0x5b, 0xea, 0x49, 0xbc, 0x74, 0x09, 0x5e,
                0x21, 0xfb, 0xaa, 0xc0,
            ],
            Elements::ScriptCMR => [
                0x14, 0xfd, 0xb5, 0x2d, 0xeb, 0x9e, 0xdd, 0xff, 0x27, 0x3a, 0x4c, 0x80, 0xdd, 0x7f,
                0x23, 0x93, 0x35, 0x16, 0xf7, 0xe8, 0xaa, 0xe7, 0x6d, 0xe7, 0x17, 0x21, 0xaf, 0x01,
                0x9c, 0x12, 0x5f, 0x4a,
            ],
            Elements::Sha256Block => [
                0xdf, 0xc9, 0x27, 0xd3, 0x9b, 0xf7, 0x14, 0x7a, 0x8b, 0x0a, 0x7f, 0x43, 0x79, 0x46,
                0x68, 0x70, 0x82, 0x4d, 0xb1, 0x02, 0x09, 0x0a, 0x00, 0x36, 0x29, 0x23, 0xa3, 0x77,
                0xa9, 0x1a, 0xf6, 0x81,
            ],
            Elements::Sha256Ctx8Add1 => [
                0x2f, 0xec, 0x1d, 0x6b, 0x08, 0xa8, 0xf8, 0x19, 0xb8, 0xf4, 0x6a, 0x60, 0xfb, 0x90,
                0xac, 0x17, 0xb8, 0x84, 0x64, 0xcf, 0xb6, 0x08, 0x38, 0x49, 0xaf, 0xea, 0xde, 0x1f,
                0x34, 0xf1, 0xe7, 0xbc,
            ],
            Elements::Sha256Ctx8Add128 => [
                0x20, 0x0b, 0x99, 0x32, 0xe7, 0x4d, 0x3d, 0x13, 0x51, 0x2d, 0x64, 0x44, 0xee, 0x64,
                0x45, 0xed, 0x17, 0xee, 0x33, 0xd2, 0x9f, 0x39, 0x79, 0xd8, 0x39, 0x24, 0xe6, 0xa5,
                0x9c, 0x99, 0xd8, 0x10,
            ],
            Elements::Sha256Ctx8Add16 => [
                0x29, 0x26, 0x3f, 0x97, 0xa3, 0xf4, 0xbd, 0x82, 0x7f, 0x20, 0x2d, 0x4b, 0xf9, 0xea,
                0x57, 0xaa, 0x84, 0x1a, 0xae, 0x6e, 0xe9, 0xa1, 0x3f, 0xa7, 0xb4, 0x59, 0xe7, 0x99,
                0x9b, 0x15, 0x44, 0x97,
            ],
            Elements::Sha256Ctx8Add2 => [
                0xb4, 0x59, 0x0d, 0x05, 0x6c, 0x52, 0xbe, 0x9c, 0x16, 0x61, 0x00, 0x03, 0x53, 0x50,
                0xd8, 0xf4, 0x3d, 0x31, 0x0f, 0x3f, 0x57, 0x56, 0x17, 0xc6, 0x39, 0x81, 0x95, 0x48,
                0xb5, 0xb4, 0x40, 0x87,
            ],
            Elements::Sha256Ctx8Add256 => [
                0xcf, 0x64, 0x38, 0x27, 0x7c, 0x66, 0xb4, 0xe6, 0x30, 0x7e, 0xcf, 0xac, 0x31, 0x52,
                0x26, 0xc7, 0xb9, 0xaa, 0xdb, 0xda, 0x33, 0xeb, 0xe1, 0x9d, 0x62, 0x04, 0x65, 0xef,
                0x48, 0x44, 0xd8, 0x08,
            ],
            Elements::Sha256Ctx8Add32 => [
                0x69, 0xa6, 0xec, 0xb2, 0x7a, 0x04, 0x49, 0xf3, 0xd6, 0x7a, 0xb9, 0xd9, 0x61, 0x96,
                0x1d, 0x6a, 0xa8, 0x15, 0x07, 0xde, 0x69, 0x51, 0xb0, 0xd4, 0xce, 0xaf, 0xf6, 0xd8,
                0xbd, 0x38, 0x80, 0x27,
            ],
            Elements::Sha256Ctx8Add4 => [
                0xa5, 0x82, 0xd2, 0xbc, 0xdf, 0xaf, 0x71, 0xfa, 0xf7, 0xb9, 0xae, 0x04, 0x5f, 0x97,
                0xaa, 0xc0, 0x55, 0xc4, 0x77, 0x9c, 0x6e, 0x80, 0x22, 0xe9, 0xfe, 0x11, 0xfb, 0xa2,
                0xa4, 0x44, 0xad, 0xb1,
            ],
            Elements::Sha256Ctx8Add512 => [
                0x89, 0x5f, 0x1b, 0x6c, 0x9a, 0x41, 0xf8, 0x3f, 0xd3, 0x27, 0x23, 0xc7, 0xa2, 0x2d,
                0xe2, 0x65, 0xa1, 0x27, 0x90, 0xaa, 0xa8, 0x83, 0x17, 0xac, 0x01, 0xdd, 0xbb, 0xd1,
                0x4b, 0x60, 0xd4, 0x1d,
            ],
            Elements::Sha256Ctx8Add64 => [
                0x78, 0x15, 0x65, 0x01, 0x89, 0xa1, 0x17, 0xba, 0x09, 0x12, 0x08, 0x8b, 0x79, 0x29,
                0x7a, 0x07, 0x66, 0xb6, 0x09, 0x45, 0x87, 0xbf, 0x5d, 0xa6, 0xe8, 0xf4, 0x6e, 0x9c,
                0x37, 0xb4, 0x34, 0x91,
            ],
            Elements::Sha256Ctx8Add8 => [
                0x1e, 0x8d, 0xb5, 0xba, 0x26, 0x05, 0xf0, 0xf3, 0x05, 0xd5, 0xb7, 0xd5, 0x0c, 0x09,
                0x00, 0x6e, 0xa4, 0x85, 0x58, 0x46, 0x3f, 0x6e, 0x0c, 0x85, 0xfe, 0x50, 0x51, 0xf0,
                0xc0, 0x53, 0xaf, 0xeb,
            ],
            Elements::Sha256Ctx8AddBuffer511 => [
                0xf8, 0x9d, 0x7e, 0x21, 0x9b, 0x3e, 0xd8, 0x81, 0x4d, 0xd9, 0xf5, 0xc1, 0xa5, 0xda,
                0xb6, 0xba, 0xd6, 0xd3, 0x2d, 0xc5, 0x72, 0xa5, 0x21, 0x35, 0x39, 0x2c, 0x81, 0x43,
                0x0e, 0x12, 0x03, 0xd1,
            ],
            Elements::Sha256Ctx8Finalize => [
                0xc9, 0xcd, 0x48, 0x36, 0x64, 0xf9, 0xaf, 0x94, 0xd5, 0xe9, 0x41, 0xdb, 0xb9, 0xe1,
                0xe9, 0xb7, 0xc6, 0x4f, 0x48, 0xef, 0x15, 0xc9, 0x06, 0x5f, 0xd1, 0xbb, 0x7f, 0xd9,
                0xc5, 0x6c, 0xbd, 0xc6,
            ],
            Elements::Sha256Ctx8Init => [
                0x88, 0xdb, 0x03, 0x5a, 0x49, 0x0d, 0xa7, 0x89, 0x30, 0xf7, 0x5e, 0x6e, 0x1b, 0x10,
                0xaa, 0x86, 0x8b, 0xd4, 0x75, 0x93, 0xc6, 0x74, 0xf3, 0xbc, 0x4b, 0xb6, 0x24, 0x22,
                0x53, 0x13, 0xe3, 0xba,
            ],
            Elements::Sha256Iv => [
                0xe4, 0xd5, 0x96, 0xe0, 0x0f, 0xf0, 0xf7, 0xdf, 0x3d, 0x99, 0x69, 0xd8, 0x2c, 0xb1,
                0x63, 0x4d, 0x59, 0xeb, 0x9d, 0x0f, 0x4b, 0x24, 0xd8, 0xca, 0x72, 0xe8, 0x48, 0xc1,
                0x0c, 0x75, 0x24, 0x4c,
            ],
            Elements::SigAllHash => [
                0xe3, 0xf0, 0x90, 0xbc, 0xf8, 0x57, 0x37, 0xb6, 0x09, 0xc2, 0x36, 0x24, 0x05, 0xb8,
                0x36, 0x15, 0x50, 0x4a, 0x0d, 0x2a, 0x1c, 0x05, 0xa5, 0xc3, 0x31, 0xa3, 0xe4, 0xb1,
                0x88, 0xd5, 0x8c, 0xa4,
            ],
            Elements::Subtract16 => [
                0x62, 0x4a, 0xf4, 0x3d, 0x27, 0x5e, 0x2a, 0x20, 0x70, 0x3e, 0x66, 0x52, 0x3d, 0x35,
                0xdd, 0x34, 0xef, 0x8e, 0x8b, 0x29, 0x3a, 0x57, 0x38, 0x0a, 0x21, 0xdb, 0xf6, 0x10,
                0xcd, 0x62, 0x7a, 0xa3,
            ],
            Elements::Subtract32 => [
                0xf7, 0x6e, 0xca, 0xd1, 0xfd, 0xa5, 0x0f, 0x13, 0x5b, 0xdf, 0xe3, 0xe5, 0x33, 0xa1,
                0x50, 0x09, 0x8f, 0x40, 0x62, 0x61, 0xc7, 0x6f, 0x6d, 0xbf, 0x67, 0x25, 0xf1, 0xe3,
                0x88, 0x3c, 0x2a, 0xe2,
            ],
            Elements::Subtract64 => [
                0x99, 0x4b, 0x25, 0x00, 0x38, 0x7d, 0x8d, 0x62, 0x02, 0x09, 0x67, 0x74, 0x16, 0xb9,
                0xe6, 0x04, 0x52, 0x6c, 0x70, 0x8f, 0xf1, 0xf9, 0x65, 0xc9, 0xc9, 0x12, 0x52, 0x04,
                0x7a, 0x3f, 0x57, 0xb3,
            ],
            Elements::Subtract8 => [
                0x57, 0xa0, 0xd0, 0x8e, 0x5f, 0x01, 0xe8, 0x38, 0x8d, 0x78, 0x5c, 0xcb, 0x26, 0x57,
                0xc4, 0xea, 0x98, 0xb8, 0x54, 0xa6, 0x58, 0x5c, 0x65, 0x8a, 0x21, 0x0d, 0xb0, 0x90,
                0x1d, 0x10, 0xf9, 0x48,
            ],
            Elements::TapEnvHash => [
                0xb8, 0xd3, 0x7d, 0x55, 0x55, 0xf3, 0x15, 0x08, 0xda, 0x71, 0xf9, 0x1f, 0x96, 0x35,
                0xc8, 0xe0, 0x94, 0xd5, 0x89, 0x8f, 0x2e, 0x43, 0xdb, 0x3f, 0x93, 0x14, 0x94, 0x48,
                0x60, 0x69, 0x87, 0xe1,
            ],
            Elements::Tapbranch => [
                0x48, 0xe9, 0x04, 0xd2, 0x7d, 0x9b, 0x04, 0xb5, 0x40, 0x8d, 0xa2, 0xc8, 0x8f, 0xca,
                0x09, 0x68, 0x2d, 0x19, 0xcf, 0xf5, 0x64, 0xcb, 0xe5, 0x2e, 0xd1, 0xe5, 0x5b, 0x91,
                0x26, 0xb2, 0x1e, 0x3b,
            ],
            Elements::TapbranchHash => [
                0x67, 0x7e, 0x98, 0x44, 0xaf, 0xac, 0x34, 0x13, 0x61, 0x3c, 0x39, 0x6b, 0x48, 0xdd,
                0xc3, 0xbe, 0x1d, 0x86, 0x07, 0x51, 0x2e, 0xdd, 0x23, 0x6d, 0x8f, 0x56, 0xfe, 0xc9,
                0xba, 0x95, 0x2a, 0x4e,
            ],
            Elements::TapleafHash => [
                0xce, 0xeb, 0xe8, 0x3d, 0xa3, 0xba, 0x51, 0x63, 0x41, 0xf3, 0xa4, 0xe0, 0xa1, 0xb2,
                0x90, 0xe3, 0x78, 0x18, 0xe1, 0xb7, 0x12, 0x7b, 0x52, 0x70, 0x36, 0x52, 0x79, 0x06,
                0xea, 0x5d, 0x33, 0x59,
            ],
            Elements::TapleafVersion => [
                0x55, 0xa2, 0xf5, 0x15, 0x24, 0x71, 0x1d, 0xfa, 0xda, 0xc0, 0x21, 0x58, 0xb3, 0x63,
                0x4a, 0x66, 0x5a, 0xd4, 0x84, 0xcd, 0x7f, 0x3a, 0x6a, 0x30, 0x0d, 0x33, 0x64, 0x24,
                0xe1, 0x44, 0xd3, 0x35,
            ],
            Elements::TxHash => [
                0x17, 0x51, 0x24, 0xb7, 0x79, 0x8c, 0xef, 0x21, 0x15, 0x83, 0x62, 0x8b, 0xde, 0x84,
                0x10, 0xd2, 0x79, 0xa6, 0xda, 0x8b, 0x4b, 0x6b, 0x57, 0xf5, 0x7f, 0xbe, 0x6b, 0xa0,
                0xa1, 0x1a, 0xa9, 0xe3,
            ],
            Elements::TxIsFinal => [
                0xfe, 0x40, 0x62, 0x77, 0x09, 0x6b, 0x0c, 0xd2, 0x1c, 0xc3, 0x83, 0xe0, 0x11, 0x0e,
                0x24, 0x4c, 0x24, 0x6f, 0x95, 0x71, 0x05, 0xe9, 0x4b, 0xb8, 0x13, 0xec, 0x75, 0xfb,
                0xeb, 0xc9, 0x7f, 0x92,
            ],
            Elements::TxLockDistance => [
                0xcc, 0xfd, 0xc6, 0x2a, 0xed, 0x51, 0x0c, 0xa0, 0xda, 0xee, 0xff, 0xb5, 0x0c, 0x6d,
                0xab, 0xc1, 0x1c, 0x03, 0xc4, 0x0c, 0x30, 0xc5, 0xc0, 0xfd, 0xb5, 0xbb, 0x57, 0xa6,
                0xf5, 0xa8, 0xc2, 0xa3,
            ],
            Elements::TxLockDuration => [
                0x85, 0x26, 0xe3, 0xd4, 0x3f, 0x43, 0xbd, 0x2b, 0x4b, 0x19, 0x85, 0xc0, 0xb9, 0x30,
                0x1d, 0x9a, 0x98, 0x29, 0x01, 0x67, 0xe5, 0x87, 0xe5, 0xc3, 0xcf, 0x13, 0x5b, 0xc6,
                0xb1, 0x23, 0x13, 0x46,
            ],
            Elements::TxLockHeight => [
                0xf9, 0x4f, 0x86, 0xa6, 0xf0, 0x79, 0x95, 0xdd, 0xcd, 0xf8, 0xcc, 0xaf, 0xde, 0xea,
                0x8e, 0x58, 0x45, 0x0b, 0xca, 0xc4, 0xde, 0x78, 0x35, 0x97, 0xf8, 0x6a, 0xb0, 0x89,
                0xeb, 0x64, 0xf7, 0xfc,
            ],
            Elements::TxLockTime => [
                0xd4, 0x4c, 0xe1, 0x4c, 0xcf, 0xb3, 0xa1, 0xbc, 0xb7, 0x12, 0x10, 0xc0, 0xa2, 0xe1,
                0x5b, 0xc9, 0xf5, 0xc9, 0xa2, 0x51, 0x29, 0xa0, 0x37, 0xbe, 0x48, 0x3f, 0x16, 0xc3,
                0xa5, 0x05, 0xdb, 0x59,
            ],
            Elements::Verify => [
                0xa1, 0x75, 0x34, 0xa4, 0x9a, 0xcb, 0xec, 0xb1, 0x55, 0x4d, 0x6e, 0xc5, 0xc6, 0xda,
                0x50, 0xc2, 0x8c, 0x9c, 0xd9, 0x5e, 0xa0, 0xb4, 0x48, 0x86, 0xb8, 0xe0, 0x94, 0xad,
                0xa7, 0x77, 0xc1, 0xd4,
            ],
            Elements::Version => [
                0xd3, 0xca, 0xdc, 0xbd, 0xaf, 0x57, 0xf8, 0xe5, 0xb4, 0x8e, 0xb7, 0x63, 0x8c, 0x75,
                0x43, 0x8d, 0x67, 0x37, 0x6c, 0x79, 0xb4, 0xcf, 0x70, 0xb2, 0x58, 0x2e, 0x32, 0xd9,
                0x31, 0xc4, 0x2c, 0xdf,
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
            Elements::AnnexHash => b"***+1h*+1*ll*+1l*+1i*+1****22*22**22*22***22*22**22*22+1***22*22**22*22*lh+1h",
            Elements::AssetAmountHash => b"***+1h*+1*ll*+1l*+1i*+1****22*22**22*22***22*22**22*22+1***22*22**22*22*lh*+*2hh+*2hl",
            Elements::Bip0340Verify => b"**hh*hh",
            Elements::BuildTapbranch => b"*hh",
            Elements::BuildTapleafSimplicity => b"h",
            Elements::CalculateAsset => b"h",
            Elements::CalculateConfidentialToken => b"h",
            Elements::CalculateExplicitToken => b"h",
            Elements::CalculateIssuanceEntropy => b"**hih",
            Elements::CheckLockDistance => b"****22*22**22*22***22*22**22*22",
            Elements::CheckLockDuration => b"****22*22**22*22***22*22**22*22",
            Elements::CheckLockHeight => b"i",
            Elements::CheckLockTime => b"i",
            Elements::CheckSigVerify => b"**h*hh*hh",
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
        };

        TypeName(name)
    }

    fn target_ty(&self) -> TypeName {
        let name: &'static [u8] = match self {
            Elements::Add16 => b"*2****22*22**22*22***22*22**22*22",
            Elements::Add32 => b"*2i",
            Elements::Add64 => b"*2l",
            Elements::Add8 => b"*2***22*22**22*22",
            Elements::AnnexHash => b"**+1h*+1*ll*+1l*+1i*+1****22*22**22*22***22*22**22*22+1***22*22**22*22*lh",
            Elements::AssetAmountHash => b"**+1h*+1*ll*+1l*+1i*+1****22*22**22*22***22*22**22*22+1***22*22**22*22*lh",
            Elements::Bip0340Verify => b"1",
            Elements::BuildTapbranch => b"h",
            Elements::BuildTapleafSimplicity => b"h",
            Elements::CalculateAsset => b"h",
            Elements::CalculateConfidentialToken => b"h",
            Elements::CalculateExplicitToken => b"h",
            Elements::CalculateIssuanceEntropy => b"h",
            Elements::CheckLockDistance => b"1",
            Elements::CheckLockDuration => b"1",
            Elements::CheckLockHeight => b"1",
            Elements::CheckLockTime => b"1",
            Elements::CheckSigVerify => b"1",
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

    fn decode<I: Iterator<Item = u8>>(bits: &mut BitIter<I>) -> Result<Self, Error> {
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
                            1 => {}
                        },
                        1 => {
                            0 => {
                                0 => {},
                                1 => {
                                    0 => {},
                                    1 => {
                                        0 => {
                                            0 => {},
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
            Elements::AnnexHash => &simplicity_sys::c_jets::jets_wrapper::annex_hash,
            Elements::AssetAmountHash => &simplicity_sys::c_jets::jets_wrapper::asset_amount_hash,
            Elements::Bip0340Verify => &simplicity_sys::c_jets::jets_wrapper::bip_0340_verify,
            Elements::BuildTapbranch => &simplicity_sys::c_jets::jets_wrapper::build_tapbranch,
            Elements::BuildTapleafSimplicity => &simplicity_sys::c_jets::jets_wrapper::build_tapleaf_simplicity,
            Elements::CalculateAsset => &simplicity_sys::c_jets::jets_wrapper::calculate_asset,
            Elements::CalculateConfidentialToken => &simplicity_sys::c_jets::jets_wrapper::calculate_confidential_token,
            Elements::CalculateExplicitToken => &simplicity_sys::c_jets::jets_wrapper::calculate_explicit_token,
            Elements::CalculateIssuanceEntropy => &simplicity_sys::c_jets::jets_wrapper::calculate_issuance_entropy,
            Elements::CheckLockDistance => &simplicity_sys::c_jets::jets_wrapper::check_lock_distance,
            Elements::CheckLockDuration => &simplicity_sys::c_jets::jets_wrapper::check_lock_duration,
            Elements::CheckLockHeight => &simplicity_sys::c_jets::jets_wrapper::check_lock_height,
            Elements::CheckLockTime => &simplicity_sys::c_jets::jets_wrapper::check_lock_time,
            Elements::CheckSigVerify => &simplicity_sys::c_jets::jets_wrapper::check_sig_verify,
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
            Elements::AnnexHash => f.write_str("annex_hash"),
            Elements::AssetAmountHash => f.write_str("asset_amount_hash"),
            Elements::Bip0340Verify => f.write_str("bip_0340_verify"),
            Elements::BuildTapbranch => f.write_str("build_tapbranch"),
            Elements::BuildTapleafSimplicity => f.write_str("build_tapleaf_simplicity"),
            Elements::CalculateAsset => f.write_str("calculate_asset"),
            Elements::CalculateConfidentialToken => f.write_str("calculate_confidential_token"),
            Elements::CalculateExplicitToken => f.write_str("calculate_explicit_token"),
            Elements::CalculateIssuanceEntropy => f.write_str("calculate_issuance_entropy"),
            Elements::CheckLockDistance => f.write_str("check_lock_distance"),
            Elements::CheckLockDuration => f.write_str("check_lock_duration"),
            Elements::CheckLockHeight => f.write_str("check_lock_height"),
            Elements::CheckLockTime => f.write_str("check_lock_time"),
            Elements::CheckSigVerify => f.write_str("check_sig_verify"),
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
        }
    }
}

impl str::FromStr for Elements {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self, Error> {
        match s {
            "add_16" => Ok(Elements::Add16),
            "add_32" => Ok(Elements::Add32),
            "add_64" => Ok(Elements::Add64),
            "add_8" => Ok(Elements::Add8),
            "annex_hash" => Ok(Elements::AnnexHash),
            "asset_amount_hash" => Ok(Elements::AssetAmountHash),
            "bip_0340_verify" => Ok(Elements::Bip0340Verify),
            "build_tapbranch" => Ok(Elements::BuildTapbranch),
            "build_tapleaf_simplicity" => Ok(Elements::BuildTapleafSimplicity),
            "calculate_asset" => Ok(Elements::CalculateAsset),
            "calculate_confidential_token" => Ok(Elements::CalculateConfidentialToken),
            "calculate_explicit_token" => Ok(Elements::CalculateExplicitToken),
            "calculate_issuance_entropy" => Ok(Elements::CalculateIssuanceEntropy),
            "check_lock_distance" => Ok(Elements::CheckLockDistance),
            "check_lock_duration" => Ok(Elements::CheckLockDuration),
            "check_lock_height" => Ok(Elements::CheckLockHeight),
            "check_lock_time" => Ok(Elements::CheckLockTime),
            "check_sig_verify" => Ok(Elements::CheckSigVerify),
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
            x => Err(Error::InvalidJetName(x.to_owned())),
        }
    }
}
