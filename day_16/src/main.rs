use std::{
    collections::{BinaryHeap, HashMap},
    fmt::format,
    fs::File,
    io::{self, BufRead},
};

use regex::Regex;

fn main() {
    let file = File::open("test_input.txt").unwrap();
    let mut nodes = HashMap::new();
    for line in io::BufReader::new(file).lines().map(|x| x.unwrap()) {
        let node = Node::parse(&line);
        nodes.insert(node.name.clone(), node);
    }

    let mut cave = Cave {
        nodes,
        time: 1,
        pressure_released: 0,
    };
    println!("Cave: {:?}", cave);
}

#[derive(Debug, Clone)]
struct Cave {
    nodes: HashMap<String, Node>,
    time: usize,
    pressure_released: usize,
}

impl Cave {
    fn tick(&mut self) {
        self.time += 1;
        for (_, node) in &mut self.nodes {
            self.pressure_released += match node.state {
                State::Open => node.rate,
                State::Closed => 0,
            };
        }
    }

    fn new(nodes: HashMap<String, Node>) -> Self {
        Cave {
            nodes,
            time: 1,
            pressure_released: 0,
        }
    }

    fn max_pressure(
        nodes: HashMap<String, Node>,
        start: String,
        remaining: usize,
        table: &HashMap<(usize, String), usize>,
    ) -> usize {
        println!("Remaining time: {}, Visiting: {}", remaining, start);
        let start_node = nodes.get(&start).unwrap().clone();
        if remaining <= 0 {
            return 0;
        } else {
            todo!("Explore the space of different paths, memoizing some state as well go");
        }
    }
}

#[derive(Debug, Clone)]
struct Node {
    name: String,
    state: State,
    rate: usize,
    neighbors: Vec<String>,
}

impl Node {
    fn parse(line: &str) -> Self {
        // Valve AA has flow rate=0; tunnels lead to valves DD, II, BB

        let re = Regex::new(r"^Valve (.*) has flow rate=(\d*); tunnels* leads* to valves* (.*)")
            .unwrap();
        if let Some(cap) = re.captures(line) {
            let neighbors = cap
                .get(3)
                .unwrap()
                .as_str()
                .split(", ")
                .map(|x| x.to_string())
                .collect();
            Node {
                name: cap.get(1).unwrap().as_str().to_string(),
                state: State::Closed,
                rate: cap.get(2).unwrap().as_str().parse().unwrap(),
                neighbors,
            }
        } else {
            panic!("Not a valid node spec: {:?}", line);
        }
    }
}

#[derive(Debug, Clone)]
enum State {
    Open,
    Closed,
}

#[cfg(test)]
mod tests {
    use std::collections::HashMap;

    use crate::{Cave, Node};

    #[test]
    fn it_works() {

        let node_a = Node {
            name: "AA".to_string(),
            state: crate::State::Closed,
            rate: 0,
            neighbors: vec!["BB".to_string()],
        };
        let node_b = Node {
            name: "BB".to_string(),
            state: crate::State::Closed,
            rate: 13,
            neighbors: vec!["CC".to_string(), "AA".to_string()],
        };
        let node_c = Node {
            name: "CC".to_string(),
            state: crate::State::Closed,
            rate: 2,
            neighbors: vec!["BB".to_string()],
        };
        //    1    2          3    4             
        // a -> b -> turn on -> c -> turn on 
        //
        // (30 * 0) + (28 * 13) + (26 * 3)
        let nodes = vec![node_a, node_b, node_c];
        let mut node_map = HashMap::new();
        for node in nodes {
            node_map.insert(node.name.clone(), node);
        }

        let table = HashMap::new();
        assert_eq!(
            428,
            Cave::max_pressure(node_map, "AA".to_string(), 30, &table)
        );
    }
}
