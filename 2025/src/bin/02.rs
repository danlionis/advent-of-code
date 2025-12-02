use itertools::Itertools;

advent_of_code::solution!(2);

pub fn parse_range(input: &str) -> Result<(u64, u64), &'static str> {
    let mut parts = input.trim().split("-");

    let first = parts
        .next()
        .ok_or("empty string")
        .and_then(|p| p.parse::<u64>().map_err(|_| "invalid number"))?;

    let second = parts
        .next()
        .ok_or("empty string")
        .and_then(|p| p.parse::<u64>().map_err(|_| "invalid number"))?;

    if parts.next().is_some() {
        return Err("unexpected parts");
    }

    Ok((first, second))
}

struct Windows {
    modulo: u64,
    remaining: u64,
}

impl Iterator for Windows {
    type Item = u64;

    fn next(&mut self) -> Option<Self::Item> {
        if self.remaining == 0 {
            return None;
        }

        let next = self.remaining % self.modulo;

        self.remaining /= self.modulo;

        Some(next)
    }
}

fn windows(number: u64, digits: u32) -> Windows {
    Windows {
        modulo: 10u64.pow(digits),
        remaining: number,
    }
}

fn invalid_part1(n: u64) -> bool {
    let number_digits = n.checked_ilog10().unwrap_or(0) + 1;

    if !number_digits.is_multiple_of(2) {
        return false;
    }

    let digits = number_digits / 2;
    let modulo = 10u64.pow(digits);
    n / modulo == n % modulo
}

fn invalid_part2(n: u64) -> bool {
    let number_digits = n.checked_ilog10().unwrap_or(0) + 1;

    let digits = number_digits / 2;

    for i in 1..=digits {
        if !number_digits.is_multiple_of(i) {
            continue;
        }

        if windows(n, i).all_equal() {
            return true;
        }
    }

    false
}

pub fn part_one(input: &str) -> Option<u64> {
    let ranges = input.trim().split(",").flat_map(parse_range);

    let mut count = 0;
    for (start, end) in ranges {
        for i in start..=end {
            if invalid_part1(i) {
                // println!("invalid {} ({},{})", i, start, end);
                count += i;
            }
        }
    }
    Some(count)
}

pub fn part_two(input: &str) -> Option<u64> {
    let ranges = input.trim().split(",").flat_map(parse_range);

    let mut count = 0;
    for (start, end) in ranges {
        for i in start..=end {
            if invalid_part2(i) {
                // println!("invalid {} ({},{})", i, start, end);
                count += i;
            }
        }
    }
    Some(count)
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_odd() {
        assert!(!invalid_part1(101));
    }

    #[test]
    fn test_windows_1() {
        let mut digits = windows(1234, 1);
        assert_eq!(digits.next(), Some(4));
        assert_eq!(digits.next(), Some(3));
        assert_eq!(digits.next(), Some(2));
        assert_eq!(digits.next(), Some(1));
        assert_eq!(digits.next(), None);
    }

    #[test]
    fn test_windows_2() {
        let mut digits = windows(1234, 2);
        assert_eq!(digits.next(), Some(34));
        assert_eq!(digits.next(), Some(12));
        assert_eq!(digits.next(), None);
    }

    #[test]
    fn test_windows_4() {
        let mut digits = windows(1234, 4);
        assert_eq!(digits.next(), Some(1234));
        assert_eq!(digits.next(), None);
    }

    #[test]
    fn test_windows_1001() {
        assert!(!invalid_part1(1001));
    }

    #[test]
    fn test_windows_101() {
        assert!(!invalid_part1(101));
    }

    #[test]
    fn test_windows_565656() {
        assert!(invalid_part2(565656));
    }

    #[test]
    fn test_invalid() {
        assert!(invalid_part1(123123));
        assert!(!invalid_part1(123321));
        assert!(invalid_part1(11));
        assert!(invalid_part1(1212));
        assert!(!invalid_part1(12312));
        assert!(invalid_part1(55));
        assert!(invalid_part1(6464));
        assert!(invalid_part1(123123));
    }

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(1227775554));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(4174379265));
    }
}
