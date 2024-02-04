


use crate::*;


pub mod zkp{

    pub use super::*;

    pub struct ZkpError;
    pub struct Verifier;
    pub struct Prover;

    pub async fn auth() -> Result<(), ZkpError>{

        Ok(())
    }

    // https://noir-lang.org/index.html
    // https://github.com/rust-cc/awesome-cryptography-rust#zero-knowledge-proofs

}