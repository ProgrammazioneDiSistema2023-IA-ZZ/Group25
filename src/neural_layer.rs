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

    /* pub fn new(input_weights: usize, next_size: usize, neuron: N) -> NeuralLayer<N> {
        let neurons = vec![neuron; layer_size];
        let input_weights: Vec<_> = (0..layer_size)
        .map(|_| generate_random_vector(layer_size, 0.0, 10.0))
        .collect();
        let mut intra_weights: Vec<_> = (0..layer_size)
        .map(|_| generate_random_vector(layer_size, -3.0, 0.0))
        .collect();
        for i in 0..layer_size { 
            intra_weights[i][i] = 0.0; 
        } 

        NeuralLayer {
            neurons,
            input_weights,
            intra_weights,
        }
    }
     */

     pub fn new(layer_size: usize, input_weights: Vec<Vec<f64>>, intra_weights: Vec<Vec<f64>>, neuron: N) -> NeuralLayer<N> {
        let neurons = vec![neuron; layer_size];

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

    pub fn get_neuron_mut(&mut self, index_neuron: usize) -> Option<&mut N> {
        self.neurons.get_mut(index_neuron)
    }

    pub fn get_neurons(&self) -> Vec<N>{
        self.neurons.clone()
    }

    pub fn get_neurons_mut(&mut self) -> Vec<N>{
        self.neurons.clone()
    }

    pub fn get_intra_weight_value<'a>(&'a self, from: usize, to: usize) -> Option<&f64> {
        if let Some(row) = self.intra_weights.get(from) {
            row.get(to)
        } else {
            None
        }
    }

    pub fn get_input_weight_value<'a>(&'a self, from: usize, to: usize) -> Option<&f64> {
        if let Some(row) = self.input_weights.get(from) {
            row.get(to)
        } else {
            None
        }
    }
}
