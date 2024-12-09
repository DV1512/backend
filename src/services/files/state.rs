use std::{env::temp_dir, fs, path::PathBuf};

pub struct FilesServiceState {
    pub upload_path: PathBuf,
}

impl FilesServiceState {
    pub fn new() -> Self {
        let upload_path = match dirs::home_dir() {
            Some(path) => {
                let directory_name = "threatmapper_files";
                let full_path = path.join(directory_name);
                match fs::create_dir_all(&full_path) {
                    Ok(_) => full_path,
                    Err(_) => temp_dir(),
                }
            }
            None => temp_dir(),
        };

        Self { upload_path }
    }

    pub fn get_path_for(&self, filename: &str) -> PathBuf {
        self.upload_path.join(filename)
    }
}
