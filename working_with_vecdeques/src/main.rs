/*
    [COMPLEXITY] - VecDeques -> O(1)

    A VecDeque is a double-ended queue, which means you can
    push and pop from both ends of the queue.
*/

use rand::seq::SliceRandom;
use rand::thread_rng;
use std::collections::VecDeque;

fn main() {
    let mut fruits: VecDeque<&str> = VecDeque::new();
    fruits.push_back("Apple");
    fruits.push_back("Pear");
    fruits.push_back("Banana");
    fruits.push_back("Plum");
    fruits.push_back("Grape");

    let mut rng = thread_rng();
    let mut fruits: Vec<_> = fruits.into_iter().collect();
    fruits.shuffle(&mut rng);

    let mut fruits: VecDeque<_> = fruits.into_iter().collect();

    fruits.push_front("Blue Berry");
    fruits.push_back("Watermelon");

    println!("\n_____Creating Fruit Salad Recipe_____\n");

    for (i, item) in fruits.iter().enumerate() {
        let _ = i != fruits.len() - 1;
        println!(" -> {}", item)
    }

    println!();
}
