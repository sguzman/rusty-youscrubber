use serde::{Deserialize, Serialize};

// pub Struct to represent the subtitles field in the json files
#[derive(Debug, Deserialize, Serialize, Clone, PartialEq, Default)]
pub struct Subtitle {
    pub ext: Option<String>,
    pub name: Option<String>,
    pub url: Option<String>,
}
