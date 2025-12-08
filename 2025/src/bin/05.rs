use std::cmp;

advent_of_code::solution!(5);

pub fn parse_ranges(input: &str) -> Vec<(u64, u64)> {
    input
        .trim()
        .lines()
        .map(|line| {
            let dash = line.bytes().position(|b| b == b'-').unwrap();

            let (start, end) = line.split_at(dash);
            let end = &end[1..];

            let start: u64 = start.parse().unwrap();
            let end: u64 = end.parse().unwrap();

            (start, end)
        })
        .collect()
}

pub fn parse_input(input: &str) -> (Vec<(u64, u64)>, Vec<u64>) {
    let input = input.trim();
    let mut parts = input.split("\n\n");
    let ranges = parts.next().unwrap();
    let mut ingredients: Vec<u64> = parts.next().unwrap().lines().flat_map(str::parse).collect();
    ingredients.sort_unstable();

    let mut ranges = parse_ranges(ranges);
    ranges.sort_unstable();
    // dbg!(&ranges, &ingredients);

    (ranges, ingredients)
}

pub fn part_one(input: &str) -> Option<u64> {
    let (ranges, ingredients) = parse_input(input);

    let mut range_idx = 0;
    let mut ingredient_idx = 0;

    let mut res = 0;

    loop {
        let range = ranges.get(range_idx).copied();
        let (start, end) = match range {
            Some(r) => r,
            None => break,
        };

        let ingredient = ingredients.get(ingredient_idx).copied();
        let ingredient = match ingredient {
            Some(r) => r,
            None => break,
        };

        if ingredient < start {
            ingredient_idx += 1;
            continue;
        }

        if ingredient > end {
            range_idx += 1;
        }

        if ingredient >= start && ingredient <= end {
            res += 1;
            ingredient_idx += 1;
            continue;
        }
    }

    Some(res)
}

pub fn merge_two_ranges(a: (u64, u64), b: (u64, u64)) -> Option<(u64, u64)> {
    if a.0 <= b.1 && b.0 <= a.1 {
        let start = cmp::min(a.0, b.0);
        let end = cmp::max(a.1, b.1);

        return Some((start, end));
    }

    None
}

pub fn merge_ranges(ranges: &[(u64, u64)]) -> Vec<(u64, u64)> {
    let mut candidate = ranges[0];
    let mut res = Vec::new();

    for &next in &ranges[1..] {
        if let Some(merged) = merge_two_ranges(candidate, next) {
            candidate = merged;
        } else {
            res.push(candidate);
            candidate = next;
        }
    }

    res.push(candidate);

    res
}

pub fn part_two(input: &str) -> Option<u64> {
    let (ranges, _) = parse_input(input);

    let merged_ranges = merge_ranges(&ranges);

    let res = merged_ranges
        .iter()
        .map(|(start, end)| end - start + 1)
        .sum();
    Some(res)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_merge_range() {
        let a = (0, 5);
        let b = (6, 10);

        assert_eq!(merge_two_ranges(a, b), None);

        let a = (0, 5);
        let b = (5, 10);

        assert_eq!(merge_two_ranges(a, b), Some((0, 10)));

        let a = (5, 10);
        let b = (0, 5);

        assert_eq!(merge_two_ranges(a, b), Some((0, 10)));

        let a = (0, 7);
        let b = (3, 10);

        assert_eq!(merge_two_ranges(a, b), Some((0, 10)));
    }

    #[test]
    fn test_merge_ranges() {
        let ranges = &[(3, 5), (10, 14), (12, 18), (16, 20)];

        let res = merge_ranges(ranges);

        assert_eq!(&res, &[(3, 5), (10, 20)]);
    }

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(3));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(14));
    }
}
