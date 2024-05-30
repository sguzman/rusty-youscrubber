use serde::{Deserialize, Serialize};

use crate::json_models::requested_format::RequestedFormat;

// pub Struct to represent the requested_downloads field in the json files
#[derive(Debug, Deserialize, Serialize, Clone, PartialEq, Default)]
pub struct RequestedDownload {
    #[serde(rename = "__write_download_archive")]
    pub write_download_archive: bool,

    pub _filename: String,
    pub abr: f32,
    pub acodec: String,
    pub aspect_ratio: f32,
    pub asr: u32,
    pub audio_channels: u8,
    pub audio_ext: String,
    pub columns: u32,
    pub dynamic_range: String,
    pub ext: String,
    pub filename: String,
    pub filesize_approx: u64,
    pub format: String,
    pub format_id: String,
    pub format_note: String,
    pub fps: f32,
    pub height: u32,
    pub language: String,
    pub protocol: String,
    pub requested_formats: Vec<RequestedFormat>,
    pub resolution: String,
    pub tbr: f32,
    pub vbr: f32,
    pub vcodec: String,
    pub width: u32,
}
