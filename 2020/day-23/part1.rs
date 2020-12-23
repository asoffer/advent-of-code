use std::collections::VecDeque;

fn main() {
    let input = "157623984";
    let mut cups: VecDeque<usize> = input.chars().map(|c| c as usize - '0' as usize).collect();
   for _ in 0..100 {
        let current = cups.front().unwrap().clone();
        cups.rotate_left(1);
        let c1 = cups.pop_front().unwrap();
        let c2 = cups.pop_front().unwrap();
        let c3 = cups.pop_front().unwrap();
        let mut next_label = 1 + (current + (input.len() - 2)) % input.len();
        while !cups.contains(&next_label) {
            next_label = 1 + (next_label + (input.len() - 2)) % input.len();
        }
        while cups.back() != Some(&next_label) {
            cups.rotate_left(1);
        }
        cups.push_back(c1);
        cups.push_back(c2);
        cups.push_back(c3);

        while cups.back() != Some(&current) {
            cups.rotate_left(1);
        }
    }

    while cups.back() != Some(&1) {
        cups.rotate_left(1);
    }
    cups.pop_back();

    println!("{:?}", cups);
}
