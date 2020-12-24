use std::collections::HashMap;

fn parse(line: &str) -> (i64, i64) {
    let mut iter = line.chars();
    let mut x = 0i64;
    let mut y = 0i64;
    loop {
        match iter.next() {
            Some('e') => { x += 2; },
            Some('w') => { x -= 2; },
            Some('n') => match iter.next().unwrap() {
                'e' => { y += 1; x += 1; },
                'w' => { y += 1; x -= 1; },
                _ => panic!(),
            },
            Some('s') => match iter.next().unwrap() {
                'e' => { y -= 1; x += 1; },
                'w' => { y -= 1; x -= 1; },
                _ => panic!(),
            },
            _ => { return (x, y); }
        };
    }
}

fn main() {
    let mut tile_flips = HashMap::<(i64, i64), usize>::new();
    let contents = std::fs::read_to_string("input.txt").expect("failed to read file");
    for (x, y) in contents.lines().map(|s| parse(s)) {
        *tile_flips.entry((x, y)).or_insert(0) += 1;
    }

    println!("{}", tile_flips.iter().filter(|(_tile, f)| (*f % 2) == 1).count());
}
