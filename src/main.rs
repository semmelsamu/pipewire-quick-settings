use std::process::Command;
use serde_json::Value;
use std::io;
use std::io::Write;

fn main() {
    let output = Command::new("pw-dump")
        .output()
        .expect("Error running pw-dump");

    if !output.status.success() {
        panic!("{}", String::from_utf8_lossy(&output.stderr));
    }

    let data: Value = serde_json::from_slice(&output.stdout).unwrap();
    
    let sinks = audio_sinks(&data);
    
    for s in &sinks {
        println!("{}", serde_json::to_string(&s).unwrap());
    }
    
    for s in &sinks {
        println!("{} {}", s["id"], s["info"]["props"]["node.description"]);
    }
    
    print!("Choose sink: > ");
    io::stdout().flush().unwrap();

    let mut input = String::new();

    io::stdin()
        .read_line(&mut input)
        .expect("Failed to read line");
        
    let sink_id: u32 = input.trim().parse().expect("Invalid sink id");

    println!("Chose sink {}", sink_id);
    
    wpctl_set_default(sink_id);
}

fn audio_sinks(data: &Value) -> Vec<&Value> {
    data.as_array()
        .into_iter()
        .flatten()
        .filter(|obj| {
            obj.get("type")
                .and_then(Value::as_str)
                == Some("PipeWire:Interface:Node")
            &&
            obj.get("info")
                .and_then(|i| i.get("props"))
                .and_then(|p| p.get("media.class"))
                .and_then(Value::as_str)
                == Some("Audio/Sink")
        })
        .collect()
}

fn wpctl_set_default(sink_id: u32) {
    let output = Command::new("wpctl")
        .arg("set-default")
        .arg(sink_id.to_string())
        .output()
        .expect("Error setting default sink");
}
