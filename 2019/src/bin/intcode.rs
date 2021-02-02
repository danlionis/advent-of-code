extern crate aoc19;

use aoc19::intcode::Computer;
use std::io::Write;
use std::{env::args, io::stdout};

use std::collections::VecDeque;

struct StdinIter {
    prompt: Option<String>,
}

impl Iterator for StdinIter {
    type Item = String;

    fn next(&mut self) -> Option<Self::Item> {
        let mut stdout = std::io::stdout();
        if let Some(prompt) = &self.prompt {
            write!(stdout, "{}", prompt).unwrap();
            stdout.flush().unwrap();
        }
        let mut buf = String::new();
        std::io::stdin().read_line(&mut buf).ok()?;

        Some(buf)
    }
}

fn main() {
    let args: Vec<String> = args().collect();

    let si = StdinIter {
        prompt: Some("input required > ".to_owned()),
    };
    let si = si.map(|i| i.trim().parse::<isize>().unwrap());

    // for s in si {
    //     println!("{}", s.unwrap());
    // }

    assert!(args.len() >= 2, "memory required");

    let mem: Vec<isize> = args[1]
        .split(",")
        .filter_map(|op| op.parse().ok())
        .collect();

    let input: Vec<isize> = {
        if args.len() >= 3 {
            args[2].split(",").filter_map(|i| i.parse().ok()).collect()
        } else {
            Vec::new()
        }
    };

    // println!("mem_len: {}", mem.len());

    let mut computer = Computer::with_mem(&mem);

    computer.run(input.into_iter().chain(si), |out| {
        println!("[OUT] {:?}", out);
    });

    dbg!(computer.mem());
}
