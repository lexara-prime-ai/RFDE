use rand::{thread_rng, Rng};
use std::collections::HashMap;

fn homophonic_cipher(plaintext: &str) -> (String, HashMap<char, Vec<char>>) {
    let mut rng = thread_rng();
    let alphabet: Vec<char> = ('a'..='z').collect();
    let mut ciphertext = String::new();
    let mut mapping: HashMap<char, Vec<char>> = HashMap::new();

    for c in &alphabet {
        let homophones: Vec<char> = (0..rng.gen_range(2..4))
            .map(|_| rng.gen_range('a'..='z'))
            .collect();

        mapping.insert(*c, homophones);
    }

    for c in plaintext.chars() {
        if let Some(c) = c.to_lowercase().next() {
            if let Some(homophones) = mapping.get(&c) {
                if let Some(&homophone) = homophones.get(rng.gen_range(0..homophones.len())) {
                    ciphertext.push(homophone);
                } else {
                    eprintln!("[ERROR] :: No homophones for character {}", c);
                }
            }
        } else {
            ciphertext.push(c);
        }
    }

    println!("[Plain Text] -> {}", plaintext);
    println!("[Ciphertext] -> {}", ciphertext);
    println!("[Mapping] -> {:?}", mapping);

    (ciphertext, mapping)
}

fn main() {
    let plaintext = "What is life";
    homophonic_cipher(plaintext);
}
