use serde_json::Value;
use crate::utils::value_as_u32;
use crate::types::Device;
use crate::interfaces::sink::Sink;
use crate::interfaces::profile::EnumProfile;
use crate::interfaces::route::EnumRoute;

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
            
            let routes: Vec<EnumRoute> = params
                .and_then(|p| p.get("EnumRoute"))
                .and_then(|v| v.as_array())
                .map(|arr| {
                    arr.iter()
                        .filter_map(|obj| EnumRoute::new(obj))
                        .collect()
                })
                .unwrap_or_default();
            
            let profiles: Vec<EnumProfile> = params
                .and_then(|p| p.get("EnumProfile"))
                .and_then(|v| v.as_array())
                .map(|arr| {
                    arr.iter()
                        .filter_map(|obj| EnumProfile::new(obj))
                        .collect()
                })
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
                routes,
                name,
            })
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