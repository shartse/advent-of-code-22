use std::{
    collections::HashSet,
    fs::File,
    io::{self, BufRead},
};

fn main() {
    let file = File::open("input.txt").unwrap();
    let mut total_value = 0;
    let mut lines = io::BufReader::new(file).lines().map(|x| x.unwrap());
    loop {
        if let Some(a) = lines.next() {
            let b = lines.next().unwrap();
            let c = lines.next().unwrap();
            let score = score(duplicate(&a, &b, &c).unwrap());
            total_value += score;
        } else {
            break;
        }
    }
    println!("Total score: {}", total_value);
}

fn score(x: u8) -> i32 {
    if x >= 'a' as u8 && x <= 'z' as u8 {
        return x as i32 - 96;
    }
    if x >= 'A' as u8 && x <= 'Z' as u8 {
        return x as i32 - 38;
    }
    panic!("character supplied not alphabetic")
}

fn duplicate(a: &str, b: &str, c: &str) -> Option<u8> {
    let mut a_set = HashSet::new();
    for x in a.as_bytes() {
        a_set.insert(x);
    }
    let mut b_set = HashSet::new();
    for x in b.as_bytes() {
        b_set.insert(x);
    }
    for x in c.as_bytes() {
        if a_set.contains(x) && b_set.contains(x) {
            return Some(x.clone());
        }
    }
    None
}
