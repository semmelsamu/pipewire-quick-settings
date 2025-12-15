mod utils;
mod pipewire;
mod printers;
mod models;

use utils::{prompt, heading};
use pipewire::{pw_dump, wpctl_set_default};
use crate::models::state::PipeWireState;

fn main() {
    heading("PipeWire Quick Settings");
    
    loop {
        heading("Options");
        println!("- 0 Exit");
        println!("- 1 Show default sink");
        println!("- 2 Set default sink");
        println!("- 3 Show devices and profiles");
        
        let input = prompt("What do you want to do?");
    
        let data = pw_dump();
        let state = PipeWireState::new(&data);

        match input.trim() {
            "0" => break,
            "1" => {
                match &state.default_sink {
                    Some(sink) => println!("Default sink: {}", sink.description),
                    None => println!("Could not find default sink"),
                }
            }
            "2" => {
                heading("Available sinks");
                
                for s in &state.sinks {
                    if state.is_default_sink(s) {
                        print!("* ");
                    } else {
                        print!("  ");
                    }
                    
                    println!("{} {}", s.id, s.description);
                }

                let input = prompt("Choose sink id");
                
                match input.trim().parse::<u32>() {
                    Ok(sink_id) => {
                        wpctl_set_default(sink_id);
                        println!("Set default sink to {}", sink_id);
                    }
                    Err(_) => println!("Invalid sink id"),
                }
            }
            "3" => {
                for d in &state.devices {
                    printers::device(d);
                }
            }
            _ => println!("Invalid option"),
        }
    }
    
    println!("Bye bye.");
}
