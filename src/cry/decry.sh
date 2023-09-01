#!/bin/bash

block_cipher=$1
iv=$2
key=$3

 
echo -n "$block_cipher" | xxd -p -r | openssl aes-256-cbc -d -nopad -K "$key" -iv "$iv" | xxd -c 16