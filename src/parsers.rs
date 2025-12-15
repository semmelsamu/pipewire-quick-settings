use serde_json::Value;
use crate::utils::value_as_u32;
use crate::types::{Device, EnumProfile, EnumRoute};
use crate::interfaces::sink::Sink;

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

            let props = obj.get("info")?.get("props")?;
            let params = obj.get("info")?.get("params");
            
            // Try device.product.name first, fallback to device.description or device.alias
            let name = props
                .get("device.product.name")
                .and_then(Value::as_str)
                .or_else(|| props.get("device.description").and_then(Value::as_str))
                .or_else(|| props.get("device.alias").and_then(Value::as_str))
                .map(String::from)?;
            
            let profiles = params
                .and_then(|p| p.get("EnumProfile"))
                .map(|v| enum_profiles(v))
                .unwrap_or_default();
            
            let current_profile_id = params
                .and_then(|p| p.get("Profile"))
                .and_then(|v| v.as_array())
                .and_then(|arr| arr.get(0))
                .and_then(|profile| profile.get("index"))
                .and_then(|idx| idx.as_u64().map(|x| x as u32));
            
            let current_profile = current_profile_id
                .and_then(|id| profiles.iter().find(|p| p.index == id).cloned());

            Some(Device {
                id: obj.get("id").and_then(value_as_u32)?,
                profiles,
                current_profile,
                routes: params
                    .and_then(|p| p.get("EnumRoute"))
                    .map(|v| enum_routes(v))
                    .unwrap_or_default(),
                name,
            })
        })
        .collect()
}

fn enum_profiles(data: &Value) -> Vec<EnumProfile> {
    data.as_array()
        .into_iter()
        .flatten()
        .filter_map(|obj| {
            Some(EnumProfile { 
                index: obj.get("index").and_then(|idx| idx.as_u64().map(|x| x as u32))?,
                description: obj.get("description").and_then(Value::as_str)?.to_owned(), 
                priority: obj.get("priority").and_then(value_as_u32)?, 
                available: obj.get("available").and_then(Value::as_str)?.to_owned() 
            })
        })
        .collect()
}

fn enum_routes(data: &Value) -> Vec<EnumRoute> {
    data.as_array()
        .into_iter()
        .flatten()
        .filter_map(|obj| {
            Some(EnumRoute { 
                index: obj.get("index").and_then(|idx| idx.as_u64().map(|x| x as u32))?,
                description: obj.get("description").and_then(Value::as_str)?.to_owned(), 
                priority: obj.get("priority").and_then(value_as_u32)?, 
                available: obj.get("available").and_then(Value::as_str)?.to_owned() })
        })
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