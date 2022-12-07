use std::collections::HashMap;
use std::str::FromStr;
use crate::lines;

#[derive(Debug)]
enum Item {
    File {
        _name: String,
        size: usize,
    },
    Directory {
        name: String,
    },
}

#[derive(Debug)]
struct FileSystem {
    data: HashMap<String, Directory>,
}

impl FileSystem {
    pub fn new() -> Self {
        Self {
            data: HashMap::new(),
        }
    }

    pub fn keys(&self) -> Vec<&String> {
        self.data.keys().collect::<Vec<&String>>()
    }

    fn directory_size(&self, name: &String) -> usize {
        let mut size = 0;
        let dir = self.data.get(name).unwrap();
        for item in &dir.files {
            size += match item {
                Item::File { size, .. } => *size,
                Item::Directory { name } => self.directory_size(&name),
            }
        }
        size
    }

    fn insert(&mut self, item: Directory) {
        self.data.insert(item.full_name(), item);
    }

    fn remove(&mut self, name: &String) -> Directory {
        self.data.remove(name).unwrap()
    }

    fn contains_key(&self, name: &String) -> bool {
        self.data.contains_key(name)
    }

    fn size(&self) -> usize {
        self.directory_size(&"".to_string())
    }
    fn sizes(&self) -> Vec<usize> {
        self.keys().into_iter()
            .map(|name| self.directory_size(name))
            .collect::<Vec<usize>>()
    }
}

#[derive(Debug)]
struct Directory {
    name: String,
    parent: Option<String>,
    files: Vec<Item>,
}

impl Directory {
    fn add_item(&mut self, file: Item) {
        self.files.push(file)
    }
    fn full_name(&self) -> String {
        match &self.parent {
            None => self.name.clone(),
            Some(name) => format!("{}/{}", name, self.name)
        }
    }
    fn subdirectory_name(&self, name: &String) -> String {
        format!("{}/{}", self.full_name(), name)
    }
    fn subdirectory(&self, name: &String) -> Self {
        Directory {
            name: name.clone(),
            parent: Some(self.full_name()),
            files: vec![],
        }
    }
}

#[derive(Debug)]
enum Command {
    Cd(CdDirectory),
    Ls,
}

impl FromStr for Command {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut split = s.split(" ")
            .collect::<Vec<&str>>();
        assert!(split.len() >= 2);
        let first = split.remove(1);
        match first {
            "cd" => {
                let second = split.remove(1);
                Ok(Command::Cd(second.parse().unwrap()))
            }
            "ls" => Ok(Command::Ls),
            _ => Err(()),
        }
    }
}

#[derive(Debug)]
enum CdDirectory {
    Directory(String),
    Root,
    Parent,
}

impl FromStr for CdDirectory {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "/" => Ok(CdDirectory::Root),
            ".." => Ok(CdDirectory::Parent),
            _ => Ok(CdDirectory::Directory(s.to_string())),
        }
    }
}

#[derive(Debug)]
enum LsOutput {
    Directory(String),
    File(String, usize),
}

impl FromStr for LsOutput {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut split = s.split(" ")
            .collect::<Vec<&str>>();
        assert_eq!(split.len(), 2);
        let first = split.remove(0);
        let second = split.remove(0);
        match first {
            "dir" => Ok(LsOutput::Directory(second.to_string())),
            _ => Ok(LsOutput::File(second.to_string(), first.parse().unwrap()))
        }
    }
}

#[derive(Debug)]
enum Output {
    Command(Command),
    Output(LsOutput),
}

impl FromStr for Output {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        if &s[0..2] == "$ " {
            Ok(Output::Command(s.parse().unwrap()))
        } else {
            Ok(Output::Output(s.parse().unwrap()))
        }
    }
}

fn parse_output(input: &Vec<String>) -> Vec<Output> {
    input.iter().filter_map(|s| s.parse().ok()).collect()
}

fn build_root(output: &Output) -> Directory {
    match output {
        Output::Command(c) => {
            match c {
                Command::Cd(d) => {
                    match d {
                        CdDirectory::Root => Directory {
                            name: "".to_string(),
                            parent: None,
                            files: vec![],
                        },
                        _ => panic!("First directory must be a /")
                    }
                }
                Command::Ls => panic!("First command must be cd")
            }
        }
        Output::Output(_) => panic!("First output must be a command")
    }
}

fn build_directories(output: &Vec<Output>) -> FileSystem {
    assert!(!output.is_empty());
    let first_line = output.get(0).unwrap();
    let mut current_directory = build_root(first_line);
    let mut file_system = FileSystem::new();
    for line in &output[1..] {
        match line {
            Output::Output(output) => {
                current_directory.add_item(match output {
                    LsOutput::Directory(name) => {
                        Item::Directory {
                            name: current_directory.subdirectory_name(name)
                        }
                    }
                    LsOutput::File(name, size) => {
                        Item::File {
                            _name: name.clone(),
                            size: *size,
                        }
                    }
                })
            }
            Output::Command(command) => {
                match command {
                    Command::Cd(name) => {
                        match name {
                            CdDirectory::Directory(name) => {
                                let current_name = current_directory.full_name();
                                assert!(!file_system.contains_key(&current_name));
                                let new_directory = current_directory.subdirectory(name);
                                file_system.insert(current_directory);
                                current_directory = new_directory;
                            }
                            CdDirectory::Parent => {
                                let current_parent = current_directory.parent.clone().expect("Cannot cd to parent from root");
                                assert!(file_system.contains_key(&current_parent), "current Directories: {:?}, {}", file_system, &current_parent);
                                file_system.insert(current_directory);
                                current_directory = file_system.remove(&current_parent)
                            }
                            CdDirectory::Root => panic!("Cannot cd to root anymore"),
                        }
                    }
                    Command::Ls => {}
                }
            }
        }
    }
    file_system.insert(current_directory);
    file_system
}

fn calculate1(file_system: &FileSystem) -> usize {
    file_system.keys().into_iter()
        .map(|name| file_system.directory_size(name))
        .filter(|s| *s <= 100_000usize)
        .sum()
}

fn calculate2(file_system: &FileSystem) -> usize {
    let currently_used = file_system.size();
    let free = 70_000_000 - currently_used;
    let needed = 30_000_000 - free;
    file_system.sizes().into_iter()
        .filter(|s| *s >= needed)
        .min().unwrap()
}

pub(crate) fn main() {
    let lines = lines::read_lines("resources/day7.txt");
    if let Ok(lines) = lines {
        let output = parse_output(&lines);
        let file_system = build_directories(&output);


        let result = calculate1(&file_system);
        println!("Day 7: Size of directories less than 100_000 = {result}");
        let result = calculate2(&file_system);
        println!("Day 7: Minimum size of directory to delete = {result}");
    }
    println!()
}

#[cfg(test)]
mod tests {
    use super::{calculate1, calculate2, parse_output, build_directories};

    fn test_data() -> Vec<String> {
        vec![
            "$ cd /",
            "$ ls",
            "dir a",
            "14848514 b.txt",
            "8504156 c.dat",
            "dir d",
            "$ cd a",
            "$ ls",
            "dir e",
            "29116 f",
            "2557 g",
            "62596 h.lst",
            "$ cd e",
            "$ ls",
            "584 i",
            "$ cd ..",
            "$ cd ..",
            "$ cd d",
            "$ ls",
            "4060174 j",
            "8033020 d.log",
            "5626152 d.ext",
            "7214296 k",
        ].iter().map(<&str>::to_string).collect()
    }

    #[test]
    fn test1() {
        let input = test_data();
        let output = parse_output(&input);
        let file_system = build_directories(&output);
        let results = calculate1(&file_system);
        let expected = 95437usize;
        assert_eq!(results, expected);
    }

    #[test]
    fn test2() {
        let input = test_data();
        let output = parse_output(&input);
        let file_system = build_directories(&output);
        let results = calculate2(&file_system);
        let expected = 24933642usize;
        assert_eq!(results, expected);
    }
}