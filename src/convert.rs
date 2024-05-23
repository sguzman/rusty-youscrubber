// This file will contain functions that will convert the JSON data
// into the sea_orm compatible data types.

use crate::data::{Channel, Payload};

// Import the sea_orm crate
use sea_orm::entity::prelude::*;

// This function will convert the Channel struct into the sea_orm compatible data types.
pub fn convert_channel(channel: Channel) -> Payload {
    sea
}