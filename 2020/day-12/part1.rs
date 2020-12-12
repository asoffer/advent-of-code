enum Instruction {
    North(usize),
    East(usize),
    South(usize),
    West(usize),
    Left(usize),
    Right(usize),
    Forward(usize),
}

struct PositionAndOrientation {
    orientation: usize,
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

fn main() {
    let p = std::fs::read_to_string("input.txt")
        .expect("failed to read file")
        .lines()
        .map(|line| line.parse::<Instruction>().unwrap())
        .fold(PositionAndOrientation{
            orientation: 0,
            x: 0,
            y: 0,
        }, |acc, inst| {
            match inst {
                Instruction::North(amount) => PositionAndOrientation{ 
                    orientation: acc.orientation,
                    x: acc.x,
                    y: acc.y + amount as i64,
                },
                Instruction::East(amount) => PositionAndOrientation{ 
                    orientation: acc.orientation,
                    x: acc.x + amount as i64,
                    y: acc.y,
                },
                Instruction::South(amount) => PositionAndOrientation{ 
                    orientation: acc.orientation,
                    x: acc.x,
                    y: acc.y - amount as i64,
                },
                Instruction::West(amount) => PositionAndOrientation{ 
                    orientation: acc.orientation,
                    x: acc.x - amount as i64,
                    y: acc.y,
                },
                Instruction::Left(amount) => PositionAndOrientation{ 
                    orientation: (acc.orientation + amount) % 360,
                    x: acc.x,
                    y: acc.y,
                },
                Instruction::Right(amount) => PositionAndOrientation{ 
                    orientation: (acc.orientation + 360 - amount) % 360,
                    x: acc.x,
                    y: acc.y,
                },
                Instruction::Forward(amount) => PositionAndOrientation{ 
                    orientation: acc.orientation,
                    x: match acc.orientation {
                        0 => acc.x + amount as i64,
                        180 => acc.x - amount as i64,
                        _ => acc.x
                    },
                    y: match acc.orientation {
                        90 => acc.y + amount as i64,
                        270 => acc.y - amount as i64,
                        _ => acc.y
                    },
                },
            }
        });
    println!("{}", p.x.abs() + p.y.abs())
}
