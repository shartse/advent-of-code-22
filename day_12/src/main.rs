use std::{
    collections::{HashMap, HashSet},
    fs::File,
    io::{self, BufRead},
};

fn main() {
    let file = File::open("input.txt").unwrap();
    let elevation_map = ElevationMap::new(file);
    let mut min_length = usize::MAX;
    for loc in elevation_map.grid.values() {
        if loc.val == 'a' || loc.val == 'S' {
            let length = elevation_map.min_path_len(loc);
            if length < min_length {
                min_length = length;
            }
        }
    }
    println!("Shortest path: {:?}", min_length);
}

struct ElevationMap {
    height: usize,
    width: usize,
    grid: HashMap<(usize, usize), Location>,
}

impl ElevationMap {
    fn new(file: File) -> Self {
        let mut grid = HashMap::new();
        let mut width = 0;
        let mut height = 0;
        for (y, line) in io::BufReader::new(file)
            .lines()
            .map(|x| x.unwrap())
            .enumerate()
        {
            height += 1;
            width = line.len();
            for (x, val) in line.chars().enumerate() {
                grid.insert((x, y), Location { val, idx: (x, y) });
            }
        }
        ElevationMap {
            height,
            width,
            grid,
        }
    }

    fn min_path_len(&self, start: &Location) -> usize {
        let mut visited: HashSet<(usize, usize)> = HashSet::new();
        let mut path_len = 0;
        let mut to_visit = Vec::new();
        to_visit.push(start);
        while to_visit.len() > 0 {
            let mut next_level = Vec::new();
            while let Some(loc) = to_visit.pop() {
                if visited.contains(&loc.idx) {
                    continue;
                }
                visited.insert(loc.idx);
                if loc.val == 'E' {
                    return path_len;
                }
                for neighbor in self.get_neighbors(&loc) {
                    next_level.push(neighbor);
                }
            }
            to_visit = next_level;
            path_len += 1;
        }
        usize::MAX
    }

    fn get_neighbors(&self, start: &Location) -> Vec<&Location> {
        let (x, y) = start.idx;
        let mut neighbors = Vec::new();
        if x > 0 {
            neighbors.push(self.get(x - 1, y));
        }
        if x < self.width - 1 {
            neighbors.push(self.get(x + 1, y));
        }
        if y > 0 {
            neighbors.push(self.get(x, y - 1));
        }
        if y < self.height - 1 {
            neighbors.push(self.get(x, y + 1));
        }

        neighbors
            .into_iter()
            .filter(|&n| n.height() <= start.height() + 1)
            .collect()
    }

    fn get(&self, x: usize, y: usize) -> &Location {
        self.grid.get(&(x, y)).unwrap()
    }
}

#[derive(Debug, Clone)]
struct Location {
    val: char,
    idx: (usize, usize),
}

impl Location {
    fn height(&self) -> usize {
        let character = match self.val {
            'S' => 'a',
            'E' => 'z',
            _ => self.val,
        };
        character as usize - 97
    }
}
