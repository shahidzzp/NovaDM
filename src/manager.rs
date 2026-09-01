use crate::job::{DownloadJob, JobStatus};

pub struct DownloadManager {
    jobs: Vec<DownloadJob>,
    next_id: u64,
}

impl DownloadManager {
    pub fn new() -> Self {
        Self {
            jobs: Vec::new(),
            next_id: 1,
        }
    }

    pub fn add_job(
        &mut self,
        url: String,
        filename: String,
        destination: std::path::PathBuf,
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

    pub fn get_job(&self, id: u64) -> Option<&DownloadJob> {
        self.jobs.iter().find(|job| job.id == id)
    }

    pub fn get_job_mut(
        &mut self,
        id: u64,
    ) -> Option<&mut DownloadJob> {
        self.jobs.iter_mut().find(|job| job.id == id)
    }

    pub fn start_job(&mut self, id: u64) -> bool {
        if let Some(job) = self.get_job_mut(id) {
            if job.status == JobStatus::Queued
                || job.status == JobStatus::Paused
            {
                job.start();
                return true;
            }
        }

        false
    }

    pub fn pause_job(&mut self, id: u64) -> bool {
        if let Some(job) = self.get_job_mut(id) {
            if job.status == JobStatus::Downloading {
                job.pause();
                return true;
            }
        }

        false
    }

    pub fn cancel_job(&mut self, id: u64) -> bool {
        if let Some(job) = self.get_job_mut(id) {
            if job.status != JobStatus::Completed
                && job.status != JobStatus::Cancelled
            {
                job.cancel();
                return true;
            }
        }

        false
    }

    pub fn complete_job(&mut self, id: u64) -> bool {
        if let Some(job) = self.get_job_mut(id) {
            job.complete();
            return true;
        }

        false
    }

    pub fn fail_job(&mut self, id: u64) -> bool {
        if let Some(job) = self.get_job_mut(id) {
            job.fail();
            return true;
        }

        false
    }

    pub fn jobs(&self) -> &[DownloadJob] {
        &self.jobs
    }

    pub fn active_jobs(&self) -> Vec<&DownloadJob> {
        self.jobs
            .iter()
            .filter(|job| {
                job.status == JobStatus::Queued
                    || job.status == JobStatus::Downloading
                    || job.status == JobStatus::Paused
            })
            .collect()
    }

    pub fn completed_jobs(&self) -> Vec<&DownloadJob> {
        self.jobs
            .iter()
            .filter(|job| job.status == JobStatus::Completed)
            .collect()
    }

    pub fn failed_jobs(&self) -> Vec<&DownloadJob> {
        self.jobs
            .iter()
            .filter(|job| job.status == JobStatus::Failed)
            .collect()
    }

    pub fn remove_job(&mut self, id: u64) -> bool {
        let old_len = self.jobs.len();

        self.jobs.retain(|job| job.id != id);

        self.jobs.len() != old_len
    }
}
