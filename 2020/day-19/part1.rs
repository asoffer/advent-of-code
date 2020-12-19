#[macro_use]
extern crate scan_fmt;

use std::collections::HashMap;

enum Rule {
    Pattern(Vec<Vec<usize>>),
    Char(char),
}

fn consume_sequence<'a>(
    rule_map: &HashMap<usize, Rule>,
    seq: &[usize],
    message: &'a str,
) -> Option<&'a str> {
    let mut m: &'a str = message;
    for n in seq {
        if let Some(s) = consume_match(rule_map, *n, m) {
            m = s;
        } else {
            return None;
        }
    }
    return Some(m);
}

fn consume_match<'a>(
    rule_map: &HashMap<usize, Rule>,
    rule_num: usize,
    message: &'a str,
) -> Option<&'a str> {
    match rule_map.get(&rule_num).unwrap() {
        Rule::Pattern(p) => {
            let mut result: Option<&'a str> = None;
            for s in p {
                if let Some(m) = consume_sequence(rule_map, &s[..], message) {
                    result = Some(m);
                    break;
                }
            }
            result
        }
        Rule::Char(c) => if message.chars().next() == Some(*c) {
            Some(&message[1..])
        } else {
            None
        }
    }
}

fn match_rule(rule_map: &HashMap<usize, Rule>, message: &str) -> bool {
    match consume_match(rule_map, 0, message) {
        None => false,
        Some(s) => s.is_empty(),
    }
}

fn main() {
    let contents = std::fs::read_to_string("input1.txt").expect("failed to read file");
    let mut iter = contents.split("\n\n");
    let rules = iter.next().unwrap();
    let messages = iter.next().unwrap();

    let mut rule_map = HashMap::<usize, Rule>::new();
    for line in rules.lines() {
        if let Ok((num, c)) = scan_fmt!(line, "{}: \"{}\"", usize, char) {
            rule_map.insert(num, Rule::Char(c));
        } else {
            let mut parts = line.split(": ");
            let num = parts.next().unwrap();
            rule_map.insert(num.parse().unwrap(),
            Rule::Pattern(parts.next().unwrap().split(" | ").map(|seq| {
                seq.split(' ').map(|s| s.parse::<usize>().unwrap()).collect::<Vec<usize>>()
            }).collect()));
        }
    }

    println!(
        "{}",
        messages
            .lines()
            .filter(|message| match_rule(&rule_map, message))
            .count()
    );
}
