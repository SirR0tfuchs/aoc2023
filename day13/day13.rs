use std::fs;
use std::path::Path;

fn main() {
    let input = parse_input("day13/input");

    let lava_islands = input.split("\n\n").collect::<Vec<&str>>();
    let mut sum: i64 = 0;
    for lava_island in lava_islands {
        let dimensions = get_dimensions(lava_island);
        let lava_island_string: Vec<char> = lava_island.replace("\n", "").chars().collect();
        match indicate_mirrored_row(dimensions, &lava_island_string) {
            Some(row_index) => {
                sum += 100 * (row_index as i64 + 1);
            }
            _ => {}
        }
        match indicate_mirrored_column(dimensions, &lava_island_string) {
            Some(col_index) => {
                sum += col_index as i64 + 1;
            }
            _ => {}
        }
    }
    println!("Day13 Task1: {}", sum);
}

fn indicate_mirrored_row(dimensions: (usize, usize), island: &Vec<char>) -> Option<usize> {
    let (x_len, y_len) = dimensions;
    let mut row_index: usize = 0;
    let mut current_row = get_nth_row(x_len, row_index, &island);

    while row_index < y_len - 1 {
        let next_row = get_nth_row(x_len, row_index + 1, &island);
        if current_row == next_row && check_row_reflection(dimensions, row_index, &island) {
            return Some(row_index);
        }
        row_index += 1;
        current_row = next_row;
    }
    None
}

fn indicate_mirrored_column(dimensions: (usize, usize), island: &Vec<char>) -> Option<usize> {
    let (x_len, _) = dimensions;
    let mut col_index: usize = 0;

    let mut current_col = get_nth_col(dimensions, col_index, &island);

    while col_index < x_len - 1 {
        let next_col = get_nth_col(dimensions, col_index + 1, &island);
        if current_col == next_col && check_col_reflection(dimensions, col_index, &island) {
            return Some(col_index);
        }
        col_index += 1;
        current_col = next_col;
    }
    None
}

fn check_row_reflection(
    dimension: (usize, usize),
    reflection_index: usize,
    island: &Vec<char>,
) -> bool {
    let (x_len, y_len) = dimension;
    let mut lower_bound: i32 = reflection_index as i32 - 1;
    let mut upper_bound = reflection_index + 2;

    while lower_bound >= 0 && upper_bound < y_len {
        if get_nth_row(x_len, lower_bound as usize, &island)
            != get_nth_row(x_len, upper_bound, &island)
        {
            return false;
        }
        lower_bound -= 1;
        upper_bound += 1;
    }
    true
}

fn check_col_reflection(
    dimensions: (usize, usize),
    reflection_index: usize,
    island: &Vec<char>,
) -> bool {
    let (x_len, _) = dimensions;
    let mut lower_bound: i32 = reflection_index as i32 - 1;
    let mut upper_bound = reflection_index + 2;

    while lower_bound >= 0 && upper_bound < x_len {
        if get_nth_col(dimensions, lower_bound as usize, &island)
            != get_nth_col(dimensions, upper_bound, &island)
        {
            return false;
        }
        lower_bound -= 1;
        upper_bound += 1;
    }
    true
}

fn get_nth_row(x_len: usize, n: usize, island: &Vec<char>) -> String {
    let mut row = String::new();

    for row_index in 0..x_len {
        row.push(island[row_index + n * x_len]);
    }
    row
}

fn get_nth_col(dimensions: (usize, usize), n: usize, island: &Vec<char>) -> String {
    let (x_len, y_len) = dimensions;
    let mut col = String::new();

    for col_index in 0..y_len {
        col.push(island[n + col_index * x_len]);
    }
    col
}

fn get_dimensions(island: &str) -> (usize, usize) {
    let x = island
        .split("\n")
        .collect::<Vec<&str>>()
        .first()
        .expect("No first element")
        .len();
    let y = island.matches("\n").count();
    (x, y + 1)
}

fn parse_input(path: &str) -> String {
    let path = Path::new(path);
    fs::read_to_string(path).expect("Could not read file.")
}

#[cfg(test)]
mod tests {
    use crate::{
        get_dimensions, get_nth_col, get_nth_row, indicate_mirrored_column, indicate_mirrored_row,
        parse_input,
    };

    #[test]
    fn test_parsing() {
        parse_input("day13/example");
    }

    #[test]
    fn test_col() {
        let input = parse_input("day13/example");
        let lava_island = input.split("\n\n").collect::<Vec<&str>>();
        let dimensions = get_dimensions(lava_island.first().expect("No first"));
        let first_island: Vec<char> = lava_island
            .first()
            .expect("No first")
            .replace("\n", "")
            .chars()
            .collect();

        assert_eq!(
            get_nth_col(dimensions, 0, &first_island),
            "#.##..#".to_string()
        )
    }

    #[test]
    fn test_row() {
        let input = parse_input("day13/example");
        let lava_island = input.split("\n\n").collect::<Vec<&str>>();
        let dimensions = get_dimensions(lava_island.first().expect("No first"));
        let first_island: Vec<char> = lava_island
            .first()
            .expect("No first")
            .replace("\n", "")
            .chars()
            .collect();

        assert_eq!(
            get_nth_row(dimensions.0, 0, &first_island),
            "#.##..##.".to_string()
        )
    }

    #[test]
    fn test_example() {
        let input = parse_input("day13/example");
        let lava_islands = input.split("\n\n").collect::<Vec<&str>>();
        let mut sum: i64 = 0;
        for lava_island in lava_islands {
            let dimensions = get_dimensions(lava_island);
            let lava_island_string: Vec<char> = lava_island.replace("\n", "").chars().collect();
            match indicate_mirrored_row(dimensions, &lava_island_string) {
                Some(row_index) => {
                    sum += 100 * (row_index as i64 + 1);
                }
                _ => {}
            }
            match indicate_mirrored_column(dimensions, &lava_island_string) {
                Some(col_index) => {
                    sum += col_index as i64 + 1;
                }
                _ => {}
            }
        }
        assert_eq!(sum, 405)
    }
}
