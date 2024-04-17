use std::io;

// rust variables are immutable by default
// when a variable is immutable, once a value is bound to a name, you cant change that value
// mut keyword used to create changeable variables and indicate intention

// constants are values that are bound to a name and are not allowed to change 
// you cant use mut with constants, theyre always immutable
// constants require data type to be specified and can exist in global scope

const HOURS_IN_SECONDS: u32 = 60 * 60;

// Rust lets us declare a new variable with the same name as a previous variable
// we say the first variable is shadowed by the second, which means the second is what the compiler sees when you use the name of the variable
// we can shadow a variable by using the same name and repeat the use of the let keyword
/*
 In effect, the second variable overshadows the first, taking any uses of the variable name to itself until either it itself is shadowed or the scope ends.
*/

/*
shadowing is different from marking variable as mut because we get compile time error if we try to reassign to this variable without using the let keyword
let allows us to perform transformations on value but ave the variable be immutable after those transformations are complete
The other difference between mut and shadowing is that because we’re effectively creating a new variable when we use the let keyword again, we can change the type of the value but reuse the same name.
*/

/*
every value in rust is of a certain data type
rust is statically typed, types must be known at compile time
compiler can usually infer what type we want to use based on the value and how its used

scalar type represents a single value, rust has 4 primary scalar types: 
integers, floats, bools and chars

integer types are numbers without fractional components
i16 is signed int 16 bit, i32 is the default int type 
u32 is unsigned int 32 bit
Each signed variant can store numbers from -(2n - 1) to 2n - 1 - 1 inclusive, where n is the number of bits that variant uses. So an i8 can store numbers from -(27) to 27 - 1, which equals -128 to 127. Unsigned variants can store numbers from 0 to 2n - 1, so a u8 can store numbers from 0 to 28 - 1, which equals 0 to 255.

isize and usize are automatic sizes based on computer architecture

Let’s say you have a variable of type u8 that can hold values between 0 and 255. If you try to change the variable to a value outside that range, such as 256, integer overflow will occur, which can result in one of two behaviors. When you’re compiling in debug mode, Rust includes checks for integer overflow that cause your program to panic at runtime if this behavior occurs. Rust uses the term panicking when a program exits with an error
When you’re compiling in release mode with the --release flag, Rust does not include checks for integer overflow that cause panics. Instead, if overflow occurs, Rust performs two’s complement wrapping. In short, values greater than the maximum value the type can hold “wrap around” to the minimum of the values the type can hold. In the case of a u8, the value 256 becomes 0, the value 257 becomes 1, and so on. The program won’t panic, but the variable will have a value that probably isn’t what you were expecting it to have. Relying on integer overflow’s wrapping behavior is considered an error.

floating point type is f32 and f64, f32 is single precision float, f64 is double

bool is only one byte in size

char is with ' ', not "", and represents 4 bytes

Integer division truncates toward zero to the nearest integer so -5/-3 is -1

compound types can group multiple values into one type, rust has two primitive compound types : tuples and arrays
tuple is a general way of grouping together a number of values with mukltiple types into one compound types
tuples have a fixed size, they cannot grow or shrink once declared

every element of an array must be of the same type
arrays in rust have a fixed length


*/



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

    let y = 9;
    let y = y + 1;  // this replaces the previous y, value is now 10
    {
        let y = y + 1;  // this shadow only exists in this scope
        println!("this is y in the scope: {y}");  // can use {} like in python
    }
    println!("this is y outside the scope: {y}");  // returns to being 10

    let spaces = " ";  // string type
    let spaces = spaces.len();  // change to number type, reuse the same name
    
    // if a type is mut, you are not allowed to shadow it
    
    let num : u32 = 64;
    let floating_num = 64.00;  // this is f64 by default

    let my_tup = (1.2, 55, "string", 'c');  // tuple, can have type annotations if wanted but not necessary
    /*
    The variable tup binds to the entire tuple because a tuple is considered a single compound element. To get the individual values out of a tuple, we can use pattern matching to destructure a tuple value
     */

    let (w1,x1,y1,z1) = my_tup;  // unpack the tuple with a pattern using let
    println!("The third value in the tuple is {y1}");

    // we can also access tuple elements using . operator

    let second_element = my_tup.1;  

    let my_arr = [1,2,3,4,5];
    // arrays are allocated on the stack rather than the heap, and we want to ensure a fixed number of elements
    // vector is similar but it is allowed to grow or shrink in size
    // arrays are more useful and efficient when we know the number of elements will not change

    let typed_arr : [i32;3] = [1,2,3];  // typed array, 3 ints

    let auto_arr = [3; 5];  // array of 5 elements all with value 3

    // arrays are fixed size and on the stack, so can be accessed via index using []
    /*
    When you attempt to access an element using indexing, Rust will check that the index you’ve specified is less than the array length. If the index is greater than or equal to the length, Rust will panic. This check has to happen at runtime because the compiler can’t possibly know what value a user will enter when they run the code later.
    This is an example of Rust’s memory safety principles in action. In many low-level languages, this kind of check is not done, and when you provide an incorrect index, invalid memory can be accessed. Rust protects you against this kind of error by immediately exiting instead of allowing the memory access and continuing. */
}
