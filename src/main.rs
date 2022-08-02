fn get_int() -> i64 {
    loop {
        let mut line = String::new();
        std::io::stdin()
            .read_line(&mut line)
            .expect("Failed to read line");

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
    println!("Hello, world!");
    println!("CBHS");

    let num = get_int();
    println!("{num}");
}
