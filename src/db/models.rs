use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct Instances {
    pub instance_id: i32,
    pub container_id: String,
    pub instance_name: String,
}