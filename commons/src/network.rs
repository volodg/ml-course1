use crate::math::lerp::lerp;
use rand::Rng;
use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize, Clone)]
pub struct NeuralNetwork {
    pub levels: Vec<Level>,
}

impl NeuralNetwork {
    pub fn create(neuron_counts: &[usize]) -> Self {
        let levels = neuron_counts
            .iter()
            .take(neuron_counts.len() - 1)
            .zip(neuron_counts.iter().skip(1))
            .map(|(count, next_count)| Level::create(*count, *next_count))
            .collect();

        Self { levels }
    }

    pub fn feed_forward(&mut self, given_inputs: Vec<f64>) -> Vec<f64> {
        self.levels
            .iter_mut()
            .fold(given_inputs, |acc, el| el.feed_forward(acc).to_vec())
    }

    // amount from 0 to 1 (aka percent)
    pub fn mutate(&mut self, amount: f64) {
        self.levels
            .iter_mut()
            .for_each(|level| level.mutate(amount))
    }
}

#[derive(Deserialize, Serialize, Clone)]
pub struct Level {
    // need for debugging
    pub inputs: Vec<f64>,
    // need for debugging
    pub outputs: Vec<f64>,
    pub biases: Vec<f64>,
    pub weights: Vec<Vec<f64>>,
}

impl Level {
    fn create(inputs_count: usize, outputs_count: usize) -> Self {
        let inputs = vec![0.0; inputs_count];
        let outputs = vec![0.0; outputs_count];
        let biases = vec![0.0; outputs_count];
        let weights = vec![vec![0.0; outputs_count]; inputs_count];

        let mut result = Self {
            inputs,
            outputs,
            biases,
            weights,
        };

        result.randomize();

        result
    }

    fn feed_forward(&mut self, given_inputs: Vec<f64>) -> &[f64] {
        self.inputs = given_inputs;

        self.outputs.iter_mut().zip(0..).for_each(|(value, index)| {
            let level = self
                .inputs
                .iter()
                .zip(&self.weights)
                .fold(0.0, |acc, (input, weights)| acc + input * weights[index]);

            *value = if level > self.biases[index] { 1.0 } else { 0.0 };
        });

        &self.outputs
    }

    fn randomize(&mut self) {
        let mut rng = rand::thread_rng();
        self.weights.iter_mut().for_each(|arr| {
            arr.iter_mut().for_each(|el| {
                *el = rng.gen_range(-1.0..=1.0);
            })
        });

        self.biases.iter_mut().for_each(|el| {
            *el = rng.gen_range(-1.0..=1.0);
        })
    }

    // amount from 0 to 1 (aka percent)
    pub fn mutate(&mut self, amount: f64) {
        let mut rng = rand::thread_rng();
        self.biases
            .iter_mut()
            .for_each(|bias| *bias = lerp(*bias, rng.gen_range(-1.0..=1.0), amount));

        self.weights.iter_mut().for_each(|arr| {
            arr.iter_mut()
                .for_each(|weight| *weight = lerp(*weight, rng.gen_range(-1.0..=1.0), amount))
        });
    }
}
