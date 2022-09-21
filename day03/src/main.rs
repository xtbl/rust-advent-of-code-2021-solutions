// --- Day 3: Binary Diagnostic ---
// https://adventofcode.com/2021/day/3




use std::fs::File;
use std::io::{BufRead, BufReader};
use std::path::Path;
use std::io::Result;


fn main() {
    println!("Binary Diagnostic");
}

fn get_lines_from_file(filename: impl AsRef<Path>) -> Result<Vec<String>> {
    BufReader::new(File::open(filename)?).lines().collect()
}

fn get_all_bits_at_position(bit_list: &Vec<String>, position: usize) -> Vec<i32> {
    let mapped_bits:Vec<i32> = bit_list.iter()
        .map(|x| x.chars().nth(position).unwrap_or('0').to_string().parse::<i32>().unwrap_or(0))
        .collect();
    println!("mapped_bits {:?}", mapped_bits);
    mapped_bits
}


fn get_rate_for_all_positions(bit_list: &Vec<String>, is_gamma: bool) -> i32 {
    let selected_line = &bit_list[0 as usize];
    let column_amount = selected_line.chars().count();
    let mapped_gamma: Vec<i32> = (0..column_amount).map(|x | {
        let bits_at_position = get_all_bits_at_position(&bit_list, x);
        let rate_ints = if is_gamma {get_gamma(&bits_at_position)} else {get_epsilon(&bits_at_position)} ;
        rate_ints
    }).collect();
    let vec_to_binary: Vec<String> = mapped_gamma.iter().map(|x| x.to_string()).collect();
    let join_binary_string = vec_to_binary.join("");
    println!("mapped_gamma is: {:?}", mapped_gamma);
    let into_decimal = i32::from_str_radix(&join_binary_string, 2).unwrap_or(0);
    into_decimal
}

fn calculate_power_comsumption(gamma: i32, epsilon: i32) -> i32 {
    gamma * epsilon
}

fn get_gamma(bits_at_position: &Vec<i32>) -> i32 {
    let filter_0: Vec<&i32> = bits_at_position.iter().filter(|x| *x == &0 as &i32).collect();
    let filter_1: Vec<&i32> = bits_at_position.iter().filter(|x| *x == &1 as &i32).collect();
    println!("filter_0: {:?}", filter_0);
    println!("filter_1: {:?}", filter_1);
    if filter_0.iter().count() >  filter_1.iter().count(){
       0
    } else {
       1
    }
}

fn get_epsilon(bits_at_position: &Vec<i32>) -> i32 {
    let filter_0: Vec<&i32> = bits_at_position.iter().filter(|x| *x == &0 as &i32).collect();
    let filter_1: Vec<&i32> = bits_at_position.iter().filter(|x| *x == &1 as &i32).collect();
    if filter_0.iter().count() >  filter_1.iter().count(){
       1
    } else {
       0
    }
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_get_lines_from_file() {
        let lines_file = get_lines_from_file("mock_input.txt");
        let lines = lines_file.unwrap(); 
        let selected_line = &lines[1 as usize];
        assert_eq!(selected_line, "11110");
    }

    #[test]
    fn test_get_all_bits_at_position() {
        let lines_file = get_lines_from_file("mock_input.txt");
        let lines = lines_file.unwrap(); 
        let bits_at_position = get_all_bits_at_position(&lines, 0);
        assert_eq!(bits_at_position,[0,1,1,1,1,0,0,1,1,1,0,0])        
    }

    #[test]
    fn test_get_gamma() {
        let lines_file = get_lines_from_file("mock_input.txt");
        let lines = lines_file.unwrap(); 
        let bits_at_position = get_all_bits_at_position(&lines, 0);
        assert_eq!(get_gamma(&bits_at_position), 1);
    }

    #[test]
    fn test_get_epsilon() {
        let lines_file = get_lines_from_file("mock_input.txt");
        let lines = lines_file.unwrap(); 
        let bits_at_position = get_all_bits_at_position(&lines, 0);
        assert_eq!(get_epsilon(&bits_at_position), 0);
    }

    #[test]
    fn test_get_rate_for_all_positions() {
        let lines_file = get_lines_from_file("input.txt");
        let lines = lines_file.as_ref().unwrap(); 
        let gamma_rate = get_rate_for_all_positions(&lines, true);
        let epsilon_rate = get_rate_for_all_positions(&lines, false);
        let power_comsumption = calculate_power_comsumption(gamma_rate, epsilon_rate);
        assert_eq!(power_comsumption, 845186);
    }

}

// get lines from input file
// calculate gamma rate: most common bit in position
    // get y column of values from input &lines[0 as usize];
    // get x char position in row (from selected_line) 
        // let byte: u8 = my_string.as_bytes()[i];
        // my_string.chars().nth(0).unwrap();
    // get most common in position: filter 0/1 and count?
   //convert to decimal
    // 10110 or 22 in decimal
// calculate epsilon rate: least common bit in position
    //convert to decimal
    // 01001 or 9 in decimal
// calculate power_comsumption: gamma * epsilon (in decimal)
