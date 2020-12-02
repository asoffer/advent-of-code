use lazy_static::lazy_static;
use regex::Regex;

fn validate_line(s: &str) -> bool {
    lazy_static! {
      static ref RE: Regex = Regex::new(
          r"^(?P<low>[0-9]+)-(?P<high>[0-9]+) (?P<char>[a-z]): (?P<password>.*)").unwrap();
    }

    return match RE.captures(s) {
      None => false,
      Some(cap) => {
        if cap.name("low").is_some() && cap.name("high").is_some()
           && cap.name("char").is_some() && cap.name("password").is_some() {
          let low = cap.name("low").unwrap().as_str().parse::<i32>().unwrap();
          let high = cap.name("high").unwrap().as_str().parse::<i32>().unwrap();
          let c = cap.name("char").unwrap().as_str().parse::<char>().unwrap();
          let password = cap.name("password").unwrap().as_str();

          let num_cs = password.chars().filter(|ch| c == *ch).count() as i32;

          low <= num_cs && num_cs <= high
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
