use serde_json::Value;
use std::io::{self, Write};
use colored::*;

pub fn prompt(question: &str) -> String {
    print!("{} > ", question.blue().bold());
    io::stdout().flush().unwrap();
    
    let mut input = String::new();
    
    io::stdin()
        .read_line(&mut input)
        .expect("Failed to read line");
        
    input
}

pub fn heading(text: &str) {
    println!();
    println!("{}", text.bold());
    println!("{}", "=".repeat(text.len()).bold());
}

pub fn value_as_u32(value: &Value) -> Option<u32> {
    match value {
        Value::Number(n) => n.as_u64().and_then(|n| u32::try_from(n).ok()),
        Value::String(s) => s.parse::<u32>().ok(),
        _ => None,
    }
}