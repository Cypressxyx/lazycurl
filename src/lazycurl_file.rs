use std::{fs::{self, File}, io::{Read, Write}, path::{Path, PathBuf}};

use chrono::Utc;
use serde::{Serialize, Deserialize};

use crate::{http_method::HTTPMethod, utils::directory::{init_collection_directory_if_not_exist, init_history_directory_if_not_exist, Directory}};
use tui_tree_widget::TreeItem;

#[derive(Serialize, Deserialize, Clone)]
pub struct LazyCurlFile {
    pub url: String,
    pub headers: Vec<String>,
    pub http_method: HTTPMethod,
}

impl LazyCurlFile {
    pub fn new(url: String, headers: Vec<String>, http_method: HTTPMethod) -> LazyCurlFile {
        Self {
            url,
            headers,
            http_method
        }
    }

    pub fn save(&mut self) -> Result<(), Box<dyn std::error::Error>> {
        init_history_directory_if_not_exist();

        let serialized = serde_json::to_string_pretty(&self)?;
        let now = Utc::now();
        let timestamp = now.format("%Y-%m-%dT%H-%M-%SZ").to_string();
        let filename = format!("{}/lazy_curl_request_{}.json", Directory::History.path(), timestamp);

        let mut file = File::create(filename)?;
        file.write_all(serialized.as_bytes())?;
        Ok(())
    }

    pub fn get_history_lazycurlfiles(&mut self) -> Result<Vec<LazyCurlFile>, Box<dyn std::error::Error>> {
        let mut lazy_curl_files: Vec<LazyCurlFile> = Vec::new();
        let dir_path = init_history_directory_if_not_exist();
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

    pub fn get_collection_lazycurl_files<'a>(&mut self, root_directory: PathBuf) -> Result<Vec<TreeItem<'a,  String>>, Box<dyn std::error::Error>> {
        let mut collection_tree = Vec::new();
        //let dir_path = init_collection_directory_if_not_exist();
        for entry in fs::read_dir(root_directory)? {
            let path = entry?.path();
            if path.is_file() {
                let file_name = self.get_file_name(&path);
                //let file_name = path.file_name().unwrap().to_str().unwrap().to_owned();
                collection_tree.push(TreeItem::new_leaf(String::from(&file_name), String::from(&file_name)));
            } else if path.is_dir() {
                let data = self.get_collection_lazycurl_files(path).unwrap();
                data.iter().for_each(|v| collection_tree.push(v.clone()));
            }
        }

        Ok(collection_tree)
    }

    fn get_file_name(&mut self, path: &PathBuf) -> String {
        String::from(path.file_name().unwrap().to_str().unwrap())
    }
}
