fn modular_inverse(n: i64, modulus: i64) -> i64 {
    for i in 1..modulus {
        if (n * i) % modulus == 1 { return i; }
    }
    panic!();
}

struct ModularConstraint {
    value: i64,
    modulus: i64,
}

fn solve(prev_solution: i64, prev_modulus: i64, next: ModularConstraint) -> i64 {
    let prev_inv = modular_inverse(prev_modulus, next.modulus);
    let p = prev_solution % next.modulus;
    return prev_solution + (next.value - p) * prev_modulus * prev_inv;
}

fn main() {
    let input = std::fs::read_to_string("input.txt")
        .expect("failed to read file");
    let v: Vec<&str> = input.lines().collect();
    let timestamp = v[0].parse::<i64>().unwrap();
    let busses: Vec<Option<i64>> = v[1].split(',').map(|s| match s.parse::<i64>() {
        Ok(n) => Some(n),
        _     => None,
    }).collect();

    let mut last_solution: i64 = 0;
    let mut product: i64 = 1;
    for (i, bus) in busses.iter().enumerate() {
        let num = match bus {
            Some(num) => num,
            None      => continue,
        };
        last_solution = solve(last_solution, product, ModularConstraint{
            value: *num - i as i64,
            modulus: *num,
        });
        product *= *num;
        last_solution %= product;
        last_solution = (product + last_solution) % product;
    }
    println!("{}", last_solution);
}
