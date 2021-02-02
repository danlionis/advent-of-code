#[allow(dead_code)]
extern crate aoc19;

use aoc19::intcode::{Computer, Status};
use std::collections::HashMap;
use std::convert::TryInto;

fn main() {
    part1();
    part2();
}

fn part2() {
    let mut comp = Computer::from_file("./input/input-13.txt");
    comp.write(0, 2);
    // let mut output = Vec::new();

    // let mut output: Vec<isize> = Vec::new();
    let mut output: HashMap<(isize, isize), isize> = HashMap::new();

    let mut input = None;

    let mut output_collector = Vec::new();

    let mut score = 0;

    loop {
        let status = comp.resume(input);
        match status {
            Status::Input => {
                std::thread::sleep(std::time::Duration::from_millis(20));
                visualize(&output);
                let mut ball = (0, 0);
                let mut paddle = (0, 0);

                for ((x, y), &tile) in output.iter() {
                    match tile {
                        3 => {
                            paddle = (*x, *y);
                        }
                        4 => {
                            ball = (*x, *y);
                        }
                        _ => {}
                    }
                }

                input = Some((ball.0 - paddle.0).signum());
            }
            Status::Output(out) => {
                output_collector.push(out);
                if output_collector.len() == 3 {
                    let x = output_collector[0];
                    let y = output_collector[1];
                    let tile = output_collector[2];
                    if x == -1 && y == 0 {
                        // println!("score= {}", tile);
                        score = tile;
                    } else {
                        *output.entry((x, y)).or_insert(0) = tile;
                    }
                    output_collector.clear();
                }
            }
            Status::Halted => break,
            _ => panic!("illegal state"),
        }
    }
    println!("part2: {}", score);
}

fn part1() {
    let mut comp = Computer::from_file("./input/input-13.txt");
    let mut output = Vec::new();
    comp.run(vec![], |o| {
        output.push(o);
    });

    let part1 = output.chunks(3).map(|c| c[2]).filter(|&t| t == 2).count();
    println!("part1: {}", part1);
}

fn visualize(output: &HashMap<(isize, isize), isize>) {
    let max_x: usize = output
        .keys()
        .map(|k| k.0)
        .max()
        .expect("max x")
        .try_into()
        .unwrap();
    let max_y: usize = output
        .keys()
        .map(|k| k.1)
        .max()
        .expect("max y")
        .try_into()
        .unwrap();

    let mut game = Vec::new();
    game.resize((max_x + 1) * (max_y + 1), " ");

    for ((x, y), tile) in output {
        if *x == -1 && *y == 0 {
            continue;
        }

        let i = *y as usize * max_x + *x as usize;
        let t = match tile {
            0 => " ",
            1 => "█",
            2 => "■",
            3 => "═",
            4 => {
                // dbg!(x, y);
                "o"
            }
            _ => panic!("unknown tile: x= {} y= {} tile= {}", x, y, tile),
        };
        // dbg!(i, x, y);
        game[i] = t;
    }
    let mut output = String::new();
    for c in game.chunks(max_x) {
        output.push_str(&c.join(""));
        output.push_str("\n");
        // println!("{}", c.join(""));
    }
    println!("{}", output);
}
