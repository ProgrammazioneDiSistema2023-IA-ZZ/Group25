
use crate::lif_neuron::Neuron;
use crate::neural_layer::NeuralLayer;

use std::sync::{Mutex, Arc};

#[derive(Clone)]
pub struct NeuralNetwork<N: Neuron> {
    /// All the sorted layers of the neural network
    pub layers: Vec<NeuralLayer<N>>
}

impl<N: Neuron> NeuralNetwork<N> {

    /*pub fn new(layer_sizes: Vec<usize>, neuron: N) -> NeuralNetwork<N> {
        let mut layers = Vec::with_capacity(layer_sizes.len());
    
        // Iterate over layer_sizes to create NeuralLayer instances
        for &size in &layer_sizes {
            // Find the next layer size
            let next_size = layer_sizes.get(layer_sizes.iter().position(|&x| x == size).unwrap_or(0) + 1)
                .cloned()
                .unwrap_or(0);
    
            // Create a new NeuralLayer with the current size, next size, and neuron
            let neural_layer = NeuralLayer::new(size, next_size, neuron.clone());
            
            // Push the created NeuralLayer into the layers vector
            layers.push(neural_layer);
        }
    
        // Create and return the NeuralNetwork with the populated layers vector
        NeuralNetwork { layers }
    }
    */

    pub fn new(layer_sizes: Vec<usize>, input_weights: Vec<Vec<Vec<f64>>>, intra_weights: Vec<Vec<Vec<f64>>>, neuron: N) -> NeuralNetwork<N> {
        let mut layers = Vec::with_capacity(layer_sizes.len());
    
        // Iterate over layer_sizes to create NeuralLayer instances
        for (index, size) in layer_sizes.iter().enumerate() {
            println!("{:?} {}", {}, index);
            let input = input_weights.get(index).expect("out of bounds");
            let intra = intra_weights.get(index).expect("out of bounds");
            // Create a new NeuralLayer with the current size, next size, and neuron
            let neural_layer = NeuralLayer::new(*size, input.to_vec(), intra.to_vec(), neuron.clone());
            
            // Push the created NeuralLayer into the layers vector
            layers.push(neural_layer);
        }
    
        // Create and return the NeuralNetwork with the populated layers vector
        NeuralNetwork { layers }
    }



    /// Get a reference to a specific layer by index
    pub fn get_layer(&self, index: usize) -> Option<&NeuralLayer<N>> {
        self.layers.get(index)
    }

    pub fn get_layer_mut(&mut self, index_layer: usize) -> Option<&mut NeuralLayer<N>> {
        self.layers.get_mut(index_layer)
    }

     /* /// Get a reference to a specific neuron
     pub fn get_neuron(&self, index_layer: usize, index_neuron: usize) -> Option<&N> {
        let l = self.get_layer(index_layer)?;
        let s = l.get_neuron(index_neuron);
        return s;
    } */

    /// Get a reference to a specific neuron
    pub fn get_neuron_mut(&mut self, index_layer: usize, index_neuron: usize) -> Option<&mut N> {
        let l = self.get_layer_mut(index_layer)?;
        let s = l.get_neuron_mut(index_neuron);
        return s;
    }


}