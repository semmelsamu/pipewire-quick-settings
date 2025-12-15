use serde_json::Value;
use std::process::Command;

pub fn pw_dump() -> Value {
    let output = Command::new("pw-dump")
        .output()
        .expect("Error running pw-dump");

    if !output.status.success() {
        panic!("{}", String::from_utf8_lossy(&output.stderr));
    }

    serde_json::from_slice(&output.stdout).expect("Failed to parse pw-dump output")
}

pub fn wpctl_set_default(sink_id: u32) {
    Command::new("wpctl")
        .arg("set-default")
        .arg(sink_id.to_string())
        .output()
        .expect("Error setting default sink");
}

pub fn wpctl_get_volume(sink_id: u32) -> Option<u32> {
    let output = Command::new("wpctl")
        .arg("get-volume")
        .arg(sink_id.to_string())
        .output()
        .ok()?;

    if !output.status.success() {
        return None;
    }

    let stdout = String::from_utf8(output.stdout).ok()?;

    // Parse output like "Volume: 0.50"
    stdout
        .lines()
        .find_map(|line| {
            if line.starts_with("Volume:") {
                let volume_str = line.strip_prefix("Volume:")?.trim();
                volume_str.parse::<f64>().ok()
            } else {
                None
            }
        })
        .map(|vol| (vol * 100.0) as u32)
}
