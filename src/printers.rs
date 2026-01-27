use crate::models::device::Device;
use crate::models::state::PipeWireState;
use crate::utils::heading;

pub fn device(device: &Device) {
    heading(&format!("{} ({})", device.name, device.id));

    println!("Profiles:");
    for (_i, p) in device
        .profiles
        .iter()
        .enumerate()
        .filter(|(_, p)| p.available != "no")
    {
        if device.current_profile.as_ref().map(|cp| cp.index) == Some(p.index) {
            print!("* ");
        } else {
            print!("  ");
        }

        println!("{} {}", p.index, p.description);
    }
    println!("Routes:");
    for (_i, r) in device
        .routes
        .iter()
        .enumerate()
        .filter(|(_, r)| r.available != "no")
    {
        println!("  {} {}", r.index, r.description);
    }
}

pub fn sinks(state: &PipeWireState) {
    println!("Available sinks");
    for s in &state.sinks {
        if state.is_default_sink(s) {
            print!("* ");
        } else {
            print!("  ");
        }

        println!(
            "{}\t{} ({}%) {}",
            s.id,
            s.description,
            s.volume,
            if s.muted { "Muted" } else { "" }
        );
    }
}

pub fn devices(state: &PipeWireState) {
    println!("Available devices");
    for d in &state.devices {
        println!("  {}\t{}", d.id, d.name);
    }
}