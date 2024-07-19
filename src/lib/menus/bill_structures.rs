use std::collections::HashMap;

#[derive(Debug, Clone)]
pub struct Bill {
    pub name: String,
    pub amount: f64
}

// TODO: Consider adding more list to this struct 
// E.g. home_bills, clothing_bills, food_bills

pub struct BillCollection {

    // Remember that HashMap's store data in Key value pairs
    // The key is String, and the value is a Bill
    // In simple terms. You enter the name of a bill into the HashMap and
    // it will return a bill that matches that name
    hashmap_of_bills: HashMap<String, Bill>
}

impl BillCollection {

    // NOTE: A method to create a new instance of the struct,
    // with the fields set

    pub fn new() -> Self {
        
        Self {
            hashmap_of_bills: HashMap::new()
        }

    }

    // This makes it easy to add a new bill 
    // to an instance of a bill collection.
    pub fn add_bill(&mut self, bill: Bill) {
        
        // I created this name variable to store a refence to the name of
        // the bill which is originally a string.
        let cloned_name: String = bill.name.clone();

        // This is a prevent the .insert() method from taking ownership of the
        // "name" field from the original bill that is passed into the 
        // add_bill() function.
        // I don't want an ownership transfer because the "name" field
        // of the bill passed into the function must be usable afterwards

        self.hashmap_of_bills.insert(cloned_name, bill);
    }

    // This will return a new Vector, and inside it we will have references
    // to the original individual bills
    pub fn view_all_bills(&self) -> Vec<&Bill> {
        
        // .iter() goes through the list without taking ownership of each item
        // in the list. So it is creating a reference to each item in the list.
        // And then .collect() collects all of these references and puts them
        // into a new vector.

        // self.hashmap_of_bills.iter().collect() returns the variable type
        // Vec<&Bill>

        self.hashmap_of_bills.values().collect()
    }

    pub fn remove_bill(&mut self, name: &str) -> bool {
        
        self.hashmap_of_bills.remove(name).is_some()

    }

}
