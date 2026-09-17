# Charon demo

This repo contains four example uses of Charon,
as support for a presentation at the Rust-for-Linux conference:

- `demo1` runs Charon on the RfL `kernel` crate and looks at the translated layouts;
- `demo2` runs Charon on a small example and detects any calls to `std::thread::spawn`;
- `demo3` runs Charon to fetch some code from the standard library, then transpiles it to Python
  using a small hand-written printer;
- `demo4` translates the RfL `kernel` crate in the Rust 2021 and 2024 editions and reports any
  differences found in function bodies (requires a couple benign fixes to the kernel crate to work
  in edition 2024).

These demos assume that `charon` is in path and that the path contains everything needed for a basic
build of the Rust-for-Linux `kernel` crate.
If you use `nix`, the `flake.nix` can also provide all that environment for you.
