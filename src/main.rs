extern crate clap;

mod commands;

use clap::{Parser, Subcommand};
use commands::decrypt::_decrypt;
use commands::encrypt::_encrypt;
use commands::hash::_hash;
use commands::encode::_encode;
use commands::decode::_decode;

#[derive(Parser)]
#[command(name = "Crypto")]
#[command(author = "SourceCode. <sourcecode042@gmail.com>")]
#[command(version = "1.0.0")]
#[command(about = "Does cryptographic functions like encryption, decryption and hashing", long_about = None)]
struct Crypto {
    #[command(subcommand)]
    command : Option<Commands>
}

#[derive(Subcommand)]
enum Commands {
    /// Decryption of data
    Decrypt {
        #[arg(short, long)]
        data : String
    },
    Decode {
        #[arg(short, long)]
        data : String
    },
    Encode {
        #[arg(short, long)]
        data : String
    },
    /// Encryption of data
    Encrypt {
        #[arg(short, long)]
        data : String
    },
    /// Hashing of data
    Hash {
        #[arg(short, long)]
        data : String
    }
}

fn main() {
    println!("Welcome to the Rust Crypto CLI!");

    let cli = Crypto::parse();

    match &cli.command {
        Some(Commands::Decrypt { data }) => {
            println!("{:?}", data);
            println!("{}", _decrypt(&*data));
        },
        Some(Commands::Decode { data }) => {
            println!("{:?}", data);
            println!("{:?}", _decode(&*data));
        },
        Some(Commands::Encode { data }) => {
            println!("{:?}", data);
            println!("{}", _encode(&*data));
        },
        Some(Commands::Encrypt { data }) => {
            println!("{:?}", data);
            println!("{:?}", _encrypt(&*data));
        },
        Some(Commands::Hash { data }) => {
            println!("{:?}", data);
            println!("{}", _hash(&data));
        },
        None => {}
    }
}
