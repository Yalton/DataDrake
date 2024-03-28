//database/structs.rs

use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use sqlx::FromRow;

#[derive(Serialize, Deserialize, FromRow, Debug, Clone)]
pub struct DirectoryScan {
    pub id: i64, // This will be set automatically by the database
    pub path: String,
    pub size: i64,
    pub root_path: String,
    pub scan_time: DateTime<Utc>,
}

impl DirectoryScan {
    // Assuming `size` is computed elsewhere and passed to this method
    pub fn new(path: String, size: i64, root_path: String, scan_time: DateTime<Utc>) -> Self {
        Self {
            id: 0, 
            path,
            size,
            root_path,
            scan_time,
        }
    }
}
