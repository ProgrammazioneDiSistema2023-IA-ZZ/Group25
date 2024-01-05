// lif_neuron.rs

pub trait Neuron: 'static + Clone {
    type ClassNeuron: 'static + Sized + Clone + Sync;}

pub struct LIFNeuron {
    membrane_potential: f64,
    reset_potential: f64,
    resting_potential: f64,
    threshold: f64,
    tau: f64
}

impl LIFNeuron {
    pub fn new(
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
        }
    }
}

#[derive(Clone, Copy, Debug)]
pub struct LeakyIntegrateFire;

impl Neuron for LeakyIntegrateFire {
    type ClassNeuron = LIFNeuron;}
