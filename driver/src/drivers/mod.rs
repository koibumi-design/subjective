mod onedrive;

use std::collections::HashMap;
use serde::Deserialize;

pub enum StaticDriver {}

pub enum DynamicDriver {}

#[derive(Debug, Deserialize, Clone)]
#[serde(tag = "type")]
pub enum DriverIndexConfig {}

pub struct RootDriver {
    pub static_drivers: HashMap<String, StaticDriver>,
    pub dynamic_drivers: HashMap<String, DynamicDriver>,
}