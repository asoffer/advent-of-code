fn validate_entry(s: &str) -> bool {
    let mut keys: Vec<&str> = s
        .split_whitespace()
        .map(|s| s.split(':').next().unwrap())
        .filter(|&s| s != "cid")
        .collect();
   keys.sort_unstable();
   keys == vec!["byr", "ecl", "eyr", "hcl", "hgt", "iyr", "pid"]
}

fn main() {
    let contents: String = std::fs::read_to_string("input.txt").expect("failed to read file");
    let num_valid = contents.split("\n\n").filter(|s| validate_entry(s)).count();
    println!("{}", num_valid);
}
