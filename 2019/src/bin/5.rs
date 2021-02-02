extern crate aoc19;

use aoc19::intcode::Computer;

fn main() {
    let mem: Vec<isize> = std::fs::read_to_string("./input/input-5.txt")
        .unwrap()
        .split(',')
        .filter_map(|l| l.parse().ok())
        .collect();

    let mut comp = Computer::with_mem(&mem);
    comp.run(vec![1], |out| println!("part1: {}", out));

    let mut comp = Computer::with_mem(&mem);
    comp.run(vec![5], |out| println!("part2: {}", out));

    // run_program(input.clone(), 5);
    // println!("part1: {}", res);
}

// fn run_program(mut prog: Vec<isize>, input: isize) -> isize {
//     let mut i: usize = 0;
//     loop {
//         println!("ip= {}", i);
//         let instr = prog[i];
//         let opcode = instr % 100;

//         let mode1 = (instr / 100) % 10;
//         let mode2 = (instr / 1000) % 10;

//         let param1 = if mode1 == 0 {
//             prog[prog[i + 1] as usize]
//         } else {
//             prog[i + 1]
//         };

//         let param2 = if mode2 == 0 {
//             prog[prog[i + 2] as usize]
//         } else {
//             prog[i + 2]
//         };
//         // println!("{:?}", prog);

//         // println!(
//         //     "i= {:<3} instr= {:0>4} opcode= {} param1= ({}) {:>8} -> {:<8} param2= ({}) {:>8} -> {:<8}",
//         //     i,
//         //     instr,
//         //     opcode,
//         //     mode1,
//         //     prog[i + 1],
//         //     param1,
//         //     mode2,
//         //     prog[i + 2],
//         //     param2
//         // );

//         match opcode {
//             1 => {
//                 let res = prog[i + 3] as usize;
//                 prog[res] = param1 + param2;
//                 i += 4;
//             }
//             2 => {
//                 let res = prog[i + 3] as usize;
//                 prog[res] = param1 * param2;
//                 i += 4;
//             }
//             3 => {
//                 let position = prog[i + 1] as usize;
//                 prog[position] = input;
//                 i += 2;
//             }
//             4 => {
//                 let position = prog[i + 1] as usize;
//                 println!("data at [{}]= {}", position, prog[position]);
//                 i += 2;
//             }
//             5 => {
//                 if param1 != 0 {
//                     i = param2 as usize;
//                 } else {
//                     i += 3;
//                 }
//             }
//             6 => {
//                 if param1 == 0 {
//                     i = param2 as usize;
//                 } else {
//                     i += 3;
//                 }
//             }
//             7 => {
//                 let res = prog[i + 3] as usize;
//                 if param1 < param2 {
//                     prog[res] = 1;
//                 } else {
//                     prog[res] = 0;
//                 }
//                 i += 4;
//             }
//             8 => {
//                 let res = prog[i + 3] as usize;
//                 if param1 == param2 {
//                     prog[res] = 1;
//                 } else {
//                     prog[res] = 0;
//                 }
//                 i += 4;
//             }
//             99 => {
//                 break;
//             }
//             _ => panic!(
//                 "unknown opcode: {} at instruction {} ({})",
//                 opcode, instr, i
//             ),
//         }
//     }

//     0
//     // prog[0]
// }
