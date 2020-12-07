use std::collections::HashMap;

#[derive(Debug)]
struct Rule {
    outer: String,
    inner: String,
    count: usize,
}

fn parse_rule(s: &str) -> Vec<Rule> {
    let mut iter = s.split(" bags contain ");
    let outer = iter.next().unwrap();
    iter.next()
        .unwrap()
        .split(", ")
        .map(|s| {
            let mut iter = s.trim_end_matches('.')
                .trim_end_matches('s')
                .trim_end_matches(" bag")
                .splitn(2, ' ');
            let num_str = iter.next().unwrap();
            let num = if num_str == "no" {
                0
            } else {
                num_str.parse::<usize>().unwrap()
            };
            (num, iter.next().unwrap().to_string())
        })
        .map(|(count, inner)| Rule {
            outer: outer.to_string(),
            inner: inner.to_string(),
            count: count,
        })
        .collect()
}

fn traverse(map: &HashMap<String, Vec<(String, usize)>>, s: &String) -> usize {
    let empty_vec: Vec<(String, usize)> = vec![];
    let v = match map.get(s) {
        Some(w) => w,
        None => &empty_vec,
    };
    let mut total: usize = 0;
    for entry in v {
        let (s, n) = &entry;
        total += n * traverse(map, &s);
    }
    total + 1
}

fn main() {
    let mut map = HashMap::<String, Vec<(String, usize)>>::new();
    let contents: String = std::fs::read_to_string("input.txt").expect("failed to read file");
    for r in contents.lines().map(|s| parse_rule(s)).flatten() {
        map.entry(r.outer).or_insert(vec![]).push((r.inner, r.count));
    }
    let shiny_gold = "shiny gold".to_string();
    println!("{}", traverse(&map, &shiny_gold) - 1);
}
