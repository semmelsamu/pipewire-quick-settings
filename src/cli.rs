use std::cmp::max;
use crate::pw_dump::{pw_dump, parse_pw_clients, parse_pw_sinks};

fn heading(text: &str) {
    println!();
    println!();
    println!("==== {} {}", text, "=".repeat(max(0, 60 - text.len() - 2)));
    println!();
}

pub fn run_cli() {
    heading("PipeWire Quick Settings CLI");
    
    print!("Running pw-dump...");
    let dump_output = pw_dump();
    println!("OK");
    
    print!("Parsing clients...");
    let clients = parse_pw_clients(dump_output.clone());
    println!("OK");
    
    print!("Parsing sinks...");
    let sinks = parse_pw_sinks(dump_output);
    println!("OK");
    
    heading(&format!("Found {} PipeWire clients", clients.len()));
    
    for (id, name) in &clients {
        println!("ID {:>3}: {}", id, name);
    }
    
    heading(&format!("Found {} PipeWire sinks", sinks.len()));
    
    for (id, description) in &sinks {
        println!("ID {:>3}: {}", id, description);
    }
}