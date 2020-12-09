#[macro_use]
extern crate scan_fmt;

fn validate(w: &[i64]) -> Option<i64> {
    let (last, preamble) = w.split_last().unwrap();
    for i in preamble {
        for j in preamble {
            if i == j { continue; }
            if i + j == *last {
                return None;
            }
        }
    }
    Some(*last)
}

fn main() {
    let contents: Vec<i64> = std::fs::read_to_string("input.txt")
        .expect("failed to read file")
        .lines()
        .map(|s| scan_fmt!(s, "{}", i64).unwrap())
        .collect();

    let num = contents.windows(26).filter_map(|w| validate(w)).nth(0).unwrap();
    for i in 0 .. contents.len() - 25 {
        for j in i + 1 .. i + 25 {
            let sum = contents[i..j].iter().sum();
            if num == sum {
                let min = contents[i..j].iter().min().unwrap();
                let max = contents[i..j].iter().max().unwrap();
                println!("{}", min + max);
            }
        }
    }
}
