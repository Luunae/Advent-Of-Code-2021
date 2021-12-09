// Fuck it, big file time.
#![allow(unreachable_code, dead_code, clippy::needless_return)]
use itertools::Itertools;
use std::str;
use std::str::FromStr;
use strum_macros::EnumString;

type Coordinate = i64;

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
    challenge_02_2();
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
