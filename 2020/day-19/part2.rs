#[macro_use]
extern crate scan_fmt;

use std::collections::HashMap;

enum Rule {
    Pattern(Vec<Vec<usize>>),
    Char(char),
}

fn consume_match<'a>(
    rule_map: &HashMap<usize, Rule>,
    rule_num: usize,
    message: &'a str,
    follow: &mut Vec<usize>,
) -> bool {
    match rule_map.get(&rule_num).unwrap() {
        Rule::Pattern(p) => {
            for seq in p {
                let mut new_follow = follow.clone();
                let mut entries: Vec<usize> = seq[1..].to_vec();
                entries.reverse();
                for s in &entries {
                    new_follow.push(*s);
                }
                if consume_match(rule_map, seq[0], message, &mut new_follow) {
                    return true;
                }
            }
            return false;
        }
        Rule::Char(c) => if message.chars().next() == Some(*c) {
            if follow.is_empty() {
                message.len() == 1
            } else {
                let f = follow.pop().unwrap();
                let result = consume_match(rule_map, f, &message[1..], follow);
                follow.push(f);
                result
            }
        } else {
            false
        }
    }
}

fn match_rule(rule_map: &HashMap<usize, Rule>, message: &str) -> bool {
    let mut follow = Vec::<usize>::new();
    consume_match(rule_map, 0, message, &mut follow)
}

fn main() {
    let contents = std::fs::read_to_string("input2.txt").expect("failed to read file");
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
