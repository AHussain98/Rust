// code should be organised into multiple modules and then files
// A package can contain multiple binary crates and optionally one library crate. As a package grows, you can extract parts into separate crates that become external dependencies.

// Packages: a Cargo feature that lets you build, test and share crates
// Crates: a tree of modules that produces a library or executable
// Modules and use: Let you control the organization, scope, and privacy of paths
// Paths: A way of naming an item, such as a struct, function, or module

// a crate is the smallest amount of code that the rust compiler considers at a time
// Crates can contain modules, and the modules may be defined in other files that get compiled with the crate

// A crate can come in one of two forms: a binary crate or a library crate. Binary crates are programs you can compile to an executable that you can run, such as a command-line program or a server. Each must have a function called main that defines what happens when the executable runs. 

// Library crates don’t have a main function, and they don’t compile to an executable. Instead, they define functionality intended to be shared with multiple projects. 
// rust users say crate when they mean library, essentially

// the crate root is a source file that the Rust compiler starts from and makes uo the root module of your crate

// a package is a bundle of crates that provides a set of functionality
//  A package contains a Cargo.toml file that describes how to build those crates. Cargo is actually a package that contains the binary crate for the command-line tool you’ve been using to build your code. The Cargo package also contains a library crate that the binary crate depends on

// a package can contain as many binary crates as we want but must only contain one library crate

// Cargo follows a convention that src/main.rs is the crate root of a binary crate with the same name as the package. Likewise, Cargo knows that if the package directory contains src/lib.rs, the package contains a library crate with the same name as the package, and src/lib.rs is its crate root. Cargo passes the crate root files to rustc to build the library or binary.

// The compilation model centers on artifacts called crates. Each compilation processes a single crate in source form, and if successful, produces a single crate in binary form: either an executable or some sort of library

// A crate is a unit of compilation and linking, as well as versioning, distribution, and runtime loading. A crate contains a tree of nested module scopes. The top level of this tree is a module that is anonymous (from the point of view of paths within the module) and any item within a crate has a canonical module path denoting its location within the crate’s module tree.
// A Rust source file describes a module, the name and location of which — in the module tree of the current crate — are defined from outside the source file: either by an explicit Module item in a referencing source file, or by the name of the crate itself. Every source file is a module, but not every module needs its own source file: module definitions can be nested within one file.
// A crate that contains a main function can be compiled to an executable. If a main function is present, it must take no arguments, must not declare any trait or lifetime bounds, must not have any where clauses, and its return type must implement the Termination trait.

// the use keyword brings a path into scope, the pub keyword makes items public

// When compiling a crate, the compiler first looks in the crate root file (usually src/lib.rs for a library crate or src/main.rs for a binary crate) for code to compile.
//  In the crate root file, you can declare new modules; say you declare a “garden” module with mod garden;. The compiler will look for the module’s code in these places:
//       Inline, within curly brackets that replace the semicolon following mod garden
//       In the file src/garden.rs
//       In the file src/garden/mod.rs
// In any file other than the crate root, you can declare submodules. For example, you might declare mod vegetables; in src/garden.rs. The compiler will look for the submodule’s code within the directory named for the parent module in these places:
//       Inline, directly following mod vegetables, within curly brackets instead of the semicolon
//       In the file src/garden/vegetables.rs
//       In the file src/garden/vegetables/mod.rs
// Once a module is part of your crate, you can refer to code in that module from anywhere else in that same crate, as long as the privacy rules allow, using the path to the code. For example, an Asparagus type in the garden vegetables module would be found at crate::garden::vegetables::Asparagus.

// Code within a module is private from its parents modules by default.
//  To make a module public, declare it with pub mod instead of mod. To make items within a public module public as well, use pub before their declarations.

// Within a scope, the use keyword creates shortcuts to items to reduce repetition of long paths. In any scope that can refer to crate::garden::vegetables::Asparagus, you can create a shortcut with use crate::garden::vegetables::Asparagus; and from then on you only need to write Asparagus to make use of that type in the scope.
// By default, functions, types, constants and modules are private
// the pub keyword makes an item public and therefore visible outside its namespace

pub mod Garden;  //  this line tells the compiler to include the code it finds in src/garden.rs

use crate::Garden::Vegetables::Asparagus;  // now we no longer have to include the whole path to use Asparagus

// Modules allow us to organise code within a crate for readability and reuse
// Modules also allow us to control the privacy of items because code within a module is private by default

// Earlier, we mentioned that src/main.rs and src/lib.rs are called crate roots. The reason for their name is that the contents of either of these two files form a module named crate at the root of the crate’s module structure, known as the module tree.

// An example of a structured crate into nested modules
mod front_of_house{  // define a module and then the modules within it after the curly braces

    mod hosting{  // hosting is not pub, so is provate to the front_of_house module
        fn seat_at_table() {}  //  remember that making the module public does not mean that the contents are public
    }

    pub mod serving {  // serving is public
        pub fn take_order(){
            super::external_func();  // use super keyword to access parent module scope, just like python
        }
    }

    pub fn external_func() {}
}

// hosting and serving are siblings, they are defined in the same module, they're both children of the same module
// front_of_house is the child of the implicit module called crate


// we can use paths -> analogous to filesystem
// to use a function, we must use its path
/*
An absolute path is the full path starting from a crate root; for code from an external crate, the absolute path begins with the crate name, and for code from the current crate, it starts with the literal crate.
A relative path starts from the current module and uses self, super, or an identifier in the current module.
*/
// Items in a parent module can’t use the private items inside child modules, but items in child modules can use the items in their ancestor modules. This is because child modules wrap and hide their implementation details, but the child modules can see the context in which they’re defined. 

/* We can also use pub to designate structs and enums as public, but there are a few extra details to the usage of pub with structs and enums. If we use pub before a struct definition, we make the struct public, but the struct’s fields will still be private.
We can make each field public or not on a case-by-case basis. */

// Adding use and a path in a scope is similar to creating a symbolic link in the filesystem. By adding use crate::front_of_house::hosting in the crate root, hosting is now a valid name in that scope, just as though the hosting module had been defined in the crate root. Paths brought into scope with use also check privacy, like any other paths.
// the use keyword just brings in whatever we specify into the current scope
// we can use the as keyword to rename any modules or types in local scope

// anything we bring into scope with use is then private to the module we bring it into scope to
// we can make it public again by using pub use

/*
Members of the Rust community have made many packages available at crates.io, and pulling any of them into your package involves these same steps: listing them in your package’s Cargo.toml file and using use to bring items from their crates into scope.

Note that the standard std library is also a crate that’s external to our package. Because the standard library is shipped with the Rust language, we don’t need to change Cargo.toml to include std. But we do need to refer to it with use to bring items from there into our package’s scope. For example, with HashMap we would use this line:

use std::collections::HashMap;
This is an absolute path starting with std, the name of the standard library crate.

paths can be nested
use std::io::{self, Write};

This line brings std::io and std::io::Write into scope.

If we want to bring all public items defined in a path into scope, we can specify that path followed by the * glob operator:

use std::collections::*;

we can put modules into seperate files and then use a mod declaration to bring that code into scope
Note that you only need to load a file using a mod declaration once in your module tree. Once the compiler knows the file is part of the project (and knows where in the module tree the code resides because of where you’ve put the mod statement), other files in your project should refer to the loaded file’s code using a path to where it was declared

The mod keyword declares modules, and Rust looks in a file with the same name as the module for the code that goes into that module.

*/


fn main() {
    println!("Hello, world!");

    let a1 = Asparagus{};

    crate::front_of_house::serving::take_order();  // call the public function
}
