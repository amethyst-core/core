use serde::{Serialize, Deserialize};
use std::fmt;
use std::collections::HashMap;

#[derive(Debug, Serialize, Deserialize)]
pub enum ServerTypes {
    Vanilla,
    Bedrock,
    Forge,
    Paper,
}

impl ServerTypes {
    pub fn from_str(s: &str) -> Option<Self> {
        match s {
            "vanilla" => Some(ServerTypes::Vanilla),
            "bedrock" => Some(ServerTypes::Bedrock),
            "forge" => Some(ServerTypes::Forge),
            "paper" => Some(ServerTypes::Paper),
            _ => None,
        }
    }
}

impl fmt::Display for ServerTypes {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            ServerTypes::Vanilla => write!(f, "Vanilla"),
            ServerTypes::Bedrock => write!(f, "Bedrock"),
            ServerTypes::Forge => write!(f, "Forge"),
            ServerTypes::Paper => write!(f, "Paper"),
        }
    }
}

#[derive(Serialize)]
pub struct State {
    pub started_at: Option<String>,
    pub running: Option<bool>,
    pub restarting: Option<bool>,
    pub dead: Option<bool>,
}

#[derive(Serialize)]
pub struct InstanceConfig {
    pub eula: bool,
    pub server_type: String,
    pub version: String,
}

#[derive(Serialize)]
pub struct Instance {
    pub container_id: String,
    pub image_id: String,
    pub created: String,
    pub instance_name: String,
    pub state: State,
    pub config: InstanceConfig,
    pub ports: HashMap<String, HashMap<(), ()>>,
    // pub address: Option<String>,
    // pub port: Option<Vec<u16>>,
    pub players: Players,
}

#[derive(Serialize)]
pub struct InstanceSummary {
    pub container_id: String,
    pub instance_name: String,
    pub created: i64,
    pub image_id: String,
    pub state: String,
    pub address: String,
    pub port: Vec<u16>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Players {
    pub player_active: u32,
    pub player_max: u32,
    pub player_list: Vec<String>,
}
