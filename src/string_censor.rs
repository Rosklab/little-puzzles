use std::collections::HashMap;
use std::fs::File;
use std::io::BufRead;
use std::io::BufReader;

const VOWELS: [char; 5] = ['a', 'e', 'i', 'o', 'u'];
const FORBIDDEN_PAIRS: [[char; 2]; 4] = [['a', 'b'], ['c', 'd'], ['p', 'q'], ['x', 'y']];

fn is_nice_string_first_set_of_rules(input: String) -> bool {
    let mut vowel_count = 0;
    let mut double_letter = false;
    let mut contain_forbidden_pair = false;

    let mut last_letter_option = None;
    for current_letter in input.chars() {
        if VOWELS.binary_search(&current_letter).is_ok() {
            vowel_count += 1;
        }

        if last_letter_option.is_some() {
            let last_letter = last_letter_option.unwrap();
            if current_letter == last_letter {
                double_letter = true;
            } else {
                let letter_pairs = [last_letter, current_letter];
                if FORBIDDEN_PAIRS.binary_search(&letter_pairs).is_ok() {
                    contain_forbidden_pair = true;
                    break;
                }
            }
        }

        last_letter_option = Some(current_letter);
    }

    return vowel_count >= 3 && double_letter && !contain_forbidden_pair;
}

fn is_nice_string_second_set_of_rules(input: String) -> bool {
    let mut double_leter_with_one_between = false;
    let mut repeated_pair = false;

    let mut pair_map: HashMap<[char; 2], usize> = HashMap::new();

    let mut last_letter_option = None;
    let mut second_last_letter_option = None;
    for (index, current_letter) in input.chars().enumerate() {
        if !repeated_pair && last_letter_option.is_some() {
            let last_letter = last_letter_option.unwrap();

            let current_pair = [last_letter, current_letter];
            let exist_index_option = pair_map.get(&current_pair);

            match exist_index_option {
                Some(exist_index) => {
                    repeated_pair = (index - exist_index) > 1;
                }
                None => {
                    pair_map.insert(current_pair, index);
                }
            }

            if repeated_pair && double_leter_with_one_between {
                return true;
            }
        }

        if !double_leter_with_one_between && second_last_letter_option.is_some() {
            let second_last_letter = second_last_letter_option.unwrap();

            if current_letter == second_last_letter {
                double_leter_with_one_between = true;
                if repeated_pair {
                    return true;
                }
            }
        }

        second_last_letter_option = last_letter_option;
        last_letter_option = Some(current_letter);
    }

    return false;
}

pub fn count_nice_string_first_set_of_rules(buf_reader: BufReader<File>) {
    let mut nice_string_count = 0;
    for (index, line) in buf_reader.lines().enumerate() {
        let result = is_nice_string_first_set_of_rules(line.unwrap());
        if result {
            println!("Line {} contains nice string", index + 1);
            nice_string_count += 1;
        }
    }
    println!("Nice string count {nice_string_count}");
}

pub fn count_nice_string_second_set_of_rules(buf_reader: BufReader<File>) {
    let mut nice_string_count = 0;
    for (index, line) in buf_reader.lines().enumerate() {
        let result = is_nice_string_second_set_of_rules(line.unwrap());
        if result {
            println!("Line {} contains nice string", index + 1);
            nice_string_count += 1;
        }
    }
    println!("Nice string count {nice_string_count}");
}
