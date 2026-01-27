use crate::models::state::PipeWireState;
use crate::pipewire::{pw_dump, wpctl_set_default, wpctl_set_mute, wpctl_set_route, wpctl_set_volume};
use crate::printers;
use crate::utils::{heading, prompt, prompt_sink, prompt_u32};
use colored::*;

pub fn cli_loop() {
    heading("PipeWire Quick Settings");
    print_options();

    loop {
        println!();
        let input = prompt("What do you want to do?");
        println!();

        let data = pw_dump();
        let state = PipeWireState::new(&data);

        match input.trim().to_lowercase().as_str() {
            "q" => {
                println!("{}", "Bye bye.".green().bold());
                break;
            }
            "s" => {
                println!("{}", "Available sinks".green().bold());

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
                println!("{}", "Available devices".green().bold());

                for d in &state.devices {
                    printers::device(d);
                }
            }
            "d" => {
                println!("{}", "Set default sink".green().bold());

                let input = prompt_u32("Choose sink id");

                println!(
                    "{}",
                    format!("Setting default sink to {}", input)
                        .magenta()
                        .bold()
                );

                wpctl_set_default(input);
            }
            "v" => {
                println!("{}", "Set volume for a sink".green().bold());

                let sink = prompt_sink(&state);

                let volume = prompt_u32("Choose volume (in %)");

                println!(
                    "{}",
                    format!("Setting volume for {} to {}", sink.id, volume)
                        .magenta()
                        .bold()
                );

                wpctl_set_volume(sink.id, volume);
            }
            "m" => {
                println!("{}", "Set mute for a sink".green().bold());

                let sink = prompt_sink(&state);

                let mute = prompt("Choose mute (y/n, leave empty for toggle)");

                let mute_bool;

                if mute.trim().is_empty() {
                    mute_bool = !sink.muted;
                } else if mute.trim().to_lowercase() == "y" {
                    mute_bool = true;
                } else if mute.trim().to_lowercase() == "n" {
                    mute_bool = false;
                } else {
                    panic!("Invalid option: {}", mute);
                }

                println!(
                    "{}",
                    format!("Setting mute for {} to {}", sink.id, mute_bool)
                        .magenta()
                        .bold()
                );

                wpctl_set_mute(sink.id, mute_bool);
            }
            "r" => {
                println!("{}", "Set route for a sink".green().bold());

                let sink = prompt_sink(&state);

                let route = prompt_u32("Choose route (0 is off)");
                
                println!(
                    "{}",
                    format!("Setting route for {} to {}", sink.id, route)
                        .magenta()
                        .bold()
                );

                wpctl_set_route(sink.id, route);
            }
            _ => {
                println!("{}", format!("Invalid option: {}", input).red().bold());
                print_options();
            }
        }
    }
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
