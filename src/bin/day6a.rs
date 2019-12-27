use std::collections::HashMap;

const INPUT: &str = include_str!("../inputs/day6");

fn main() {
    let mut orbit_map: HashMap<String, Planet> = HashMap::new();
    for line in INPUT.lines() {
        let x = &line[0..3];
        let y = &line[4..7];
        orbit_map.insert(y.into(), Planet { parent: x.into() });
    }

    let mut orbits = 0;

    for k in orbit_map.keys() {
        let mut cur_key = k;
        while let Some(planet) = orbit_map.get(cur_key) {
            cur_key = &planet.parent;
            orbits += 1;
        }
    }

    println!("{}", orbits);
}

struct Planet {
    parent: String,
}
