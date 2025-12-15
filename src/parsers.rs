use serde_json::Value;
use crate::types::Device;
use crate::interfaces::sink::Sink;

pub fn devices(data: &Value) -> Vec<Device> {
    data.as_array()
        .into_iter()
        .flatten()
        .filter_map(|obj| Device::new(obj))
        .collect()
}


pub fn audio_sinks(data: &Value) -> Vec<Sink> {
    data.as_array()
        .into_iter()
        .flatten()
        .filter_map(|obj| Sink::new(obj))
        .collect()
}

pub fn default_sink(dump: &Value) -> Option<String> {
    dump
        .as_array()?
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