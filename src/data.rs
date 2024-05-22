use serde::{Deserialize, Serialize};

// Struct to represent the files_to_move field in the json files
#[derive(Debug, Deserialize, Serialize, Clone, PartialEq, Eq, Hash, Default)]
struct FilesToMove {}

// Struct to represent the version field in the json files
#[derive(Debug, Deserialize, Serialize, Clone, PartialEq, Eq, Hash, Default)]
struct Version {
    current_git_head: Option<String>,
    release_git_head: Option<String>,
    repository: Option<String>,
    version: Option<String>,
}

// Structs to represent the json files
#[derive(Debug, Deserialize, Serialize, Clone, PartialEq, Eq, Hash, Default)]
pub struct Payload {
    #[serde(rename = "__files_to_move")]
    files_to_move: FilesToMove,
    // type_of but in json, its called "_type"
    #[serde(rename = "_type")]
    type_of: String,

    #[serde(rename = "_version")]
    version: Version,
}
