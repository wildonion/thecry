


use std::fmt::Write;
use ring::{signature as ring_signature, rand as ring_rand};
use ring::signature::Ed25519KeyPair;
use once_cell::sync::Lazy;
use serde::{Deserialize, Serialize};
use ring::{signature::KeyPair, pkcs8::Document};
use wallexerr::misc::Wallet;
use std::io::BufWriter;
use std::str::FromStr;
use std::{fs::OpenOptions, io::BufReader};
use rand::random;

use crypter;
pub mod constants;
pub mod cry;

pub mod zkp;

/*
    if we want to use Result<(), impl std::error::Error + Send + Sync + 'static>
    as the return type of the error part, the exact error type instance must be 
    sepecified also the Error trait must be implemented for the error type (impl 
    Error for ErrorType{}) since we're implementing the Error trait for the error 
    type in return type which insists that the instance of the type implements the 
    Error trait. by returning a boxed error trait we're returning the Error trait 
    as a heap object behind a valid pointer which handles all error type at runtime, 
    this is the solution to return traits as an object cause we don't know what type 
    causes the error at runtiem and is the implementor of the Error trait which 
    forces us to return the trait as the error itself and since traits are dynamically
    sized we can't treat them as a typed object directly we must put them behind 
    pointer like &'valid dyn Trait or box them to send them on the heap, also by 
    bounding the Error trait to Send + Sync + 'static we'll make it sefable, sendable 
    and shareable to move it between different scopes and threads.
*/
#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error + Send + Sync + 'static>> {

    // cry::poison::aespaddingattack::thecry();

    let data = String::from("wildonion here guys");
    let mut wallet = Wallet::new_ed25519();
    let signature = cry::poison::wannacry::ed25519_with_aes_signing(&data, wallet);
    println!("base58 ed25519 signature >> {:?}", signature);
    
    //----------------------------------------------------------------------------------
    //---- file encryption using ed25519 wallet with aes256 themis secure cell signing
    //----------------------------------------------------------------------------------
    let mut encrypted = cry::poison::wannacry::encrypt_file("secret.txt").await;
    let decrypted = cry::poison::wannacry::decrypt_file("secret.txt.dec", &mut encrypted.1).await;


    Ok(())     


}
