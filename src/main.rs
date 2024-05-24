use log::error;
use log::info;

use std::path::Path;

mod constructors;
pub mod data;
mod integrate;
pub mod sea_orm_models;

use integrate::insert;

// Set logging to debug
fn init_logger() {
    let mut builder = env_logger::Builder::from_default_env();
    builder.filter_level(log::LevelFilter::Info);
    builder.init();
}

pub async fn convert_json_to_db() -> Vec<data::Channel> {
    let path = Path::new("resources");

    let mut files = Vec::new();

    for entry in path.read_dir().expect("read_dir call failed") {
        let entry = entry.expect("entry failed");
        let path = entry.path();

        if path.is_file() {
            files.push(path);
        }
    }

    let mut channels = Vec::new();
    for file in files {
        // Use serde to parse the json file
        let contents =
            std::fs::read_to_string(file).expect("Something went wrong reading the file");
        let res_payload: Result<data::Channel, _> = serde_json::from_str(&contents);
        match res_payload {
            Ok(payload) => {
                info!("File {:#?} is valid", payload.title);
                channels.push(payload);
            }
            Err(e) => {
                error!("Error: {}", e);
                std::process::exit(1);
            }
        }
    }

    channels
}

#[tokio::main]
async fn main() {
    init_logger();
    info!("Hello, world!");
    let cs = convert_json_to_db().await;
    insert(cs).await;
    info!("Goodbye, world!");
}
