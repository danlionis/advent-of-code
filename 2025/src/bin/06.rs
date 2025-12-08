advent_of_code::solution!(6);

pub fn parse_input(input: &str) -> (Vec<Vec<u64>>, Vec<u8>) {
    let num_lines = input.lines().count();
    let nums = input
        .lines()
        .take(num_lines - 1)
        .map(|l| {
            l.split_whitespace()
                .flat_map(str::parse)
                .collect::<Vec<u64>>()
        })
        .collect();

    let ops = input.lines().nth(num_lines - 1).unwrap();
    let ops = ops
        .split_whitespace()
        .map(|e| match e {
            "*" => b'*',
            "+" => b'+',
            _ => unreachable!(),
        })
        .collect();

    (nums, ops)
}

pub fn part_one(input: &str) -> Option<u64> {
    let (nums, ops) = parse_input(input);

    let mut res = 0;
    for (i, op) in ops.iter().enumerate() {
        res += nums
            .iter()
            .map(|v| v[i])
            .reduce(|acc, e| match op {
                b'*' => acc * e,
                b'+' => acc + e,
                _ => unreachable!(),
            })
            .unwrap();
    }

    Some(res)
}

pub fn get_num_from_char(c: u8) -> u64 {
    if c.is_ascii_digit() {
        return (c - b'0') as u64;
    }

    0
}

pub fn process_problem(problem: &Vec<&str>, len: usize) -> u64 {
    let operation = match problem.last().map(|s| s.trim()).unwrap() {
        "*" => |a: u64, b: u64| a * b,
        "+" => |a: u64, b: u64| a + b,
        _ => unreachable!(),
    };

    let nums = &problem[..problem.len() - 1];

    let mut res = Vec::new();
    for i in 0..len {
        let mut num = 0u64;
        for line in nums {
            let n = line.as_bytes()[i];
            if n.is_ascii_digit() {
                num *= 10;
                num += (n - b'0') as u64;
            }
        }

        res.push(num);
    }

    res.iter().cloned().reduce(operation).unwrap()
}

pub fn part_two(input: &str) -> Option<u64> {
    let mut lines: Vec<_> = input.lines().collect();

    let mut res = 0;
    let mut done = false;

    while !done {
        let first_whitespace = lines
            .iter()
            .map(|l| l.bytes().position(|e| e == b' ').unwrap_or(l.len()))
            .max()
            .unwrap();

        let mut problem = Vec::new();

        for line in lines.iter_mut() {
            let (num, rest) = line.split_at(first_whitespace);

            problem.push(num);

            if rest.is_empty() {
                done = true;
            }

            if !done {
                *line = &rest[1..];
            }
        }

        res += process_problem(&problem, first_whitespace);
    }

    Some(res)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(4277556));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(3263827));
    }
}
