extern crate magic_crypt;

use magic_crypt::{new_magic_crypt, MagicCryptTrait};

pub fn _decrypt(data : &String) -> String {
    let mc = new_magic_crypt!("Crypto", 256);

    mc.decrypt_base64_to_string(data).unwrap()
}