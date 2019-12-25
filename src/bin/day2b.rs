const INPUT: &str = include_str!("../inputs/day2");
const SUCCESS_CRITERION: usize = 19690720;

fn main() {
    let original_memory: Vec<usize> = INPUT
        .trim()
        .split(',')
        .map(|i| i.parse().unwrap())
        .collect();

    for a in 0..99 {
        for b in 0..99 {
            let mut positions = original_memory.clone();
            positions[1] = a;
            positions[2] = b;

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
            if positions[0] == SUCCESS_CRITERION {
                println!("{}", 100 * a + b);
                return;
            }
        }
    }
}
