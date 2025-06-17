use aes::Aes256;
use aes::cipher::{
    BlockCypher, BlockEncrypt, BlockDecrypt, KeyInit, generic_array::GenericArray
};
use std::io::{prelude::*, BufReader};
use std::fs::File;


fn main() -> io::Result<()> {

    //Get file for packing

    let mut input = String::new();
    println!("Enter file name to pack: \n>");
    io::stdin().read_line(&mut input)?;
    
    let data = encrypt(&input);
    add_stub();
}

fn encrypt(name: &str) {

    //read file into u8s
    let data: Vec<u8> = fs::read(name.trim();)
        
    let key = GenericArray::from([0u8; 16]);
    let cipher = Aes128::new(&key);
    let dataCopy = data.clone();
    cipher.encrypt_block(&mut dataCopy);
}
