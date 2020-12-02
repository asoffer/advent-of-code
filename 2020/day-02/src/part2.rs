use lazy_static::lazy_static;
use regex::Regex;

fn validate_line(s: &str) -> bool {
    lazy_static! {
      static ref RE: Regex = Regex::new(
          r"^(?P<first>[0-9]+)-(?P<second>[0-9]+) (?P<char>[a-z]): (?P<password>.*)").unwrap();
    }

    return match RE.captures(s) {
      None => false,
      Some(cap) => {
        if cap.name("first").is_some() && cap.name("second").is_some()
           && cap.name("char").is_some() && cap.name("password").is_some() {
          let first = cap.name("first").unwrap().as_str().parse::<usize>().unwrap();
          let second = cap.name("second").unwrap().as_str().parse::<usize>().unwrap();
          let c = cap.name("char").unwrap().as_str().parse::<char>().unwrap();
          let password = cap.name("password").unwrap().as_str();

          (password.chars().nth(first - 1) == Some(c))
              ^ (password.chars().nth(second - 1) == Some(c))
        } else {
          false
        }
      }
    }
}

fn main() {
    let contents: String = std::fs::read_to_string("input.txt")
            .expect("failed to read file");
    let num_valid_passwords = contents
            .split("\n")
            .filter(|s| validate_line(s))
            .count();
    print!("{}", num_valid_passwords);
}
