use anyhow::Result;
mod eth_wallet;
mod utils;
use regex::Regex;
use std::env;
use std::io;

use std::str::FromStr;
use web3::types::Address;

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
                Ok(_) => println!("\nSuccessfully found balance for current adress !"),
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
        println!("Wallet 24-words : {:?} ", loaded_wallet.seed_phrase);
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
        // Convertit le montant en type f64
        let amount: f64 = amount_str.trim().parse().expect("Invalid amount");

        let loaded_wallet = eth_wallet::Wallet::from_file(wallet_file_path)?;

        let web3_con = eth_wallet::establish_web3_connection(&endpoint).await?;

        let balance = loaded_wallet.get_balance_in_eth(&web3_con).await?;

        if balance > amount {
            let transaction =
                eth_wallet::create_eth_transaction(Address::from_str(&address)?, amount);

            //Address::from_str("---- Any Wallet you want to send funds to (0x) ---- ")?,
            let transact_hash =
                eth_wallet::sign_and_send(&web3_con, transaction, &loaded_wallet.get_secret_key()?)
                    .await?;
            println!("transaction hash: {:?}", transact_hash);
        } else {
            println!("Insufficient funds on your wallet ");
        };

        Ok(())
    }

    println!("\n Looking forward ! - @ClementCauffet");

    Ok(())
}
