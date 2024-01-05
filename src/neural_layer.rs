// neural_layer.rs
use crate::lif_neuron::SpikingNeuron;

#[derive(Clone)]
pub struct Layer<M: Model> {
    /// List of all neurons in this layer
    pub(crate) neurons: Vec<M::Neuron>,
    /// Matrix of the input weights. For the first layer, this must be a square diagonal matrix.
    pub(crate) input_weights: Vec<Vec<f64>>,
    /// Square matrix of the intra-layer weights
    pub(crate) intra_weights: Vec<Vec<f64>>
}

impl<M: Model> Layer<M> {
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
