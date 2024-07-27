use caesar_ciphers::{decrypt, encrypt};

fn main() {
    let plaintext = "Why is life";
    let shift = 3;
    let ciphertext = encrypt(&plaintext, shift);
    let decrypted_text = decrypt(&ciphertext, shift);

    println!("{}\n{}\n{}\n", plaintext, ciphertext, decrypted_text);
}
