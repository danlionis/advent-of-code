use std::collections::HashMap;

#[derive(Debug)]
enum Occurences {
    None,
    Once(u32),
    Twice(u32, u32),
}

impl Default for Occurences {
    fn default() -> Self {
        Occurences::None
    }
}

impl Occurences {
    fn shift(&mut self, input: u32) {
        match self {
            Occurences::None => *self = Occurences::Once(input),
            Occurences::Once(x) | Occurences::Twice(_, x) => *self = Occurences::Twice(*x, input),
        }
    }
}

fn main() {
    let input = &[0, 13, 16, 17, 1, 10, 6];

    let mut spoken: HashMap<u32, Occurences> = HashMap::new();

    let mut last = 0;
    let mut turn = 1;
    for &i in input.iter() {
        spoken.entry(i).or_default().shift(turn);
        last = i;
        turn += 1;
    }

    while turn != 30000001 {
        if turn == 2021 {
            println!("part1: {}", last);
        }

        match spoken.get(&last) {
            Some(Occurences::Once(_)) => {
                spoken.get_mut(&&0).unwrap().shift(turn);
                last = 0;
            }
            Some(Occurences::Twice(x, y)) => {
                let diff = y - x;
                spoken.entry(diff).or_default().shift(turn);
                last = diff;
            }
            _ => unreachable!(),
        }
        turn += 1;
    }

    println!("part1: {}", last);
}
