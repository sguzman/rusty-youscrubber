use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize, Clone, PartialEq, Default)]
pub struct AutomaticCaption {
    pub ext: Option<String>,
    pub protocol: Option<String>,
    pub url: Option<String>,
    pub name: Option<String>,
}
