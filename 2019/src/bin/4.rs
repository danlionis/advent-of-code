fn main() {
    part1();
    part2();
    // dbg!(password_valid2(112233));
    // dbg!(password_valid2(123444));
    // dbg!(password_valid2(111122));
    // dbg!(password_valid2(334444));
}

fn part1() {
    let lower_bound: usize = 234208;
    let upper_bound: usize = 765869;

    let result = (lower_bound..=upper_bound)
        .filter(|pass| password_valid(*pass))
        .count();
    println!("part1: {}", result);
}

fn part2() {
    let lower_bound: usize = 234208;
    let upper_bound: usize = 765869;

    let result = (lower_bound..=upper_bound)
        .filter(|pass| password_valid2(*pass))
        // .for_each(|p| println!("{}", p));
        .count();
    println!("part2: {}", result);
}

fn password_valid(pass: usize) -> bool {
    only_increase(pass) && two_adjacent(pass)
}

fn password_valid2(pass: usize) -> bool {
    only_increase(pass) && only_two_adjacent(pass)
}

fn only_increase(pass: usize) -> bool {
    let mut last = 10;
    for d in Digits::new(pass) {
        if d > last {
            return false;
        }
        last = d;
    }

    true
}

fn two_adjacent(pass: usize) -> bool {
    let mut last = 10;

    for d in Digits::new(pass) {
        if d == last {
            return true;
        }
        last = d;
    }

    false
}

fn only_two_adjacent(pass: usize) -> bool {
    let mut last = 10;
    let mut group_size = 1;

    for d in Digits::new(pass) {
        if d == last {
            group_size += 1;
        } else {
            if group_size == 2 {
                return true;
            }
            group_size = 1;
        }
        last = d;
    }

    if group_size == 2 {
        return true;
    }

    false
}

struct Digits {
    data: usize,
}

impl Digits {
    fn new(data: usize) -> Self {
        Digits { data }
    }
}

impl Iterator for Digits {
    type Item = u8;
    fn next(&mut self) -> Option<Self::Item> {
        if self.data == 0 {
            return None;
        }
        let res = self.data % 10;
        self.data /= 10;
        Some(res as u8)
    }
}
