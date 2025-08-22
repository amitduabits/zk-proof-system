//! Pedersen commitment implementation

use ff::Field;
use group::Group;

/// Pedersen commitment structure
#[derive(Clone, Debug)]
pub struct PedersenCommitment<G: Group> {
    /// Commitment value
    pub value: G,
}

impl<G: Group> PedersenCommitment<G> {
    /// Create a new Pedersen commitment
    pub fn new(value: G) -> Self {
        Self { value }
    }
}
