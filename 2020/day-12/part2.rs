enum Instruction {
    North(usize),
    East(usize),
    South(usize),
    West(usize),
    Left(usize),
    Right(usize),
    Forward(usize),
}

struct Position {
    x: i64,
    y: i64,
}

impl std::str::FromStr for Instruction {
    type Err = scan_fmt::parse::ScanError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let (head, tail) = s.split_at(1);
        let c: char = head.bytes().next().unwrap() as char;
        let amount = tail.parse::<usize>().unwrap();

        match c {
            'N' => Ok(Instruction::North(amount)),
            'E' => Ok(Instruction::East(amount)),
            'S' => Ok(Instruction::South(amount)),
            'W' => Ok(Instruction::West(amount)),
            'L' => Ok(Instruction::Left(amount)),
            'R' => Ok(Instruction::Right(amount)),
            'F' => Ok(Instruction::Forward(amount)),
            _ => panic!(),
        }
    }
}

fn rotated(p: Position, amount: i64) -> Position {
    let mut q: Position = p;
    for _ in 0 .. amount / 90 {
        let x = -q.y;
        let y = q.x;
        q.x = x;
        q.y = y;
    }
    q
}

fn main() {
    let (ship, _) = std::fs::read_to_string("input.txt")
        .expect("failed to read file")
        .lines()
        .map(|line| line.parse::<Instruction>().unwrap())
        .fold(
            (Position{ x: 0, y: 0 }, Position{ x: 10, y: 1 }),
            |(ship, waypoint), inst| {
                match inst {
                Instruction::North(amount) => (ship, Position{ 
                    x: waypoint.x,
                    y: waypoint.y + amount as i64,
                }),
                Instruction::East(amount) => (ship, Position{ 
                    x: waypoint.x + amount as i64,
                    y: waypoint.y,
                }),
                Instruction::South(amount) => (ship, Position{ 
                    x: waypoint.x,
                    y: waypoint.y - amount as i64,
                }),
                Instruction::West(amount) => (ship, Position{ 
                    x: waypoint.x - amount as i64,
                    y: waypoint.y,
                }),
                Instruction::Left(amount) => (ship, rotated(waypoint, amount as i64)),
                Instruction::Right(amount) => (ship, rotated(waypoint, (360 - amount as i64))),
                Instruction::Forward(amount) => (Position{
                    x: ship.x + amount as i64 * waypoint.x,
                    y: ship.y + amount as i64 * waypoint.y,
                }, waypoint),
        }});
    println!("{}", ship.x.abs() + ship.y.abs())
}
