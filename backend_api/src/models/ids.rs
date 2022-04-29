use std::{str::FromStr, num::ParseIntError};

use serde::{Serialize, Deserialize};

pub trait Id {
    fn get_id(&self) -> u64;
}

#[derive(Clone, Debug, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub struct GuildId(pub u64);

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

#[derive(Clone, Debug, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub struct UserId(pub u64);

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

#[derive(Clone, Debug, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub struct SoundFileId(pub u64);

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

#[derive(Clone, Debug, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub struct ChannelId(pub u64);

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
