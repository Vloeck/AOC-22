use std::ops::RangeInclusive;

use crate::lines;

fn fully_contains<T>(r1: &RangeInclusive<T>, r2: &RangeInclusive<T>) -> bool
    where T: PartialOrd {
    r1.contains(r2.start()) && r1.contains(r2.end()) || r2.contains(r1.start()) && r2.contains(r1.end())
}

fn partly_contains<T>(r1: &RangeInclusive<T>, r2: &RangeInclusive<T>) -> bool
    where T: PartialOrd {
    r1.contains(r2.start()) || r1.contains(r2.end()) || r2.contains(r1.start()) || r2.contains(r1.end())
}

fn get_sections(lines: &Vec<String>) -> Vec<(RangeInclusive<u32>, RangeInclusive<u32>)> {
    lines.iter()
        .map(|s| {
            let mut split = s.split(",")
                .map(|s| {
                    let split = s.split("-")
                        .filter_map(|s| s.parse().ok())
                        .collect::<Vec<u32>>();
                    assert_eq!(split.len(), 2);
                    split[0]..=split[1]
                })
                .collect::<Vec<RangeInclusive<u32>>>();
            assert_eq!(split.len(), 2);
            (split.remove(0), split.remove(0))
        })
        .collect::<Vec<(RangeInclusive<u32>, RangeInclusive<u32>)>>()
}

fn calculate1(sections: &Vec<(RangeInclusive<u32>, RangeInclusive<u32>)>) -> usize {
    sections.into_iter()
        .filter(|s| fully_contains(&s.0, &s.1))
        .count()
}

fn calculate2(sections: &Vec<(RangeInclusive<u32>, RangeInclusive<u32>)>) -> usize {
    sections.into_iter()
        .filter(|s| partly_contains(&s.0, &s.1))
        .count()
}

pub(crate) fn main() {
    let lines = lines::read_lines("resources/day4.txt");
    if let Ok(lines) = lines {
        let sections = get_sections(&lines);
        let contained = calculate1(&sections);
        println!("Day 4: Count of fully contained sections = {contained}");
        let contained = calculate2(&sections);
        println!("Day 4: Count of partly contained sections = {contained}");
    }
    println!()
}

#[cfg(test)]
mod tests {
    use super::{calculate1, calculate2, get_sections};

    fn test_data() -> Vec<String> {
        vec![
            "2-4,6-8",
            "2-3,4-5",
            "5-7,7-9",
            "2-8,3-7",
            "6-6,4-6",
            "2-6,4-8",
        ].iter().map(<&str>::to_string).collect()
    }

    #[test]
    fn test1() {
        let input = test_data();
        let sections = get_sections(&input);
        let contained = calculate1(&sections);
        assert_eq!(contained, 2);
    }

    #[test]
    fn test2() {
        let input = test_data();
        let sections = get_sections(&input);
        let priority_sum = calculate2(&sections);
        assert_eq!(priority_sum, 4);
    }
}