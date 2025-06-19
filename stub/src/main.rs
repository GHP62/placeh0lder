use aes_gcm::aead::{Aead, AeadCore, Aes256Gcm};

fn main() {
    let key = PLACEHOLDER;
    let cipher = AES256Gcm::new(&key);
}
