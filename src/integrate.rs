use sea_orm::prelude::DateTime;
use sea_orm::ActiveValue;
use sea_orm::ActiveValue::{NotSet, Set};

use crate::sea_orm_models::payload::ActiveModel;

use crate::data;

// If option set to None, set to NotSet
pub fn set<T>(option: Option<T>) -> ActiveValue<T>
where
    sea_orm::Value: From<T>,
{
    match option {
        Some(value) => Set(value),
        None => NotSet,
    }
}

fn setui(option: Option<u64>) -> ActiveValue<Option<i32>> {
    match option {
        Some(value) => Set(Some(value as i32)),
        None => NotSet,
    }
}

fn setsi(option: Option<String>) -> ActiveValue<Option<i32>> {
    match option {
        Some(value) => Set(Some(value.parse::<i32>().unwrap())),
        None => NotSet,
    }
}

fn setus(option: Option<u64>) -> ActiveValue<Option<sea_orm::prelude::DateTime>> {
    match option {
        Some(value) => Set(Some(DateTime::from_timestamp(value as i64, 0).into())),
        None => NotSet,
    }
}

fn setsin(option: Option<String>) -> ActiveValue<i32> {
    match option {
        Some(value) => Set(value.parse::<i32>().unwrap()),
        None => NotSet,
    }
}

fn setsu(option: Option<String>) -> ActiveValue<Option<sea_orm::prelude::DateTime>> {
    match option {
        Some(value) => Set(Some(
            DateTime::from_timestamp(value.parse::<i64>().unwrap(), 0).into(),
        )),
        None => NotSet,
    }
}

fn setu3i(option: Option<u32>) -> ActiveValue<Option<i32>> {
    match option {
        Some(value) => Set(Some(value as i32)),
        None => NotSet,
    }
}

pub fn create(payload: data::Channel) -> ActiveModel {
    ActiveModel {
        channel: set(payload.channel),
        channel_follower_count: setui(payload.channel_follower_count),
        channel_id: set(payload.channel_id),
        channel_url: Set(payload.channel_url),
        availability: setsi(payload.availability),
        description: Set(payload.description),
        epoch: setus(payload.epoch),
        extractor: Set(payload.extractor),
        extractor_key: Set(payload.extractor_key),
        payload_id: set(payload.id),
        modified_date: setsu(payload.modified_date),
        original_url: Set(payload.original_url),
        playlist_count: setui(payload.playlist_count),
        release_year: setu3i(payload.release_year),
        title: set(payload.title),
        type_of: Set(Some(payload.type_of)),
        uploader: set(payload.uploader),
        uploader_id: set(payload.uploader_id),
        uploader_url: Set(payload.uploader_url),
        view_count: setui(payload.view_count),
        webpage_url: Set(payload.webpage_url),
        webpage_url_basename: Set(payload.webpage_url_basename),
        webpage_url_host: Set(payload.webpage_url_domain),
        id: NotSet,
    }
}
