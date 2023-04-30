use std::{env, io::ErrorKind};

use crate::{connectors::{track_changer_connector, current_track_connector}, configuration};

use super::vault_handler;

pub fn identify_and_run_args() -> Result<(), ErrorKind> {
    let args: Vec<String> = env::args().collect();
    if args.len() == 1 {
        eprintln!("Faltam argumentos para executar programa");
        return Err(ErrorKind::WriteZero)
    }
    if args[1] == "vault" {
        println!("Entered vault");
        return Ok(vault_handler::handle_vault_arguments());
    }
    return execute_command(&args[1]);
}



fn execute_command(command: &str) -> Result<(), ErrorKind> {
    if configuration::TRACK_CHANGE_COMMANDS.contains(&command) {
        return track_changer_connector::change_track(command);
    }
    if configuration::CURRENT_TRACK_COMMANDS.contains(&command) {
        return current_track_connector::change_current_track(command);
    }
    return Err(ErrorKind::InvalidInput);
}


pub fn handle_error(err: ErrorKind) {
    match err {
        ErrorKind::InvalidInput => {
            let arg_passed: Vec<String> = env::args().collect();
            println!("Invalid command: '{}'\n", arg_passed[1]);
            println!("Usage: spoth [command]");
            println!("Available commands:");
            configuration::get_track_commands(None)
        },
        _ => {}
    }
}