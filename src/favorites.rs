extern crate xdg;

use std::fs::File;
use std::io::prelude::*;
use toml::Value;


fn open_favorites_files() {

    let xdg_dirs = xdg::BaseDirectories::with_prefix("mpv_stream").unwrap();
    let config_path = xdg_dirs.find_config_file("config.toml").expect("cannot find config file");
    let file = File::open(config_path).unwrap();

    let mut contents = String::new();
    file.read_to_string(&mut contents).unwrap();
    let value = contents.parse::<Value>().unwrap();

    let favorites_streamers = value["favorites"].as_array().unwrap();


}
