use crate::models::state::PipeWireState;
use crate::pipewire::{
    pw_dump, wpctl_set_default, wpctl_set_mute, wpctl_set_profile, wpctl_set_route,
    wpctl_set_volume,
};
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
                printers::sinks(&state);
            }
            "c" => {
                for d in &state.devices {
                    printers::device(d);
                }
            }
            "d" => {
                println!("{}", "Set default sink".green().bold());

                let sink = prompt_sink(&state, false);

                println!(
                    "{}",
                    format!("Setting default sink to {}", sink.id)
                        .magenta()
                        .bold()
                );

                wpctl_set_default(sink.id);
            }
            "v" => {
                println!("{}", "Set volume for a sink".green().bold());

                let sink = prompt_sink(&state, true);
                
                println!("Current volume: {}%", sink.volume);

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

                let sink = prompt_sink(&state, true);
                
                println!("Current state: {}", if sink.muted { "Muted" } else { "Unmuted" });

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

                let sink = prompt_sink(&state, true);

                let route = prompt_u32("Choose route (0 is off)");

                println!(
                    "{}",
                    format!("Setting route for {} to {}", sink.id, route)
                        .magenta()
                        .bold()
                );

                wpctl_set_route(sink.id, route);
            }
            "p" => {
                println!("{}", "Set profile for a device".green().bold());
                
                printers::devices(&state);

                let device_id = prompt_u32("Choose device id");
                
                let device = state.devices.iter().find(|d| d.id == device_id).expect("Device not found");
                
                printers::device(device);

                let profile = prompt_u32("Choose profile");

                println!(
                    "{}",
                    format!("Setting profile for {} to {}", device.id, profile)
                        .magenta()
                        .bold()
                );

                wpctl_set_profile(device.id, profile);
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
