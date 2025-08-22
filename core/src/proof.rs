//! Proof generation and management


/// Proof structure
#[derive(Clone, Debug)]
pub struct Proof {
    /// Serialized proof data
    pub data: Vec<u8>,
}

impl Proof {
    /// Create a new proof
    #[must_use] pub fn new(data: Vec<u8>) -> Self {
        Self { data }
    }

    /// Serialize proof to bytes
    #[must_use] pub fn to_bytes(&self) -> &[u8] {
        &self.data
    }
}
