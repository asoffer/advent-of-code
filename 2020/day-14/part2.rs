#[macro_use] extern crate scan_fmt;

use std::collections::HashMap;

fn all_masks(mask: &str) -> Vec<(u64, u64)> {
    let mut v = Vec::<(u64, u64)>::new();
    v.push((0, 0));
    for c in mask.chars() {
        match c {
            '0' => {
                v = v.iter().map(|&(z, o)| (z * 2 + 1, o * 2)).collect();
            },
            '1' => {
                v = v.iter().map(|&(z, o)| (z * 2 + 1, o * 2 + 1)).collect();
            },
            'X' => {
                let mut doubled = Vec::<(u64, u64)>::new();
                for (z, o) in v {
                    doubled.push((z * 2, o * 2));
                    doubled.push((z * 2 + 1, o * 2 + 1));
                }
                v = doubled;
            },
            _ => panic!(),
        }
    }
    v
}


fn main() {
    let input = std::fs::read_to_string("input.txt")
        .expect("failed to read file");
    let lines: Vec<&str> = input.lines().collect();

    let mut memory = HashMap::<u64, u64>::new();
    let mut addr_masks = Vec::<(u64, u64)>::new();

    for line in lines {
        match line.as_bytes()[1] as char {
            'a' => {
                addr_masks = match scan_fmt!(line, "mask = {}", String) {
                    Ok(m) => all_masks(m.as_str()),
                    _ => panic!(),
                };
            },
            'e' => {
                let (addr, val) = match scan_fmt!(line, "mem[{}] = {}", u64, u64) {
                    Ok((a, v)) => (a, v),
                    _ => panic!(),
                };

                for (zero_mask, one_mask)in &addr_masks {
                    *memory.entry((addr | one_mask) & zero_mask).or_insert(0) = val;
                }

            },
            _ => panic!(),
        }
    }
    let mut total = 0;
    for (_, v) in memory {
        total += v;
    }

    println!("{}", total);
}
