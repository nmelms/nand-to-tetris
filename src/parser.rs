use std::fs;

#[derive(Debug)]
pub struct Parser {
    pub lines: Vec<String>,
    pub current_index: usize,
    pub label_index: usize,
    pub var_index: usize, 
}

impl Parser {
    pub fn new(file_name: &str) -> Self {
        let file_contents: String = fs::read_to_string(file_name)
            .expect("there was an error reading from the file in parser");
        let current_index = 0;
        let label_index = 0;
        let var_index = 16;
        let mut lines = Vec::new();


        for line in file_contents.lines() {
            // println!("this is the current like in new: {}", line);
            lines.push(line.to_string());
        }

        Self {
            current_index,
            lines,
            label_index,
            var_index,
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
        let line = self.lines[self.current_index].trim();
        if line.starts_with("@") {
            String::from("A_INSTRUCTION")
        } else if line.starts_with("(") {
            String::from("L_INSTRUCTION")
        } else {
            String::from("C_INSTRUCTION")
        }
    }
    #[allow(dead_code)]
    pub fn symbol(&self) -> String {
        if self.instruction_type() == "A_INSTRUCTION" {
            let value = self.lines[self.current_index]
                .trim()
                .strip_prefix('@')
                .expect("Expected A-instruction");

            value.to_string()
        } else if self.instruction_type() == "L_INSTRUCTION" {
            let value = self.lines[self.current_index]
                .trim()
                .strip_prefix('(')
                .and_then(|s| s.strip_suffix(')'))
                .expect(&format!(
                    "Expected L-instruction, got: {:?}",
                    self.lines[self.current_index]
                ));

            value.to_string()
        } else {
            String::from("Not an a or c instruction")
        }
    }

    pub fn jump(&self) -> String {
        if &self.instruction_type() == "C_INSTRUCTION" {
            // contains a jump?
            let cur = &self.lines[self.current_index];
            let has_semi: bool = self.lines[self.current_index].contains(";");
            let split_on_semi: Vec<&str> = cur.split(";").collect();

            if has_semi {
                split_on_semi[1].to_string()
            } else {
                String::from("null")
            }
        } else {
            String::from("This is not C instruction")
        }
    }

    pub fn dest(&self) -> String {
        //get current line
        if &self.instruction_type() == "C_INSTRUCTION" {
            let cur = &self.lines[self.current_index].trim();
            let has_dest: bool = self.lines[self.current_index].trim().contains("=");
            let split: Vec<&str> = cur.split("=").collect();
            if has_dest {
                split[0].to_string()
            } else {
                String::from("This instruction does not have a dest")
            }
        } else {
            String::from("This is not C instruction")
        }
    }

    pub fn comp(&mut self) -> String {
        let is_c = self.instruction_type() == "C_INSTRUCTION";
        let mut cur = self.lines[self.current_index].trim().to_string();

        if is_c {
            match cur.find(";") {
                Some(i) => {
                    let _jump = cur.split_off(i);
                }
                None => {
                    println!("no Jump");
                }
            }

            match cur.find("=") {
                Some(i) => {
                    let right = cur.split_off(i + 1);
                    cur = right;
                }
                None => {
                    println!("no dest");
                }
            }
        }

        cur
    }
}
