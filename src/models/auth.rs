use serde::{Serialize};
use uniffi;

#[derive(Debug, Serialize, uniffi::Record)]
pub struct Capability {
    pub path: String,
    pub permission: String,
}

#[derive(Debug, Serialize, uniffi::Record)]
pub struct PubkyAuthDetails {
    pub relay: String,
    pub capabilities: Vec<Capability>,
    pub secret: String,
}