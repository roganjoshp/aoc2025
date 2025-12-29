use rayon::prelude::*;
use std::collections::HashSet;
use std::fs;

fn read_input(filename: &str) -> Vec<Vec<i64>> {
    let file_data = fs::read_to_string(filename).expect("Cannot find file");
    let ranges: Vec<Vec<i64>> = file_data
        .split(",")
        .map(|c| {
            c.split("-")
                .map(|digit| digit.parse::<i64>().unwrap())
                .collect::<Vec<i64>>()
        })
        .collect();
    ranges
}

fn check_for_split_repeats(range: &[i64]) -> i64 {
    let repeated_numbers: i64 = (range[0]..range[1] + 1)
        .filter(|val| {
            let str_val = val.to_string();
            let (str_start, str_end) = str_val.split_at(str_val.len() / 2);
            str_start == str_end
        })
        .sum();
    repeated_numbers
}

fn check_for_all_repeats(range: &[i64]) -> i64 {
    let max_split_digits = range[1].to_string().len() / 2;

    let all_invalid: HashSet<i64> = (range[0]..range[1])
        .filter(|val| {
            let char_vals: Vec<char> = val.to_string().chars().collect();
            if char_vals.len() == 1 {
                return false;
            }
            let mut is_valid = false;
            for x in (1..max_split_digits + 1) {
                let chunks: Vec<_> = char_vals.chunks(x).collect();
                let first = chunks[0];
                if chunks.iter().all(|&chunk| chunk == first) {
                    is_valid = true;
                    break;
                }
            }
            is_valid
        })
        .collect();
    all_invalid.iter().sum()
}

fn main() {
    let input = read_input("input.txt");
    let part_1: i64 = input
        .par_iter()
        .map(|val| check_for_split_repeats(val))
        .sum();
    println!("Part 1: {:?}", part_1);
    let part_2: i64 = input.par_iter().map(|val| check_for_all_repeats(val)).sum();
    println!("Part 2: {:?}", part_2);
}
