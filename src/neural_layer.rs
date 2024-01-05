// neural_layer.rs
use crate::lif_neuron::Neuron;
use rand::{Rng, thread_rng};

fn generate_random_vector(n: usize, min: f64, max: f64) -> Vec<f64> {
    let mut rng = thread_rng();
    (0..n).map(|_| rng.gen_range(min..max)).collect()
}

#[derive(Clone)]
pub struct NeuralLayer<N: Neuron> {
    /// List of all neurons in this layer
    pub(crate) neurons: Vec<N>,
    /// Matrix of the input weights. For the first layer, this must be a square diagonal matrix.
    pub(crate) input_weights: Vec<Vec<f64>>,
    /// Square matrix of the intra-layer weights
    pub(crate) intra_weights: Vec<Vec<f64>>
}

impl<N: Neuron> NeuralLayer<N> {

    pub fn new(layer_size: usize, next_size: usize, neuron: N) -> NeuralLayer<N> {
        let neurons = vec![neuron; layer_size];
        let input_weights: Vec<_> = (0..layer_size)
        .map(|_| generate_random_vector(next_size, 0.0, 10.0))
        .collect();
    let intra_weights: Vec<_> = (0..layer_size)
        .map(|_| generate_random_vector(layer_size, -10.0, 0.0))
        .collect();

        NeuralLayer {
            neurons,
            input_weights,
            intra_weights,
        }
    }
    

    pub fn num_neurons(&self) -> usize {
        self.neurons.len()
    }
    
    pub fn get_neuron(&self, neuron: usize) -> Option<&N> {
        self.neurons.get(neuron)
    }

    fn get_matrix_value<'a>(&'a self, matrix: &'a Vec<Vec<f64>>, x: usize, y: usize) -> Option<&f64> {
        if let Some(row) = matrix.get(x) {
            row.get(y)
        } else {
            None
        }
    }
}
