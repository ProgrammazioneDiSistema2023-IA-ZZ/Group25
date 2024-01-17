// lif_neuron.rs

pub trait Neuron: 'static + Clone {
    type ClassNeuron: 'static + Sized + Clone + Sync;

    fn put_sum(&mut self, value: f64);
    fn handle_spike(&mut self, current_spike_time: u128) -> u128;
}


#[derive(Clone, Copy, Debug)]
pub struct LIFNeuron {
    membrane_potential: f64,
    pub reset_potential: f64,
    pub resting_potential: f64,
    pub threshold: f64,
    pub tau: f64,
    last_spike_time: u128,
    last_membrane_potential: f64,
    sum: f64
}

impl LIFNeuron {
    pub fn new(
        reset_potential: f64,
        resting_potential: f64,
        threshold: f64,
        tau: f64
    ) -> Self {
        Self {
            membrane_potential: resting_potential,
            reset_potential,
            resting_potential,
            threshold,
            tau,
            last_spike_time : 0,
            last_membrane_potential: resting_potential,
            sum: 0.0
        }
    }



}

impl Neuron for LIFNeuron {
    type ClassNeuron = LIFNeuron;

    // Metodo per aggiungere un valore a sum
    fn put_sum(&mut self, value: f64) {
        self.sum += value;
    }

    fn handle_spike(&mut self, current_spike_time: u128) -> u128 {
        // This early exit serves as a small optimization
        if self.sum == 0.0 { return 0 }
        
        let delta_t = (current_spike_time - self.last_spike_time)as f64;
        self.last_spike_time = current_spike_time;

        // compute the new v_mem value
        self.membrane_potential = self.resting_potential + (self.membrane_potential - self.resting_potential) * (-delta_t / self.tau).exp() + self.sum;
        self.sum = 0.0;
        if self.membrane_potential > self.threshold {
            self.membrane_potential = self.reset_potential;
            1
        } else {
            0
        }
    }
}
