use serde_json::Value;
use crate::utils::value_as_u32;
use crate::types::{Sink, Device};

pub fn devices(data: &Value) -> Vec<Device> {
    data.as_array()
        .into_iter()
        .flatten()
        .filter_map(|obj| {
            let is_device = obj.get("type").and_then(Value::as_str)
                == Some("PipeWire:Interface:Device");
                
            let media_class = obj.get("info")?.get("props")?
                .get("media.class")
                .and_then(Value::as_str)?
                .to_owned();
                
            if !is_device || media_class != "Audio/Device" {
                return None;
            }

            Some(Device {
                id: obj.get("id").and_then(value_as_u32)?,
                profiles: Vec::new(),
                current_profile: 0,
                routes: Vec::new(),
                name: obj.get("info")?.get("props")?
                    .get("device.product.name")
                    .and_then(Value::as_str)?
                    .to_owned(),
            })
        })
        .collect()
}

pub fn audio_sinks(data: &Value) -> Vec<Sink> {
    data.as_array()
        .into_iter()
        .flatten()
        .filter_map(|obj| {
            let is_sink = obj.get("type").and_then(Value::as_str)
                == Some("PipeWire:Interface:Node")
                && obj
                    .get("info")
                    .and_then(|i| i.get("props"))
                    .and_then(|p| p.get("media.class"))
                    .and_then(Value::as_str)
                    == Some("Audio/Sink");

            if !is_sink {
                return None;
            }

            let id = obj.get("id").and_then(value_as_u32)?;
            let props = obj.get("info")?.get("props")?;
            let device = props.get("device.id").and_then(value_as_u32)?;
            let port = props.get("card.profile.device").and_then(value_as_u32)?;
            let name = props
                .get("node.name")
                .and_then(Value::as_str)?
                .to_owned();
            let description = props
                .get("node.description")
                .and_then(Value::as_str)?
                .to_owned();

            Some(Sink { id, device, port, name,  description })
        })
        .collect()
}

pub fn default_sink(dump: &Value) -> Option<Sink> {
    let default_sink_name = dump
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
        })?;
        
    audio_sinks(dump)
        .into_iter()
        .find(|sink| sink.name == default_sink_name)
}