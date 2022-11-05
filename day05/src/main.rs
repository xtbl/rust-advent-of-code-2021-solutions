// Hydrothermal Venture
// submarine produces a list of lines, avoid lines
// each line in format x1,y1 -> x2,y2
// An entry like 1,1 -> 1,3 covers points 1,1, 1,2, and 1,3.
// An entry like 9,7 -> 7,7 covers points 9,7, 8,7, and 7,7. 
// For now, only consider horizontal and vertical lines: lines where either x1 = x2 or y1 = y2.
// Each position is shown as the number of lines which cover that point or . if no line covers that point
// determine the number of points where at least two lines overlap - this is anywhere in the diagram with a 2 or larger - a total of 5 points.
// Consider only horizontal and vertical lines. At how many points do at least two lines overlap?


// get each input line into Line
// filter only hor and vert lines
// Point(x: i32, y: i32)
    // Point -> Point 
//TODO: get all lines converted to points, add all the points in the PointList, calculate overlaps
// Line(Point, Point)
    // method: GetLinePoints returns list of Points that conform the Line
// create a diagram
// DrawLine in diagram
    // check if position has another Point in it
    // if yes count how many and add the number of overlaps
// CalculateOverlaps
// traverse the diagram and calculate
// PaintDiagram: create from input
// CalculateOverlaps amount


use std::collections::HashMap;
use std::fs::File;
use std::io::{BufRead, BufReader};
use std::path::Path;
use std::io::Result;

fn main() {
    println!("Hydrothermal Venture");
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
struct Point {
    x: i32,
    y: i32,
}

#[derive(Debug, PartialEq, Eq, Hash)]
struct Line {
    a: Point,
    b: Point,
}

type PointList = HashMap<Point, i32>;
type LineList = Vec<Line>;

fn convert_parsed_line_into_line(parsed_line: Vec<&str>) -> Line {
    let point_a = convert_input_into_point(&String::from(parsed_line[0]));
    let point_b = convert_input_into_point(&String::from(parsed_line[1]));
    Line {
        a: point_a,
        b: point_b,
    }
}

fn convert_input_into_point(input_pair: &String) -> Point {
    let splitted: Vec<&str> = input_pair.split(",").collect();
    Point {
        x: splitted[0].parse::<i32>().unwrap(),
        y: splitted[1].parse::<i32>().unwrap(),
    }
}

fn parse_line(input_lines: &String) -> Vec<&str> {
    let splitted: Vec<&str> = input_lines.split(" -> ").collect();
    splitted
}

fn get_lines_from_file(filename: impl AsRef<Path>) -> Result<Vec<String>> {
    BufReader::new(File::open(filename)?).lines().collect()
}

fn get_input_file_name() -> &'static str {
  "mock.txt"
}

fn add_point_to_hashmap(point: Point, point_list: &PointList) -> PointList {
    // let mut point_list_copy: PointList = point_list.into_iter().map(|(k,v)| { (*k,*v) }).collect();
    let mut point_list_clone: PointList = PointList::new();
    point_list_clone.clone_from(point_list);
    if point_list_clone.contains_key(&point) {
        println!("--- CONTAINS KEY: {:?}", &point);
        let current_value_in_point = match point_list_clone.get(&point) {
            Some(value_in_point) => value_in_point,
            None => panic!("Error getting Point from PointList"),
        };
        point_list_clone.insert(point, current_value_in_point + 1);
    } else {
        point_list_clone.insert(point, 1);
    }
    point_list_clone
}

fn convert_all_input_into_lines(all_lines: Vec<String>) -> LineList {
    all_lines.into_iter().map(|string_line| {
        let parsed_line = parse_line(&string_line);
        convert_parsed_line_into_line(parsed_line)
    }).collect()
}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn test_convert_all_input_into_lines() {
        let lines = match get_lines_from_file(get_input_file_name()) {
            Ok(line) => line,
            Err(error) => panic!("Error getting line {:?}", error),
        };
        let lines_from_input = convert_all_input_into_lines(lines);

        let first_line = &lines_from_input[0 as usize];
        let expected_line = Line { a: Point { x: 0, y: 9 }, b: Point { x: 5, y: 9 } };
        assert_eq!(&expected_line, first_line);
    }

    #[test]
    fn test_add_points_to_hashmap() {
        let point = Point{x: 1, y: 3};
        let empty_point_list: PointList = HashMap::new(); 
        let mut expected_point_list: PointList = HashMap::new(); 
        expected_point_list.insert(Point{x: 1, y: 3}, 1);
        let mut updated_list = add_point_to_hashmap(point, &empty_point_list);

        assert_eq!(expected_point_list, updated_list);

        updated_list.insert(Point{x: 2, y: 1}, 2);
        let point2 = Point{x: 2, y: 1};
        let updated_list = add_point_to_hashmap(point2, &updated_list);
        expected_point_list.insert(Point{x: 2, y: 1}, 3);

        assert_eq!(expected_point_list, updated_list);
    }

    #[test]
    fn test_convert_parsed_line_into_line() {
        let selected_line = String::from("9,4 -> 3,4");
        let parsed_line = parse_line(&selected_line);
        let line = convert_parsed_line_into_line(parsed_line);
        let expected_result = Line {
            a: Point{x: 9, y: 4}, 
            b: Point{x: 3, y: 4}, 
        };
        assert_eq!(line, expected_result);
    }

    #[test]
    fn test_convert_input_into_point() {
        let pair = String::from("9,4");
        let point = convert_input_into_point(&pair);
        assert_eq!(point, Point{x: 9, y: 4});
    }

    #[test]
    fn test_parse_line() {
        let selected_line = String::from("9,4 -> 3,4");
        let parsed = parse_line(&selected_line);
        assert_eq!(parsed, ["9,4", "3,4"]);
    }

    // #[test]
    // fn test_convert_input_into_lines() {
    //     let lines = match get_lines_from_file(get_input_file_name()) {
    //         Ok(line) => line,
    //         Err(error) => panic!("Error getting line {:?}", error),
    //     };
    //     let selected_line = &lines[2 as usize];
    //     let line_converted = convert_input_into_lines(selected_line);
    //     assert_eq!(line_converted, 1);
    // }

    #[test]
    fn test_get_lines_from_file() {
        let lines = match get_lines_from_file(get_input_file_name()) {
            Ok(line) => line,
            Err(error) => panic!("Error getting line {:?}", error),
        };
        let selected_line = &lines[2 as usize];
        assert_eq!(selected_line, "9,4 -> 3,4");
    }

}
