//! # Rust Examples
//!
//! This library provides some functions created in studying Rust.

// To use macros in a crate, #[macro_use] must be called in the root file of the crate.
#[macro_use(s)]
extern crate ndarray;

pub mod ndarray_test;
