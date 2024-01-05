
// File: main.rs

mod lif_neuron;
pub mod neural_layer; // Importa il modulo neuron
use crate::lif_neuron::LIFNeuron;

mod neural_network; // Importa il modulo NN
use neural_network::NeuralNetwork;

const RESET_POTENTIAL: f64 = 0.0;
const RESTING_POTENTIAL: f64 = 10.0;
const THRESHOLD: f64 = 55.0;
const TAU: f64 = 10.0;

fn main() {
    // Configura il neurone di partenza
    let neuron_params = LIFNeuron::new(RESET_POTENTIAL, RESTING_POTENTIAL, THRESHOLD, TAU);

    // Configura la rete neurale
    let network = NeuralNetwork::new(vec![3,2,3], neuron_params);

    // Definisci gli input per la simulazione (ad esempio, un impulso iniziale)
    let input = vec![true, false, false, false, false];
}