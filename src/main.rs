mod utils;
mod pipewire;
mod parsers;
mod types;

use utils::prompt;
use pipewire::{pw_dump, wpctl_set_default};
use parsers::{default_sink, audio_sinks};

fn main() {
    println!("PipeWire Quick Settings");
    
    loop {
        println!();
        println!("0 = Exit");
        println!("1 = Show default sink");
        println!("2 = Set default sink");
        
        let input = prompt("What do you want to do?");

        match input.trim() {
            "0" => break,
            "1" => {
                let data = pw_dump();
                match default_sink(&data) {
                    Some(name) => println!("Default sink: {}", name),
                    None => println!("Default sink not found"),
                }
            }
            "2" => {
                let data = pw_dump();
                let sinks = audio_sinks(&data);

                for s in &sinks {
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
            _ => println!("Invalid option"),
        }
    }
}
