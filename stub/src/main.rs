use aes_gcm::Aes256Gcm;
use aes_gcm::KeyInit;
use aes_gcm::aead::{AeadInPlace, Aead, AeadCore, generic_array::GenericArray};
use aes_gcm::Nonce;
use aes_gcm::Key;
use std::io;
use std::io::prelude::*;
use std::fs::File;
use std::env;
use std::io::SeekFrom;
use aes::cipher::generic_array::{typenum::U16};

fn main() -> io::Result<()> {
    match env::var("key") {
        Ok(key) => key,
        Err(e) => e,
    };
    //Get env vars for decryption
    let key = Key::from_slice(&env::var("KEY").expect("key").as_bytes());
    let nonce = Nonce::from_slice(&env::var("NONCE").expect("nonce").as_bytes()); 
    let size = env::var("SIZE")?; 
    let filename = env::current_exe()?;
    let file = File::open(&filename)?;
    let mut buffer = Vec::new();
    
    
    
    //Read file with offest 
    file.seek(SeekFrom::End(-1*size));
    file.read_to_end(&mut buffer)?; 

    //Decrypt 
    let cipher = Aes256Gcm::new(&key);  
    cipher.decrypt_in_place(&nonce, b"", &mut buffer);


    Ok(()) 

}

fn string_to_key(string: &String) -> io::Result<()>  {
    let string_bytes = string.as_bytes();
    return Key::<Aes256Gcm>::from_slice(&string_bytes).clone();
}
