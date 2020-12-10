fn main() {
    let mut adapters: Vec<i64> = std::fs::read_to_string("input.txt")
        .expect("failed to read file")
        .lines()
        .map(|s| s.parse::<i64>().unwrap())
        .collect();
    adapters.push(0);
    adapters.sort();
    adapters.push(adapters.last().unwrap() + 3);
    let diffs = adapters
        .windows(2)
        .map(|w| w[1] - w[0])
        .collect::<Vec<i64>>();
    println!(
        "{}",
        diffs.iter().filter(|&n| *n == 1).count() * diffs.iter().filter(|&n| *n == 3).count()
    );
}
