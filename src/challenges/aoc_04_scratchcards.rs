use std::collections::HashMap;
use crate::challenges::read_input_file_by_line;

type CardNo = u32;
type CardCount = u32;

pub fn part_01() {
    let inputs = read_input_file_by_line("src/data/04_example.txt");
    let mut sum = 0;

    for input in inputs {
        let split_values = input.split(|c| c == '|' || c == ':').collect::<Vec<&str>>();
        let winning_numbers = split_values[1].split_whitespace().collect::<Vec<&str>>();
        let remaining_numbers = split_values[2].split_whitespace().collect::<Vec<&str>>();

        let mut hits = -1;
        for remaining_number in remaining_numbers {
            if winning_numbers.contains(&remaining_number) {
                hits += 1i32;
            }
        }

        if hits >= 0 {
            sum += 2i32.pow(hits as u32);
        }
    }

    println!("=== Points: {}", sum);
}

pub fn part_02() {
    let inputs = read_input_file_by_line("src/data/04_input.txt");

    let mut cards: HashMap<CardNo, CardCount> = HashMap::new();

    for (processed_card_idx, input) in inputs.iter().enumerate() {
        // Add original value
        if !cards.contains_key(&(processed_card_idx as u32)) {
            cards.insert(processed_card_idx as u32, 0);
        }

        *cards.get_mut(&(processed_card_idx as u32)).unwrap() += 1;

        let split_values = input.split(|c| c == '|' || c == ':').collect::<Vec<&str>>();
        let winning_numbers = split_values[1].split_whitespace().collect::<Vec<&str>>();
        let remaining_numbers = split_values[2].split_whitespace().collect::<Vec<&str>>();

        // Check for numbers on scratchcard
        let mut hits = 0usize;
        for remaining_number in remaining_numbers {
            if winning_numbers.contains(&remaining_number) {
                hits += 1;
            }
        }

        // Adding scratchcards to map
        let value = *cards.get(&(processed_card_idx as u32)).unwrap_or(&(0 as CardCount));
        let start_card = processed_card_idx + 1;

        for j in start_card .. start_card + hits {
            if !cards.contains_key(&(j as u32)) {
                cards.insert(j as u32, 0);
            }

           *cards.get_mut(&(j as u32)).unwrap() += value;
        }
    }

    let cards_sum: u32 = cards.values().sum();

    println!("=== Sum cards: {}", cards_sum);
}