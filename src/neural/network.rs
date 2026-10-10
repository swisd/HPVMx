use crate::neural::helpers::WeightPairIdentifiable;
use crate::neural::layer::Layer;
use alloc::string::String;
use alloc::vec::Vec;

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct NetworkConfig {
    /// Number of input neurons.
    pub input_len: usize,
    /// Number of neurons in each hidden layer, in order.
    pub compute_lengths: Vec<usize>,
    /// Number of output neurons.
    pub output_len: usize,
}

impl NetworkConfig {
    /// Creates a network configuration from input, hidden-layer, and output sizes.
    pub fn new(input_len: usize, compute_lengths: &[usize], output_len: usize) -> Self {
        Self {
            input_len,
            compute_lengths: compute_lengths.into(),
            output_len,
        }
    }
}

#[derive(Clone)]
pub struct Network {
    input_len: usize,
    compute_layers: Vec<Layer>,
    output_layer: Layer,
}

impl Network {
    /// Creates a fully connected network with the supplied layer sizes.
    pub fn new(input_len: usize, compute_lengths: &[usize], output_len: usize) -> Self {
        Self::from_config(&NetworkConfig::new(input_len, compute_lengths, output_len))
    }

    /// Builds a fully connected network with clipped hidden activations and linear outputs.
    pub fn from_config(config: &NetworkConfig) -> Self {
        let mut previous_len = config.input_len;
        let compute_layers = config
            .compute_lengths
            .iter()
            .map(|&length| {
                let layer = Layer::new(length, previous_len, false);
                previous_len = length;
                layer
            })
            .collect();

        Self {
            input_len: config.input_len,
            compute_layers,
            output_layer: Layer::new(config.output_len, previous_len, true),
        }
    }

    /// Initializes all connections with Xavier-scaled values from a deterministic seed.
    pub fn initialize(&mut self, seed: &mut u32) {
        for layer in &mut self.compute_layers {
            layer.initialize(seed);
        }
        self.output_layer.initialize(seed);
    }

    /// Feeds one value per configured input and returns the final layer's outputs.
    pub fn feed_forward(
        &mut self,
        initial_input: Vec<WeightPairIdentifiable>,
    ) -> Vec<WeightPairIdentifiable> {
        assert_eq!(
            initial_input.len(),
            self.input_len,
            "network input length does not match its configuration"
        );

        let mut current_signals = initial_input;
        for layer in &mut self.compute_layers {
            layer.input(current_signals);
            current_signals = layer.output();
        }
        self.output_layer.input(current_signals);
        self.output_layer.output()
    }

    pub fn mutate(&mut self, rate: f32, amount: f32, seed: &mut u32) {
        for layer in &mut self.compute_layers {
            layer.mutate(rate, amount, seed);
        }
        self.output_layer.mutate(rate, amount, seed);
    }

    pub fn export_weights(&self) -> String {
        let mut state = String::from("NEURAL_NETWORK_V2\n");
        for layer in &self.compute_layers {
            state.push_str(&layer.export_state());
        }
        state.push_str(&self.output_layer.export_state());
        state
    }

    pub fn import_weights(&mut self, state: &str) -> bool {
        let mut candidate = self.clone();
        if !candidate.apply_weights(state) {
            return false;
        }
        *self = candidate;
        true
    }

    fn apply_weights(&mut self, state: &str) -> bool {
        let mut layers_data = state
            .strip_prefix("NEURAL_NETWORK_V2\n")
            .unwrap_or(state)
            .split("---\n");

        for layer in self
            .compute_layers
            .iter_mut()
            .chain(core::iter::once(&mut self.output_layer))
        {
            let Some(layer_state) = layers_data.next() else {
                return false;
            };
            let mut lines = layer_state.lines();
            for (neuron, _) in &mut layer.neurons {
                let Some(line) = lines.next() else {
                    return false;
                };
                if !neuron.import_state(line) {
                    return false;
                }
            }
            if lines.next().is_some() {
                return false;
            }
        }
        !layers_data.any(|extra_layer| !extra_layer.trim().is_empty())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use alloc::vec;

    #[test]
    fn each_input_channel_has_an_independent_connection() {
        let mut network = Network::new(3, &[3], 3);
        assert!(network.import_weights(
            "NEURAL_NETWORK_V2\n1,0,0,0\n0,1,0,0\n0,0,1,0\n---\n1,0,0,0\n0,1,0,0\n0,0,1,0\n---\n",
        ));
        let outputs = network.feed_forward(vec![((0.2, 1.0), 0), ((0.5, 1.0), 1), ((0.8, 1.0), 2)]);

        assert_eq!(outputs[0].0.0, 0.2);
        assert_eq!(outputs[1].0.0, 0.5);
        assert_eq!(outputs[2].0.0, 0.8);
    }

    #[test]
    fn output_layer_supports_negative_values() {
        let mut network = Network::new(1, &[], 1);
        assert!(network.import_weights("NEURAL_NETWORK_V2\n-1,0\n---\n"));
        let output = network.feed_forward(vec![((0.4, 1.0), 0)]);

        assert_eq!(output[0].0.0, -0.4);
    }

    #[test]
    fn initialized_networks_have_nonzero_weights() {
        let mut network = Network::new(3, &[4], 2);
        let mut seed = 42;
        network.initialize(&mut seed);
        let exported = network.export_weights();
        assert!(
            exported
                .lines()
                .filter(|line| *line != "---" && *line != "NEURAL_NETWORK_V2")
                .any(|line| line
                    .split(',')
                    .any(|value| value.parse::<f32>().unwrap() != 0.0))
        );
    }

    #[test]
    fn exported_weights_restore_network_outputs() {
        let mut original = Network::new(2, &[3], 1);
        let mut seed = 17;
        original.initialize(&mut seed);
        let saved = original.export_weights();

        let mut restored = Network::new(2, &[3], 1);
        assert!(restored.import_weights(&saved));
        let input = vec![((0.4, 1.0), 0), ((0.7, 1.0), 1)];
        let original_output = original.feed_forward(input.clone());
        let restored_output = restored.feed_forward(input);

        assert_eq!(original_output, restored_output);
    }

    #[test]
    fn incompatible_weights_leave_network_unchanged() {
        let mut network = Network::new(2, &[], 1);
        let mut seed = 9;
        network.initialize(&mut seed);
        let saved = network.export_weights();

        assert!(!network.import_weights("legacy weights"));
        assert_eq!(network.export_weights(), saved);
    }
}
