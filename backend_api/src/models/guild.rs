use serde::{Deserialize, Serialize};

use crate::utils::deserialize_string_to_number;

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Guild {
    #[serde(deserialize_with = "deserialize_string_to_number")]
    id: i64,
    name: String,
    icon: Option<String>,
    icon_hash: Option<String>,
}

impl Guild {
    pub fn new(id: i64, name: String, icon: Option<String>, icon_hash: Option<String>) -> Self {
        return Self {
            id,
            name,
            icon,
            icon_hash,
        };
    }

    pub fn get_id(&self) -> &i64 {
        return &self.id;
    }

    pub fn get_name(&self) -> &String {
        return &self.name;
    }

    pub fn get_icon(&self) -> Option<&String> {
        return self.icon.as_ref();
    }

    pub fn get_icon_hash(&self) -> Option<&String> {
        return self.icon_hash.as_ref();
    }
}
