#![allow(dead_code, mutable_borrow_reservation_conflict)]
use std::collections::HashMap;

const INPUT: &str = include_str!("../inputs/day11");

fn main() {
    let canvas = run_with_input(0);

    println!("{}", canvas.len());

    let proper_canvas = run_with_input(1);

    for i in 0..6 {
        for j in 0..30 {
            let colour = proper_canvas.get(&(j, i)).unwrap_or(&Colour::Black);
            match colour {
                Colour::Black => print!(" "),
                Colour::White => print!("."),
            }
        }
        println!();
    }
}

fn run_with_input(initial_input: i64) -> HashMap<(i64, i64), Colour> {
    let mut robot = Robot::new(INPUT, initial_input);
    let mut canvas: HashMap<(i64, i64), Colour> = HashMap::new();
    let mut done = false;
    while !done {
        robot.machine.run();
        done = robot.machine.get_status().1;
        let turn_signal = Turn::new(robot.machine.provide_output().unwrap());
        let colour = Colour::new(robot.machine.provide_output().unwrap());
        canvas.insert(robot.position, colour);
        robot.move_one(turn_signal);
        robot.machine.provide_input(
            canvas
                .get(&robot.position)
                .unwrap_or(&Colour::Black)
                .to_input(),
        );
    }
    canvas
}

fn get_previous(i: usize) -> usize {
    match i {
        0 => 4,
        _ => i - 1,
    }
}

fn parse_input_ops(input: &str) -> Vec<i64> {
    input
        .trim()
        .split(',')
        .map(|i| i.parse().unwrap())
        .collect()
}

enum Colour {
    Black,
    White,
}

impl Colour {
    fn new(int: i64) -> Self {
        match int {
            0 => Colour::Black,
            1 => Colour::White,
            _ => panic!("Unknown colour"),
        }
    }

    fn to_input(&self) -> i64 {
        match self {
            Colour::Black => 0,
            Colour::White => 1,
        }
    }
}

enum Turn {
    Left,
    Right,
}

impl Turn {
    fn new(int: i64) -> Self {
        match int {
            0 => Turn::Left,
            1 => Turn::Right,
            _ => panic!("Unknown turn"),
        }
    }
}

enum Direction {
    Up,
    Down,
    Left,
    Right,
}
impl Direction {
    fn rotate_left(&mut self) {
        *self = match self {
            Direction::Up => Direction::Left,
            Direction::Left => Direction::Down,
            Direction::Down => Direction::Right,
            Direction::Right => Direction::Up,
        }
    }
    fn rotate_right(&mut self) {
        self.rotate_left();
        self.rotate_left();
        self.rotate_left();
    }

    fn move_coords(&self, (x, y): (i64, i64)) -> (i64, i64) {
        match self {
            Direction::Up => (x, y - 1),
            Direction::Left => (x - 1, y),
            Direction::Down => (x, y + 1),
            Direction::Right => (x + 1, y),
        }
    }
}

struct Robot {
    direction: Direction,
    machine: IntCodeMachine,
    position: (i64, i64),
}

impl Robot {
    fn new(instruction_input: &str, initial_input: i64) -> Self {
        Robot {
            direction: Direction::Up,
            machine: IntCodeMachine::new(instruction_input, initial_input),
            position: (0, 0),
        }
    }

    fn move_one(&mut self, turn_signal: Turn) {
        match turn_signal {
            Turn::Left => self.direction.rotate_left(),
            Turn::Right => self.direction.rotate_right(),
        }
        self.position = self.direction.move_coords(self.position);
    }
}

struct IntCodeMachine {
    position: usize,
    memory: Vec<i64>,
    input_buffer: Vec<i64>,
    output_buffer: Vec<i64>,
    finished: bool,
    stuck: bool,
    relative_base_offset: i64,
    output_counter: usize,
}

impl IntCodeMachine {
    fn new(input: &str, phase_setting: i64) -> Self {
        IntCodeMachine {
            position: 0,
            memory: parse_input_ops(input),
            input_buffer: vec![phase_setting],
            output_buffer: vec![],
            finished: false,
            stuck: false,
            relative_base_offset: 0,
            output_counter: 0,
        }
    }

    fn run(&mut self) {
        self.stuck = false;
        while self.finished != true && self.stuck != true {
            self.stuck = self.advance();
        }
    }

    fn print_mem_as_str(&self) {
        println!("Position: {}", self.position);
        for i in self.memory.iter().cloned() {
            print!("{},", i);
        }
        println!();
    }

    fn get_status(&self) -> (bool, bool) {
        (self.stuck, self.finished)
    }

    fn provide_input(&mut self, input: i64) {
        self.input_buffer.push(input);
    }

    fn provide_output(&mut self) -> Option<i64> {
        self.output_buffer.pop()
    }

    fn advance(&mut self) -> bool {
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
                match self.input_buffer.get(0) {
                    Some(value) => {
                        self.set_value(*value, dest);
                        self.input_buffer.remove(0);
                    }
                    None => return true,
                }
            }
            Instruction::Output(c) => {
                let value = self.get_value(c, 1);
                self.output_counter += 1;
                //println!("Output #{}: {}", self.output_counter, value);
                self.output_buffer.push(value);
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
            Instruction::RelativeBaseOffset(c) => {
                self.relative_base_offset += self.get_value(c, 1);
            }
            Instruction::Stop => {
                self.finished = true;
            }
        }

        if !skip_position_shift {
            self.position += instruction.len();
        }

        false
    }

    fn set_value(&mut self, value: i64, position: usize) {
        if position >= self.memory.len() {
            self.memory.resize(position + 1, 0);
        }
        self.memory[position] = value;
    }

    fn get_position(&self, mode: Mode, offset: usize) -> usize {
        let position = self.position + offset;
        match mode {
            Mode::Immediate => position,
            Mode::Position => self.get(position) as usize,
            Mode::Relative => (self.get(position) + self.relative_base_offset) as usize,
        }
    }

    fn get_value(&self, mode: Mode, offset: usize) -> i64 {
        let position = self.get_position(mode, offset);
        self.get(position)
    }

    fn get(&self, position: usize) -> i64 {
        self.memory.get(position).map(|p| *p).unwrap_or(0)
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
    RelativeBaseOffset(Mode),
    Stop,
}

impl Instruction {
    fn new(input: i64) -> Self {
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
            9 => Instruction::RelativeBaseOffset(c),
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
            Instruction::RelativeBaseOffset(_) => 2,
            Instruction::Stop => 0,
        }
    }
}

#[derive(Debug, Copy, Clone)]
enum Mode {
    Position,
    Immediate,
    Relative,
}

impl Mode {
    fn new(input: i64) -> Self {
        match input {
            0 => Mode::Position,
            1 => Mode::Immediate,
            2 => Mode::Relative,
            _ => panic!("Unknown mode: {}", input),
        }
    }
}
