extern crate hindex_viewer;
extern crate diesel;

// use std::time::{Instant};
use self::hindex_viewer::*;
use self::models::*;
use self::diesel::prelude::*;
use std::io;

fn main() {
    // let now = Instant::now();
    use hindex_viewer::schema::block::dsl::*; //brings block schema into scope
    let connection = establish_connection();

    println!("******************************************************");
    println!("");
    println!("This is a temporary mockup for getting a");
    println!("specific block from our PostgreSQL database");
    println!("");
    println!("******************************************************");

    // Mocks a click on a specific block height on HNScan
    loop {
        println!("Block Height?");

        // Mutable variable declaration as a new empty string type
        let mut user_input = String::new();

        // Get the input from user, read it, and store it as a string in our variable
        io::stdin().read_line(&mut user_input)
            .expect("Failed to read line"); // Error handling

        let user_input: i32 = match user_input.trim().parse() {
            Ok(num) => num, // Allow inputs that are numbers
            Err(_) => continue, // Ignore non-number guesses
        };

        let result = block.filter(id.eq(user_input))
            .load::<Block>(&connection)
            .expect("Error loading block");
        // println!("{}", now.elapsed().subsec_millis());

        // Only print the result if we got something back from the database
        if result.len() == 1 {
            println!("{:?}", result);
            println!("");
        } else {
            println!("Invalid block height, please try again");
        }
    }
}
