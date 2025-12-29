use std::{collections::HashMap, num::ParseIntError, str::FromStr};

use itertools::Itertools;

advent_of_code::solution!(8);

#[derive(Hash, PartialEq, Eq, Debug, PartialOrd, Ord, Copy, Clone)]
struct Point3 {
    x: i64,
    y: i64,
    z: i64,
}

impl Point3 {
    fn distance(&self, other: &Point3) -> f64 {
        let tmp = (self.x - other.x).pow(2) + (self.y - other.y).pow(2) + (self.z - other.z).pow(2);
        let f = tmp as f64;

        f.sqrt()
    }
}

impl FromStr for Point3 {
    type Err = ParseIntError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut i = s.split(",");
        let x = i.next().unwrap().parse()?;
        let y = i.next().unwrap().parse()?;
        let z = i.next().unwrap().parse()?;

        Ok(Point3 { x, y, z })
    }
}

pub fn part_one(input: &str) -> Option<u64> {
    part_one_with_connections(input, 1000)
}

pub fn part_one_with_connections(input: &str, max_connections: usize) -> Option<u64> {
    let points = input
        .trim()
        .lines()
        .map(|l| l.parse().unwrap())
        .collect::<Vec<Point3>>();

    let mut circuits: HashMap<Point3, usize> = HashMap::new();

    let mut distances = Vec::new();
    for pairs in points.iter().combinations(2) {
        // pairs.sort();

        let dist = Point3::distance(pairs[0], pairs[1]);

        distances.push((dist, *pairs[0], *pairs[1]));
    }

    distances.sort_unstable_by(|(a, _, _), (b, _, _)| f64::total_cmp(a, b));

    let mut next_circuit = 0;

    for (_, a, b) in distances.into_iter().take(max_connections) {
        let circuit_a = circuits.get(&a);
        let circuit_b = circuits.get(&b);

        if circuit_a.is_none() && circuit_b.is_none() {
            circuits.insert(a, next_circuit);
            circuits.insert(b, next_circuit);

            next_circuit += 1;
            continue;
        }

        if circuit_a.is_none()
            && let Some(circuit_b) = circuit_b
        {
            circuits.insert(a, *circuit_b);
            continue;
        }

        if circuit_b.is_none()
            && let Some(circuit_a) = circuit_a
        {
            circuits.insert(b, *circuit_a);
            continue;
        }

        let circuit_a = *circuit_a.unwrap();
        let circuit_b = *circuit_b.unwrap();

        if circuit_a == circuit_b {
            continue;
        }

        circuits
            .iter_mut()
            .filter(|(_, circuit)| **circuit == circuit_b)
            .for_each(|(_, circuit)| {
                *circuit = circuit_a;
            });
    }

    let mut counts: HashMap<usize, u64> = HashMap::new();

    for v in circuits.values() {
        *counts.entry(*v).or_default() += 1;
    }

    let res = counts.values().sorted().rev().take(3).product();

    Some(res)
}

pub fn part_two(input: &str) -> Option<u64> {
    let points = input
        .trim()
        .lines()
        .map(|l| l.parse().unwrap())
        .collect::<Vec<Point3>>();

    let mut circuits: HashMap<Point3, usize> = HashMap::new();

    let mut distances = Vec::new();

    for mut pairs in points.iter().combinations(2) {
        pairs.sort();

        let dist = Point3::distance(pairs[0], pairs[1]);

        distances.push((dist, *pairs[0], *pairs[1]));
    }

    distances.sort_unstable_by(|(a, _, _), (b, _, _)| f64::total_cmp(a, b));

    let mut next_circuit = 0;

    for (_, a, b) in distances {
        let circuit_a = circuits.get(&a);
        let circuit_b = circuits.get(&b);

        if circuit_a.is_none() && circuit_b.is_none() {
            circuits.insert(a, next_circuit);
            circuits.insert(b, next_circuit);

            next_circuit += 1;
        } else if circuit_a.is_none()
            && let Some(circuit_b) = circuit_b
        {
            circuits.insert(a, *circuit_b);
        } else if circuit_b.is_none()
            && let Some(circuit_a) = circuit_a
        {
            circuits.insert(b, *circuit_a);
        } else {
            let circuit_a = *circuit_a.unwrap();
            let circuit_b = *circuit_b.unwrap();

            if circuit_a == circuit_b {
                continue;
            }

            circuits
                .iter_mut()
                .filter(|(_, circuit)| **circuit == circuit_b)
                .for_each(|(_, circuit)| {
                    *circuit = circuit_a;
                });
        }

        if circuits.len() == points.len() && circuits.values().all_equal() {
            return Some((a.x * b.x) as u64);
        }
    }

    None
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result =
            part_one_with_connections(&advent_of_code::template::read_file("examples", DAY), 10);
        assert_eq!(result, Some(40));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(25272));
    }
}
