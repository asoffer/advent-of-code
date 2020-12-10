fn main() {
    let mut adapters: Vec<i64> = std::fs::read_to_string("input.txt")
        .expect("failed to read file")
        .lines()
        .map(|s| s.parse::<i64>().unwrap())
        .collect();
    adapters.push(0);
    adapters.sort();
    adapters.push(adapters.last().unwrap() + 3);
    let (ones, threes) = adapters
        .windows(2)
        .map(|w| w[1] - w[0])
        .map(|n| ((n == 1) as i64, (n == 3) as i64))
        .fold((0, 0), |(acc_ones, acc_threes), (is_one, is_three)| {
            (acc_ones + is_one, acc_threes + is_three)
        });
    println!("{}", ones * threes);
}
