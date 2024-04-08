use super::{
    common::*,
    util::{read_user_command_setting_data, write_user_command_setting_data},
};
use std::{process::Command, thread, time::Duration};

pub struct GetAllCommands;

impl CommandTrait for GetAllCommands {
    fn execute(_args: &serde_json::Value) -> Result<serde_json::Value, String> {
        let commands_data = match read_user_command_setting_data() {
            Ok(cont) => Some(cont),
            Err(_) => None,
        };
        println!("获取到的内容： {:?}", commands_data);
        Ok(serde_json::json!({ "result": commands_data }))
    }
}

pub struct AddCommandGroup;
impl CommandTrait for AddCommandGroup {
    fn execute(args: &serde_json::Value) -> Result<serde_json::Value, String> {
        println!("指令：{}", args);
        let params = serde_json::from_str::<AddCommandGroupParams>(&args.to_string()).unwrap();
        println!("准备写入指令数据：{:?}", params);

        let group_name = match params.group_name {
            Some(name) => name,
            None => "default".to_string(),
        };

        let group_icon = match params.group_icon {
            Some(icon) => icon,
            None => "/setting.icon".to_string(),
        };

        let contents = match read_user_command_setting_data() {
            Ok(cont) => Some(cont),
            Err(_) => None,
        };

        let mut all_cmd_data: Vec<CommandGroupData> = vec![CommandGroupData {
            group_name: String::from("default"),
            group_icon: None,
            commands: Vec::new(),
        }];

        if let Some(cont) = contents {
            all_cmd_data = serde_json::from_str::<Vec<CommandGroupData>>(&cont).unwrap();
            all_cmd_data.push(CommandGroupData {
                group_name: group_name,
                group_icon: Some(group_icon),
                commands: Vec::new(),
            })
        } else {
            all_cmd_data[0].group_name = group_name;
            all_cmd_data[0].group_icon = Some(group_icon);
        }

        println!("现有数据： {:?}", all_cmd_data);

        let json_data = serde_json::to_string(&all_cmd_data).unwrap();
        println!("添加完之后的数据: {:?}", json_data);

        match write_user_command_setting_data(json_data) {
            Ok(_) => println!("写入成功"),
            Err(why) => println!("写入失败: {}", why),
        };

        Ok(serde_json::json!({ "result": "CommandOne executed" }))
    }
}

pub struct AddCommand;
impl CommandTrait for AddCommand {
    fn execute(args: &serde_json::Value) -> Result<serde_json::Value, String> {
        println!("指令：{}", args);
        let params = serde_json::from_str::<AddCommandParams>(&args.to_string()).unwrap();
        println!("准备写入指令数据：{:?}", params);

        let group_name = match params.group_name {
            Some(name) => name,
            None => "default".to_string(),
        };

        let contents = match read_user_command_setting_data() {
            Ok(cont) => Some(cont),
            Err(_) => None,
        };

        let mut all_cmd_data: Vec<CommandGroupData> = vec![CommandGroupData {
            group_name: String::from("default"),
            group_icon: None,
            commands: Vec::new(),
        }];

        if let Some(cont) = contents {
            all_cmd_data = serde_json::from_str::<Vec<CommandGroupData>>(&cont).unwrap();
        }

        println!("现有数据： {:?}", all_cmd_data);

        for cmd_group in all_cmd_data.iter_mut() {
            if group_name == cmd_group.group_name {
                cmd_group.commands.push(CommandData {
                    cmd: params.cmd.clone(),
                    args: params.args.clone(),
                    current_dir: params.current_dir.clone(),
                });
            }
        }

        let json_data = serde_json::to_string(&all_cmd_data).unwrap();
        println!("添加完之后的数据: {:?}", json_data);

        match write_user_command_setting_data(json_data) {
            Ok(_) => println!("写入成功"),
            Err(why) => println!("写入失败: {}", why),
        };

        Ok(serde_json::json!({ "result": "CommandOne executed" }))
    }
}

pub struct ExecuteCmd;

impl CommandTrait for ExecuteCmd {
    fn execute(args: &serde_json::Value) -> Result<serde_json::Value, String> {
        let params = match serde_json::from_str::<CommandData>(&args.to_string()) {
            Ok(arg) => arg,
            Err(_) => return Err("参数解析失败！".to_string()),
        };

        if params.cmd.is_empty() {
            return Err("请传入指令".to_string());
        }
        let mut cmd = Command::new(params.cmd);
        cmd.args(&params.args);

        if let Some(cur_dir) = params.current_dir {
            cmd.current_dir(cur_dir);
        }

        let handle = thread::spawn(move || {
            let status = cmd.status().expect("Failed to execute command");

            thread::sleep(Duration::from_secs(5));

            // 检查命令是否成功执行
            if status.success() {
                println!("Script executed successfully!");
            } else {
                println!("Script failed to execute: {}", status);
            }

            status
        });

        let status = handle.join().unwrap();

        Ok(serde_json::json!({ "result": format!("{}", status) }))
    }
}
