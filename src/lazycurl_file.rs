use std::{fs::{self, File}, io::Write, path::Path};

use chrono::Utc;
use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize)]
pub struct LazyCurlFile {
    url: String
}

impl LazyCurlFile {
    pub fn new(url: String) -> LazyCurlFile {
        Self {
            url,
        }
    }

    pub fn save(&mut self) -> Result<(), Box<dyn std::error::Error>> {
       // Ensure the "history" directory exists
        let dir_path = Path::new("history");
        fs::create_dir_all(dir_path)?;

        let serialized = serde_json::to_string_pretty(&self)?;
        let now = Utc::now();
        let timestamp = now.format("%Y-%m-%dT%H-%M-%SZ").to_string();
        let filename = format!("history/lazy_curl_request_{}.json", timestamp);

        let mut file = File::create(filename)?;
        file.write_all(serialized.as_bytes())?;
        Ok(())
    }
}
