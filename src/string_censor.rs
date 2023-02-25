use std::fs::File;
use std::io::BufRead;
use std::io::BufReader;

const VOWELS: [char; 5] = ['a', 'e', 'i', 'o', 'u'];
const FORBIDDEN_PAIRS: [[char; 2]; 4] = [['a', 'b'], ['c', 'd'], ['p', 'q'], ['x', 'y']];

fn is_nice_string(input: String) -> bool {
    let mut vowel_count = 0;
    let mut double_letter = false;
    let mut contain_forbidden_pair = false;

    let mut last_char_option = None;
    for current_char in input.chars() {
        if VOWELS.binary_search(&current_char).is_ok() {
            vowel_count += 1;
        }

        if last_char_option.is_some() {
            let last_char = last_char_option.unwrap();
            if current_char == last_char {
                double_letter = true;
            } else {
                let char_pairs = [last_char, current_char];
                if FORBIDDEN_PAIRS.binary_search(&char_pairs).is_ok() {
                    contain_forbidden_pair = true;
                    break;
                }
            }
        }

        last_char_option = Some(current_char);
    }

    return vowel_count >= 3 && double_letter && !contain_forbidden_pair;
}

pub fn count_nice_string(buf_reader: BufReader<File>) {
    let mut nice_string_count = 0;
    for (index, line) in buf_reader.lines().enumerate() {
        let result = is_nice_string(line.unwrap());
        println!("Index {index} result {result}");
        if result {
            nice_string_count += 1;
        }
    }
    println!("Nice string count {nice_string_count}");
}
