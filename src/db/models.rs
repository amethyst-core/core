use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct Instances {
    pub instance_id: i32,
    pub instance_name: String,
    pub container_id: String,
    pub container_name: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Images {
    pub image_id: i32,
    pub image_name: String,
    pub image_tag: String,
    pub image_docker_id: String,
    pub image_status: String,
}