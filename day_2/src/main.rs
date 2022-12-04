use std::{
    fs::File,
    io::{self, BufRead},
};

fn main() {
    let file = File::open("input.txt").unwrap();
    let mut total_score = 0;
    for line in io::BufReader::new(file).lines().map(|x| x.unwrap()) {
        let (left, right) = line.split_once(" ").unwrap();
        let (opponent, outcome) = (Action::parse(left), Outcome::parse(right));
        let action = play(&opponent, &outcome);
        total_score += action.score() + outcome.score();
    }
    println!("Total score: {}", total_score);
}

enum Action {
    Rock,
    Paper,
    Scissors,
}

impl Action {
    // 1 for Rock, 2 for Paper, and 3 for Scissors
    fn score(&self) -> i32 {
        match self {
            Action::Rock => 1,
            Action::Paper => 2,
            Action::Scissors => 3,
        }
    }

    // A for Rock, B for Paper, and C for Scissors.
    fn parse(letter: &str) -> Self {
        match letter {
            "A" => Action::Rock,
            "B" => Action::Paper,
            "C" => Action::Scissors,
            _ => panic!("{} is not a valid Action", letter),
        }
    }
}

enum Outcome {
    Win,
    Draw,
    Lose,
}

impl Outcome {
    // 0 if you lost, 3 if the round was a draw, and 6 if you won
    fn score(&self) -> i32 {
        match self {
            Outcome::Win => 6,
            Outcome::Draw => 3,
            Outcome::Lose => 0,
        }
    }

    // X means you need to lose, Y means you need to end the round in a draw, and Z means you need to win. Good luck!"
    fn parse(letter: &str) -> Self {
        match letter {
            "X" => Outcome::Lose,
            "Y" => Outcome::Draw,
            "Z" => Outcome::Win,
            _ => panic!("{} is not a valid Outcome", letter),
        }
    }
}

fn play(opponent: &Action, outcome: &Outcome) -> Action {
    match (opponent, outcome) {
        (Action::Rock, Outcome::Win) => Action::Paper,
        (Action::Rock, Outcome::Draw) => Action::Rock,
        (Action::Rock, Outcome::Lose) => Action::Scissors,
        (Action::Paper, Outcome::Win) => Action::Scissors,
        (Action::Paper, Outcome::Draw) => Action::Paper,
        (Action::Paper, Outcome::Lose) => Action::Rock,
        (Action::Scissors, Outcome::Win) => Action::Rock,
        (Action::Scissors, Outcome::Draw) => Action::Scissors,
        (Action::Scissors, Outcome::Lose) => Action::Paper,
    }
}
