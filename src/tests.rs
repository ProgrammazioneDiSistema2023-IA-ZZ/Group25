// tests/test.rs

use crate::lif_neuron::LIFNeuron;
use crate::neural_layer::NeuralLayer;
use crate ::neural_network::NeuralNetwork;
//mod lif_neuron;

//use lif_neuron::LIFNeuron;

#[test]
fn test_lif_neuron_creation() {
    const RESET_POTENTIAL: f64 = 0.7;
    const RESTING_POTENTIAL: f64 = 2.0;
    const THRESHOLD: f64 = 2.5;
    const TAU: f64 = 1.0;

    // Creazione di un neurone utilizzando i valori di default
    let neuron_params = LIFNeuron::new(RESET_POTENTIAL, RESTING_POTENTIAL, THRESHOLD, TAU);

    // Assert per verificare che i valori siano quelli attesi
    assert_eq!(neuron_params.reset_potential, RESET_POTENTIAL);
    assert_eq!(neuron_params.resting_potential, RESTING_POTENTIAL);
    assert_eq!(neuron_params.threshold, THRESHOLD);
    assert_eq!(neuron_params.tau, TAU);
}


#[test]
fn test_neural_network_configuration() {
    // Configurazione della rete neurale
    const RESET_POTENTIAL: f64 = 0.7;
    const RESTING_POTENTIAL: f64 = 2.0;
    const THRESHOLD: f64 = 2.5;
    const TAU: f64 = 1.0;
    let neuron_params = LIFNeuron::new(RESET_POTENTIAL, RESTING_POTENTIAL, THRESHOLD, TAU);

    // Assert per verificare che i valori siano quelli attesi
    assert_eq!(neuron_params.reset_potential, RESET_POTENTIAL);
    assert_eq!(neuron_params.resting_potential, RESTING_POTENTIAL);
    assert_eq!(neuron_params.threshold, THRESHOLD);
    assert_eq!(neuron_params.tau, TAU);

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

    let num_layers = 3;
    // Verifica che num_layers sia maggiore o uguale a 3
    assert!(num_layers >= 3, "Il numero di layer deve essere maggiore o uguale a 3");
    let num_neurons_for_layer= vec![3,4,3].clone();

    // Configurazione della rete neurale
    let mut network = NeuralNetwork::new(num_neurons_for_layer.clone(), input_weights, intra_weights, neuron_params);


   // Verifica che la configurazione sia corretta
for (layer_index, &expected_neurons) in num_neurons_for_layer.iter().enumerate() {
    assert_eq!(
        network.get_layer(layer_index).unwrap().num_neurons(),
        expected_neurons
    );
}


}

// #[test]
// fn test_neural_network_propagation() {
//     // ... test di propagazione degli impulsi nella rete ...
// }

// #[test]
// fn test_neural_network_training() {
//     // ... test di addestramento della rete ...
// }
