//! Script to parse results from benchmarks folder and generate worst
//! case results for each jets
//!
//! We rely on criterion directory structure in 0.4.0

use std::{collections::BTreeMap, str::FromStr};

use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct MeasurementData {
    // The timestamp of when these measurements were saved.
    pub datetime: String,
    // The number of iterations in each sample
    pub iterations: Vec<f64>,
    // The measured values from each sample
    pub values: Vec<f64>,
    // The average values from each sample, ie. values / iterations
    pub avg_values: Vec<f64>,
    // The statistical estimates from this run
    pub estimates: Estimates,
}

#[derive(Debug, Deserialize)]
pub struct Estimates {
    pub mean: Estimate,
    pub median: Estimate,
    pub median_abs_dev: Estimate,
    pub slope: Option<Estimate>,
    pub std_dev: Estimate,
}

#[derive(Clone, PartialEq, Deserialize, Debug)]
pub struct Estimate {
    /// The confidence interval for this estimate
    pub confidence_interval: ConfidenceInterval,
    /// The point estimate for this estimate
    pub point_estimate: f64,
    /// The standard error of this estimate
    pub standard_error: f64,
}

#[derive(Clone, PartialEq, Deserialize, Debug)]
pub struct ConfidenceInterval {
    pub confidence_level: f64,
    pub lower_bound: f64,
    pub upper_bound: f64,
}

fn main() -> Result<(), String> {
    // Simplicity root is the grand-parent directory of this file
    let mut bench_results = BTreeMap::new();
    let simplicity_root = std::env::current_dir()
        .unwrap()
        .parent()
        .unwrap()
        .to_path_buf();
    // Obtain the path to the criterion directory
    let criterion_dir = simplicity_root
        .join("jets-bench")
        .join("target")
        .join("criterion")
        .join("data")
        .join("main");

    // enumerate all folders in criterion directory
    let jets_iter = match std::fs::read_dir(criterion_dir) {
        Ok(dir) => dir.map(|e| e.unwrap().path()),
        Err(e) => {
            return Err(format!(
                "Failed to open criterion data directory (did you run `cargo criterion`?): {}",
                e
            ))
        }
    };

    for jet in jets_iter {
        let mut worst_case = None;
        // Iterate over all files in jets directory
        for jet_folder in std::fs::read_dir(&jet).unwrap().map(|e| e.unwrap().path()) {
            // Parse the file
            // Filter all files that start with measurements
            let mut measurement_files = jet_folder
                .read_dir()
                .unwrap()
                .map(|e| e.unwrap().path())
                .filter(|e| {
                    e.file_name()
                        .unwrap()
                        .to_str()
                        .unwrap()
                        .starts_with("measurement_")
                })
                .collect::<Vec<_>>();
            // We expect only one file
            assert_eq!(measurement_files.len(), 1);
            let file = measurement_files.pop().unwrap();

            let file = std::fs::File::open(file).unwrap();
            let data: MeasurementData = serde_cbor::from_reader(file).unwrap();

            // Select the worst case amongst all measurements
            match worst_case {
                None => worst_case = Some(data.estimates.mean.point_estimate),
                Some(ref mut worst_case) => {
                    if *worst_case < data.estimates.mean.point_estimate {
                        *worst_case = data.estimates.mean.point_estimate;
                    }
                }
            }
            bench_results.insert(
                jet.file_name().unwrap().to_str().unwrap().to_string(),
                worst_case.unwrap(),
            );
        }
    }

    // Haskell requires debug formatted strings.
    let bench_results: BTreeMap<_, _> = bench_results.into_iter().map(|(k, v)| {
        let debug_str = simplicity::jet::Elements::from_str(&k).unwrap();
        (format!("{:?}", debug_str), v)
    }).collect();

    // Create a file output json file with jet_bench_timestamp.json
    let timestamp = chrono::Utc::now().format("%Y-%m-%d_%H-%M-%S");
    let output_file = simplicity_root
        .join("data")
        .join(format!("jet_bench_{}.json", timestamp));
    let mut output_file = std::fs::File::create(output_file).unwrap();
    serde_json::to_writer_pretty(&mut output_file, &bench_results).unwrap();

    Ok(())
}
