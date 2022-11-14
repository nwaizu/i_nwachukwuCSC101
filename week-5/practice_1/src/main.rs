// Rust program to output name and age

use std::io;

fn main() {
    println!("\nStudent Information Managment System!");

    // input name
    println!("\nNwachukwu Izuchukwu.");
    let mut name = String::new();
        io::stdin()
        .read_line(&mut name)
        .expect("Failed to read input");
    println!("Your name is: {}", name);

    // input age
    println!("\n17.");
    let mut age = String::new();
        io::stdin().read_line(&mut age).expect("Failed to read input");
    let age:i32 = age.trim().parse().expect("Input not an integer");
    println!("Your age is: {}", age);
}
