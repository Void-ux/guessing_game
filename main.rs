use std::io;
use std::cmp::Ordering;
use rand::Rng;
fn main() {
    println!("Guess the number!");

	let secret_number = rand::thread_rng().gen_range(1..101);

	loop {
		println!("Please input your guess.");

		let mut guess = String::new(); // object is mutable (changeable). immutable by defaul in Rust

		io::stdin()
			.read_line(&mut guess)
			.expect("Failed to read line");

		// expect will handle any errors if the io::result is not Ok and instead is an Err
		
		let guess: u32 = match guess.trim().parse() {
			Ok(num) => num,
			Err(_) => continue, // the _ represents a catchall value
		};
		// trim() will remove whitespace infront and behind the string. 
		//It also removes the \n from the string when clicking enter on the keyboard
		
		// parse() will convert the string into an unsigned 32-bit int as specified after the :
		// 32-bit unsigned int is a small positive number

		println!("You guessed: {}", guess); // {} is a placeholder

		match guess.cmp(&secret_number) {
			Ordering::Less => println!("Too small!"),
			Ordering::Greater => println!("Too big!"),
			Ordering::Equal => {
				println!("You win!");
				break; // leaving the loop when you win
			}
		}
	}
}
