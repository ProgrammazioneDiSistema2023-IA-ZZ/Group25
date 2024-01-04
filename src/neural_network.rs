// neural_network.rs
use crate::neural_layer::NeuralLayer; // Assicurati di importare il modulo del layer neurale

pub struct SpikingNeuralNetwork {
    pub input_layer: NeuralLayer,
    pub hidden_layers: Vec<NeuralLayer>,
    pub output_layer: NeuralLayer,
}

impl SpikingNeuralNetwork {
    pub fn new(
        input_size: usize,
        hidden_layer_sizes: &[usize],
        output_size: usize,
        threshold: f64,
        resistance: f64,
        capacitance: f64,
        resting_potential: f64,
    ) -> Self {
        let input_layer = NeuralLayer::new(input_size, threshold, resistance, capacitance, resting_potential);
        let hidden_layers: Vec<NeuralLayer> = hidden_layer_sizes
            .iter()
            .map(|&size| NeuralLayer::new(size, threshold, resistance, capacitance, resting_potential))
            .collect();
        let output_layer = NeuralLayer::new(output_size, threshold, resistance, capacitance, resting_potential);

        Self {
            input_layer,
            hidden_layers,
            output_layer,
        }
    }

    pub fn update(&mut self, input_spikes: &[f64], time_step: f64) {
        // Logica di aggiornamento qui
        // Aggiorna il layer di input con gli impulsi di input
        self.input_layer.update(input_spikes, time_step);

        // Aggiorna i layer nascosti
        for hidden_layer in &mut self.hidden_layers {
            hidden_layer.update(&[], time_step); // In questo esempio, non ci sono impulsi di input tra i layer nascosti
        }

        // Aggiorna il layer di output
        self.output_layer.update(&[], time_step); // In questo esempio, non ci sono impulsi di input tra l'ultimo layer e l'output
    }
}
