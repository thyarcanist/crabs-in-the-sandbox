fn main() {
    let mut self_value = 0;
    let mut increment = true;

    // Print the ASCII art
    println!("            _     _ _____ _____");
    println!("           (_)   | |  _  /  ___|");
    println!("__   _____  _  __| | | | \\ `--. ");
    println!("\\ \\ / / _ \\| |/ _` | | | |`--. \\");
    println!(" \\ V / (_) | | (_| \\ \\_/ /\\__/ /");
    println!("  \\_/ \\___/|_|\\__,_|\\___/\\____/ ");
    println!();

    let mut spirit_printed = false;
    let mut feeling_printed = false;

    loop {
        if !spirit_printed {
            print!("I've got the spirit  ");
            spirit_printed = true;
        } else {
            print!("                    ");
        }
        for _i in 0..self_value {
            print!("▒");
        }
        for _i in self_value..10 {
            print!("░");
        }
        println!("");

        if !feeling_printed {
            print!("but lose the feeling  ");
            feeling_printed = true;
        } else {
            print!("                       ");
        }
        for _i in self_value..10 {
            print!("░");
        }
        for _i in 0..self_value {
            print!("▒");
        }
        println!("");

        let mut input = String::new();
        match std::io::stdin().read_line(&mut input) {
            Ok(_) => {
                match input.trim() {
                    "+" => {
                        if self_value < 10 {
                            self_value += 1;
                        }
                        spirit_printed = false;
                        feeling_printed = false;
                    }
                    "-" => {
                        if self_value > 0 {
                            self_value -= 1;
                        }
                        spirit_printed = false;
                        feeling_printed = false;
                    }
                    "q" => {
                        return;
                    }
                    _ => {}
                }
            }
            Err(_) => {}
        }

        if increment {
            self_value += 1;
        } else {
            self_value -= 1;
        }

        if self_value == 0 || self_value == 10 {
            increment = !increment;
        }

        std::thread::sleep(std::time::Duration::from_millis(100)); // Wait for 0.1 seconds
    }
}
