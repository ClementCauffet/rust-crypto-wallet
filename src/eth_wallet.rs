//Error handling helper
use crate::utils;
use anyhow::Result;
use rand::seq::SliceRandom;
use rand::thread_rng;
use std::fs::File;

//ellipitc curve
use secp256k1::{
    rand::{rngs, SeedableRng},
    PublicKey, SecretKey,
};

//Reading from -Writing to a file
use serde::{Deserialize, Serialize};
use std::collections::hash_map::DefaultHasher;
use std::hash::{Hash, Hasher};
use std::io::BufRead;
use std::io::BufWriter;
use std::{fs::OpenOptions, io::BufReader};
use tiny_keccak::keccak256;
use web3::transports::WebSocket;

use web3::{
    transports,
    types::{Address, TransactionParameters, H256, U256},
    Web3,
};

pub fn choose_words(filename: &str) -> Vec<String> {
    let file = File::open(filename).unwrap();
    let reader = BufReader::new(file);
    let mut words: Vec<String> = reader.lines().map(|line| line.unwrap()).collect();

    let mut rng = thread_rng();
    words.shuffle(&mut rng);

    words.into_iter().take(24).collect()
}

//key generation (based on the rng variable)
pub fn generate_keypair() -> (Vec<String>, SecretKey, PublicKey) {
    // Initialisation de la bibliothèque secp256k1
    let secp = secp256k1::Secp256k1::new();
    let chosen_words = choose_words("words.txt");

    // Hashing the Vector to use with the seed with the elliptic curve
    let mut hasher = DefaultHasher::new();

    chosen_words.hash(&mut hasher);

    let seed_integer = hasher.finish();

    // Seed generation
    let mut rng = rngs::StdRng::seed_from_u64(seed_integer);

    // Key genenation
    let (secret_key, public_key) = secp.generate_keypair(&mut rng);

    //let seed_phrase = chosen_words.join(" ");

    (chosen_words, secret_key, public_key)
}

//key generation (based on the rng variable)
pub fn generate_keypair_from_seed(
    recovery_phrase: Vec<&str>,
) -> (Vec<String>, SecretKey, PublicKey) {
    // Initialisation de la bibliothèque secp256k1
    let secp = secp256k1::Secp256k1::new();

    // Hashing the Vector to use with the seed with the elliptic curve
    let mut hasher = DefaultHasher::new();

    let vec_string: Vec<String> = recovery_phrase.iter().map(|&s| s.to_string()).collect();
    vec_string.hash(&mut hasher);

    let seed_integer = hasher.finish();

    // Seed generation
    let mut rng = rngs::StdRng::seed_from_u64(seed_integer);

    // Key genenation
    let (secret_key, public_key) = secp.generate_keypair(&mut rng);

    (vec_string, secret_key, public_key)
}

//derivating public address from public key
pub fn public_key_address(public_key: &PublicKey) -> Address {
    let public_key = public_key.serialize_uncompressed();

    debug_assert_eq!(public_key[0], 0x04);
    let hash = keccak256(&public_key[1..]);

    Address::from_slice(&hash[12..])
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Wallet {
    pub seed_phrase: Vec<String>,
    pub secret_key: String,
    pub public_key: String,
    pub public_address: String,
}

impl Wallet {
    pub fn new(seed_phrase: &Vec<String>, secret_key: &SecretKey, public_key: &PublicKey) -> Self {
        let addr: Address = public_key_address(&public_key);
        Wallet {
            seed_phrase: seed_phrase.to_vec(),
            secret_key: secret_key.to_string(),
            public_key: public_key.to_string(),
            public_address: format!("{:?}", addr),
        }
    }

    pub fn save_to_file(&self, file_path: &str) -> Result<()> {
        let file = OpenOptions::new()
            .write(true)
            .create(true)
            .truncate(true)
            .open(file_path)?;
        let buf_writer = BufWriter::new(file);
        serde_json::to_writer_pretty(buf_writer, self)?;
        Ok(())
    }

    pub fn from_file(file_path: &str) -> Result<Wallet> {
        let file = OpenOptions::new().read(true).open(file_path)?;
        let buf_reader = BufReader::new(file);

        let wallet: Wallet = serde_json::from_reader(buf_reader)?;
        Ok(wallet)
    }

    pub fn get_secret_key(&self) -> Result<SecretKey> {
        let secret_key = (&self.secret_key).parse().unwrap();
        Ok(secret_key)
    }

    // pub fn get_public_key(&self) -> Result<PublicKey> {
    //     let pub_key = (&self.public_key).parse().unwrap();
    //     Ok(pub_key)
    // }

    pub async fn get_balance(&self, web3_connection: &Web3<transports::WebSocket>) -> Result<U256> {
        let wallet_address = (&self.public_address).parse().unwrap();
        let balance = web3_connection.eth().balance(wallet_address, None).await?;

        Ok(balance)
    }

    pub async fn get_balance_in_eth(
        &self,
        web3_connection: &Web3<transports::WebSocket>,
    ) -> Result<f64> {
        let wei_balance = self.get_balance(web3_connection).await?;
        Ok(utils::wei_to_eth(wei_balance))
    }
}

//connection to websocket&
pub async fn establish_web3_connection(url: &str) -> Result<Web3<WebSocket>> {
    let transport = transports::WebSocket::new(url).await?;
    Ok(Web3::new(transport))
}

pub fn create_eth_transaction(to: Address, eth_value: f64) -> TransactionParameters {
    TransactionParameters {
        to: Some(to),
        value: utils::eth_to_wei(eth_value),
        ..Default::default()
    }
}

//sending transactions
pub async fn sign_and_send(
    web3: &Web3<transports::WebSocket>,
    transaction: TransactionParameters,
    secret_key: &SecretKey,
) -> Result<H256> {
    let signed = web3
        .accounts()
        .sign_transaction(transaction, secret_key)
        .await?;

    let transaction_result = web3
        .eth()
        .send_raw_transaction(signed.raw_transaction)
        .await?;
    Ok(transaction_result)
}
