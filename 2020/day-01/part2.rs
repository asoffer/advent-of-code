fn main() {
    let contents: String = std::fs::read_to_string("input.txt")
            .expect("failed to read file");
    let values: Vec<i32> = contents
            .split("\n")
            .map(|s| s.parse::<i32>().unwrap_or(0))
            .filter(|n| n.is_positive())
            .collect();
    for i in 0 .. values.len() {
      for j in i+1 .. values.len() {
        for k in j+1 .. values.len() {
          if values[i] + values[j] + values[k] == 2020 {
            print!("{}\n", values[i] * values[j] * values[k]);
          }
        }
      }
    }
}
