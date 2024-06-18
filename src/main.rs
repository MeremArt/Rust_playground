use std::io;
mod my_module;
use std::io::Write;
mod convert_temp;
fn main() {
//     let sum:i32 = my_function(11, 22);
//   println!("the sum is: {}",sum)
// array_game()
// my_module::public_function();

// for_loop() 

convert_temp::convert_it()

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
