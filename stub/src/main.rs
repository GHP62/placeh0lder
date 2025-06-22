use aes_gcm::Aes256Gcm;
use aes_gcm::KeyInit;
use aes_gcm::aead::{AeadInPlace, generic_array::GenericArray};
use std::io;
use std::io::prelude::*;
use std::fs::File;
use std::env;
use std::io::SeekFrom;

fn main() -> io::Result<()> {

    let key = match env::var("KEY") {
        Ok(key) => key.into_bytes(),
        Err(e) => e.to_string().into_bytes(),
    };

    let nonce = match env::var("NONCE") {
        Ok(nonce) => nonce.into_bytes(),
        Err(e) => e.to_string().into_bytes(),
    };

    let size = match env::var("SIZE") {
        Ok(size) => size.into_bytes(),
        Err(e) => e.to_string().into_bytes(),
    };
    
    let key_form = GenericArray::from_slice(&key).clone();

    let nonce_form = GenericArray::from_slice(&nonce).clone();

    let size_form: [u8; 4] = size.try_into().expect("");
    let mut buffer = Vec::new();
    let mut file = File::open(env::current_exe().expect(""))?;

     let _ =file.seek(SeekFrom::End((-1*(i32::from_le_bytes(size_form))).into()));
    let _ = file.read_to_end(&mut buffer)?; 

    //Decrypt 
    let cipher = Aes256Gcm::new(&key_form);  
    let _ = cipher.decrypt_in_place(&nonce_form, b"", &mut buffer);


    Ok(()) 

}

