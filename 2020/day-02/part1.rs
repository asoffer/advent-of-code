// %d-%d %c: %s
fn validate_line(s: String) -> Option<bool> {
    let parts = s.split(":");
    let policy_str = parts.next();
    let password = parts.next();
    return Some(true);
}

fn main() {
    let contents: String = std::fs::read_to_string("input.txt")
            .expect("failed to read file");
    let num_valid_passwords = contents
            .split("\n")
            .filter_map(validate_line)
            .count();
}
