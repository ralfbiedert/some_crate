#![no_std]
//! This crate promises (in a non-legal sense) to be as side-effect-free as crate-possible:
//!
//! - should compile with Rust 1.0.0,
//! - no dependencies,
//! - `#![no_std]`,
//! - only a handful of functions exported.
//!
//! Some non-default feature flags might be added later to opt into additional behavior.
//!

use core::sync::atomic::{AtomicUsize};

/// Does absolutely nothing.
pub fn f() {}

/// Also does nothing.
pub fn g() {}

/// A global static counter.
pub static GLOBAL_COUNTER: AtomicUsize = AtomicUsize::new(0);