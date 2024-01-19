use std::collections::HashMap;

#[derive(Debug)]
pub struct SimulationError {
    pub components: Vec<String>,
    pub error_type: ErrorType,
    pub occurrences: usize,
}

#[derive(Debug, PartialEq)]  // Aggiunto #[derive(PartialEq)]
pub enum ErrorType {
    StuckAt0,
    StuckAt1,
    BitFlip,
}

impl SimulationError {
    pub fn new(components: Vec<&str>, error_type: &str, occurrences: &usize) -> Self {
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
            occurrences: *occurrences,
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
}


