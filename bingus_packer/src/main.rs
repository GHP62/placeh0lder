use aes_gcm::aead::{Aead, OsRng, rand_core::RngCore};
use aes_gcm::Aes256Gcm;
use aes_gcm::AeadCore;
use std::fs;
use rand::Rng;
use std::io;
use aes_gcm::Key;
use aes_gcm::KeyInit;
fn main() -> io::Result<()> {
    //Ask user for file they wanna pack
    println!("What file would you like to pack?");
    //Handle error from ask_for_input
    let file = match ask_for_input(){
        Err(e) => return Err(e),
        Ok(file) => file,
    };
    //put file into u8 vec
    let data: Vec<u8> = fs::read(file)?;
    Ok(())
} 

fn ask_for_input() -> io::Result<String> {
    let mut input = String::new();
    io::stdin().read_line(&mut input)?;
    return Ok(input)
}

fn encrypt(file: Vec<u8>) -> Result<Vec<u8>> {
    //setup encryption variables
    let key = Aes256Gcm::generate_key(&mut OsRng);
    let cipher = Aes256Gcm::new(&key);
    let nonce = Aes256Gcm::generate_nonce(&mut OsRng);

    let mut buffer: Vec<u8, > = Vec::new();
    buffer.extend_from_slice(&file);



}
