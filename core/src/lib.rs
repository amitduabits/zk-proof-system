//! Core functionality for the ZK proof system
//!
//! This module provides the fundamental building blocks and abstractions
//! for zero-knowledge proof construction using Halo2.

#![warn(missing_docs)]
#![warn(clippy::all)]
#![warn(clippy::pedantic)]
#![allow(clippy::module_name_repetitions)]

pub mod circuit;
pub mod error;
pub mod proof;
pub mod utils;

pub use error::{Error, Result};

/// Re-export commonly used types from dependencies
pub mod prelude {
    pub use halo2_proofs::{
        arithmetic::Field,
        circuit::{Layouter, SimpleFloorPlanner, Value},
        plonk::{Circuit, ConstraintSystem, Error},
    };
    pub use pasta_curves::{pallas, vesta};
}
