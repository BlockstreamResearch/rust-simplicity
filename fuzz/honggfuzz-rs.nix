{
  nixpkgs ? import <nixpkgs> {}
}:
nixpkgs.rustPlatform.buildRustPackage rec {
  pname = "honggfuzz-rs";
  version = "0.5.55-git";

  src = fetchGit {
    url = "https://github.com/rust-fuzz/honggfuzz-rs";
    ref = "master";
  };
  cargoLock = {
    lockFile = "${src}/Cargo.lock";
  };
  # Pinned version because of breaking change in args to init_disassemble_info
  buildInputs = with nixpkgs; [
    libopcodes_2_38 # for dis-asm.h and bfd.h
    libunwind       # for libunwind-ptrace.h
  ];
}
