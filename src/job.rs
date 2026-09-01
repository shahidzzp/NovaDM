use std::path::PathBuf;

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum JobStatus {
    Queued,
    Downloading,
    Paused,
    Completed,
    Failed,
    Cancelled,
}

#[derive(Debug, Clone)]
pub struct DownloadJob {
    pub id: u64,
    pub url: String,
    pub filename: String,
    pub destination: PathBuf,
    pub total_size: Option<u64>,
    pub downloaded: u64,
    pub status: JobStatus,
}

impl DownloadJob {
    pub fn new(
        id: u64,
        url: String,
        filename: String,
        destination: PathBuf,
        total_size: Option<u64>,
    ) -> Self {
        Self {
            id,
            url,
            filename,
            destination,
            total_size,
            downloaded: 0,
            status: JobStatus::Queued,
        }
    }

    pub fn start(&mut self) {
        self.status = JobStatus::Downloading;
    }

    pub fn pause(&mut self) {
        self.status = JobStatus::Paused;
    }

    pub fn complete(&mut self) {
        self.status = JobStatus::Completed;

        if let Some(total) = self.total_size {
            self.downloaded = total;
        }
    }

    pub fn fail(&mut self) {
        self.status = JobStatus::Failed;
    }

    pub fn cancel(&mut self) {
        self.status = JobStatus::Cancelled;
    }

    pub fn progress(&self) -> f64 {
        match self.total_size {
            Some(total) if total > 0 => {
                self.downloaded as f64 / total as f64 * 100.0
            }
            _ => 0.0,
        }
    }
}
