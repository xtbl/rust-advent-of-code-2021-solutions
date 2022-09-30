// --- Day 4: Giant Squid ---
// bingo subsystem
// 5x5 grid of numbers
// Numbers are chosen at random, 
// and the chosen number is marked on all boards on which it appears
// If all numbers in any row or any column of a board are marked, that board wins. (Diagonals don't count.)
// automatically generates a random order in which to draw numbers and a random set of boards (your puzzle input)

// TODO:
// parse input
//     numbers  - getNumbers
//     boards   - getBoards
// draw numbers - drawNumber
// next: --->  
//  load board but use board_values tuple
// load real board

// mark boards with matching numbers - markBoard(num)
// check if there are matching complete row or column - hasCompleteRow, hasCompleteCol
// if winning board - isWinningBoard
// calculate score:
//     sum of all unmarked numbers
//     multiply that sum by the number that was just called when the board won

// struct board
// load_board_data_into_vecs: receive 5 lines -> Vec<Vec<(number, isMarked)>>
// boardValues: Vec<Vec<(number, isMarked)>>

// markBoard
// lastMarkedNumber
// hasCompleteRow, hasCompleteCol
// isWinningBoard

// calculateScore

use std::fs::File;
use std::io::{BufRead, BufReader};
use std::path::Path;
use std::io::Result;
// use std::str::FromStr;
// use core::slice::RChunksExactMut;


fn main() {
    println!("Giant Squid");
}

fn get_lines_from_file(filename: impl AsRef<Path>) -> Result<Vec<String>> {
    BufReader::new(File::open(filename)?).lines().collect()
}

fn get_numbers_to_draw(filename: impl AsRef<Path>) -> Result<Vec<i32>> {
    let lines_file:Result<Vec<String>> = BufReader::new(File::open(filename)?).lines().collect();
    let lines = lines_file.unwrap();
    let selected_line: &String = &lines[0 as usize];
    let splitted: Vec<&str> = selected_line.split(",").collect();
    let splitted_mapped: Vec<i32> = splitted.into_iter().map(|x|x.parse::<i32>().unwrap_or(0)).collect();
    Ok(splitted_mapped)
}

// discard first line with numbers
// then start looping  or mapping and  processing the rest
fn get_all_boards_together(filename: impl AsRef<Path>) -> Result<Vec<(i32, bool)>> {
    let lines_file:Result<Vec<String>> = BufReader::new(File::open(filename)?).lines().collect();
    // load real board
    let lines = lines_file.unwrap();
    // loop through lines
    // let mut line_counter = 0;

    let mapped_lines_into_ints:Vec<_> = lines.into_iter().map(|current_line| {
        let split: Vec<&str> = current_line.split_whitespace().collect();
        let splitted_map: Vec<_> = split.into_iter().map(|x| x.parse::<i32>().unwrap_or(0)).collect();
        splitted_map
    }).filter(|x| x.len() == 5).collect();
    let mut mapped_lines_cloned: Vec<(i32, bool)> = mapped_lines_into_ints.into_iter().map(|x| (x[0], false)).collect::<Vec<(i32, bool)>>();
    // let mut mapped_lines_cloned = mapped_lines_into_ints;


    // TODO:
    // do window 5 to group filtered arrays
    // what if we return an iter? https://docs.rs/windows/0.6.0/windows/struct.Array.html#method.rchunks_exact_mut
    // or what if we return 2 dimensional arrays/boards on each iter? to be directly inserted in the boards list
    // how to build a board? [0, [iter.next()]], [1, [iter.next()]], [2, [iter.next()]]...[4...]
    // what if we do the rchunks part in the receiving fn and handle that there
    // Ok(mapped_lines_into_ints)
    // Ok(iter)
    Ok(mapped_lines_cloned)
    // Ok(vec![])
}

// fn load_board_data(arg: Type) -> RetType {
//     unimplemented!();
// }

fn board_loader(board_values: Vec<(i32, bool)>) -> Board {
    let mut iter = board_values.rchunks_exact_mut(5);
    println!("------------- chunks 01 {:?}", iter.next());
    println!("------------- chunks 02 {:?}", iter.next());
    println!("------------- chunks 03 {:?}", iter.next());

    let mut new_board = Board {
        ..Default::default()
    };
    new_board.load_board_data(iter.next());
    new_board
}

#[derive(Debug, Default)]
struct Board {
    board_values: Vec<Vec<i32>>,
    has_complete_row: bool,
    has_complete_col: bool,
    is_winning_board: bool
}

pub trait LoadBoardData {
    fn load_board_data(&mut self, new_board_values: Vec<Vec<i32>>);
}

pub trait GetBoardData {
    fn get_board_data(&self) -> Vec<Vec<i32>>;
}

impl LoadBoardData for Board {
   fn load_board_data(&mut self, new_board_values: Vec<Vec<i32>>) {
        self.board_values = new_board_values;
   }
}

impl GetBoardData for Board {
   fn get_board_data(&self) -> Vec<Vec<i32>> {
        self.board_values.clone()
   }
}




#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_board_loader() {
        let mut new_board = Board {
            ..Default::default()
        };
        new_board.load_board_data(vec![vec![1,1,2]]);
        let board_data = new_board.get_board_data();
        assert_eq!(board_data, [[1,1,2]]);
    }

    #[test]
    fn test_load_board_struct_impl() {
        let mut new_board = Board {
            ..Default::default()
        };
        new_board.load_board_data(vec![vec![1,1,2]]);
        let result = new_board.get_board_data();
        assert_eq!(result, [[1,1,2]]);
    }

    #[test]
    fn test_board_struct_exists() {
        let new_board = Board {
            ..Default::default()
        };
        assert_eq!(new_board.has_complete_col, false);
    }

    #[test]
    fn test_get_all_boards_together() {
        // let get_numbers_line = get_numbers_to_draw("mock.txt");
        // let line = get_numbers_line.unwrap();
        let all_boards_together = get_all_boards_together("mock.txt");
        let all_boards_together_unwrap = all_boards_together.unwrap();
        let expected_result = [(1, false)];
        assert_eq!(all_boards_together_unwrap, expected_result);
    }

    #[test]
    fn test_get_lines_from_file() {
        let lines_file = get_lines_from_file("mock.txt");
        let lines = lines_file.unwrap();
        let selected_line = &lines[2 as usize];
        assert_eq!(selected_line, "22 13 17 11  0");
    }

    #[test]
    fn test_get_numbers_to_draw() {
        let get_numbers_line = get_numbers_to_draw("mock.txt");
        let line = get_numbers_line.unwrap();
        let expected_result = [7, 4, 9, 5, 11, 17, 23, 2, 0, 14, 21, 24, 10, 16, 13, 6, 15, 25, 12, 22, 18, 20, 8, 19, 3, 26, 1];
        assert_eq!(line, expected_result);
    }



}
