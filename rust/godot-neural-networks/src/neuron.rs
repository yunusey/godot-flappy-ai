use std::ops::{Index, IndexMut};

#[derive(Clone)]
pub struct Neuron {
    value: f64,
    bias: f64,
    input_weights: Vec<f64>,
}

impl Neuron {
    pub fn new(p_num_inputs: usize) -> Self {
        let random_weights: Vec<f64> = (0..p_num_inputs).map(|_| godot::global::randf_range(-1.0, 1.0)).collect();
        Neuron {
            value: 0.0,
            bias: godot::global::randf_range(-1.0, 1.0),
            input_weights: random_weights,
        }
    }

    pub fn sigmoid(x: f64) -> f64 {
        1.0 / (1.0 + (-x).exp())
    }

    pub fn feed_forward(&mut self, p_inputs: &Vec<f64>) {
        let dot_product: f64 = p_inputs.iter().zip(self.input_weights.iter()).map(|(x, y)| x * y).sum();
        self.value = Self::sigmoid(dot_product + self.bias);
    }

    pub fn get_num_inputs(&self) -> usize {
        self.input_weights.len()
    }

    pub fn get_output_value(&self) -> f64 {
        self.value
    }
}

impl Index<usize> for Neuron {
    type Output = f64;

    fn index(&self, index: usize) -> &Self::Output {
        &self.input_weights[index]
    }
}

impl IndexMut<usize> for Neuron {
    fn index_mut(&mut self, index: usize) -> &mut Self::Output {
        &mut self.input_weights[index]
    }
}
