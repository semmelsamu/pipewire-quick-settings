use std::process::Command;
use serde_json::Value;
use std::io;
use std::io::Write;

fn main() {
    let input = prompt("What do you want to do?");
    
    match input {
        _ => panic!("No valid option")
    }
    
    let data = pw_dump();
  
    let sinks = audio_sinks(&data);
    
    println!("Default sink name: {:?}", get_default_sink_name(&data));
    
    for s in &sinks {
        println!("{} {}", s["id"], s["info"]["props"]["node.description"]);
    }
    
    let input = prompt("Choose sink");
    let sink_id: u32 = input.trim().parse().expect("Invalid sink id");

    println!("Chose sink {}", sink_id);
    
    wpctl_set_default(sink_id);
}

fn prompt(question: &str) -> String {
    print!("{} > ", question);
    io::stdout().flush().unwrap();
    
    let mut input = String::new();
    
    io::stdin()
        .read_line(&mut input)
        .expect("Failed to read line");
        
    input
}

fn pw_dump() -> Value {
    let output = Command::new("pw-dump")
        .output()
        .expect("Error running pw-dump");

    if !output.status.success() {
        panic!("{}", String::from_utf8_lossy(&output.stderr));
    }

    serde_json::from_slice(&output.stdout).expect("Failed to parse pw-dump output")
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
    Command::new("wpctl")
        .arg("set-default")
        .arg(sink_id.to_string())
        .output()
        .expect("Error setting default sink");
}

fn get_default_sink_name(dump: &Value) -> Option<String> {
    dump.as_array()?
        .iter()
        .find(|obj| {
            obj.get("type").and_then(Value::as_str)
                == Some("PipeWire:Interface:Metadata")
                &&
            obj.get("props")
                .and_then(|p| p.get("metadata.name"))
                .and_then(Value::as_str)
                == Some("default")
        })?
        .get("metadata")?
        .as_array()?
        .iter()
        .find(|item| {
            item.get("key").and_then(Value::as_str)
                == Some("default.audio.sink")
        })
        .and_then(|item| {
            let value = item.get("value")?;
            match value {
                Value::Object(map) => {
                    map.get("name")?.as_str().map(String::from)
                }
                Value::String(s) => Some(s.clone()),
                _ => None,
            }
        })
}
fn profiles(data: &Value) {
    
}
