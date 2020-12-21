#[macro_use]
extern crate scan_fmt;

use std::collections::HashMap;

#[derive(Debug)]
struct Tile<'a> {
    lines: Vec<&'a str>,
}

#[derive(Debug)]
struct Boundary {
    set: Vec<String>,
}

fn make_tile<'a>(s: &'a str) -> (usize, Tile<'a>, Boundary) {
    let mut iter = s.lines();
    let num = scan_fmt!(iter.next().unwrap(), "Tile {}:", usize).unwrap();

    let tile = Tile {
        lines: iter.collect::<Vec<&'a str>>(),
    };
    let mut boundaries = Vec::<String>::new();
    boundaries.push(tile.lines.first().unwrap().to_string());
    boundaries.push(tile.lines.last().unwrap().to_string());

    let mut first_col = String::new();
    let mut last_col = String::new();
    for line in &tile.lines {
        first_col.push(line.to_string().chars().nth(0).unwrap());
        last_col.push(line.to_string().chars().nth(line.len() - 1).unwrap());
    }
    boundaries.push(first_col);
    boundaries.push(last_col);

    (num, tile, Boundary { set: boundaries })
}

fn main() {
    let contents = std::fs::read_to_string("input.txt").expect("failed to read file");
    let tiles: Vec<(usize, Tile, Boundary)> =
        contents.split("\n\n").map(|s| make_tile(s)).collect();
    let mut boundary_map = HashMap::<String, Vec<usize>>::new();
    for (n, t, boundary) in &tiles {
        for b in &boundary.set {
            boundary_map
                .entry(b.clone())
                .or_insert(Vec::<usize>::new())
                .push(*n);
            boundary_map
                .entry(b.chars().rev().collect())
                .or_insert(Vec::<usize>::new())
                .push(*n);
        }
    }
    let mut degree = HashMap::<usize, usize>::new();
    for (b, v) in &boundary_map {
        for tile_id in v {
            *degree.entry(*tile_id).or_insert(0) += v.len() - 1;
        }
    }
    let mut total = 1;
    for (tile, deg) in degree {
        if deg == 4 {
            total *= tile;
        }
    }
    println!("{:?}", total);
}
