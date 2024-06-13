use regex::Regex;

use crate::server::handlers::types;
use crate::server::services::docker::DockerClient;

// List all online players using exec command to the docker container
pub async fn list_players(docker: &DockerClient, container_id: &str, pool: &sqlx::SqlitePool) -> Result<types::PlayersResponse, types::HandlerError> {
    
    let output = match docker.exec_command(container_id, "list", pool).await {
        Ok(output) => output,
        Err(error) => {
            return Err(types::HandlerError::CommandExecution(error.to_string()));
        },
    };
    
    let re = regex::Regex::new(r"There are (\d+) of a max of (\d+) players online:\s*(.*)").map_err(|err| {
        types::HandlerError::RegexCompilation(format!("Error while compiling regex: {}", err))
    });

    let players_response = match re {
        Ok(re) => {
            if let Some(captures) = re.captures(&output) {
                let current_players_str = captures.get(1).map_or("", |m| m.as_str());
                let max_players_str = captures.get(2).map_or("", |m| m.as_str());
                let player_list_str = captures.get(3).map_or("", |m| m.as_str().trim());

                let current_players = current_players_str;
                let max_players = max_players_str;
                let player_list = if player_list_str.is_empty() {
                    Vec::new()
                } else {
                    player_list_str.split(", ").map(String::from).collect::<Vec<String>>()
                };

                match (current_players, max_players) {
                    (current, max) => types::PlayersResponse {
                        player_active: Some(current.parse().unwrap()),
                        player_max: Some(max.parse().unwrap()),
                        player_list: Some(player_list),
                    }
                }
            } else {
                types::PlayersResponse {
                    player_active: None,
                    player_max: None,
                    player_list: None,
                }
            }
        },
        Err(_) => {
            return Err(types::HandlerError::RegexParsing("Error while parsing regex".to_string()));
        }
    };

    Ok(players_response)
}