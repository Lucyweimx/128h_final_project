pub fn morse_code_encode(inputstring: &str) -> String{
    let mut strhere = "".to_string();
    for i in inputstring.chars(){
        strhere.push_str(&each_char(i.to_ascii_uppercase()));
        strhere.push_str(&" ");
    }
    return strhere; 
}
 
pub fn each_char(inputchar: char) -> String{
    match inputchar{
        'A'=> return ".-".to_string(), 'B'=> return "-...".to_string(),
        'C'=> return "-.-.".to_string(), 'D'=> return "-..".to_string(),
        'E'=> return ".".to_string(), 'F'=> return "..-.".to_string(),
        'G'=> return "--.".to_string(), 'H'=> return "....".to_string(),
        'I'=> return "..".to_string(), 'J'=> return ".---".to_string(),
        'K'=> return "-.-".to_string(), 'L'=> return ".-..".to_string(),
        'M'=> return "--".to_string(), 'n'=> return "-.".to_string(),
        'O'=> return "---".to_string(), 'P'=> return ".--.".to_string(),
        'Q'=> return "--.-".to_string(), 'R'=> return ".-.".to_string(),
        'S'=> return "...".to_string(), 'T'=> return "-".to_string(),
        'U'=> return "..-".to_string(), 'V'=> return "...-".to_string(),
        'W'=> return ".--".to_string(), 'X'=> return "-..-".to_string(),
        'Y'=> return "-.--".to_string(), 'Z'=> return "--..".to_string(),
        
        '1'=> return ".----".to_string(), '2'=> return "..---".to_string(),
        '3'=> return "...--".to_string(), '4'=> return "....-".to_string(),
        '5'=> return ".....".to_string(), '6'=> return "-....".to_string(),
        '7'=> return "--...".to_string(), '8'=> return "---..".to_string(),
        '9'=> return "----.".to_string(),'0'=> return "-----".to_string(),
        
        '.'=> return ".-.-.-".to_string(),','=> return "--..--".to_string(),
        ' '=> return "/".to_string(),
        _=>return "dne".to_string(),
    };
}

pub fn morse_code_decode(inputstring: &str) -> String{
    let mut strhere = "".to_string();
    let v: Vec<&str> = inputstring.split(&[' '][..]).collect();
    for i in v{
        strhere.push_str(&each_morse(i).to_ascii_lowercase());
    }
    return strhere;
}

pub fn each_morse(inputstr: &str) -> String{
    match inputstr{
        ".-"=> return "A".to_string(), "-..."=> return "B".to_string(),
        "-.-."=> return "C".to_string(), "-.."=> return "D".to_string(),
        "."=> return "E".to_string(), "..-."=> return "F".to_string(),
        "--."=> return "G".to_string(), "...."=> return "H".to_string(),
        ".."=> return "I".to_string(), ".---"=> return "J".to_string(),
        "-.-"=> return "K".to_string(), ".-.."=> return "L".to_string(),
        "--"=> return "M".to_string(), "-."=> return "N".to_string(),
        "---"=> return "O".to_string(), ".--."=> return "P".to_string(),
        "--.-"=> return "Q".to_string(), ".-."=> return "R".to_string(),
        "..."=> return "S".to_string(), "-"=> return "T".to_string(),
        "..-"=> return "U".to_string(), "...-"=> return "V".to_string(),
        ".--"=> return "W".to_string(), "-..-"=> return "X".to_string(),
        "-.--"=> return "Y".to_string(), "--.."=> return "Z".to_string(),
        
        ".----"=> return "1".to_string(), "..---"=> return "6".to_string(),
        "...--"=> return "2".to_string(), "....-"=> return "7".to_string(),
        "....."=> return "3".to_string(), "-...."=> return "8".to_string(),
        "--..."=> return "4".to_string(), "---.."=> return "9".to_string(),
        "----."=> return "5".to_string(),"-----"=> return "0".to_string(),
        
        ".-.-.-"=> return ".".to_string(), "--..--"=> return ",".to_string(),
        "/"=> return " ".to_string(), "dne"=> "nothing".to_string(),
        _=>return "not_a_morse_code".to_string(),
    };
    return "nothing happend".to_string();
}
