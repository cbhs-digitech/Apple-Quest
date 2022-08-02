use std::io::stdin;

fn get_int() -> i64 {
    loop {
        let mut line = String::new();
        stdin().read_line(&mut line).expect("Failed to read line");

        let line = match line.trim().parse() {
            Ok(num) => num,
            Err(_) => {
                println!("Sorry that's not a number");
                continue;
            }
        };
        return line;
    }
}

fn main() {
    println!("Hello, and welcome to the Apple Inc. questionare");
    println!("    ");
    println!("Please input your name");

    let mut usr_name = String::new();
    stdin()
        .read_line(&mut usr_name)
        .expect("Failed to read user name");
    let usr_name: String = usr_name.trim().parse().unwrap();
    println!("Welcome, {usr_name}");

    let num = get_int();
    println!("{num}");
}
