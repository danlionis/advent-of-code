extern crate aoc19;
extern crate itertools;

use aoc19::intcode::{Computer, Status};
use itertools::*;

fn main() {
    part1();
    part2();
}

fn part1() {
    let mem: Vec<isize> = std::fs::read_to_string("./input/input-7.txt")
        .unwrap()
        .split(',')
        .filter_map(|l| l.parse().ok())
        .collect();

    let out = (0..5)
        .permutations(5)
        .map(|perm| {
            let mut out = 0;
            for i in 0..5 {
                let mut amp = Computer::with_mem(&mem);
                let _ = amp.resume(Some(perm[i]));
                let status = amp.resume(Some(out));
                if let Status::Output(output) = status {
                    out = output;
                }
            }
            out
        })
        .max()
        .unwrap();
    println!("part1: {}", out);
    // out
}

fn part2() {
    let mem: Vec<isize> = std::fs::read_to_string("./input/input-7.txt")
        .unwrap()
        .split(',')
        .filter_map(|l| l.parse().ok())
        .collect();

    let out = (5..10)
        .permutations(5)
        .map(|perm| {
            // let perm = vec![9, 7, 8, 5, 6];

            let mut amps: Vec<Computer> = (0..5).map(|_| Computer::with_mem(&mem)).collect();
            amps.iter_mut().enumerate().for_each(|(i, amp)| {
                let _ = amp.resume(Some(perm[i]));
            });
            let mut out = 0;
            while amps.iter().all(|c| !c.halted()) {
                for amp in amps.iter_mut() {
                    let status = amp.resume(Some(out));
                    if let Status::Output(output) = status {
                        out = output;
                    }
                }
            }
            out
        })
        .max()
        .unwrap();
    println!("part2: {}", out);
}
