use array2d::Array2D;
use std::collections::{HashMap, HashSet};
use std::fs;
use std::path::Path;

fn main() {
    let elements_of_loop = task1((20, 102), (20, 103), "east", "day10/input");
    task2(&elements_of_loop, (126, 120), "day10/input");

    // let elements_of_loop = task1((1, 2), (1, 1), "north", "day10/example");
    // task2(&elements_of_loop, (0, 0), "day10/example");
}

fn task2(
    elements_of_loop: &HashMap<(usize, usize), Connection>,
    starting_pos: (usize, usize),
    path: &str,
) {
    let input_string = parse_input(path);
    let field = Array2D::from_rows(
        &input_string
            .split("\n")
            .map(|line| line.chars().collect())
            .collect::<Vec<Vec<char>>>(),
    )
    .expect("No possible :/");
    let dimensions = (field.num_rows(), field.num_columns());
    let mut padded_array = create_padded_array(dimensions, &field);
    let elements_with_padded_loop = create_padded_loop(&mut padded_array, elements_of_loop);
    write_array_with_only_loop(&mut padded_array, &elements_with_padded_loop);
    fill_array(&elements_with_padded_loop, &mut padded_array, starting_pos);

    let mut out_first = String::new();
    for row_index in 0..padded_array.num_rows() {
        for field in padded_array.row_iter(row_index).expect("No row.") {
            out_first.push(*field);
        }
        out_first.push('\n');
    }
    fs::write("day10/out", out_first).unwrap();

    let mut sum = 0;
    let mut out = String::new();
    for row_index in (0..padded_array.num_rows() - 1).step_by(2) {
        for field in padded_array
            .row_iter(row_index)
            .expect("No row.")
            .step_by(2)
        {
            if *field == 'X' {
                sum += 1;
            }
            out.push(*field);
        }
        out.push('\n');
    }
    fs::write("day10/out3", out).unwrap();
    println!("Day10 Task2: {}", sum);

    let inverse = dimensions.0 * dimensions.1 - 13804 - sum;
    println!("End with: {}", inverse);
}

fn write_array_with_only_loop(array: &mut Array2D<char>, pipe_loop: &HashSet<(usize, usize)>) {
    let mut out = String::new();
    for row_index in 0..array.num_rows() {
        for col_index in 0..array.num_columns() {
            if !pipe_loop.contains(&(row_index, col_index)) {
                array.get_mut(row_index, col_index).map(|x| *x = '.');
            }
            out.push(*array.get(row_index, col_index).unwrap())
        }
        out.push('\n');
    }
    fs::write("day10/out2", out).unwrap();
}

fn create_padded_array(dimensions: (usize, usize), field: &Array2D<char>) -> Array2D<char> {
    let (x_len, y_len) = dimensions;
    let mut padded_array = Array2D::filled_with('.', 2 * x_len - 1, 2 * y_len - 1);
    for (row_index, row) in field.rows_iter().enumerate() {
        for (col_index, row_char) in row.enumerate() {
            padded_array
                .get_mut(row_index * 2, col_index * 2)
                .map(|x| *x = *row_char);
        }
    }
    padded_array
}

fn create_padded_loop(
    array: &mut Array2D<char>,
    pipe_loop: &HashMap<(usize, usize), Connection>,
) -> HashSet<(usize, usize)> {
    let (x_len, y_len) = (array.num_rows(), array.num_columns());
    let mut padded_loop: HashMap<(usize, usize), Connection> = HashMap::new();
    for ((x_pos, y_pos), current_conn) in pipe_loop.iter() {
        padded_loop.insert((*x_pos * 2, *y_pos * 2), *current_conn);
    }
    for row in 0..array.num_rows() {
        for col in 0..array.num_columns() {
            if col > 0
                && col < y_len - 1
                && padded_loop.contains_key(&(row, col - 1))
                && padded_loop.contains_key(&(row, col + 1))
            {
                if padded_loop.get(&(row, col - 1)).unwrap().east
                    && padded_loop.get(&(row, col + 1)).unwrap().west
                {
                    array.get_mut(row, col).map(|x| *x = '-');
                    padded_loop.insert(
                        (row, col),
                        Connection {
                            north: true,
                            south: true,
                            ..Default::default()
                        },
                    );
                }
            } else if row > 0
                && row < x_len - 1
                && padded_loop.contains_key(&(row - 1, col))
                && padded_loop.contains_key(&(row + 1, col))
            {
                if padded_loop.get(&(row - 1, col)).unwrap().south
                    && padded_loop.get(&(row + 1, col)).unwrap().north
                {
                    array.get_mut(row, col).map(|x| *x = '|');
                    padded_loop.insert(
                        (row, col),
                        Connection {
                            east: true,
                            west: true,
                            ..Default::default()
                        },
                    );
                }
            }
        }
    }
    padded_loop
        .keys()
        .cloned()
        .collect::<HashSet<(usize, usize)>>()
}

fn fill_array(
    pipe_loop: &HashSet<(usize, usize)>,
    array: &mut Array2D<char>,
    starting_pos: (usize, usize),
) {
    //println!("Loop: {:?}", pipe_loop);

    let (x_len, y_len) = (array.num_rows(), array.num_columns());

    let mut visited_points: HashSet<(usize, usize)> = HashSet::new();
    let mut points_to_visit: HashSet<(usize, usize)> = HashSet::new();
    points_to_visit.insert(starting_pos);

    loop {
        //println!("Points to visit: {:?}", points_to_visit);
        if points_to_visit.is_empty() {
            break;
        }

        //println!("Before {:?}", points_to_visit);
        let (current_x, current_y) = *points_to_visit.iter().nth(0).unwrap();
        points_to_visit.remove(&(current_x, current_y));
        visited_points.insert((current_x, current_y));
        //println!("After {:?}", points_to_visit);
        //println!("Visited {:?}", visited_points);

        array.get_mut(current_x, current_y).map(|x| *x = 'X');

        if current_x > 0
            && !pipe_loop.contains(&(current_x - 1, current_y))
            && !visited_points.contains(&(current_x - 1, current_y))
        {
            points_to_visit.insert((current_x - 1, current_y));
            //println!("Appended: {} {}", current_x - 1, current_y);
        }
        if (current_x < x_len - 1)
            && !pipe_loop.contains(&(current_x + 1, current_y))
            && !visited_points.contains(&(current_x + 1, current_y))
        {
            points_to_visit.insert((current_x + 1, current_y));
            //println!("Appended: {} {}", current_x + 1, current_y);
        }
        if current_y > 0
            && !pipe_loop.contains(&(current_x, current_y - 1))
            && !visited_points.contains(&(current_x, current_y - 1))
        {
            points_to_visit.insert((current_x, current_y - 1));
            //println!("Appended: {} {}", current_x, current_y - 1);
        }
        if (current_y < y_len - 1)
            && !pipe_loop.contains(&(current_x, current_y + 1))
            && !visited_points.contains(&(current_x, current_y + 1))
        {
            points_to_visit.insert((current_x, current_y + 1));
            //println!("Appended: {} {}", current_x, current_y + 1);
        }
    }
}

fn task1(
    starting_pos: (usize, usize),
    compare_pos: (usize, usize),
    start_came_from: &str,
    path: &str,
) -> HashMap<(usize, usize), Connection> {
    let input_string = parse_input(path);
    let input = input_string.split("\n");

    let mut pipes: Vec<Vec<char>> = vec![];
    let mut current_pos: (usize, usize) = starting_pos;
    for line in input {
        let mut row_vec: Vec<char> = vec![];
        for information in line.chars() {
            row_vec.push(information)
        }
        pipes.push(row_vec);
    }

    let mut elements_of_loop: HashMap<(usize, usize), Connection> = HashMap::new();
    let mut came_from = start_came_from;
    let mut loop_length = 1;
    loop {
        elements_of_loop.insert(
            current_pos,
            connected_to(pipes[current_pos.0][current_pos.1]),
        );
        if current_pos == compare_pos {
            break;
        }

        let currently_connected_to = connected_to(pipes[current_pos.0][current_pos.1]);
        pipes[current_pos.0][current_pos.1] = 'X';
        if currently_connected_to.north && !(came_from == "north") {
            current_pos = (current_pos.0 - 1, current_pos.1);
            came_from = "south";
        } else if currently_connected_to.east && !(came_from == "east") {
            current_pos = (current_pos.0, current_pos.1 + 1);
            came_from = "west";
        } else if currently_connected_to.south && !(came_from == "south") {
            current_pos = (current_pos.0 + 1, current_pos.1);
            came_from = "north";
        } else if currently_connected_to.west && !(came_from == "west") {
            current_pos = (current_pos.0, current_pos.1 - 1);
            came_from = "east";
        }
        loop_length += 1;
    }
    println!("Distance: {}", loop_length / 2);
    println!("Full loop: {}", loop_length);
    elements_of_loop
}

fn parse_input(path: &str) -> String {
    let path = Path::new(path);
    fs::read_to_string(path).expect("Could not read file.")
}

#[derive(Debug, Copy, Clone)]
struct Connection {
    north: bool,
    east: bool,
    south: bool,
    west: bool,
}

fn connected_to(info: char) -> Connection {
    match info {
        '|' => Connection {
            north: true,
            south: true,
            ..Default::default()
        },
        '-' => Connection {
            east: true,
            west: true,
            ..Default::default()
        },
        'L' => Connection {
            north: true,
            east: true,
            ..Default::default()
        },
        'J' => Connection {
            north: true,
            west: true,
            ..Default::default()
        },
        '7' => Connection {
            west: true,
            south: true,
            ..Default::default()
        },
        'F' => Connection {
            east: true,
            south: true,
            ..Default::default()
        },
        'S' => Connection {
            south: true,
            west: true,
            ..Default::default()
        },
        _ => Connection {
            ..Default::default()
        },
    }
}

impl Default for Connection {
    fn default() -> Self {
        Connection {
            north: false,
            east: false,
            south: false,
            west: false,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::parse_input;

    #[test]
    fn test_parsing() {
        let input = parse_input("day10/input");
        println!("{}", input)
    }
}
