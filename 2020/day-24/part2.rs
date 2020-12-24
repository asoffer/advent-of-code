use std::collections::HashMap;
use std::collections::HashSet;

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

fn flip_tiles(tiles: &HashSet<(i64, i64)>) -> HashSet<(i64, i64)> {
    let mut adjacencies = HashMap::<(i64, i64), usize>::new();

    for &(x, y) in tiles.iter() {
        adjacencies.entry((x, y)).or_insert(0);
        for (dx, dy) in vec![(2, 0), (-2, 0), (1, 1), (1, -1), (-1, 1), (-1, -1)] {
            *adjacencies.entry((x + dx, y + dy)).or_insert(0) += 1;
        }
    }

    let mut result = HashSet::<(i64, i64)>::new();
    for (loc, count) in adjacencies.iter() {
        let is_black = tiles.contains(loc);
        if is_black && (*count == 1 || *count == 2) {
            result.insert(*loc);
        } else if !is_black && *count == 2 {
            result.insert(*loc);
        }
    }
    result
}

fn main() {
    let mut tile_flips = HashMap::<(i64, i64), usize>::new();
    let contents = std::fs::read_to_string("input.txt").expect("failed to read file");
    for (x, y) in contents.lines().map(|s| parse(s)) {
        *tile_flips.entry((x, y)).or_insert(0) += 1;
    }

    let mut tiles: HashSet<(i64, i64)> = tile_flips.iter().filter_map(|(tile, f)| if (*f % 2) == 1 {
        Some(*tile)
    } else {
        None
    }).collect();

    for _ in 0..100 {
        tiles = flip_tiles(&tiles);
    }
    println!("{}", tiles.len());
}
