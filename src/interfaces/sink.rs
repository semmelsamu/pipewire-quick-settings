use serde_json::Value;
use crate::utils::value_as_u32;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Sink {
    pub id: u32,
    pub device: u32,
    pub port: u32,
    pub name: String,
    pub description: String,
}

impl Sink {
    /// Create a new Sink from a JSON object
    pub fn new(data: &Value) -> Option<Self> {
        let id = data.get("id").and_then(value_as_u32)?;
        let props = data.get("info")?.get("props")?;
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

        Some(Sink { id, device, port, name, description })
    }
}
