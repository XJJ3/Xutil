use super::{
    common::*,
    util::{read_user_command_setting_data, write_user_command_setting_data},
};

#[derive(Debug)]
pub struct CommandOne;

impl CommandTrait for CommandOne {
    fn execute(args: &serde_json::Value) -> Result<serde_json::Value, String> {
        // 执行命令逻辑

        println!("指令：{}", args);

        let user_data = UserData {
            name: String::from("张三"),
            age: 0,
        };

        let json_data = serde_json::to_string(&user_data).unwrap();

        println!("准备写入数据：{:?}", json_data);

        match write_user_command_setting_data(json_data) {
            Ok(_) => println!("写入成功"),
            Err(why) => println!("写入失败: {}", why),
        };

        Ok(serde_json::json!({ "result": "CommandOne executed" }))
    }
}

#[derive(Debug)]
pub struct CommandTwo;

impl CommandTrait for CommandTwo {
    fn execute(_args: &serde_json::Value) -> Result<serde_json::Value, String> {
        let contents = match read_user_command_setting_data() {
            Ok(cont) => Some(cont),
            Err(_) => None,
        };

        println!("获取到的内容： {:?}", contents);

        if let Some(c) = contents {
            let all_cmd_data = serde_json::from_str::<Vec<CommandGroupData>>(&c).unwrap();
            // 输出文件内容
            println!("File contents: {:?}", all_cmd_data);
        } else {
            println!("无内容");
        }

        // 执行命令逻辑
        Ok(serde_json::json!({ "result": "CommandTwo executed" }))
    }
}

#[derive(Debug)]
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
