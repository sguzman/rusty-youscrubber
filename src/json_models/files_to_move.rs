use serde::{Deserialize, Serialize};

// pub Struct to represent the files_to_move field in the json files
#[derive(Debug, Deserialize, Serialize, Clone, PartialEq, Default)]
pub struct FilesToMove {}
