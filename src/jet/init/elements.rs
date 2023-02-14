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
use crate::jet::elements::ElementsEnv;
use simplicity_sys::CElementsTxEnv;

/// Elements jet family
#[derive(Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Debug, Hash)]
pub enum Elements {
    Add32,
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
    Eq256,
    Eq32,
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
    FullAdd32,
    FullMultiply32,
    FullSubtract32,
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
    Le32,
    LinearCombination1,
    LinearVerify1,
    LockTime,
    Low32,
    Multiply32,
    NewIssuanceContract,
    NonceHash,
    NumInputs,
    NumOutputs,
    One32,
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
    Subtract32,
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
            Elements::Add32 => [
                0xe4, 0x04, 0x66, 0xa7, 0xec, 0xf7, 0x1c, 0xe8, 0x62, 0xfb, 0x3c, 0x15, 0x4c, 0x1e,
                0x8f, 0x84, 0x5d, 0x7e, 0x57, 0x07, 0x46, 0x3a, 0x89, 0x45, 0x37, 0xa3, 0x2f, 0xc7,
                0x21, 0x49, 0x00, 0xad,
            ],
            Elements::AnnexHash => [
                0x58, 0xc8, 0xf7, 0x13, 0xb6, 0x3a, 0xb9, 0x33, 0x98, 0xdd, 0x98, 0x49, 0xbb, 0x5a,
                0xc0, 0x36, 0x9b, 0x0d, 0x6b, 0x91, 0xf4, 0x32, 0xbe, 0x19, 0x35, 0x90, 0x75, 0x44,
                0x7c, 0xb4, 0xbb, 0x85,
            ],
            Elements::AssetAmountHash => [
                0xd8, 0xa7, 0xaa, 0x94, 0x95, 0x20, 0x89, 0xda, 0x93, 0xef, 0x45, 0xc9, 0xa3, 0xc2,
                0x05, 0x3b, 0xa9, 0xfa, 0x02, 0x67, 0xdf, 0x07, 0x8b, 0x74, 0x36, 0xc8, 0xe3, 0xcb,
                0x0f, 0x64, 0xb7, 0xb0,
            ],
            Elements::Bip0340Verify => [
                0x6d, 0xa5, 0x26, 0xad, 0xe3, 0x4e, 0xb9, 0xd3, 0xaa, 0x8c, 0xd7, 0x05, 0xba, 0xf9,
                0xac, 0xeb, 0x3e, 0xb5, 0x85, 0x23, 0xba, 0x84, 0x28, 0x5c, 0x55, 0xfc, 0x04, 0x6a,
                0x2b, 0x0d, 0x2a, 0xea,
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
                0x05, 0xe7, 0xd6, 0xc4, 0x38, 0x6b, 0x46, 0x7a, 0x7d, 0x93, 0xc7, 0x21, 0x76, 0x5c,
                0x90, 0x41, 0x0d, 0x30, 0x9f, 0x08, 0xc6, 0x1c, 0x4e, 0x27, 0xd7, 0xbb, 0x69, 0x20,
                0x7b, 0xa0, 0x86, 0xd3,
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
            Elements::FullAdd32 => [
                0x47, 0x27, 0x36, 0x1e, 0xa0, 0x03, 0xc1, 0xa4, 0x83, 0xe5, 0x75, 0x05, 0xcf, 0x5b,
                0x40, 0x5a, 0x82, 0x27, 0xda, 0x1a, 0xdd, 0xc4, 0x7e, 0x2b, 0x01, 0x6c, 0x2d, 0x09,
                0xbe, 0x04, 0x7f, 0xe8,
            ],
            Elements::FullMultiply32 => [
                0xaa, 0xc2, 0x53, 0x61, 0xe5, 0x98, 0xe3, 0x54, 0x38, 0xb9, 0x18, 0xb5, 0x8f, 0xd2,
                0xce, 0xf4, 0xdb, 0x3c, 0x5d, 0x8c, 0x5e, 0x63, 0xaa, 0x4f, 0x25, 0xe9, 0xce, 0xc0,
                0xcf, 0xd9, 0xdf, 0xb1,
            ],
            Elements::FullSubtract32 => [
                0x6d, 0x96, 0xf6, 0x8a, 0x94, 0x5c, 0x22, 0xe7, 0x62, 0x11, 0x5c, 0x09, 0x42, 0x97,
                0xb1, 0x94, 0xbe, 0xdc, 0x0c, 0xe5, 0xa0, 0xc9, 0x2d, 0xb6, 0x4b, 0x83, 0x0a, 0x18,
                0xb4, 0x4d, 0xf0, 0xd0,
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
                0x80, 0x75, 0xb5, 0x90, 0x28, 0x64, 0x90, 0x87, 0xde, 0xd0, 0xe7, 0x66, 0x6e, 0x1a,
                0x00, 0x51, 0x45, 0x09, 0xd0, 0xc9, 0x87, 0xd8, 0x8d, 0x18, 0x68, 0x1a, 0xc8, 0x9b,
                0xc9, 0x60, 0xeb, 0x25,
            ],
            Elements::GejDouble => [
                0x80, 0xd0, 0x82, 0x5d, 0x57, 0xce, 0x42, 0x4c, 0xc8, 0xb8, 0x9d, 0xc2, 0x51, 0x0d,
                0x7e, 0x65, 0x85, 0x8a, 0x99, 0x4e, 0x8e, 0x98, 0x76, 0x23, 0xd1, 0x0e, 0x34, 0x83,
                0xde, 0x26, 0xdf, 0x9e,
            ],
            Elements::GejGeAdd => [
                0xdd, 0xc1, 0x2b, 0xb2, 0x89, 0x17, 0x5f, 0xf5, 0x87, 0xa5, 0x70, 0xb3, 0x02, 0xaf,
                0x61, 0x08, 0xfa, 0x56, 0x5b, 0xca, 0xce, 0x9c, 0x37, 0x4a, 0x10, 0xb7, 0x83, 0x63,
                0x95, 0x1d, 0xaa, 0x64,
            ],
            Elements::GejGeAddEx => [
                0x4a, 0x89, 0x02, 0xb1, 0x1d, 0x73, 0x9b, 0x4f, 0xb8, 0x48, 0xd1, 0x85, 0x28, 0x73,
                0x6a, 0xb1, 0xd6, 0x68, 0xbe, 0x2e, 0xab, 0x1f, 0xed, 0x8d, 0x06, 0x83, 0x65, 0xb5,
                0x3b, 0xd7, 0x7d, 0x88,
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
                0x93, 0xc2, 0x40, 0xe4, 0x07, 0xb1, 0xb6, 0xa8, 0xc2, 0xf7, 0x87, 0xf9, 0x3c, 0xa9,
                0x92, 0x92, 0xc0, 0xe6, 0xb2, 0x48, 0x5a, 0x88, 0x2b, 0xfe, 0xac, 0xf9, 0x87, 0x8d,
                0xc3, 0xb1, 0xb9, 0xbe,
            ],
            Elements::GenesisBlockHash => [
                0x8a, 0x38, 0x03, 0xed, 0x96, 0xeb, 0xf9, 0x6c, 0x5b, 0xc9, 0x9d, 0x7d, 0xdc, 0x31,
                0x87, 0xd7, 0x94, 0x00, 0x41, 0x01, 0x90, 0x56, 0x41, 0x4f, 0x86, 0x96, 0x06, 0x1d,
                0x97, 0x79, 0x13, 0xc2,
            ],
            Elements::InputAmount => [
                0x6e, 0x0a, 0x61, 0xfa, 0xae, 0xbc, 0x2c, 0xf4, 0x22, 0x0d, 0xea, 0x6c, 0xc5, 0x9c,
                0x46, 0xe8, 0x0c, 0x90, 0xc4, 0x8e, 0x9b, 0xd0, 0xe4, 0xe5, 0xf6, 0x3b, 0x04, 0xe1,
                0xec, 0xdb, 0xbf, 0xdf,
            ],
            Elements::InputAmountsHash => [
                0xb5, 0x49, 0x6e, 0x58, 0xc7, 0xcb, 0xeb, 0xe4, 0x46, 0x55, 0x20, 0xda, 0x8f, 0x43,
                0x5d, 0xad, 0x29, 0x4e, 0xb4, 0x22, 0x41, 0x25, 0x91, 0xeb, 0x4b, 0xdd, 0xc9, 0x1a,
                0xde, 0x10, 0x0b, 0xbe,
            ],
            Elements::InputAnnexHash => [
                0xde, 0x1a, 0x81, 0x80, 0xbc, 0x9b, 0x81, 0x41, 0x4a, 0x86, 0xd3, 0x12, 0x67, 0x1f,
                0x6a, 0x57, 0x13, 0x2f, 0xec, 0x6b, 0xbc, 0xd4, 0x3f, 0x9f, 0xf5, 0xc0, 0x77, 0x37,
                0xea, 0xbe, 0x59, 0x5f,
            ],
            Elements::InputAnnexesHash => [
                0xf0, 0x25, 0xdf, 0x99, 0x6d, 0x2e, 0x02, 0x71, 0x9c, 0xd4, 0x91, 0xea, 0xd3, 0x31,
                0xa1, 0x73, 0xd0, 0x1b, 0x17, 0xac, 0xd3, 0x82, 0xcf, 0x29, 0xa7, 0xab, 0x56, 0x5a,
                0x04, 0x8d, 0xe4, 0xaf,
            ],
            Elements::InputAsset => [
                0x7d, 0xa2, 0x46, 0xf6, 0xf7, 0xe0, 0x83, 0x17, 0x2e, 0xb7, 0xc7, 0x0d, 0x81, 0xba,
                0xe5, 0x57, 0x2f, 0x2c, 0x10, 0x26, 0x4d, 0x46, 0xf2, 0x5c, 0x68, 0x27, 0xe4, 0x42,
                0xe3, 0x66, 0xe0, 0x18,
            ],
            Elements::InputOutpointsHash => [
                0x7b, 0x79, 0xe2, 0xd8, 0x1e, 0x53, 0xcf, 0x14, 0x12, 0xb9, 0x36, 0x38, 0x6b, 0xde,
                0xbf, 0x4d, 0x94, 0x24, 0x3d, 0xc0, 0xd1, 0x8b, 0xb6, 0x84, 0x17, 0x3a, 0xb0, 0xc4,
                0xd2, 0x19, 0xd9, 0x7c,
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
                0x24, 0xf9, 0x10, 0x2f, 0xc5, 0x19, 0xf8, 0xca, 0x4d, 0x14, 0x02, 0x74, 0xec, 0xb8,
                0xff, 0xe9, 0x83, 0x13, 0x11, 0x43, 0xa0, 0x87, 0x9f, 0x7e, 0x64, 0xd1, 0x1a, 0xa3,
                0x4b, 0xc0, 0x0a, 0xaf,
            ],
            Elements::InputScriptsHash => [
                0x24, 0x2a, 0xff, 0x32, 0xac, 0x3d, 0x3b, 0x21, 0x6c, 0xe2, 0xbc, 0x30, 0xbc, 0xea,
                0x13, 0x5c, 0xb7, 0xea, 0x86, 0xd5, 0xa4, 0xec, 0x92, 0xe0, 0x8a, 0x9f, 0x52, 0x42,
                0x98, 0x38, 0x29, 0x33,
            ],
            Elements::InputSequence => [
                0x27, 0x43, 0x07, 0x36, 0xfe, 0xd8, 0x57, 0x46, 0x33, 0xa6, 0xde, 0x9a, 0x19, 0x8b,
                0x3c, 0x63, 0xff, 0xf8, 0x0f, 0x18, 0xd8, 0x5a, 0xef, 0x70, 0x6c, 0x64, 0x46, 0x9e,
                0x35, 0xc3, 0x99, 0x89,
            ],
            Elements::InputSequencesHash => [
                0x2b, 0x7d, 0xe4, 0xcc, 0xb8, 0x0b, 0xa2, 0xb6, 0x77, 0x04, 0x31, 0x86, 0xed, 0xc3,
                0x98, 0x30, 0xf9, 0x84, 0xb3, 0x3e, 0xad, 0xa9, 0x97, 0x4e, 0x16, 0xdc, 0x62, 0x0f,
                0x52, 0x98, 0x52, 0xe4,
            ],
            Elements::InputUtxosHash => [
                0xd4, 0x07, 0x70, 0x0a, 0xa9, 0xa9, 0x53, 0x7b, 0x9c, 0x7b, 0x56, 0x61, 0x4f, 0x71,
                0x7a, 0xcb, 0xb9, 0xe8, 0xf5, 0xeb, 0xe3, 0xbb, 0x7f, 0xf7, 0x10, 0x97, 0xa2, 0x2d,
                0xdf, 0x5a, 0xb7, 0x55,
            ],
            Elements::InputsHash => [
                0x7e, 0x54, 0x38, 0x35, 0x05, 0x07, 0xab, 0x6e, 0x56, 0x49, 0xa4, 0xf5, 0x56, 0x6c,
                0xd0, 0xd1, 0x0c, 0x97, 0x13, 0x57, 0xc3, 0x5a, 0xd1, 0x6b, 0xc4, 0xf9, 0x68, 0xfa,
                0xf5, 0xa9, 0x17, 0x98,
            ],
            Elements::InternalKey => [
                0xbd, 0x8e, 0x8c, 0x3a, 0xf9, 0xe2, 0x3c, 0xa1, 0x92, 0x52, 0x51, 0xd0, 0xc1, 0xbf,
                0xd8, 0x9b, 0x06, 0xe1, 0xe9, 0xf5, 0x23, 0x88, 0xdd, 0x64, 0x67, 0x87, 0xe1, 0x5b,
                0xe2, 0x16, 0xc7, 0x66,
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
                0xdb, 0xe7, 0xa6, 0xb1, 0xbd, 0x03, 0xcb, 0xff, 0x7c, 0xf3, 0x3b, 0x9c, 0x04, 0x3e,
                0x37, 0x34, 0xdd, 0x4e, 0x11, 0xa8, 0x4b, 0x53, 0x17, 0x6d, 0x2e, 0x28, 0x30, 0x4e,
                0x3d, 0x5f, 0xca, 0x61,
            ],
            Elements::IssuanceAssetProof => [
                0x41, 0xb3, 0x37, 0xce, 0x07, 0xad, 0x73, 0xc0, 0x6a, 0x29, 0xf0, 0xc6, 0xa0, 0xfd,
                0xd0, 0x8a, 0x14, 0xf3, 0x33, 0xfd, 0x52, 0x33, 0xaf, 0x84, 0x94, 0xb0, 0x27, 0x3c,
                0x13, 0x01, 0x40, 0xb9,
            ],
            Elements::IssuanceBlindingEntropyHash => [
                0x26, 0x71, 0x65, 0x02, 0x37, 0x62, 0xf3, 0x88, 0x26, 0xc6, 0xcb, 0xa5, 0xd2, 0x91,
                0x00, 0xd6, 0xc4, 0xd8, 0xdb, 0xf2, 0x88, 0xc5, 0x94, 0x86, 0xf7, 0x84, 0x39, 0x83,
                0x60, 0x14, 0x44, 0x61,
            ],
            Elements::IssuanceEntropy => [
                0x08, 0x5c, 0xa1, 0x03, 0x54, 0xa3, 0xd3, 0xdb, 0xf0, 0x94, 0x19, 0xf7, 0xba, 0x57,
                0x7a, 0xda, 0xf0, 0xec, 0x6c, 0x0b, 0xa2, 0xad, 0x7f, 0xa9, 0x2e, 0x75, 0x24, 0xb7,
                0xda, 0x71, 0xa4, 0x08,
            ],
            Elements::IssuanceRangeProofsHash => [
                0xe3, 0x41, 0xc8, 0x23, 0x8f, 0x20, 0x60, 0x3d, 0xcd, 0x52, 0xc7, 0x84, 0xe1, 0x6f,
                0x0e, 0x2b, 0xb2, 0xa0, 0xf4, 0xac, 0x87, 0x9f, 0xca, 0x4a, 0x9d, 0xb0, 0x1a, 0x92,
                0x20, 0x56, 0xf1, 0x15,
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
                0x84, 0xf5, 0xaf, 0x14, 0x88, 0x21, 0xd4, 0xcd, 0x55, 0x5d, 0xf2, 0x26, 0xdf, 0x82,
                0x94, 0x1e, 0x25, 0x5c, 0x30, 0xa3, 0x2b, 0x03, 0xcf, 0x75, 0x9b, 0x83, 0x2a, 0x03,
                0xf7, 0xf9, 0x08, 0xd9,
            ],
            Elements::IssuanceTokenProof => [
                0xdf, 0x95, 0x35, 0xf1, 0x65, 0x23, 0xaa, 0xca, 0xa7, 0x60, 0x59, 0x9f, 0x2e, 0xd8,
                0xee, 0x4a, 0xbf, 0x16, 0xce, 0x97, 0xfd, 0x2d, 0xcb, 0x9c, 0xdd, 0x4f, 0xeb, 0x53,
                0x89, 0x74, 0x8a, 0x67,
            ],
            Elements::IssuancesHash => [
                0x07, 0xbb, 0x13, 0x34, 0x76, 0x5e, 0x19, 0x24, 0x8f, 0x4c, 0x11, 0x61, 0xc5, 0x99,
                0xd1, 0x5f, 0x0b, 0xcc, 0x99, 0x5f, 0xa2, 0x3b, 0xec, 0x69, 0xa8, 0xf9, 0xbc, 0xcc,
                0x44, 0x62, 0x28, 0x1d,
            ],
            Elements::Le32 => [
                0xea, 0x77, 0x2c, 0x0b, 0x5a, 0xec, 0xde, 0x7d, 0xc1, 0x6e, 0x3f, 0x1f, 0x95, 0x27,
                0x89, 0xf0, 0x13, 0x70, 0x89, 0x04, 0xce, 0xaa, 0x62, 0xcf, 0xdc, 0xf6, 0x4d, 0x30,
                0xa3, 0x91, 0x17, 0xaf,
            ],
            Elements::LinearCombination1 => [
                0x50, 0x49, 0xe2, 0x71, 0xce, 0xf0, 0x93, 0x75, 0xb6, 0x80, 0x52, 0xc0, 0x45, 0xdf,
                0x00, 0xfd, 0xc0, 0x2d, 0x10, 0xbe, 0xe0, 0x1c, 0xbf, 0x62, 0x46, 0xeb, 0xbc, 0xc5,
                0xec, 0xa5, 0xf1, 0x44,
            ],
            Elements::LinearVerify1 => [
                0x46, 0xe3, 0x7c, 0x8c, 0xd9, 0xde, 0xf4, 0xf6, 0xf8, 0x53, 0x78, 0x5b, 0x91, 0xdf,
                0x8b, 0xb5, 0x3b, 0x2e, 0x61, 0x02, 0xcf, 0x88, 0xc8, 0xa6, 0xb8, 0xd6, 0x10, 0x85,
                0xa2, 0x3c, 0x5f, 0xec,
            ],
            Elements::LockTime => [
                0x67, 0x3a, 0x98, 0x79, 0xc3, 0x37, 0xf7, 0x69, 0x63, 0xbb, 0xb6, 0xcd, 0x9a, 0x42,
                0x4e, 0x69, 0x24, 0x37, 0xc3, 0xb0, 0x2e, 0x8d, 0xb0, 0x12, 0xa2, 0x03, 0x12, 0x5b,
                0xb9, 0x65, 0xe2, 0x07,
            ],
            Elements::Low32 => [
                0x96, 0xbe, 0x53, 0x34, 0x5d, 0x52, 0x14, 0xb0, 0x05, 0xc7, 0xfc, 0x5d, 0xe4, 0x0a,
                0x92, 0x62, 0x56, 0x60, 0x41, 0x40, 0x63, 0x35, 0x27, 0xc1, 0xd0, 0x2b, 0xe0, 0xd5,
                0xf6, 0x0c, 0xc2, 0xad,
            ],
            Elements::Multiply32 => [
                0x16, 0x1f, 0xd0, 0x3a, 0x92, 0xc6, 0x21, 0xb3, 0x28, 0x98, 0x49, 0xff, 0x29, 0xad,
                0x81, 0x34, 0x99, 0xd6, 0x3e, 0xd9, 0x73, 0xdb, 0x0e, 0x97, 0x51, 0x78, 0x54, 0x21,
                0xf5, 0x68, 0xd1, 0x8f,
            ],
            Elements::NewIssuanceContract => [
                0x8d, 0x58, 0xd7, 0x7a, 0x39, 0xe4, 0xf1, 0x2d, 0x60, 0x2e, 0x9c, 0x01, 0x2b, 0x0a,
                0xcf, 0x2a, 0xb0, 0xd9, 0x1c, 0xbb, 0x1f, 0x12, 0x0d, 0x8c, 0x25, 0x4f, 0x97, 0x0c,
                0x3b, 0xe1, 0x55, 0xc9,
            ],
            Elements::NonceHash => [
                0x92, 0xf8, 0xc9, 0x26, 0xa3, 0x59, 0x16, 0x43, 0x1e, 0x3e, 0x47, 0x21, 0xf7, 0xca,
                0xbd, 0x52, 0xd0, 0xe6, 0x55, 0x8d, 0xa5, 0x8a, 0xe7, 0xa9, 0xaf, 0x6a, 0xe4, 0x3f,
                0x16, 0x27, 0x29, 0x62,
            ],
            Elements::NumInputs => [
                0x62, 0xe1, 0xc7, 0xbe, 0xc3, 0x9f, 0x56, 0xf2, 0x72, 0xf3, 0x35, 0x23, 0x40, 0x74,
                0x06, 0xec, 0x2d, 0x35, 0x19, 0x7a, 0x68, 0x83, 0x07, 0x10, 0x78, 0x8b, 0x26, 0x91,
                0xc4, 0x1e, 0x13, 0xea,
            ],
            Elements::NumOutputs => [
                0xf1, 0x67, 0x6e, 0xf7, 0xb7, 0xd0, 0xc3, 0x8e, 0x16, 0xc9, 0xfd, 0x1b, 0x6e, 0x91,
                0x61, 0x4e, 0x12, 0x77, 0xd8, 0x7c, 0x55, 0x17, 0x1f, 0xd4, 0xe0, 0x8d, 0x5b, 0xac,
                0xd3, 0xd7, 0xec, 0x46,
            ],
            Elements::One32 => [
                0x45, 0x26, 0x09, 0x9d, 0x0f, 0x84, 0x0b, 0xa6, 0xab, 0xe5, 0x5e, 0xd9, 0xf3, 0xa9,
                0xb8, 0xfc, 0xa0, 0x84, 0xf2, 0x51, 0xbf, 0xb2, 0x6d, 0x01, 0x1c, 0xf7, 0xaf, 0x27,
                0xb0, 0xd8, 0x8e, 0x29,
            ],
            Elements::OutpointHash => [
                0x29, 0xc5, 0x9c, 0xe6, 0xff, 0x4f, 0xfb, 0x32, 0xa9, 0x56, 0xa2, 0xe3, 0xd0, 0x1f,
                0xf7, 0x0a, 0x93, 0x10, 0xd7, 0x0c, 0x85, 0x0c, 0xa5, 0x05, 0x6d, 0xd3, 0x60, 0x3b,
                0xe1, 0xb0, 0xc3, 0xb2,
            ],
            Elements::OutputAmount => [
                0x1d, 0x89, 0xd5, 0x74, 0x43, 0x88, 0x3c, 0x2f, 0x75, 0x58, 0x77, 0x79, 0x77, 0xc6,
                0x48, 0x00, 0x8d, 0x80, 0x4b, 0xd7, 0x4d, 0x83, 0x28, 0x86, 0xba, 0x78, 0x2a, 0x79,
                0x2a, 0x42, 0xeb, 0x97,
            ],
            Elements::OutputAmountsHash => [
                0x66, 0x42, 0x0c, 0x81, 0x6d, 0xc6, 0xc5, 0xc7, 0xc0, 0x1b, 0x39, 0x7d, 0x8a, 0x76,
                0x7a, 0xd2, 0xfc, 0xf7, 0x8f, 0x46, 0x46, 0x36, 0xf8, 0x2f, 0xcd, 0xdb, 0x1f, 0x9f,
                0x6c, 0xe2, 0x6f, 0x85,
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
                0xaa, 0x72, 0xb3, 0xe6, 0xbc, 0x0d, 0x1e, 0x90, 0x85, 0xf6, 0x41, 0xae, 0x78, 0xa9,
                0x7d, 0x6b, 0x59, 0xad, 0x91, 0xc4, 0x4b, 0x19, 0x13, 0xdd, 0x8d, 0x62, 0xc6, 0x63,
                0x5b, 0x0e, 0x8d, 0x35,
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
                0x74, 0x8e, 0x97, 0x28, 0xbf, 0x3e, 0x54, 0x48, 0x65, 0xfb, 0xb2, 0x48, 0x9f, 0x5a,
                0x5e, 0x1f, 0x97, 0xc9, 0x60, 0x8c, 0xe6, 0x12, 0xfb, 0x04, 0xcf, 0x17, 0xef, 0x1c,
                0x44, 0x22, 0xa3, 0x1c,
            ],
            Elements::OutputScriptHash => [
                0xfd, 0x0c, 0xeb, 0x21, 0x13, 0x9d, 0x07, 0x77, 0xa4, 0xa6, 0x1f, 0x05, 0xa5, 0xdc,
                0x1f, 0xda, 0xa7, 0x61, 0xf6, 0xfd, 0xf2, 0x54, 0x78, 0x12, 0x2b, 0xe9, 0xe0, 0x60,
                0xea, 0x21, 0x86, 0x35,
            ],
            Elements::OutputScriptsHash => [
                0x74, 0x59, 0x3e, 0x1d, 0xb1, 0x63, 0x0a, 0xf9, 0xf1, 0xa0, 0x95, 0xd2, 0x0f, 0x44,
                0x05, 0xe0, 0xea, 0x8a, 0x90, 0xe9, 0x8b, 0x72, 0x7b, 0x9a, 0xf1, 0x9f, 0x8c, 0x47,
                0x2c, 0xb7, 0xe9, 0x1f,
            ],
            Elements::OutputSurjectionProof => [
                0x10, 0x99, 0x78, 0x18, 0xdc, 0x55, 0xd3, 0x95, 0xc9, 0xdf, 0x59, 0x97, 0x93, 0xeb,
                0xdb, 0xca, 0xcb, 0x27, 0x7c, 0x70, 0x10, 0x4f, 0x83, 0xa0, 0x04, 0x96, 0x88, 0x67,
                0xe8, 0x94, 0xb6, 0xf9,
            ],
            Elements::OutputSurjectionProofsHash => [
                0x68, 0x1e, 0x44, 0xcb, 0xcc, 0xcc, 0x5e, 0x20, 0x5d, 0xa4, 0x23, 0x37, 0x54, 0xca,
                0x57, 0xd5, 0x73, 0x08, 0xaf, 0x6b, 0x06, 0xda, 0xec, 0x3e, 0x8f, 0xa5, 0x74, 0x1e,
                0xcf, 0x83, 0x70, 0xca,
            ],
            Elements::OutputsHash => [
                0x3c, 0xa8, 0xf6, 0xc1, 0xcf, 0xa4, 0xf7, 0xcf, 0x01, 0x98, 0x37, 0x2a, 0x48, 0x19,
                0xdf, 0xb1, 0x17, 0xf3, 0x8f, 0xa6, 0x38, 0xd0, 0x50, 0x33, 0x89, 0xf1, 0x25, 0xca,
                0xb7, 0xd6, 0x06, 0x19,
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
                0x91, 0x91, 0x22, 0x37, 0x47, 0x17, 0x87, 0xc5, 0xff, 0x2b, 0xdb, 0xd9, 0x41, 0xbc,
                0xeb, 0xda, 0xee, 0x90, 0xa2, 0x25, 0x24, 0x3c, 0x6a, 0x86, 0x85, 0xdd, 0x5f, 0x81,
                0x22, 0xdc, 0x66, 0xba,
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
                0xa1, 0x8f, 0xe4, 0x0d, 0x7a, 0x5c, 0xea, 0x9a, 0x02, 0x88, 0xb7, 0xbd, 0xba, 0xa1,
                0xbf, 0xad, 0x4f, 0x47, 0x5f, 0x4d, 0x1a, 0xd7, 0x7a, 0x3a, 0x88, 0x3f, 0xa3, 0xc8,
                0x22, 0xd6, 0x88, 0x38,
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
                0x2e, 0xcd, 0x3d, 0xbe, 0x41, 0xe1, 0xfb, 0x41, 0x7d, 0x24, 0x21, 0xdd, 0x07, 0xef,
                0x6e, 0x6e, 0xdb, 0xad, 0xb8, 0x82, 0x69, 0x81, 0x61, 0xeb, 0xcc, 0xf6, 0xca, 0xf0,
                0xee, 0xb9, 0x98, 0x30,
            ],
            Elements::Sha256Ctx8Add128 => [
                0x3d, 0x5d, 0x67, 0xe2, 0x7e, 0xfc, 0x25, 0xa7, 0x2d, 0xa6, 0x18, 0x6d, 0x2a, 0x6f,
                0xa1, 0x06, 0xe7, 0x57, 0x43, 0x57, 0xb1, 0xf8, 0x76, 0xfa, 0x68, 0x79, 0xa6, 0x11,
                0x36, 0x82, 0xb7, 0x35,
            ],
            Elements::Sha256Ctx8Add16 => [
                0x7b, 0xa6, 0xef, 0xb6, 0x92, 0xa1, 0x91, 0xe4, 0xdd, 0xd7, 0xa5, 0x84, 0x1d, 0x5a,
                0x06, 0x3d, 0x6a, 0xa3, 0xf0, 0xcf, 0xcd, 0x16, 0xfd, 0x84, 0xde, 0xff, 0x5d, 0x1f,
                0x3b, 0x3b, 0x08, 0xcb,
            ],
            Elements::Sha256Ctx8Add2 => [
                0x12, 0x3c, 0x7d, 0x8c, 0xe5, 0x39, 0xd8, 0xab, 0xc6, 0xda, 0xc5, 0xf9, 0x9c, 0xe8,
                0x23, 0x39, 0xb9, 0x56, 0x41, 0x68, 0xe6, 0x56, 0x43, 0x37, 0x28, 0x5e, 0x89, 0x82,
                0x4a, 0x23, 0x9f, 0x33,
            ],
            Elements::Sha256Ctx8Add256 => [
                0x76, 0xe2, 0xb4, 0xfc, 0xa5, 0x4b, 0x4d, 0x7a, 0x39, 0x0b, 0x92, 0xb0, 0xf0, 0xe1,
                0x20, 0x49, 0x1d, 0xe8, 0x6d, 0xfe, 0xa6, 0xe9, 0x62, 0x58, 0x80, 0x97, 0x23, 0x92,
                0xb1, 0xf5, 0x38, 0x4c,
            ],
            Elements::Sha256Ctx8Add32 => [
                0xa4, 0xd6, 0x39, 0xe4, 0xec, 0x73, 0x39, 0x9d, 0x31, 0x93, 0x9c, 0xe3, 0xc0, 0x25,
                0x06, 0xd1, 0xd2, 0x3a, 0x20, 0x29, 0x3b, 0xb9, 0x5a, 0xb3, 0x7c, 0xb8, 0x7d, 0xa2,
                0xb1, 0x87, 0xc6, 0xf8,
            ],
            Elements::Sha256Ctx8Add4 => [
                0x8a, 0x41, 0xbf, 0xb6, 0x24, 0x29, 0x5a, 0x3d, 0x8f, 0x67, 0xdc, 0x5b, 0xda, 0x5e,
                0x7c, 0x7d, 0x97, 0xc9, 0xcc, 0x30, 0x21, 0xd3, 0x81, 0xce, 0x8b, 0x04, 0x7b, 0x3d,
                0x79, 0xd1, 0x87, 0xc9,
            ],
            Elements::Sha256Ctx8Add512 => [
                0x2c, 0x80, 0x99, 0x15, 0x75, 0x65, 0x4b, 0x3d, 0x81, 0x65, 0x8d, 0xca, 0xde, 0xa5,
                0x19, 0x49, 0xcc, 0x18, 0x25, 0x30, 0xaf, 0x7e, 0x9f, 0x41, 0xee, 0x5b, 0x93, 0xfc,
                0x33, 0xf0, 0xc8, 0x0f,
            ],
            Elements::Sha256Ctx8Add64 => [
                0x10, 0xae, 0x93, 0x7f, 0x66, 0x97, 0x00, 0xd3, 0x21, 0x55, 0x01, 0x53, 0xd4, 0x60,
                0x25, 0x20, 0x8c, 0x03, 0x70, 0xda, 0x86, 0x6d, 0xd3, 0xd4, 0xc9, 0xb3, 0x4d, 0x42,
                0x2c, 0xcc, 0x2c, 0xc8,
            ],
            Elements::Sha256Ctx8Add8 => [
                0x48, 0xd6, 0x97, 0x4c, 0xd3, 0x6c, 0x37, 0x6c, 0x4b, 0xd2, 0x60, 0xd1, 0x1f, 0xf6,
                0x2e, 0x74, 0x20, 0x76, 0x93, 0x4e, 0xc3, 0x10, 0x8d, 0x9f, 0x8f, 0xf2, 0x7f, 0xb4,
                0x54, 0xe4, 0x94, 0x5f,
            ],
            Elements::Sha256Ctx8AddBuffer511 => [
                0x3d, 0x59, 0x9f, 0x24, 0xf3, 0xb1, 0x53, 0x3c, 0x86, 0x64, 0xc4, 0x05, 0x04, 0xdc,
                0x39, 0xbf, 0xc5, 0x75, 0x6f, 0x2d, 0x9a, 0x6a, 0x4d, 0x0d, 0x65, 0xbe, 0x29, 0x9e,
                0x88, 0xb4, 0xa6, 0xd5,
            ],
            Elements::Sha256Ctx8Finalize => [
                0x8c, 0x44, 0xd9, 0x30, 0x4f, 0x8f, 0x85, 0xe8, 0x56, 0x64, 0x90, 0x63, 0xf7, 0x84,
                0x2a, 0xc5, 0xa6, 0xe8, 0x0f, 0xd3, 0x7a, 0x01, 0x41, 0xc9, 0xff, 0xde, 0x74, 0x2f,
                0xbe, 0x8c, 0x9f, 0xf8,
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
                0x7e, 0x98, 0x89, 0x37, 0x0b, 0xfb, 0x13, 0x55, 0xa0, 0x6b, 0xd3, 0x1a, 0xc2, 0x60,
                0xda, 0xfc, 0xa8, 0x21, 0xdc, 0xce, 0xa8, 0x87, 0x43, 0x19, 0xc4, 0x14, 0x91, 0x4e,
                0x65, 0xc6, 0xda, 0x8a,
            ],
            Elements::Subtract32 => [
                0xf7, 0x6e, 0xca, 0xd1, 0xfd, 0xa5, 0x0f, 0x13, 0x5b, 0xdf, 0xe3, 0xe5, 0x33, 0xa1,
                0x50, 0x09, 0x8f, 0x40, 0x62, 0x61, 0xc7, 0x6f, 0x6d, 0xbf, 0x67, 0x25, 0xf1, 0xe3,
                0x88, 0x3c, 0x2a, 0xe2,
            ],
            Elements::TapEnvHash => [
                0xa2, 0x18, 0xf1, 0xb2, 0x5e, 0x78, 0xc6, 0xa8, 0xf0, 0xf4, 0x63, 0xd4, 0x0d, 0xbe,
                0xbc, 0x4a, 0x1b, 0x09, 0x35, 0x70, 0x63, 0x95, 0xee, 0xb4, 0x20, 0x83, 0x85, 0x7e,
                0xa5, 0x1b, 0x2d, 0x1b,
            ],
            Elements::Tapbranch => [
                0x48, 0xe9, 0x04, 0xd2, 0x7d, 0x9b, 0x04, 0xb5, 0x40, 0x8d, 0xa2, 0xc8, 0x8f, 0xca,
                0x09, 0x68, 0x2d, 0x19, 0xcf, 0xf5, 0x64, 0xcb, 0xe5, 0x2e, 0xd1, 0xe5, 0x5b, 0x91,
                0x26, 0xb2, 0x1e, 0x3b,
            ],
            Elements::TapbranchHash => [
                0xcf, 0xf7, 0x51, 0x4a, 0xee, 0x34, 0x9b, 0x39, 0x30, 0x08, 0xef, 0x3a, 0xe3, 0x5c,
                0x11, 0x08, 0xc8, 0x96, 0x8e, 0x29, 0xa2, 0x67, 0x8c, 0x49, 0xe0, 0x0d, 0xbb, 0xbf,
                0x37, 0x83, 0x64, 0x1a,
            ],
            Elements::TapleafHash => [
                0x71, 0xe4, 0xb5, 0x58, 0xf6, 0x18, 0x7b, 0x6c, 0xbf, 0x57, 0x22, 0xd3, 0x78, 0x8f,
                0x6c, 0x88, 0x9a, 0xff, 0x4e, 0x93, 0x8b, 0x9c, 0x9e, 0x1c, 0x59, 0x24, 0x7f, 0xea,
                0xfa, 0x7e, 0xdd, 0xa0,
            ],
            Elements::TapleafVersion => [
                0x55, 0xa2, 0xf5, 0x15, 0x24, 0x71, 0x1d, 0xfa, 0xda, 0xc0, 0x21, 0x58, 0xb3, 0x63,
                0x4a, 0x66, 0x5a, 0xd4, 0x84, 0xcd, 0x7f, 0x3a, 0x6a, 0x30, 0x0d, 0x33, 0x64, 0x24,
                0xe1, 0x44, 0xd3, 0x35,
            ],
            Elements::TxHash => [
                0x38, 0x4c, 0xa8, 0xdd, 0xec, 0xbf, 0xa0, 0xa9, 0xcf, 0xc4, 0x01, 0x1a, 0x3c, 0x46,
                0x40, 0xb0, 0x0f, 0x1b, 0xe3, 0x57, 0xf0, 0x2b, 0xca, 0x03, 0x5d, 0xfb, 0x6e, 0x15,
                0x73, 0x95, 0xb9, 0xd8,
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
            Elements::Add32 => b"l",
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
            Elements::Eq256 => b"*hh",
            Elements::Eq32 => b"l",
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
            Elements::FullAdd32 => b"*2l",
            Elements::FullMultiply32 => b"*ll",
            Elements::FullSubtract32 => b"*2l",
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
            Elements::Le32 => b"l",
            Elements::LinearCombination1 => b"**h**hhhh",
            Elements::LinearVerify1 => b"***h*hhh*hh",
            Elements::LockTime => b"1",
            Elements::Low32 => b"1",
            Elements::Multiply32 => b"l",
            Elements::NewIssuanceContract => b"i",
            Elements::NonceHash => b"***+1h*+1*ll*+1l*+1i*+1****22*22**22*22***22*22**22*22+1***22*22**22*22*lh+1+*2hh",
            Elements::NumInputs => b"1",
            Elements::NumOutputs => b"1",
            Elements::One32 => b"1",
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
            Elements::Subtract32 => b"l",
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
            Elements::Add32 => b"*2i",
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
            Elements::Eq256 => b"2",
            Elements::Eq32 => b"2",
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
            Elements::FullAdd32 => b"*2i",
            Elements::FullMultiply32 => b"l",
            Elements::FullSubtract32 => b"*2i",
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
            Elements::Le32 => b"2",
            Elements::LinearCombination1 => b"**hhh",
            Elements::LinearVerify1 => b"1",
            Elements::LockTime => b"i",
            Elements::Low32 => b"i",
            Elements::Multiply32 => b"l",
            Elements::NewIssuanceContract => b"+1+1h",
            Elements::NonceHash => b"**+1h*+1*ll*+1l*+1i*+1****22*22**22*22***22*22**22*22+1***22*22**22*22*lh",
            Elements::NumInputs => b"i",
            Elements::NumOutputs => b"i",
            Elements::One32 => b"i",
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
            Elements::Subtract32 => b"*2i",
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
            Elements::Low32 => (305, 11),
            Elements::Eq32 => (7025, 15),
            Elements::Eq256 => (14056, 16),
            Elements::One32 => (561, 11),
            Elements::FullAdd32 => (2353, 13),
            Elements::Add32 => (2417, 13),
            Elements::FullSubtract32 => (19697, 16),
            Elements::Subtract32 => (39473, 17),
            Elements::FullMultiply32 => (39729, 17),
            Elements::Multiply32 => (39793, 17),
            Elements::Le32 => (639025, 21),
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
                                    0 => {},
                                    1 => {
                                        0 => {
                                            0 => {
                                                0 => {
                                                    0 => {},
                                                    1 => {Elements::Low32}
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
                                                    0 => {},
                                                    1 => {
                                                        0 => {
                                                            0 => {
                                                                0 => {
                                                                    0 => {},
                                                                    1 => {Elements::Eq32}
                                                                },
                                                                1 => {}
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
                                    0 => {},
                                    1 => {
                                        0 => {
                                            0 => {
                                                0 => {
                                                    0 => {},
                                                    1 => {Elements::One32}
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
                                0 => {
                                    0 => {
                                        0 => {},
                                        1 => {
                                            0 => {},
                                            1 => {
                                                0 => {
                                                    0 => {
                                                        0 => {
                                                            0 => {},
                                                            1 => {Elements::FullAdd32}
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
                                        0 => {},
                                        1 => {
                                            0 => {},
                                            1 => {
                                                0 => {
                                                    0 => {
                                                        0 => {
                                                            0 => {},
                                                            1 => {Elements::Add32}
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
                                            0 => {},
                                            1 => {
                                                0 => {},
                                                1 => {
                                                    0 => {},
                                                    1 => {
                                                        0 => {},
                                                        1 => {
                                                            0 => {
                                                                0 => {
                                                                    0 => {
                                                                        0 => {},
                                                                        1 => {Elements::FullSubtract32}
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
                                        1 => {
                                            0 => {
                                                0 => {
                                                    0 => {
                                                        0 => {},
                                                        1 => {
                                                            0 => {},
                                                            1 => {
                                                                0 => {
                                                                    0 => {
                                                                        0 => {
                                                                            0 => {},
                                                                            1 => {Elements::Subtract32}
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
                                            1 => {
                                                0 => {
                                                    0 => {
                                                        0 => {},
                                                        1 => {
                                                            0 => {},
                                                            1 => {
                                                                0 => {
                                                                    0 => {
                                                                        0 => {
                                                                            0 => {},
                                                                            1 => {Elements::FullMultiply32}
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
                                                        0 => {},
                                                        1 => {
                                                            0 => {},
                                                            1 => {
                                                                0 => {
                                                                    0 => {
                                                                        0 => {
                                                                            0 => {},
                                                                            1 => {Elements::Multiply32}
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
                                                                            0 => {},
                                                                            1 => {
                                                                                0 => {
                                                                                    0 => {
                                                                                        0 => {
                                                                                            0 => {},
                                                                                            1 => {Elements::Le32}
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
                                                        },
                                                        1 => {}
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
            Elements::Add32 => &simplicity_sys::c_jets::jets_wrapper::add_32,
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
            Elements::Eq256 => &simplicity_sys::c_jets::jets_wrapper::eq_256,
            Elements::Eq32 => &simplicity_sys::c_jets::jets_wrapper::eq_32,
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
            Elements::FullAdd32 => &simplicity_sys::c_jets::jets_wrapper::full_add_32,
            Elements::FullMultiply32 => &simplicity_sys::c_jets::jets_wrapper::full_multiply_32,
            Elements::FullSubtract32 => &simplicity_sys::c_jets::jets_wrapper::full_subtract_32,
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
            Elements::Le32 => &simplicity_sys::c_jets::jets_wrapper::le_32,
            Elements::LinearCombination1 => &simplicity_sys::c_jets::jets_wrapper::linear_combination_1,
            Elements::LinearVerify1 => &simplicity_sys::c_jets::jets_wrapper::linear_verify_1,
            Elements::LockTime => &simplicity_sys::c_jets::jets_wrapper::lock_time,
            Elements::Low32 => &simplicity_sys::c_jets::jets_wrapper::low_32,
            Elements::Multiply32 => &simplicity_sys::c_jets::jets_wrapper::multiply_32,
            Elements::NewIssuanceContract => &simplicity_sys::c_jets::jets_wrapper::new_issuance_contract,
            Elements::NonceHash => &simplicity_sys::c_jets::jets_wrapper::nonce_hash,
            Elements::NumInputs => &simplicity_sys::c_jets::jets_wrapper::num_inputs,
            Elements::NumOutputs => &simplicity_sys::c_jets::jets_wrapper::num_outputs,
            Elements::One32 => &simplicity_sys::c_jets::jets_wrapper::one_32,
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
            Elements::Subtract32 => &simplicity_sys::c_jets::jets_wrapper::subtract_32,
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
