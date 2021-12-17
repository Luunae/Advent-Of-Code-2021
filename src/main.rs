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
    challenge_03_2();
}

fn get_input(file: &str) -> Vec<&str> {
    let list: Vec<&str> = file.trim().split('\n').collect();
    return list;
}

fn convert_vec_of_str_to_vec_of_int(str_list: Vec<&str>) -> Vec<i64> {
    let mut int_list: Vec<i64> = Vec::new();
    for element in str_list {
        int_list.push(element.trim().parse().expect("not a valid number"));
    }
    println!("{:?}", int_list);
    return int_list;
}

fn parse_vec_of_str_binary_as_vec_of_base_10_int(input: Vec<&str>) -> Vec<i64> {
    let mut int_list: Vec<i64> = Vec::new();
    for element in input {
        int_list.push(i64::from_str_radix(element, 2).unwrap());
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

fn get_zero_counts(input: Vec<&str>) -> Vec<i32> {
    let mut zero_counts = vec![0; 12];
    let int_input = parse_vec_of_str_binary_as_vec_of_base_10_int(input);
    // "Trust AoC to come up with something fucky."
    for number in &int_input {
        let mut bitwise_comparator = 2048;
        for i in 0..zero_counts.len() {
            // The specific suggestion from Clippy(needless_range_loop) does some weird wrong shit. Intentionally ignoring.
            // This is probably as a result of me doing things in a Pythonic way rather than a Rusty way.
            if bitwise_comparator & number == 0 {
                zero_counts[i] += 1
            }
            if bitwise_comparator > 1 {
                bitwise_comparator >>= 1
            }
        }
    }
    return zero_counts;
}

fn find_oxygen_generator_rating(input: Vec<&str>) -> i64 {
    let mut rating: i64 = 0;
    let mut bitwise_comparator = 2048;
    let mut working_input = parse_vec_of_str_binary_as_vec_of_base_10_int(input);
    for i in 0..12 {
        let zero_counts = get_zero_counts(working_input); // ????? TFW can't into borrowing.
        if zero_counts[i] > (working_input.len() / 2).try_into().unwrap() {
            for number in working_input {
                if number & bitwise_comparator == 1 {
                    working_input.pop(number); // ?????
                }
            }
        }
        if working_input.len() == 1 {
            return working_input[0]; // I'll move *your* value.
        }
        bitwise_comparator >>= 1;
    }
    return working_input[0];
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
    let input = get_input(include_str!("../inputs/03.txt"));
    let mut gamma_rate = vec![0; 12];
    let mut gamma_sum = 0;
    let mut epsilon_rate = vec![0; 12];
    let mut epsilon_sum = 0;
    let zero_counts = get_zero_counts(input);
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

fn challenge_03_2() {
    unimplemented!(); // Screaming
    let input = get_input(include_str!("../inputs/03.txt"));
    let o2_rating = find_oxygen_generator_rating(input);
    println!("{:?}", o2_rating)
}
