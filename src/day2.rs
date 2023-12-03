use nom::bytes::complete::tag;
use nom::character::complete::{alpha1, digit1};
use nom::IResult;

pub struct Dice {
    color: String,
    count: i32,
}

impl Dice {
    fn is_valid_count(&self) -> bool {
        match &self.color[..] {
            "red" => self.count <= 12,
            "green" => self.count <= 13,
            "blue" => self.count <= 14,
            _ => false
        }
    }
}

pub fn day2_task1() {
    let lines = include_str!("day2.txt").lines();
    let mut sum = 0;
    for line in lines {
        let mut split = line.split(":");
        let game = split.next().unwrap();
        let rest = split.next().unwrap().replace(" ", "");

        let (_, game_number) = extract_game_number(game).unwrap();
        let mut valid_game = true;

        let rounds = rest.split(";");
        'outer: for round in rounds {
            for throw in round.split(",") {
                let (_, dice): (&str, Dice) = extract_dice(throw).unwrap();
                if !dice.is_valid_count() {
                    valid_game = false;
                    break 'outer;
                }
            }
        }
        if valid_game {
            sum += game_number;
        }
    }
    println!("Day2 Task1: {}", sum)
}

fn extract_game_number(input: &str) -> IResult<&str, i32> {
    let (input, _) = tag("Game ")(input)?;
    let (input, num) = digit1(input)?;
    Ok((input, num.parse::<i32>().unwrap()))
}

fn extract_dice(input: &str) -> IResult<&str, Dice> {
    let (input, count) = digit1(input)?;
    let (input, color) = alpha1(input)?;

    Ok((input, Dice{
        color: color.to_string(),
        count: count.parse::<i32>().unwrap()
    }))
}

pub fn day2_task2() {
    let lines = include_str!("day2.txt").lines();
    let mut sum = 0;
    for line in lines {
        let mut split = line.split(":");
        let _ = split.next().unwrap();
        let rest = split.next().unwrap().replace(" ", "");


        let rounds = rest.split(";");
        let mut dieces: Vec<Dice> = vec![];
        for round in rounds {
            for throw in round.split(",") {
                let (_, dice): (&str, Dice) = extract_dice(throw).unwrap();
                dieces.push(dice);
            }
        }
        let min_red = dieces.iter().filter(|d| d.color == "red").map(|d| d.count).max().unwrap();
        let min_blue = dieces.iter().filter(|d| d.color == "blue").map(|d| d.count).max().unwrap();
        let min_green = dieces.iter().filter(|d| d.color == "green").map(|d| d.count).max().unwrap();
        sum += min_red * min_blue * min_green;
    }
    println!("Day2 Task2: {}", sum)
}