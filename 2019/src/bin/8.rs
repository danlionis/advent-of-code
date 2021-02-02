use std::fs;
use std::u8;

fn main() {
    part1();
    part2();
}

fn part2() {
    let width = 25;
    let height = 6;
    let layer_size = width * height;

    let image: Vec<u8> = fs::read_to_string("./input/input-8.txt")
        .unwrap()
        .split("")
        .filter_map(|c| c.parse().ok())
        .collect();

    // let layer_size = 4;
    // let image: Vec<u8> = "0222112222120002"
    //     .split("")
    //     .filter_map(|c| c.parse().ok())
    //     .collect();

    let mut result = Vec::new();
    for i in 0..layer_size {
        let mut tmp_index = i;

        while image[tmp_index] == 2 {
            tmp_index += layer_size;
        }

        result.push(image[tmp_index]);
    }

    for (i, &n) in result.iter().enumerate() {
        if i % width == 0 {
            println!("");
        }

        if n == 1 {
            print!("@");
        } else {
            print!(" ");
        }
    }

    // let res = String::from_iter(result.iter().map(|r| r.to_string()));
    // println!("part2: {}", res);
}

fn part1() {
    let layer_size = 25 * 6;

    let image: Vec<u8> = fs::read_to_string("./input/input-8.txt")
        .unwrap()
        .split("")
        .filter_map(|c| c.parse().ok())
        .collect();

    let min = image
        .chunks(layer_size)
        .map(|layer| {
            let zeroes = layer.iter().filter(|&&p| p == 0).count();
            let ones = layer.iter().filter(|&&p| p == 1).count();
            let twos = layer.iter().filter(|&&p| p == 2).count();

            (zeroes, ones * twos)
        })
        .min_by_key(|x| x.0)
        .unwrap()
        .1;
    println!("part1: {}", min);
}
