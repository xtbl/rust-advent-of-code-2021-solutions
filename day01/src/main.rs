//--- Day 1: Sonar Sweep ---
// https://adventofcode.com/2021/day/1

use std::env;
use std::fs::File;
use std::io::{self, BufRead, BufReader};
use std::path::Path;
use std::io::Result;

fn main() {
    println!("sonar sweep");
}

fn get_lines_from_file(filename: impl AsRef<Path>) -> Result<Vec<String>> {
    BufReader::new(File::open(filename)?).lines().collect()
}

fn map_lines_to_int(lines: Vec<String>) -> Vec<i32> {
    lines.into_iter().map(|x| x.parse::<i32>().unwrap()).collect()
}

fn get_amount_of_increased_sweeps(sonar_sweeps: Vec<i32>) -> usize {
    let filtered = sonar_sweeps.windows(2).filter(|x| x[0] < x[1]);
    filtered.count()
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_scanner() {
        assert_eq!(1,1);
    }

    #[test]
    fn test_get_lines_from_file() {
        let lines_file = get_lines_from_file("input.txt");
        let lines = lines_file.unwrap(); 
        let selected_line = &lines[1 as usize];
        let parsed_selected_line = selected_line.parse::<i32>().unwrap();
        assert_eq!(parsed_selected_line, 150);
    }

    #[test]
    fn test_map_lines_to_int() {
        let lines_file = get_lines_from_file("input.txt");
        let lines = lines_file.unwrap(); 
        let lines_to_int = map_lines_to_int(lines);
        let selected_line = lines_to_int[1 as usize];
        assert_eq!(selected_line, 150);
    }

    #[test]
    fn test_get_amount_of_increased_sweeps() {
        let lines_file = get_lines_from_file("input.txt");
        let lines = lines_file.unwrap(); 
        let lines_to_int = map_lines_to_int(lines);
        let amount_of_increase = get_amount_of_increased_sweeps(lines_to_int);
        assert_eq!(amount_of_increase, 1226);
    }

}
