use std::fs;

#[derive(Debug)]
pub struct Parser {
    pub lines: Vec<String>,
    pub current_index: usize,
}

impl Parser {
    pub fn new(file_name: &str) -> Self {
        let file_contents: String = fs::read_to_string(file_name)
            .expect("there was an error reading from the file in parser");
        let current_index = 0;
        let mut lines = Vec::new();

        for line in file_contents.lines() {
            lines.push(line.to_string());
        }

        Self {
            current_index,
            lines,
        }
    }

    pub fn has_more_lines(&self) -> bool {
        if self.current_index < self.lines.len() {
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
    #[allow(dead_code)]
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
        } else {
            "This is not C instruction"
        }
    }

    pub fn comp(&self) -> &str {
        if &self.instruction_type() == "C_INSTRUCTION" {
            let cur = &self.lines[self.current_index];

            let has_equal: bool = self.lines[self.current_index].contains("=");
            let has_semi: bool = self.lines[self.current_index].contains(";");

            // only comp
            if has_equal {
                let split_on_eq: Vec<&str> = cur.split("=").collect();
                split_on_eq[1]
            } else if has_semi {
                let split_on_semi: Vec<&str> = cur.split(";").collect();
                split_on_semi[0]
            } else {
                return cur;
            }
        } else {
            "This is not C instruction"
        }
    }

    pub fn jump(&self) -> &str {
        if &self.instruction_type() == "C_INSTRUCTION" {
            // contains a jump?
            let cur = &self.lines[self.current_index];
            let has_semi: bool = self.lines[self.current_index].contains(";");
            let split_on_semi: Vec<&str> = cur.split(";").collect();

            if has_semi {
                split_on_semi[1]
            } else {
                "null"
            }
        } else {
            "This is not C instruction"
        }
    }
}
