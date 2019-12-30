use permutohedron::Heap;

const INPUT: &str = include_str!("../inputs/day7");

fn main() {
    let mut phase_settings = vec![5, 6, 7, 8, 9];
    let heap = Heap::new(&mut phase_settings);
    let mut permutations = Vec::new();
    for data in heap {
        permutations.push(data);
    }

    let mut highest_signal = 0;
    for permutation in permutations {
        let mut machines = Vec::with_capacity(5);
        let mut status = vec![false; 5];
        println!("Running permutation: {:?}", permutation);
        for i in 0..5 {
            machines.push(IntCodeMachine::new(INPUT, permutation[i]));
            println!("Started {}", i);
            machines[i].run();
        }
        println!("All machines running");
        machines[0].provide_input(0);
        machines[0].run();
        while status.contains(&false) {
            for i in 0..5 {
                let (stuck, finished) = &machines[i].get_status();
                status[i] = finished.clone();
                if !finished && stuck.clone() {
                    if let Some(output) = machines[get_previous(i)].provide_output() {
                        println!("Trying to get output from {}", get_previous(i));
                        machines[i].provide_input(output);
                        machines[i].run();
                    }

                }
            }
        }
        let output_signal = machines[4].provide_output().unwrap();
        if output_signal > highest_signal {
            highest_signal = output_signal;
        }
    }
    
    println!("{}", highest_signal);
}

fn get_previous(i: usize) -> usize {
    match i {
        0 => 4,
        _ => i - 1
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
    input_buffer: Vec<i32>,
    output_buffer: Vec<i32>,
    finished: bool,
    stuck: bool,
}

impl IntCodeMachine {
    fn new(input: &str, phase_setting: i32) -> Self {
        IntCodeMachine {
            position: 0,
            memory: parse_input_ops(input),
            input_buffer: vec![phase_setting],
            output_buffer: vec![],
            finished: false,
            stuck: false,
        }
    }

    fn run(&mut self) {
        self.stuck = false;
        while self.finished != true && self.stuck != true {
            self.stuck = self.advance();
        }
        //self.output_buffer.pop().unwrap()
    }

    fn get_status(&self) -> (bool, bool) {
        (self.stuck, self.finished)
    }

    fn provide_input(&mut self, input: i32) {
        self.input_buffer.push(input);
    }

    fn provide_output(&mut self) -> Option<i32> {
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
                        println!("Consumed from input buffer: {}", value);
                        self.set_value(*value, dest);
                        self.input_buffer.remove(0);
                    }
                    None => return true
                }
                
            }
            Instruction::Output(c) => {
                let value = self.get_value(c, 1);
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
            Instruction::Stop => {
                self.finished = true;
            }
        }

        if !skip_position_shift {
            self.position += instruction.len();
        }

        false
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
