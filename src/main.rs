use std::{env, fs::File, io::Write};
mod code;
mod parser;
use std::io::Result;

//assmbler
fn main() -> Result<()> {
    let args: Vec<String> = env::args().collect();
    let file_name = &args[1];

    let mut parser = parser::Parser::new(file_name);

    let mut file = File::create("output.hack")?;
    let mut decoded_instructions: Vec<String> = Vec::new();

    while parser.has_more_lines() {


        if parser.instruction_type() == "C_INSTRUCTION"{
            let mut current: u16 = 0b1110000000000000;

            let cur = parser.current_index;
            let dest = parser.dest();
            let comp = parser.comp();
            let jump = parser.jump();

            let dest = code::dest(dest);
            let comp = code::comp(comp);
            let jump = code::jump(jump);
            
            current |= comp << 6;
            current |= dest << 3;
            current |= jump;

            println!("finished string: {:b}", current)




        };




        // println!(
        //     "
        //     Current Instruction: {}
        //     Dest: {}
        //     Comp: {}
        //     Jump: {}
        // ",
        //     cur, dest, comp, jump
        // );


        parser.advance();
    }

    // file.write_all(b"file");
    Ok(())
}
