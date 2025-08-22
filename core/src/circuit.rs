//! Circuit implementations and helpers

use halo2_proofs::{
    circuit::{Layouter, SimpleFloorPlanner},
    plonk::{Circuit, ConstraintSystem, Error},
};

/// Example circuit structure
#[derive(Clone, Debug)]
pub struct ExampleCircuit<F> {
    _marker: std::marker::PhantomData<F>,
}

impl<F> Default for ExampleCircuit<F> {
    fn default() -> Self {
        Self {
            _marker: std::marker::PhantomData,
        }
    }
}
