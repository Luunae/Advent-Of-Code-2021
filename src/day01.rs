use std::env;
use std::fs;
use std::str::FromStr;

const CONTENTS: &str = include_str!("../inputs/input1-1.txt");

pub fn challenge_01_1() {
    let list: Vec<i32> = CONTENTS.trim().split("\n").map(|it| i32::from_str(it.trim()).expect("should be valid int")).collect(); //I *despise* this.
    let mut increases = 0;
    for index in 1..list.len() {
        if list[index-1] < list[index] {
            increases += 1;
        }
    }
    print!("{}", increases)
}
