use rand::Rng;

// neural_layer.rs
use crate::lif_neuron::Neuron;
use std::sync::{Mutex, Arc};
use crate::{simulation_error::ErrorType, errors::modify_weight_based_on_error};  
/* use rand::{Rng, thread_rng};

fn generate_random_vector(n: usize, min: f64, max: f64) -> Vec<f64> {
    let mut rng = thread_rng();
    (0..n).map(|_| rng.gen_range(min..max)).collect()
}
 */
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

     pub fn new(layer_size: usize, input_weights: Vec<Vec<f64>>, intra_weights: Vec<Vec<f64>>, mut neuron: N) -> NeuralLayer<N> {
        let mut neurons = vec![neuron; layer_size];

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

    pub fn get_intra_weight_value(&self, from: usize, to: usize) -> &f64 {
        self.intra_weights.get(from).unwrap().get(to).unwrap()
    }

    pub fn get_input_weight_value(&self, from: usize, to: usize) -> &f64 {
        self.input_weights.get(from).unwrap().get(to).unwrap()
    }

    pub fn print_intra_weights(&self) { 
        println!("Intra-layer weights matrix:"); 
        for row in &self.intra_weights { 
            for &value in row { 
                print!("{:.14}  ", value); // Modifica il formato di stampa come preferisci 
            } 
            println!(); // Nuova riga per ogni riga della matrice 
        } 
    } 
 
    pub fn print_input_weights(&self) { 
        println!("Input-layer weights matrix:"); 
        for row in &self.input_weights { 
            for &value in row { 
                print!("{:.14}  ", value); // Modifica il formato di stampa come preferisci 
            } 
            println!(); // Nuova riga per ogni riga della matrice 
        } 
    } 
 
    //modifica weights 
    pub fn modify_weights_layer(&mut self, error_type: &ErrorType) { 
        let mut rng = rand::thread_rng(); 
    
        // Decide whether to modify intra_weights or input_weights 
        let modify_intra_weights = rng.gen_bool(0.5); 
    
        if modify_intra_weights { 
            // Modify intra_weights 
            self.print_intra_weights(); 
            self.modify_random_intra_weight(error_type); 
            self.print_intra_weights(); 
        } else { 
            // Modify input_weights 
            self.print_input_weights(); 
            self.modify_random_input_weight(error_type); 
            self.print_input_weights(); 
        } 
    } 
    
    pub fn modify_random_intra_weight(&mut self, error_type: &ErrorType) { 
        let mut rng = rand::thread_rng(); 
    
        // Scegli casualmente gli indici della matrice intra_weights 
        let from_index = rng.gen_range(0..self.intra_weights.len()); 
        let to_index = rng.gen_range(0..self.intra_weights[from_index].len()); 
        println!{"{} {}",from_index,to_index}; 
        modify_weight_based_on_error(&mut self.intra_weights[from_index][to_index], error_type); 
    } 
    
    
    pub fn modify_random_input_weight(&mut self, error_type: &ErrorType) { 
        let mut rng = rand::thread_rng(); 
    
        // Scegli casualmente gli indici della matrice intra_weights 
        let from_index = rng.gen_range(0..self.input_weights.len()); 
        let to_index = rng.gen_range(0..self.input_weights[from_index].len()); 
        println!{"{} {}",from_index,to_index}; 
        modify_weight_based_on_error(&mut self.input_weights[from_index][to_index], error_type); 
    }
}
