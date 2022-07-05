// Rust Simplicity Library
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

extern crate simplicity;

use simplicity::bititer::BitIter;
use simplicity::core::term_dag::TermDag;
use simplicity::decode;
use simplicity::jet::application::Core;

fn do_test(data: &[u8]) {
    let mut iter = BitIter::new(data.iter().cloned());

    if let Ok(program) = decode::decode_program_no_witness::<_, Core>(&mut iter) {
        let dag = TermDag::from_untyped_program(&program);
        let program_from_dag = dag.to_untyped_program();
        assert_eq!(program, program_from_dag);
    }
}

#[cfg(feature = "afl")]
#[macro_use]
extern crate afl;
#[cfg(feature = "afl")]
fn main() {
    fuzz!(|data| {
        do_test(&data);
    });
}

#[cfg(feature = "honggfuzz")]
#[macro_use]
extern crate honggfuzz;
#[cfg(feature = "honggfuzz")]
fn main() {
    loop {
        fuzz!(|data| {
            do_test(data);
        });
    }
}
