// Fuck it, big file time.
#![allow(unreachable_code, dead_code, clippy::needless_return)]

fn main() {
    challenge_01_2();
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
