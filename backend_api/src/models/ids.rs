use crate::utils::deserialize_string_to_number;
use std::{convert::TryFrom, num::ParseIntError, str::FromStr};

use serde::{Deserialize, Serialize};

pub trait Id {
    fn get_id(&self) -> u64;
}

#[derive(Clone, Debug, Deserialize, PartialEq, Eq, Hash)]
pub struct GuildId(#[serde(deserialize_with = "deserialize_string_to_number")] pub u64);

impl Id for GuildId {
    fn get_id(&self) -> u64 {
        return self.0;
    }
}

impl FromStr for GuildId {
    type Err = ParseIntError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        return Ok(Self(s.parse()?));
    }
}

impl TryFrom<String> for GuildId {
    type Error = ParseIntError;

    fn try_from(value: String) -> Result<Self, Self::Error> {
        return Self::from_str(&value);
    }
}

impl Serialize for GuildId {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        return serializer.serialize_str(&self.0.to_string());
    }
}

#[derive(Clone, Debug, Deserialize, PartialEq, Eq, Hash)]
pub struct UserId(#[serde(deserialize_with = "deserialize_string_to_number")] pub u64);

impl Id for UserId {
    fn get_id(&self) -> u64 {
        return self.0;
    }
}

impl FromStr for UserId {
    type Err = ParseIntError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        return Ok(Self(s.parse()?));
    }
}

impl TryFrom<String> for UserId {
    type Error = ParseIntError;

    fn try_from(value: String) -> Result<Self, Self::Error> {
        return Self::from_str(&value);
    }
}

impl Serialize for UserId {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        return serializer.serialize_str(&self.0.to_string());
    }
}

#[derive(Clone, Debug, Deserialize, PartialEq, Eq, Hash)]
pub struct SoundFileId(#[serde(deserialize_with = "deserialize_string_to_number")] pub u64);

impl Id for SoundFileId {
    fn get_id(&self) -> u64 {
        return self.0;
    }
}

impl FromStr for SoundFileId {
    type Err = ParseIntError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        return Ok(Self(s.parse()?));
    }
}

impl TryFrom<String> for SoundFileId {
    type Error = ParseIntError;

    fn try_from(value: String) -> Result<Self, Self::Error> {
        return Self::from_str(&value);
    }
}

impl Serialize for SoundFileId {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        return serializer.serialize_str(&self.0.to_string());
    }
}

#[derive(Clone, Debug, Deserialize, PartialEq, Eq, Hash)]
pub struct ChannelId(#[serde(deserialize_with = "deserialize_string_to_number")] pub u64);

impl Id for ChannelId {
    fn get_id(&self) -> u64 {
        return self.0;
    }
}

impl FromStr for ChannelId {
    type Err = ParseIntError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        return Ok(Self(s.parse()?));
    }
}

impl TryFrom<String> for ChannelId {
    type Error = ParseIntError;

    fn try_from(value: String) -> Result<Self, Self::Error> {
        return Self::from_str(&value);
    }
}

impl Serialize for ChannelId {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        return serializer.serialize_str(&self.0.to_string());
    }
}
