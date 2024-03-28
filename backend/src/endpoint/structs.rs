use serde::{Deserialize, Serialize};
// use std::path::PathBuf;
use uuid::Uuid;

#[derive(Debug, Serialize, Deserialize)]
pub struct PathRequest {
    pub path: String,
}
#[derive(Clone, Deserialize)]
pub struct ContentQuery {
    pub id: Uuid,
}
