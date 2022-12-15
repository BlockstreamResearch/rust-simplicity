simplicity-sys
=============


This crate provides Rust definitions for the FFI structures and methods.


## Vendoring

The default build process is to build using the vendored simplicity sources in
the depend folder. There is no support for symbols yet.

To update the vendored sources, use the `vendor-simplicity.sh` script:

```
$ ./vendor-simplicity.sh depend <rev>
```

- Where `<rev>` is the git revision of ElementsProject/simplicity to checkout.