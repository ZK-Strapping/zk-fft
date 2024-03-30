use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct CircuitInput {
    pub n: usize,
    pub ax: Vec<f32>,
    pub m: usize,
    pub bx: Vec<f32>,
}
pub type CircuitOutput = Vec<f32>;

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct CircuitJournal {
    pub input: CircuitInput,
    pub output: CircuitOutput,
}
