fn main() {
    let input: Vec<i32> = std::fs::read_to_string("./input/input-1.txt")
        .unwrap()
        .split('\n')
        .filter_map(|l| l.parse().ok())
        .collect();

    println!("part1: {}", part1(&input));
    println!("part2: {}", part2(&input));
}

fn part1(input: &[i32]) -> i32 {
    input.iter().map(|&m| fuel(m)).sum()
}

fn fuel(x: i32) -> i32 {
    x / 3 - 2
}

fn part2(input: &[i32]) -> i32 {
    input
        .iter()
        .map(|&m| {
            let mut tmp = fuel(m);
            let mut res = 0;
            while tmp > 0 {
                res += tmp;
                tmp = fuel(tmp);
            }
            res
        })
        .sum()
}
