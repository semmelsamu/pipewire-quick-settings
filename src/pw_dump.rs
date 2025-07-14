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
    
    // Parse the JSON string
    let json: Value = serde_json::from_str(&pw_dump)
        .expect("Failed to parse JSON from pw-dump output");
    
    // Iterate through the array of objects
    if let Value::Array(array) = json {
        for obj in array {
            if let Value::Object(map) = obj {
                // Check if this is a PipeWire client
                if let Some(Value::String(obj_type)) = map.get("type") {
                    if obj_type == "PipeWire:Interface:Client" {
                        // Extract the ID
                        if let Some(Value::Number(id)) = map.get("id") {
                            if let Some(id_i32) = id.as_i64().map(|x| x as i32) {
                                // Extract the application name from props
                                if let Some(Value::Object(info)) = map.get("info") {
                                    if let Some(Value::Object(props)) = info.get("props") {
                                        if let Some(Value::String(app_name)) = props.get("application.name") {
                                            clients.insert(id_i32, app_name.clone());
                                        }
                                    }
                                }
                            }
                        }
                    }
                }
            }
        }
    }
    
    clients
}

pub fn parse_pw_sinks(pw_dump: String) -> HashMap<i32, String> {
    let mut sinks = HashMap::new();
    
    // Parse the JSON string
    let json: Value = serde_json::from_str(&pw_dump)
        .expect("Failed to parse JSON from pw-dump output");
    
    // Iterate through the array of objects
    if let Value::Array(array) = json {
        for obj in array {
            if let Value::Object(map) = obj {
                // Check if this is a PipeWire node
                if let Some(Value::String(obj_type)) = map.get("type") {
                    if obj_type == "PipeWire:Interface:Node" {
                        // Extract the ID
                        if let Some(Value::Number(id)) = map.get("id") {
                            if let Some(id_i32) = id.as_i64().map(|x| x as i32) {
                                // Check if this is an Audio/Sink node
                                if let Some(Value::Object(info)) = map.get("info") {
                                    if let Some(Value::Object(props)) = info.get("props") {
                                        if let Some(Value::String(media_class)) = props.get("media.class") {
                                            if media_class == "Audio/Sink" {
                                                // Extract the description (prefer node.description, fallback to node.nick)
                                                let description = if let Some(Value::String(desc)) = props.get("node.description") {
                                                    desc.clone()
                                                } else if let Some(Value::String(nick)) = props.get("node.nick") {
                                                    nick.clone()
                                                } else {
                                                    "Unknown Sink".to_string()
                                                };
                                                
                                                sinks.insert(id_i32, description);
                                            }
                                        }
                                    }
                                }
                            }
                        }
                    }
                }
            }
        }
    }
    
    sinks
}

