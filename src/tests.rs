// tests/test.rs

use crate::lif_neuron::LIFNeuron;
use crate::neural_layer::NeuralLayer;
use crate ::neural_network::NeuralNetwork;

const RESET_POTENTIAL: f64 = 0.7;
const RESTING_POTENTIAL: f64 = 2.0;
const THRESHOLD: f64 = 2.5;
const TAU: f64 = 1.0;

//mod lif_neuron;

//use lif_neuron::LIFNeuron;


fn test_lif_neuron_creation(neuron: LIFNeuron) {
    // Assert per verificare che i valori siano quelli attesi
    assert_eq!(neuron.reset_potential, RESET_POTENTIAL);
    assert_eq!(neuron.resting_potential, RESTING_POTENTIAL);
    assert_eq!(neuron.threshold, THRESHOLD);
    assert_eq!(neuron.tau, TAU);
}


fn common_test_neural_network_configuration(
    num_neurons_for_layer: Vec<usize>,
    input_weights: Vec<Vec<Vec<f64>>>,
    intra_weights: Vec<Vec<Vec<f64>>>,
    neuron_params: LIFNeuron,
) {

    test_lif_neuron_creation(neuron_params);

    let num_layers = num_neurons_for_layer.len();
    // Verifica che num_layers sia maggiore o uguale a 3
    assert!(num_layers >= 3, "Il numero di layer deve essere maggiore o uguale a 3");

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


#[test]
fn test_neural_network_configuration() {


    let neuron_params = LIFNeuron::default();

    test_lif_neuron_creation(neuron_params);

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

    common_test_neural_network_configuration(
        vec![3, 4, 3], input_weights, intra_weights, neuron_params
    );
}


#[test]
fn test_neural_network_configuration_2() {

    let neuron_params = LIFNeuron::default();

    test_lif_neuron_creation(neuron_params);

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
    common_test_neural_network_configuration(
        vec![2, 2, 2], input_weights, intra_weights, neuron_params
    );
}







// #[test]
// fn test_neural_network_propagation() {
//     // ... test di propagazione degli impulsi nella rete ...
// }

// #[test]
// fn test_neural_network_training() {
//     // ... test di addestramento della rete ...
// }
