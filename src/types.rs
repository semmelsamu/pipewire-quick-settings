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

