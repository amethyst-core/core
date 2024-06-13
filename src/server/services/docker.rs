#![allow(dead_code)]

extern crate bollard;
extern crate futures_util;

use crate::db::queries::{insert_server, delete_server, get_container_name, insert_image};

use bollard::{container, Docker};
use bollard::image::CreateImageOptions;
use bollard::container::{ Config , CreateContainerOptions, ListContainersOptions, StartContainerOptions, LogOutput };
use bollard::exec::{CreateExecOptions, CreateExecResults, StartExecOptions, StartExecResults};
use futures::stream::StreamExt;
use rand::{thread_rng, Rng};
use sqlx::pool;
use std::default::Default;
use std::collections::HashMap;

use indicatif::{ProgressBar, ProgressStyle};
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

    pub async fn pull_image(&self) -> Result<(), bollard::errors::Error> {
        let options = CreateImageOptions {
            from_image: "itzg/minecraft-server",
            tag: "latest",
            ..Default::default()
        };

        let mut stream = self.docker.create_image(Some(options), None, None);
        // TODO: add the image to db with status as pulling

        while let Some(_create_image_response) = stream.next().await {}

        Ok(())
    }

    pub async fn create_container(&self, server_type: &str, server_version: &str, port: &str, pool: &sqlx::SqlitePool) -> Result<String, bollard::errors::Error> {
        
        // Generate a random string for the container name
        let random_string: String = thread_rng()
            .sample_iter(rand::distributions::Uniform::new_inclusive(b'a', b'z'))
            .take(12)
            .map(|c| c as char)
            .collect();

        let container_name = format!("amethyst-{}", random_string);
        let server_version = format!("VERSION={}", server_version);
        let server_type = format!("TYPE={}", server_type);

        // Set up the configuration for the container
        let config = Config {
            image: Some("itzg/minecraft-server"),
            env: Some(vec![
                "EULA=TRUE",
                &server_type,
                &server_version,

            ]),
            host_config: Some(bollard::service::HostConfig {
                port_bindings: Some(HashMap::from([(
                    "25565/tcp".to_string(),
                    Some(vec![bollard::service::PortBinding {
                        host_ip: Some("0.0.0.0".to_string()),
                        host_port: Some(port.to_string()),
                    }]),
                )])),
                ..Default::default()
            }),
            ..Default::default()
        };

        // Create the container
        let container = &self.docker.create_container(Some(CreateContainerOptions {
            name: &container_name,
            platform: Some(&"linux/amd64".to_string()),
        }), config).await?;

        // Start the container
        let _ = &self.docker.start_container(&container_name, None::<StartContainerOptions<String>>).await?;

        // Insert the container into the database
        if let Err(e) = insert_server(&pool, &container.id, &container_name).await {
            eprintln!("Failed to insert server: {}", e);
        } else {
            println!("Server inserted successfully with container name: {}", container_name);
        }

        Ok(container.id.to_string())
    }

    pub async fn list_containers(&self) -> Result<Vec<bollard::models::ContainerSummary>, bollard::errors::Error> {
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

    pub async fn start_container(&self, container_id: &str) -> Result<(), bollard::errors::Error> {
        self.docker.start_container::<String>(container_id, None).await?;
        // TODO: beautify
        Ok(())
    }

    pub async fn restart_container(&self, container_id: &str) -> Result<(), bollard::errors::Error> {
        self.docker.restart_container(container_id, None).await?;
        Ok(())
    }

    pub async fn stop_container(&self, container_id: &str) -> Result<(), bollard::errors::Error> {
        self.docker.stop_container(container_id, None).await?;
        Ok(())
    }

    pub async fn delete_container(&self, container_id: &str, pool: &sqlx::SqlitePool) -> Result<String, bollard::errors::Error> {
        self.docker.remove_container(container_id, None).await?;
        // Delete the container from the database
        if let Err(e) = delete_server(&pool, container_id).await {
            eprintln!("Failed to delete server: {}", e);
        }
        Ok(container_id.to_string())
    }

    pub async fn exec_command(&self, container_id: &str, command: &str, pool: &sqlx::SqlitePool) -> Result<String, bollard::errors::Error> {

        // get container_name from pool using container_id
        let container_name = get_container_name(&pool, container_id).await.unwrap();

        let exec_create = CreateExecOptions {
            cmd: Some(vec![
                "/usr/local/bin/rcon-cli",
                command,
            ]),
            attach_stdout: Some(true),
            attach_stderr: Some(true), // Attach stderr to capture error output
            ..Default::default()
        };

        // get container_name from pool using container_id

        let exec_id = &self.docker.create_exec(&container_name, exec_create).await?.id;
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