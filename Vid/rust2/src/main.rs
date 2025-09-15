use std::io;

fn main() {
    let first_name = String::from("Bebongnchu"); // use this format if you do not know the value at that time, or it might change or you need to get the value from the user

    let last_name = "Yannick";

    println!("{first_name} {}", last_name);

    println!("Hey, what is your name?");
    let mut name = String::new();

    io::stdin()
        .read_line(&mut name)
        .expect("Failed to read line");
    println!("Your name is: {},  welcome", name.trim());
}
