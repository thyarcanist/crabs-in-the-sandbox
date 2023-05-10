use std::io;

fn main() {
    println!("Reporting!");
    commence();

    loop {
        let mut user_input = String::new();
        io::stdin()
            .read_line(&mut user_input)
            .expect("Failed to read line.");

        let user_input: u32 = match user_input.trim().parse() {
            Ok(num) => num,
            Err(_) => {
                println!("Invalid input. Please enter a valid number.");
                continue;
            },
        };

        report_check(user_input);
        break;
    }
}

fn commence() {
    println!("Enter your squad size: ");
}

fn report_check(x: u32) {
    if x < 2 {
        println!("Your squad isn't large enough.");
    } else {
        println!("Your squad is large enough.");
    }
}
