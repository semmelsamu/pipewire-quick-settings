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
