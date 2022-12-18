use std::{
    collections::{HashMap, HashSet},
    fs::File,
    io::{self, BufRead},
};

use regex::Regex;

fn main() {
    let file = File::open("input.txt").unwrap();
    Cave::new(file);
}

struct Cave {
    origin: Pos,
    max: Pos,
    sensors: HashMap<Pos, Sensor>,
    beacons: HashSet<Pos>,
}

impl Cave {
    fn new(file: File) -> Cave {
        let (mut min_x, mut min_y) = (i32::MAX, i32::MAX);
        let (mut max_x, mut max_y) = (i32::MIN, i32::MIN);

        let mut sensors = HashMap::new();
        let mut beacons = HashSet::new();
        for line in io::BufReader::new(file).lines().map(|x| x.unwrap()) {
            let sensor = Sensor::parse(&line);
            let distance: i32 = sensor.beacon_distance() as i32;

            min_x = i32::min(min_x, sensor.pos.x() - distance);
            min_y = i32::min(min_y, sensor.pos.y() - distance);
            max_x = i32::max(max_x, sensor.pos.x() + distance);
            max_y = i32::max(max_y, sensor.pos.y() + distance);
            let beacon = sensor.closest_beacon.clone();
            beacons.insert(beacon);
            sensors.insert(sensor.pos.clone(), sensor);
        }

        let cave = Cave {
            origin: Pos(min_x, min_y),
            max: Pos(max_x, max_y),
            sensors,
            beacons,
        };

        println!(
            "Min: {}, {}, Max: {}, {}",
            cave.origin.x(),
            cave.origin.y(),
            cave.max.x(),
            cave.max.y()
        );
        let row = 2000000;
        println!(
            "{} calls that cannot contain a beacon in row {}",
            cave.get_row(row).len(),
            row
        );
        let loc = cave.search(&Pos(4000000, 4000000));
        println!("{:?} is where the beacon is", loc);
        println!("{:?} is the tuning frequency", loc.tuning_frequency());
        cave
    }

    fn search(&self, max: &Pos) -> Pos {
        let candiates = self.sensors.values().clone();

        for s in candiates {
            for pos in s.edges() {
                if pos.x() >= 0 && pos.x() <= max.x() && pos.y() >= 0 && pos.y() <= max.y() {
                    if self.sensors.get(&pos).is_none() && self.beacons.get(&pos).is_none() {
                        let mut open = true;
                        for s in self.sensors.values() {
                            if s.is_within_coverage(&pos) {
                                open = false;
                            }
                        }
                        if open {
                            return pos;
                        }
                    }
                }
            }
        }
        panic!("Couldn't find spot");
    }

    fn get_row(&self, y: i32) -> Vec<Pos> {
        let mut row = Vec::new();
        for x in self.origin.x()..self.max.x() {
            if self.sensors.get(&Pos(x, y)).is_none() && self.beacons.get(&Pos(x, y)).is_none() {
                for s in self.sensors.values() {
                    if s.is_within_coverage(&Pos(x, y)) {
                        row.push(Pos(x, y));
                        break;
                    }
                }
            }
        }
        row
    }

    fn draw(&self) {
        print!("  ");
        for x in self.origin.x()..self.max.x() + 1 {
            print!("{:3}", x);
        }
        println!();

        for y in self.origin.y()..self.max.y() + 1 {
            print!("{:3}", y);
            for x in self.origin.x()..self.max.x() + 1 {
                if self.sensors.get(&Pos(x, y)).is_some() {
                    print!(" S ");
                } else if self.beacons.get(&Pos(x, y)).is_some() {
                    print!(" B ");
                } else {
                    let mut covered = false;
                    for s in self.sensors.values() {
                        if s.is_within_coverage(&Pos(x, y)) {
                            covered = true;
                        }
                    }
                    if covered {
                        print!(" # ");
                    } else {
                        print!(" . ");
                    }
                }
            }
            println!()
        }
    }
}

#[derive(Debug, Eq, Hash, PartialEq, Clone)]
struct Pos(i32, i32);

impl Pos {
    fn distance(&self, other: &Pos) -> u32 {
        i32::abs_diff(self.x(), other.x()) + i32::abs_diff(self.y(), other.y())
    }

    fn x(&self) -> i32 {
        self.0
    }

    fn y(&self) -> i32 {
        self.1
    }

    /// Tuning frequency, which can be found by multiplying its x coordinate by 4000000 and then adding its y coordinate.
    fn tuning_frequency(&self) -> i64 {
        (self.x() as i64 * 4000000) + self.y() as i64
    }
}

#[derive(Debug, Clone)]
struct Sensor {
    pos: Pos,
    closest_beacon: Pos,
}

impl Sensor {
    fn parse(line: &str) -> Self {
        let re = Regex::new(
            r"^Sensor at x=(-*\d*), y=(-*\d*): closest beacon is at x=(-*\d*), y=(-*\d*)",
        )
        .unwrap();
        if let Some(cap) = re.captures(line) {
            Sensor {
                pos: Pos(
                    cap.get(1).unwrap().as_str().parse().unwrap(),
                    cap.get(2).unwrap().as_str().parse().unwrap(),
                ),
                closest_beacon: Pos(
                    cap.get(3).unwrap().as_str().parse().unwrap(),
                    cap.get(4).unwrap().as_str().parse().unwrap(),
                ),
            }
        } else {
            panic!("Not a valid Sensor string")
        }
    }

    fn beacon_distance(&self) -> u32 {
        self.pos.distance(&self.closest_beacon)
    }

    fn is_within_coverage(&self, pos: &Pos) -> bool {
        return self.pos.distance(pos) <= self.beacon_distance();
    }

    fn edges(&self) -> HashSet<Pos> {
        let mut edges = HashSet::new();
        let dist = self.beacon_distance() as i32 + 1;
        let (x, y) = (self.pos.x(), self.pos.y());
        for vertical in 0..dist + 1 {
            let horizontal = dist - vertical;
            edges.insert(Pos(x - horizontal, y + vertical));
            edges.insert(Pos(x - horizontal, y - vertical));
            edges.insert(Pos(x + horizontal, y + vertical));
            edges.insert(Pos(x + horizontal, y - vertical));
        }
        edges
    }
}

#[cfg(test)]
mod tests {
    use crate::{Pos, Sensor};
    #[test]
    fn edges() {
        let test = Sensor {
            pos: Pos(0, 0),
            closest_beacon: Pos(1, 1),
        };
        let set = vec![
            Pos(3, 0),
            Pos(2, 1),
            Pos(1, 2),
            Pos(0, 3),
            Pos(-1, 2),
            Pos(-2, 1),
            Pos(-3, 0),
            Pos(-2, -1),
            Pos(-1, -2),
            Pos(0, -3),
            Pos(1, -2),
            Pos(2, -1),
        ]
        .into_iter()
        .collect();
        assert_eq!(test.edges(), set);
    }
}
