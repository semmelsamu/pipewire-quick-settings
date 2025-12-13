use std::process::Command;
use serde_json::Value;

fn main() {
    let output = Command::new("pw-dump")
        .output()
        .expect("Error running pw-dump");

    if !output.status.success() {
        panic!("{}", String::from_utf8_lossy(&output.stderr));
    }

    let data: Value = serde_json::from_slice(&output.stdout).unwrap();
    println!("{}", data[0]);
}
