use log::error;
use log::info;

use std::path::Path;

pub mod data;
mod integrate;
pub mod sea_orm_models;

use integrate::create;
use yourust::validate_json_files;

// Set logging to debug
fn init_logger() {
    let mut builder = env_logger::Builder::from_default_env();
    builder.filter_level(log::LevelFilter::Debug);
    builder.init();
}

pub async fn convert_json_to_db() {
    let path = Path::new("resources");

    let mut files = Vec::new();

    for entry in path.read_dir().expect("read_dir call failed") {
        let entry = entry.expect("entry failed");
        let path = entry.path();

        if path.is_file() {
            files.push(path);
        }
    }

    for file in files {
        println!("{}", file.display());
        // Use serde to parse the json file
        let contents =
            std::fs::read_to_string(file).expect("Something went wrong reading the file");
        let res_payload: Result<data::Channel, _> = serde_json::from_str(&contents);
        match res_payload {
            Ok(payload) => {
                info!("File {:#?} is valid", payload.title);
                create(payload).await;
            }
            Err(e) => {
                error!("Error: {}", e);
                std::process::exit(1);
            }
        }
    }
}

#[tokio::main]
async fn main() {
    init_logger();
    info!("Hello, world!");
    validate_json_files();
    convert_json_to_db().await;
    info!("Goodbye, world!");
}
