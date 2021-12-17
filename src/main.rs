// Fuck it, big file time.
#![allow(unreachable_code, dead_code, clippy::needless_return)]
use itertools::Itertools;
use std::str;
use std::str::FromStr;
use strum_macros::EnumString;

type Coordinate = i64;
type PositiveCounter = u64;

#[derive(Clone, Copy)]
struct Location {
    aim: Option<Coordinate>,
    x: Coordinate,
    y: Coordinate,
}

#[derive(Debug, PartialEq, EnumString)]
enum Direction {
    #[strum(ascii_case_insensitive)]
    Down,
    #[strum(ascii_case_insensitive)]
    Up,
    #[strum(ascii_case_insensitive)]
    Forward,
}

fn main() {
    challenge_03_1();
}

fn get_input(file: &str) -> Vec<&str> {
    let list: Vec<&str> = file.trim().split('\n').collect();
    return list;
}

fn convert_vec_of_str_to_vec_of_int(str_list: Vec<&str>) -> Vec<i64> {
    let mut int_list: Vec<i64> = Vec::new();
    for element in str_list {
        int_list.push(element.trim().parse().expect("not a valid number"))
    }
    return int_list;
}

fn submarine_navigate(mut position: Location, movement: &str) -> Location {
    let (direction, magnitude): (&str, &str) = movement
        .split(' ')
        .collect_tuple()
        .expect("Should have enough entries");
    let magnitude: u32 = magnitude.parse().expect("Should be a valid number");
    let magnitude = magnitude as Coordinate;
    let direction: Direction = Direction::from_str(direction).expect("Valid Direction?");

    if let Some(mut current_aim) = position.aim {
        match direction {
            Direction::Down => current_aim += magnitude,
            Direction::Up => current_aim -= magnitude,
            Direction::Forward => {
                position.x += magnitude;
                position.y += current_aim * magnitude;
            }
        }
        position.aim = Some(current_aim)
    } else {
        match direction {
            Direction::Down => position.y += magnitude,
            Direction::Up => position.y -= magnitude,
            Direction::Forward => position.x += magnitude,
        };
    }
    return position;
}

fn challenge_01_1() {
    let list: Vec<i64> =
        convert_vec_of_str_to_vec_of_int(get_input(include_str!("../inputs/01.txt")));
    let mut increases: usize = 0;
    for index in 0..list.len() - 1 {
        if list[index] < list[index + 1] {
            increases += 1;
        }
    }
    print!("{}", increases)
}

fn challenge_01_2() {
    let list = convert_vec_of_str_to_vec_of_int(get_input(include_str!("../inputs/01.txt")));
    let mut increases: usize = 0;
    for index in 0..list.len() - 1 {
        if index == list.len() - 3 {
            break;
        }
        if list[index].to_owned() + list[index + 1] + list[index + 2]
            < list[index + 1].to_owned() + list[index + 2] + list[index + 3]
        {
            increases += 1
        }
    }
    print!("{}", increases)
}

fn challenge_02_1() {
    let list = get_input(include_str!("../inputs/02.txt"));
    let mut position: Location = Location {
        aim: None,
        x: 0,
        y: 0,
    };
    for command in list {
        position = submarine_navigate(position, command);
    }
    let result = position.x * position.y;
    println!("{}", result);
}

// Sorry Nerath. <3
fn challenge_02_2() {
    let list = get_input(include_str!("../inputs/02.txt"));
    let mut position: Location = Location {
        aim: Some(0),
        x: 0,
        y: 0,
    };
    for command in list {
        position = submarine_navigate(position, command);
    }
    let result = position.x * position.y;
    println!("{}", result)
}

fn challenge_03_1() {
    // Pretend I have the data as an easy to use thing here. We can backfill it later.
    let input = get_input(include_str!("../inputs/03.txt"));
    let mut gamma_rate = vec![0; 12];
    let mut gamma_sum = 0;
    let mut epsilon_rate = vec![0; 12];
    let mut epsilon_sum = 0;
    let mut zero_counts = vec![0; 12];
    for line in input {
        for (index, char) in line.trim().chars().enumerate() {
            match char {
                '0' => zero_counts[index] += 1,
                '1' => (),
                _ => panic!("Invalid char {}", char),
            }
        }
    }
    // println!("{:?}", zero_counts);
    for index in 0..zero_counts.len() {
        if zero_counts[index] < 500 {
            gamma_rate[index] = 1;
        } else {
            epsilon_rate[index] = 1;
        }
    }
    // I wonder if there's a better way to do this...
    for i in 0..12 {
        match gamma_rate[11 - i] {
            0 => epsilon_sum += 2u32.pow(i as u32),
            1 => gamma_sum += 2u32.pow(i as u32),
            _ => panic!("WHAT.\n{}", gamma_rate[11 - i]),
        }
    }
    let product = epsilon_sum * gamma_sum;
    println!(
        "Epsilon Sum: {}\nGamma Sum: {}\nProduct: {}",
        epsilon_sum, gamma_sum, product
    )
}
