use std::fs;
use itertools::Itertools;

// Much more elegant solutions from:
//   https://github.com/NickyMeuleman/scrapyard/blob/main/advent_of_code/2022/src/day_03.rs
// Retrieved this to debug my solution down below
pub fn part_1() -> u32 {
    let input = std::fs::read_to_string("input.txt").unwrap();

    input
        .lines()
        .map(|line| line.as_bytes())
        .filter_map(|line| {
            let (left, right) = line.split_at(line.len() / 2);

            right
                .iter()
                .find(|num| left.contains(*num))
                .map(|num| match num {
                    b'a'..=b'z' => (num - b'a') as u32 + 1,
                    _ => (num - b'A') as u32 + 1 + 26
                })
        })
        .sum::<u32>()
}

fn part_2() -> u32 {
    let input = std::fs::read_to_string("input.txt").unwrap();

    input
        .lines()
        .map(|line| line.as_bytes())
        .tuples()
        .filter_map(|(sack1, sack2, sack3)| {
            sack1
                .iter()
                .find(|num| sack2.contains(num) && sack3.contains(num))
                .map(|common| match common {
                    b'a'..=b'z' => (common - b'a') as u32 + 1,
                    _ => (common - b'A') as u32 + 1 + 26,
                })
        })
        .sum::<u32>()
}

fn part_1_mine() {
    let contents = fs::read_to_string("input.txt")
        .expect("Should have been able to read the file");
    let mut sum_prio: u32 = 0;
    for line in contents.lines() {
        let split = line.len() / 2;
        let (first_comp, second_comp) = line.split_at(split);
        for a in first_comp.chars() {
            if second_comp.contains(a) {
                let score = {
                    if a.is_ascii_lowercase() {
                        a as u32 - 'a' as u32 + 1
                    } else {
                        a as u32 - 'A' as u32 + 27
                    }
                };
                sum_prio = sum_prio + score;
                // I initially made this a continue, which screwed everything up
                break
            }
        }
    }
    println!("Sum of priority is: {sum_prio}")
}

pub fn part_2_mine() {
    let contents = fs::read_to_string("input.txt")
        .expect("Should have been able to read the file");
    let mut sum_prio: u32 = 0;
    for (sack_1, sack_2, sack_3) in contents.lines().tuples() {
        for a in sack_1.chars() {
            if sack_2.contains(a) && sack_3.contains(a) {
                let score = {
                    if a.is_ascii_lowercase() {
                        a as u32 - 'a' as u32 + 1
                    } else {
                        a as u32 - 'A' as u32 + 27
                    }
                };
                sum_prio = sum_prio + score;
                // I initially made this a continue, which screwed everything up
                break
            }
        }
    }
    println!("Sum of priority is: {sum_prio}")
}

fn main() {
    let expected = part_1();
    println!("{expected}");
    let expected = part_2();
    println!("{expected}");
    part_1_mine();
    part_2_mine();
}
