mod utils;
mod pipewire;
mod parsers;

use utils::prompt;
use serde_json::Value;
use pipewire::{pw_dump, wpctl_set_default};
use parsers::{default_sink, audio_sinks};

fn main() {
    let input = prompt("What do you want to do?");
    
    match input {
        _ => panic!("No valid option")
    }
    
    let data = pw_dump();
  
    let sinks = audio_sinks(&data);
    
    println!("Default sink name: {:?}", default_sink(&data));
    
    for s in &sinks {
        println!("{} {}", s["id"], s["info"]["props"]["node.description"]);
    }
    
    let input = prompt("Choose sink");
    let sink_id: u32 = input.trim().parse().expect("Invalid sink id");

    println!("Chose sink {}", sink_id);
 
    wpctl_set_default(sink_id);
}