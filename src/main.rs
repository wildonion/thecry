


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
    the error part of the result type is a pointer to a std Error trait 
    bounded to Send, Sync and 'static lifetime to be shareable between
    threads, the reason is actually we don't know the type that will cause
    the error at runtime and in order to handle the error for that type
    we must bound the type to this trait at runtime which can handle all
    the possible errors of the type, also since traits are not fix sized 
    and they're on the heap we must put them behind a pointer like &dyn Trait
    with a valid lifetime or inside the Box like Box<dyn Trait> which has 
    its own valid lifetime.
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
