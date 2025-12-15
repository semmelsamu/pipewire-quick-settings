use serde_json::Value;
use crate::models::device::Device;
use crate::models::sink::Sink;

pub fn devices(data: &Value) -> Vec<Device> {
    data.as_array()
        .into_iter()
        .flatten()
        .filter_map(|obj| Device::new(obj))
        .collect()
}

pub fn sinks(data: &Value) -> Vec<Sink> {
    data.as_array()
        .into_iter()
        .flatten()
        .filter_map(|obj| Sink::new(obj))
        .collect()
}

pub fn default_sink_name(dump: &Value) -> Option<String> {
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

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct PipeWireState {
    pub devices: Vec<Device>,
    pub sinks: Vec<Sink>,
    pub default_sink_name: Option<String>,
}

impl PipeWireState {
    pub fn new(data: &Value) -> Self {
        PipeWireState {
            devices: devices(data),
            sinks: sinks(data),
            default_sink_name: default_sink_name(data),
        }
    }
}
