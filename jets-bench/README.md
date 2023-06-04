# Instructions for running benchmarks


- Install `cargo-criterion` with `cargo install cargo-criterion`

The following benchmarks are run with `cargo-criterion`: `1.1.0`
and criterion: `0.4.0`. `cargo-criterion` is the recommended way to run benchmarks. This generates some data in the `target/criterion` directory.

Then you can run `cargo run` from `scripts/jets_bench` to parse that data and generate a json file that is stored in `data` directory.