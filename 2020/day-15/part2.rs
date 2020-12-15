use std::collections::HashMap;

fn insert(map: &mut HashMap<usize, usize>, index: usize, num: usize) -> usize {
    let mut next_num_to_say = 0;
    map.entry(num)
        .and_modify(|n| {
             next_num_to_say = index - *n;
            *n = index;
        })
        .or_insert(index);
    next_num_to_say
}

fn main() {
    let seeds = vec![14,1,17,0,3,20];
    let mut sequence = HashMap::<usize, usize>::new();
    for i in 0..(seeds.len() - 1) {
        insert(&mut sequence, i + 1, seeds[i]);
    }

    let mut last: usize = *seeds.last().unwrap();
    for i in seeds.len()..30000000 {
        last = insert(&mut sequence, i, last);
    }
    println!("{}", last);
}
