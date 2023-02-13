use anyhow::Result;
mod eth_wallet;
use std::env;

#[tokio::main]
async fn main() -> Result<()> {
    //Loading dotenv
    dotenv::dotenv().ok();

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

    let endpoint = env::var("INFURA_GORLI_WS")?;
    let web3_con = eth_wallet::establish_web3_connection(&endpoint).await?;

    let block_number = web3_con.eth().block_number().await?;
    println!("[GORLI] block number: {}", &block_number);

    Ok(())
}
