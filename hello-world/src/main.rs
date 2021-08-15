use std::fmt::{Display, Formatter};
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

    // Implement the Display
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

    // ##############################################################################

    // 2- Add a println! macro that prints: Pi is roughly 3.142 by controlling the number of
    // decimal places shown. For the purposes of this exercise, use let pi = 3.141592 as an
    // estimate for pi. (Hint: you may need to check the std::fmt documentation for setting the
    // number of decimals to display)
    let pi = 3.141592;

    println!("Pi is roughly {0:.3}", pi); // This will tell Rust to only print 3 decimals after the ,

    // ##############################################################################

    // DEBUG Trait

    // Contrary to the Display trait, the Debug one can be easily implemented:
    #[derive(Debug)]
    struct Structure2(i32);

    // Even for nested structs, will make both printable with :? formatter
    #[derive(Debug)]
    struct Deep(Structure2);

    // so now this works
    println!("My Structure2: {:?}", Structure2(2));

    // and more pretty
    println!("My Structure2 pretty: {:#?}", Structure2(2));

    // ##############################################################################
    // Implementing Display vs Debug
    // A structure holding two numbers. `Debug` will be derived so the results can
    // be contrasted with `Display`.
    #[derive(Debug)]
    struct MinMax(i64, i64);

    // Implement `Display` for `MinMax`.
    impl fmt::Display for MinMax {
        fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
            // Use `self.number` to refer to each positional data point.
            write!(f, "({}, {})", self.0, self.1)
        }
    }

    let big_range =   MinMax(-300, 300);
    let small_range = MinMax(-3, 3);

    // This should work now!
    println!("The big range is {big} and the small is {small}",
             small = small_range,
             big = big_range);

    // Define a structure where the fields are nameable for comparison.
    #[derive(Debug)]
    struct Point2D {
        x: f64,
        y: f64,
    }
    // Similarly, implement `Display` for `Point2D`
    impl fmt::Display for Point2D {
        fn fmt(&self, f: &mut Formatter) -> fmt::Result {
            // Customize so only `x` and `y` are denoted.
            write!(f, "x: {}, y: {}", self.x, self.y)
        }
    }

    let point = Point2D { x: 3.3, y: 7.2 };

    // Both of these work now since Debug and Display are implemented
    println!("Compare points:");
    println!("Display: {}", point);
    println!("Debug: {:?}", point);

    // Activity
    // After checking the output of the above example, use the Point2D struct as a guide
    // to add a Complex struct to the example. When printed in the same way,
    // the output should be:
    // Display: 3.3 + 7.2i
    // Debug: Complex { real: 3.3, imag: 7.2 }

    #[derive(Debug)]
    struct Complex {
        real: f64,
        imag: f64,
    }
    impl fmt::Display for Complex {
        fn fmt(&self, f: &mut Formatter) -> fmt::Result {
            // Customize so only `x` and `y` are denoted.
            write!(f, "{}, {}i", self.real, self.imag)
        }
    }
    let complex = Complex { real: 3.3, imag: 7.2};
    println!("Display: {}", complex);
    println!("Debug: {:?}", complex);

}
