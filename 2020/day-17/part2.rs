use std::collections::HashSet;
use std::collections::HashMap;
use std::collections::hash_map::Entry;


fn step(prev: &HashSet<(i32, i32, i32, i32)>) -> HashSet<(i32, i32, i32, i32)> {
    let mut neighbors = HashMap::<(i32, i32, i32, i32), usize>::new();
    for (w, x, y, z) in prev.iter() {
        for ww in (w-1)..=(w+1) {
            for xx in (x-1)..=(x+1) {
                for yy in (y-1)..=(y+1) {
                    for zz in (z-1)..=(z+1) {
                        if ww == *w && xx == *x && yy == *y && zz == *z { continue; }
                        *neighbors.entry((ww, xx, yy, zz)).or_insert(0) += 1;
                    }
                }
            }
        }
    }

    let mut next = HashSet::<(i32, i32, i32, i32)>::new();
    for (cell, count) in neighbors.iter() {
        if *count == 3 {
            next.insert(*cell);
        }
    }
    for cell in prev {
        if match neighbors.entry(*cell) {
            Entry::Occupied(entry) => *entry.get() == 2,
            _ => false,
        } {
            next.insert(*cell);
        }
    }

    next
}

fn main() {
    let mut active_cells = HashSet::<(i32, i32, i32, i32)>::new();
    let contents: String = std::fs::read_to_string("input.txt").expect("failed to read file");
    for (w, line) in contents.lines().enumerate() {
        for (x, c) in line.chars().enumerate() {
            if c == '#' {
                active_cells.insert((w as i32, x as i32, 0, 0));
            }
        }
    }
    for _ in 0..6 {
        active_cells = step(&active_cells);
    }

    println!("{}", active_cells.len());
}
