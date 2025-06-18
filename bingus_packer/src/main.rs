use aes_gcm::aead::{Aead, OsRng, rand_core::RngCore};
use std::fs;
use rand::Rng;
use std::io;


fn main() -> io::Result<()> {

    println!("What file would you like to pack?");
    let file = ask_for_input()?;
    let data: Vec<u8> = fs::read(file)?;
    println!("What would you like the key to be? (")
    Ok(())
} 

fn ask_for_input() -> io::Result<String> {
    let mut input = String::new();
    io::stdin().read_line(&mut input)?;
    return Ok(input)
}

fn encrypt(file: Vec<u8>) -> Result<Vec<u8>> {

    let mut nonce = [0u8; 12];
    OsRng.fill_bytes(&mut nonce);
    
