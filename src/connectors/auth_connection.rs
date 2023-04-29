use std::{error::Error, io::ErrorKind};

use base64::{Engine, engine::{self, general_purpose}, alphabet};
use reqwest;
use serde_json::{self, Value};
use crate::configuration;

pub fn get_auth_token(api_url: &str) -> Result<String, ErrorKind> {
    let encoded_str: String = format!("Basic {}", create_buffer_string());
    let response = reqwest::blocking::Client::new()
    .post(api_url)
    .header("Authorization", encoded_str)
    .form(&[("grant_type", "client_credentials")])
    .send();

    if response.is_err() {
        return Err(ErrorKind::ConnectionRefused)
    }

    let response_json: Value = serde_json::from_str(response.unwrap().text().unwrap().as_str()).unwrap();
    let str_json = serde_json::to_string(&response_json["access_token"]);
    return Ok(str_json.unwrap())
}

fn create_buffer_string() -> String {
    let buff_str = format!("{}:{}", configuration::CLIENT_ID, configuration::CLIENT_SECRET);
    let b64 = engine::GeneralPurpose::new(&alphabet::URL_SAFE, general_purpose::NO_PAD);
    return b64.encode(buff_str)
}



mod test {
    use super::get_auth_token;
    #[test]
    fn ao_entregar_url_trazer_token() {
        let url = "https://accounts.spotify.com/api/token";
        let result = get_auth_token(url);
        assert!(&result.is_ok());
        println!("{}", result.unwrap());
    }

}
