mod pw_dump;

use pw_dump::{pw_dump, parse_pw_clients, parse_pw_sinks};

fn main() {
    println!("Running pw-dump and parsing...");
    
    // Get the pw-dump output
    let dump_output = pw_dump();
    
    // Parse the clients
    let clients = parse_pw_clients(dump_output.clone());
    
    // Parse the sinks
    let sinks = parse_pw_sinks(dump_output);
    
    // Output the results
    println!("Found {} PipeWire clients:", clients.len());
    println!("{}", "=".repeat(40));
    
    for (id, name) in &clients {
        println!("ID {:>3}: {}", id, name);
    }
    
    println!("\nFound {} PipeWire sinks:", sinks.len());
    println!("{}", "=".repeat(40));
    
    for (id, description) in &sinks {
        println!("ID {:>3}: {}", id, description);
    }
}