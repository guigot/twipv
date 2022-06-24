use std::fs::File;
use std::io::prelude::*;
use std::vec::Vec;
use toml::Value;

fn data_config_file() -> toml::Value {
    let xdg_dirs = xdg::BaseDirectories::with_prefix("twipv").unwrap();
    let config_path = xdg_dirs
        .find_config_file("config.toml")
        .expect("Cannot find config file");
    let mut file = File::open(config_path).unwrap();
    let mut contents = String::new();
    file.read_to_string(&mut contents).unwrap();

    contents.parse::<Value>().unwrap()
}

pub fn array_field_config(field: &str) -> Vec<toml::Value> {
    let config_file = data_config_file();
    let field_value = config_file[field]
        .as_array()
        .expect("Get array field failed");

    field_value.to_vec()
}

pub fn string_field_config(field: &str) -> String {
    let config_file = data_config_file();
    let field_value = config_file[field]
        .as_str()
        .expect("Get string field failed");

    field_value.to_string()
}
