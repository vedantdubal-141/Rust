/*Tuples Overview
A type of "record" that stores data anonymously.
-No need to name fields (anonymous tuples).
-Useful for returning pairs or groups of data from functions.
-Can be easily "destructured" into variables.
Example Code
1. Defining and Using Tuples*/
// Define an enum for access control
enum Access {
    Full,
}

// Function returning a tuple
fn one_two_three() -> (i32, i32, i32) {
    (1, 2, 3)
}

let numbers = one_two_three();

// Destructuring the tuple
let (x, y, z) = one_two_three();
println!("First: {}, Second: {}", x, y);
println!("Third: {}", z);

// Using a named tuple for clarity with more complex data
let employee_access_tuple = ("Jake", Access::Full);


/*2. Recap
-Tuples allow anonymous data access.
-Useful when destructuring tuples into variables.
-Can contain any number of fields, but prefer struct for more than two or three fields.*/