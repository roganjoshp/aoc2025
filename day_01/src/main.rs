use anyhow::{Context, Result};
use std::fs::File;
use std::io::{self, BufRead};

fn read_lines(filename: &str) -> Result<io::Lines<io::BufReader<File>>> {
    let file =
        File::open(filename).with_context(|| format!("Unable to find file: {}", filename))?;
    Ok(io::BufReader::new(file).lines())
}

fn calc_move_result(curr_value: i32, movement: &str) -> i32 {
    let (direction, distance) = movement.split_at(1);
    let distance = distance.parse::<i32>().unwrap().rem_euclid(100);

    match direction {
        "L" => return (curr_value - distance) % 100,
        "R" => return (curr_value + distance) % 100,
        _ => panic!("Unkown direction"),
    }
}

fn count_rotations(curr_value: i32, movement: &str) -> i32 {
    let (direction, distance) = movement.split_at(1);
    let distance = distance.parse::<i32>().unwrap();

    let tick = match direction {
        "L" => -1,
        "R" => 1,
        _ => panic!("Unknown direction"),
    };

    let mut num_ticks = 0;
    let mut safe_position = curr_value;

    for _ in 0..distance {
        safe_position += tick;
        safe_position %= 100;
        if safe_position == 0 {
            num_ticks += 1;
        }
    }
    num_ticks
}

fn main() {
    let file = read_lines("input01.txt").unwrap();
    let mut curr_safe_value: i32 = 50;
    let mut move_result: i32;

    let mut rotation_count: i32;

    let mut total_zero_count = 0;
    let mut total_rotation_count: i32 = 0;

    for row in file {
        match row {
            Ok(row) => {
                move_result = calc_move_result(curr_safe_value, &row);
                rotation_count = count_rotations(curr_safe_value, &row);
            }
            Err(e) => continue,
        }
        if move_result == 0 {
            total_zero_count += 1;
        }
        total_rotation_count += rotation_count;

        curr_safe_value = move_result;
    }

    println!("Part 1: {:?}", total_zero_count);
    println!("Part 2: {:?}", total_rotation_count);
}
