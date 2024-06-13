use serde::Deserialize;

#[derive(Deserialize)]
pub struct CreateInstanceRequest {
    pub server_type: String,
    pub server_version: String,
    pub port: String,
}

#[derive(Deserialize)]
pub struct ManageInstanceRequest {
    pub container_id: String,
}