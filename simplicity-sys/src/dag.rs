// Simplicity Bindings
// Written in 2022 by
//   Christian Lewe <clewe@blockstream.com>
//
// To the extent possible under law, the author(s) have dedicated all
// copyright and related and neighboring rights to this software to
// the public domain worldwide. This software is distributed without
// any warranty.
//
// You should have received a copy of the CC0 Public Domain Dedication
// along with this software.
// If not, see <http://creativecommons.org/publicdomain/zero/1.0/>.
//

use crate::bitstream::Bitstream;
use crate::bitstring::Bitstring;
use crate::error::Error;
use crate::util;
use libc::{c_void, size_t};
use std::fmt;

/// Kind of Simplicity node.
#[repr(C)]
#[derive(Copy, Clone, Eq, PartialEq, Debug)]
pub enum Tag {
    COMP,
    CASE,
    ASSERTL,
    ASSERTR,
    PAIR,
    DISCONNECT,
    INJL,
    INJR,
    TAKE,
    DROP,
    IDEN,
    UNIT,
    HIDDEN,
    WITNESS,
    JET,
}

#[repr(C)]
#[derive(Copy, Clone)]
union AuxTypes {
    /// scratch space for verifyCanonicalOrder
    aux: size_t,
    /// source type, target type
    types: [size_t; 2],
}

#[repr(C)]
#[derive(Copy, Clone)]
union IndicesChildrenWitness {
    /// source index, target index
    indices: [size_t; 2],
    /// first child, second child
    children: [size_t; 2],
    /// witness
    witness: Bitstring,
}

#[repr(C)]
struct PrivateDagNode {
    jet: *const c_void,
    cmr: [u32; 8],
    aux_types: AuxTypes,
    indices_children_witness: IndicesChildrenWitness,
    tag: Tag,
}

extern "C" {
    fn decodeMallocDag(
        dag: *mut *mut PrivateDagNode,
        combinator_counters: *const c_void,
        stream: *mut Bitstream,
    ) -> i32;
}

/// Simplicity DAG (typed or untyped).
/// Uses the Elements extension by default.
pub struct Dag {
    first: *mut PrivateDagNode,
    len: usize,
}

impl fmt::Debug for Dag {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("Dag").field("cmr", &self.cmr()).finish()
    }
}

impl Dag {
    /// Decode a Simplicity DAG from bytes.
    pub fn decode(bytes: &[u8]) -> Result<Self, Error> {
        let mut bitstream = Bitstream::from(bytes);
        let mut dag: *mut PrivateDagNode = std::ptr::null_mut();

        unsafe {
            let len =
                Error::get_result(decodeMallocDag(&mut dag, std::ptr::null(), &mut bitstream))?;

            if dag.is_null() {
                Err(Error::Malloc)
            } else {
                Ok(Dag {
                    first: dag,
                    len: len as usize,
                })
            }
        }
    }

    /// Return the CMR of the DAG.
    pub fn cmr(&self) -> [u8; 32] {
        unsafe { util::into_u8_merkle_root(&(*self.root_node()).cmr) }
    }

    /// Return the root node of the DAG.
    unsafe fn root_node(&self) -> *const PrivateDagNode {
        self.get(self.len - 1)
    }

    // TODO: Add notion of `DagNode<'a>` that is spawned by &'a self
    // and that enables read-only access to node fields
    /// Return the node at the given `index` in the DAG.
    /// Panics if `index` is out of bounds.
    unsafe fn get(&self, index: usize) -> *const PrivateDagNode {
        self.first.add(index)
    }
}

impl Drop for Dag {
    fn drop(&mut self) {
        unsafe {
            libc::free(self.first as *mut libc::c_void);
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use crate::test::SCHNORR0;

    #[test]
    fn decode_cmr() {
        let bytes = SCHNORR0.bytes;
        let dag = Dag::decode(bytes).expect("decoding");
        assert_eq!(SCHNORR0.cmr(), dag.cmr());
    }
}
