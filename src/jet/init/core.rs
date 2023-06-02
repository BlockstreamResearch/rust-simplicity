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

/// Core jet family
#[derive(Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Debug, Hash)]
pub enum Core {
    Add16,
    Add32,
    Add64,
    Add8,
    Bip0340Verify,
    CheckSigVerify,
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
    LinearCombination1,
    LinearVerify1,
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
    One16,
    One32,
    One64,
    One8,
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
    Subtract16,
    Subtract32,
    Subtract64,
    Subtract8,
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
            Core::Add16 => [
                0x51, 0x7a, 0x52, 0x5b, 0x98, 0xc8, 0x78, 0x60, 0x88, 0x41, 0x82, 0xcf, 0xc0, 0x14,
                0x81, 0x64, 0xaf, 0x35, 0x14, 0x63, 0xe0, 0xfb, 0xac, 0x34, 0x60, 0x65, 0x2b, 0x2f,
                0xb5, 0xed, 0xcf, 0x19,
            ],
            Core::Add32 => [
                0xe4, 0x04, 0x66, 0xa7, 0xec, 0xf7, 0x1c, 0xe8, 0x62, 0xfb, 0x3c, 0x15, 0x4c, 0x1e,
                0x8f, 0x84, 0x5d, 0x7e, 0x57, 0x07, 0x46, 0x3a, 0x89, 0x45, 0x37, 0xa3, 0x2f, 0xc7,
                0x21, 0x49, 0x00, 0xad,
            ],
            Core::Add64 => [
                0x81, 0x2e, 0x6e, 0x75, 0x70, 0x12, 0x1f, 0x8c, 0xc3, 0x52, 0xc0, 0x9a, 0x0a, 0xc7,
                0x1a, 0xe6, 0xcc, 0xfe, 0x85, 0x9e, 0x84, 0x97, 0x60, 0xbd, 0x47, 0x8d, 0x29, 0x8a,
                0xea, 0x82, 0xae, 0xb0,
            ],
            Core::Add8 => [
                0x62, 0x61, 0x3c, 0x74, 0x98, 0xfe, 0x9c, 0x5e, 0x98, 0xa1, 0xf2, 0x4e, 0xe1, 0x07,
                0x0a, 0x02, 0xe9, 0xdc, 0xf2, 0x7f, 0x7f, 0x73, 0x63, 0x8c, 0x0a, 0x8f, 0xce, 0x85,
                0xb7, 0x64, 0x15, 0x1a,
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
            Core::Decrement16 => [
                0x4a, 0x10, 0x94, 0x9e, 0xb5, 0x98, 0xc0, 0x04, 0xbc, 0xae, 0xaf, 0xb8, 0x2f, 0x02,
                0xac, 0x62, 0x06, 0xc2, 0xa0, 0xf2, 0x32, 0xff, 0x95, 0x40, 0xa4, 0xff, 0x02, 0xa5,
                0x54, 0x71, 0xf8, 0xfa,
            ],
            Core::Decrement32 => [
                0x29, 0x1e, 0xab, 0x34, 0x34, 0xe2, 0x01, 0x77, 0x5e, 0xcb, 0x68, 0x21, 0x5b, 0x54,
                0x03, 0xfe, 0x8c, 0x9e, 0xef, 0xbe, 0x88, 0x0c, 0xb2, 0xe0, 0x96, 0xe7, 0x60, 0x22,
                0xb8, 0x30, 0x1b, 0x59,
            ],
            Core::Decrement64 => [
                0xe1, 0x8e, 0xce, 0x15, 0xc9, 0x14, 0xaa, 0x0c, 0x22, 0x2d, 0xec, 0x3b, 0x0c, 0xa0,
                0xab, 0x39, 0xcd, 0x81, 0xed, 0x38, 0x25, 0x6c, 0xf8, 0x77, 0x0b, 0x78, 0x17, 0x3b,
                0xb9, 0xc6, 0xb4, 0x87,
            ],
            Core::Decrement8 => [
                0xd2, 0xaf, 0xfa, 0xd9, 0xae, 0xa4, 0xfd, 0xff, 0x91, 0x6d, 0xbe, 0x5f, 0x90, 0xa6,
                0xf9, 0x3e, 0x2f, 0x4d, 0x30, 0xba, 0x87, 0x3a, 0x52, 0x81, 0xa4, 0x41, 0xd4, 0xa7,
                0x93, 0x70, 0xbe, 0x96,
            ],
            Core::DivMod16 => [
                0xf5, 0x09, 0x47, 0x22, 0x26, 0xc5, 0x32, 0x5a, 0x4a, 0x41, 0x7d, 0x68, 0xc9, 0x97,
                0xd6, 0x39, 0x5d, 0xe0, 0x99, 0x29, 0xdd, 0xc2, 0x27, 0x1e, 0xc7, 0x28, 0x20, 0x46,
                0xed, 0xbd, 0xdc, 0x29,
            ],
            Core::DivMod32 => [
                0xa2, 0x90, 0x83, 0x06, 0x91, 0xf7, 0x3a, 0x5f, 0x40, 0x89, 0x18, 0x6f, 0xce, 0x3b,
                0x22, 0xff, 0x4c, 0xc1, 0x0a, 0xee, 0x1a, 0xb2, 0x58, 0xa4, 0xac, 0x51, 0x86, 0x4d,
                0x12, 0xf4, 0xe3, 0x3b,
            ],
            Core::DivMod64 => [
                0x64, 0x97, 0x16, 0xf1, 0x59, 0x72, 0xf7, 0xb9, 0xc7, 0x0f, 0x9a, 0xd2, 0x2f, 0x65,
                0xb3, 0x91, 0xff, 0x68, 0x52, 0xac, 0xc8, 0x78, 0x37, 0xbb, 0xce, 0xdd, 0x17, 0x1d,
                0xcb, 0x8b, 0xd3, 0x14,
            ],
            Core::DivMod8 => [
                0xd9, 0xbe, 0x81, 0xb7, 0x53, 0x49, 0xe3, 0xbb, 0x22, 0x21, 0xc4, 0xd4, 0x5a, 0xe0,
                0x4f, 0x8d, 0xf1, 0xbe, 0x4e, 0x7e, 0x76, 0x8b, 0x36, 0x83, 0xb1, 0xa1, 0x55, 0x68,
                0xb8, 0x31, 0xfe, 0xfb,
            ],
            Core::Divide16 => [
                0xd3, 0x1f, 0xf5, 0x9b, 0x05, 0x84, 0x17, 0x2f, 0xc1, 0xe4, 0xce, 0x0e, 0x5e, 0x89,
                0xbe, 0x12, 0x80, 0x90, 0x8e, 0x49, 0x47, 0x28, 0xee, 0xd7, 0x53, 0xbf, 0xa7, 0x1f,
                0xb6, 0xd8, 0x51, 0x93,
            ],
            Core::Divide32 => [
                0xab, 0x97, 0x45, 0xca, 0xf9, 0x3e, 0x6b, 0xaa, 0x48, 0xaf, 0x0e, 0xe3, 0xd2, 0xbe,
                0xf3, 0x9d, 0xb8, 0x0f, 0xe5, 0x4d, 0x06, 0xd2, 0xa8, 0xa6, 0x78, 0x75, 0x6d, 0x07,
                0x75, 0x32, 0x5f, 0xc5,
            ],
            Core::Divide64 => [
                0xae, 0x18, 0xfb, 0xff, 0xb0, 0x4e, 0x5c, 0x0d, 0x9f, 0xe9, 0x73, 0xa4, 0x5d, 0xcc,
                0x9a, 0xcd, 0x0e, 0xdd, 0xe7, 0x74, 0xb8, 0x0a, 0x6d, 0xb6, 0x63, 0x91, 0x23, 0x75,
                0x63, 0x46, 0xde, 0xec,
            ],
            Core::Divide8 => [
                0x61, 0x37, 0x2c, 0x0c, 0xd4, 0xa7, 0xe4, 0x78, 0x2d, 0x01, 0xd5, 0xb9, 0xce, 0x03,
                0x4d, 0xa9, 0x2c, 0x8a, 0x35, 0x13, 0x84, 0xcd, 0x28, 0xd6, 0x48, 0x48, 0x47, 0x93,
                0xc7, 0x08, 0x66, 0x39,
            ],
            Core::Divides16 => [
                0xf6, 0x6f, 0x19, 0x64, 0x09, 0x34, 0xa8, 0x25, 0xc4, 0x66, 0x08, 0xcd, 0x06, 0x5d,
                0xca, 0x61, 0x5d, 0x9c, 0xb8, 0x8b, 0x6d, 0x6e, 0xeb, 0xd6, 0x3c, 0xde, 0x22, 0x12,
                0x9b, 0xb4, 0xf6, 0xb4,
            ],
            Core::Divides32 => [
                0x38, 0x59, 0x74, 0x5d, 0x44, 0x09, 0x64, 0xa8, 0x80, 0x2e, 0xbe, 0x32, 0xb1, 0x0c,
                0x56, 0xf1, 0x86, 0x52, 0xd8, 0xe0, 0xde, 0xd3, 0xc8, 0x8d, 0xe0, 0x32, 0x76, 0xbf,
                0xb3, 0xcf, 0x15, 0x63,
            ],
            Core::Divides64 => [
                0x38, 0xa5, 0xd0, 0x32, 0xe9, 0x2f, 0x57, 0x8f, 0x4d, 0x39, 0xa3, 0x19, 0x2a, 0xce,
                0x98, 0xc7, 0x50, 0xfa, 0x1c, 0x4b, 0x13, 0x58, 0xa6, 0x0e, 0xec, 0x9a, 0x23, 0xc6,
                0x93, 0x47, 0x7e, 0x54,
            ],
            Core::Divides8 => [
                0x54, 0x6b, 0x4e, 0x70, 0x78, 0xa7, 0xd4, 0xf2, 0x7b, 0x44, 0xe6, 0x50, 0xdc, 0x29,
                0xb3, 0x61, 0x42, 0x97, 0xb7, 0xea, 0x32, 0xab, 0x9a, 0x94, 0x89, 0xac, 0xcc, 0x81,
                0xfc, 0x8d, 0xb6, 0xcd,
            ],
            Core::Eq16 => [
                0x9a, 0x5e, 0x8d, 0xb0, 0xdd, 0x21, 0xfb, 0xbc, 0x8c, 0x6a, 0x57, 0xa5, 0xf5, 0x49,
                0x61, 0x83, 0xca, 0xd7, 0x88, 0xed, 0xb8, 0x9c, 0x81, 0x59, 0x9d, 0x77, 0x09, 0xf3,
                0xb1, 0x97, 0x1e, 0x81,
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
            Core::Eq64 => [
                0xdb, 0xd0, 0xa3, 0x5e, 0x1f, 0x78, 0xe6, 0x1a, 0xd6, 0x75, 0xbd, 0xc1, 0x46, 0x29,
                0xc0, 0xd8, 0x32, 0xae, 0x1d, 0xe6, 0x6b, 0xd7, 0x73, 0xaa, 0x5c, 0xb3, 0x15, 0x79,
                0x81, 0x4c, 0x16, 0xba,
            ],
            Core::Eq8 => [
                0xcb, 0xe1, 0xb7, 0x99, 0xb0, 0x70, 0xeb, 0xad, 0xb0, 0x3d, 0x6a, 0x2e, 0xb3, 0x16,
                0x2c, 0xf5, 0x2c, 0xa0, 0x15, 0xc9, 0xb6, 0xc6, 0x95, 0x90, 0x31, 0x52, 0xc1, 0xe3,
                0x48, 0x7d, 0xca, 0x3d,
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
            Core::FullAdd16 => [
                0xf8, 0xe1, 0x52, 0xa9, 0x0a, 0xd9, 0x2b, 0x82, 0x05, 0xc5, 0x56, 0x98, 0x21, 0xef,
                0x09, 0x79, 0x90, 0xd9, 0xf7, 0xf9, 0x8f, 0x21, 0xca, 0xa8, 0xd9, 0xad, 0x55, 0xb5,
                0xed, 0xbc, 0xb6, 0x4d,
            ],
            Core::FullAdd32 => [
                0x47, 0x27, 0x36, 0x1e, 0xa0, 0x03, 0xc1, 0xa4, 0x83, 0xe5, 0x75, 0x05, 0xcf, 0x5b,
                0x40, 0x5a, 0x82, 0x27, 0xda, 0x1a, 0xdd, 0xc4, 0x7e, 0x2b, 0x01, 0x6c, 0x2d, 0x09,
                0xbe, 0x04, 0x7f, 0xe8,
            ],
            Core::FullAdd64 => [
                0x8e, 0x29, 0x53, 0x1e, 0xa3, 0x0a, 0x34, 0xc0, 0x72, 0x97, 0x86, 0x07, 0x7f, 0x5d,
                0xc3, 0x79, 0xcb, 0x22, 0x45, 0xfd, 0xe4, 0x41, 0xdb, 0x16, 0xa6, 0x85, 0x6e, 0x26,
                0x80, 0x3d, 0x26, 0xb9,
            ],
            Core::FullAdd8 => [
                0x3f, 0xd9, 0x59, 0x05, 0x5a, 0x7b, 0x6f, 0x51, 0xb1, 0x47, 0x44, 0x75, 0x37, 0x66,
                0x02, 0x8f, 0x51, 0x83, 0x1b, 0xbd, 0x7c, 0xf7, 0x0e, 0x1e, 0xcc, 0x70, 0x06, 0x0d,
                0xfc, 0xcf, 0xb6, 0x4c,
            ],
            Core::FullDecrement16 => [
                0xb8, 0x35, 0xbc, 0xde, 0x81, 0x20, 0x7b, 0x8a, 0x1f, 0x68, 0x6b, 0xb5, 0xad, 0x00,
                0x8d, 0xd7, 0xf2, 0x7f, 0xfb, 0xa2, 0xd9, 0x1f, 0xab, 0x34, 0x63, 0x52, 0xc4, 0x11,
                0x8a, 0x13, 0xba, 0x57,
            ],
            Core::FullDecrement32 => [
                0x91, 0xde, 0x80, 0x21, 0xe3, 0xae, 0x86, 0x0b, 0xd8, 0x27, 0x04, 0xb2, 0xe5, 0x14,
                0x9d, 0xfe, 0x62, 0xb7, 0x4e, 0x1a, 0x6c, 0x71, 0xdb, 0xf0, 0xcb, 0xea, 0xbb, 0x7b,
                0x9e, 0xb2, 0x6a, 0x2b,
            ],
            Core::FullDecrement64 => [
                0x9e, 0x85, 0xc0, 0x20, 0xcd, 0x4d, 0xbb, 0x7a, 0x5d, 0xfb, 0x69, 0x7a, 0x44, 0x4c,
                0x06, 0x74, 0x8e, 0x22, 0x87, 0xae, 0x22, 0x37, 0x81, 0x3b, 0x1c, 0x5f, 0xa4, 0xa5,
                0x95, 0x8e, 0x6f, 0x1b,
            ],
            Core::FullDecrement8 => [
                0x16, 0xdb, 0x3f, 0x6d, 0x60, 0x02, 0xd0, 0x70, 0x51, 0xff, 0xf6, 0xf4, 0x94, 0x04,
                0x66, 0x61, 0xf0, 0x38, 0x79, 0xd6, 0x36, 0x54, 0x67, 0xfc, 0xaa, 0xf9, 0xab, 0xab,
                0xa0, 0xc7, 0xf2, 0x81,
            ],
            Core::FullIncrement16 => [
                0x08, 0xf3, 0xe2, 0x48, 0x41, 0xfe, 0x94, 0x8d, 0x15, 0x29, 0x76, 0x52, 0x0f, 0x32,
                0xdd, 0x12, 0xf7, 0x5c, 0x38, 0x2a, 0xb9, 0xf5, 0x96, 0x6a, 0x16, 0x2c, 0x05, 0x06,
                0xe2, 0xb3, 0x7b, 0x55,
            ],
            Core::FullIncrement32 => [
                0xff, 0x69, 0xcd, 0xe4, 0x86, 0xc1, 0x91, 0xf4, 0x26, 0x60, 0x7c, 0xaa, 0xdb, 0x45,
                0xaa, 0x4b, 0x82, 0xd2, 0x1f, 0xc3, 0x5f, 0x99, 0xba, 0x0d, 0x7f, 0x56, 0xa8, 0x89,
                0x4c, 0x24, 0xa2, 0x6c,
            ],
            Core::FullIncrement64 => [
                0x4f, 0x08, 0x0c, 0x7a, 0xb1, 0x90, 0xbb, 0x41, 0xc2, 0x18, 0x8a, 0x37, 0xa4, 0xb3,
                0xb5, 0x2d, 0x6f, 0x75, 0x66, 0x86, 0x1f, 0x5b, 0x46, 0xa9, 0x80, 0x2d, 0xbe, 0xb1,
                0x42, 0x35, 0x0a, 0xe0,
            ],
            Core::FullIncrement8 => [
                0xba, 0x53, 0x0e, 0x83, 0x49, 0xb1, 0x3d, 0x90, 0x27, 0xff, 0x14, 0x16, 0xa4, 0x12,
                0x12, 0x1f, 0x9b, 0x6b, 0x5e, 0x3c, 0x62, 0xea, 0xa1, 0x18, 0x2a, 0xd2, 0x44, 0xb0,
                0xf7, 0x0f, 0x12, 0xc5,
            ],
            Core::FullMultiply16 => [
                0x78, 0x3d, 0x05, 0xcf, 0xb8, 0x4d, 0x0c, 0x91, 0x76, 0x60, 0x38, 0x26, 0xc1, 0x6c,
                0x5c, 0xf7, 0x30, 0xd0, 0x1c, 0x50, 0xa1, 0x69, 0x29, 0x79, 0xf7, 0x92, 0xe0, 0x98,
                0xf6, 0x99, 0x80, 0x12,
            ],
            Core::FullMultiply32 => [
                0xaa, 0xc2, 0x53, 0x61, 0xe5, 0x98, 0xe3, 0x54, 0x38, 0xb9, 0x18, 0xb5, 0x8f, 0xd2,
                0xce, 0xf4, 0xdb, 0x3c, 0x5d, 0x8c, 0x5e, 0x63, 0xaa, 0x4f, 0x25, 0xe9, 0xce, 0xc0,
                0xcf, 0xd9, 0xdf, 0xb1,
            ],
            Core::FullMultiply64 => [
                0x4e, 0x7b, 0xe1, 0x50, 0x1c, 0xda, 0x15, 0xae, 0x5c, 0xfb, 0x21, 0xee, 0xec, 0xe0,
                0x66, 0xe1, 0x02, 0x0e, 0x0f, 0xd9, 0xb3, 0xe8, 0x36, 0x5b, 0x68, 0x14, 0x02, 0x70,
                0xf9, 0xbc, 0x69, 0x47,
            ],
            Core::FullMultiply8 => [
                0xd5, 0xe4, 0xc5, 0x6d, 0xe7, 0x83, 0x94, 0x0d, 0xd6, 0xb6, 0xec, 0xe1, 0x3e, 0xa5,
                0x8e, 0xf7, 0x9e, 0x1b, 0x70, 0x84, 0xba, 0x2c, 0x0c, 0xc5, 0x71, 0x20, 0xb7, 0xc5,
                0xc6, 0x2b, 0x3f, 0xdc,
            ],
            Core::FullSubtract16 => [
                0xfc, 0x6d, 0x4c, 0xd4, 0xa9, 0x9d, 0xbc, 0x0e, 0x01, 0xa5, 0x1e, 0x3d, 0x98, 0x6d,
                0x6b, 0x04, 0x1c, 0x65, 0x57, 0xb8, 0xfc, 0x2e, 0x8b, 0x8c, 0x2f, 0xb6, 0xd2, 0x29,
                0x2a, 0x31, 0x12, 0x61,
            ],
            Core::FullSubtract32 => [
                0x6d, 0x96, 0xf6, 0x8a, 0x94, 0x5c, 0x22, 0xe7, 0x62, 0x11, 0x5c, 0x09, 0x42, 0x97,
                0xb1, 0x94, 0xbe, 0xdc, 0x0c, 0xe5, 0xa0, 0xc9, 0x2d, 0xb6, 0x4b, 0x83, 0x0a, 0x18,
                0xb4, 0x4d, 0xf0, 0xd0,
            ],
            Core::FullSubtract64 => [
                0xcd, 0x12, 0x1b, 0x7f, 0x43, 0x84, 0x0f, 0x96, 0xc4, 0x0a, 0xa4, 0x05, 0x6d, 0xef,
                0x80, 0xd1, 0x77, 0xb6, 0x19, 0xb2, 0x50, 0x5a, 0x94, 0x7a, 0x21, 0x47, 0x55, 0xfa,
                0x83, 0x2c, 0x7f, 0x0c,
            ],
            Core::FullSubtract8 => [
                0x11, 0x62, 0x66, 0x81, 0x54, 0x66, 0xfe, 0x6e, 0x6d, 0x47, 0x57, 0xf6, 0xb8, 0xad,
                0xe3, 0x34, 0xbe, 0xcd, 0xa9, 0xdb, 0x8c, 0x4e, 0xd7, 0xe3, 0x28, 0x13, 0x1f, 0x36,
                0x98, 0x08, 0x67, 0xf2,
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
            Core::Increment16 => [
                0x89, 0xca, 0xc7, 0x0e, 0x2b, 0xf1, 0x4c, 0xd4, 0xd4, 0x32, 0x75, 0xa0, 0x13, 0x5a,
                0x9f, 0xab, 0xc0, 0xeb, 0x5a, 0x33, 0xf6, 0x2d, 0x0f, 0x4f, 0xa4, 0x4a, 0x3c, 0xa7,
                0x3e, 0xda, 0x85, 0x27,
            ],
            Core::Increment32 => [
                0x44, 0xf6, 0x64, 0x2e, 0x7b, 0x8d, 0xe6, 0x98, 0x7b, 0x5f, 0x1e, 0x9e, 0x2f, 0x2e,
                0x28, 0x4a, 0x4c, 0xcb, 0xbc, 0x3c, 0x75, 0x5f, 0x23, 0x11, 0xc3, 0x4b, 0x50, 0xf9,
                0x4f, 0xa4, 0x48, 0xbe,
            ],
            Core::Increment64 => [
                0xe0, 0xb2, 0x61, 0x72, 0x28, 0x67, 0x29, 0xf5, 0xcd, 0xaf, 0x25, 0x16, 0x18, 0xb9,
                0x9e, 0x8e, 0xab, 0x93, 0xd8, 0x4a, 0xb9, 0xba, 0x87, 0x03, 0x06, 0xe6, 0x4d, 0xbc,
                0x5e, 0x3e, 0x09, 0x3d,
            ],
            Core::Increment8 => [
                0x71, 0xd9, 0x4f, 0xdb, 0x37, 0x95, 0x9c, 0x9a, 0x89, 0xcc, 0x1b, 0x79, 0x71, 0x2c,
                0xa1, 0x67, 0xda, 0xea, 0x47, 0xbe, 0xf8, 0x5f, 0x92, 0xd4, 0x06, 0x6b, 0x6e, 0x94,
                0xcc, 0x16, 0x16, 0xb7,
            ],
            Core::IsOne16 => [
                0x67, 0x31, 0xde, 0x96, 0x8d, 0x5c, 0xa6, 0xd1, 0x96, 0xd9, 0x0f, 0xb9, 0x3e, 0x23,
                0x2e, 0x20, 0x90, 0x1b, 0x49, 0x34, 0xfd, 0x17, 0xb7, 0x21, 0x14, 0x96, 0x92, 0xf8,
                0xfc, 0xa4, 0x3a, 0x4f,
            ],
            Core::IsOne32 => [
                0xea, 0x1a, 0x8c, 0xe7, 0x2f, 0x57, 0xbe, 0x4e, 0x29, 0x11, 0xb9, 0x14, 0xe2, 0x06,
                0x47, 0xda, 0xdc, 0xec, 0x87, 0xad, 0x13, 0x5c, 0xdf, 0x64, 0x3b, 0x67, 0x7f, 0xc2,
                0x14, 0x9d, 0xd6, 0x65,
            ],
            Core::IsOne64 => [
                0xf2, 0x8a, 0xe2, 0x7a, 0x4e, 0x4b, 0xeb, 0x42, 0xeb, 0x71, 0x9c, 0x5e, 0xae, 0xa4,
                0xc1, 0xaf, 0xac, 0x66, 0x8b, 0xdc, 0x08, 0x6a, 0x5f, 0x15, 0x4e, 0xb5, 0x79, 0x13,
                0x4f, 0xc2, 0x06, 0x42,
            ],
            Core::IsOne8 => [
                0xf1, 0xa2, 0x81, 0xd5, 0x56, 0xbf, 0x41, 0xcd, 0xa8, 0x78, 0x6c, 0x04, 0x53, 0x0e,
                0x32, 0x19, 0xfd, 0x58, 0x2f, 0xba, 0x2d, 0xff, 0x4e, 0x99, 0xbd, 0x27, 0x5d, 0x7f,
                0xf7, 0x9a, 0x45, 0x0f,
            ],
            Core::IsZero16 => [
                0x58, 0x22, 0x9c, 0x26, 0x64, 0x15, 0xd1, 0x05, 0xcf, 0x22, 0xb4, 0x2a, 0x9e, 0xde,
                0x72, 0x51, 0x48, 0xb3, 0xbc, 0x01, 0x76, 0x47, 0x5d, 0xe6, 0x2a, 0x74, 0x2d, 0x8e,
                0x11, 0xb5, 0x58, 0x39,
            ],
            Core::IsZero32 => [
                0x3d, 0x03, 0xf7, 0x9a, 0xda, 0x94, 0x9a, 0x47, 0xc4, 0x6e, 0x4e, 0x97, 0x36, 0xb5,
                0x2d, 0x2a, 0xc1, 0x93, 0x57, 0xd6, 0xca, 0xe9, 0x71, 0x0d, 0xd6, 0xde, 0xd8, 0x2d,
                0x12, 0x0a, 0xa5, 0xb5,
            ],
            Core::IsZero64 => [
                0xb4, 0x39, 0x4c, 0x5d, 0x5f, 0xd1, 0xdf, 0x3b, 0x6d, 0x02, 0x48, 0x95, 0x3f, 0xb2,
                0x61, 0x4c, 0x86, 0x7f, 0x96, 0xee, 0x4e, 0x0b, 0x05, 0x23, 0x0a, 0xe3, 0x52, 0x88,
                0x7f, 0x38, 0x32, 0x46,
            ],
            Core::IsZero8 => [
                0xdc, 0x3a, 0xae, 0xa2, 0x21, 0x96, 0xcc, 0x94, 0x94, 0xce, 0xe4, 0x24, 0xdc, 0x71,
                0x60, 0xad, 0x52, 0x8c, 0x62, 0xd7, 0x4d, 0xdf, 0x99, 0xd2, 0xdb, 0x9e, 0x7b, 0x5e,
                0x7e, 0xd2, 0xa0, 0x1b,
            ],
            Core::Le16 => [
                0x17, 0x43, 0xd1, 0xbc, 0x39, 0x38, 0xed, 0xc8, 0xdc, 0x65, 0x78, 0xa3, 0x52, 0xc4,
                0xf3, 0xec, 0x1f, 0x82, 0xea, 0xae, 0xfe, 0xa9, 0x33, 0x61, 0x89, 0xb9, 0xe2, 0x5c,
                0x3f, 0x13, 0x29, 0xf4,
            ],
            Core::Le32 => [
                0xea, 0x77, 0x2c, 0x0b, 0x5a, 0xec, 0xde, 0x7d, 0xc1, 0x6e, 0x3f, 0x1f, 0x95, 0x27,
                0x89, 0xf0, 0x13, 0x70, 0x89, 0x04, 0xce, 0xaa, 0x62, 0xcf, 0xdc, 0xf6, 0x4d, 0x30,
                0xa3, 0x91, 0x17, 0xaf,
            ],
            Core::Le64 => [
                0x37, 0xb6, 0x54, 0xea, 0xd2, 0x33, 0xc3, 0xef, 0xb8, 0x0b, 0x4f, 0x88, 0xa6, 0x13,
                0xba, 0x70, 0x24, 0xd9, 0x8e, 0xb3, 0x1a, 0x2c, 0x3e, 0xdc, 0x93, 0xb5, 0x1a, 0xa4,
                0x60, 0xbc, 0xf7, 0x83,
            ],
            Core::Le8 => [
                0xff, 0xdf, 0x7f, 0x6a, 0x8d, 0x3b, 0x2c, 0x1b, 0x06, 0xbd, 0x86, 0x40, 0xbd, 0x98,
                0xda, 0xa8, 0x9d, 0x2f, 0xcc, 0xa3, 0xa8, 0xb0, 0xc5, 0x70, 0x43, 0xea, 0xc2, 0x9a,
                0x05, 0xe7, 0xf2, 0xd3,
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
            Core::Low16 => [
                0x11, 0xd9, 0xe7, 0xcd, 0x36, 0x81, 0x3e, 0x73, 0xa9, 0x47, 0x92, 0xcf, 0xae, 0x99,
                0x63, 0xef, 0x4f, 0x64, 0x64, 0x8d, 0xf4, 0x8a, 0xac, 0xff, 0xe5, 0x7f, 0xb7, 0x0f,
                0x4e, 0x19, 0xac, 0xac,
            ],
            Core::Low32 => [
                0x96, 0xbe, 0x53, 0x34, 0x5d, 0x52, 0x14, 0xb0, 0x05, 0xc7, 0xfc, 0x5d, 0xe4, 0x0a,
                0x92, 0x62, 0x56, 0x60, 0x41, 0x40, 0x63, 0x35, 0x27, 0xc1, 0xd0, 0x2b, 0xe0, 0xd5,
                0xf6, 0x0c, 0xc2, 0xad,
            ],
            Core::Low64 => [
                0x61, 0x2a, 0x86, 0xb8, 0x22, 0x44, 0x94, 0xd8, 0x23, 0x0e, 0xe1, 0x16, 0xba, 0x47,
                0x89, 0x46, 0x4c, 0x57, 0xa7, 0xe0, 0x58, 0x52, 0x95, 0xb6, 0x1e, 0xc5, 0xc9, 0x6f,
                0x59, 0x83, 0x77, 0x60,
            ],
            Core::Low8 => [
                0x43, 0x95, 0xde, 0x24, 0xfd, 0x70, 0xf5, 0xc8, 0x62, 0xfe, 0xe6, 0x8c, 0x6f, 0x67,
                0xd6, 0x17, 0x12, 0xd8, 0xd6, 0x00, 0x3a, 0x38, 0xe1, 0xbd, 0x85, 0xbe, 0x4b, 0x94,
                0x52, 0x69, 0x92, 0xba,
            ],
            Core::Lt16 => [
                0xf9, 0xd9, 0xee, 0xbe, 0xc9, 0xc2, 0x3b, 0x0b, 0x9e, 0xb8, 0xfd, 0x72, 0x2a, 0x31,
                0x92, 0xcf, 0x7d, 0x28, 0x7d, 0xaa, 0xcf, 0xa5, 0x3e, 0x5d, 0xdc, 0xe7, 0x1b, 0x26,
                0x92, 0x94, 0x0f, 0x1c,
            ],
            Core::Lt32 => [
                0xc5, 0x23, 0x3b, 0x33, 0x0d, 0xec, 0x92, 0x06, 0x5e, 0x9e, 0xd3, 0x2d, 0x20, 0xd4,
                0xe0, 0xcf, 0xfd, 0x97, 0x56, 0xef, 0x11, 0xc8, 0x43, 0x86, 0x9f, 0xa7, 0x54, 0x37,
                0xef, 0x00, 0xda, 0xaf,
            ],
            Core::Lt64 => [
                0x3b, 0x8c, 0xbf, 0xda, 0x60, 0x38, 0xef, 0x26, 0x95, 0x06, 0x03, 0xdd, 0xd8, 0x61,
                0x9c, 0x19, 0x7f, 0x8d, 0xbe, 0x4e, 0x15, 0x4c, 0x15, 0x0b, 0x93, 0xd3, 0x8d, 0x0a,
                0xb9, 0x68, 0xf7, 0x67,
            ],
            Core::Lt8 => [
                0x4e, 0xe0, 0x55, 0x24, 0x23, 0xc4, 0x59, 0x57, 0x5f, 0xca, 0x1b, 0x39, 0xfc, 0xdc,
                0xf0, 0x56, 0x6f, 0xa8, 0x82, 0x1f, 0x14, 0x0c, 0x0a, 0x3c, 0xa4, 0x71, 0x35, 0xbb,
                0x3c, 0xd6, 0xa1, 0xf9,
            ],
            Core::Max16 => [
                0x13, 0xc1, 0xad, 0xf5, 0x49, 0xc7, 0x73, 0x0b, 0x87, 0x43, 0xf8, 0x95, 0x06, 0x4f,
                0xa4, 0xab, 0xd7, 0x79, 0xf4, 0xb4, 0x1d, 0x6d, 0xf7, 0xc6, 0x33, 0x08, 0x71, 0xa3,
                0x7a, 0xa7, 0x8c, 0xd7,
            ],
            Core::Max32 => [
                0xc7, 0xf9, 0xaa, 0x5b, 0x7e, 0x5e, 0x03, 0xeb, 0xad, 0x3b, 0xec, 0x46, 0xe7, 0x73,
                0x45, 0x9f, 0xf8, 0x6a, 0x73, 0x9f, 0x9a, 0x09, 0x01, 0x9e, 0xcc, 0x0b, 0x6c, 0x3c,
                0xdf, 0xf4, 0x1a, 0x7b,
            ],
            Core::Max64 => [
                0x12, 0x7d, 0xd5, 0x9e, 0x06, 0xc5, 0xd6, 0x93, 0x04, 0xeb, 0x92, 0xcf, 0xd6, 0xb4,
                0xf3, 0x65, 0xb7, 0xc2, 0xcd, 0x62, 0x5e, 0x90, 0x45, 0xbe, 0xde, 0xab, 0xf2, 0x31,
                0xc0, 0x14, 0xe8, 0x60,
            ],
            Core::Max8 => [
                0xaf, 0xe3, 0xc6, 0x8d, 0x17, 0x39, 0x5d, 0xea, 0x61, 0x8e, 0x15, 0xbd, 0x34, 0xae,
                0xad, 0xb8, 0xcb, 0xfb, 0xc8, 0x56, 0x00, 0x71, 0xfb, 0xd1, 0x61, 0x4d, 0xa8, 0xb1,
                0x3f, 0xdf, 0x5d, 0xdd,
            ],
            Core::Median16 => [
                0xa1, 0xa4, 0x1f, 0x4a, 0x60, 0x17, 0x94, 0x89, 0x9a, 0xa0, 0x50, 0x75, 0x66, 0x3e,
                0x6a, 0xfd, 0xe4, 0xb2, 0x13, 0x82, 0xb2, 0x77, 0x8d, 0xaf, 0x8f, 0x5c, 0x26, 0xeb,
                0xb3, 0xd3, 0xab, 0x53,
            ],
            Core::Median32 => [
                0xc5, 0xfe, 0x19, 0x50, 0x41, 0x84, 0xdc, 0xa2, 0xd3, 0xe8, 0xd0, 0x86, 0xf2, 0x11,
                0xe6, 0xab, 0x5f, 0x44, 0x6d, 0x6a, 0xcf, 0xc1, 0xe8, 0xfb, 0xa8, 0x12, 0x75, 0x90,
                0xd8, 0xcf, 0xc0, 0x4e,
            ],
            Core::Median64 => [
                0xf5, 0x32, 0x56, 0x7c, 0x74, 0x17, 0xcb, 0x0d, 0x19, 0xe4, 0x18, 0xd7, 0x2b, 0x31,
                0x9a, 0xf8, 0xa1, 0xce, 0x3c, 0xca, 0x90, 0x3b, 0xa2, 0x6d, 0x3b, 0x8f, 0x98, 0xf2,
                0xd8, 0x31, 0xa6, 0x1d,
            ],
            Core::Median8 => [
                0x1a, 0x64, 0xb1, 0x35, 0xf4, 0xb3, 0xa8, 0x04, 0xdb, 0xc1, 0x2a, 0x30, 0x59, 0xd9,
                0x54, 0xa1, 0x8c, 0xfb, 0x7f, 0x0b, 0x9f, 0xc5, 0x2e, 0xb9, 0x5f, 0x4a, 0x1c, 0x30,
                0xef, 0x48, 0x6c, 0xad,
            ],
            Core::Min16 => [
                0x26, 0xc0, 0x2d, 0x7c, 0xff, 0xb8, 0x84, 0xad, 0x54, 0x92, 0x88, 0x2b, 0x4b, 0x96,
                0x62, 0x8e, 0x07, 0x53, 0xeb, 0xa9, 0xf5, 0x13, 0x7f, 0x13, 0x49, 0xe9, 0x43, 0xaf,
                0x70, 0x3a, 0xd4, 0x39,
            ],
            Core::Min32 => [
                0xc6, 0x18, 0xd5, 0x77, 0xa5, 0xbd, 0x0c, 0xe3, 0xeb, 0x3d, 0xbb, 0xe5, 0x1b, 0xad,
                0x5c, 0x9f, 0x9f, 0x10, 0xce, 0xc4, 0x70, 0x59, 0xcb, 0xb5, 0x82, 0x0f, 0x8a, 0xba,
                0x05, 0x47, 0xa5, 0xca,
            ],
            Core::Min64 => [
                0xdb, 0x6d, 0xe6, 0x99, 0x42, 0x80, 0x6c, 0xfe, 0xbf, 0x21, 0xae, 0x01, 0x80, 0x67,
                0x3e, 0xd3, 0x72, 0x93, 0x42, 0xc0, 0x53, 0x48, 0x5b, 0x9b, 0x7e, 0xa7, 0xae, 0xaa,
                0x51, 0xb6, 0xbf, 0xd7,
            ],
            Core::Min8 => [
                0x25, 0xc8, 0xdc, 0x61, 0xf7, 0x3e, 0xfa, 0xe8, 0xd7, 0xdf, 0x91, 0x70, 0xc5, 0xf0,
                0xcb, 0xb9, 0xf7, 0x62, 0x65, 0xa3, 0xad, 0x95, 0xcc, 0x5c, 0x27, 0xe7, 0x28, 0x74,
                0x05, 0xe0, 0x6d, 0x8c,
            ],
            Core::Modulo16 => [
                0x25, 0x22, 0x08, 0xa3, 0xe2, 0x82, 0x5d, 0x78, 0xa9, 0x49, 0x5d, 0x81, 0x02, 0x5b,
                0x83, 0x99, 0x08, 0xaa, 0x70, 0x54, 0xe9, 0x0b, 0x0d, 0x2d, 0xdb, 0xbb, 0x0c, 0x8d,
                0x84, 0xe5, 0xc8, 0x37,
            ],
            Core::Modulo32 => [
                0x1a, 0xa0, 0xf2, 0xed, 0x81, 0xe6, 0xaf, 0x95, 0x8d, 0x1a, 0x3b, 0x72, 0xa6, 0xda,
                0x7f, 0x17, 0x38, 0x53, 0x63, 0xf2, 0xd2, 0xbd, 0x20, 0xd2, 0x93, 0x50, 0x6c, 0x0a,
                0x18, 0x58, 0x28, 0xfb,
            ],
            Core::Modulo64 => [
                0x00, 0xd4, 0x46, 0x3d, 0xec, 0x3c, 0x73, 0x0d, 0x7f, 0xed, 0x1a, 0x23, 0xc6, 0x6b,
                0xc3, 0xf4, 0xa2, 0xc7, 0x16, 0xa0, 0xce, 0x8e, 0x5d, 0x80, 0x50, 0x5b, 0x28, 0xef,
                0xb9, 0x3d, 0xbc, 0x8b,
            ],
            Core::Modulo8 => [
                0x57, 0x8a, 0xe6, 0x7d, 0xa0, 0x09, 0xf3, 0xfe, 0xc1, 0x0f, 0x45, 0x64, 0x40, 0xaa,
                0xcd, 0x54, 0x16, 0x0a, 0x16, 0xf4, 0x47, 0x5f, 0x05, 0x8f, 0xb3, 0x4b, 0xd2, 0xd3,
                0x75, 0x10, 0xc9, 0x56,
            ],
            Core::Multiply16 => [
                0x3c, 0xd8, 0x35, 0x22, 0x72, 0xaa, 0xa2, 0xc5, 0x49, 0x65, 0xf0, 0xe8, 0x66, 0xa6,
                0xe0, 0x81, 0xb2, 0x09, 0x84, 0x8c, 0x3b, 0x0a, 0x90, 0x6e, 0xcf, 0x02, 0x64, 0x15,
                0xa6, 0x53, 0x4a, 0xb3,
            ],
            Core::Multiply32 => [
                0x16, 0x1f, 0xd0, 0x3a, 0x92, 0xc6, 0x21, 0xb3, 0x28, 0x98, 0x49, 0xff, 0x29, 0xad,
                0x81, 0x34, 0x99, 0xd6, 0x3e, 0xd9, 0x73, 0xdb, 0x0e, 0x97, 0x51, 0x78, 0x54, 0x21,
                0xf5, 0x68, 0xd1, 0x8f,
            ],
            Core::Multiply64 => [
                0x22, 0x25, 0xf1, 0xaf, 0x99, 0xf5, 0x2f, 0xf9, 0x49, 0xea, 0x46, 0xb8, 0xf1, 0xce,
                0xf6, 0x2f, 0x68, 0xaa, 0x3a, 0x42, 0x60, 0x11, 0x2e, 0xc9, 0xcd, 0x74, 0xd7, 0x8e,
                0xbe, 0x1d, 0x15, 0xe7,
            ],
            Core::Multiply8 => [
                0x80, 0x24, 0x3b, 0x83, 0x65, 0x8c, 0x33, 0x0c, 0x3b, 0xc1, 0x93, 0x9e, 0x45, 0x4f,
                0x53, 0xaa, 0x74, 0xdf, 0x6c, 0xf0, 0xa1, 0x9d, 0x1d, 0x67, 0xf2, 0x14, 0x6c, 0x9c,
                0xbe, 0xe9, 0x51, 0x30,
            ],
            Core::Negate16 => [
                0x02, 0x69, 0xd7, 0x3f, 0x09, 0x4d, 0x59, 0x41, 0x19, 0x73, 0xea, 0xcd, 0xd5, 0xd3,
                0xd9, 0xe4, 0x7c, 0xab, 0xb8, 0x27, 0x7e, 0x6e, 0xf4, 0x11, 0x15, 0x7a, 0x44, 0x8a,
                0xe1, 0x25, 0x2d, 0x33,
            ],
            Core::Negate32 => [
                0xf6, 0xb2, 0x1e, 0x3f, 0x59, 0x3e, 0xbd, 0x97, 0xec, 0x16, 0x1f, 0xd4, 0xf8, 0x54,
                0x43, 0xd3, 0x65, 0xc0, 0x23, 0x07, 0x5a, 0x22, 0xb6, 0x8c, 0xa2, 0x6c, 0xf6, 0xb7,
                0x8b, 0xab, 0x94, 0xb0,
            ],
            Core::Negate64 => [
                0x2b, 0xf4, 0x8c, 0x08, 0xa5, 0x7f, 0x18, 0x36, 0xea, 0x2f, 0x57, 0x22, 0xc0, 0x79,
                0x59, 0xa7, 0xd6, 0x4c, 0xbe, 0xf7, 0x05, 0xdc, 0xc3, 0xca, 0xba, 0x3a, 0x90, 0x05,
                0x03, 0xd4, 0x89, 0x59,
            ],
            Core::Negate8 => [
                0x25, 0x6d, 0x50, 0x79, 0x8b, 0x7f, 0xe9, 0xe7, 0xcb, 0x16, 0x96, 0xfb, 0x18, 0x83,
                0x19, 0x01, 0xef, 0x88, 0x95, 0x2c, 0xc4, 0x60, 0x6b, 0x68, 0x97, 0x4a, 0x79, 0xca,
                0x8a, 0x8a, 0x00, 0x44,
            ],
            Core::One16 => [
                0x8e, 0x67, 0xce, 0x64, 0xee, 0x18, 0xb6, 0x44, 0xc5, 0x5f, 0xd0, 0x7c, 0x71, 0x93,
                0x2c, 0xa6, 0xe0, 0x29, 0x0b, 0xab, 0xef, 0xf9, 0x49, 0x25, 0xdd, 0x6d, 0xf2, 0x3a,
                0x2e, 0xe4, 0xd1, 0xa2,
            ],
            Core::One32 => [
                0x45, 0x26, 0x09, 0x9d, 0x0f, 0x84, 0x0b, 0xa6, 0xab, 0xe5, 0x5e, 0xd9, 0xf3, 0xa9,
                0xb8, 0xfc, 0xa0, 0x84, 0xf2, 0x51, 0xbf, 0xb2, 0x6d, 0x01, 0x1c, 0xf7, 0xaf, 0x27,
                0xb0, 0xd8, 0x8e, 0x29,
            ],
            Core::One64 => [
                0xc1, 0xa3, 0x4a, 0x00, 0xe3, 0x1e, 0xe1, 0x55, 0x2e, 0x6d, 0x09, 0xc6, 0x70, 0x77,
                0x2b, 0xcb, 0x18, 0x63, 0xc1, 0x0f, 0x14, 0x82, 0xd0, 0x8c, 0xf0, 0xd9, 0xa1, 0x83,
                0x2c, 0x59, 0xec, 0xf4,
            ],
            Core::One8 => [
                0xf9, 0x58, 0x76, 0x11, 0x81, 0xef, 0xf2, 0x30, 0xcb, 0xc5, 0x1c, 0xc0, 0xe4, 0x5f,
                0x66, 0x91, 0x1b, 0x32, 0x19, 0x16, 0x0e, 0x62, 0x7f, 0xa8, 0x10, 0xc0, 0x7d, 0xfe,
                0xa0, 0xb9, 0x9d, 0x6a,
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
                0x2f, 0xec, 0x1d, 0x6b, 0x08, 0xa8, 0xf8, 0x19, 0xb8, 0xf4, 0x6a, 0x60, 0xfb, 0x90,
                0xac, 0x17, 0xb8, 0x84, 0x64, 0xcf, 0xb6, 0x08, 0x38, 0x49, 0xaf, 0xea, 0xde, 0x1f,
                0x34, 0xf1, 0xe7, 0xbc,
            ],
            Core::Sha256Ctx8Add128 => [
                0x20, 0x0b, 0x99, 0x32, 0xe7, 0x4d, 0x3d, 0x13, 0x51, 0x2d, 0x64, 0x44, 0xee, 0x64,
                0x45, 0xed, 0x17, 0xee, 0x33, 0xd2, 0x9f, 0x39, 0x79, 0xd8, 0x39, 0x24, 0xe6, 0xa5,
                0x9c, 0x99, 0xd8, 0x10,
            ],
            Core::Sha256Ctx8Add16 => [
                0x29, 0x26, 0x3f, 0x97, 0xa3, 0xf4, 0xbd, 0x82, 0x7f, 0x20, 0x2d, 0x4b, 0xf9, 0xea,
                0x57, 0xaa, 0x84, 0x1a, 0xae, 0x6e, 0xe9, 0xa1, 0x3f, 0xa7, 0xb4, 0x59, 0xe7, 0x99,
                0x9b, 0x15, 0x44, 0x97,
            ],
            Core::Sha256Ctx8Add2 => [
                0xb4, 0x59, 0x0d, 0x05, 0x6c, 0x52, 0xbe, 0x9c, 0x16, 0x61, 0x00, 0x03, 0x53, 0x50,
                0xd8, 0xf4, 0x3d, 0x31, 0x0f, 0x3f, 0x57, 0x56, 0x17, 0xc6, 0x39, 0x81, 0x95, 0x48,
                0xb5, 0xb4, 0x40, 0x87,
            ],
            Core::Sha256Ctx8Add256 => [
                0xcf, 0x64, 0x38, 0x27, 0x7c, 0x66, 0xb4, 0xe6, 0x30, 0x7e, 0xcf, 0xac, 0x31, 0x52,
                0x26, 0xc7, 0xb9, 0xaa, 0xdb, 0xda, 0x33, 0xeb, 0xe1, 0x9d, 0x62, 0x04, 0x65, 0xef,
                0x48, 0x44, 0xd8, 0x08,
            ],
            Core::Sha256Ctx8Add32 => [
                0x69, 0xa6, 0xec, 0xb2, 0x7a, 0x04, 0x49, 0xf3, 0xd6, 0x7a, 0xb9, 0xd9, 0x61, 0x96,
                0x1d, 0x6a, 0xa8, 0x15, 0x07, 0xde, 0x69, 0x51, 0xb0, 0xd4, 0xce, 0xaf, 0xf6, 0xd8,
                0xbd, 0x38, 0x80, 0x27,
            ],
            Core::Sha256Ctx8Add4 => [
                0xa5, 0x82, 0xd2, 0xbc, 0xdf, 0xaf, 0x71, 0xfa, 0xf7, 0xb9, 0xae, 0x04, 0x5f, 0x97,
                0xaa, 0xc0, 0x55, 0xc4, 0x77, 0x9c, 0x6e, 0x80, 0x22, 0xe9, 0xfe, 0x11, 0xfb, 0xa2,
                0xa4, 0x44, 0xad, 0xb1,
            ],
            Core::Sha256Ctx8Add512 => [
                0x89, 0x5f, 0x1b, 0x6c, 0x9a, 0x41, 0xf8, 0x3f, 0xd3, 0x27, 0x23, 0xc7, 0xa2, 0x2d,
                0xe2, 0x65, 0xa1, 0x27, 0x90, 0xaa, 0xa8, 0x83, 0x17, 0xac, 0x01, 0xdd, 0xbb, 0xd1,
                0x4b, 0x60, 0xd4, 0x1d,
            ],
            Core::Sha256Ctx8Add64 => [
                0x78, 0x15, 0x65, 0x01, 0x89, 0xa1, 0x17, 0xba, 0x09, 0x12, 0x08, 0x8b, 0x79, 0x29,
                0x7a, 0x07, 0x66, 0xb6, 0x09, 0x45, 0x87, 0xbf, 0x5d, 0xa6, 0xe8, 0xf4, 0x6e, 0x9c,
                0x37, 0xb4, 0x34, 0x91,
            ],
            Core::Sha256Ctx8Add8 => [
                0x1e, 0x8d, 0xb5, 0xba, 0x26, 0x05, 0xf0, 0xf3, 0x05, 0xd5, 0xb7, 0xd5, 0x0c, 0x09,
                0x00, 0x6e, 0xa4, 0x85, 0x58, 0x46, 0x3f, 0x6e, 0x0c, 0x85, 0xfe, 0x50, 0x51, 0xf0,
                0xc0, 0x53, 0xaf, 0xeb,
            ],
            Core::Sha256Ctx8AddBuffer511 => [
                0xf8, 0x9d, 0x7e, 0x21, 0x9b, 0x3e, 0xd8, 0x81, 0x4d, 0xd9, 0xf5, 0xc1, 0xa5, 0xda,
                0xb6, 0xba, 0xd6, 0xd3, 0x2d, 0xc5, 0x72, 0xa5, 0x21, 0x35, 0x39, 0x2c, 0x81, 0x43,
                0x0e, 0x12, 0x03, 0xd1,
            ],
            Core::Sha256Ctx8Finalize => [
                0xc9, 0xcd, 0x48, 0x36, 0x64, 0xf9, 0xaf, 0x94, 0xd5, 0xe9, 0x41, 0xdb, 0xb9, 0xe1,
                0xe9, 0xb7, 0xc6, 0x4f, 0x48, 0xef, 0x15, 0xc9, 0x06, 0x5f, 0xd1, 0xbb, 0x7f, 0xd9,
                0xc5, 0x6c, 0xbd, 0xc6,
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
            Core::Subtract16 => [
                0x62, 0x4a, 0xf4, 0x3d, 0x27, 0x5e, 0x2a, 0x20, 0x70, 0x3e, 0x66, 0x52, 0x3d, 0x35,
                0xdd, 0x34, 0xef, 0x8e, 0x8b, 0x29, 0x3a, 0x57, 0x38, 0x0a, 0x21, 0xdb, 0xf6, 0x10,
                0xcd, 0x62, 0x7a, 0xa3,
            ],
            Core::Subtract32 => [
                0xf7, 0x6e, 0xca, 0xd1, 0xfd, 0xa5, 0x0f, 0x13, 0x5b, 0xdf, 0xe3, 0xe5, 0x33, 0xa1,
                0x50, 0x09, 0x8f, 0x40, 0x62, 0x61, 0xc7, 0x6f, 0x6d, 0xbf, 0x67, 0x25, 0xf1, 0xe3,
                0x88, 0x3c, 0x2a, 0xe2,
            ],
            Core::Subtract64 => [
                0x99, 0x4b, 0x25, 0x00, 0x38, 0x7d, 0x8d, 0x62, 0x02, 0x09, 0x67, 0x74, 0x16, 0xb9,
                0xe6, 0x04, 0x52, 0x6c, 0x70, 0x8f, 0xf1, 0xf9, 0x65, 0xc9, 0xc9, 0x12, 0x52, 0x04,
                0x7a, 0x3f, 0x57, 0xb3,
            ],
            Core::Subtract8 => [
                0x57, 0xa0, 0xd0, 0x8e, 0x5f, 0x01, 0xe8, 0x38, 0x8d, 0x78, 0x5c, 0xcb, 0x26, 0x57,
                0xc4, 0xea, 0x98, 0xb8, 0x54, 0xa6, 0x58, 0x5c, 0x65, 0x8a, 0x21, 0x0d, 0xb0, 0x90,
                0x1d, 0x10, 0xf9, 0x48,
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
            Core::Add16 => b"i",
            Core::Add32 => b"l",
            Core::Add64 => b"*ll",
            Core::Add8 => b"****22*22**22*22***22*22**22*22",
            Core::Bip0340Verify => b"**hh*hh",
            Core::CheckSigVerify => b"**h*hh*hh",
            Core::Decompress => b"*2h",
            Core::Decrement16 => b"****22*22**22*22***22*22**22*22",
            Core::Decrement32 => b"i",
            Core::Decrement64 => b"l",
            Core::Decrement8 => b"***22*22**22*22",
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
            Core::FullMultiply16 => b"l",
            Core::FullMultiply32 => b"*ll",
            Core::FullMultiply64 => b"h",
            Core::FullMultiply8 => b"i",
            Core::FullSubtract16 => b"*2i",
            Core::FullSubtract32 => b"*2l",
            Core::FullSubtract64 => b"*2*ll",
            Core::FullSubtract8 => b"*2****22*22**22*22***22*22**22*22",
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
            Core::LinearCombination1 => b"**h**hhhh",
            Core::LinearVerify1 => b"***h*hhh*hh",
            Core::Low16 => b"1",
            Core::Low32 => b"1",
            Core::Low64 => b"1",
            Core::Low8 => b"1",
            Core::Lt16 => b"i",
            Core::Lt32 => b"l",
            Core::Lt64 => b"*ll",
            Core::Lt8 => b"****22*22**22*22***22*22**22*22",
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
            Core::Subtract16 => b"i",
            Core::Subtract32 => b"l",
            Core::Subtract64 => b"*ll",
            Core::Subtract8 => b"****22*22**22*22***22*22**22*22",
            Core::Verify => b"2",
        };

        TypeName(name)
    }

    fn target_ty(&self) -> TypeName {
        let name: &'static [u8] = match self {
            Core::Add16 => b"*2****22*22**22*22***22*22**22*22",
            Core::Add32 => b"*2i",
            Core::Add64 => b"*2l",
            Core::Add8 => b"*2***22*22**22*22",
            Core::Bip0340Verify => b"1",
            Core::CheckSigVerify => b"1",
            Core::Decompress => b"+1*hh",
            Core::Decrement16 => b"*2****22*22**22*22***22*22**22*22",
            Core::Decrement32 => b"*2i",
            Core::Decrement64 => b"*2l",
            Core::Decrement8 => b"*2***22*22**22*22",
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
            Core::FullMultiply16 => b"i",
            Core::FullMultiply32 => b"l",
            Core::FullMultiply64 => b"*ll",
            Core::FullMultiply8 => b"****22*22**22*22***22*22**22*22",
            Core::FullSubtract16 => b"*2****22*22**22*22***22*22**22*22",
            Core::FullSubtract32 => b"*2i",
            Core::FullSubtract64 => b"*2l",
            Core::FullSubtract8 => b"*2***22*22**22*22",
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
            Core::LinearCombination1 => b"**hhh",
            Core::LinearVerify1 => b"1",
            Core::Low16 => b"****22*22**22*22***22*22**22*22",
            Core::Low32 => b"i",
            Core::Low64 => b"l",
            Core::Low8 => b"***22*22**22*22",
            Core::Lt16 => b"2",
            Core::Lt32 => b"2",
            Core::Lt64 => b"2",
            Core::Lt8 => b"2",
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
            Core::Subtract16 => b"*2****22*22**22*22***22*22**22*22",
            Core::Subtract32 => b"*2i",
            Core::Subtract64 => b"*2l",
            Core::Subtract8 => b"*2***22*22**22*22",
            Core::Verify => b"1",
        };

        TypeName(name)
    }

    fn encode<W: Write>(&self, w: &mut BitWriter<W>) -> std::io::Result<usize> {
        let (n, len) = match self {
            Core::Verify => (0, 2),
            Core::Low8 => (37, 7),
            Core::Low16 => (304, 10),
            Core::Low32 => (305, 10),
            Core::Low64 => (306, 10),
            Core::Eq8 => (877, 11),
            Core::Eq16 => (7024, 14),
            Core::Eq32 => (7025, 14),
            Core::Eq64 => (7026, 14),
            Core::Eq256 => (14056, 15),
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
                                                                1 => {}
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
            Core::Add16 => &simplicity_sys::c_jets::jets_wrapper::add_16,
            Core::Add32 => &simplicity_sys::c_jets::jets_wrapper::add_32,
            Core::Add64 => &simplicity_sys::c_jets::jets_wrapper::add_64,
            Core::Add8 => &simplicity_sys::c_jets::jets_wrapper::add_8,
            Core::Bip0340Verify => &simplicity_sys::c_jets::jets_wrapper::bip_0340_verify,
            Core::CheckSigVerify => &simplicity_sys::c_jets::jets_wrapper::check_sig_verify,
            Core::Decompress => &simplicity_sys::c_jets::jets_wrapper::decompress,
            Core::Decrement16 => &simplicity_sys::c_jets::jets_wrapper::decrement_16,
            Core::Decrement32 => &simplicity_sys::c_jets::jets_wrapper::decrement_32,
            Core::Decrement64 => &simplicity_sys::c_jets::jets_wrapper::decrement_64,
            Core::Decrement8 => &simplicity_sys::c_jets::jets_wrapper::decrement_8,
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
            Core::FullMultiply16 => &simplicity_sys::c_jets::jets_wrapper::full_multiply_16,
            Core::FullMultiply32 => &simplicity_sys::c_jets::jets_wrapper::full_multiply_32,
            Core::FullMultiply64 => &simplicity_sys::c_jets::jets_wrapper::full_multiply_64,
            Core::FullMultiply8 => &simplicity_sys::c_jets::jets_wrapper::full_multiply_8,
            Core::FullSubtract16 => &simplicity_sys::c_jets::jets_wrapper::full_subtract_16,
            Core::FullSubtract32 => &simplicity_sys::c_jets::jets_wrapper::full_subtract_32,
            Core::FullSubtract64 => &simplicity_sys::c_jets::jets_wrapper::full_subtract_64,
            Core::FullSubtract8 => &simplicity_sys::c_jets::jets_wrapper::full_subtract_8,
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
            Core::LinearCombination1 => &simplicity_sys::c_jets::jets_wrapper::linear_combination_1,
            Core::LinearVerify1 => &simplicity_sys::c_jets::jets_wrapper::linear_verify_1,
            Core::Low16 => &simplicity_sys::c_jets::jets_wrapper::low_16,
            Core::Low32 => &simplicity_sys::c_jets::jets_wrapper::low_32,
            Core::Low64 => &simplicity_sys::c_jets::jets_wrapper::low_64,
            Core::Low8 => &simplicity_sys::c_jets::jets_wrapper::low_8,
            Core::Lt16 => &simplicity_sys::c_jets::jets_wrapper::lt_16,
            Core::Lt32 => &simplicity_sys::c_jets::jets_wrapper::lt_32,
            Core::Lt64 => &simplicity_sys::c_jets::jets_wrapper::lt_64,
            Core::Lt8 => &simplicity_sys::c_jets::jets_wrapper::lt_8,
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
            Core::Subtract16 => &simplicity_sys::c_jets::jets_wrapper::subtract_16,
            Core::Subtract32 => &simplicity_sys::c_jets::jets_wrapper::subtract_32,
            Core::Subtract64 => &simplicity_sys::c_jets::jets_wrapper::subtract_64,
            Core::Subtract8 => &simplicity_sys::c_jets::jets_wrapper::subtract_8,
            Core::Verify => &simplicity_sys::c_jets::jets_wrapper::verify,
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
            Core::Bip0340Verify => f.write_str("bip_0340_verify"),
            Core::CheckSigVerify => f.write_str("check_sig_verify"),
            Core::Decompress => f.write_str("decompress"),
            Core::Decrement16 => f.write_str("decrement_16"),
            Core::Decrement32 => f.write_str("decrement_32"),
            Core::Decrement64 => f.write_str("decrement_64"),
            Core::Decrement8 => f.write_str("decrement_8"),
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
            Core::FullMultiply16 => f.write_str("full_multiply_16"),
            Core::FullMultiply32 => f.write_str("full_multiply_32"),
            Core::FullMultiply64 => f.write_str("full_multiply_64"),
            Core::FullMultiply8 => f.write_str("full_multiply_8"),
            Core::FullSubtract16 => f.write_str("full_subtract_16"),
            Core::FullSubtract32 => f.write_str("full_subtract_32"),
            Core::FullSubtract64 => f.write_str("full_subtract_64"),
            Core::FullSubtract8 => f.write_str("full_subtract_8"),
            Core::GeIsOnCurve => f.write_str("ge_is_on_curve"),
            Core::GeNegate => f.write_str("ge_negate"),
            Core::GejAdd => f.write_str("gej_add"),
            Core::GejDouble => f.write_str("gej_double"),
            Core::GejGeAdd => f.write_str("gej_ge_add"),
            Core::GejGeAddEx => f.write_str("gej_ge_add_ex"),
            Core::GejInfinity => f.write_str("gej_infinity"),
            Core::GejIsInfinity => f.write_str("gej_is_infinity"),
            Core::GejIsOnCurve => f.write_str("gej_is_on_curve"),
            Core::GejNegate => f.write_str("gej_negate"),
            Core::GejNormalize => f.write_str("gej_normalize"),
            Core::GejRescale => f.write_str("gej_rescale"),
            Core::GejXEquiv => f.write_str("gej_x_equiv"),
            Core::GejYIsOdd => f.write_str("gej_y_is_odd"),
            Core::Generate => f.write_str("generate"),
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
            Core::LinearCombination1 => f.write_str("linear_combination_1"),
            Core::LinearVerify1 => f.write_str("linear_verify_1"),
            Core::Low16 => f.write_str("low_16"),
            Core::Low32 => f.write_str("low_32"),
            Core::Low64 => f.write_str("low_64"),
            Core::Low8 => f.write_str("low_8"),
            Core::Lt16 => f.write_str("lt_16"),
            Core::Lt32 => f.write_str("lt_32"),
            Core::Lt64 => f.write_str("lt_64"),
            Core::Lt8 => f.write_str("lt_8"),
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
            Core::ParseLock => f.write_str("parse_lock"),
            Core::ParseSequence => f.write_str("parse_sequence"),
            Core::PointVerify1 => f.write_str("point_verify_1"),
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
            Core::Subtract16 => f.write_str("subtract_16"),
            Core::Subtract32 => f.write_str("subtract_32"),
            Core::Subtract64 => f.write_str("subtract_64"),
            Core::Subtract8 => f.write_str("subtract_8"),
            Core::Verify => f.write_str("verify"),
        }
    }
}

impl str::FromStr for Core {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self, Error> {
        match s {
            "add_16" => Ok(Core::Add16),
            "add_32" => Ok(Core::Add32),
            "add_64" => Ok(Core::Add64),
            "add_8" => Ok(Core::Add8),
            "bip_0340_verify" => Ok(Core::Bip0340Verify),
            "check_sig_verify" => Ok(Core::CheckSigVerify),
            "decompress" => Ok(Core::Decompress),
            "decrement_16" => Ok(Core::Decrement16),
            "decrement_32" => Ok(Core::Decrement32),
            "decrement_64" => Ok(Core::Decrement64),
            "decrement_8" => Ok(Core::Decrement8),
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
            "full_multiply_16" => Ok(Core::FullMultiply16),
            "full_multiply_32" => Ok(Core::FullMultiply32),
            "full_multiply_64" => Ok(Core::FullMultiply64),
            "full_multiply_8" => Ok(Core::FullMultiply8),
            "full_subtract_16" => Ok(Core::FullSubtract16),
            "full_subtract_32" => Ok(Core::FullSubtract32),
            "full_subtract_64" => Ok(Core::FullSubtract64),
            "full_subtract_8" => Ok(Core::FullSubtract8),
            "ge_is_on_curve" => Ok(Core::GeIsOnCurve),
            "ge_negate" => Ok(Core::GeNegate),
            "gej_add" => Ok(Core::GejAdd),
            "gej_double" => Ok(Core::GejDouble),
            "gej_ge_add" => Ok(Core::GejGeAdd),
            "gej_ge_add_ex" => Ok(Core::GejGeAddEx),
            "gej_infinity" => Ok(Core::GejInfinity),
            "gej_is_infinity" => Ok(Core::GejIsInfinity),
            "gej_is_on_curve" => Ok(Core::GejIsOnCurve),
            "gej_negate" => Ok(Core::GejNegate),
            "gej_normalize" => Ok(Core::GejNormalize),
            "gej_rescale" => Ok(Core::GejRescale),
            "gej_x_equiv" => Ok(Core::GejXEquiv),
            "gej_y_is_odd" => Ok(Core::GejYIsOdd),
            "generate" => Ok(Core::Generate),
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
            "linear_combination_1" => Ok(Core::LinearCombination1),
            "linear_verify_1" => Ok(Core::LinearVerify1),
            "low_16" => Ok(Core::Low16),
            "low_32" => Ok(Core::Low32),
            "low_64" => Ok(Core::Low64),
            "low_8" => Ok(Core::Low8),
            "lt_16" => Ok(Core::Lt16),
            "lt_32" => Ok(Core::Lt32),
            "lt_64" => Ok(Core::Lt64),
            "lt_8" => Ok(Core::Lt8),
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
            "parse_lock" => Ok(Core::ParseLock),
            "parse_sequence" => Ok(Core::ParseSequence),
            "point_verify_1" => Ok(Core::PointVerify1),
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
            "subtract_16" => Ok(Core::Subtract16),
            "subtract_32" => Ok(Core::Subtract32),
            "subtract_64" => Ok(Core::Subtract64),
            "subtract_8" => Ok(Core::Subtract8),
            "verify" => Ok(Core::Verify),
            x => Err(Error::InvalidJetName(x.to_owned())),
        }
    }
}
