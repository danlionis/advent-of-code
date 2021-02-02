use std::collections::{HashMap, HashSet};
use std::fs;

fn main() {
    part1();
    part2();
}

fn part1() {
    let input = fs::read_to_string("input/input-6.txt").unwrap();

    let mut orbits: HashMap<&str, &str> = HashMap::new();

    for line in input.split("\n") {
        let mut parts = line.split(")");
        let center = parts.next().expect("center");
        let outside = parts.next().expect("outside");

        orbits.insert(outside, center);
    }

    let mut count = 0;

    for (k, _) in orbits.iter() {
        let mut current: &str = k;

        while let Some(k) = orbits.get(current) {
            current = k;
            count += 1;
        }
    }
    println!("part1: {}", count);
}

fn part2() {
    let input = fs::read_to_string("input/input-6.txt").unwrap();

    let mut orbits: HashMap<&str, &str> = HashMap::new();

    for line in input.split("\n") {
        let mut parts = line.split(")");
        let center = parts.next().expect("center");
        let outside = parts.next().expect("outside");

        orbits.insert(outside, center);
    }

    let mut santa_path = HashSet::new();

    let mut current: &str = "SAN";

    while let Some(&k) = orbits.get(current) {
        santa_path.insert(k);
        current = k;
    }

    let mut lowest_orbit = "";

    let mut current: &str = "YOU";
    while let Some(k) = orbits.get(current) {
        if santa_path.contains(k) {
            // we found the "lowest" orbit both SAN and YOU orbit
            lowest_orbit = k;
            break;
        }
        current = k;
    }

    let mut count = 0;
    let mut current: &str = "SAN";

    while let Some(&k) = orbits.get(current) {
        if k == lowest_orbit {
            break;
        }
        current = k;
        count += 1;
    }

    let mut current: &str = "YOU";
    while let Some(&k) = orbits.get(current) {
        if k == lowest_orbit {
            break;
        }
        current = k;
        count += 1;
    }

    println!("part2: {}", count);
}
