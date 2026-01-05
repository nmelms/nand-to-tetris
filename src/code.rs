pub fn jump(code: &str) -> u8 {
    match code {
        "null" => 0b000,
        "JGT" => 0b001,
        "JEQ" => 0b010,
        "JGE" => 0b011,
        "JLT" => 0b100,
        "JNE" => 0b101,
        "JLE" => 0b110,
        "JMP" => 0b111,
        _ => unreachable!("Invalid JMP code: {}", code),
    }
}

pub fn dest(code: &str) -> u8 {
    match code {
        "null" => 0b000,
        "M" => 0b001,
        "D" => 0b010,
        "DM" => 0b011,
        "A" => 0b100,
        "AM" => 0b101,
        "AD" => 0b110,
        "ADM" => 0b111,
        _ => unreachable!("Invalid DEST code: {}", code),
    }
}
// might want to go back on these and add 0 or 1 at the beginning for a or m
pub fn comp(code: &str) -> u8 {
    match code {
        "0" => 0b101010,
        "1" => 0b111111, 
        "-1" => 0b111010,
        "D" => 0b001100,
        "A" => 0b11000,
        "M" => 0b11000,
        "!D" => 0b001101,
        "!A" => 0b110011, 
        "!M" => 0b110011,
        "-D" => 0b00111,
        "-A" => 0b110011,
        "-M" => 0b110011,
        "D+1" => 0b011111,
        "A+1" => 0b110111,
        "M+1" => 0b110111,
        "D-1" => 0b001110,
        "A-1" => 0b110010,
        "M-1" => 0b110010,
        "D+A" => 0b000010,
        "D+M" => 0b000010,
        "D-A"=> 0b010011,
        "D-M" => 0b010011,
        "A-D" => 0b000111,
        "M-D" => 0b000111,
        "D&A" => 0b000000,
        "D&M" => 0b000000,
        "D|A" => 0b0101001,
        "D|M" => 0b0101001,
        _ => unreachable!("Invalid COMP code: {}", code),



    }
}