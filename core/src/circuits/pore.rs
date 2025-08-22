// core/src/circuits/pore.rs
use halo2_proofs::{
    arithmetic::Field,
    circuit::{AssignedCell, Layouter, SimpleFloorPlanner, Value},
    plonk::{
        Advice, Circuit, Column, ConstraintSystem, Error, Expression, Fixed, 
        Instance, Selector, TableColumn
    },
    poly::Rotation,
};
use std::marker::PhantomData;

/// Configuration for the PoRE circuit
#[derive(Debug, Clone)]
pub struct PoREConfig {
    /// Advice columns for witness values
    pub advice: [Column<Advice>; 10],
    /// Instance columns for public inputs
    pub instance: [Column<Instance>; 3],
    /// Fixed column for constants
    pub fixed: Column<Fixed>,
    /// Selector for addition/multiplication fusion gate
    pub s_add_mul: Selector,
    /// Selector for range check
    pub s_range: Selector,
    /// Table column for lookup arguments
    pub table: TableColumn,
    /// Constraint counter
    pub constraint_count: std::cell::RefCell<usize>,
}

impl PoREConfig {
    /// Create custom gate for fused addition and multiplication
    /// Computes: out = (a + b) * c + d
    /// This reduces constraint count by combining operations
    fn configure_add_mul_gate(&self, cs: &mut ConstraintSystem<impl Field>) {
        cs.create_gate("add_mul fusion", |meta| {
            let s = meta.query_selector(self.s_add_mul);
            
            let a = meta.query_advice(self.advice[0], Rotation::cur());
            let b = meta.query_advice(self.advice[1], Rotation::cur());
            let c = meta.query_advice(self.advice[2], Rotation::cur());
            let d = meta.query_advice(self.advice[3], Rotation::cur());
            let out = meta.query_advice(self.advice[4], Rotation::cur());
            
            // Constraint: out = (a + b) * c + d
            vec![s * (out - ((a + b) * c + d))]
        });
        
        *self.constraint_count.borrow_mut() += 1;
    }
    
    /// Configure 8-bit range check lookup table
    fn configure_range_table(&self, cs: &mut ConstraintSystem<impl Field>) {
        cs.lookup("8-bit range", |meta| {
            let value = meta.query_advice(self.advice[0], Rotation::cur());
            let s_range = meta.query_selector(self.s_range);
            
            vec![(s_range * value, self.table)]
        });
        
        *self.constraint_count.borrow_mut() += 1;
    }
}

/// Main PoRE Circuit implementation
#[derive(Default)]
pub struct PoRECircuit<F: Field> {
    /// Private witness values
    pub witnesses: Vec<Value<F>>,
    /// Public inputs
    pub public_inputs: Vec<F>,
    _marker: PhantomData<F>,
}

impl<F: Field> PoRECircuit<F> {
    /// Create a new PoRE circuit
    pub fn new(witnesses: Vec<Value<F>>, public_inputs: Vec<F>) -> Self {
        Self {
            witnesses,
            public_inputs,
            _marker: PhantomData,
        }
    }
    
    /// Get constraint count for the circuit
    pub fn constraint_count(&self) -> usize {
        // This will be updated during synthesis
        0
    }
}

impl<F: Field> Circuit<F> for PoRECircuit<F> {
    type Config = PoREConfig;
    type FloorPlanner = SimpleFloorPlanner;
    
    fn without_witnesses(&self) -> Self {
        Self::default()
    }
    
    fn configure(cs: &mut ConstraintSystem<F>) -> Self::Config {
        let advice = [(); 10].map(|_| cs.advice_column());
        let instance = [(); 3].map(|_| cs.instance_column());
        let fixed = cs.fixed_column();
        
        // Enable equality for copy constraints
        for column in &advice {
            cs.enable_equality(*column);
        }
        for column in &instance {
            cs.enable_equality(*column);
        }
        cs.enable_equality(fixed);
        
        let s_add_mul = cs.selector();
        let s_range = cs.selector();
        let table = cs.lookup_table_column();
        
        let config = PoREConfig {
            advice,
            instance,
            fixed,
            s_add_mul,
            s_range,
            table,
            constraint_count: std::cell::RefCell::new(0),
        };
        
        // Configure custom gates
        config.configure_add_mul_gate(cs);
        config.configure_range_table(cs);
        
        config
    }
    
    fn synthesize(
        &self,
        config: Self::Config,
        mut layouter: impl Layouter<F>,
    ) -> Result<(), Error> {
        // Load lookup table for 8-bit values
        layouter.assign_table(
            || "8-bit range table",
            |mut table| {
                for value in 0..256 {
                    table.assign_cell(
                        || format!("value {}", value),
                        config.table,
                        value,
                        || Value::known(F::from(value as u64)),
                    )?;
                }
                Ok(())
            },
        )?;
        
        // Example synthesis - replace with actual PoRE logic
        layouter.assign_region(
            || "main region",
            |mut region| {
                // Track constraint usage
                let mut constraint_counter = 0;
                
                // Example: Use add_mul gate
                config.s_add_mul.enable(&mut region, 0)?;
                constraint_counter += 1;
                
                // Assign witness values
                for (i, witness) in self.witnesses.iter().enumerate() {
                    if i < 10 {
                        region.assign_advice(
                            || format!("witness {}", i),
                            config.advice[i],
                            0,
                            || *witness,
                        )?;
                    }
                }
                
                // Update global constraint count
                *config.constraint_count.borrow_mut() = constraint_counter;
                
                Ok(())
            },
        )?;
        
        // Copy public inputs to instance columns
        for (i, public_input) in self.public_inputs.iter().enumerate() {
            if i < 3 {
                layouter.constrain_instance(
                    config.advice[i].into(),
                    config.instance[i],
                    0,
                )?;
            }
        }
        
        // Report constraint count
        let total_constraints = *config.constraint_count.borrow();
        if total_constraints > 25000 {
            eprintln!("WARNING: Constraint count {} exceeds target of 25,000", total_constraints);
        } else {
            eprintln!("Constraint count: {} / 25,000", total_constraints);
        }
        
        Ok(())
    }
}