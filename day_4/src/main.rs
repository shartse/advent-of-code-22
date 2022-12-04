use std::{
    fs::File,
    io::{self, BufRead},
};

fn main() {
    let file = File::open("input.txt").unwrap();
    let mut total_overlaps = 0;
    for line in io::BufReader::new(file).lines().map(|x| x.unwrap()) {
        let (right, left) = line.split_once(',').unwrap();
        if overlap(parse_range(right), parse_range(left)) {
            total_overlaps += 1;
        }
    }
    println!("Total number overlaps: {}", total_overlaps);
}

fn parse_range(val: &str) -> (i32, i32) {
    let (right, left) = val.split_once('-').unwrap();
    (right.parse::<i32>().unwrap(), left.parse::<i32>().unwrap())
}

fn overlap((a1, a2): (i32, i32), (b1, b2): (i32, i32)) -> bool {
    return (a1 >= b1 && a1 <= b2)
        || (a2 >= b1 && a2 <= b2)
        || (b1 >= a1 && b1 <= a2)
        || (b2 >= a1 && b2 <= a2);
}