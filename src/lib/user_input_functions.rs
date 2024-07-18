use std::io;

pub fn get_user_input() -> Option<String> {

    let mut input_buffer: String = String::new();

    while io::stdin().read_line(&mut input_buffer).is_err() {
        println!("There was an error when reading your input.
        Please try again");
    }

    let formated_input: String = input_buffer.trim().to_string();

    if formated_input != "".to_string() {
        return Some(formated_input);
    } else {
        return None;
    }

}
