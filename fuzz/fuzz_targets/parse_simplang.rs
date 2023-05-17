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

use std::str;
use honggfuzz::fuzz;
use simplang::parse::parse;
use simplicity::jet::Elements;

fn do_test(data: &[u8]) {
    let s = match str::from_utf8(data) {
        Ok(s) => s,
        Err(_) => return,
    };

    if let Ok(program) = parse::<Elements>(s) {
        let reserialize= simplang::decompile::to_human_readable(&program);
        let round_trip = parse::<Elements>(&reserialize).unwrap();
        assert_eq!(program, round_trip);
    }
}

fn main() {
    loop {
        fuzz!(|data| {
            do_test(data);
        });
    }
}
