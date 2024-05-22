use std::path::Path;

// Import the Payload struct from the data.rs
mod data;
use data::Payload;

// Iterate across all the json files in resources directory
pub fn validate_json_files() {
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
        let payload: Payload = serde_json::from_str(&contents).expect("Error parsing json");
        println!("{:?}", payload);
    }
}
