use bitcoin::secp256k1::{rand, Secp256k1};
use bitcoin::{Address, Network, PrivateKey, PublicKey};

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
