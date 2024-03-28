use super::structs::DirectoryScan;
use anyhow::{Error, Result};
use sqlx::ConnectOptions;
use sqlx::Connection;
use sqlx::{migrate::Migrator, SqlitePool};
static MIGRATOR: Migrator = sqlx::migrate!();

#[derive(Debug, Clone)]
pub struct DatabaseService {
    db_pool: SqlitePool,
}

impl DatabaseService {
    #[tracing::instrument(skip_all)]
    pub async fn connect(database_url: &str) -> Result<Self> {
        // Extract the filename from the database URL
        let filename = database_url.trim_start_matches("sqlite:");

        // Check if the SQLite database file exists
        let db_exists = std::path::Path::new(filename).exists();

        // Create the database file if it doesn't exist
        if !db_exists {
            tracing::info!("Creating SQLite database file: {}", filename);
            let conn = sqlx::sqlite::SqliteConnectOptions::new()
                .filename(filename)
                .create_if_missing(true)
                .connect()
                .await?;
            conn.close().await?;
        } else {
            tracing::info!("Database exists, proceeding...")
        }

        let db_pool = SqlitePool::connect(database_url).await?;
        // Attempt to run migrations
        match MIGRATOR.run(&db_pool).await {
            Ok(_) => tracing::info!("Database migrations applied successfully."),
            Err(e) => tracing::error!("Failed to apply database migrations: {}", e),
        }
        Ok(Self { db_pool })
    }

    // This method should now accept a Vec<DirectoryScan> for batch insertion
    #[tracing::instrument(skip(self, scans))]
    pub async fn insert_directory_scans(&self, scans: Vec<DirectoryScan>) -> Result<(), Error> {
        tracing::info!("Inserting directory scans in batch");

        let mut transaction = self.db_pool.begin().await?;

        for scan in scans.iter() {
            sqlx::query("INSERT INTO directory_scans (path, size, root_path, scan_time) VALUES (?, ?, ?, ?)")
        .bind(&scan.path)
        .bind(scan.size)
        .bind(&scan.root_path)
        .bind(scan.scan_time)
        .execute(&mut *transaction) // Explicitly dereference transaction here
        .await?;
        }

        transaction.commit().await?;

        tracing::info!("Batch directory scans inserted successfully.");
        Ok(())
    }

    #[tracing::instrument(skip(self))]
    pub async fn fetch_directory_scans(&self) -> Result<Vec<DirectoryScan>, Error> {
        tracing::info!("Fetching all directory scans.");
        match sqlx::query_as::<_, DirectoryScan>("SELECT * FROM directory_scans")
            .fetch_all(&self.db_pool)
            .await
        {
            Ok(scans) => {
                tracing::info!("Successfully fetched directory scans.");
                Ok(scans)
            }
            Err(e) => {
                tracing::error!("Failed to fetch directory scans: {}", e);
                Err(Error::from(e))
            }
        }
    }

    #[tracing::instrument(skip(self))]
    pub async fn fetch_unique_root_paths(&self) -> Result<Vec<String>, Error> {
        tracing::info!("Fetching unique root paths");
        match sqlx::query_scalar("SELECT DISTINCT root_path FROM directory_scans")
            .fetch_all(&self.db_pool)
            .await
        {
            Ok(root_paths) => {
                tracing::info!("Successfully fetched unique root paths.");
                Ok(root_paths)
            }
            Err(e) => {
                tracing::error!("Failed to fetch unique root paths: {}", e);
                Err(Error::from(e))
            }
        }
    }
    #[tracing::instrument(skip(self))]
    pub async fn delete_directory_scan_by_id(&self, id: i64) -> Result<(), Error> {
        tracing::info!("Deleting directory scan with ID: {}", id);
        match sqlx::query("DELETE FROM directory_scans WHERE id = ?")
            .bind(id)
            .execute(&self.db_pool)
            .await
        {
            Ok(_) => tracing::info!("Directory scan deleted successfully."),
            Err(e) => {
                tracing::error!("Failed to delete directory scan: {}", e);
                return Err(Error::from(e));
            }
        }
        Ok(())
    }

    #[tracing::instrument(skip(self))]
    pub async fn delete_scan_by_path(&self, path: &str) -> Result<(), Error> {
        tracing::info!("Deleting scan entry with path: {}", path);
        match sqlx::query("DELETE FROM directory_scans WHERE path = ?")
            .bind(path)
            .execute(&self.db_pool)
            .await
        {
            Ok(_) => {
                tracing::info!("Scan entry deleted successfully.");
                Ok(())
            }
            Err(e) => {
                tracing::error!("Failed to delete scan entry: {}", e);
                Err(Error::from(e))
            }
        }
    }

    /// Fetches all directory scans with the same root directory.
    #[tracing::instrument(skip(self))]
    pub async fn fetch_scans_by_root_directory(
        &self,
        root_directory: &str,
    ) -> Result<Vec<DirectoryScan>, Error> {
        tracing::info!(
            "Fetching directory scans for root directory: {:?}",
            root_directory
        );
        match sqlx::query_as::<_, DirectoryScan>(
            "SELECT * FROM directory_scans WHERE root_path = ?",
        )
        .bind(root_directory)
        .fetch_all(&self.db_pool)
        .await
        {
            Ok(scans) => {
                tracing::info!("Successfully fetched directory scans for root directory.");
                Ok(scans)
            }
            Err(e) => {
                tracing::error!("Failed to fetch directory scans for root directory: {}", e);
                Err(Error::from(e))
            }
        }
    }

    /// Deletes all directory scans with the same root directory.
    #[tracing::instrument(skip(self))]
    pub async fn delete_scans_by_root_directory(&self, root_directory: &str) -> Result<(), Error> {
        tracing::info!(
            "Deleting directory scans for root directory: {:?}",
            root_directory
        );
        match sqlx::query("DELETE FROM directory_scans WHERE root_path = ?")
            .bind(root_directory)
            .execute(&self.db_pool)
            .await
        {
            Ok(_) => {
                tracing::info!("Directory scans for root directory deleted successfully.");
                Ok(())
            }
            Err(e) => {
                tracing::error!("Failed to delete directory scans for root directory: {}", e);
                Err(Error::from(e))
            }
        }
    }
    #[tracing::instrument(skip(self))]
    pub async fn delete_all_scans(&self) -> Result<(), Error> {
        tracing::info!("Deleting all directory scans.");
        match sqlx::query("DELETE FROM directory_scans")
            .execute(&self.db_pool)
            .await
        {
            Ok(_) => {
                tracing::info!("All directory scans deleted successfully.");
                Ok(())
            }
            Err(e) => {
                tracing::error!("Failed to delete all directory scans: {}", e);
                Err(Error::from(e))
            }
        }
    }
    #[tracing::instrument(skip(self))]
    pub async fn fetch_scans_by_path(&self, path: &str) -> Result<Vec<DirectoryScan>, Error> {
        tracing::info!("Fetching directory scans for path: {:?}", path);
        match sqlx::query_as::<_, DirectoryScan>("SELECT * FROM directory_scans WHERE path = ?")
            .bind(path)
            .fetch_all(&self.db_pool)
            .await
        {
            Ok(scans) => {
                tracing::info!("Successfully fetched directory scans for path.");
                Ok(scans)
            }
            Err(e) => {
                tracing::error!("Failed to fetch directory scans for path: {}", e);
                Err(Error::from(e))
            }
        }
    }
}
