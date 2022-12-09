use std::{
    collections::BinaryHeap,
    fs::File,
    io::{self, BufRead},
};

fn main() {
    let file = File::open("input.txt").unwrap();

    let mut rows = Vec::new();
    for line in io::BufReader::new(file).lines().map(|x| x.unwrap()) {
        let cur_row: Vec<u32> = line.chars().map(|x| x.to_digit(10).unwrap()).collect();
        rows.push(cur_row);
    }

    let cols: Vec<Vec<u32>> = transpose(&rows);
    let num_rows = rows.len();
    let num_cols = rows.get(0).unwrap().len();

    let mut total_visible = 0;
    for y in 0..num_rows {
        for x in 0..num_cols {
            if visible_in_row(x, rows.get(y).unwrap()) || visible_in_row(y, cols.get(x).unwrap()) {
                total_visible += 1;
            }
        }
    }
    println!("Visible count: {}", total_visible);

    let mut heap = BinaryHeap::new();
    for y in 0..num_rows {
        for x in 0..num_cols {
            let (right, left) = distance_visible(x, rows.get(y).unwrap());
            let (up, down) = distance_visible(y, cols.get(x).unwrap());
            let score = right * left * up * down;
            heap.push(score);
        }
    }
    println!("Best view: {}", heap.pop().unwrap());
}

/// Whether or not the tree at this location is visible from the left or right
fn visible_in_row(idx: usize, line: &Vec<u32>) -> bool {
    let value = line.get(idx).unwrap();
    return visble(value, &line[0..idx]) || visble(value, &line[idx + 1..]);
}

/// Whether or not the tree at the start or end of this line is visible
fn visble(value: &u32, line: &[u32]) -> bool {
    line.iter().fold(true, |acc, x| acc && (x < value))
}

/// The distance (in trees) to the right and left that is visible from this tree
fn distance_visible(idx: usize, line: &Vec<u32>) -> (usize, usize) {
    let value = line.get(idx).unwrap();

    let right_side = &line[idx + 1..];
    let mut left_side = line[..idx].to_vec();
    left_side.reverse();
    (
        distance_right(value, right_side),
        distance_right(value, &left_side),
    )
}

/// The distance to the right that is visible from this tree
fn distance_right(value: &u32, line: &[u32]) -> usize {
    line.iter()
        .take_while(|&x| x < value)
        .fold(0, |acc, _| acc + 1)
}

/// Transpose the 2D array
fn transpose<T>(v: &Vec<Vec<T>>) -> Vec<Vec<T>>
where
    T: Clone,
{
    assert!(!v.is_empty());
    (0..v[0].len())
        .map(|i| v.iter().map(|inner| inner[i].clone()).collect::<Vec<T>>())
        .collect()
}
