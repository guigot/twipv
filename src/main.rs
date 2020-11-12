mod config;
mod twitch;
mod ui;
mod use_mpv;
// mod twitchv2;

use cursive::Cursive;
use cursive::CursiveExt;
use serde_json::Value;
use std::env;

fn check_lives() {
    let mut number_lives = 0;
    let favorites_streamers = config::value_array_field_config("favorites");
    for streamer in favorites_streamers {
        let live_streamer = twitch::check_live(streamer.as_str().unwrap());

        let value: Value = serde_json::from_str(&live_streamer).unwrap();
        if value["stream"] != Value::Null {
            number_lives = number_lives + 1;
        }
    }

    println!("{}", number_lives);
}

fn rofi_lives() {
    let mut output = String::new();
    let favorites_streamers = config::value_array_field_config("favorites");
    for streamer in favorites_streamers {
        let live_streamer = twitch::check_live(streamer.as_str().unwrap());
        let value: Value = serde_json::from_str(&live_streamer).unwrap();
        if value["stream"] != Value::Null {
            let temp_output = format!(
                "{: <20}{: <50}{: <25}\n",
                streamer.as_str().unwrap(),
                value["stream"]["channel"]["status"].as_str().unwrap(),
                value["stream"]["game"].as_str().unwrap()
            );
            output = output + &temp_output;
        }
    }

    println!("{}", output);
}

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() > 1 {
        let first_arg = &args[1];
        if first_arg == "number_lives" {
            check_lives();
        } else if first_arg == "rofi" {
            rofi_lives();
        }
    } else {
        let mut siv: Cursive = Cursive::default();
        ui::construct_ui(&mut siv);
        siv.run();
    }
}
