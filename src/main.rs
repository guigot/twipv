mod config;
mod twitch_api;
mod ui;
mod use_mpv;

use cursive::Cursive;
use cursive::CursiveExt;
use futures::executor::block_on;
use serde_json::Value;
use tokio::runtime;
use std::env;
use std::fs::File;
use std::io::prelude::*;

enum Livestream {
    Number,
    Rofi,
}

async fn check_lives(livestream: Livestream) {
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

    let xdg_dirs = xdg::BaseDirectories::with_prefix("twipv").unwrap();
    let runtime_path = xdg_dirs.place_runtime_file("output").expect("Cannot create runtime directory");
    match livestream {
        Livestream::Number => {
            let mut output_string = String::new();
            let mut number_lives = 0;
            for livestreamer in &items {
                let value: Value = serde_json::from_str(&livestreamer).unwrap();
                if value["data"].as_array().unwrap().len() > 0 {
                    number_lives += 1;
                    let temp_output = format!(
                        "{:<15}{: <35.35}{:>20}\n",
                        value["data"][0]["user_login"].as_str().unwrap(),
                        value["data"][0]["title"].as_str().unwrap(),
                        value["data"][0]["game_name"].as_str().unwrap(),
                    );
                    output_string += &temp_output;
                }
            }
            println!("{}", number_lives);
            let mut file = File::create(runtime_path).unwrap();
            file.write_all(output_string.as_bytes()).unwrap();
            file.sync_data().unwrap();
        }
        Livestream::Rofi => {
            let mut file = File::open(runtime_path).unwrap();
            let mut output_string = String::new();
            file.read_to_string(&mut output_string).unwrap();
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
            block_on(check_lives(Livestream::Number));
        } else if first_arg == "rofi" {
            block_on(check_lives(Livestream::Rofi));
        }
    } else {
        let mut siv: Cursive = Cursive::default();

        ui::construct_ui(&mut siv);
        siv.run();
    }
}
