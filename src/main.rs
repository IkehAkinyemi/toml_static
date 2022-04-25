#![allow(dead_code)]
use serde::{Deserialize, Serialize};
use std::env;
use std::fs;

#[derive(Deserialize, Debug, Serialize)]
struct Input {
    xml_file: String,
    json_file: String,
}

#[derive(Deserialize, Debug, Serialize)]
struct Redis {
    host: String,
}

#[derive(Deserialize, Debug, Serialize)]
struct Sqlite {
    db_file: String
}

#[derive(Deserialize, Debug, Serialize)]
struct Postgresql {
    username: String,
    password: String,
    host: String,
    port: String,
    database: String
}

#[derive(Deserialize, Debug, Serialize)]
struct Config {
    input: Input,
    redis: Redis,
    sqlite: Sqlite,
    postgresql: Postgresql
}

fn main() {
    let config: Config = {
        let config_path = env::args().nth(1).expect("error reading command line arguments");
        let config_text = fs::read_to_string(config_path).expect("error reading file");
        toml::from_str(&config_text).expect("error reading stream")
    };

    let serialized = serde_json::to_string(&config).expect("error serializing to json");

    // println!("{:?}", config);
    println!("{}", serialized);
}
