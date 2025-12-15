use serde_json::Value;
use crate::utils::value_as_u32;
use crate::interfaces::profile::EnumProfile;
use crate::interfaces::route::EnumRoute;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Device {
    pub id: u32,
    pub profiles: Vec<EnumProfile>,
    pub current_profile: Option<EnumProfile>,
    pub routes: Vec<EnumRoute>,
    pub name: String,
}

impl Device {
    /// Create a new Device from a JSON object
    pub fn new(data: &Value) -> Option<Self> {
        let is_device = data.get("type").and_then(Value::as_str)
            == Some("PipeWire:Interface:Device");
            
        let media_class = data.get("info")?.get("props")?
            .get("media.class")
            .and_then(Value::as_str)?
            .to_owned();
            
        if !is_device || media_class != "Audio/Device" {
            return None;
        }

        let props = data.get("info")?.get("props")?;
        let params = data.get("info")?.get("params");
        
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
            id: data.get("id").and_then(value_as_u32)?,
            profiles,
            current_profile,
            routes,
            name,
        })
    }
}

