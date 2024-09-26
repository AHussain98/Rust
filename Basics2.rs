// Ownership is what enables rust to make memory safe guarantees without needing a garbage collector
// ownership is a set of rules that govern how a rust program manages memory
// rust doesnt have a garbage collector nor does the programmer explicitly allocate and free memory
// Rust uses a third approach: memory is managed through a system of ownership with a set of rules that the compiler checks. If any of the rules are violated, the program won’t compile. None of the features of ownership will slow down your program while it’s running.

// stack and heap, these are different parts of memory available for code to use at runtime
// stack stores objects first in first out, its foxed size and what you store on it must have a fixed size at runtime
// The heap is less organized: when you put data on the heap, you request a certain amount of space. The memory allocator finds an empty spot in the heap that is big enough, marks it as being in use, and returns a pointer, which is the address of that location. This process is called allocating on the heap
// pointer to heap is a known fixed size, so can be stored on the stack

// pushing to the stack is faster than allocating to the heap because you dont have to search for a new piece to memory to use for storage, its always at the top of the stack
// accessing data on the heap is slower because you have to follow a pointer to get there
// processors are faster if they don't have to jump around in memory

//  the main purpose of ownership is to manage heap data 

// each value in rust has an owner
// there can only be one owner at a time
// when the owner goes out of scope, the value will be dropped

// string literals are stored on the stack, they're immutable
// String types are stored on the heap

// String can be mutated, the literal cannot
// in the case of a string literal, we know the contents at compile time, so the text is hardocded directly into the final executable
// this is why string literals are fast and efficient, but these properties only come from the string literals immutability

// with the String type, we allocate a an amount of memory on the heap at runtime to hold the contents
// when we call String::from, its implementation requests the memory it needs
// when it comes time to free the heap memory, the memory is automatically returned once the variable that owns it goes out of scope

// when a heap variable goes out of scope, Rust calls a special function for us called drop() which is where the class author puts code for returning memory
// rust calls drop automatically at the closing curly bracket

// rust essentially implements RAII through drop()

/*
Rust has a special annotation called the Copy trait that we can place on types that are stored on the stack, as integers are. If a type implements the Copy trait, variables that use it do not move, but rather are trivially copied, making them still valid after assignment to another variable.

Rust won’t let us annotate a type with Copy if the type, or any of its parts, has implemented the Drop trait. If the type needs something special to happen when the value goes out of scope and we add the Copy annotation to that type, we’ll get a compile-time error.

types that implement the copy trait are copied trivially,not moved, so can still be used after assignment, these are stored on the stack
examples are ints, bools, floats, char, tuples (if they only contains copy types)
*/

// we can also use references when passing variables to functions, to avoid transfer of ownership
// A reference is like a pointer in that it’s an address we can follow to access the data stored at that address; that data is owned by some other variable. Unlike a pointer, a reference is guaranteed to point to a valid value of a particular type for the life of that reference.

fn calculate_length(s : &String) -> usize  // reference, refer to some value without taking ownership
{
    s.len();  
}  // reference is non-owning, s is not dropped when going out of scope

// we call the action of creating a reference borrowing
// we cannot modify something we are borrowing
// just as variables are immutable by default, so are references
// we can have mutable references

fn change(some_string: &mut String)  // function accepts a mutable reference
{
    some_string.push_str(", world");
}

// mutable references can singular, you can only have one mutable reference to a value at a time
/*
The restriction preventing multiple mutable references to the same data at the same time allows for mutation but in a very controlled fashion. It’s something that new Rustaceans struggle with because most languages let you mutate whenever you’d like. The benefit of having this restriction is that Rust can prevent data races at compile time. A data race is similar to a race condition and happens when these three behaviors occur:

Two or more pointers access the same data at the same time.
At least one of the pointers is being used to write to the data.
There’s no mechanism being used to synchronize access to the data.
Data races cause undefined behavior and can be difficult to diagnose and fix when you’re trying to track them down at runtime; Rust prevents this problem by refusing to compile code with data races!

you can have multiple immutable references, but only if there are no mutable ones
*/

fn make_change(some_string: &mut String)  // pass in a mutable reference
{
    some_string.push_str(", added!");
}


/*
At any given time, you can have either one mutable reference or any number of immutable references.
References must always be valid, rust will not allow dangling references.
*/

// slices let you reference a contigous sequence of elements in a collection rather than the whole collection
// slice is like a reference, it does not have ownership


fn main() {

    let s = "hello";  // s is a hardcoded string literal, the variable is valid at the point it is declared until it goes out of scope
    let mut s2 = String::from("hello");  // String type, stored on heap and created via literal
    s2.push_str(" world!");  // push_str() appends to string

    println!("{s2}");  

    let x = 5;
    let y = x;
    // y is a copy of x, these two values are pushed onto the stack, they are simple integers with a known fixed size
    // anything we know the size of at compile time is automatically stored on the stack, which means it is trivially copyable
    // types such as integers that have a known size at compile time are stored entirely on the stack, so copies of the actual values are quick to make. That means there’s no reason we would want to prevent x from being valid after we create the variable y. In other words, there’s no difference between deep and shallow copying here, so calling clone wouldn’t do anything different from the usual shallow copying, and we can leave it out.


    let mut s3 = s2;
    // the same does not happen here
    // A String is made up of three parts, a pointer to the memory that holds the contents of the string, a length, and a capacity. This group of data is stored on the stack.
    // s3 and s2 point to the same heap memory
    // here we have performed a copy of the pointer, but not of the data on the heap

    // when a variable goes out of scope, rust automatically calls the drop function and cleans up the heap memory for that variable
    // when s3 and s2 go out of scope, they will both try to free the same memory which is an error
    // to ensure memory safety, after the line let mut s3 = s2;, rust considers s2 as no longer valid
    // therefore rust doesnt need to free anything after s2 goes out of scope

    //println!("{s2}");  // this causes an error, s2 has been "moved" from, in rust terms
    // we say s2 has been moved from and is now no longer valid
    // in reality, the pointer s2 is just no longer valid, no data has actually been moved 
    // this design choice means rust will never automatically create deep copies of data, so automatic copying can be considered cheap

    // if we want to deeply copy the heap data of the string, not just the stack data, we can use the clone method
    let s4 = s3.clone();
    println!("{s3},{s4}");  // both valid, s3 and s4 point to different heap string objects
    
    // clone() means some code is executing which may be expensive, keep an eye out for this

    // the mechanics of passing a value to a function are similar to those when assigning a value to a variable

    takes_ownership(s3);  // s3 is no longer valid in this scope

    makes_copy(x);  // x is stack based, so copied into function, x is still valid here

    // returning values can also transfer ownership back
    
    let s5 = gives_ownership();  

    let s6 = takes_and_gives_ownership(s4);  
    // now s4 is moved into s6, s4 now no longer useable

    let ss1 = String::from("hello");
    let len = calculate_length(&ss1);  // we will refer to ss1 without taking ownership of it
    // &ss1 lets us create a reference that refers to the value of ss1 but does not own it
    // because it does not own it, the value it points to will not be dropped when the reference stops being used
    println!("The length of '{ss1}' is {len}.");
  
    let mut mutstr = String:from("hello");
    make_change(&mut mutstr);  // make it clear mutstr is passed as a mutable reference
  
    // if we have a mutable reference to a value, we can have no other references to that value existing at the same time
    {
        let r1 = &mut mutstr;  //r1 is a mutable reference to mutstr
    }  // r1 goes out of scope here, so we can create another mutable reference to mutstr

    let r2 = &mut mutstr;  // works
    // neat trick, using curly brackets to create a new scope
    // we can have mutiple simultaenous immutable references to a variable, but not an immutable and mutable reference at the same time

    // IMPORTANT NOTE:
    // a references scope starts from where it is introduced and continues until the last time that the reference is used
    let mut mutstr2 = String::from("hello");

    let rr1 = &mutstr2; // no problem
    let rr2 = &mutstr2; // no problem
    println!("{r1} and {r2}");
    // variables r1 and r2 will not be used after this point

    let r3 = &mut mutstr2; // no problem
    println!("{r3}");
    //The scopes of the immutable references r1 and r2 end after the println! where they are last used, which is before the mutable reference r3 is created.
    // These scopes don’t overlap, so this code is allowed: the compiler can tell that the reference is no longer being used at a point before the end of the scope.  
  
    // string slice is reference to part of a string
    let sslice = String::from("Asad Hussain") ;
    let firstname = &sslice[0..4];
    // firstname is a reference to the first part of the string
    // in memory, this is implemented as a pointer to the byte at the String's starting index and a length
    // ending index is one more than the last position in the slice

    //the type that signifies string slice is &str
    /*
    slices help us understand string literals, which are stored inside the binary
    let s = "Hello, world!";
    The type of s here is &str: it’s a slice pointing to that specific point of the binary. This is also why string literals are immutable; &str is an immutable reference.

    Knowing that you can take slices of literals and String values leads us to one more improvement on first_word, and that’s its signature:

fn first_word(s: &String) -> &str {
A more experienced Rustacean would write the signature shown below instead because it allows us to use the same function on both &String values and &str values.

fn first_word(s: &str) -> &str {
 Improving the first_word function by using a string slice for the type of the s parameter

If we have a string slice, we can pass that directly. If we have a String, we can pass a slice of the String or a reference to the String.

you can also have slices of arrays, and other contigous types
    */

  }  // drop is called here for s2, x and y are popped off stack first


fn takes_ownership(some_string : String)  // some_string comes into scope
{
    println!("{some_string}");
}  // some_string goes out of scope and drop is called, the memory is freed


fn makes_copy(int : u32)
{
    println!("{int}");  // stack based int, copied, no memory released on heap, drop not called
}


fn gives_ownership() -> String  // creates string and returns it, gives ownership to caller
{
    let s = String::from("Yo!");
    s
}


fn takes_and_gives_ownership(some_string: String) -> String  // takes ownership from caller, modifies string and then returns ownership
{
    some_string  // returned
}
