// synapse.rs

#[derive(Clone, Debug)]
pub struct Synapse {
    pub post_synaptic_layer_id: usize,
    pub post_synaptic_neuron_id: usize,
    pub weight: f64,
    // Altri attributi della sinapsi se necessario
}

impl Synapse {
    pub fn get_post_synaptic_layer_id(&self) -> usize {
        self.post_synaptic_layer_id
    }

    pub fn get_post_synaptic_neuron_id(&self) -> usize {
        self.post_synaptic_neuron_id
    }

    pub fn get_weight(&self) -> f64 {
        self.weight
    }
}