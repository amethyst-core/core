use regex::Regex;

use crate::common::types;
use crate::common::errors;

use crate::server::services::docker::DockerClient;

// List all online players using exec command to the docker container
pub async fn list_players(
    docker: &DockerClient,
    container_id: &str,
) -> Result<types::Players, errors::Errors> {
    let output = match docker.exec_command(container_id, "list").await {
        Ok(output) => output,
        Err(error) => {
            return Err(errors::Errors::CommandExecution(error.to_string()));
        }
    };

    let re = Regex::new(r"There are (\d+) of a max of (\d+) players online:\s*(.*)").map_err(|err| {
        errors::Errors::RegexCompilation(format!("Error while compiling regex: {}", err))
    });

    // for old mc versions
    // let re2 = Regex::new(r"There are (\d+)/(\d+) players online:\s*(.*)").map_err(|err| {
    //     errors::Errors::RegexCompilation(format!("Error while compiling regex: {}", err))
    // });

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
                    (current, max) =>
                        types::Players {
                            player_active: current.parse().unwrap(),
                            player_max: max.parse().unwrap(),
                            player_list: player_list,
                        },
                }
            } else {
                return Err(errors::Errors::FetchPlayers("Invalid console output".to_string()));
            }
        }
        Err(_) => {
            return Err(errors::Errors::RegexParsing("Error while parsing regex".to_string()));
        }
    };

    Ok(players_response)
}
