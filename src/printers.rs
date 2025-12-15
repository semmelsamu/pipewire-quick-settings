use crate::types::Device;
use crate::utils::heading;

pub fn device(device: &Device) {
    heading(&format!("{} ({})", device.name, device.id));
    
    println!("Profiles:");
    for (_i, p) in device.profiles.iter().enumerate().filter(|(_, p)| p.available != "no") {
        if p.index == device.current_profile {
            print!("* ");
        } else {
            print!("  ");
        }
        
        println!("{} {}", p.index, p.description);
    }
    println!("Routes:");
    for (_i, r) in device.routes.iter().enumerate().filter(|(_, r)| r.available != "no") {
        println!("  {} {}", r.index, r.description);
    }
}