use crate::neural_network::NeuralNetwork;

#[derive(Clone)]
pub struct Agent {
    neural_network: NeuralNetwork,
    collective_score: i32,
    score: i32,
}

impl Agent {
    pub fn new(p_architecture: &Vec<usize>) -> Self {
        Self {
            neural_network: NeuralNetwork::new(p_architecture),
            collective_score: 0,
            score: 0,
        }
    }

    pub fn feed_forward(&mut self, p_inputs: Vec<f64>) -> Vec<f64> {
        self.neural_network.feed_forward(&p_inputs)
    }

    pub fn make_child(&self, p_other: &Agent, p_mutation_prob: f64) -> Agent {
        Agent {
            neural_network: self
                .neural_network
                .crossover(&p_other.neural_network, p_mutation_prob),
            collective_score: 0,
            score: 0,
        }
    }

    pub fn get_score(&self) -> i32 {
        self.score
    }

    pub fn set_score(&mut self, p_score: i32) {
        self.score = p_score;
    }

    pub fn get_collective_score(&self) -> i32 {
        self.collective_score
    }

    pub fn set_collective_score(&mut self, p_score: i32) {
        self.collective_score = p_score;
    }
}

impl Ord for Agent {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        (self.score, self.collective_score).cmp(&(other.score, other.collective_score))
    }
}

impl PartialOrd for Agent {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

impl PartialEq for Agent {
    fn eq(&self, other: &Self) -> bool {
        (self.score, self.collective_score) == (other.score, other.collective_score)
    }
}

impl Eq for Agent {}
