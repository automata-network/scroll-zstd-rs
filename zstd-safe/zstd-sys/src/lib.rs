#![allow(non_upper_case_globals)]
#![allow(non_camel_case_types)]
#![allow(non_snake_case)]
#![cfg_attr(not(feature = "std"), no_std)]
//! Low-level bindings to the [zstd] library.
//!
//! [zstd]: https://facebook.github.io/zstd/

#[cfg(feature = "tstd")]
#[macro_use]
extern crate sgxlib as std;

#[cfg(target_arch = "wasm32")]
extern crate alloc;

#[cfg(target_arch = "wasm32")]
mod wasm_shim;

// If running bindgen, we'll end up with the correct bindings anyway.
#[cfg(feature = "bindgen")]
include!(concat!(env!("OUT_DIR"), "/bindings.rs"));

// The bindings used depend on a few feature flags.
#[cfg(all(not(feature = "experimental"), not(feature = "bindgen")))]
include!("bindings_zstd.rs");

#[cfg(all(
    not(feature = "experimental"),
    feature = "zdict_builder",
    not(feature = "bindgen")
))]
include!("bindings_zdict.rs");

#[cfg(all(feature = "experimental", not(feature = "bindgen")))]
include!("bindings_zstd_experimental.rs");

// patch
#[cfg(all(feature = "experimental", not(feature = "bindgen")))]
extern "C" {
    pub fn ZSTD_createCCtx_advanced_v2(
        customMem: ZSTD_customMem,
    ) -> *mut ZSTD_CCtx;
}

#[cfg(all(
    feature = "experimental",
    feature = "zdict_builder",
    not(feature = "bindgen")
))]
include!("bindings_zdict_experimental.rs");