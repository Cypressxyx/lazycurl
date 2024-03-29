use std::{fs::{self, File}, io::{Read, Write}, path::Path};

use chrono::Utc;
use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize, Clone)]
pub struct LazyCurlFile {
    pub url: String,
    pub headers: Vec<String>
}

impl LazyCurlFile {
    pub fn new(url: String, headers: Vec<String>) -> LazyCurlFile {
        Self {
            url,
            headers
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

    pub fn get_history_lazycurlfiles(&mut self) -> Result<Vec<LazyCurlFile>, Box<dyn std::error::Error>> {
        let mut lazy_curl_files: Vec<LazyCurlFile> = Vec::new();
       // Ensure the "history" directory exists
        let dir_path = Path::new("history");
        fs::create_dir_all(dir_path)?;

        for entry in fs::read_dir(dir_path)? {
            let entry = entry?;
            let path = entry.path();
            if path.is_file() {
                let mut file = File::open(path)?;

                let mut contents = String::new();
                file.read_to_string(&mut contents)?;

                let lazy_curl_file: LazyCurlFile = serde_json::from_str(&contents)?;
                lazy_curl_files.push(lazy_curl_file);
            }
        }

        Ok(lazy_curl_files)
    }
}
