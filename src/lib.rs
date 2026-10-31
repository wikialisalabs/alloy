//! # alloy
//!
//! **Meta-crate** — unified entry point for the Wikialisalabs EVM stack.
//!
//! Re-exports all sub-crates behind feature flags so you can pull in exactly
//! what you need with a single dependency.
//!
//! ## Feature Flags
//!
//! | Flag | Includes | Default |
//! |------|----------|---------|
//! | `core` | `alloy-core` (primitives, RLP, sol-types) | ✅ |
//! | `evm` | `alloy-evm` (interpreter + inspector) | ✅ |
//! | `eips` | `alloy-eips` (hardfork activation) | ✅ |
//! | `evm2` | `alloy-evm2` (Block-STM + ZK-IR) | ❌ |
//! | `trie` | `alloy-trie` (Merkle Patricia Trie) | ❌ |
//! | `op` | `op-alloy` (OP Stack types) | ❌ |
//! | `svm` | `svm-rs` (solc version manager) | ❌ |
//! | `full` | All of the above | ❌ |
//!
//! ## Quick Start
//!
//! ```toml
//! [dependencies]
//! alloy = { git = "https://github.com/wikialisalabs/alloy", features = ["full"] }
//! ```

#![warn(missing_docs)]

// Re-exports

#[cfg(feature = "core")]
pub use alloy_core as core;

#[cfg(feature = "evm")]
pub use alloy_evm as evm;

#[cfg(feature = "evm2")]
pub use alloy_evm2 as evm2;

#[cfg(feature = "trie")]
pub use alloy_trie as trie;

#[cfg(feature = "eips")]
pub use alloy_eips as eips;

#[cfg(feature = "op")]
pub use op_alloy as op;

#[cfg(feature = "svm")]
pub use svm_rs as svm;

pub mod middleware;
pub mod prelude;
