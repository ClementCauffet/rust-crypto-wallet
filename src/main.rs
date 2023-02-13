use anyhow::Result;
mod eth_wallet;
mod utils;
use std::env;
use std::io;
use std::result;

use std::str::FromStr;
use web3::types::Address;

#[tokio::main]
async fn main() -> Result<()> {
    //Loading dotenv
    dotenv::dotenv().ok();
    let wallet_file_path = "crypto_wallet.json";
    let endpoint = env::var("INFURA_GORLI_WS")?;

    // let (seed_phrase, secret_key, pub_key) = eth_wallet::generate_keypair();

    // println!("24-words generated seed_phrase :");
    // for (index, word) in seed_phrase.iter().enumerate() {
    //     println!("{}: {}", index + 1, word);
    // }
    // println!("secret key: {}", &secret_key.to_string());
    // println!("public key: {}", &pub_key.to_string());

    // let pub_address = eth_wallet::public_key_address(&pub_key);
    // println!("public address: {:?}", pub_address);

    // let crypto_wallet = eth_wallet::Wallet::new(&seed_phrase, &secret_key, &pub_key);
    // println!("crypto_wallet: {:?}", &crypto_wallet);

    // crypto_wallet.save_to_file(wallet_file_path)?;

    let loaded_wallet = eth_wallet::Wallet::from_file(wallet_file_path)?;
    println!("loaded_wallet: {:?}", loaded_wallet);

    let web3_con = eth_wallet::establish_web3_connection(&endpoint).await?;

    let balance = loaded_wallet.get_balance_in_eth(&web3_con).await?;
    println!("wallet balance: {} eth", &balance);

    let transaction = eth_wallet::create_eth_transaction(
        Address::from_str("0x56aF1D1C733c2ce73b1d1513C749Aca24b471D1D")?,
        0.01,
    );
    let transact_hash =
        eth_wallet::sign_and_send(&web3_con, transaction, &loaded_wallet.get_secret_key()?).await?;
    println!("transaction hash: {:?}", transact_hash);

    // let block_number = web3_con.eth().block_number().await?;
    // println!("[GORLI] block number: {}", &block_number);

    Ok(())
}
