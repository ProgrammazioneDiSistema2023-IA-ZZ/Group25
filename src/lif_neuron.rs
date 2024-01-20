// lif_neuron.rs
use std::io;
use rand::Rng;

use crate::{
    simulation_error::{Component, ErrorType},
    errors::modify_weight_based_on_error,
};

const RESET_POTENTIAL: f64 = 0.7;
const RESTING_POTENTIAL: f64 = 2.0;
const THRESHOLD: f64 = 2.5;
const TAU: f64 = 1.0;

pub trait Neuron: 'static + Clone + Send {
    type ClassNeuron: 'static + Sized + Clone + Sync + Send;

    fn handle_spike(&mut self, sum: f64, current_spike_time: u128) -> u128;
    fn adjust_weight(&mut self, input: f64);
}

#[derive(Clone,Copy, Debug)]
pub struct Error {
    // struttura gestione errore
    pub flag: bool,
    pub error_type: ErrorType,
    pub index: Option<usize>,
}

#[derive(Clone, Copy, Debug)]
pub struct LIFNeuron {
    pub membrane_potential: f64,
    pub reset_potential: f64,
    pub resting_potential: f64,
    pub threshold: f64,
    pub tau: f64,
    pub last_spike_time: u128,
    pub error: Error,
}

impl LIFNeuron {
    pub fn new(reset_potential: f64, resting_potential: f64, threshold: f64, tau: f64) -> Self {
        Self {
            membrane_potential: resting_potential,
            reset_potential,
            resting_potential,
            threshold,
            tau,
            last_spike_time: 0,
            error: Error {
                flag: false,
                error_type: ErrorType::StuckAt0, // Inizializza con un valore di default
                index: None,
            },
        }
    }

    pub fn default() -> Self {
        LIFNeuron {
            membrane_potential: RESTING_POTENTIAL,
            reset_potential: RESET_POTENTIAL,
            resting_potential: RESTING_POTENTIAL,
            threshold: THRESHOLD,
            tau: TAU,
            last_spike_time: 0,
            error: Error {
                flag: false,
                error_type: ErrorType::StuckAt0, // Inizializza con un valore di default
                index: None,
            },
        }
    }

    pub fn default_random() -> Self {
        let mut rng = rand::thread_rng();

        LIFNeuron {
            membrane_potential: RESTING_POTENTIAL + rng.gen_range(-0.5..0.5),
            reset_potential: RESET_POTENTIAL + rng.gen_range(-0.5..0.5),
            resting_potential: RESTING_POTENTIAL + rng.gen_range(-0.5..0.5),
            threshold: THRESHOLD + rng.gen_range(-0.5..0.5),
            tau: TAU + rng.gen_range(-0.5..0.5),
            last_spike_time: 0,
            error: Error {
                flag: false,
                error_type: ErrorType::StuckAt0, // Inizializza con un valore di default
                index: None,
            },
        }
    }

    // Funzione di supporto per leggere l'input utente
    fn read_user_input() -> f64 {
        let mut input = String::new();
        io::stdin()
            .read_line(&mut input)
            .expect("Errore durante la lettura dell'input");
        input.trim().parse().expect("Impossibile convertire l'input in f64")
    }

    // Metodo per chiedere all'utente di inserire i valori
    pub fn from_user_input() -> Self {
        println!("Inserisci i valori del neurone:");

        println!("Reset Potential:");
        let reset_potential: f64 = Self::read_user_input();

        println!("Resting Potential:");
        let resting_potential: f64 = Self::read_user_input();

        println!("Threshold:");
        let threshold: f64 = Self::read_user_input();

        println!("Tau:");
        let tau: f64 = Self::read_user_input();

        //Inserire errori(?)

        LIFNeuron {
            membrane_potential: resting_potential,
            reset_potential,
            resting_potential,
            threshold,
            tau,
            last_spike_time: 0,
            error: Error {
                flag: false,
                error_type: ErrorType::StuckAt0, // Inizializza con un valore di default
                index: None,
            },
        }
    }

    // Funzione per ottenere un riferimento mutabile alla struttura Error
    pub fn get_error_mut(&mut self) -> &mut Error {
        &mut self.error
    }
    //gestione errori
        //gestione errori

    // Funzione per modificare la struttura Error
    pub fn modify_error(error: &mut Error, error_type: &ErrorType, index: Option<usize>) {
        error.flag = true;
        error.error_type = *error_type;
        error.index = index;
    }
   
   

    pub fn modify_parameters_neuron(&mut self, component: Component, error_type: &ErrorType) {
        let mut index: Option<usize> = None;
        match component {
            Component::Threshold => {
                index = modify_weight_based_on_error(&mut self.threshold, error_type);
            }
            Component::ResetPotential => {
                index = modify_weight_based_on_error(&mut self.reset_potential, error_type);
            }
            Component::RestingPotential => {
                index = modify_weight_based_on_error(&mut self.resting_potential, error_type);
            }
            Component::MembranePotential => {
                index = modify_weight_based_on_error(&mut self.membrane_potential, error_type);
            }
            Component::Tau => {
                index = modify_weight_based_on_error(&mut self.tau, error_type);
            }
        }
        LIFNeuron::modify_error( &mut self.error, error_type, index);
    }

    pub fn print_neuron_parameters(&self) {
        println!("Neuron Parameters:");
        println!("Membrane Potential: {:.14}", self.membrane_potential);
        println!("Reset Potential: {:.14}", self.reset_potential);
        println!("Resting Potential: {:.14}", self.resting_potential);
        println!("Threshold: {:.14}", self.threshold);
        println!("Tau: {:.14}", self.tau);
        println!("Last Spike Time: {}", self.last_spike_time);
        println!("Error: {:?}", self.error);
    }
}

impl Neuron for LIFNeuron {
    type ClassNeuron = LIFNeuron;

    fn handle_spike(&mut self, mut sum: f64, current_spike_time: u128) -> u128 {
        // This early exit serves as a small optimization
        if sum == 0.0 {
            return 0;
        }

        //println!("last spike time: {:.3}", self.last_spike_time);
        let delta_t = (current_spike_time - self.last_spike_time) as f64;
        //println!("delta_t: {:.3}", delta_t);
        self.last_spike_time = current_spike_time;

        // compute the new v_mem value
        //println!("Potenziale prima: {:.3}", self.membrane_potential);
        let expo = (-delta_t / self.tau).exp();
        //println!("expo: {:.3}", expo);
        let intermediate = (self.membrane_potential - self.resting_potential) * expo;
        //println!("mult+exp: {:.3}", intermediate);
        self.membrane_potential = self.resting_potential + intermediate + sum;
        //println!("Potenziale dopo: {:.3}", self.membrane_potential);
        sum = 0.0;
        if self.membrane_potential > self.threshold {
            self.membrane_potential = self.reset_potential;
            //println!("Potenziale dopo threshold: {:.3}", self.membrane_potential);
            1
        } else {
            //println!("------------------> Potenziale dopo threshold: {:.3}", self.membrane_potential);
            0
        }
    }

    fn adjust_weight(&mut self, input: f64) {
        self.membrane_potential = self.membrane_potential + input;
    }
}

 
