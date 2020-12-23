use std::collections::HashSet;
use std::collections::VecDeque;

fn play(
    p1: &mut VecDeque<usize>,
    p2: &mut VecDeque<usize>,
) -> usize {
    let mut games = HashSet::<(VecDeque<usize>, VecDeque<usize>)>::new();
    loop {
        if p1.is_empty() {
            return 2;
        } else if p2.is_empty() {
            return 1;
        }

        if !games.insert((p1.clone(), p2.clone())) { 

            return 1; }

        let c1 = p1.pop_front().unwrap();
        let c2 = p2.pop_front().unwrap();
        if c1 > p1.len() || c2 > p2.len() {
            assert!(c1 != c2);
            if c1 > c2 {
                p1.push_back(c1);
                p1.push_back(c2);
            } else {
                p2.push_back(c2);
                p2.push_back(c1);
            }
            continue;
        }

        let mut sub1 = p1.clone();
        sub1.truncate(c1);
        let mut sub2 = p2.clone();
        sub2.truncate(c2);
        match play(&mut sub1, &mut sub2) {
            1 => {
                p1.push_back(c1);
                p1.push_back(c2);
            },
            _ => {
                p2.push_back(c2);
                p2.push_back(c1);
            },
        }
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

    let result = play(&mut player1, &mut player2);
    let winner = match result {
        1 => &player1,
        _ => &player2,
    };

    let length = winner.len();
    let mut total = 0;
    for (i, s) in winner.iter().enumerate() {
        total += (length - i) * s;
    }
    println!("{}", total);
}
