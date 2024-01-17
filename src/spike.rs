
use crate::{neural_network::NeuralNetwork, lif_neuron::Neuron};

#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Debug)]
pub struct Spike {
    /// Stands for "time of the spike", and represents a timestamp of when the spike occurs
    pub spike_time: u128,
    /// Index of the neuron this spike applies to inside its layer
    pub neuron_id: usize,
    /// Index of the layer this spike applies
    pub layer_id: usize
}

impl Spike {
    /// Create a new spike at time `ts` for neuron `neuron_id`
    pub fn new(spike_time: u128, neuron_id: usize, layer_id: usize) -> Spike {
        Spike {
            spike_time,
            neuron_id,
            layer_id
        }
    }

     /// Get the spike time of the current spike
     pub fn get_spike_time(&self) -> u128 {
        self.spike_time
    }

     /// Get the spike neuron id of the current spike
     pub fn get_spike_neuron_id(&self) -> usize {
        self.neuron_id
    }

      /// Get the spike layer id of the current spike
      pub fn get_spike_layer_id(&self) -> usize {
        self.layer_id
    }

    /// Create an array of spikes for a single neuron, given its ID.
    /// The `ts_vec` does not need to be ordered.
    /// 
    
    pub fn create_spike_vec(neuron_id: usize, layer_id: usize, ts_vec: Vec<u128>) -> Vec<Spike> {
        let mut spike_vec : Vec<Spike> = Vec::with_capacity(ts_vec.len());
        
        //Creating the Spikes array for a single Neuron
        for ts in ts_vec.into_iter() {
            spike_vec.push(Spike::new(ts, neuron_id, layer_id));
        }

        //Order the ts vector
        spike_vec.sort();

        spike_vec
    }


    /// Create an ordered array starting from all the spikes sent to the NN.
    /// 
    /// It takes a Matrix where i-th row represents an array of spikes for the i-th entry neuron,
    /// then a single Vec is created. Eventually the array is sorted.
    
    pub fn get_all_spikes(spikes: Vec<Vec<Spike>>) -> Vec<u128> {
        let mut res: Vec<u128> = Vec::new();

        for line in spikes {
            for spike in line {
                res.push(spike.get_spike_time());
            }
        }
        res.sort(); //ascending
    
        res
    }
/* 
    pub fn get_all_spike_time(spikes: Vec<Vec<Spike>>) -> Vec<Vec<u128>> {
        let mut res: Vec<Vec<u128>> = Vec::new();

        for line in spikes {
            let mut riga: Vec<u128> = Vec::new();
            for spike in line {
                riga.push(spike.get_spike_time());
            }
            res.push(riga);
        }
        res.sort(); //ascending
    
        res
    } */
}


fn contains_time<'a>(spike_vec: &'a [Spike], time: u128) -> Option<&'a Spike> {
    for spike in spike_vec.iter() {
        if spike.spike_time == time {
            return Some(spike);
        }
    }
    None
}

  // Assumiamo che la struttura Spike sia già definita, ad esempio:
// struct Spike { /* definizione dei campi */ }

pub fn action_spike<N: Neuron>(spikes: Vec<Vec<Spike>>, time: u128, network: &mut NeuralNetwork<N>, nn: &NeuralNetwork<N>) -> bool {
    for riga in spikes.iter() {
        match contains_time(&riga, time) {
            Some(spike) => {
                if let Some(neuron) = network.get_neuron_mut(spike.layer_id, spike.neuron_id) {
                    neuron.put_sum(1.0);
                    call_handle_spike(neuron, time, spike.neuron_id, spike.layer_id, nn);
                    println!("Spike trovato - Neuron ID: {}, Layer ID: {}, Time: {}", spike.neuron_id, spike.layer_id, time);
                } else {
                    println!("Failed to get mutable reference to neuron.");
                }
            }
            None => {
                println!("Spike con tempo {} non trovato.", time);
            }
        }
    }
    false
}

pub fn propagate_spike<N: Neuron>(spike: &mut Spike, network: &NeuralNetwork<N>) {
    let n = spike.neuron_id;
    let time = spike.spike_time + 1;
    
    let current_layer_id = spike.layer_id;
    let next_layer_id = spike.layer_id + 1;

    let current_layer = network.get_layer(current_layer_id).expect("non esiste layer corrente");

    if let Some(next_layer) = network.get_layer(next_layer_id) {
        for (index, neuron) in next_layer.get_neurons().iter_mut().enumerate() {
            let weight = next_layer.get_input_weight_value(n, index).expect("non esiste il peso");
            neuron.put_sum(*weight);
            println!("Neurone aggiornato: {} {}", index, next_layer_id);
            call_handle_spike(neuron, time, index, next_layer_id, network);
        }
    }

    for (index, neuron) in current_layer.get_neurons().iter_mut().enumerate() {
        let weight = current_layer.get_intra_weight_value(n, index).expect("non esiste il peso");
        neuron.put_sum(*weight);
        println!("Neurone aggiornato: {} {}", index, current_layer_id);
        call_handle_spike(neuron, time, index, current_layer_id, network);
    }
}


pub fn call_handle_spike<N: Neuron>(neuron: &mut N, time: u128, neuron_id: usize, layer_id: usize, network: &NeuralNetwork<N>) {
    let result = neuron.handle_spike(time);
    match result {
        1 => {
            let mut spike = Spike::new(time, neuron_id, layer_id);
            propagate_spike(&mut spike, network);
        }
        _ => {}
    }
}
