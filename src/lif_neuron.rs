// lif_neuron.rs
use std::io;

const RESET_POTENTIAL: f64 = 0.7;
const RESTING_POTENTIAL: f64 = 2.0;
const THRESHOLD: f64 = 2.5;
const TAU: f64 = 1.0;

pub trait Neuron: 'static + Clone {
    type ClassNeuron: 'static + Sized + Clone + Sync + Send;

    fn put_sum(&mut self, value: f64);
    fn handle_spike(&mut self, current_spike_time: u128) -> u128;
}


#[derive(Clone, Copy, Debug)]
pub struct LIFNeuron {
    pub membrane_potential: f64,
    pub reset_potential: f64,
    pub resting_potential: f64,
    pub threshold: f64,
    pub tau: f64,
    pub last_spike_time: u128,
    pub sum: f64
}

impl LIFNeuron {
    /* pub fn new(
        reset_potential: f64,
        resting_potential: f64,
        threshold: f64,
        tau: f64
    ) -> Self {
        Self {
            membrane_potential: resting_potential,
            reset_potential,
            resting_potential,
            threshold,
            tau,
            last_spike_time : 0,
            sum: 0.0
        }
    } */

    // Metodo per creare una nuova istanza con valori di default
    pub fn default() -> Self {
        LIFNeuron {
            membrane_potential: RESTING_POTENTIAL,
            reset_potential: RESET_POTENTIAL,
            resting_potential: RESTING_POTENTIAL,
            threshold: THRESHOLD,
            tau: TAU,
            last_spike_time : 0,
            sum: 0.0
        }
    }

    
    // Funzione di supporto per leggere l'input utente
    fn read_user_input() -> f64 {
        let mut input = String::new();
        io::stdin().read_line(&mut input).expect("Errore durante la lettura dell'input");
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

        LIFNeuron {
            membrane_potential: resting_potential,
            reset_potential,
            resting_potential,
            threshold,
            tau,
            last_spike_time : 0,
            sum: 0.0
        }
    }
}

impl Neuron for LIFNeuron {
    type ClassNeuron = LIFNeuron;

    // Metodo per aggiungere un valore a sum
    fn put_sum(&mut self, value: f64) {
        self.sum += value;
    }

    fn handle_spike(&mut self, current_spike_time: u128) -> u128 {
        // This early exit serves as a small optimization
        if self.sum == 0.0 { return 0 }
        
        let delta_t = (current_spike_time - self.last_spike_time)as f64;
        self.last_spike_time = current_spike_time;

        // compute the new v_mem value
        self.membrane_potential = self.resting_potential + (self.membrane_potential - self.resting_potential) * (-delta_t / self.tau).exp() + self.sum;
        self.sum = 0.0;
        if self.membrane_potential > self.threshold {
            self.membrane_potential = self.reset_potential;
            1
        } else {
            0
        }
    }
}
