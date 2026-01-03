use std::fs;

#[derive(Debug)]
pub struct Parser {
    file_contents: String,
    lines: Vec<String>,
    current_index: usize,
}

impl Parser {
    pub fn init(file_name: &str) -> Self {
     let file_contents: String = fs::read_to_string(file_name).expect("there was an error reading from the file in parser");
     let mut current_index = 0;
     let mut lines = Vec::new();

     for line in file_contents.lines(){
        lines.push(line.to_string());
     }

     Self {file_contents, current_index, lines}
    
    }

    pub fn has_more_lines(&self) -> bool{
        if self.current_index < self.file_contents.len()  {
            true 
        }  else{
            false
        } 

    }

    pub fn advance(&mut self){
        self.current_index += 1
    }

    pub fn instruction_type(&self) -> String{
        if self.lines[self.current_index].starts_with("@"){
            String::from("A_INSTRUCTION")
        }else if self.lines[self.current_index].starts_with("symbol"){
            String::from("L_INSTRUCTION")
        }else{
            String::from("C_INSTRUCTION")
        }
    }   
}



