use std::collections::HashMap;
use std::collections::HashSet;
use std::collections::VecDeque;

#[derive(Debug)]
struct Rule<'a> {
    outer: &'a str,
    inner: &'a str,
}

fn parse_rule(s: &str) -> Vec<Rule> {
    let mut iter = s.split(" bags contain ");
    let outer = iter.next().unwrap();
    iter.next()
        .unwrap()
        .split(", ")
        .map(|s| {
            s.trim_end_matches('.')
                .trim_end_matches('s')
                .trim_end_matches(" bag")
                .trim_start_matches(char::is_numeric)
                .trim_start_matches(' ')
        })
        .map(|i| Rule {
            outer: outer,
            inner: i,
        })
        .collect()
}

fn traverse<'a>(map: &HashMap<&'a str, Vec<&'a str>>, q: &mut VecDeque<&'a str>) -> usize {
    let mut seen = HashSet::<&'a str>::new();
    let empty_vec: Vec<&'a str> = vec![];
    while !q.is_empty() {
        let s = q.pop_front().unwrap();
        seen.insert(s);
        let v = match map.get(s) {
            Some(v) => v,
            None => &empty_vec,
        };
        for entry in v {
            q.push_back(entry);
        }
    }
    seen.len() - 1 // minus 1 because it can't be just the bag on its own
}

fn main() {
    let mut map = HashMap::<&str, Vec<&str>>::new();
    let contents: String = std::fs::read_to_string("input.txt").expect("failed to read file");
    for r in contents.lines().map(|s| parse_rule(s)).flatten() {
        map.entry(r.inner).or_insert(vec![]).push(r.outer);
    }
    let mut q = VecDeque::<&str>::new();
    q.push_back("shiny gold");
    println!("{}", traverse(&map, &mut q));
}
