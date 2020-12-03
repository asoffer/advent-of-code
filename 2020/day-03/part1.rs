fn main() {
    let contents: String = std::fs::read_to_string("input.txt")
            .expect("failed to read file");
    let see_tree_map: Vec<&str> = contents
            .lines()
            .collect();
    let mut tree_count = 0;
    for (index, line) in see_tree_map.iter().enumerate() {
        if line.chars().nth(3 * index % line.len()).unwrap() == '#' {
            tree_count += 1;
        }
    }
    println!("{}", tree_count);
}
