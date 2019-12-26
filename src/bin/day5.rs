use scan_fmt::{scan_fmt, scanln_fmt};
use std::io::Write;

const INPUT: &str = include_str!("../inputs/day5");

fn main() {
    let mut machine = IntCodeMachine::new(INPUT);
    loop {
        machine.advance();
    }
}

fn parse_input_ops(input: &str) -> Vec<i32> {
    input
        .trim()
        .split(',')
        .map(|i| i.parse().unwrap())
        .collect()
}

struct IntCodeMachine {
    position: usize,
    memory: Vec<i32>,
}

impl IntCodeMachine {
    fn new(input: &str) -> Self {
        IntCodeMachine {
            position: 0,
            memory: parse_input_ops(input),
        }
    }

    fn advance(&mut self) {
        let instruction = Instruction::new(self.memory[self.position]);
        let mut skip_position_shift = false;
        match instruction {
            Instruction::Add(c, b, a) => {
                let source1 = self.get_value(c, 1);
                let source2 = self.get_value(b, 2);
                let dest = self.get_position(a, 3) as usize;
                self.set_value(source1 + source2, dest);
            }
            Instruction::Multiply(c, b, a) => {
                let source1 = self.get_value(c, 1);
                let source2 = self.get_value(b, 2);
                let dest = self.get_position(a, 3) as usize;
                self.set_value(source1 * source2, dest);
            }
            Instruction::Input(c) => {
                let dest = self.get_position(c, 1) as usize;
                print!("{} > ", dest);
                std::io::stdout().flush().ok();
                let value = scanln_fmt!("{}", i32).unwrap();
                self.set_value(value, dest);
            }
            Instruction::Output(c) => {
                let source = self.get_position(c, 1);
                let value = self.get_value(c, 1);
                println!("Output from {}: {}", source, value);
            }
            Instruction::JumpIfTrue(c, b) => {
                let value = self.get_value(c, 1);
                match value {
                    0 => (),
                    _ => {
                        let new_position = self.get_value(b, 2) as usize;
                        self.position = new_position;
                        skip_position_shift = true;
                    }
                }
            }
            Instruction::JumpIfFalse(c, b) => {
                let value = self.get_value(c, 1);
                match value {
                    0 => {
                        let new_position = self.get_value(b, 2) as usize;
                        self.position = new_position;
                        skip_position_shift = true;
                    }
                    _ => (),
                }
            }
            Instruction::LessThan(c, b, a) => {
                let source1 = self.get_value(c, 1);
                let source2 = self.get_value(b, 2);
                let mut value = 0;
                if source1 < source2 {
                    value = 1;
                }
                let dest = self.get_position(a, 3);
                self.set_value(value, dest);
            }
            Instruction::Equals(c, b, a) => {
                let source1 = self.get_value(c, 1);
                let source2 = self.get_value(b, 2);
                let mut value = 0;
                if source1 == source2 {
                    value = 1;
                }
                let dest = self.get_position(a, 3);
                self.set_value(value, dest);
            }
            Instruction::Stop => {
                println!("All done");
                std::process::exit(0);
            }
        }

        if !skip_position_shift {
            self.position += instruction.len();
        }
    }

    fn set_value(&mut self, value: i32, position: usize) {
        if position > self.memory.len() {
            self.memory.resize(position + 1, 0);
        }
        self.memory[position] = value;
    }

    fn get_position(&self, mode: Mode, offset: usize) -> usize {
        let position = self.position + offset;
        match mode {
            Mode::Immediate => position,
            Mode::Parameter => self.memory[position] as usize,
        }
    }

    fn get_value(&self, mode: Mode, offset: usize) -> i32 {
        match mode {
            Mode::Immediate => self.memory[self.position + offset],
            Mode::Parameter => {
                let position = self.memory[self.position + offset] as usize;
                self.memory[position]
            }
        }
    }
}

#[derive(Debug)]
enum Instruction {
    Add(Mode, Mode, Mode),
    Multiply(Mode, Mode, Mode),
    Input(Mode),
    Output(Mode),
    JumpIfTrue(Mode, Mode),
    JumpIfFalse(Mode, Mode),
    LessThan(Mode, Mode, Mode),
    Equals(Mode, Mode, Mode),
    Stop,
}

impl Instruction {
    fn new(input: i32) -> Self {
        let operation = input % 100;
        let a = Mode::new((input / 10000) % 10);
        let b = Mode::new((input / 1000) % 10);
        let c = Mode::new((input / 100) % 10);
        match operation {
            1 => Instruction::Add(c, b, a),
            2 => Instruction::Multiply(c, b, a),
            3 => Instruction::Input(c),
            4 => Instruction::Output(c),
            5 => Instruction::JumpIfTrue(c, b),
            6 => Instruction::JumpIfFalse(c, b),
            7 => Instruction::LessThan(c, b, a),
            8 => Instruction::Equals(c, b, a),
            99 => Instruction::Stop,
            _ => panic!("Unknown operation: {}", operation),
        }
    }

    fn len(&self) -> usize {
        match self {
            Instruction::Add(..) => 4,
            Instruction::Multiply(..) => 4,
            Instruction::Input(_) => 2,
            Instruction::Output(_) => 2,
            Instruction::JumpIfTrue(..) => 3,
            Instruction::JumpIfFalse(..) => 3,
            Instruction::LessThan(..) => 4,
            Instruction::Equals(..) => 4,
            Instruction::Stop => 0,
        }
    }
}

#[derive(Debug, Copy, Clone)]
enum Mode {
    Parameter,
    Immediate,
}

impl Mode {
    fn new(input: i32) -> Self {
        match input {
            0 => Mode::Parameter,
            1 => Mode::Immediate,
            _ => panic!("Unknown mode: {}", input),
        }
    }
}
