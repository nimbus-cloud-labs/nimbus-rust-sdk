//! Generated types – do not edit by hand.
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct BackendStatus {
    pub id: String,
    pub name: String,
    pub version: i64,
    pub protocol: TransportProtocol,
    pub targets: BackendStatusTargetsList,
}

pub type BackendStatusTargetsList = Vec<TargetStatus>;

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct CommandResponse {
    pub message: String,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct HealthResponse {
    pub status: String,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct ListListenersResponse {
    pub listeners: ListListenersResponseListenersList,
}

pub type ListListenersResponseListenersList = Vec<ListenerStatus>;

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct ListenerStatus {
    pub id: String,
    pub host: String,
    pub port: i32,
    pub version: i64,
    pub protocol: TransportProtocol,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub backend: Option<BackendStatus>,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct TargetStatus {
    pub id: String,
    pub address: String,
    pub port: i32,
    pub enabled: bool,
    pub protocol: TransportProtocol,
    pub version: i64,
}

#[derive(Clone, Debug, Serialize, Deserialize, PartialEq, Eq)]
pub enum TransportProtocol {
    #[serde(rename = "tcp")]
    Tcp,
    #[serde(rename = "udp")]
    Udp,
}
