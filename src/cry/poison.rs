





use crate::*;
use self::constants::{gen_random_number, gen_random_idx, CHARS};
use std::process::Command;
use std::io::Write;
use std::io::Read;


/* 
    shellcode ram injection, the shellcode is the hex code of the 
    compiled code that can be dumpped using objdump 
*/
pub mod mem_poisoning{

    use super::*;

    /* 
        > every two chars in hex is one byte thus for example 64 chras in hex is 32 bytes
        > shellcode must be in form `488d35140000006a01586a0c5a4889c70f056a3c5831ff0f05ebfe68656c6c6f20776f726c640a` 
            inside the `shellcode.bin`
        > shellcode is the `.data` section of the compile code into the `asm` language which 
            must be injected into `.text` section of the memory 
    */

    // pub const SHELLCODE_BYTES: &[u8] = include_bytes!("shellcode.bin"); //// includes a file as a reference to a byte array of a binary file in form &[u8], we're loading the bin file into an slice of utf8 bytes
    // pub const SHELLCODE_LENGTH: usize = SHELLCODE_BYTES.len();
    // //// DEP (Data Execution Prevention) prevents code from being run from data pages such as the default heap, stacks, and memory pools, 
    // ///      if an application attempts to run code from a data page that is protected, a memory access violation exception occurs, 
    // //       and if the exception is not handled, the calling process is terminated.
    // //// shellcodes might be in non executable section inside the memory 
    // //// dereferencing requires known size thus we must dereference the loaded shellcode int [u8; SHELLCODE_LENGTH]
    // //// we must dereference the &[u8] shellcode to inject the buffer itself otherwise the reference of the buffer will be injected  
    // #[no_mangle]
    // #[link_section=".text"] //// means we're executing the shellcode inside the .text section of the memory
    // pub static SHELLCODE: [u8; SHELLCODE_LENGTH] = *include_bytes!("shellcode.bin"); //// includes a file as a reference to a byte array of a binary file thus we must dereference it in order to coerce it into [u8] since it returns &[u8]




    // pub fn inject(){
    //     //// the equivalent of () in C is *const ()
    //     //// following unsafe block will return 
    //     //// a pointer to a function with C ABI
    //     //// which is `*const ()`, also:
    //     //// - *const () is the C function pointer 
    //     //// - fn is the rust function pointer 
    //     //// - () is the empty type in rust 
    //     //// - fn bar(x: i32) {} is just a function not a function pointer
    //     //// - casting bar to a function pointer: let bar_ptr: fn(i32) = bar;  
    //     let exec_shellcode: extern "C" fn() -> () = unsafe{ //// the type of exec_shellcode is a C function pointer which will return nothing; since everything in rust must have a specific size thus the compiler cannot predict what memory address the () would be associated with at execution time
    //         std::mem::transmute(&SHELLCODE as *const [u8] as *const ()) //// it copies the bits from the source value into the destination value; in our case we're transmutting the shellcode [u8] buffer into a C function pointer which is () in rust so we can call it later to execute it
    //     };
    //     exec_shellcode();

    // }

}


pub mod wannacry{

    /* 

        ransomewere, steganography and files encryption
        
        encrypting using aes256 cbc with pbkdf2 and salt then showing key and iv
                    
        openssl aes-256-cbc -a -salt -pbkdf2 -in secrets.txt -out secrets.txt.enc -p
        openssl aes-256-cbc -d -a -pbkdf2 -in secrets.txt.enc -out secrets.txt.new -p 

        gpg --output encrypted.data --symmetric --cipher-algo AES256 un_encrypted.data
        gpg --output un_encrypted.data --decrypt encrypted.data
    
    */
}


pub mod decryptaes256{


    pub use super::*;

    fn gen_pswd() -> String{

        (0..13)
            .map(|n|{
                let idx = gen_random_idx(random::<u8>() as usize);
                CHARS[idx] as char
            })
            .collect::<String>()

    }

    fn gen_kdf(password: &str) -> Vec<u8>{


        let mut hasher = sha2::Sha384::new();
        hasher.update(password);
        let pswdsha384: &[u8] = &hasher.finalize();

        pswdsha384.to_vec()

    }

    pub fn thecry(){
        
        println!("current dir {}", std::env::current_dir().unwrap().display());
        let mut enc_file = std::fs::File::open("sec.json").unwrap();
        let mut buffer = Vec::new();
        let vector_length = enc_file.read_to_end(&mut buffer).unwrap();


        // --------------------------------------------------------------------------------
        // https://crypto.stackexchange.com/questions/72658/how-do-i-detect-a-failed-aes-256-decryption-programmatically/79948#79948
        /*  

            imagine the ciphertext for the last block is 5307f7afffa3798f386e7c6c144c6a9c, 
            and the ciphertext for the previous block is 3b2364d1d04a35c8081bbc6fdeacbd86. 
            this is equivalent to decrypting one block of ciphertext 5307f7afffa3798f386e7c6c144c6a9c, 
            using an iv of 3b2364d1d04a35c8081bbc6fdeacbd86, also aes requires the length 
            of each cipherblock to be 16 bytes
        */
        
        /* this is initialization vector that is going to be used as the iv */
        let block_cipher_1_as_iv = &buffer[0..16];
        
        /* this is the cipherblock that is going to be used to decrypt */
        let first_block_cipher = &buffer[16..32];

        /* converting each cipherblock into hex string */
        let hex_first_block_cipher = hex::encode(first_block_cipher);
        let hex_block_cipher_1_as_iv = hex::encode(block_cipher_1_as_iv);
        // --------------------------------------------------------------------------------

        let mut log = std::fs::File::create("decrypt.log").unwrap();
        let mut used_passwd = Vec::new();
        for i in 0..1000{

            let pass = gen_pswd();
            let secret = gen_kdf(pass.as_str());
            
            if used_passwd.contains(&pass){
                continue;
            }
            used_passwd.push(pass.clone());

            /* we can use the first 32 bytes as the key and the rest as the iv */
            let key = &secret.as_slice()[..32];
            let iv = &secret.as_slice()[32..];

            let hex_key = hex::encode(key);
            let hex_iv = hex::encode(iv);

            /* creating a 32 bytes iv from the iv using try_from() method */
            let new_iv = <[u8; 32]>::try_from(iv).unwrap();

            /* decrypting process */
            let shell_command = Command::new("./decry.sh")
                .arg(hex_first_block_cipher.as_str())
                .arg(hex_block_cipher_1_as_iv.as_str())
                .arg(hex_key)
                .output()
                .unwrap();

            if shell_command.status.success(){
                let output_str = std::str::from_utf8(&shell_command.stdout).unwrap();
                let decrypted_cipher = &output_str[10..];
                println!("❌ cipherblock [{hex_first_block_cipher:}] | key [{pass:}] | iv [{hex_block_cipher_1_as_iv:}] : {decrypted_cipher:}");
                let content = format!("❌ cipherblock [{hex_first_block_cipher:}] | key [{pass:}] | iv [{hex_block_cipher_1_as_iv:}] : {decrypted_cipher:}");
                log.write(content.as_bytes()).unwrap();
                // println!("✅ valid padding {}", output_str);
            }

        }

    }


}