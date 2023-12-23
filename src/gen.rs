use bitcoin::secp256k1::{rand, Secp256k1};
use bitcoin::{Address, Network, PrivateKey, PublicKey};

use num_format::{Locale, ToFormattedString};
use std::fs::OpenOptions;

// use std::io::Write;
use std::io::{self, Write};
use std::time::Instant;

use crate::{not_mutex_print_progress, print_force};

pub fn generate_keys() -> (String, String, String) {
    // Generate random key pair.
    let s = Secp256k1::new();
    let (secret_key, public_key) = s.generate_keypair(&mut rand::thread_rng());

    // Generate pay-to-pubkey-hash address.
    let address = Address::p2pkh(&PublicKey::new(public_key), Network::Bitcoin).to_string();
    let address_segwit = Address::p2wpkh(&PublicKey::new(public_key), Network::Bitcoin)
        .unwrap()
        .to_string();
    let private_key = PrivateKey::new(secret_key, Network::Bitcoin).to_string();

    return (address, address_segwit, private_key);
}

pub fn print_keys(address: String, address_segwit: String, private_key: String) {
    println!("Address: {}", address);
    println!("SegWit Address: {}", address_segwit);
    println!("Private Key: {}", private_key);
}

pub fn gen(save: bool, output: String, count: u32, head: bool, rewrite: bool) {
    println!("NeCryWaGen - Generator\n-----------------\n");

    let mut k = 0;

    let start = Instant::now();

    let mut file: Box<dyn Write + Send> = if save {
        Box::new(
            OpenOptions::new()
                .write(rewrite)
                .append(!rewrite)
                .create(true)
                .truncate(rewrite)
                .open(&output)
                .unwrap(),
        )
    } else {
        Box::new(io::stdout())
    };

    if head {
        writeln!(file, "Address\tSegWit\tPrivate").unwrap();
    }

    loop {
        let keys = generate_keys();
        writeln!(file, "{}\t{}\t{}", keys.0, keys.1, keys.2).unwrap();
        not_mutex_print_progress!(k, start, Locale::fr, count);
    }

    print_force!(k, start, Locale::fr);
}
