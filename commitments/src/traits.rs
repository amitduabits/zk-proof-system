//! Traits for commitment schemes

/// Generic commitment trait
pub trait Commitment {
    /// The type of the commitment
    type Output;

    /// Create a commitment
    fn commit(&self) -> Self::Output;

    /// Verify a commitment
    fn verify(&self, commitment: &Self::Output) -> bool;
}

/// Commitment scheme trait
pub trait CommitmentScheme {
    /// The type of values being committed to
    type Value;

    /// The type of commitments
    type Commitment;

    /// The type of opening proofs
    type Opening;

    /// Commit to a value
    fn commit(&self, value: &Self::Value) -> Self::Commitment;

    /// Open a commitment
    fn open(&self, commitment: &Self::Commitment, value: &Self::Value) -> Self::Opening;

    /// Verify an opening
    fn verify(&self, commitment: &Self::Commitment, opening: &Self::Opening) -> bool;
}
