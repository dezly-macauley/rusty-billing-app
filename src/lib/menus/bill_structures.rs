#[derive(Debug, Clone)]
pub struct Bill {
    pub name: String,
    pub amount: f64
}

// TODO: Consider adding more list to this struct 
// E.g. home_bills, clothing_bills, food_bills

pub struct BillCollection {
    list_of_bills: Vec<Bill>
}

impl BillCollection {

    // NOTE: A method to create a new instance of the struct,
    // with the fields set

    pub fn new() -> Self {
        
        Self {
            list_of_bills: vec![]
        }

    }

    // This makes it easy to add a new bill 
    // to an instance of a bill collection.
    pub fn add_bill(&mut self, bill: Bill) {
        self.list_of_bills.push(bill);
    }

    // This will return a new Vector, and inside it we will have references
    // to the original individual bills
    pub fn get_all(&self) -> Vec<&Bill> {
        
        // .iter() goes through the list without taking ownership of each item
        // in the list. So it is creating a reference to each item in the list.
        // And then .collect() collects all of these references and puts them
        // into a new vector.

        // self.list_of_bills.iter().collect() returns the variable type
        // Vec<&Bill>

        self.list_of_bills.iter().collect()
    }

}
