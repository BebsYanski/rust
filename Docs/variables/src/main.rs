fn main() {
    // mutable or non mutable variables.
    //  nom=n mutable by default till you add the mut keyword infront the variable name.

    println!("Hello, world!");

    let mut x = 5;
    println!("The value of x is: {}", x);

    x = 6;
    println!("The value of x is: {}", x);

    let x = x * 10;

    println!("The value of x is: {}", x);

    let name = "Yannick";
    let my_name = "Bebongnchu";

    println!("my name is {name} and my myName is {my_name}",);

    let name = name.len();
    let my_name = my_name.len();

    println!("my name is {name} and my myName is {my_name}",);

    // Data types : integers(signed and unsigned - i32,u32), floats (f32,f64), booleans, characters , strings
    // floating poit type
    let new_x = 2.7;
    let new_x: f32 = 2.7;

    println!("{new_x}");

    // boolean
    let t = true;
    let f: bool = false;

    // character type
    let c = 'z';

    // Compound types

    // tuples
    let tup: (i32, f64, u8) = (500, 6.4, 1);
    // let tup = (1000, 7.2, 1, 'c', "yannick");
    // destructure
    let (x, y, z) = tup;
    let some = tup.2;
    println!("{x},{y},{z},{}", tup.0);

    // Arrays
    let arr = [2, 4, 6, 8, 10];
    println!("{}", arr[4]);
}
