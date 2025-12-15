use std::fmt::Display;

use advent_of_code::{BoundedGrid, print_grid};

advent_of_code::solution!(7);

#[derive(Debug, Clone, PartialEq, Eq)]
enum State {
    Active,
    Inactive,
    Beam,
    Start,
}

impl Display for State {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let res = match self {
            State::Active => "x",
            State::Inactive => "^",
            State::Beam => "|",
            State::Start => "S",
        };

        f.write_str(res)
    }
}

impl TryFrom<u8> for State {
    type Error = ();

    fn try_from(value: u8) -> Result<Self, Self::Error> {
        match value {
            b'S' => Ok(State::Start),
            b'^' => Ok(State::Inactive),
            _ => Err(()),
        }
    }
}

fn parse_input(input: &str) -> (BoundedGrid<State>, (usize, usize)) {
    let width = input.bytes().position(|c| c == b'\n').unwrap();
    let height = input.len() / width - 1;

    let mut grid = BoundedGrid::new(width, height);
    let mut start = (0, 0);

    for (col, line) in input.lines().enumerate() {
        for (row, c) in line.bytes().enumerate() {
            if let Ok(state) = State::try_from(c) {
                if state == State::Start {
                    start = (row, col);
                    // grid.set(row, col, State::);
                } else {
                    grid.set(row, col, state);
                }
            }
        }
    }
    // print_grid(&grid, ".");

    (grid, start)
}

pub fn part_one(input: &str) -> Option<usize> {
    let (mut grid, start) = parse_input(input);

    let mut to_check = Vec::new();
    to_check.push(start);

    while let Some((row, col)) = to_check.pop() {
        // dbg!((row, col));
        // dbg!(grid.get(row, col));
        print_grid(&grid, ".");
        match grid.get(row, col) {
            None => {
                grid.set(row, col, State::Beam);

                if col + 1 < grid.height {
                    to_check.push((row, col + 1));
                }
            }
            Some(State::Inactive) => {
                grid.set(row, col, State::Active);
                to_check.push((row + 1, col));
                to_check.push((row - 1, col));
            }
            Some(_) => {}
        }
    }

    // print_grid(&grid, ".");

    let res = grid.iter().filter(|&e| e == &State::Active).count();

    Some(res)
    // None
}

pub fn part_two(input: &str) -> Option<usize> {
    None
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(21));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(40));
    }
}
