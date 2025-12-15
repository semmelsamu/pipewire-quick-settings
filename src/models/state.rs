use serde_json::Value;
use crate::models::device::Device;
use crate::models::sink::Sink;
use crate::parsers;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct PipeWireState {
    pub devices: Vec<Device>,
    pub sinks: Vec<Sink>,
    pub default_sink_name: Option<String>,
}

impl PipeWireState {
    pub fn new(data: &Value) -> Self {
        PipeWireState {
            devices: parsers::devices(data),
            sinks: parsers::audio_sinks(data),
            default_sink_name: parsers::default_sink(data),
        }
    }
}
