pub mod helpers;
mod layer;
mod network;
mod neuron;
mod rgb_to_yc_test;
mod trainer;

pub use network::{Network, NetworkConfig};
pub use trainer::{EvolutionaryTrainer, RgbToYcocgTask, TrainerConfig, TrainingTask};
