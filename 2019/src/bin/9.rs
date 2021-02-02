extern crate aoc19;

use aoc19::intcode::Computer;

fn main() {
    let mem: Vec<isize> = std::fs::read_to_string("./input/input-9.txt")
        .unwrap()
        .split(',')
        .filter_map(|l| l.parse().ok())
        .collect();

    let mut comp = Computer::with_mem(&mem);
    comp.run(vec![1], |out| {
        println!("part1: {}", out);
    });

    let mut comp = Computer::with_mem(&mem);
    comp.run(vec![2], |out| {
        println!("part2: {}", out);
    });
}
