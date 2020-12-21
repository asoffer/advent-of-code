use std::collections::hash_map;
use std::collections::HashMap;
use std::collections::HashSet;

fn main() {
    let contents = std::fs::read_to_string("input.txt").expect("failed to read file");
    let stuff: Vec<(HashSet<String>, Vec<String>)> = contents.lines().map(|s| {
        let mut iter = s.split(" (contains ");
        let ingredients = iter.next().unwrap();
        let allergens = iter.next().unwrap();
        (ingredients, allergens.split(")").next().unwrap())
    }).map(|(i, a)| {
        let ingredients: HashSet<String> = i.split(' ').map(|s| s.to_string()).collect();
        let allergens: Vec<String> = a.split(", ").map(|s| s.to_string()).collect();
        (ingredients, allergens)
    }).collect();
    let mut map = HashMap::<String, HashSet<String>>::new();
    for (ingredients, allergens) in &stuff {
        for a in allergens {
            match map.entry(a.clone()) {
                hash_map::Entry::Occupied(mut o) => {
                    *o.get_mut() = o.get().intersection(&ingredients).cloned().collect();
                },
                hash_map::Entry::Vacant(v) => {
                    v.insert(ingredients.clone());
                },
            }
        }
    }

            println!("{:?}", map);
    let mut ingredient_to_allergen = HashMap::<String, String>::new();
    let mut found = true;
    while found {
        found = false;
        for (allergen, is) in &map {
            let set: HashSet<String> = is.iter().filter(|i| !ingredient_to_allergen.contains_key(i.clone())).map(|i| i.clone()).collect();
            if set.len() == 1 {
                let i = set.iter().next().unwrap();
                ingredient_to_allergen.insert(i.clone(), allergen.clone());
                found = true;
            }
        }
    }

    let mut v: Vec<(&String, &String)> = ingredient_to_allergen.iter().collect();
    v.sort_by(|(_, a), (_, b)| a.cmp(b));
    for (i, _) in v {
        print!("{},", i);
    }
}
