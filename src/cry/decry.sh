#!/bin/bash

block_cipher=$1
block_cipher_1_as_iv=$2
key=$3

# openssl aes-256-cbc -md sha384 -in secret.txt -out secrets.txt.enc  
echo -n "$block_cipher" | xxd -p -r | openssl aes-256-cbc -d -nopad -K "$key" -iv "$block_cipher_1_as_iv" | xxd -c 16
# echo -n 32e88fcd70e427d62cfadb44d71d955c | xxd -p -r | openssl aes-256-cbc -d -nopad -K 123 -iv 53616c7465645f5f8df362c4e646011 | xxd -c 16
# openssl aes-256-cbc -d -nopad -md sha384 -in secrets.txt.enc 