use std::collections::HashMap;
use std::fmt;

#[derive(Debug)]
pub struct SimulationError {
    pub components: Vec<String>,
    pub error_type: ErrorType,
    pub occurrences: usize,
}

#[derive(Debug, PartialEq, Clone, Copy)]
pub enum ErrorType {
    StuckAt0,
    StuckAt1,
    BitFlip,
}

// Enum per rappresentare i tipi di componenti


#[derive(Clone, Copy, Debug, PartialEq)]
pub enum Component {
    Threshold,
    ResetPotential,
    RestingPotential,
    MembranePotential,
    Tau,
}

impl fmt::Display for Component {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{:?}", self)
    }
}

impl SimulationError {
    pub fn new(components: Vec<Component>, error_type: &str, occurrences: usize) -> Self {
        let components = components.iter().map(|c| c.to_string()).collect();
        let error_type = match error_type.to_lowercase().as_str() {
            "stuck-at-0" => ErrorType::StuckAt0,
            "stuck-at-1" => ErrorType::StuckAt1,
            "bit-flip" => ErrorType::BitFlip,
            _ => panic!("Invalid error type"),
        };

        Self {
            components,
            error_type,
            occurrences,
        }
    }

    pub fn print_info(&self) {
        println!("Error Type: {:?}", self.error_type);
        println!("Occurrences: {}", self.occurrences);
        println!("Components:");
        for component in &self.components {
            println!("  - {}", component);
        }
    }

    pub fn string_to_component(nome: &str) -> Option<Component> {
        match nome {
            "Threshold" => Some(Component::Threshold),
            "ResetPotential" => Some(Component::ResetPotential),
            "RestingPotential" => Some(Component::RestingPotential),
            "MembranePotential" => Some(Component::MembranePotential),
            "Tau" => Some(Component::Tau),
            _ => None,
        }
    }
}

#[test]
fn main() {
    // Creazione di una lista di componenti
    let components_list = vec![
        Component::Threshold,
        Component::ResetPotential,
        Component::RestingPotential,
        Component::MembranePotential,
        Component::Tau,
    ];

    // Creazione di un'istanza di SimulationError
    let simulation_error = SimulationError::new(components_list, "bit-flip", 3);

    // Stampa a scopo di debug
    simulation_error.print_info();
}
