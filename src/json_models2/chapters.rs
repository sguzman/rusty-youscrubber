use serde::{Deserialize, Serialize};

// pub Struct to represent the chapters field in the json files
#[derive(Debug, Deserialize, Serialize, Clone, PartialEq, Default)]
pub struct Chapters {
    pub start_time: Option<f32>,
    pub end_time: Option<f32>,
    pub title: Option<String>,
}
