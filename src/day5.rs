use std::collections::{BTreeMap, VecDeque};
use crate::lines;

#[derive(Debug)]
struct MoveInstruction {
    count: u32,
    from_stack: String,
    to_stack: String,
}

#[derive(Debug, Clone)]
struct Stack {
    names: Vec<String>,
    stack: BTreeMap<String, VecDeque<String>>,
}

impl Stack {
    fn move_crates(&mut self, instructions: &Vec<MoveInstruction>) {
        let stack = &mut self.stack;
        for instruction in instructions {
            let mut i = 0;
            while i < instruction.count {
                let from = stack.get_mut(&instruction.from_stack).unwrap();
                let crate_name = from.pop_front().expect("No more Crates on Stack");
                // println!("Moving {crate_name} from {} to {} ({}/{})", instruction.from_stack, instruction.to_stack, i + 1, instruction.count);
                let to = stack.get_mut(&instruction.to_stack).unwrap();
                to.push_front(crate_name);
                // println!("{:?}", stack);
                i += 1;
            }
        }
    }

    fn move_stacked_crates(&mut self, instructions: &Vec<MoveInstruction>) {
        let stack = &mut self.stack;
        for instruction in instructions {
            let mut i = 0;
            let mut crates= vec![];
            while i < instruction.count {
                let from = stack.get_mut(&instruction.from_stack).unwrap();
                let crate_name = from.pop_front().expect("No more Crates on Stack");
                crates.insert(0, crate_name);
                // println!("Moving {crate_name} from {} to {} ({}/{})", instruction.from_stack, instruction.to_stack, i + 1, instruction.count);
                // println!("{:?}", stack);
                i += 1;
            }
            let to = stack.get_mut(&instruction.to_stack).unwrap();
            for crate_name in crates {
                to.push_front(crate_name);
            }
        }
    }

    fn get_top(&self) -> String {
        self.names.iter()
            .fold(String::new(), |accu, item| accu + self.stack.get(item).unwrap().front().unwrap())
    }
}

impl From<&String> for MoveInstruction {
    fn from(s: &String) -> Self {
        let parts = s.split(" ")
            .collect::<Vec<&str>>();
        assert_eq!(parts.len(), 6);
        Self {
            count: parts[1].parse().unwrap(),
            from_stack: parts[3].to_string(),
            to_stack: parts[5].to_string(),
        }
    }
}

fn prepare_lines(lines: &Vec<String>) -> (&[String], &[String]) {
    let mut split = lines.split(|s| s.is_empty()).collect::<Vec<&[String]>>();
    assert_eq!(split.len(), 2);
    (split.remove(0), split.remove(0))
}

fn prepare_stack(stack_lines: &[String]) -> Stack {
    let mut stack_lines = stack_lines.to_vec();
    stack_lines.reverse();
    let name_str = stack_lines.remove(0);
    let mut i = 0;
    let mut names = vec![];
    while i < name_str.len() {
        names.push(name_str[i + 1..i + 2].to_string());
        i += 4;
    }
    let mut stack = BTreeMap::new();
    for name in &names {
        stack.insert(name.clone(), VecDeque::new());
    }
    for line in stack_lines {
        let mut i = 0;
        let mut n = 0;
        while i < line.len() {
            let item = String::from(&line[i + 1..i + 2]);
            if item != " " {
                let name = names.get(n).unwrap();
                stack.get_mut(name).unwrap().push_front(item.clone());
            }
            i += 4;
            n += 1;
        }
    }
    // println!("{:?}", stack);
    Stack {
        names,
        stack,
    }
}

fn prepare_instructions(lines: &[String]) -> Vec<MoveInstruction> {
    lines.iter()
        .map(<&String>::into)
        .collect()
}

fn calculate1(stack: &mut Stack, instructions: &Vec<MoveInstruction>) -> String {
    stack.move_crates(instructions);
    stack.get_top()
}

fn calculate2(stack: &mut Stack, instructions: &Vec<MoveInstruction>) -> String {
    stack.move_stacked_crates(instructions);
    stack.get_top()
}

pub(crate) fn main() {
    let lines = lines::read_lines("resources/day5.txt");
    if let Ok(lines) = lines {
        let (stack, procedure) = prepare_lines(&lines);
        let mut stack = prepare_stack(&stack);
        let procedure = prepare_instructions(&procedure);
        let contained = calculate1(&mut stack.clone(), &procedure);
        println!("Day 5: Top of Stack (CrateMover 9000) = {contained}");
        let contained = calculate2(&mut stack, &procedure);
        println!("Day 5: Top of Stack (CrateMover 9001) = {contained}");
    }
    println!()
}

#[cfg(test)]
mod tests {
    use super::{calculate1, calculate2, prepare_stack, prepare_lines, prepare_instructions};

    fn test_data() -> Vec<String> {
        vec![
            "    [D]    ",
            "[N] [C]    ",
            "[Z] [M] [P]",
            " 1   2   3 ",
            "",
            "move 1 from 2 to 1",
            "move 3 from 1 to 3",
            "move 2 from 2 to 1",
            "move 1 from 1 to 2",
        ].iter().map(<&str>::to_string).collect()
    }

    #[test]
    fn test1() {
        let input = test_data();
        let lines = prepare_lines(&input);
        let mut stack = prepare_stack(&lines.0);
        // println!("{:?}", stack.stack);
        let procedure = prepare_instructions(&lines.1);
        let x = calculate1(&mut stack, &procedure);
        // println!("{:?}", stack.stack);
        assert_eq!(x, "CMZ");
    }

    #[test]
    fn test2() {
        let input = test_data();
        let lines = prepare_lines(&input);
        let mut stack = prepare_stack(&lines.0);
        let procedure = prepare_instructions(&lines.1);
        let x = calculate2(&mut stack, &procedure);
        assert_eq!(x, "MCD");
    }
}