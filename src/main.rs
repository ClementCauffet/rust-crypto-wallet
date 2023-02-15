use anyhow::Result;
mod eth_wallet;
mod utils;
use regex::Regex;
use serde::{Deserialize, Serialize};
use std::env;
use std::fs::File;
use std::io;
use std::io::Read;

use std::str::FromStr;
use web3::types::Address;

fn create_wallet(wallet_file_path: &str) {
    let (seed_phrase, secret_key, pub_key) = eth_wallet::generate_keypair();

    println!("24-words generated seed_phrase :");
    for (index, word) in seed_phrase.iter().enumerate() {
        println!("{}: {}", index + 1, word);
    }
    println!("\nsecret key: {}", &secret_key.to_string());
    println!("public key: {}", &pub_key.to_string());

    let pub_address = eth_wallet::public_key_address(&pub_key);
    println!("public address: {:?}", pub_address);

    let crypto_wallet = eth_wallet::Wallet::new(&seed_phrase, &secret_key, &pub_key);

    match crypto_wallet.save_to_file(wallet_file_path) {
        Err(e) => println!("{:?}", e),
        _ => println!("\nWallet saved to 'crypto_wallet.json' successfully !"),
    }
}

fn load_wallet(wallet_file_path: &str) {
    let mut seed = String::new();
    println!("Paste or type your seed_phrase :");

    io::stdin().read_line(&mut seed).unwrap();

    let re = Regex::new(r"\b\w+\b").unwrap();
    let recovery_phrase: Vec<&str> = re.find_iter(&seed).map(|m| m.as_str()).collect();

    println!("\nSeed input : {:?}", recovery_phrase);
    println!("\nGenerating wallet info based on seed input ... ");

    let (seed_reference, secret_key, pub_key) =
        eth_wallet::generate_keypair_from_seed(recovery_phrase);

    println!("\nsecret key: {}", &secret_key.to_string());
    println!("public key: {}", &pub_key.to_string());

    let pub_address = eth_wallet::public_key_address(&pub_key);
    println!("public address: {:?}", pub_address);

    let crypto_wallet = eth_wallet::Wallet::new(&seed_reference, &secret_key, &pub_key);

    match crypto_wallet.save_to_file(wallet_file_path) {
        Err(e) => println!("{:?}", e),
        _ => println!("\nWallet saved to 'crypto_wallet.json' successfully !"),
    }
}

async fn get_balance(
    wallet_file_path: &str,
    endpoint: &str,
) -> Result<(), Box<dyn std::error::Error>> {
    let loaded_wallet = eth_wallet::Wallet::from_file(wallet_file_path)?;

    let web3_con = eth_wallet::establish_web3_connection(&endpoint).await?;

    let balance = loaded_wallet.get_balance_in_eth(&web3_con).await?;
    println!(
        "\n[GOERLI] wallet balance for {} : {} eth",
        &loaded_wallet.public_address, &balance
    );

    Ok(())
}

async fn get_wallet_info(wallet_file_path: &str) -> Result<(), Box<dyn std::error::Error>> {
    let loaded_wallet = eth_wallet::Wallet::from_file(wallet_file_path)?;
    println!("\nWallet 24-words : {:?} ", loaded_wallet.seed_phrase);
    println!("Wallet secret key : {} ", loaded_wallet.secret_key);
    println!("Wallet public key : {} ", loaded_wallet.public_key);
    println!("Wallet public address : {} ", loaded_wallet.public_address);

    Ok(())
}

async fn send_transaction(
    wallet_file_path: &str,
    endpoint: &str,
) -> Result<(), Box<dyn std::error::Error>> {
    println!("Recipient's Ethereum address  :");
    let mut address = String::new();
    io::stdin()
        .read_line(&mut address)
        .expect("Error while reading address");

    println!("Amount to send (float) :");
    let mut amount_str = String::new();
    io::stdin()
        .read_line(&mut amount_str)
        .expect("Error while reading address");

    // Parse to f64S
    let amount: f64 = amount_str.trim().parse().expect("Invalid amount");

    let loaded_wallet = eth_wallet::Wallet::from_file(wallet_file_path)?;

    let web3_con = eth_wallet::establish_web3_connection(&endpoint).await?;

    let balance = loaded_wallet.get_balance_in_eth(&web3_con).await?;

    if balance > amount {
        let transaction = eth_wallet::create_eth_transaction(Address::from_str(&address)?, amount);

        let transact_hash =
            eth_wallet::sign_and_send(&web3_con, transaction, &loaded_wallet.get_secret_key()?)
                .await?;
        println!("transaction hash: {:?}", transact_hash);
    } else {
        println!("Insufficient funds on your wallet ");
    };

    Ok(())
}

#[tokio::main]
async fn main() -> Result<()> {
    //Loading dotenv
    dotenv::dotenv().ok();
    let wallet_file_path = "crypto_wallet.json";
    let endpoint = env::var("INFURA_GOERLI_WS")?;

    loop {
        println!("\n ---------- Choose an option : ---------- ");
        println!("1. Create a wallet");
        println!("2. Load a wallet using your 24 words");
        println!("3. Display wallet balance");
        println!("4. Display wallet info");
        println!("5. Send ETH");
        println!("6. Quit");

        let mut choix = String::new();

        io::stdin()
            .read_line(&mut choix)
            .expect("Erreur de lecture de la ligne");

        let choix: u32 = match choix.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };

        match choix {
            1 => {
                println!("Are you sure ? All data for this wallet will be erased");
                println!("1. Yes");
                println!("2. No");

                let mut confirmation = String::new();

                io::stdin()
                    .read_line(&mut confirmation)
                    .expect("Error while reading the input");

                let confirmation: u32 = match confirmation.trim().parse() {
                    Ok(num) => num,
                    Err(_) => continue,
                };

                match confirmation {
                    1 => create_wallet(wallet_file_path),
                    2 => println!("Cancelling wallet creation"),
                    _ => println!("Please enter a valid input"),
                }
            }
            2 => load_wallet(wallet_file_path),
            3 => match get_balance(wallet_file_path, &endpoint).await {
                Ok(_) => println!("\nSuccessfully found balance for current adress !"),
                Err(e) => println!("Error retrieving balance: {:?}", e),
            },
            4 => match get_wallet_info(wallet_file_path).await {
                Ok(_) => println!("\nSuccessfully found info for current wallet !"),
                Err(e) => println!("Error retrieving balance: {:?}", e),
            },

            5 => match send_transaction(wallet_file_path, &endpoint).await {
                Ok(_) => println!("\nSuccessfully sent transaction !"),
                Err(e) => println!("Error sending transaction: {:?}", e),
            },
            6 => break,
            _ => println!("Please enter a valid option"),
        }
    }

    println!("\n Looking forward ! - @ClementCauffet");

    Ok(())
}

#[derive(Serialize, Deserialize)]
struct WalletData {
    seed_phrase: Vec<String>,
    secret_key: String,
    public_key: String,
    public_address: String,
}

//Tests
#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_create_wallet() {
        let wallet_file_path = "test_wallet.json";

        // Call the function being tested
        create_wallet(wallet_file_path);

        // Check that the wallet file was created
        let wallet_file = File::open(wallet_file_path);
        assert!(wallet_file.is_ok(), "Wallet file was not created");

        let mut file = File::open(wallet_file_path).unwrap();
        let mut contents = String::new();
        file.read_to_string(&mut contents).unwrap();

        // Deserialize wallet data into a WalletData strut type
        let my_data: WalletData = serde_json::from_str(&contents).unwrap();

        // Check if fields have the same length as expected (adress / keys must have the same format all the time -> this test will fail if user input a 12 words seed_phrase but we spectified 24)
        let num_seed_phrases = my_data.seed_phrase.len();
        let num_secret_key_chars = my_data.secret_key.len();
        let num_public_key_chars = my_data.public_key.len();
        let num_public_address_chars = my_data.public_address.len();

        assert_eq!(num_seed_phrases, 24);
        assert_eq!(num_secret_key_chars, 64);
        assert_eq!(num_public_key_chars, 66);
        assert_eq!(num_public_address_chars, 42);

        // Clean up test file
        std::fs::remove_file(wallet_file_path).unwrap();
    }

    #[test]
    fn test_load_wallet() {
        let recovery_phrase = vec![
            "test", "test", "test", "test", "test", "test", "test", "test", "test", "test", "test",
            "test", "test", "test", "test", "test", "test", "test", "test", "test", "test", "test",
            "test", "test",
        ];
        let (seed_reference, secret_key, pub_key) =
            eth_wallet::generate_keypair_from_seed(recovery_phrase);

        let crypto_wallet = eth_wallet::Wallet::new(&seed_reference, &secret_key, &pub_key);

        let wallet_file_path = "test_wallet.json";

        match crypto_wallet.save_to_file(wallet_file_path) {
            Err(e) => println!("{:?}", e),
            _ => println!("\nWallet saved to 'test_wallet.json' successfully !"),
        }

        // Check if the wallet file was created
        let file = std::fs::File::open(&wallet_file_path);
        assert!(file.is_ok(), "Wallet file not created");

        // Check if the wallet file has the correct content
        let wallet_json: serde_json::Value =
            serde_json::from_reader(file.unwrap()).expect("Failed to parse wallet file");

        assert_eq!(
            wallet_json["seed_phrase"].as_array().unwrap().len(),
            24,
            "Invalid seed phrase length"
        );

        assert_eq!(
            wallet_json["secret_key"].as_str().unwrap().len(),
            64,
            "Invalid secret key length"
        );

        assert_eq!(
            wallet_json["public_key"].as_str().unwrap().len(),
            66,
            "Invalid public key length"
        );

        assert_eq!(
            wallet_json["public_address"].as_str().unwrap().len(),
            42,
            "Invalid public address length"
        );

        // Delete the temporary file
        std::fs::remove_file(&wallet_file_path).unwrap();
    }
}
