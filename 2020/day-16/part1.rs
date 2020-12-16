#[macro_use]
extern crate scan_fmt;

use std::collections::HashMap;

fn main() {
    let contents: String = std::fs::read_to_string("input.txt").expect("failed to read file");
    let mut iter = contents.split("\n\n");
    let unparsed_rules = iter.next().unwrap();
    let _my_ticket = iter.next().unwrap();
    let other_tickets = iter.next().unwrap();

    let mut rules = HashMap::<String, Vec<std::ops::RangeInclusive<usize>>>::new();
    unparsed_rules.lines().for_each(|s| {
        let (name, low1, high1, low2, high2) = scan_fmt!(
            s,
            "{[a-zA-Z ]}: {}-{} or {}-{}",
            String,
            usize,
            usize,
            usize,
            usize
        )
        .unwrap();
        let mut v = Vec::<std::ops::RangeInclusive<usize>>::new();
        v.push(low1..=high1);
        v.push(low2..=high2);
        rules.insert(name, v);
    });

    let mut total_invalid: usize = 0;
    let mut other_tickets_iter = other_tickets.lines();
    other_tickets_iter.next(); // Ignore header
    other_tickets_iter.for_each(|s| {
        s.split(',')
            .map(|entry| entry.parse::<usize>().unwrap())
            .for_each(|n| {
                for (_name, valid_ranges) in rules.iter() {
                    for range in valid_ranges {
                        if range.contains(&n) {
                            return;
                        }
                    }
                }
                total_invalid += n;
            });
    });
    println!("{}", total_invalid);
}
