use std::{os, env, io::{ErrorKind, self, Write}, fs::File};

use serde_json::json;

use crate::configuration::{self, VAULT_COMMANDS};

struct Vault {
    pub client_id: String,
    pub client_secret: String
}

impl Vault {
    pub fn new(client_id: &str, client_secret: &str) -> Self {
        return Vault { client_id: client_id.to_owned(), client_secret: client_secret.to_owned() }
    }


    pub fn save(&self) {
        let json_info = json!({
            "client_id": self.client_id,
            "client_secret": self.client_secret
        });

        let as_str = serde_json::to_string(&json_info);
        if as_str.is_ok() {
            let file = File::create("vault.json");
            if file.is_err() {
                eprintln!("An error occourred when try to create the vault file");
                panic!();
            }
            file.unwrap().write_all(as_str.unwrap().as_bytes()).unwrap();
        }
    }
}


pub fn handle_vault_arguments() {
    let complete_args = env::args().collect::<Vec<String>>();
    let vault_args = complete_args.clone()[2..].to_vec();
    let command_arg = vault_args[0].clone();
    println!("Command {}", &command_arg);
    if command_arg == "add-new" {
        println!("Is add new");
        let expected_args: u8 = 2;
        let args_error = check_complex_arguments(&vault_args, expected_args);
        if args_error.is_err() {
            eprintln!("An error occoured when try process command arguments");
            eprintln!("Expected: {}, got: {}", expected_args, &vault_args.len());
            panic!()
        }
        let vault = Vault::new(vault_args[1].as_str(), vault_args[2].as_str());
        println!("Created vault");
        vault.save();
    }
}



fn check_complex_arguments(vault_args: &Vec<String>, expected: u8) -> Result<(), ErrorKind> {
    if vault_args.len() < expected as usize {
        return Err(ErrorKind::InvalidInput);
    }
    return Ok(());
}