extern crate magic_crypt;

use magic_crypt::{new_magic_crypt, MagicCryptTrait};

pub fn _encrypt(data : &String) -> Vec<u8> {
    let mc = new_magic_crypt!("Crypto", 256);
    
    // mc.encrypt_str_to_base64(data)
    mc.encrypt_str_to_bytes(data)
}