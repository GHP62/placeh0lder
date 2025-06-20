use aes_gcm::AeadInPlace;
use aes_gcm::aead::OsRng;
use aes_gcm::Aes256Gcm;
use aes_gcm::AeadCore;
use std::fs;
use std::io;
use aes_gcm::KeyInit;
use std::fs::File;
use std::io::Write;

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
    output.write_all(&encrypted_data)?;
    
    //Append stub file 
    let stub: Vec<u8> = fs::read("stub")?;
    output.write_all(&stub);

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
    let cipher = Aes256Gcm::new(&key);
    let nonce = Aes256Gcm::generate_nonce(&mut OsRng);

    //Encryption
    let mut buffer: Vec<u8> = Vec::new();
    buffer.extend_from_slice(&file);
    cipher.encrypt_in_place(&nonce, b"", &mut buffer).expect("Encrypt Error");
    Ok(buffer)

}
