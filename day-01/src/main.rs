// Load Local Modules
mod read;

// Imports
use day_01::retrieve_calibration_value;


fn main() {

    let input = read::read_chars("input.txt".to_owned());
    
    // Part 1
    let sum_calibration: u32 = input.clone().into_iter()
        .map(|line| retrieve_calibration_value(line, false))
        .filter_map(|value_option| value_option.map(|value| value.get_value()))
        .sum();
    println!("\r📐 Sum of calibration values: '{}' (Part 1)", sum_calibration);
    
    // Part 2
    let sum_calibration_fixed: u32 = input.clone().into_iter()
        .map(|line| retrieve_calibration_value(line, true))
        .filter_map(|value_option| value_option.map(|value| value.get_value()))
        .sum();
    println!("\r📐 Sum of calibration values parsed: '{}' (Part 2)", sum_calibration_fixed);
}