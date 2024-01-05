// neural_network.rs
use crate::neural_layer::NeuralLayer;

#[derive(Clone)]
pub struct NeuralNetwork<M: Model> {
    /// All the sorted layers of the neural network
    layers: Vec<NeuralLayer<M>>
}

impl<M: Model> NeuralNetwork<M> {
    /// Get a reference to a specific layer by index
    pub fn get_layer(&self, index: usize) -> Option<&NeuralLayer<M>> {
        self.layers.get(index)
    }
}