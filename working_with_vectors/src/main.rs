use rand::seq::SliceRandom;
use rand::thread_rng;

fn main() {
    let mut fruits = vec!["Apple", "Orange", "Plum", "Strawberry"];

    let mut rng = thread_rng();
    fruits.shuffle(&mut rng);

    println!("\n_______Creating Fruit Salad Recipe_______\n");

    for (i, item) in fruits.iter().enumerate() {
        match i != fruits.len() - 1 {
            _i => println!("-> {} ", item),
        }
    }

    println!();
}
