use std::collections::HashMap;

pub const CLIENT_ID: &str="";
pub const CLIENT_SECRET: &str="";
pub const TOKEN_URL: &str="https://accounts.spotify.com/api/token";
pub const PLAYER_HANDLING_URL: &str="https://api.spotify.com/v1/me/player";
pub const TRACK_CHANGE_COMMANDS: [&str; 2]=["next", "previous"];
pub const CURRENT_TRACK_COMMANDS: [&str; 3]=["pause", "resume", "repeat"];



fn get_commands_map() -> HashMap<String, String> {
    let commands = HashMap::from([
        (String::from("next"), String::from("jumps to next track")),
        (String::from("previous"), String::from("return to previous track")),
        (String::from("pause"), String::from("Stops the current track")),
        (String::from("resume"), String::from("Start the current track")),
        (String::from("repeat"), String::from("Restarts the current track"))
    ]);
    return commands;
}

pub fn get_commands(filtered_by: Option<Vec<&str>>) {
    if let Some(filter_commands) = filtered_by {
        for command in filter_commands {
            let command_from_map = get_commands_map()[command].clone();
            println!("\t{}: {}", command, command_from_map);
        }
        return;
    }
    for (c, d) in IntoIterator::into_iter(get_commands_map().clone()) {
        println!("\t{}: {}", c, d);
    }
}