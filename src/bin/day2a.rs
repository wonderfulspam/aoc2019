const INPUT: &str = include_str!("../inputs/day2");

fn main() {
    let mut positions: Vec<usize> = INPUT
        .trim()
        .split(',')
        .map(|i| i.parse().unwrap())
        .collect();
    positions[1] = 12;
    positions[2] = 2;

    let mut counter = 0;
    while positions[counter] != 99 {
        let op = positions[counter];
        let src1 = positions[counter + 1];
        let src2 = positions[counter + 2];
        let dest = positions[counter + 3];
        match op {
            1 => positions[dest] = positions[src1] + positions[src2],
            2 => positions[dest] = positions[src1] * positions[src2],
            _ => panic!("Unknown operation"),
        }
        counter += 4;
    }
    println!("{}", positions[0]);
}
