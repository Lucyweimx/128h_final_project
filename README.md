# 128h_final_project (Group name: Cryptography)

## Group member names and NetIDs
Mengxuan Wei(mwei10) 

## Project Introduction
We plan to make an Encryption & Decryption project because we think it is very interesting.

Implement 3 common encryption algorithms (from the easiest to the hardest): Morse Code, Caesar cipher, AES(Advanced Encryption Standard, but only part of this).

## Technical Overview
tools will use for the visualization part (no visualization)
3rd party crates plan to use 

### General description of the algorithms mentioned: 

Morse code encryption is replacing each normal character with their corresponding morse code, and decryption is replacing morse code with their corresponding normal character. 

Caesar cipher encryption and decryption is replacing each character with their corresponding character after adding/subtracting the number input. 

"AES 256-bit encryption is the strongest and most robust encryption standard that is commercially available today." -- https://www.idera.com/aes-256-bit-encryption/#:~:text=AES%20256%2Dbit%20encryption%20is,encryption%20has%20never%20been%20cracked 


### Plan to finish before the first checkpoint (11/6 to 11/11): 
1. Understand the Encryption & Decryption algorithms. 
2. Finish writing the rust codes for Morse Code, Caesar cipher (without visualizing).

### Plan to finish before the second checkpoint (11/27 to 12/1):
1. Understand the AES Algotithms
2. Finish writing the rust codes for AES Encryption.

## Possible Challenges

AES is very complicated. (So my code is only part of aes)

Matrix calculation in rust. (So I turned them into just one array)

String/Hex array transformation

# future plan (after finals/during winter break)

finish writing aes 

## References

Reference for the understanding of AES (and some of the algorithms understanding, though not using them) https://zhuanlan.zhihu.com/p/78913397?utm_psn=1713621561817395201

Reference for the SBOX and the RCON: https://github.com/TheAlgorithms/Rust/blob/master/src/ciphers/aes.rs (the algorithms of his/her code is hard to understand, so I didn't reference his code other than his SBOX and RCON and the idea to make plain/key into an [u8; 16] array)
