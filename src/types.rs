#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Device {
    pub id: u32,
    pub profiles: Vec<EnumProfile>,
    pub current_profile: Option<EnumProfile>,
    pub routes: Vec<EnumRoute>,
    pub name: String,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct EnumProfile {
    pub index: u32,
    pub description: String,
    pub priority: u32,
    pub available: String,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct EnumRoute {
    pub index: u32,
    pub description: String,
    pub priority: u32,
    pub available: String,
}

