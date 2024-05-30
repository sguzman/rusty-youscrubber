use crate::json_models::format::Format;
use serde::{Deserialize, Serialize};

// pub Struct to represent the requested_downloads field in the json files
#[derive(Debug, Deserialize, Serialize, Clone, PartialEq, Default)]
pub struct RequestedDownload {
    #[serde(rename = "__write_download_archive")]
    pub write_download_archive: Option<bool>,

    pub _filename: Option<String>,
    pub abr: Option<f32>,
    pub acodec: Option<String>,
    pub aspect_ratio: Option<f32>,
    pub asr: Option<u32>,
    pub audio_channels: Option<u8>,
    pub audio_ext: Option<String>,
    pub columns: Option<u32>,
    pub dynamic_range: Option<String>,
    pub ext: Option<String>,
    pub filename: Option<String>,
    pub filesize_approx: Option<u64>,
    pub format: Option<String>,
    pub format_id: Option<String>,
    pub format_note: Option<String>,
    pub fps: Option<f32>,
    pub height: Option<u32>,
    pub language: Option<String>,
    pub protocol: Option<String>,
    pub requested_formats: Option<Vec<Format>>,
    pub resolution: Option<String>,
    pub tbr: Option<f32>,
    pub vbr: Option<f32>,
    pub vcodec: Option<String>,
    pub width: Option<u32>,
}
