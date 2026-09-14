# Charon demo

This repo contains three example uses of Charon,
as support for a presentation at the Rust-for-Linux conference:

- `demo1` runs Charon on the RfL `kernel` crate and looks at the translated layouts;
- `demo2` runs Charon on a small example and detects any calls to `std::thread::spawn`;
- `demo3` runs Charon to fetch some code from the standard library, then transpiles it to Python
  using a small hand-written printer.
