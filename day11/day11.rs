use itertools::Itertools;
use num::abs;
use std::fs;
use std::path::Path;

fn main() {
    task1();
    task2_try2();
}

fn task1() {
    let input_string = read_file("day11/input");
    let mut input: Vec<Vec<char>> = input_string
        .split("\n")
        .map(|line| line.chars().collect())
        .collect();

    let mut row_index = 0;
    loop {
        if row_index >= input.len() {
            break;
        }
        let mut galaxy = false;
        for column_index in 0..input[row_index].len() {
            if input[row_index][column_index] == '#' {
                galaxy = true;
            }
        }
        if !galaxy {
            let append_row = vec!['.'; input[row_index].len()];
            input.insert(row_index, append_row);
            row_index += 1;
        }
        row_index += 1;
    }

    let mut column_index = 0;
    loop {
        if column_index >= input[0].len() {
            break;
        }
        let mut galaxy = false;
        for row_index in 0..input.len() {
            if input[row_index][column_index] == '#' {
                galaxy = true;
            }
        }
        if !galaxy {
            for row_index in 0..input.len() {
                input[row_index].insert(column_index, '.');
            }
            column_index += 1;
        }
        column_index += 1;
    }

    let mut galaxies: Vec<(usize, usize)> = vec![];
    for row in 0..input.len() {
        for column in 0..input[row].len() {
            if input[row][column] == '#' {
                galaxies.push((row, column));
            }
        }
    }
    let solution: i32 = galaxies
        .into_iter()
        .tuple_combinations()
        .map(|(a, b)| abs(a.0 as i32 - b.0 as i32) + abs(a.1 as i32 - b.1 as i32))
        .sum();
    println!("Day11 Task1: {}", solution);
    // 0, 3 1, 7 2, 0
}

fn task2_try2() {
    let input_string = read_file("day11/input");
    let mut input: Vec<Vec<char>> = input_string
        .split("\n")
        .map(|line| line.chars().collect())
        .collect();

    let mut row_index = 0;
    loop {
        if row_index >= input.len() {
            break;
        }
        let mut galaxy = false;
        for column_index in 0..input[row_index].len() {
            if input[row_index][column_index] == '#' {
                galaxy = true;
            }
        }
        if !galaxy {
            let append_row = vec!['/'; input[row_index].len()];
            input[row_index] = append_row;
        }
        row_index += 1;
    }

    let mut column_index = 0;
    loop {
        if column_index >= input[0].len() {
            break;
        }
        let mut galaxy = false;
        for row_index in 0..input.len() {
            if input[row_index][column_index] == '#' {
                galaxy = true;
            }
        }
        if !galaxy {
            for row_index in 0..input.len() {
                input[row_index][column_index] = '/';
            }
        }
        column_index += 1;
    }

    let mut galaxies: Vec<(usize, usize)> = vec![];
    for row in 0..input.len() {
        for column in 0..input[row].len() {
            if input[row][column] == '#' {
                let mut final_col = 0;
                for index in 0..=column {
                    if input[0][index] == '/' {
                        final_col += 1_000_000;
                    } else {
                        final_col += 1;
                    }
                }
                let mut final_row = 0;
                for index in 0..=row {
                    if input[index][0] == '/' {
                        final_row += 1_000_000;
                    } else {
                        final_row += 1;
                    }
                }
                galaxies.push((final_row, final_col));
            }
        }
    }
    let solution: i64 = galaxies
        .into_iter()
        .tuple_combinations()
        .map(|(a, b)| abs(a.0 as i64 - b.0 as i64) + abs(a.1 as i64 - b.1 as i64))
        .sum();
    println!("Day11 Task2: {}", solution);
    // 0, 3 1, 7 2, 0
}

fn read_file(path: &str) -> String {
    let path = Path::new(path);
    fs::read_to_string(path).expect("Could not read file.")
}
