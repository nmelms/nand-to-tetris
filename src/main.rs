use std::env;
mod code;
mod parser;

//assmbler
fn main() {
    let args: Vec<String> = env::args().collect();
    let file_name = &args[1];

    let mut parser = parser::Parser::new(file_name);

    println!("{:?}", parser.lines);
    println!("{}", parser.has_more_lines());
    while parser.has_more_lines() {
        println!("{}", parser.has_more_lines());

        let cur = parser.current_index;
        let dest = parser.dest();
        let comp = parser.comp();
        let jump = parser.jump();

        println!(
            "
            Current Instruction: {}
            Dest: {}
            Comp: {}
            Jump: {}
        ",
            cur, dest, comp, jump
        );
        let decode = code::comp("M+1");
        println!("Decode is : {:03b}", decode);

        parser.advance();
    }
}
