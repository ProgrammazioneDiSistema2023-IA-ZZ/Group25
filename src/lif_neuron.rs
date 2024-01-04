// lif_neuron.rs

pub trait Neuron: 'static + Debug + Clone {
    type ClassNeuron: 'static + Sized + Clone + Sync + RefInto<Self::SolverVars>;}

pub struct LIFNeuron {
    membrane_potential: f64,
    reset_potential: f64,
    resting_potential: f64,
    threshold: f64,
}

impl LIFNeuron {
    pub fn new(
        reset_potential: f64,
        resting_potential: f64,
        threshold: f64
    ) -> Self {
        Self {
            membrane_potential: resting_potential,
            reset_potential,
            resting_potential,
            threshold
        }
    }

#[derive(Clone, Copy, Debug)]
pub struct LeakyIntegrateFire;

impl Neuron for LeakyIntegrateFire {
    type ClassNeuron = LIFNeuron;}
    
    pub fn integrate(&mut self, input_spikes: &[T], time_step: f64) -> bool {
        // Logica di integrazione qui
        // Aggiorna il potenziale di membrana in base agli input e al tempo

        //cambiare synaptic_weights, perchè sara una matrice fuori
        let weighted_sum: f64 = input_spikes.iter().zip(&self.synaptic_weights).map(|(x, w)| *x * *w).sum();
        self.membrane_potential = self.membrane_potential * self.decay_factor + weighted_sum;

        // Verifica se il neurone ha superato la soglia e restituisce true in caso di fuoco
        if self.membrane_potential >= self.threshold {
            self.membrane_potential = self.reset_potential; // Reset del potenziale di membrana dopo lo scoppio
            return true;
        }

        false
    }
}
