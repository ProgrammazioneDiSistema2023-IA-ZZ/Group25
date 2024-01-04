// neural_layer.rs
use crate::lif_neuron::SpikingNeuron;

pub struct NeuralLayer<T> {
    neurons: Vec<SpikingNeuron<T>>,
}

impl<T> NeuralLayer<T> {
    pub fn new(
        size: usize,
        reset_potential: f64,
        resting_potential: f64,
        threshold: f64,
        decay_factor: f64,
        synaptic_weights: Vec<T>,
    ) -> Self {
        let neurons = (0..size)
            .map(|_| SpikingNeuron::new(reset_potential, resting_potential, threshold, decay_factor, synaptic_weights.clone()))
            .collect();

        Self { neurons }
    }

    pub fn update(&mut self, input_spikes: &[Vec<T>], time_step: f64, impulse_duration: f64) -> Vec<bool> {
        let mut output_spikes = Vec::with_capacity(self.neurons.len());

        for (neuron, input_spike) in self.neurons.iter_mut().zip(input_spikes) {
            let fired = neuron.integrate(input_spike, time_step, impulse_duration);
            output_spikes.push(fired);
        }

        output_spikes
    }
}
