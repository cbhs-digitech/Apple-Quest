use std::io::stdin;

fn get_string() -> String {
    // Varable to store input
    let mut line = String::new();
    // Reading and storing user input
    stdin().read_line(&mut line).expect("Failed to read line");
    // This function is to trim out the unwanted whitespace
    let line: String = line.trim().parse().unwrap();
    // Return the value of line
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

        // Checking if the answer is valid being between 0 and 5
        if line <= 0 && line >= 5 {
            println!("Sorry that's not a valid answer (The answers are from 1 to 4)");
            continue;
        }
        // Return the value of line
        return line;
    }
}

fn quest_one() -> u32 {
    // Question
    println!("When did Apple first reveal the iPhone?");

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
        1
    } else {
        println!("Incorrect");
        0
    }
}

fn quest_two() -> u32 {
    // Question
    println!("Who was the founder and CEO of Apple Inc.");

    // Selection of answers
    println!("1. Jeff Bezos");
    println!("2. Steve Jobs");
    println!("3. Tim Cook");
    println!("4. Sundar Pichai");

    // Getting user's answer
    let answer = get_answer();

    // Seeing if user's answer is correct
    if answer == 2 {
        println!("Correct");
        1
    } else {
        println!("Incorrect");
        0
    }
}

fn quest_three() -> u32 {
    // Question
    println!("When did Apple unveil the colourful Mac G4");

    // Selection of answers
    println!("1. 1999");
    println!("2. 1993");
    println!("3. 2002");
    println!("4. 2001");

    // Getting user's answer
    let answer = get_answer();

    // Seeing if user's answer is correct
    if answer == 1 {
        println!("Correct");
        1
    } else {
        println!("Incorrect");
        0
    }
}

// Main function to welcome the user and start the loop
fn main() {
    // The total points at the end of the questionare
    let mut total_points = 0;

    // Welcome message
    println!("Hello, and welcome to the Apple Inc. questionare");
    println!("    ");

    // User name
    println!("Please input your name");
    // User input for name
    let usr_name = get_string();
    println!("Welcome, {usr_name}");
    println!("    ");

    // Rules
    println!("Here is the rule: ");
    println!("You are to answer by the number corrosponding to the answer you think is correct");
    println!("    ");

    // Questions
    println!("Alright let the game begin!");
    println!("    ");

    total_points += quest_one();
    println!("    ");
    total_points += quest_two();
    println!("    ");
    total_points += quest_three();
    println!("    ");

    // Grammatically correct total point show
    if total_points == 1 {
        println!("You got {total_points} point");
    } else {
        println!("You got {total_points} points");
    }
}
