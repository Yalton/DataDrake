

use crate::endpoint::Result;

use crate::services::*;

use std::path::PathBuf;
use uuid::Uuid;

#[tracing::instrument(skip_all)]
pub async fn process_job(
    job_id: Uuid,
    root_path: String,
    job_service: JobService,
    scan_service: ScanService,
    database_service: DatabaseService,
) -> Result<String> {
    {
        let mut queue = job_service.job_queue.lock().unwrap();
        if let Some(job) = queue.get_mut(&job_id) {
            job.state = JobState::InProgress;
        }
    }

    let result =
        scan_directory_logic(&scan_service, &database_service, root_path).await;

    {
        let mut queue = job_service.job_queue.lock().unwrap();
        if let Some(job) = queue.get_mut(&job_id) {
            job.state = match &result {
                Ok(result) => JobState::Completed(result.clone()),
                Err(e) => JobState::Failed(e.to_string()),
            };
        }
    }
    tracing::info!("Job with id: {:?} is complete", job_id);
    result
}

pub async fn scan_directory_logic(
    scan_service: &ScanService,
    database_service: &DatabaseService,
    root_path: String,
) -> Result<String> {
    tracing::info!("Performing scan on {:?}", root_path);
    let scans = scan_service.scan_directory(PathBuf::from(&root_path)).await;
    tracing::info!("Deleting old entries with RootPath: {:?}", root_path);
    database_service
        .delete_scans_by_root_directory(&root_path)
        .await?;
    database_service
        .insert_directory_scans(scans.clone())
        .await?;
    
    Ok(String::from("Scan completed successfully"))
}