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
    let mut ingredients_with_allergens = HashSet::<String>::new();
    for (_, is) in map {
        for i in &is {
            ingredients_with_allergens.insert(i.clone());
        }
    }
    let mut total = 0;
    for (ingredients, _) in &stuff {
        for ingredient in ingredients {
            if !ingredients_with_allergens.contains(ingredient) {
                total += 1;
            }
        }
    }
    println!("{}", total);
}
