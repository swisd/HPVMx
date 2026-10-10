use crate::neural::helpers::{WeightPair, WeightPairIdentifiable};
use crate::neural::neuron::{Neuron, NeuronIdentifiable};
use alloc::string::String;
use alloc::vec::Vec;
use crate::localmodules::math::sqrt;

#[derive(Clone)]
pub struct Layer {
    pub(crate) neurons: Vec<NeuronIdentifiable>,
    input_count: usize,
    linear: bool,
}

impl Layer {
    pub fn new(length: usize, input_count: usize, linear: bool) -> Self {
        let neurons = (0..length)
            .map(|index| (Neuron::new(input_count), index))
            .collect();
        Self {
            neurons,
            input_count,
            linear,
        }
    }

    pub fn output(&self) -> Vec<WeightPairIdentifiable> {
        self.neurons
            .iter()
            .map(|(neuron, id)| (neuron.output(), *id))
            .collect()
    }

    pub fn input(&mut self, input: Vec<WeightPairIdentifiable>) {
        let values: Vec<WeightPair> = input.into_iter().map(|(pair, _)| pair).collect();
        for (neuron, _) in &mut self.neurons {
            neuron.collect_ins(&values, self.linear);
        }
    }

    pub fn initialize(&mut self, seed: &mut u32) {
        let fan_sum = (self.input_count + self.neurons.len()).max(1) as f32;
        let limit = sqrt((6.0 / fan_sum) as f64);
        for (neuron, _) in &mut self.neurons {
            neuron.initialize(limit as f32, seed);
        }
    }

    pub fn mutate(&mut self, rate: f32, amount: f32, seed: &mut u32) {
        for (neuron, _) in &mut self.neurons {
            neuron.mutate(rate, amount, seed);
        }
    }

    pub fn export_state(&self) -> String {
        let mut state = String::new();
        for (neuron, _) in &self.neurons {
            state.push_str(&neuron.export_state());
        }
        state.push_str("---\n");
        state
    }
}
