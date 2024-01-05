// neural_layer.rs
use crate::lif_neuron::SpikingNeuron;
use rand::{Rng, thread_rng};

fn generate_random_vector(n: usize, min: f64, max: f64) -> Vec<f64> {
    let mut rng = thread_rng();
    (0..n).map(|_| rng.gen_range(min..max)).collect()
}

#[derive(Clone)]
pub struct Layer<N: Neuron> {
    /// List of all neurons in this layer
    pub(crate) neurons: Vec<M::Neuron>,
    /// Matrix of the input weights. For the first layer, this must be a square diagonal matrix.
    pub(crate) input_weights: Vec<Vec<f64>>,
    /// Square matrix of the intra-layer weights
    pub(crate) intra_weights: Vec<Vec<f64>>
}

impl<N: Neuron> Layer<N> {

    pub fn new(layer_size: usize, neuron: N) -> Layer<N> {
        let neuron_vector = vec![neuron; layer_size];
        let input_vec = generate_random_vector(layer_size, 0, 10);
        let intra_vec = generate_random_vector(layer_size, -10, 0);

        Layer {
            neuron_vector,
            input_vec,
            intra_vec,
        }
    }
    

    pub fn num_neurons(&self) -> usize {
        self.neurons.len()
    }
    
    pub fn get_neuron(&self, neuron: usize) -> Option<&M::Neuron> {
        self.neurons.get(neuron)
    }

    fn get_matrix_value<T>(&self, x: usize, y: usize) -> Option<&T> {
        if let Some(row) = matrix.get(x) {
            row.get(y)
        } else {
            None
        }
    }
}
