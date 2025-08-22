//! Traits for verification

/// Result of verification
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum VerifierResult {
    /// Proof is valid
    Valid,
    /// Proof is invalid
    Invalid,
    /// Verification error occurred
    Error,
}

/// Verifier trait
pub trait Verifier {
    /// Verify a proof
    fn verify(&self, proof: &[u8]) -> VerifierResult;
}
