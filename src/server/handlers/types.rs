impl std::fmt::Display for HandlerError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            HandlerError::CommandExecution(err) => write!(f, "Command execution error: {}", err),
            HandlerError::RegexCompilation(err) => write!(f, "Regex compilation error: {}", err),
            HandlerError::RegexParsing(err) => write!(f, "Regex parsing error: {}", err),
        }
    }
}
pub enum HandlerError {
    CommandExecution(String),
    RegexCompilation(String),
    RegexParsing(String),
}

pub struct PlayersResponse {
    pub player_active: Option<u32>,
    pub player_max: Option<u32>,
}
