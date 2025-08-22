//! Circuit implementations and helpers


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
