use moople_derive::MooplePacket;

use crate::id::job_id::{JobId, SubJob};

#[derive(MooplePacket, Debug)]
pub struct Job {
 pub job_id: JobId,
 pub sub_job: SubJob,
}

impl Job {
    pub fn new(job_id: JobId, sub_job: SubJob) -> Self {
        Self {
            job_id,
            sub_job
        }
    }
}

