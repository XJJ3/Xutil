use std::collections::HashMap;
use std::str::FromStr;

use cron::Schedule;
use tokio_cron_scheduler::{Job, JobScheduler, JobSchedulerError};
use uuid::Uuid;

use crate::common::{entity::SchedulerData, util::read_user_scheduler_setting_data};

pub struct SchedulerManage {
    scheduler: JobScheduler,
    running_job_map: HashMap<String, Uuid>,
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
                let notice_title = sched_job.notice_title.clone();
                println!("{}", sched_job.scheduler);
                let scheduler = &sched_job.scheduler;
                let schedule = Schedule::from_str(scheduler).unwrap();

                let job_id = self
                    .scheduler
                    .add(
                        Job::new(schedule, move |_uuid, _l| {
                            println!("{}", notice_title);
                        })
                        .unwrap(),
                    )
                    .await
                    .unwrap();
                self.running_job_map
                    .insert(sched_job.scheduler_id.to_string(), job_id);
            }
        }

        self.scheduler.start().await.unwrap();
        self
    }

    pub async fn add_job(&mut self, sched_job: &SchedulerData) -> Result<(), JobSchedulerError> {
        let notice_title = sched_job.notice_title.clone();
        let scheduler = &sched_job.scheduler;
        let schedule = Schedule::from_str(scheduler).unwrap();

        let job_id = self
            .scheduler
            .add(Job::new(schedule, move |_uuid, _l| {
                println!("{}", notice_title);
            })?)
            .await?;
        self.running_job_map
            .insert(sched_job.scheduler_id.clone(), job_id);
        Ok(())
    }

    pub async fn switch_job(&mut self, sched_job: &SchedulerData) -> Result<(), JobSchedulerError> {
        let sched_job_id = &sched_job.scheduler_id;
        let is_running = &sched_job.is_run;
        if *is_running {
            let job_id_option = self.running_job_map.get(sched_job_id);
            if let Some(job_id) = job_id_option {
                let _ = self.scheduler.remove(job_id).await;
            }
        } else {
            let notice_title = sched_job.notice_title.clone();
            let scheduler = &sched_job.scheduler;
            let schedule = Schedule::from_str(scheduler).unwrap();

            let job_id = self
                .scheduler
                .add(Job::new(schedule, move |_uuid, _l| {
                    println!("{}", notice_title);
                })?)
                .await?;
            self.running_job_map
                .insert(sched_job.scheduler_id.clone(), job_id);
        }

        Ok(())
    }

    pub fn get_running_jobs(&self) -> &HashMap<String, Uuid> {
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
//     sched.start().await ?; // Start the scheduler
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
