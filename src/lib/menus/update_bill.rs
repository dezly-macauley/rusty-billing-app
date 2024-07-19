use crate::menus::bill_structures::*;
use crate::user_input::input_functions::get_user_input;

pub fn update_bill(bill_collection: &mut BillCollection) {

    for bill in bill_collection.view_all_bills() {
        println!("{:?}", bill);
    }

    println!("Enter bill name to update:");

    let name = match get_user_input() {
        Some(name) => name,
        None => {
            println!("bill not found");
            return
        }
        
    };
    
    println!("Enter the new amount for this bill:");

    let new_amount: f64 = match get_user_input() {
        Some(user_input) => match user_input.parse::<f64>() {
            Ok(converted_input) => converted_input,
            Err(_) => {
                println!("Invalid input. Please enter a number");
                return;
            }
        } 
        None => return,
    };

    if bill_collection.update_bill(&name, new_amount) {
        println!("bill updated");
    } else {
        println!("bill not updated");
    }

}
