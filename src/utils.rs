use std::io::{self, Write};

pub fn prompt(question: &str) -> String {
    print!("{} > ", question);
    io::stdout().flush().unwrap();
    
    let mut input = String::new();
    
    io::stdin()
        .read_line(&mut input)
        .expect("Failed to read line");
        
    input
}