use std::cmp::Ordering;
use crate::day2::Hand::{Paper, Rock, Scissors};
use crate::day2::Outcome::{Draw, Lose, Win};
use crate::lines;

#[derive(PartialEq, Copy, Clone, Debug)]
enum Hand {
    Rock,
    Paper,
    Scissors,
}

impl From<&str> for Hand {
    fn from(input: &str) -> Self {
        match input {
            "A" | "X" => Rock,
            "B" | "Y" => Paper,
            "C" | "Z" => Scissors,
            _ => panic!("Could not match"),
        }
    }
}

impl PartialOrd for Hand {
    fn partial_cmp(&self, _other: &Self) -> Option<Ordering> {
        todo!()
    }

    fn lt(&self, other: &Self) -> bool {
        match self {
            Rock => *other == Paper,
            Paper => *other == Scissors,
            Scissors => *other == Rock,
        }
    }

    fn le(&self, other: &Self) -> bool {
        self.lt(other) || self == other
    }

    fn gt(&self, other: &Self) -> bool {
        match self {
            Rock => *other == Scissors,
            Paper => *other == Rock,
            Scissors => *other == Paper,
        }
    }

    fn ge(&self, other: &Self) -> bool {
        self.gt(other) || self == other
    }
}

impl Hand {
    fn against(self, other: Hand) -> Outcome {
        if self > other {
            Win
        } else if self == other {
            Draw
        } else {
            Lose
        }
    }

    fn value(&self) -> u32 {
        match self {
            Rock => 1,
            Paper => 2,
            Scissors => 3,
        }
    }
}

#[derive(PartialEq, Copy, Clone, Debug)]
enum Outcome {
    Win,
    Lose,
    Draw,
}

impl Outcome {
    fn value(&self) -> u32 {
        match self {
            Win => 6,
            Lose => 0,
            Draw => 3,
        }
    }
    fn against_hand(self, other: Hand) -> Hand {
        vec![Rock, Paper, Scissors].into_iter()
            .find(|hand| hand.against(other) == self)
            .unwrap()
    }
}

impl From<&str> for Outcome {
    fn from(input: &str) -> Self {
        match input {
            "X" => Lose,
            "Y" => Draw,
            "Z" => Win,
            _ => panic!("Could not match"),
        }
    }
}

fn calculate1(lines: Vec<String>) -> u32 {
    let mut score = 0u32;
    for line in lines {
        let parts = line.split(" ").collect::<Vec<&str>>();
        if parts.len() != 2 {
            panic!("Cannot parse line");
        }
        let other: Hand = parts[0].into();
        let hand: Hand = parts[1].into();
        let outcome = hand.against(other);
        score += hand.value() + outcome.value();
    }
    score
}

fn calculate2(lines: Vec<String>) -> u32 {
    let mut score = 0u32;
    for line in lines {
        let parts = line.split(" ").collect::<Vec<&str>>();
        if parts.len() != 2 {
            panic!("Cannot parse line");
        }
        let other: Hand = parts[0].into();
        let outcome: Outcome = parts[1].into();
        let hand = outcome.against_hand(other);
        score += hand.value() + outcome.value();
    }
    score
}

pub(crate) fn main() {
    let lines = lines::read_lines("resources/day2.txt");
    if let Ok(lines) = lines {
        let score = calculate1(lines.clone());
        println!("Day 2: First Score: {score}");
        let score = calculate2(lines);
        println!("Day 2: Second Score: {score}");
    }
    println!()
}

#[cfg(test)]
mod tests {
    use super::{calculate1, calculate2};

    fn test_data() -> Vec<String> {
        vec![
            "A Y",
            "B X",
            "C Z",
        ].iter().map(<&str>::to_string).collect()
    }

    #[test]
    pub fn test1() {
        let lines = test_data();
        let score = calculate1(lines);
        assert_eq!(score, 15);
    }

    #[test]
    pub fn test2() {
        let lines = test_data();
        let score = calculate2(lines);
        assert_eq!(score, 12);
    }
}
