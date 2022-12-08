use std::{
    collections::{HashMap, HashSet},
    fs::File,
    io::{self, BufRead},
};

use regex::Regex;

const ROOT: &str = "ROOT";

fn main() {
    let file = File::open("input.txt").unwrap();
    let mut dirs = traverse_dirs(file);
    let dir_sizes = resolve_sizes(&mut dirs);

    // Part 1
    let sum: usize = dir_sizes.values().filter(|&x| x <= &100000).sum();
    println!("Sum of sizes for all dirs <= 100,000: {}", sum);

    // Part 2
    let currently_free = 70000000 - dir_sizes.get(ROOT).unwrap();
    let need_to_free = 30000000 - currently_free;
    let mut candidates_to_free: Vec<&usize> =
        dir_sizes.values().filter(|&x| x >= &need_to_free).collect();
    candidates_to_free.sort();
    println!(
        "Size of smallest dir to free : {}",
        candidates_to_free.first().unwrap()
    );
}

fn resolve_sizes(sizes: &mut HashMap<String, Directory>) -> HashMap<String, usize> {
    let dirs: Vec<String> = sizes.keys().map(|key| key.to_string()).collect();
    let mut complete = HashMap::new();
    while complete.len() < dirs.len() {
        // Go through each known directory and attempt to resolve the sizes of its children.
        for name in dirs.iter() {
            let dir = sizes.get_mut(name).unwrap();
            let children_copy = dir.children.clone();
            // If there are no more children, add it to the complete list
            if dir.children.len() == 0 {
                complete.insert(name.clone(), dir.size);
            } else {
                // Otherwise, check if any of the children are in the complete list and use their sizes
                for child in children_copy.iter() {
                    let key = format!("{}/{}", name, child);
                    if let Some(size) = complete.get(&key) {
                        dir.children.remove(child);
                        dir.size += size;
                    }
                }
            }
        }
    }
    complete
}

fn traverse_dirs(file: File) -> HashMap<String, Directory> {
    let mut sizes: HashMap<String, Directory> = HashMap::new();
    let mut cur_path: Vec<String> = vec![ROOT.to_string()];

    for line in io::BufReader::new(file).lines().map(|x| x.unwrap()) {
        match Value::parse(&line) {
            Value::File(file) => match file {
                // Add the size of the file to the CWD size
                FileEntry::File { size } => {
                    if let Some(entry) = sizes.get_mut(&path(&cur_path)) {
                        entry.size += size;
                    } else {
                        sizes.insert(
                            path(&cur_path),
                            Directory {
                                size: size.clone(),
                                children: HashSet::new(),
                            },
                        );
                    }
                }
                // If none exists, create an entry for the CWD. Add this dir to its children
                FileEntry::Dir { name } => {
                    if let Some(entry) = sizes.get_mut(&path(&cur_path)) {
                        entry.children.insert(name.clone());
                    } else {
                        let mut children = HashSet::new();
                        children.insert(name.clone());
                        sizes.insert(path(&cur_path), Directory { size: 0, children });
                    }
                }
            },
            Value::Action(action) => match action {
                Action::Ls => (),
                Action::Cd(cd) => match cd {
                    // Reset the CWD to the root
                    CdDirection::Root => {
                        cur_path = vec![ROOT.to_string()];
                    }
                    // Move up the stack one level
                    CdDirection::Up => {
                        cur_path.pop().unwrap();
                    }
                    // Move down the stack into a directory
                    CdDirection::Into(name) => {
                        cur_path.push(name.to_string());
                    }
                },
            },
        }
    }
    sizes
}

fn path(path: &Vec<String>) -> String {
    path.join("/")
}

#[derive(Debug, Clone)]
struct Directory {
    size: usize,
    children: HashSet<String>,
}

enum Value {
    File(FileEntry),
    Action(Action),
}

impl Value {
    fn parse(line: &str) -> Self {
        if let Some(action) = Action::parse(&line) {
            return Value::Action(action);
        } else if let Some(file) = FileEntry::parse(&line) {
            return Value::File(file);
        }
        panic!("Every line should be one of the known variants")
    }
}

#[derive(Debug, Clone)]
enum FileEntry {
    File { size: usize },
    Dir { name: String },
}

impl FileEntry {
    fn parse(line: &str) -> Option<Self> {
        let re = Regex::new(r"(^\d*) (.*)").unwrap();
        if let Some(matches) = re.captures(line) {
            if let Some(size) = matches.get(1) {
                return Some(FileEntry::File {
                    size: size.as_str().parse().unwrap(),
                });
            }
        }
        let re = Regex::new(r"^dir (\D*)").unwrap();
        if let Some(matches) = re.captures(line) {
            if let Some(name) = matches.get(1) {
                return Some(FileEntry::Dir {
                    name: name.as_str().to_string(),
                });
            }
        }
        None
    }
}

#[derive(Debug)]
enum Action {
    Ls,
    Cd(CdDirection),
}

#[derive(Debug)]
enum CdDirection {
    Root,
    Up,
    Into(String),
}

impl Action {
    fn parse(line: &str) -> Option<Self> {
        if line.starts_with("$ ls") {
            return Some(Action::Ls);
        } else {
            let re = Regex::new(r"^\$ cd (.*)").unwrap();
            if let Some(matches) = re.captures(line) {
                if let Some(action) = matches.get(1) {
                    let direction = match action.as_str() {
                        ".." => CdDirection::Up,
                        "/" => CdDirection::Root,
                        x => CdDirection::Into(x.to_string()),
                    };
                    return Some(Action::Cd(direction));
                }
            }
        }
        return None;
    }
}
