use std::env;
use std::fs;
use std::str::FromStr;

const CONTENTS: &str = include_str!("../inputs/input1-1.txt");

fn get_input() -> Vec<i32> {
    let list: Vec<i32> = CONTENTS.trim().split("\n").map(|it| i32::from_str(it.trim()).expect("should be valid int")).collect(); //I *despise* this.
    return list // *Fuck* implicit returns.
}

pub fn challenge_01_1() {
    let list = get_input();
    let mut increases = 0;
    for index in 0..list.len()-1 {
        if list[index] < list[index+1] {
            increases += 1;
        }
    }
    print!("{}", increases)
}

pub fn challenge_01_2() {
    let list = get_input();
    let mut increases = 0;
    for index in 0..list.len()-1 {
        if index == list.len() - 3 {
            break
        }
        if list[index] + list[index+1] + list[index+2] < list[index+1] + list[index+2] + list[index+3] {
            increases += 1
        }
    }
    print!("{}", increases)
}