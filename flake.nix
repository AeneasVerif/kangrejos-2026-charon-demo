{
  inputs = {
    charon.url = "github:AeneasVerif/charon/b2a7a167215b84bd29a230cda07064f02b867441";
    flake-utils.follows = "charon/flake-utils";
    nixpkgs.follows = "charon/nixpkgs";
  };

  outputs = { charon, flake-utils, nixpkgs, ... }:
    flake-utils.lib.eachSystem [ "x86_64-linux" ] (system:
      let
        pkgs = import nixpkgs { inherit system; };
        rustToolchain = charon.packages.${system}.rustToolchain;
        llvm = pkgs.llvmPackages;
      in
      {
        devShells.default = pkgs.mkShell {
          packages = [
            rustToolchain
            llvm.clang-unwrapped
            llvm.lld
            llvm.llvm
            pkgs.bc
            pkgs.bison
            pkgs.cpio
            pkgs.elfutils
            pkgs.flex
            pkgs.gawk
            pkgs.git
            pkgs.gnumake
            pkgs.kmod
            pkgs.ncurses
            pkgs.openssl
            pkgs.pahole
            pkgs.perl
            pkgs.pkg-config
            pkgs.python3
            pkgs.rust-bindgen-unwrapped
            pkgs.zlib
          ];

          CHARON_TOOLCHAIN_IS_IN_PATH = 1;
          LIBCLANG_PATH = "${llvm.libclang.lib}/lib";
          LD_LIBRARY_PATH = pkgs.lib.makeLibraryPath [
            rustToolchain
            pkgs.stdenv.cc.cc.lib
            pkgs.openssl
            pkgs.zlib
          ];
          RUST_LIB_SRC = "${rustToolchain}/lib/rustlib/src/rust/library";
        };
      });
}
