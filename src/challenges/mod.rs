use std::fs::File;
use std::io;
use std::io::BufRead;

pub mod aoc_01_trebuchet;
pub mod aoc_02_cube_conundrum;
pub mod aoc_03_gear_ratios;

fn read_input_file_by_line(path: &str) -> Vec<String> {
    let file = File::open(path).expect("Should contain a input file");
    let reader = io::BufReader::new(file);

    reader.lines().map(|line| line.expect("Should contain a line")).collect::<Vec<String>>()
}