use std::io;

fn main() 
{
    // search 'up_to' this value:
    print!("Please enter the value up to which primes shall be searched:");
    let mut up_to = String::new();
    io::stdin().read_line(&mut up_to).expect("failed to read line");

    // due to being parsed as an unsigned integer, this will panic if a negative number
    // has been entered
    let up_to :u64 = up_to.trim().parse().expect("this is not a valid number!");
    
    // if the number is zero, don't even bother
    if up_to > 0 { find_primes(up_to);}
    else         { println!("The number is zero, please enter a greater number!");}
    
}

fn find_primes(x: u64) 
{
    // vector to store found primes in
    let mut primes = Vec::new();
    
    for element in 0..x
    { 
        match element % 10 
        {
            // if the element ends on one of the following, it is definitely not prime
            0| 4| 6| 8    => continue,
            // if the element is 2 or 5 it is prime, else it can't be 
            2| 5          => if element / 10 == 0 { primes.push(element);} else { continue;},
            // if the element ends on one of the following it might be prime 
            3| 7| 9       => if check_further(element) { primes.push(element);} else { continue;},
            // if the element ends on 1, but isn't 1, it might be prime 
            1             => if element / 10 != 0 && check_further(element) { primes.push(element);} else { continue;},
            // this is the "default" case, which in here should be impossible. however, 'match' requires it
            _             => panic!("something went terribly wrong!"),
        }
    }

    // when finished, print the vector: 
    println!("{:?}", primes);
    
}

fn check_further(element: u64) -> bool
{
    // result is true by default, if this doesn't get altered,
    // the number is prime
    let mut result = true;

    // get half of the value as upper border for the dividability check 
    // the element's numerical value will definitely be odd, so add 1 to 
    // get a "rounded up" value 
    let biggest_divisor = (element+1)/2;
    for divisor in 2..biggest_divisor
    {
        // mod-div the element with every number up to half its value
        // if this equals 0, the number isn't prime
        if element % divisor == 0 
        { 
            // the requirements for a prime are not satisfied if this
            // is executed, so set result to false to signal a miss
            result = false;
            // then break the loop and return the result
            break;
        }
    }
    result 
}
