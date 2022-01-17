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
    return int_list;
}

fn parse_vec_of_str_binary_as_vec_of_base_10_int(input: &[&str]) -> Vec<i64> {
    let mut int_list: Vec<i64> = Vec::new();
    for element in input {
        int_list.push(i64::from_str_radix(element.trim(), 2).unwrap());
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

fn get_zero_counts(int_input: &[i64], number_of_bits: usize) -> Vec<i32> {
    let mut zero_counts = vec![0; number_of_bits];
    // "Trust AoC to come up with something fucky."
    for number in int_input {
        let mut bitwise_comparator = 2_i64.pow(number_of_bits as u32 - 1);
        for item in &mut zero_counts.iter_mut() {
            // Maybe one day this'll make sense.
            if bitwise_comparator & number != bitwise_comparator {
                *item += 1;
            }
            if bitwise_comparator > 1 {
                bitwise_comparator >>= 1
            }
        }
        // dbg!(&zero_counts);
    }
    return zero_counts;
}

fn find_oxygen_generator_rating(input: &[&str]) -> i64 {
    // Fuuuuuck.
    let number_of_bits = input[0].trim().len();
    let mut bitwise_comparator = 2_i64.pow(number_of_bits as u32);
    let mut working_input = parse_vec_of_str_binary_as_vec_of_base_10_int(input);
    for i in 0..number_of_bits {
        // Future me, make a list of all the variables you're going to need to touch before refactoring this into a function.
        // Also make sure to properly design it out on pen and paper, you won't be able to hold it all in memory.
        bitwise_comparator >>= 1;
        let zero_counts = get_zero_counts(&working_input, number_of_bits);
        let are_more_zeros_than_ones =
            zero_counts[i] > (working_input.len() / 2).try_into().unwrap();
        if are_more_zeros_than_ones {
            working_input.retain(|number| number & bitwise_comparator != bitwise_comparator);
        } else {
            working_input.retain(|number| number & bitwise_comparator == bitwise_comparator);
        }
        if working_input.len() == 1 {
            return working_input[0];
        }
    }
    return working_input[0];
}

fn find_co2_scrubber_rating(input: &[&str]) -> i64 {
    let number_of_bits = input[0].trim().len();
    let mut bitwise_comparator = 2_i64.pow(number_of_bits as u32);
    let mut working_input = parse_vec_of_str_binary_as_vec_of_base_10_int(input);
    for i in 0..number_of_bits {
        bitwise_comparator >>= 1;
        let zero_counts = get_zero_counts(&working_input, number_of_bits);
        let are_more_zeros_than_ones =
            zero_counts[i] > (working_input.len() / 2).try_into().unwrap();
        if are_more_zeros_than_ones {
            working_input.retain(|number| number & bitwise_comparator == bitwise_comparator);
        } else {
            working_input.retain(|number| number & bitwise_comparator != bitwise_comparator);
        }
        if working_input.len() == 1 {
            return working_input[0];
        }
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
    let int_inputs = parse_vec_of_str_binary_as_vec_of_base_10_int(&input);
    let number_of_bits = input[0].trim().len();
    let zero_counts = get_zero_counts(&int_inputs, number_of_bits);
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
    let input = get_input(include_str!("../inputs/03.txt"));
    let o2_rating = find_oxygen_generator_rating(&input);
    let co2_scrubber_rating = find_co2_scrubber_rating(&input);
    println!("o2: {:?}\nco2: {:?}", o2_rating, co2_scrubber_rating);
    let life_support_rating = o2_rating * co2_scrubber_rating;
    println!("LSR: {:?}", life_support_rating)
}

fn challenge_04_1() {
    // Get input.
    let input = get_input(include_str!("../inputs/04.txt"));
    // Split out draws.
    let draws: &str = input[0];
    // Build Prime Boards. *
    // Build Shadow Boards. (Bitwise 25-bit reference?) *

    // Define bitwise bingo positions.
    // For each Drawn number:
    // // For each Prime Board:
    // // // If num on board, mark its Shadow. * If Shadow matches any bingo pattern, call bingo.
    // For each position on Shadow Board:
    // // If False/0/bitwise-comparator-not-equal:
    // // // Add value of position on Prime Board to Sum.
    // Multiply Sum by 24.
}
