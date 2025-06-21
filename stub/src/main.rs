use aes_gcm::Aes256Gcm;
use aes_gcm::KeyInit;
use aes_gcm::aead::{AeadInPlace, Aead, AeadCore, generic_array::GenericArray};
use aes_gcm::Nonce;
use std::io;
use std::io::prelude::*;
use std::fs::File;
use std::env;
use std::io::SeekFrom;

fn main() -> io::Result<()> {

    //Get env vars for decryption
    let key = env::var("KEY").expect("");
    let nonce = env::var("NONCE").expect("");
    let size = env::var("SIZE").expect("").parse()?; 
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
