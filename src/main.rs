// use std::io;
// dealing with input is too complicated, just directly "cargo run" this file
mod caesar;
mod morse_code;
mod aes;

fn main() {
    // CAESAR encrypt and decrypt instruction
    let default_str1: &str="Testcode"; 
        // you could change "Testcode" to any string consist of letters
    let default_length1: u8=12; 
        // you could change this length to the length you want to move 
    let result1: String = crate::caesar::caesar_encode(default_str1, default_length1);
    println!("this is the result 1 of encrypt default_str1 using caesar: {}",result1); 
        // this is using your input string and length to create caesar cipher and printing it
    let result_back: String = crate::caesar::caesar_decode(&result1, default_length1);
    println!("this is the result to decrypt the result 1 back to your original string: {}", &result_back);
        // this is decrypt of the result 1 back to the default string with the default length
        // if you want to decrypt strings, you could copy the line 9 line 11 line 16, rename the variable
        // and change the strings and numbers to what you want 


    // MORSE encrypt and decrypt
    let default_str2: &str="test code, here is what we have got";
        // you could change the default_str2 to any string consist of letters, numbers, ".", ",", and " "
    let result2: String= crate::morse_code::morse_code_encode(default_str2);
        // you could change the result2 to any string 
    println!("this is the morse result: {}", &result2);
        // this is printing the code encrypting result
    let default_morse: &str="- . ... - / -.-. --- -.. . --..-- / .... . .-. . / .. ... / .-- .... .- - / .-- . / .... .- ...- . / --. --- -";
        // you could change this default_morse to a morse code you want
        // remember to type a space " " between characters
        // and a forward slash "/" to represent the space between the 
    let result_back2: String =crate::morse_code::morse_code_decode(default_morse);
        // you could change this "default_morse" we used to "result2" to check my code is correct or not
        // and it is correct, I have checked it :) 
    println!("this is the morse code decryption: {}", &result_back2);
        // this is printing the code decrypting result
    

    // very simplified AES encrypt and decrypt
    // only contains plain_s_substitution + shift_row + add_round_key 
    // instead of the complete AES calculation, which is so hard
    let mut plain1: [u8; 16] = crate::aes::form_random_array(); 
        // generate a random hex array for plain, since it is hard to type it
        // if want to write the string to encrypt, use the code below
    crate::aes::print_array(plain1, "random plain1");
        // print the plain array in a string
    let key1: [u8; 16] = crate::aes::form_random_array(); 
        // generate a random hex array for key, since it is hard to type it
    crate::aes::print_array(key1, "random key1");
        // print the key array in a string
    let mut cipher: [u8; 16] = crate::aes::aes_encrypt(&mut plain1, &key1);
        // encrypt them to a cipher (this step only contains plain_s_substitution
        // + shift_row + add_round_key instead of the complete AES, which is so hard, calculation)
    crate::aes::print_array(cipher, "result cipher for random plain and key");
        // print the correspond cipher
    let plainback: [u8; 16] = crate::aes::aes_decrypt(& mut cipher,&key1);
        // decrypt it back to the plain, to check if this is symmetric, 
        // and to check if we are able to track back our plain using the cipher and key
    crate::aes::print_array(plainback, "plain back check");
        // print the decrypted back plain
    
    // very simplified AES using input string
    let plain2: &str="this is length 1"; 
        // this plain2 length need to be 16 characters, you could change it to other string that satisfy
    let key2: &str="this is length 2"; 
        // this key2 length must be 16 characters, you could change it to other string that satisfy
    let cipher2: [u8; 16] = crate::aes::aes_from_string_encrypt(plain2,key2);
        // using the plain2 and key2 to encrypt the cipher
    crate::aes::print_array(cipher2, "cipher2");
        // print the cipher in hex form
    
    // better visualize the hex array
    let s: String = crate::aes::hex_array_to_str(&cipher2);
        // turn the cipher2, or any hex array, into string form. It might looks like just a random string. 
    println!("this is how the array looks like: {}", s);
        // print the string form

    // I make the functions all to be public, so we shoule be able to use them
    // and able to customize a encrypting way using: plain:&mut [u8;16], key:[u8;16], cipher:&mut [u8;16]
    // crate::aes::plain_s_substitution(plain)    inverse_s_substitution(cipher)
    // crate::aes::shift_row(plain)    crate::aes::shift_back_rows(cipher)
    // crate::aes::add_round_key(plain,key) and I think the "bit wise or" is the same for coming back

}

