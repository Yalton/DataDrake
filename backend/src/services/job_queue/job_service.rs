use std::{collections::HashMap, sync::{Arc, Mutex}};


use super::structs::JobQueue;

#[derive(Debug, Clone)]
pub struct JobService {
    pub job_queue: JobQueue,
}

impl JobService {
    pub fn new() -> Self {
        JobService {
            job_queue: Arc::new(Mutex::new(HashMap::new())),
        }
    }
}
