fn main() {
    let input: Vec<usize> = std::fs::read_to_string("./input/input-2.txt")
        .unwrap()
        .split(',')
        .filter_map(|l| l.parse().ok())
        .collect();

    let res = run_program(input.clone(), 12, 2);
    println!("part1: {}", res);

    for noun in 0..=99 {
        for verb in 0..=99 {
            let res = run_program(input.clone(), noun, verb);
            if res == 19690720 {
                // println!("noun= {} verb= {} => {}", noun, verb, 100 * noun + verb);
                println!("part2: {}", 100 * noun + verb);

                break;
            }
        }
    }
}

fn run_program(mut input: Vec<usize>, noun: usize, verb: usize) -> usize {
    input[1] = noun;
    input[2] = verb;
    let mut i: usize = 0;
    loop {
        match input[i] {
            1 => {
                let add1 = input[i + 1];
                let add2 = input[i + 2];
                let res = input[i + 3];
                input[res] = input[add1] + input[add2];
            }
            2 => {
                let mul1 = input[i + 1];
                let mul2 = input[i + 2];
                let res = input[i + 3];
                input[res] = input[mul1] * input[mul2];
            }
            99 => {
                break;
            }
            _ => panic!("unknown opcode"),
        }
        i += 4;
    }

    input[0]
}
