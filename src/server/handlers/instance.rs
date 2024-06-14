
use crate::common::types;
use crate::common::errors;

use crate::server::services::docker::DockerClient;
use crate::server::handlers::players::list_players;

use crate::db::queries::get_instance_name;
use std::collections::HashMap;

pub async fn get_instance(
    docker: &DockerClient,
    pool: &sqlx::SqlitePool,
    container_id: &str
) -> Result<types::Instance, errors::Errors> {
    match docker.inspect_container(container_id).await {
        Ok(container) => {
            let instance_name = get_instance_name(pool, container_id).await.unwrap_or_else(|_| "Unknown".to_string());
            
            let config = container.config.as_ref().unwrap();
            let state = container.state.as_ref().unwrap();
            let env = config.env.clone();

            // Parse the env vector into a HashMap
            let mut env_map = HashMap::new();

            if let Some(env_vars) = env {
                for var in env_vars {
                    let mut split = var.splitn(2, '=');
                    if let (Some(key), Some(value)) = (split.next(), split.next()) {
                        env_map.insert(key.to_string(), value.to_string());
                    }
                }
            }


            let players = match list_players(docker, container_id).await {
                Ok(players) => players,
                Err(_) => types::Players { player_active: 0, player_max: 0, player_list: vec![] },
            };


            Ok(types::Instance {
                container_id: container_id.to_string(),
                instance_name: instance_name,
                created: container.created,
                image_id: container.image,
                ports: config.exposed_ports.clone(),
                players: players,
                config: types::InstanceConfig {
                    eula: env_map.get("EULA").map(|value| value.to_lowercase().to_string() == "true"),
                    server_type: env_map.get("TYPE").map(|value| value.to_string()),
                    version: env_map.get("VERSION").map(|value| value.to_string()),
                },
                state: types::State{
                    started_at: state.started_at.clone(),
                    running: state.running.clone(),
                    restarting: state.restarting.clone(),
                    dead: state.dead.clone(),
                },
            })
        },
        Err(e) => Err(errors::Errors::GetInstance(e.to_string()))
    }
}

pub async fn list_instances(
    docker: &DockerClient,
    pool: &sqlx::SqlitePool
) -> Result<Vec<types::InstanceSummary>, errors::Errors> {
    match docker.list_containers().await {
        Ok(containers) => {

            let mut instances: Vec<types::InstanceSummary> = Vec::new();

            for container in containers {

                if let Some(id) = &container.id {

                    let instance_name = get_instance_name(pool, id).await.unwrap_or_else(|_| "Unknown".to_string());

                    // Extract ports
                    let ports: Option<Vec<u16>> = container.ports.as_ref().map(|ports| {
                        ports.iter()
                            .filter_map(|port| port.public_port)
                            .collect()
                    });

                    // Extract address
                    let address: String = container.network_settings
                        .as_ref()
                        .and_then(|net_settings| net_settings.networks.as_ref())
                        .and_then(|networks| networks.get("bridge"))
                        .and_then(|bridge| bridge.ip_address.clone())
                        .unwrap_or_else(|| "127.0.0.1".to_string());

                    instances.push(types::InstanceSummary {
                        container_id: id.to_string(),
                            instance_name: instance_name,
                            image_id: container.image_id,
                            created: container.created,
                            state: container.state,
                            address: address,
                            port: ports,
                        })
                    }
                }

                Ok(instances)
        },
        Err(err) => Err(errors::Errors::ListInstances(err.to_string())),
    }

}