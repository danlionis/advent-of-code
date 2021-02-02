use aoc19::intcode::{Computer, Status};
use std::collections::{HashSet, VecDeque};

fn main() {
    let mut droid = Computer::from_file("./input/input-15.txt");

    let mut graph = HashSet::new();

    graph.insert((0, 0));

    let mut pos = (0, 0);
    let mut mov_cmd = 1;
    let mut move_stack: Vec<isize> = Vec::new();

    loop {
        let status = droid.resume(Some(mov_cmd));

        match status {
            Status::Output(out) => if out == 0 {},
            Status::Input => {}
            _ => panic!("unknown state: {:?}", status),
        }
    }
}

fn move_pos(pos: (isize, isize), cmd: isize) -> (isize, isize) {
    match cmd {
        1 => (pos.0, pos.1 + 1),
        2 => (pos.0, pos.1 - 1),
        3 => (pos.0 - 1, pos.1),
        4 => (pos.0 + 1, pos.1),
        _ => panic!("illegal move"),
    }
}

fn inverse_move(mov: isize) -> isize {
    match mov {
        1 => 2,
        2 => 1,
        3 => 4,
        4 => 3,
        _ => panic!("illegal move"),
    }
}
