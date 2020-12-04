#[macro_use] extern crate scan_fmt;

#[test]
fn test_birth_year() {
    assert!(valid_birth_year("byr:1920"));
    assert!(valid_birth_year("byr:1921"));
    assert!(valid_birth_year("byr:2002"));
    assert!(!valid_birth_year("byr:2003"));
    assert!(!valid_birth_year("byr:1903"));
    assert!(!valid_birth_year("byx:2000"));
}

fn valid_birth_year(s: &str) -> bool {
  // byr (Birth Year) - four digits; at least 1920 and at most 2002.
  if let Ok(birth_year) = scan_fmt!(s, "byr:{}", i32) {
    if birth_year < 1920 || birth_year > 2002 { return false; }
  } else {
    return false;
  }
  true
}

#[test]
fn test_eye_color() {
    assert!(valid_eye_color("ecl:amb"));
    assert!(valid_eye_color("ecl:grn"));
    assert!(valid_eye_color("ecl:hzl"));
    assert!(!valid_eye_color("ecl:GRN"));
    assert!(!valid_eye_color("ecl:ame"));
    assert!(!valid_eye_color("ecx:amb"));
}

fn valid_eye_color(s: &str) -> bool {
  // ecl (Eye Color) - exactly one of: amb blu brn gry grn hzl oth.
  if let Ok(eye_color) = scan_fmt!(s, "ecl:{}", String) {
      if eye_color != "amb"
      && eye_color != "blu"
      && eye_color != "brn"
      && eye_color != "gry"
      && eye_color != "grn"
      && eye_color != "hzl"
      && eye_color != "oth" { return false; }
  } else {
    return false;
  }
  true
}

#[test]
fn test_expiration_year() {
    assert!(valid_expiration_year("eyr:2020"));
    assert!(valid_expiration_year("eyr:2021"));
    assert!(valid_expiration_year("eyr:2029"));
    assert!(valid_expiration_year("eyr:2030"));
    assert!(!valid_expiration_year("eyr:2019"));
    assert!(!valid_expiration_year("eyr:2031"));
    assert!(!valid_expiration_year("eyx:2025"));
}

fn valid_expiration_year(s: &str) -> bool {
  // eyr (Expiration Year) - four digits; at least 2020 and at most 2030.
  if let Ok(expiration_year) = scan_fmt!(s, "eyr:{}", i32) {
    if expiration_year < 2020 || expiration_year > 2030 { return false; }
  } else {
    return false;
  }
  true
}

#[test]
fn test_hair_color() {
    assert!(valid_hair_color("hcl:#123456"));
    assert!(valid_hair_color("hcl:#ababab"));
    assert!(valid_hair_color("hcl:#ab1234"));
    assert!(!valid_hair_color("hcl:123456"));
    assert!(!valid_hair_color("hcl:#1234"));
    assert!(!valid_hair_color("hcx:#123456"));
    assert!(!valid_hair_color("hcl:#1234567"));
}

fn valid_hair_color(s: &str) -> bool {
  // hcl (Hair Color) - a # followed by exactly six characters 0-9 or a-f.
  if let Ok(hair_color) = scan_fmt!(s, "hcl:#{x}", String) {
    if hair_color.len() != 6 { return false; }
  } else {
    return false;
  }
  true
}

#[test]
fn test_height() {
    assert!(valid_height("hgt:150cm"));
    assert!(valid_height("hgt:193cm"));
    assert!(!valid_height("hgt:cm"));
    assert!(valid_height("hgt:59in"));
    assert!(valid_height("hgt:76in"));
    assert!(valid_height("hgt:76in"));
    assert!(!valid_height("hgt:in"));
    assert!(!valid_height("hgx:180cm"));
}

fn valid_height(s: &str) -> bool {
  // hgt (Height) - a number followed by either cm or in:
  // If cm, the number must be at least 150 and at most 193.
  // If in, the number must be at least 59 and at most 76.
  if let Ok((height, unit)) = scan_fmt!(s, "hgt:{d}{}", i32, String) {
    if unit == "cm" {
      if !(150 <= height && height <= 193) { return false; }
    } else if unit == "in" {
      if !(59 <= height && height <= 76) { return false; }
    } else {
      return false;
    }
  } else {
    return false;
  }
  true
}

#[test]
fn test_issue_year() {
    assert!(valid_issue_year("iyr:2010"));
    assert!(valid_issue_year("iyr:2020"));
    assert!(!valid_issue_year("iyr:2021"));
    assert!(!valid_issue_year("iyx:2021"));
}

fn valid_issue_year(s: &str) -> bool {
  // iyr (Issue Year) - four digits; at least 2010 and at most 2020.
  if let Ok(iyr) = scan_fmt!(s, "iyr:{}", i32) {
    if iyr < 2010 || iyr > 2020 { return false; }
  } else {
    return false;
  }
  true
}

#[test]
fn test_passport_id() {
    assert!(valid_passport_id("pid:000002010"));
    assert!(valid_passport_id("pid:999999999"));
    assert!(!valid_passport_id("pid:99999999"));
    assert!(!valid_passport_id("pid:9999999999"));
    assert!(!valid_passport_id("pid:2021"));
    assert!(!valid_passport_id("pix:99999999"));
}

fn valid_passport_id(s: &str) -> bool {
  // pid (Passport ID) - a nine-digit number, including leading zeroes.
  if let Ok(pid) = scan_fmt!(s, "pid:{d}", String) {
    if pid.len() != 9 { return false; }
  } else {
    return false;
  }
  true
}

fn validate_entry(s: &str) -> bool {
  let mut keys: Vec<&str> = s
       .split_whitespace()
       .filter(|&s| !s.starts_with("cid:"))
       .collect();
  keys.sort_unstable();
  if keys.len() != 7 { return false; }

  valid_birth_year(keys[0]) && valid_eye_color(keys[1]) && valid_expiration_year(keys[2])
      && valid_hair_color(keys[3]) && valid_height(keys[4]) && valid_issue_year(keys[5])
      && valid_passport_id(keys[6])
}

fn main() {
    let contents: String = std::fs::read_to_string("input.txt").expect("failed to read file");
    let num_valid = contents.split("\n\n").filter(|s| validate_entry(s)).count();
    println!("{}", num_valid);
}
