use std::collections::{HashMap, HashSet};
use std::fs;

fn main() {
    let input = fs::read_to_string("input/input-3.txt").unwrap();
    let mut inputs = input.splitn(2, "\n");
    let wire1: Vec<&str> = inputs.next().unwrap().split(",").collect();
    let wire2: Vec<&str> = inputs.next().unwrap().split(",").collect();
    part1(&wire1, &wire2);
    part2(&wire1, &wire2);
}

type Position = (isize, isize);

fn part1(wire1input: &[&str], wire2input: &[&str]) {
    let mut wire1 = HashSet::new();

    let mut current_pos: Position = (0, 0);

    // wire1.insert(current_pos);

    wire1input.iter().for_each(|instruction| {
        let direction = &instruction[..1];
        let amount: usize = instruction[1..].parse().unwrap();

        for _ in 0..amount {
            match direction {
                "R" => current_pos.0 += 1,
                "L" => current_pos.0 -= 1,
                "U" => current_pos.1 += 1,
                "D" => current_pos.1 -= 1,
                _ => panic!("unknown instruction"),
            }
            wire1.insert(current_pos);
        }
    });

    let mut intersections = HashSet::new();

    current_pos = (0, 0);

    wire2input.iter().for_each(|instruction| {
        let direction = &instruction[..1];
        let amount: usize = instruction[1..].parse().unwrap();
        for _ in 0..amount {
            match direction {
                "R" => current_pos.0 += 1,
                "L" => current_pos.0 -= 1,
                "U" => current_pos.1 += 1,
                "D" => current_pos.1 -= 1,
                _ => panic!("unknown instruction"),
            }
            if wire1.contains(&current_pos) {
                intersections.insert(current_pos);
            }
        }
    });

    let result = intersections
        .iter()
        .map(|(x, y)| x.abs() + y.abs())
        .min()
        .unwrap();

    println!("part1: {}", result);
}

fn part2(wire1input: &[&str], wire2input: &[&str]) {
    let mut wire1: HashMap<Position, usize> = HashMap::new();

    let mut current_pos: Position = (0, 0);

    let mut current_step: usize = 0;

    wire1input.iter().for_each(|instruction| {
        let direction = &instruction[..1];
        let amount: usize = instruction[1..].parse().expect("parse amount");
        for _ in 0..amount {
            match direction {
                "R" => current_pos.0 += 1,
                "L" => current_pos.0 -= 1,
                "U" => current_pos.1 += 1,
                "D" => current_pos.1 -= 1,
                _ => panic!("unknown instruction"),
            };
            current_step += 1;
            if !wire1.contains_key(&current_pos) {
                wire1.insert(current_pos, current_step);
            }
        }
    });

    let mut intersections: HashMap<Position, usize> = HashMap::new();

    current_pos = (0, 0);

    current_step = 0;

    wire2input.iter().for_each(|instruction| {
        let direction = &instruction[..1];
        let amount: usize = instruction[1..].parse().expect("parse");
        for _ in 0..amount {
            match direction {
                "R" => current_pos.0 += 1,
                "L" => current_pos.0 -= 1,
                "U" => current_pos.1 += 1,
                "D" => current_pos.1 -= 1,
                _ => panic!("unknown instruction"),
            };
            current_step += 1;
            if let Some(steps) = wire1.get(&current_pos) {
                intersections.insert(current_pos, steps + current_step);
            }
        }
    });

    let result = intersections.iter().map(|(_, v)| v).min().expect("no min");

    println!("part2: {}", result);
}
