use std::{
    fs::File,
    io::{self, BufRead},
};

fn main() {
    let file = File::open("input.txt").unwrap();
    let rocks: Vec<Barrier> = io::BufReader::new(file)
        .lines()
        .map(|x| x.unwrap())
        .map(|x| Barrier::parse(&x))
        .collect();

    let pos: Vec<Pos> = rocks.clone().into_iter().map(|x| x.0).flatten().collect();
    let max_x = pos
        .iter()
        .fold(0, |acc, pos| if pos.x() > acc { pos.x() } else { acc })
        + 200;
    let min_x = pos
        .iter()
        .fold(max_x, |acc, pos| if pos.x() < acc { pos.x() } else { acc })
        - 200;
    let max_y = pos
        .iter()
        .fold(0, |acc, pos| if pos.y() > acc { pos.y() } else { acc })
        + 2;

    println!(
        "min_x: {}, max_x: {}, min_y: {}, max_y: {}",
        min_x, max_x, 0, max_y
    );
    let mut cave = Cave::new(Pos(min_x, 0), Pos(max_x, max_y));
    for barrier in rocks {
        cave.insert_barrier(barrier)
    }

    let count = cave.spawn_sand();
    println!("{} grains of sand", count);
}

#[derive(Debug)]
struct Cave {
    origin: Pos,
    grid: Vec<Vec<Material>>,
}

impl Cave {
    fn new(origin: Pos, size: Pos) -> Self {
        let mut grid = Vec::new();
        for _ in origin.y()..size.y() + 1 {
            let mut line = Vec::new();
            for _ in origin.x()..size.x() + 1 {
                line.push(Material::Air)
            }
            grid.push(line)
        }
        Cave { origin, grid }
    }

    fn draw(&self) {
        let start_x = self.origin.x();
        for y in 0..self.grid.len() {
            for x in start_x..start_x + self.grid.get(y).unwrap().len() {
                match self.get(&Pos(x, y)).unwrap() {
                    Material::Rock => print! {"#"},
                    Material::Air => print!("."),
                    Material::FallingSand => print!("+"),
                    Material::RestingSand => print!("o"),
                }
            }
            println!();
        }
    }

    fn get_mut(&mut self, pos: &Pos) -> &mut Material {
        let y = pos.y() - self.origin.y();
        let x = pos.x() - self.origin.x();
        self.grid.get_mut(y).unwrap().get_mut(x).unwrap()
    }

    fn get(&self, pos: &Pos) -> Option<&Material> {
        if pos.y() == self.grid.len() - 1 {
            return Some(&Material::Rock);
        }

        if pos.y() >= self.origin.y() && pos.x() >= self.origin.x() {
            let y = pos.y() - self.origin.y();
            let x = pos.x() - self.origin.x();
            if let Some(row) = self.grid.get(y) {
                return row.get(x);
            }
        }
        return None;
    }

    fn set(&mut self, pos: &Pos, item: Material) {
        let cell = self.get_mut(pos);
        *cell = item;
    }

    fn spawn_sand(&mut self) -> usize {
        let start = Pos(500, 0);
        self.set(&start, Material::FallingSand);
        let mut grains = 1;
        while self.advance_sand(&start) {
            self.draw();
            grains += 1;
        }
        grains
    }

    /// A unit of sand always falls down one step if possible. If the tile immediately below is
    /// blocked (by rock or sand), the unit of sand attempts to instead move diagonally one step
    /// down and to the left. If that tile is blocked, the unit of sand attempts to instead move
    /// diagonally one step down and to the right. Sand keeps moving as long as it is able to do so,
    /// at each step trying to move down, then down-left, then down-right. If all three possible
    /// destinations are blocked, the unit of sand comes to rest and no longer moves, at which point
    /// the next unit of sand is created back at the source.
    fn advance_sand(&mut self, pos: &Pos) -> bool {
        let mut pos = pos.clone();
        while let Some(next) = self.try_advance(&pos) {
            if let Some(_) = self.get(&next) {
                if pos == next {
                    self.set(&pos, Material::RestingSand);
                    if pos == Pos(500, 0) {
                        println!("filled to the top");
                        return false;
                    } else {
                        return true;
                    }
                }
                self.set(&pos, Material::Air);
                self.set(&next, Material::FallingSand);
                pos = next;
            } else {
                panic!("Trying to move out of bounds")
            }
        }
        println!("fell off the side");
        return false;
    }

    fn try_advance(&self, pos: &Pos) -> Option<Pos> {
        if let Some(below) = self.get(&pos.down()) {
            if let Material::Air = below {
                return Some(pos.down());
            }
        } else {
            return None;
        }

        if let Some(below) = self.get(&pos.down_left()) {
            if let Material::Air = below {
                return Some(pos.down_left());
            }
        } else {
            return None;
        }

        if let Some(below) = self.get(&pos.down_right()) {
            if let Material::Air = below {
                return Some(pos.down_right());
            }
        } else {
            return None;
        }

        Some(pos.clone())
    }

    fn insert_barrier(&mut self, barrier: Barrier) {
        let mut rocks = barrier.0.iter().peekable();
        while let Some(pos) = rocks.next() {
            if let Some(next) = rocks.peek() {
                if pos.x() == next.x() {
                    let start = usize::min(pos.y(), next.y());
                    let end = usize::max(pos.y(), next.y());
                    for y in start..end + 1 {
                        self.set(&Pos(pos.x(), y), Material::Rock)
                    }
                } else if pos.y() == next.y() {
                    let start = usize::min(pos.x(), next.x());
                    let end = usize::max(pos.x(), next.x());
                    for x in start..end + 1 {
                        self.set(&Pos(x, pos.y()), Material::Rock)
                    }
                } else {
                    panic!("Rocks should always be in lines")
                }
            };
        }
    }
}

#[derive(Debug)]
enum Material {
    Rock,
    Air,
    FallingSand,
    RestingSand,
}

#[derive(Debug, Clone, PartialEq)]
struct Pos(usize, usize);

impl Pos {
    fn parse(line: &str) -> Pos {
        let (x, y) = line.split_once(',').unwrap();
        Pos(x.parse().unwrap(), y.parse().unwrap())
    }

    fn x(&self) -> usize {
        self.0
    }

    fn y(&self) -> usize {
        self.1
    }

    fn down(&self) -> Self {
        Pos(self.0, self.1 + 1)
    }

    fn down_left(&self) -> Self {
        Pos(self.0 - 1, self.1 + 1)
    }

    fn down_right(&self) -> Self {
        Pos(self.0 + 1, self.1 + 1)
    }
}

#[derive(Debug, Clone)]
struct Barrier(Vec<Pos>);

impl Barrier {
    fn parse(line: &str) -> Barrier {
        Barrier(
            line.split(" -> ")
                .into_iter()
                .map(|x| Pos::parse(x))
                .collect(),
        )
    }
}
