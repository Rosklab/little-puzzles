use std::env;
use std::fs::File;
use std::io;
use std::io::BufRead;
use std::io::BufReader;
use std::process;

const PUZZLE_2015_01_01: &str = "2015_01_01";
const PUZZLE_2015_01_02: &str = "2015_01_02";
const PUZZLES: [&str; 2] = [
    PUZZLE_2015_01_01,
    PUZZLE_2015_01_02
];

struct Config {
    puzzle: String,
    file_path: String
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

fn read_lines(file_path: String) -> io::Lines<BufReader<File>> {
    let file = File::open(file_path).unwrap();
    return io::BufReader::new(file).lines();
}

fn find_last_floor(input: String) -> i32 {
    let mut floor = 0;
    for direction in input.chars() {
        floor += match direction {
            '(' => 1,
            ')' => -1,
            _ => 0
        };
    }
    return floor;
}

fn find_first_basement_position(input: String) -> i32 {
    let mut floor = 0;
    let mut position = 0;

    for direction in input.chars() {
        position += 1;

        floor += match direction {
            '(' => 1,
            ')' => -1,
            _ => 0
        };

        if floor == -1 {
            return position;
        }
    }

    return -1;
}

fn main() {
    let args: Vec<String> = env::args().collect();
    let config = parse_arguments(args);
    let puzzle = config.puzzle.as_str();
    if !PUZZLES.contains(&puzzle) {
        println!("Incorrect puzzle");
        process::exit(1);
    }
    let lines = read_lines(config.file_path);

    for (index, line) in lines.enumerate() {
        let input = line.unwrap();
        let result = match puzzle {
            PUZZLE_2015_01_01 => find_last_floor(input),
            PUZZLE_2015_01_02 => find_first_basement_position(input),
            _ => panic!()
        };
        println!("Index {index} result {result}");
    }
}
