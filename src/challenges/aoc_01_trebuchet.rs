use std::collections::HashMap;
use crate::challenges::read_input_file_by_line;

pub fn part_01() {
    // Read the input file contents
    let input = read_input_file_by_line("src/data/01_p01_input.txt");

    // Setup
    let mut sum = 0;

    // Iterate through all inputs
    for (i, input) in input.iter().enumerate() {
        let digits = input
            .chars()
            .filter(|e| e.is_numeric())
            .map(|e| match e.to_digit(10) {
                None => 0,
                Some(value) => value,
            })
            .collect::<Vec<u32>>();

        println!("Values at {:?}: {:?}", i, digits);

        if let (Some(first), Some(last)) = (digits.first(), digits.last()) {
            sum += (first * 10) + last;
        }
    }

    println!("Sum: {:?}", sum);
}

pub fn part_02() {
    let input = read_input_file_by_line("src/data/01_p02_input.txt");

    // Examples
    // let input  = vec![
    //     "two1nine",
    //     "eightwothree",
    //     "abcone2threexyz",
    //     "xtwone3four",
    //     "4nineeightseven2",
    //     "zoneight234",
    //     "7pqrstsixteen",
    //     "njnjnj"
    // ];

    let valid_digits = HashMap::<&str, u32>::from([
        ("one", 1),
        ("two", 2),
        ("three", 3),
        ("four", 4),
        ("five", 5),
        ("six", 6),
        ("seven", 7),
        ("eight", 8),
        ("nine", 9),
    ]);

    let mut sum = 0;

    for word in input {
        let word_chars = word.chars().collect::<Vec<char>>();
        let mut digits = Vec::<u32>::new();
        let mut last_position = 0usize;

        for i in 0..word.len() {
            if i < last_position {
                continue;
            }

            // Check if it's a digit
            if let Some(char) = word_chars.get(i) {
                if char.is_numeric() {
                    digits.push(char.to_digit(10).expect("Should be a valid digit"));
                    // Skip the rest
                    continue;
                }
            }

            // Check if it's a word
            for j in 3..=5 {
                if i + j <= word.len() {
                    let temp_word = &word[i..i + j];

                    if let Some(digit) = valid_digits.get(temp_word) {
                        digits.push(*digit);
                        last_position = i + j;
                        break;
                    }
                }
            }
        }

        println!("Digits: {:?}", digits);

        if let (Some(first), Some(last)) = (digits.first(), digits.last()) {
            sum += (first * 10) + last;
        }
    }

    println!("Sum: {}", sum);
}