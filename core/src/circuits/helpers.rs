// core/src/circuits/helpers.rs
use super::pore::PoREConfig;
use halo2_proofs::plonk::ConstraintSystem;
use halo2_proofs::arithmetic::Field;

/// Circuit metrics and analysis
pub struct CircuitMetrics {
    pub total_constraints: usize,
    pub advice_columns_used: usize,
    pub lookups_used: usize,
    pub custom_gates: usize,
    pub gate_utilization: f64,
}

impl CircuitMetrics {
    /// Analyze circuit configuration
    pub fn analyze<F: Field>(cs: &ConstraintSystem<F>) -> Self {
        // This would analyze the actual constraint system
        // For now, returning placeholder values
        Self {
            total_constraints: 0,
            advice_columns_used: 10,
            lookups_used: 1,
            custom_gates: 1,
            gate_utilization: 0.0,
        }
    }
    
    /// Generate visualization of circuit layout
    pub fn visualize(&self) -> String {
        format!(
            "Circuit Layout:\n\
             ================\n\
             Constraints: {}/{}\n\
             Advice Columns: {}/10\n\
             Lookups: {}\n\
             Custom Gates: {}\n\
             Gate Utilization: {:.2}%\n",
            self.total_constraints, 25000,
            self.advice_columns_used,
            self.lookups_used,
            self.custom_gates,
            self.gate_utilization * 100.0
        )
    }
}

/// Constraint counter for debugging
pub struct ConstraintCounter {
    count: usize,
    details: Vec<(String, usize)>,
}

impl ConstraintCounter {
    pub fn new() -> Self {
        Self {
            count: 0,
            details: Vec::new(),
        }
    }
    
    pub fn add(&mut self, gate_name: &str, constraints: usize) {
        self.count += constraints;
        self.details.push((gate_name.to_string(), constraints));
    }
    
    pub fn report(&self) {
        println!("=== Constraint Report ===");
        for (gate, count) in &self.details {
            println!("{}: {} constraints", gate, count);
        }
        println!("Total: {} constraints", self.count);
        
        if self.count > 25000 {
            println!("⚠️  WARNING: Exceeds 25k constraint target!");
        } else {
            println!("✓ Within constraint budget ({}/25000)", self.count);
        }
    }
}