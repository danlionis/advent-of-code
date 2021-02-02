extern crate aoc19;

use aoc19::intcode::Computer;
use std::collections::{HashMap, HashSet};
use std::convert::TryInto;

fn main() {
    part1();
    part2();
}

fn part1() {
    let mem: Vec<isize> = std::fs::read_to_string("./input/input-11.txt")
        .unwrap()
        .split(',')
        .filter_map(|l| l.parse().ok())
        .collect();

    let mut brain = Computer::with_mem(&mem);

    let mut paint: HashMap<(isize, isize), isize> = HashMap::new();

    let mut current_location = (0, 0);
    let mut current_direction = Direction::Up;

    let mut already_painted = HashSet::new();

    while !brain.halted() {
        let color = paint.entry(current_location).or_insert(0);
        let new_color = brain.resume(Some(*color));

        let new_color = new_color.output();
        if new_color != *color {
            already_painted.insert(current_location);
        }

        paint.insert(current_location, new_color);

        let direction = brain.resume(None);

        if direction.output() == 0 {
            current_direction = current_direction.left();
        } else {
            current_direction = current_direction.right();
        }

        match current_direction {
            Direction::Up => current_location.1 += 1,
            Direction::Right => current_location.0 += 1,
            Direction::Down => current_location.1 -= 1,
            Direction::Left => current_location.0 -= 1,
        }
        brain.resume(None);
    }

    println!("part1: {}", already_painted.len());
}

fn part2() {
    let mem: Vec<isize> = std::fs::read_to_string("./input/input-11.txt")
        .unwrap()
        .split(',')
        .filter_map(|l| l.parse().ok())
        .collect();

    let mut brain = Computer::with_mem(&mem);

    let mut paint: HashMap<(isize, isize), isize> = HashMap::new();
    paint.insert((0, 0), 1);

    let mut current_location = (0, 0);
    let mut current_direction = Direction::Up;

    while !brain.halted() {
        let color = paint.entry(current_location).or_insert(0);
        let new_color = brain.resume(Some(*color));

        let new_color = new_color.output();

        paint.insert(current_location, new_color);

        let direction = brain.resume(None);
        // dbg!(&direction);

        if direction.output() == 0 {
            current_direction = current_direction.left();
        } else {
            current_direction = current_direction.right();
        }

        match current_direction {
            Direction::Up => current_location.1 -= 1,
            Direction::Right => current_location.0 += 1,
            Direction::Down => current_location.1 += 1,
            Direction::Left => current_location.0 -= 1,
        }
        brain.resume(None);
    }

    let min_x = paint.keys().map(|(x, _)| x).min().unwrap();
    let max_x = paint.keys().map(|(x, _)| x).max().unwrap();
    let diff_x: usize = (*max_x - *min_x).try_into().unwrap();

    let min_y = paint.keys().map(|(_, y)| y).min().unwrap();
    let max_y = paint.keys().map(|(_, y)| y).max().unwrap();
    let diff_y: usize = (*max_y - *min_y).try_into().unwrap();

    let mut disp = Vec::new();
    let vec_size = (diff_x + 1) as usize * (diff_y + 1) as usize;
    disp.resize(vec_size, 0);

    for ((x, y), v) in paint.iter() {
        // dbg!(x, y);
        let x: usize = (x + min_x.abs()).try_into().expect("x");
        let y: usize = (y + min_y.abs()).try_into().expect("y");
        // dbg!(x, y);
        let i: usize = (y * diff_x) + x;
        // dbg!(i);
        disp[i] = *v;
    }

    println!("part2: ");
    for chunk in disp.chunks(diff_x) {
        for d in chunk {
            if *d == 1 {
                print!("█");
            } else {
                print!(" ");
            }
        }
        println!("");
    }

    // println!("part2: {}", );
}

enum Direction {
    Left,
    Right,
    Up,
    Down,
}

impl Direction {
    fn right(self) -> Self {
        match self {
            Direction::Up => Direction::Right,
            Direction::Right => Direction::Down,
            Direction::Down => Direction::Left,
            Direction::Left => Direction::Up,
        }
    }

    fn left(self) -> Self {
        match self {
            Direction::Up => Direction::Left,
            Direction::Left => Direction::Down,
            Direction::Down => Direction::Right,
            Direction::Right => Direction::Up,
        }
    }
}
