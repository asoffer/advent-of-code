fn tree_count(down: usize, right: usize, see_tree_map: &[&str]) -> usize {
    let mut count = 0;
    for (index, line) in see_tree_map.iter().step_by(down).enumerate() {
        // index will be the number of rows processed so far (rather than the
        // row number we are currently on, which differ if the step_by is not 1).
        if line.chars().nth(right * index % line.len()).unwrap() == '#' {
            count += 1;
        }
    }

    return count;
}

fn main() {
    let contents: String = std::fs::read_to_string("input.txt")
            .expect("failed to read file");
    let see_tree_map: Vec<&str> = contents
            .lines()
            .collect();
    let product: usize = vec![(1, 1), (1, 3), (1, 5), (1, 7), (2, 1)]
        .iter()
        .map(|&(down, right)| tree_count(down, right, &see_tree_map))
        .product();
    println!("{}", product);
}
