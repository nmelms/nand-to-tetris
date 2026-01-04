use std::env;
use std::fs;
mod parser;

// parser
//code
//assmbler
fn main() {
    let args: Vec<String> = env::args().collect();
    let file_name = &args[1];

    let mut parser = parser::Parser::new(file_name);
    println!("{:?}", parser.instruction_type());

    while parser.has_more_lines() {
        let curr = parser.current_index;
        // println!("THis is the current lline: {}", parser.lines[curr]);
        let dest = parser.dest();
        println!("
        Curr: {}
        Dest: {}
        ", curr,dest);
        parser.advance();
    }
}
