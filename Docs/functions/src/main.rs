fn main() {
    println!("Hello, world!");
    another_function(23);
    let sum = add(5, 6);
    println!("The sum is: {}", sum);
    let x = five();
    println!("The value of x is: {}", x);
}

fn another_function(x: i32) {
    println!("Another function");
    println!("The value of x is: {}", x);
}
fn add(x: i32, y: i32) -> i32 {
    x + y
}
fn five() -> i32 {
    5
}
