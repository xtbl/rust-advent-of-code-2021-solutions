// --- Day 2: Dive! ---
// https://adventofcode.com/2021/day/2

// example
// forward 5
// down 5
// forward 8
// up 3
// down 8
// forward 2

// after following these instructions, you would have a horizontal position of 15 and a depth of 10. (Multiplying these together produces 150.)

// TODO:
// ✅ get inputs
// ✅ parse inputs into direction, value
// ✅ process parsed inputs: + - by direction
// ✅ multiply processed horizontal and depth 
// ✅ implement part 2 rules

//struct SubmarineMovement
// completed_movements<Vec<[string, i32]>>
// impl move_to_position(string, i32)
// current_position_horizontal
// current_position_depth
// get multiplied position


use std::env;
use std::fs::File;
use std::io::{self, BufRead, BufReader};
use std::path::Path;
use std::io::Result;


fn main() {
    println!("Dive!");
}

fn get_lines_from_file(filename: impl AsRef<Path>) -> Result<Vec<String>> {
    BufReader::new(File::open(filename)?).lines().collect()
}

fn convert_line_to_movement(line: &str) -> (&str, i32)  {
    let mut splitted = line.split_whitespace();
    let direction = splitted.next().unwrap_or("");
    let value = splitted.next().unwrap_or("0").parse::<i32>().unwrap(); 
    (direction, value)
}

fn process_movement_instructions(lines: Vec<String>) -> (i32, i32) {
    let mut horizontal_position = 0;
    let mut depth_position = 0;
    let mut aim = 0;

    for line in lines {
        let movement: (&str, i32) = convert_line_to_movement(&line);
        match movement {
            ("forward", _) => {
                horizontal_position = horizontal_position + movement.1;
                depth_position = depth_position + (aim * movement.1);
            },            
            ("up", _) => aim = aim - movement.1,
            ("down", _) => aim = aim + movement.1,
            (&_, _) => horizontal_position = horizontal_position
        }
    }
    (horizontal_position, depth_position)
}

fn get_final_position_multiplied(final_position: (i32, i32)) -> i32 {
    final_position.0 * final_position.1
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_get_lines_from_file() {
        let lines_file = get_lines_from_file("input.txt");
        let lines = lines_file.unwrap(); 
        let selected_line = &lines[1 as usize];
        assert_eq!(selected_line, "forward 8");
    }

    #[test]
    fn test_convert_line_into_movement() {
        let lines_file = get_lines_from_file("input.txt");
        let lines = lines_file.unwrap(); 
        let selected_line = &lines[1 as usize];
        let movement: (&str, i32) = convert_line_to_movement(selected_line);
        assert_eq!(movement, ("forward", 8));
    }

    #[test]
    fn test_process_movement_instructions() {
        let lines_file = get_lines_from_file("input.txt");
        let lines = lines_file.unwrap(); 
        let processed_movements = process_movement_instructions(lines);
        assert_eq!(processed_movements, (2162, 987457));
    }

    #[test]
    fn test_get_final_position_multiplied() {
        let lines_file = get_lines_from_file("input.txt");
        let lines = lines_file.unwrap(); 
        let processed_movements = process_movement_instructions(lines);
        let final_position = get_final_position_multiplied(processed_movements);
        assert_eq!(final_position, 2134882034);
    }
}
