#[test]
fn test_bits() {
    assert_eq!(bits("abc"), 7);
    assert_eq!(bits("ac"), 5);
    assert_eq!(bits(""), 0);
    assert_eq!(bits("bbb"), 2);
    assert_eq!(bits("bbb\n"), 2);
}

fn bits(s: &str) -> i32 {
    let mut bitset: i32 = 0;
    for c in s.bytes() {
        if c == '\n' as u8 { return bitset; }
        bitset |= 1 << (c - 'a' as u8);
    }
    bitset
}

#[test]
fn test_popcount() {
    assert_eq!(popcount(0), 0);
    assert_eq!(popcount(1), 1);
    assert_eq!(popcount(2), 1);
    assert_eq!(popcount(3), 2);
    assert_eq!(popcount(4), 1);
}

fn popcount(mut bitset: i32) -> usize {
    let mut count: usize = 0;
    while bitset > 0 {
        if bitset % 2 != 0 { count += 1; }
        bitset /= 2;
    }
    count
}

#[test]
fn test_merged_bitsets() {
    assert_eq!(merged_bitsets(""), 0x3ffffff);
    assert_eq!(merged_bitsets("a"), 1);
    assert_eq!(merged_bitsets("ab\na"), 1);
    assert_eq!(merged_bitsets("ab\nb"), 2);
    assert_eq!(merged_bitsets("ab\nbc\nac"), 0);
    assert_eq!(merged_bitsets("dab\nbdc\nacd"), 8);
}

fn merged_bitsets(s: &str) -> i32 {
    s.lines().map(|l| bits(l)).fold(0x3ffffff, |acc, x| acc & x)
}

fn main() {
    let contents: String = std::fs::read_to_string("input.txt").expect("failed to read file");
    println!("{}", contents.split("\n\n")
                           .map(|s| merged_bitsets(s))
                           .map(|n| popcount(n))
                           .fold(0, |acc, x| acc + x));
}
