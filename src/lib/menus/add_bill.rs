// NOTE: Importing Libraries into other libraries
// crate:: is a directory path that starts from the "lib" folder

// This will import all of structs from the bill_structures.rs file 
use crate::menus::bill_structures::*;

// This will import the get_user_input() function from 
// the input_functions.rs file
use crate::user_input::input_functions::get_user_input;

pub fn add_bill(bill_collection: &mut BillCollection) {
    print!("Enter name of Bill: ");
    
    let name_of_bill: String = match get_user_input() {
        Some(user_input) => user_input,
        None => return,
    };

    // user_input is a String so it must be converted to an f64 first,
    // However you have to ensure that the user's input is actually something
    // that can be converted to an f64
    // E.g. If the user typed "ice cream". There's no way to convert that into
    // a float
    // match user_input.parse::<f64>() returns a Result enum.
    // This will return Ok if the input could be converted. 
    // And Err if the input could not be converted

    print!("Enter the amount you owe: ");
    let amount_owed: f64 = match get_user_input() {
        Some(user_input) => match user_input.parse::<f64>() {
            Ok(converted_input) => converted_input,
            Err(_) => {
                println!("Invalid input. Please enter a number");
                return;
            }
        } 
        None => return,
    };

    let new_bill: Bill = Bill {
        name: name_of_bill,
        amount: amount_owed
    };

    bill_collection.add_bill(new_bill);
    println!("You bill was added");

}
