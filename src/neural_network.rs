// neural_network.rs
use crate::neural_layer::NeuralLayer;

pub struct SpikingNeuralNetwork<T> {
    input_layer: NeuralLayer<T>,
    hidden_layers: Vec<NeuralLayer<T>>,
    output_layer: NeuralLayer<T>,
}

impl<T> SpikingNeuralNetwork<T> {
    pub fn new(
        input_size: usize,
        hidden_layer_sizes: &[usize],
        output_size: usize,
        reset_potential: f64,
        resting_potential: f64,
        threshold: f64,
        decay_factor: f64,
        synaptic_weights: Vec<T>,
    ) -> Self {
        let input_layer = NeuralLayer::new(
            input_size,
            reset_potential,
            resting_potential,
            threshold,
            decay_factor,
            synaptic_weights.clone(),
        );
        let hidden_layers: Vec<NeuralLayer<T>> = hidden_layer_sizes
            .iter()
            .map(|&size| NeuralLayer::new(size, reset_potential, resting_potential, threshold, decay_factor, synaptic_weights.clone()))
            .collect();
        let output_layer = NeuralLayer::new(
            output_size,
            reset_potential,
            resting_potential,
            threshold,
            decay_factor,
            synaptic_weights,
        );

        Self {
            input_layer,
            hidden_layers,
            output_layer,
        }
    }

    pub fn update(&mut self, input_spikes: &[Vec<T>], time_step: f64, impulse_duration: f64) -> Vec<bool> {
        // Aggiorna il layer di input con gli impulsi di input
        let input_layer_output = self.input_layer.update(input_spikes, time_step, impulse_duration);

        // Aggiorna i layer nascosti
        let mut hidden_layers_output = Vec::new();
        for hidden_layer in &mut self.hidden_layers {
            hidden_layers_output = hidden_layer.update(&[input_layer_output.clone()], time_step, impulse_duration);
        }

        // Aggiorna il layer di output
        self.output_layer.update(&hidden_layers_output, time_step, impulse_duration)
    }
}
