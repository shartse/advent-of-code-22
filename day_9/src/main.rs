use std::{
    collections::HashSet,
    fs::File,
    io::{self, BufRead},
};

fn main() {
    let file = File::open("input.txt").unwrap();
    let mut visited: HashSet<Location> = HashSet::new();
    let mut rope = vec![Location { x: 0, y: 0 }; 10];
    for line in io::BufReader::new(file).lines().map(|x| x.unwrap()) {
        Location::apply_action(&mut Action::parse(&line), &mut visited, &mut rope);
    }
    println!(
        "Number of locations visited at least once by the tail: {}",
        visited.len()
    );
}

#[derive(Debug)]
enum Action {
    Up(i32),
    Down(i32),
    Left(i32),
    Right(i32),
}

impl Action {
    fn parse(line: &str) -> Self {
        let (action, distance) = line.split_once(' ').unwrap();
        let distance: i32 = distance.parse().unwrap();
        match action {
            "U" => Action::Up(distance),
            "D" => Action::Down(distance),
            "L" => Action::Left(distance),
            "R" => Action::Right(distance),
            _ => panic!("Invalid input: {}", line),
        }
    }

    fn dist(&self) -> &i32 {
        match self {
            Action::Up(dist) => dist,
            Action::Down(dist) => dist,
            Action::Left(dist) => dist,
            Action::Right(dist) => dist,
        }
    }

    fn dec_dist(&mut self) {
        match self {
            Action::Up(dist) => *dist -= 1,
            Action::Down(dist) => *dist -= 1,
            Action::Left(dist) => *dist -= 1,
            Action::Right(dist) => *dist -= 1,
        };
    }
}

fn show(rope: &Vec<Location>) {
    let head = rope.first().unwrap();
    for y in 0..(head.y + 5) {
        'middle: for x in 0..(head.x + 5) {
            for (i, each) in rope.iter().enumerate() {
                if x == each.x && y == each.y {
                    print!("{}", i);
                    continue 'middle;
                }
            }
            print!(".");
        }
        println!();
    }
}

#[derive(Debug, Eq, Hash, PartialEq, Clone)]
struct Location {
    x: i32,
    y: i32,
}

impl Location {
    fn apply_action(
        action: &mut Action,
        visited: &mut HashSet<Location>,
        rope: &mut Vec<Location>,
    ) {
        while action.dist() > &0 {
            let mut prev = None;
            for curr in rope.iter_mut() {
                if let Some(head) = prev {
                    curr.catch_up_to(&head);
                } else {
                    curr.move_once(action);
                }
                prev = Some(curr.clone());
            }
            visited.insert(prev.unwrap());
        }
    }

    fn move_once(&mut self, action: &mut Action) {
        match action {
            Action::Up(_) => self.y += 1,
            Action::Down(_) => self.y -= 1,
            Action::Left(_) => self.x -= 1,
            Action::Right(_) => self.x += 1,
        }
        action.dec_dist();
    }

    /// If the head is ever two steps directly up, down, left, or right from the tail,
    /// the tail must also move one step in that direction so it remains close enough.
    /// Otherwise, if the head and tail aren't touching and aren't in the same row or column,
    /// the tail always moves one step diagonally to keep up.
    fn catch_up_to(&mut self, head: &Location) {
        let same_row = self.x == head.x;
        let same_col = self.y == head.y;

        if self.is_adjacent(head) {
            return;
        }
        if self.is_diagonal(head) {
            return;
        }

        if same_col {
            match self.x - head.x {
                2 => self.x -= 1,
                -2 => self.x += 1,
                _ => panic!(
                    "Must be within 2 rows of each other tail {:?}, head {:?}",
                    self, head
                ),
            }
            return;
        }

        if same_row {
            match self.y - head.y {
                2 => self.y -= 1,
                -2 => self.y += 1,
                _ => panic!(
                    "Must be within 2 rows of each other tail {:?}, head {:?}",
                    self, head
                ),
            }
            return;
        }

        if self.y > head.y {
            self.y -= 1
        } else if self.y < head.y {
            self.y += 1
        }

        if self.x > head.x {
            self.x -= 1
        } else if self.x < head.x {
            self.x += 1
        }
    }

    fn is_adjacent(&self, head: &Location) -> bool {
        let within_one_col = i32::abs(self.y - head.y) <= 1;
        let within_one_row = i32::abs(self.x - head.x) <= 1;
        return (within_one_col && self.x == head.x) || (within_one_row && self.y == head.y);
    }

    fn is_diagonal(&self, head: &Location) -> bool {
        let within_one_col = i32::abs(self.y - head.y) == 1;
        let within_one_row = i32::abs(self.x - head.x) == 1;
        return within_one_col && within_one_row;
    }
}
