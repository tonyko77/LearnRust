use rand::Rng;
use std::cmp::Ordering;
use std::io::{self, Write};


fn main() {
    println!("Guess the number!");

    // generate the number to be guessed
    let secret_number: i32 = rand::thread_rng().gen_range(1..=100);
    // println!("(TEMP) The secret number is: {secret_number}");

    let mut cnt_guesses = 0;
    loop {
        cnt_guesses += 1;

        print!("\n({cnt_guesses}) Please input your guess: ");
        io::stdout().flush().unwrap();
    
        // read user input
        let guess: i32 = match read_number() {
            Ok(num) => num,
            Err(ReadError::BadNumber) => {
                println!("Please type a number between 1 and 100!");
                cnt_guesses -= 1;
                continue;
            },
            Err(ReadError::Quit) => {
                println!("You gave up after ({}) attempts :(", cnt_guesses - 1);
                break;
            }
        };
    
        match guess.cmp(&secret_number) {
            Ordering::Less => println!("Too small!"),
            Ordering::Greater => println!("Too big!"),
            Ordering::Equal => {
                println!("You win after ({cnt_guesses}) attempts :)");
                break;
            }
        }
    }
}


enum ReadError {
    BadNumber,
    Quit
}


fn read_number() -> Result<i32, ReadError> {
    // read user input
    let mut guess: String = String::new();
    io::stdin()
        .read_line(&mut guess)
        .expect("Failed to read line");

    // TODO trim/lowercase String + convert to immutable &str - not sure if ok :((
    let guess: &str = &guess.trim().to_ascii_lowercase();

    // quit if user gave up
    if ["exit", "quit"].contains(&guess) {
        Err(ReadError::Quit)
    }
    else {
        match guess.parse() {
            Ok(num) => {
                if num >= 1 && num <= 100 {
                    Ok(num)
                } else {
                    Err(ReadError::BadNumber)
                }
            },
            Err(_) => Err(ReadError::BadNumber)
        }
    }
}
