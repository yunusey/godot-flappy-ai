use crate::layer::Layer;

#[derive(Clone)]
pub struct NeuralNetwork {
    layers: Vec<Layer>,
}

impl NeuralNetwork {
    pub fn new(p_architecture: &Vec<usize>) -> Self {
        let mut layers: Vec<Layer> = Vec::new();
        for i in 0..p_architecture.len() - 1 {
            let layer = Layer::new(p_architecture[i + 1], p_architecture[i]);
            layers.push(layer);
        }

        NeuralNetwork { layers }
    }

    pub fn feed_forward(&mut self, p_inputs: &Vec<f64>) -> Vec<f64> {
        assert!(p_inputs.len() == self.layers[0].get_num_inputs());

        let mut new_inputs = p_inputs.clone();
        for i in 0..self.layers.len() {
            new_inputs = self.layers[i].feed_forward(&new_inputs);
        }

        return new_inputs;
    }

    /// Crossover method
    ///
    /// We will either take the gene from either parents with a 50% chance
    /// or mutate the gene with a new random value with a chance of `p_mutation_prob`.
    pub fn crossover(&self, p_other: &NeuralNetwork, p_mutation_prob: f64) -> NeuralNetwork {
        let mut neural_network = self.clone();
        for i in 0..neural_network.layers.len() {
            for j in 0..neural_network.layers[i].get_num_neurons() {
                for k in 0..neural_network.layers[i][j].get_num_inputs() {
                    let prob = godot::global::randf();
                    if prob < 0.5 {
                        neural_network.layers[i][j][k] = p_other.layers[i][j][k];
                    }
                    else {
                        neural_network.layers[i][j][k] = self.layers[i][j][k];
                    }

                    let prob = godot::global::randf();
                    if prob < p_mutation_prob {
                        neural_network.layers[i][j][k] = godot::global::randf_range(-1.0, 1.0);
                    }
                }
            }
        }

        return neural_network;
    }
}
