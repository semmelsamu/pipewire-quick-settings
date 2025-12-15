mod utils;
mod pipewire;
mod parsers;
mod types;

use utils::prompt;
use pipewire::{pw_dump, wpctl_set_default};
use parsers::{default_sink, audio_sinks, devices};

fn main() {
    println!("PipeWire Quick Settings");
    
    loop {
        println!();
        println!("- 0 Exit");
        println!("- 1 Show default sink");
        println!("- 2 Set default sink");
        println!("- 3 Show devices");
        
        let input = prompt("What do you want to do?");

        match input.trim() {
            "0" => break,
            "1" => {
                let data = pw_dump();
                match default_sink(&data) {
                    Some(sink) => println!("Default sink: {}", sink.description),
                    None => println!("Default sink not found"),
                }
            }
            "2" => {
                let data = pw_dump();
                let sinks = audio_sinks(&data);

                for s in &sinks {
                    println!("- {} {}", s.id, s.description);
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
                    println!("- {} {}", d.id, d.name);
                    println!("  Profiles:");
                    for (i, p) in d.profiles.iter().enumerate().filter(|(_, p)| p.available != "no") {
                        println!("  - {} {}", i, p.description);
                    }
                    println!("  Routes:");
                    for (i, r) in d.routes.iter().enumerate().filter(|(_, r)| r.available != "no") {
                        println!("  - {} {}", i, r.description);
                    }
                }
            }
            _ => println!("Invalid option"),
        }
    }
    
    println!("Bye bye.");
}
