use adjacent_pair_iterator::AdjacentPairIterator;

fn parse_seat(s: &str) -> usize {
    s.bytes().map(|c| match c as char {
        'F' => 0,
        'B' => 1,
        'L' => 0,
        'R' => 1,
        _   => panic!()
    }).fold(0, |acc, x| acc * 2 + x)
}

fn main() {
    let contents: String = std::fs::read_to_string("input.txt").expect("failed to read file");
    let mut seats: Vec<usize> = contents.lines().map(|s| parse_seat(s)).collect();
    seats.sort();
    seats.iter().adjacent_pairs().filter(|(&a, &b)| a + 2 == b).for_each(|(&a, _)| println!("{}", a + 1));
}
