//! Commitment schemes for the ZK proof system
//!
//! This module implements various commitment schemes including
//! Pedersen commitments and polynomial commitments.

#![warn(missing_docs)]
#![warn(clippy::all)]
#![warn(clippy::pedantic)]

pub mod pedersen;
pub mod polynomial;
pub mod traits;

pub use traits::{Commitment, CommitmentScheme};

/// Re-export commonly used types
pub mod prelude {
    pub use super::pedersen::PedersenCommitment;
    pub use super::traits::{Commitment, CommitmentScheme};
}
