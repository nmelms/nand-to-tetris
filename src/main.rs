use std::env;
use std::fs;
mod parser;
// parser
//code
//assmbler
fn main() {

    let args: Vec<String> = env::args().collect();
    // let file_name = &args[1];

    parser::hello();

    // let contents = fs::read_to_string(file_name)
    //     .expect("there was an error reading from the file");


    // println!("{:?}", contents);
}
