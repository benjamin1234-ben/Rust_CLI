extern crate hex;

use hex::decode;

pub fn _decode(data : &String) -> Vec<u8> {
    decode(data).unwrap()
}