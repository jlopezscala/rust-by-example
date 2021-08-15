use std::fmt::Display;
use std::fmt;


fn main() {
    println!("Hello, world!");
    println!("I'm a Rustacean!"); // Basic print lines into the console

    println!("I'm {} years old", 27); // Will stringify 27 and replace in the {}, 27 will default to i32
    println!("I was born in {}", 1994i64); // Same as above but 1994 in this case will be forced to i64

    println!("My name is {0} {1}", "Julian", "Lopez"); // Will fill with positional arguments

    println!("I'm currently coding {current_language}, but learning {learning_language}",
             current_language = "Python",
             learning_language = "Rust"
    ); // We can also use named parameters

    println!("We can also get {:b} formatted", 1); // This will format 1 into binary

    // Activities:
    // 1- Fix the two issues in the above code (see FIXME) so that it runs without error
    // Create a structure named `Structure` which contains an `i32`.
    #[allow(dead_code)]
    struct Structure(i32);

    impl Display for Structure {
        // This is king of implementing toString in Java or __str__ in Python.
        // See also implementing Debug.
        // Implements Display for the Structure struct created, without this implementation
        // print cannot be called to a Struct
        // Below we define fmt trait for Structure, which ends up implementing ToString for the struct
        fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
            // Write strictly the first element into the supplied output
            // stream: `f`. Returns `fmt::Result` which indicates whether the
            // operation succeeded or failed. Note that `write!` uses syntax which
            // is very similar to `println!`.
            write!(f, "{}", self.0)
        }
    }
    println!("This struct `{}` prints...", Structure(3));


    // 2- Add a println! macro that prints: Pi is roughly 3.142 by controlling the number of
    // decimal places shown. For the purposes of this exercise, use let pi = 3.141592 as an
    // estimate for pi. (Hint: you may need to check the std::fmt documentation for setting the
    // number of decimals to display)
    let pi = 3.141592;

    println!("Pi is roughly {0:.3}", pi); // This will tell Rust to only print 3 decimals after the ,
}
