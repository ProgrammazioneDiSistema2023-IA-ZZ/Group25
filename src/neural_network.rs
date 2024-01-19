
use crate::lif_neuron::Neuron;
use crate::neural_layer::NeuralLayer;

use std::{sync::{Mutex, Arc}, thread};

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

    pub fn update_neurons(&mut self, time_step: u128, spike_input: Vec<f64>, num_layers: usize) -> Vec<f64> {
        // Inizializza un vettore per le membrane potenziali
        let mut membrane_potentials = Vec::new();
    
        // Itera attraverso ogni layer
        for layer_index in 0..num_layers {
            if layer_index == 0 {
                // Se il layer è il primo, crea un buffer di spike condiviso tra i thread
                let spike_buffer = Arc::new(Mutex::new(Vec::<f64>::new()));
                let mut threads = Vec::new();
    
                // Itera attraverso ogni neurone nel layer corrente
                for neuron_index in 0..self.layers.get(layer_index).unwrap().num_neurons() {
                    
                    let spike_buffer = spike_buffer.clone();
                    let mut current_layer = self.layers.get(layer_index).unwrap().clone();
                    let input_spike = spike_input.clone();
    
                    // Crea un thread per ogni neurone
                    threads.push(thread::spawn(move || {
                        // Calcola il potenziale di membrana del neurone attuale e lo aggiunge il potenziale al buffer di spike
                        spike_buffer.lock().unwrap().push(current_layer.get_neuron_mut(neuron_index).unwrap().handle_spike(*input_spike.get(neuron_index).unwrap(), time_step) as f64);
                        
                    }));
                }
    
                // Attendi che tutti i thread terminino
                for thread_handle in threads {
                    thread_handle.join().unwrap();
                }
    
                // Estrai le membrane potenziali dal buffer condiviso
                membrane_potentials = spike_buffer.lock().unwrap().to_vec();
                drop(spike_buffer);
    
                // Stampa le membrane potenziali del primo layer
                println!("stampastampastampa1---> {}", membrane_potentials.iter().map(|&x| x.to_string()).collect::<Vec<String>>().join("  "));
            } else {
                // Se il layer non è il primo, crea un buffer di spike condiviso tra i thread
                let spike_buffer = Arc::new(Mutex::new(Vec::<f64>::new()));
                let mut threads = Vec::new();
    
                // Itera attraverso ogni neurone nel layer corrente
                for neuron_index in 0..self.layers[layer_index].num_neurons() {
                    let spike_buffer = spike_buffer.clone();
                    let mut current_layer = self.layers[layer_index].clone();
                    let previous_layer = self.layers[layer_index - 1].clone();
                    let temporary_spikes = membrane_potentials.clone();
    
                    // Crea un thread per ogni neurone
                    threads.push(thread::spawn(move || {
                        // Calcola la somma pesata totale degli spike dal layer precedente
                        let total_weighted_sum = temporary_spikes.iter().zip(previous_layer.neurons.iter().enumerate())
                            .map(|(spike, previous_neuron)| spike * current_layer.get_input_weight_value(neuron_index, previous_neuron.0).unwrap())
                            .sum();
    
                        // Calcola il potenziale membranoso del neurone attuale
                        let potential = current_layer.get_neuron_mut(neuron_index).unwrap().handle_spike(total_weighted_sum, time_step) as f64;
                        // Aggiunge il potenziale al buffer di spike
                        spike_buffer.lock().unwrap().push(potential);
                    }));
                }
    
                // Attendi che tutti i thread terminino
                for thread_handle in threads {
                    thread_handle.join().unwrap();
                }
    
                // Estrai le membrane potenziali dal buffer condiviso
                membrane_potentials = spike_buffer.lock().unwrap().to_vec();
                drop(spike_buffer);
    
                // Stampa le membrane potenziali per il layer corrente
                println!("{}", membrane_potentials.iter().map(|&x| x.to_string()).collect::<Vec<String>>().join("  "));
            }
        }
    
        // Restituisci le membrane potenziali aggiornate
        membrane_potentials
    }
    

    /* pub fn update_neurons(&mut self, time: u128, spike: Vec<f64>) -> Vec<f64> {
        let mut output_spikes = Vec::new();

        // Iteriamo su tutti i layer nella rete neurale
        for (layer_index, layer) in self.layers.iter().enumerate() {
            // Creiamo una struttura dati condivisa (Mutex) per memorizzare gli spike temporanei di questo layer
            let mut temp = Arc::new(Mutex::new(Vec::<f64>::new()));
            let mut vt = Vec::new();

            // Iteriamo su tutti i neuroni nel layer corrente
            for neuron_index in 0..layer.num_neurons() {
                let temp = temp.clone();
                let layer_temp = layer.clone();
                let spike_temp = spike.clone();

                // Creiamo thread per gestire gli spike di ciascun neurone in parallelo
                vt.push(thread::spawn(move || {
                    // Aggiorniamo il potenziale di membrana del neurone e otteniamo lo spike
                    let new_spike = layer_temp
                        .get_neuron_mut(neuron_index)
                        .unwrap()
                        .handle_spike(*spike_temp.get(neuron_index).unwrap(), time);

                    // Aggiungiamo lo spike temporaneo al vettore condiviso 
                    // (contiene 0 o 1 in base al risultato della handle_spike effettuata in precedenza)
                    temp.lock().unwrap().push(new_spike as f64);
                }));
            }

            // Attendiamo che tutti i thread finiscano l'esecuzione
            for v in vt {
                v.join().unwrap();
            }

            // Creiamo una struttura dati condivisa per memorizzare i contributi degli spike intra-layer
            let layer_internal_temp = Arc::new(Mutex::new(vec![0.0; layer.num_neurons()]));
            let mut vet_internal_spike = Vec::new();

            // Iteriamo su tutti i neuroni nel layer corrente
            for current_neuron_index in 0..layer.num_neurons() {
                let layer_internal_temp = layer_internal_temp.clone();
                let layer_temp = layer.clone();
                let temporaneo = temp.clone();

                // Creiamo thread per calcolare i contributi degli spike intra-layer in parallelo
                vet_internal_spike.push(thread::spawn(move || {
                    // Iteriamo su tutti i neuroni nel layer corrente e aggiorniamo i contributi
                    for previous_neuron_index in 0..layer_temp.num_neurons() {
                        layer_internal_temp.lock().unwrap()[previous_neuron_index] =
                            temporaneo.lock().unwrap()[current_neuron_index]
                                * layer_temp.get_intra_weight_value(current_neuron_index, previous_neuron_index).unwrap();
                    }
                }));
            }

            // Attendiamo che tutti i thread finiscano l'esecuzione
            for v in vet_internal_spike {
                v.join().unwrap();
            }

            // Estraiamo i valori dal vettore condiviso
            let internal_temp_values;
            {
                internal_temp_values = layer_internal_temp.lock().unwrap().to_vec();
            }

            // Aggiorniamo i neuroni nel layer corrente con i contributi intra-layer
            for index in 0..internal_temp_values.len() {
                layer.neurons.get(index).unwrap().put_sum(*internal_temp_values.get(index).unwrap());
            }

            // Estraiamo gli spike dal vettore condiviso
            output_spikes = temp.lock().unwrap().to_vec();

            // Aggiungiamo un output per la stampa o il logging, se necessario
            for spike_value in &output_spikes {
                print!("{}  ", spike_value);
            }
            print!("\n");
        }

        // Restituisci gli spike finali
        output_spikes
    }
    */
}   
