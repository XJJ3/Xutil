use std::collections::HashMap;

use tokio_cron_scheduler::{Job, JobScheduler, JobSchedulerError};

use crate::common::{entity::SchedulerData, util::read_user_scheduler_setting_data};

pub struct SchedulerManage {
    scheduler: JobScheduler,
    running_job_map: HashMap<String, String>,
}

impl SchedulerManage {
    pub async fn new() -> Result<Self, JobSchedulerError> {
        let sched = JobScheduler::new().await?;
        Ok(SchedulerManage {
            scheduler: sched,
            running_job_map: HashMap::new(),
        })
    }

    pub async fn init(mut self) -> Self {
        let mut all_scheduler_job: Vec<SchedulerData> = vec![];
        if let Ok(cont) = read_user_scheduler_setting_data() {
            all_scheduler_job = serde_json::from_str::<Vec<SchedulerData>>(&cont).unwrap();
        }

        for sched_job in all_scheduler_job.iter() {
            if sched_job.is_run {
                let job_id = self
                    .scheduler
                    .add(
                        Job::new("1/10 * * * * *", |_uuid, _l| {
                            println!("I run every 10 seconds");
                        })
                        .unwrap(),
                    )
                    .await
                    .unwrap();
                self.running_job_map
                    .insert(sched_job.scheduler_id.to_string(), job_id.to_string());
            }
        }

        self.scheduler.start().await.unwrap();
        self
    }

    pub async fn add_job(&mut self, id: String) -> Result<(), JobSchedulerError> {
        let job_id = self
            .scheduler
            .add(Job::new("1/10 * * * * *", |_uuid, _l| {
                println!("I run every 10 seconds");
            })?)
            .await?;
        self.running_job_map.insert(id, job_id.to_string());
        Ok(())
    }

    pub fn get_running_jobs(&self) -> &HashMap<String, String> {
        &self.running_job_map
    }
}

// pub async fn init_job_scheduler() -> Result<(), JobSchedulerError> {
//     println!("==================开始注册任务系统========================");
//     let sched = JobScheduler::new().await?;

//     sched
//         .add(Job::new("1/10 * * * * *", |_uuid, _l| {
//             println!("I run every 10 seconds");
//         })?)
//         .await?;
//     sched.start().await?; // Start the scheduler
//     println!("==================完成注册任务系统========================");
//     Ok(())
// }

// pub async fn run_job_scheduler(scheduler: SchedulerData) -> Result<(), JobSchedulerError> {
//     let sched = JobScheduler::new().await?;

//     sched
//         .add(Job::new("1/10 * * * * *", |_uuid, _l| {
//             println!("I run every 10 seconds");
//         })?)
//         .await?;
//     sched.start().await?; // Start the scheduler

//     Ok(())
// }
