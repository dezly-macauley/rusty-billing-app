use rba_library::user_input::input_functions::get_user_input;

use rba_library::menus::bill_structures::BillCollection;
use rba_library::menus::add_bill::add_bill;
use rba_library::menus::view_bills::view_bills;
use rba_library::menus::remove_bill::remove_bill;

pub enum MainMenu {
    AddBill,
    ViewBills,
    RemoveBill
}

// Adding functionality to the enum
impl MainMenu {

    fn show_options() {
        println!("");
        println!("=================| Rusty Billing App |=================");
        println!("--------------------------------------------------------");
        println!("                  1 - Add Bill");
        println!("                  2 - View Bills");
        println!("                  3 - Remove Bill");
        println!("-------------------------------------------------------");
        println!("To select a menu option, type a number and press Enter:");
    }

    fn from_str(input: &str) -> Option<MainMenu>  {

        match input {
            "1" => Some(Self::AddBill),
            "2" => Some(Self::ViewBills),
            "3" => Some(Self::RemoveBill),
            _ => None
        }

    }

}

fn main() {

    // Create an new instance of a bill collection (A list that can store
    // bills. And each bill is a struct)
    let mut bill_collection: BillCollection = BillCollection::new();

    loop {
        MainMenu::show_options();
        // If the user doesn't enter any data the program will stop working
        let input = get_user_input().expect("No data entered");

        // Remember that the user input from the get_user_input function
        // is returned as a String.
        // However the from_str function accepts a string slice
        match MainMenu::from_str(input.as_str()) {
            Some(MainMenu::AddBill) => add_bill(&mut bill_collection),
            Some(MainMenu::ViewBills) => view_bills(&bill_collection),
            Some(MainMenu::RemoveBill) => remove_bill(&mut bill_collection),
            // This will exit the program immeadiatley if there is an error
            // FIX: Handle this error more gracefully
            None => return
        }
    }

}
