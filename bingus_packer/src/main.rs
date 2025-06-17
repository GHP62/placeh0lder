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

    let mut file = File::open(name.trim());

}
