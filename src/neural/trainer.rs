use crate::neural::network::{Network, NetworkConfig};
use crate::neural::rgb_to_yc_test::rgb_to_ycocg;
use alloc::vec;
use alloc::vec::Vec;

/// Defines how a trainer creates samples and scores a network against them.
pub trait TrainingTask {
    type Sample;

    /// Generates one dataset for a generation using the trainer's deterministic seed.
    fn generate_dataset(&mut self, size: usize, rng_seed: &mut u32) -> Vec<Self::Sample>;

    /// Returns a score where lower values represent better networks.
    fn calculate_error(&self, network: &mut Network, dataset: &[Self::Sample]) -> f32;
}

/// Network topology and evolutionary hyperparameters.
#[derive(Clone, Debug, PartialEq)]
pub struct TrainerConfig {
    pub network: NetworkConfig,
    pub population_size: usize,
    pub elite_count: usize,
    pub mutation_rate: f32,
    pub mutation_amount: f32,
    pub initial_seed: u32,
}

impl TrainerConfig {
    /// Creates trainer settings for the selected network and evolutionary strategy.
    pub fn new(
        network: NetworkConfig,
        population_size: usize,
        mutation_rate: f32,
        mutation_amount: f32,
        initial_seed: u32,
    ) -> Self {
        Self {
            network,
            population_size,
            elite_count: 2,
            mutation_rate,
            mutation_amount,
            initial_seed,
        }
    }

    /// Sets how many top-scoring networks survive unchanged into the next generation.
    pub fn with_elite_count(mut self, elite_count: usize) -> Self {
        self.elite_count = elite_count;
        self
    }
}

pub struct EvolutionaryTrainer<T: TrainingTask> {
    population: Vec<Network>,
    config: TrainerConfig,
    task: T,
    rng_seed: u32,
    training_dataset: Option<Vec<T::Sample>>,
}

impl<T: TrainingTask> EvolutionaryTrainer<T> {
    /// Creates a population whose members all use the configured network topology.
    pub fn new(config: TrainerConfig, task: T) -> Self {
        assert!(
            config.population_size > 0,
            "population size must be nonzero"
        );
        let mut rng_seed = config.initial_seed;
        let population = (0..config.population_size)
            .map(|_| {
                let mut network = Network::from_config(&config.network);
                network.initialize(&mut rng_seed);
                network.mutate(config.mutation_rate, config.mutation_amount, &mut rng_seed);
                network
            })
            .collect();

        Self {
            population,
            config,
            task,
            rng_seed,
            training_dataset: None,
        }
    }

    /// Scores the population on a stable dataset, keeps the top networks, and mutates their offspring.
    ///
    /// `batch_size` is used the first time training starts. Call `refresh_dataset` to
    /// switch to a new batch while comparing all candidates on the same samples.
    pub fn train_generation(&mut self, batch_size: usize) -> f32 {
        assert!(batch_size > 0, "batch size must be nonzero");
        if self.training_dataset.is_none() {
            self.training_dataset =
                Some(self.task.generate_dataset(batch_size, &mut self.rng_seed));
            assert!(
                !self.training_dataset.as_ref().unwrap().is_empty(),
                "training task generated an empty dataset"
            );
        }

        let population_to_score = core::mem::take(&mut self.population);
        let dataset = self.training_dataset.as_deref().unwrap();

        let mut scored_population: Vec<(f32, Network)> = population_to_score
            .into_iter()
            .map(|mut network| {
                let error = self.task.calculate_error(&mut network, dataset);
                (error, network)
            })
            .collect();

        scored_population.sort_by(|a, b| a.0.total_cmp(&b.0));
        let best_error = scored_population[0].0;
        let elite_count = self
            .config
            .elite_count
            .max(1)
            .min(self.config.population_size);
        let elites: Vec<Network> = scored_population
            .into_iter()
            .take(elite_count)
            .map(|(_, network)| network)
            .collect();
        self.population.extend(elites.iter().cloned());

        while self.population.len() < self.config.population_size {
            self.rng_seed = self.rng_seed.wrapping_mul(1664525).wrapping_add(1013904223);
            let parent_idx = (self.rng_seed as usize) % elites.len();
            let mut clone = elites[parent_idx].clone();
            clone.mutate(
                self.config.mutation_rate,
                self.config.mutation_amount,
                &mut self.rng_seed,
            );
            self.population.push(clone);
        }

        best_error
    }

    pub fn get_champion(&self) -> &Network {
        &self.population[0]
    }

    /// Replaces the dataset used to compare candidates in subsequent generations.
    pub fn refresh_dataset(&mut self, batch_size: usize) {
        assert!(batch_size > 0, "batch size must be nonzero");
        let dataset = self.task.generate_dataset(batch_size, &mut self.rng_seed);
        assert!(
            !dataset.is_empty(),
            "training task generated an empty dataset"
        );
        self.training_dataset = Some(dataset);
    }
}

pub struct RgbToYcocgTask;

impl TrainingTask for RgbToYcocgTask {
    type Sample = (f32, f32, f32, f32, f32, f32);

    fn generate_dataset(&mut self, size: usize, rng_seed: &mut u32) -> Vec<Self::Sample> {
        let mut dataset = Vec::with_capacity(size);
        for _ in 0..size {
            *rng_seed = rng_seed.wrapping_mul(1664525).wrapping_add(1013904223);
            let r = (*rng_seed as f32) / (u32::MAX as f32);
            *rng_seed = rng_seed.wrapping_mul(1664525).wrapping_add(1013904223);
            let g = (*rng_seed as f32) / (u32::MAX as f32);
            *rng_seed = rng_seed.wrapping_mul(1664525).wrapping_add(1013904223);
            let b = (*rng_seed as f32) / (u32::MAX as f32);

            let (target_y, target_co, target_cg) = rgb_to_ycocg(r, g, b);
            dataset.push((r, g, b, target_y, target_co, target_cg));
        }
        dataset
    }

    fn calculate_error(&self, network: &mut Network, dataset: &[Self::Sample]) -> f32 {
        let mut total_error = 0.0;
        for &(r, g, b, target_y, target_co, target_cg) in dataset {
            let input = vec![((r, 1.0), 0), ((g, 1.0), 1), ((b, 1.0), 2)];
            let output = network.feed_forward(input);

            let err_y = target_y - output[0].0.0;
            let err_co = target_co - output[1].0.0;
            let err_cg = target_cg - output[2].0.0;
            total_error += (err_y * err_y) + (err_co * err_co) + (err_cg * err_cg);
        }

        total_error / (dataset.len() as f32 * 3.0)
    }
}

impl EvolutionaryTrainer<RgbToYcocgTask> {
    /// Creates an RGB-to-YCoCg trainer with caller-supplied topology and hyperparameters.
    pub fn for_rgb_to_ycocg(config: TrainerConfig) -> Self {
        Self::new(config, RgbToYcocgTask)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use alloc::vec;

    struct ConstantTask {
        dataset_generations: usize,
    }

    impl TrainingTask for ConstantTask {
        type Sample = ();

        fn generate_dataset(&mut self, size: usize, _rng_seed: &mut u32) -> Vec<Self::Sample> {
            self.dataset_generations += 1;
            vec![(); size]
        }

        fn calculate_error(&self, network: &mut Network, _dataset: &[Self::Sample]) -> f32 {
            network
                .feed_forward(vec![((0.5, 1.0), 0), ((0.25, 1.0), 1)])
                .len() as f32
        }
    }

    #[test]
    fn trainer_uses_configured_network_and_custom_task() {
        let config = TrainerConfig::new(NetworkConfig::new(2, &[3, 5], 4), 3, 1.0, 0.2, 7)
            .with_elite_count(2);
        let mut trainer = EvolutionaryTrainer::new(
            config,
            ConstantTask {
                dataset_generations: 0,
            },
        );
        assert_ne!(
            trainer.population[0].export_weights(),
            trainer.population[1].export_weights()
        );
        assert_eq!(trainer.train_generation(2), 4.0);
        assert_eq!(trainer.task.dataset_generations, 1);
        assert_eq!(trainer.train_generation(2), 4.0);
        assert_eq!(trainer.task.dataset_generations, 1);
        trainer.refresh_dataset(2);
        assert_eq!(trainer.task.dataset_generations, 2);
    }
}
