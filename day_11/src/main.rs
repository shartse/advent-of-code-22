use std::{
    collections::{BinaryHeap, HashMap},
    fs::File,
    io::{self, BufRead},
};

fn main() {
    let factors: Vec<i64> = vec![11, 5, 7, 2, 17, 13, 3, 19];
    let file = File::open("input.txt").unwrap();
    let mut monkeys: HashMap<usize, Monkey> = HashMap::new();
    let mut tmp = Vec::new();
    for (i, line) in io::BufReader::new(file)
        .lines()
        .map(|x| x.unwrap())
        .enumerate()
    {
        if i != 0 && i % 6 == 0 {
            let monkey = Monkey::parse(tmp.clone(), &factors);
            monkeys.insert(monkey.id, monkey);
            tmp = Vec::new();
        }
        tmp.push(line);
    }
    let monkey = Monkey::parse(tmp.clone(), &factors);
    monkeys.insert(monkey.id, monkey);

    for _ in 0..10000 {
        for i in 0..monkeys.len() {
            let mut monkey = monkeys.remove(&i).unwrap();
            monkey.execute(&mut monkeys);
            monkeys.insert(i, monkey);
        }
    }

    let mut heap = BinaryHeap::new();
    for monkey in monkeys.values() {
        heap.push(monkey.inspections)
    }
    println!(
        "Total monkey business: {:?}",
        heap.pop().unwrap() * heap.pop().unwrap()
    )
}

fn show(monkeys: &HashMap<usize, Monkey>) {
    for i in 0..monkeys.len() {
        println!("{:?}", monkeys.get(&i).unwrap())
    }
    println!();
}

#[derive(Debug)]
struct Monkey {
    id: usize,
    starting_items: Vec<WorryLevel>,
    operation: Op,
    test: Test,
    inspections: usize,
}
impl Monkey {
    fn parse(lines: Vec<String>, factors: &Vec<i64>) -> Self {
        let id = {
            let (_, id) = lines.get(0).unwrap().split_once("Monkey ").unwrap();
            id.strip_suffix(':').unwrap().parse::<usize>().unwrap()
        };
        let starting_items = {
            let (_, right) = lines.get(1).unwrap().split_once(": ").unwrap();
            let items: Vec<WorryLevel> = right
                .clone()
                .split(", ")
                .into_iter()
                .map(|x| WorryLevel::new(x.parse::<i64>().unwrap(), &factors))
                .collect();
            items
        };
        let operation = Op::parse(lines.get(2).unwrap());
        let test = Test::parse(lines[3..6].to_vec());

        Monkey {
            id,
            starting_items,
            operation,
            test,
            inspections: 0,
        }
    }

    fn execute(&mut self, monkeys: &mut HashMap<usize, Monkey>) {
        self.starting_items.reverse();
        while let Some(mut worry_level) = self.starting_items.pop() {
            self.inspections += 1;
            worry_level.apply(&self.operation);
            // Evaluate the test to pick next monkey
            match self.test.expr {
                Expr::DivisibleBy(n) => {
                    let new_monkey = if worry_level.divisible_by(n) {
                        monkeys.get_mut(&self.test.if_arm).unwrap()
                    } else {
                        monkeys.get_mut(&self.test.else_arm).unwrap()
                    };
                    new_monkey.starting_items.push(worry_level);
                }
            };
        }
    }
}

#[derive(Debug)]
struct WorryLevel {
    mods: HashMap<i64, i64>,
}

impl WorryLevel {
    fn new(base: i64, factors: &Vec<i64>) -> Self {
        let mut mods = HashMap::new();
        for n in factors {
            mods.insert(n.clone(), base % n);
        }
        return WorryLevel { mods };
    }

    fn apply(&mut self, op: &Op) {
        for (n, entry) in self.mods.iter_mut() {
            match op {
                Op::Mult(num) => *entry = (*entry * num) % n,
                Op::Add(num) => *entry = (*entry + num) % n,
                Op::Exp => *entry = (*entry * *entry) % n,
            }
        }
    }

    fn divisible_by(&self, n: i64) -> bool {
        return self.mods.get(&n).unwrap() == &0;
    }
}

#[derive(Debug)]
enum Op {
    Mult(i64),
    Add(i64),
    Exp,
}

impl Op {
    fn parse(line: &str) -> Self {
        let (_, val) = line.split_once('=').unwrap();
        if let Some((_, right)) = val.split_once(" * ") {
            if right == "old" {
                return Op::Exp;
            } else {
                return Op::Mult(right.parse().unwrap());
            }
        } else if let Some((_, right)) = val.split_once(" + ") {
            return Op::Add(right.parse().unwrap());
        }
        panic!("Not a valid operation expression");
    }
}

#[derive(Debug)]
struct Test {
    expr: Expr,
    if_arm: usize,
    else_arm: usize,
}

impl Test {
    fn parse(line: Vec<String>) -> Test {
        let expr = Expr::parse(line.get(0).unwrap());
        let (_, if_arm) = line.get(1).unwrap().split_once(" monkey ").unwrap();
        let (_, else_arm) = line.get(2).unwrap().split_once(" monkey ").unwrap();
        Test {
            expr,
            if_arm: if_arm.parse().unwrap(),
            else_arm: else_arm.parse().unwrap(),
        }
    }
}

#[derive(Debug)]
enum Expr {
    DivisibleBy(i64),
}

impl Expr {
    fn parse(line: &str) -> Expr {
        if let Some((_, right)) = line.split_once(" by ") {
            Expr::DivisibleBy(right.parse().unwrap())
        } else {
            panic!("Not a valid expression");
        }
    }
}
