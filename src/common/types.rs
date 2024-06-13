use serde::{Serialize, Deserialize};
use std::fmt;

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
pub struct Instance {
    pub container_id: String,
    pub instance_name: String,
    pub address: Option<String>,
    pub port: Option<u16>,
    pub server_type: ServerTypes,
    pub server_version: String,
    pub players: Players,
}


#[derive(Serialize, Deserialize, Debug)]
pub struct Players {
    pub player_active: Option<u32>,
    pub player_max: Option<u32>,
    pub player_list: Option<Vec<String>>,
}
