use std::env;
use std::fs::File;
use std::io::BufRead;
use std::io::BufReader;
use std::process;

mod house_present;
mod mining;

const PUZZLE_2015_01_01: &str = "2015_01_01";
const PUZZLE_2015_01_02: &str = "2015_01_02";
const PUZZLE_2015_02_01: &str = "2015_02_01";
const PUZZLE_2015_02_02: &str = "2015_02_02";
const PUZZLE_2015_03_01: &str = "2015_03_01";
const PUZZLE_2015_03_02: &str = "2015_03_02";
const PUZZLE_2015_04_01: &str = "2015_04_01";
const PUZZLE_2015_04_02: &str = "2015_04_02";

const PUZZLES: [&str; 8] = [
    PUZZLE_2015_01_01,
    PUZZLE_2015_01_02,
    PUZZLE_2015_02_01,
    PUZZLE_2015_02_02,
    PUZZLE_2015_03_01,
    PUZZLE_2015_03_02,
    PUZZLE_2015_04_01,
    PUZZLE_2015_04_02,
];

struct Config {
    puzzle: String,
    file_path: String,
}

fn parse_arguments(args: Vec<String>) -> Config {
    if args.len() < 3 {
        println!("Incorrect number of arguments");
        process::exit(1);
    }
    let puzzle = args[1].clone();
    let file_path = args[2].clone();
    return Config { puzzle, file_path };
}

fn read_lines(file_path: String) -> BufReader<File> {
    let file = File::open(file_path).unwrap();
    return BufReader::new(file);
}

fn find_last_floor(input: String) -> i32 {
    let mut floor = 0;
    for direction in input.chars() {
        floor += match direction {
            '(' => 1,
            ')' => -1,
            _ => 0,
        };
    }
    return floor;
}

fn find_last_floor_from_file(buf_reader: BufReader<File>) {
    for (index, line) in buf_reader.lines().enumerate() {
        let result = find_last_floor(line.unwrap());
        println!("Index {index} result {result}");
    }
}

fn find_first_basement_position(input: String) -> i32 {
    let mut floor = 0;
    let mut position = 0;

    for direction in input.chars() {
        position += 1;

        floor += match direction {
            '(' => 1,
            ')' => -1,
            _ => 0,
        };

        if floor == -1 {
            return position;
        }
    }

    return -1;
}

fn find_first_basement_position_from_file(buf_reader: BufReader<File>) {
    for (index, line) in buf_reader.lines().enumerate() {
        let result = find_first_basement_position(line.unwrap());
        println!("Index {index} result {result}");
    }
}

fn find_wrapping_paper_square(edge_sizes: Vec<i32>) -> i32 {
    let mut total_square = 0;
    let mut min_square: Option<i32> = None;

    for i in 0..edge_sizes.len() - 1 {
        for j in i + 1..edge_sizes.len() {
            let square = edge_sizes[i] * edge_sizes[j];
            if min_square.is_none() || min_square.unwrap() > square {
                min_square = Some(square);
            }
            total_square += square;
        }
    }

    // Formula of surface area for the cuboid rectangle: 2(ab+bc+ac).
    // Multiply by two sum of diferent faces.
    total_square *= 2;

    // Add min square for slack.
    total_square += min_square.unwrap();

    return total_square;
}

fn find_wrapping_paper_square_from_file(buf_reader: BufReader<File>) {
    let mut total_square = 0;
    for (index, line) in buf_reader.lines().enumerate() {
        let edge_sizes: Vec<i32> = line
            .unwrap()
            .split("x")
            .map(|e| e.parse().unwrap())
            .collect();
        let square = find_wrapping_paper_square(edge_sizes);
        println!("Index {index} square {square}");
        total_square += square;
    }
    println!("Total square {total_square}");
}

fn find_ribbon_length(edge_sizes: Vec<i32>) -> i32 {
    let mut total_length = 0;
    let mut volume = 1;
    let mut max_edge_size: Option<i32> = None;

    for edge_size in edge_sizes {
        volume *= edge_size;
        if max_edge_size.is_none() || max_edge_size.unwrap() < edge_size {
            total_length += max_edge_size.unwrap_or(0);
            max_edge_size = Some(edge_size);
            continue;
        }
        total_length += edge_size;
    }

    // Formula for perimeter: 2(a+b).
    // Multiply by two sum of edges sizes.
    total_length *= 2;

    // Add volume for bow.
    total_length += volume;

    return total_length;
}

fn find_ribbon_length_from_file(buf_reader: BufReader<File>) {
    let mut total_length = 0;
    for (index, line) in buf_reader.lines().enumerate() {
        let edge_sizes: Vec<i32> = line
            .unwrap()
            .split("x")
            .map(|e| e.parse().unwrap())
            .collect();
        let length = find_ribbon_length(edge_sizes);
        println!("Index {index} length {length}");
        total_length += length;
    }
    println!("Total length {total_length}");
}

fn main() {
    let args: Vec<String> = env::args().collect();
    let config = parse_arguments(args);

    let puzzle = config.puzzle.as_str();
    if !PUZZLES.contains(&puzzle) {
        println!("Incorrect puzzle");
        process::exit(1);
    }

    let buf_reader = read_lines(config.file_path);
    match puzzle {
        PUZZLE_2015_01_01 => find_last_floor_from_file(buf_reader),
        PUZZLE_2015_01_02 => find_first_basement_position_from_file(buf_reader),
        PUZZLE_2015_02_01 => find_wrapping_paper_square_from_file(buf_reader),
        PUZZLE_2015_02_02 => find_ribbon_length_from_file(buf_reader),
        PUZZLE_2015_03_01 => house_present::find_houses_count_from_file(buf_reader),
        PUZZLE_2015_03_02 => house_present::find_houses_count_two_delivery_from_file(buf_reader),
        PUZZLE_2015_04_01 => mining::find_mining_number_for_five_zeroes_from_file(buf_reader),
        PUZZLE_2015_04_02 => mining::find_mining_number_for_six_zeroes_from_file(buf_reader),
        _ => panic!(),
    }
}
