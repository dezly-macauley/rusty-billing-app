use crate::menus::bill_structures::*;
use crate::user_input::input_functions::get_user_input;

pub fn remove_bill(bill_collection: &mut BillCollection) {

    for bill in bill_collection.view_all_bills() {
        println!("{:?}", bill);
    }

    println!("Enter bill name to remove:");

    let name = match get_user_input() {
        Some(name) => name,
        None => return,
        
    };

    if bill_collection.remove_bill(&name) {
        println!("bill removed");
    } else {
        println!("bill not found");
    }

}
