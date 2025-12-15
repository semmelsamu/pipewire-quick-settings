use serde_json::Value;
use crate::utils::value_as_u32;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct EnumRoute {
    pub index: u32,
    pub description: String,
    pub priority: u32,
    pub available: String,
}

impl EnumRoute {
    /// Create a new EnumRoute from a JSON object
    pub fn new(data: &Value) -> Option<Self> {
        Some(EnumRoute {
            index: data.get("index").and_then(|idx| idx.as_u64().map(|x| x as u32))?,
            description: data.get("description").and_then(Value::as_str)?.to_owned(),
            priority: data.get("priority").and_then(value_as_u32)?,
            available: data.get("available").and_then(Value::as_str)?.to_owned(),
        })
    }
}

