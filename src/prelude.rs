//! Convenience re-exports for the most commonly used types.
//!
//! ```rust
//! use alloy::prelude::*;
//! ```

#[cfg(feature = "core")]
pub use alloy_core::primitives::{Address, B256, U256};

#[cfg(feature = "core")]
pub use alloy_core::rlp::{Decodable, Encodable};

#[cfg(feature = "evm")]
pub use alloy_evm::interpreter::{run_simple, ExitReason};

#[cfg(feature = "evm")]
pub use alloy_evm::context::EvmContext;

#[cfg(feature = "eips")]
pub use alloy_eips::hardfork::{Hardfork, HardforkConfig};

pub use crate::middleware::{EvmRequest, EvmResponse, Middleware};
