fn day3_task1() {
    let lines = include_str!("day3.txt").lines();

    let array: Vec<Vec<char>> = lines
        .map(|line| line.chars().map(|char| char).collect())
        .collect();
    //println!("Array: {:?}", array);

    let mut sum = 0;

    for i in 0..array.len() {
        let line = array.get(i).unwrap();
        let mut j = 0;
        while j < line.len() {
            if line[j].is_digit(10) {
                // println!("is digit: {}", line[j]);
                let (number, start, end) = construct_number(line, j);
                // println!("Numbers: {}, Start: {}, End: {}", number, start, end);

                if is_valid_part(start, end, i, &array) {
                    //println!("Valid parts: {}", number);
                    sum += number;
                }
                j = end;
            }
            j += 1;
        }
    }
    println!("Day3 Task1: {}", sum);
}

fn construct_number(line: &Vec<char>, index: usize) -> (i32, usize, usize) {
    let mut number = String::new();
    number.push(line[index]);
    let line_length = line.len();

    let mut end = index + 1;
    while end < line_length {
        let next = line[end];
        if !next.is_digit(10) {
            break;
        }
        number.push(next);
        end += 1;
    }
    (number.parse().unwrap(), index, end - 1usize)
}

fn is_valid_part(start: usize, end: usize, y: usize, array: &Vec<Vec<char>>) -> bool {
    let width = array[y].len();
    let height = array.len();

    let from_y = if y as i32 - 1 < 0 { y } else { y - 1 };
    let to_y = if y + 1 >= height { y } else { y + 1 };
    let from_x = if start as i32 - 1 < 0 {
        start
    } else {
        start - 1
    };
    let to_x = if end + 1 >= width { end } else { end + 1 };

    for i in from_y..=to_y {
        for j in from_x..=to_x {
            if !array[i][j].is_digit(10) && array[i][j] != '.' {
                return true;
            }
        }
    }
    false
}

fn day3_task2() {
    let lines = include_str!("day3.txt").lines();

    let mut numbers: Vec<Number> = vec![];
    let mut gears: Vec<Gear> = vec![];

    let array: Vec<Vec<char>> = lines
        .map(|line| line.chars().map(|char| char).collect())
        .collect();

    for i in 0..array.len() {
        let line = &array[i];
        let mut j = 0;
        while j < array[i].len() {
            if line[j].is_digit(10) {
                let (number, x_start, x_end) = construct_number(line, j);
                numbers.push(Number {
                    value: number,
                    x_start,
                    x_end,
                    y: i,
                });
                j = x_end;
            }
            if line[j] == '*' {
                gears.push(Gear { x: j, y: i })
            }
            j += 1;
        }
    }
    let mut sum = 0;
    for gear in gears {
        let mut neighbours: Vec<Number> = vec![];
        for num in &numbers {
            if gear.is_neighbor(num) {
                neighbours.push(*num);
            }
        }
        if neighbours.len() == 2 {
            sum += neighbours[0].value * neighbours[1].value;
        }
    }
    println!("Day3 Task2: {}", sum);
}

#[derive(Clone, Copy)]
struct Number {
    value: i32,
    x_start: usize,
    x_end: usize,
    y: usize,
}

#[derive(Clone, Copy)]
struct Gear {
    x: usize,
    y: usize,
}

impl Gear {
    fn is_neighbor(self, number: &Number) -> bool {
        let x_range = number.x_start as i32 - 1..=number.x_end as i32 + 1;
        let y_range = number.y as i32 - 1..=number.y as i32 + 1;

        if x_range.contains(&(self.x as i32)) && y_range.contains(&(self.y as i32)) {
            return true;
        }
        false
    }
}

fn main() {
    day3_task1();
    day3_task2();
}
