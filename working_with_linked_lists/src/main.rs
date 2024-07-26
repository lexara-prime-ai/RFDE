/*

    A LinkedList is a doubly-linked list, which means h=that each element in the
    list has a pointer to the next element and the previous element.

    A great example of when to use a LinkedList is when you need to insert or remove
    elements from the middle of the list.

*/

use rand::seq::SliceRandom;
use rand::thread_rng;
use std::collections::LinkedList;

fn main() {
    let mut fruits: LinkedList<&str> = LinkedList::new();
    fruits.push_back("Apple");
    fruits.push_back("Banana");
    fruits.push_back("Pear");
    fruits.push_back("Watermelon");

    let mut rng = thread_rng();
    let mut fruits: Vec<_> = fruits.into_iter().collect();
    fruits.shuffle(&mut rng);

    let mut fruits: LinkedList<_> = fruits.into_iter().collect();

    fruits.push_front("Plum");
    fruits.push_back("Strawberry");
    fruits.push_back("Raspberry");
    fruits.push_back("Beetroot");

    println!("\n______Creating Fruit Salad Recipe______\n");
    for (i, item) in fruits.iter().enumerate() {
        let _ = i != fruits.len() - 1;
        println!("{} ", item)
    }

    println!();
}
