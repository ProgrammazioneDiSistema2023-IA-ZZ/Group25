// neural_layer.rs
use crate::lif_neuron::LIFNeuron; // Assicurati di importare il modulo del neurone LIF

pub struct NeuralLayer {
    pub neurons: Vec<LIFNeuron>,
}

impl NeuralLayer {
    pub fn new(size: usize, threshold: f64, resistance: f64, capacitance: f64, resting_potential: f64) -> Self {
        let neurons = (0..size)
            .map(|_| LIFNeuron::new(threshold, resistance, capacitance, resting_potential))
            .collect();

        Self { neurons }
    }

    pub fn update(&mut self, input_spikes: &[f64], time_step: f64) {
        // Logica di aggiornamento qui
        // Itera attraverso i neuroni e aggiorna ciascun neurone con gli impulsi di input
        for (neuron, &input_spike) in self.neurons.iter_mut().zip(input_spikes) {
            neuron.integrate(input_spike, time_step);
        }
    }
}
