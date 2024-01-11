
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
    }
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

pub fn action_spike<N: Neuron>(mut spikes: Vec<Vec<Spike>>, time: u128, network: NeuralNetwork<N>) -> bool {
    for riga in spikes.iter() {
        match contains_time(&riga, time) {
            Some(spike) => {
                let neuron = network.get_neuron(spike.layer_id, spike.neuron_id);
                neuron.put_sum(1); // SISTEMARE QUA
                println!("Spike trovato - Neuron ID: {}, Layer ID: {}, Time: {}", spike.neuron_id, spike.layer_id, time);
            }
            None => {
                println!("Spike con tempo {} non trovato.", time);
            }
        }
    
}
false}

// Funzione per azionare uno spike
fn action_single_spike(spike: &mut Spike) -> i32 {
    // Fai qualcosa con lo spike e restituisci un risultato (ad esempio, 1 se ha avuto successo)
    //prendi il layer id dallo spike per poter usare la funzione handle_spike sul neurone (ottenuto sempre dallo spike)
    

    1 // Modifica questo valore in base alle tue esigenze
}

// Funzione per propagare uno spike nel caso di successo
fn propagate_spike(layer_spikes: &mut Vec<Spike>) {
    // Implementa la propagazione dello spike nel vettore
    // ...

    // Esempio di propagazione: inserisci uno spike nel vettore
    // let new_spike = Spike { /* inizializza i campi dello spike */ };
    // layer_spikes.push(new_spike);
}

