use std::collections::HashMap;
use std::collections::HashSet;

const INPUT: &str = include_str!("../inputs/day3");

fn main() {
    let inputs: Vec<_> = INPUT.lines().collect();
    let (wire1, steps1) = parse(inputs[0]);
    let (wire2, steps2) = parse(inputs[1]);
    let mut min_steps = 999999999;
    for intersection in wire1.intersection(&wire2) {
        let steps = steps1.get(intersection).unwrap() + steps2.get(intersection).unwrap();
        if min_steps > steps {
            min_steps = steps;
        }
    }
    println!("{}", min_steps);
}

fn parse(input: &str) -> (HashSet<(i32, i32)>, HashMap<(i32, i32), i32>) {
    let instructions: Vec<_> = input.trim().split(',').collect();
    let mut result: HashSet<(i32, i32)> = HashSet::new();
    let mut result_step: HashMap<(i32, i32), i32> = HashMap::new();
    let mut position = (0, 0);
    let mut steps = 0;
    for instruction in instructions.iter() {
        let (direction, length) = instruction.split_at(1);
        for _ in 0..length.parse().unwrap() {
            match direction {
                "R" => position.0 += 1,
                "L" => position.0 -= 1,
                "U" => position.1 += 1,
                "D" => position.1 -= 1,
                _ => panic!("Unknown direction"),
            }
            steps += 1;
            result.insert(position);
            result_step.insert(position, steps);
        }
    }
    (result, result_step)
}
