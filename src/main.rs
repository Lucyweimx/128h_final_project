//use std::io;
mod caesar;
mod morse_code;
mod aes;

fn main() {
    println!("{}",crate::caesar::caesar_encode("test", 12));
    println!("{}",crate::caesar::caesar_decode("fqef", 12));
    println!("{}",crate::caesar::caesar_decode("code", 14));
    println!("{}",crate::caesar::caesar_encode("oapq", 14));

    println!("{}",crate::morse_code::morse_code_encode("test code, here is what we have got"));
    println!("{}",crate::morse_code::morse_code_decode("- . ... - / -.-. --- -.. . --..-- / .... . .-. . / .. ... / .-- .... .- - / .-- . / .... .- ...- . / --. --- -"));
    
    let key1: [u8; 16] = crate::aes::form_random_array();//generate a randome hex array
    crate::aes::print_array(key1, "key1");
    let mut plain1: [u8; 16] = crate::aes::form_random_array();//generate a randome hex array
    crate::aes::print_array(plain1, "plain1");
    let cipher: [u8; 16] = crate::aes::aes_encrypt(&mut plain1, &key1);
    crate::aes::print_array(cipher, "cipher");
    let plain2: &str="this is length 1";
    let key2: &str="the is length 2";
    let cipher2 = crate::aes::aes_from_string_excrypt(plain2,key2);
    crate::aes::print_array(cipher2, "cipher2");
    let s = crate::aes::hex_array_to_str(&key1);
    println!("{}", s);
}

