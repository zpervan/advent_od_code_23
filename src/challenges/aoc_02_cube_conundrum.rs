use regex::Regex;
use crate::challenges::read_input_file_by_line;

const RED_CUBES_COUNT: u32 = 12;
const GREEN_CUBES_COUNT: u32 = 13;
const BLUE_CUBES_COUNT: u32 = 14;

pub fn part_01() {
    let inputs = read_input_file_by_line("src/data/02_input.txt");

    let game_num_regex = Regex::new(r"\b\d{1,3}:").unwrap();
    let cube_count_regex = Regex::new(r"\b(\d+) (\w+);?\b").unwrap();

    let mut game_num = 0;
    let mut sum_of_games = 0;
    let mut is_possible = true;

    for input in inputs {
        for result in game_num_regex.find_iter(input.as_str()) {
            game_num = result.as_str().trim_end_matches(":").parse().unwrap();
            println!("Game number: {}", result.as_str().trim_end_matches(":"));
        }

        for values in cube_count_regex.captures_iter(input.as_str()) {
            println!("Value: {}", &values[1]);
            println!("Color: {}", &values[2]);

            // If it has more red cubes than available
            if &values[2] == "red" && values[1].parse::<u32>().unwrap() > RED_CUBES_COUNT {
                println!("---- Exceeding RED cube availability");
                is_possible = false;
            }

            // If it has more green cubes than available
            if &values[2] == "green" && values[1].parse::<u32>().unwrap() > GREEN_CUBES_COUNT {
                println!("---- Exceeding GREEN cube availability");
                is_possible = false;
            }

            // If it has more blue cubes than available
            if &values[2] == "blue" && values[1].parse::<u32>().unwrap() > BLUE_CUBES_COUNT {
                println!("---- Exceeding BLUE cube availability");
                is_possible = false;
            }
        }

        if is_possible {
            println!("Summing game number");
            sum_of_games += game_num;
        }

        // Reset the value
        is_possible = true;
    }

    println!("---- Final game sum: {}", sum_of_games);
}

pub fn part_02() {
    let inputs = read_input_file_by_line("src/data/02_input.txt");
    let cube_count_regex = Regex::new(r"\b(\d+) (\w+);?\b").unwrap();
    let mut sum_of_powers = 0;

    for input in inputs {
        let mut red_num = 0;
        let mut green_num = 0;
        let mut blue_num = 0;

        for values in cube_count_regex.captures_iter(input.as_str()) {
            let captured_value = values[1].parse::<u32>().unwrap();

            if &values[2] == "red" && captured_value > red_num {
                red_num = captured_value;
            }

            if &values[2] == "green" && captured_value > green_num {
                green_num = captured_value;
            }

            if &values[2] == "blue" && captured_value > blue_num {
                blue_num = captured_value;
            }
        }

        if red_num > 0 && green_num > 0 && blue_num > 0 {
            sum_of_powers += red_num * green_num * blue_num;
            println!("Summing powers value: {}", sum_of_powers);
            println!("Minimal values - Red: {}, Green: {}, Blue: {}", red_num, green_num, blue_num);
        } else {
            println!("-- MISSING CUBE -- Red: {}, Green: {}, Blue: {}", red_num, green_num, blue_num);
        }
    }

    println!("---- Final sum: {}", sum_of_powers);
}