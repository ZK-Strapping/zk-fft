use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct CircuitInput {
    pub n: usize,
    pub ax: Vec<f64>,
    pub m: usize,
    pub bx: Vec<f64>,
}
pub type CircuitOutput = Vec<f64>;

#[derive(Serialize, Deserialize)]
pub struct CircuitJournal {
    pub input: CircuitInput,
    pub output: CircuitOutput,
}
