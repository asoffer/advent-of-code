use std::collections::VecDeque;

fn take_turn(p1: &mut VecDeque<usize>, p2: &mut VecDeque<usize>) -> () {
    let c1 = p1.pop_front().unwrap();
    let c2 = p2.pop_front().unwrap();
    assert!(c1 != c2);
    if c1 > c2 {
        p1.push_back(c1);
        p1.push_back(c2);
    } else {
        p2.push_back(c2);
        p2.push_back(c1);
    }
}

fn main() {
    let contents = std::fs::read_to_string("input.txt").expect("failed to read file");
    let mut iter = contents.split("\n\n");
    let mut player1: VecDeque<usize> = iter
        .next()
        .unwrap()
        .lines()
        .filter_map(|s| match s.parse() {
            std::result::Result::Ok(n) => Some(n),
            _ => None,
        })
        .collect();
    let mut player2: VecDeque<usize> = iter
        .next()
        .unwrap()
        .lines()
        .filter_map(|s| match s.parse() {
            std::result::Result::Ok(n) => Some(n),
            _ => None,
        })
        .collect();

    while player1.len() != 0 && player2.len() != 0 {
        take_turn(&mut player1, &mut player2);
    }

    let winner = if player1.len() == 0 {
        &player2
    } else {
        &player1
    };

    let length = winner.len();
    let mut total = 0;
    for (i, s) in winner.iter().enumerate() {
        total += (length - i) * s;
    }
    println!("{}", total);
}
