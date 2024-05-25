use shiplift::{rep::ContainerCreateInfo, ContainerOptions, Docker, Error};

pub async fn create_container(image_url: String) -> Result<ContainerCreateInfo, Error> {
    let docker = Docker::new();
    let image = image_url;

    match docker
        .containers()
        .create(&ContainerOptions::builder(&image.as_ref()).build())
        .await
    {
        Ok(container) => Ok(container),
        Err(error) => Err(error),
    }
}

pub async fn container_logs(id: String) {}

pub async fn version() -> Result<shiplift::rep::Version, Error> {
    let docker = Docker::new();
    match docker.version().await {
        Ok(ver) => Ok(ver),
        Err(e) => Err(e),
    }
}
