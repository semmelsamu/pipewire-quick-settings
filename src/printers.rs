use crate::types::Device;
use crate::utils::heading;

pub fn device(device: &Device) {
    heading(&format!("{} ({})", device.name, device.id));
    
    println!("Profiles:");
    for (i, p) in device.profiles.iter().enumerate().filter(|(_, p)| p.available != "no") {
        print!("- {} {}", i, p.description);
        if i == device.current_profile as usize {
            print!(" (current)");
        }
        println!();
    }
    println!("Routes:");
    for (i, r) in device.routes.iter().enumerate().filter(|(_, r)| r.available != "no") {
        println!("- {} {}", i, r.description);
    }
}