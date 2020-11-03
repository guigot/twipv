extern crate xdg;

use std::fs::File;
use std::vec::Vec;
use std::io::prelude::*;
use toml::Value;

fn open_config_file() -> toml::Value {

    let xdg_dirs = xdg::BaseDirectories::with_prefix("mpv_stream").unwrap();
    // TODO : let user to not have a config file
    let config_path = xdg_dirs.find_config_file("config.toml").expect("Cannot find config file");
    let mut file = File::open(config_path).unwrap();

    let mut contents = String::new();
    file.read_to_string(&mut contents).unwrap();

    let value = contents.parse::<Value>().unwrap();
    value
}


// TODO : how handle differents types of fields 
pub fn value_array_field_config(field : &str) -> Vec<toml::Value> {

    let value = open_config_file();

    // TODO : check if field is array/not present
    let field_value = value[field].as_array().expect("Retrieve array field failed");

    field_value.to_vec()
}

pub fn value_string_field_config(field : &str) -> String {

    let value = open_config_file();

    let field_value = value[field].as_str().expect("Retrieve string field failed");

    field_value.to_string()
}

