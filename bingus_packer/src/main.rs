use aes_gcm::AeadInPlace;
use std::process::Command;
use aes_gcm::aead::OsRng;
use aes_gcm::Aes256Gcm;
use aes_gcm::AeadCore;
use std::fs;
use std::io;
use aes_gcm::KeyInit;
use std::fs::File;
use std::io::Write;
use std::env;
use base64::{engine::general_purpose, Engine};


fn main() -> io::Result<()> {

    //Ask user for file they wanna pack
    println!("What file would you like to pack?");
    //Handle error from ask_for_input
    let file = match ask_for_input(){
        Err(e) => return Err(e),
        Ok(file) => file,
    };

    //put file into u8 vec to &[u8]
    let data: Vec<u8> = fs::read(&file.trim())?;
    let encrypted_data = match encrypt(data) {
        Err(e) => return Err(e),
        Ok(encrypted_data) => encrypted_data,
    };
    

    //Create the output file
    let mut output = File::create(format!("encrypted-{}", file.trim()))?;
    let path = env::current_exe().expect("Could not get path");
    let dir = path.parent().expect("Cannot get parent");
    let out = Command::new("cargo").arg("build").current_dir(dir).status().expect("f ts");
    println!("{}", out);
    let stub: Vec<u8> = fs::read("stub")?;
    let _ = output.write_all(&stub)?;
    let _ = output.write_all(&encrypted_data)?;
    Ok(())
}

fn ask_for_input() -> io::Result<String> {

    let mut input = String::new();
    io::stdin().read_line(&mut input)?;
    return Ok(input)

}

fn encrypt(file: Vec<u8>) -> io::Result<Vec<u8>> {

    //setup encryption variables
    let key = Aes256Gcm::generate_key(&mut OsRng);
    let enc_key = general_purpose::STANDARD.encode(&key);
    println!("{}", key.bytes().len());
    unsafe {
        let _ = env::set_var("KEY", enc_key);
    }
 
    let cipher = Aes256Gcm::new(&key);

    let nonce = Aes256Gcm::generate_nonce(&mut OsRng);
    let enc_nonce= general_purpose::STANDARD.encode(&nonce);
    unsafe {
        let _ = env::set_var("NONCE", enc_nonce);
    }
    //Encryption
    let mut buffer: Vec<u8> = Vec::new();
    buffer.extend_from_slice(&file);
    cipher.encrypt_in_place(&nonce, b"", &mut buffer).expect("Encrypt Error");
    unsafe {
        let _ = env::set_var("SIZE", buffer.len().to_string());
    }
    Ok(buffer)

}
