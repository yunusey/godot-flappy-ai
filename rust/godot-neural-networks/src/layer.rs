use std::ops::{Index, IndexMut};
use crate::neuron::Neuron;

#[derive(Clone)]
pub struct Layer {
    num_inputs: usize,
    neurons: Vec<Neuron>,
}

impl Layer {
    pub fn new(p_num_neurons: usize, p_num_inputs: usize) -> Self {
        let neurons: Vec<Neuron> = (0..p_num_neurons).map(|_| Neuron::new(p_num_inputs)).collect();

        Layer {
            num_inputs: p_num_inputs,
            neurons,
        }
    }

    pub fn feed_forward(&mut self, p_inputs: &Vec<f64>) -> Vec<f64> {
        self.neurons.iter_mut().map(|neuron| {
            neuron.feed_forward(p_inputs);
            neuron.get_output_value()
        }).collect()
    }

    pub fn get_num_inputs(&self) -> usize {
        self.num_inputs
    }

    pub fn get_num_neurons(&self) -> usize {
        self.neurons.len()
    }
}

impl Index<usize> for Layer {
    type Output = Neuron;

    fn index(&self, index: usize) -> &Self::Output {
        &self.neurons[index]
    }
}

impl IndexMut<usize> for Layer {
    fn index_mut(&mut self, index: usize) -> &mut Self::Output {
        &mut self.neurons[index]
    }
}
