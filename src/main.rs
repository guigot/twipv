mod config;
mod twitch_api;
mod ui;
mod use_mpv;

use cursive::Cursive;
use cursive::CursiveExt;
use futures::executor;
use serde_json::Value;
use std::env;
use tokio;

enum Livestream {
    Number,
    Rofi,
}

async fn check_lives_bis(livestream: Livestream) {
    let favorites_streamers = config::array_field_config("favorites");

    let streamers: Vec<_> = favorites_streamers
        .into_iter()
        .map(|streamer| {
            tokio::spawn(async move {
                let check_stream = twitch_api::check_stream(streamer.as_str().unwrap()).await;
                check_stream
            })
        })
        .collect();

    let mut items = vec![];
    for streamer in streamers {
        items.push(streamer.await.unwrap());
    }

    match livestream {
        Livestream::Number => {
            let mut number_lives = 0;
            for livestreamer in &items {
                let value: Value = serde_json::from_str(&livestreamer).unwrap();
                if value["stream"] != Value::Null {
                    number_lives += 1;
                }
            }
            println!("{}", number_lives);
        }
        Livestream::Rofi => {
            let mut output_string = String::new();
            for livestreamer in &items {
                let value: Value = serde_json::from_str(&livestreamer).unwrap();
                if value["stream"] != Value::Null {
                    let temp_output = format!(
                        "{: <20}{: <50}{: <25}\n",
                        value["stream"]["channel"]["name"].as_str().unwrap(),
                        value["stream"]["channel"]["status"].as_str().unwrap(),
                        value["stream"]["game"].as_str().unwrap()
                    );
                    output_string += &temp_output;
                }
            }
            println!("{}", output_string);
        }
    }
}

#[tokio::main]
async fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() > 1 {
        let first_arg = &args[1];
        if first_arg == "number_lives" {
            executor::block_on(check_lives_bis(Livestream::Number));
        } else if first_arg == "rofi" {
            executor::block_on(check_lives_bis(Livestream::Rofi));
        }
    } else {
        let mut siv: Cursive = Cursive::default();
        ui::construct_ui(&mut siv);
        siv.run();
    }
}
