use secp256k1::{
    rand::{rngs, SeedableRng},
    PublicKey, SecretKey,
};

//key generation (based on the rng variable)
pub fn generate_keypair() -> (SecretKey, PublicKey) {
    let secp = secp256k1::Secp256k1::new();
    let mut rng = rngs::StdRng::seed_from_u64(01234567890);
    secp.generate_keypair(&mut rng)
}
