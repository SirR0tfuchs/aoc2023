use std::fs;
use std::path::Path;

use nom::{
    character::complete as cc,
    character::complete::space1,
    multi::separated_list1,
    bytes::complete::tag,
    character::complete::alphanumeric1,
    sequence::tuple,
    IResult,
};

use std::iter::zip;

struct Race {
    time: i64,
    distance: i64,
}

impl Race {
    fn count_winning_races(&self) -> i64 {
        let mut num_winning_races = 0;
        let quot = (self.time + 1) / 2;
        let remainder = self.time % 2;

        for i in 1..=quot {
            let remaining_time = self.time - i;
            let boat_distance = remaining_time * i;
            if boat_distance > self.distance {
                num_winning_races += 1;
            }
        }
        num_winning_races *= 2;
        if num_winning_races != 0 && remainder != 1 {
            num_winning_races -= 1;
        } else if num_winning_races != 0 && remainder != 0 {
            num_winning_races -= 2;
        }
        return num_winning_races
    }
}

fn main() {
    let path = Path::new("day06/day6.txt");
    let contents = fs::read_to_string(path).expect("Should have been able to read file.");

    day6_task1(contents.clone());
    day6_task2(contents.clone());
}

fn day6_task1(input: String) {
    let races = parse_input(&input).unwrap().1;
    let solution = races.into_iter().map(|race| race.count_winning_races()).product::<i64>();
    println!("Day6 Task1: {}", solution);
}

fn parse_input(i: &str) -> IResult<&str, Vec<Race>> {
    let (i, (..)) = tuple((alphanumeric1, tag(":"), space1))(i)?;
    let (i, (list1, _)) = tuple((separated_list1(space1, cc::i64), tag("\n")))(i)?;
    let (i, (..)) = tuple((alphanumeric1, tag(":"), space1))(i)?;
    let (i, (list2, _)) = tuple((separated_list1(space1, cc::i64), tag("\n")))(i)?;

    let races = zip(list1, list2).map(|(time, distance)| Race{time, distance}).collect();
    Ok((i, races))
}

fn parse_input_task2(i: &str) -> IResult<&str, Race> {
    let (i, (..)) = tuple((alphanumeric1, tag(":"), space1))(i)?;
    let (i, (list1, _)) = tuple((separated_list1(space1, alphanumeric1), tag("\n")))(i)?;
    let (i, (..)) = tuple((alphanumeric1, tag(":"), space1))(i)?;
    let (i, (list2, _)) = tuple((separated_list1(space1, alphanumeric1), tag("\n")))(i)?;
    let mut time = String::new();
    let mut distance = String::new();
    for item in list1 {
        time += item;
    }
    for item in list2 {
        distance += item;
    }
    let single_race = Race {
        time: time.parse().unwrap(),
        distance: distance.parse().unwrap(),
    };
    Ok((i, single_race))
}

fn day6_task2(input: String) {
    let race = parse_input_task2(&input).unwrap().1;
    println!("Day6 Task2: {}", race.count_winning_races());
}