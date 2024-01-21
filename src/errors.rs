use rand::Rng;
use crate::simulation_error::ErrorType;

pub fn modify_weight_based_on_error(original_value: &mut f64, error_type: &ErrorType) -> Option<usize> {
    match error_type {
        ErrorType::StuckAt0 => stuck_at_0(original_value),
        ErrorType::StuckAt1 => stuck_at_1(original_value),
        ErrorType::BitFlip => bit_flip(original_value),
    }
}

pub fn stuck_at_x(value: &mut f64, index: usize, new_bit: u8) -> Option<usize> {
    // Convert f64 to byte array
    let mut value_bytes: [u8; 8] = value.to_ne_bytes();

    // Ensure the index is within bounds
    if index < 64 {
        // Calculate the byte index and bit index within the byte
        let byte_index = index / 8;
        let bit_index = index % 8;

        // Create a mask with a 1 at the bit position
        let bit_mask = 1 << bit_index;

        // Clear the specific bit
        value_bytes[byte_index] &= !bit_mask;

        // Set the new bit
        value_bytes[byte_index] |= (new_bit & 1) << bit_index;

        // Convert byte array back to f64
        *value = f64::from_ne_bytes(value_bytes);
        
        println!("valore aggiornato {:.14} ", value);
        // Return the modified bit index
        Some(index)
    } else {
        None
    }
}

pub fn stuck_at_0(original_value: &mut f64) -> Option<usize> {
    // Generate a random bit index to toggle (0 to 63)
    let bit_index_to_toggle = rand::thread_rng().gen_range(0..64);
    println!("valore {:.14} all'indice {}", original_value, bit_index_to_toggle);
    // Toggle the bit using the byte array with the value 1
    stuck_at_x(original_value, bit_index_to_toggle, 0)
}

pub fn stuck_at_1(original_value: &mut f64) -> Option<usize> {
    // Generate a random bit index to toggle (0 to 63)
    let bit_index_to_toggle = rand::thread_rng().gen_range(0..64);
    println!("valore {:.14} all'indice {}", original_value, bit_index_to_toggle);
    // Toggle the bit using the byte array with the value 1
    stuck_at_x(original_value, bit_index_to_toggle, 1)
}

pub fn bit_flip(value: &mut f64) -> Option<usize> {
    // Generate a random bit index to toggle (0 to 63)
    let index = rand::thread_rng().gen_range(0..64);
    println!("valore {:.14} all'indice {}", value, index);

    // Toggle the bit using the byte array with the value 1
    stuck_at_x(value, index, 1)
}


#[test]
fn test_bit_flip() {
    // Test case 1: Flip the least significant bit
    let mut value = 42.0;
    bit_flip(&mut value);
    println!("{}",value);
    assert_ne!(value, 42.0 ); // 42.0 + 2^(-52)

    // // Test case 2: Flip a bit in the middle
    // let mut value = 42.0;
    // bit_flip(&mut value, 30);
    // assert_ne!(value, 42.0 + 0.00000024); // 42.0 + 2^(-22)

    // Test case 3: Flip the most significant bit
    let mut value = 42.0;
    bit_flip(&mut value);
    println!("{}",value);
    assert_ne!(value, 42.0); // -2^62 (due a causa della rappresentazione a complemento a due)

    // Test case 4: Index out of bounds (no modification should occur)
    let mut value = 42.0;
    bit_flip(&mut value);
    println!("{}",value);
    assert_ne!(value, 42.0); // Value should remain unchanged
}

