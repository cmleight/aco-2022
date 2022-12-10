extern crate core;

use std::fs;

#[derive(Clone, Copy)]
enum Move {
    Rock = 1,
    Paper = 2,
    Scissors = 3,
}

#[derive(Clone, Copy)]
enum ResultType {
    Win = 6,
    Draw = 3,
    Loss = 0,
}

fn determine_result(a: Move, b: Move) -> i32 {
    let result_type = match (a, b) {
        (Move::Rock, Move::Scissors) => ResultType::Win,
        (Move::Rock, Move::Rock) => ResultType::Draw,
        (Move::Rock, Move::Paper) => ResultType::Loss,
        (Move::Paper, Move::Rock) =>  ResultType::Win,
        (Move::Paper, Move::Paper) =>  ResultType::Draw,
        (Move::Paper, Move::Scissors) => ResultType::Loss,
        (Move::Scissors, Move::Paper) => ResultType::Win,
        (Move::Scissors, Move::Scissors) => ResultType::Draw,
        (Move::Scissors, Move::Rock) => ResultType::Loss,
    };
    result_type as i32 + a as i32
}

fn get_score_from_prediction(opponent_move: Move, result_type: ResultType) -> i32 {
    let call = match (opponent_move, result_type) {
        (Move::Rock, ResultType::Win) => Move::Paper,
        (Move::Rock, ResultType::Draw) => Move::Rock,
        (Move::Rock, ResultType::Loss) => Move::Scissors,
        (Move::Paper, ResultType::Win) => Move::Scissors,
        (Move::Paper, ResultType::Draw) => Move::Paper,
        (Move::Paper, ResultType::Loss) => Move::Rock,
        (Move::Scissors, ResultType::Win) => Move::Rock,
        (Move::Scissors, ResultType::Draw) => Move::Scissors,
        (Move::Scissors, ResultType::Loss) => Move::Paper,
    };
    call as i32 + result_type as i32
}

fn parse_line(line: &str) -> (Move, Move, ResultType) {
    let (opponent, yours) = line
        .split_once(" ")
        .expect("More than two values found for input row!");
    (
        match opponent {
            "A" => Move::Rock,
            "B" => Move::Paper,
            "C" => Move::Scissors,
            _ => panic!("Unexpected input for opponent!"),
        },
        match yours {
            "X" => Move::Rock,
            "Y" => Move::Paper,
            "Z" => Move::Scissors,
            _ => panic!("Unexpected input for player!"),
        },
        match yours {
            "X" => ResultType::Loss,
            "Y" => ResultType::Draw,
            "Z" => ResultType::Win,
            _ => panic!("Unexpected input for player!"),
        },
    )
}

fn main() {
    let contents = fs::read_to_string("input.txt").expect("Should have been able to read the file");
    let mut part_1_score = 0;
    let mut part_2_score = 0;
    for line in contents.lines() {
        let (opponent, player, outcome) = parse_line(line);
        part_1_score += determine_result(player, opponent);
        part_2_score += get_score_from_prediction(opponent, outcome);
    }
    println!("Resulting score part 1: {part_1_score}");
    println!("Resulting score part 2: {part_2_score}");
}
