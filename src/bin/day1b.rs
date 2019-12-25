const INPUT: &str = include_str!("../inputs/day1");

fn main() {
    let mut fuel = 0;
    for line in INPUT.lines() {
        let mass: i64 = line.parse().unwrap();
        let mut int_fuel = mass / 3 - 2;
        while int_fuel > 0 {
            fuel += int_fuel;
            int_fuel = int_fuel / 3 - 2;
        }
    }
    println!("{}", fuel);
}
