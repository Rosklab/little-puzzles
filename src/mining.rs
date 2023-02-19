use std::fs::File;
use std::io::BufRead;
use std::io::BufReader;

fn find_mining_number(input: String, prefix: &str) -> i32 {
    let mut mining_number = 1;
    loop {
        let data = format!("{input}{mining_number}");
        let result = format!("{:x}", md5::compute(data));
        if result.starts_with(prefix) {
            break;
        }
        mining_number += 1;
    }
    return mining_number;
}

pub fn find_mining_number_for_five_zeroes_from_file(buf_reader: BufReader<File>) {
    for (index, line) in buf_reader.lines().enumerate() {
        let result = find_mining_number(line.unwrap(), "00000");
        println!("Index {} result {}", index, result);
    }
}

pub fn find_mining_number_for_six_zeroes_from_file(buf_reader: BufReader<File>) {
    for (index, line) in buf_reader.lines().enumerate() {
        let result = find_mining_number(line.unwrap(), "000000");
        println!("Index {} result {}", index, result);
    }
}
