#[macro_use] extern crate scan_fmt;

use std::collections::HashSet;

#[derive(Debug, Clone, Copy)]
enum Instruction {
    Nop(i32),
    Acc(i32),
    Jmp(i32),
    Halt,
}

#[derive(Debug)]
struct ExecutionContext {
    current: *const Instruction,
    acc: i32,
}

impl ExecutionContext {
    fn step(&mut self) -> bool {
        match unsafe { self.current.read() } {
            Instruction::Nop(_) => {
                self.current = unsafe { self.current.offset(1) };
                true
            },
            Instruction::Acc(n) => {
                self.acc += n;
                self.current = unsafe { self.current.offset(1) };
                true
            },
            Instruction::Jmp(n) => {
                self.current = unsafe { self.current.offset(n as isize) };
                true
            },
            Instruction::Halt => false,
        }
    }
}

fn halts(instructions: &[Instruction]) -> (i32, bool) {
    let mut ctx = ExecutionContext{
        current: &instructions[0],
        acc: 0,
    };

    let mut seen = HashSet::<*const Instruction>::new();
    loop {
        if !seen.insert(ctx.current) {
            return (ctx.acc, false);
        }
        if !ctx.step() {
            return (ctx.acc, true);
        }
    }
}

fn main() {
    let contents: String = std::fs::read_to_string("input.txt").expect("failed to read file");
    let mut instructions: Vec<Instruction> = contents
        .lines()
        .map(|l| {
            let (s, n) = scan_fmt!(l, "{} {}", String, i32).unwrap();
            match s.as_str() {
                "acc" => Instruction::Acc(n),
                "jmp" => Instruction::Jmp(n),
                "nop" => Instruction::Nop(n),
                _     => panic!(s),
            }
        }).collect();
    instructions.push(Instruction::Halt);

    for i in 0 .. instructions.len() {
        let replacement = match &instructions[i] {
            Instruction::Jmp(n) => Instruction::Nop(*n),
            Instruction::Nop(n) => Instruction::Jmp(*n),
            _                   => continue,
        };
        let old_instruction = instructions[i];
        instructions[i] = replacement;
        let (acc, halted) = halts(instructions.as_slice());
        if halted {
            println!("{}", acc);
            break;
        } else {
            instructions[i] = old_instruction;
            continue;
        }
    }
}
