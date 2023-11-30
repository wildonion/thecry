





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

    /* e2e and file and msg encryption using aes256 bits

        tools: RSA ed25519 ECC curve with aes256 hash in wallexerr, openssl and ring for RSA + KDF like sha256 and keccak256
        ransomewere, steganography and files encryption to generate unique assets by encrypting using 
        aes256 cbc with pbkdf2 + sha384 + salt then showing key and iv like:
                    
            openssl aes-256-cbc -a -salt -pbkdf2 -in img.png -out img.png.enc -p
            openssl aes-256-cbc -d -a -pbkdf2 -in img.png.enc -out img.png.new -p 

            openssl aes-256-cbc -md sha384 -in secret.txt -out img.png.enc -p
            openssl aes-256-cbc -d -nopad -md sha384 -in img.png.enc -p

            gpg --output encrypted.data --symmetric --cipher-algo AES256 un_encrypted.data
            gpg --output un_encrypted.data --decrypt encrypted.data
    
    */
}


pub mod aespaddingattack{


    use std::collections::HashMap;

    pub struct Lock<'d>{
        pub data: std::rc::Rc<std::cell::RefCell<&'d [u8]>> /* single thread ownership sharing */
    } 

    pub use super::*;
    pub const ITERS: usize = 1000;
    pub const BLOCKSIZE: usize = 16;


    fn check_padding(decrypted_cipher_hex: &str) -> bool{

        /* 
            PKCS#7 is a common padding scheme where the value of each padding 
            byte is the number of padding bytes. So, if you're one byte short, 
            you add 0x01. if you're two bytes short, you add 0x02, 0x02, and so on.
        */
        let dec_cipher = decrypted_cipher_hex.replace(" ", "");
        let bytes = hex::decode(dec_cipher).unwrap();

        /* 
            slicing needs to borrow the vector since [u8] 
            doesn't have fixed size at compiletime 
        */
        let block = &bytes[0..BLOCKSIZE]; /* every block has 16 bytes length */
        
        let mut map = HashMap::<u8, u8>::new();
        for bidx in 0..block.len(){
            if bidx == 0{
                continue;
            }
            
            /* mapping between the ascii byte and its repetition */
            map.entry(block[bidx])
                .and_modify(|c| *c+=1)
                .or_insert(1);

        }

       for (k, v) in map{
            
            if k == v{
                let last_v_bytes = &block[BLOCKSIZE-v as usize..BLOCKSIZE];
                if last_v_bytes.iter().all(|x| x == &v){
                    return true
                }
            } else{
                return false;
            }
       }

        false

    }

    fn gen_pswd() -> String{

        (0..13)
            .map(|n|{
                let idx = gen_random_idx(random::<u8>() as usize);
                CHARS[idx] as char
            })
            .collect::<String>()

    }

    fn gen_kdf(password: &str) -> Vec<u8>{

        // echo -n '03261985' | sha384sum
        use sha2::Digest;
        let mut hasher = sha2::Sha384::new();
        hasher.update(password);
        let pswdsha384: &[u8] = &hasher.finalize();

        pswdsha384.to_vec()

    }

    pub fn thecry(){
        
        // --------------------------------------------------------------------------------
        /* 
            https://crypto.stackexchange.com/questions/72658/how-do-i-detect-a-failed-aes-256-decryption-programmatically/79948#79948
            https://github.com/flast101/padding-oracle-attack-explained
            https://github.com/mpgn/Padding-oracle-attack
        
            AES256 bits is a 32 bytes utf8 encoding which contains 32 hex chars 
            in its each block cipher so in hex file of an encrypted AES256 we have 
            rows contain 32 hex chars which is 16 bytes length so &buffer[0..16] 
            gives us the first row bytes of hex chars and by encoding it using 
            hex::encode() we'll take the 32 hex chars of the first row like 
            b7dbc950533d55faab8cf88561600cb3
        */
        println!("current dir {}", std::env::current_dir().unwrap().display());
        let mut enc_file = std::fs::File::open("src/cry/sec.json").unwrap();
        let mut buffer = Vec::new();
        let vector_length = enc_file.read_to_end(&mut buffer).unwrap();

        /* this is initialization vector that is going to be used as the iv */
        let block_cipher_1_as_iv = &buffer[16..32];
        
        /* this is the cipherblock that is going to be used to decrypt */
        let first_block_cipher = &buffer[32..48];

        /* converting each cipherblock into hex string */
        let hex_first_block_cipher = hex::encode(first_block_cipher);
        let hex_block_cipher_1_as_iv = hex::encode(block_cipher_1_as_iv);
        // --------------------------------------------------------------------------------

        let mut log = std::fs::File::create("src/cry/decrypt.log").unwrap();
        let mut passwords = (0..ITERS)
            .map(|_|{
                let pass = gen_pswd();
                pass
            })
            .collect::<Vec<String>>();

        passwords.push("RemSummer2023".to_string());

        for idx_pass in 0..ITERS + 1{ /* since the 1000 th element contains the last password */

            let pass = &passwords[idx_pass];
            let secret = gen_kdf(pass);

            /* we can use the first 32 bytes as the key and the rest as the iv which is 16 bytes */
            let key = &secret.as_slice()[..32];
            let iv = &secret.as_slice()[32..];
            let hex_key = hex::encode(key);
            let hex_iv = hex::encode(iv);

            /* ------------------------------------------------------------- */
            /* ------------------- ORACLE PADDING ATTACK ------------------- */
            /* ------------------------------------------------------------- */
            /* 
                    decompiling using ghidra to find
                        goal: find the key??
                        goal: find the iv??
                        goal: find kdf??
                        goal: find salt??

                if we want to mutate an slice we have to make sure that its 
                underlying data is a mutable pointer like &mut [u8] or &mut Vec<u8>
                cause slices are behind pointer by default thus to mutate them 
                we must have a mutable pointer to them

                I2 represents the "intermediate state" of the decryption for 
                    the second block of ciphertext. It is the state after running 
                    just the block cipher decryption but before the XOR with the 
                    preceding ciphertext block (or IV in the case of the first block).
                C1 represents the ciphertext block after the first block,  
                    in CBC mode, each block of plaintext is XORed with the 
                    previous ciphertext block before being encrypted, the 
                    very first block is XORed with an Initialization Vector (IV)
                C1' is a manipulated version of a ciphertext block, created randomly.
                C2 is the real ciphertext block that the attacker wants to decipher.
                C1' + C2 is just these blocks concatenated, which will be decrypted by the server.
                P2 represents the plaintext of the second block
                P2' is the block we're trying to fill its byte if the padding was 
                    correct to find the I2
                 
            */

            let c1_hex = String::from("b7dbc950533d55faab8cf88561600cb3");
            let mut c1_bytes_vec = hex::decode(c1_hex.clone()).unwrap();
            let c1_bytes_real = c1_bytes_vec.as_mut_slice(); /* creating a longer lifetime by converting the vector into mutable slice */
            
            /* C1' must be filled up with random bytes */
            let c1_chars = &mut constants::gen_random_chars((BLOCKSIZE-2) as u32); /* 0 up to 14th bytes are random */
            let c1_bytes_randomly = unsafe{ c1_chars.as_bytes_mut() }; /* accessing mutable bytes of string is unsafe */
            let c1_random_hex = hex::encode(&c1_bytes_randomly);

            let hex_key = hex_key.clone();
            let mut pbyte = 0x00;
            c1_bytes_randomly[BLOCKSIZE-1] = pbyte; /* setting the 15th index to 0x00 */
            let pblocks = &mut [0u8; 16]; /* pblocks is a mutable pointer to its underlying data which is [0u8; 16] which is the acutal plaintext bytes */
            let iblocks = &mut [0u8; 16]; /* pblocks is a mutable pointer to its underlying data which is [0u8; 16] which is the intermediate blocks */
            
            /* this is a cipherblock inside the encrypted file */
            let c2_hex = String::from("5814cbbccd968e0d2a72baf30bb06c70");
            let c2_bytes = hex::decode(&c2_hex).unwrap();
            
            for bidx in (BLOCKSIZE-1..0).rev(){    

                /* send C1' + C2 for decryption */
                c1_bytes_randomly.to_vec().extend_from_slice(&c2_bytes);
                let final_hex = hex::encode(&c1_bytes_randomly);

                let shell_command = Command::new("./decry.sh")
                    .current_dir("src/cry")
                    .arg(final_hex.as_str())
                    .arg(c1_hex.clone()) /* iv */
                    .arg(hex_key.clone())
                    .output()
                    .unwrap();

                if shell_command.status.success(){
                
                    let output_str = std::str::from_utf8(&shell_command.stdout).unwrap();
                    let decrypted_cipher = &output_str[10..49];
                    
                    let mut content = String::from("");
                    let is_correct_padding = check_padding(decrypted_cipher);
                    if is_correct_padding{

                        println!("✅ cipherblock [{hex_first_block_cipher:}] | key [{pass:}] | iv [{c1_hex:}] | dec => {decrypted_cipher:}");
                        content = format!("✅ cipherblock [{hex_first_block_cipher:}] | key [{pass:}] | iv [{c1_hex:}] | dec => {decrypted_cipher:}");

                        /* 
                            since we change the last byte of the C1' to be 0x00 thus if 
                            the padding is valid means that the last byte of the pblocks 
                            must be 0x01 because the last byte of plaintext is
                            padded correctly and since it's only 1 byte we set the last byte
                            of plaintext to 0x01

                            --------------
                            I2 = C1' ˆ P2' 
                            P2 = C1 ˆ I2
                            --------------

                            EXAMPLE FOR 15th BYTE
                                I2     = C1'     ^ P2'
                                I2[15] = C1'[15] ^ P2'[15]
                                       = 94      ^ 01
                                       = 95
                                
                                P2[15] = C1[15] ^ I2[15]
                                       = C1[15] ^ 95
                        */

                        iblocks[bidx] = c1_bytes_randomly[bidx] ^ (BLOCKSIZE - bidx) as u8;
                        pblocks[bidx] = c1_bytes_real[bidx] ^ iblocks[bidx];

                        /* set (bidx-2) bytes of the c1 cipherblock to be random bytes */
                        let c1_chars = &mut constants::gen_random_chars((bidx-2) as u32);
                        let new_c1_bytes = unsafe{ c1_chars.as_bytes_mut() };
                        c1_bytes_randomly[0..bidx-2].copy_from_slice(&new_c1_bytes);
                        
                        /* and set the bidx-1 to be the 0x00 */
                        pbyte = 0x00;
                        c1_bytes_randomly[bidx-1] = pbyte;

                    } else{
                        
                        println!("❌ cipherblock [{hex_first_block_cipher:}] | key [{pass:}] | iv [{c1_hex:}] | dec => {decrypted_cipher:}");
                        content = format!("❌ cipherblock [{hex_first_block_cipher:}] | key [{pass:}] | iv [{c1_hex:}] | dec => {decrypted_cipher:}");
                    
                        /* add 1 byte to last byte of the c1 cipherblock until we hit the jackpot! */
                        pbyte += 1;
                        c1_bytes_randomly[bidx] = pbyte;

                    }
                    
                    /* logging */
                    log.write(content.as_bytes()).unwrap();

                }

            }

        }

    }


}