mod utils;
mod pipewire;
mod parsers;
mod types;
mod printers;
mod interfaces;

use utils::{prompt, heading};
use pipewire::{pw_dump, wpctl_set_default};
use parsers::{default_sink, audio_sinks, devices};

fn main() {
    heading("PipeWire Quick Settings");
    
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
                match default_sink(&data) {
                    Some(sink) => heading(&format!("Default sink: {}", sink.description)),
                    None => heading("Default sink not found"),
                }
            }
            "2" => {
                let data = pw_dump();
                let sinks = audio_sinks(&data);
                
                heading("Available sinks");
                
                for s in sinks {
                    if Some(s.id) == default_sink(&data).map(|s| s.id) {
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
