/// Intcode Computer
///
/// This program errors at the end but somehow still manages to deliver the correct result :)
///
use std::convert::TryInto;
use std::fs;
use std::path::Path;

#[derive(Debug)]
pub struct Computer {
    mem: Vec<isize>,
    ip: isize,
    relative_base: isize,
    counter: usize,
}

impl Computer {
    pub fn with_mem(mem: &[isize]) -> Self {
        Computer {
            mem: mem.to_vec(),
            ip: 0,
            relative_base: 0,
            counter: 0,
        }
    }

    pub fn from_file<P>(path: P) -> Self
    where
        P: AsRef<Path>,
    {
        let mem = fs::read_to_string(path)
            .unwrap()
            .split(",")
            .map(|c| c.parse().unwrap())
            .collect();

        Computer {
            mem: mem,
            ip: 0,
            relative_base: 0,
            counter: 0,
        }
    }

    pub fn halted(&self) -> bool {
        self.read(self.ip()) == 99
    }

    pub fn mem(&self) -> &[isize] {
        &self.mem
    }

    pub fn mem_mut(&mut self) -> &mut [isize] {
        &mut self.mem
    }

    pub fn reserve(&mut self, size: usize) {
        if self.mem().len() < size {
            self.mem.resize(size, 0);
        }
    }

    pub fn counter(&self) -> usize {
        self.counter
    }

    pub fn step(&mut self, input: Option<isize>) -> Status {
        let (instruction, param_modes) = extract_command(self.read(self.ip()));
        self.counter += 1;

        match instruction {
            Instruction::Add => {
                let a = self.param(1, &param_modes[0]);
                let b = self.param(2, &param_modes[1]);
                // let out = self.out(3, &param_modes[2]);

                // self.write(out, a + b);
                self.out(3, &param_modes[2], a + b);
                self.ip += 4;
            }
            Instruction::Mul => {
                let a = self.param(1, &param_modes[0]);
                let b = self.param(2, &param_modes[1]);
                // let out = self.out(3, &param_modes[2]);

                // self.write(out, a * b);
                self.out(3, &param_modes[2], a * b);
                self.ip += 4;
            }
            Instruction::Input => {
                // let out = self.out(1, &param_modes[2]);
                if let Some(input) = input {
                    // self.write(out, input);
                    self.out(1, &param_modes[0], input);
                    self.ip += 2;
                } else {
                    return Status::Input;
                }
            }
            Instruction::Output => {
                let out = self.param(1, &param_modes[0]);
                // let output = self.read(out);
                self.ip += 2;
                return Status::Output(out);
            }
            Instruction::TJump => {
                let a = self.param(1, &param_modes[0]);
                let b = self.param(2, &param_modes[1]);

                if a != 0 {
                    self.ip = b;
                } else {
                    self.ip += 3;
                }
            }
            Instruction::FJump => {
                let a = self.param(1, &param_modes[0]);
                let b = self.param(2, &param_modes[1]);

                if a == 0 {
                    self.ip = b;
                } else {
                    self.ip += 3;
                }
            }
            Instruction::Less => {
                let a = self.param(1, &param_modes[0]);
                let b = self.param(2, &param_modes[1]);
                // let out = self.out(3, &param_modes[2]);

                let result = if a < b { 1 } else { 0 };

                // self.write(out, result);
                self.out(3, &param_modes[2], result);
                self.ip += 4;
            }
            Instruction::RelativeBaseOffset => {
                let a = self.param(1, &param_modes[0]);

                self.relative_base += a;
                self.ip += 2;
            }
            Instruction::Equal => {
                let a = self.param(1, &param_modes[0]);
                let b = self.param(2, &param_modes[1]);

                let result = if a == b { 1 } else { 0 };

                // self.write(out, result);
                self.out(3, &param_modes[2], result);
                self.ip += 4;
            }
            Instruction::Halt => return Status::Halted,
        }

        Status::Cont
    }

    pub fn resume(&mut self, mut input: Option<isize>) -> Status {
        loop {
            let status = self.step(input.take());
            match status {
                Status::Cont => {}
                _ => return status,
            }
        }
    }

    pub fn run<I, O>(&mut self, input: I, mut output: O) -> Status
    where
        I: IntoIterator<Item = isize>,
        O: FnMut(isize),
    {
        let mut input = input.into_iter();
        let mut next_input = None;

        loop {
            let status = self.resume(next_input);

            match status {
                Status::Input => {
                    next_input = input.next();
                }
                Status::Output(out) => {
                    output(out);
                }
                other => return other,
            }
        }
    }

    pub fn read(&self, i: isize) -> isize {
        let i: usize = i.try_into().unwrap();
        // self.reserve(i + 1);
        *self.mem.get(i).unwrap_or(&0)
    }

    pub fn write(&mut self, i: isize, val: isize) {
        let i: usize = i.try_into().unwrap();

        self.reserve(i + 1);
        self.mem[i] = val;
    }

    pub fn ip(&self) -> isize {
        self.ip
    }

    fn param(&self, offset: isize, mode: &ParamMode) -> isize {
        let raw: isize = self.read(self.ip() + offset);

        match mode {
            ParamMode::Position => self.read(raw),
            ParamMode::Immediate => raw,
            ParamMode::Relative => self.read(raw + self.relative_base),
        }
    }

    fn out(&mut self, offset: isize, mode: &ParamMode, value: isize) {
        let raw: isize = self.read(self.ip() + offset);

        match mode {
            ParamMode::Position => self.write(raw, value),
            ParamMode::Relative => self.write(raw + self.relative_base, value),
            _ => panic!("unknown param mode"),
        }
    }
}

#[derive(Debug)]
pub enum Status {
    Cont,
    Input,
    Output(isize),
    Halted,
}

impl Status {
    pub fn output(&self) -> isize {
        match self {
            Status::Output(output) => *output,
            _ => panic!("Cannot get output from incompatible state"),
        }
    }
}

#[derive(Debug)]
enum Instruction {
    Add,
    Mul,
    Input,
    Output,
    TJump,
    FJump,
    Less,
    Equal,
    RelativeBaseOffset,
    Halt,
}

impl From<isize> for Instruction {
    fn from(input: isize) -> Self {
        match input {
            1 => Instruction::Add,
            2 => Instruction::Mul,
            3 => Instruction::Input,
            4 => Instruction::Output,
            5 => Instruction::TJump,
            6 => Instruction::FJump,
            7 => Instruction::Less,
            8 => Instruction::Equal,
            9 => Instruction::RelativeBaseOffset,
            99 => Instruction::Halt,
            _ => panic!("unknown instruction code: {}", input),
        }
    }
}

#[derive(Debug)]
enum ParamMode {
    Position,
    Immediate,
    Relative,
}

impl From<isize> for ParamMode {
    fn from(input: isize) -> Self {
        match input {
            0 => ParamMode::Position,
            1 => ParamMode::Immediate,
            2 => ParamMode::Relative,
            _ => panic!("unknown param mode: {}", input),
        }
    }
}

fn extract_command(opcode: isize) -> (Instruction, [ParamMode; 3]) {
    let instruction = Instruction::from(opcode % 100);

    let mode1 = (opcode / 100) % 10;
    let mode2 = (opcode / 1000) % 10;
    let mode3 = (opcode / 10000) % 10;

    (instruction, [mode1.into(), mode2.into(), mode3.into()])
}
