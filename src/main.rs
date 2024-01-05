
// File: main.rs

mod lif_neuron; // Importa il modulo neuron
use lif_neuron::LIFNeuron; 

const RESET_POTENTIAL: f64 = 0.0;
const RESTING_POTENTIAL: f64 = 10.0;
const THRESHOLD: f64 = 55.0;
const TAU: f64 = 10.0;

fn main() {
    // Configura il neurone di partenza
    let neuron_params = LIFNeuron::new(RESET_POTENTIAL, RESTING_POTENTIAL, THRESHOLD, TAU);

    // Configura la rete neurale con un singolo strato di 5 neuroni
    let network = NeuralNetwork::new(vec![5], neuron_params);

    // Connetti gli strati della rete
    network.connect_layers();

    // Definisci gli input per la simulazione (ad esempio, un impulso iniziale)
    let input = vec![true, false, false, false, false];

    // Esegui la simulazione
    network.run_simulation(input);
}