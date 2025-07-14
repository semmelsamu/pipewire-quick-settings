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

        // Check if it's a Node type
        let Some(obj_type) = map.get("type").and_then(|v| v.as_str()) else { continue };
        if obj_type != "PipeWire:Interface:Node" {
            continue;
        }

        // Get ID
        let Some(id) = map.get("id").and_then(|v| v.as_i64()) else { continue };
        let id_i32 = id as i32;

        // Get application name using chained navigation
        let Some(app_name) = map
            .get("info")
            .and_then(|v| v.as_object())
            .and_then(|obj| obj.get("props"))
            .and_then(|v| v.as_object())
            .and_then(|obj| obj.get("application.name"))
            .and_then(|v| v.as_str()) else { continue };

        clients.insert(id_i32, app_name.to_string());
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
        
        // Check if it's a Node type
        let Some(obj_type) = map.get("type").and_then(|v| v.as_str()) else { continue };
        if obj_type != "PipeWire:Interface:Node" {
            continue;
        } 
        
        // Get ID
        let Some(id) = map.get("id").and_then(|v| v.as_i64()) else { continue };
        let id_i32 = id as i32;
        
        // Get media class using chained navigation
        let Some(media_class) = map
            .get("info")
            .and_then(|v| v.as_object())
            .and_then(|obj| obj.get("props"))
            .and_then(|v| v.as_object())
            .and_then(|obj| obj.get("media.class"))
            .and_then(|v| v.as_str()) else { continue };
        
        if media_class != "Audio/Sink" {
            continue;
        }
        
        // Get description with fallback logic
        let Some(props) = map
            .get("info")
            .and_then(|v| v.as_object())
            .and_then(|obj| obj.get("props"))
            .and_then(|v| v.as_object()) else { continue };
            
        let description = props
            .get("node.description")
            .and_then(|v| v.as_str())
            .or_else(|| props.get("node.nick")
            .and_then(|v| v.as_str()))
            .unwrap_or("Unknown Sink");
        
        sinks.insert(id_i32, description.to_string());
    }
    
    sinks
}

