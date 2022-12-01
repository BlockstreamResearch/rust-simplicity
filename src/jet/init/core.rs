/* This file has been automatically generated. */

use crate::bititer::BitIter;
use crate::bitwriter::BitWriter;
use crate::jet::type_name::TypeName;
use crate::jet::Jet;
use crate::merkle::cmr::Cmr;
use crate::{decode_bits, Error};
use bitcoin_hashes::sha256::Midstate;
use simplicity_sys::c_jets::CTxEnv;
use simplicity_sys::CFrameItem;
use std::io::Write;

/// Core jet family
#[derive(Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Debug, Hash)]
pub enum Core {
    Add32,
    FullAdd32,
    Sub32,
    FullSub32,
    Mul32,
    FullMul32,
    Eq32Verify,
    Eq256Verify,
    Lt32Verify,
    Sha256,
    Sha256Block,
    Bip0340Verify,
}

impl Jet for Core {
    type Environment = ();

    fn cmr(&self) -> Cmr {
        let bytes = match self {
            Core::Add32 => [
                0x5d, 0x5c, 0x8f, 0xf3, 0x86, 0xd5, 0xa0, 0x14, 0x08, 0xe9, 0xe0, 0x79, 0xed, 0x95,
                0x2c, 0xb9, 0xc1, 0xdc, 0x86, 0x14, 0xfc, 0x1f, 0x3e, 0x54, 0x61, 0xab, 0x1c, 0x30,
                0x24, 0xdc, 0xea, 0x54,
            ],
            Core::FullAdd32 => [
                0xf0, 0x95, 0x9d, 0x3c, 0xb9, 0x2c, 0x72, 0x8c, 0xd0, 0x86, 0x26, 0x81, 0x71, 0xaa,
                0x1f, 0xdd, 0x5c, 0x97, 0x4c, 0xbe, 0x3f, 0xf6, 0x4a, 0x09, 0x94, 0x13, 0x28, 0x76,
                0x6d, 0x24, 0xbf, 0xf1,
            ],
            Core::Sub32 => [
                0x01, 0x6d, 0x32, 0x48, 0xee, 0x72, 0x7e, 0xb7, 0x27, 0xc3, 0x3a, 0xa6, 0xf2, 0xcf,
                0xb8, 0xb8, 0x7e, 0x7d, 0x07, 0x46, 0x55, 0x40, 0xdc, 0x3f, 0x9a, 0xb3, 0x22, 0x93,
                0x78, 0x85, 0x2a, 0xc7,
            ],
            Core::FullSub32 => [
                0x34, 0x73, 0xfa, 0x10, 0xe0, 0xe7, 0xd9, 0x80, 0x2d, 0x53, 0x3b, 0x13, 0x01, 0xb2,
                0x09, 0x83, 0x85, 0x92, 0xb3, 0x1a, 0xf9, 0xd9, 0x14, 0xb0, 0xe7, 0x46, 0x11, 0x32,
                0xf5, 0x3d, 0x79, 0x7a,
            ],
            Core::Mul32 => [
                0x02, 0x44, 0x52, 0xa5, 0x7a, 0xc5, 0x8c, 0xd0, 0xa1, 0x97, 0x57, 0xbb, 0xf1, 0x68,
                0xa3, 0xa8, 0xcb, 0x6a, 0x02, 0x38, 0xa8, 0x0f, 0x61, 0x81, 0x3e, 0xf7, 0x9c, 0x92,
                0x6c, 0x8f, 0x08, 0x9e,
            ],
            Core::FullMul32 => [
                0x47, 0xe0, 0xca, 0x35, 0x3a, 0x6f, 0x93, 0x4b, 0xd9, 0x97, 0x5d, 0xfe, 0x04, 0x27,
                0x62, 0x96, 0x42, 0x94, 0xf7, 0x51, 0xd1, 0xd4, 0x6d, 0x39, 0xcf, 0xa5, 0xee, 0x5f,
                0x3a, 0x37, 0x8b, 0xfd,
            ],
            Core::Eq32Verify => [
                0x32, 0x61, 0x8d, 0x01, 0xfb, 0xfe, 0x81, 0x9f, 0x29, 0x69, 0xb7, 0x1c, 0xda, 0xbf,
                0x40, 0x5d, 0xde, 0x3d, 0xa1, 0x7c, 0x04, 0x45, 0xe8, 0xd0, 0x53, 0x47, 0x65, 0x7c,
                0x5b, 0x53, 0x2f, 0x72,
            ],
            Core::Eq256Verify => [
                0x7c, 0x1d, 0x68, 0x82, 0xe5, 0x38, 0x22, 0xe8, 0x0c, 0x5d, 0x7d, 0x36, 0xf8, 0x59,
                0xc1, 0xc4, 0x02, 0xfe, 0x29, 0x10, 0xcf, 0xbc, 0xa2, 0x32, 0xc0, 0x67, 0x97, 0x25,
                0x6b, 0xe3, 0xdb, 0x07,
            ],
            Core::Lt32Verify => [
                0xa1, 0xfa, 0x71, 0x96, 0xbe, 0x58, 0x35, 0x47, 0xcf, 0xb5, 0x15, 0x25, 0xc7, 0x65,
                0x2f, 0xc2, 0x14, 0x0c, 0x70, 0x46, 0xab, 0xab, 0x4a, 0x8c, 0x3a, 0x25, 0x1f, 0x1e,
                0xa3, 0xc3, 0x94, 0xc1,
            ],
            Core::Sha256 => [
                0xd6, 0x49, 0xd3, 0x03, 0xd1, 0x96, 0xcd, 0x53, 0xfe, 0x29, 0x86, 0xfc, 0x6b, 0x81,
                0x25, 0x08, 0xb5, 0x5d, 0x23, 0xaa, 0xa3, 0x92, 0xf4, 0xf3, 0xa7, 0xd0, 0x8c, 0x6c,
                0xad, 0xb2, 0xf8, 0xac,
            ],
            Core::Sha256Block => [
                0xd5, 0xb6, 0xf8, 0x48, 0x44, 0x17, 0x32, 0x12, 0xe2, 0x69, 0x9e, 0x99, 0xa8, 0x9b,
                0xcd, 0x3e, 0xb7, 0xf8, 0xe9, 0x6c, 0x0c, 0xc8, 0x46, 0x7f, 0x1c, 0xf2, 0xc2, 0x50,
                0x14, 0x74, 0x59, 0x48,
            ],
            Core::Bip0340Verify => [
                0xd6, 0xdf, 0x54, 0x94, 0x7e, 0xb9, 0x27, 0x86, 0xb3, 0x79, 0xd3, 0xec, 0x81, 0x93,
                0x99, 0x4a, 0x0f, 0xfe, 0xdd, 0xc2, 0x86, 0x46, 0xa9, 0x21, 0x4e, 0xea, 0xc8, 0xf9,
                0x34, 0x1f, 0x56, 0x4e,
            ],
        };

        Cmr(Midstate(bytes))
    }

    fn source_ty(&self) -> TypeName {
        let name: &'static [u8] = match self {
            Core::Add32 => b"l",
            Core::FullAdd32 => b"*l2",
            Core::Sub32 => b"l",
            Core::FullSub32 => b"*l2",
            Core::Mul32 => b"l",
            Core::FullMul32 => b"*ll",
            Core::Eq32Verify => b"l",
            Core::Eq256Verify => b"*hh",
            Core::Lt32Verify => b"l",
            Core::Sha256 => b"h",
            Core::Sha256Block => b"*h*hh",
            Core::Bip0340Verify => b"**hh*hh",
        };

        TypeName(name)
    }

    fn target_ty(&self) -> TypeName {
        let name: &'static [u8] = match self {
            Core::Add32 => b"*2i",
            Core::FullAdd32 => b"*2i",
            Core::Sub32 => b"*2i",
            Core::FullSub32 => b"*2i",
            Core::Mul32 => b"l",
            Core::FullMul32 => b"l",
            Core::Eq32Verify => b"1",
            Core::Eq256Verify => b"1",
            Core::Lt32Verify => b"1",
            Core::Sha256 => b"h",
            Core::Sha256Block => b"h",
            Core::Bip0340Verify => b"1",
        };

        TypeName(name)
    }

    fn encode<W: Write>(&self, w: &mut BitWriter<W>) -> std::io::Result<usize> {
        let (n, len) = match self {
            Core::Add32 => (16, 5),
            Core::FullAdd32 => (20, 5),
            Core::Sub32 => (17, 5),
            Core::FullSub32 => (21, 5),
            Core::Mul32 => (9, 4),
            Core::FullMul32 => (11, 4),
            Core::Eq32Verify => (116, 7),
            Core::Eq256Verify => (113, 7),
            Core::Lt32Verify => (115, 7),
            Core::Sha256 => (114, 7),
            Core::Sha256Block => (6, 3),
            Core::Bip0340Verify => (112, 7),
        };

        w.write_bits_be(n, len)
    }

    fn decode<I: Iterator<Item = u8>>(bits: &mut BitIter<I>) -> Result<Self, Error> {
        decode_bits!(bits, {
            0 => {},
            1 => {
                0 => {
                    0 => {
                        0 => {
                            0 => {Core::Add32},
                            1 => {Core::Sub32}
                        },
                        1 => {Core::Mul32}
                    },
                    1 => {
                        0 => {
                            0 => {Core::FullAdd32},
                            1 => {Core::FullSub32}
                        },
                        1 => {Core::FullMul32}
                    }
                },
                1 => {
                    0 => {Core::Sha256Block},
                    1 => {
                        0 => {
                            0 => {
                                0 => {
                                    0 => {Core::Bip0340Verify},
                                    1 => {Core::Eq256Verify}
                                },
                                1 => {
                                    0 => {Core::Sha256},
                                    1 => {Core::Lt32Verify}
                                }
                            },
                            1 => {
                                0 => {
                                    0 => {Core::Eq32Verify},
                                    1 => {}
                                },
                                1 => {}
                            }
                        },
                        1 => {}
                    }
                }
            }
        })
    }

    fn c_jet_ptr(&self) -> &'static dyn Fn(&mut CFrameItem, CFrameItem, &CTxEnv) -> bool {
        match self {
            Core::Add32 => &simplicity_sys::c_jets::jets_wrapper::add_32,
            Core::FullAdd32 => todo!(),
            Core::Sub32 => todo!(),
            Core::FullSub32 => todo!(),
            Core::Mul32 => todo!(),
            Core::FullMul32 => todo!(),
            Core::Eq32Verify => todo!(),
            Core::Eq256Verify => todo!(),
            Core::Lt32Verify => todo!(),
            Core::Sha256 => todo!(),
            Core::Sha256Block => todo!(),
            Core::Bip0340Verify => todo!(),
        }
    }
}
