// NOTE: This is the entry point of the library.
// Think of this as home page of a website.

// This will list the "user_input" folder.
pub mod user_input {
    // and all of the .rs files inside it
    pub mod input_functions;
}

// This will list the "menu_functionality" folder,
pub mod menus {
    // and all of the .rs files inside it
    pub mod bill_structures;
    pub mod add_bill;
    pub mod view_bills;
    pub mod remove_bill;
}
