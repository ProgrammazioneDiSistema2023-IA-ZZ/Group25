use std::collections::HashMap;
use std::fmt;

use rand::Rng;

use crate::LIFNeuron;
use crate::neural_network::NeuralNetwork;
use crate::spike::{Spike, action_spike};

#[derive(Debug)]
pub struct SimulationError {
    pub components: Vec<String>,
    pub error_type: ErrorType,
    pub occurrences: usize,
    pub output: Vec<Vec<f64>>,
    pub output_errors: Vec<Vec<f64>>,
}

#[derive(Debug, PartialEq, Clone, Copy)]
pub enum ErrorType {
    StuckAt0,
    StuckAt1,
    BitFlip,
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub enum Component {
    Threshold,
    ResetPotential,
    RestingPotential,
    MembranePotential,
    Tau,
    Weights,
}

impl fmt::Display for Component {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{:?}", self)
    }
}

impl SimulationError {
    pub fn new(components: Vec<Component>, error_type: &str, occurrences: usize) -> Self {
        let components = components.into_iter().map(|c| c.to_string()).collect();
        let error_type = match error_type.to_lowercase().as_str() {
            "stuck-at-0" => ErrorType::StuckAt0,
            "stuck-at-1" => ErrorType::StuckAt1,
            "bit-flip" => ErrorType::BitFlip,
            _ => panic!("Invalid error type"),
        };

        let output_errors = vec![vec![0u64; 64]; occurrences]; //da rivedere
        Self {
            components,
            error_type,
            occurrences,
            output: Vec::new(),
            output_errors: Vec::new(),
        }
    }

    pub fn print_info(&self) {
        println!("Error Type: {:?}", self.error_type);
        println!("Occurrences: {}", self.occurrences);
        println!("Components:");
        for component in &self.components {
            println!("  - {}", component);
        }
    
        // Stampa il contenuto di self.output
        println!("Output:");
        for row in &self.output {
            println!("  {:?}", row);
        }
    
        let mut output_sim_counter = 0;

        // Stampa il contenuto di self.output_errors
        println!("Output Errors:");
           

        for row in &self.output_errors {
            

          // Stampa "output sim x" ogni 10 stampe di row
        if output_sim_counter % 10 == 0 {
            println!("\n\n\n");
            println!("output sim {}", output_sim_counter / 10 +1 ); 
        }   
            output_sim_counter += 1;

            println!("  {:?}", row);
        }
    }
        
    
    

    pub fn string_to_component(nome: &str) -> Option<Component> {
        match nome.to_lowercase().as_str() {
            "threshold" => Some(Component::Threshold),
            "resetpotential" => Some(Component::ResetPotential),
            "restingpotential" => Some(Component::RestingPotential),
            "membranepotential" => Some(Component::MembranePotential),
            "tau" => Some(Component::Tau),
            "weights" => Some(Component::Weights),
            _ => None,
        }
    }

    pub fn run_simulation_wrapper(
        &mut self,
        layer_sizes: Vec<usize>,
        num_layers: usize,
        input_weights: Vec<Vec<Vec<f64>>>,
        intra_weights: Vec<Vec<Vec<f64>>>,
        neuron_params: LIFNeuron,
        spikes: Vec<Vec<Spike>>,
    ) {
        // Esegui run_simulation una volta
        self.run_simulation(
            layer_sizes.clone(),
            num_layers,
            input_weights.clone(),
            intra_weights.clone(),
            neuron_params.clone(),
            spikes.clone(),
        );
    
        // Esegui run_simulation_error num_occurrences volte
        for _ in 0..self.occurrences {
            let mut rng = rand::thread_rng();
            let comp_index = rng.gen_range(0..self.components.len());
            let selected_component = SimulationError::string_to_component(&self.components[comp_index]);
            self.run_simulation_with_error(
                layer_sizes.clone(),
                num_layers,
                input_weights.clone(),
                intra_weights.clone(),
                neuron_params.clone(),
                spikes.clone(),
                selected_component,
            );
        }
    }
    

    pub fn run_simulation(
        &mut self,
        layer_sizes: Vec<usize>,
        num_layers: usize,
        input_weights: Vec<Vec<Vec<f64>>>,
        intra_weights: Vec<Vec<Vec<f64>>>,
        neuron_params: LIFNeuron,
        spikes: Vec<Vec<Spike>>,
    ) {
        // Creazione rete neurale
        let mut network = NeuralNetwork::new(layer_sizes, input_weights, intra_weights, neuron_params);

        let sorted_spike_array_for_nn = Spike::get_all_spikes(spikes.clone());
        let max_value = *sorted_spike_array_for_nn.iter().max().unwrap();
        let mut time = 0;

        while time < max_value {
            // Incrementa il contatore
            time += 1;
            let mut s = vec![0.0, 0.0];

            if sorted_spike_array_for_nn.contains(&time) {
                s = action_spike(spikes.clone(), time);
            }
            
            // Ciclo sui neuroni per calcolo soglia
            println!("TIME----------------------> {:?}", time);
            println!("PRE----> {:?}", s);
            s = network.update_neurons_parallel(time, s, num_layers);
            self.output.push(s.clone());
            println!("POST----> {:?}", s);
        }

        println!("OUTPUT: {:?}", self.output);
        println!("Condizione raggiunta dopo {} iterazioni", time);
    }


    pub fn run_simulation_with_error(
        &mut self,
        layer_sizes: Vec<usize>,
        num_layers: usize,
        input_weights: Vec<Vec<Vec<f64>>>,
        intra_weights: Vec<Vec<Vec<f64>>>,
        neuron_params: LIFNeuron,
        spikes: Vec<Vec<Spike>>,
        component: Option<Component>,
    ) {
        // Creazione rete neurale
        let mut network = NeuralNetwork::new(layer_sizes, input_weights, intra_weights, neuron_params);
    
        let sorted_spike_array_for_nn = Spike::get_all_spikes(spikes.clone());
        let max_value = *sorted_spike_array_for_nn.iter().max().unwrap();
        let mut time = 0;
    
        while time < max_value {
            // Incrementa il contatore
            time += 1;
            let mut s = vec![0.0, 0.0];
    
            if sorted_spike_array_for_nn.contains(&time) {
                

                s = action_spike(spikes.clone(), time);
            }
            
            // Ciclo sui neuroni per calcolo soglia
            println!("TIME----------------------> {:?}", time);
            println!("PRE----> {:?}", s);
            //CHIAMA ERRORE, USA FUNZIONE APPLY_ERROR SU NEURAL NETWORK
            network.apply_error(component, self.error_type);
            s = network.update_neurons_parallel(time, s, num_layers);
            self.output_errors.push(s.clone());
            println!("POST----> {:?}", s);
        }
    

        println!("OUTPUT: {:?}", self.output);
        println!("Condizione raggiunta dopo {} iterazioni", time);
    }
    
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_simulation_error() {
        let components_list = vec![
            Component::Threshold,
            Component::ResetPotential,
            Component::RestingPotential,
            Component::MembranePotential,
            Component::Tau,
            Component::Weights,
        ];

        let simulation_error = SimulationError::new(components_list, "bit-flip", 3);

        assert_eq!(simulation_error.occurrences, 3);
        assert_eq!(simulation_error.error_type, ErrorType::BitFlip);
        assert_eq!(simulation_error.components.len(), 6);
    }
}
