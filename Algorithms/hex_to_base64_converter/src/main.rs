use std::io;
use std::u32;

///
/// Hex to Base64 Converter
///
fn main() 
{
    // Read hex string from command line
    let mut hex_string = String::new();
    println!("please enter hex string:");
    io::stdin().read_line(&mut hex_string).expect("failed to read line");
    convert(hex_string);
}


fn convert (hex_string: String) 
{
    // split string into chars into a vector
    let digits: Vec<char> = hex_string.chars().collect();

    let mut bytes: Vec<u32> = Vec::new();

    for i in 0..(digits.len()-2)
    {
        // get hex value from string 
        let mut symbol = "0".to_owned();
        let number = digits[i].to_string().to_owned();
        symbol.push_str(&number);
        let hex_number: u32 = u32::from_str_radix(&symbol, 16).unwrap();
        bytes.push(hex_number);

    }

    let mut index = 0;
    let mut hex: Vec<u32> = Vec::new();
     
    // concatenate two byte values (effectively only 4 bit are set, so only 4 bit need to be shifted) 
    while index < bytes.len()-1
    {
         hex.push((bytes[index] << 4) + bytes[index+1]);
         index += 2;
    }

    index = 0;
    let mut buffer: u32 = 0;
    let mut base64: Vec<char> = Vec::new();

    while index < hex.len()
    {
        buffer = (hex[index]<<16) + (hex[index+1]<<8) + hex[index+2];

        base64.push(translate_to_base64(buffer>>18));
        base64.push(translate_to_base64(buffer>>12));
        base64.push(translate_to_base64(buffer>> 6));
        base64.push(translate_to_base64(buffer));
        
        index += 3;
    }

    // sequence for testing: 49276d206b696c6c696e6720796f757220627261696e206c696b65206120706f69736f6e6f7573206d757368726f6f6d
    let test = vec!['S','S','d','t','I','G','t','p','b','G','x','p','b','m','c','g','e','W','9','1','c','i','B','i','c','m','F','p','b','i','B','s','a','W','t','l','I','G','E','g','c','G','9','p','c','2','9','u','b','3','V','z','I','G','1','1','c','2','h','y','b','2','9','t'];
    assert_eq!(base64, test);

    // assemble output string
    let output: String = base64.into_iter().collect();
    println!("{:?}", output);   
}


fn translate_to_base64(hex: u32) -> char
{
    // shift bits around, so only six bits are set
    let mut six_bit_number = hex << 26;
    six_bit_number = six_bit_number >>26;

    let mut symbol = ' ';
    let symbols = vec!['A','B','C','D','E','F','G','H','I','J','K','L','M','N','O','P','Q','R','S','T','U','V','W','X','Y','Z',
                       'a','b','c','d','e','f','g','h','i','j','k','l','m','n','o','p','q','r','s','t','u','v','w','x','y','z',
                       '0','1','2','3','4','5','6','7','8','9','+','/' ];
    
    // return char depending on given value
    for i in 0..64
    {
        if i == six_bit_number
        {
            symbol = symbols[i as usize];
        }
    }
    symbol
}