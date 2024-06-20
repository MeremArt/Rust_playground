use std::io;
use sha2::{Digest, Sha256};
use std::fmt;
use std::time::{SystemTime, UNIX_EPOCH};
use std::thread;
use std::time::Duration;
use chrono::{NaiveDateTime, DateTime, Utc};
mod my_module;
use std::io::Write;
mod convert_temp;
mod blockchain_mining;
fn main() {


blockchain_mining::main_block();

}
fn my_function(x: i32, y:i32) -> i32 {

    println!("The value of x is:{}",x);
    println!("The value of y is: {}",y);
x + y
}

fn array_game() {
    let a = [1,2,3,4,5];
    println!("please enter an array index");
    let mut index = String::new();
    io::stdin().read_line(&mut index).expect("failed to read line");

    let  index:  usize = index.trim().parse().expect("index entered was not a number");

    let _element = a[index];
    println!("The value of the element index {index}");
}

fn for_loop() {
    // let a =[10,20,30,40,50];

    // for element in a {
    //     println! ("the value is: {element}")
    // }

    // let x = 5;
    // let y = &x; // y is a reference to x

    // println!("x: {}, y: {}", x, y);
    let mut x = 5;
    let y = &mut x; // y is a mutable reference to x

    *y += 1; // Dereference y to modify the value of x

    println!("x: {}", x);
}

fn get_input(prompt: &str) -> f64 {
    loop {
        print!("{}", prompt);
        io::stdout().flush().unwrap();

        let mut input = String::new();
        io::stdin().read_line(&mut input).expect("Failed to read line");

        match input.trim().parse::<f64>() {
            Ok(value) => return value,
            Err(_) => println!("Invalid input, please enter a number."),
        }
    }
}

fn print_value(x: &i32) {
    println!("Value: {}", x);
}

fn _immutable() {
    let value = 10;
    let value_ref = &value; // Immutable borrow
    print_value(value_ref); // Pass the immutable reference to the function
    println!("Original value: {}", value); // Can still use `value` directly
}

fn increment_value(x: &mut i32) {
    *x += 1;
}

fn _mutable() {
    let mut value = 10;
    {
        let value_ref = &mut value; // Mutable borrow
        increment_value(value_ref); // Pass the mutable reference to the function
    } // Mutable borrow ends here
    println!("Incremented value: {}", value); // Now we can use `value` again
}
