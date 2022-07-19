use std::{convert::TryFrom, num::ParseIntError, str::FromStr};

use serde::{Deserialize, Serialize};

const KICK_MEMBERS: i64 = 1 << 1;
const BAN_MEMBERS: i64 = 1 << 2;
const ADMINISTRATOR: i64 = 1 << 3;
const MANAGE_CHANNELS: i64 = 1 << 4;
const MANAGE_GUILD: i64 = 1 << 5;
const MANAGE_MESSAGES: i64 = 1 << 13;
const MANAGE_ROLES: i64 = 1 << 28;
const MANAGE_THREADS: i64 = 1 << 34;
const MODERATE_MEMBERS: i64 = 1 << 40;

#[derive(Clone, Debug, Deserialize, PartialEq, Eq, Hash)]
#[serde(try_from = "String")]
pub struct Permissions(pub i64);

impl Permissions {
    pub fn is_admin(&self) -> bool {
        return self.0 & ADMINISTRATOR == ADMINISTRATOR;
    }

    pub fn other(&self) -> bool {
        return self.0 & MANAGE_GUILD == MANAGE_GUILD
            || self.0 & KICK_MEMBERS == KICK_MEMBERS
            || self.0 & BAN_MEMBERS == BAN_MEMBERS
            || self.0 & MANAGE_CHANNELS == MANAGE_CHANNELS
            || self.0 & MANAGE_MESSAGES == MANAGE_MESSAGES
            || self.0 & MANAGE_ROLES == MANAGE_ROLES
            || self.0 & MANAGE_THREADS == MANAGE_THREADS
            || self.0 & MODERATE_MEMBERS == MODERATE_MEMBERS;
    }
}

impl FromStr for Permissions {
    type Err = ParseIntError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        return Ok(Self(s.parse()?));
    }
}

impl TryFrom<String> for Permissions {
    type Error = ParseIntError;

    fn try_from(value: String) -> Result<Self, Self::Error> {
        return Self::from_str(&value);
    }
}

impl Serialize for Permissions {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        return serializer.serialize_str(&self.0.to_string());
    }
}

#[cfg(test)]
mod tests {
    use super::Permissions;

    #[test]
    fn test_is_admin() {
        let admin = Permissions(1 << 3);
        let not_admin = Permissions(1 << 0 | 1 << 1 | 1 << 2 | 1 << 4);

        assert!(admin.is_admin());
        assert!(!not_admin.is_admin());
    }

    #[test]
    fn test_other() {
        let valid = Permissions(1 << 1 | 1 << 2 | 1 << 3 | 1 << 4 | 1 << 5);
        let invalid = Permissions(1 << 6 | 1 << 12 | 1 << 27);
        let mix = Permissions(1 << 6 | 1 << 12 | 1 << 27 | 1 << 34 | 1 << 41);

        assert!(valid.other());
        assert!(!invalid.other());
        assert!(mix.other());
    }
}
