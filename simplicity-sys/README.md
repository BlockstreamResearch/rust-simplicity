simplicity-sys
=============


This crate provides Rust definitions for the FFI structures and methods.


## Vendoring

The default build process is to build using the vendored libsecp256k1 sources in
the depend folder. There is no support for symbols yet.

To update the vendored sources, use the `vendor-simplicity.sh` script:

```
$ ./vendor-libsecp.sh depend <rev>
```

- Where `<rev>` is the git revision of libsecp256k1 to checkout.