pub fn caesar_encode(inputstring: &str, shift_number: u8) -> String {
    let mut strhere = "".to_string();
    let mut start: u8;
    for i in inputstring.chars(){
        if i.is_ascii_alphabetic(){
            if i.is_ascii_lowercase(){
                start = b'a';
            }else{
                start = b'A';
            }
            let each_char: char=(start + (i as u8 - start + shift_number) % 26) as char;
            strhere.push(each_char);
        }
    }
    return strhere;
}
 
pub fn caesar_decode(inputstring: &str, shift_number: u8) -> String {
    let mut strhere = "".to_string();
    let mut start: u8;
    for i in inputstring.chars(){
        if i.is_ascii_alphabetic(){
            if i.is_ascii_lowercase(){
                start = b'a';
            }else{
                start = b'A';
            }
            let each_char: char=(start + (26 as u8 + i as u8 - start - shift_number) % 26) as char;
            strhere.push(each_char);
        }
    }
    return strhere;
} 
