// core/src/circuits/tests.rs
#[cfg(test)]
mod tests {
    use super::*;
    use halo2_proofs::{
        dev::MockProver,
        pasta::Fp,
        circuit::Value,
    };
    use std::time::Instant;
    
    #[test]
    fn test_constraint_count() {
        let circuit = PoRECircuit::<Fp>::new(
            vec![Value::known(Fp::from(1)); 10],
            vec![Fp::from(1); 3],
        );
        
        // Mock prover to count constraints
        let k = 8; // 2^8 rows
        let prover = MockProver::run(k, &circuit, vec![vec![Fp::from(1); 3]]).unwrap();
        
        // Verify constraint count is under limit
        // Note: Actual constraint counting would need circuit analysis
        assert!(circuit.constraint_count() < 25000, "Constraint count exceeds limit");
    }
    
    #[test]
    fn test_custom_gates() {
        let circuit = PoRECircuit::<Fp>::new(
            vec![
                Value::known(Fp::from(2)), // a
                Value::known(Fp::from(3)), // b
                Value::known(Fp::from(4)), // c
                Value::known(Fp::from(5)), // d
                Value::known(Fp::from(25)), // expected output: (2+3)*4+5 = 25
            ],
            vec![],
        );
        
        let k = 8;
        let prover = MockProver::run(k, &circuit, vec![]).unwrap();
        prover.assert_satisfied();
    }
    
    #[test]
    fn test_lookup_table_correctness() {
        // Test that values 0-255 are in lookup table
        for value in 0..256u64 {
            let circuit = PoRECircuit::<Fp>::new(
                vec![Value::known(Fp::from(value))],
                vec![],
            );
            
            let k = 8;
            let prover = MockProver::run(k, &circuit, vec![]).unwrap();
            prover.assert_satisfied();
        }
        
        // Test that 256 is NOT in lookup table (should fail)
        let circuit = PoRECircuit::<Fp>::new(
            vec![Value::known(Fp::from(256))],
            vec![],
        );
        
        let k = 8;
        let prover = MockProver::run(k, &circuit, vec![]);
        assert!(prover.is_err() || !prover.unwrap().verify().is_empty());
    }
    
    #[test]
    #[cfg(not(debug_assertions))] // Only run in release mode for accurate timing
    fn test_proving_time() {
        let circuit = PoRECircuit::<Fp>::new(
            vec![Value::known(Fp::from(1)); 10],
            vec![Fp::from(1); 3],
        );
        
        let k = 8;
        let start = Instant::now();
        
        // Run mock prover (actual proving would use real prover)
        let prover = MockProver::run(k, &circuit, vec![vec![Fp::from(1); 3]]).unwrap();
        prover.assert_satisfied();
        
        let elapsed = start.elapsed();
        
        // Target: <20ms on M2 (adjust based on your hardware)
        println!("Proving time: {:?}", elapsed);
        
        // Note: MockProver is faster than real proving
        // Real benchmark would use actual prover
        assert!(elapsed.as_millis() < 100, "Proving time exceeds target");
    }
    
    #[test]
    fn test_circuit_metrics() {
        use crate::circuits::helpers::CircuitMetrics;
        use halo2_proofs::plonk::ConstraintSystem;
        
        let mut cs = ConstraintSystem::<Fp>::default();
        let _config = PoRECircuit::<Fp>::configure(&mut cs);
        
        let metrics = CircuitMetrics::analyze(&cs);
        println!("{}", metrics.visualize());
        
        assert!(metrics.total_constraints < 25000);
        assert_eq!(metrics.advice_columns_used, 10);
    }
}