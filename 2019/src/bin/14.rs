use std::collections::HashMap;
use std::str::FromStr;

#[derive(Debug, Clone)]
struct Pair {
    name: String,
    amount: i64,
}

impl FromStr for Pair {
    type Err = std::num::ParseIntError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut output = s.split_ascii_whitespace();

        Ok(Pair {
            amount: output.next().unwrap().parse()?,
            name: output.next().unwrap().to_owned(),
        })
    }
}

fn main() {
    println!("part1: {}", part1());
    println!("part2: {}", part2());
}

fn part1() -> i64 {
    let lines: Vec<String> = std::fs::read_to_string("./input/input-14.txt")
        .unwrap()
        .split("\n")
        .map(String::from)
        .collect();
    let mut recipies: HashMap<String, (Pair, Vec<Pair>)> = HashMap::new();

    for line in lines {
        let mut parts = line.split("=>").map(str::trim);
        let inputs = parts
            .next()
            .unwrap()
            .split(',')
            .filter_map(|s| Pair::from_str(s).ok())
            .collect();
        let output = {
            let output = parts.next().unwrap();
            Pair::from_str(output).unwrap()
        };
        recipies.insert(output.name.clone(), (output, inputs));
    }

    let mut inventory: HashMap<String, i64> = HashMap::new();
    // Start with 1 FUEL and 0 ORE
    inventory.insert("ORE".to_owned(), 0);
    inventory.insert("FUEL".to_owned(), 1);

    let mut tmp = Vec::new();
    let mut finished = false;

    while !finished {
        // try to split every item in inventory that has a positive amount and is not an ore
        inventory
            .iter_mut()
            .filter(|(name, amount)| **amount > 0 && name.as_str() != "ORE")
            .for_each(|(name, amount)| {
                let r = recipies.get(name).unwrap();
                let a = r.0.amount;
                *amount -= a;

                tmp.extend_from_slice(&r.1);
            });

        tmp.drain(..).for_each(|p| {
            *inventory.entry(p.name).or_insert(0) += p.amount;
        });

        // finished if all elements except for ore are either zero or spare
        finished = inventory.iter().filter(|i| i.0 != "ORE").all(|i| *i.1 <= 0);
    }

    return *inventory.get("ORE").unwrap();
}

fn part2() -> i64 {
    let lines: Vec<String> = std::fs::read_to_string("./input/input-14.txt")
        .unwrap()
        .split("\n")
        .map(String::from)
        .collect();
    let mut recipies: HashMap<String, (Pair, Vec<Pair>)> = HashMap::new();

    for line in lines {
        let mut parts = line.split("=>").map(str::trim);
        let inputs = parts
            .next()
            .unwrap()
            .split(',')
            .filter_map(|s| Pair::from_str(s).ok())
            .collect();
        let output = {
            let output = parts.next().unwrap();
            Pair::from_str(output).unwrap()
        };
        recipies.insert(output.name.clone(), (output, inputs));
    }

    for i in 0.. {
        let mut inventory: HashMap<String, i64> = HashMap::new();
        // Start with 1 FUEL and 0 ORE
        inventory.insert("ORE".to_owned(), 0);
        inventory.insert("FUEL".to_owned(), 10i64.pow(i));

        let mut tmp = Vec::new();
        let mut finished = false;

        while !finished {
            // try to split every item in inventory that has a positive amount and is not an ore
            inventory
                .iter_mut()
                .filter(|(name, amount)| **amount > 0 && name.as_str() != "ORE")
                .for_each(|(name, amount)| {
                    let r = recipies.get(name).unwrap();
                    let a = r.0.amount;
                    *amount -= a;

                    tmp.extend_from_slice(&r.1);
                });

            tmp.drain(..).for_each(|p| {
                *inventory.entry(p.name).or_insert(0) += p.amount;
            });

            // finished if all elements except for ore are either zero or spare
            finished = inventory.iter().filter(|i| i.0 != "ORE").all(|i| *i.1 <= 0);
        }

        println!("{}", *inventory.get("ORE").unwrap());
    }

    0
}
