#[macro_use] extern crate scan_fmt;

use std::collections::HashMap;

fn main() {
    let mut memory = HashMap::<u64, u64>::new();
    let mut zero_mask: u64 = 0;
    let mut one_mask: u64 = 0;

    std::fs::read_to_string("input.txt")
        .expect("failed to read file")
        .lines()
        .for_each(|s| match s.as_bytes()[1] as char {
            'a' => {
                zero_mask = 0;
                one_mask = 0;
                let mask_str = match scan_fmt!(s, "mask = {}", String) {
                    Ok(m) => m,
                    _ => panic!(),
                };
                for c in mask_str.chars() {
                    zero_mask *= 2;
                    one_mask *= 2;
                    match c {
                        '0' => {},
                        '1' => {
                            one_mask += 1;
                            zero_mask += 1;
                        }
                        'X' => {
                            zero_mask += 1;
                        }
                        _ => panic!(),
                    };
                }
            },
            'e' => {
                let (addr, val) = match scan_fmt!(s, "mem[{}] = {}", u64, u64) {
                    Ok((a, v)) => (a, v),
                    _ => panic!(),
                };

                *memory.entry(addr).or_insert(0) = (val | one_mask) & zero_mask;
            },
            _ => panic!(),
        });

    let mut total = 0;
    for (_, v) in memory {
        total += v;
    }

    println!("{}", total);
}
