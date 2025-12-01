use std::{fmt::Display, str::FromStr};

advent_of_code::solution!(1);

#[derive(Debug)]
enum Direction {
    Left,
    Right,
}

impl FromStr for Direction {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "L" => Ok(Direction::Left),
            "R" => Ok(Direction::Right),
            _ => Err(()),
        }
    }
}

impl Display for Direction {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let c = match self {
            Direction::Left => "L",
            Direction::Right => "R",
        };
        write!(f, "{}", c)
    }
}

#[derive(Debug)]
struct Rotation {
    direction: Direction,
    distance: i64,
}

impl Rotation {
    fn value(&self) -> i64 {
        match self.direction {
            Direction::Left => -self.distance,
            Direction::Right => self.distance,
        }
    }
}

impl FromStr for Rotation {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let direction = s[0..1].parse()?;
        let distance = s[1..].parse().map_err(|_| ())?;

        Ok(Rotation {
            direction,
            distance,
        })
    }
}

impl Display for Rotation {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}{}", self.direction, self.distance)
    }
}

pub fn part_one(input: &str) -> Option<i64> {
    let rotations = input.lines().flat_map(Rotation::from_str);

    let mut current = 50;
    let mut zero_count = 0;

    for rot in rotations {
        current += rot.value();
        current = current.rem_euclid(100);

        if current == 0 {
            zero_count += 1;
        }
    }

    Some(zero_count)
}

pub fn part_two(input: &str) -> Option<i64> {
    let rotations = input.lines().flat_map(Rotation::from_str);

    let mut current = 50;
    let mut zero_count = 0;

    for rot in rotations {
        let new = current + rot.value() % 100;
        if !(1..100).contains(&new) && current != 0 {
            zero_count += 1;
        }
        zero_count += rot.distance / 100;

        current += rot.value();
        current = current.rem_euclid(100);
    }

    Some(zero_count)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(3));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(6));
    }
}
