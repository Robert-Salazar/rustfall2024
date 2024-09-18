fn main() {
    /*
    let x: i32 = 5;

    let x: &str = "word";
    
    // Shadowing and changes same variable to word str
     println!("The value of x is: {}", x); // = word
    */


     /*
    let x: i32 = 5;

    {
    let x: &str = "word";
    }

    // Shadowing only works in the same scope, 
    // since word is out of scope it doesnt change
     println!("The value of x is: {}", x); // = 5
     */

     /*
        // Converting a string to a number
        let num = "25";
        let num: u32 = num.parse()
            .expect("Please provide a valid number!");
        
        println!("Parsed number: {}", num);
        
        // Further transformations
        let num = num + 25;
        let num = num * 2;
        
        println!("Final value of num: {}", num);
    */

    /*
     // Shadowing
     let x = 5;
     let x = x + 1;  // Creates a new variable
     
     // Mutation
     let mut y = 5;
     y = y + 1;  // Modifies the existing variable
     
     println!("x: {}, y: {}", x, y);
    */

  // Overflow example (uncomment to see the panic in debug mode)
    // let mut money: u8 = 0;
    // money -= 1;
    /*
        let c = 'z';
        let z: char = 'ℤ';
        let heart_eyed_cat = '😻';
    
        println!("c: {}, z: {}, cat: {}", c, z, heart_eyed_cat);
    */

    /*
        // Creating a tuple
        let tup: (i32, f64, u8) = (500, 6.4, 1);
    
        // Destructuring (pattern matching)
        let (x, y, z) = tup;
        println!("x: {}, y: {}, z: {}", x, y, z);
        
        // Accessing tuple elements using dot notation
        println!("First: {}, Second: {}, Third: {}", tup.0, tup.1, tup.2);
    
        // Tuple as a return type
        let (product, sum) = calculate(3, 4);
        println!("Product: {}, Sum: {}", product, sum);
    
    
    fn calculate(x: i32, y: i32) -> (i32, i32) {
        (x * y, x + y)
    }

    */
/*
     // Creating an array
     let months = ["January", "February", "March", "April", "May", "June", "July",
     "August", "September", "October", "November", "December"];

// Accessing array elements
let first = months[0];
let second = months[1];
println!("First month: {}, Second month: {}", first, second);

// Array with explicit type and size
let numbers: [i32; 5] = [1, 2, 3, 4, 5];

// Array with repeated values
let repeated = [3; 5]; // Equivalent to [3, 3, 3, 3, 3]

println!("Third number: {}, First repeated: {}", numbers[2], repeated[0]);

// Printing entire array (debug format)
println!("Numbers: {:?}", numbers);

// Demonstrating bounds checking
let index: usize = 0;
// Uncommenting the next line would cause a runtime panic
let element: i32 = numbers[index];
println!("Num: {:?}", element);
*/

/*
fn message() {
    println!("Just function");
}


    message();


fn message_with_param(favnum: i32) {
    println!("My favorite number is: {}", favnum);
}


    message_with_param(777);
*/

/*
let x = 5;
let y = {
    // Curly braces create a different scope
    let x = 3;
    x + 1 // Expression, returns the value
};

println!("X is: {}. Y is {}", x, y);

//### 4 Functions with Return Values

//Functions can return values. The return type is specified after an arrow `->`:

//```rust
fn add_five(mynum: i32) -> i32 {
    mynum + 5
}

    println!("Calling add_five function, relying on expression return {}", add_five(100));
*/

/*
//### 5 Type Safety in Functions

//Rust enforces type safety in functions. Here's an example:

//```rust
fn subtract_five(mynum: i32) -> u8 {
    mynum as u8 - 5 // mynum - 5 would not be correct
    // can convert data types mynum as i16
}

    println!("Calling subtract_five function, relying on expression return {}", subtract_five(6));
    // Try to pass 4 to see what happens
*/

/*
fn pattern_match_simple() {
    let num = 3;
    let letter = match num {
        1 => 'A',
        2 => 'B',
        3 => {
            (64 + 1 + 2 as u8) as char
        },
        _ => '#', // rust will not guess
    };
    println!("{}", letter);
}

pattern_match_simple();
*/

fn concat_strings(s1: &String, s2: &String) -> String {
    let mut new_word:
    
}

fn main() {
    let s1 = String::from("Hello, ");
    let s2 = String::from("World!");
    let result = concat_strings(&s1, &s2);
    println!("{}", result); // Should print: "Hello, World!"
}
    }

