use std::collections::HashMap;

const INPUT: &str = include_str!("../inputs/day6");

fn main() {
    let mut orbit_map: HashMap<String, Planet> = HashMap::new();
    for line in INPUT.lines() {
        let x = &line[0..3];
        let y = &line[4..7];
        orbit_map.insert(y.into(), Planet { parent: x.into() });
    }

    let mut hops_from_me_map = HashMap::new();

    let mut cur_key = "YOU";
    let mut hops = 0;
    while let Some(planet) = orbit_map.get(cur_key) {
        cur_key = &planet.parent;
        hops_from_me_map.insert(cur_key.clone(), hops);
        hops += 1;
    }

    cur_key = "SAN";
    hops = 0;

    while let Some(planet) = orbit_map.get(cur_key) {
        cur_key = &planet.parent;
        if let Some(n) = hops_from_me_map.get(cur_key) {
            hops += *n;
            break;
        }
        hops += 1;
    }

    println!("{}", hops);
}

struct Planet {
    parent: String,
}
