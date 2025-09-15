fn main() {
    let mut name = "Yannick"; // using mut to make it mutable
    println!("Hello, world! {name}");

    name = "Bebongnchu Yannick";

    let first_name = "Bebongnchu";
    let last_name = "Yannick";

    println!("Hello, world! {name}");

    println!("Hello, {name}!"); // macros are represented by the !

    println!("Hello, {} {last_name}", first_name);

    let data = [1, 2, 3, 4];

    println!("{data:?}");
    // println!("{}", );
}
