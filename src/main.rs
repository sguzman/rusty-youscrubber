use serde::{Deserialize, Serialize};
use std::path::Path;

// Structs to represent the json files
#[derive(Debug, Deserialize, Serialize, Clone, PartialEq, Eq, Hash, Default)]
struct Payload {
    // type_of but in json, its called "_type"
    #[serde(rename = "_type")]
    type_of: String,
}

// Iterate across all the json files in resources directory
fn iterate_json_files() {
    let path = Path::new("resources");
    let display = path.display();

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

fn main() {
    println!("Hello, world!");
    iterate_json_files();
}
