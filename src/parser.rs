use std::fs;


pub fn init_parser(file_name: &String) -> String {
    let file_contents = fs::read_to_string(file_name).expect("there was an error reading from the file in parser");

    file_contents
    
}