mod eth_wallet;
fn main() {
    let (secret_key, pub_key) = eth_wallet::generate_keypair();
    println!("secret key: {}", &secret_key.to_string());
    println!("public key: {}", &pub_key.to_string());

    let pub_address = eth_wallet::public_key_address(&pub_key);
    println!("public address: {:?}", pub_address);

    let crypto_wallet = eth_wallet::Wallet::new(&secret_key, &pub_key);
    println!("crypto_wallet: {:?}", &crypto_wallet);
}
