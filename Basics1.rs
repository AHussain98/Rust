use std::io;

// rust variables are immutable by default
// when a variable is immutable, once a value is bound to a name, you cant change that value
// mut keyword used to create changeable variables and indicate intention

// constants are values that are bound to a name and are not allowed to change 
// you cant use mut with constants, theyre always immutable
// constants require data type to be specified and can exist in global scope

const HOURS_IN_SECONDS: u32 = 60 * 60;

// Rust lets us declare a new variable with the same name as a previous variable

fn main() {  // main fucntion is the entry point into the program, fn declares a new fucntion

    let mut x = 6;
    x = 5;    // error, cannot assign twice to immutable variable, change x definition to mut will allow this

    println!("Guess the number");  // macro that prints to the screen
    println!("Please enter your number");

    let mut guess = String::new();  // let creates a new variable, variables are immutable by default
    let apples = 5;  // immutable

    // string::new() is a function that returns a new instance of a string, strings provided by the standard library

    io::stdin()
        .read_line(&mut guess)
        .expect("Failed to read line");
// &mut tells the function which object to store the result in
// & indicates reference which means no copy of the object
// references are immutable by default
    println!("You guessed: {guess}");
}
