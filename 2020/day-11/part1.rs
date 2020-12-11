fn step(seats: &mut Vec<Vec<char>>) -> bool {
    let mut modified = false;
    let seats_len = seats.len();
    for i in 0..seats_len {
        let row_len = seats[i].len();
        for j in 0..row_len {
            let mut adj_occupied = 0;
            for r in (std::cmp::max(1, i) - 1)..=std::cmp::min(i + 1, seats_len - 1) {
                for c in (std::cmp::max(1, j) - 1)..=std::cmp::min(j + 1, row_len - 1) {
                    if r == i && c == j {
                        continue;
                    }
                    adj_occupied += (seats[r][c] == '#') as usize;
                    adj_occupied += (seats[r][c] == 'l') as usize;
                }
            }

            seats[i][j] = match seats[i][j] {
                'L' => {
                    if adj_occupied == 0 {
                        modified = true;
                        '3'
                    } else {
                        seats[i][j]
                    }
                }
                '#' => {
                    if adj_occupied >= 4 {
                        modified = true;
                        'l'
                    } else {
                        seats[i][j]
                    }
                }
                _ => seats[i][j],
            };
        }
    }

    if modified {
        for row in seats.iter_mut() {
            for seat in row.iter_mut() {
                *seat = match seat {
                    '3' => '#',
                    'l' => 'L',
                    _ => seat.clone(),
                };
            }
        }
    }
    modified
}

fn main() {
    let mut seats: Vec<Vec<char>> = std::fs::read_to_string("input.txt")
        .expect("failed to read file")
        .lines()
        .map(|line| line.chars().collect())
        .collect();
    while step(&mut seats) {}
    println!(
        "{}",
        seats.iter().flatten().filter(|&seat| *seat == '#').count()
    );
}
