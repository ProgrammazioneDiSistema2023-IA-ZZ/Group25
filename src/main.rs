// File: main.rs

use std::{fs, error};
use std::fs::File;
use std::io::{self, BufRead, Write};
use std::path::Path;
use std::process::Output;

mod lif_neuron;
mod errors;
mod simulation_error;
use crate::errors::{stuck_at_0,stuck_at_1,bit_flip};
use crate::simulation_error::*; 

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
    println!("Benvenuto in Brain Training!");
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

    // Leggi le matrici di pesi da file
    let input_weights_file = "data/input_weights_222.txt";
    let intra_weights_file = "data/intra_weights_222.txt";

    let input_weights = read_matrix_file(input_weights_file).expect("Errore durante la lettura del file di input_weights");
    let intra_weights = read_matrix_file(intra_weights_file).expect("Errore durante la lettura del file di intra_weights");

    println!("{:?}", input_weights); 
    println!("{:?}", intra_weights); 

    //Creazione rete neurale
    let layer_sizes = vec![2,2,2];
    let num_layers: usize = 3;
    let mut network = NeuralNetwork::new(layer_sizes.clone(), input_weights, intra_weights, neuron_params);

    // Leggi le spike da file
    let spike_file = generate_spike_file(layer_sizes[0]);
    //let spike_file = "data/spike2.txt";
    let spikes = create_spike(spike_file);

    let sorted_spike_array_for_nn = Spike::get_all_spikes(spikes.clone());
    let max_value = *sorted_spike_array_for_nn
    .iter()
    .max()
    .unwrap();
    let mut time = 0;


    let mut output = Vec::new();
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
        output.push(s.clone());
        println!("POST----> {:?}", s);
    }

    println!("OUTPUT: {:?}", output);
    println!("Condizione raggiunta dopo {} iterazioni", time);

    //introduzione degli errori, proviamo a modificare un peso del layer 1 
    let error_type = ErrorType::StuckAt1; // Sostituisci con il tipo di errore desiderato 
    let component = Component::Tau;
    let error_type_2 = ErrorType::StuckAt0;
    let component_2 = Component::Threshold ;
    //prova sui pesi
    // network.get_layer_mut(1).unwrap().modify_weights_layer(&error_type); 
    // network.get_layer_mut(1).unwrap().print_input_weights(); 
    // network.get_layer_mut(1).unwrap().print_intra_weights();

    //prova sulle component del neurone
    println!("test compo neurone");
    network.get_layer_mut(1).unwrap().get_neuron_mut(0).unwrap().print_neuron_parameters();
    //network.get_layer_mut(1).unwrap().get_neuron_mut(0).unwrap().modify_parameters_neuron(component, &error_type);
    //network.get_layer_mut(1).unwrap().get_neuron_mut(0).unwrap().modify_parameters_neuron(component_2, &error_type_2);
    network.get_layer_mut(1).unwrap().get_neuron_mut(0).unwrap().print_neuron_parameters();


    error_menu();
}

fn create_spike(file_path: &str) -> Vec<Vec<Spike>> {
    // Open the file
    let file = File::open(file_path).expect("File non trovato");
    let reader = io::BufReader::new(file);

    // Read lines and parse into Vec<Vec<usize>>
    let result: Vec<Vec<u128>> = reader.lines()
        .filter_map(|line| line.ok())
        .map(|line| {
            line.split(';')
                .flat_map(|part| part.split(','))
                .filter_map(|num| num.trim().parse().ok())
                .collect()
        })
        .collect();

    // Call Spike::create_spike_vec for each Vec<usize>
    let spikes: Vec<Vec<Spike>> = result.into_iter().enumerate().map(|(idx, vec)| {
        Spike::create_spike_vec(idx, 0, vec)
    }).collect();

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


fn error_menu() {
    // Lista di nomi
    let errors = vec!["stuck-at-0", "stuck-at-1", "bit-flip"];
    let positions = vec!["Threshold", "ResetPotential","RestingPotential","MembranePotential","Tau"];
    
    // Chiedere all'utente il numero di iterazioni
    println!("Inserisci il numero di occorrenze:");
    let mut num_occurrences = String::new();
    io::stdin().read_line(&mut num_occurrences).expect("Errore durante la lettura dell'input");
    let iterazioni: usize = num_occurrences.trim().parse().expect("Inserisci un numero valido");

    // Stampare la lista di errori
    println!("Lista di errori disponibili:");
    for (idx, error) in errors.iter().enumerate() {
        println!("{}: {}", idx + 1, error);
    }

    let error_choice;
    loop{
    // Chiedere all'utente di scegliere un valore dalla lista di errori
    println!("Scegli un valore dalla lista (inserisci il numero corrispondente):");
    let mut input_choice = String::new();
    io::stdin().read_line(&mut input_choice).expect("Errore durante la lettura dell'input");
    let choice: usize = input_choice.trim().parse().expect("Inserisci un numero valido");

    // Verificare se la scelta è valida
    
    if choice >= 1 && choice <= errors.len() {
        error_choice = errors[choice - 1];
        println!("Scelto: {}", error_choice);
        break;
    } else {
        println!("Scelta non valida. Riprova.");
        continue;
    }
}
    // Stampare la lista di posizioni
    println!("Lista di errori disponibili:");
    for (idx, position) in positions.iter().enumerate() {
        println!("{}: {}", idx + 1, position);
    }
    
    let mut position_choices = Vec::new();
    loop {
    println!("Scegli un valore dalla lista (inserisci il numero corrispondente, inserisci 0 per terminare):");
        let mut input_choice = String::new();
        io::stdin().read_line(&mut input_choice).expect("Errore durante la lettura dell'input");

        // Converte l'input in un numero
        let choice: usize = match input_choice.trim().parse() {
            Ok(num) => num,
            Err(_) => {
                println!("Inserisci un numero valido.");
                continue;
            }
        };

        // Verifica se l'utente ha inserito 0 per terminare
        if choice == 0 {
            break;
        }

        // Verifica se la scelta è valida e pusha la scelta nel vettore
        if choice >= 1 && choice <= positions.len() {
            let position_choice = positions[choice - 1];
            println!("Scelto: {}", position_choice);
            position_choices.push(position_choice);
        } else {
            println!("Scelta non valida. Riprova.");
        }
    }

    let components: Vec<Component> = position_choices
        .iter()
        .filter_map(|&nome|  SimulationError::string_to_component(nome))
        .collect();

    let mut simulation = SimulationError::new(components, error_choice, iterazioni);
    
    let input_weights_file = "data/input_weights_222.txt";
    let intra_weights_file = "data/intra_weights_222.txt";

    let input_weights = read_matrix_file(input_weights_file).expect("Errore durante la lettura del file di input_weights");
    let intra_weights = read_matrix_file(intra_weights_file).expect("Errore durante la lettura del file di intra_weights");

   

    //Creazione rete neurale
    let layer_sizes = vec![2,2,2];
    let num_layers: usize = 3;
    let neuron_params = LIFNeuron::default();
    // Leggi le spike da file
    let spike_file = "data/spike2.txt";
    let spikes = create_spike(spike_file);

    
    simulation.run_simulation_wrapper(layer_sizes,num_layers, input_weights, intra_weights,neuron_params, spikes);
    simulation.print_info();

}

pub fn generate_spike_file(n: usize) -> &'static str {
    // Apre o crea il file "output.txt"
    let file_path = "spike.txt";
    let mut file = File::create(file_path).expect("Errore durante la creazione del file");

    for i in 0..n {
        // Richiede all'utente di inserire la riga di numeri
        println!("Inserisci la riga di numeri separati da virgole (Treno di spike #{}):", i+1);
        let mut input_line = String::new();
        io::stdin().read_line(&mut input_line).expect("Errore durante la lettura dell'input");

        // Scrive la riga nel file
        write!(file, "{}", input_line.trim()).expect("Errore durante la scrittura nel file");
        write!(file, ";\n").expect("Errore durante la scrittura nel file");
    }

    file_path
}