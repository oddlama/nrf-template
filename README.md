[![built with nix](https://builtwithnix.org/badge.svg)](https://builtwithnix.org)

## ❄️ 🦀 nRF52840 Embedded Rust Template

This is a template for embedded rust development for the nRF52840.
It uses the [embassy](https://github.com/embassy-rs/embassy) embedded framework and [Nix](https://nixos.org) to provide reproducible builds and a full development environment.

To build the project:

```bash
$ nix build
# The compiled package resides in ./result
```

## Developing

You can use `nix develop` to enter a development shell that has all the necessary
tools setup so you can incrementally build and test your project:

```bash
# Enter development shell
$ nix develop

# Run cargo as usual
$ cargo build
$ cargo run
```
