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
use std::cmp::Ordering;

fn main() {
    println!("Hydrothermal Venture");
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
struct Point {
    x: i32,
    y: i32,
}

#[derive(Debug, PartialEq, Eq, Hash, Copy, Clone)]
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
        point_list_clone.insert(point, point_list_clone.get(&point).unwrap_or(&1).to_owned() + 1);
    } else {
        point_list_clone.insert(point, 1);
    }

    println!("--- point_list_clone: {:?}", point_list_clone);
    point_list_clone
}

fn convert_all_input_into_lines(all_lines: Vec<String>) -> LineList {
    all_lines.into_iter().map(|string_line| {
        let parsed_line = parse_line(&string_line);
        convert_parsed_line_into_line(parsed_line)
    }).collect()
}

//TODO: get line
// convert into point list
// find x1, x2, get intermediate points between them: which is highest - lowest
// create points
// add x scenarios, y scenarios
// consider moving it to a method https://doc.rust-lang.org/rust-by-example/fn/methods.html
fn convert_line_into_point_list(line: Line) -> PointList {
    // get x1 and x2
    let x1 = line.a.x;
    let x2 = line.b.x;
    let y1 = line.a.y;
    let y2 = line.b.y;
    
    #[derive(Debug)]
    enum LineOrientation {
        Horizontal,
        Vertical,
        NotClear
    }

    let line_orientation = match line {
        _l if x1 == x2 && y1 != y2 => LineOrientation::Vertical,
        _l if y1 == y2 && x1 != x2 => LineOrientation::Horizontal,
        _ => LineOrientation::NotClear,
    };

    // return diff according to comparison result
    // we can return a range array then map Points into it
    // https://doc.rust-lang.org/std/ops/struct.Range.html
    let range_x = match x1.cmp(&x2) {
        Ordering::Less => x1..=x2,
        Ordering::Equal => x1..=x1,
        Ordering::Greater => x2..=x1,
    };
    let range_y = match y1.cmp(&y2) {
        Ordering::Less => y1..=y2,
        Ordering::Equal => y1..=y1,
        Ordering::Greater => y2..=y1,
    };
    let range_none = 0..=0;
    
    println!("********** orientation {:?}", line_orientation);
    println!("********** range_x {:?}", range_x);
    println!("********** range_y {:?}", range_y);

    let line_range = match line_orientation {
        LineOrientation::Horizontal => range_x,
        LineOrientation::Vertical => range_y,
        LineOrientation::NotClear => range_none,
    };

    let mut x_range_points = PointList::new();
    for range_x_item in line_range.clone() {
        // println!("********** range_x {:?}", range_x_item);
        x_range_points.insert(Point { x: range_x_item, y: y1 }, 1);
    }

    let mut y_range_points = PointList::new();
    for range_y_item in line_range.clone() {
        // println!("********** range_y_item {:?}", range_y_item);

        y_range_points.insert(Point { x: x1, y: range_y_item }, 1);
    }
    // println!("---- y_range_points {:?}", y_range_points);
    // println!("---- line_range.clone() {:?}", line_range.clone());
    // println!("---- line_orientation {:?}", line_orientation);

    let range_points_result = match line_orientation {
        LineOrientation::Horizontal => x_range_points,
        LineOrientation::Vertical => y_range_points.clone(),
        LineOrientation::NotClear => PointList::new(),
    };

    // println!("---------- line_range.clone {:?}", line_range.clone());
    // println!("---------- line_orientation {:?}", line_orientation);
    // println!("---------- range_points_result {:?}", range_points_result);

    range_points_result
}


// TODO: check if point is already set and increment accordingly 
// https://stackoverflow.com/questions/30414424/how-can-i-update-a-value-in-a-mutable-hashmap
fn add_line_to_diagram(diagram: &PointList, line: &Line) -> PointList {
    let mut new_diagram = PointList::new();
    for (k,v) in diagram.into_iter() {
        new_diagram.insert(*k,*v);
    }
    let line_as_point_list = convert_line_into_point_list(line.clone());
    for (k,v) in line_as_point_list.into_iter() {
        let increment = match new_diagram.get(&k){
            Some(value) => value,
            None => &0,
        };
        new_diagram.insert(k,v + increment);
    }

    new_diagram
}

#[cfg(test)]
mod tests {

    use super::*;
    use pretty_assertions::{assert_eq, assert_ne};



    //TODO: convert_all_input_into_lines, filter Lines, convert Lines into individual Points, then 
    // add_point_to_hashmap
    // add more tests
    #[test]
    fn test_add_all_lines_to_diagram() {
        // let lines = match get_lines_from_file(get_input_file_name()) {
        //     Ok(line) => line,
        //     Err(error) => panic!("Error getting line {:?}", error),
        // };

        // let mut expected_point_list: PointList = PointList::from([
        //     (Point { x: 0, y: 0 }, 1),
        //     (Point { x: 0, y: 1 }, 2),
        // ]);
        // let expected_point_value = expected_point_list.get(&Point { x: 0, y: 1 }).unwrap();
        // println!("expected_point {:?}", expected_point_value);
        // let line = lines.get(&Point { x: 0, y: 1 }).unwrap();

        // let diagram = add_all_lines_to_diagram(&diagram, &lines);
        // let diagram_point =  diagram.get(&Point { x: 0, y: 1 }).unwrap();
        // assert_eq!(*expected_point_value, diagram_point);
    }


    #[test]
    fn test_add_line_to_diagram() {
        let mut diagram = PointList::new();
        let line_01 = Line { a: Point { x: 0, y: 0 }, b: Point { x: 0, y: 2 } };
        let line_02 = Line { a: Point { x: 0, y: 1 }, b: Point { x: 3, y: 1 } };
        let expected_point_list_01: PointList = PointList::from([
            (Point { x: 0, y: 0 }, 1),
            (Point { x: 0, y: 1 }, 1),
            (Point { x: 0, y: 2 }, 1),
        ]);
        let diagram = add_line_to_diagram(&diagram, &line_01);

        assert_eq!(expected_point_list_01, diagram);


        let mut expected_point_list_02: PointList = PointList::from([
            (Point { x: 0, y: 0 }, 0),
            (Point { x: 0, y: 1 }, 2),
            (Point { x: 0, y: 2 }, 1),
            (Point { x: 1, y: 1 }, 1),
            (Point { x: 2, y: 1 }, 1),
            (Point { x: 3, y: 1 }, 1),
        ]);
        println!("expected_point_list_02 {:?}", expected_point_list_02);
        let diagram = add_line_to_diagram(&diagram, &line_02);
        
        assert_eq!(expected_point_list_02, diagram);
    }


    #[test]
    fn test_convert_line_into_point_list() {
        let line = Line { a: Point { x: 0, y: 9 }, b: Point { x: 5, y: 9 } };
        let expected_point_list: PointList = PointList::from([
            (Point { x: 0, y: 9 }, 1),
            (Point { x: 1, y: 9 }, 1),
            (Point { x: 2, y: 9 }, 1),
            (Point { x: 3, y: 9 }, 1),
            (Point { x: 4, y: 9 }, 1),
            (Point { x: 5, y: 9 }, 1)
        ]);
        let points_result = convert_line_into_point_list(line);

        assert_eq!(expected_point_list, points_result);

        let line_y = Line { a: Point { x: 2, y: 9 }, b: Point { x: 2, y: 2 } };
        let expected_point_list_y: PointList = PointList::from([
            (Point { x: 2, y: 2 }, 1),
            (Point { x: 2, y: 3 }, 1),
            (Point { x: 2, y: 4 }, 1),
            (Point { x: 2, y: 5 }, 1),
            (Point { x: 2, y: 6 }, 1),
            (Point { x: 2, y: 7 }, 1),
            (Point { x: 2, y: 8 }, 1),
            (Point { x: 2, y: 9 }, 1),
        ]);
        let points_result_y = convert_line_into_point_list(line_y);

        assert_eq!(expected_point_list_y, points_result_y);


        let line_diagonal = Line { a: Point { x: 3, y: 1 }, b: Point { x: 2, y: 2 } };
        let expected_point_list_diagonal: PointList = PointList::new();

        let points_result_diagonal = convert_line_into_point_list(line_diagonal);
        assert_eq!(expected_point_list_diagonal, points_result_diagonal);

    }

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
