use secp256k1::{
    rand::{rngs, SeedableRng},
    PublicKey, SecretKey,
};

use tiny_keccak::keccak256;
use web3::types::Address;

//key generation (based on the rng variable)
pub fn generate_keypair() -> (SecretKey, PublicKey) {
    let secp = secp256k1::Secp256k1::new();
    let mut rng = rngs::StdRng::seed_from_u64(0123);
    secp.generate_keypair(&mut rng)
}

//derivating public address from public key
pub fn public_key_address(public_key: &PublicKey) -> Address {
    let public_key = public_key.serialize_uncompressed();

    debug_assert_eq!(public_key[0], 0x04);
    let hash = keccak256(&public_key[1..]);

    Address::from_slice(&hash[12..])
}
