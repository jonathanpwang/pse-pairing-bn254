//! This module provides common utilities, traits and structures for group and
//! field arithmetic.
//!
//! This module is temporary, and the extension traits defined here are expected to be
//! upstreamed into the `ff` and `group` crates after some refactoring.

//mod curves;
mod fields;
//mod pairing;

//pub use curves::*;
pub use fields::*;
//pub use pairing::*;
