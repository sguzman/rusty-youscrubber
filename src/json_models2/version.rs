use serde::{Deserialize, Serialize};

// pub Struct to represent the version field in the json files
#[derive(Debug, Deserialize, Serialize, Clone, PartialEq, Default)]
pub struct Version {
    pub current_git_head: Option<String>,
    pub release_git_head: Option<String>,
    pub repository: Option<String>,
    pub version: Option<String>,
}
