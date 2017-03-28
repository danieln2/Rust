use std::io;

///
/// XORs a hex sequence with a single byte
///
// TODO: Output all calculated sequences into file, not into terminal
fn main() 
{
    // check for every ascii value of a letter 
    for c in 63..123
    {
        // TODO: make cipher settable from command line, so it does not need to be hardcoded
        let mut cipher = String::from("0e3647e8592d35514a081243582536ed3de6734059001e3f535ce6271032");
        let ascii_number: u8 = c; 
        println!("With key:{:?}\n {:?}\n", ascii_number as char, decrypt(cipher, ascii_number));
    }
}


fn decrypt(cipher: String, key: u8) -> Vec<char>
{
    let first_hex_bytes: Vec<u8> = get_bytes(cipher);
    //let second_hex_bytes: Vec<u8> = get_bytes(key);
    let mut result = Vec::new();
    let mut index = 0;
    // XOR every bit of each token
    for i in 0..(first_hex_bytes.len()/2)
    {
        result.push((((first_hex_bytes[index] << 4) + first_hex_bytes[index+1]) ^ key) as char);
        index += 2;
    }
    result
}


fn get_bytes(x: String) -> Vec<u8>
{
    // split string into chars into a vector
    let digits: Vec<char> = x.chars().collect();

    let mut bytes: Vec<u8> = Vec::new();

    for i in 0..(digits.len())
    {
        // get numerical value from hex string 
        let mut symbol = "0".to_owned();
        let number = digits[i].to_string().to_owned();
        symbol.push_str(&number);
        let hex_number: u8 = u8::from_str_radix(&symbol, 16).unwrap();
        bytes.push(hex_number);
    }
    bytes
}


