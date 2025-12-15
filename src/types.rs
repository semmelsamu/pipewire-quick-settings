#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Device {
    pub id: u32,
    pub profiles: Vec<EnumProfile>,
    pub current_profile: u32,
    pub routes: Vec<EnumRoute>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct EnumProfile {
    pub name: String,
    pub description: String,
    pub priority: u32,
    pub available: String,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Sink {
    pub id: u32,
    pub device: u32,
    pub port: u32,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct EnumRoute {
    pub name: String,
    pub description: String,
    pub priority: u32,
    pub available: String,
}

