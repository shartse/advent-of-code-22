use std::{
    collections::BinaryHeap,
    fs::File,
    io::{self, BufRead},
};

fn main() {
    let file = File::open("input.txt").unwrap();
    let mut elves: BinaryHeap<i32> = BinaryHeap::new();
    let mut total_value = 0;
    for line in io::BufReader::new(file).lines().map(|x| x.unwrap()) {
        if line.len() > 0 {
            total_value += line.parse::<i32>().unwrap();
        } else {
            elves.push(total_value);
            total_value = 0;
        }
    }
    println!(
        "Total value of top 3 elves: {}",
        elves.pop().unwrap() + elves.pop().unwrap() + elves.pop().unwrap()
    );
}