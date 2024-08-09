use axum::extract::{Json, State};
use axum::http::StatusCode;
use axum::response::IntoResponse;
use chrono::Utc;
use serde_json::json;
use std::fs;
use crate::endpoint::jobs::*;
use crate::endpoint::structs::*;
use crate::endpoint::Error;
use crate::endpoint::Result;
use crate::services::*;
use uuid::Uuid;
use futures::stream::{self, StreamExt};
use serde::Serialize;
use tokio_util::codec::{BytesCodec, FramedRead};
use futures::TryStreamExt;

//curl -X POST -H "Content-Type: application/json" -d '{"path": "DIRECTORY"}' http://0.0.0.0:8000/scan_directory
#[tracing::instrument(skip_all)]
pub async fn scan_directory(
    State(scan_service): State<ScanService>,
    State(job_service): State<JobService>,
    State(database_service): State<DatabaseService>,
    Json(request): Json<PathRequest>,
) -> Result<impl IntoResponse> {
    tracing::info!("Scanning Root Directory {}", request.path);
    let job_id = Uuid::new_v4();
    let job = Job {
        id: job_id,
        root_path: request.path.clone(),
        state: JobState::Pending,
        created_at: Utc::now(),
    };
    
    tracing::info!("Created new ScanJob with id {}", job_id);


    let mut queue = job_service.job_queue.lock().unwrap();
    queue.insert(job_id, job);
    drop(queue);

    tracing::info!("Spawning thread for ScanJob with id {}", job_id);
    tokio::spawn(process_job(
        job_id,
        request.path.clone(),
        job_service.clone(),
        scan_service.clone(),
        database_service.clone(),
    ));

    Ok((StatusCode::ACCEPTED, Json(json!({ "job_id": job_id }))))
}


#[tracing::instrument(skip_all)]
pub async fn get_all_jobs(
    State(job_service): State<JobService>,
) -> Result<(StatusCode, axum::Json<String>)> {
    tracing::info!("Getting all jobs");

    let queue = job_service.job_queue.lock().unwrap();
    let jobs: Vec<Job> = queue.values().cloned().collect();

    let json_string = serde_json::to_string(&jobs).unwrap();
    Ok((StatusCode::OK, Json(json_string)))
}

//curl -X POST -H "Content-Type: application/json" -d '{"path": "/home/yalt/Pictures"}' http://0.0.0.0:8000/get_scan
#[tracing::instrument(skip_all)]
pub async fn get_all_scans_by_root_path(
    State(_scan): State<ScanService>,
    State(_db): State<DatabaseService>,
    Json(_json): Json<PathRequest>,
) -> Result<impl IntoResponse> {
    match _db.fetch_scans_by_root_directory(&_json.path).await {
        Ok(scans) => {
            let stream = stream::iter(scans)
                .map(|scan| serde_json::to_vec(&scan))
                .map(|result| {
                    result.map_err(|e| {
                        std::io::Error::new(std::io::ErrorKind::Other, e.to_string())
                    })
                })
                .map(|chunk| chunk.map(axum::body::Bytes::from))
                .map(|chunk| {
                    chunk.map(|bytes| {
                        let mut buffer = bytes.to_vec();
                        buffer.push(b'\n');
                        axum::body::Bytes::from(buffer)
                    })
                })
                .map_err(|e| e);

            let body = axum::body::StreamBody::new(stream);

            Ok((
                StatusCode::OK,
                axum::response::Response::builder()
                    .header("Content-Type", "application/json")
                    .body(body)
                    .unwrap(),
            ))
        }
        Err(e) => {
            tracing::error!("Failed to fetch scans by root path: {}", e);
            Err(Error::new("Failed to fetch scans by root path").with_status(StatusCode::INTERNAL_SERVER_ERROR))
        }
    }
}

//curl -X POST http://0.0.0.0:8000/get_scan/all
#[tracing::instrument(skip_all)]
pub async fn get_all_scans(
    State(_scan): State<ScanService>,
    State(_db): State<DatabaseService>,
) -> Result<impl IntoResponse> {
    match _db.fetch_directory_scans().await {
        Ok(scans) => Ok((StatusCode::OK, Json(scans))),
        Err(e) => {
            tracing::error!("Failed to fetch all scans: {}", e);
            Err(Error::new("Failed to fetch all scans").with_status(StatusCode::INTERNAL_SERVER_ERROR))
        }
    }
}

// curl -X DELETE -H "Content-Type: application/json" -d '{"root_path": "/home/yalt/Pictures"}' http://0.0.0.0:8000/delete_scan
#[tracing::instrument(skip_all)]
pub async fn delete_all_scans_by_root_path(
    State(_scan): State<ScanService>,
    State(_db): State<DatabaseService>,
    Json(_json): Json<PathRequest>,
) -> Result<impl IntoResponse> {
    match _db.delete_scans_by_root_directory(&_json.path).await {
        Ok(_) => Ok((StatusCode::OK, Json("Deleted".to_string()))),
        Err(e) => {
            tracing::error!("Failed to delete scans by root path: {}", e);
            Err(Error::new("Failed to delete scans by root path").with_status(StatusCode::INTERNAL_SERVER_ERROR))
        }
    }
}

// curl -X DELETE http://0.0.0.0:8000/delete_scan/all
#[tracing::instrument(skip_all)]
pub async fn delete_all_scans(
    State(_scan): State<ScanService>,
    State(_db): State<DatabaseService>,
) -> Result<impl IntoResponse> {
    match _db.delete_all_scans().await {
        Ok(_) => Ok((StatusCode::OK, Json("Deleted".to_string()))),
        Err(e) => {
            tracing::error!("Failed to delete all scans: {}", e);
            Err(Error::new("Failed to delete all scans").with_status(StatusCode::INTERNAL_SERVER_ERROR))
        }
    }
}

// curl -X GET http://0.0.0.0:8000/get_root_paths
#[tracing::instrument(skip_all)]
pub async fn get_root_paths(
    State(_scan): State<ScanService>,
    State(_db): State<DatabaseService>,
) -> Result<impl IntoResponse> {
    match _db.fetch_unique_root_paths().await {
        Ok(unique_paths) => Ok((StatusCode::OK, Json(unique_paths))),
        Err(e) => {
            tracing::error!("Failed to fetch unique root paths: {}", e);
            Err(Error::new("Failed to fetch unique root paths").with_status(StatusCode::INTERNAL_SERVER_ERROR))
        }
    }
}



#[tracing::instrument(skip_all)]
pub async fn delete_file_by_path(
    State(_scan): State<ScanService>,
    State(_db): State<DatabaseService>,
    Json(payload): Json<PathRequest>,
) -> Result<impl IntoResponse> {
    let path = &payload.path;

    // Check if the file exists in the database
    match _db.fetch_scans_by_path(path).await {
        Ok(scans) => {
            if !scans.is_empty() {
                // Delete the file from the file system
                match fs::remove_file(path) {
                    Ok(_) => {
                        // Delete the scan entry from the database
                        match _db.delete_scan_by_path(path).await {
                            Ok(_) => Ok((StatusCode::OK, Json("File deleted successfully".to_string()))),
                            Err(e) => {
                                tracing::error!("Failed to delete scan entry: {}", e);
                                Err(Error::new("Failed to delete scan entry").with_status(StatusCode::INTERNAL_SERVER_ERROR))
                            }
                        }
                    }
                    Err(e) => {
                        tracing::error!("Failed to delete file: {}", e);
                        Err(Error::new("Failed to delete file").with_status(StatusCode::INTERNAL_SERVER_ERROR))
                    }
                }
            } else {
                Err(Error::new("File not found in the database").with_status(StatusCode::NOT_FOUND))
            }
        }
        Err(e) => {
            tracing::error!("Failed to fetch scans by path: {}", e);
            Err(Error::new("Failed to fetch scans by path").with_status(StatusCode::INTERNAL_SERVER_ERROR))
        }
    }
}