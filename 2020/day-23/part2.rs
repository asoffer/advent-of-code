use std::collections::HashMap;

fn next(cups: &HashMap<usize, usize>, c: usize) -> usize {
    match cups.get(&c) {
        Some(&n) => n,
        None => (c % 1_000_000) + 1,
    }
}

fn main() {
    let mut cups = HashMap::<usize, usize>::new();
    cups.insert(1, 5);
    cups.insert(5, 7);
    cups.insert(7, 6);
    cups.insert(6, 2);
    cups.insert(2, 3);
    cups.insert(3, 9);
    cups.insert(9, 8);
    cups.insert(8, 4);
    cups.insert(4, 10);
    cups.insert(1_000_000, 1);

    let mut current = 1;
    for _ in 0..10_000_000 {
        let c1 = next(&cups, current);
        let c2 = next(&cups, c1);
        let c3 = next(&cups, c2);

        let mut next = match current - 1 {
            0 => 1_000_000,
            n => n,
        };
        while vec![c1, c2, c3].contains(&next) {
            next -= 1;
            if next == 0 {
                next = 1_000_000;
            }
        }

        let displaced = match cups.insert(next, c1) {
            Some(n) => n,
            None => (next % 1_000_000) + 1,
        };
        let displaced = match cups.insert(c3, displaced) {
            Some(n) => n,
            None => (c3 % 1_000_000) + 1,
        };
        cups.insert(current, displaced);

        current = displaced;
    }

    let a = next(&cups, 1);
    let b = next(&cups, a);

    println!("{}", a * b);
}
