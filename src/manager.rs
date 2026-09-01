use crate::job::{DownloadJob, JobStatus};
use std::path::PathBuf;

pub struct DownloadManager {
    jobs: Vec<DownloadJob>,
    next_id: u64,
}

impl DownloadManager {
    /// Create a new empty download manager.
    pub fn new() -> Self {
        Self {
            jobs: Vec::new(),
            next_id: 1,
        }
    }

    /// Add a new download to the queue.
    pub fn add_job(
        &mut self,
        url: String,
        filename: String,
        destination: PathBuf,
        total_size: Option<u64>,
    ) -> u64 {
        let id = self.next_id;

        let job = DownloadJob::new(
            id,
            url,
            filename,
            destination,
            total_size,
        );

        self.jobs.push(job);
        self.next_id += 1;

        id
    }

    /// Return the next queued job.
    pub fn next_queued_job(&self) -> Option<&DownloadJob> {
        self.jobs
            .iter()
            .find(|job| job.status == JobStatus::Queued)
    }

    /// Get a job by ID.
    pub fn get_job(&self, id: u64) -> Option<&DownloadJob> {
        self.jobs
            .iter()
            .find(|job| job.id == id)
    }

    /// Get a mutable job by ID.
    pub fn get_job_mut(
        &mut self,
        id: u64,
    ) -> Option<&mut DownloadJob> {
        self.jobs
            .iter_mut()
            .find(|job| job.id == id)
    }

    /// Start a queued or paused job.
    pub fn start_job(&mut self, id: u64) -> bool {
        if let Some(job) = self.get_job_mut(id) {
            match job.status {
                JobStatus::Queued | JobStatus::Paused => {
                    job.start();
                    true
                }

                _ => false,
            }
        } else {
            false
        }
    }

    /// Pause a running job.
    pub fn pause_job(&mut self, id: u64) -> bool {
        if let Some(job) = self.get_job_mut(id) {
            if job.status == JobStatus::Downloading {
                job.pause();
                return true;
            }
        }

        false
    }

    /// Mark a job as completed.
    pub fn complete_job(&mut self, id: u64) -> bool {
        if let Some(job) = self.get_job_mut(id) {
            job.complete();
            return true;
        }

        false
    }

    /// Mark a job as failed.
    pub fn fail_job(&mut self, id: u64) -> bool {
        if let Some(job) = self.get_job_mut(id) {
            job.fail();
            return true;
        }

        false
    }

    /// Cancel a job.
    pub fn cancel_job(&mut self, id: u64) -> bool {
        if let Some(job) = self.get_job_mut(id) {
            match job.status {
                JobStatus::Completed
                | JobStatus::Cancelled => false,

                _ => {
                    job.cancel();
                    true
                }
            }
        } else {
            false
        }
    }

    /// Return all jobs.
    pub fn jobs(&self) -> &[DownloadJob] {
        &self.jobs
    }

    /// Return all active jobs.
    pub fn active_jobs(&self) -> Vec<&DownloadJob> {
        self.jobs
            .iter()
            .filter(|job| {
                matches!(
                    job.status,
                    JobStatus::Queued
                        | JobStatus::Downloading
                        | JobStatus::Paused
                )
            })
            .collect()
    }

    /// Return completed jobs.
    pub fn completed_jobs(&self) -> Vec<&DownloadJob> {
        self.jobs
            .iter()
            .filter(|job| job.status == JobStatus::Completed)
            .collect()
    }

    /// Return failed jobs.
    pub fn failed_jobs(&self) -> Vec<&DownloadJob> {
        self.jobs
            .iter()
            .filter(|job| job.status == JobStatus::Failed)
            .collect()
    }

    /// Return cancelled jobs.
    pub fn cancelled_jobs(&self) -> Vec<&DownloadJob> {
        self.jobs
            .iter()
            .filter(|job| job.status == JobStatus::Cancelled)
            .collect()
    }

    /// Count all jobs.
    pub fn job_count(&self) -> usize {
        self.jobs.len()
    }

    /// Count queued jobs.
    pub fn queued_count(&self) -> usize {
        self.jobs
            .iter()
            .filter(|job| job.status == JobStatus::Queued)
            .count()
    }

    /// Count currently downloading jobs.
    pub fn downloading_count(&self) -> usize {
        self.jobs
            .iter()
            .filter(|job| job.status == JobStatus::Downloading)
            .count()
    }

    /// Count completed jobs.
    pub fn completed_count(&self) -> usize {
        self.jobs
            .iter()
            .filter(|job| job.status == JobStatus::Completed)
            .count()
    }

    /// Remove a job.
    ///
    /// Completed, failed, and cancelled jobs can be removed.
    /// Active downloads are protected from accidental removal.
    pub fn remove_job(&mut self, id: u64) -> bool {
        let removable = self
            .get_job(id)
            .map(|job| {
                matches!(
                    job.status,
                    JobStatus::Completed
                        | JobStatus::Failed
                        | JobStatus::Cancelled
                )
            })
            .unwrap_or(false);

        if !removable {
            return false;
        }

        let old_len = self.jobs.len();

        self.jobs.retain(|job| job.id != id);

        self.jobs.len() != old_len
    }

    /// Clear completed, failed, and cancelled jobs.
    pub fn clear_finished(&mut self) {
        self.jobs.retain(|job| {
            !matches!(
                job.status,
                JobStatus::Completed
                    | JobStatus::Failed
                    | JobStatus::Cancelled
            )
        });
    }
}

impl Default for DownloadManager {
    fn default() -> Self {
        Self::new()
    }
}
