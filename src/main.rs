use std::{env, fs::File, io::Write};
mod code;
mod parser;
mod symbol_table;
use std::io::Result;

//assmbler
fn main() -> Result<()> {
    let args: Vec<String> = env::args().collect();
    let file_name = &args[1];

    let mut parser = parser::Parser::new(file_name);
    let mut symbol_table = symbol_table::SymbolTable::new();

    let mut file = File::create("output.hack")?;
    let mut decoded_instructions: Vec<u8> = Vec::new();
    // first pass adding symbols to symbol table

    while parser.has_more_lines(){
        if parser.instruction_type() == "L_INSTRUCTION" {
           let symbol = parser.symbol();
            symbol_table.add_entry(symbol.to_string(), (parser.current_index + 1) as u32);

        }
        parser.advance();
    }
    // set parser back to 0
    parser.current_index = 0;

    while parser.has_more_lines() {
        // c instruction
        println!("current instruction type: {}", parser.instruction_type());
        if parser.instruction_type() == "C_INSTRUCTION" {
            let mut current: u16 = 0b1110000000000000;
            // let cur = parser.current_index;
            let dest = parser.dest();
            let comp = parser.comp();
            let jump = parser.jump();

            let dest = code::dest(dest.as_str());
            let comp = code::comp(comp.as_str());
            let jump = code::jump(jump.as_str());

            current |= comp << 6;
            current |= dest << 3;
            current |= jump;

            println!("{:b}", current);
            let line = format!("{:016b}\n", current);
            decoded_instructions.extend_from_slice(line.as_bytes());

            // A instruction
        } else if parser.instruction_type() == "A_INSTRUCTION" {
            // let num = &mut parser.lines[parser.current_index];
            let symbol = parser.symbol();

            let value = match symbol.parse::<u32>() {
                Ok(num) => num,
                Err(_) => symbol_table.get_address(&symbol)
            };
            let line = format!("{:016b}\n", value);
            decoded_instructions.extend_from_slice(line.as_bytes());

            // l instruction
        } 

        parser.advance();
    }



    file.write_all(&decoded_instructions);

    Ok(())
}
