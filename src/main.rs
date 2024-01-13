// File: main.rs

mod lif_neuron;
pub mod neural_layer; // Importa il modulo neuron
use crate::{lif_neuron::LIFNeuron, spike::action_spike};

mod neural_network; // Importa il modulo NN
use neural_network::NeuralNetwork;

mod spike; // Importa il modulo Spike
use spike::Spike;

const RESET_POTENTIAL: f64 = 0.0;
const RESTING_POTENTIAL: f64 = 10.0;
const THRESHOLD: f64 = 1.0;
const TAU: f64 = 10.0;

fn main() {
    // Configura il neurone di partenza
    //DOBBIAMO METTERLO MUT?????????????
    let neuron_params = LIFNeuron::new(RESET_POTENTIAL, RESTING_POTENTIAL, THRESHOLD, TAU);

    // Configura la rete neurale
    let mut network = NeuralNetwork::new(vec![3,3,3], neuron_params);

    let spikes_neuron_1 = [1, 2, 3, 4, 5].to_vec();
    let spike_vec_for_neuron_1 = Spike::create_spike_vec(1, 1, spikes_neuron_1);
     
    let spikes_neuron_2 = [10, 29, 3, 11, 22].to_vec();
    let spike_vec_for_neuron_2 = Spike::create_spike_vec(2, 1, spikes_neuron_2);
     
    let mut spikes = Vec::new();
    spikes.push(spike_vec_for_neuron_1);
    spikes.push(spike_vec_for_neuron_2);
    
    let sorted_spike_array_for_nn = Spike::get_all_spikes(spikes.clone());
    let mut time = 0;

    while !check_empty_spike_vec(sorted_spike_array_for_nn.clone()) {
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


fn check_empty_spike_vec(sorted_spike_array_for_nn: Vec<u128>) -> bool {
    sorted_spike_array_for_nn.is_empty()
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