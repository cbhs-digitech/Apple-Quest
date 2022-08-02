use std::io::stdin;

// Getting an integer input from the user
fn get_int() -> i64 {
    loop {
        // The variable used to get the user input
        let mut line = String::new();
        // Getting user input
        stdin().read_line(&mut line).expect("Failed to read line");

        // Seeing if the user input is an interger or not
        // This method of redeclaring a variable is called shadowing so you dont have a headache with names
        let line = match line.trim().parse() {
            // The input goes smooth sailing if it is a number
            Ok(num) => num,
            // If the input is not a number then we tell the user to try again
            Err(_) => {
                println!("Sorry that's not a number");
                // The continue is here to tell the loop to skip all the other commands and do it again
                continue;
            }
        };
        return line;
    }
}

// Main function to welcome the user and start the loop
fn main() {
    // Welcome message
    println!("Hello, and welcome to the Apple Inc. questionare");
    println!("    ");

    // User name
    println!("Please input your name");

    // User input for name
    let mut usr_name = String::new();
    stdin()
        .read_line(&mut usr_name)
        .expect("Failed to read user name");
    let usr_name: String = usr_name.trim().parse().unwrap();

    // Printing user's name
    println!("Welcome, {usr_name}");

    // Testing if the integer input function and printing it out
    let num = get_int();
    println!("{num}");
}
