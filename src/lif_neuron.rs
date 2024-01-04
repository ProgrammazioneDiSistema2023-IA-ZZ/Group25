// lif_neuron.rs

pub struct SpikingNeuron<T> {
    membrane_potential: f64,
    reset_potential: f64,
    resting_potential: f64,
    threshold: f64,
    decay_factor: f64,
    synaptic_weights: Vec<T>,
}

impl<T> SpikingNeuron<T> {
    pub fn new(
        reset_potential: f64,
        resting_potential: f64,
        threshold: f64,
        decay_factor: f64,
        synaptic_weights: Vec<T>,
    ) -> Self {
        Self {
            membrane_potential: resting_potential,
            reset_potential,
            resting_potential,
            threshold,
            decay_factor,
            synaptic_weights,
        }
    }

    pub fn integrate(&mut self, input_spikes: &[T], time_step: f64) -> bool {
        // Logica di integrazione qui
        // Aggiorna il potenziale di membrana in base agli input e al tempo
        let weighted_sum: f64 = input_spikes.iter().zip(&self.synaptic_weights).map(|(x, w)| *x * *w).sum();
        self.membrane_potential = self.membrane_potential * self.decay_factor + weighted_sum;

        // Verifica se il neurone ha superato la soglia e restituisce true in caso di fuoco
        if self.membrane_potential >= self.threshold {
            self.membrane_potential = self.reset_potential; // Reset del potenziale di membrana dopo lo scoppio
            return true;
        }

        false
    }
}
