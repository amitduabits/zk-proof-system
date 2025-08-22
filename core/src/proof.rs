//! Proof generation and management

use crate::Result;

/// Proof structure
#[derive(Clone, Debug)]
pub struct Proof {
    /// Serialized proof data
    pub data: Vec<u8>,
}

impl Proof {
    /// Create a new proof
    pub fn new(data: Vec<u8>) -> Self {
        Self { data }
    }

    /// Serialize proof to bytes
    pub fn to_bytes(&self) -> &[u8] {
        &self.data
    }
}
