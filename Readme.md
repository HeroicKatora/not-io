# not-io

Not the `std::io` module, but a very close compatibility interface for
no-std/no-alloc environments. This dissolves into a shim-layer when std
features are enabled, with a trick to _guarantee_ compatibility.

This crate aims to stay as close to `std` and `alloc` as possible. No
extensions are planned and the traits will strictly _tail_ stabilized
interface. No exceptions.

## Comparison to similar crates

There are several alternatives. Why should you choose this one? Simply put, no
other crate I found is *stable*, *compatible*, and well behaved for feature
combinations. Most either target _purely_ no-std or are extensions. All other
crates get the SemVer/feature compatibility wrong, that is they permit adding
incompatible impls in some configuration.

- [`genio`](https://docs.rs/genio), extension.
- [`acid_io`](https://docs.rs/acid_io), extension.
- [`bare-io`](https://docs.rs/bare-io), extension,
- [`core_io`](https://crates.io/crates/core_io), nightly
- [`ciborium-io`](https://docs.rs/ciborium-io), feature incompatible, extension
- [`zorio`](https://crates.io/crates/zorio), no std compatibility
