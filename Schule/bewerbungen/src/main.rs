use std::io;

fn main()
{
    // Define new strings to take user input from console
    let mut younger = String::new();
    let mut mcse = String::new();
    let mut degree = String::new();

    // Possible output strings
    let interview = "Der Bewerber wird zu einem Vorstellungsgespraech eingeladen!";
    let test = "Der Bewerber wird zu einem schriftlichen Test eingeladen.";
    let rejection = "Der Bewerber wird abgelehnt, aber die Bewerbungsunterlagen einbehalten.";
    let rejection_and_return = "Der Bewerber wird abgelehent und die Bewerbungsunterlagen werden zurueck gesendet.";
    let mut answer = String::new();

    // Define bools to determine correct output string
    let mut is_younger: bool = false;
    let mut has_mcse: bool = false;
    let mut has_degree: bool = false;

    // Check if applicant is younger than 30 y/o
    println!("Ist der Bewerber jünger als 30 Jahre? (y/n)");
    io::stdin().read_line(&mut younger).expect("failed to read first value");
    
    let younger =  younger.trim();

    match younger.as_ref()
    {
        "y" | "Y" | "j" | "J" => is_younger = true,
        "n" | "N" => is_younger = false,
        _ => println!("unknown input, assuming 'n'"),
    };

    // Determine output
    if is_younger
    {
        // Check if applicant has MCSE cert
        println!("Kann der Bewerber ein MCSE-Zertifikat vorweisen? (y/n)");
        io::stdin().read_line(&mut mcse).expect("failed to read second value");

        let mcse =  mcse.trim();

        match mcse.as_ref()
        {
            "y" | "Y" | "j" | "J" => has_mcse = true,
            "n" | "N" => has_mcse = false,
            _ => println!("unknown input, assuming 'n'"),
        };
        
        // Check if applicant has college degree
        println!("Hat der Bewerber einen Hochschulabschluss? (y/n)");
        io::stdin().read_line(&mut degree).expect("failed to read third value");

        let degree =  degree.trim();

        match degree.as_ref()
        {
            "y" | "Y" | "j" | "J" => has_degree = true,
            "n" | "N" => has_degree = false,
            _ => println!("unknown input, assuming 'n'"),
        };
   
        if has_mcse
        {
            answer = test.to_string();

            if has_degree
            {
                answer = interview.to_string();
            }  
        } 

        else 
        {
            answer = rejection.to_string();
            
            if has_degree
            {
                answer = test.to_string();
            }
        }
    }

    else
    {
        answer = rejection_and_return.to_string();
    }

    // Output elaborated string
    println!("{:?}", &answer);
}

