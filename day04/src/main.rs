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
    let mut iter = board_values.rchunks(5);
    println!("------------- chunks 01 {:?}", iter.next());
    println!("------------- chunks 02 {:?}", iter.next());
    println!("------------- chunks 03 {:?}", iter.next());

    let mut new_board = Board {
        ..Default::default()
    };
    let board_content = iter.next().unwrap();
    // convert to board vector
    new_board.load_board_data(board_content.to_vec());
    new_board
}

#[derive(Debug, Default)]
struct Board {
    board_values: Vec<Vec<(i32, bool)>>,
    has_complete_row: bool,
    has_complete_col: bool,
    is_winning_board: bool
}

pub trait LoadBoardData {
    fn load_board_data(&mut self, new_board_values: Vec<(i32, bool)>);
}

pub trait GetBoardData {
    fn get_board_data(&self) -> Vec<Vec<(i32, bool)>>;
}

impl LoadBoardData for Board {
   fn load_board_data(&mut self, new_board_values: Vec<(i32, bool)>) {
        let row_col_size = 5;
        // https://stackoverflow.com/questions/65191637/idiomatic-way-to-insert-after-a-certain-element-in-vector 

        // one board is 5 rows x 5 cols
        // index 0...24 - 0 or index % 5 === 0
        // let mut vec_2d: Vec<Vec<(i32, bool)>> = vec![vec![(0, false)]];
        let default_board_value = (0,false);
        let default_board_row:Vec<(i32, bool)> = (0..row_col_size).map(|_| default_board_value).collect();
        let mut default_board_2d: Vec<Vec<(i32, bool)>> = (0..row_col_size).map(|_| default_board_row.clone()).collect();
        // set new board values
        default_board_2d[0][0] = (2, true);
        // how to replace values using map?
        // default_board_2d.map() - get index 0, replace with first window of values
        let mut chunked_new_board_values = new_board_values.chunks(row_col_size);

        //TODO: convert to vec vec board format - replacing default row tuples with chunked new board values rows
        let mut edited_board_2d: Vec<Vec<(i32, bool)>> = default_board_2d.clone().into_iter().enumerate().map({
            |(index, x)| {

                println!("[inside edited board index] --> {:?}", index);
                println!("[inside edited board x] --> {:?}", x);
                // println!("[inside edited board chunked_new_board_values] --> {:?}", chunked_new_board_values.next().unwrap_or(&vec![(0,false)]));
                chunked_new_board_values.next().unwrap_or(&vec![(0,false)]).to_vec()} 
        }).collect();
        
        println!("[default_board_2d] --> {:?}", default_board_2d);
        println!("[new_board_values] --> {:?}", new_board_values);
        // for row in new_board_values {
        //     for col in row {
        //         vec_2d.push(col);
        //     }
        // }

        // to iterate
        // https://stackoverflow.com/questions/72843130/how-to-make-iterate-through-a-2d-vector
        // for row in new_board_values {
        //     for col in row {
        //         println!("row {:?} - col {:?}", row.0, col.1);
        //     }
        // }

        // self.board_values = vec![ vec![(1,false), (2, true)], vec![(1,false), (2, true)] ];
        println!("[edited_board_2d] --> {:?}", edited_board_2d);
        self.board_values = edited_board_2d;

   }
}

impl GetBoardData for Board {
   fn get_board_data(&self) -> Vec<Vec<(i32, bool)>> {
        self.board_values.clone()
   }
}




#[cfg(test)]
mod tests {
    use super::*;

    // #[test]
    // fn test_board_loader() {
    //     let mut new_board = Board {
    //         ..Default::default()
    //     };
    //     new_board.load_board_data(vec![vec![1,1,2]]);
    //     let board_data = new_board.get_board_data();
    //     assert_eq!(board_data, [[1,1,2]]);
    // }

    #[test]
    fn test_load_board_struct_impl() {
        let mut new_board = Board {
            ..Default::default()
        };
        new_board.load_board_data(vec![
            (0, false), (0, false),(0, false),(0, false),(0, false),
            (1, false), (1, false),(1, false),(1, false),(1, false),
            (2, false), (2, false),(2, false),(2, false),(2, false),
            (3, false), (3, false),(3, false),(3, false),(3, false),
            (4, false), (4, false),(4, false),(4, false),(4, false)
        ]);

        let input_to_compare = vec![[(0, false), (0, false), (0, false), (0, false), (0, false)], [(1, false), (1, false), (1, false), (1, false), (1, false)], [(2, false), (2, false), (2, false), (2, false), (2, false)], [(3, false), (3, false), (3, false), (3, false), (3, false)], [(4, false), (4, false), (4, false), (4, false), (4, false)]];
        let result = new_board.get_board_data();
        println!("--- result; {:?}", result);
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
        // let get_numbers_line = get_numbers_to_draw("mock.txt");
        // let line = get_numbers_line.unwrap();
        let all_boards_together = get_all_boards_together("mock.txt");
        let all_boards_together_unwrap = all_boards_together.unwrap();
        let expected_result = [(22, false), (8, false), (21, false), (6, false), (1, false), (3, false), (9, false), (19, false), (20, false), (14, false), (14, false), (10, false), (18, false), (22, false), (2, false)];
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
