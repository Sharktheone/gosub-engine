//! Shared functionality
//!
//! This crate supplies a lot of shared functionality in the gosub engine.
//!
pub mod byte_stream;
pub mod timing;
pub mod types;
#[cfg(target_arch = "wasm32")]
pub mod worker;
