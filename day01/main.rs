fn day1_task1() {
    let input = include_str!("day1.txt");
    let lines = input.lines();
    let mut sum: i32 = 0;
    for line in lines {
        let mut number: String = String::from("");
        for char in line.chars() {
            if char.is_digit(10) {
                number.push(char);
                break;
            }
        }
        for char in line.chars().rev() {
            if char.is_digit(10) {
                number.push(char);
                break;
            }
        }
        sum = sum + number.parse::<i32>().unwrap()
    }
    println!("Day1 Task1: {}", sum)
}

fn day1_task2() {
    let digits = [
        "one", "two", "three", "four", "five", "six", "seven", "eight", "nine", "1", "2", "3", "4",
        "5", "6", "7", "8", "9",
    ];
    let lines = include_str!("day1.txt").lines();
    let mut sum = 0;
    for line in lines {
        let mut number: String = String::from("");

        let mut lowest_element = "";
        let mut lowest_index = line.len();
        let mut highest_element = "";
        let mut highest_index = 0;

        for digit in digits {
            if let Some(index) = line.find(digit) {
                if index <= lowest_index {
                    lowest_index = index;
                    lowest_element = convert_string_to_digit(digit);
                }
            }
            if let Some(index) = line.rfind(digit) {
                if index >= highest_index {
                    highest_index = index;
                    highest_element = convert_string_to_digit(digit);
                }
            }
        }
        number.push(lowest_element.chars().next().unwrap());
        number.push(highest_element.chars().next().unwrap());
        sum = sum + number.parse::<i32>().unwrap();
    }
    println!("Day1 Task2: {}", sum);
}

fn convert_string_to_digit(string: &str) -> &str {
    match string {
        "one" => "1",
        "two" => "2",
        "three" => "3",
        "four" => "4",
        "five" => "5",
        "six" => "6",
        "seven" => "7",
        "eight" => "8",
        "nine" => "9",
        default => default,
    }
}

fn main() {
    day1_task1();
    day1_task2();
}
