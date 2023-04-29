use std::io::ErrorKind;

use crate::configuration;

use super::auth_connection;

pub fn change_track(command: &str) -> Result<(), ErrorKind> {
    let token = auth_connection::get_auth_token(configuration::TOKEN_URL);
    if token.is_err() {
        eprintln!("Ocorreu um erro ao buscar token da aplicação");
        return Err(ErrorKind::ConnectionRefused);
    }

    let full_token = format!("Bearer {}", token.unwrap());

    let client = reqwest::blocking::Client::new()
    .post(format!("{}/{}", configuration::PLAYER_HANDLING_URL, command))
    .header("Authorization", full_token)
    .form::<[(String, String); 0]>(&[])
    .send();

    let response_status = client.unwrap().status();


    if response_status.as_u16() != 200 {
        eprintln!("Status: {}", response_status.as_u16());
        return Err(ErrorKind::PermissionDenied);
    }

    return Ok(());
}