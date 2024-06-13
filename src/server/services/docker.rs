extern crate bollard;
extern crate futures_util;

use crate::db::queries::{ insert_server, delete_server };

use crate::common::types;
use crate::common::errors;

use bollard::Docker;
use bollard::image::CreateImageOptions;
use bollard::container::{
    Config,
    CreateContainerOptions,
    ListContainersOptions,
    StartContainerOptions,
    LogOutput,
};
use bollard::exec::{ CreateExecOptions, StartExecOptions, StartExecResults };
use futures::stream::StreamExt;
use rand::{ thread_rng, Rng };
use std::default::Default;
use std::collections::HashMap;

pub struct DockerClient {
    docker: Docker,
}

impl Clone for DockerClient {
    fn clone(&self) -> Self {
        Self {
            docker: self.docker.clone(),
        }
    }
}

impl DockerClient {
    pub fn new() -> Result<Self, bollard::errors::Error> {
        let docker = Docker::connect_with_socket_defaults().unwrap();
        Ok(Self { docker })
    }

    pub async fn pull_image(&self) -> Result<(), errors::Errors> {
        let options = CreateImageOptions {
            from_image: "itzg/minecraft-servers",
            tag: "latest",
            ..Default::default()
        };

        let mut stream = self.docker.create_image(Some(options), None, None);

        if stream.next().await.is_none() {
            return Err(errors::Errors::PullImage("Failed to pull image".to_string()));
        }

        while let Some(_create_image_response) = stream.next().await {}

        Ok(())
    }

    pub async fn create_container(
        &self,
        server_type: types::ServerTypes,
        server_version: &str,
        port: &str,
        pool: &sqlx::SqlitePool
    ) -> Result<types::Instance, errors::Errors> {
        // Generate a random string for the container name
        let random_string: String = thread_rng()
            .sample_iter(rand::distributions::Uniform::new_inclusive(b'a', b'z'))
            .take(12)
            .map(|c| c as char)
            .collect();

        let container_name = format!("amethyst-{}", random_string);
        let server_version_var = format!("VERSION={}", server_version);
        let server_type_var = format!("TYPE={}", server_type);

        // Set up the configuration for the container
        let config = Config {
            image: Some("itzg/minecraft-server"),
            env: Some(vec!["EULA=TRUE", &server_type_var, &server_version_var]),
            host_config: Some(bollard::service::HostConfig {
                port_bindings: Some(
                    HashMap::from([
                        (
                            "25565/tcp".to_string(),
                            Some(
                                vec![bollard::service::PortBinding {
                                    host_ip: Some("0.0.0.0".to_string()),
                                    host_port: Some(port.to_string()),
                                }]
                            ),
                        ),
                    ])
                ),
                ..Default::default()
            }),
            ..Default::default()
        };

        // Create the container
        let container = &self.docker.create_container(
            Some(CreateContainerOptions {
                name: &container_name,
                platform: Some(&"linux/amd64".to_string()),
            }),
            config
        ).await;

        // Start the container
        let _ = &self.docker.start_container(
            &container_name,
            None::<StartContainerOptions<String>>
        ).await;

        match &container {
            Ok(container_data) => {
                if let Err(e) = insert_server(&pool, &container_data.id, &container_name).await {
                    Err(errors::Errors::DatabaseInsertion(e.to_string()))
                } else {
                    Ok(types::Instance {
                        container_id: container_data.id.clone(),
                        instance_name: format!("amethyst-{}", random_string),
                        address: None,
                        port: None,
                        server_type: server_type,
                        server_version: server_version.to_string(),
                        players: types::Players {
                            player_active: None,
                            player_max: None,
                            player_list: None,
                        },
                    })
                }
            },
            Err(e) => {
                return Err(errors::Errors::CreateContainer(e.to_string()));
            }
        }
    }

    pub async fn list_containers(
        &self
    ) -> Result<Vec<bollard::models::ContainerSummary>, bollard::errors::Error> {
        let options = Some(ListContainersOptions::<String> {
            all: true,
            // TODO: Get the list of servers from the database
            // Return those only.
            // Pass only values that required
            filters: HashMap::from([("name".to_string(), vec!["amethyst-".to_string()])]),
            ..Default::default()
        });

        let containers = self.docker.list_containers(options).await?;

        Ok(containers)
    }

    pub async fn start_container(&self, container_id: &str) -> Result<(), errors::Errors> {
        if let Err(e) = self.docker.start_container::<String>(container_id, None).await {
            return Err(errors::Errors::StartContainer(e.to_string()));
        }

        Ok(())
    }

    pub async fn restart_container(&self, container_id: &str) -> Result<(), errors::Errors> {
        if let Err(e) = self.docker.restart_container(container_id, None).await {
            return Err(errors::Errors::RestartContainer(e.to_string()));
        };

        Ok(())
    }

    pub async fn stop_container(&self, container_id: &str) -> Result<(), errors::Errors> {
        if let Err(e) = self.docker.stop_container(container_id, None).await {
            return Err(errors::Errors::StopContainer(e.to_string()));
        };

        Ok(())
    }

    pub async fn delete_container(
        &self,
        container_id: &str,
        pool: &sqlx::SqlitePool
    ) -> Result<String, errors::Errors> {
        if let Err(e) = self.docker.remove_container(container_id, None).await {
            return Err(errors::Errors::DeleteContainer(e.to_string()));
        }   

        if let Err(e) = delete_server(&pool, container_id).await {
            return Err(errors::Errors::DatabaseDeletion(e.to_string()));
        }
        Ok(container_id.to_string())
    }

    pub async fn exec_command(
        &self,
        container_id: &str,
        command: &str,
    ) -> Result<String, bollard::errors::Error> {

        let exec_create = CreateExecOptions {
            cmd: Some(vec!["/usr/local/bin/rcon-cli", command]),
            attach_stdout: Some(true),
            attach_stderr: Some(true), // Attach stderr to capture error output
            ..Default::default()
        };

        let exec_id = &self.docker.create_exec(&container_id, exec_create).await?.id;
        let start_options = StartExecOptions {
            detach: false, // Set detach to false to capture the output
            ..Default::default()
        };

        let exec_stream = self.docker.start_exec(exec_id, Some(start_options)).await?;
        let mut output_str = String::new();

        if let StartExecResults::Attached { mut output, .. } = exec_stream {
            while let Some(msg) = output.next().await {
                if let Ok(LogOutput::StdOut { message }) = msg {
                    output_str = String::from_utf8_lossy(&message).trim_end().to_string();
                }
            }
        }

        Ok(output_str)
    }
}
