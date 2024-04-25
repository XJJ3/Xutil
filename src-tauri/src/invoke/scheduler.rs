use crate::common::{entity::SchedulerData, util::*};

use super::common::*;

pub struct GetAllSchedulerJob;
impl CommandTrait for GetAllSchedulerJob {
    fn execute(_args: &serde_json::Value) -> Result<serde_json::Value, String> {
        let scheduler = match read_user_scheduler_setting_data() {
            Ok(cont) => Some(cont),
            Err(_) => None,
        };
        // println!("获取到的内容： {:?}", commands_data);
        Ok(serde_json::json!({ "result": scheduler }))
    }
}

pub struct AddSchedulerJob;
impl CommandTrait for AddSchedulerJob {
    fn execute(args: &serde_json::Value) -> Result<serde_json::Value, String> {
        println!("任务：{}", args);
        let params: SchedulerData =
            serde_json::from_str::<SchedulerData>(&args.to_string()).unwrap();
        println!("准备写入任务数据：{:?}", params);

        let contents = match read_user_scheduler_setting_data() {
            Ok(cont) => Some(cont),
            Err(_) => None,
        };

        let mut all_scheduler_job: Vec<SchedulerData> = vec![];
        if let Some(cont) = contents {
            all_scheduler_job = serde_json::from_str::<Vec<SchedulerData>>(&cont).unwrap();
            all_scheduler_job.push(params);
        }

        let json_data = serde_json::to_string(&all_scheduler_job).unwrap();

        match write_user_scheduler_setting_data(json_data) {
            Ok(_) => println!("写入成功"),
            Err(why) => println!("写入失败: {}", why),
        };

        Ok(serde_json::json!({ "result": "all_cmd_data" }))
    }
}

pub struct RunSchedulerJob;
impl CommandTrait for RunSchedulerJob {
    fn execute(args: &serde_json::Value) -> Result<serde_json::Value, String> {
        let params: SchedulerData =
            serde_json::from_str::<SchedulerData>(&args.to_string()).unwrap();

        Ok(serde_json::json!({ "result": "" }))
    }
}
