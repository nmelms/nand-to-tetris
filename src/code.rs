pub fn jump(code: &str) -> u8 {
    match code {
        "null" => 000,
        "JGT" => 001,
        "JEQ" => 010,
        "JGE" => 011,
        "JLT" => 100,
        "JNE" => 101,
        "JLE" => 110,
        "JMP" => 111,
        _ => unreachable!("Invalid jump code: {}", code),
    }
}

pub fn dest(code: &str) -> u8 {
    match code {
        "null" => 000,
        "M" => 001,
        "D" => 010,
        "DM" => 011,
        "A" => 100,
        "AM" => 101,
        "AD" => 110,
        "ADM" => 111,
        _ => unreachable!("Invalid jump code: {}", code),
    }
}
