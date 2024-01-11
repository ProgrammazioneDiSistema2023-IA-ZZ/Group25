use crate::lif_neuron::Neuron;
use crate::neural_layer::NeuralLayer;

#[derive(Clone)]
pub struct NeuralNetwork<N: Neuron> {
    /// All the sorted layers of the neural network
    pub layers: Vec<NeuralLayer<N>>
}

impl<N: Neuron> NeuralNetwork<N> {

    pub fn new(layer_sizes: Vec<usize>, neuron: N) -> NeuralNetwork<N> {
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


    /// Get a reference to a specific layer by index
    pub fn get_layer(&self, index: usize) -> Option<&NeuralLayer<N>> {
        self.layers.get(index)
    }

    /// Get a reference to a specific neuron
    pub fn get_neuron(&self, index_layer: usize, index_neuron: usize) -> Option<&N> {
        return self.get_layer(index_layer)?.get_neuron(index_neuron);
    }

}