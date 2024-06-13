use sqlx::sqlite::{SqlitePool, SqlitePoolOptions, SqliteConnectOptions};
use sqlx::{Row, Error};

pub async fn initialize_db() -> Result<SqlitePool, Error> {
    let pool = SqlitePoolOptions::new()
        .connect_with(
            SqliteConnectOptions::new()
                .create_if_missing(true)
                .filename("amethyst.db")
        )
        .await?;
    Ok(pool)
}

pub async fn insert_server(pool: &SqlitePool, container_id: &str, instance_name: &str) -> Result<(), sqlx::Error> {
    sqlx::query(
        r#"
        INSERT INTO instances (containerId, containerName, instanceName) 
        VALUES (?, ?, ?);
        "#
    )
    .bind(container_id)
    .bind(instance_name) 
    .bind(instance_name)
    .execute(pool)
    .await?;
    Ok(())
}

pub async fn delete_server(pool: &SqlitePool, container_id: &str) -> Result<(), sqlx::Error> {
    sqlx::query(
        r#"
        DELETE FROM instances WHERE containerId = ?;
        "#
    )
    .bind(container_id)
    .execute(pool)
    .await?;
    Ok(())
}

pub async fn get_container_name(pool: &SqlitePool, container_id: &str) -> Result<String, sqlx::Error> {
    let row = sqlx::query(
        r#"
        SELECT containerName FROM instances WHERE containerId = ?;
        "#
    )
    .bind(container_id)
    .fetch_one(pool)
    .await?;

    Ok(row.try_get("containerName")?)
}

pub async fn insert_image(pool: &SqlitePool, image_name: &str, image_tag: &str, image_docker_id: &str, image_status: &str) -> Result<(), sqlx::Error> {
    sqlx::query(
        r#"
        INSERT INTO images (imageName, imageTag, imageDockerId, imageStatus) 
        VALUES (?, ?, ?, ?);
        "#
    )
    .bind(image_name)
    .bind(image_tag)
    .bind(image_docker_id)
    .bind(image_status)
    .execute(pool)
    .await?;
    Ok(())
}