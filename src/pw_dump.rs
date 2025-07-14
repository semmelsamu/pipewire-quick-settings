use std::process::Command;
use std::collections::HashMap;
use serde_json::Value;

pub fn pw_dump() -> String {
    let output = Command::new("pw-dump")
        .output()
        .expect("Failed to execute command pw-dump");

    String::from_utf8_lossy(&output.stdout).to_string()
}

pub fn parse_pw_clients(pw_dump: String) -> HashMap<i32, String> {
    let mut clients = HashMap::new();
    
    let json: Value = serde_json::from_str(&pw_dump)
        .expect("Failed to parse JSON from pw-dump output");
    
    let Value::Array(array) = json else { return clients };
    
    for obj in array {
        let Value::Object(map) = obj else { continue };

        let Some(Value::String(obj_type)) = map.get("type") else { continue };
        if obj_type != "PipeWire:Interface:Node" {
            continue;
        }

        // Nasty stuff.
        // Try to get the ID from `map`. Convert it to a JSON number. Continue if it fails.
        let Some(Value::Number(id)) = map.get("id") else { continue };
        // Convert the JSON number to an i32 (|x| x as i32 is a closure function). Continue if it fails.
        let Some(id_i32) = id.as_i64().map(|x| x as i32) else { continue };

        let Some(Value::Object(info)) = map.get("info") else { continue };
        let Some(Value::Object(props)) = info.get("props") else { continue };
        let Some(Value::String(app_name)) = props.get("application.name") else { continue };

        clients.insert(id_i32, app_name.clone());
    }
    
    clients
}

pub fn parse_pw_sinks(pw_dump: String) -> HashMap<i32, String> {
    let mut sinks = HashMap::new();
    
    let json: Value = serde_json::from_str(&pw_dump)
        .expect("Failed to parse JSON from pw-dump output");
    
    let Value::Array(array) = json else { return sinks };
    
    for obj in array {
        let Value::Object(map) = obj else { continue };
        
        let Some(Value::String(obj_type)) = map.get("type") else { continue };
        if obj_type != "PipeWire:Interface:Node" {
            continue;
        } 
        
        let Some(Value::Number(id)) = map.get("id") else { continue };
        let Some(id_i32) = id.as_i64().map(|x| x as i32) else { continue };
        
        let Some(Value::Object(info)) = map.get("info") else { continue };
        let Some(Value::Object(props)) = info.get("props") else { continue };
        let Some(Value::String(media_class)) = props.get("media.class") else { continue };
        
        if media_class != "Audio/Sink" {
            continue;
        }
        
        let description = if let Some(Value::String(desc)) = props.get("node.description") {
            desc.clone()
        } else if let Some(Value::String(nick)) = props.get("node.nick") {
            nick.clone()
        } else {
            "Unknown Sink".to_string()
        };
        
        sinks.insert(id_i32, description);
    }
    
    sinks
}

