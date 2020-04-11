// It is necessary to bring this trait to scope in order to use its methods
use rand::Rng;
use std::cmp::Ordering;
use std::io;

fn main() {
    println!("Guess the number!");

    let secret_number = rand::thread_rng().gen_range(1, 101);

    println!("This is the secret number: {}", secret_number);

    loop {
        println!("Please input your guess.");

        let mut guess = String::new();

        //Stdin is the standard input of the current process.
        // As this application is running on the terminal the standard input is the keyboard.
        // If we had piped a file into the application the standard input would be the content of that file

        //read_line() returns a io::Result so the rust compiler will force us to handle the possible errors.
        //the expect() function is basically a placeholder while you didn't treat the error properly.
        //You check it inside Result's implementation it will return the value if the Result is ok
        //or it will panic the application with the string passed as an argument if there is an error
        io::stdin()
            .read_line(&mut guess)
            .expect("Failed to read line");

        // Here the first guess variable is shadowed by the new one. The first guess variable
        // will not exist anymore after this line
        //Here instead of using expect and panicking the application of the operation fails
        //we'll just ignore it and continue to the next iteration of this loop
        //Instead of crashing on an error we're handling the error
        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };

        println!("You guessed: {}", guess);

        match guess.cmp(&secret_number) {
            Ordering::Less => println!("Too small"),
            Ordering::Greater => println!("Too big"),
            Ordering::Equal => {
                println!(" You win");
                break;
            }
        }
    }
}
