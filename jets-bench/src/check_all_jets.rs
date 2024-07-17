use std::collections::BTreeMap;
use std::sync::Mutex;

use simplicity::jet::Jet as _;
use simplicity::BitIter;

type Jet = simplicity::jet::Elements;

static CHECKED_JETS: Mutex<Option<BTreeMap<Jet, usize>>> = Mutex::new(None);
pub const N_TOTAL: usize = 469;

pub fn initialize() {
    // This is insanely inefficient but will find all jets with encodings <= 24 bits
    // and empirically it takes less than a second to run against the multi-hour
    // benchmark run, so whatever.
    //
    // In our generated Rust code we should add an integer<>jet map, a count of all
    // jets, an iterator over all jets, etc., but for now we will make do.
    let mut map = BTreeMap::new();
    for u0 in 0..=255 {
        for u1 in 0..=255 {
            for u2 in 0..=255 {
                let arr = [u0, u1, u2];
                let mut iter = BitIter::new(arr.iter().copied());
                if let Ok(jet) = Jet::decode(&mut iter) {
                    map.insert(jet, 0);
                }
            }
        }
    }

    *CHECKED_JETS.lock().unwrap() = Some(map);
}

pub fn record(j: Jet) {
    let mut lock = CHECKED_JETS.lock().unwrap();
    let map = lock.as_mut().expect("call check_all_jets::initialize");
    *map.get_mut(&j).expect("all jets to be preinitialized") += 1;
}

pub fn check_all_covered() {
    let lock = CHECKED_JETS.lock().unwrap();
    let map = lock.as_ref().expect("call check_all_jets::initialize");
    let actual_n = map.len();

    let mut missed_any = false;
    for (entry, count) in map {
        if *count == 0 {
            println!("Didn't cover jet: {entry}");
            missed_any = true;
        }
    }

    if missed_any {
        panic!("Failed to cover jets.");
    }

    if actual_n != N_TOTAL {
        println!(
            "Warning: covered {} jets but TOTAL_N is set to {}. You should update the constant in src/check_all_jets.rs",
            actual_n,
            N_TOTAL,
        );
    }
}
