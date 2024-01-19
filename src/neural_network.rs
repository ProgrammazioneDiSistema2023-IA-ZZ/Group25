
use crate::lif_neuron::Neuron;
use crate::neural_layer::NeuralLayer;

use std::sync::Mutex;
use std::sync::Arc;
use std::thread;

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
        let l = self.get_layer_mut(index_layer).unwrap();
        let s = l.get_neuron_mut(index_neuron);
        return s;
    }

    pub fn update_neurons_parallel(&mut self, time_step: u128, spike_input: Vec<f64>, num_layers: usize) -> Vec<f64> {
        let mut layer_spikes = Vec::new();
    
        for layer_idx in 0..num_layers {
            if layer_idx == 0 {
                let mut spike_buffer = Vec::<f64>::new();
    
                for neuron_idx in 0..self.layers.get(layer_idx).unwrap().num_neurons() {
                    let layer= self.layers.get_mut(layer_idx).unwrap();
                    let spikes_clone = spike_input.clone();
    
                    let neuron_potential = layer.get_neuron_mut(neuron_idx)
                        .unwrap()
                        .handle_spike(*spikes_clone.get(neuron_idx).unwrap(), time_step) as f64;
    
                    spike_buffer.push(neuron_potential);
                    drop(layer);
                }

                layer_spikes = spike_buffer.clone();
    
            } 
            else 
            {
                // Aggiornamento degli altri layer
                let mut spike_buffer = Vec::<f64>::new();
    
                for current_neuron_idx in 0..self.layers.get(layer_idx).unwrap().num_neurons() {
                    let current_layer = self.layers.get(layer_idx).unwrap();
                    let layer_spikes_clone = layer_spikes.clone();
    
                    let mut weighted_sum = 0.0;

                    for other_neuron_idx in 0..current_layer.num_neurons() {
                        weighted_sum += layer_spikes_clone.get(other_neuron_idx).unwrap()
                            * current_layer.get_input_weight_value(current_neuron_idx, other_neuron_idx);
                    }

                    let current_layer = self.layers.get_mut(layer_idx).unwrap();

                    let neuron_potential = current_layer.get_neuron_mut(current_neuron_idx)
                        .unwrap()
                        .handle_spike(weighted_sum, time_step) as f64;

                    spike_buffer.push(neuron_potential);
                }
    
                layer_spikes = spike_buffer.clone();
            }
    
                let internal_spike_buffer = Arc::new(Mutex::new(vec![0.0; self.layers.get(layer_idx).unwrap().num_neurons()]));
                let mut vt =  Vec::new(); //vettore dei thread
    
                for current_neuron_idx in 0..self.layers.get(layer_idx).unwrap().num_neurons() {
                    let internal_spike_buffer_clone = Arc::clone(&internal_spike_buffer);
                    let current_layer = self.layers.get(layer_idx).unwrap().clone();
                    let layer_spikes_clone = layer_spikes.clone();  
                    vt.push(thread::spawn(move || {
                        for other_neuron_idx in 0..current_layer.num_neurons() {
                            internal_spike_buffer_clone.lock().unwrap()[other_neuron_idx] = 
                                layer_spikes_clone.get(current_neuron_idx).unwrap()
                                    * current_layer.get_intra_weight_value(current_neuron_idx, other_neuron_idx);
                        }
                    }));
                }
                
                for v in vt{ //aspettiamo le terminazioni dei thread
                    v.join().unwrap();
                }

                let internal_spike_len;
                {
                    internal_spike_len = internal_spike_buffer.lock().unwrap().len();
                }
    
                for neuron_idx in 0..internal_spike_len {
                    self.layers.get_mut(layer_idx).unwrap().neurons.get_mut(neuron_idx).unwrap()
                        .adjust_weight(*internal_spike_buffer.lock().unwrap().get(neuron_idx).unwrap());
                }
                drop(internal_spike_buffer);
    
                println!("values layer {}: {:?}", layer_idx, layer_spikes);
            }
        
    
        return layer_spikes;
    }

/* 
    pub fn update_neurons(&mut self, time_step: u128, spike_input: Vec<f64>, num_layers: usize) -> Vec<f64> {
        let mut layer_spikes = Vec::new();
    
        for layer_idx in 0..num_layers {
            if layer_idx == 0 {
                let mut spike_buffer = Vec::<f64>::new();
    
                for neuron_idx in 0..self.layers.get(layer_idx).unwrap().num_neurons() {
                    let layer= self.layers.get_mut(layer_idx).unwrap();
                    let spikes_clone = spike_input.clone();
    
                    let neuron_potential = layer.get_neuron_mut(neuron_idx)
                        .unwrap()
                        .handle_spike(*spikes_clone.get(neuron_idx).unwrap(), time_step) as f64;
    
                    spike_buffer.push(neuron_potential);
                }

                layer_spikes = spike_buffer.clone();
                println!("values layer {}: {:?}", layer_idx, layer_spikes);
    
                let mut internal_spike_buffer = vec![0.0; self.layers.get(layer_idx).unwrap().num_neurons()];
    
                for current_neuron_idx in 0..self.layers.get(layer_idx).unwrap().num_neurons() {
                    let current_layer = self.layers.get(layer_idx).unwrap();
                    let layer_spikes_clone = layer_spikes.clone();

                    for other_neuron_idx in 0..current_layer.num_neurons() {
                        internal_spike_buffer[other_neuron_idx] =
                            layer_spikes_clone.get(current_neuron_idx).unwrap()
                                * current_layer.get_intra_weight_value(current_neuron_idx, other_neuron_idx);
                    }
                }
    
                let internal_spike_len;
                {
                    internal_spike_len = internal_spike_buffer.len();
                }
    
                for neuron_idx in 0..internal_spike_len {
                    self.layers.get_mut(layer_idx).unwrap().neurons.get_mut(neuron_idx).unwrap()
                            .adjust_weight(*internal_spike_buffer.to_vec().get(neuron_idx).unwrap());
                }
            } 
            else 
            {
                // Aggiornamento degli altri layer
                let mut spike_buffer = Vec::<f64>::new();
    
                for current_neuron_idx in 0..self.layers.get(layer_idx).unwrap().num_neurons() {
                    let current_layer = self.layers.get(layer_idx).unwrap();
                    let layer_spikes_clone = layer_spikes.clone();
    
                    let mut weighted_sum = 0.0;

                    for other_neuron_idx in 0..current_layer.num_neurons() {
                        weighted_sum += layer_spikes_clone.get(other_neuron_idx).unwrap()
                            * current_layer.get_input_weight_value(current_neuron_idx, other_neuron_idx);
                    }

                    let current_layer = self.layers.get_mut(layer_idx).unwrap();

                    let neuron_potential = current_layer.get_neuron_mut(current_neuron_idx)
                        .unwrap()
                        .handle_spike(weighted_sum, time_step) as f64;

                    spike_buffer.push(neuron_potential);
                }
    
                layer_spikes = spike_buffer.clone();
                drop(spike_buffer);
    
                let mut internal_spike_buffer = vec![0.0; self.layers.get(layer_idx).unwrap().num_neurons()];
    
                for current_neuron_idx in 0..self.layers.get(layer_idx).unwrap().num_neurons() {
                    let current_layer_clone = self.layers.get(layer_idx).unwrap().clone();
                    let layer_spikes_clone = layer_spikes.clone();
    
                    for other_neuron_idx in 0..current_layer_clone.num_neurons() {
                        internal_spike_buffer[other_neuron_idx] =
                            layer_spikes_clone.get(current_neuron_idx).unwrap()
                                * current_layer_clone.get_intra_weight_value(current_neuron_idx, other_neuron_idx);
                    }
                }
    
                let internal_spike_len;
                {
                    internal_spike_len = internal_spike_buffer.len();
                }
    
                for neuron_idx in 0..internal_spike_len {
                    self.layers.get_mut(layer_idx).unwrap().neurons.get_mut(neuron_idx).unwrap()
                        .adjust_weight(*internal_spike_buffer.get(neuron_idx).unwrap());
                }
                drop(internal_spike_buffer);
    
                println!("values layer {}: {:?}", layer_idx, layer_spikes);
            }
        }
    
        return layer_spikes;
    }
     */
}   
