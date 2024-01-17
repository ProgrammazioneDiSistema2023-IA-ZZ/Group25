// File: main.rs

mod lif_neuron;
pub mod neural_layer; // Importa il modulo neuron
use crate::{lif_neuron::LIFNeuron, spike::action_spike};

mod neural_network; // Importa il modulo NN
use neural_network::NeuralNetwork;

mod spike; // Importa il modulo Spike
use spike::Spike;

mod tests;

const RESET_POTENTIAL: f64 = 0.7;
const RESTING_POTENTIAL: f64 = 2.0;
const THRESHOLD: f64 = 2.5;
const TAU: f64 = 1.0;

fn main() {
    // Configura il neurone di partenza
    //DOBBIAMO METTERLO MUT?????????????
    let neuron_params = LIFNeuron::new(RESET_POTENTIAL, RESTING_POTENTIAL, THRESHOLD, TAU);

    // Configura la rete neurale
    let input_weights: Vec<Vec<Vec<f64>>> = 
    vec![
        vec![
            vec![1.0, 0.0, 0.0], 
            vec![0.0, 1.0, 0.0], 
            vec![0.0, 0.0, 1.0]],
        vec![
            vec![9.05, 0.03, 4.49], 
            vec![2.24, 1.79, 1.33], 
            vec![4.78, 5.75, 9.94], 
            vec![5.16, 3.92, 5.64]],
        vec![
            vec![0.54, 9.09, 1.91, 0.63], 
            vec![5.58, 4.72, 2.12, 8.82], 
            vec![5.58, 0.92, 2.70, 0.93]]
    ];

    let intra_weights: Vec<Vec<Vec<f64>>> = 
    vec![
        vec![
            vec![0.00, -2.23, -2.12], 
            vec![-2.70, 0.00, -0.25], 
            vec![-0.06, -1.39, 0.00]],
        vec![
            vec![0.00, -1.87, -2.98, -2.68],
            vec![-0.50, 0.00, -1.77, -0.18], 
            vec![-2.63, -1.55, 0.00, -0.31], 
            vec![-0.59, -2.66, -1.29, 0.00]
            ],
        vec![
            vec![0.00, -0.24, -2.24 ],
            vec![-1.84, 0.00, -1.32],
            vec![-0.12, -1.73, 0.00]]
    ];

    let mut network = NeuralNetwork::new(vec![3,4,3], input_weights, intra_weights, neuron_params);

    let spikes = create_spike();
    let sorted_spike_array_for_nn = Spike::get_all_spikes(spikes.clone());
    let max_value = sorted_spike_array_for_nn
    .iter()
    .max()
    .expect("");
    let mut time = 0;

    while time < *max_value {
        // Incrementa il contatore
        time += 1;

        if sorted_spike_array_for_nn.contains(&time) {
            let nn = network.clone();
            action_spike(spikes.clone(), time, &mut network, &nn);
            //ATTENZIONE TOGLIERE I VALORI DI sorted_spike_array_for_nn (CICLO INFINITO)
        }
    }

    println!("Condizione raggiunta dopo {} iterazioni", time);
}


/* fn check_empty_spike_vec(sorted_spike_array_for_nn: Vec<u128>) -> bool {
    sorted_spike_array_for_nn.is_empty()
} */

fn create_spike() -> Vec<Vec<Spike>>{
    
    let spikes_neuron_1 = [1, 5, 7, 9, 20].to_vec();
    let spike_vec_for_neuron_1 = Spike::create_spike_vec(1, 1, spikes_neuron_1);
     
    let spikes_neuron_2 = [10, 29, 3, 11, 22].to_vec();
    let spike_vec_for_neuron_2 = Spike::create_spike_vec(2, 1, spikes_neuron_2);

    let spikes_neuron_3 = [1, 3, 5, 10, 13, 14].to_vec();
    let spike_vec_for_neuron_3 = Spike::create_spike_vec(2, 1, spikes_neuron_3);
     
    let mut spikes = Vec::new();
    spikes.push(spike_vec_for_neuron_1);
    spikes.push(spike_vec_for_neuron_2);
    spikes.push(spike_vec_for_neuron_3);
    
    spikes
}


    /* match network.get_layer(0) {
        Some(layer) => {
            // Estrarre il valore dal layer e chiamare get_input_weight_value
            let value = layer.get_intra_weight_value(0, 0);
            let value2 = layer.get_intra_weight_value(1, 2);
            let value3 = layer.get_input_weight_value(1, 1);
            let value4 = layer.get_input_weight_value(2, 2);
            // Fai qualcosa con il valore ottenuto
            println!("{:?} {:?} {:?} {:?}", value, value2, value3, value4);
        },
        None => {
            // Gestisci il caso in cui get_layer restituisce None
            println!("Layer non trovato");
        }
    }
 */