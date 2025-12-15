mod utils;
mod pipewire;
mod printers;
mod models;

use utils::{prompt, heading};
use pipewire::{pw_dump, wpctl_set_default};
use crate::models::state::{PipeWireState, devices, sinks, default_sink_name};

fn main() {
    heading("PipeWire Quick Settings");
    
    let data = pw_dump();
    let state = PipeWireState::new(&data);
    
    loop {
        heading("Options");
        println!("- 0 Exit");
        println!("- 1 Show default sink");
        println!("- 2 Set default sink");
        println!("- 3 Show devices and profiles");
        
        let input = prompt("What do you want to do?");

        match input.trim() {
            "0" => break,
            "1" => {
                let data = pw_dump();
                match default_sink_name(&data) {
                    Some(sink_name) => heading(&format!("Default sink: {}", sink_name)),
                    None => heading("Default sink not found"),
                }
            }
            "2" => {
                let data = pw_dump();
                let sinks = sinks(&data);
                
                heading("Available sinks");
                
                let default_sink_name = default_sink_name(&data);
                for s in sinks {
                    if default_sink_name.as_ref() == Some(&s.name) {
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
                let data = pw_dump();
                let devices = devices(&data);
                for d in &devices {
                    printers::device(d);
                }
            }
            _ => println!("Invalid option"),
        }
    }
    
    println!("Bye bye.");
}
