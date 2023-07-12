# Fuzzing

`simplicity` has a fuzzing harness setup for use with honggfuzz.

To run the fuzz-tests as in CI -- briefly fuzzing every target -- simply
run

    ./fuzz.sh

in this directory.

The script builds honggfuzz as part of its startup.

To build honggfuzz you must have libunwind on your system, as well as
libopcodes and libbfd from binutils **2.38** on your system. The most
recently-released binutils 2.39 has changed their API in a breaking way.

On Nix, you can use the provided nix shell

    nix-shell

Then run fuzz.sh as above.

# Long-term fuzzing

To see the full list of targets, the most straightforward way is to run

    source ./fuzz-util.sh
    listTargetNames

To run each target for an hour, run

    ./cycle.sh

This script uses the `chrt` utility to try to reduce the priority of the
jobs. If you would like to run for longer, the most straightforward way
is to edit `cycle.sh` before starting. To run the fuzz-tests in parallel,
you will need to implement a custom harness.

To manually run a single target indefinitely, run

    cargo hfuzz run <target>

You may need to install cargo-honggfuzz:

    cargo install honggfuzz --no-default-features

It is installed by the shell scripts and is available in the provided nix shell.

If you get linker errors, see the above information about libopcodes and libbfd.

On Nix, use the provided nix shell

    nix-shell

And then run `cargo hfuzz`.

# Adding fuzz tests

All fuzz tests can be found in the `fuzz_target/` directory. Adding a new
one is as simple as copying an existing one and editing the `do_test`
function to do what you want.

If your test clearly belongs to a specific crate, please put it in that
crate's directory. Otherwise you can put it directly in `fuzz_target/`.

If you need to add dependencies, edit the file `generate-files.sh` to add
it to the generated `Cargo.toml`.

Once you've added a fuzztest, regenerate the `Cargo.toml` and CI job by
running

    ./generate-files.sh

Then to test your fuzztest, run

    ./fuzz.sh <target>

If it is working, you will see a rapid stream of data for many seconds
(you can hit Ctrl+C to stop it early). If not, you should quickly see
an error.

# Reproducing Failures

If fuzz.sh or cycle.sh fail, they will exit with a summary which looks something like

```
...
 fuzzTarget      : hfuzz_target/x86_64-unknown-linux-gnu/release/decode_natural
CRASH:
DESCRIPTION:
ORIG_FNAME: 00000000000000000000000000000000.00000000.honggfuzz.cov
FUZZ_FNAME: hfuzz_workspace/decode_natural/SIGABRT.PC.7ffff7e42adc.STACK.1a1d230819.CODE.-6.ADDR.0.INSTR.mov____%eax,%ebx.fuzz
...
=====================================================================
fff400610004
```

If you run `cargo hfuzz` manually, run the following to see the report

    source ./fuzz-util.sh
    getReport <target>

The final line is a hex-encoded version of the input that caused the crash. You
can test this directly by editing the `duplicate_crash` test to copy/paste the
hex output into the call to `extend_vec_from_hex`. Then run the test with

    cargo test

Note that if you set your `RUSTFLAGS` while fuzzing (see above) you must make
sure they are set the same way when running `cargo test`.
