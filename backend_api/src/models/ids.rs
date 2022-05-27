use std::{convert::TryFrom, num::ParseIntError, str::FromStr};

use serde::{Deserialize, Serialize};

pub trait Id {
    fn get_id(&self) -> u64;
}

#[derive(Clone, Debug, Deserialize, PartialEq, Eq, Hash)]
#[serde(try_from = "String")]
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
#[serde(try_from = "String")]
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
#[serde(try_from = "String")]
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
#[serde(try_from = "String")]
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

#[cfg(test)]
mod tests {
    use super::{GuildId, UserId, SoundFileId, ChannelId};
    const TESTNUM: u64 = 123;

    #[test]
    fn test_deserialize_guild_id() {
        let id: GuildId = serde_json::from_str(r#""123""#).unwrap();
        assert_eq!(id.0, TESTNUM);
        let id: GuildId = serde_json::from_str(r#"123"#).unwrap();
        assert_eq!(id.0, TESTNUM);
    }

    #[test]
    fn test_deserialize_user_id() {
        let id: UserId = serde_json::from_str(r#""123""#).unwrap();
        assert_eq!(id.0, TESTNUM);
        let id: UserId = serde_json::from_str(r#"123"#).unwrap();
        assert_eq!(id.0, TESTNUM);
    }

    #[test]
    fn test_deserialize_sound_file_id() {
        let id: SoundFileId = serde_json::from_str(r#""123""#).unwrap();
        assert_eq!(id.0, TESTNUM);
        let id: SoundFileId = serde_json::from_str(r#"123"#).unwrap();
        assert_eq!(id.0, TESTNUM);
    }

    #[test]
    fn test_deserialize_channel_id() {
        let id: ChannelId = serde_json::from_str(r#""123""#).unwrap();
        assert_eq!(id.0, TESTNUM);
        let id: ChannelId = serde_json::from_str(r#"123"#).unwrap();
        assert_eq!(id.0, TESTNUM);
    }
}
