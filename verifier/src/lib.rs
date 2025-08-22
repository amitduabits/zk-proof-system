//! Verification module for the ZK proof system
//!
//! This module provides verification functionality for zero-knowledge proofs.

#![warn(missing_docs)]
#![warn(clippy::all)]
#![warn(clippy::pedantic)]

pub mod batch;
pub mod single;
pub mod traits;

pub use traits::{Verifier, VerifierResult};

/// Re-export commonly used types
pub mod prelude {
    pub use super::batch::BatchVerifier;
    pub use super::single::SingleVerifier;
    pub use super::traits::{Verifier, VerifierResult};
}
