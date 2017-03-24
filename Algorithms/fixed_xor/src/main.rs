use std::io;

///
/// Hex-XOR-combinator
///
fn main() {
    
    let mut first_hex = String::new();

    println!("This Program XORs two hex strings of the same length.");
    println!("Please enter first hex string:");
    io::stdin().read_line(&mut first_hex).expect("failed to read line");

    let mut second_hex = String::new();
    println!("Now enter the second:");
     io::stdin().read_line(&mut second_hex).expect("failed to read line");

    if first_hex.len() != second_hex.len() { println!("The strings aren't of the same length!");}
    else { xor(first_hex, second_hex)};
}

fn xor(first: String, second: String)
{
    let first_hex_bytes = get_bytes(first);
    let second_hex_bytes = get_bytes(second);

    let mut result = Vec::new();
    
    // XOR every bit of each token
    for i in 0..first_hex_bytes.len()
    {
        result.push(first_hex_bytes[i] ^ second_hex_bytes[i]);
    }
    
    // print result
    let result_string = get_hex(result);
    println!("{:?}", result_string);
}

fn get_bytes(x: String) -> Vec<u32>
{
    // split string into chars into a vector
    let digits: Vec<char> = x.chars().collect();

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
    bytes
}

fn get_hex(x: Vec<u32>) -> String
{
    // This is solely to print the output in correct hex notation.
    // Therefore all numbers in the vector are concatenated to a single string,
    // with conversion of numbers higher than 9 to their corresponding chars.  
    let mut result = String::new().to_owned();
    for i in 0..x.len()
    {
        let mut number = String::new();
        
        // if number is smaller than 10, append number itself to result string
        if x[i] < 10 
        { 
            number = x[i].to_string().to_owned();
        }
        
        // if number is greater than 9, append representing character
        else 
        {
            number = match x[i]
            {
                10 => "A",
                11 => "B",
                12 => "C",
                13 => "D",
                14 => "E",
                15 => "F",
                _ => "X",
            }.to_string().to_owned();
        }
        result.push_str(&number);
    }
    result
}