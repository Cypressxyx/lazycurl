use std::{fs, path::{Path, PathBuf}};

pub enum Directory {
    Root,
    History,
}

impl Directory {
    fn path(&self) -> String {
        match self {
            Directory::Root => ".".to_string(),
            Directory::History => {
                let root = Directory::Root.path();
                format!("{}/history", root)
            },
        }
    }
}

// Ensure the "history" directory exists
pub fn init_history_directory_if_not_exist() -> PathBuf {
    let dir = Directory::History.path();
    let dir_path = Path::new(dir.as_str());
    fs::create_dir_all(dir_path);
    dir_path.to_owned()
}
