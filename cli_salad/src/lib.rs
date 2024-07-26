use rand::seq::SliceRandom;
use rand::thread_rng;

pub fn create_fruit_salad(number_of_fruits: usize) -> Vec<String> {
    let mut fruits = vec![
        "Apple".to_owned(),
        "Banana".to_owned(),
        "Blue Berry".to_owned(),
        "Strawberry".to_owned(),
        "Watermelon".to_owned(),
        "Grapes".to_owned(),
        "Plum".to_owned(),
        "Beetroot".to_owned(),
        "Loquat".to_owned(),
        "Raspberry".to_owned(),
        "Pear".to_owned(),
    ];

    let mut rng = thread_rng();
    fruits.shuffle(&mut rng);

    println!("\n[No. of Fruits] -> {}\n", number_of_fruits);

    let mut batch: Vec<_> = Vec::new();

    for (i, item) in fruits.iter().enumerate() {
        batch.push(item.to_owned());
        if batch.len() == number_of_fruits {
            println!("[ADDING] :: {} -> {}", i, item);
            break;
        }
    }
    println!("{:?}", batch);
    batch
}
