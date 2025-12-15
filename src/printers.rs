use crate::utils::heading;
use crate::models::device::Device;

pub fn device(device: &Device) {
    heading(&format!("{} ({})", device.name, device.id));
    
    println!("Profiles:");
    for (_i, p) in device.profiles.iter().enumerate().filter(|(_, p)| p.available != "no") {
        if device.current_profile.as_ref().map(|cp| cp.index) == Some(p.index) {
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