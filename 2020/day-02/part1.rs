struct PasswordLine {
    low: usize,
    high: usize,
    character: char,
    password: String,
}

impl std::str::FromStr for PasswordLine {
    type Err = text_io::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut iter = s.bytes();
        let low: usize;
        let high: usize;
        let character: char;
        let password: String;
        text_io::try_scan!(iter => "{}-{} {}: {}", low, high, character, password);

        return Ok(PasswordLine {
            low: low,
            high: high,
            character: character,
            password: password,
        });
    }
}

fn validate_line(s: &str) -> bool {
    let pw = match s.parse::<PasswordLine>() {
        Ok(pw) => pw,
        Err(_) => return false,
    };
    let char_count = pw.password.chars().filter(|&c| c == pw.character).count();

    pw.low <= char_count && char_count <= pw.high
}

fn main() {
    let contents: String = std::fs::read_to_string("input.txt").expect("failed to read file");
    let num_valid_passwords = contents.split("\n").filter(|s| validate_line(s)).count();
    print!("{}", num_valid_passwords);
}
