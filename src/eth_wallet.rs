//Error handling helper
use anyhow::{bail, Result};
use rand::seq::SliceRandom;
use rand::thread_rng;
use std::fs::File;

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
use std::str::FromStr;
use std::{fs::OpenOptions, io::BufReader};
use tiny_keccak::keccak256;
use web3::types::Address;

pub fn choose_words(filename: &str) -> Vec<String> {
    let file = File::open(filename).unwrap();
    let reader = BufReader::new(file);
    let mut words: Vec<String> = reader.lines().map(|line| line.unwrap()).collect();

    let mut rng = thread_rng();
    words.shuffle(&mut rng);

    words.into_iter().take(24).collect()
}

//key generation (based on the rng variable)
pub fn generate_keypair() -> (SecretKey, PublicKey) {
    // Initialisation de la bibliothèque secp256k1
    let secp = secp256k1::Secp256k1::new();
    let chosen_words = choose_words("words.txt");
    for (index, word) in chosen_words.iter().enumerate() {
        println!("{}: {}", index + 1, word);
    }
    // Hachage du vecteur de mots pour obtenir un entier à utiliser pour générer la graine aléatoire
    let mut hasher = DefaultHasher::new();
    chosen_words.hash(&mut hasher);
    let seed_integer = hasher.finish();

    // Génération de la graine aléatoire à partir de l'entier haché
    let mut rng = rngs::StdRng::seed_from_u64(seed_integer);

    // Génération de la clé à partir de la graine aléatoire
    secp.generate_keypair(&mut rng)
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
    pub secret_key: String,
    pub public_key: String,
    pub public_address: String,
}

impl Wallet {
    pub fn new(secret_key: &SecretKey, public_key: &PublicKey) -> Self {
        let addr: Address = public_key_address(&public_key);
        Wallet {
            secret_key: secret_key.to_string(),
            public_key: public_key.to_string(),
            public_address: format!("{:?}", addr),
        }
    }

    pub fn save_to_file(&self, file_path: &str) -> Result<()> {
        let file = OpenOptions::new()
            .write(true)
            .create(true)
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
        let secret_key = SecretKey::from_str(&self.secret_key)?;
        Ok(secret_key)
    }
    pub fn get_public_key(&self) -> Result<PublicKey> {
        let pub_key = PublicKey::from_str(&self.public_key)?;
        Ok(pub_key)
    }
}
