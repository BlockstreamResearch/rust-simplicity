//! Execution related FFIs, only used for testing rust-simplicity
//! Also exports test cases from C simplicity
use std::os::raw::c_uchar;

use libc::size_t;

extern "C" {
    // These structures are stored as C arrays arr[]
    // The FFI bindings give the value of the first element and NOT the address
    // of the first element. This is why we have schnorr0 as u8 and not *const u8.
    // Another cleaner/longer way to do this would be write a wrapper function that
    // takes a rust allocated array and copies the C array into it.
    static schnorr0: [c_uchar; 0usize];
    static sizeof_schnorr0: size_t;
    static schnorr0_cmr: [u32; 8]; // size 8
    static schnorr0_amr: [u32; 8]; // size 8
    static schnorr0_imr: [u32; 8]; // size 8

    static schnorr6: [c_uchar; 0usize];
    static sizeof_schnorr6: size_t;
    static schnorr6_cmr: [u32; 8]; // size 8
    static schnorr6_amr: [u32; 8]; // size 8
    static schnorr6_imr: [u32; 8]; // size 8

    static hashBlock: [c_uchar; 0usize];
    static sizeof_hashBlock: size_t;
    static hashBlock_cmr: [u32; 8]; // size 8
    static hashBlock_amr: [u32; 8]; // size 8
    static hashBlock_imr: [u32; 8]; // size 8
}

pub fn parse_root(ptr: &[u32; 8]) -> [u8; 32] {
    let mut a = [0u8; 32];
    for i in 0..8 {
        let x = ptr[i];
        a[i * 4] = (x >> 24) as u8;
        a[i * 4 + 1] = (x >> 16) as u8;
        a[i * 4 + 2] = (x >> 8) as u8;
        a[i * 4 + 3] = x as u8;
    }
    a
}

pub fn parse_prog(ptr: *const u8, len: usize) -> Vec<u8> {
    // Could construct from raw parts, but we just allocate because
    // this is only used in tests
    let mut v = Vec::with_capacity(len);
    unsafe {
        for i in 0..len {
            v.push(*ptr.offset(i as isize));
        }
    }
    v
}

/// Data structure to hold test cases from C simplicity
pub struct TestData {
    pub cmr: [u8; 32],
    pub amr: [u8; 32],
    pub imr: [u8; 32],
    pub prog: Vec<u8>,
}

pub fn schnorr0_test_data() -> TestData {
    unsafe {
        TestData {
            cmr: parse_root(&schnorr0_cmr),
            amr: parse_root(&schnorr0_amr),
            imr: parse_root(&schnorr0_imr),
            prog: parse_prog(schnorr0.as_ptr(), sizeof_schnorr0 as usize),
        }
    }
}

pub fn schnorr6_test_data() -> TestData {
    unsafe {
        TestData {
            cmr: parse_root(&schnorr6_cmr),
            amr: parse_root(&schnorr6_amr),
            imr: parse_root(&schnorr6_imr),
            prog: parse_prog(schnorr6.as_ptr(), sizeof_schnorr6 as usize),
        }
    }
}

pub fn hash_block_test_data() -> TestData {
    unsafe {
        TestData {
            cmr: parse_root(&hashBlock_cmr),
            amr: parse_root(&hashBlock_amr),
            imr: parse_root(&hashBlock_imr),
            prog: parse_prog(hashBlock.as_ptr(), sizeof_hashBlock as usize),
        }
    }
}
