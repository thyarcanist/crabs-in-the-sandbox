use std::io;
use std::cmp::Ordering;
use rand::Rng;

fn main() {
    println!("Guess the number!");
	let secret_number = rand::thread_rng().gen_range(1..=100);
	//println!("The secret_number is: {secret_number}");
    
	loop {
		
		let mut guess = String::new(); //generates guess
		println!("Please input your guess.");
		
		io::stdin() // reads user input
            .read_line(&mut guess)
            .expect("Failed to read line");
	
	// line let guess: u32 parses the readLine and converts it into an unsigned 32
    let guess: u32 = match guess.trim().parse() {
		// used to be .expect("Please type a number!"); without match
		Ok(num) => num,
		Err(_) => continue,
	};

    println!("You guessed: {guess}");	
	
    match guess.cmp(&secret_number) {
		Ordering::Less => println!("Too small!"),
		Ordering::Greater => println!("Too big!"),
		Ordering::Equal => {
			println!("You win!");
		    break;}
	}
		
	}
}
