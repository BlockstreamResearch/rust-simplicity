// SPDX-License-Identifier: CC0-1.0

//! Checks that every Bitcoin jet's `cmr()` and `cost()` agree with the values
//! in libsimplicity's vendored `primitiveJetNode.inc`.

use std::collections::HashMap;

use simplicity_sys::tests::BITCOIN_PRIMITIVE_JET_NODE_INC;

use super::Bitcoin;
use crate::analysis::Cost;
use crate::jet::Jet;
use crate::merkle::cmr::Cmr;

struct JetSpec {
    cmr: Cmr,
    cost: Cost,
}

/// Parses `primitiveJetNode.inc` into a map from each jet to its CMR and cost.
///
/// Every entry of the C table has the same shape:
///
/// ```text
/// ,[ADD_32] =
/// { .tag = JET
/// , .jet = rustsimplicity_<major>_<minor>_add_32
/// , .cmr = {{0x4668cd55u, ..., 0xd3cebf9fu}}
/// , .sourceIx = ty_w64
/// , .targetIx = ty_pbw32
/// , .cost = 92 /* milli weight units */
/// }
/// ```
///
/// `[` appears nowhere else in the file, so splitting on it yields one chunk per entry.
fn parse_inc(inc: &str) -> HashMap<Bitcoin, JetSpec> {
    let mut specs = HashMap::new();

    for entry in inc.split('[').skip(1) {
        let (label, _) = entry
            .split_once("] =")
            .unwrap_or_else(|| panic!("no `] =` in C entry: {entry}"));
        let jet: Bitcoin = label
            .to_ascii_lowercase()
            .parse()
            .unwrap_or_else(|_| panic!("unknown C jet [{label}]"));
        let cost = between(entry, ".cost = ", " ")
            .parse()
            .expect("cost is a u32");

        let spec = JetSpec {
            cmr: parse_cmr(between(entry, ".cmr = {{", "}}")),
            cost: Cost::from_milliweight(cost),
        };

        assert!(
            specs.insert(jet, spec).is_none(),
            "{jet}: duplicate C entry"
        );
    }

    specs
}

/// Returns the text between the first `start` in `entry` and the next `end`.
fn between<'a>(entry: &'a str, start: &str, end: &str) -> &'a str {
    entry
        .split_once(start)
        .and_then(|(_, rest)| rest.split_once(end))
        .map(|(found, _)| found)
        .unwrap_or_else(|| panic!("no `{start}`..`{end}` in C entry: {entry}"))
}

/// Parses `0x49425a86u, 0xe20a676du, ...` (eight 32-bit words, each serialized big-endian) into a CMR
///
/// Each word is eight hex digits, so the digits in order are the CMR's hex encoding.
fn parse_cmr(words: &str) -> Cmr {
    let hex: String = words
        .split(", ")
        .map(|word| {
            word.strip_prefix("0x")
                .and_then(|w| w.strip_suffix('u'))
                .filter(|digits| digits.len() == 8)
                .unwrap_or_else(|| panic!("{word}: CMR word is not 0x%08xu"))
        })
        .collect();

    hex.parse().expect("CMR is 32 bytes of hex")
}

#[test]
fn generated_tables_match_primitive_jet_node_inc() {
    let mut specs = parse_inc(BITCOIN_PRIMITIVE_JET_NODE_INC);

    assert_eq!(
        specs.len(),
        Bitcoin::ALL.len(),
        ".inc entries vs Bitcoin::ALL"
    );

    for jet in Bitcoin::ALL {
        let spec = specs
            .remove(&jet)
            .unwrap_or_else(|| panic!("{jet:?}: no .inc entry"));

        assert_eq!(jet.cmr(), spec.cmr, "{jet:?}: CMR differs from the .inc");
        assert_eq!(jet.cost(), spec.cost, "{jet:?}: cost differs from the .inc");
    }

    assert!(specs.is_empty(), "extra C entries: {:?}", specs.keys());
}
