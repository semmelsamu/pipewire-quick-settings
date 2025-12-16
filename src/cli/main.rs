use crate::utils::{heading, prompt};
use crate::models::state::PipeWireState;
use crate::pipewire::{pw_dump, wpctl_set_default};
use crate::printers;

pub fn cli_loop() {
    heading("PipeWire Quick Settings");
    print_options();

    loop {

        let input = prompt("What do you want to do?");

        let data = pw_dump();
        let state = PipeWireState::new(&data);

        match input.trim().to_lowercase().as_str() {
            "q" => break,
            "s" => {
                heading("Available sinks");

                for s in &state.sinks {
                    if state.is_default_sink(s) {
                        print!("* ");
                    } else {
                        print!("  ");
                    }

                    println!(
                        "{} {} ({}%) {}",
                        s.id,
                        s.description,
                        s.volume,
                        if s.muted { "Muted" } else { "" }
                    );
                }
            }
            "c" => {
                for d in &state.devices {
                    printers::device(d);
                }
            }
            "d" => {
                let input = prompt("Choose sink id");

                match input.trim().parse::<u32>() {
                    Ok(sink_id) => {
                        wpctl_set_default(sink_id);
                        println!("Set default sink to {}", sink_id);
                    }
                    Err(_) => println!("Invalid sink id"),
                }
            }
            _ => {
                println!("Invalid option");
                print_options();
            }
        }
    }

    println!("Bye bye.");
}

fn print_options() {
    heading("Options");
    println!("q - Quit application");
    println!("Dumps:");
    println!("s - Show all sinks");
    println!("c - Show all devices");
    println!("Settings:");
    println!("d - Set default sink");
    println!("v - Set volume for a sink");
    println!("m - Set mute for a sink");
    println!("r - Set route for a sink");
    println!("p - Set profile for a device");
}