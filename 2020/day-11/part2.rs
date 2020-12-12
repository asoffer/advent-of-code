fn step(seats: &mut Vec<Vec<char>>) -> bool {
    let mut modified = false;
    let seats_len = seats.len();
    for i in 0..seats_len {
        let row_len = seats[i].len();
        for j in 0..row_len {
            let mut adj_occupied = 0;
            for r in -1..=1 {
                for c in -1..=1 {
                    if r == 0 && c == 0 {
                        continue;
                    }
                    let mut check_r = (i as i32) + (r as i32);
                    let mut check_c = (j as i32) + (c as i32);
                    while check_r >= 0i32
                        && check_r < seats_len as i32
                        && check_c >= 0i32
                        && check_c < row_len as i32
                    {
                        if seats[check_r as usize][check_c as usize] != '.' {
                            adj_occupied += (seats[check_r as usize][check_c as usize] == '#') as usize;
                            adj_occupied += (seats[check_r as usize][check_c as usize] == 'l') as usize;
                            break;
                        }

                        check_r += r;
                        check_c += c;
                    }
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
                    if adj_occupied >= 5 {
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
