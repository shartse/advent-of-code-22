use std::{
    fs::File,
    io::{self, BufRead},
};

fn main() {
    let file = File::open("input.txt").unwrap();
    let mut state = State::new();
    for line in io::BufReader::new(file).lines().map(|x| x.unwrap()) {
        match Instruction::parse(&line) {
            Instruction::Noop => state.run_noop(),
            Instruction::AddX(x) => state.run_addx(x),
        }
    }

    let mut signal_strength = 0;
    for (i, value) in state.x_history.iter().enumerate() {
        let cycle = i as i32 + 1;
        if (cycle - 20) % 40 == 0 {
            signal_strength += value * cycle;
        }
    }
    println!("Sampled signal strength: {:?}", signal_strength);

    state.draw();
}

struct State {
    x: i32,
    cycle: i32,
    x_history: Vec<i32>,
}

impl State {
    fn new() -> Self {
        Self {
            x: 1,
            cycle: 0,
            x_history: Vec::new(),
        }
    }

    fn run_noop(&mut self) {
        self.tick();
    }

    fn run_addx(&mut self, x: i32) {
        self.tick();
        self.tick();
        self.x += x
    }

    fn tick(&mut self) {
        self.cycle += 1;
        self.x_history.push(self.x);
    }

    /// X register controls the horizontal position of a sprite. Specifically, the sprite is
    /// 3 pixels wide, and the X register sets the horizontal position of the middle of that
    /// sprite. (In this system, there is no such thing as "vertical position": if the sprite's
    /// horizontal position puts its pixels where the CRT is currently drawing, then those pixels
    /// will be drawn.)
    fn draw(&self) {
        for y in 0..6 {
            for x in 0..40 {
                let idx = x + 40 * y;
                let sprite_pos = self.x_history.get(idx).unwrap();
                if (sprite_pos - x as i32).abs() <= 1 {
                    print!("#");
                } else {
                    print!(".");
                }
            }
            println!();
        }
    }
}

#[derive(Debug)]
enum Instruction {
    Noop,
    AddX(i32),
}

impl Instruction {
    fn parse(line: &str) -> Self {
        match line {
            "noop" => Instruction::Noop,
            _ => {
                if line.starts_with("addx") {
                    let (_, num) = line.split_once(' ').unwrap();
                    Instruction::AddX(
                        num.parse()
                            .expect(&format!("Not a valid number: {:?}", num)),
                    )
                } else {
                    panic!("Invalid instruction: {:?}", line);
                }
            }
        }
    }
}
