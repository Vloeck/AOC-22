use std::collections::HashSet;
use crate::lines;

fn find_marker(input: &String, len: usize) -> Option<usize> {
    let mut i = 0;
    while i < input.len() - len {
        let chars = &input[i..=i + len - 1];
        let uniq_chars = chars.chars().collect::<HashSet<char>>();
        if uniq_chars.len() == len {
            return Some(i + len);
        }
        i += 1;
    }
    None
}

fn find_packet_marker(input: &String) -> Option<usize> {
    find_marker(input, 4)
}

fn find_message_marker(input: &String) -> Option<usize> {
    find_marker(input, 14)
}

fn calculate1(input: &Vec<String>) -> Vec<usize> {
    input.iter()
        .filter_map(find_packet_marker)
        .collect::<Vec<usize>>()
}

fn calculate2(input: &Vec<String>) -> Vec<usize> {
    input.iter()
        .filter_map(find_message_marker)
        .collect::<Vec<usize>>()
}

pub(crate) fn main() {
    let lines = lines::read_lines("resources/day6.txt");
    if let Ok(lines) = lines {
        let results = calculate1(&lines);
        results.iter().for_each(|s| println!("Day 6: Start position of Packet is {s}"));
        let results = calculate2(&lines);
        results.iter().for_each(|s| println!("Day 6: Start position of Message is {s}"));
    }
    println!()
}

#[cfg(test)]
mod tests {
    use super::{calculate1, calculate2};

    fn test_data() -> Vec<String> {
        vec![
            "mjqjpqmgbljsphdztnvjfqwrcgsmlb",
            "bvwbjplbgvbhsrlpgdmjqwftvncz",
            "nppdvjthqldpwncqszvftbrmjlhg",
            "nznrnfrfntjfmvfwmzdfjlvtqnbhcprsg",
            "zcfzfwzzqfrljwzlrfnpqdbhtmscgvjw",
        ].iter().map(<&str>::to_string).collect()
    }

    #[test]
    fn test1() {
        let input = test_data();
        let results = calculate1(&input);
        let expected: Vec<usize> = vec![7, 5, 6, 10, 11];
        assert_eq!(results, expected);
    }

    #[test]
    fn test2() {
        let input = test_data();
        let results = calculate2(&input);
        let expected: Vec<usize> = vec![19, 23, 23, 29, 26];
        assert_eq!(results, expected);
    }
}