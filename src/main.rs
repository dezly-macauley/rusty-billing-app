// Enables the io module (input output) from the Rust standard library,
// to handle user input
use std::io;

// This function returns Option<String>
// The Option enum can return one of two options
// 1. Some(String)  A formated String if the user did type something.
// 2. None          If the user pressed enter without typing an input. 
fn get_user_input() -> Option<String> {

    // This variable will be used to capture 
    // the user's raw input from the terminal 
    // It is set to an empty String, that will be replaced with the actual
    // input of the user
    let mut input_buffer: String = String::new();

    // NOTE: io:stdin

    // io::stdin() returns an instance of the type std::io::Stdin.
    // std::io::Stdin is a struct,
    // When io::stdin is used, the terminal will wait for the user to enter
    // their input.
    // That raw input can then be read and stored into a variable like, 
    // using methods like read_line()

    // NOTE: .read_line(&mut input_buffer)

    // .read_line(&mut input_buffer)
    // This will read a line from terminal and store the raw input into
    // the variable input_buffer

    // NOTE: Why is &mut used

    // & is used because I don't want .readline() to take ownership
    // of the buffer. 
    // I want the variable to still be available after the read.
    // So read_line() will get a reference to the input_buffer variable
    // &mut is used because read_line() must have permission to update the
    // contents of the original input_buffer

    // NOTE: What does .read_line(&mut input_buffer) return?
    // Result<usize, std::io::Error>
    // This is a result type that can either be:
    // 1. Ok(usize): Indicates that the operation was successful. 
    // It returns the number of bytes read (usize).
    // 2. Err(std::io::Error)
    // If there is an error (for example,
    // if reading from standard input fails),
    // it returns Err with an std::io::Error describing the problem.

    // NOTE: is_err()
    // is_err() returns a boolean if there was a problem with the user input

    // As long as there is an error reading the user's input they will be
    // prompted to keep trying.
    // If the read was successful then the raw user input will be saved to the
    // variable input_buffer
    while io::stdin().read_line(&mut input_buffer).is_err() {
        println!("There was an error when reading your input.
        Please try again");
    }

    // SEC: Formatting The Raw User Input
   
    // .trim() will remove any whitespace from the raw input
    // whitespace is extra spaces, trailing characters, the Enter key etc.
    // .trim() returns a string slice (&str)
    // and this function should return a String in the case of success.
    // So .to_string() will convert &str to String 
    let formated_input: String = input_buffer.trim().to_string();

    // SEC: Return either the formated String, or None

    if formated_input != "".to_string() {
        // println!("{:?}", formated_input);
        return Some(formated_input);
    } else {
        // println!("The user didn't type anything");
        return None;
    }

}

fn main() {

    get_user_input();

}
