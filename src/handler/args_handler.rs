use std::{env, collections::HashMap, io::ErrorKind};

use crate::connectors::auth_connection;

pub fn identify_and_run_args() -> Result<String, ErrorKind> {
    let args: Vec<String> = env::args().collect();
    let result = get_entry_points_hash_table()[&args[1]].clone();
    if let Some(value) = result.clone().unwrap() {
        return Ok(value)
    }
    return Err(result.err().unwrap())
}



fn get_entry_points_hash_table<T>() -> HashMap<String, Result<T, ErrorKind>> {
    todo!()
}