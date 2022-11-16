/* This file has been automatically generated. */

use crate::bititer::BitIter;
use crate::bitwriter::BitWriter;
use crate::exec::BitMachine;
use crate::jet::bitcoin::BitcoinEnv;
use crate::jet::type_name::TypeName;
use crate::jet::{Jet, JetFailed};
use crate::merkle::cmr::Cmr;
use crate::{decode_bits, Error};
use bitcoin_hashes::sha256::Midstate;
use std::io::Write;

/// Bitcoin jet family
#[derive(Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Debug, Hash)]
pub enum Bitcoin {
    Version,
    LockTime,
    InputsHash,
    OutputsHash,
    NumInputs,
    TotalInputValue,
    CurrentPrevOutpoint,
    CurrentValue,
    CurrentSequence,
    CurrentIndex,
    InputPrevOutpoint,
    InputValue,
    InputSequence,
    NumOutputs,
    TotalOutputValue,
    OutputValue,
    OutputScriptHash,
    ScriptCMR,
    SighashAll,
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

impl Jet for Bitcoin {
    type Environment = BitcoinEnv;

    fn cmr(&self) -> Cmr {
        let bytes = match self {
            Bitcoin::Version => [
                0xeb, 0xa0, 0x5b, 0xe6, 0x60, 0xd9, 0x9d, 0xa6, 0x0b, 0xbd, 0xd4, 0xfe, 0x72, 0x23,
                0xe1, 0x0b, 0x79, 0x67, 0x75, 0x62, 0xfc, 0xb6, 0xab, 0xd9, 0x84, 0xae, 0xcf, 0xf5,
                0xda, 0xe9, 0xaa, 0x79,
            ],
            Bitcoin::LockTime => [
                0x0f, 0x45, 0xe8, 0x46, 0x79, 0xe5, 0xf5, 0x33, 0x81, 0xb6, 0x97, 0x1f, 0x95, 0x81,
                0x26, 0x6f, 0x58, 0x4c, 0xb9, 0xd0, 0x76, 0xf4, 0x5b, 0x7b, 0xed, 0xad, 0x9d, 0x4f,
                0x3e, 0x04, 0x43, 0x0c,
            ],
            Bitcoin::InputsHash => [
                0x0f, 0xa9, 0xbf, 0xa7, 0x71, 0x84, 0x89, 0x0a, 0xd9, 0x2f, 0xfc, 0x45, 0x8e, 0x42,
                0x19, 0xa4, 0xa5, 0x3d, 0x1e, 0x98, 0x53, 0x0c, 0xe0, 0xd0, 0xdc, 0x9e, 0x00, 0x0c,
                0x33, 0x9a, 0xd9, 0xab,
            ],
            Bitcoin::OutputsHash => [
                0x45, 0x9a, 0x08, 0xe7, 0x6b, 0x75, 0x19, 0x37, 0xd8, 0x02, 0x2f, 0xef, 0xe4, 0x11,
                0xba, 0xb2, 0x62, 0xd7, 0xa1, 0xdd, 0x4b, 0x71, 0x61, 0x09, 0x63, 0x87, 0x17, 0xe2,
                0x19, 0x19, 0x96, 0x21,
            ],
            Bitcoin::CurrentValue => [
                0xcf, 0x34, 0x62, 0x80, 0xec, 0xb3, 0x9f, 0x59, 0x1d, 0x12, 0x15, 0xc9, 0x3b, 0x08,
                0x66, 0x43, 0x70, 0x3b, 0x16, 0x24, 0x49, 0x97, 0xe6, 0x73, 0xf7, 0x13, 0xf9, 0x40,
                0x63, 0x69, 0x82, 0x98,
            ],
            Bitcoin::CurrentSequence => [0; 32],
            Bitcoin::CurrentIndex => [
                0xd8, 0x7a, 0x1f, 0x6d, 0x73, 0x91, 0x1d, 0xcb, 0x6f, 0x58, 0x20, 0xf6, 0x2a, 0xe3,
                0x85, 0xae, 0x65, 0x90, 0x17, 0x0f, 0x84, 0xe7, 0x51, 0x68, 0x41, 0xda, 0x5a, 0x17,
                0xcb, 0x50, 0x94, 0xe7,
            ],
            Bitcoin::SighashAll => [
                0x93, 0xfc, 0x7f, 0x9a, 0x3c, 0x97, 0xdf, 0x07, 0x3c, 0xc4, 0x95, 0x7f, 0x97, 0x80,
                0xdf, 0x88, 0x65, 0x99, 0x63, 0xad, 0x4e, 0x4d, 0x37, 0xd1, 0x8b, 0x1a, 0xa5, 0x4a,
                0x45, 0x57, 0x4a, 0x66,
            ],
            Bitcoin::Add32 => [
                0x5d, 0x5c, 0x8f, 0xf3, 0x86, 0xd5, 0xa0, 0x14, 0x08, 0xe9, 0xe0, 0x79, 0xed, 0x95,
                0x2c, 0xb9, 0xc1, 0xdc, 0x86, 0x14, 0xfc, 0x1f, 0x3e, 0x54, 0x61, 0xab, 0x1c, 0x30,
                0x24, 0xdc, 0xea, 0x54,
            ],
            Bitcoin::FullAdd32 => [
                0xf0, 0x95, 0x9d, 0x3c, 0xb9, 0x2c, 0x72, 0x8c, 0xd0, 0x86, 0x26, 0x81, 0x71, 0xaa,
                0x1f, 0xdd, 0x5c, 0x97, 0x4c, 0xbe, 0x3f, 0xf6, 0x4a, 0x09, 0x94, 0x13, 0x28, 0x76,
                0x6d, 0x24, 0xbf, 0xf1,
            ],
            Bitcoin::Sub32 => [
                0x01, 0x6d, 0x32, 0x48, 0xee, 0x72, 0x7e, 0xb7, 0x27, 0xc3, 0x3a, 0xa6, 0xf2, 0xcf,
                0xb8, 0xb8, 0x7e, 0x7d, 0x07, 0x46, 0x55, 0x40, 0xdc, 0x3f, 0x9a, 0xb3, 0x22, 0x93,
                0x78, 0x85, 0x2a, 0xc7,
            ],
            Bitcoin::FullSub32 => [
                0x34, 0x73, 0xfa, 0x10, 0xe0, 0xe7, 0xd9, 0x80, 0x2d, 0x53, 0x3b, 0x13, 0x01, 0xb2,
                0x09, 0x83, 0x85, 0x92, 0xb3, 0x1a, 0xf9, 0xd9, 0x14, 0xb0, 0xe7, 0x46, 0x11, 0x32,
                0xf5, 0x3d, 0x79, 0x7a,
            ],
            Bitcoin::Mul32 => [
                0x02, 0x44, 0x52, 0xa5, 0x7a, 0xc5, 0x8c, 0xd0, 0xa1, 0x97, 0x57, 0xbb, 0xf1, 0x68,
                0xa3, 0xa8, 0xcb, 0x6a, 0x02, 0x38, 0xa8, 0x0f, 0x61, 0x81, 0x3e, 0xf7, 0x9c, 0x92,
                0x6c, 0x8f, 0x08, 0x9e,
            ],
            Bitcoin::FullMul32 => [
                0x47, 0xe0, 0xca, 0x35, 0x3a, 0x6f, 0x93, 0x4b, 0xd9, 0x97, 0x5d, 0xfe, 0x04, 0x27,
                0x62, 0x96, 0x42, 0x94, 0xf7, 0x51, 0xd1, 0xd4, 0x6d, 0x39, 0xcf, 0xa5, 0xee, 0x5f,
                0x3a, 0x37, 0x8b, 0xfd,
            ],
            Bitcoin::Eq32Verify => [
                0x32, 0x61, 0x8d, 0x01, 0xfb, 0xfe, 0x81, 0x9f, 0x29, 0x69, 0xb7, 0x1c, 0xda, 0xbf,
                0x40, 0x5d, 0xde, 0x3d, 0xa1, 0x7c, 0x04, 0x45, 0xe8, 0xd0, 0x53, 0x47, 0x65, 0x7c,
                0x5b, 0x53, 0x2f, 0x72,
            ],
            Bitcoin::Eq256Verify => [
                0x7c, 0x1d, 0x68, 0x82, 0xe5, 0x38, 0x22, 0xe8, 0x0c, 0x5d, 0x7d, 0x36, 0xf8, 0x59,
                0xc1, 0xc4, 0x02, 0xfe, 0x29, 0x10, 0xcf, 0xbc, 0xa2, 0x32, 0xc0, 0x67, 0x97, 0x25,
                0x6b, 0xe3, 0xdb, 0x07,
            ],
            Bitcoin::Lt32Verify => [
                0xa1, 0xfa, 0x71, 0x96, 0xbe, 0x58, 0x35, 0x47, 0xcf, 0xb5, 0x15, 0x25, 0xc7, 0x65,
                0x2f, 0xc2, 0x14, 0x0c, 0x70, 0x46, 0xab, 0xab, 0x4a, 0x8c, 0x3a, 0x25, 0x1f, 0x1e,
                0xa3, 0xc3, 0x94, 0xc1,
            ],
            Bitcoin::Sha256 => [
                0xd6, 0x49, 0xd3, 0x03, 0xd1, 0x96, 0xcd, 0x53, 0xfe, 0x29, 0x86, 0xfc, 0x6b, 0x81,
                0x25, 0x08, 0xb5, 0x5d, 0x23, 0xaa, 0xa3, 0x92, 0xf4, 0xf3, 0xa7, 0xd0, 0x8c, 0x6c,
                0xad, 0xb2, 0xf8, 0xac,
            ],
            Bitcoin::Sha256Block => [
                0xd5, 0xb6, 0xf8, 0x48, 0x44, 0x17, 0x32, 0x12, 0xe2, 0x69, 0x9e, 0x99, 0xa8, 0x9b,
                0xcd, 0x3e, 0xb7, 0xf8, 0xe9, 0x6c, 0x0c, 0xc8, 0x46, 0x7f, 0x1c, 0xf2, 0xc2, 0x50,
                0x14, 0x74, 0x59, 0x48,
            ],
            Bitcoin::Bip0340Verify => [
                0xd6, 0xdf, 0x54, 0x94, 0x7e, 0xb9, 0x27, 0x86, 0xb3, 0x79, 0xd3, 0xec, 0x81, 0x93,
                0x99, 0x4a, 0x0f, 0xfe, 0xdd, 0xc2, 0x86, 0x46, 0xa9, 0x21, 0x4e, 0xea, 0xc8, 0xf9,
                0x34, 0x1f, 0x56, 0x4e,
            ],
            Bitcoin::NumInputs
            | Bitcoin::TotalInputValue
            | Bitcoin::CurrentPrevOutpoint
            | Bitcoin::InputPrevOutpoint
            | Bitcoin::InputValue
            | Bitcoin::InputSequence
            | Bitcoin::NumOutputs
            | Bitcoin::TotalOutputValue
            | Bitcoin::OutputValue
            | Bitcoin::OutputScriptHash
            | Bitcoin::ScriptCMR => unimplemented!("Undefined jet CMR"),
        };

        Cmr(Midstate(bytes))
    }

    fn source_ty(&self) -> TypeName {
        let name: &'static [u8] = match self {
            Bitcoin::Version => b"1",
            Bitcoin::LockTime => b"1",
            Bitcoin::InputsHash => b"1",
            Bitcoin::OutputsHash => b"1",
            Bitcoin::CurrentValue => b"1",
            Bitcoin::CurrentSequence => b"1",
            Bitcoin::CurrentIndex => b"1",
            Bitcoin::SighashAll => b"1",
            Bitcoin::Add32 => b"l",
            Bitcoin::FullAdd32 => b"*l2",
            Bitcoin::Sub32 => b"l",
            Bitcoin::FullSub32 => b"*l2",
            Bitcoin::Mul32 => b"l",
            Bitcoin::FullMul32 => b"*ll",
            Bitcoin::Eq32Verify => b"l",
            Bitcoin::Eq256Verify => b"*hh",
            Bitcoin::Lt32Verify => b"l",
            Bitcoin::Sha256 => b"h",
            Bitcoin::Sha256Block => b"*h*hh",
            Bitcoin::Bip0340Verify => b"**hh*hh",
            Bitcoin::NumInputs
            | Bitcoin::TotalInputValue
            | Bitcoin::CurrentPrevOutpoint
            | Bitcoin::InputPrevOutpoint
            | Bitcoin::InputValue
            | Bitcoin::InputSequence
            | Bitcoin::NumOutputs
            | Bitcoin::TotalOutputValue
            | Bitcoin::OutputValue
            | Bitcoin::OutputScriptHash
            | Bitcoin::ScriptCMR => unimplemented!("Undefined jet source type"),
        };

        TypeName(name)
    }

    fn target_ty(&self) -> TypeName {
        let name: &'static [u8] = match self {
            Bitcoin::Version => b"i",
            Bitcoin::LockTime => b"i",
            Bitcoin::InputsHash => b"h",
            Bitcoin::OutputsHash => b"h",
            Bitcoin::CurrentValue => b"l",
            Bitcoin::CurrentSequence => b"i",
            Bitcoin::CurrentIndex => b"i",
            Bitcoin::SighashAll => b"h",
            Bitcoin::Add32 => b"*2i",
            Bitcoin::FullAdd32 => b"*2i",
            Bitcoin::Sub32 => b"*2i",
            Bitcoin::FullSub32 => b"*2i",
            Bitcoin::Mul32 => b"l",
            Bitcoin::FullMul32 => b"l",
            Bitcoin::Eq32Verify => b"1",
            Bitcoin::Eq256Verify => b"1",
            Bitcoin::Lt32Verify => b"1",
            Bitcoin::Sha256 => b"h",
            Bitcoin::Sha256Block => b"h",
            Bitcoin::Bip0340Verify => b"1",
            Bitcoin::NumInputs
            | Bitcoin::TotalInputValue
            | Bitcoin::CurrentPrevOutpoint
            | Bitcoin::InputPrevOutpoint
            | Bitcoin::InputValue
            | Bitcoin::InputSequence
            | Bitcoin::NumOutputs
            | Bitcoin::TotalOutputValue
            | Bitcoin::OutputValue
            | Bitcoin::OutputScriptHash
            | Bitcoin::ScriptCMR => unimplemented!("Undefined jet target type"),
        };

        TypeName(name)
    }

    fn encode<W: Write>(&self, w: &mut BitWriter<W>) -> std::io::Result<usize> {
        let (n, len) = match self {
            Bitcoin::Version => (0, 6),
            Bitcoin::LockTime => (1, 6),
            Bitcoin::InputsHash => unimplemented!("Undefined jet encoding"),
            Bitcoin::OutputsHash => (2, 5),
            Bitcoin::NumInputs => (3, 5),
            Bitcoin::TotalInputValue => (4, 5),
            Bitcoin::CurrentPrevOutpoint => (5, 5),
            Bitcoin::CurrentValue => (6, 5),
            Bitcoin::CurrentSequence => (7, 5),
            Bitcoin::CurrentIndex => (16, 6),
            Bitcoin::InputPrevOutpoint => (17, 6),
            Bitcoin::InputValue => (9, 5),
            Bitcoin::InputSequence => (10, 5),
            Bitcoin::NumOutputs => (11, 5),
            Bitcoin::TotalOutputValue => (12, 5),
            Bitcoin::OutputValue => (13, 5),
            Bitcoin::OutputScriptHash => (14, 5),
            Bitcoin::ScriptCMR => (15, 5),
            Bitcoin::SighashAll => (1, 5),
            Bitcoin::Add32 => (16, 5),
            Bitcoin::FullAdd32 => (20, 5),
            Bitcoin::Sub32 => (17, 5),
            Bitcoin::FullSub32 => (21, 5),
            Bitcoin::Mul32 => (9, 4),
            Bitcoin::FullMul32 => (11, 4),
            Bitcoin::Eq32Verify => (116, 7),
            Bitcoin::Eq256Verify => (113, 7),
            Bitcoin::Lt32Verify => (115, 7),
            Bitcoin::Sha256 => (114, 7),
            Bitcoin::Sha256Block => (6, 3),
            Bitcoin::Bip0340Verify => (112, 7),
        };

        w.write_bits_be(n, len)
    }

    fn decode<I: Iterator<Item = u8>>(bits: &mut BitIter<I>) -> Result<Self, Error> {
        decode_bits!(bits, {
            0 => {
                0 => {
                    0 => {
                        0 => {
                            0 => {
                                0 => {Bitcoin::Version},
                                1 => {Bitcoin::LockTime}
                            },
                            1 => {Bitcoin::SighashAll}
                        },
                        1 => {
                            0 => {Bitcoin::OutputsHash},
                            1 => {Bitcoin::NumInputs}
                        }
                    },
                    1 => {
                        0 => {
                            0 => {Bitcoin::TotalInputValue},
                            1 => {Bitcoin::CurrentPrevOutpoint}
                        },
                        1 => {
                            0 => {Bitcoin::CurrentValue},
                            1 => {Bitcoin::CurrentSequence}
                        }
                    }
                },
                1 => {
                    0 => {
                        0 => {
                            0 => {
                                0 => {Bitcoin::CurrentIndex},
                                1 => {Bitcoin::InputPrevOutpoint}
                            },
                            1 => {Bitcoin::InputValue}
                        },
                        1 => {
                            0 => {Bitcoin::InputSequence},
                            1 => {Bitcoin::NumOutputs}
                        }
                    },
                    1 => {
                        0 => {
                            0 => {Bitcoin::TotalOutputValue},
                            1 => {Bitcoin::OutputValue}
                        },
                        1 => {
                            0 => {Bitcoin::OutputScriptHash},
                            1 => {Bitcoin::ScriptCMR}
                        }
                    }
                }
            },
            1 => {
                0 => {
                    0 => {
                        0 => {
                            0 => {Bitcoin::Add32},
                            1 => {Bitcoin::Sub32}
                        },
                        1 => {Bitcoin::Mul32}
                    },
                    1 => {
                        0 => {
                            0 => {Bitcoin::FullAdd32},
                            1 => {Bitcoin::FullSub32}
                        },
                        1 => {Bitcoin::FullMul32}
                    }
                },
                1 => {
                    0 => {Bitcoin::Sha256Block},
                    1 => {
                        0 => {
                            0 => {
                                0 => {
                                    0 => {Bitcoin::Bip0340Verify},
                                    1 => {Bitcoin::Eq256Verify}
                                },
                                1 => {
                                    0 => {Bitcoin::Sha256},
                                    1 => {Bitcoin::Lt32Verify}
                                }
                            },
                            1 => {
                                0 => {
                                    0 => {Bitcoin::Eq32Verify},
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

    fn exec(&self) -> fn(&mut BitMachine, &Self::Environment) -> Result<(), JetFailed> {
        match self {
            Bitcoin::Version => crate::jet::bitcoin::version,
            Bitcoin::LockTime => crate::jet::bitcoin::lock_time,
            Bitcoin::InputsHash => crate::jet::bitcoin::inputs_hash,
            Bitcoin::OutputsHash => crate::jet::bitcoin::outputs_hash,
            Bitcoin::CurrentValue => crate::jet::bitcoin::current_value,
            Bitcoin::CurrentIndex => crate::jet::bitcoin::current_index,
            Bitcoin::Add32 => crate::jet::bitcoin::add_32,
            Bitcoin::FullAdd32 => crate::jet::bitcoin::full_add_32,
            Bitcoin::Sub32 => crate::jet::bitcoin::sub_32,
            Bitcoin::FullSub32 => crate::jet::bitcoin::full_sub_32,
            Bitcoin::Mul32 => crate::jet::bitcoin::mul_32,
            Bitcoin::FullMul32 => crate::jet::bitcoin::full_mul_32,
            Bitcoin::Eq32Verify => crate::jet::bitcoin::eq_32_verify,
            Bitcoin::Eq256Verify => crate::jet::bitcoin::eq_256_verify,
            Bitcoin::Lt32Verify => crate::jet::bitcoin::lt_32_verify,
            Bitcoin::Sha256 => crate::jet::bitcoin::sha256,
            Bitcoin::Sha256Block => crate::jet::bitcoin::sha256_block,
            Bitcoin::NumInputs
            | Bitcoin::TotalInputValue
            | Bitcoin::CurrentPrevOutpoint
            | Bitcoin::CurrentSequence
            | Bitcoin::InputPrevOutpoint
            | Bitcoin::InputValue
            | Bitcoin::InputSequence
            | Bitcoin::NumOutputs
            | Bitcoin::TotalOutputValue
            | Bitcoin::OutputValue
            | Bitcoin::OutputScriptHash
            | Bitcoin::ScriptCMR
            | Bitcoin::SighashAll
            | Bitcoin::Bip0340Verify => unimplemented!("Undefined jet execution"),
        }
    }
}
