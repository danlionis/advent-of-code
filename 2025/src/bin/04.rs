use advent_of_code::BoundedGrid;
use rustc_hash::FxHashSet;

advent_of_code::solution!(4);

type HashSet<T> = FxHashSet<T>;

const DELTAS: &[(isize, isize)] = &[
    (-1isize, -1isize),
    (0, -1),
    (1, -1),
    (-1, 0),
    (1, 0),
    (-1, 1),
    (0, 1),
    (1, 1),
];

fn parse_input(input: &str) -> BoundedGrid<bool> {
    let bytes = input.trim().as_bytes();

    let first_newline = bytes.iter().position(|&e| e == b'\n').unwrap();

    let width = first_newline;
    let height = (bytes.len()) / first_newline;

    let mut grid = BoundedGrid::new(width, height);

    let mut y = 0;
    let mut x = 0;
    for chr in input.bytes() {
        match chr {
            b'@' => {
                grid.set(x, y, true);
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
    grid
}

fn num_neighbors(grid: &BoundedGrid<bool>) -> BoundedGrid<usize> {
    let mut res = BoundedGrid::new(grid.width, grid.height);
    for (x, y, _) in grid.positions().filter(|(_, _, set)| **set) {
        let mut count = 0;
        for (dx, dy) in deltas((x, y), DELTAS) {
            if let Some(true) = grid.get(dx, dy) {
                count += 1;
            }
        }
        res.set(x, y, count);
    }

    res
}

#[inline(never)]
fn deltas(pos: (usize, usize), ds: &[(isize, isize)]) -> impl Iterator<Item = (usize, usize)> {
    ds.iter().filter_map(move |(dx, dy)| {
        let x = pos.0.checked_add_signed(*dx)?;
        let y = pos.1.checked_add_signed(*dy)?;

        Some((x, y))
    })
}

pub fn part_one(input: &str) -> Option<u64> {
    let grid = parse_input(input);

    let mut res = 0;
    for pos in grid
        .positions()
        .filter_map(|(x, y, val)| if *val { Some((x, y)) } else { None })
    {
        let mut count = 0;
        for (dx, dy) in deltas(pos, DELTAS) {
            if let Some(true) = grid.get(dx, dy) {
                count += 1;
            }
        }

        if count < 4 {
            res += 1;
        }
    }
    Some(res)
}

pub fn precompute_deltas(
    grid: &BoundedGrid<bool>,
    ds: &[(isize, isize)],
) -> BoundedGrid<Box<[(usize, usize)]>> {
    let mut res = BoundedGrid::new(grid.width, grid.height);

    for (x, y) in grid
        .positions()
        .filter_map(|(x, y, set)| if *set { Some((x, y)) } else { None })
    {
        let mut v = Vec::with_capacity(ds.len());
        v.extend(deltas((x, y), ds).filter(|&(x, y)| x < grid.width && y < grid.height));

        res.set(x, y, v.into_boxed_slice());
    }

    res
}

pub fn part_two(input: &str) -> Option<usize> {
    part_two_opt2(input)
}

pub fn part_two_opt2(input: &str) -> Option<usize> {
    let grid = parse_input(input);
    let mut neighbors = num_neighbors(&grid);
    let rolls_total = neighbors.len();

    // let mut to_remove = Vec::with_capacity(rolls_total);
    let mut to_remove = neighbors
        .positions()
        .filter_map(|(x, y, num)| if *num < 4 { Some((x, y)) } else { None })
        .collect::<HashSet<_>>();

    let mut tmp = HashSet::default();
    // dbg!(&to_remove);

    loop {
        // print_grid(&neighbors, ".");
        if to_remove.is_empty() {
            break;
        }

        // neighbors.cleanup();
        // println!("\nremoving {} rolls ({})", to_remove.len(), neighbors.len());
        //
        // let mut removing = BoundedGrid::new(grid.width, grid.height);
        // for (x, y) in to_remove.iter() {
        //     removing.set(*x, *y, "x");
        // }
        // print_grid(&removing, ".");

        for (x, y) in to_remove.drain() {
            neighbors.remove(x, y);
            tmp.remove(&(x, y));

            for (dx, dy) in deltas((x, y), DELTAS) {
                if let Some(v) = neighbors.get_mut(dx, dy) {
                    *v -= 1;

                    if *v < 4 {
                        tmp.insert((dx, dy));
                    }
                }
            }
        }

        std::mem::swap(&mut to_remove, &mut tmp);
        tmp.clear();
    }

    neighbors.cleanup();
    let rolls_end = neighbors.len();

    Some(rolls_total - rolls_end)
}

pub fn part_two_opt(input: &str) -> Option<usize> {
    let mut grid = parse_input(input);
    let rolls_total = grid.iter().filter(|&&e| e).count();

    // let ds = precompute_deltas(&grid, DELTAS);

    // let mut check = Vec::
    let mut check = grid
        .positions()
        .filter_map(|(x, y, set)| if *set { Some((x, y)) } else { None })
        .collect::<HashSet<_>>();
    // let mut check = grid.clone();
    let mut to_remove = Vec::with_capacity(rolls_total);

    loop {
        // println!(
        //     "{} / {} ",
        //     check
        //         .positions()
        //         .filter_map(|(x, y, set)| if *set { Some((x, y)) } else { None })
        //         .count(),
        //     check.data.len(),
        // );
        'check: for &(x, y) in check.iter() {
            let mut count = 0;
            for (dx, dy) in deltas((x, y), DELTAS) {
                if let Some(true) = grid.get(dx, dy) {
                    count += 1;
                    if count >= 4 {
                        continue 'check;
                    }
                }
            }
            to_remove.push((x, y));
        }

        check.clear();

        // dbg!(to_remove.len());
        if to_remove.is_empty() {
            break;
        }

        for (x, y) in to_remove.drain(..) {
            grid.set(x, y, false);

            // for &(dx, dy) in ds.get_unchecked(x, y) {
            //     if *grid.get_unchecked(dx, dy) {
            for (dx, dy) in deltas((x, y), DELTAS) {
                if let Some(true) = grid.get(dx, dy) {
                    check.insert((dx, dy));
                }
            }
        }
    }

    let rolls_end = grid.iter().filter(|&&e| e).count();

    Some(rolls_total - rolls_end)
}

// pub fn part_two_old(input: &str) -> Option<usize> {
//     let mut grid = parse_input(input);
//
//     let rolls_total = grid.len();
//
//     let deltas = &[
//         (-1isize, -1isize),
//         (0, -1),
//         (1, -1),
//         (-1, 0),
//         (1, 0),
//         (-1, 1),
//         (0, 1),
//         (1, 1),
//     ];
//
//     let mut res = rolls_total;
//     let mut old_res = 0;
//
//     while res != old_res {
//         let grid_copied = grid.clone();
//
//         grid.retain(|pos| {
//             let count = deltas
//                 .iter()
//                 .map(|delta| (pos.0 + delta.0, pos.1 + delta.1))
//                 .map(|p| grid_copied.contains(&p))
//                 .filter(|e| identity(*e))
//                 .count();
//
//             count >= 4
//         });
//
//         old_res = res;
//         res = grid.len();
//     }
//
//     Some(rolls_total - res)
// }

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_deltas() {
        let mut ds = deltas((0, 0), DELTAS);
        let expected = &[(1, 0), (0, 1), (1, 1)];

        for d in expected {
            assert_eq!(ds.next(), Some(*d));
        }

        let mut ds = deltas((1, 1), DELTAS);
        let expected = &[
            (0, 0),
            (1, 0),
            (2, 0),
            (0, 1),
            (2, 1),
            (0, 2),
            (1, 2),
            (2, 2),
        ];

        for d in expected {
            assert_eq!(ds.next(), Some(*d));
        }
    }

    #[test]
    fn test_parsing() {
        let input = ".@@.\n@@..\n..@@";

        let expected =
            std::collections::HashSet::from_iter([(1, 0), (2, 0), (0, 1), (1, 1), (2, 2), (3, 2)]);

        let parsed = parse_input(input);

        let set = parsed
            .positions()
            .filter_map(|(x, y, val)| if *val { Some((x, y)) } else { None })
            .collect::<std::collections::HashSet<_>>();

        assert_eq!(set, expected);
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
