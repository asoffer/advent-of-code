#[macro_use] extern crate scan_fmt;

use std::collections::HashSet;

#[derive(Debug)]
enum Instruction {
    Nop,
    Acc(i32),
    Jmp(i32),
}

#[derive(Debug)]
struct ExecutionContext {
    current: *const Instruction,
    acc: i32,
}

impl ExecutionContext {
    fn step(&mut self) -> () {
        match unsafe { self.current.read() } {
            Instruction::Nop =>
                self.current = unsafe { self.current.offset(1) },
            Instruction::Acc(n) => {
                self.acc += n;
                self.current = unsafe { self.current.offset(1) };
            },
            Instruction::Jmp(n) => {
                self.current = unsafe { self.current.offset(n as isize) }
            },
        }
    }
}

fn main() {
    let contents: String = std::fs::read_to_string("input.txt").expect("failed to read file");
    let instructions: Vec<Instruction> = contents
        .lines()
        .map(|l| {
            let (s, n) = scan_fmt!(l, "{} {}", String, i32).unwrap();
            match s.as_str() {
                "acc" => Instruction::Acc(n),
                "jmp" => Instruction::Jmp(n),
                "nop" => Instruction::Nop,
                _     => panic!(s),
            }
        }).collect();
    let mut ctx = ExecutionContext{
        current: &instructions[0],
        acc: 0,
    };
    let mut seen = HashSet::<*const Instruction>::new();
    loop {
        if !seen.insert(ctx.current) {
            break;
        }
        ctx.step();
        println!("{}", ctx.acc);
    }
}
