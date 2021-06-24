# sky-sl

🚧 work in progress 🚧

A rusty shading language experiment primarily for `sky-gfx`.

## scope

### goals

* VScode integration via Language Server Protocol, similar to rust-analyzer.
* Compiler as a library to integrate with asset systems.
* Typical compiler CLI.
* A workspace API to query shaders.
* Some sort of integration with regular rust code, to add some type safety between CPU and GPU data types.

### maybe-goals

* compatibility with [naga](https://github.com/gfx-rs/naga)

### non-goals

* Optimizing compiler (we'll rely on SPIR-V tools).
* Anything but SPIR-V as output

## license

sky-sl and its components is primarily distributed under the terms of both the MIT license and the Apache License (Version 2.0).

See [LICENSE-APACHE](LICENSE-APACHE) and [LICENSE-MIT](LICENSE-MIT) for details.
