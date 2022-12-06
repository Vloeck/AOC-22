use std::collections::HashSet;
use crate::lines;

fn priority(c: char) -> u32 {
    if c.is_uppercase() {
        c as u32 - 'A' as u32 + 27
    } else if c.is_lowercase() {
        c as u32 - 'a' as u32 + 1
    } else {
        panic!("invalid char")
    }
}

fn find_doubles_in_elves(group: &[String]) -> HashSet<char> {
    let s1 = &group[0];
    let s2 = &group[1];
    let s3 = &group[2];
    let c1 = find_doubles(s1, s2);
    let c2 = find_doubles(s2, s3);
    let c3 = find_doubles(s3, s1);
    let i1: HashSet<char> = c1.intersection(&c2).map(|c| *c).collect();
    let i2: HashSet<char> = i1.intersection(&c3).map(|c| *c).collect();
    assert_eq!(i2.len(), 1);
    i2
}

fn find_doubles_in_backpack(s: &String) -> HashSet<char> {
    let len = s.len();
    let s1 = &s[..len / 2];
    let s2 = &s[len / 2..];
    find_doubles(s1, s2)
}

fn find_doubles(s1: &str, s2: &str) -> HashSet<char> {
    s1.chars()
        .filter(|c| s2.chars().into_iter()
            .any(|c2| c2 == *c)
        )
        .collect()
}

fn calculate1(lines: &Vec<String>) -> u32 {
    lines.iter()
        .flat_map(find_doubles_in_backpack)
        .map(priority)
        .sum()
}

fn calculate2(lines: &Vec<String>) -> u32 {
    lines.chunks(3)
        .flat_map(find_doubles_in_elves)
        .map(priority)
        .sum()
}

pub(crate) fn main() {
    let lines = lines::read_lines("resources/day3.txt");
    if let Ok(lines) = lines {
        let priority_sum = calculate1(&lines);
        println!("Day 3: Priority Sum = {priority_sum}");
        let priority_sum = calculate2(&lines);
        println!("Day 3: Priority Sum = {priority_sum}");
    }
    println!()
}

#[cfg(test)]
mod tests {
    use super::{calculate1, calculate2};

    fn test_data () -> Vec<String> {
        vec![
            "vJrwpWtwJgWrhcsFMMfFFhFp",
            "jqHRNqRjqzjGDLGLrsFMfFZSrLrFZsSL",
            "PmmdzqPrVvPwwTWBwg",
            "wMqvLMZHhHMvwLHjbvcjnnSBnvTQFn",
            "ttgJtRGJQctTZtZT",
            "CrZsJsPPZsGzwwsLwLmpwMDw",
        ].iter().map(<&str>::to_string).collect()
    }

    #[test]
    fn test1() {
        let input = test_data();
        let priority_sum = calculate1(&input);
        assert_eq!(priority_sum, 157);
    }

    #[test]
    fn test2() {
        let input = test_data();
        let priority_sum = calculate2(&input);
        assert_eq!(priority_sum, 70);
    }
}