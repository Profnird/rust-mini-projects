use radio::Sink;
use rand::Rng;
use std::cmp::Ordering;
use std::io;
fn main() {
    // Rust Gamble Game
    println!("----------------------------------------------");
    println!("||||||||||||||||||||||||||||            |||||              |||||");
    println!("||||||||||||||||||||||||||||            |||||              |||||");
    println!("|||                        |||          |||||              |||||");
    println!("|||     Authors:           |||          |||||              |||||");
    println!("|||          nird          |||          |||||              |||||");
    println!("|||                        |||          |||||              |||||");
    println!("||||||||||||||||||||||||||              |||||              |||||");
    println!("||||||||||||||||||||||||||              |||||              |||||");
    println!("|||                       ||||          |||||              |||||   ");
    println!("|||                       ||||            |||              |||   ");
    println!("|||                       ||||             ||||||||||||||||||");
    println!("----------------------------------------------");

    println!("============== =================== ================");
    println!("============== Gamble all you want ================");
    println!("=============(   Enjoy Your Day!  )================");
    println!("============== =================== ================");
    println!("============== =================== ================");
    println!("Enter your Power Rusty number: ");
    let secret_number = rand::thread_rng().gen_range(0, 100); //using rand (lib) (random) to generate a random number between 0 and 100

    loop {
        let mut guessed_number = String::new(); //mutable variable to store your entered number

        io::stdin()
            .read_line(&mut guessed_number)
            .expect("Failed to read input"); // uses the input/output provided by the standard lib and readline to take inputs , expect is used to handle error (a must use)

        let guessed_number: u32 = match guessed_number.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };

        println!("++++++++++");
        println!("++      ++");
        println!("++  {}  ++", guessed_number);
        println!("++      ++");
        println!("++++++++++");

        match guessed_number.cmp(&secret_number) {
            // match is used together with the cmp(compare) to compare the guessed_number to the target number
            // ordering (less, greater , equal) are used to check if (== or > or < ) then returns a message or value you want to display
            Ordering::Less => println!("You are too low, you not rusty"),
            Ordering::Greater => println!("You are high, no way close"),
            Ordering::Equal => {
                println!("You Win ! Big win Comarade ! You are Rusty");
                break; // break is used to break the loop when you / user guess right
            }
        };
    }
}
