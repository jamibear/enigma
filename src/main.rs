use clap::{Parser, ValueEnum};

#[derive(Parser)]
#[command(author, version, about, long_about = None)]
struct Cli {
    /// What mode to run the program in
    #[arg(long, short)]
    cipher: Cipher,

    #[arg(long, short)]
    key: usize,

    #[arg(long, short)]
    message: String,
}

#[derive(Copy, Clone, PartialEq, Eq, PartialOrd, Ord, ValueEnum)]
enum Cipher {
    Ceasar,
    // Atbash,
    // Affine,
    // Keyword,
}

// Encrypt message in Ceasar Cipher
fn encrypt_ceasar(message: &String, key: &usize) {
    let alphabet = vec![
        'a', 'b', 'c', 'd', 'e', 'f', 'g', 'h', 'i', 'j', 'k', 'l', 'm', 'n', 'o', 'p', 'q', 'r',
        's', 't', 'u', 'v', 'w', 'x', 'y', 'z',
    ];

    let mut encrypted_message = String::new();
    for letter in message.chars() {
        // find the index of each letter in the message
        let index = alphabet.iter().position(|&x| x == letter);
        match index {
            Some(i) => {
                // add the key from the current index
                let encrypted_letter = alphabet[(i as usize + *key as usize) % alphabet.len()];
                // push the found letter to the encrypted message
                encrypted_message.push(encrypted_letter)
            }
            None => encrypted_message.push(' '),
        }
    }
    println!("{}", encrypted_message)
}

fn main() {
    let cli = Cli::parse();

    match cli.cipher {
        Cipher::Ceasar => {
            // shift every letter
            encrypt_ceasar(&cli.message, &cli.key);
        }
        // TODO 
        // Cipher::Atbash => {
           // println!("Atbash");
        //}
        // Cipher::Affine => {
            // println!("Affine");
        // }
        // Cipher::Keyword => {
            // println!("Keyword");
        // }
    }
}
