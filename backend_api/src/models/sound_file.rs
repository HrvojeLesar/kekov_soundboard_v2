use serde::{Serialize, Deserialize};

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct SoundFile {
    /// unique file_name generated as snowflake
    #[serde(skip)]
    file_name: String,
    display_name: String,
}

impl SoundFile {
    pub fn new(file_name: String, display_name: String) -> Self {
        return Self {
            file_name,
            display_name,
        };
    }

    pub fn get_file_name(&self) -> &String {
        return &self.file_name;
    }
    
    pub fn get_display_name(&self) -> &String {
        return &self.display_name;
    }
}
