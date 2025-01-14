use godot::prelude::*;
use crate::agent::Agent;

#[derive(GodotClass)]
#[class(no_init)]
pub struct Stadium {
    generation_index: usize,
    architecture: Vec<usize>,
    agents: Vec<Agent>,
}

#[godot_api]
impl Stadium {
    #[func]
    fn initialize_stadium(p_num_agents: i32, p_architecture: godot::builtin::PackedInt32Array) -> Gd<Self> {
        let architecture: Vec<usize> = p_architecture.to_vec().iter().map(|x| *x as usize).collect();
        let agents = (0..p_num_agents).map(|_| Agent::new(&architecture)).collect();
        Gd::from_object(Self {
            generation_index: 0,
            architecture,
            agents,
        })
    }

    #[func]
    pub fn feed_forward(&mut self, p_agent_id: i32, p_inputs: godot::builtin::PackedFloat64Array) -> godot::builtin::PackedFloat64Array {
        let inputs_vector = p_inputs.to_vec();
        let outputs = self.agents[p_agent_id as usize].feed_forward(inputs_vector);
        godot::builtin::PackedFloat64Array::from(outputs)
    }

    #[func]
    pub fn set_score(&mut self, p_agent_id: i32, p_score: i32) {
        let collective_score = self.agents[p_agent_id as usize].get_collective_score();
        self.agents[p_agent_id as usize].set_score(p_score);
        self.agents[p_agent_id as usize].set_collective_score(collective_score + p_score);
    }

    #[func]
    pub fn handle_new_generation(&mut self, p_survival_percentage: f64, p_mutation_prob: f64, p_exploration_rate: f64) {
        self.agents.sort_by(|a, b| b.cmp(&a)); // in descending order

        let num_survivors: usize = (self.agents.len() as f64 * p_survival_percentage) as usize;
        for i in num_survivors..self.agents.len() {
            // exploration
            if godot::global::randf() < p_exploration_rate {
                self.agents[i] = Agent::new(&self.architecture);
            }
            // mutation and crossover
            else {
                let parent1 = godot::global::randi_range(0, (num_survivors - 1) as i64) as usize;
                let parent2 = godot::global::randi_range(0, (num_survivors -1) as i64) as usize;
                self.agents[i] = self.agents[parent1].make_child(&self.agents[parent2], p_mutation_prob);
            }
        }

        self.agents.iter_mut().for_each(|agent| agent.set_score(0));
        self.generation_index += 1;
    }

    #[func]
    pub fn get_average_score(&self) -> f64 {
        self.agents.iter().map(|agent| agent.get_score() as f64).sum::<f64>() / self.agents.len() as f64
    }
}
