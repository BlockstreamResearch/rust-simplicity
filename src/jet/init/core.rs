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

/// Core jet family
#[derive(Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Debug, Hash)]
pub enum Core {
    Add32,
    Bip0340Verify,
    CheckSigVerify,
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
    Le32,
    LinearCombination1,
    LinearVerify1,
    Low32,
    Multiply32,
    One32,
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
    Subtract32,
    Verify,
}

impl Jet for Core {

    type Environment = ();
    type CJetEnvironment = ();

    fn c_jet_env<'env>(&self, env: &'env Self::Environment) -> &'env Self::CJetEnvironment {
        env
    }

    fn cmr(&self) -> Cmr {
        let bytes = match self {
            Core::Add32 => [
                0xe4, 0x04, 0x66, 0xa7, 0xec, 0xf7, 0x1c, 0xe8, 0x62, 0xfb, 0x3c, 0x15, 0x4c, 0x1e,
                0x8f, 0x84, 0x5d, 0x7e, 0x57, 0x07, 0x46, 0x3a, 0x89, 0x45, 0x37, 0xa3, 0x2f, 0xc7,
                0x21, 0x49, 0x00, 0xad,
            ],
            Core::Bip0340Verify => [
                0xc6, 0x19, 0xa5, 0x79, 0x12, 0xb7, 0x6c, 0x30, 0x5d, 0xc9, 0xe2, 0x62, 0xf6, 0x98,
                0xe2, 0x05, 0x06, 0x1f, 0x46, 0x26, 0xc0, 0x70, 0x04, 0x7b, 0x0e, 0x1b, 0xa9, 0x99,
                0xdb, 0xf1, 0x00, 0x6c,
            ],
            Core::CheckSigVerify => [
                0xfe, 0x5e, 0x35, 0xd8, 0xa4, 0xa8, 0x46, 0xa1, 0x05, 0xac, 0x4b, 0x03, 0xfc, 0xe4,
                0xa9, 0x7a, 0x55, 0xbc, 0x40, 0xb5, 0x50, 0x4d, 0x9a, 0x60, 0x3c, 0x23, 0x9b, 0x05,
                0x59, 0xe3, 0x11, 0x02,
            ],
            Core::Decompress => [
                0xab, 0x5a, 0x2d, 0xbc, 0x0f, 0x2c, 0x82, 0x08, 0x6d, 0x2d, 0x46, 0xbb, 0xa5, 0x69,
                0x1f, 0x13, 0x12, 0xba, 0xcc, 0x94, 0x6b, 0x08, 0xef, 0xfb, 0xe7, 0xf8, 0x51, 0x51,
                0x14, 0x1f, 0x7d, 0xcf,
            ],
            Core::Eq256 => [
                0xd8, 0xbb, 0x63, 0x54, 0x45, 0xb5, 0x6b, 0xc0, 0x6a, 0x33, 0x5e, 0x1d, 0xe8, 0xe5,
                0x56, 0xcc, 0x06, 0xf4, 0x93, 0x08, 0x67, 0x4c, 0xb8, 0x71, 0x6a, 0x5b, 0xa8, 0x1c,
                0xce, 0xd7, 0x54, 0x0e,
            ],
            Core::Eq32 => [
                0x88, 0xbf, 0xc8, 0x93, 0x7c, 0x11, 0x87, 0x3b, 0x06, 0xe3, 0xd8, 0xaf, 0xa4, 0xf9,
                0xf9, 0xe6, 0x22, 0xdb, 0x93, 0xf4, 0x70, 0x6b, 0xde, 0x36, 0xbc, 0x60, 0x14, 0x03,
                0x34, 0x48, 0xf3, 0x01,
            ],
            Core::FeAdd => [
                0xc0, 0x9d, 0x58, 0xe3, 0x8d, 0xc4, 0xce, 0x1a, 0xcc, 0x09, 0xda, 0xe8, 0xa5, 0x88,
                0x19, 0x08, 0xfe, 0x1e, 0xbc, 0x7f, 0x1f, 0xc7, 0x42, 0xc6, 0x1c, 0xdc, 0x84, 0x93,
                0xf2, 0x33, 0xb9, 0x4a,
            ],
            Core::FeInvert => [
                0xb6, 0xb1, 0x1b, 0xd6, 0x02, 0x58, 0xd3, 0x26, 0xb1, 0x9b, 0x6f, 0x78, 0x38, 0x7f,
                0xf3, 0xaa, 0xbf, 0x6a, 0x4c, 0x41, 0x9d, 0x0c, 0x5a, 0x8d, 0xda, 0x6b, 0x44, 0x05,
                0x0b, 0xc6, 0xe9, 0xd5,
            ],
            Core::FeIsOdd => [
                0x4d, 0x21, 0xe4, 0xb5, 0x47, 0x6f, 0xcf, 0x56, 0xa3, 0x69, 0xe5, 0xa1, 0x89, 0xc4,
                0x86, 0xe0, 0x98, 0x7f, 0x93, 0x32, 0xaa, 0xf4, 0xc0, 0xa7, 0x5a, 0xc3, 0xaf, 0x09,
                0xf8, 0x1b, 0x17, 0x09,
            ],
            Core::FeIsZero => [
                0x52, 0x35, 0x9f, 0x07, 0xed, 0x7a, 0x7c, 0x97, 0x0e, 0x94, 0x7d, 0xc8, 0xc4, 0x99,
                0x40, 0x56, 0x6c, 0x1a, 0x1e, 0x09, 0x5d, 0x03, 0x60, 0x4a, 0xfd, 0xb4, 0x43, 0x3d,
                0xa1, 0x3a, 0x87, 0xec,
            ],
            Core::FeMultiply => [
                0xac, 0x5a, 0x36, 0x26, 0xb5, 0xfc, 0x2b, 0x5a, 0x20, 0x6a, 0xc1, 0x8f, 0x1b, 0x0f,
                0x9e, 0xca, 0xcb, 0x5c, 0x63, 0x14, 0xc4, 0xef, 0xda, 0x59, 0xe0, 0xfa, 0xd3, 0xa1,
                0xb5, 0x99, 0xa1, 0xbd,
            ],
            Core::FeMultiplyBeta => [
                0xe7, 0xa6, 0x98, 0xe2, 0x5e, 0xbb, 0xf7, 0x0f, 0x8c, 0xed, 0x31, 0x30, 0xac, 0x04,
                0xd6, 0x74, 0xa9, 0xda, 0x39, 0xe0, 0x61, 0x76, 0x1b, 0xfd, 0x9d, 0x87, 0xd7, 0x94,
                0x89, 0x8f, 0x8a, 0x7a,
            ],
            Core::FeNegate => [
                0x87, 0x66, 0x58, 0x5d, 0x27, 0xd1, 0x88, 0x63, 0x42, 0x71, 0x44, 0x43, 0x2b, 0xa4,
                0x83, 0xb3, 0x6c, 0xd2, 0xdd, 0x1f, 0x36, 0x18, 0x14, 0x10, 0xac, 0xc7, 0x14, 0x93,
                0x9c, 0x0c, 0xb5, 0x6a,
            ],
            Core::FeNormalize => [
                0xc0, 0x70, 0xad, 0xba, 0xab, 0x2c, 0x7b, 0xe6, 0xff, 0x57, 0x7a, 0x75, 0x07, 0xaf,
                0xf0, 0xe7, 0x76, 0x57, 0xf3, 0x09, 0xe6, 0x5d, 0x05, 0xfa, 0x23, 0xc1, 0x92, 0x76,
                0xf7, 0x38, 0x52, 0xeb,
            ],
            Core::FeSquare => [
                0x8e, 0x77, 0xcc, 0x8c, 0x63, 0x69, 0x3a, 0x2a, 0xcd, 0x9a, 0x65, 0x26, 0x6a, 0x02,
                0x89, 0x06, 0xf8, 0x64, 0x21, 0x4a, 0xf6, 0x6b, 0xa5, 0x4c, 0xce, 0x11, 0xac, 0xb0,
                0x37, 0xc9, 0x43, 0x93,
            ],
            Core::FeSquareRoot => [
                0xf7, 0x71, 0x81, 0x03, 0x30, 0x4c, 0xb4, 0x36, 0x96, 0xbd, 0xf9, 0x2f, 0x61, 0x4c,
                0x83, 0x8d, 0x24, 0xd7, 0xdd, 0x7b, 0xa8, 0xdc, 0x01, 0xab, 0x5c, 0x6a, 0x77, 0x26,
                0x3c, 0x15, 0xf7, 0x29,
            ],
            Core::FullAdd32 => [
                0x47, 0x27, 0x36, 0x1e, 0xa0, 0x03, 0xc1, 0xa4, 0x83, 0xe5, 0x75, 0x05, 0xcf, 0x5b,
                0x40, 0x5a, 0x82, 0x27, 0xda, 0x1a, 0xdd, 0xc4, 0x7e, 0x2b, 0x01, 0x6c, 0x2d, 0x09,
                0xbe, 0x04, 0x7f, 0xe8,
            ],
            Core::FullMultiply32 => [
                0xaa, 0xc2, 0x53, 0x61, 0xe5, 0x98, 0xe3, 0x54, 0x38, 0xb9, 0x18, 0xb5, 0x8f, 0xd2,
                0xce, 0xf4, 0xdb, 0x3c, 0x5d, 0x8c, 0x5e, 0x63, 0xaa, 0x4f, 0x25, 0xe9, 0xce, 0xc0,
                0xcf, 0xd9, 0xdf, 0xb1,
            ],
            Core::FullSubtract32 => [
                0x6d, 0x96, 0xf6, 0x8a, 0x94, 0x5c, 0x22, 0xe7, 0x62, 0x11, 0x5c, 0x09, 0x42, 0x97,
                0xb1, 0x94, 0xbe, 0xdc, 0x0c, 0xe5, 0xa0, 0xc9, 0x2d, 0xb6, 0x4b, 0x83, 0x0a, 0x18,
                0xb4, 0x4d, 0xf0, 0xd0,
            ],
            Core::GeIsOnCurve => [
                0xd9, 0xaa, 0x66, 0x06, 0x5c, 0xb0, 0xed, 0x2c, 0x71, 0x68, 0x60, 0x9d, 0xfd, 0x62,
                0xab, 0x64, 0x3a, 0xa8, 0x7c, 0x27, 0xe0, 0xdb, 0xbf, 0x96, 0xf2, 0x91, 0x45, 0x28,
                0xfe, 0xef, 0x52, 0xc5,
            ],
            Core::GeNegate => [
                0xa4, 0x97, 0xe7, 0x1c, 0x40, 0x3c, 0x4c, 0xe2, 0xb7, 0x81, 0x89, 0x3c, 0xd6, 0x9a,
                0x52, 0x85, 0xea, 0x02, 0xd7, 0xb7, 0xfe, 0x8e, 0xdf, 0xac, 0xe7, 0x8a, 0x20, 0x5a,
                0xad, 0x2e, 0xc0, 0x33,
            ],
            Core::GejAdd => [
                0x86, 0x0a, 0x61, 0x5b, 0xb2, 0x5d, 0x22, 0xc1, 0x0a, 0x04, 0x48, 0x72, 0xd1, 0xb8,
                0xfb, 0x04, 0x98, 0x25, 0x86, 0x86, 0x28, 0x81, 0x81, 0x9c, 0xed, 0x25, 0xd6, 0x75,
                0xdc, 0x4f, 0x7d, 0xfe,
            ],
            Core::GejDouble => [
                0x73, 0x2c, 0x6a, 0xb2, 0x5e, 0xab, 0x89, 0x66, 0x8d, 0x0c, 0xe2, 0x1c, 0x5b, 0x36,
                0x50, 0x18, 0x83, 0xb8, 0xdb, 0x67, 0x86, 0x4f, 0xf3, 0x4f, 0x98, 0x1a, 0x56, 0x8e,
                0xb9, 0xc5, 0xe8, 0xf0,
            ],
            Core::GejGeAdd => [
                0xd2, 0x89, 0x43, 0x14, 0xa4, 0x8e, 0xa3, 0xf7, 0x15, 0x88, 0x91, 0x07, 0x48, 0xb6,
                0x75, 0x3a, 0x9e, 0x53, 0x9f, 0xf2, 0xb3, 0x6d, 0x1b, 0xf0, 0x89, 0x74, 0x93, 0xdc,
                0x14, 0x0a, 0xa3, 0xce,
            ],
            Core::GejGeAddEx => [
                0x86, 0x64, 0x35, 0x4f, 0x50, 0x56, 0x65, 0xc8, 0xbe, 0x4b, 0x3b, 0xc1, 0xfa, 0x08,
                0x6f, 0x4f, 0x02, 0xf3, 0xaf, 0x69, 0x1d, 0x20, 0x9e, 0x85, 0x7c, 0x6f, 0x61, 0x5b,
                0x0d, 0xc4, 0x80, 0x32,
            ],
            Core::GejInfinity => [
                0x2d, 0x9d, 0x36, 0xb4, 0x6e, 0xad, 0x02, 0xdb, 0x63, 0xb5, 0x85, 0xdc, 0xa6, 0x7e,
                0x5e, 0x4d, 0xcb, 0x94, 0x05, 0x89, 0xbb, 0x13, 0x3c, 0x99, 0x10, 0x0d, 0x61, 0x7c,
                0x27, 0x12, 0x6e, 0x96,
            ],
            Core::GejIsInfinity => [
                0xe1, 0x86, 0xf9, 0xbd, 0xbe, 0x49, 0x16, 0xc7, 0x2f, 0x6c, 0x3b, 0xc2, 0xad, 0xf3,
                0xe0, 0x47, 0x22, 0xef, 0x4c, 0xec, 0x29, 0x72, 0x53, 0xe3, 0xec, 0xaa, 0x4e, 0x4c,
                0xc5, 0x51, 0xef, 0x2c,
            ],
            Core::GejIsOnCurve => [
                0xa8, 0xc8, 0x2e, 0x8b, 0x3a, 0x61, 0x99, 0xda, 0x25, 0xb2, 0xb1, 0x9c, 0xd1, 0x49,
                0xf9, 0xff, 0x4c, 0x3f, 0xdc, 0x0b, 0x00, 0xb2, 0x64, 0x80, 0xc0, 0x00, 0x65, 0x53,
                0xd4, 0x3c, 0x1f, 0x6e,
            ],
            Core::GejNegate => [
                0x71, 0xee, 0xff, 0xb5, 0xb6, 0x37, 0xaf, 0x51, 0xc4, 0x97, 0x80, 0x02, 0xc2, 0x12,
                0xcd, 0xaf, 0x39, 0x6c, 0xf8, 0xef, 0xca, 0x33, 0xaa, 0xb0, 0xf8, 0x33, 0xf8, 0x1a,
                0x9f, 0xb6, 0xa9, 0x89,
            ],
            Core::GejNormalize => [
                0xb4, 0x19, 0x98, 0xde, 0x56, 0x4e, 0xf6, 0x4f, 0x63, 0xa4, 0xc9, 0xfa, 0xd1, 0x39,
                0x50, 0x64, 0x83, 0x2e, 0x7d, 0x5c, 0x4c, 0x77, 0x1d, 0x18, 0x0f, 0xce, 0xd2, 0x8d,
                0x8a, 0x76, 0x5b, 0xd6,
            ],
            Core::GejRescale => [
                0xfa, 0x70, 0xaa, 0x15, 0x3d, 0xab, 0x6b, 0xc9, 0x93, 0x55, 0xc1, 0x0c, 0x61, 0xe5,
                0xbf, 0xcf, 0xa3, 0x97, 0xb3, 0x81, 0xf7, 0xef, 0x59, 0x15, 0x9d, 0x83, 0x79, 0x1a,
                0x2a, 0x6a, 0x58, 0x24,
            ],
            Core::GejXEquiv => [
                0x65, 0xa8, 0x60, 0xfa, 0x7e, 0x74, 0x60, 0x1c, 0xb6, 0xd8, 0x37, 0x55, 0x3b, 0xa1,
                0x9d, 0x60, 0xc4, 0x77, 0x3c, 0x1c, 0x12, 0xb4, 0xb0, 0xf0, 0x27, 0x8b, 0x45, 0xfb,
                0x23, 0xfc, 0xe9, 0x67,
            ],
            Core::GejYIsOdd => [
                0xcf, 0x91, 0xc7, 0x1d, 0xa7, 0x13, 0x98, 0xec, 0x8c, 0x64, 0xfd, 0xbf, 0x8f, 0xdc,
                0x91, 0x2e, 0xd5, 0xa8, 0xc1, 0xfa, 0xc6, 0x56, 0x6e, 0x0b, 0x59, 0x13, 0xeb, 0xe9,
                0xc4, 0xb1, 0x06, 0x17,
            ],
            Core::Generate => [
                0x31, 0x78, 0x42, 0xd3, 0xdd, 0x20, 0x4c, 0x31, 0xc5, 0xd8, 0x34, 0x86, 0x94, 0x0d,
                0x15, 0xbb, 0x6a, 0x05, 0x3e, 0x3d, 0x25, 0x61, 0xee, 0x13, 0x6e, 0xa9, 0x1e, 0x74,
                0x67, 0x74, 0x41, 0xae,
            ],
            Core::Le32 => [
                0xea, 0x77, 0x2c, 0x0b, 0x5a, 0xec, 0xde, 0x7d, 0xc1, 0x6e, 0x3f, 0x1f, 0x95, 0x27,
                0x89, 0xf0, 0x13, 0x70, 0x89, 0x04, 0xce, 0xaa, 0x62, 0xcf, 0xdc, 0xf6, 0x4d, 0x30,
                0xa3, 0x91, 0x17, 0xaf,
            ],
            Core::LinearCombination1 => [
                0x35, 0xcb, 0xfa, 0x56, 0x1b, 0x44, 0xf4, 0x48, 0xdc, 0x84, 0x3a, 0xa6, 0x05, 0x2a,
                0xdd, 0x1a, 0x4f, 0xf6, 0xfb, 0x56, 0xd6, 0x86, 0xc3, 0x21, 0xc5, 0xf8, 0x3b, 0x55,
                0x25, 0xef, 0xe1, 0x8b,
            ],
            Core::LinearVerify1 => [
                0x34, 0xae, 0x24, 0x21, 0x03, 0xad, 0x8c, 0xf1, 0x09, 0x82, 0x44, 0x44, 0x13, 0xdd,
                0x0d, 0xcb, 0x57, 0x77, 0x48, 0x67, 0x90, 0xdd, 0x21, 0x98, 0xeb, 0xbb, 0xe6, 0x4a,
                0x5a, 0xef, 0x0a, 0x35,
            ],
            Core::Low32 => [
                0x96, 0xbe, 0x53, 0x34, 0x5d, 0x52, 0x14, 0xb0, 0x05, 0xc7, 0xfc, 0x5d, 0xe4, 0x0a,
                0x92, 0x62, 0x56, 0x60, 0x41, 0x40, 0x63, 0x35, 0x27, 0xc1, 0xd0, 0x2b, 0xe0, 0xd5,
                0xf6, 0x0c, 0xc2, 0xad,
            ],
            Core::Multiply32 => [
                0x16, 0x1f, 0xd0, 0x3a, 0x92, 0xc6, 0x21, 0xb3, 0x28, 0x98, 0x49, 0xff, 0x29, 0xad,
                0x81, 0x34, 0x99, 0xd6, 0x3e, 0xd9, 0x73, 0xdb, 0x0e, 0x97, 0x51, 0x78, 0x54, 0x21,
                0xf5, 0x68, 0xd1, 0x8f,
            ],
            Core::One32 => [
                0x45, 0x26, 0x09, 0x9d, 0x0f, 0x84, 0x0b, 0xa6, 0xab, 0xe5, 0x5e, 0xd9, 0xf3, 0xa9,
                0xb8, 0xfc, 0xa0, 0x84, 0xf2, 0x51, 0xbf, 0xb2, 0x6d, 0x01, 0x1c, 0xf7, 0xaf, 0x27,
                0xb0, 0xd8, 0x8e, 0x29,
            ],
            Core::ParseLock => [
                0xd5, 0x96, 0x9c, 0x25, 0xb5, 0x6e, 0x87, 0xb4, 0xfb, 0x28, 0x80, 0xea, 0xee, 0x90,
                0xcc, 0x94, 0x49, 0xfa, 0xa2, 0xf3, 0xd0, 0x76, 0x0d, 0xe9, 0xfe, 0xe5, 0x65, 0xdf,
                0xcf, 0x6a, 0x16, 0xc6,
            ],
            Core::ParseSequence => [
                0x04, 0xb9, 0x5b, 0xbe, 0x88, 0x1e, 0x1a, 0x6b, 0x41, 0x9f, 0x82, 0x88, 0x3f, 0xf0,
                0x73, 0xea, 0xdb, 0xc1, 0x4f, 0x2c, 0x8c, 0x56, 0xb3, 0x6f, 0x19, 0xdc, 0xec, 0xac,
                0x27, 0xe7, 0x73, 0x7f,
            ],
            Core::PointVerify1 => [
                0xef, 0xe2, 0x87, 0xa5, 0xcb, 0xe2, 0x81, 0x8b, 0x11, 0x3d, 0x87, 0x00, 0x22, 0x32,
                0x6e, 0xb3, 0x0a, 0x4b, 0xe0, 0xad, 0xa4, 0x40, 0x23, 0xd5, 0x45, 0xb6, 0xc1, 0x51,
                0xf0, 0xcd, 0xa7, 0x08,
            ],
            Core::ScalarAdd => [
                0x67, 0xe4, 0x1f, 0xad, 0x70, 0x45, 0x00, 0xce, 0x97, 0x50, 0x91, 0x32, 0xd4, 0xf6,
                0x97, 0x34, 0x2e, 0x85, 0x83, 0xed, 0x7e, 0x9f, 0x7b, 0xed, 0xb9, 0x95, 0xd3, 0x6c,
                0xf6, 0x5f, 0xf3, 0x2e,
            ],
            Core::ScalarInvert => [
                0xc0, 0xe2, 0xad, 0x1b, 0xa6, 0xbf, 0xd9, 0x10, 0x44, 0x24, 0xf5, 0x94, 0xd0, 0x07,
                0x3e, 0xa1, 0x99, 0x40, 0x5e, 0x5c, 0xa5, 0xb7, 0x12, 0x83, 0x94, 0xb6, 0x13, 0xb9,
                0xe1, 0xbd, 0x36, 0xfc,
            ],
            Core::ScalarIsZero => [
                0x38, 0xa4, 0x57, 0xca, 0xb1, 0xc3, 0x0c, 0x51, 0x4e, 0x20, 0xe2, 0x41, 0xd5, 0x84,
                0x65, 0x40, 0x75, 0xec, 0x4d, 0x05, 0x49, 0x6c, 0x7e, 0x0b, 0x1c, 0xe2, 0xde, 0x5e,
                0x2f, 0xc1, 0x99, 0x16,
            ],
            Core::ScalarMultiply => [
                0x14, 0x51, 0x3c, 0xf4, 0x41, 0x17, 0x9e, 0x62, 0xfb, 0x42, 0x93, 0xbb, 0x35, 0x3e,
                0xe5, 0xbf, 0x01, 0xed, 0xdf, 0x8d, 0x81, 0xce, 0x03, 0x10, 0x06, 0x2f, 0x09, 0xa8,
                0x1d, 0x2f, 0xbc, 0xa8,
            ],
            Core::ScalarMultiplyLambda => [
                0xf6, 0x24, 0x00, 0xf5, 0xbe, 0x74, 0x00, 0xa7, 0x12, 0xe7, 0x4a, 0x1d, 0xc1, 0xa8,
                0x41, 0xe6, 0x02, 0x4a, 0x96, 0x18, 0x55, 0x1c, 0x33, 0x64, 0xc4, 0xe8, 0x15, 0x6d,
                0x1c, 0xdd, 0xed, 0x83,
            ],
            Core::ScalarNegate => [
                0x6a, 0x97, 0x6a, 0x67, 0x68, 0xbd, 0x72, 0x8f, 0xe2, 0x10, 0x51, 0x85, 0x1c, 0x60,
                0xeb, 0x25, 0x72, 0xe5, 0xd0, 0x6c, 0x95, 0x56, 0x6d, 0xfa, 0xe9, 0x28, 0x20, 0xc8,
                0x42, 0x4a, 0xaa, 0x4e,
            ],
            Core::ScalarNormalize => [
                0x90, 0xe0, 0x25, 0x78, 0x96, 0x99, 0x0b, 0xa6, 0xf0, 0xb0, 0x76, 0x54, 0x29, 0x19,
                0xcd, 0x06, 0x4a, 0xc0, 0x8b, 0x27, 0x7f, 0xae, 0x34, 0x79, 0xe4, 0x09, 0x18, 0xeb,
                0x6f, 0x5b, 0x07, 0xd8,
            ],
            Core::ScalarSquare => [
                0xd6, 0x36, 0xb4, 0x9d, 0xc6, 0xb2, 0x26, 0x6c, 0xce, 0xcb, 0x7b, 0xc0, 0x41, 0x68,
                0x82, 0x3b, 0x2a, 0x5d, 0x7a, 0x1d, 0x2a, 0xc3, 0x43, 0xda, 0x60, 0x54, 0x19, 0xd3,
                0x8d, 0xfd, 0xfd, 0xe0,
            ],
            Core::Scale => [
                0xac, 0x65, 0xf0, 0xb8, 0x0d, 0x5e, 0xfc, 0xeb, 0xb5, 0x01, 0xe5, 0xe9, 0x62, 0xf4,
                0x4f, 0xb8, 0x14, 0x6b, 0x6d, 0x4e, 0x46, 0x5b, 0xea, 0x49, 0xbc, 0x74, 0x09, 0x5e,
                0x21, 0xfb, 0xaa, 0xc0,
            ],
            Core::Sha256Block => [
                0xdf, 0xc9, 0x27, 0xd3, 0x9b, 0xf7, 0x14, 0x7a, 0x8b, 0x0a, 0x7f, 0x43, 0x79, 0x46,
                0x68, 0x70, 0x82, 0x4d, 0xb1, 0x02, 0x09, 0x0a, 0x00, 0x36, 0x29, 0x23, 0xa3, 0x77,
                0xa9, 0x1a, 0xf6, 0x81,
            ],
            Core::Sha256Ctx8Add1 => [
                0x2e, 0xcd, 0x3d, 0xbe, 0x41, 0xe1, 0xfb, 0x41, 0x7d, 0x24, 0x21, 0xdd, 0x07, 0xef,
                0x6e, 0x6e, 0xdb, 0xad, 0xb8, 0x82, 0x69, 0x81, 0x61, 0xeb, 0xcc, 0xf6, 0xca, 0xf0,
                0xee, 0xb9, 0x98, 0x30,
            ],
            Core::Sha256Ctx8Add128 => [
                0x3d, 0x5d, 0x67, 0xe2, 0x7e, 0xfc, 0x25, 0xa7, 0x2d, 0xa6, 0x18, 0x6d, 0x2a, 0x6f,
                0xa1, 0x06, 0xe7, 0x57, 0x43, 0x57, 0xb1, 0xf8, 0x76, 0xfa, 0x68, 0x79, 0xa6, 0x11,
                0x36, 0x82, 0xb7, 0x35,
            ],
            Core::Sha256Ctx8Add16 => [
                0x7b, 0xa6, 0xef, 0xb6, 0x92, 0xa1, 0x91, 0xe4, 0xdd, 0xd7, 0xa5, 0x84, 0x1d, 0x5a,
                0x06, 0x3d, 0x6a, 0xa3, 0xf0, 0xcf, 0xcd, 0x16, 0xfd, 0x84, 0xde, 0xff, 0x5d, 0x1f,
                0x3b, 0x3b, 0x08, 0xcb,
            ],
            Core::Sha256Ctx8Add2 => [
                0x12, 0x3c, 0x7d, 0x8c, 0xe5, 0x39, 0xd8, 0xab, 0xc6, 0xda, 0xc5, 0xf9, 0x9c, 0xe8,
                0x23, 0x39, 0xb9, 0x56, 0x41, 0x68, 0xe6, 0x56, 0x43, 0x37, 0x28, 0x5e, 0x89, 0x82,
                0x4a, 0x23, 0x9f, 0x33,
            ],
            Core::Sha256Ctx8Add256 => [
                0x76, 0xe2, 0xb4, 0xfc, 0xa5, 0x4b, 0x4d, 0x7a, 0x39, 0x0b, 0x92, 0xb0, 0xf0, 0xe1,
                0x20, 0x49, 0x1d, 0xe8, 0x6d, 0xfe, 0xa6, 0xe9, 0x62, 0x58, 0x80, 0x97, 0x23, 0x92,
                0xb1, 0xf5, 0x38, 0x4c,
            ],
            Core::Sha256Ctx8Add32 => [
                0xa4, 0xd6, 0x39, 0xe4, 0xec, 0x73, 0x39, 0x9d, 0x31, 0x93, 0x9c, 0xe3, 0xc0, 0x25,
                0x06, 0xd1, 0xd2, 0x3a, 0x20, 0x29, 0x3b, 0xb9, 0x5a, 0xb3, 0x7c, 0xb8, 0x7d, 0xa2,
                0xb1, 0x87, 0xc6, 0xf8,
            ],
            Core::Sha256Ctx8Add4 => [
                0x8a, 0x41, 0xbf, 0xb6, 0x24, 0x29, 0x5a, 0x3d, 0x8f, 0x67, 0xdc, 0x5b, 0xda, 0x5e,
                0x7c, 0x7d, 0x97, 0xc9, 0xcc, 0x30, 0x21, 0xd3, 0x81, 0xce, 0x8b, 0x04, 0x7b, 0x3d,
                0x79, 0xd1, 0x87, 0xc9,
            ],
            Core::Sha256Ctx8Add512 => [
                0x2c, 0x80, 0x99, 0x15, 0x75, 0x65, 0x4b, 0x3d, 0x81, 0x65, 0x8d, 0xca, 0xde, 0xa5,
                0x19, 0x49, 0xcc, 0x18, 0x25, 0x30, 0xaf, 0x7e, 0x9f, 0x41, 0xee, 0x5b, 0x93, 0xfc,
                0x33, 0xf0, 0xc8, 0x0f,
            ],
            Core::Sha256Ctx8Add64 => [
                0x10, 0xae, 0x93, 0x7f, 0x66, 0x97, 0x00, 0xd3, 0x21, 0x55, 0x01, 0x53, 0xd4, 0x60,
                0x25, 0x20, 0x8c, 0x03, 0x70, 0xda, 0x86, 0x6d, 0xd3, 0xd4, 0xc9, 0xb3, 0x4d, 0x42,
                0x2c, 0xcc, 0x2c, 0xc8,
            ],
            Core::Sha256Ctx8Add8 => [
                0x48, 0xd6, 0x97, 0x4c, 0xd3, 0x6c, 0x37, 0x6c, 0x4b, 0xd2, 0x60, 0xd1, 0x1f, 0xf6,
                0x2e, 0x74, 0x20, 0x76, 0x93, 0x4e, 0xc3, 0x10, 0x8d, 0x9f, 0x8f, 0xf2, 0x7f, 0xb4,
                0x54, 0xe4, 0x94, 0x5f,
            ],
            Core::Sha256Ctx8AddBuffer511 => [
                0x3d, 0x59, 0x9f, 0x24, 0xf3, 0xb1, 0x53, 0x3c, 0x86, 0x64, 0xc4, 0x05, 0x04, 0xdc,
                0x39, 0xbf, 0xc5, 0x75, 0x6f, 0x2d, 0x9a, 0x6a, 0x4d, 0x0d, 0x65, 0xbe, 0x29, 0x9e,
                0x88, 0xb4, 0xa6, 0xd5,
            ],
            Core::Sha256Ctx8Finalize => [
                0x8c, 0x44, 0xd9, 0x30, 0x4f, 0x8f, 0x85, 0xe8, 0x56, 0x64, 0x90, 0x63, 0xf7, 0x84,
                0x2a, 0xc5, 0xa6, 0xe8, 0x0f, 0xd3, 0x7a, 0x01, 0x41, 0xc9, 0xff, 0xde, 0x74, 0x2f,
                0xbe, 0x8c, 0x9f, 0xf8,
            ],
            Core::Sha256Ctx8Init => [
                0x88, 0xdb, 0x03, 0x5a, 0x49, 0x0d, 0xa7, 0x89, 0x30, 0xf7, 0x5e, 0x6e, 0x1b, 0x10,
                0xaa, 0x86, 0x8b, 0xd4, 0x75, 0x93, 0xc6, 0x74, 0xf3, 0xbc, 0x4b, 0xb6, 0x24, 0x22,
                0x53, 0x13, 0xe3, 0xba,
            ],
            Core::Sha256Iv => [
                0xe4, 0xd5, 0x96, 0xe0, 0x0f, 0xf0, 0xf7, 0xdf, 0x3d, 0x99, 0x69, 0xd8, 0x2c, 0xb1,
                0x63, 0x4d, 0x59, 0xeb, 0x9d, 0x0f, 0x4b, 0x24, 0xd8, 0xca, 0x72, 0xe8, 0x48, 0xc1,
                0x0c, 0x75, 0x24, 0x4c,
            ],
            Core::Subtract32 => [
                0xf7, 0x6e, 0xca, 0xd1, 0xfd, 0xa5, 0x0f, 0x13, 0x5b, 0xdf, 0xe3, 0xe5, 0x33, 0xa1,
                0x50, 0x09, 0x8f, 0x40, 0x62, 0x61, 0xc7, 0x6f, 0x6d, 0xbf, 0x67, 0x25, 0xf1, 0xe3,
                0x88, 0x3c, 0x2a, 0xe2,
            ],
            Core::Verify => [
                0xa1, 0x75, 0x34, 0xa4, 0x9a, 0xcb, 0xec, 0xb1, 0x55, 0x4d, 0x6e, 0xc5, 0xc6, 0xda,
                0x50, 0xc2, 0x8c, 0x9c, 0xd9, 0x5e, 0xa0, 0xb4, 0x48, 0x86, 0xb8, 0xe0, 0x94, 0xad,
                0xa7, 0x77, 0xc1, 0xd4,
            ],
        };

        Cmr(Midstate(bytes))
    }

    fn source_ty(&self) -> TypeName {
        let name: &'static [u8] = match self {
            Core::Add32 => b"l",
            Core::Bip0340Verify => b"**hh*hh",
            Core::CheckSigVerify => b"**h*hh*hh",
            Core::Decompress => b"*2h",
            Core::Eq256 => b"*hh",
            Core::Eq32 => b"l",
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
            Core::FullAdd32 => b"*2l",
            Core::FullMultiply32 => b"*ll",
            Core::FullSubtract32 => b"*2l",
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
            Core::Le32 => b"l",
            Core::LinearCombination1 => b"**h**hhhh",
            Core::LinearVerify1 => b"***h*hhh*hh",
            Core::Low32 => b"1",
            Core::Multiply32 => b"l",
            Core::One32 => b"1",
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
            Core::Subtract32 => b"l",
            Core::Verify => b"2",
        };

        TypeName(name)
    }

    fn target_ty(&self) -> TypeName {
        let name: &'static [u8] = match self {
            Core::Add32 => b"*2i",
            Core::Bip0340Verify => b"1",
            Core::CheckSigVerify => b"1",
            Core::Decompress => b"+1*hh",
            Core::Eq256 => b"2",
            Core::Eq32 => b"2",
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
            Core::FullAdd32 => b"*2i",
            Core::FullMultiply32 => b"l",
            Core::FullSubtract32 => b"*2i",
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
            Core::Le32 => b"2",
            Core::LinearCombination1 => b"**hhh",
            Core::LinearVerify1 => b"1",
            Core::Low32 => b"i",
            Core::Multiply32 => b"l",
            Core::One32 => b"i",
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
            Core::Subtract32 => b"*2i",
            Core::Verify => b"1",
        };

        TypeName(name)
    }

    fn encode<W: Write>(&self, w: &mut BitWriter<W>) -> std::io::Result<usize> {
        let (n, len) = match self {
            Core::Verify => (0, 2),
            Core::Low32 => (305, 10),
            Core::Eq32 => (7025, 14),
            Core::Eq256 => (14056, 15),
            Core::One32 => (561, 10),
            Core::FullAdd32 => (2353, 12),
            Core::Add32 => (2417, 12),
            Core::FullSubtract32 => (19697, 15),
            Core::Subtract32 => (39473, 16),
            Core::FullMultiply32 => (39729, 16),
            Core::Multiply32 => (39793, 16),
            Core::Le32 => (639025, 20),
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

    fn decode<I: Iterator<Item = u8>>(bits: &mut BitIter<I>) -> Result<Self, Error> {
        decode_bits!(bits, {
            0 => {
                0 => {Core::Verify},
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
                                                1 => {Core::Low32}
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
                                                                1 => {Core::Eq32}
                                                            },
                                                            1 => {}
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
                                0 => {},
                                1 => {
                                    0 => {
                                        0 => {
                                            0 => {
                                                0 => {},
                                                1 => {Core::One32}
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
                                                        1 => {Core::FullAdd32}
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
                                                        1 => {Core::Add32}
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
                                                                    1 => {Core::FullSubtract32}
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
                                                                        1 => {Core::Subtract32}
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
                                                                        1 => {Core::FullMultiply32}
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
                                                                        1 => {Core::Multiply32}
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
                                                                                        1 => {Core::Le32}
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
            Core::Add32 => &simplicity_sys::c_jets::jets_wrapper::add_32,
            Core::Bip0340Verify => &simplicity_sys::c_jets::jets_wrapper::bip_0340_verify,
            Core::CheckSigVerify => &simplicity_sys::c_jets::jets_wrapper::check_sig_verify,
            Core::Decompress => &simplicity_sys::c_jets::jets_wrapper::decompress,
            Core::Eq256 => &simplicity_sys::c_jets::jets_wrapper::eq_256,
            Core::Eq32 => &simplicity_sys::c_jets::jets_wrapper::eq_32,
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
            Core::FullAdd32 => &simplicity_sys::c_jets::jets_wrapper::full_add_32,
            Core::FullMultiply32 => &simplicity_sys::c_jets::jets_wrapper::full_multiply_32,
            Core::FullSubtract32 => &simplicity_sys::c_jets::jets_wrapper::full_subtract_32,
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
            Core::Le32 => &simplicity_sys::c_jets::jets_wrapper::le_32,
            Core::LinearCombination1 => &simplicity_sys::c_jets::jets_wrapper::linear_combination_1,
            Core::LinearVerify1 => &simplicity_sys::c_jets::jets_wrapper::linear_verify_1,
            Core::Low32 => &simplicity_sys::c_jets::jets_wrapper::low_32,
            Core::Multiply32 => &simplicity_sys::c_jets::jets_wrapper::multiply_32,
            Core::One32 => &simplicity_sys::c_jets::jets_wrapper::one_32,
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
            Core::Subtract32 => &simplicity_sys::c_jets::jets_wrapper::subtract_32,
            Core::Verify => &simplicity_sys::c_jets::jets_wrapper::verify,
        }
    }
}
