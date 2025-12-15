use crate::pipewire::{wpctl_get_mute, wpctl_get_volume};
use crate::utils::value_as_u32;
use serde_json::Value;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Sink {
    pub id: u32,
    pub device: u32,
    pub port: u32,
    pub name: String,
    pub description: String,
    pub volume: u32,
    pub muted: bool,
}

impl Sink {
    /// Create a new Sink from a JSON object
    pub fn new(data: &Value) -> Option<Self> {
        let is_sink = data.get("type").and_then(Value::as_str) == Some("PipeWire:Interface:Node")
            && data
                .get("info")
                .and_then(|i| i.get("props"))
                .and_then(|p| p.get("media.class"))
                .and_then(Value::as_str)
                == Some("Audio/Sink");

        if !is_sink {
            return None;
        }

        let id = data.get("id").and_then(value_as_u32)?;
        let props = data.get("info")?.get("props")?;
        let device = props.get("device.id").and_then(value_as_u32)?;
        let port = props.get("card.profile.device").and_then(value_as_u32)?;
        let name = props.get("node.name").and_then(Value::as_str)?.to_owned();
        let description = props
            .get("node.description")
            .and_then(Value::as_str)?
            .to_owned();

        let volume = wpctl_get_volume(id).unwrap();
        let muted = wpctl_get_mute(id).unwrap();

        Some(Sink {
            id,
            device,
            port,
            name,
            description,
            volume,
            muted,
        })
    }
}
