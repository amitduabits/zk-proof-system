//! Polynomial commitment schemes

/// Polynomial commitment structure
#[derive(Clone, Debug)]
pub struct PolynomialCommitment {
    /// Commitment data
    pub data: Vec<u8>,
}

impl PolynomialCommitment {
    /// Create a new polynomial commitment
    pub fn new(data: Vec<u8>) -> Self {
        Self { data }
    }
}
