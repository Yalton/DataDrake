use std::collections::HashMap;
use std::sync::{Arc, Mutex};
use serde::Serialize;

use uuid::Uuid;
use chrono::{DateTime, Utc};

fn serialize_datetime<S>(datetime: &DateTime<Utc>, serializer: S) -> Result<S::Ok, S::Error>
where
    S: serde::Serializer,
{
    let s = datetime.to_rfc3339();
    serializer.serialize_str(&s)
}

#[derive(Debug, Clone, Serialize)]
pub struct Job {
    pub id: Uuid,
    pub root_path: String,
    pub state: JobState,
    #[serde(serialize_with = "serialize_datetime")]
    pub created_at: DateTime<Utc>,
}

#[derive(Debug, Clone, PartialEq, Serialize)]
pub enum JobState {
    Pending,
    InProgress,
    Completed(String), 
    Failed(String), 
}

pub type JobQueue = Arc<Mutex<HashMap<Uuid, Job>>>;
