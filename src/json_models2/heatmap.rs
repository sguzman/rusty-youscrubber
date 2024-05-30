use serde::{Deserialize, Serialize};

// pub Struct to represent the heatmaps field in the json files
#[derive(Debug, Deserialize, Serialize, Clone, PartialEq, Default)]
pub struct HeatMap {
    pub end_time: Option<f32>,
    pub start_time: Option<f32>,
    pub value: Option<f32>,
}
