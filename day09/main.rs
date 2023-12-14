use std::fs;
use std::path::Path;

fn main() {
    let input = parse_input("day09/day9.txt");
    let series: i64 = input.clone().into_iter().map(|x| calculate_distances(x)).sum();
    let predict: i64 = input.clone().into_iter().map(|x| predict_first_elements(x)).sum();
    println!("Day9 Task1: {}", series);
    println!("Day9 Task2: {}", predict);
}

fn parse_input(path: &str) -> Vec<Vec<i64>> {
    let path = Path::new(path);
    let contents = fs::read_to_string(path).expect("Could not read file.");
    return contents
        .split("\n")
        .map(|line| line.split(" ").map(|value| value.parse::<i64>().unwrap()).collect()).collect();
}

fn calculate_distances(input: Vec<i64>) -> i64 {
    let mut distances: Vec<Vec<i64>> = vec![];
    distances.push(input);

    loop {
        let last_vector = distances.last().unwrap();
        if (last_vector.into_iter().filter(|x| **x == 0).count() == last_vector.len()
        || last_vector.len() == 1) {
            break;
        }

        let mut new_vec: Vec<i64> = vec![];
        for index in 0..(last_vector.len()-1) {
            new_vec.push((last_vector.get(index + 1).unwrap() - last_vector.get(index).unwrap()));
        }
        distances.push(new_vec);
    }

    loop {
        if distances.len() <= 1 {
            break;
        }
        let last_vector = distances.pop().unwrap();
        let mut second_last = distances.pop().unwrap();

        second_last.push(second_last.last().unwrap() + last_vector.last().unwrap());
        distances.push(second_last);
    }
    *distances.first().unwrap().last().unwrap()
}

fn predict_first_elements(input: Vec<i64>) -> i64 {
    let mut distances: Vec<Vec<i64>> = vec![];
    distances.push(input);

    loop {
        let last_vector = distances.last().unwrap();
        if last_vector.into_iter().filter(|x| **x == 0).count() == last_vector.len()
            || last_vector.len() == 1 {
            break;
        }

        let mut new_vec: Vec<i64> = vec![];
        for index in 0..(last_vector.len()-1) {
            new_vec.push(last_vector.get(index + 1).unwrap() - last_vector.get(index).unwrap());
        }
        distances.push(new_vec);
    }

    loop {
        if distances.len() <= 1 {
            break;
        }
        let last_vector = distances.pop().unwrap();
        let mut second_last = distances.pop().unwrap();

        second_last.insert(0, second_last.first().unwrap() - last_vector.first().unwrap());
        distances.push(second_last);
    }
    *distances.first().unwrap().first().unwrap()
}


#[cfg(test)]
mod tests {
    use super::{parse_input, predict_first_elements};
    use super::calculate_distances;

    #[test]
    fn parse_example_input() {
        let compare: Vec<Vec<i64>> = vec![
          vec![0, 3, 6, 9, 12, 15],
          vec![1, 3, 6, 10, 15, 21],
          vec![10, 13, 16, 21, 30, 45],
        ];
        assert_eq!(parse_input("day09/example.txt"), compare);
    }

    #[test]
    fn test_calculate_distances() {
        let mut input = parse_input("day09/example.txt");

        assert_eq!(calculate_distances(input.pop().unwrap()), 68);
    }

    #[test]
    fn test_example_solution() {
        let input = parse_input("day09/example.txt");
        let series: i64 = input.into_iter().map(|x| calculate_distances(x)).sum();
        assert_eq!(series, 114);
    }

    #[test]
    fn test_example_part_2() {
        let mut input = parse_input("day09/example.txt");
        assert_eq!(predict_first_elements(input.pop().unwrap()), 5);
    }

    #[test]
    fn test_example_solution_part_2() {
        let input = parse_input("day09/example.txt");
        let series: i64 = input.into_iter().map(|x| predict_first_elements(x)).sum();
        assert_eq!(series, 2);
    }
}
