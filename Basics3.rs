// structs are custom data types, they let us package together and name multiple related values
// structs are similar to tuples, they san store multiple types
// structs mean you have to name each piece of data
// structs typically own the data within them, but they can also store references, but this requires the use of lifetimes

struct User  // struct and fields, struct definition is a general template for the type
{
    active: bool,
    username: String,
    email: String,
    sign_in_count: u64
}

// the entire instance of a struct must be mutable if we want to change something
// Rust does not let us label specific fields as mutable

// function that returns a struct
fn build_user(email : String, username : String) -> User  
{
    User{
        active: true,
        username,  // username and email will be taken from the Strings passed in, this is the syntax for that
        email,
        sign_in_count : 1
    }  // don't put ; because we are returning this
}

// rust also supports structs that look similar to tuples, called tuple structs
// tuple structs dont have names associated with their fields

struct Colour(i32, i32, i32);

// we can also implement structs without any data

struct NoData;


#[derive(Debug)]  // attribute that adds functionality to print out debug information
struct Rectangle {
    width : u32,
    length : u32
}

fn area(rectangle : &Rectangle) -> u32 // take struct by immutable reference (borrow), return u32
{
    rectangle.width * rectangle.length
}
// function only borrows the rectangle, doesnt take ownership of it away from main
// accessing the fields of a borrowed struct does not move the fields, so no movement happens here

// methods are functions defined in the conext of a struct, or enum or trait
// the first parameter is always self, which represents the instance of the struct the method is being called on
// define the method within the context of Rectangle

impl Rectangle {  // impl means implementation block, everything in this block happens within the context of Rectangle
    fn rect_area(&self) -> u32 {  // self is borrowed immutably, method cannot make changes
        self.width * self.length
    }

    fn new(size : u32) -> Self {  // this is the constrictor, Self gets replaced with the type
        Self {
            width: size,
            length : size
        }
    }  // The Self keywords in the return type and in the body of the function are aliases for the type that appears after the impl keyword, which in this case is Rectangle.
}

/*

If we wanted to change the instance that we’ve called the method on as part of what the method does, we’d use &mut self as the first parameter. Having a method that takes ownership of the instance by using just self as the first parameter is rare; this technique is usually used when the method transforms self into something else and you want to prevent the caller from using the original instance after the transformation.

The main reason for using methods instead of functions, in addition to providing method syntax and not having to repeat the type of self in every method’s signature, is for organization. We’ve put all the things we can do with an instance of a type in one impl block rather than making future users of our code search for capabilities of Rectangle in various places in the library we provide.

Methods can take multiple parameters that we add to the signature after the self parameter, and those parameters work just like parameters in functions.

All functions defined within an impl block are associated functions because they're associated with the type after the impl. 

We can define associated functions that don’t have self as their first parameter (and thus are not methods) because they don’t need an instance of the type to work with. We’ve already used one function like this: the String::from function that’s defined on the String type.

A struct can have multiple impl blocks

*/

// Where structs give you a way of grouping together related fields and data, like a Rectangle with its width and height, enums give you a way of saying a value is one of a possible set of values. 

emun IpAddrKind {
    V4,
    V6
}

// IpAddrKind is now a custom data type that we can use elsewhere in our code
// enums can also have their own impl blocks, to create associated functions

//the Option<T> enum allows us to check for null values, the types within it are None or some<T>
// the match expression is a control flow construct that runs code depending on the type of enum it is given


fn main()
{
    // create an instance of the struct, assigning all fields
    let mut u1 = User {  // this instance is mutable
        active : true,
        username : String::from("AHus"),
        email : String::from("Yahoo"),
        sign_in_count : 1
    };
    
    u1.active = false;  // change the mutable type
    
    // we can create instances from other instances with struct update syntax

    let mut u2 = User{
        active : u1.active,  // active and username is taken from user1
        username : u1.username,
        email : String::from("Google"),
        sign_in_count : 1
    };   
    
    // we can do this in an easier way using the .. notation

    let mut u3 = User{
        email : String::from("Hotmail"),
        ..u2  // this means take the rest of the fields from the instance named u2
    };

    // the ..u2 must come last to specify any remaining fields come from it

    // struct update notation uses = like an assignment, it moves the data
    // so we cannot use u2 as a whole after creating u3 because the String in the username of u2 was moved into u3
    // If we had given u3 new String values for both email and username, and thus only used the active and sign_in_count values from u2, then u2 would still be valid after creating u3. 
    // Both active and sign_in_count are types that implement the Copy trait

    let black = Colour(0,0,0);   
    // tuple structs can also be destructed into their constituent fields, like regular tuples

    let colour1 = black.0;

    let rect1 = Rectangle{
        width : 30,
        length : 50
    };

    println!(" the area of the rectangle is {}",area(&rect1));
    
    println!("rect1 is {rect1:?}"); // :? means debug formatting
    // println! is a macro that takes reference to an expression, no ownership

    rect1.rect_area();  // call the member function
    
    let sq = rectangle::new(5);  // call the associated function that acts as a constructor

    let four = IpAddrKind::V4;
    let six = IpAddrKind::V6;




}

fn route(ip_kind: IpAddrKind) {}   // function takes any IpAddrKind type


