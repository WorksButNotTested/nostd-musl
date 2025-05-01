//! # nostd-musl
//!
//! A rust library containing  which `XXXX-unknown-linux-gnu` toolchains expect to
//! find in `libc`, so that they can be linked without dependency on `libc` using:
//! * `-nodefaultlibs`
//! * `-nostartfiles`
//! * `-nostdlib`.
#![no_std]
#![allow(non_camel_case_types)]
include!(concat!(env!("OUT_DIR"), "/bindings.rs"));
