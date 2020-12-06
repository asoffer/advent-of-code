fn count_chars(s: &str) -> usize {
    let mut bitset: i32 = 0;
    for c in s.bytes() {
        if c == '\n' as u8 { continue; }
        bitset |= 1 << (c - 'a' as u8);
    }
    let mut count: usize  = 0;
    while bitset > 0 {
        if bitset % 2 != 0 { count += 1; }
        bitset /= 2;
    }
    count
}

fn main() {
    let contents: String = std::fs::read_to_string("input.txt").expect("failed to read file");
    println!("{}", contents.split("\n\n").map(|s| count_chars(s)).fold(0, |acc, x| acc + x));
}
