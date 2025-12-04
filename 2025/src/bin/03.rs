advent_of_code::solution!(3);

pub fn find_max(s: &str) -> (usize, u64) {
    let mut max = 0;
    let mut pos = 0;
    for (i, c) in s.bytes().enumerate() {
        if c > max {
            max = c;
            pos = i;
        }
    }

    let max = max - b'0';

    (pos, max.into())
}

pub fn process_bank_part1(bank: &str) -> u64 {
    let (pos, first) = find_max(&bank[..(bank.len() - 1)]);

    let (_, second) = find_max(&bank[(pos + 1)..]);

    first * 10 + second
}

pub fn process_bank_part2(bank: &str, nums: usize) -> u64 {
    let mut res = 0;
    let mut pos = 0;

    for i in 1..=nums {
        let remaining = nums - i;
        let window_len = bank.len() - pos - remaining;

        let windowed_bank = &bank[pos..][..window_len];

        let (idx, x) = find_max(windowed_bank);
        pos += idx + 1;
        // dbg!(nums, i, remaining, window_len, windowed_bank, pos, idx, x);

        res = res * 10 + x;
    }

    res
}

pub fn part_one(input: &str) -> Option<u64> {
    // let res = input.lines().map(|e| process_bank_part2(e, 2)).sum();
    let res = input.lines().map(process_bank_part1).sum();
    Some(res)
}

pub fn part_two(input: &str) -> Option<u64> {
    let res = input.lines().map(|e| process_bank_part2(e, 12)).sum();
    Some(res)
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_max() {
        assert_eq!(find_max("1234"), (3, 4));
        assert_eq!(find_max("6543"), (0, 6));
    }

    #[test]
    fn test_process_bank() {
        assert_eq!(process_bank_part1("1234"), 34);
        assert_eq!(process_bank_part1("6543"), 65);
    }

    #[test]
    fn test_process_bank_p2() {
        assert_eq!(process_bank_part2("1234", 2), 34);
        assert_eq!(process_bank_part2("6543", 2), 65);
        assert_eq!(process_bank_part2("987654321111111", 12), 987654321111);
        assert_eq!(process_bank_part2("234234234234278", 12), 434234234278);
    }

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(357));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(3121910778619));
    }
}
