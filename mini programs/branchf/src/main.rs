// using this to test functions while going through this: https://doc.rust-lang.org/book/

fn main() {
	rev_collectionloop();
}

// loop_label is functional
fn loop_label() {
	let mut count = 0;
	'counting_up: loop { // the outer loop has the label counting up
		println!("count = {count}");
		let mut remaining = 10;
		
		loop{
			println!("remaining = {remaining}");
			if remaining == 9 {
				break;
			}
			if count == 2 {
				break 'counting_up;
			}
			remaining -= 1;
		}
		
		count += 1;
	}
	
	println!("End count = {count}");
}

// multiple types of conditional checks
fn conditional_check() {
	   let number = 7;
	
	// checks to see if number is less than 5
	if number < 5 {
		println!("Condition was true.");
	} else {
		println!("Condition was false.");
	}
	
	
	// checks to see is num does not equal zero
	let num = 3;
	
	if num != 0 {
		println!("Num was something other than zero");
	}
	
	// checks the condition of true, if its true con_number is 5
	// if its false con_number is 6
	let condition = true;
	let con_number = if condition {5} else {6};
	
	println!("The value of the con_number is: {con_number}");
}


// checks to see if a specific variable is divisible (WILL EXPAND LATER)
fn divisible() {
	// checks to see if var is divisible
	let divisible = 6;
	
	if divisible % 4 == 0 {
		println!("divisible var is divisible by 4");
	} else if divisible % 3 == 0 {
		println!("divisible var is divisible by 3");
	} else if divisible % 2 == 0 {
		println!("divisible var is divisible by 2");
	} else {
		println!("divisible var is not divisible by 4, 3 or 2.");
	}	
}

// prints counter result for this scope
fn print_counter_result() {
		// LOOPS
	
	let mut counter = 0;
	
	let result = loop {
		counter += 1;
		
		if counter == 10 {
			break counter * 2;
		}
	};
	
	println!("The result is {result}");
	
}

// counts down intakes a number
fn countdown(number: u32) {
	let mut number = number;
	
	while number != 0 {
		println!("{number}");
		number -= 1;
	}
	
	println!("LIFT OFF!");
}

// loop collections
fn primitive_loopthroughcollection() {
	let a = [10, 20, 30, 40, 50];
	let mut index = 0;
	
	while index < 5 {
		println!("The value is: {}", a[index]);
		
		index += 1;
	}
}

fn better_collectionloop() {
	let a = [10, 20, 30, 40, 50];
	let mut index = 0;
	
	for element in a {
	println!("The value is: {element}");
	// this version increases the safety of the code and elimates the chance to bugs.
	}
}

// loops through collected but reverse (reverse the range)
fn rev_collectionloop() {
	for number in (1..4).rev() {
		println!("{number}!");
	}
	
	println!("LIFT OFF!!");
}






