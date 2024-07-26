use rand::seq::SliceRandom;
use rand::thread_rng;
use std::collections::BTreeSet;

fn main() {
    let fruits = vec![
        "Apple",
        "Banana",
        "Blue Berry",
        "Strawberry",
        "Watermelon",
        "Grapes",
        "Plum",
        "Beetroot",
        "Loquat",
        "Raspberry",
        "Pear",
    ];

    let amounts = [1, 3, 5, 7, 9];

    let mut rng = thread_rng();

    for amount in amounts.iter() {
        let mut fruits_set: BTreeSet<&str> = BTreeSet::new();
        let mut shuffled_fruits = fruits.clone();
        shuffled_fruits.shuffle(&mut rng);

        for fruit in shuffled_fruits {
            fruits_set.insert(fruit);
            if fruits_set.len() >= *amount {
                break;
            }
        }
        println!("{}: {:?}", amount, fruits_set);
    }
}
