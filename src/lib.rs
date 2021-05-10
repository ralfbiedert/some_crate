#![no_std]
//! This crate promises (in a non-legal sense) to be as side-effect-free as crate-possible:
//!
//! - should compile with Rust 1.0.0,
//! - no dependencies,
//! - `#![no_std]`,
//! - doesn't set any app-global state,
//! - only a handful of functions exported.
//!
//! Some non-default feature flags might be added later to opt into additional behavior.
//!

/// Does absolutely nothing.
pub fn f() {}
