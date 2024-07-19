use crate::menus::bill_structures::*;

pub fn view_bills(bill_collection: &BillCollection) {

    for bill in bill_collection.get_all() {
        println!("{:?}", bill);
    }

}
