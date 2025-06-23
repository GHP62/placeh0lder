use aes_gcm::Aes256Gcm;
use aes_gcm::KeyInit;
use aes_gcm::aead::{AeadInPlace, generic_array::GenericArray};
use std::io;
use std::io::prelude::*;
use std::fs::File;
use std::env;
use std::io::SeekFrom;

fn main() -> io::Result<()> {

    let key: &'static str =env!("KEY");
    let nonce: &'static str= env!("NONCE");
    let size: &'static str = env!("SIZE"); 

    println!("{}, {}, {}", size,key ,nonce ); 
    let key_form = GenericArray::from_slice(key.as_bytes());

    let nonce_form = GenericArray::from_slice(nonce.as_bytes());

    let size_form: i64= size.parse().expect("parseerror"); 
    let mut buffer = Vec::new();
    let mut file = File::open(env::current_exe().expect(""))?;

    let _ = file.seek(SeekFrom::End(-1*size_form));
    let _ = file.read_to_end(&mut buffer)?; 

    //Decrypt 
    let cipher = Aes256Gcm::new(&key_form);  
    let _ = cipher.decrypt_in_place(&nonce_form, b"", &mut buffer);

    let mut output = File::create("decrypted.txt")?;
    let _ = output.write_all(&buffer);

    Ok(()) 

}


