use sqlx::SqlitePool;

pub async fn create_schema(pool: &SqlitePool) -> Result<(), sqlx::Error> {
    sqlx::query(
        r#"
        CREATE TABLE IF NOT EXISTS instances (
            instanceId INTEGER PRIMARY KEY AUTOINCREMENT,
            instanceName TEXT NOT NULL,
            containerId TEXT NOT NULL,
            containerName TEXT NOT NULL
        );
        
        CREATE TABLE IF NOT EXISTS images (
            imageId INTEGER PRIMARY KEY AUTOINCREMENT,
            imageName TEXT NOT NULL,
            imageTag TEXT NOT NULL,
            imageDockerId TEXT NOT NULL,
            imageStatus TEXT NOT NULL
        );
        "#,
    )
    .execute(pool)
    .await?;
    Ok(())
}