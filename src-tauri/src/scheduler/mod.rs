use tokio_cron_scheduler::{Job, JobScheduler, JobSchedulerError};

pub async fn init_job_scheduler() -> Result<(), JobSchedulerError> {
    println!("开始初始化任务");
    let sched = JobScheduler::new().await?;
    println!("=========");

    // Add basic cron job
    sched
        .add(Job::new("1/10 * * * * *", |_uuid, _l| {
            println!("I run every 10 seconds");
        })?)
        .await?;

    // Start the scheduler
    sched.start().await?;

    println!("开始跑任务！！");

    // // Wait while the jobs run
    // tokio::time::sleep(Duration::from_secs(100)).await;

    Ok(())
}
