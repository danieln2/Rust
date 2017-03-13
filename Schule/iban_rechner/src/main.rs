fn main() {
    let mut x: Vec<u16> = vec![0,0,4,1,3,1,0,9,8,7,6,5,4,3,2,1,0,0,1,0,9,0,0,7];
<<<<<<< HEAD
    
=======
   
>>>>>>> 2dbce92ad964df384b09982ce76225614455fc4e
   
    let y = 97;
    let mut return_string = String::new();
    

    let mut result = 0;
    while x.len() >= 3 {
        // nimmt die letzten drei Zahlen aus dem Vector und macht sie zu einer dreistelligen Zahl 
        let buff = x[x.len()-1] * 100 + x[x.len()-2] * 10 + x[x.len()-3];

        // nimmt die zusammengesetzte Zahl modul dem festgelegten, zweistelligen Wert
        result = buff % y;

        // entfernt die letzten drei Werte aus dem Vector
        for i in 0..3
        {
            x.pop();
        }

        // schreibt den in der Modulo-Rechnung ermittelten Wert in umgekehrter Reihenfolge der Wertigkeit
        // zurück in den Vector
        x.push(result % 10);
        x.push(result / 10);
    }

    let temp_ones: u16 = x[0]; 
    let temp_tens: u16 = x[1]*10;
    
    result = 98 - (temp_ones + temp_tens);

    if result / 10 == 0
    {
        return_string = "0".to_string() + &result.to_string();
    }
    else 
    {
        return_string = result.to_string();
    }

    println!("{:?}", return_string );

}
