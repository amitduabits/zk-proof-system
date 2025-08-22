// core/src/circuits/mod.rs
pub mod pore;
pub mod dci;
pub mod helpers;

#[cfg(test)]
mod tests;

pub use pore::{PoRECircuit, PoREConfig};
pub use dci::{DCICircuit, DCIConfig, PoseidonChip};
pub use helpers::{CircuitMetrics, ConstraintCounter};