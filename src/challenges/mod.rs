use std::fs::File;
use std::io;
use std::io::BufRead;

pub mod challenges_2023;
pub mod challenges_2024;

fn read_input_file_by_line(path: &str) -> Vec<String> {
    let file = File::open(path).expect("Should contain a input file");
    let reader = io::BufReader::new(file);

    reader.lines().map(|line| line.expect("Should contain a line")).collect::<Vec<String>>()
}