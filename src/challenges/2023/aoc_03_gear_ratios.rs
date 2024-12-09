use multimap::MultiMap;
use crate::challenges::read_input_file_by_line;

type Positions = Vec<i32>;

pub fn part_01() {
    let inputs = read_input_file_by_line("src/data/03_input.txt");

    // let inputs = vec![
    //     "467#.114..",
    //     "###*......",
    //     ".13#.@633.",
    //     "..........",
    //     "617.#.....",
    //     ".....+.58.",
    //     "..592.....",
    //     "......755.",
    //     "...$.*..8*",
    //     ".664.664.7",
    // ];

    let mut sum = 0;

    let mut upper_row_symbols: Vec<i32> = Vec::new();
    let mut current_row_digits: Vec<(String, Positions)> = Vec::new();
    let mut current_row_symbols: Vec<i32> = Vec::new();
    let mut bottom_row_digits: Vec<(String, Positions)> = Vec::new();
    let mut bottom_row_symbols: Vec<i32> = Vec::new();

    for i in 0..inputs.len() {
        // Fetch upper row (which was previously the current_row)
        if i > 0 {
            upper_row_symbols = current_row_symbols.clone();
        }

        // Process current row
        if current_row_symbols.is_empty() {
            // Only process it the first time
            (current_row_digits, current_row_symbols) = process_char_row(&inputs[i]);
        } else {
            current_row_digits = bottom_row_digits.clone();
            current_row_symbols = bottom_row_symbols.clone();
        }

        // Process bot row (the only one which needs to actually be processed)
        if i < inputs.len() - 1 {
            (bottom_row_digits, bottom_row_symbols) = process_char_row(&inputs[i + 1]);
        } else {
            bottom_row_digits.clear();
            bottom_row_symbols.clear();
        }

        println!("Upper symbols: {:?}", upper_row_symbols);
        println!("Current symbols: {:?}", current_row_symbols);
        println!("Current digits: {:?}", current_row_digits);
        println!("Bot symbols: {:?}", bottom_row_symbols);

        for (value, digit_positions) in current_row_digits {
            let mut is_part = false;

            is_part |= has_symbol_near(&digit_positions, &upper_row_symbols);
            is_part |= has_symbol_near(&digit_positions, &current_row_symbols);
            is_part |= has_symbol_near(&digit_positions, &bottom_row_symbols);

            if is_part {
                println!("Value {} is part", value);
                sum += value.parse::<i32>().unwrap();
                println!("Current sum: {}", sum);
            }
        }
        println!("############");
    }

    println!("----> Sum: {}", sum);
}

fn process_char_row(row: &str) -> (Vec<(String, Positions)>, Vec<i32>) {
    let mut value = String::new();
    let mut positions = Positions::new();
    let mut result_digits: Vec<(String, Positions)> = Vec::new();
    let mut result_symbols: Vec<i32> = Vec::new();

    // Iterate through the chars
    for (i, char) in row.char_indices() {
        if char.is_numeric() {
            value.push(char);
            positions.push(i as i32);
        }

        if char.ne(&'.') && !char.is_numeric() {
            result_symbols.push(i as i32);
        }

        if !value.is_empty() && !positions.is_empty() && (i == row.len() - 1 || !char.is_numeric()) {
            // Dump positions into multimap
            result_digits.push((value.clone(), positions.clone()));
            value.clear();
            positions.clear();
        }
    }

    (result_digits, result_symbols)
}

fn has_symbol_near(digit_positions: &Positions, symbol_positions: &Vec<i32>) -> bool {
    for symbol_position in symbol_positions {
        // Check top and bottom
        if digit_positions.contains(&symbol_position) {
            return true;
        }

        // Check left
        if (digit_positions.first().unwrap() - symbol_position).abs() == 1 {
            return true;
        }

        // Check right
        if (symbol_position - digit_positions.last().unwrap()).abs() == 1 {
            return true;
        }
    }

    false
}
