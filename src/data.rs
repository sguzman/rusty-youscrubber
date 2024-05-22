use serde::{Deserialize, Serialize};

// Struct to represent the files_to_move field in the json files
#[derive(Debug, Deserialize, Serialize, Clone, PartialEq, Eq, Hash, Default)]
struct FilesToMove {}

// Structs to represent the json files
#[derive(Debug, Deserialize, Serialize, Clone, PartialEq, Eq, Hash, Default)]
pub struct Payload {
    #[serde(rename = "__files_to_move")]
    files_to_move: FilesToMove,
    // type_of but in json, its called "_type"
    #[serde(rename = "_type")]
    type_of: String,
}
