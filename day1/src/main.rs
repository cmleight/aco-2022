use std::fs;

use nom::{
    IResult,
    multi::{separated_list1, fold_many1},
    combinator::map_res,
    character::complete::{digit1, newline},
};
use std::str::FromStr;
use nom::sequence::terminated;


fn _main_original() {
    let contents = fs::read_to_string("input.txt")
        .expect("Should have been able to read the file");
    let mut worker = 0;
    let mut array: Vec<i32> = Vec::from([0]);
    for line in contents.lines() {
        match line {
            "" => {
                worker += 1;
                array.insert(worker, 0);
            },
            line =>
                array[worker] = array[worker] + line.parse::<i32>().unwrap(),
        }
    }
    let mut value = 0;
    for i in 0..array.len() {
        if array[i] > value {
            value = array[i];
        }
    }
    array.sort();
    let first = array[worker];
    let second = array[worker - 1];
    let third = array[worker - 2];
    let total = first + second + third;

    println!("{first}");
    println!("{second}");
    println!("{third}");
    println!("total: {total}")
}

/// Parse a `u32` from the start of the input string.
pub fn parse_numbers(input: &str) -> IResult<&str, u32> {
    map_res(terminated(digit1, newline), u32::from_str)(input)
}

pub fn parse_input(s: &str) -> Vec<u32> {
    let (remaining_input, results) = separated_list1(
        newline,   fold_many1(
            parse_numbers,
            || 0,
            |acc: u32, item| acc + item
        )
    )(s)
        .unwrap();
    assert_eq!(remaining_input, "");
    results
}


fn main() {
    let contents = fs::read_to_string("input.txt")
        .expect("Should have been able to read the file");
    let mut array = parse_input(contents.as_str());
    array.sort();
    let end = array.len() - 1;
    let first = array[end];
    let second = array[end - 1];
    let third = array[end - 2];
    let total = first + second + third;

    println!("{first}");
    println!("{second}");
    println!("{third}");
    println!("total: {total}")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_input() {
        let tests = [
            ("1\n2\n", Vec::from([3])),
            ("1\n2\n\n4\n", Vec::from([3, 4])),
        ];
        for (input, expected_output) in tests {
            let output = parse_input(input);
            assert_eq!(output, expected_output);
        }
    }
}
