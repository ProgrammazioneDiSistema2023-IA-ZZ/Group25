// File: main.rs

use std::fs;
use std::fs::File;
use std::io::{self, BufRead, Write};
use std::path::Path;

mod lif_neuron;
mod errors;
mod simulation_error;
use crate::errors::{stuck_at_0,stuck_at_1,bit_flip};
use crate::simulation_error::{ErrorType, Component}; 

pub mod neural_layer;
use crate::lif_neuron::Neuron;
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
    println!("Vuoi inserire i valori del neurone manualmente? (y/n)");

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
    let input_weights_file = "input_weights_222.txt";
    let intra_weights_file = "intra_weights_222.txt";

    let input_weights = read_matrix_file(input_weights_file).expect("Errore durante la lettura del file di input_weights");
    let intra_weights = read_matrix_file(intra_weights_file).expect("Errore durante la lettura del file di intra_weights");

    println!("{:?}", input_weights); 
    println!("{:?}", intra_weights); 

    // Leggi le matrici di pesi da file
   // Configura la rete neurale
   /* let neuron_params= LIFNeuron::default();
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
    ]; */
    let layer_sizes = vec![2,2,2];
    let num_layers: usize = 3;
    let mut network = NeuralNetwork::new(layer_sizes, input_weights, intra_weights, neuron_params);

    /* println!("inizio");
    network.get_layer_mut(0).unwrap().get_neuron_mut(0).unwrap().handle_spike(5.0, 0);
    network.get_layer_mut(0).unwrap().get_neuron_mut(0).unwrap().handle_spike(5.0, 0);
    println!("fine");
 */
    let spikes = create_spike();
    let sorted_spike_array_for_nn = Spike::get_all_spikes(spikes.clone());
    let max_value = *sorted_spike_array_for_nn
    .iter()
    .max()
    .unwrap();
    let mut time = 0;
    while time < max_value {
        // Incrementa il contatore
        time += 1;
        let mut s= vec![0.0, 0.0];

        if sorted_spike_array_for_nn.contains(&time) {
            s = action_spike(spikes.clone(), time);
        }
        //ciclo sui neuroni per calcolo soglia
        println!("TIME----------------------> {:?}", time);
        println!("PRE----> {:?}", s);
        s = network.update_neurons_parallel(time, s, num_layers);
        println!("POST----> {:?}", s);
    }

    println!("Condizione raggiunta dopo {} iterazioni", time);

    //introduzione degli errori, proviamo a modificare un peso del layer 1 
    let error_type = ErrorType::StuckAt1; // Sostituisci con il tipo di errore desiderato 
    let component = Component::Tau;
    //prova sui pesi
    // network.get_layer_mut(1).unwrap().modify_weights_layer(&error_type); 
    // network.get_layer_mut(1).unwrap().print_input_weights(); 
    // network.get_layer_mut(1).unwrap().print_intra_weights();

    //prova sulle component del neurone
    println!("test compo neurone");
    network.get_layer_mut(1).unwrap().get_neuron_mut(0).unwrap().print_neuron_parameters();
    network.get_layer_mut(1).unwrap().get_neuron_mut(0).unwrap().modify_parameters_neuron(component, &error_type);
    network.get_layer_mut(1).unwrap().get_neuron_mut(0).unwrap().print_neuron_parameters();


}


/* fn check_empty_spike_vec(sorted_spike_array_for_nn: Vec<u128>) -> bool {
    sorted_spike_array_for_nn.is_empty()
} */

fn create_spike() -> Vec<Vec<Spike>>{
    
    let spikes_neuron_1 = [1, 5, 7].to_vec();
    let spike_vec_for_neuron_1 = Spike::create_spike_vec(0,0, spikes_neuron_1);
     
    let spikes_neuron_2 = [10, 2, 4].to_vec();
    let spike_vec_for_neuron_2 = Spike::create_spike_vec(1, 0, spikes_neuron_2);

    let spikes_neuron_3 = [2, 3, 5, 10].to_vec();
    let spike_vec_for_neuron_3 = Spike::create_spike_vec(2, 0, spikes_neuron_3);
     
    let mut spikes = Vec::new();
    spikes.push(spike_vec_for_neuron_1);
    spikes.push(spike_vec_for_neuron_2);
    spikes.push(spike_vec_for_neuron_3);
    
    spikes
}

fn read_matrix_file(file_path: &str) -> Result<Vec<Vec<Vec<f64>>>, io::Error> {
    let file = File::open(file_path).expect("File non trovato");
    let reader = io::BufReader::new(file);

    let mut result: Vec<Vec<Vec<f64>>> = Vec::new();
    let mut current_matrix: Vec<Vec<f64>> = Vec::new();

    for line in reader.lines() {
        let line = line?;
        let values: Vec<f64> = line
            .split(',')
            .filter_map(|s| s.trim().parse().ok())
            .collect();

        if values.is_empty() {
            // Empty line, end of matrix
            result.push(current_matrix);
            current_matrix = Vec::new();
        } else {
            current_matrix.push(values);
        }
    }

    // Push the last matrix if there is any
    if !current_matrix.is_empty() {
        result.push(current_matrix);
    }

    Ok(result)
}