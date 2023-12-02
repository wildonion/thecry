

use ring::pkcs8::Document;
use crate::*;
use ring::{signature as ring_signature, rand as ring_rand};
use once_cell::sync::Lazy;
use rand::Rng;
use rand::random;


pub static CHARS: &[u8] = b"abcdefghijklmnopqrstuvwxyzABCDEFGHIJKLMNOPQRSTUVWXYZ0123456789";

pub fn gen_random_chars(size: u32) -> String{
    let mut rng = rand::thread_rng();
    (0..size).map(|_|{
        /* converint the generated random ascii to char */
        char::from_u32(rng.gen_range(33..126)).unwrap() // generating a char from the random output of type u32 using from_u32() method
    }).collect()
}

pub fn gen_random_number(from: u32, to: u32) -> u32{
    let mut rng = rand::thread_rng(); // we can't share this between threads and across .awaits
    rng.gen_range(from..to)
} 

pub fn gen_random_idx(idx: usize) -> usize{
    if idx < CHARS.len(){
        idx
    } else{
        gen_random_idx(random::<u8>() as usize)
    }
}


/* converting an slice array of u8 bytes into an array with 32 byte length */
fn convert_into_u8_32(data: &[u8]) -> Option<[u8; 32]>{
    data.try_into().ok()
}

pub fn string_to_static_str(s: String) -> &'static str { 
    /* 
        we cannot obtain &'static str from a String because Strings may not live 
        for the entire life of our program, and that's what &'static lifetime means. 
        we can only get a slice parameterized by String own lifetime from it, we can 
        obtain a static str but it involves leaking the memory of the String. this is 
        not something we should do lightly, by leaking the memory of the String, this 
        guarantees that the memory will never be freed (thus the leak), therefore, any 
        references to the inner object can be interpreted as having the 'static lifetime.
        
        also here it's ok to return the reference from function since our reference lifetime 
        is static and is valid for the entire life of the app

        leaking the memory of the heap data String which allows us to have an 
        unfreed allocation that can be used to define static str using it since
        static means we have static lifetime during the whole lifetime of the app
        and reaching this using String is not possible because heap data types 
        will be dropped from the heap once their lifetime destroyed in a scope
        like by moving them into another scope hence they can't be live longer 
        than static lifetime

        Note: this will leak memory! the memory for the String will not be freed 
        for the remainder of the program. Use this sparingly
    */
    Box::leak(s.into_boxed_str()) 

}

/* 
    we cannot obtain &'static str from a Vec because Vecs may not live 
    for the entire life of our program, and that's what &'static lifetime means. 
    we can only get a slice parameterized by Vec own lifetime from it, we can 
    obtain a static str but it involves leaking the memory of the Vec. this is 
    not something we should do lightly, by leaking the memory of the Vec, this 
    guarantees that the memory will never be freed (thus the leak), therefore, any 
    references to the inner object can be interpreted as having the 'static lifetime.
    
    also here it's ok to return the reference from function since our reference lifetime 
    is static and is valid for the entire life of the app
*/
pub fn vector_to_static_slice(s: Vec<u32>) -> &'static [u32] { 
    /* 
        leaking the memory of the heap data Vec which allows us to have an 
        unfreed allocation that can be used to define static str using it since
        static means we have static lifetime during the whole lifetime of the app
        and reaching this using Vec is not possible because heap data types 
        will be dropped from the heap once their lifetime destroyed in a scope
        like by moving them into another scope hence they can't be live longer 
        than static lifetime

        Note: this will leak memory! the memory for the Vec will not be freed 
        for the remainder of the program. Use this sparingly
    */
    Box::leak(s.into_boxed_slice()) 
}