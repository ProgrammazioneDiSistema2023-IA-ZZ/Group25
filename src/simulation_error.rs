use std::collections::HashMap;
use std::fmt;

#[derive(Debug)]
pub struct SimulationError {
    pub components: Vec<String>,
    pub error_type: ErrorType,
    pub occurrences: usize,
    pub output: Vec<Vec<u64>>,
    pub output_errors: Vec<Vec<u64>>,
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
            output_errors,
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
    
        // Stampa il contenuto di self.output_errors
        println!("Output Errors:");
        for row in &self.output_errors {
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
