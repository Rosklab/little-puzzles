use std::collections::HashSet;
use std::fs::File;
use std::io::BufRead;
use std::io::BufReader;

#[derive(Hash, Eq, PartialEq, Clone, Copy, Debug)]
struct Position(i32, i32);

fn next_position(position: Position, direction: char) -> Position {
    return match direction {
        '>' => Position(position.0 + 1, position.1),
        '<' => Position(position.0 - 1, position.1),
        '^' => Position(position.0, position.1 + 1),
        'v' => Position(position.0, position.1 - 1),
        _ => position,
    };
}

fn find_house_count(input: String) -> usize {
    let mut positions: HashSet<Position> = HashSet::new();
    let mut last_position = Position(0, 0);

    positions.insert(last_position);

    for direction in input.chars() {
        last_position = next_position(last_position, direction);
        positions.insert(last_position);
    }

    return positions.len();
}

fn find_houses_count_two_delivery(input: String) -> usize {
    let mut positions: HashSet<Position> = HashSet::new();
    let mut first_delivery_last_position = Position(0, 0);
    let mut second_delivery_last_position = Position(0, 0);

    positions.insert(first_delivery_last_position);
    positions.insert(second_delivery_last_position);

    for (index, direction) in input.chars().enumerate() {
        if index % 2 == 0 {
            first_delivery_last_position = next_position(first_delivery_last_position, direction);
            positions.insert(first_delivery_last_position);
        } else {
            second_delivery_last_position = next_position(second_delivery_last_position, direction);
            positions.insert(second_delivery_last_position);
        }
    }

    return positions.len();
}

pub fn find_houses_count_from_file(buf_reader: BufReader<File>) {
    for (index, line) in buf_reader.lines().enumerate() {
        let result = find_house_count(line.unwrap());
        println!("Index {} result {}", index, result);
    }
}

pub fn find_houses_count_two_delivery_from_file(buf_reader: BufReader<File>) {
    for (index, line) in buf_reader.lines().enumerate() {
        let result = find_houses_count_two_delivery(line.unwrap());
        println!("Index {} result {}", index, result);
    }
}
