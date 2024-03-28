// Assuming `DirectoryScan` and necessary imports are defined elsewhere
use base64::{engine::general_purpose, Engine as _};
use chrono::Utc;
use std::{
    fs::File, io::Write, path::PathBuf
};
use walkdir::WalkDir;

use super::database::*;

#[derive(Debug, Clone)]
pub struct ScanService {}

impl ScanService {
    pub fn new() -> Self {
        ScanService {}
    }
    /// Scans a directory and its subdirectories, returning a list of `DirectoryScan` objects.
    #[tracing::instrument(skip_all)]
    pub async fn scan_directory(&self, root_path: PathBuf) -> Vec<DirectoryScan> {
        tracing::info!("Starting directory scan for {:?}", root_path.display());
        let mut scans = Vec::new();

        for entry in WalkDir::new(&root_path).into_iter().filter_map(Result::ok) {
            let path = entry.path();
            let metadata = match entry.metadata() {
                Ok(metadata) => metadata,
                Err(e) => {
                    tracing::warn!("Failed to get metadata for {:?}: {}", path.display(), e);
                    continue;
                }
            };

            let size = if metadata.is_file() {
                metadata.len() as i64
            } else {
                let mut dir_size = 0;
                for entry in WalkDir::new(path).into_iter().filter_map(Result::ok) {
                    if let Ok(metadata) = entry.metadata() {
                        if metadata.is_file() {
                            dir_size += metadata.len() as i64;
                        }
                    }
                }
                dir_size
            };

            let encoded_path = general_purpose::STANDARD.encode(path.to_string_lossy().as_bytes());

            scans.push(DirectoryScan {
                id: 0, // ID will be set by the database
                path: encoded_path.clone(),
                size,
                root_path: general_purpose::STANDARD.encode(root_path.to_string_lossy().as_bytes()),
                scan_time: Utc::now(),
            });
        }

        // Log scans to a file
        let log_file_path = "scans.log";
        let mut log_file = match File::create(log_file_path) {
            Ok(file) => file,
            Err(e) => {
                tracing::error!("Failed to create log file: {}", e);
                return scans;
            }
        };

        for scan in &scans {
            let log_entry = format!("{:?}\n", scan);
            if let Err(e) = log_file.write_all(log_entry.as_bytes()) {
                tracing::error!("Failed to write to log file: {}", e);
            }
        }

        tracing::info!("Completed directory scan for {:?}", root_path.display());
        scans
    }
}
