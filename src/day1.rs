use crate::lines;

fn get_elves(lines: &Vec<String>) -> Vec<u32> {
    let split: Vec<&[String]> = lines.split(|e| e.is_empty()).collect();
    let mut elves = split.into_iter()
        .map(|list| list.into_iter()
            .filter_map(|e| e.parse::<u32>().ok())
            .collect::<Vec<u32>>()
        )
        .map(|c| c.into_iter().sum())
        .collect::<Vec<u32>>();
    elves.sort();
    elves.reverse();
    elves
}

fn top_n_elves(elves: &Vec<u32>, n: usize) -> u32 {
    elves[0..n].iter()
        .fold(0u32, |accu, item| accu + *item)
}

fn calculate1(elves: &Vec<u32>) -> u32 {
    top_n_elves(elves, 1)
}

fn calculate2(elves: &Vec<u32>) -> u32 {
    top_n_elves(elves, 3)
}

pub(crate) fn main() {
    let lines = lines::read_lines("resources/day1.txt");
    if let Ok(lines) = lines {
        let elves = get_elves(&lines);
        println!("Day 1: The Elf with the most calories carries {:?} calories", calculate1(&elves));
        println!("Day 1: The 3 Elves with the most calories carry {:?} calories together", calculate2(&elves));
    }
    println!()
}

#[cfg(test)]
mod tests {
    use super::{calculate1, calculate2, get_elves};

    fn test_data() -> Vec<String> {
        vec![
            "1000",
            "2000",
            "3000",
            "",
            "4000",
            "",
            "5000",
            "6000",
            "",
            "7000",
            "8000",
            "9000",
            "",
            "10000",
        ].iter().map(<&str>::to_string).collect()
    }
    #[test]
    fn test1() {
        let lines = test_data();
        let elves = get_elves(&lines);
        let score = calculate1(&elves);
        assert_eq!(score, 24_000);
    }

    #[test]
    fn test2() {
        let lines = test_data();
        let elves = get_elves(&lines);
        let score = calculate2(&elves);
        assert_eq!(score, 45_000);
    }
}
