
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
}


pub fn contains_time<'a>(spike_vec: &'a [Spike], time: u128) -> Option<&'a Spike> {
    for spike in spike_vec.iter() {
        if spike.spike_time == time {
            return Some(spike);
        }
    }
    None
}

pub fn action_spike(spikes: Vec<Vec<Spike>>, time: u128) -> Vec<f64>{

    let mut v = vec![];
    for riga in spikes.iter() {
        match contains_time(&riga, time) {
            Some(_) => {
                v.push(1.0);
            }
            None => {
                v.push(0.0);
            }
        }
    }

    v
}