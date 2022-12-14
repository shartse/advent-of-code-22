use serde::{Deserialize, Serialize};
use std::{
    cmp::Ordering,
    fmt,
    fs::File,
    io::{self, BufRead},
};
// For this once, I got very stuck on parsing and got a lot of help from: https://fasterthanli.me/series/advent-of-code-2022/part-13
fn main() {
    let file = File::open("input.txt").unwrap();
    let mut idx_sum = 0;
    let mut idx = 0;

    let mut nodes: Vec<Node> = io::BufReader::new(file)
        .lines()
        .map(|x| x.unwrap())
        .filter(|x| x.len() > 0)
        .map(|x| serde_json::from_str::<Node>(&x).unwrap())
        .collect();

    for pair in nodes.chunks(2) {
        idx += 1;
        let (left, right) = (pair.get(0).unwrap(), pair.get(1).unwrap());
        println!();
        println!("{:?}", left);
        println!("{:?}", right);
        if left < right {
            idx_sum += idx;
        }
    }
    println!("Sum of valid message indices: {}", idx_sum);

    let div_1 = Node::List(vec![Node::Value(2)]);
    let div_2 = Node::List(vec![Node::Value(6)]);
    nodes.push(div_1.clone());
    nodes.push(div_2.clone());
    nodes.sort();

    let loc_1 = nodes.binary_search(&div_1).unwrap();
    let loc_2 = nodes.binary_search(&div_2).unwrap();
    println!("Product of divider indices: {}", (loc_1 + 1) * (loc_2 + 1));
}

#[derive(Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(untagged)]
enum Node {
    Value(i64),
    List(Vec<Node>),
}

impl fmt::Debug for Node {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Value(n) => write!(f, "{n}"),
            Self::List(n) => f.debug_list().entries(n).finish(),
        }
    }
}

impl std::cmp::PartialOrd for Node {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        match (self, other) {
            (Node::Value(l), Node::Value(r)) => l.partial_cmp(r),
            (l, r) => {
                let r = match r {
                    Node::Value(rr) => vec![Node::Value(rr.clone())],
                    Node::List(v) => v.to_vec(),
                };
                let l = match l {
                    Node::Value(ll) => vec![Node::Value(ll.clone())],
                    Node::List(v) => v.to_vec(),
                };

                for (l, r) in l.iter().zip(r.iter()) {
                    if let Some(v) = l.partial_cmp(r) {
                        if v != Ordering::Equal {
                            return Some(v);
                        }
                    }
                }
                return l.len().partial_cmp(&r.len());
            }
        }
    }
}

impl std::cmp::Ord for Node {
    fn cmp(&self, other: &Self) -> Ordering {
        self.partial_cmp(other).unwrap()
    }
}
