use crate::{
    common::{entity::SchedulerData, util::*},
    GLOBAL_SCHEDULER_MANAGE,
};

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
            println!("当前内容： {:?}", cont);
            all_scheduler_job = serde_json::from_str::<Vec<SchedulerData>>(&cont).unwrap();
        }
        all_scheduler_job.push(params.clone());

        let json_data = serde_json::to_string(&all_scheduler_job).unwrap();

        println!("最后的结果{:?}", json_data);

        match write_user_scheduler_setting_data(json_data) {
            Ok(_) => println!("写入成功"),
            Err(why) => {
                println!("写入失败: {}", why);
                return Err("写入失败".to_string());
            }
        };

        let manage = unsafe { GLOBAL_SCHEDULER_MANAGE.get_mut().unwrap() };
        manage.add_job(&params);

        Ok(serde_json::json!({ "result": "all_cmd_data" }))
    }
}

pub struct SwitchSchedulerJob;
impl CommandTrait for SwitchSchedulerJob {
    fn execute(args: &serde_json::Value) -> Result<serde_json::Value, String> {
        let params: SchedulerData =
            serde_json::from_str::<SchedulerData>(&args.to_string()).unwrap();

        Ok(serde_json::json!({ "result": "" }))
    }
}
