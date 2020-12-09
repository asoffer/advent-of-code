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

    println!("{}", contents.windows(26).filter_map(|w| validate(w)).nth(0).unwrap());
}
