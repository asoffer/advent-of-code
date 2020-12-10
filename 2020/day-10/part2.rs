fn count_arrangements_ending_with(index: usize,
                                  adapters: &[i64],
                                  cache: &[usize]) -> usize {
    if index == 0 {
        return 1;
    }
    let mut i = index - 1;
    let mut total = 0;
    while adapters[index] - adapters[i] <= 3 {
        total += cache[i];
        if i == 0 {
            break;
        }
        i -= 1;
    }
    total
}

fn main() {
    let mut adapters: Vec<i64> = std::fs::read_to_string("input.txt")
        .expect("failed to read file")
        .lines()
        .map(|s| s.parse::<i64>().unwrap())
        .collect();
    adapters.push(0);
    adapters.sort();
    let mut cache = Vec::<usize>::new();
    for i in 0..adapters.len() {
        let count = count_arrangements_ending_with(i, &adapters, &cache);
        cache.push(count);
    }
    println!("{}",cache.last().unwrap());
}
