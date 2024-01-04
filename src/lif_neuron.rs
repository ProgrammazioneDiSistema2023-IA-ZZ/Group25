// lif_neuron.rs

pub struct LIFNeuron {
    pub membrane_potential: f64,
    pub threshold: f64,
    pub resistance: f64,
    pub capacitance: f64,
    pub resting_potential: f64,
}

impl LIFNeuron {
    pub fn new(threshold: f64, resistance: f64, capacitance: f64, resting_potential: f64) -> Self {
        Self {
            membrane_potential: resting_potential,
            threshold,
            resistance,
            capacitance,
            resting_potential,
        }
    }

    pub fn integrate(&mut self, input_spike: f64, time_step: f64) {
        // Logica di integrazione qui
        // Aggiorna il potenziale di membrana in base all'input e al tempo

        // Verifica se il neurone ha superato la soglia e scoppia
        if self.membrane_potential >= self.threshold {
            self.fire();
        }
    }

    pub fn fire(&mut self) {
        // Logica di scoppio qui
        // Gestisci cosa succede quando il neurone scoppia
        println!("Neuron fired!");
        self.membrane_potential = self.resting_potential;  // Reset del potenziale di membrana dopo lo scoppio
    }
}
