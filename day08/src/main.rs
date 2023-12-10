use std::collections::HashMap;
use std::fs;
use std::path::Path;
use num::integer::lcm;


use nom::{
    IResult,
    bytes::complete::tag,
    sequence::tuple,
    character::complete::alphanumeric1,
    multi::separated_list1,
};

fn main() {
    let path = Path::new("src/day8.txt");
    let content = fs::read_to_string(path).expect("Could not read in.");

    println!("Day8 Task1: {}", task1(content.clone(), "AAA"));
    println!("Day8 Task2: {}", task2(content.clone()));
}

fn task1(input: String, start: &str) -> usize {
    let (sequence, labyrinth_map) = parse_input(&input).expect("Parsing failed").1;

    let mut current = start;
    let mut count = 0;
    let mut found = false;
    while !found {
        match sequence.chars().nth(count%sequence.len()) {
            Some('L') => current = labyrinth_map.get(current).unwrap().0,
            Some('R') => current = labyrinth_map.get(current).unwrap().1,
            _ => (),
        }
        count += 1;
        if current == "ZZZ" {
            found = true;
        }
    }
    count
}

fn helper(input: String, start: &str) -> usize {
    let (sequence, labyrinth_map) = parse_input(&input).expect("Parsing failed").1;

    let mut current = start;
    let mut count = 0;
    let mut found = false;
    while !found {
        match sequence.chars().nth(count%sequence.len()) {
            Some('L') => current = labyrinth_map.get(current).unwrap().0,
            Some('R') => current = labyrinth_map.get(current).unwrap().1,
            _ => (),
        }
        count += 1;
        if current.chars().nth(2).expect("No three letters.") == 'Z' {
            found = true;
        }
    }
    count
}

fn task2(input: String) -> usize {
    let (sequence, labyrinth_map) = parse_input(&input).expect("Parsing failed").1;

    let mut currents = labyrinth_map.clone().into_keys().filter(|x| x.chars().nth(2).expect("No 3 letters?") == 'A').collect::<Vec<&str>>();
    let counts: Vec<usize> = currents.into_iter().map(|x| helper(input.clone(), x)).collect();
    let mut sol = *counts.first().expect("No first element") as i64;
    for count in counts {
        sol = lcm(sol, count as i64);
    }
    sol.try_into().unwrap()
}

fn parse_input(i: &str) -> IResult<&str, (String, HashMap<&str, (&str, &str)>)> {
    let (i, sequence) = parse_first_line(i)?;
    let (i, _) = tag("\n")(i)?;
    let (i, generated_map) = parse_map(i)?;
    Ok((i, (sequence.to_string(), generated_map)))
}

fn parse_first_line(i: &str) -> IResult<&str, &str> {
    let (i, (sequence, _)) = tuple((alphanumeric1, tag("\n")))(i)?;
    Ok((i, sequence))
}

fn parse_map(i: &str) -> IResult<&str, HashMap<&str, (&str, &str)>> {
    let (i, lines) = separated_list1(tag("\n"), parse_map_line)(i)?;
    let mut labyrinth_map: HashMap<&str, (&str, &str)> = HashMap::new();
    for line in &lines {
        labyrinth_map.insert(&line.0, (&line.1, &line.2));
    }
    Ok((i, labyrinth_map))
}

fn parse_map_line(i: &str) -> IResult<&str, (&str, &str, &str)> {
    let (i, (from, _, left, _, right, _)) = tuple((alphanumeric1, tag(" = ("), alphanumeric1, tag(", "), alphanumeric1, tag(")")))(i)?;
    Ok((i, (from, left, right)))
}


#[cfg(test)]
mod tests {
    use std::collections::HashMap;
    use std::fs;
    use super::parse_input;
    use std::path::Path;
    use super::task1;
    use super::task2;

    #[test]
    fn test_parsing() {
        let path = Path::new("src/day8_test.txt");
        let content = fs::read_to_string(path).expect("Could not read in.");

        let (sequence, labyrinth_map) = parse_input(&content).unwrap().1;
        let compare_map: HashMap<&str, (&str, &str)> = [
            ("AAA", ("BBB", "BBB")),
            ("BBB", ("AAA", "ZZZ")),
            ("ZZZ", ("ZZZ", "ZZZ")),
        ].into();
        assert_eq!(sequence, "LLR".to_string());
        assert_eq!(labyrinth_map, compare_map);
    }

    #[test]
    fn test_task1() {
        let path = Path::new("src/day8_test.txt");
        let content = fs::read_to_string(path).expect("Could not read in.");

        assert_eq!(task1(content.clone(), "AAA"), 6);
    }

    #[test]
    fn test_task2() {
        let path = Path::new("src/day8_test2.txt");
        let content = fs::read_to_string(path).expect("Could not read in.");

        assert_eq!(task2(content.clone()), 6);
    }
}