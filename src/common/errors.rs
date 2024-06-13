#[derive(Debug, thiserror::Error)]
pub enum Errors {
    #[error("Failed to pull image: {0}")] PullImage(String),
    
    // #[error("Failed to get server from database: {0}")] DatabaseQuery(String),
    #[error("Failed to insert server into database: {0}")] DatabaseInsertion(String),
    #[error("Failed to delete server from database: {0}")] DatabaseDeletion(String),
    
    #[error("Failed to create container: {0}")] CreateContainer(String),
    #[error("Invalid server type: {0}")] InvalidServerType(String),
    
    #[error("Failed to get container name for server: {0}")] DeleteContainer(String),
    
    #[error("Failed to start container: {0}")] StartContainer(String),
    #[error("Failed to restart container: {0}")] RestartContainer(String),
    #[error("Failed to stop container: {0}")] StopContainer(String),

    #[error("Failed to execute command: {0}")] CommandExecution(String),
    #[error("Failed to compile regex: {0}")] RegexCompilation(String),
    #[error("Failed to parse regex: {0}")] RegexParsing(String),
}
