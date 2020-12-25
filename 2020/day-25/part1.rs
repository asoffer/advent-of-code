fn pow_mod(a: usize, b: usize, m: usize) -> usize {
    let mut n = 1;
    for _ in 0..b {
        n *= a;
        n %= m;
    }
    n
}

fn eavesdrop(key1: usize, key2: usize) -> usize {
    let mut i = 0;
    let mut n = 1;
    loop {
        n *= 7;
        n %= 20201227;
        i += 1;
        if n == key1 {
            return pow_mod(key2, i, 20201227);
        } else if n == key1 {
            return pow_mod(key1, i, 20201227);
        }
    }
}

fn main() {
    let contents = std::fs::read_to_string("input.txt").expect("failed to read file");
    let keys: Vec<usize> = contents.lines().map(|s| s.parse().unwrap()).collect();
    println!("{}", eavesdrop(keys[0], keys[1]));
}
