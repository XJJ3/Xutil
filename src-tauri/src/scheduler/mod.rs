use tokio_cron_scheduler::{Job, JobScheduler, JobSchedulerError};

pub async fn init_job_scheduler() -> Result<(), JobSchedulerError> {
    println!("==================开始注册任务系统========================");
    let sched = JobScheduler::new().await?;

    sched
        .add(Job::new("1/10 * * * * *", |_uuid, _l| {
            println!("I run every 10 seconds");
        })?)
        .await?;
    sched.start().await?; // Start the scheduler
    println!("==================完成注册任务系统========================");
    Ok(())
}
