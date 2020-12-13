fn main() {
    let input = std::fs::read_to_string("input.txt")
        .expect("failed to read file");
    let v: Vec<&str> = input.lines().collect();
    let timestamp = v[0].parse::<u64>().unwrap();
    let (product, _) = v[1].split(',').filter_map(|s| match s.parse::<u64>() {
        Ok(n) => Some(n),
        _     => None,
    }).map(|bus_num| (bus_num * (bus_num - timestamp % bus_num), bus_num - timestamp % bus_num))
    .fold((0, timestamp), |(product, min_wait), (prod, wait)| {
        if min_wait < wait {
            (product, min_wait)
        } else {
            (prod, wait)
        }
    });
    println!("{}", product);
}
