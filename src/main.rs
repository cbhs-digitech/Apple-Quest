use rand::Rng;
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
        let line: i32 = match line.trim().parse() {
            Ok(num) => num,
            Err(_) => {
                println!("Sorry that's not a valid answer (Please use a number)");
                // The continue is here to tell the loop to skip all the other commands and do it again
                continue;
            }
        };

        // Checking if the answer is valid being between 0 and 5
        if line <= 0 || line >= 5 {
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

fn quest_four() -> u32 {
    // Question
    println!("What was the previous CPU architecture that was used on Macs when Apple switched to Intel ");

    // Selection of answers
    println!("1. RISC-V");
    println!("2. PowerPC");
    println!("3. x86");
    println!("4. ARM");

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

fn quest_five() -> u32 {
    // Question
    println!("When was the iPod nano brought into market as a successor to the iPod mini");

    // Selection of answers
    println!("1. 2007");
    println!("2. 2006");
    println!("3. 2005");
    println!("4. 2004");

    // Getting user's answer
    let answer = get_answer();

    // Seeing if user's answer is correct
    if answer == 3 {
        println!("Correct");
        1
    } else {
        println!("Incorrect");
        0
    }
}

// Main function to welcome the user and start the loop
fn main() {
    // Welcome message
    println!("Hello, and welcome to the Apple Inc. questionare");
    println!("    ");

    // Rules
    println!("Here is the rule: ");
    println!("You are to answer by the number corrosponding to the answer you think is correct");
    println!("    ");

    // User name
    println!("Please input your name");
    // User input for name
    let mut usr_name = get_string();
    while usr_name == "" {
        println!("Well I cant address nothing, try again");
        usr_name = get_string();
    }

    loop {
        // The total points at the end of the questionare
        let mut total_points = 0;

        // Printing out user's name
        println!("Welcome, {usr_name}");
        println!("    ");

        // Questions
        println!("Alright let the game begin!");
        println!("    ");

        let mut quest_options = [false, false, false, false, false];

        // For the 5 questions in the list
        for x in 0..5 {
            // Choosing which question randomly
            let mut choice = rand::thread_rng().gen_range(0..5);

            // Seeing if we have done that question
            while quest_options[choice] == true {
                choice = rand::thread_rng().gen_range(0..5);
            }

            let y = x + 1;

            // Question 1
            if choice == 0 {
                println!("Question {y}");
                total_points += quest_one();
                println!("    ");
            // Question 2
            } else if choice == 1 {
                println!("Question {y}");
                total_points += quest_two();
                println!("    ");
            // Question 3
            } else if choice == 2 {
                println!("Question {y}");
                total_points += quest_three();
                println!("    ");
            // Question 4
            } else if choice == 3 {
                println!("Question {y}");
                total_points += quest_four();
                println!("    ");

            // Question 5
            } else if choice == 4 {
                println!("Question {y}");
                total_points += quest_five();
                println!("    ");
            } else {
                println!("What the hell happened?");
            }
            quest_options[choice] = true;
        }

        // Grammatically correct total point show
        if total_points == 1 {
            println!("You got {total_points} point");
        } else if total_points == 5 {
            println!("Hey you got all {total_points} points");
        } else {
            println!("You got {total_points} points");
        }

        // Asking user if they want to play it again
        println!("Would you like to try again? (Yes/No)");
        let contin = get_string().to_ascii_lowercase();
        if !(contin == "y".to_owned()) && !(contin == "yes".to_owned()) {
            println!("Goodbye");
            break;
        } else {
            println!("   ");
        }
    }
}
