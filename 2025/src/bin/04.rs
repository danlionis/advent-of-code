use std::{collections::HashSet, convert::identity};

advent_of_code::solution!(4);

fn parse_input(input: &str) -> HashSet<(i64, i64)> {
    let mut res = HashSet::with_capacity(input.len());
    let mut y = 0;
    let mut x = 0;
    for chr in input.bytes() {
        match chr {
            b'@' => {
                res.insert((x, y));
                x += 1;
            }
            b'.' => x += 1,
            b'\n' => {
                y += 1;
                x = 0;
            }
            _ => {}
        };
    }
    res
}

pub fn part_one(input: &str) -> Option<u64> {
    let grid = parse_input(input);

    let deltas = &[
        (-1i64, -1i64),
        (0, -1),
        (1, -1),
        (-1, 0),
        (1, 0),
        (-1, 1),
        (0, 1),
        (1, 1),
    ];

    let mut res = 0;
    for pos in grid.iter() {
        let count = deltas
            .iter()
            .map(|delta| (pos.0 + delta.0, pos.1 + delta.1))
            .map(|p| grid.contains(&p))
            .filter(|e| identity(*e))
            .count();

        if count < 4 {
            res += 1;
        }
    }
    Some(res)
}

pub fn part_two(input: &str) -> Option<usize> {
    let mut grid = parse_input(input);

    let rolls_total = grid.len();

    let deltas = &[
        (-1i64, -1i64),
        (0, -1),
        (1, -1),
        (-1, 0),
        (1, 0),
        (-1, 1),
        (0, 1),
        (1, 1),
    ];

    let mut res = rolls_total;
    let mut old_res = 0;

    while res != old_res {
        let grid_copied = grid.clone();

        grid.retain(|pos| {
            let count = deltas
                .iter()
                .map(|delta| (pos.0 + delta.0, pos.1 + delta.1))
                .map(|p| grid_copied.contains(&p))
                .filter(|e| identity(*e))
                .count();

            count >= 4
        });

        old_res = res;
        res = grid.len();
    }

    Some(rolls_total - res)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parsing() {
        let input = r#".@@.
@@..
..@@"#;

        let expected = HashSet::from([(1, 0), (2, 0), (0, 1), (1, 1), (2, 2), (3, 2)]);

        assert_eq!(parse_input(input), expected);
    }

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(13));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(43));
        // assert_eq!(result, None);
    }
}
