#[macro_use]
extern crate scan_fmt;

use std::collections::HashMap;

fn valid_for_rule(n: usize, valid_ranges: &Vec<std::ops::RangeInclusive<usize>>) -> bool {
    valid_ranges.iter().any(|range| range.contains(&n))
}

fn log2(num: usize) -> usize {
    let mut n = num;
    let mut result = 0;
    while n > 0 {
        n /= 2;
        result += 1;
    }
    return result - 1;
}

fn parse_ticket(s: &str) -> (String, Vec<std::ops::RangeInclusive<usize>>) {
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
    (name, v)
}

fn main() {
    let contents: String = std::fs::read_to_string("input.txt").expect("failed to read file");
    let mut iter = contents.split("\n\n");
    let unparsed_rules = iter.next().unwrap();
    let my_ticket = iter.next().unwrap();
    let other_tickets = iter.next().unwrap();

    let mut rules = HashMap::<String, Vec<std::ops::RangeInclusive<usize>>>::new();
    unparsed_rules.lines().for_each(|s| {
        let (name, ranges) = parse_ticket(s);
        rules.insert(name, ranges);
    });

    let mut other_tickets_iter = other_tickets.lines();
    other_tickets_iter.next(); // Ignore header
    let tickets: Vec<Vec<usize>> = other_tickets_iter
        .filter_map(|s| {
            let row: Vec<usize> = s
                .split(',')
                .map(|entry| entry.parse::<usize>().unwrap())
                .collect();
            for n in &row {
                if !rules
                    .iter()
                    .any(|(_, valid_ranges)| valid_for_rule(*n, &valid_ranges))
                {
                    return None;
                }
            }
            Some(row)
        })
        .collect();

    let mut bitset_map = HashMap::<String, usize>::new();
    for (name, _rule) in &rules {
        bitset_map.insert(name.clone(), (1 << rules.len()) - 1);
    }

    for i in 0..rules.len() {
        for ticket in &tickets {
            for (name, rule) in &rules {
                if !valid_for_rule(ticket[i], &rule) {
                    *bitset_map.get_mut(name).unwrap() &= !(1 << i);
                }
            }
        }
    }

    let mut my_ticket_iter = my_ticket.lines();
    my_ticket_iter.next().unwrap(); // Ignore header
    let ticket: Vec<usize> = my_ticket_iter
        .next()
        .unwrap()
        .split(',')
        .map(|entry| entry.parse::<usize>().unwrap())
        .collect();

    let mut entries: Vec<(&String, &usize)> = bitset_map.iter().collect();
    entries.sort_by(|(_, lhs), (_, rhs)| lhs.cmp(rhs));
    let mut accumulated_value = 0;
    let mut total = 1;
    for (name, value) in entries {
        if name.starts_with("departure") {
            total *= ticket[log2(value - accumulated_value)];
        }
        accumulated_value |= value;
    }
    println!("{}", total);
}
