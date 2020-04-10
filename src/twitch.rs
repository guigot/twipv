use curl::easy::{Handler, Easy2, List, WriteError};
use serde_json::Value;

struct Collector(Vec<u8>);

impl Handler for Collector {
    fn write(&mut self, data: &[u8]) -> Result<usize, WriteError> {
        self.0.extend_from_slice(data);
        Ok(data.len())
    }
}

fn retrieve_id_from_username(username : &str) -> u32 {

    let mut easy = Easy2::new(Collector(Vec::new()));
    easy.get(true).unwrap();

    let mut list = List::new();
    list.append("Accept: application/vnd.twitchtv.v5+json").unwrap();
    list.append("Client-ID: ja58d80v5sp3m5y3p6kw068xuq49pw").unwrap();

    let url = format!("{}{}", "https://api.twitch.tv/kraken/users?login=", username);

    easy.url(&url).unwrap();

    easy.http_headers(list).unwrap();
    easy.perform().unwrap();
    let contents = easy.get_ref();
    let result = String::from_utf8_lossy(&contents.0).to_string();

    let mut user_id : u32 = 0;
    let val: Value = serde_json::from_str(&result).unwrap();
    
    if val["_total"].as_u64().unwrap() > 0 {
        user_id = val["users"][0]["_id"].as_str().unwrap().parse::<u32>().unwrap();
    }
    
    user_id
}

pub fn retrieve_videos(username : &str) -> String {

    let user_id : u32 = retrieve_id_from_username(username);

    let mut easy = Easy2::new(Collector(Vec::new()));
    easy.get(true).unwrap();
    // user_id default mistermv
    let url = format!("https://api.twitch.tv/kraken/channels/{}/videos?limit=10",
                      user_id);
    easy.url(&url).unwrap();

    let mut list = List::new();
    list.append("Accept: application/vnd.twitchtv.v5+json").unwrap();
    list.append("Client-ID: ja58d80v5sp3m5y3p6kw068xuq49pw").unwrap();

    easy.http_headers(list).unwrap();
    easy.perform().unwrap();
    let contents = easy.get_ref();

    String::from_utf8_lossy(&contents.0).to_string()
}
