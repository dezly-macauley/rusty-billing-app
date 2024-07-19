use rba_library::user_input::input_functions::get_user_input;

pub enum MainMenu {
    AddBill,
    ViewBills,
}

// Adding functionality to the enum
impl MainMenu {

    fn show_options() {
        println!("");
        println!("=================| Rusty Billing App |=================");
        println!("--------------------------------------------------------");
        println!("                  1 - Add Bill");
        println!("                  2 - View Bills");
        println!("-------------------------------------------------------");
        println!("To select a menu option, type a number and press Enter:");
    }

    fn from_str(input: &str) -> Option<MainMenu>  {

        match input {
            "1" => Some(Self::AddBill),
            "2" => Some(Self::ViewBills),
            _ => None
        }

    }

}

fn main() {

    // NOTE: Hidden for now
    // get_user_input();

    // FIX: Handle these errors better

    loop {
        MainMenu::show_options();
        // If the user doesn't enter any data the program will stop working
        let input = get_user_input().expect("No data entered");

        // Remember that the user input from the get_user_input function
        // is returned as a String.
        // However the from_str function accepts a string slice
        match MainMenu::from_str(input.as_str()) {
            // means () do nothing
            Some(MainMenu::AddBill) => (),
            Some(MainMenu::ViewBills) => (),
            // This will exit the program
            None => return
        }
    }

}
