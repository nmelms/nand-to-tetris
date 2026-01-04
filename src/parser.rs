use std::fs;

#[derive(Debug)]
pub struct Parser {
    file_contents: String,
    pub lines: Vec<String>,
    pub current_index: usize,
}

impl Parser {
    pub fn new(file_name: &str) -> Self {
        let file_contents: String = fs::read_to_string(file_name)
            .expect("there was an error reading from the file in parser");
        let mut current_index = 0;
        let mut lines = Vec::new();

        for line in file_contents.lines() {
            lines.push(line.to_string());
        }

        Self {
            file_contents,
            current_index,
            lines,
        }
    }

    pub fn has_more_lines(&self) -> bool {
        if self.current_index < self.file_contents.len() {
            true
        } else {
            false
        }
    }

    pub fn advance(&mut self) {
        self.current_index += 1
    }

    pub fn instruction_type(&self) -> String {
        if self.lines[self.current_index].starts_with("@") {
            String::from("A_INSTRUCTION")
        } else if self.lines[self.current_index].starts_with("symbol") {
            String::from("L_INSTRUCTION")
        } else {
            String::from("C_INSTRUCTION")
        }
    }

    pub fn symbol(&self) -> Option<String> {
        if self.instruction_type() == "A_INSTRUCTION" || self.instruction_type() == "L_INSTRUCTION"
        {
            Some(self.lines[self.current_index].clone())
        } else {
            None
        }
    }

    pub fn dest(&self) -> &str {
        //get current line
        if &self.instruction_type() == "C_INSTRUCTION" {
            let cur = &self.lines[self.current_index];
            let has_dest: bool = self.lines[self.current_index].contains("=");
            let split: Vec<&str> = cur.split("=").collect();
            if has_dest {
                split[0]
            } else {
                "This instruction does not have a dest"
            }
        }else{
            "This is not an A instruction"
        }
    }
}
