// File: main.rs
use std::sync::{Mutex, Arc};
use std::thread;

use std::rc::Rc;
use std::cell::RefCell;

use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

mod lif_neuron;
pub mod neural_layer; use crate::spike::update_neurons;
// Importa il modulo neuron
use crate::{lif_neuron::LIFNeuron, spike::action_spike};

mod neural_network; // Importa il modulo NN
use neural_network::NeuralNetwork;

mod spike; // Importa il modulo Spike
use spike::Spike;

mod tests;


fn main() {
    // Configura il neurone di partenza
    // Chiedi all'utente se vuole inserire i valori del neurone
    /* println!("Vuoi inserire i valori del neurone manualmente? (y/n)");

    let mut manual_input = String::new();
    io::stdin().read_line(&mut manual_input).expect("Errore durante la lettura dell'input");

    let neuron_params: LIFNeuron;
    // Crea il neurone utilizzando i parametri
    if manual_input.trim().to_lowercase() == "y" {
        neuron_params = LIFNeuron::from_user_input();
    } else {
        neuron_params = LIFNeuron::default();
    }

    // Configura la rete neurale
    let input_weights_file = "src/input_weights.txt";
    let intra_weights_file = "src/intra_weights.txt";

    let input_weights = read_matrix_from_file(input_weights_file).expect("Errore durante la lettura del file di input_weights");
    let intra_weights = read_matrix_from_file(intra_weights_file).expect("Errore durante la lettura del file di intra_weights");

 */
    let neuron_params = LIFNeuron::default();

    // Leggi le matrici di pesi da file
   // Configura la rete neurale
   let input_weights: Vec<Vec<Vec<f64>>> = 
   vec![
        vec![
            vec![1.0, 0.0], 
            vec![0.0, 1.0]],
        vec![
            vec![4.05, 0.03], 
            vec![2.24, 1.79]],
        vec![
            vec![0.54, 3.09], 
            vec![2.70, 0.93]]
    ];

    let intra_weights: Vec<Vec<Vec<f64>>> = 
    vec![
        vec![
            vec![0.00, -1.23], 
            vec![-1.70, 0.00]
            ],
        vec![
            vec![0.00, -0.87],
            vec![-0.50, 0.00],
            ],
        vec![
            vec![0.00, -0.24],
            vec![-0.84, 0.00]
            ]
    ];

    let layer_sizes = vec![2,2,2];
    let num_layers = 3;
    let network = Rc::new(RefCell::new(NeuralNetwork::new(layer_sizes, input_weights, intra_weights, neuron_params)));
    let spikes = create_spike();
    let sorted_spike_array_for_nn = Spike::get_all_spikes(spikes.clone());
    let max_value = *sorted_spike_array_for_nn
    .iter()
    .max()
    .expect("max not existing") + num_layers;
    let mut time = 0;


    while time < max_value {
        // Incrementa il contatore
        time += 1;

        if sorted_spike_array_for_nn.contains(&time) {
            let nn = network.clone();
            action_spike(spikes.clone(), time, Rc::clone(&network));
            
            //ciclo sui neuroni per calcolo soglia
            update_neurons(time, Rc::clone(&network))
        }
    }

    println!("Condizione raggiunta dopo {} iterazioni", time);
}


/* fn check_empty_spike_vec(sorted_spike_array_for_nn: Vec<u128>) -> bool {
    sorted_spike_array_for_nn.is_empty()
} */

fn create_spike() -> Vec<Vec<Spike>>{
    
    let spikes_neuron_1 = [1, 5, 7].to_vec();
    let spike_vec_for_neuron_1 = Spike::create_spike_vec(1, 1, spikes_neuron_1);
     
    //let spikes_neuron_2 = [10, 2, 4].to_vec();
    //let spike_vec_for_neuron_2 = Spike::create_spike_vec(2, 1, spikes_neuron_2);

    let spikes_neuron_3 = [1, 3, 5, 10].to_vec();
    let spike_vec_for_neuron_3 = Spike::create_spike_vec(2, 1, spikes_neuron_3);
     
    let mut spikes = Vec::new();
    spikes.push(spike_vec_for_neuron_1);
    //spikes.push(spike_vec_for_neuron_2);
    spikes.push(spike_vec_for_neuron_3);
    
    spikes
}





















fn read_matrix_from_file(file_path: &str) -> io::Result<Vec<Vec<Vec<f64>>>> {
    let file = File::open(file_path)?;
    let reader = io::BufReader::new(file);

    let mut matrix = Vec::new();

    for line in reader.lines() {
        let row: Vec<Vec<f64>> = line?
            .split_whitespace()
            .map(|s| {
                s.split(',')
                    .filter_map(|num| num.parse().ok())
                    .collect()
            })
            .collect();
        matrix.push(row);
    }

    Ok(matrix)
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