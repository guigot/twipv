use crate::config::string_field_config;
use curl::easy::{Easy2, Handler, List, WriteError};
use serde_json::Value;

struct Collector(Vec<u8>);

impl Handler for Collector {
    fn write(&mut self, data: &[u8]) -> Result<usize, WriteError> {
        self.0.extend_from_slice(data);
        Ok(data.len())
    }
}

fn query_twitch_api(url: &str) -> String {
    let mut easy = Easy2::new(Collector(Vec::new()));
    let mut list = List::new();
    let client_id = format!("Client-Id: {}", string_field_config("twitch-api-client-id"));

    let token = format!(
        "Authorization: Bearer {}",
        string_field_config("twitch-api-client-token")
    );
    list.append(&client_id).unwrap();
    list.append(&token).unwrap();
    easy.get(true).unwrap();
    easy.url(url).unwrap();
    easy.http_headers(list).unwrap();
    easy.perform().unwrap();
    let contents = easy.get_ref();

    String::from_utf8_lossy(&contents.0).to_string()
}

fn id_from_username(username: &str) -> u32 {
    let url = format!("{}{}", "https://api.twitch.tv/helix/users?login=", username);
    let result = query_twitch_api(&url);
    let mut user_id: u32 = 0;
    let val: Value = serde_json::from_str(&result).unwrap();

    if val["data"].as_array().unwrap().len() > 0 {
        user_id = val["data"][0]["id"]
            .as_str()
            .unwrap()
            .parse::<u32>()
            .unwrap();
    }

    user_id
}

pub fn get_vods(username: &str) -> String {
    let user_id: u32 = id_from_username(username);
    let url = format!(
        "https://api.twitch.tv/helix/videos?user_id={}&first=10&type=archive",
        user_id
    );

    query_twitch_api(&url)
}

pub async fn check_stream(username: &str) -> String {
    let user_id: u32 = id_from_username(username);
    let url = format!("https://api.twitch.tv/helix/streams?user_id={}", user_id);

    query_twitch_api(&url)
}
