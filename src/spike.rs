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

    /// Create an array of spikes for a single neuron, given its ID.
    /// The `ts_vec` does not need to be ordered.
    /// 
    
    pub fn create_spike_vec(neuron_id: usize, ts_vec: Vec<u128>) -> Vec<Spike> {
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
    
    pub fn get_all_spikes(spikes: Vec<Vec<Spike>>) -> Vec<Spike> {
        let mut res: Vec<Spike> = Vec::new();

        for line in spikes {
            for spike in line {
                res.push(spike);
            }
        }
        res.sort(); //ascending
    
        res
    }
}
