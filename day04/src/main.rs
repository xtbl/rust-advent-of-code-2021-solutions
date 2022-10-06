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

fn get_all_boards_together(filename: impl AsRef<Path>) -> Result<Vec<(i32, bool)>> {
    let lines_file:Result<Vec<String>> = BufReader::new(File::open(filename)?).lines().collect();
    let lines = lines_file.unwrap();
    // println!("-------------- lines: {:?}", lines);
    let mapped_lines_into_ints:Vec<_> = lines.into_iter().map(|current_line| {
        // print!("-------------- current_line: {:?}", current_line);
        let split: Vec<&str> = current_line.split_whitespace().collect();
        let splitted_map: Vec<_> = split.into_iter().map(|x| x.parse::<i32>().unwrap_or(0)).collect();
        splitted_map
    }).filter(|x| x.len() == 5).collect();
    // println!("-------------- mapped_lines_into_ints: {:?}", mapped_lines_into_ints);
    let flatten_mapped_lines_into_ints:Vec<i32> = mapped_lines_into_ints.into_iter().flat_map(|array| array.into_iter()).collect();
    // println!("-------------- flatten_mapped_lines_into_ints: {:?}", flatten_mapped_lines_into_ints);

    let mut mapped_lines_cloned: Vec<(i32, bool)> = 
        flatten_mapped_lines_into_ints.into_iter().map(|x| (x, false)).collect::<Vec<(i32, bool)>>();
    // println!("-------------- mapped_lines_cloned: {:?}", mapped_lines_cloned);
    Ok(mapped_lines_cloned)
}

// fn load_board_data(arg: Type) -> RetType {
//     unimplemented!();
// }

fn board_loader(board_values: Vec<(i32, bool)>) -> Board {
    let mut iter = board_values.rchunks(5);
    let mut new_board = Board {
        ..Default::default()
    };
    let board_content = iter.next().unwrap();
    new_board.load_board_data(board_content.to_vec());
    new_board
}

#[derive(Debug, Default, Clone)]
struct Board {
    board_values: Vec<Vec<(i32, bool)>>,
    has_complete_row: bool,
    has_complete_col: bool,
    is_winning_board: bool,
    lastMarkedNumber: i32
}

pub trait LoadBoardData {
    fn load_board_data(&mut self, new_board_values: Vec<(i32, bool)>);
}

pub trait GetBoardData {
    fn get_board_data(&self) -> Vec<Vec<(i32, bool)>>;
}

pub trait MarkBoard {
    fn mark_board(&mut self, mark_num: i32) -> &mut Self;
}

pub trait checkWinning {
    fn check_winning(&self) -> bool;
}

pub trait calculateScore {
    fn calculate_score(&self) -> i32;
}

impl LoadBoardData for Board {
   fn load_board_data(&mut self, new_board_values: Vec<(i32, bool)>) {
        let row_col_size = 5;
        let default_board_value = (0,false);
        let default_board_row:Vec<(i32, bool)> = (0..row_col_size).map(|_| default_board_value).collect();
        let default_board_2d: Vec<Vec<(i32, bool)>> = (0..row_col_size).map(|_| default_board_row.clone()).collect();
        let mut chunked_new_board_values = new_board_values.chunks(row_col_size);
        let edited_board_2d: Vec<Vec<(i32, bool)>> = default_board_2d.clone().into_iter().enumerate().map({
            |_| chunked_new_board_values.next().unwrap_or(&vec![(0,false)]).to_vec()
        }).collect();
        self.board_values = edited_board_2d;
   }
}

impl GetBoardData for Board {
   fn get_board_data(&self) -> Vec<Vec<(i32, bool)>> {
        self.board_values.clone()
   }
}

impl MarkBoard for Board {
    fn mark_board(&mut self, mark_num: i32) -> &mut Self {
        let mut board_data = self.get_board_data();
        let marked_board_data:Vec<Vec<(i32, bool)>> = board_data.iter_mut()
            .map({|row| row.iter_mut()
                .map({|(num, is_marked)|
                    if num == &mark_num {
                        (num.clone(), true)
                    } else {
                        (num.clone(), is_marked.clone())
                    }
            }).collect()
        }).collect();
        self.board_values = marked_board_data;
        self.lastMarkedNumber = mark_num;
        self
    }
}

impl checkWinning for Board {
    fn check_winning(&self) -> bool {
        let board_data = &self.get_board_data();
        let mut winning_row = false;
        for row in board_data {
            winning_row = row.iter().all(|(_, marked)| *marked);
            if winning_row {
                break;
            }
        }

        let board_data_2 = &self.get_board_data();
        let mut winning_col = false;
          for row in 0..5 {
            let mut build_column:Vec<bool> = vec![];
            for col in 0..5 {
                build_column.push(board_data_2[col][row].1);
           }
            winning_col = build_column.iter().all(|x| *x);
            if winning_col {
                break;
            }
        }
        winning_row || winning_col
    }
}

fn load_data_into_boards(boards_data: Vec<(i32, bool)>) -> Vec<Board> {
    let full_board_item_amount = 25;
    let mut iter_boards_data = boards_data.chunks(full_board_item_amount);
    let mapped_boards: Vec<Board> = iter_boards_data.map(|board_data| {
        let mut new_board = Board {
            ..Default::default()
        };
        new_board.load_board_data(board_data.to_vec());
        new_board
    }).collect();
    println!("--------- mapped_boards {:?}", mapped_boards);

    mapped_boards
}


impl calculateScore for Board  {
    fn calculate_score (&self) -> i32 {
        let mut board_data = self.get_board_data();
        let mut unmarked_sum = 0;
        let marked_board_data:Vec<Vec<(i32, bool)>> = board_data.iter_mut()
            .map({|row| row.iter_mut()
                .map({|(num, is_marked)| {
                    if *is_marked == true {
                        unmarked_sum = unmarked_sum + num.clone() as i32; 
                    }            
                    (num.clone(), is_marked.clone())
                }
                }).collect()
            }).collect();
        let last_number = self.lastMarkedNumber;
        println!("unmarked_sum: {:?}", unmarked_sum);
        println!("last_number: {:?}", last_number);
        unmarked_sum * last_number
    }
}

fn run_game(data_file: &str) -> i32 {
    let all_boards_together = get_all_boards_together(data_file).unwrap();
    let loaded_boards = load_data_into_boards(all_boards_together);
    let mut loaded_boards_cloned: Vec<Board> = loaded_boards.into_iter().map(|x| x.clone()).collect();
    // let first_board_content = loaded_boards[0].get_board_data();

    let get_numbers_line = get_numbers_to_draw(data_file).unwrap();
    println!("--- get_numbers_line: {:?} ", get_numbers_line);

    
    let numbers_iter = get_numbers_line.iter();
    // let mut loaded_boards_iter = loaded_boards_cloned.iter();
    
    for mark_num in numbers_iter {
        for loaded_board in loaded_boards_cloned {
            println!("--- num iter: {:?} ", mark_num);
            loaded_board.mark_board(*mark_num);
        }
    }


    println!("------------------ ");
    // println!("--- loaded_boards: {:?} ", loaded_boards_cloned);
    10
}




#[cfg(test)]
mod tests {
    use super::*;

//TODO: get numbers and compare with boards, mark boards, check if winning

    #[test]
    fn test_run_game() {


        
        
        assert_eq!(run_game("mock.txt"), 20);
    }

    #[test]
    fn test_calculate_score() {
        let mut new_board = Board {
            ..Default::default()
        };
        // row
        new_board.load_board_data(vec![
            (1, false), (1, false),(1, false),(1, false),(1, false),
            (1, false), (1, false),(1, false),(1, false),(1, false),
            (2, true), (2, true),(2, true),(2, true),(2, true),
            (1, false), (1, false),(1, false),(1, false),(1, false),
            (1, false), (1, false),(1, false),(1, false),(1, false)]);
        
        new_board.lastMarkedNumber = 2;

        // 30 * last number marked
        assert_eq!(new_board.calculate_score(), 20);
    }



    #[test]
    fn test_check_winning() {
        let mut new_board = Board {
            ..Default::default()
        };
        // row
        new_board.load_board_data(vec![
            (0, false), (0, false),(0, false),(0, false),(0, false),
            (1, false), (1, false),(1, false),(1, false),(1, false),
            (2, true), (2, true),(2, true),(2, true),(2, true),
            (1, false), (1, false),(3, false),(1, false),(1, false),
            (4, false), (4, false),(4, false),(4, false),(4, false)]);
        
        assert_eq!(new_board.check_winning(), true);

        // col
        new_board.load_board_data(vec![
            (0, true), (0, false),(0, false),(0, false),(0, false),
            (1, true), (1, false),(1, false),(1, false),(1, false),
            (2, true), (2, false),(2, false),(2, false),(2, false),
            (1, true), (1, false),(3, false),(1, false),(1, false),
            (4, true), (4, false),(4, false),(4, false),(4, false)]);
        
        assert_eq!(new_board.check_winning(), true);
    }

    #[test]
    fn test_mark_board() {
        let mut new_board = Board {
            ..Default::default()
        };
        new_board.load_board_data(vec![
            (0, false), (0, false),(0, false),(0, false),(0, false),
            (1, false), (1, false),(1, false),(1, false),(1, false),
            (2, false), (2, false),(2, false),(2, false),(2, false),
            (1, false), (1, false),(3, false),(1, false),(1, false),
            (4, false), (4, false),(4, false),(4, false),(4, false)]);

        let input_to_compare = vec![
            [(0, false), (0, false), (0, false), (0, false), (0, false)], 
            [(1, false), (1, false), (1, false), (1, false), (1, false)], 
            [(2, false), (2, false), (2, false), (2, false), (2, false)], 
            [(1, false), (1, false), (3, true), (1, false), (1, false)], 
            [(4, false), (4, false), (4, false), (4, false), (4, false)]];
        let mark_num = 3;
        new_board.mark_board(mark_num);
        let result = new_board.get_board_data();
        assert_eq!(result[3], input_to_compare[3]);
    }


    #[test]
    fn test_load_data_into_boards() {
        let all_boards_together = get_all_boards_together("mock.txt");
        let loaded_boards = load_data_into_boards(all_boards_together.unwrap());
        let first_board_content = loaded_boards[0].get_board_data();
        let expected_result = [
            [(22, false), (13, false), (17, false), (11, false), (0, false)], 
            [(8, false), (2, false), (23, false), (4, false), (24, false)], 
            [(21, false), (9, false), (14, false), (16, false), (7, false)], 
            [(6, false), (10, false), (3, false), (18, false), (5, false)], 
            [(1, false), (12, false), (20, false), (15, false), (19, false)]];

        assert_eq!(first_board_content, expected_result);
    }

    #[test]
    fn test_load_board_data() {
        let mut new_board = Board {
            ..Default::default()
        };
        new_board.load_board_data(vec![
            (0, false), (0, false),(0, false),(0, false),(0, false),
            (1, false), (1, false),(1, false),(1, false),(1, false),
            (2, false), (2, false),(2, false),(2, false),(2, false),
            (3, false), (3, false),(3, false),(3, false),(3, false),
            (4, false), (4, false),(4, false),(4, false),(4, false)]);

        let input_to_compare = vec![
            [(0, false), (0, false), (0, false), (0, false), (0, false)], 
            [(1, false), (1, false), (1, false), (1, false), (1, false)], 
            [(2, false), (2, false), (2, false), (2, false), (2, false)], 
            [(3, false), (3, false), (3, false), (3, false), (3, false)], 
            [(4, false), (4, false), (4, false), (4, false), (4, false)]];
        let result = new_board.get_board_data();
        assert_eq!(result[0], input_to_compare[0]);
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
        let all_boards_together = get_all_boards_together("mock.txt");
        let all_boards_together_unwrap = all_boards_together.unwrap();
        // let expected_result = [(22, false), (8, false), (21, false), (6, false), (1, false), (3, false), (9, false), (19, false), (20, false), (14, false), (14, false), (10, false), (18, false), (22, false), (2, false)];
        // let expected_result = [(22, false), (13, false), (17, false), (11, false), (0, false), (8, false), (2, false), (23, false), (4, false), (24, false), (21, false), (9, false), (14, false), (16, false), (7, false), (6, false), (10, false), (3, false),(18, false),(5, false), 
        //     (1, false), (12, false), (20, false), (15, false), (19, false)];
         let expected_result = [(22, false), (13, false), (17, false), (11, false), (0, false), (8, false), (2, false), (23, false), (4, false), (24, false), (21, false), (9, false), (14, false), (16, false), (7, false), (6, false), (10, false), (3, false), (18, false), (5, false), (1, false), (12, false), (20, false), (15, false), (19, false), (3, false), (15, false), (0, false), (2, false), (22, false), (9, false), (18, false), (13, false), (17, false), (5, false), (19, false), (8, false), (7, false), (25, false), (23, false), (20, false), (11, false), (10, false), (24, false), (4, false), (14, false), (21, false), (16, false), (12, false), (6, false), (14, false), (21, false), (17, false), (24, false), (4, false), (10, false), (16, false), (15, false), (9, false), (19, false), (18, false), (8, false), (23, false), (26, false), (20, false), (22, false), (11, false), (13, false), (6, false), (5, false), (2, false), (0, false), (12, false), (3, false), (7, false)];
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
