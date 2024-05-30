use std::path::Path;

// Import the log and env_logger crates
use log::{error, info, warn};

// Iterate across all the json files in resources directory
pub fn validate_json_files<A, B>()
where
    A: serde::de::DeserializeOwned,
    A: std::fmt::Debug,
    B: serde::de::DeserializeOwned,
    B: std::fmt::Debug,
{
    info!("Validating json files");
    let path = Path::new("resources");

    let mut files = Vec::new();

    for entry in path.read_dir().expect("read_dir call failed") {
        let entry = entry.expect("entry failed");
        let path = entry.path();

        if path.is_file() {
            files.push(path);
        }
    }

    // Sort the files
    files.sort();

    for file in files {
        println!("{}", file.display());
        // Use serde to parse the json file
        let contents =
            std::fs::read_to_string(file).expect("Something went wrong reading the file");
        let res_payload: Result<A, _> = serde_json::from_str(&contents);
        match res_payload {
            Ok(_) => {
                info!("File (Type A) is valid");
            }
            Err(e) => {
                error!("Error: {}", e);
                warn!("Trying Type B");
                let res_payload: Result<B, _> = serde_json::from_str(&contents);
                match res_payload {
                    Ok(_) => {
                        info!("File (Type B) is valid");
                    }
                    Err(e) => {
                        error!("Error: {}", e);
                        std::process::exit(1);
                    }
                }
            }
        }
    }

    info!("All files are valid");
}
