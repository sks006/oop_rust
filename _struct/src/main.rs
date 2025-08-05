//there is 3 structs
/*
1. Tuple structs, which are, basically, named tuples and they have no named fields.
   => tuple structure has () first brackets.
   Example: struct Color(u8, u8, u8);
   Usage: let black = Color(0, 0, 0);

2. The classic C structs and C++ structs, which are, basically, collections of named fields.
    Example: struct Employ { name: String, salary:i32 , working_year: f64, age: u32,
    active: bool, };
    Usage: let emp = Employ { name: "John", salary: 10000, working_year: 5.0, age: 30, active: true };

2. Unit structs, which are field-less, are useful for generics zero-sized types in general.
    => takes no memory at runtime
    => if you don't want to rewrite the same type of code again & again and you want to indicate that a type is a specific type, you can use unit structs.
   
    Example: struct ConsoleLogger;
    Usage: struct ConsoleLogger;

           impl Logger for ConsoleLogger {
                fn log(&self, message: &str) {
                println!("[Console] {}", message);
    }

     => Traits + Unit Structs
     Example:
     trait Logger {
        fn log(&self, message: &str);
        }
        struct ConsoleLogger;
        impl Logger for ConsoleLogger {
            fn log(&self, message: &str) {
                println!("[Console] {}", message);
            }
        }
}

*/


// An attribute to hide warnings for unused code.
#![allow(dead_code)]

#[derive(Debug)]
struct Person {
    name: String,
    age: u8,
}

// A unit struct
struct Unit;

// A tuple struct
struct Pair(i32, f32);

// A struct with two fields
struct Point {
    x: f32,
    y: f32,
}

// Structs can be reused as fields of another struct
struct Rectangle {
    // A rectangle can be specified by where the top left and bottom right
    // corners are in space.
    top_left: Point,
    bottom_right: Point,
}

fn main() {
    // Create struct with field init shorthand
    let name = String::from("Peter");
    let age = 27;
    let peter = Person { name, age };

    // Print debug struct
    println!("{:?}", peter);

    // Instantiate a `Point`
    let point: Point = Point { x: 5.2, y: 0.4 };
    let another_point: Point = Point { x: 10.3, y: 0.2 };

    // Access the fields of the point
    println!("point coordinates: ({}, {})", point.x, point.y);

    // Make a new point by using struct update syntax to use the fields of our
    // other one
    let bottom_right = Point { x: 10.3, ..another_point };

    // `bottom_right.y` will be the same as `another_point.y` because we used that field
    // from `another_point`
    println!("second point: ({}, {})", bottom_right.x, bottom_right.y);

    // Destructure the point using a `let` binding
    let Point { x: left_edge, y: top_edge } = point;

    let _rectangle = Rectangle {
        // struct instantiation is an expression too
        top_left: Point { x: left_edge, y: top_edge },
        bottom_right: bottom_right,
    };

    // Instantiate a unit struct
    let _unit = Unit;

    // Instantiate a tuple struct
    let pair = Pair(1, 0.1);

    // Access the fields of a tuple struct
    println!("pair contains {:?} and {:?}", pair.0, pair.1);

    // Destructure a tuple struct
    let Pair(integer, decimal) = pair;

    println!("pair contains {:?} and {:?}", integer, decimal);
}
