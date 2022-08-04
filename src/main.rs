use std::io::stdin;

fn get_string() -> String {
    // Varable to store input
    let mut line = String::new();
    // Reading and storing user input
    stdin().read_line(&mut line).expect("Failed to read line");
    // This function is to trim out the unwanted whitespace
    let line: String = line.trim().parse().unwrap();
    return line;
}

// Getting an integer input from the user
fn get_answer() -> i32 {
    loop {
        // The variable used to get the user input
        let mut line = String::new();
        // Getting user input
        stdin().read_line(&mut line).expect("Failed to read line");

        // Seeing if the user input is an interger or not
        // This method of redeclaring a variable is called shadowing so you dont have a headache with names
        let line = match line.trim().parse() {
            Ok(num) => num,
            Err(_) => {
                println!("Sorry that's not a valid answer (Please use a number)");
                // The continue is here to tell the loop to skip all the other commands and do it again
                continue;
            }
        };

        if !(line > 0) || !(line < 5) {
            println!("Sorry that's not a valid answer");
            continue;
        }

        return line;
    }
}

fn rules() {
    println!("The rules are that you answer in the number corrosponding to the answer");
}

fn quest_one() -> u32 {
    // Question
    println!("When did apple first reveal the iPhone?");

    // Selection of answers
    println!("1. 1999");
    println!("2. 2001");
    println!("3. 2005");
    println!("4. 2007");

    // Getting user's answer
    let answer = get_answer();

    // Seeing if user's answer is correct
    if answer == 4 {
        println!("Correct");
        return 1;
    } else {
        println!("Incorrect");
        return 0;
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
    let usr_name = get_string();
    // Printing user's name
    println!("Welcome, {usr_name}");

    println!("    ");

    // Asking if the user wants to see the currently barely written rules
    println!("Do you want to here the rules (Yes/No)");
    let yes_or_no: String = get_string().to_ascii_lowercase();
    if yes_or_no == "y".to_string() || yes_or_no == "yes".to_string() {
        rules();
    }

    println!("    ");
    println!("Ok");
    quest_one();
}
