use user_input_functions::get_user_input;

enum MainMenu {
    AddBill,
    ViewBills,
}

// Adding functionality to the enum
impl MainMenu {


    fn show_options() {
        println!("");
        println!("========| Rusty Billing App |========");
        println!("-------------------------------------");
        println!("1 - Add Bill");
        println!("2 - View Bills");
        println!("-------------------------------------");
        println!("To select a menu option,");
        print!("type a number and press Enter: ");
    }

    fn from_str(input: &str) -> Option<MainMenu>  {

        match input {
            "1" => Some(Self::AddBill),
            "2" => Some(Self::ViewBills),
            _ => None
        }

    }

}

#[derive(Debug, Clone)]
struct Bill {
    name: String,
    amount: f64
}

// TODO: Consider adding more list to this struct 
// E.g. home_bills, clothing_bills, food_bills

struct BillCollection {
    list_of_bills: Vec<Bill>
}

impl BillCollection {

    // NOTE: A method to create a new instance of the struct,
    // with the fields set

    fn new() -> Self {
        
        Self {
            list_of_bills: vec![]
        }

    }

    // This makes it easy to add a new bill to an instance of a bill.
    fn add_bill(&mut self, bill: Bill) {
        self.list_of_bills.push(bill);
    }

    fn get_all(&self) -> Vec<&Bill> {
        //
        self.list_of_bills.iter().collect()
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
