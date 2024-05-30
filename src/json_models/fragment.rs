use serde::{Deserialize, Serialize};

// pub Struct to represent the fragment field in the json files
#[derive(Debug, Deserialize, Serialize, Clone, PartialEq, Default)]
pub struct Fragment {
    pub duration: f32,
    pub url: String,
}
