use aes_gcm::aead::{Aead, AeadCore, Aes256Gcm};
use std::io;
use std::io::prelude::*;
use std::fs::File;
use std::env;
use std::io::SeekFrom;

fn main() -> io::Result<()> {

    //Get env vars for decryption
    let key = env::var("KEY")?;
    let nonce = env::var("NONCE")?;
    let size = env::var("SIZE")?.parse()?; 
    let filename = env::current_exe()?;
    let file = File::open(&filename)?;
    let mut buffer = Vec::new();

    //Read file with offest 
    file.seek(SeekFrom::End(-1*size));
    file.read_to_end(&mut buffer)?; 

    //Decrypt 
    let cipher = Aes256Gcm::new(&key);  
    cipher.decrypt_in_pace(&nonce, b"", &mut buffer)?;


    Ok(()) 

}
