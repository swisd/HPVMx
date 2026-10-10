use crate::neural::helpers::WeightPair;
use alloc::format;
use alloc::string::String;
use alloc::vec;
use alloc::vec::Vec;

pub type NeuronIdentifiable = (Neuron, usize);

#[derive(Clone, Debug)]
pub struct Neuron {
    value: f32,
    weights: Vec<f32>,
    bias: f32,
}

impl Neuron {
    pub fn new(input_count: usize) -> Self {
        Self {
            value: 0.0,
            weights: vec![0.0; input_count],
            bias: 0.0,
        }
    }

    pub fn collect_ins(&mut self, inputs: &[WeightPair], linear: bool) {
        let weighted_sum = inputs
            .iter()
            .zip(&self.weights)
            .map(|(&(value, input_weight), &weight)| value * input_weight * weight)
            .sum::<f32>()
            + self.bias;
        self.value = if linear {
            weighted_sum
        } else {
            weighted_sum.clamp(-2.0, 2.0)
        };
    }

    #[inline]
    pub fn output(&self) -> WeightPair {
        (self.value, 1.0)
    }

    pub fn initialize(&mut self, limit: f32, seed: &mut u32) {
        for weight in &mut self.weights {
            *weight = next_signed(seed) * limit;
        }
        self.bias = 0.0;
    }

    #[inline]
    pub fn mutate(&mut self, rate: f32, amount: f32, seed: &mut u32) {
        for weight in &mut self.weights {
            if next_unit(seed) < rate {
                *weight += next_signed(seed) * amount;
            }
        }
        if next_unit(seed) < rate {
            self.bias += next_signed(seed) * amount;
        }
    }

    pub fn export_state(&self) -> String {
        let mut state = String::new();
        for weight in &self.weights {
            state.push_str(&format!("{weight},"));
        }
        state.push_str(&format!("{}\n", self.bias));
        state
    }

    pub fn import_state(&mut self, state: &str) -> bool {
        let values: Option<Vec<f32>> = state
            .split(',')
            .map(|part| part.parse::<f32>().ok())
            .collect();
        let Some(values) = values else {
            return false;
        };
        let weight_count = self.weights.len();
        if values.len() != weight_count + 1 {
            return false;
        }
        self.weights.copy_from_slice(&values[..weight_count]);
        self.bias = values[weight_count];
        true
    }
}

pub(crate) fn next_unit(seed: &mut u32) -> f32 {
    *seed = seed.wrapping_mul(1664525).wrapping_add(1013904223);
    (*seed as f32) / (u32::MAX as f32)
}

pub(crate) fn next_signed(seed: &mut u32) -> f32 {
    next_unit(seed) * 2.0 - 1.0
}
