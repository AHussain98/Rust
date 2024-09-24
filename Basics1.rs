use std::env;

// by definition, once a variable is initialised, it cant be changed 
// ! means macro invocation, not a function call
// println! is a macro that prints text to the console

// let statement declares a local variable
// if a function body ends with an expression that is not followed by a semicolon, that's the function's return value
// #[test] attribute above function definition means its a test function, skipped in normal compilation

// Vec is Rust's growable vector type 

// variables are immutable by default
// when a variable is immutable, once a value is bound to a name, you cant change that value

// adding mut infront of variable decleration makes them mutable

// let y : u64 = 1;  // wont work, you have to use const or static for global variables

// constants are valid for the entire time a program runs, within the scope in which they were declared

// you can delcare a new variable with the same name as a previous variable, the first variable is shadowed by the second, which means the second variable is what the compiler will see when you use the name of the variable

// Data Types:
// rust is a statically typed language so we must know the types of all variables at compile time
// compiler can infer what type we want based on the value and how we use it
// we can add type annotations like : u32

// a scalar type represents a singular value, these are ints, floats, bools and chars
// ints can be signed or unsigned
// u32 takes up 32 bits of space
// isize and usize depends on the architechure of the machine, 64 bits if on a 64 bit architecture

/* Let’s say you have a variable of type u8 that can hold values between 0 and 255. If you try to change the variable to a value outside that range, such as 256, integer overflow will occur, which can result in one of two behaviors. */
// if overflow occurs, Rust performs two’s complement wrapping. In short, values greater than the maximum value the type can hold “wrap around” to the minimum of the values the type can hold. In the case of a u8, the value 256 becomes 0, the value 257 becomes 1, and so on. The program won’t panic, but the variable will have a value that probably isn’t what you were expecting it to have. Relying on integer overflow’s wrapping behavior is considered an error.

/* Rust also has two primitive types for floating-point numbers, which are numbers with decimal points. Rust’s floating-point types are f32 and f64, which are 32 bits and 64 bits in size, respectively. The default type is f64 because on modern CPUs, it’s roughly the same speed as f32 but is capable of more precision.  */

// compound types can group multiple values into one type, rust has 2 primitive compound types, tuples and arrays

// tuple is a general way of grouping together a number of values with a variety of tyes into one compound type
// tuples have a fixed length, once declared they cannot grow or shrink in size

// arrays, every element must be of the same type

// main function is the entry point of the program
// fn keyword allows us to declare new functions

fn another_function(x : i32)
{
    println!("This is another function!");
}

// Rust doesnt care where we define the function, only that they can be seen by the caller
// lines execute in the order they appear in main()

// in function signatures, you must declare the type of each parameter

// statements are instructions that perform some action and do not return a value
// expressions evaluate to a resultant value, they do not end in semi-colon
// if you add a semicolon to the end of a statement, you turn it into an expression and it will not return a value

// functions can also return values

fn returning_function() -> u32{  // return type is 32
    5  // return value of the fucntion is the result of the final expression in the block
    // notice there is no semicolon, this is an expression!! not a statement
}








fn main() {

    let t : u64 = 100;  // immutable
    println!("the value of t is {t}");
   // t = 101;  // error, cannot assign twice to immutable value
    println!("the value of t is {t}");


    let mut numbers = Vec::<u64>::new();
    println!("Hello, world!");

    const x : u64 = 10;  // const, immutable by default, always immutable
    // constants can be declared in any scope, even global scope
    // constants may be set only to a constant expression, not the result of a value that can only be computed at runtime
    const HOURS_IN_SECONDS: u32 = 60 * 60;  // doesnt need to be calculated at runtime, so valid

    // shadowing, reusing the name of a variable within scope
    let y : u64 = 1;
    let y = y + 1;  // to shadow, we have to use the let keyword
    println!("the value of y is {y}");  //   shadowed successfully

    let spaces = "   ";
    let spaces = spaces.len();  // shadowing also lets us change type
    // using mut here would cause an error since we couldn't mutate a variables type

    let tup : (u32, String, bool) = (500, "hi everyone".to_string(), true);  // tuple
    // we can get the elements from a tuple by pattern matching
    let (x1,y1,z1) = tup;
    let boolean = z1;
    // we can also access elements in the tuple using the . operator
    let first_element = tup.0;  // first element

    let arr1 = [1,2,3,4,5];
    // arrays are useful when we want our data allocated on the stack rather than the heap
    // arrays do not grow in size, vectors do and are allocated on the heap
    let arr2: [i32;5] = [6,7,8,9,10];  // explicitly show the type and number of elements
    let arr3 = [5; 10];  // contains 5 elements each with value 10

    // When you attempt to access an element using indexing, Rust will check that the index you’ve specified is less than the array length. If the index is greater than or equal to the length, Rust will panic. This check has to happen at runtime, especially in this case, because the compiler can’t possibly know what value a user will enter when they run the code later.

    // This is an example of Rust’s memory safety principles in action. In many low-level languages, this kind of check is not done, and when you provide an incorrect index, invalid memory can be accessed. Rust protects you against this kind of error by immediately exiting instead of allowing the memory access and continuing.  
    
    another_function(5);

    let num = returning_function();
    println!("value returned is {num}");
    
    if num < 10 
    {
        println!("num is less than 10!");
    }
    
    // rust will not automatically convert non boolean types to boolean, like C++ does with ints
    
    // we can use if in a let statement
    let number = if num > 10 {1} else {0};  // here the value of number is conditional
    
    let mut count = 0;
   let result = loop {   // loop keyword will loop forever until a break, loops can return values in Rust
        println!("in the loop!");
        if count == 5
        {
            break count;  // counter is returned, anything after the break can be returned from the loop
        }
        count += 1;
        
    };

    // these loops can have labels with names, so you can choose which loop to break when there are nested loops 

    // rust also has traditional for and while loops
    for element in arr1{
        println!("{element}");
    }
}
