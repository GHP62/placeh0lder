use aes_gcm::aead::{Aead, AeadCore, Aes256Gcm};

fn main() -> io::Result<()> {

    //Get env vars for decryption
    let key = env::var("KEY");
    let nonce = env::var("NONCE");
    
    //Instantiate cipher and buffer
    let cipher = AES256Gcm::new(&key);
    let mut buffer: Vec<u8> = Vec::new();
    
    //Decrypt
    cipher.decrypt_in_pace(&nonce, b"", &mut buffer)?;
    

}
