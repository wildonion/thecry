
## 🎯 Run

> first clone the repo then install the followings (refer to https://docs.cossacklabs.com/themis/installation/installation-from-packages/ if you don't want to build themis from source):

```bash
wget http://archive.ubuntu.com/ubuntu/pool/main/o/openssl/libssl1.1_1.1.1f-1ubuntu2_amd64.deb
sudo dpkg -i libssl1.1_1.1.1f-1ubuntu2_amd64.deb
sudo apt update -y && sudo apt upgrade && sudo apt install -y libpq-dev pkg-config build-essential libudev-dev libssl-dev librust-openssl-dev
git clone https://github.com/cossacklabs/themis.git
cd themis
make
sudo make install
# install themis on MacOS M1
brew install libthemis
```

```bash
cargo run --bin thecry
```

## 🐍 Mem Poisoning Usage

* compile the onion exploit into the `linux` or `windows` executable

* extract the shellcode from the executable into `shellcode.bin` using ```objdump -d ./onion | grep -Po '\s\K[a-f0-9]{2}(?=\s)' | perl -pe 's/\r?\n//' | sed 's/$/\n/' > shellcode.bin``` command

* inject the shellcode into the memory by invoking the `inject()` method.

# 🥃 AES-256-CBC Decryption Process

## AES-256-CBC
- AES is the Advanced Encryption Standard.
- 256 refers to the key size (256 bits).
- CBC is Cipher Block Chaining, a mode of operation for AES.
- `sha384` will be used to hash the password or the cipher key in which the first 32 bytes is the key and the rest 16 bytes can be used as the `IV`.
- AES requires blocks of 16 bytes in length to start encryption.
- in AES a single key or a password in form of a KDF (hashed using `sha384` or `pbkdf2`) can be used to encrypt and decrypt data.
- the first cipherblock would have been encrypted using an IV (initialization vector), a secret cipherblock chosen by the encrypter during the encryption process, then the rest of the cipherblocks will be generated using the last one as the `IV` since this cipher block chaining process.
- in CBC encryption, each block of plaintext is XORed with the previous ciphertext block before being passed into the cipher. So in CBC decryption, each ciphertext is passed through the cipher, then XORed with the previous ciphertext block to give the plaintext.
- the intermediate state is the state of a ciphertext block after being decrypted by the block cipher but before being XORed with the previous ciphertext block.

### CBC-Encryption

<p align="center">
    <img src="https://github.com/wildonion/thecry/blob/main/src/cry/CBC_encryption.svg">
</p>

### CBC-Decryption

<p align="center">
    <img src="https://github.com/wildonion/thecry/blob/main/src/cry/CBC_decryption.png">
</p>

## PKCS#7 Padding
**PKCS#7** is a standard for padding messages to a block size suitable for encryption. The padding works by appending
`N` bytes of value `N` to the message, where `N` is the number of bytes required to reach the block size. In the context of AES, which has a 16-byte block size:
- If the last block of plaintext is `X` bytes where `X < 16`, we append `16 - X` bytes to the message or the plaintext of the value `16 - X`.
- If the last block of plaintext is exactly 16 bytes, a new block is appended with 16 bytes, each of value `0x10`.

## Valid PKCS#7 Padding
For the padding to be valid, the last `N` bytes of the decrypted plaintext must be a sequence from 1 to `N`, each of value `N`. For example, the last two bytes could be `0x02, 0x02` or the last three bytes could be `0x03, 0x03, 0x03`.

## How to Determine a Correct Key
To verify whether you've used the correct key for decryption, you decrypt the last block of the ciphertext using that key and the ciphertext from the second-to-last block as the `IV` (Initialization Vector). If the decryption is successful and the plaintext contains valid **PKCS#7** padding, then the key is most likely correct.

## Decryption Process 

> more on [Padding Oracle Attack](https://robertheaton.com/2013/07/29/padding-oracle-attack/)

In our `sec.json` file example, we would select two cipher blocks inside the json file using a hex editor, like `5307f7afffa3798f386e7c6c144c6a9c` and its next block like `3b2364d1d04a35c8081bbc6fdeacbd86` to decrypt the second one using the second-to-last block as the `IV` which `5307f7afffa3798f386e7c6c144c6a9c`. If the resulting plaintext block has a valid **PKCS#7** padding, then the key is correct. This check works because if the wrong key or IV is used for decryption, the resulting plaintext is highly unlikely to have valid **PKCS#7** padding.

## Conclusion

if we have an encrypted file contains the cipher blocks (assuming that these are simply AES-256-CBC encrypted blocks), then the decryption process would generally involve reading those blocks from the file, then decrypting each block, in our case the output hex inside `sec.json` is formatted with 16 bytes in each row cause AES requires blocks of 16 bytes in length (16 bytes plaintext --> 16 bytes cipher block), so that each row of 16 bytes represents one block of ciphertext (hex string) in **AES-256** and if we decrypt a cipher block that will give us a correct padding we can say that we used the correct key.
