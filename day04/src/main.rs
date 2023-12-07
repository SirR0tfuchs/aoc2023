use std::{fs, path};

fn main() {
    day4_task1();
    day4_task2();
}

fn day4_task1() {
    let cards_file_path = path::Path::new("src/day4.txt");
    let _cards_file_path_str = cards_file_path.display();

    let mut sum = 0;

    let contents = fs::read_to_string(cards_file_path).expect("Could not read file.");

    let lines: Vec<&str> = contents.trim().split("\n").map(|line| line.split(":").nth(1).expect("Couldnt split correctly.")).collect();
    for (_index, line) in lines.iter().enumerate() {
        let mut cards = line.split("|");
        let winning: Vec<i32> = cards.nth(0).expect("No 0th").split(" ").filter(|x| !x.trim().is_empty()).map(|numbers| numbers.parse::<i32>().expect("Couldnt make number 1")).collect();
        let draws: Vec<i32> = cards.nth(0).expect("No 1th").split(" ").filter(|x| !x.trim().is_empty()).map(|numbers| numbers.parse::<i32>().expect("Couldnt make number")).collect();

        let num_winning_cards = winning.iter().filter(|x| draws.contains(x)).count();
        // println!("Line {}, has {} winning cards.", index, num_winning_cards);

        if num_winning_cards > 0 {
            sum += i32::pow(2, num_winning_cards as u32 - 1);
        }
    }
    println!("Day4 Task1: {}", sum);
}

fn day4_task2() {
    let cards_file_path = path::Path::new("src/day4.txt");
    let mut sum = 0;
    let contents = fs::read_to_string(cards_file_path).expect("Haha");
    let lines: Vec<&str> = contents.trim().split("\n")
        .map(|line| line.split(":").nth(1).expect("Could not split correctly."))
        .collect();
    let mut times = vec![1; lines.len() as usize];
    for (index, line) in lines.iter().enumerate() {
        // println!("Line: {}", line);
        let mut cards = line.split("|");
        let winning: Vec<i32> = cards.nth(0).expect("No 0th").split(" ").filter(|x| !x.trim().is_empty()).map(|numbers| numbers.parse::<i32>().expect("Couldnt make number 1")).collect();
        let draws: Vec<i32> = cards.nth(0).expect("No 1th").split(" ").filter(|x| !x.trim().is_empty()).map(|numbers| numbers.parse::<i32>().expect("Couldnt make number")).collect();

        let num_winning_cards = winning.iter().filter(|x| draws.contains(x)).count();

        let mut upper_bound = index + 1 + num_winning_cards;
        if upper_bound >= lines.len() {
            upper_bound = lines.len();
        }
        for i in index+1..upper_bound {
            times[i] += 1 * times[index];
        }
        println!("Times: {}, Index: {}", times[index], index)
    }
    sum = times.into_iter().sum();
    println!("Day4 Task2: {}", sum);
}