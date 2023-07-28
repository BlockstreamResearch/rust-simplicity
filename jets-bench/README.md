# Instructions for running benchmarks

- Install `cargo-criterion` with `cargo install cargo-criterion`

The following benchmarks are run with `cargo-criterion`: `1.1.0`
and criterion: `0.4.0`.

To run the benchmarks:

1. Install `cargo-criterion` with `cargo install cargo-criterion`
2. Run `cargo criterion --plotting-backend disabled` in the `jets-bench` directory. This will output data to the `target/criterion` directory.
3. Run `cargo run` to parse that data and output a JSON file into the `data/` directory.

