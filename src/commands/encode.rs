extern crate hex;

use hex::encode;

pub fn _encode(data : &String) -> String {
    encode(data.to_owned().into_bytes())
}