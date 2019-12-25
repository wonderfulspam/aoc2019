use std::collections::HashSet;

const INPUT: &str = include_str!("../inputs/day3");

fn main() {
    let inputs: Vec<_> = INPUT.lines().collect();
    let wire1 = parse(inputs[0]);
    let wire2 = parse(inputs[1]);
    let mut min_dist = 999999999;
    for intersection in wire1.intersection(&wire2) {
        let dist = intersection.0.abs() + intersection.1.abs();
        if min_dist > dist {
            min_dist = dist;
        }
    }
    println!("{}", min_dist);
}

fn parse(input: &str) -> HashSet<(i32, i32)> {
    let instructions: Vec<_> = input
    .trim()
    .split(',')
    .collect();
    let mut result: HashSet<(i32, i32)> = HashSet::new();
    let mut position = (0, 0);
    for instruction in instructions.iter() {
        let (direction, length) = instruction.split_at(1);
        for _ in 0..length.parse().unwrap() {
            match direction {
                "R" => position.0 += 1,
                "L" => position.0 -= 1,
                "U" => position.1 += 1,
                "D" => position.1 -= 1,
                _   => panic!("Unknown direction")
            }
            result.insert(position);
        }
    }
    result
}