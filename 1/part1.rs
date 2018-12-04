fn main() {
  let contents = std::fs::read_to_string("input.txt")
    .expect("failed to read file");
  let total = contents
    .split("\n")
    .map(|s| s.parse::<i32>().unwrap_or(0))
    .fold(0, |acc, x| acc + x);
  print!("{}", total);
}
