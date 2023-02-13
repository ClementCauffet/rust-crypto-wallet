use anyhow::Result;
mod eth_wallet;

fn main() -> Result<()> {
    let (seed_phrase, secret_key, pub_key) = eth_wallet::generate_keypair();

    println!("24-words generated seed_phrase :");
    for (index, word) in seed_phrase.iter().enumerate() {
        println!("{}: {}", index + 1, word);
    }
    println!("secret key: {}", &secret_key.to_string());
    println!("public key: {}", &pub_key.to_string());

    let pub_address = eth_wallet::public_key_address(&pub_key);
    println!("public address: {:?}", pub_address);

    let crypto_wallet = eth_wallet::Wallet::new(&seed_phrase, &secret_key, &pub_key);
    println!("crypto_wallet: {:?}", &crypto_wallet);

    let wallet_file_path = "crypto_wallet.json";
    crypto_wallet.save_to_file(wallet_file_path)?;

    let loaded_wallet = eth_wallet::Wallet::from_file(wallet_file_path)?;
    println!("loaded_wallet: {:?}", loaded_wallet);

    Ok(())
}
