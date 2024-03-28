// setup/state.rs
use axum::extract::FromRef;

use crate::services::{DatabaseService, ScanService, JobService};
use crate::setup::{Args};

#[derive(Debug, Clone)]
pub struct State {
    scan_service: ScanService,
    database: DatabaseService,
    job_service: JobService,
}

impl State {
    pub async fn new(_args: &Args) -> anyhow::Result<Self> {
        let scan_service = ScanService::new();
        let database = DatabaseService::connect("sqlite:dir_scans.db").await?;
        let job_service = JobService::new();


        Ok(Self {
            scan_service,
            database,
            job_service
        })
    }
}

macro_rules! impl_di {
    ($($t:ty: $f:ident),+) => {$(
        impl FromRef<State> for $t {
            fn from_ref(state: &State) -> Self {
                state.$f.clone()
            }
        }
    )+};
}

impl_di!(ScanService: scan_service);
impl_di!(DatabaseService: database);
impl_di!(JobService: job_service);
