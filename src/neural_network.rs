// neural_network.rs

mod lif_neuron; // Importa il modulo neuron
use lif_neuron::Neuron; 

use crate::neural_layer::NeuralLayer;

#[derive(Clone)]
pub struct NeuralNetwork<N: Neuron> {
    /// All the sorted layers of the neural network
    layers: Vec<NeuralLayer<N>>
}

impl<N: Neuron> NeuralNetwork<N> {

    pub fn new(layer_sizes: Vec<usize>, neuron: N) -> NeuralNetwork<N> {
        let mut layers = Vec::with_capacity(layer_sizes.len());

        for &size in &layer_sizes {
            let neural_layer = NeuralLayer::new(size, neuron.clone());
            layers.push(neural_layer);
        }

        NeuralNetwork { layers }
    }


    /// Get a reference to a specific layer by index
    pub fn get_layer(&self, index: usize) -> Option<&NeuralLayer<N>> {
        self.layers.get(index)
    }
}