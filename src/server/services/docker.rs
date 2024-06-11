#![allow(dead_code)]

extern crate bollard;
extern crate futures_util;

use bollard::Docker;
use bollard::image::CreateImageOptions;
use bollard::container::{ Config , CreateContainerOptions, ListContainersOptions, StartContainerOptions };
use futures::stream::StreamExt;
use rand::{thread_rng, Rng};
use std::default::Default;
use std::collections::HashMap;

pub struct DockerClient {
    docker: Docker,
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

        while let Some(create_image_response) = stream.next().await {
            if let Some(status) = create_image_response.unwrap().status {
                println!("{}", status);
            }
            // TODO: Show progress bar
        }
        Ok(())
    }

    pub async fn create_container(&self, server_type: &str, server_version: &str, port: &str) -> Result<String, bollard::errors::Error> {
        
        // Generate a random string for the container name
        let random_string: String = thread_rng()
            .sample_iter(rand::distributions::Uniform::new_inclusive(b'a', b'z'))
            .take(32)
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

        Ok(container.id.to_string())
    }

    pub async fn list_containers(&self) -> Result<Vec<bollard::models::ContainerSummary>, bollard::errors::Error> {
        let options = Some(ListContainersOptions::<String> {
            all: true,
            // Only list containers with the name "amethyst-"
            filters: HashMap::from([("name".to_string(), vec!["amethyst-".to_string()])]),
            ..Default::default()
        });

        let containers = self.docker.list_containers(options).await?;
        
        Ok(containers)
    }
    
    pub async fn start_container(&self, container_id: &str) -> Result<(), bollard::errors::Error> {
        self.docker.start_container::<String>(container_id, None).await?;
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

    pub async fn delete_container(&self, container_id: &str) -> Result<String, bollard::errors::Error> {
        self.docker.remove_container(container_id, None).await?;
        Ok(container_id.to_string())
    }

}
